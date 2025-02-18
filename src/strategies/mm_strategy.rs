use super::types::Config;
use crate::collectors::time_collector::NewTick;
use anyhow::{Result};
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_mm::{
    reader::Reader,
    eventemitter::{EventEmitter},
    exchangerouter::LiquidationUtils::LiquidationParams,
    exchangerouter::ExchangeRouter,
    //shared_types::LiquidationParams,
    //ierc20::IERC20,
};
// use std::future::IntoFuture;
// use bindings_liquidator::{
//     liquidator::{Liquidator, LiquidationParams, Asset},
// };
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::error::Error; 
use std::fs::File;
use std::iter::zip;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Instant};
use tracing::{error, info};
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, ValueEnum};
use super::types::{Action, Event};
use sha3::{Digest, Keccak256};

use tracing::warn;

use alloy::{
    contract as alloy_contract,
    network::{ Network, TransactionBuilder},
    sol_types::private::{Address},
    primitives::{FixedBytes, U256, U512},
};

use alloy::sol_types::SolValue;

type Bytes32 = FixedBytes<32>;

#[derive(Debug)]
struct DeploymentConfig {
    data_store: Address,
    reader: Address,
    event_emitter: Address,
    exchange_router: Address,
    last_block_number: u64,
    total_profit: u128,
    update_all_pools_ticks: u64,
    activity_level_decrease_ticks: u64,
    activity_level_init: u64,
    calc_all_positions_ticks: u64,
    //monitor_margin_level_thresold: u128,
}

#[derive(Debug, Clone, Parser, ValueEnum )]
pub enum Deployment {
    LOCALNET,
    BASE,
}

#[derive(Debug, PartialEq)]
enum ActionType {
    Deposit,
    Borrow,
    Repay,
    Withdraw,
    Swap,
    Liquidation,
    Closed,
}

