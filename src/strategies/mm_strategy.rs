use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{anyhow, Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_mm::{
    reader::Reader,
    eventemitter::EventEmitter,
    //shared_types::LiquidationParams,
    //ierc20::IERC20,
};
use std::future::IntoFuture;
// use bindings_liquidator::{
//     liquidator::{Liquidator, LiquidationParams, Asset},
// };
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, ValueEnum};
use super::types::{Action, Event};
//use alloy_primitives::{Address, Uint, String};
use sha3::{Digest, Keccak256};
//use alloy::contract as alloy_contract;

use alloy::{
    contract as alloy_contract,
    network::{EthereumWallet,TransactionRequest},
    //signers::local::PrivateKeySigner,
    providers::{ProviderBuilder}, 
    sol_types::private::{Address, Uint},
    primitives::{FixedBytes, U256, U512},
    consensus::{TypedTransaction},

};

//type Bytes32 = [u8; 32];
type Bytes32 = FixedBytes<32>;
// type U256 = Uint<256, 4>;
// type U256 = alloy_primitives::Uint<256, 4>;

// use std::marker::PhantomData;

#[derive(Debug)]
struct DeploymentConfig {
    data_store: Address,
    reader: Address,
    event_emitter: Address,
    last_block_number: u64,
}

#[derive(Debug, Clone, Parser, ValueEnum )]
pub enum Deployment {
    LOCALNET
}

// admin stuff
pub const DEPLOYED_ADDRESSES: &str = "deployments/deployed_addresses.json";
pub const STATE_CACHE_FILE: &str = "borrowers.json";
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 100;
pub const RETRY_DURATION_IN_SECS: i64 = 60;

