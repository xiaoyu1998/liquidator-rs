pub use borrow_utils::*;
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
pub mod borrow_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BorrowCapacityExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BorrowCapacityExceeded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowCapacity"),
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
                    ::std::borrow::ToOwned::to_owned("CollateralBalanceIsZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralBalanceIsZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyBorrowAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyBorrowAmounts"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientLiquidityForBorrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientLiquidityForBorrow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToBorrow"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "availableLiquidity",
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
                (
                    ::std::borrow::ToOwned::to_owned("PoolNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolNotFound"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BORROWUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct BorrowUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BorrowUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BorrowUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BorrowUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BorrowUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BorrowUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BorrowUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BORROWUTILS_ABI.clone(),
                    client,
                ),
            )
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BorrowUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BorrowCapacityExceeded` with signature `BorrowCapacityExceeded(uint256,uint256)` and selector `0xc20e3b1a`
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
        name = "BorrowCapacityExceeded",
        abi = "BorrowCapacityExceeded(uint256,uint256)"
    )]
    pub struct BorrowCapacityExceeded {
        pub total_debt: ::ethers::core::types::U256,
        pub borrow_capacity: ::ethers::core::types::U256,
    }
    ///Custom Error type `CollateralBalanceIsZero` with signature `CollateralBalanceIsZero()` and selector `0xe43ec917`
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
    #[etherror(name = "CollateralBalanceIsZero", abi = "CollateralBalanceIsZero()")]
    pub struct CollateralBalanceIsZero;
    ///Custom Error type `EmptyBorrowAmounts` with signature `EmptyBorrowAmounts()` and selector `0x79646aaf`
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
    #[etherror(name = "EmptyBorrowAmounts", abi = "EmptyBorrowAmounts()")]
    pub struct EmptyBorrowAmounts;
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
    ///Custom Error type `InsufficientLiquidityForBorrow` with signature `InsufficientLiquidityForBorrow(uint256,uint256)` and selector `0xbcdafd92`
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
        name = "InsufficientLiquidityForBorrow",
        abi = "InsufficientLiquidityForBorrow(uint256,uint256)"
    )]
    pub struct InsufficientLiquidityForBorrow {
        pub amount_to_borrow: ::ethers::core::types::U256,
        pub available_liquidity: ::ethers::core::types::U256,
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
    ///Custom Error type `PoolNotFound` with signature `PoolNotFound(address)` and selector `0x6a34f98c`
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
    #[etherror(name = "PoolNotFound", abi = "PoolNotFound(address)")]
    pub struct PoolNotFound {
        pub key: ::ethers::core::types::Address,
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
    pub enum BorrowUtilsErrors {
        BorrowCapacityExceeded(BorrowCapacityExceeded),
        CollateralBalanceIsZero(CollateralBalanceIsZero),
        EmptyBorrowAmounts(EmptyBorrowAmounts),
        HealthFactorLowerThanLiquidationThreshold(
            HealthFactorLowerThanLiquidationThreshold,
        ),
        InsufficientLiquidityForBorrow(InsufficientLiquidityForBorrow),
        PoolIsFrozen(PoolIsFrozen),
        PoolIsInactive(PoolIsInactive),
        PoolIsNotBorrowing(PoolIsNotBorrowing),
        PoolIsPaused(PoolIsPaused),
        PoolNotFound(PoolNotFound),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BorrowUtilsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BorrowCapacityExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BorrowCapacityExceeded(decoded));
            }
            if let Ok(decoded) = <CollateralBalanceIsZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralBalanceIsZero(decoded));
            }
            if let Ok(decoded) = <EmptyBorrowAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyBorrowAmounts(decoded));
            }
            if let Ok(decoded) = <HealthFactorLowerThanLiquidationThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorLowerThanLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidityForBorrow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidityForBorrow(decoded));
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
            if let Ok(decoded) = <PoolNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolNotFound(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BorrowUtilsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BorrowCapacityExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyBorrowAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidityForBorrow(element) => {
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
                Self::PoolNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BorrowUtilsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BorrowCapacityExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CollateralBalanceIsZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyBorrowAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HealthFactorLowerThanLiquidationThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidityForBorrow as ::ethers::contract::EthError>::selector() => {
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
                _ if selector
                    == <PoolNotFound as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BorrowUtilsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BorrowCapacityExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptyBorrowAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidityForBorrow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolIsFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsInactive(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsNotBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolIsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BorrowUtilsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BorrowCapacityExceeded> for BorrowUtilsErrors {
        fn from(value: BorrowCapacityExceeded) -> Self {
            Self::BorrowCapacityExceeded(value)
        }
    }
    impl ::core::convert::From<CollateralBalanceIsZero> for BorrowUtilsErrors {
        fn from(value: CollateralBalanceIsZero) -> Self {
            Self::CollateralBalanceIsZero(value)
        }
    }
    impl ::core::convert::From<EmptyBorrowAmounts> for BorrowUtilsErrors {
        fn from(value: EmptyBorrowAmounts) -> Self {
            Self::EmptyBorrowAmounts(value)
        }
    }
    impl ::core::convert::From<HealthFactorLowerThanLiquidationThreshold>
    for BorrowUtilsErrors {
        fn from(value: HealthFactorLowerThanLiquidationThreshold) -> Self {
            Self::HealthFactorLowerThanLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityForBorrow> for BorrowUtilsErrors {
        fn from(value: InsufficientLiquidityForBorrow) -> Self {
            Self::InsufficientLiquidityForBorrow(value)
        }
    }
    impl ::core::convert::From<PoolIsFrozen> for BorrowUtilsErrors {
        fn from(value: PoolIsFrozen) -> Self {
            Self::PoolIsFrozen(value)
        }
    }
    impl ::core::convert::From<PoolIsInactive> for BorrowUtilsErrors {
        fn from(value: PoolIsInactive) -> Self {
            Self::PoolIsInactive(value)
        }
    }
    impl ::core::convert::From<PoolIsNotBorrowing> for BorrowUtilsErrors {
        fn from(value: PoolIsNotBorrowing) -> Self {
            Self::PoolIsNotBorrowing(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for BorrowUtilsErrors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
    impl ::core::convert::From<PoolNotFound> for BorrowUtilsErrors {
        fn from(value: PoolNotFound) -> Self {
            Self::PoolNotFound(value)
        }
    }
}
