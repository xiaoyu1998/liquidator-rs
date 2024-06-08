pub use config::*;
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
pub mod config {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_roleStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract RoleStore"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_dataStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract DataStore"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dataStore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dataStore"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract DataStore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("roleStore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roleStore"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract RoleStore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDebtMultiplierFactorForRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDebtMultiplierFactorForRedeem",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("multiplierFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "setHealthFactorCollateralRateThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setHealthFactorCollateralRateThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "setHealthFactorLiquidationThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setHealthFactorLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOracleDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOracleDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("precision"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolActive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("active"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolBorrowCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPoolBorrowCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPoolBorrowingEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolFeeFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolFeeFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("freeze"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolPaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolSupplyCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPoolSupplyCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supplyCapacity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolUsd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolUsd"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("usd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBorrowCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidBorrowCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowCapacity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "MaxValidBorrowCapacity",
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
                    ::std::borrow::ToOwned::to_owned("InvalidDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("MaxValidDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidFeeFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeeFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("MaxValidFeeFactor"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidSupplyCapacity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSupplyCapacity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supplyCapacity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "MaxValidSupplyCapacity",
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
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
    pub static CONFIG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Config<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Config<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Config<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Config<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Config<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Config)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Config<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONFIG_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `dataStore` (0x660d0d67) function
        pub fn data_store(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([102, 13, 13, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roleStore` (0x4a4a7b04) function
        pub fn role_store(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 74, 123, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDebtMultiplierFactorForRedeem` (0xfc979383) function
        pub fn set_debt_multiplier_factor_for_redeem(
            &self,
            multiplier_factor: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 151, 147, 131], multiplier_factor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDex` (0x75045691) function
        pub fn set_dex(
            &self,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
            dex: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [117, 4, 86, 145],
                    (underlying_asset_a, underlying_asset_b, dex),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHealthFactorCollateralRateThreshold` (0x7b2bc7f8) function
        pub fn set_health_factor_collateral_rate_threshold(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 43, 199, 248], (underlying_asset, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHealthFactorLiquidationThreshold` (0x49883194) function
        pub fn set_health_factor_liquidation_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 136, 49, 148], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOracle` (0x5c38eb3a) function
        pub fn set_oracle(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 56, 235, 58], (underlying_asset, oracle))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOracleDecimals` (0x221bd9fb) function
        pub fn set_oracle_decimals(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            precision: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 27, 217, 251], (underlying_asset, precision))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolActive` (0xe0e037df) function
        pub fn set_pool_active(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            active: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 224, 55, 223], (underlying_asset, active))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolBorrowCapacity` (0x4f296a40) function
        pub fn set_pool_borrow_capacity(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            borrow_capacity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 41, 106, 64], (underlying_asset, borrow_capacity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolBorrowingEnabled` (0xc8706f82) function
        pub fn set_pool_borrowing_enabled(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 112, 111, 130], (underlying_asset, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolDecimals` (0xec273390) function
        pub fn set_pool_decimals(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            decimals: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 39, 51, 144], (underlying_asset, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolFeeFactor` (0x287d0a33) function
        pub fn set_pool_fee_factor(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            fee_factor: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 125, 10, 51], (underlying_asset, fee_factor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolFrozen` (0x0249f1cf) function
        pub fn set_pool_frozen(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            freeze: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 73, 241, 207], (underlying_asset, freeze))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolPaused` (0xb900de11) function
        pub fn set_pool_paused(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            paused: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 0, 222, 17], (underlying_asset, paused))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolSupplyCapacity` (0x9e71ff10) function
        pub fn set_pool_supply_capacity(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            supply_capacity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 113, 255, 16], (underlying_asset, supply_capacity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolUsd` (0x8b9f4485) function
        pub fn set_pool_usd(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            usd: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 159, 68, 133], (underlying_asset, usd))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Config<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidBorrowCapacity` with signature `InvalidBorrowCapacity(uint256,uint256)` and selector `0x95c0bec8`
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
        name = "InvalidBorrowCapacity",
        abi = "InvalidBorrowCapacity(uint256,uint256)"
    )]
    pub struct InvalidBorrowCapacity {
        pub borrow_capacity: ::ethers::core::types::U256,
        pub max_valid_borrow_capacity: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidDecimals` with signature `InvalidDecimals(uint256,uint256)` and selector `0xc3ca0e37`
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
    #[etherror(name = "InvalidDecimals", abi = "InvalidDecimals(uint256,uint256)")]
    pub struct InvalidDecimals {
        pub decimals: ::ethers::core::types::U256,
        pub max_valid_decimals: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidFeeFactor` with signature `InvalidFeeFactor(uint256,uint256)` and selector `0x9d1ea70b`
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
    #[etherror(name = "InvalidFeeFactor", abi = "InvalidFeeFactor(uint256,uint256)")]
    pub struct InvalidFeeFactor {
        pub fee_factor: ::ethers::core::types::U256,
        pub max_valid_fee_factor: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidSupplyCapacity` with signature `InvalidSupplyCapacity(uint256,uint256)` and selector `0x178d1d2b`
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
        name = "InvalidSupplyCapacity",
        abi = "InvalidSupplyCapacity(uint256,uint256)"
    )]
    pub struct InvalidSupplyCapacity {
        pub supply_capacity: ::ethers::core::types::U256,
        pub max_valid_supply_capacity: ::ethers::core::types::U256,
    }
    ///Custom Error type `Unauthorized` with signature `Unauthorized(address,string)` and selector `0xa35b150b`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized(address,string)")]
    pub struct Unauthorized {
        pub msg_sender: ::ethers::core::types::Address,
        pub role: ::std::string::String,
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
    pub enum ConfigErrors {
        InvalidBorrowCapacity(InvalidBorrowCapacity),
        InvalidDecimals(InvalidDecimals),
        InvalidFeeFactor(InvalidFeeFactor),
        InvalidSupplyCapacity(InvalidSupplyCapacity),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConfigErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidBorrowCapacity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidBorrowCapacity(decoded));
            }
            if let Ok(decoded) = <InvalidDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDecimals(decoded));
            }
            if let Ok(decoded) = <InvalidFeeFactor as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeFactor(decoded));
            }
            if let Ok(decoded) = <InvalidSupplyCapacity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSupplyCapacity(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfigErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidBorrowCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSupplyCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ConfigErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidBorrowCapacity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeFactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSupplyCapacity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConfigErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidBorrowCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSupplyCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ConfigErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidBorrowCapacity> for ConfigErrors {
        fn from(value: InvalidBorrowCapacity) -> Self {
            Self::InvalidBorrowCapacity(value)
        }
    }
    impl ::core::convert::From<InvalidDecimals> for ConfigErrors {
        fn from(value: InvalidDecimals) -> Self {
            Self::InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidFeeFactor> for ConfigErrors {
        fn from(value: InvalidFeeFactor) -> Self {
            Self::InvalidFeeFactor(value)
        }
    }
    impl ::core::convert::From<InvalidSupplyCapacity> for ConfigErrors {
        fn from(value: InvalidSupplyCapacity) -> Self {
            Self::InvalidSupplyCapacity(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for ConfigErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    ///Container type for all input parameters for the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    #[ethcall(name = "dataStore", abi = "dataStore()")]
    pub struct DataStoreCall;
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `roleStore` function with signature `roleStore()` and selector `0x4a4a7b04`
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
    #[ethcall(name = "roleStore", abi = "roleStore()")]
    pub struct RoleStoreCall;
    ///Container type for all input parameters for the `setDebtMultiplierFactorForRedeem` function with signature `setDebtMultiplierFactorForRedeem(uint256)` and selector `0xfc979383`
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
        name = "setDebtMultiplierFactorForRedeem",
        abi = "setDebtMultiplierFactorForRedeem(uint256)"
    )]
    pub struct SetDebtMultiplierFactorForRedeemCall {
        pub multiplier_factor: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setDex` function with signature `setDex(address,address,address)` and selector `0x75045691`
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
    #[ethcall(name = "setDex", abi = "setDex(address,address,address)")]
    pub struct SetDexCall {
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
        pub dex: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setHealthFactorCollateralRateThreshold` function with signature `setHealthFactorCollateralRateThreshold(address,uint256)` and selector `0x7b2bc7f8`
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
        name = "setHealthFactorCollateralRateThreshold",
        abi = "setHealthFactorCollateralRateThreshold(address,uint256)"
    )]
    pub struct SetHealthFactorCollateralRateThresholdCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setHealthFactorLiquidationThreshold` function with signature `setHealthFactorLiquidationThreshold(uint256)` and selector `0x49883194`
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
        name = "setHealthFactorLiquidationThreshold",
        abi = "setHealthFactorLiquidationThreshold(uint256)"
    )]
    pub struct SetHealthFactorLiquidationThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setOracle` function with signature `setOracle(address,address)` and selector `0x5c38eb3a`
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
    #[ethcall(name = "setOracle", abi = "setOracle(address,address)")]
    pub struct SetOracleCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub oracle: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOracleDecimals` function with signature `setOracleDecimals(address,uint256)` and selector `0x221bd9fb`
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
    #[ethcall(name = "setOracleDecimals", abi = "setOracleDecimals(address,uint256)")]
    pub struct SetOracleDecimalsCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub precision: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolActive` function with signature `setPoolActive(address,bool)` and selector `0xe0e037df`
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
    #[ethcall(name = "setPoolActive", abi = "setPoolActive(address,bool)")]
    pub struct SetPoolActiveCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub active: bool,
    }
    ///Container type for all input parameters for the `setPoolBorrowCapacity` function with signature `setPoolBorrowCapacity(address,uint256)` and selector `0x4f296a40`
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
        name = "setPoolBorrowCapacity",
        abi = "setPoolBorrowCapacity(address,uint256)"
    )]
    pub struct SetPoolBorrowCapacityCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub borrow_capacity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolBorrowingEnabled` function with signature `setPoolBorrowingEnabled(address,bool)` and selector `0xc8706f82`
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
        name = "setPoolBorrowingEnabled",
        abi = "setPoolBorrowingEnabled(address,bool)"
    )]
    pub struct SetPoolBorrowingEnabledCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setPoolDecimals` function with signature `setPoolDecimals(address,uint256)` and selector `0xec273390`
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
    #[ethcall(name = "setPoolDecimals", abi = "setPoolDecimals(address,uint256)")]
    pub struct SetPoolDecimalsCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub decimals: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolFeeFactor` function with signature `setPoolFeeFactor(address,uint256)` and selector `0x287d0a33`
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
    #[ethcall(name = "setPoolFeeFactor", abi = "setPoolFeeFactor(address,uint256)")]
    pub struct SetPoolFeeFactorCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub fee_factor: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolFrozen` function with signature `setPoolFrozen(address,bool)` and selector `0x0249f1cf`
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
    #[ethcall(name = "setPoolFrozen", abi = "setPoolFrozen(address,bool)")]
    pub struct SetPoolFrozenCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub freeze: bool,
    }
    ///Container type for all input parameters for the `setPoolPaused` function with signature `setPoolPaused(address,bool)` and selector `0xb900de11`
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
    #[ethcall(name = "setPoolPaused", abi = "setPoolPaused(address,bool)")]
    pub struct SetPoolPausedCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setPoolSupplyCapacity` function with signature `setPoolSupplyCapacity(address,uint256)` and selector `0x9e71ff10`
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
        name = "setPoolSupplyCapacity",
        abi = "setPoolSupplyCapacity(address,uint256)"
    )]
    pub struct SetPoolSupplyCapacityCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub supply_capacity: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolUsd` function with signature `setPoolUsd(address,bool)` and selector `0x8b9f4485`
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
    #[ethcall(name = "setPoolUsd", abi = "setPoolUsd(address,bool)")]
    pub struct SetPoolUsdCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub usd: bool,
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
    pub enum ConfigCalls {
        DataStore(DataStoreCall),
        Multicall(MulticallCall),
        RoleStore(RoleStoreCall),
        SetDebtMultiplierFactorForRedeem(SetDebtMultiplierFactorForRedeemCall),
        SetDex(SetDexCall),
        SetHealthFactorCollateralRateThreshold(
            SetHealthFactorCollateralRateThresholdCall,
        ),
        SetHealthFactorLiquidationThreshold(SetHealthFactorLiquidationThresholdCall),
        SetOracle(SetOracleCall),
        SetOracleDecimals(SetOracleDecimalsCall),
        SetPoolActive(SetPoolActiveCall),
        SetPoolBorrowCapacity(SetPoolBorrowCapacityCall),
        SetPoolBorrowingEnabled(SetPoolBorrowingEnabledCall),
        SetPoolDecimals(SetPoolDecimalsCall),
        SetPoolFeeFactor(SetPoolFeeFactorCall),
        SetPoolFrozen(SetPoolFrozenCall),
        SetPoolPaused(SetPoolPausedCall),
        SetPoolSupplyCapacity(SetPoolSupplyCapacityCall),
        SetPoolUsd(SetPoolUsdCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConfigCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <SetDebtMultiplierFactorForRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDebtMultiplierFactorForRedeem(decoded));
            }
            if let Ok(decoded) = <SetDexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDex(decoded));
            }
            if let Ok(decoded) = <SetHealthFactorCollateralRateThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetHealthFactorCollateralRateThreshold(decoded));
            }
            if let Ok(decoded) = <SetHealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetHealthFactorLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <SetOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOracle(decoded));
            }
            if let Ok(decoded) = <SetOracleDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOracleDecimals(decoded));
            }
            if let Ok(decoded) = <SetPoolActiveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolActive(decoded));
            }
            if let Ok(decoded) = <SetPoolBorrowCapacityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolBorrowCapacity(decoded));
            }
            if let Ok(decoded) = <SetPoolBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolBorrowingEnabled(decoded));
            }
            if let Ok(decoded) = <SetPoolDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolDecimals(decoded));
            }
            if let Ok(decoded) = <SetPoolFeeFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolFeeFactor(decoded));
            }
            if let Ok(decoded) = <SetPoolFrozenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolFrozen(decoded));
            }
            if let Ok(decoded) = <SetPoolPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolPaused(decoded));
            }
            if let Ok(decoded) = <SetPoolSupplyCapacityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolSupplyCapacity(decoded));
            }
            if let Ok(decoded) = <SetPoolUsdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPoolUsd(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfigCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDebtMultiplierFactorForRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetHealthFactorCollateralRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOracleDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolBorrowCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolFeeFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolSupplyCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolUsd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConfigCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDebtMultiplierFactorForRedeem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDex(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHealthFactorCollateralRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOracleDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolBorrowCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolFeeFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolSupplyCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolUsd(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DataStoreCall> for ConfigCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for ConfigCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for ConfigCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<SetDebtMultiplierFactorForRedeemCall> for ConfigCalls {
        fn from(value: SetDebtMultiplierFactorForRedeemCall) -> Self {
            Self::SetDebtMultiplierFactorForRedeem(value)
        }
    }
    impl ::core::convert::From<SetDexCall> for ConfigCalls {
        fn from(value: SetDexCall) -> Self {
            Self::SetDex(value)
        }
    }
    impl ::core::convert::From<SetHealthFactorCollateralRateThresholdCall>
    for ConfigCalls {
        fn from(value: SetHealthFactorCollateralRateThresholdCall) -> Self {
            Self::SetHealthFactorCollateralRateThreshold(value)
        }
    }
    impl ::core::convert::From<SetHealthFactorLiquidationThresholdCall> for ConfigCalls {
        fn from(value: SetHealthFactorLiquidationThresholdCall) -> Self {
            Self::SetHealthFactorLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<SetOracleCall> for ConfigCalls {
        fn from(value: SetOracleCall) -> Self {
            Self::SetOracle(value)
        }
    }
    impl ::core::convert::From<SetOracleDecimalsCall> for ConfigCalls {
        fn from(value: SetOracleDecimalsCall) -> Self {
            Self::SetOracleDecimals(value)
        }
    }
    impl ::core::convert::From<SetPoolActiveCall> for ConfigCalls {
        fn from(value: SetPoolActiveCall) -> Self {
            Self::SetPoolActive(value)
        }
    }
    impl ::core::convert::From<SetPoolBorrowCapacityCall> for ConfigCalls {
        fn from(value: SetPoolBorrowCapacityCall) -> Self {
            Self::SetPoolBorrowCapacity(value)
        }
    }
    impl ::core::convert::From<SetPoolBorrowingEnabledCall> for ConfigCalls {
        fn from(value: SetPoolBorrowingEnabledCall) -> Self {
            Self::SetPoolBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<SetPoolDecimalsCall> for ConfigCalls {
        fn from(value: SetPoolDecimalsCall) -> Self {
            Self::SetPoolDecimals(value)
        }
    }
    impl ::core::convert::From<SetPoolFeeFactorCall> for ConfigCalls {
        fn from(value: SetPoolFeeFactorCall) -> Self {
            Self::SetPoolFeeFactor(value)
        }
    }
    impl ::core::convert::From<SetPoolFrozenCall> for ConfigCalls {
        fn from(value: SetPoolFrozenCall) -> Self {
            Self::SetPoolFrozen(value)
        }
    }
    impl ::core::convert::From<SetPoolPausedCall> for ConfigCalls {
        fn from(value: SetPoolPausedCall) -> Self {
            Self::SetPoolPaused(value)
        }
    }
    impl ::core::convert::From<SetPoolSupplyCapacityCall> for ConfigCalls {
        fn from(value: SetPoolSupplyCapacityCall) -> Self {
            Self::SetPoolSupplyCapacity(value)
        }
    }
    impl ::core::convert::From<SetPoolUsdCall> for ConfigCalls {
        fn from(value: SetPoolUsdCall) -> Self {
            Self::SetPoolUsd(value)
        }
    }
    ///Container type for all return fields from the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    pub struct DataStoreReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `roleStore` function with signature `roleStore()` and selector `0x4a4a7b04`
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
    pub struct RoleStoreReturn(pub ::ethers::core::types::Address);
}