fn get_deployment_config(deployment: Deployment, last_block_number: u64) -> DeploymentConfig {

    let file = File::open(DEPLOYED_ADDRESSES).unwrap();
    let up_contracts: HashMap<String, Address> = serde_json::from_reader(file).unwrap();

    match deployment {
        Deployment::LOCALNET => DeploymentConfig {
            data_store: *up_contracts.get("DataStore#DataStore").unwrap(),
            reader: *up_contracts.get("Reader#Reader").unwrap(),
            event_emitter: *up_contracts.get("EventEmitter#EventEmitter").unwrap(),
            last_block_number: last_block_number,
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Token {
//     #[serde(rename = "address")]
//     address: Address,

//     #[serde(rename = "decimals")]
//     decimals: u64,

//     #[serde(rename = "oracle")]
//     oracle: Address,

//     #[serde(rename = "oracleDecimals")]
//     oracle_decimals: u64,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    pools: HashMap<Bytes32, Pool>,
    positions: HashMap<Bytes32, Position>,
    //sents: HashMap<Address, DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    account: Address,
    position_id: U256,
    pool: Bytes32,
    base_collateral: U256,
    base_debt_scaled: U256,
    meme_collateral: U256,
    meme_debt_scaled: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    price: U256,
    price_decimals: U256,
    base_token: Address,
    base_symbol: String,
    base_token_decimals: U256,
    base_borrow_index: U256,
    meme_token: Address,
    meme_symbol: String,
    meme_token_decimals: U256,
    meme_borrow_index: U256,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MmStrategy<T, P, N = alloy_contract::private::Ethereum>
{
    client: Arc<P>,
    last_block_number: u64,
    pools: HashMap<Bytes32, Pool>,
    positions: HashMap<Bytes32, Position>,
    //sents: HashMap<Address, DateTime<Utc>>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    liquidation_threshold: U256,
     _network_transport: ::core::marker::PhantomData<(N, T)>,
    //reader: Reader::ReaderInstance<T, P, N>,
    //event_emitter: EventEmitter::EventEmitterInstance<T, P, N>,
}

impl<   
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N> + 'static,
    N: alloy_contract::private::Network,
> MmStrategy<T, P, N> {
    pub fn new(
        client: Arc<P>,
        config: Config,
        deployment: Deployment,
        liquidator_address: String,
        last_block_number: u64,
    ) -> Self {
        let deployment_config = get_deployment_config(deployment, last_block_number);
        Self {
            client,
            last_block_number: last_block_number,
            positions: HashMap::new(),
            pools: HashMap::new(),
            //sents: HashMap::new(),
            chain_id: config.chain_id,
            config: deployment_config,
            liquidator: Address::from_str(&liquidator_address).expect("invalid liquidator address"),
            liquidation_threshold: U256::ZERO,
            _network_transport: ::core::marker::PhantomData,
            //reader:Reader::new(deployment_config.reader.clone(), client.clone()),
            //event_emitter:EventEmitter::new(deployment_config.event_emitter, client.clone()),
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Asset {
//     token: Address,
//     amount: U256,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// struct LiquidationOpportunity {
//     borrower: Address,
//     profit: I256,
//     collaterals: Vec<Asset>,
//     debts: Vec<Asset>,
//     uniswap_fee: u32,
//     gas_fee_usd: U256,
// }

#[async_trait]
impl<   
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>  + 'static,
    N: alloy_contract::private::Network,
> Strategy<Event, Action> for MmStrategy<T, P, N>{
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");
        info!("self.config.data_store {:?}", self.config.data_store);
        info!("self.config.reader {:?}", self.config.reader);
        info!("self.config.event_emitter {:?}", self.config.event_emitter);
                
        self.update_pools().await?;
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
                .unwrap_or_else(Vec::new),
        }
    }
}

impl<   
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>  + 'static,
    N: alloy_contract::private::Network,
> MmStrategy<T, P, N> {

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Vec<Action>> {
        info!("received new tick: {:?}", event);

        self.update_pools()
            .await
            .map_err(|e| error!("Update Pools error: {}", e))
            .ok()?;

        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        info!("Total position count: {}", self.positions.len());
        let underwaters = self.get_underwater_positions().await;

        let mut actions: Vec<Action> = Vec::new();
        for (account, position_id, margin_level, collateral, debt) in underwaters {
            let action = async {
                self.build_liquidation_tx(&account, position_id)
                    .await
                    .map_err(|e| {
                        error!("Error building liquidation: {}", e);
                        None
                    })
                    .ok()
                    .map(|tx| SubmitTxToMempool {
                        tx,
                        gas_bid_info: Some(GasBidInfo{total_profit:U256::ZERO, bid_percentage:0}),
                    })
            }
            .await;

            if let Some(action) = action {
                actions.push(action);
            }
        }

        Some(actions)
    }

    async fn build_liquidation_tx(&self, account: &Address, position_id: U256) -> Result<TransactionRequest> {
        // let liquidator = Liquidator::new(self.liquidator, self.client.clone());
        // let mut call = liquidator.liquidate(LiquidationParams{
        //     account: account,
        //     position_id: position_id
        // });
        // Ok(call.tx.set_chain_id(self.chain_id).clone())
        None
    }

    // for all known borrowers, return a sorted set of those with health factor < liquidation_threshold
    async fn get_underwater_positions(&mut self) -> Option<Vec<(Address, U256, U256, U256, U256)>> {
        let mut underwater_positions = Vec::new();

        let positions: Vec<&Position> = self
            .positions
            .values()
            .filter(|p| p.base_debt_scaled > U256::ZERO || p.meme_debt_scaled > U256::ZERO)
            .collect();

        for chunk in positions.chunks(MULTICALL_CHUNK_SIZE) {
            let result: Vec<(U256, U256, U256)> = chunk.iter().map(|position| {
                let mut user_total_collateral_usd = U256::ZERO;
                let mut user_total_debt_usd = U256::ZERO;

                let pool = self.pools.get(&position.pool).unwrap();

                let price = pool.price;
                let price_decimals = pool.price_decimals;

                //meme
                let base_borrow_index = pool.base_borrow_index;
                let base_debt = ray_mul(position.base_debt_scaled, base_borrow_index);
                let base_decimals = pool.base_token_decimals;
                user_total_collateral_usd += adjust_precision(position.base_collateral, base_decimals);
                user_total_debt_usd += adjust_precision(base_debt, base_decimals); 

                let meme_borrow_index = pool.meme_borrow_index;
                let meme_debt = ray_mul(position.meme_debt_scaled, meme_borrow_index);
                let meme_decimals = pool.meme_token_decimals;
                user_total_collateral_usd += ray_mul(price, adjust_precision(position.meme_collateral, meme_decimals));
                user_total_debt_usd += ray_mul(price, adjust_precision(meme_debt, meme_decimals)); 


                let margin_level = if user_total_debt_usd == U256::ZERO {
                    U256::MAX
                } else {
                    ray_div(user_total_collateral_usd, user_total_debt_usd) 
                };

                (margin_level, user_total_collateral_usd, user_total_debt_usd) 
            }).collect();

            for (position, (margin_level, user_total_collateral_usd, user_total_debt_usd)) in zip(chunk, result) {
                //info!("account {:?} {} {}", borrower.address, margin_level, self.liquidation_threshold);
                if margin_level < self.liquidation_threshold {
                    // //prevent resend tx
                    // let now: DateTime<Utc> = Utc::now();
                    // match self.sents.get(&borrower.address) {
                    //     Some(&sent_time) => {
                    //         let duration: Duration = now - sent_time;
                    //         if duration.num_seconds() < RETRY_DURATION_IN_SECS {
                    //             continue;
                    //         }
                    //     },
                    //     None => {}
                    // }

                    // info!(
                    //     "Found underwater borrower {:?} -  healthFactor: {} - user_total_collateral_usd: {} - user_total_debt_usd: {}",
                    //     borrower.address, margin_level, user_total_collateral_usd, user_total_debt_usd
                    // );
                    underwater_positions.push((position.account, position.position_id, margin_level, user_total_collateral_usd, user_total_debt_usd));
                }
            }
        }

        // sort positions by health factor
        //underwater_positions.sort_by(|a, b| a.1.cmp(&b.1));
        Some(underwater_positions)
    }

//     async fn get_liquidation_profit(
//         &self,
//         borrower: &Borrower,
// //        margin_level: &U256,
//         user_total_collateral_usd: &U256,
//         user_total_debt_usd: &U256,
//     ) -> Result<LiquidationOpportunity> {
//         let mut profit: U256 = U256::ZERO;
//         if user_total_collateral_usd > user_total_debt_usd {
//             profit = user_total_collateral_usd - user_total_debt_usd;
//         }

//         let eth_price = self.pools.get(&self.config.eth).unwrap().price;
//         let mut op = LiquidationOpportunity {
//             borrower: borrower.address,
//             profit: I256::from_dec_str(&ray_div( profit, eth_price).to_string())?,
//             collaterals: Vec::new(),
//             debts: Vec::new(),
//             uniswap_fee: 3000,
//             gas_fee_usd: U256::ZERO 
//         };
//         for (_, position) in &borrower.positions {
//             if position.collateral > U256::ZERO {
//                 op.collaterals.push(Asset{
//                     token: position.pool.clone(), 
//                     amount: position.collateral, 
//                 });
//             }
//             if position.debt_scaled > U256::ZERO {
//                 op.debts.push(Asset{
//                     token: position.pool.clone(), 
//                     amount: position.debt_scaled, 
//                 });
//             }
//         }
//         info!(
//             "op {:?} ", op
//         );


//         Ok(op)
//     }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.positions = cache.positions;
                self.pools = cache.pools;
                //self.sents = cache.sents;
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
        if start_block > latest_block {
            start_block = latest_block;
        }
        info!(
            "Updating state from block {} to {}",
            start_block, latest_block
        );

        //self.update_liquidation_threshold().await?;
        self.get_deposit_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("deposit {:?} {} {} {} {} {}", log.depositor, self.pools.get(&hash_pool_key(log.baseToken, log.memeToken)).unwrap().meme_symbol, log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled);  
                let user = log.depositor;      
                self.update_position(
                    hash_position_key(user, log.positionId),
                    user, 
                    log.positionId,
                    hash_pool_key(log.baseToken, log.memeToken), 
                    log.baseCollateral, 
                    log.baseDebtScaled, 
                    log.memeCollateral, 
                    log.memeDebtScaled
                );
                return;
            });

        self.get_borrow_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                info!("borrow {:?} {} {} {} {} {}", log.borrower, self.pools.get(&hash_pool_key(log.baseToken, log.memeToken)).unwrap().meme_symbol, log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled);   
                let user = log.borrower;
                self.update_position(
                    hash_position_key(user, log.positionId),
                    user, 
                    log.positionId,
                    hash_pool_key(log.baseToken, log.memeToken), 
                    log.baseCollateral, 
                    log.baseDebtScaled, 
                    log.memeCollateral, 
                    log.memeDebtScaled
                );
                return;
            });

        // self.get_repay_logs(start_block.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         info!("repay {:?} {} {} {} {} {}", log.repayer, self.pools.get(hash_pool_key(log.baseToken, log.memeToken)).unwrap().symbol, log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled);   
        //         let user = log.repayer;
        //         self.update_position(
        //             user, 
        //             hash_pool_key(log.baseToken, log.memeToken), 
        //             log.baseCollateral, 
        //             log.baseDebtScaled, 
        //             log.memeCollateral, 
        //             log.memeDebtScaled
        //         );          
        //         return;
        //     });

        // self.get_redeem_logs(start_block.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         info!("redeem {:?} {} {} {} {} {}", log.redeemer, self.pools.get(&hash_pool_key(log.baseToken, log.memeToken)).unwrap().symbol, log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled); 
        //         let user = log.redeemer;
        //         self.update_position(user, hash_pool_key(log.baseToken, log.memeToken), log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled);        
        //         return;
        //     });

        // self.get_swap_logs(start_block.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         info!("swapIn {:?} {} {} {} {} {}", log.account, self.pools.get(&hash_pool_key(log.tokenIn, log.tokenOut)).unwrap().symbol, log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled); 
        //         let user = log.account;
        //         self.update_position(
        //             user, 
        //             hash_pool_key(log.tokenIn, log.tokenOut), 
        //             log.baseCollateral, 
        //             log.baseDebtScaled, 
        //             log.memeCollateral, 
        //             log.memeDebtScaled
        //         );
        //         return;
        //     });

        // self.get_liquidation_logs(start_block.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         info!("liquidation {:?}", log.account);
        //         let user = log.account;
        //         self.borrowers.remove(&user);
        //         return;
        //     });
        // self.get_close_logs(start_block.into(), latest_block)
        //     .await?
        //     .into_iter()
        //     .for_each(|log| {
        //         info!("close {:?}", log.account);
        //         let user = log.account;
        //         self.borrowers.remove(&user);
        //         return;
        //     });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block,
            pools: self.pools.clone(),
            positions: self.positions.clone(),
            //sents: self.sents.clone(),
        };
        self.last_block_number = latest_block;
        let file = File::create(STATE_CACHE_FILE)?;
        serde_json::to_writer_pretty(file, &cache)?;

        Ok(())
    }

    // fetch all deposit events from the from_block to to_block
    async fn get_deposit_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Deposit>> {
        let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());
        let mut res = Vec::new();
        for start_block in
            (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
            event_emitter.Deposit_filter()  
                .from_block(start_block)
                .to_block(end_block)
                .address(self.config.event_emitter)
                .query()
                .await?
                .into_iter()
                .for_each(|(deposit,_)| {
                    res.push(deposit);
                });
        }

        Ok(res)
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Borrow>> {
        let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());
        let mut res = Vec::new();
        for start_block in
            (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
            event_emitter.Borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .address(self.config.event_emitter)
                .query()
                .await?
                .into_iter()
                .for_each(|(borrow,_)| {
                    res.push(borrow);
                });
        }

        Ok(res)
    }

    // // fetch all repay events from the from_block to to_block
    // async fn get_repay_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Repay>> {
    //     let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
    //         event_emitter.Repay_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.event_emitter))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    // // fetch all redeem events from the from_block to to_block
    // async fn get_redeem_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Redeem>> {
    //     let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
    //         event_emitter.redeem_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.event_emitter))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    // // fetch all swap events from the from_block to to_block
    // async fn get_swap_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Swap>> {
    //     let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
    //         event_emitter.Swap_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.event_emitter))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    // // fetch all liquidation events from the from_block to to_block
    // async fn get_liquidation_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Liquidation>> {
    //     let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
    //         event_emitter.liquidation_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.event_emitter))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    // // fetch all close events from the from_block to to_block
    // async fn get_close_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Close>> {
    //     let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());

    //     let mut res = Vec::new();
    //     for start_block in
    //         (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
    //     {
    //         let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
    //         event_emitter.close_filter()
    //             .from_block(start_block)
    //             .to_block(end_block)
    //             .address(ValueOrArray::Value(self.config.event_emitter))
    //             .query()
    //             .await?
    //             .into_iter()
    //             .for_each(|log| {
    //                 res.push(log);
    //             });
    //     }

    //     Ok(res)
    // }

    async fn update_pools(&mut self) -> Result<()> {
        // info!("self.config.reader {:?}", self.config.reader);
        // let all_pools = self.reader.getPools_0(self.config.data_store.clone()).await?;

        let reader = Reader::new(self.config.reader.clone(), self.client.clone());
        let all_pools = reader.getPools_0(self.config.data_store.clone()).call().await.unwrap();

        for pool in all_pools._0.iter() {
            info!("pool {:?} {} {} ", pool.assets[0].token, pool.assets[1].symbol, pool.price);
            self.pools.insert(
                hash_pool_key(pool.assets[0].token, pool.assets[1].token),
                Pool {
                    price: pool.price,
                    price_decimals: pool.priceDecimals,
                    base_token: pool.assets[0].token,
                    base_symbol: pool.assets[0].symbol.clone(),
                    base_token_decimals: pool.assets[0].decimals,
                    base_borrow_index: pool.assets[0].borrowIndex,
                    meme_token: pool.assets[1].token,
                    meme_symbol: pool.assets[1].symbol.clone(),
                    meme_token_decimals: pool.assets[1].decimals,
                    meme_borrow_index: pool.assets[1].borrowIndex,
                },
            );           
        }

        Ok(())
    }

    // async fn update_liquidation_threshold(&mut self) -> Result<()> {
    //     let reader = Reader::new(self.config.reader, self.client.clone());
    //     self.liquidation_threshold = reader.get_liquidation_margin_level(self.config.data_store).await?;
    //     info!("liquidation_threshold {:?}", self.liquidation_threshold);
    //     Ok(())
    // }

    fn update_position(
        &mut self, 
        positionKey: Bytes32,
        account: Address, 
        position_id: U256,
        pool: Bytes32, 
        base_collateral: U256, 
        base_debt_scaled: U256, 
        meme_collateral: U256, 
        meme_debt_scaled: U256
    ) {
        //remove position and remove borrower
        if base_collateral == U256::ZERO && meme_collateral == U256::ZERO && 
           base_debt_scaled == U256::ZERO && meme_debt_scaled == U256::ZERO {
            if self.positions.contains_key(&positionKey) {
                //let position = self.positions.get_mut(&positionKey).unwrap();
                self.positions.remove(&positionKey);
            }
            return
        }

        //insert position
        self.positions.insert(
            positionKey, 
            Position {
                account:account,
                position_id:position_id,
                pool:pool,
                base_collateral:base_collateral,
                base_debt_scaled:base_debt_scaled,
                meme_collateral:meme_collateral,
                meme_debt_scaled:meme_debt_scaled,
            }
        );

    }

}

