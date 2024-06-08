use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_uf::{
    reader::Reader,
    data_store::DataStore,
    event_emitter::EventEmitter,
    exchange_router::ExchangeRouter,
    liquidation_handler::LiquidationHandler,
    ierc20::IERC20,
};
use clap::{Parser, ValueEnum};
use ethers::{
    contract::builders::ContractCall,
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, ValueOrArray, H160, I256, U256, U64},
};
use ethers_contract::Multicall;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    data_store: Address,
    reader: Address,
    event_emitter: Address,
    exchange_router: Address,
    liquidation_handler: Address,
    pool_configuration_utils: Address,
    weth: Address,
    multicall: Address,
    creation_block: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    TESTNET,
    OPTIMISM,
}

// admin stuff
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const RAY_DECIMALS: U256 = 27;
pub const WETH_DECIMALS: U256 = 18;
pub const STATE_CACHE_FILE: &str = "borrowers.json";

fn get_deployment_config(deployment: Deployment) -> DeploymentConfig {
    match deployment {
        Deployment::TESTNET => DeploymentConfig {
            data_store: Address::from_str("0x2d8A3C5677189723C4cB8873CfC9C8976FDF38Ac").unwrap(),
            reader: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            event_emitter: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            exchange_router: Address::from_str("0xA238Dd80C259a72e81d7e4664a9801593F98d1c5").unwrap(),
            liquidation_handler: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            pool_configuration_utils: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            weth: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            multicall: Address::from_str("0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156").unwrap(),
            creation_block: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    health: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    underlying_asset: Address,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UpStrategy<M> {
    client: Arc<M>,
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    pools: HashMap<Address, Pool>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    //liquidation_threshold:u64
}

impl<M: Middleware + 'static> UpStrategy<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
    ) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            pools: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment),
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
        }
    }
}

struct LiquidationProfit {
    borrower: Address,
    profit: I256,
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UpStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_pools().await?;
        self.approve_tokens().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> UpStrategy<M> {

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);
        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        info!("Total borrower count: {}", self.borrowers.len());
        let op = self
            .get_best_liquidation_opportunity()
            .await
            .map_err(|e| error!("Error finding liq ops: {}", e))
            .ok()??;

        info!("Best op - profit: {}", op.profit);

        if op.profit < I256::from(0) {
            info!("No profitable ops, passing");
            return None;
        }

