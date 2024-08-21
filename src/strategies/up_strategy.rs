use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_up::{
    reader::Reader,
    //exchange_router::ExchangeRouter,
    event_emitter::{
        EventEmitter, 
        DepositFilter, 
        BorrowFilter, 
        RepayFilter, 
        RedeemFilter, 
        SwapFilter, 
        LiquidationFilter, 
        ClosePositionFilter, 
        CloseFilter
    },
    //shared_types::LiquidationParams,
    //ierc20::IERC20,
};
use bindings_liquidator::{
    liquidator::{Liquidator, LiquidationParams, Asset},
};
use clap::{Parser, ValueEnum};
use ethers::{
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, ValueOrArray, I256, U256, U64, U512},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::fs::File;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};
use chrono::{DateTime, Duration, Utc};

use super::types::{Action, Event};

#[derive(Debug)]
struct DeploymentConfig {
    data_store: Address,
    reader: Address,
    event_emitter: Address,
    exchange_router: Address,
    liquidation_handler: Address,
    eth: Address,
    usd: Address,
    last_block_number: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Deployment {
    LOCALNET
}

// admin stuff
pub const DEPLOYED_ADDRESSES: &str = "deployments/deployed_addresses.json";
pub const UNDERLYASSET_ADDRESSES: &str = "deployments/underlyAsset_addresses.json";
pub const STATE_CACHE_FILE: &str = "borrowers.json";
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const RETRY_DURATION_IN_SECS: i64 = 60;

fn get_deployment_config(deployment: Deployment, last_block_number: u64) -> DeploymentConfig {

    let file = File::open(DEPLOYED_ADDRESSES).unwrap();
    let up_contracts: HashMap<String, Address> = serde_json::from_reader(file).unwrap();
    let file = File::open(UNDERLYASSET_ADDRESSES).unwrap();
    let underly_assets: HashMap<String, UnderlyAsset> = serde_json::from_reader(file).unwrap();

    match deployment {
        Deployment::LOCALNET => DeploymentConfig {
            data_store: *up_contracts.get("DataStore#DataStore").unwrap(),
            reader: *up_contracts.get("Reader#Reader").unwrap(),
            event_emitter: *up_contracts.get("EventEmitter#EventEmitter").unwrap(),
            exchange_router: *up_contracts.get("ExchangeRouter#ExchangeRouter").unwrap(),
            liquidation_handler: *up_contracts.get("LiquidationHandler#LiquidationHandler").unwrap(),
            eth: underly_assets.get("WETH").unwrap().address,
            usd: underly_assets.get("USDT").unwrap().address,
            last_block_number: last_block_number,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnderlyAsset {
    #[serde(rename = "address")]
    address: Address,

    #[serde(rename = "decimals")]
    decimals: u64,

    #[serde(rename = "oracle")]
    oracle: Address,

    #[serde(rename = "oracleDecimals")]
    oracle_decimals: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    pools: HashMap<Address, Pool>,
    sents: HashMap<Address, DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pool: Address,
    collateral: U256,
    debt_scaled: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    positions: HashMap<Address, Position>,
    //health: u64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    underlying_asset: Address,
    symbol: String,
    price: U256,
    decimals: U256,
    borrow_index: U256,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UpStrategy<M> {
    client: Arc<M>,
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    pools: HashMap<Address, Pool>,
    sents: HashMap<Address, DateTime<Utc>>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    liquidation_threshold: U256
}

impl<M: Middleware + 'static> UpStrategy<M> {
    pub fn new(
        client: Arc<M>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
        last_block_number: u64,
    ) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: last_block_number,
            borrowers: HashMap::new(),
            pools: HashMap::new(),
            sents: HashMap::new(),
            chain_id: config.chain_id,
            config: get_deployment_config(deployment, last_block_number),
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
            liquidation_threshold: U256::zero()
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Asset {
//     token: Address,
//     amount: U256,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LiquidationOpportunity {
    borrower: Address,
    profit: I256,
    collaterals: Vec<Asset>,
    debts: Vec<Asset>,
    uniswap_fee: u32,
    gas_fee_usd: U256,
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UpStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");
        info!("self.config.data_store {:?}", self.config.data_store);
        info!("self.config.reader {:?}", self.config.reader);
        info!("self.config.event_emitter {:?}", self.config.event_emitter);
        info!("self.config.exchange_router {:?}", self.config.exchange_router);
        info!("self.config.liquidation_handler {:?}", self.config.liquidation_handler);
        info!("self.config.eth {:?}", self.config.eth);
        info!("self.config.usd {:?}", self.config.usd);
                
        self.update_pools().await?;
        //self.approve_tokens().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewTick(block) => self
                .process_new_tick_event(block)
                .await
                .map_or(vec![], |a| vec![a]),
        }
    }
}

impl<M: Middleware + 'static> UpStrategy<M> {

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);

        self.update_pools()
            .await
            .map_err(|e| error!("Update Pools error: {}", e))
            .ok()?;

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

        if op.profit <= I256::from(0) {
            info!("No profitable ops, passing");
            return None;
        }

        //prevent resend tx
        // let now: DateTime<Utc> = Utc::now();
        // match self.sents.get(&op.borrower) {
        //     Some(&sent_time) => {
        //         let duration: Duration = now - sent_time;
        //         if duration.num_seconds() < RETRY_DURATION_IN_SECOND {
        //             return None;
        //         }
        //         self.sents.insert(op.borrower, now);
        //     },
        //     None => {
        //         self.sents.insert(op.borrower, now);
        //     }
        // }

        let now: DateTime<Utc> = Utc::now();
        self.sents.insert(op.borrower, now);

        //sent tx
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

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
                self.pools = cache.pools;
                self.sents = cache.sents;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = self.config.last_block_number;
            }
        };

        Ok(())
    }


    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        let mut start_block = self.last_block_number;
        if start_block > latest_block.as_u64() {
            start_block = latest_block.as_u64();
        }
        info!(
            "Updating state from block {} to {}",
            start_block, latest_block
        );

        self.update_liquidation_threshold().await?;

    
        self.get_deposit_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("deposit {:?} {} {} {}", log.depositer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.depositer;      
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.positions.insert(
                        log.pool, 
                        Position {
                            pool:log.pool,
                            collateral:log.collateral,
                            debt_scaled:log.debt_scaled,
                        }
                    );
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            positions: HashMap::from([(
                                log.pool, 
                                Position {
                                    pool:log.pool,
                                    collateral:log.collateral,
                                    debt_scaled:log.debt_scaled,                               
                                }
                            )])
                        },
                    );
                }
                return;
            });

        self.get_borrow_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("borrow {:?} {} {} {}", log.borrower, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.borrower;
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.positions.insert(
                        log.pool, 
                        Position {
                            pool:log.pool,
                            collateral:log.collateral,
                            debt_scaled:log.debt_scaled,
                        }
                    );
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            positions: HashMap::from([(
                                log.pool, 
                                Position {
                                    pool:log.pool,
                                    collateral:log.collateral,
                                    debt_scaled:log.debt_scaled
                                }
                            )])
                        },
                    );
                }
                return;
            });

        self.get_repay_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("repay {:?} {} {} {}", log.repayer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled);   
                let user = log.repayer;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool, 
                    Position {
                        pool:log.pool,
                        collateral:log.collateral,
                        debt_scaled:log.debt_scaled
                    }
                ); 
                if log.collateral == U256::from(0) && log.debt_scaled == U256::from(0)  {
                    self.borrowers.remove(&user);
                }          
                return;
            });

        self.get_redeem_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("redeem {:?} {} {} {}", log.redeemer, self.pools.get(&log.pool).unwrap().symbol, log.collateral, log.debt_scaled); 
                let user = log.redeemer;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool,
                    Position {
                        pool:log.pool,
                        collateral:log.collateral,
                        debt_scaled:log.debt_scaled,
                    }
                );
                if log.collateral == U256::from(0)  && log.debt_scaled == U256::from(0)  {
                    self.borrowers.remove(&user);
                }          
                return;
            });

        self.get_swap_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("swapIn {:?} {} {} {}", log.account, self.pools.get(&log.pool_in).unwrap().symbol, log.collateral_in, log.debt_scaled_in); 
                info!("swapOut {:?} {} {} {}", log.account, self.pools.get(&log.pool_out).unwrap().symbol, log.collateral_out, log.debt_scaled_out); 
                let user = log.account;
                //info!("swap {}", log);
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.insert(
                    log.pool_in, 
                    Position {
                        pool:log.pool_in,
                        collateral:log.collateral_in,
                        debt_scaled:log.debt_scaled_in
                    }
                );
                borrower.positions.insert(
                    log.pool_out,
                    Position {
                        pool:log.pool_out,
                        collateral:log.collateral_out,
                        debt_scaled:log.debt_scaled_out
                    }
                );
                return;
            });

        self.get_liquidation_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("liquidation {:?}", log.account);
                let user = log.account;
                self.borrowers.remove(&user);
                return;
            });

        self.get_close_position_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("close_position {:?} {} {} {} {} ", log.account, self.pools.get(&log.pool).unwrap().symbol, self.pools.get(&log.pool_usd).unwrap().symbol, log.collateral_usd, log.debt_scaled_usd); 
                let user = log.account;
                let borrower = self.borrowers.get_mut(&user).unwrap();
                borrower.positions.remove(&log.pool);
                borrower.positions.insert(
                    log.pool_usd, 
                    Position {
                        pool:log.pool_usd,
                        collateral:log.collateral_usd,
                        debt_scaled:log.debt_scaled_usd,
                    }
                );
                return;
            });

        self.get_close_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("close {}", log.account);
                let user = log.account;
                self.borrowers.remove(&user);
                return;
            });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block.as_u64(),
            borrowers: self.borrowers.clone(),
            pools: self.pools.clone(),
            sents: self.sents.clone(),
        };
        self.last_block_number = latest_block.as_u64();
        let file = File::create(STATE_CACHE_FILE)?;
        //file.write_all(serde_json::to_string(&cache)?.as_bytes())?;
        serde_json::to_writer_pretty(file, &cache)?;

        Ok(())
    }

    // fetch all deposit events from the from_block to to_block
    async fn get_deposit_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<DepositFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());
        //info!("event_emitter {:?}", event_emitter );
        //info!("client {:?}", self.client );
        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            //info!("get_deposit_logs start_block {} end_block {}", start_block, end_block );
            event_emitter.deposit_filter()  
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

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

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
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

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

    // fetch all redeem events from the from_block to to_block
    async fn get_redeem_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<RedeemFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.redeem_filter()
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

    // fetch all swap events from the from_block to to_block
    async fn get_swap_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SwapFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.swap_filter()
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
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

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

    // fetch all close position events from the from_block to to_block
    async fn get_close_position_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<ClosePositionFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.close_position_filter()
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

    // fetch all close events from the from_block to to_block
    async fn get_close_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<CloseFilter>> {
        let event_emitter = EventEmitter::<M>::new(self.config.event_emitter, self.client.clone());

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            event_emitter.close_filter()
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

    async fn update_pools(&mut self) -> Result<()> {
        //info!("self.config.reader {:?}", self.config.reader);
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());
        let all_pools = reader.get_pools_price(self.config.data_store).await?;
        for pool in all_pools {
            info!("pool {:?} {} {} ", pool.underlying_asset, pool.symbol, pool.price);
            self.pools.insert(
                pool.underlying_asset,
                Pool {
                    underlying_asset: pool.underlying_asset,
                    symbol: pool.symbol,
                    price: pool.price,
                    decimals: pool.decimals,
                    borrow_index: pool.borrow_index,
                },
            );           
        }

        Ok(())
    }

    async fn update_liquidation_threshold(&mut self) -> Result<()> {
        let reader = Reader::<M>::new(self.config.reader, self.client.clone());
        self.liquidation_threshold = reader.get_liquidation_health_factor(self.config.data_store).await?;
        info!("liquidation_threshold {:?}", self.liquidation_threshold);
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

        for (borrower, _health_factor, user_total_collateral_usd, user_total_debt_usd) in underwaters {
            if let Some(op) = self
                .get_liquidation_profit(
                    self.borrowers
                        .get(&borrower)
                        .ok_or(anyhow!("Borrower not found"))?,
                    //&health_factor,
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

    // for all known borrowers, return a sorted set of those with health factor < liquidation_threshold
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256, U256, U256)>> {
        let mut underwater_borrowers = Vec::new();

        let borrowers: Vec<&Borrower> = self
            .borrowers
            .values()
            .filter(|b| b.positions.len() > 0)
            .collect();

        for chunk in borrowers.chunks(MULTICALL_CHUNK_SIZE) {
            let result: Vec<(U256, U256, U256)> = chunk.iter().map(|borrower| {
                let mut user_total_collateral_usd = U256::from(0);
                let mut user_total_debt_usd = U256::from(0);
                for (_, position) in &borrower.positions {
                    let price = self.pools.get(&position.pool).unwrap().price;
                    let decimals = self.pools.get(&position.pool).unwrap().decimals;
                    let borrow_index = self.pools.get(&position.pool).unwrap().borrow_index;
                    let debt = ray_mul(position.debt_scaled, borrow_index);
                    user_total_collateral_usd += ray_mul(price, adjust_precision(position.collateral, decimals));
                    user_total_debt_usd += ray_mul(price, adjust_precision(debt, decimals));
                }               

                let health_factor = if user_total_debt_usd == U256::zero() {
                    U256::max_value()
                } else {
                    ray_div(user_total_collateral_usd, user_total_debt_usd) 
                };

                (health_factor, user_total_collateral_usd, user_total_debt_usd) 
            }).collect();

            for (borrower, (health_factor, user_total_collateral_usd, user_total_debt_usd)) in zip(chunk, result) {
                //info!("account {:?} {} {}", borrower.address, health_factor, self.liquidation_threshold);
                if health_factor < self.liquidation_threshold {
                    //prevent resend tx
                    let now: DateTime<Utc> = Utc::now();
                    match self.sents.get(&borrower.address) {
                        Some(&sent_time) => {
                            let duration: Duration = now - sent_time;
                            if duration.num_seconds() < RETRY_DURATION_IN_SECS {
                                continue;
                            }
                        },
                        None => {}
                    }

                    info!(
                        "Found underwater borrower {:?} -  healthFactor: {} - user_total_collateral_usd: {} - user_total_debt_usd: {}",
                        borrower.address, health_factor, user_total_collateral_usd, user_total_debt_usd
                    );
                    underwater_borrowers.push((borrower.address, health_factor, user_total_collateral_usd, user_total_debt_usd));
                }
            }
        }

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    async fn get_liquidation_profit(
        &self,
        borrower: &Borrower,
//        health_factor: &U256,
        user_total_collateral_usd: &U256,
        user_total_debt_usd: &U256,
    ) -> Result<LiquidationOpportunity> {
        let mut profit: U256 = U256::zero();
        if user_total_collateral_usd > user_total_debt_usd {
            profit = user_total_collateral_usd - user_total_debt_usd;
        }

        let eth_price = self.pools.get(&self.config.eth).unwrap().price;
        let mut op = LiquidationOpportunity {
            borrower: borrower.address,
            profit: I256::from_dec_str(&ray_div( profit, eth_price).to_string())?,
            collaterals: Vec::new(),
            debts: Vec::new(),
            uniswap_fee: 3000,
            gas_fee_usd: U256::zero() 
        };
        for (_, position) in &borrower.positions {
            if position.collateral > U256::zero() {
                op.collaterals.push(Asset{
                    token: position.pool.clone(), 
                    amount: position.collateral, 
                });
            }
            if position.debt_scaled > U256::zero() {
                op.debts.push(Asset{
                    token: position.pool.clone(), 
                    amount: position.debt_scaled, 
                });
            }
        }
        info!(
            "op {:?} ", op
        );


        Ok(op)
    }

    async fn build_liquidation_tx(&self, op: &LiquidationOpportunity) -> Result<TypedTransaction> {
        let liquidator = Liquidator::new(self.liquidator, self.client.clone());
        let mut call = liquidator.liquidate(LiquidationParams{
            account: op.borrower,
            usd_token: self.config.usd,
            collaterals: op.collaterals.clone(),
            debts: op.debts.clone(),
            uniswap_fee: 3000,
            gas_fee_usd: U256::zero()

        });
        Ok(call.tx.set_chain_id(self.chain_id).clone())
    }

}
// TODO: update to constant
// static PRECISION: U256 = U256::from(10).pow(U256::from(27));
// static HALF_PRECISION: U256 = U256::from(5)*U256::from(10).pow(U256::from(26));

fn ray_mul(a: U256, b: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let half_precision: U512 = U512::from(5)*U512::from(10).pow(U512::from(26));
    let a512 : U512 = U512::from(a);
    let b512 : U512 = U512::from(b);
    return U256::from_dec_str(&((a512*b512 + half_precision)/precision).to_string()).unwrap(); 
}

fn ray_div(a: U256, b: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let a512 : U512 = U512::from(a);
    let b512 : U512 = U512::from(b);
    let two : U512 = U512::from(2);

    return U256::from_dec_str(&((a512*precision+ b512/two)/b512).to_string()).unwrap();
}

fn adjust_precision(a: U256, decimals: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let a512 : U512 = U512::from(a);
    return U256::from_dec_str(&(a512*precision/U512::from(10).pow(U512::from(decimals))).to_string()).unwrap();
}