impl ActionType {
    fn from_u256(value: U256) -> Option<ActionType> {
        match value.try_into().unwrap() {
            0 => Some(ActionType::Deposit),
            1 => Some(ActionType::Borrow),
            2 => Some(ActionType::Repay),
            3 => Some(ActionType::Withdraw),
            4 => Some(ActionType::Swap),
            5 => Some(ActionType::Liquidation),
            6 => Some(ActionType::Closed),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match *self {
            ActionType::Deposit => "Deposit",
            ActionType::Borrow => "Borrow",
            ActionType::Repay => "Repay",
            ActionType::Withdraw => "Withdraw",
            ActionType::Swap => "Swap",
            ActionType::Liquidation => "Liquidation",
            ActionType::Closed => "Closed",
        }
    }
}

// admin stuff
pub const DEPLOYED_ADDRESSES: &str = "deployments/deployed_addresses.json";
pub const STATE_CACHE_FILE: &str = "borrowers.json";
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const MULTICALL_CHUNK_SIZE: usize = 1000;
pub const RETRY_DURATION_IN_SECS: i64 = 600;
pub const POLL_POOL_CHUNK_SIZE: u64 = 100;
pub const LIQUIDATIONL_TOP_CHUNK_SIZE: u64 = 100;
pub const LIQUIDATIONL_BATCH_SIZE: u64 = 100;
pub const ACTIVITY_LEVEL_INIT: u64 = 100;
pub const ACTIVITY_LEVEL_INIT_TEST: u64 = 5;

//production
// self.config.update_all_pools_ticks: u64 = 16000;//about 2days poll all pools 
// self.config.activity_level_decrease_ticks: u64 = 600;//about 7days to 0
// self.config.calc_all_positions_ticks: u64 = 8000;//about 1day
// self.config.monitor_margin_level_thresold: u128 = 130 ;

//testing
// self.config.update_all_pools_ticks: u64 = 10;//about 100 seconds
// self.config.activity_level_decrease_ticks: u64 = 5;//about 4 minutes to 0
// self.config.calc_all_positions_ticks: u64 = 5;//about 50 seconds
// self.config.monitor_margin_level_thresold: u128 = 150;

fn get_deployment_config(
    deployment: Deployment, 
    last_block_number: u64,
    total_profit: u128,
    pool_interval_secs: u64,
    update_all_pools_secs: u64,
    activity_level_clean_secs: u64,
    calc_all_positions_secs: u64,
    //monitor_margin_level_thresold: u128,
) -> DeploymentConfig {

    let file = File::open(DEPLOYED_ADDRESSES).unwrap();
    let mm_contracts: HashMap<String, Address> = serde_json::from_reader(file).unwrap();

    match deployment {
        Deployment::LOCALNET => DeploymentConfig {
            data_store: *mm_contracts.get("DataStore#DataStore").unwrap(),
            reader: *mm_contracts.get("Reader#Reader").unwrap(),
            event_emitter: *mm_contracts.get("EventEmitter#EventEmitter").unwrap(),
            exchange_router: *mm_contracts.get("ExchangeRouter#ExchangeRouter").unwrap(),
            last_block_number: last_block_number,
            total_profit: total_profit,
            update_all_pools_ticks: update_all_pools_secs/pool_interval_secs,
            activity_level_decrease_ticks: activity_level_clean_secs/(pool_interval_secs*ACTIVITY_LEVEL_INIT),
            activity_level_init: ACTIVITY_LEVEL_INIT,
            calc_all_positions_ticks: calc_all_positions_secs/pool_interval_secs,
            //monitor_margin_level_thresold: monitor_margin_level_thresold,
        },
        Deployment::BASE => DeploymentConfig {
            data_store: *mm_contracts.get("DataStore#DataStore").unwrap(),
            reader: *mm_contracts.get("Reader#Reader").unwrap(),
            event_emitter: *mm_contracts.get("EventEmitter#EventEmitter").unwrap(),
            exchange_router: *mm_contracts.get("ExchangeRouter#ExchangeRouter").unwrap(),
            last_block_number: last_block_number,
            total_profit: total_profit,
            update_all_pools_ticks: update_all_pools_secs/pool_interval_secs,
            activity_level_decrease_ticks: activity_level_clean_secs/(pool_interval_secs*ACTIVITY_LEVEL_INIT_TEST),
            activity_level_init: ACTIVITY_LEVEL_INIT_TEST,
            calc_all_positions_ticks: calc_all_positions_secs/pool_interval_secs,
            //monitor_margin_level_thresold: monitor_margin_level_thresold,
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    pools: HashMap<Bytes32, Pool>,
    positions: HashMap<Bytes32, Position>,
    sents: HashMap<Bytes32, DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    account: Address,
    position_id: U256,
    pool: Bytes32,
    meme_symbol: String,
    base_collateral: U256,
    base_debt_scaled: U256,
    meme_collateral: U256,
    meme_debt_scaled: U256,
    margin_level: U256,
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
    activity_level: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MmStrategy<T, P, N = alloy_contract::private::Ethereum>
{
    client: Arc<P>,
    last_block_number: u64,
    pools: HashMap<Bytes32, Pool>,
    positions: HashMap<Bytes32, Position>,
    //positions_critical: Vec<Position>,
    positions_active: Vec<Position>,
    positions_all: Vec<Position>,
    sents: HashMap<Bytes32, DateTime<Utc>>,
    chain_id: u64,
    config: DeploymentConfig,
    liquidator: Address,
    margin_level_threshold: U256,
    tick_counter: u64,
     _network_transport: ::core::marker::PhantomData<(N, T)>,
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
        liquidator_address: Address,
        last_block_number: u64,
        total_profit: u128,
        pool_interval_secs : u64,
        update_all_pools_secs: u64,
        activity_level_clean_secs: u64,
        calc_all_positions_secs: u64,
    ) -> Self {
        let deployment_config = get_deployment_config(
            deployment, 
            last_block_number, 
            total_profit,
            pool_interval_secs,
            update_all_pools_secs,
            activity_level_clean_secs,
            calc_all_positions_secs,         
        );
        Self {
            client,
            last_block_number: last_block_number,
            positions: HashMap::new(),
            //positions_critical: Vec::new(),
            positions_active: Vec::new(),
            positions_all: Vec::new(),
            pools: HashMap::new(),
            sents: HashMap::new(),
            chain_id: config.chain_id,
            config: deployment_config,
            liquidator: liquidator_address,
            margin_level_threshold: U256::ZERO,
            tick_counter: 0,
            _network_transport: ::core::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<   
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>  + 'static,
    N: alloy_contract::private::Network,
> Strategy<Event, Action<N>> for MmStrategy<T, P, N>{
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");
        info!("self.config.data_store {:?}", self.config.data_store);
        info!("self.config.reader {:?}", self.config.reader);
        info!("self.config.event_emitter {:?}", self.config.event_emitter);
                
        self.load_cache()?;
        self.update_pools().await?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // async fn process_event(&mut self, event: Event) -> Vec<Action<N>> {
    //     match event {
    //         Event::NewTick(block) => self
    //             .process_new_tick_event(block)
    //             .await
    //             .unwrap_or_else(Vec::new),
    //     }
    // }

    async fn process_event(&mut self, event: Event) -> Vec<Action<N>> {
        match event {
            Event::NewTick(block) => {
                let actions = self.process_new_tick_event(block).await;
                match actions {
                    Some(actions) => {
                        if actions.is_empty() {
                            warn!("No actions generated for NewTick event");
                        }
                        actions
                    }
                    None => {
                        warn!("No actions returned for NewTick event");
                        Vec::new()  // Return an empty vector if None is returned
                    }
                }
            }
        }
    }
}

impl<   
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>  + 'static,
    N: alloy_contract::private::Network,
> MmStrategy<T, P, N> {

    // /// Process new block events, updating the internal state.
    // async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Vec<Action<N>>> {
    //     info!("received new tick: {:?}", event);

    //     // Update pools and handle error separately
    //     if let Err(e) = self.update_pools().await {
    //         error!("Update Pools error: {}", e);
    //     }

    //     // Update state and handle error separately
    //     if let Err(e) = self.update_state().await {
    //         error!("Update State error: {}", e);
    //     }

    //     info!("Total position count: {}", self.positions.len());
    //     let underwaters = self.get_underwater_positions().await?;
    //     //let position_chunks = active_pools_ids.chunks(LIQUIDATIONL_BATCH_SIZE as usize);

    //     self.tick_counter = self.tick_counter + 1;

    //     let mut actions: Vec<Action<N>> = Vec::new();
    //     for (account, position_id, _margin_level, _collateral, _debt) in underwaters {
    //         info!("underwater: {:?} position_id:{} ", account, position_id);

    //         let now: DateTime<Utc> = Utc::now();
    //         self.sents.insert(hash_position_key(account.clone(), position_id), now);

    //         let action = async {
    //             self.build_liquidation_tx(&account, position_id)
    //                 .await
    //                 .map_err(|e| {
    //                     error!("Error building liquidation: {}", e);
    //                     e
    //                 })
    //                 .ok()
    //                 .map(|tx| Action::SubmitTx(SubmitTxToMempool {
    //                     tx,
    //                     gas_bid_info: Some(GasBidInfo{total_profit:self.config.total_profit, bid_percentage:0}),
    //                 }))
    //         }
    //         .await;

    //         if let Some(action) = action {
    //             actions.push(action);
    //         }
    //     }

    //     Some(actions)
    //     //None
    // }

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Vec<Action<N>>> {
        info!("received new tick: {:?}", event);

        // Update pools and handle error separately
        if let Err(e) = self.update_pools().await {
            error!("Update Pools error: {}", e);
        }

        // Update state and handle error separately
        if let Err(e) = self.update_state().await {
            error!("Update State error: {}", e);
        }

        info!("Total position count: {}", self.positions.len());
        let underwaters = self.get_underwater_positions().await?;
        let underwaters_chunks = underwaters.chunks(LIQUIDATIONL_BATCH_SIZE as usize);

        self.tick_counter = self.tick_counter + 1;

        let mut actions: Vec<Action<N>> = Vec::new();
        for chunk in underwaters_chunks {
            //info!("underwater: {:?} position_id:{} ", account, position_id);
            let mut account_list: Vec<LiquidationParams>;
            for (account, position_id, _margin_level, _collateral, _debt) in underwaters {
                let now: DateTime<Utc> = Utc::now();
                self.sents.insert(hash_position_key(account.clone(), position_id), now);
                account_list.push(LiquidationParams{&account, positionId: position_id});
            }

            let action = async {
                self.build_liquidation_tx(&account_list)
                    .await
                    .map_err(|e| {
                        error!("Error building liquidation: {}", e);
                        e
                    })
                    .ok()
                    .map(|tx| Action::SubmitTx(SubmitTxToMempool {
                        tx,
                        gas_bid_info: Some(GasBidInfo{total_profit:self.config.total_profit, bid_percentage:0}),
                    }))
            }
            .await;

            if let Some(action) = action {
                actions.push(action);
            }
        }

        Some(actions)
        //None
    }

    // async fn build_liquidation_tx(&self, account: &Address, position_id: U256) -> Result<<N as Network>::TransactionRequest> {
    //     let exchange_router = ExchangeRouter::new(self.config.exchange_router, self.client.clone());
    //     let call_build = exchange_router.executeLiquidation(LiquidationParams{
    //         account: *account,
    //         positionId: position_id
    //     });
        
    //     let mut tx = call_build.into_transaction_request();
    //     tx.set_chain_id(self.chain_id);
    //     tx.set_from(self.liquidator);

    //     Ok(tx)
    // }

    async fn build_liquidation_tx(&self, liquidation_list: &Vec<LiquidationParams>) -> Result<<N as Network>::TransactionRequest> {
        let exchange_router = ExchangeRouter::new(self.config.exchange_router, self.client.clone());
        let call_build = exchange_router.executeLiquidationBatch(liquidation_list);
        
        let mut tx = call_build.into_transaction_request();
        tx.set_chain_id(self.chain_id);
        tx.set_from(self.liquidator);

        Ok(tx)
    }

     async fn get_underwater_positions(&mut self) -> Option<Vec<(Address, U256, U256, U256, U256)>> {

            let start = Instant::now();  // Record the start time

            let mut underwater_positions = Vec::new();  
            // let positions : &mut Vec<Position> = if self.tick_counter % self.config.calc_all_positions_ticks == 0 {
            //     self.positions_all = self.positions.iter().map(|(_, pos)| pos.clone()).collect::<Vec<Position>>();
            //     &mut self.positions_all
            // } else {
            //     &mut self.positions_critical
            // };

            let positions : &mut Vec<Position> = if self.tick_counter % self.config.calc_all_positions_ticks == 0 {
                self.positions_all = self.positions.iter().map(|(_, pos)| pos.clone()).collect::<Vec<Position>>();
                &mut self.positions_all
            } else {
                self.positions_active = self.positions.iter()
                    .filter_map(|(_, pos)| {
                        if let Some(pool) = self.pools.get(&pos.pool) {
                            if pool.activity_level > 0 {
                                Some(pos.clone()) // Include the position if the pool has activity_level > 0
                            } else {
                                None // Skip this position if the pool's activity_level is not greater than 0
                            }
                        } else {
                            info!("No pool found for position {:?}", pos.pool);
                            None // Skip this position if no pool is found
                        }
                    })
                    .collect();
                &mut self.positions_active
            };

            //self.positions_all = self.positions.iter().map(|(_, pos)| pos.clone()).collect::<Vec<Position>>();
            //let positions : &mut Vec<Position> = &mut self.positions_all;
            // dbg!(&positions);

            for position in positions.iter_mut() {
                let mut user_total_collateral_usd = U256::ZERO;
                let mut user_total_debt_usd = U256::ZERO;

                // Check if the pool exists before processing
                let pool = match self.pools.get(&position.pool) {
                    Some(pool) => pool,
                    None => {
                        info!("No pool found for position {:?}", position.pool);
                        continue; // Skip this position
                    }
                };

                //if didn't get price
                if pool.price <= U256::ZERO  {
                    continue;
                }

                let price = pool.price;
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

                // Update the margin_level in position
                position.margin_level = margin_level;

                if margin_level < self.margin_level_threshold {
                    //prevent resend tx
                    let now: DateTime<Utc> = Utc::now();
                    match self.sents.get(&hash_position_key(position.account.clone(), position.position_id)) {
                        Some(&sent_time) => {
                            let duration: Duration = now - sent_time;
                            if duration.num_seconds() < RETRY_DURATION_IN_SECS {
                                continue;
                            }
                        },
                        None => {}
                    }

                    underwater_positions.push((
                        position.account,
                        position.position_id,
                        margin_level,
                        user_total_collateral_usd,
                        user_total_debt_usd,
                    ));
                }
            }
            
            // dbg!(&self.positions_all);
            // dbg!(&self.positions_critical);

            // Process positions on every 5th tick
            // if self.tick_counter % self.config.calc_all_positions_ticks == 0 {
            //     self.positions_all.sort_by(|a, b| a.margin_level.cmp(&b.margin_level));
            //     self.positions_critical = self.positions_all.iter()
            //         .filter(|p| p.margin_level < U256::from(self.config.monitor_margin_level_thresold))
            //         .cloned()
            //         .collect();
            // } else {
            //     self.positions_critical.sort_by(|a, b| a.margin_level.cmp(&b.margin_level));
            // }

            info!("Underwater count: {}", underwater_positions.len());
            underwater_positions.sort_by(|a, b| a.1.cmp(&b.1));
            let top_underwater_positions = underwater_positions
                .iter()
                .take(LIQUIDATIONL_TOP_CHUNK_SIZE as usize)
                .cloned()
                .collect();

            let duration = start.elapsed();  // Calculate elapsed time
            info!("Underwater elapsed time: {:?}", duration);

            Some(top_underwater_positions)  
        }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.positions = cache.positions;
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
        if start_block > latest_block {
            start_block = latest_block;
        }
        info!(
            "Updating state from block {} to {}",
            start_block, latest_block
        );

        self.update_margin_levle_threshold().await?;

        self.get_position_logs(start_block.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let pool_key = hash_pool_key(log.baseToken, log.memeToken);

                // Check if the pool exists, and set meme_symbol accordingly
                let meme_symbol = if let Some(pool) = self.pools.get(&pool_key) {
                    pool.meme_symbol.clone()  // If the pool exists, get the meme_symbol
                } else {
                    "".to_string()  // If the pool doesn't exist, set meme_symbol to an empty string
                };

                info!("{} {:?} {} {} {} {} {}", 
                    ActionType::from_u256(log.actionType).map_or("Unknown".to_string(), |action| action.to_string().into()), 
                    log.account, meme_symbol, 
                    log.baseCollateral, log.baseDebtScaled, log.memeCollateral, log.memeDebtScaled
                ); 

                // Insert or update the pool's activity_level to 100
                if let Some(existing_pool) = self.pools.get_mut(&pool_key) {
                    existing_pool.activity_level = self.config.activity_level_init;  // Update existing pool's activity level
                } else {
                    let new_pool = Pool {
                        price: U256::ZERO,  // Set to a default value, update later as needed
                        price_decimals: U256::ZERO,  // Set to a default value
                        base_token: log.baseToken,
                        base_symbol: "".to_string(),  // Replace with actual base symbol
                        base_token_decimals: U256::ZERO,  // Set to a default value
                        base_borrow_index: U256::ZERO,  // Set to a default value
                        meme_token: log.memeToken,
                        meme_symbol: "".to_string(),
                        meme_token_decimals: U256::ZERO,  // Set to a default value
                        meme_borrow_index: U256::ZERO,  // Set to a default value
                        activity_level: self.config.activity_level_init,  // Set activity_level to self.config.activity_level_init when the pool is new
                    };
                    self.pools.insert(pool_key, new_pool);
                } 

                let user = log.account; 
                if ActionType::from_u256(log.actionType).map_or(false, |action| action == ActionType::Liquidation) || 
                   ActionType::from_u256(log.actionType).map_or(false, |action| action == ActionType::Closed) {
                    self.positions.remove(&hash_position_key(user, log.positionId)); 
                    self.sents.remove(&hash_position_key(user, log.positionId));
                    return;
                }     
                self.update_position(
                    hash_position_key(user, log.positionId),
                    user, 
                    log.positionId,
                    hash_pool_key(log.baseToken, log.memeToken), 
                    meme_symbol,
                    log.baseCollateral, 
                    log.baseDebtScaled, 
                    log.memeCollateral, 
                    log.memeDebtScaled
                );
                return;
            });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block,
            pools: self.pools.clone(),
            positions: self.positions.clone(),
            sents: self.sents.clone(),
        };

        self.last_block_number = latest_block;
        let file = File::create(STATE_CACHE_FILE)?;
        serde_json::to_writer_pretty(file, &cache)?;

        Ok(())
    }

        // fetch all position events from the from_block to to_block
    async fn get_position_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<EventEmitter::Position>> {
        let event_emitter = EventEmitter::new(self.config.event_emitter, self.client.clone());
        let mut res = Vec::new();
        for start_block in
            (from_block..to_block).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block);
            event_emitter.Position_filter()  
                .from_block(start_block)
                .to_block(end_block)
                .address(self.config.event_emitter)
                .query()
                .await?
                .into_iter()
                .for_each(|(log, _)| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    fn insert_or_update_pool(&mut self, pool: Pool) {
        let pool_id = hash_pool_key(pool.base_token, pool.meme_token);

        // If the pool doesn't exist, insert it
        if !self.pools.contains_key(&pool_id) {
            info!("Inserting new pool: {:?}", pool_id);
            self.pools.insert(pool_id, pool);
        } else {
            // If it exists, update the pool while keeping the existing activity_level
            if let Some(existing_pool) = self.pools.get_mut(&pool_id) {
                existing_pool.price = pool.price;
                existing_pool.price_decimals = pool.price_decimals;
                existing_pool.base_token = pool.base_token;
                existing_pool.base_symbol = pool.base_symbol;
                existing_pool.base_token_decimals = pool.base_token_decimals;
                existing_pool.base_borrow_index = pool.base_borrow_index;
                existing_pool.meme_token = pool.meme_token;
                existing_pool.meme_symbol = pool.meme_symbol;
                existing_pool.meme_token_decimals = pool.meme_token_decimals;
                existing_pool.meme_borrow_index = pool.meme_borrow_index;
            }
        }
    }

    async fn update_pools(&mut self) -> Result<()> {
        let reader = Reader::new(self.config.reader.clone(), self.client.clone());

        info!("tick_counter: {:?}", self.tick_counter);

        if self.tick_counter % self.config.update_all_pools_ticks == 0 {  // Every 50 seconds (5p) update all pools
            info!("getPoolsInfo_1");
            //1.get pool account
            let mut pools_count :u64 = 0;
            let pools_count_ret = reader.getPoolsCount(
                self.config.data_store.clone(), 
            ).call().await;
            match pools_count_ret {
                Ok(ret) => {
                    //all_pools.extend(pools._0);
                    pools_count = ret._0.try_into().unwrap();
                }
                Err(e) => {
                    error!("Error fetching pools account:{:?}", e);
                    //break;
                }
            }

            //2.get pools
            let mut all_pools: Vec<_> = Vec::new();
            for i in 0..(pools_count / POLL_POOL_CHUNK_SIZE) {

                let pools_result = reader.getPoolsInfo_1(
                    self.config.data_store.clone(), 
                    U256::from(i * POLL_POOL_CHUNK_SIZE), 
                    U256::from((i + 1) * POLL_POOL_CHUNK_SIZE)
                ).call().await;

                match pools_result {
                    Ok(pools) => {
                        all_pools.extend(pools._0);
                    }
                    Err(e) => {
                        error!("Error fetching pools by getPoolsInfo_1 at index {}: {:?}", i, e);
                        break;
                    }
                }
            }

            // 3.Get remaining pools if pools_count is not divisible by POLL_POOL_CHUNK_SIZE
            if pools_count % POLL_POOL_CHUNK_SIZE != 0 {
                let pools_result = reader.getPoolsInfo_1(
                    self.config.data_store.clone(), 
                    U256::from((pools_count / POLL_POOL_CHUNK_SIZE) * POLL_POOL_CHUNK_SIZE), 
                    U256::from(pools_count),
                ).call().await;

                match pools_result {
                    Ok(pools) => {
                        all_pools.extend(pools._0);
                    }
                    Err(e) => {
                        error!("Error fetching rest pools by getPoolsInfo_1: {:?}", e);
                        //break;
                    }
                }

            }

            info!("all_pools: {:?}", all_pools.len());
            // 4.update pools
            for pool in all_pools.iter() {
                let pool_data = Pool {
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
                    activity_level: 0,  // Set to zero or leave as placeholder (not provided by reader)
                };

                self.insert_or_update_pool(pool_data);  // Insert or update the pool
            }

        } else {  // Every tick  update first  pools with activity_level > 0
            // 1.Filter pools with activity_level > 0
            let mut filtered_pools: Vec<_> = self.pools.values()
                .filter(|pool| pool.activity_level > 0)  // Only pools with activity_level > 0
                .collect();

            // 2.Sort pools by activity_level in descending order
            filtered_pools.sort_by(|a, b| b.activity_level.cmp(&a.activity_level));  // Sort descending by activity_level

            // 3.Get active pool ids
            let active_pools_ids: Vec<_> = filtered_pools.iter()
                .map(|pool| hash_pool_key(pool.base_token, pool.meme_token))
                .collect();

            info!("getPoolsInfo_2");

            // 4.Get pools by chunks
            let pool_chunks = active_pools_ids.chunks(POLL_POOL_CHUNK_SIZE as usize);
            let mut active_pools = Vec::new(); 
            for chunk in pool_chunks {
                let chunk_clone = chunk.to_vec();
                let pools_result = reader.getPoolsInfo_2(
                    self.config.data_store.clone(), 
                    chunk_clone, 
                ).call().await;

                match pools_result {
                    Ok(pools) => {
                        active_pools.extend(pools._0);
                    }
                    Err(e) => {
                        error!("Error fetching pools by getPoolsInfo_2: {:?}", e);
                        break;
                    }
                }

 
            }

            info!("active_pools: {:?}", active_pools.len());

            // Insert the updated first 10 pools
            for pool in active_pools.iter() {
                let pool_data = Pool {
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
                    activity_level: 0,  // Set to zero or leave as placeholder (not provided by reader)
                };

                self.insert_or_update_pool(pool_data);  // Insert or update the pool
            }
        }

        // Decrease activity_level every ACTIVITY_LEVEL_DECREASE_PER_TIMES ticks
        if self.tick_counter % self.config.activity_level_decrease_ticks == 0 {
            for pool in self.pools.values_mut() {
                if pool.activity_level > 0 {
                    pool.activity_level -= 1;
                    //info!("Decreased activity_level for pool: {:?}", pool);
                }
            }
        }

        Ok(())
    }

    async fn update_margin_levle_threshold(&mut self) -> Result<()> {
        let reader = Reader::new(self.config.reader, self.client.clone());
        self.margin_level_threshold = reader.getMarginLevelThreshold(self.config.data_store).call().await.unwrap()._0;
        info!("margin_levle_threshold {:?}", self.margin_level_threshold);
        Ok(())
    }

    fn update_position(
        &mut self, 
        position_key: Bytes32,
        account: Address, 
        position_id: U256,
        pool: Bytes32, 
        meme_symbol: String,
        base_collateral: U256, 
        base_debt_scaled: U256, 
        meme_collateral: U256, 
        meme_debt_scaled: U256
    ) {
        //remove position and remove borrower
        if base_collateral == U256::ZERO && meme_collateral == U256::ZERO && 
           base_debt_scaled == U256::ZERO && meme_debt_scaled == U256::ZERO {
            if self.positions.contains_key(&position_key) {
                self.positions.remove(&position_key);
            }
            return
        }

        //insert position
        self.positions.insert(
            position_key, 
            Position {
                account:account,
                position_id:position_id,
                pool:pool,
                meme_symbol:meme_symbol,
                base_collateral:base_collateral,
                base_debt_scaled:base_debt_scaled,
                meme_collateral:meme_collateral,
                meme_debt_scaled:meme_debt_scaled,
                margin_level:U256::ZERO,
            }
        );

    }

    // async fn get_underwater_positions(&mut self) -> Option<Vec<(Address, U256, U256, U256, U256)>> {

    //     let positions: Vec<Position> = if self.tick_counter % self.config.calc_all_positions_ticks == 0 {
    //         self.positions_all = self.positions.iter().map(|(_, pos)| pos.clone()).collect::<Vec<Position>>();
    //         self.positions_all.clone()
    //     } else {
    //         self.positions_critical.clone()
    //     };

    //     // Split positions into chunks for parallel processing
    //     let chunk_size = MULTICALL_CHUNK_SIZE;  // Adjust as needed
    //     //let mut chunks = positions.chunks(chunk_size);  // Use `chunks_mut` to modify positions
    //     let mut chunks: Vec<Vec<Position>> = positions.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    //     // Spawn tasks to process each chunk
    //     let mut tasks = Vec::new();

    //     for chunk in chunks.iter_mut() {
    //         // Spawn a task for each chunk
    //         let task = tokio::spawn({
    //             let mut chunk_underwater_positions = Vec::new(); // Collect underwater positions for this chunk

    //             async move {
    //                 for position in chunk.iter_mut() {
    //                     let mut user_total_collateral_usd = U256::ZERO;
    //                     let mut user_total_debt_usd = U256::ZERO;

    //                     // Check if the pool exists before processing
    //                     let pool = match self.pools.get(&position.pool) {
    //                         Some(pool) => pool,
    //                         None => {
    //                             info!("No pool found for position {:?}", position.pool);
    //                             continue; // Skip this position
    //                         }
    //                     };

    //                     let price = pool.price;
    //                     let base_borrow_index = pool.base_borrow_index;
    //                     let base_debt = ray_mul(position.base_debt_scaled, base_borrow_index);
    //                     let base_decimals = pool.base_token_decimals;
    //                     user_total_collateral_usd += adjust_precision(position.base_collateral, base_decimals);
    //                     user_total_debt_usd += adjust_precision(base_debt, base_decimals);

    //                     let meme_borrow_index = pool.meme_borrow_index;
    //                     let meme_debt = ray_mul(position.meme_debt_scaled, meme_borrow_index);
    //                     let meme_decimals = pool.meme_token_decimals;
    //                     user_total_collateral_usd += ray_mul(price, adjust_precision(position.meme_collateral, meme_decimals));
    //                     user_total_debt_usd += ray_mul(price, adjust_precision(meme_debt, meme_decimals));

    //                     let margin_level = if user_total_debt_usd == U256::ZERO {
    //                         U256::MAX
    //                     } else {
    //                         ray_div(user_total_collateral_usd, user_total_debt_usd)
    //                     };

    //                     // Update the margin_level in position
    //                     position.margin_level = margin_level;
    //                     let mut updated_position = position.clone();
    //                     updated_position.margin_level = margin_level;

    //                     if margin_level < self.margin_level_threshold {
    //                         chunk_underwater_positions.push((
    //                             updated_position.account,
    //                             updated_position.position_id,
    //                             margin_level,
    //                             user_total_collateral_usd,
    //                             user_total_debt_usd,
    //                         ));
    //                     }
    //                 }

    //                 chunk_underwater_positions
    //             }
    //         });

    //         tasks.push(task);
    //     }

    //     // Wait for all tasks to finish and collect the results
    //     let mut all_underwater_positions = Vec::new();
    //     let results = futures::future::join_all(tasks).await;

    //     for result in results {
    //         if let Ok(chunk_positions) = result {
    //             all_underwater_positions.extend(chunk_positions);
    //         }
    //     }

    //     // After processing, merge the underwater positions back into positions
    //     let mut updated_positions = Vec::new();
    //     for chunk in chunks {
    //         updated_positions.extend_from_slice(&chunk);  // Add modified positions back
    //     }

    //     // Sort and filter positions if necessary
    //     if self.tick_counter % 5 == 0 {
    //         self.positions_all = updated_positions.clone(); 
    //         self.positions_all.sort_by(|a, b| a.margin_level.cmp(&b.margin_level));
    //         self.positions_critical = self.positions_all.iter()
    //             .filter(|p| p.margin_level < U256::from(self.config.monitor_margin_level_thresold))
    //             .cloned()
    //             .collect();
    //     } else {
    //         self.positions_critical = updated_positions.clone(); 
    //         self.positions_critical.sort_by(|a, b| a.margin_level.cmp(&b.margin_level));
    //     }

    //     Some(all_underwater_positions)  // Return the final result after all tasks are done
    // }
}

pub fn hash_data(data: Vec<Vec<u8>>) -> Vec<u8> {
    // Concatenate all byte arrays in the data
    let mut result = Vec::new();
    for item in data {
        result.extend(item);
    }
    result
}

pub fn keccak256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn hash_pool_key(token0: Address, token1: Address) -> Bytes32 {
    let pool_bytes = "POOL".to_string().abi_encode();
    let pool_hash = keccak256_hash(&pool_bytes);

    // Ensure the tokens are in lexicographical order (lowercase comparison)
    let (token0, token1) = if token0 < token1 {
        (token0, token1)
    } else {
        (token1, token0)
    };

    let token0_bytes = token0.abi_encode();
    let token1_bytes = token1.abi_encode();

    // Concatenate POOL (bytes32), token0, and token1 (addresses)
    let encoded_data = vec![pool_hash, token0_bytes, token1_bytes];

    // // Return the hash of the concatenated data
    let concatenated = hash_data(encoded_data);
    
    // Perform Keccak256 hashing on the concatenated bytes
    let mut hasher = Keccak256::new();
    hasher.update(concatenated);
    let hash_result = hasher.finalize();

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

