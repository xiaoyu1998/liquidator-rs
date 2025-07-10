# liquidator-rs
This liquidator bot will constant checking the status of all the accounts in dex and liquidation 10K positions offchain in a millsecond.

# Usage
#### Download

```shell
git clone git@github.com:xiaoyu1998/liquidator-rs.git --recursive
```
#### copy addresses from contracts
```
cp deployed_addresses.json /path/to/liquidator-rs/crates/liquidator-contract/deployments
cp underlyAsset_addresses.json /path/to/liquidator-rs/deployments

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
cargo run -- --rpc http://127.0.0.1:8545 --private-key "private-key" --deployment localnet --bid-percentage 10 --total-profit 800000000000000 --chain-id 31337 --last-block-number 1 --pool-interval-secs 10

```