fn hash_pool_key(addr1: Address, addr2: Address) -> Bytes32 {
    // Determine the order: the smaller address comes first
    let (first, second) = if addr1 < addr2 {
        (addr1, addr2)
    } else {
        (addr2, addr1)
    };

    let bytes1: [u8; 20] = first.into();  // Convert Address to a 20-byte array
    let bytes2: [u8; 20] = second.into();  // Convert Address to a 20-byte array

    // Create a Keccak256 hasher
    let mut hasher = Keccak256::new();

    // Concatenate the addresses in the determined order
    hasher.update(&bytes1);
    hasher.update(&bytes2);

    // Finalize the hash and convert to Bytes32
    let hash_result = hasher.finalize();

    // Convert the result to Bytes32
    FixedBytes::try_from(hash_result.as_slice()).expect("Hash should be 32 bytes")

}

fn hash_position_key(account: Address, position_id: U256) -> Bytes32 {

    let bytes1: [u8; 20] = account.into();  // Convert Address to a 20-byte array
    let bytes2: [u8; 32] = position_id.to_be_bytes();  // Convert Address to a 20-byte array

    // Create a Keccak256 hasher
    let mut hasher = Keccak256::new();

    // Concatenate the addresses in the determined order
    hasher.update(&bytes1);
    hasher.update(&bytes2);

    // Finalize the hash and convert to Bytes32
    let hash_result = hasher.finalize();

    // Convert the result to Bytes32
    FixedBytes::try_from(hash_result.as_slice()).expect("Hash should be 32 bytes")

}