        return Some(Action::SubmitTx(SubmitTxToMempool {
            tx: self
                .build_liquidation_tx(&op)
                .await
                .map_err(|e| error!("Error building liquidation: {}", e))
                .ok()?,
            gas_bid_info: Some(GasBidInfo {
                bid_percentage: self.bid_percentage,
                total_profit: U256::from_dec_str(&op.profit.to_string())
                    .map_err(|e| error!("Failed to bid: {}", e))
                    .ok()?,
            }),
        }));
    }

    // for all known borrowers, return a sorted set of those with health factor < 1
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256, U256, U256)>> {
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());

        let mut underwater_borrowers = Vec::new();

        // call pool.getUserAccountData(user) for each borrower
        let mut multicall = Multicall::new(self.client.clone(), self.config.multicall).await?;
        let borrowers: Vec<&Borrower> = self
            .borrowers
            .values()
            .filter(|b| b.debt.len() > 0)
            .collect();

        for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
            multicall.clear_calls();

            for borrower in chunk {
                multicall.add_call(reader.get_liquidation_health_factor(borrower.address), false);
            }

            let result: Vec<(U256, U256, bool, U256, U256)> = multicall.call_array().await?;
            for (borrower, (health_factor, is_health_factor_higher_than_liquidation_threshold, user_total_collateral_usd, user_total_debt_usd,) in zip(chunk, result) {
                if (!is_health_factor_higher_than_liquidation_threshold) {
                    info!(
                        "Found underwater borrower {:?} -  healthFactor: {} - user_total_collateral_usd: {} - user_total_debt_usd: {}",
                        borrower, health_factor, user_total_collateral_usd, user_total_debt_usd
                    );
                    underwater_borrowers.push((borrower.address, health_factor, user_total_collateral_usd, user_total_debt_usd));
                }
            }
        }

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.creation_block;
            }
        };

        Ok(())
    }

    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        info!(
            "Updating state from block {} to {}",
            self.last_block_number, latest_block
        );

        self.get_borrow_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.borrower;
                // fetch assets if user already a borrower
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    return;
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                        },
                    );
                }
            });

        self.get_repay_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.repayer;
                let reader = Reader::<M>::new(self.config.reader, self.client.clone());
                let factor = reader.get_liquidation_health_factor(user).await?;
                if (factor.user_total_debt_usd == 0) {
                    self.borrowers.remove(user.address);
                }
                return;
            });

        self.get_liquidation_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.account;
                self.borrowers.remove(user.address);
                return;
            });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block.as_u64(),
            borrowers: self.borrowers.clone(),
        };
        self.last_block_number = latest_block.as_u64();
        let mut file = File::create(STATE_CACHE_FILE)?;
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let event_emitter = Pool::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all repay events from the from_block to to_block
    async fn get_repay_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<RepayFilter>> {
        let event_emitter = Pool::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.repay_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all liquidation events from the from_block to to_block
    async fn get_liquidation_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<LiquidationFilter>> {
        let event_emitter = Pool::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.liquidation_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(ValueOrArray::Value(self.config.event_emitter))
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    async fn approve_tokens(&mut self) -> Result<()> {
        let mut nonce = self
            .client
            .get_transaction_count(
                self.client
                    .default_sender()
                    .ok_or(anyhow!("No connected sender"))?,
                None,
            )
            .await?;
        for pool in self.pools.keys() {
            let underlying_asset = IERC20::new(pool.underlying_asset.clone(), self.client.clone());
            let allowance = underlying_asset
                .allowance(self.liquidator, self.config.liquidation_handler)
                .call()
                .await?;
            if allowance == U256::zero() {
                underlying_asset
                    .approve(self.config.liquidation_handler, U256::MAX()
                    .nonce(nonce)
                    .send()
                    .await
                    .map_err(|e| {
                        error!("approve failed: {:?}", e);
                        e
                    })?;
                nonce = nonce + 1;
            }
        }

        Ok(())
    }

    async fn update_pools(&mut self) -> Result<()> {
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());
        let all_pools = reader.get_pools(self.config.data_store).await?;
        info!("all_pools: {:?}", all_pools);
        for pool in all_pools {
            self.pools.insert(
                pool.underlying_asset
                Pool {
                    underlying_asset: pool.underlying_asset,
                },
            );           
        }

        Ok(())
    }

    async fn get_best_liquidation_opportunity(&mut self) -> Result<Option<LiquidationOpportunity>> {
        let underwaters = self.get_underwater_borrowers().await?;

        if underwaters.len() == 0 {
            return Err(anyhow!("No underwater borrowers found"));
        }

        info!("Found {} underwaters", underwaters.len());
        let mut best_bonus: I256 = I256::MIN;
        let mut best_op: Option<LiquidationOpportunity> = None;

        for (borrower, health_factor, user_total_collateral_usd, user_total_debt_usd) in underwaters {
            if let Some(op) = self
                .get_liquidation_profit(
                    self.borrowers
                        .get(&borrower)
                        .ok_or(anyhow!("Borrower not found"))?,
                    &health_factor,
                    &user_total_collateral_usd,
                    &user_total_debt_usd,
                )
                .await
                .map_err(|e| info!("Liquidation op failed {}", e))
                .ok()
            {
                if op.profit > best_bonus {
                    best_bonus = op.profit;
                    best_op = Some(op);
                }
            }
        }

        Ok(best_op)
    }

    async fn get_liquidation_profit(
        &self,
        borrower: &Borrower,
        health_factor: &U256,
        health_factor: &user_total_collateral_usd,
        health_factor: &user_total_debt_usd,
    ) -> Result<LiquidationOpportunity> {
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());
        let eth_price = reader.get_price(self.config.data_store, self.config.weth).await?;

        let mut op = LiquidationOpportunity {
            borrower: borrower.address,
            profit: (user_total_collateral_usd - user_total_debt_usd)*10_U256.pow(WETH_DECIMALS)/eth_price ,
        };

        Ok(op)
    }

    async fn build_liquidation_tx(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let exchange_router = ExchangeRouter::new(self.config.exchange_router, self.client.clone());
        let mut call = exchange_router.execute_liquidation({account:op.borrower});
        Ok(call.tx.set_chain_id(self.chain_id).clone())
    }

}