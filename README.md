# up-liquidator-rs
This is an [paradigmxyz/artemis](https://github.com/paradigmxyz/artemis) bot that will constant checking the accounts and liquidate positions on chain

# Usage
#### Download

```shell
git clone git@github.com:xiaoyu1998/up-liquidator-rs.git --recursive
```
#### copy addresses from up-contracts
```
cp deployed_addresses.json /path/to/up-liquidator-rs/crates/liquidator-contract/deployments
cp underlyAsset_addresses.json /path/to/up-liquidator-rs/deployments

```
#### deploy liquidator contract
```shell
cd crates/liquidator-contract
npx hardhat run scripts/deployLiquidator.ts --network localnet
```
#### transfer usdt to liquidator contract
```
usdt.transfer("liquidator contract address", amount);

```
#### copy addresses from crates/liquidator-contract
```
cp deployed_addresses.json /path/to/up-liquidator-rs/deployments

```
#### add liquidator address to command line and run liquidator
```
cargo run -- --rpc http://192.168.2.106:8545 --private-key "private-key " --deployment localnet --bid-percentage 10 --liquidator-address 0xa58FEa4CAD0e8D14294d861c54cCa3606820A871 --chain-id 1998 --last-block-number 1000 --pool-interval-secs 10

```