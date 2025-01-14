use std::{ops::Mul, sync::Arc};
use tracing::info;

use anyhow::{Context, Result};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use alloy::{
    contract as alloy_contract,
    //providers::Provider, 
    // network::Ethereum,
    // transports::BoxTransport,
    network::TransactionBuilder,
};

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<T, P, N = alloy_contract::private::Ethereum> {
    client: Arc<P>,
    sender_client: Arc<P>,
    _network_transport: ::core::marker::PhantomData<(N, T)>,
}

impl<
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>,
    N: alloy_contract::private::Network,
> ProtectExecutor<T, P, N> {
    pub fn new(client: Arc<P>, sender_client: Arc<P>) -> Self {
        Self {
            client,
            sender_client,
            _network_transport: ::core::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>,
    N: alloy_contract::private::Network,
> Executor<SubmitTxToMempool<N>> for ProtectExecutor<T, P, N>
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool<N>) -> Result<()> {
        info!("Executing tx {:?}", action.tx);
        //action.tx.set_from(self.client.default_signer_address());
        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx)
            .await
            .context("Error estimating gas usage: {}");

        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result?;

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // Just use estimated gas price but throw if its too low
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
            let estimated_cost = bid_gas_price.mul(gas_usage as u128);
            info!(
                "Gas bid info: {:?}, estimated cost: {}, bid gas price: {}",
                gas_bid_info, estimated_cost, bid_gas_price
            );
            if estimated_cost > gas_bid_info.total_profit {
                anyhow::bail!("Estimated cost of tx is greater than total profit");
            }
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
        }
        info!("bid_gas_price {:?}", bid_gas_price);
        action.tx.set_gas_price(bid_gas_price);
        action.tx.set_gas_limit(gas_usage);
        info!("gas limit {:?}", action.tx.gas_limit().unwrap());
        let _ = self.sender_client.send_transaction(action.tx).await?;
        Ok(())
    }
}
