pub use config_store_utils::*;
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
pub mod config_store_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getDebtMultiplierFactorForRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDebtMultiplierFactorForRedeem",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getHealthFactorCollateralRateThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getHealthFactorCollateralRateThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getHealthFactorLiquidationThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getHealthFactorLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolActive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolBorrowCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPoolBorrowCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPoolBorrowingEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolFeeFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolFeeFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolSupplyCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPoolSupplyCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolUsd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolUsd"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONFIGSTOREUTILS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct ConfigStoreUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConfigStoreUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConfigStoreUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConfigStoreUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConfigStoreUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConfigStoreUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConfigStoreUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONFIGSTOREUTILS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getDebtMultiplierFactorForRedeem` (0x349c1a72) function
        pub fn get_debt_multiplier_factor_for_redeem(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 156, 26, 114], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthFactorCollateralRateThreshold` (0xef5cb53d) function
        pub fn get_health_factor_collateral_rate_threshold(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 92, 181, 61], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthFactorLiquidationThreshold` (0xb3b8c8e5) function
        pub fn get_health_factor_liquidation_threshold(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 184, 200, 229], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolActive` (0xa168153f) function
        pub fn get_pool_active(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([161, 104, 21, 63], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolBorrowCapacity` (0x48b6b66c) function
        pub fn get_pool_borrow_capacity(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([72, 182, 182, 108], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolBorrowingEnabled` (0xf9d2f0ef) function
        pub fn get_pool_borrowing_enabled(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 210, 240, 239], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolDecimals` (0x163c352f) function
        pub fn get_pool_decimals(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 60, 53, 47], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolFeeFactor` (0xcc4313e5) function
        pub fn get_pool_fee_factor(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([204, 67, 19, 229], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolFrozen` (0x37a0c732) function
        pub fn get_pool_frozen(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 160, 199, 50], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolPaused` (0x4c70b718) function
        pub fn get_pool_paused(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 112, 183, 24], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolSupplyCapacity` (0x3756a9bd) function
        pub fn get_pool_supply_capacity(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 86, 169, 189], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolUsd` (0xb2d2ba4c) function
        pub fn get_pool_usd(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([178, 210, 186, 76], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConfigStoreUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getDebtMultiplierFactorForRedeem` function with signature `getDebtMultiplierFactorForRedeem(address)` and selector `0x349c1a72`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getDebtMultiplierFactorForRedeem",
        abi = "getDebtMultiplierFactorForRedeem(address)"
    )]
    pub struct GetDebtMultiplierFactorForRedeemCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getHealthFactorCollateralRateThreshold` function with signature `getHealthFactorCollateralRateThreshold(address,address)` and selector `0xef5cb53d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getHealthFactorCollateralRateThreshold",
        abi = "getHealthFactorCollateralRateThreshold(address,address)"
    )]
    pub struct GetHealthFactorCollateralRateThresholdCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getHealthFactorLiquidationThreshold` function with signature `getHealthFactorLiquidationThreshold(address)` and selector `0xb3b8c8e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getHealthFactorLiquidationThreshold",
        abi = "getHealthFactorLiquidationThreshold(address)"
    )]
    pub struct GetHealthFactorLiquidationThresholdCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolActive` function with signature `getPoolActive(address,address)` and selector `0xa168153f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolActive", abi = "getPoolActive(address,address)")]
    pub struct GetPoolActiveCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolBorrowCapacity` function with signature `getPoolBorrowCapacity(address,address)` and selector `0x48b6b66c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getPoolBorrowCapacity",
        abi = "getPoolBorrowCapacity(address,address)"
    )]
    pub struct GetPoolBorrowCapacityCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolBorrowingEnabled` function with signature `getPoolBorrowingEnabled(address,address)` and selector `0xf9d2f0ef`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getPoolBorrowingEnabled",
        abi = "getPoolBorrowingEnabled(address,address)"
    )]
    pub struct GetPoolBorrowingEnabledCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolDecimals` function with signature `getPoolDecimals(address,address)` and selector `0x163c352f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolDecimals", abi = "getPoolDecimals(address,address)")]
    pub struct GetPoolDecimalsCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolFeeFactor` function with signature `getPoolFeeFactor(address,address)` and selector `0xcc4313e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolFeeFactor", abi = "getPoolFeeFactor(address,address)")]
    pub struct GetPoolFeeFactorCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolFrozen` function with signature `getPoolFrozen(address,address)` and selector `0x37a0c732`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolFrozen", abi = "getPoolFrozen(address,address)")]
    pub struct GetPoolFrozenCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolPaused` function with signature `getPoolPaused(address,address)` and selector `0x4c70b718`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolPaused", abi = "getPoolPaused(address,address)")]
    pub struct GetPoolPausedCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolSupplyCapacity` function with signature `getPoolSupplyCapacity(address,address)` and selector `0x3756a9bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getPoolSupplyCapacity",
        abi = "getPoolSupplyCapacity(address,address)"
    )]
    pub struct GetPoolSupplyCapacityCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolUsd` function with signature `getPoolUsd(address,address)` and selector `0xb2d2ba4c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolUsd", abi = "getPoolUsd(address,address)")]
    pub struct GetPoolUsdCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
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
    pub enum ConfigStoreUtilsCalls {
        GetDebtMultiplierFactorForRedeem(GetDebtMultiplierFactorForRedeemCall),
        GetHealthFactorCollateralRateThreshold(
            GetHealthFactorCollateralRateThresholdCall,
        ),
        GetHealthFactorLiquidationThreshold(GetHealthFactorLiquidationThresholdCall),
        GetPoolActive(GetPoolActiveCall),
        GetPoolBorrowCapacity(GetPoolBorrowCapacityCall),
        GetPoolBorrowingEnabled(GetPoolBorrowingEnabledCall),
        GetPoolDecimals(GetPoolDecimalsCall),
        GetPoolFeeFactor(GetPoolFeeFactorCall),
        GetPoolFrozen(GetPoolFrozenCall),
        GetPoolPaused(GetPoolPausedCall),
        GetPoolSupplyCapacity(GetPoolSupplyCapacityCall),
        GetPoolUsd(GetPoolUsdCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConfigStoreUtilsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetDebtMultiplierFactorForRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDebtMultiplierFactorForRedeem(decoded));
            }
            if let Ok(decoded) = <GetHealthFactorCollateralRateThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealthFactorCollateralRateThreshold(decoded));
            }
            if let Ok(decoded) = <GetHealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealthFactorLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <GetPoolActiveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolActive(decoded));
            }
            if let Ok(decoded) = <GetPoolBorrowCapacityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolBorrowCapacity(decoded));
            }
            if let Ok(decoded) = <GetPoolBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolBorrowingEnabled(decoded));
            }
            if let Ok(decoded) = <GetPoolDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolDecimals(decoded));
            }
            if let Ok(decoded) = <GetPoolFeeFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolFeeFactor(decoded));
            }
            if let Ok(decoded) = <GetPoolFrozenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolFrozen(decoded));
            }
            if let Ok(decoded) = <GetPoolPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolPaused(decoded));
            }
            if let Ok(decoded) = <GetPoolSupplyCapacityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolSupplyCapacity(decoded));
            }
            if let Ok(decoded) = <GetPoolUsdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolUsd(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfigStoreUtilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetDebtMultiplierFactorForRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthFactorCollateralRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolBorrowCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolFeeFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolSupplyCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolUsd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConfigStoreUtilsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetDebtMultiplierFactorForRedeem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHealthFactorCollateralRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPoolActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolBorrowCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPoolBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPoolDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolFeeFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolSupplyCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPoolUsd(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetDebtMultiplierFactorForRedeemCall>
    for ConfigStoreUtilsCalls {
        fn from(value: GetDebtMultiplierFactorForRedeemCall) -> Self {
            Self::GetDebtMultiplierFactorForRedeem(value)
        }
    }
    impl ::core::convert::From<GetHealthFactorCollateralRateThresholdCall>
    for ConfigStoreUtilsCalls {
        fn from(value: GetHealthFactorCollateralRateThresholdCall) -> Self {
            Self::GetHealthFactorCollateralRateThreshold(value)
        }
    }
    impl ::core::convert::From<GetHealthFactorLiquidationThresholdCall>
    for ConfigStoreUtilsCalls {
        fn from(value: GetHealthFactorLiquidationThresholdCall) -> Self {
            Self::GetHealthFactorLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<GetPoolActiveCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolActiveCall) -> Self {
            Self::GetPoolActive(value)
        }
    }
    impl ::core::convert::From<GetPoolBorrowCapacityCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolBorrowCapacityCall) -> Self {
            Self::GetPoolBorrowCapacity(value)
        }
    }
    impl ::core::convert::From<GetPoolBorrowingEnabledCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolBorrowingEnabledCall) -> Self {
            Self::GetPoolBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<GetPoolDecimalsCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolDecimalsCall) -> Self {
            Self::GetPoolDecimals(value)
        }
    }
    impl ::core::convert::From<GetPoolFeeFactorCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolFeeFactorCall) -> Self {
            Self::GetPoolFeeFactor(value)
        }
    }
    impl ::core::convert::From<GetPoolFrozenCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolFrozenCall) -> Self {
            Self::GetPoolFrozen(value)
        }
    }
    impl ::core::convert::From<GetPoolPausedCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolPausedCall) -> Self {
            Self::GetPoolPaused(value)
        }
    }
    impl ::core::convert::From<GetPoolSupplyCapacityCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolSupplyCapacityCall) -> Self {
            Self::GetPoolSupplyCapacity(value)
        }
    }
    impl ::core::convert::From<GetPoolUsdCall> for ConfigStoreUtilsCalls {
        fn from(value: GetPoolUsdCall) -> Self {
            Self::GetPoolUsd(value)
        }
    }
    ///Container type for all return fields from the `getDebtMultiplierFactorForRedeem` function with signature `getDebtMultiplierFactorForRedeem(address)` and selector `0x349c1a72`
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
    pub struct GetDebtMultiplierFactorForRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getHealthFactorCollateralRateThreshold` function with signature `getHealthFactorCollateralRateThreshold(address,address)` and selector `0xef5cb53d`
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
    pub struct GetHealthFactorCollateralRateThresholdReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getHealthFactorLiquidationThreshold` function with signature `getHealthFactorLiquidationThreshold(address)` and selector `0xb3b8c8e5`
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
    pub struct GetHealthFactorLiquidationThresholdReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPoolActive` function with signature `getPoolActive(address,address)` and selector `0xa168153f`
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
    pub struct GetPoolActiveReturn(pub bool);
    ///Container type for all return fields from the `getPoolBorrowCapacity` function with signature `getPoolBorrowCapacity(address,address)` and selector `0x48b6b66c`
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
    pub struct GetPoolBorrowCapacityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPoolBorrowingEnabled` function with signature `getPoolBorrowingEnabled(address,address)` and selector `0xf9d2f0ef`
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
    pub struct GetPoolBorrowingEnabledReturn(pub bool);
    ///Container type for all return fields from the `getPoolDecimals` function with signature `getPoolDecimals(address,address)` and selector `0x163c352f`
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
    pub struct GetPoolDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPoolFeeFactor` function with signature `getPoolFeeFactor(address,address)` and selector `0xcc4313e5`
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
    pub struct GetPoolFeeFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPoolFrozen` function with signature `getPoolFrozen(address,address)` and selector `0x37a0c732`
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
    pub struct GetPoolFrozenReturn(pub bool);
    ///Container type for all return fields from the `getPoolPaused` function with signature `getPoolPaused(address,address)` and selector `0x4c70b718`
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
    pub struct GetPoolPausedReturn(pub bool);
    ///Container type for all return fields from the `getPoolSupplyCapacity` function with signature `getPoolSupplyCapacity(address,address)` and selector `0x3756a9bd`
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
    pub struct GetPoolSupplyCapacityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPoolUsd` function with signature `getPoolUsd(address,address)` and selector `0xb2d2ba4c`
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
    pub struct GetPoolUsdReturn(pub bool);
}
