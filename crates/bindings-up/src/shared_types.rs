///`BorrowParams(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct BorrowParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
///`CloseParams(address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct CloseParams {
    pub underlying_asset_usd: ::ethers::core::types::Address,
}
///`ClosePositionParams(address,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ClosePositionParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub underlying_asset_usd: ::ethers::core::types::Address,
    pub percentage: ::ethers::core::types::U256,
}
///`DepositParams(address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DepositParams {
    pub underlying_asset: ::ethers::core::types::Address,
}
///`CalculateInterestRatesParams(uint256,uint256,uint256,address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct CalculateInterestRatesParams {
    pub total_available_liquidity: ::ethers::core::types::U256,
    pub total_debt: ::ethers::core::types::U256,
    pub fee_factor: ::ethers::core::types::U256,
    pub underlying_asset: ::ethers::core::types::Address,
    pub pool_token: ::ethers::core::types::Address,
}
///`LiquidationParams(address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct LiquidationParams {
    pub account: ::ethers::core::types::Address,
}
///`Props(uint256,uint256,uint256,uint256,uint256,address,address,address,address,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PoolProps {
    pub key_id: ::ethers::core::types::U256,
    pub liquidity_index: ::ethers::core::types::U256,
    pub liquidity_rate: ::ethers::core::types::U256,
    pub borrow_index: ::ethers::core::types::U256,
    pub borrow_rate: ::ethers::core::types::U256,
    pub interest_rate_strategy: ::ethers::core::types::Address,
    pub underlying_asset: ::ethers::core::types::Address,
    pub pool_token: ::ethers::core::types::Address,
    pub debt_token: ::ethers::core::types::Address,
    pub configuration: ::ethers::core::types::U256,
    pub total_fee: ::ethers::core::types::U256,
    pub unclaimed_fee: ::ethers::core::types::U256,
    pub last_update_timestamp: ::ethers::core::types::U256,
}
///`Props(address,address,uint256,uint256,uint256,uint256,uint256,bool,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PositionProps {
    pub account: ::ethers::core::types::Address,
    pub underlying_asset: ::ethers::core::types::Address,
    pub entry_long_price: ::ethers::core::types::U256,
    pub acc_long_amount: ::ethers::core::types::U256,
    pub entry_short_price: ::ethers::core::types::U256,
    pub acc_short_amount: ::ethers::core::types::U256,
    pub position_type: ::ethers::core::types::U256,
    pub has_collateral: bool,
    pub has_debt: bool,
}
///`GetLiquidationHealthFactor(uint256,uint256,bool,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct GetLiquidationHealthFactor {
    pub health_factor: ::ethers::core::types::U256,
    pub health_factor_liquidation_threshold: ::ethers::core::types::U256,
    pub is_health_factor_higher_than_liquidation_threshold: bool,
    pub user_total_collateral_usd: ::ethers::core::types::U256,
    pub user_total_debt_usd: ::ethers::core::types::U256,
}
///`RedeemParams(address,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RedeemParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub to: ::ethers::core::types::Address,
}
///`RepaySubstituteParams(address,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RepaySubstituteParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub substitute: ::ethers::core::types::Address,
}
///`RepayParams(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RepayParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
///`SupplyParams(address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SupplyParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
}
///`SwapParams(address,address,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SwapParams {
    pub underlying_asset_in: ::ethers::core::types::Address,
    pub underlying_asset_out: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub sqrt_price_limit_x96: ::ethers::core::types::U256,
}
///`WithdrawParams(address,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct WithdrawParams {
    pub underlying_asset: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub to: ::ethers::core::types::Address,
}
