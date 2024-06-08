pub use i_event_emitter::*;
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
pub mod i_event_emitter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("emitBorrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitBorrow"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowRate"),
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
                    ::std::borrow::ToOwned::to_owned("emitClose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitClose"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "underlyingAssetUsd",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdStartClose",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdAfterRepayAndSellCollateral",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdAfterBuyCollateralAndRepay",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("emitClosePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitClosePosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "underlyingAssetUsd",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("collateralAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("remainAmountUsd"),
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
                    ::std::borrow::ToOwned::to_owned("emitDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDeposit"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("depositAmount"),
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
                    ::std::borrow::ToOwned::to_owned("emitLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitLiquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalCollateralUsd",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebtUsd"),
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
                    ::std::borrow::ToOwned::to_owned("emitPoolUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitPoolUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowIndex"),
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
                    ::std::borrow::ToOwned::to_owned("emitPositionLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "emitPositionLiquidation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("emitRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitRedeem"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("redeemAmount"),
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
                    ::std::borrow::ToOwned::to_owned("emitRepay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitRepay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("repayAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("useCollateral"),
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
                    ::std::borrow::ToOwned::to_owned("emitSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitSupply"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("SupplyAmount"),
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
                    ::std::borrow::ToOwned::to_owned("emitSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "underlyingAssetOut",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
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
                    ::std::borrow::ToOwned::to_owned("emitWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitWithdraw"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawAmount"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IEVENTEMITTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IEventEmitter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IEventEmitter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IEventEmitter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IEventEmitter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IEventEmitter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IEventEmitter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IEventEmitter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IEVENTEMITTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `emitBorrow` (0xd30597e2) function
        pub fn emit_borrow(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            borrow_amount: ::ethers::core::types::U256,
            borrow_rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [211, 5, 151, 226],
                    (underlying_asset, account, borrow_amount, borrow_rate),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitClose` (0x324ac765) function
        pub fn emit_close(
            &self,
            underlying_asset_usd: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            amount_usd_start_close: ::ethers::core::types::U256,
            amount_usd_after_repay_and_sell_collateral: ::ethers::core::types::U256,
            amount_usd_after_buy_collateral_and_repay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [50, 74, 199, 101],
                    (
                        underlying_asset_usd,
                        account,
                        amount_usd_start_close,
                        amount_usd_after_repay_and_sell_collateral,
                        amount_usd_after_buy_collateral_and_repay,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitClosePosition` (0x6c1456dd) function
        pub fn emit_close_position(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            underlying_asset_usd: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            collateral_amount: ::ethers::core::types::U256,
            debt_amount: ::ethers::core::types::U256,
            remain_amount_usd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [108, 20, 86, 221],
                    (
                        underlying_asset,
                        underlying_asset_usd,
                        account,
                        collateral_amount,
                        debt_amount,
                        remain_amount_usd,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDeposit` (0x8b7711b4) function
        pub fn emit_deposit(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [139, 119, 17, 180],
                    (underlying_asset, account, deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitLiquidation` (0x94e0dd1d) function
        pub fn emit_liquidation(
            &self,
            liquidator: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            health_factor: ::ethers::core::types::U256,
            health_factor_liquidation_threshold: ::ethers::core::types::U256,
            total_collateral_usd: ::ethers::core::types::U256,
            total_debt_usd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [148, 224, 221, 29],
                    (
                        liquidator,
                        account,
                        health_factor,
                        health_factor_liquidation_threshold,
                        total_collateral_usd,
                        total_debt_usd,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitPoolUpdated` (0x7c24dab7) function
        pub fn emit_pool_updated(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            liquidity_rate: ::ethers::core::types::U256,
            borrow_rate: ::ethers::core::types::U256,
            liquidity_index: ::ethers::core::types::U256,
            borrow_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [124, 36, 218, 183],
                    (
                        underlying_asset,
                        liquidity_rate,
                        borrow_rate,
                        liquidity_index,
                        borrow_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitPositionLiquidation` (0x054a90c5) function
        pub fn emit_position_liquidation(
            &self,
            liquidator: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            collateral: ::ethers::core::types::U256,
            debt: ::ethers::core::types::U256,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [5, 74, 144, 197],
                    (liquidator, underlying_asset, account, collateral, debt, price),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitRedeem` (0xbb0e0b2c) function
        pub fn emit_redeem(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            redeem_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [187, 14, 11, 44],
                    (underlying_asset, account, to, redeem_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitRepay` (0xe764ac72) function
        pub fn emit_repay(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            repayer: ::ethers::core::types::Address,
            repay_amount: ::ethers::core::types::U256,
            use_collateral: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [231, 100, 172, 114],
                    (underlying_asset, repayer, repay_amount, use_collateral),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitSupply` (0x9324595b) function
        pub fn emit_supply(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            supply_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [147, 36, 89, 91],
                    (underlying_asset, account, to, supply_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitSwap` (0x68c0c115) function
        pub fn emit_swap(
            &self,
            underlying_asset_in: ::ethers::core::types::Address,
            underlying_asset_out: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            amount_out: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 192, 193, 21],
                    (
                        underlying_asset_in,
                        underlying_asset_out,
                        account,
                        amount_in,
                        amount_out,
                        fee,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitWithdraw` (0xa250ab9a) function
        pub fn emit_withdraw(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            withdraw_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [162, 80, 171, 154],
                    (underlying_asset, account, to, withdraw_amount),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IEventEmitter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `emitBorrow` function with signature `emitBorrow(address,address,uint256,uint256)` and selector `0xd30597e2`
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
    #[ethcall(name = "emitBorrow", abi = "emitBorrow(address,address,uint256,uint256)")]
    pub struct EmitBorrowCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub borrow_amount: ::ethers::core::types::U256,
        pub borrow_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitClose` function with signature `emitClose(address,address,uint256,uint256,uint256)` and selector `0x324ac765`
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
        name = "emitClose",
        abi = "emitClose(address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitCloseCall {
        pub underlying_asset_usd: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub amount_usd_start_close: ::ethers::core::types::U256,
        pub amount_usd_after_repay_and_sell_collateral: ::ethers::core::types::U256,
        pub amount_usd_after_buy_collateral_and_repay: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitClosePosition` function with signature `emitClosePosition(address,address,address,uint256,uint256,uint256)` and selector `0x6c1456dd`
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
        name = "emitClosePosition",
        abi = "emitClosePosition(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitClosePositionCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub underlying_asset_usd: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub collateral_amount: ::ethers::core::types::U256,
        pub debt_amount: ::ethers::core::types::U256,
        pub remain_amount_usd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitDeposit` function with signature `emitDeposit(address,address,uint256)` and selector `0x8b7711b4`
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
    #[ethcall(name = "emitDeposit", abi = "emitDeposit(address,address,uint256)")]
    pub struct EmitDepositCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitLiquidation` function with signature `emitLiquidation(address,address,uint256,uint256,uint256,uint256)` and selector `0x94e0dd1d`
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
        name = "emitLiquidation",
        abi = "emitLiquidation(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct EmitLiquidationCall {
        pub liquidator: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub health_factor: ::ethers::core::types::U256,
        pub health_factor_liquidation_threshold: ::ethers::core::types::U256,
        pub total_collateral_usd: ::ethers::core::types::U256,
        pub total_debt_usd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitPoolUpdated` function with signature `emitPoolUpdated(address,uint256,uint256,uint256,uint256)` and selector `0x7c24dab7`
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
        name = "emitPoolUpdated",
        abi = "emitPoolUpdated(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct EmitPoolUpdatedCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub liquidity_rate: ::ethers::core::types::U256,
        pub borrow_rate: ::ethers::core::types::U256,
        pub liquidity_index: ::ethers::core::types::U256,
        pub borrow_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitPositionLiquidation` function with signature `emitPositionLiquidation(address,address,address,uint256,uint256,uint256)` and selector `0x054a90c5`
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
        name = "emitPositionLiquidation",
        abi = "emitPositionLiquidation(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitPositionLiquidationCall {
        pub liquidator: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub collateral: ::ethers::core::types::U256,
        pub debt: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitRedeem` function with signature `emitRedeem(address,address,address,uint256)` and selector `0xbb0e0b2c`
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
    #[ethcall(name = "emitRedeem", abi = "emitRedeem(address,address,address,uint256)")]
    pub struct EmitRedeemCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub redeem_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitRepay` function with signature `emitRepay(address,address,uint256,bool)` and selector `0xe764ac72`
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
    #[ethcall(name = "emitRepay", abi = "emitRepay(address,address,uint256,bool)")]
    pub struct EmitRepayCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub repayer: ::ethers::core::types::Address,
        pub repay_amount: ::ethers::core::types::U256,
        pub use_collateral: bool,
    }
    ///Container type for all input parameters for the `emitSupply` function with signature `emitSupply(address,address,address,uint256)` and selector `0x9324595b`
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
    #[ethcall(name = "emitSupply", abi = "emitSupply(address,address,address,uint256)")]
    pub struct EmitSupplyCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub supply_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitSwap` function with signature `emitSwap(address,address,address,uint256,uint256,uint256)` and selector `0x68c0c115`
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
        name = "emitSwap",
        abi = "emitSwap(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitSwapCall {
        pub underlying_asset_in: ::ethers::core::types::Address,
        pub underlying_asset_out: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitWithdraw` function with signature `emitWithdraw(address,address,address,uint256)` and selector `0xa250ab9a`
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
        name = "emitWithdraw",
        abi = "emitWithdraw(address,address,address,uint256)"
    )]
    pub struct EmitWithdrawCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub withdraw_amount: ::ethers::core::types::U256,
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
    pub enum IEventEmitterCalls {
        EmitBorrow(EmitBorrowCall),
        EmitClose(EmitCloseCall),
        EmitClosePosition(EmitClosePositionCall),
        EmitDeposit(EmitDepositCall),
        EmitLiquidation(EmitLiquidationCall),
        EmitPoolUpdated(EmitPoolUpdatedCall),
        EmitPositionLiquidation(EmitPositionLiquidationCall),
        EmitRedeem(EmitRedeemCall),
        EmitRepay(EmitRepayCall),
        EmitSupply(EmitSupplyCall),
        EmitSwap(EmitSwapCall),
        EmitWithdraw(EmitWithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IEventEmitterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EmitBorrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitBorrow(decoded));
            }
            if let Ok(decoded) = <EmitCloseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitClose(decoded));
            }
            if let Ok(decoded) = <EmitClosePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitClosePosition(decoded));
            }
            if let Ok(decoded) = <EmitDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitDeposit(decoded));
            }
            if let Ok(decoded) = <EmitLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitLiquidation(decoded));
            }
            if let Ok(decoded) = <EmitPoolUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitPoolUpdated(decoded));
            }
            if let Ok(decoded) = <EmitPositionLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitPositionLiquidation(decoded));
            }
            if let Ok(decoded) = <EmitRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitRedeem(decoded));
            }
            if let Ok(decoded) = <EmitRepayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitRepay(decoded));
            }
            if let Ok(decoded) = <EmitSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitSupply(decoded));
            }
            if let Ok(decoded) = <EmitSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitSwap(decoded));
            }
            if let Ok(decoded) = <EmitWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmitWithdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IEventEmitterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EmitBorrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitClose(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitClosePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitPoolUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitPositionLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitRepay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IEventEmitterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmitBorrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitClose(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitClosePosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitLiquidation(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitPoolUpdated(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitPositionLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmitRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitRepay(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitWithdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmitBorrowCall> for IEventEmitterCalls {
        fn from(value: EmitBorrowCall) -> Self {
            Self::EmitBorrow(value)
        }
    }
    impl ::core::convert::From<EmitCloseCall> for IEventEmitterCalls {
        fn from(value: EmitCloseCall) -> Self {
            Self::EmitClose(value)
        }
    }
    impl ::core::convert::From<EmitClosePositionCall> for IEventEmitterCalls {
        fn from(value: EmitClosePositionCall) -> Self {
            Self::EmitClosePosition(value)
        }
    }
    impl ::core::convert::From<EmitDepositCall> for IEventEmitterCalls {
        fn from(value: EmitDepositCall) -> Self {
            Self::EmitDeposit(value)
        }
    }
    impl ::core::convert::From<EmitLiquidationCall> for IEventEmitterCalls {
        fn from(value: EmitLiquidationCall) -> Self {
            Self::EmitLiquidation(value)
        }
    }
    impl ::core::convert::From<EmitPoolUpdatedCall> for IEventEmitterCalls {
        fn from(value: EmitPoolUpdatedCall) -> Self {
            Self::EmitPoolUpdated(value)
        }
    }
    impl ::core::convert::From<EmitPositionLiquidationCall> for IEventEmitterCalls {
        fn from(value: EmitPositionLiquidationCall) -> Self {
            Self::EmitPositionLiquidation(value)
        }
    }
    impl ::core::convert::From<EmitRedeemCall> for IEventEmitterCalls {
        fn from(value: EmitRedeemCall) -> Self {
            Self::EmitRedeem(value)
        }
    }
    impl ::core::convert::From<EmitRepayCall> for IEventEmitterCalls {
        fn from(value: EmitRepayCall) -> Self {
            Self::EmitRepay(value)
        }
    }
    impl ::core::convert::From<EmitSupplyCall> for IEventEmitterCalls {
        fn from(value: EmitSupplyCall) -> Self {
            Self::EmitSupply(value)
        }
    }
    impl ::core::convert::From<EmitSwapCall> for IEventEmitterCalls {
        fn from(value: EmitSwapCall) -> Self {
            Self::EmitSwap(value)
        }
    }
    impl ::core::convert::From<EmitWithdrawCall> for IEventEmitterCalls {
        fn from(value: EmitWithdrawCall) -> Self {
            Self::EmitWithdraw(value)
        }
    }
}
