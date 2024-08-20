pub use exchange_router::*;
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
pub mod exchange_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_router"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Router"),
                        ),
                    },
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_supplyHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISupplyHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_withdrawHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IWithdrawHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_depositHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IDepositHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_borrowHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IBorrowHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_repayHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IRepayHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_redeemHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IRedeemHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_swapHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISwapHandler"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_liquidationHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract ILiquidationHandler",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_closeHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ICloseHandler"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("borrowHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBorrowHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("closeHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("closeHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ICloseHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("depositHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IDepositHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBorrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBorrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BorrowUtils.BorrowParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeClose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeClose"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct CloseUtils.CloseParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeClosePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeClosePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct CloseUtils.ClosePositionParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DepositUtils.DepositParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LiquidationUtils.LiquidationParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RedeemUtils.RedeemParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRepay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRepay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepayUtils.RepayParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRepaySubstitute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeRepaySubstitute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepaySubstituteUtils.RepaySubstituteParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSupply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SupplyUtils.SupplyParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapUtils.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSwapExactOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeSwapExactOut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapUtils.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct WithdrawUtils.WithdrawParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidationHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidationHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ILiquidationHandler",
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRedeemHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRepayHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Router"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supplyHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISupplyHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISwapHandler"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawHandler"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IWithdrawHandler",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXCHANGEROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x14\x9A8\x03\x80a\x14\x9A\x839\x81\x01`@\x81\x90Ra\x000\x91a\0\xB3V[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x9A\x8B\x16`\x80R\x9A\x8A\x16`\xA0R\x97\x89\x16`\xC0R\x95\x88\x16`\xE0R\x93\x87\x16a\x01\0R\x91\x86\x16a\x01 R\x85\x16a\x01@R\x84\x16a\x01`R\x83\x16a\x01\x80R\x82\x16a\x01\xA0R\x81\x16a\x01\xC0R\x16a\x01\xE0Ra\x01\x9DV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xA0W`\0\x80\xFD[PV[\x80Qa\0\xAE\x81a\0\x8BV[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\x80\x8D\x8F\x03\x12\x15a\0\xD6W`\0\x80\xFD[\x8CQa\0\xE1\x81a\0\x8BV[` \x8E\x01Q\x90\x9CPa\0\xF2\x81a\0\x8BV[`@\x8E\x01Q\x90\x9BPa\x01\x03\x81a\0\x8BV[``\x8E\x01Q\x90\x9APa\x01\x14\x81a\0\x8BV[`\x80\x8E\x01Q\x90\x99Pa\x01%\x81a\0\x8BV[\x97Pa\x013`\xA0\x8E\x01a\0\xA3V[\x96Pa\x01A`\xC0\x8E\x01a\0\xA3V[\x95Pa\x01O`\xE0\x8E\x01a\0\xA3V[\x94Pa\x01^a\x01\0\x8E\x01a\0\xA3V[\x93Pa\x01ma\x01 \x8E\x01a\0\xA3V[\x92Pa\x01|a\x01@\x8E\x01a\0\xA3V[\x91Pa\x01\x8Ba\x01`\x8E\x01a\0\xA3V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0Qa\x12\x11a\x02\x89`\09`\0\x81\x81a\x04\xB0\x01R\x81\x81a\x05\xE5\x01Ra\x07E\x01R`\0\x81\x81a\x045\x01Ra\t\xB9\x01R`\0\x81\x81a\x03\x0C\x01R\x81\x81a\x07\x9D\x01Ra\x0B\x1F\x01R`\0\x81\x81a\x02\xC5\x01Ra\x06\xED\x01R`\0\x81\x81a\x03\x9A\x01R\x81\x81a\x06=\x01Ra\x07\xF5\x01R`\0\x81\x81a\x03\xCE\x01Ra\x05M\x01R`\0\x81\x81a\x03f\x01Ra\n\xC7\x01R`\0\x81\x81a\x01\xAD\x01Ra\x08M\x01R`\0\x81\x81a\x04i\x01Ra\x06\x95\x01R`\0a\x02k\x01R`\0\x81\x81a\x05\n\x01Ra\n8\x01R`\0a\x027\x01Ra\x12\x11`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\x81W`\x005`\xE0\x1C\x80c\x92\x05\x8B\x8F\x11a\0\xD1W\x80c\xD2Z\xDE\xB3\x11a\0\x8AW\x80c\xED'\xAF\xAF\x11a\0dW\x80c\xED'\xAF\xAF\x14a\x04\x9EW\x80c\xEFh\xAC\x96\x14a\x04\xD2W\x80c\xF0\xE4dk\x14a\x04\xE5W\x80c\xF8\x87\xEA@\x14a\x04\xF8W`\0\x80\xFD[\x80c\xD2Z\xDE\xB3\x14a\x04#W\x80c\xD6\x17x\xFA\x14a\x04WW\x80c\xE6\xD6j\xC8\x14a\x04\x8BW`\0\x80\xFD[\x80c\x92\x05\x8B\x8F\x14a\x03AW\x80c\x9C\x8B,\xFB\x14a\x03TW\x80c\x9DE\x1D\x0C\x14a\x03\x88W\x80c\xA8.\xD4\xCE\x14a\x03\xBCW\x80c\xAC\x96P\xD8\x14a\x03\xF0W\x80c\xB8\\Q\xD5\x14a\x04\x10W`\0\x80\xFD[\x80cf\r\rg\x11a\x01>W\x80c\x85\x8E\x85o\x11a\x01\x18W\x80c\x85\x8E\x85o\x14a\x02\xB3W\x80c\x87`\xDE\x8B\x14a\x02\xE7W\x80c\x8AS\xAA\xAC\x14a\x02\xFAW\x80c\x91R\xCD\x1E\x14a\x03.W`\0\x80\xFD[\x80cf\r\rg\x14a\x02YW\x80c{'\xAF\x0B\x14a\x02\x8DW\x80c\x82~\xB7\x8A\x14a\x02\xA0W`\0\x80\xFD[\x80c\x02\x0C\xCF\x1C\x14a\x01\x86W\x80c\x084s\xEF\x14a\x01\x9BW\x80c\x1E\xCC\xF4o\x14a\x01\xECW\x80c+\xF5\xE9e\x14a\x01\xFFW\x80c4[\xDEt\x14a\x02\x12W\x80cJJ{\x04\x14a\x02%W[`\0\x80\xFD[a\x01\x99a\x01\x946`\x04a\x0C\xC5V[a\x05,V[\0[4\x80\x15a\x01\xA7W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x99a\x01\xFA6`\x04a\x0C\xFAV[a\x05\xC4V[a\x01\x99a\x02\r6`\x04a\r(V[a\x06\x1CV[a\x01\x99a\x02 6`\x04a\x0C\xC5V[a\x06tV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02eW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x02\x9B6`\x04a\r(V[a\x06\xCCV[a\x01\x99a\x02\xAE6`\x04a\r(V[a\x07$V[4\x80\x15a\x02\xBFW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x02\xF56`\x04a\rDV[a\x07|V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x03<6`\x04a\x0C\xC5V[a\x07\xD4V[a\x01\x99a\x03O6`\x04a\r(V[a\x08,V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x94W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xC8W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x03a\x03\xFE6`\x04a\r_V[a\x08\x84V[`@Qa\x01\xE3\x91\x90a\x0E&V[a\x01\x99a\x04\x1E6`\x04a\x0C\xFAV[a\t\x98V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04cW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x04\x996`\x04a\x0E\xA7V[a\t\xF0V[4\x80\x15a\x04\xAAW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x04\xE06`\x04a\x0C\xFAV[a\n\xA6V[a\x01\x99a\x04\xF36`\x04a\rDV[a\n\xFEV[4\x80\x15a\x05\x04W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x054a\x0BVV[`@QcW\x7F\xFE1`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cW\x7F\xFE1\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x03V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xB2W=`\0\x80>=`\0\xFD[PPPPPa\x05\xC1`\x01`\0UV[PV[a\x05\xCCa\x0BVV[`@Qc\xAE\xBF\x17\x9D`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xAE\xBF\x17\x9D\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\x06$a\x0BVV[`@Qc\x15\xC5\x9A\x01`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x15\xC5\x9A\x01\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[a\x06|a\x0BVV[`@Qc#\x92y\xED`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cG$\xF3\xDA\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\xA1V[a\x06\xD4a\x0BVV[`@Qc\x19e\xEF\xE9`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x19e\xEF\xE9\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[a\x07,a\x0BVV[`@Qcbn\xCB\x93`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cbn\xCB\x93\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\xE2V[a\x07\x84a\x0BVV[`@Qcir\xA9\xF5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\xE5S\xEA\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x100V[a\x07\xDCa\x0BVV[`@Qc\xEBh<M`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEBh<M\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x03V[a\x084a\x0BVV[`@Qc~HHI`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\x90\x90\x92\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9FWa\x08\x9Fa\x10\x89V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD2W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xBDW\x90P[P\x90P`\0[\x82\x81\x10\x15a\t\x91W`\0\x800\x86\x86\x85\x81\x81\x10a\x08\xF6Wa\x08\xF6a\x10\x9FV[\x90P` \x02\x81\x01\x90a\t\x08\x91\x90a\x10\xB5V[`@Qa\t\x16\x92\x91\x90a\x11\x03V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\tQW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tVV[``\x91P[P\x91P\x91P\x81a\tiWa\ti\x81a\x0B\xB4V[\x80\x84\x84\x81Q\x81\x10a\t|Wa\t|a\x10\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08\xD8V[P\x92\x91PPV[a\t\xA0a\x0BVV[`@Qc$A\x8E\xC1`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\x83\x1D\x82\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\t\xF8a\x0BVV[a\n\x01\x82a\x0B\xEDV[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x92W=`\0\x80>=`\0\xFD[PPPPPa\n\xA1`\x01`\0UV[PPPV[a\n\xAEa\x0BVV[`@Qcj\x818\xA5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD5\x02qJ\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\x0B\x06a\x0BVV[`@QcmB\xF7\xA9`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmB\xF7\xA9\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x100V[`\x02`\0T\x03a\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x80a\x0B\xC0\x83a\x0C\x14V[\x91P\x91P\x80\x15a\x0B\xE4W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA4\x91\x90a\x11\x13V[a\n\xA1\x83a\x0C\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xC1W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0`D\x83Q\x10\x15a\x0C;WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x0CH\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x0C\x87W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x0C|\x91\x90a\x11&V[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[`\0`@\x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x0C\xD7W`\0\x80\xFD[a\x0C\xE1\x83\x83a\x0C\xADV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x0CW`\0\x80\xFD[a\x0C\xE1\x83\x83a\x0C\xE8V[`\0``\x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\r:W`\0\x80\xFD[a\x0C\xE1\x83\x83a\r\x16V[`\0`\x80\x82\x84\x03\x12\x80\x15a\rWW`\0\x80\xFD[P\x90\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\rrW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x89W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\r\x9AW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xB1W`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\r\xC6W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a\r\xF1W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD9V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0E\x12\x81` \x86\x01` \x86\x01a\r\xD6V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x0E\x7FW`?\x19\x87\x86\x03\x01\x84Ra\x0Ej\x85\x83Qa\r\xFAV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x0ENV[P\x92\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xA2W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xBCW`\0\x80\xFD[a\x0E\xC5\x84a\x0E\x8BV[\x92Pa\x0E\xD3` \x85\x01a\x0E\x8BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03a\x0E\xF5\x82a\x0E\x8BV[\x16\x82R` \x90\x81\x015\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0C\xE1` \x83\x01\x84a\x0E\xE4V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@\x82\x01\x90a\x0F;\x84a\x0E\x8BV[\x16` \x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03a\x0FY\x82a\x0E\x8BV[\x16\x82R` \x81\x81\x015\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03a\x0Fz`@\x83\x01a\x0E\x8BV[\x16`@\x83\x01RPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`\x80\x81\x01a\x0C\xE1` \x83\x01\x84a\x0FHV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R``\x82\x01\x90a\x0F\xBC\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x0F\xD5` \x85\x01a\x0E\x8BV[\x16`@\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\x80\x82\x01\x90a\x0F\xFD\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10\x16` \x85\x01a\x0E\x8BV[\x16`@\x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015``\x90\x91\x01R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xA0\x82\x01\x90a\x10K\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10d` \x85\x01a\x0E\x8BV[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R\x90\x92\x015`\x80\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\xCCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xE7W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x10\xFCW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R`\0a\x0C\xE1` \x83\x01\x84a\r\xFAV[`\0` \x82\x84\x03\x12\x15a\x118W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11OW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x11`W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11zWa\x11za\x10\x89V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\xA9Wa\x11\xA9a\x10\x89V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x11\xC1W`\0\x80\xFD[a\x11\xD2\x82` \x83\x01` \x86\x01a\r\xD6V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xCF\xC2Yd\x14\xBE\xAB\x85K\xD3K\x8F\xD7\xA6O\x9BfQ\xB0Ds\x804\xA5?#0W\x83\x01\xD2\xCBdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static EXCHANGEROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\x81W`\x005`\xE0\x1C\x80c\x92\x05\x8B\x8F\x11a\0\xD1W\x80c\xD2Z\xDE\xB3\x11a\0\x8AW\x80c\xED'\xAF\xAF\x11a\0dW\x80c\xED'\xAF\xAF\x14a\x04\x9EW\x80c\xEFh\xAC\x96\x14a\x04\xD2W\x80c\xF0\xE4dk\x14a\x04\xE5W\x80c\xF8\x87\xEA@\x14a\x04\xF8W`\0\x80\xFD[\x80c\xD2Z\xDE\xB3\x14a\x04#W\x80c\xD6\x17x\xFA\x14a\x04WW\x80c\xE6\xD6j\xC8\x14a\x04\x8BW`\0\x80\xFD[\x80c\x92\x05\x8B\x8F\x14a\x03AW\x80c\x9C\x8B,\xFB\x14a\x03TW\x80c\x9DE\x1D\x0C\x14a\x03\x88W\x80c\xA8.\xD4\xCE\x14a\x03\xBCW\x80c\xAC\x96P\xD8\x14a\x03\xF0W\x80c\xB8\\Q\xD5\x14a\x04\x10W`\0\x80\xFD[\x80cf\r\rg\x11a\x01>W\x80c\x85\x8E\x85o\x11a\x01\x18W\x80c\x85\x8E\x85o\x14a\x02\xB3W\x80c\x87`\xDE\x8B\x14a\x02\xE7W\x80c\x8AS\xAA\xAC\x14a\x02\xFAW\x80c\x91R\xCD\x1E\x14a\x03.W`\0\x80\xFD[\x80cf\r\rg\x14a\x02YW\x80c{'\xAF\x0B\x14a\x02\x8DW\x80c\x82~\xB7\x8A\x14a\x02\xA0W`\0\x80\xFD[\x80c\x02\x0C\xCF\x1C\x14a\x01\x86W\x80c\x084s\xEF\x14a\x01\x9BW\x80c\x1E\xCC\xF4o\x14a\x01\xECW\x80c+\xF5\xE9e\x14a\x01\xFFW\x80c4[\xDEt\x14a\x02\x12W\x80cJJ{\x04\x14a\x02%W[`\0\x80\xFD[a\x01\x99a\x01\x946`\x04a\x0C\xC5V[a\x05,V[\0[4\x80\x15a\x01\xA7W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x99a\x01\xFA6`\x04a\x0C\xFAV[a\x05\xC4V[a\x01\x99a\x02\r6`\x04a\r(V[a\x06\x1CV[a\x01\x99a\x02 6`\x04a\x0C\xC5V[a\x06tV[4\x80\x15a\x021W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02eW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x02\x9B6`\x04a\r(V[a\x06\xCCV[a\x01\x99a\x02\xAE6`\x04a\r(V[a\x07$V[4\x80\x15a\x02\xBFW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x02\xF56`\x04a\rDV[a\x07|V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x03<6`\x04a\x0C\xC5V[a\x07\xD4V[a\x01\x99a\x03O6`\x04a\r(V[a\x08,V[4\x80\x15a\x03`W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x94W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xC8W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x03a\x03\xFE6`\x04a\r_V[a\x08\x84V[`@Qa\x01\xE3\x91\x90a\x0E&V[a\x01\x99a\x04\x1E6`\x04a\x0C\xFAV[a\t\x98V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04cW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x04\x996`\x04a\x0E\xA7V[a\t\xF0V[4\x80\x15a\x04\xAAW`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x99a\x04\xE06`\x04a\x0C\xFAV[a\n\xA6V[a\x01\x99a\x04\xF36`\x04a\rDV[a\n\xFEV[4\x80\x15a\x05\x04W`\0\x80\xFD[Pa\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x054a\x0BVV[`@QcW\x7F\xFE1`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cW\x7F\xFE1\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x03V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xB2W=`\0\x80>=`\0\xFD[PPPPPa\x05\xC1`\x01`\0UV[PV[a\x05\xCCa\x0BVV[`@Qc\xAE\xBF\x17\x9D`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xAE\xBF\x17\x9D\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\x06$a\x0BVV[`@Qc\x15\xC5\x9A\x01`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x15\xC5\x9A\x01\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[a\x06|a\x0BVV[`@Qc#\x92y\xED`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cG$\xF3\xDA\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\xA1V[a\x06\xD4a\x0BVV[`@Qc\x19e\xEF\xE9`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x19e\xEF\xE9\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[a\x07,a\x0BVV[`@Qcbn\xCB\x93`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cbn\xCB\x93\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\xE2V[a\x07\x84a\x0BVV[`@Qcir\xA9\xF5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\xE5S\xEA\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x100V[a\x07\xDCa\x0BVV[`@Qc\xEBh<M`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xEBh<M\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x03V[a\x084a\x0BVV[`@Qc~HHI`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\x90\x90\x92\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F\x84V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9FWa\x08\x9Fa\x10\x89V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD2W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xBDW\x90P[P\x90P`\0[\x82\x81\x10\x15a\t\x91W`\0\x800\x86\x86\x85\x81\x81\x10a\x08\xF6Wa\x08\xF6a\x10\x9FV[\x90P` \x02\x81\x01\x90a\t\x08\x91\x90a\x10\xB5V[`@Qa\t\x16\x92\x91\x90a\x11\x03V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\tQW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tVV[``\x91P[P\x91P\x91P\x81a\tiWa\ti\x81a\x0B\xB4V[\x80\x84\x84\x81Q\x81\x10a\t|Wa\t|a\x10\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08\xD8V[P\x92\x91PPV[a\t\xA0a\x0BVV[`@Qc$A\x8E\xC1`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\x83\x1D\x82\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\t\xF8a\x0BVV[a\n\x01\x82a\x0B\xEDV[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x92W=`\0\x80>=`\0\xFD[PPPPPa\n\xA1`\x01`\0UV[PPPV[a\n\xAEa\x0BVV[`@Qcj\x818\xA5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD5\x02qJ\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x0F V[a\x0B\x06a\x0BVV[`@QcmB\xF7\xA9`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmB\xF7\xA9\x90a\x05\x84\x90\x84\x90\x86\x90`\x04\x01a\x100V[`\x02`\0T\x03a\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x80a\x0B\xC0\x83a\x0C\x14V[\x91P\x91P\x80\x15a\x0B\xE4W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA4\x91\x90a\x11\x13V[a\n\xA1\x83a\x0C\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xC1W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0`D\x83Q\x10\x15a\x0C;WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x0CH\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x0C\x87W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x0C|\x91\x90a\x11&V[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[`\0`@\x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x0C\xD7W`\0\x80\xFD[a\x0C\xE1\x83\x83a\x0C\xADV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\r\x0CW`\0\x80\xFD[a\x0C\xE1\x83\x83a\x0C\xE8V[`\0``\x82\x84\x03\x12\x15a\x0C\xBFW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\r:W`\0\x80\xFD[a\x0C\xE1\x83\x83a\r\x16V[`\0`\x80\x82\x84\x03\x12\x80\x15a\rWW`\0\x80\xFD[P\x90\x92\x91PPV[`\0\x80` \x83\x85\x03\x12\x15a\rrW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x89W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\r\x9AW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xB1W`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\r\xC6W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a\r\xF1W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD9V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0E\x12\x81` \x86\x01` \x86\x01a\r\xD6V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x0E\x7FW`?\x19\x87\x86\x03\x01\x84Ra\x0Ej\x85\x83Qa\r\xFAV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x0ENV[P\x92\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xA2W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xBCW`\0\x80\xFD[a\x0E\xC5\x84a\x0E\x8BV[\x92Pa\x0E\xD3` \x85\x01a\x0E\x8BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03a\x0E\xF5\x82a\x0E\x8BV[\x16\x82R` \x90\x81\x015\x91\x01RV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0C\xE1` \x83\x01\x84a\x0E\xE4V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@\x82\x01\x90a\x0F;\x84a\x0E\x8BV[\x16` \x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03a\x0FY\x82a\x0E\x8BV[\x16\x82R` \x81\x81\x015\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03a\x0Fz`@\x83\x01a\x0E\x8BV[\x16`@\x83\x01RPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`\x80\x81\x01a\x0C\xE1` \x83\x01\x84a\x0FHV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R``\x82\x01\x90a\x0F\xBC\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x0F\xD5` \x85\x01a\x0E\x8BV[\x16`@\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\x80\x82\x01\x90a\x0F\xFD\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10\x16` \x85\x01a\x0E\x8BV[\x16`@\x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015``\x90\x91\x01R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xA0\x82\x01\x90a\x10K\x84a\x0E\x8BV[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10d` \x85\x01a\x0E\x8BV[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R\x90\x92\x015`\x80\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x10\xCCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xE7W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x10\xFCW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R`\0a\x0C\xE1` \x83\x01\x84a\r\xFAV[`\0` \x82\x84\x03\x12\x15a\x118W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11OW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x11`W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11zWa\x11za\x10\x89V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\xA9Wa\x11\xA9a\x10\x89V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x11\xC1W`\0\x80\xFD[a\x11\xD2\x82` \x83\x01` \x86\x01a\r\xD6V[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xCF\xC2Yd\x14\xBE\xAB\x85K\xD3K\x8F\xD7\xA6O\x9BfQ\xB0Ds\x804\xA5?#0W\x83\x01\xD2\xCBdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static EXCHANGEROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ExchangeRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExchangeRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExchangeRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExchangeRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExchangeRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExchangeRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExchangeRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXCHANGEROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                EXCHANGEROUTER_ABI.clone(),
                EXCHANGEROUTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `borrowHandler` (0xa82ed4ce) function
        pub fn borrow_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 46, 212, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeHandler` (0xed27afaf) function
        pub fn close_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([237, 39, 175, 175], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `depositHandler` (0x9c8b2cfb) function
        pub fn deposit_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 139, 44, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBorrow` (0x020ccf1c) function
        pub fn execute_borrow(
            &self,
            params: BorrowParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 12, 207, 28], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeClose` (0x1eccf46f) function
        pub fn execute_close(
            &self,
            params: CloseParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 204, 244, 111], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeClosePosition` (0x827eb78a) function
        pub fn execute_close_position(
            &self,
            params: ClosePositionParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 126, 183, 138], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeDeposit` (0xef68ac96) function
        pub fn execute_deposit(
            &self,
            params: DepositParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 104, 172, 150], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeLiquidation` (0xb85c51d5) function
        pub fn execute_liquidation(
            &self,
            params: LiquidationParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 92, 81, 213], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRedeem` (0x7b27af0b) function
        pub fn execute_redeem(
            &self,
            params: RedeemParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 39, 175, 11], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRepay` (0x9152cd1e) function
        pub fn execute_repay(
            &self,
            params: RepayParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 82, 205, 30], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRepaySubstitute` (0x2bf5e965) function
        pub fn execute_repay_substitute(
            &self,
            params: RepaySubstituteParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 245, 233, 101], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSupply` (0x345bde74) function
        pub fn execute_supply(
            &self,
            params: SupplyParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 91, 222, 116], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwap` (0xf0e4646b) function
        pub fn execute_swap(
            &self,
            params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 228, 100, 107], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwapExactOut` (0x8760de8b) function
        pub fn execute_swap_exact_out(
            &self,
            params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 96, 222, 139], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithdraw` (0x92058b8f) function
        pub fn execute_withdraw(
            &self,
            params: WithdrawParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 5, 139, 143], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationHandler` (0xd25adeb3) function
        pub fn liquidation_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 90, 222, 179], ())
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
        ///Calls the contract's `redeemHandler` (0x858e856f) function
        pub fn redeem_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([133, 142, 133, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayHandler` (0x9d451d0c) function
        pub fn repay_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([157, 69, 29, 12], ())
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
        ///Calls the contract's `router` (0xf887ea40) function
        pub fn router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTokens` (0xe6d66ac8) function
        pub fn send_tokens(
            &self,
            token: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 214, 106, 200], (token, receiver, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyHandler` (0xd61778fa) function
        pub fn supply_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([214, 23, 120, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapHandler` (0x8a53aaac) function
        pub fn swap_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([138, 83, 170, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawHandler` (0x083473ef) function
        pub fn withdraw_handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 52, 115, 239], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExchangeRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EmptyReceiver` with signature `EmptyReceiver()` and selector `0xd551823d`
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
    #[etherror(name = "EmptyReceiver", abi = "EmptyReceiver()")]
    pub struct EmptyReceiver;
    ///Container type for all input parameters for the `borrowHandler` function with signature `borrowHandler()` and selector `0xa82ed4ce`
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
    #[ethcall(name = "borrowHandler", abi = "borrowHandler()")]
    pub struct BorrowHandlerCall;
    ///Container type for all input parameters for the `closeHandler` function with signature `closeHandler()` and selector `0xed27afaf`
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
    #[ethcall(name = "closeHandler", abi = "closeHandler()")]
    pub struct CloseHandlerCall;
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
    ///Container type for all input parameters for the `depositHandler` function with signature `depositHandler()` and selector `0x9c8b2cfb`
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
    #[ethcall(name = "depositHandler", abi = "depositHandler()")]
    pub struct DepositHandlerCall;
    ///Container type for all input parameters for the `executeBorrow` function with signature `executeBorrow((address,uint256))` and selector `0x020ccf1c`
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
    #[ethcall(name = "executeBorrow", abi = "executeBorrow((address,uint256))")]
    pub struct ExecuteBorrowCall {
        pub params: BorrowParams,
    }
    ///Container type for all input parameters for the `executeClose` function with signature `executeClose((address))` and selector `0x1eccf46f`
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
    #[ethcall(name = "executeClose", abi = "executeClose((address))")]
    pub struct ExecuteCloseCall {
        pub params: CloseParams,
    }
    ///Container type for all input parameters for the `executeClosePosition` function with signature `executeClosePosition((address,address,uint256))` and selector `0x827eb78a`
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
        name = "executeClosePosition",
        abi = "executeClosePosition((address,address,uint256))"
    )]
    pub struct ExecuteClosePositionCall {
        pub params: ClosePositionParams,
    }
    ///Container type for all input parameters for the `executeDeposit` function with signature `executeDeposit((address))` and selector `0xef68ac96`
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
    #[ethcall(name = "executeDeposit", abi = "executeDeposit((address))")]
    pub struct ExecuteDepositCall {
        pub params: DepositParams,
    }
    ///Container type for all input parameters for the `executeLiquidation` function with signature `executeLiquidation((address))` and selector `0xb85c51d5`
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
    #[ethcall(name = "executeLiquidation", abi = "executeLiquidation((address))")]
    pub struct ExecuteLiquidationCall {
        pub params: LiquidationParams,
    }
    ///Container type for all input parameters for the `executeRedeem` function with signature `executeRedeem((address,uint256,address))` and selector `0x7b27af0b`
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
    #[ethcall(name = "executeRedeem", abi = "executeRedeem((address,uint256,address))")]
    pub struct ExecuteRedeemCall {
        pub params: RedeemParams,
    }
    ///Container type for all input parameters for the `executeRepay` function with signature `executeRepay((address,uint256))` and selector `0x9152cd1e`
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
    #[ethcall(name = "executeRepay", abi = "executeRepay((address,uint256))")]
    pub struct ExecuteRepayCall {
        pub params: RepayParams,
    }
    ///Container type for all input parameters for the `executeRepaySubstitute` function with signature `executeRepaySubstitute((address,uint256,address))` and selector `0x2bf5e965`
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
        name = "executeRepaySubstitute",
        abi = "executeRepaySubstitute((address,uint256,address))"
    )]
    pub struct ExecuteRepaySubstituteCall {
        pub params: RepaySubstituteParams,
    }
    ///Container type for all input parameters for the `executeSupply` function with signature `executeSupply((address,address))` and selector `0x345bde74`
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
    #[ethcall(name = "executeSupply", abi = "executeSupply((address,address))")]
    pub struct ExecuteSupplyCall {
        pub params: SupplyParams,
    }
    ///Container type for all input parameters for the `executeSwap` function with signature `executeSwap((address,address,uint256,uint256))` and selector `0xf0e4646b`
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
        name = "executeSwap",
        abi = "executeSwap((address,address,uint256,uint256))"
    )]
    pub struct ExecuteSwapCall {
        pub params: SwapParams,
    }
    ///Container type for all input parameters for the `executeSwapExactOut` function with signature `executeSwapExactOut((address,address,uint256,uint256))` and selector `0x8760de8b`
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
        name = "executeSwapExactOut",
        abi = "executeSwapExactOut((address,address,uint256,uint256))"
    )]
    pub struct ExecuteSwapExactOutCall {
        pub params: SwapParams,
    }
    ///Container type for all input parameters for the `executeWithdraw` function with signature `executeWithdraw((address,uint256,address))` and selector `0x92058b8f`
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
        name = "executeWithdraw",
        abi = "executeWithdraw((address,uint256,address))"
    )]
    pub struct ExecuteWithdrawCall {
        pub params: WithdrawParams,
    }
    ///Container type for all input parameters for the `liquidationHandler` function with signature `liquidationHandler()` and selector `0xd25adeb3`
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
    #[ethcall(name = "liquidationHandler", abi = "liquidationHandler()")]
    pub struct LiquidationHandlerCall;
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
    ///Container type for all input parameters for the `redeemHandler` function with signature `redeemHandler()` and selector `0x858e856f`
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
    #[ethcall(name = "redeemHandler", abi = "redeemHandler()")]
    pub struct RedeemHandlerCall;
    ///Container type for all input parameters for the `repayHandler` function with signature `repayHandler()` and selector `0x9d451d0c`
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
    #[ethcall(name = "repayHandler", abi = "repayHandler()")]
    pub struct RepayHandlerCall;
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
    ///Container type for all input parameters for the `router` function with signature `router()` and selector `0xf887ea40`
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
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    ///Container type for all input parameters for the `sendTokens` function with signature `sendTokens(address,address,uint256)` and selector `0xe6d66ac8`
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
    #[ethcall(name = "sendTokens", abi = "sendTokens(address,address,uint256)")]
    pub struct SendTokensCall {
        pub token: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `supplyHandler` function with signature `supplyHandler()` and selector `0xd61778fa`
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
    #[ethcall(name = "supplyHandler", abi = "supplyHandler()")]
    pub struct SupplyHandlerCall;
    ///Container type for all input parameters for the `swapHandler` function with signature `swapHandler()` and selector `0x8a53aaac`
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
    #[ethcall(name = "swapHandler", abi = "swapHandler()")]
    pub struct SwapHandlerCall;
    ///Container type for all input parameters for the `withdrawHandler` function with signature `withdrawHandler()` and selector `0x083473ef`
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
    #[ethcall(name = "withdrawHandler", abi = "withdrawHandler()")]
    pub struct WithdrawHandlerCall;
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
    pub enum ExchangeRouterCalls {
        BorrowHandler(BorrowHandlerCall),
        CloseHandler(CloseHandlerCall),
        DataStore(DataStoreCall),
        DepositHandler(DepositHandlerCall),
        ExecuteBorrow(ExecuteBorrowCall),
        ExecuteClose(ExecuteCloseCall),
        ExecuteClosePosition(ExecuteClosePositionCall),
        ExecuteDeposit(ExecuteDepositCall),
        ExecuteLiquidation(ExecuteLiquidationCall),
        ExecuteRedeem(ExecuteRedeemCall),
        ExecuteRepay(ExecuteRepayCall),
        ExecuteRepaySubstitute(ExecuteRepaySubstituteCall),
        ExecuteSupply(ExecuteSupplyCall),
        ExecuteSwap(ExecuteSwapCall),
        ExecuteSwapExactOut(ExecuteSwapExactOutCall),
        ExecuteWithdraw(ExecuteWithdrawCall),
        LiquidationHandler(LiquidationHandlerCall),
        Multicall(MulticallCall),
        RedeemHandler(RedeemHandlerCall),
        RepayHandler(RepayHandlerCall),
        RoleStore(RoleStoreCall),
        Router(RouterCall),
        SendTokens(SendTokensCall),
        SupplyHandler(SupplyHandlerCall),
        SwapHandler(SwapHandlerCall),
        WithdrawHandler(WithdrawHandlerCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExchangeRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BorrowHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BorrowHandler(decoded));
            }
            if let Ok(decoded) = <CloseHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseHandler(decoded));
            }
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <DepositHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositHandler(decoded));
            }
            if let Ok(decoded) = <ExecuteBorrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBorrow(decoded));
            }
            if let Ok(decoded) = <ExecuteCloseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteClose(decoded));
            }
            if let Ok(decoded) = <ExecuteClosePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteClosePosition(decoded));
            }
            if let Ok(decoded) = <ExecuteDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteDeposit(decoded));
            }
            if let Ok(decoded) = <ExecuteLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteLiquidation(decoded));
            }
            if let Ok(decoded) = <ExecuteRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRedeem(decoded));
            }
            if let Ok(decoded) = <ExecuteRepayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepay(decoded));
            }
            if let Ok(decoded) = <ExecuteRepaySubstituteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepaySubstitute(decoded));
            }
            if let Ok(decoded) = <ExecuteSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSupply(decoded));
            }
            if let Ok(decoded) = <ExecuteSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSwap(decoded));
            }
            if let Ok(decoded) = <ExecuteSwapExactOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSwapExactOut(decoded));
            }
            if let Ok(decoded) = <ExecuteWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithdraw(decoded));
            }
            if let Ok(decoded) = <LiquidationHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationHandler(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <RedeemHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RedeemHandler(decoded));
            }
            if let Ok(decoded) = <RepayHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayHandler(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <RouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Router(decoded));
            }
            if let Ok(decoded) = <SendTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendTokens(decoded));
            }
            if let Ok(decoded) = <SupplyHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupplyHandler(decoded));
            }
            if let Ok(decoded) = <SwapHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapHandler(decoded));
            }
            if let Ok(decoded) = <WithdrawHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawHandler(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExchangeRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BorrowHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloseHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteBorrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteClose(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteClosePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRepay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRepaySubstitute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSwapExactOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Router(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExchangeRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BorrowHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBorrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteClose(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteClosePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRepay(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRepaySubstitute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwapExactOut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapHandler(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawHandler(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BorrowHandlerCall> for ExchangeRouterCalls {
        fn from(value: BorrowHandlerCall) -> Self {
            Self::BorrowHandler(value)
        }
    }
    impl ::core::convert::From<CloseHandlerCall> for ExchangeRouterCalls {
        fn from(value: CloseHandlerCall) -> Self {
            Self::CloseHandler(value)
        }
    }
    impl ::core::convert::From<DataStoreCall> for ExchangeRouterCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<DepositHandlerCall> for ExchangeRouterCalls {
        fn from(value: DepositHandlerCall) -> Self {
            Self::DepositHandler(value)
        }
    }
    impl ::core::convert::From<ExecuteBorrowCall> for ExchangeRouterCalls {
        fn from(value: ExecuteBorrowCall) -> Self {
            Self::ExecuteBorrow(value)
        }
    }
    impl ::core::convert::From<ExecuteCloseCall> for ExchangeRouterCalls {
        fn from(value: ExecuteCloseCall) -> Self {
            Self::ExecuteClose(value)
        }
    }
    impl ::core::convert::From<ExecuteClosePositionCall> for ExchangeRouterCalls {
        fn from(value: ExecuteClosePositionCall) -> Self {
            Self::ExecuteClosePosition(value)
        }
    }
    impl ::core::convert::From<ExecuteDepositCall> for ExchangeRouterCalls {
        fn from(value: ExecuteDepositCall) -> Self {
            Self::ExecuteDeposit(value)
        }
    }
    impl ::core::convert::From<ExecuteLiquidationCall> for ExchangeRouterCalls {
        fn from(value: ExecuteLiquidationCall) -> Self {
            Self::ExecuteLiquidation(value)
        }
    }
    impl ::core::convert::From<ExecuteRedeemCall> for ExchangeRouterCalls {
        fn from(value: ExecuteRedeemCall) -> Self {
            Self::ExecuteRedeem(value)
        }
    }
    impl ::core::convert::From<ExecuteRepayCall> for ExchangeRouterCalls {
        fn from(value: ExecuteRepayCall) -> Self {
            Self::ExecuteRepay(value)
        }
    }
    impl ::core::convert::From<ExecuteRepaySubstituteCall> for ExchangeRouterCalls {
        fn from(value: ExecuteRepaySubstituteCall) -> Self {
            Self::ExecuteRepaySubstitute(value)
        }
    }
    impl ::core::convert::From<ExecuteSupplyCall> for ExchangeRouterCalls {
        fn from(value: ExecuteSupplyCall) -> Self {
            Self::ExecuteSupply(value)
        }
    }
    impl ::core::convert::From<ExecuteSwapCall> for ExchangeRouterCalls {
        fn from(value: ExecuteSwapCall) -> Self {
            Self::ExecuteSwap(value)
        }
    }
    impl ::core::convert::From<ExecuteSwapExactOutCall> for ExchangeRouterCalls {
        fn from(value: ExecuteSwapExactOutCall) -> Self {
            Self::ExecuteSwapExactOut(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdrawCall> for ExchangeRouterCalls {
        fn from(value: ExecuteWithdrawCall) -> Self {
            Self::ExecuteWithdraw(value)
        }
    }
    impl ::core::convert::From<LiquidationHandlerCall> for ExchangeRouterCalls {
        fn from(value: LiquidationHandlerCall) -> Self {
            Self::LiquidationHandler(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for ExchangeRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<RedeemHandlerCall> for ExchangeRouterCalls {
        fn from(value: RedeemHandlerCall) -> Self {
            Self::RedeemHandler(value)
        }
    }
    impl ::core::convert::From<RepayHandlerCall> for ExchangeRouterCalls {
        fn from(value: RepayHandlerCall) -> Self {
            Self::RepayHandler(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for ExchangeRouterCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<RouterCall> for ExchangeRouterCalls {
        fn from(value: RouterCall) -> Self {
            Self::Router(value)
        }
    }
    impl ::core::convert::From<SendTokensCall> for ExchangeRouterCalls {
        fn from(value: SendTokensCall) -> Self {
            Self::SendTokens(value)
        }
    }
    impl ::core::convert::From<SupplyHandlerCall> for ExchangeRouterCalls {
        fn from(value: SupplyHandlerCall) -> Self {
            Self::SupplyHandler(value)
        }
    }
    impl ::core::convert::From<SwapHandlerCall> for ExchangeRouterCalls {
        fn from(value: SwapHandlerCall) -> Self {
            Self::SwapHandler(value)
        }
    }
    impl ::core::convert::From<WithdrawHandlerCall> for ExchangeRouterCalls {
        fn from(value: WithdrawHandlerCall) -> Self {
            Self::WithdrawHandler(value)
        }
    }
    ///Container type for all return fields from the `borrowHandler` function with signature `borrowHandler()` and selector `0xa82ed4ce`
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
    pub struct BorrowHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `closeHandler` function with signature `closeHandler()` and selector `0xed27afaf`
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
    pub struct CloseHandlerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `depositHandler` function with signature `depositHandler()` and selector `0x9c8b2cfb`
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
    pub struct DepositHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidationHandler` function with signature `liquidationHandler()` and selector `0xd25adeb3`
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
    pub struct LiquidationHandlerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `redeemHandler` function with signature `redeemHandler()` and selector `0x858e856f`
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
    pub struct RedeemHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `repayHandler` function with signature `repayHandler()` and selector `0x9d451d0c`
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
    pub struct RepayHandlerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `router` function with signature `router()` and selector `0xf887ea40`
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
    pub struct RouterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supplyHandler` function with signature `supplyHandler()` and selector `0xd61778fa`
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
    pub struct SupplyHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `swapHandler` function with signature `swapHandler()` and selector `0x8a53aaac`
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
    pub struct SwapHandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdrawHandler` function with signature `withdrawHandler()` and selector `0x083473ef`
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
    pub struct WithdrawHandlerReturn(pub ::ethers::core::types::Address);
}