// fn convert_alloy_to_ethers(alloy_uint: Uint<256, 4>) -> U256 {
//     // Convert the alloy Uint<256, 4> into a byte array (assuming little-endian format)
//     let bytes = alloy_uint.to_bytes();  // You may need to adjust this based on the actual API
//     let mut byte_array = [0u8; 32];  // U256 is 32 bytes
    
//     // Copy the relevant bytes into the byte_array (assumes that `bytes` is 32 bytes or less)
//     byte_array[0..bytes.len()].copy_from_slice(&bytes);
    
//     // Convert the byte array to a U256
//     U256::from_little_endian(&byte_array)
// }

fn ray_mul(a: U256, b: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let half_precision: U512 = U512::from(5)*U512::from(10).pow(U512::from(26));
    let a512 : U512 = U512::from(a);
    let b512 : U512 = U512::from(b);
    return U256::from_str(&((a512*b512 + half_precision)/precision).to_string()).unwrap(); 
}

fn ray_div(a: U256, b: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let a512 : U512 = U512::from(a);
    let b512 : U512 = U512::from(b);
    let two : U512 = U512::from(2);

    return U256::from_str(&((a512*precision+ b512/two)/b512).to_string()).unwrap();
}

fn adjust_precision(a: U256, decimals: U256) -> U256 {
    let precision: U512 = U512::from(10).pow(U512::from(27));
    let a512 : U512 = U512::from(a);
    return U256::from_str(&(a512*precision/U512::from(10).pow(U512::from(decimals))).to_string()).unwrap();
}

