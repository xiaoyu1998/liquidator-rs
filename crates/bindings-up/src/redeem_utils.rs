pub use redeem_utils::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod redeem_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EmptyCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyCollateral"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyPosition"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyRedeemAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyRedeemAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "HealthFactorLowerThanLiquidationThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HealthFactorLowerThanLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("healthFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "healthFactorLiquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolIsFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolIsFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolIsInactive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolIsInactive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolIsNotBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolIsNotBorrowing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolIsPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolIsPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static REDEEMUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct RedeemUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RedeemUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RedeemUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RedeemUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RedeemUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RedeemUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RedeemUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REDEEMUTILS_ABI.clone(),
                    client,
                ),
            )
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RedeemUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EmptyCollateral` with signature `EmptyCollateral()` and selector `0x6c53056d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "EmptyCollateral", abi = "EmptyCollateral()")]
    pub struct EmptyCollateral;
    ///Custom Error type `EmptyPool` with signature `EmptyPool(address)` and selector `0x00ee0bb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "EmptyPool", abi = "EmptyPool(address)")]
    pub struct EmptyPool {
        pub key: ::ethers::core::types::Address,
    }
    ///Custom Error type `EmptyPosition` with signature `EmptyPosition()` and selector `0x4dfbbff3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "EmptyPosition", abi = "EmptyPosition()")]
    pub struct EmptyPosition;
    ///Custom Error type `EmptyRedeemAmount` with signature `EmptyRedeemAmount()` and selector `0xaa0ac363`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "EmptyRedeemAmount", abi = "EmptyRedeemAmount()")]
    pub struct EmptyRedeemAmount;
    ///Custom Error type `HealthFactorLowerThanLiquidationThreshold` with signature `HealthFactorLowerThanLiquidationThreshold(uint256,uint256)` and selector `0x9c240105`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "HealthFactorLowerThanLiquidationThreshold",
        abi = "HealthFactorLowerThanLiquidationThreshold(uint256,uint256)"
    )]
    pub struct HealthFactorLowerThanLiquidationThreshold {
        pub health_factor: ::ethers::core::types::U256,
        pub health_factor_liquidation_threshold: ::ethers::core::types::U256,
    }
    ///Custom Error type `PoolIsFrozen` with signature `PoolIsFrozen(address)` and selector `0x1f01e9db`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolIsFrozen", abi = "PoolIsFrozen(address)")]
    pub struct PoolIsFrozen {
        pub pool: ::ethers::core::types::Address,
    }
    ///Custom Error type `PoolIsInactive` with signature `PoolIsInactive(address)` and selector `0xb81ba3bd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolIsInactive", abi = "PoolIsInactive(address)")]
    pub struct PoolIsInactive {
        pub pool: ::ethers::core::types::Address,
    }
    ///Custom Error type `PoolIsNotBorrowing` with signature `PoolIsNotBorrowing(address)` and selector `0x147e8a5d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolIsNotBorrowing", abi = "PoolIsNotBorrowing(address)")]
    pub struct PoolIsNotBorrowing {
        pub pool: ::ethers::core::types::Address,
    }
    ///Custom Error type `PoolIsPaused` with signature `PoolIsPaused(address)` and selector `0xdca37e03`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PoolIsPaused", abi = "PoolIsPaused(address)")]
    pub struct PoolIsPaused {
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum RedeemUtilsErrors {
        EmptyCollateral(EmptyCollateral),
        EmptyPool(EmptyPool),
        EmptyPosition(EmptyPosition),
        EmptyRedeemAmount(EmptyRedeemAmount),
        HealthFactorLowerThanLiquidationThreshold(
            HealthFactorLowerThanLiquidationThreshold,
        ),
        PoolIsFrozen(PoolIsFrozen),
        PoolIsInactive(PoolIsInactive),
        PoolIsNotBorrowing(PoolIsNotBorrowing),
        PoolIsPaused(PoolIsPaused),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RedeemUtilsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EmptyCollateral as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyCollateral(decoded));
            }
            if let Ok(decoded) = <EmptyPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyPool(decoded));
            }
            if let Ok(decoded) = <EmptyPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyPosition(decoded));
            }
            if let Ok(decoded) = <EmptyRedeemAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyRedeemAmount(decoded));
            }
            if let Ok(decoded) = <HealthFactorLowerThanLiquidationThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorLowerThanLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <PoolIsFrozen as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolIsFrozen(decoded));
            }
            if let Ok(decoded) = <PoolIsInactive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolIsInactive(decoded));
            }
            if let Ok(decoded) = <PoolIsNotBorrowing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolIsNotBorrowing(decoded));
            }
            if let Ok(decoded) = <PoolIsPaused as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolIsPaused(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RedeemUtilsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EmptyCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyRedeemAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolIsFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolIsInactive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolIsNotBorrowing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolIsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RedeemUtilsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EmptyCollateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EmptyPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyRedeemAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HealthFactorLowerThanLiquidationThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolIsFrozen as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolIsInactive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolIsNotBorrowing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolIsPaused as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RedeemUtilsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmptyCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyRedeemAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolIsFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsInactive(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsNotBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolIsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RedeemUtilsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EmptyCollateral> for RedeemUtilsErrors {
        fn from(value: EmptyCollateral) -> Self {
            Self::EmptyCollateral(value)
        }
    }
    impl ::core::convert::From<EmptyPool> for RedeemUtilsErrors {
        fn from(value: EmptyPool) -> Self {
            Self::EmptyPool(value)
        }
    }
    impl ::core::convert::From<EmptyPosition> for RedeemUtilsErrors {
        fn from(value: EmptyPosition) -> Self {
            Self::EmptyPosition(value)
        }
    }
    impl ::core::convert::From<EmptyRedeemAmount> for RedeemUtilsErrors {
        fn from(value: EmptyRedeemAmount) -> Self {
            Self::EmptyRedeemAmount(value)
        }
    }
    impl ::core::convert::From<HealthFactorLowerThanLiquidationThreshold>
    for RedeemUtilsErrors {
        fn from(value: HealthFactorLowerThanLiquidationThreshold) -> Self {
            Self::HealthFactorLowerThanLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<PoolIsFrozen> for RedeemUtilsErrors {
        fn from(value: PoolIsFrozen) -> Self {
            Self::PoolIsFrozen(value)
        }
    }
    impl ::core::convert::From<PoolIsInactive> for RedeemUtilsErrors {
        fn from(value: PoolIsInactive) -> Self {
            Self::PoolIsInactive(value)
        }
    }
    impl ::core::convert::From<PoolIsNotBorrowing> for RedeemUtilsErrors {
        fn from(value: PoolIsNotBorrowing) -> Self {
            Self::PoolIsNotBorrowing(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for RedeemUtilsErrors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
}
