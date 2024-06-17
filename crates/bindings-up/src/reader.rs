pub use reader::*;
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
pub mod reader {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getDexPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDexPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDexPoolFeeAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDexPoolFeeAmount",
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
                    ::std::borrow::ToOwned::to_owned("getDexPoolSwapConstantFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDexPoolSwapConstantFee",
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("getLiquidationHealthFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationHealthFactor",
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationHealthFactor",
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderPositionUtils.GetLiquidationHealthFactor",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityAndDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidityAndDebt",
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
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetLiquidityAndDebt",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityAndDebts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidityAndDebts",
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetLiquidityAndDebt[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarginAndSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMarginAndSupply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetMarginAndSupply",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarginsAndSupplies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMarginsAndSupplies",
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetMarginAndSupply[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxAmountToRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMaxAmountToRedeem",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.Props"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetPoolInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPools"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.Props[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolsInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolsInfo"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetPoolInfo[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolsPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolsPrice"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderUtils.GetPoolPrice[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Position.Props"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositionInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositionInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderPositionUtils.GetPositionInfo",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Position.Props[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositionsInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositionsInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReaderPositionUtils.GetPositionInfo[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPrice"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
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
    pub static READER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Reader<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Reader<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Reader<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Reader<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Reader<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Reader)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Reader<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    READER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getDexPool` (0xfb59fa82) function
        pub fn get_dex_pool(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [251, 89, 250, 130],
                    (data_store, underlying_asset_a, underlying_asset_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDexPoolFeeAmount` (0xa2096669) function
        pub fn get_dex_pool_fee_amount(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [162, 9, 102, 105],
                    (data_store, underlying_asset_a, underlying_asset_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDexPoolSwapConstantFee` (0x14cfff29) function
        pub fn get_dex_pool_swap_constant_fee(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [20, 207, 255, 41],
                    (data_store, underlying_asset_a, underlying_asset_b, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationHealthFactor` (0x4d4de17d) function
        pub fn get_liquidation_health_factor(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 77, 225, 125], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationHealthFactor` (0x6d7c4892) function
        pub fn get_liquidation_health_factor_with_account(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, GetLiquidationHealthFactor> {
            self.0
                .method_hash([109, 124, 72, 146], (data_store, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityAndDebt` (0xe578f18a) function
        pub fn get_liquidity_and_debt(
            &self,
            data_store: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, GetLiquidityAndDebt> {
            self.0
                .method_hash([229, 120, 241, 138], (data_store, pool_key, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityAndDebts` (0x67e2cac8) function
        pub fn get_liquidity_and_debts(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<GetLiquidityAndDebt>,
        > {
            self.0
                .method_hash([103, 226, 202, 200], (data_store, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarginAndSupply` (0x5f84a6da) function
        pub fn get_margin_and_supply(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, GetMarginAndSupply> {
            self.0
                .method_hash([95, 132, 166, 218], (data_store, account, pool_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarginsAndSupplies` (0xc5e67a30) function
        pub fn get_margins_and_supplies(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<GetMarginAndSupply>,
        > {
            self.0
                .method_hash([197, 230, 122, 48], (data_store, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxAmountToRedeem` (0x3c6b4c1d) function
        pub fn get_max_amount_to_redeem(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 107, 76, 29], (data_store, underlying_asset, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x531aa03e) function
        pub fn get_pool(
            &self,
            data_store: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, PoolProps> {
            self.0
                .method_hash([83, 26, 160, 62], (data_store, pool_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolInfo` (0x486e51f4) function
        pub fn get_pool_info(
            &self,
            data_store: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, GetPoolInfo> {
            self.0
                .method_hash([72, 110, 81, 244], (data_store, pool_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPools` (0x5c39f467) function
        pub fn get_pools(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PoolProps>> {
            self.0
                .method_hash([92, 57, 244, 103], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolsInfo` (0x1a308175) function
        pub fn get_pools_info(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<GetPoolInfo>,
        > {
            self.0
                .method_hash([26, 48, 129, 117], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolsPrice` (0xf91c5877) function
        pub fn get_pools_price(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<GetPoolPrice>,
        > {
            self.0
                .method_hash([249, 28, 88, 119], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPosition` (0x713390f5) function
        pub fn get_position(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, PositionProps> {
            self.0
                .method_hash([113, 51, 144, 245], (data_store, account, pool_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositionInfo` (0xaf0efff8) function
        pub fn get_position_info(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            pool_key: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, GetPositionInfo> {
            self.0
                .method_hash([175, 14, 255, 248], (data_store, account, pool_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositions` (0x739118a4) function
        pub fn get_positions(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<PositionProps>,
        > {
            self.0
                .method_hash([115, 145, 24, 164], (data_store, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositionsInfo` (0x71c7d82b) function
        pub fn get_positions_info(
            &self,
            data_store: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<GetPositionInfo>,
        > {
            self.0
                .method_hash([113, 199, 216, 43], (data_store, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPrice` (0xac41865a) function
        pub fn get_price(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([172, 65, 134, 90], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Reader<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `getDexPool` function with signature `getDexPool(address,address,address)` and selector `0xfb59fa82`
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
    #[ethcall(name = "getDexPool", abi = "getDexPool(address,address,address)")]
    pub struct GetDexPoolCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDexPoolFeeAmount` function with signature `getDexPoolFeeAmount(address,address,address)` and selector `0xa2096669`
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
        name = "getDexPoolFeeAmount",
        abi = "getDexPoolFeeAmount(address,address,address)"
    )]
    pub struct GetDexPoolFeeAmountCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDexPoolSwapConstantFee` function with signature `getDexPoolSwapConstantFee(address,address,address,uint256)` and selector `0x14cfff29`
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
        name = "getDexPoolSwapConstantFee",
        abi = "getDexPoolSwapConstantFee(address,address,address,uint256)"
    )]
    pub struct GetDexPoolSwapConstantFeeCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLiquidationHealthFactor` function with signature `getLiquidationHealthFactor(address)` and selector `0x4d4de17d`
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
        name = "getLiquidationHealthFactor",
        abi = "getLiquidationHealthFactor(address)"
    )]
    pub struct GetLiquidationHealthFactorCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidationHealthFactor` function with signature `getLiquidationHealthFactor(address,address)` and selector `0x6d7c4892`
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
        name = "getLiquidationHealthFactor",
        abi = "getLiquidationHealthFactor(address,address)"
    )]
    pub struct GetLiquidationHealthFactorWithAccountCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidityAndDebt` function with signature `getLiquidityAndDebt(address,address,address)` and selector `0xe578f18a`
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
        name = "getLiquidityAndDebt",
        abi = "getLiquidityAndDebt(address,address,address)"
    )]
    pub struct GetLiquidityAndDebtCall {
        pub data_store: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidityAndDebts` function with signature `getLiquidityAndDebts(address,address)` and selector `0x67e2cac8`
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
        name = "getLiquidityAndDebts",
        abi = "getLiquidityAndDebts(address,address)"
    )]
    pub struct GetLiquidityAndDebtsCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMarginAndSupply` function with signature `getMarginAndSupply(address,address,address)` and selector `0x5f84a6da`
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
        name = "getMarginAndSupply",
        abi = "getMarginAndSupply(address,address,address)"
    )]
    pub struct GetMarginAndSupplyCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMarginsAndSupplies` function with signature `getMarginsAndSupplies(address,address)` and selector `0xc5e67a30`
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
        name = "getMarginsAndSupplies",
        abi = "getMarginsAndSupplies(address,address)"
    )]
    pub struct GetMarginsAndSuppliesCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMaxAmountToRedeem` function with signature `getMaxAmountToRedeem(address,address,address)` and selector `0x3c6b4c1d`
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
        name = "getMaxAmountToRedeem",
        abi = "getMaxAmountToRedeem(address,address,address)"
    )]
    pub struct GetMaxAmountToRedeemCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address)` and selector `0x531aa03e`
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
    #[ethcall(name = "getPool", abi = "getPool(address,address)")]
    pub struct GetPoolCall {
        pub data_store: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolInfo` function with signature `getPoolInfo(address,address)` and selector `0x486e51f4`
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
    #[ethcall(name = "getPoolInfo", abi = "getPoolInfo(address,address)")]
    pub struct GetPoolInfoCall {
        pub data_store: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPools` function with signature `getPools(address)` and selector `0x5c39f467`
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
    #[ethcall(name = "getPools", abi = "getPools(address)")]
    pub struct GetPoolsCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolsInfo` function with signature `getPoolsInfo(address)` and selector `0x1a308175`
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
    #[ethcall(name = "getPoolsInfo", abi = "getPoolsInfo(address)")]
    pub struct GetPoolsInfoCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPoolsPrice` function with signature `getPoolsPrice(address)` and selector `0xf91c5877`
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
    #[ethcall(name = "getPoolsPrice", abi = "getPoolsPrice(address)")]
    pub struct GetPoolsPriceCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPosition` function with signature `getPosition(address,address,address)` and selector `0x713390f5`
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
    #[ethcall(name = "getPosition", abi = "getPosition(address,address,address)")]
    pub struct GetPositionCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPositionInfo` function with signature `getPositionInfo(address,address,address)` and selector `0xaf0efff8`
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
        name = "getPositionInfo",
        abi = "getPositionInfo(address,address,address)"
    )]
    pub struct GetPositionInfoCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPositions` function with signature `getPositions(address,address)` and selector `0x739118a4`
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
    #[ethcall(name = "getPositions", abi = "getPositions(address,address)")]
    pub struct GetPositionsCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPositionsInfo` function with signature `getPositionsInfo(address,address)` and selector `0x71c7d82b`
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
    #[ethcall(name = "getPositionsInfo", abi = "getPositionsInfo(address,address)")]
    pub struct GetPositionsInfoCall {
        pub data_store: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPrice` function with signature `getPrice(address,address)` and selector `0xac41865a`
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
    #[ethcall(name = "getPrice", abi = "getPrice(address,address)")]
    pub struct GetPriceCall {
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
    pub enum ReaderCalls {
        GetDexPool(GetDexPoolCall),
        GetDexPoolFeeAmount(GetDexPoolFeeAmountCall),
        GetDexPoolSwapConstantFee(GetDexPoolSwapConstantFeeCall),
        GetLiquidationHealthFactor(GetLiquidationHealthFactorCall),
        GetLiquidationHealthFactorWithAccount(GetLiquidationHealthFactorWithAccountCall),
        GetLiquidityAndDebt(GetLiquidityAndDebtCall),
        GetLiquidityAndDebts(GetLiquidityAndDebtsCall),
        GetMarginAndSupply(GetMarginAndSupplyCall),
        GetMarginsAndSupplies(GetMarginsAndSuppliesCall),
        GetMaxAmountToRedeem(GetMaxAmountToRedeemCall),
        GetPool(GetPoolCall),
        GetPoolInfo(GetPoolInfoCall),
        GetPools(GetPoolsCall),
        GetPoolsInfo(GetPoolsInfoCall),
        GetPoolsPrice(GetPoolsPriceCall),
        GetPosition(GetPositionCall),
        GetPositionInfo(GetPositionInfoCall),
        GetPositions(GetPositionsCall),
        GetPositionsInfo(GetPositionsInfoCall),
        GetPrice(GetPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReaderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetDexPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPool(decoded));
            }
            if let Ok(decoded) = <GetDexPoolFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPoolFeeAmount(decoded));
            }
            if let Ok(decoded) = <GetDexPoolSwapConstantFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPoolSwapConstantFee(decoded));
            }
            if let Ok(decoded) = <GetLiquidationHealthFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidationHealthFactor(decoded));
            }
            if let Ok(decoded) = <GetLiquidationHealthFactorWithAccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidationHealthFactorWithAccount(decoded));
            }
            if let Ok(decoded) = <GetLiquidityAndDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidityAndDebt(decoded));
            }
            if let Ok(decoded) = <GetLiquidityAndDebtsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidityAndDebts(decoded));
            }
            if let Ok(decoded) = <GetMarginAndSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMarginAndSupply(decoded));
            }
            if let Ok(decoded) = <GetMarginsAndSuppliesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMarginsAndSupplies(decoded));
            }
            if let Ok(decoded) = <GetMaxAmountToRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMaxAmountToRedeem(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetPoolInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolInfo(decoded));
            }
            if let Ok(decoded) = <GetPoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPools(decoded));
            }
            if let Ok(decoded) = <GetPoolsInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolsInfo(decoded));
            }
            if let Ok(decoded) = <GetPoolsPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolsPrice(decoded));
            }
            if let Ok(decoded) = <GetPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPosition(decoded));
            }
            if let Ok(decoded) = <GetPositionInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositionInfo(decoded));
            }
            if let Ok(decoded) = <GetPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositions(decoded));
            }
            if let Ok(decoded) = <GetPositionsInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositionsInfo(decoded));
            }
            if let Ok(decoded) = <GetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReaderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetDexPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDexPoolFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDexPoolSwapConstantFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationHealthFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationHealthFactorWithAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityAndDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityAndDebts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarginAndSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarginsAndSupplies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxAmountToRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPools(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolsInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolsPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositionInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositionsInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReaderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetDexPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDexPoolFeeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDexPoolSwapConstantFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidationHealthFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidationHealthFactorWithAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidityAndDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidityAndDebts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMarginAndSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMarginsAndSupplies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxAmountToRedeem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolsInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolsPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionsInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetDexPoolCall> for ReaderCalls {
        fn from(value: GetDexPoolCall) -> Self {
            Self::GetDexPool(value)
        }
    }
    impl ::core::convert::From<GetDexPoolFeeAmountCall> for ReaderCalls {
        fn from(value: GetDexPoolFeeAmountCall) -> Self {
            Self::GetDexPoolFeeAmount(value)
        }
    }
    impl ::core::convert::From<GetDexPoolSwapConstantFeeCall> for ReaderCalls {
        fn from(value: GetDexPoolSwapConstantFeeCall) -> Self {
            Self::GetDexPoolSwapConstantFee(value)
        }
    }
    impl ::core::convert::From<GetLiquidationHealthFactorCall> for ReaderCalls {
        fn from(value: GetLiquidationHealthFactorCall) -> Self {
            Self::GetLiquidationHealthFactor(value)
        }
    }
    impl ::core::convert::From<GetLiquidationHealthFactorWithAccountCall>
    for ReaderCalls {
        fn from(value: GetLiquidationHealthFactorWithAccountCall) -> Self {
            Self::GetLiquidationHealthFactorWithAccount(value)
        }
    }
    impl ::core::convert::From<GetLiquidityAndDebtCall> for ReaderCalls {
        fn from(value: GetLiquidityAndDebtCall) -> Self {
            Self::GetLiquidityAndDebt(value)
        }
    }
    impl ::core::convert::From<GetLiquidityAndDebtsCall> for ReaderCalls {
        fn from(value: GetLiquidityAndDebtsCall) -> Self {
            Self::GetLiquidityAndDebts(value)
        }
    }
    impl ::core::convert::From<GetMarginAndSupplyCall> for ReaderCalls {
        fn from(value: GetMarginAndSupplyCall) -> Self {
            Self::GetMarginAndSupply(value)
        }
    }
    impl ::core::convert::From<GetMarginsAndSuppliesCall> for ReaderCalls {
        fn from(value: GetMarginsAndSuppliesCall) -> Self {
            Self::GetMarginsAndSupplies(value)
        }
    }
    impl ::core::convert::From<GetMaxAmountToRedeemCall> for ReaderCalls {
        fn from(value: GetMaxAmountToRedeemCall) -> Self {
            Self::GetMaxAmountToRedeem(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for ReaderCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolInfoCall> for ReaderCalls {
        fn from(value: GetPoolInfoCall) -> Self {
            Self::GetPoolInfo(value)
        }
    }
    impl ::core::convert::From<GetPoolsCall> for ReaderCalls {
        fn from(value: GetPoolsCall) -> Self {
            Self::GetPools(value)
        }
    }
    impl ::core::convert::From<GetPoolsInfoCall> for ReaderCalls {
        fn from(value: GetPoolsInfoCall) -> Self {
            Self::GetPoolsInfo(value)
        }
    }
    impl ::core::convert::From<GetPoolsPriceCall> for ReaderCalls {
        fn from(value: GetPoolsPriceCall) -> Self {
            Self::GetPoolsPrice(value)
        }
    }
    impl ::core::convert::From<GetPositionCall> for ReaderCalls {
        fn from(value: GetPositionCall) -> Self {
            Self::GetPosition(value)
        }
    }
    impl ::core::convert::From<GetPositionInfoCall> for ReaderCalls {
        fn from(value: GetPositionInfoCall) -> Self {
            Self::GetPositionInfo(value)
        }
    }
    impl ::core::convert::From<GetPositionsCall> for ReaderCalls {
        fn from(value: GetPositionsCall) -> Self {
            Self::GetPositions(value)
        }
    }
    impl ::core::convert::From<GetPositionsInfoCall> for ReaderCalls {
        fn from(value: GetPositionsInfoCall) -> Self {
            Self::GetPositionsInfo(value)
        }
    }
    impl ::core::convert::From<GetPriceCall> for ReaderCalls {
        fn from(value: GetPriceCall) -> Self {
            Self::GetPrice(value)
        }
    }
    ///Container type for all return fields from the `getDexPool` function with signature `getDexPool(address,address,address)` and selector `0xfb59fa82`
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
    pub struct GetDexPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDexPoolFeeAmount` function with signature `getDexPoolFeeAmount(address,address,address)` and selector `0xa2096669`
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
    pub struct GetDexPoolFeeAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDexPoolSwapConstantFee` function with signature `getDexPoolSwapConstantFee(address,address,address,uint256)` and selector `0x14cfff29`
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
    pub struct GetDexPoolSwapConstantFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidationHealthFactor` function with signature `getLiquidationHealthFactor(address)` and selector `0x4d4de17d`
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
    pub struct GetLiquidationHealthFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidationHealthFactor` function with signature `getLiquidationHealthFactor(address,address)` and selector `0x6d7c4892`
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
    pub struct GetLiquidationHealthFactorWithAccountReturn(
        pub GetLiquidationHealthFactor,
    );
    ///Container type for all return fields from the `getLiquidityAndDebt` function with signature `getLiquidityAndDebt(address,address,address)` and selector `0xe578f18a`
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
    pub struct GetLiquidityAndDebtReturn(pub GetLiquidityAndDebt);
    ///Container type for all return fields from the `getLiquidityAndDebts` function with signature `getLiquidityAndDebts(address,address)` and selector `0x67e2cac8`
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
    pub struct GetLiquidityAndDebtsReturn(pub ::std::vec::Vec<GetLiquidityAndDebt>);
    ///Container type for all return fields from the `getMarginAndSupply` function with signature `getMarginAndSupply(address,address,address)` and selector `0x5f84a6da`
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
    pub struct GetMarginAndSupplyReturn(pub GetMarginAndSupply);
    ///Container type for all return fields from the `getMarginsAndSupplies` function with signature `getMarginsAndSupplies(address,address)` and selector `0xc5e67a30`
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
    pub struct GetMarginsAndSuppliesReturn(pub ::std::vec::Vec<GetMarginAndSupply>);
    ///Container type for all return fields from the `getMaxAmountToRedeem` function with signature `getMaxAmountToRedeem(address,address,address)` and selector `0x3c6b4c1d`
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
    pub struct GetMaxAmountToRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address)` and selector `0x531aa03e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetPoolReturn(pub PoolProps);
    ///Container type for all return fields from the `getPoolInfo` function with signature `getPoolInfo(address,address)` and selector `0x486e51f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetPoolInfoReturn(pub GetPoolInfo);
    ///Container type for all return fields from the `getPools` function with signature `getPools(address)` and selector `0x5c39f467`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetPoolsReturn(pub ::std::vec::Vec<PoolProps>);
    ///Container type for all return fields from the `getPoolsInfo` function with signature `getPoolsInfo(address)` and selector `0x1a308175`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetPoolsInfoReturn(pub ::std::vec::Vec<GetPoolInfo>);
    ///Container type for all return fields from the `getPoolsPrice` function with signature `getPoolsPrice(address)` and selector `0xf91c5877`
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
    pub struct GetPoolsPriceReturn(pub ::std::vec::Vec<GetPoolPrice>);
    ///Container type for all return fields from the `getPosition` function with signature `getPosition(address,address,address)` and selector `0x713390f5`
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
    pub struct GetPositionReturn(pub PositionProps);
    ///Container type for all return fields from the `getPositionInfo` function with signature `getPositionInfo(address,address,address)` and selector `0xaf0efff8`
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
    pub struct GetPositionInfoReturn(pub GetPositionInfo);
    ///Container type for all return fields from the `getPositions` function with signature `getPositions(address,address)` and selector `0x739118a4`
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
    pub struct GetPositionsReturn(pub ::std::vec::Vec<PositionProps>);
    ///Container type for all return fields from the `getPositionsInfo` function with signature `getPositionsInfo(address,address)` and selector `0x71c7d82b`
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
    pub struct GetPositionsInfoReturn(pub ::std::vec::Vec<GetPositionInfo>);
    ///Container type for all return fields from the `getPrice` function with signature `getPrice(address,address)` and selector `0xac41865a`
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
    pub struct GetPriceReturn(pub ::ethers::core::types::U256);
    ///`GetPositionInfo(address,address,uint256,int256,int256,uint256,uint256,int256,uint256,uint256)`
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
    pub struct GetPositionInfo {
        pub account: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
        pub position_type: ::ethers::core::types::U256,
        pub equity: ::ethers::core::types::I256,
        pub equity_usd: ::ethers::core::types::I256,
        pub index_price: ::ethers::core::types::U256,
        pub entry_price: ::ethers::core::types::U256,
        pub pnl_usd: ::ethers::core::types::I256,
        pub liquidation_price: ::ethers::core::types::U256,
        pub presentage_to_liquidation_price: ::ethers::core::types::U256,
    }
    ///`GetLiquidityAndDebt(address,address,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct GetLiquidityAndDebt {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub scaled: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub scaled_debt: ::ethers::core::types::U256,
        pub debt: ::ethers::core::types::U256,
    }
    ///`GetMarginAndSupply(address,address,uint256,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct GetMarginAndSupply {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub balance_asset: ::ethers::core::types::U256,
        pub debt: ::ethers::core::types::U256,
        pub borrow_apy: ::ethers::core::types::U256,
        pub max_withdraw_amount: ::ethers::core::types::U256,
        pub balance_supply: ::ethers::core::types::U256,
        pub supply_apy: ::ethers::core::types::U256,
    }
    ///`GetPoolInfo(uint256,uint256,uint256,uint256,uint256,address,address,address,address,uint256,uint256,uint256,uint256,bool,bool,bool,bool,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,string,uint256,bool)`
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
    pub struct GetPoolInfo {
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
        pub is_active: bool,
        pub is_paused: bool,
        pub is_frozen: bool,
        pub borrowing_enabled: bool,
        pub decimals: ::ethers::core::types::U256,
        pub borrow_capacity: ::ethers::core::types::U256,
        pub supply_capacity: ::ethers::core::types::U256,
        pub fee_factor: ::ethers::core::types::U256,
        pub scaled_total_supply: ::ethers::core::types::U256,
        pub total_supply: ::ethers::core::types::U256,
        pub total_collateral: ::ethers::core::types::U256,
        pub available_liquidity: ::ethers::core::types::U256,
        pub scaled_total_debt: ::ethers::core::types::U256,
        pub total_debt: ::ethers::core::types::U256,
        pub borrow_usage_ratio: ::ethers::core::types::U256,
        pub optimal_usage_ratio: ::ethers::core::types::U256,
        pub rate_base: ::ethers::core::types::U256,
        pub rate_slope_1: ::ethers::core::types::U256,
        pub rate_slope_2: ::ethers::core::types::U256,
        pub symbol: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub is_usd: bool,
    }
    ///`GetPoolPrice(address,string,uint256,uint256)`
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
    pub struct GetPoolPrice {
        pub underlying_asset: ::ethers::core::types::Address,
        pub symbol: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
    }
}
