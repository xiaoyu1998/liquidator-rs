pub use event_emitter::*;
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
pub mod event_emitter {
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
                ],
            }),
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
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledUsd"),
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
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
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
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
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
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
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
                                    name: ::std::borrow::ToOwned::to_owned("supplyAmount"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledOut"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Close"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Close"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdStartClose",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdAfterRepayAndSellCollateral",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "amountUsdAfterBuyCollateralAndRepay",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ClosePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClosePosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remainUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateralUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("depositer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Liquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Liquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("healthFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "healthFactorLiquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalCollateralUsd",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebtUsd"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionLiquidation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("redeemer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("repayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("useCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("supplier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateralIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateralOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtScaledOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
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
    pub static EVENTEMITTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x0C\x978\x03\x80a\x0C\x97\x839\x81\x01`@\x81\x90R`,\x91`<V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`jV[`\0` \x82\x84\x03\x12\x15`MW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0C\x0Ba\0\x8C`\09`\0\x81\x81a\x01\x14\x01Ra\x07~\x01Ra\x0C\x0B`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c{\xA5\x9A\x8B\x11a\0\x8CW\x80c\x93$Y[\x11a\0fW\x80c\x93$Y[\x14a\x01\xB1W\x80c\x94\xE0\xDD\x1D\x14a\x01\xC4W\x80c\xA2P\xAB\x9A\x14a\x01\xD7W\x80c\xF0\x02\xE0\xBD\x14a\x01\xEAW`\0\x80\xFD[\x80c{\xA5\x9A\x8B\x14a\x01xW\x80c|$\xDA\xB7\x14a\x01\x8BW\x80c\x87\xA4T\xEC\x14a\x01\x9EW`\0\x80\xFD[\x80c\x05J\x90\xC5\x14a\0\xD4W\x80c%\xEC\xCA\x95\x14a\0\xE9W\x80c2J\xC7e\x14a\0\xFCW\x80cJJ{\x04\x14a\x01\x0FW\x80c[c\x81n\x14a\x01RW\x80cf\xE4\xD1z\x14a\x01eW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x08;V[a\x01\xFDV[\0[a\0\xE7a\0\xF76`\x04a\x08;V[a\x02\xAAV[a\0\xE7a\x01\n6`\x04a\x08\x9AV[a\x03\rV[a\x016\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x01`6`\x04a\x08\xF8V[a\x03xV[a\0\xE7a\x01s6`\x04a\tYV[a\x03\xECV[a\0\xE7a\x01\x866`\x04a\x08\x9AV[a\x04zV[a\0\xE7a\x01\x996`\x04a\t\xDDV[a\x04\xDAV[a\0\xE7a\x01\xAC6`\x04a\n\x1FV[a\x05FV[a\0\xE7a\x01\xBF6`\x04a\n\x90V[a\x05\xC4V[a\0\xE7a\x01\xD26`\x04a\n\xDBV[a\x063V[a\0\xE7a\x01\xE56`\x04a\n\x90V[a\x06\x9AV[a\0\xE7a\x01\xF86`\x04a\n\xDBV[a\x06\xFBV[a\x02M`@Q` \x01a\x02\x0F\x90a\x0B0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x07bV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\xDEt\xDC\xB4\xB9\x89M\x99\xAF\xEEI\xAB\xAA[\x8E\xEE\xCE\x83\x99B,@<Q(\xB2\x95\x8F\r\xD6\x18\x05\x90``\x01[`@Q\x80\x91\x03\x90\xA4PPPPPPV[a\x02\xBC`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\xF7\xE4'\x8E\x88\x05\xEE \x18SD\xA74\x9A\xAA\x91\xCD\xCF_1o\xBFf\x89OaJ3\xDE\xBD\x06y\x90``\x01a\x02\x9AV[a\x03\x1F`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7F\xB1IG\x02\x01`\r\x81\xE1i\x8A\x9D\0\x1D \xF8\xD2\xA06\xD2\xC0\xF4%Cke\xA1T\xE9^\xB5:\x90``\x01[`@Q\x80\x91\x03\x90\xA3PPPPPV[a\x03\x8A`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R\x84\x15\x15` \x82\x01R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\\\xA7\xF1\xE4/\xBA\xE6\xC2p$\xA86M\x7F\x80\xE1\xEC\xE7\x02\xFC\xCA\x05\xB4\r\xE6\x9E.\x8C\xBC\xA2\xFA\xC9\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x03\xFE`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x8B\x82\x16\x91\x8D\x16\x90\x7FS8U\xCB\x8A\x87\xF0\x99_l3\x90f^\xB3}\xA8\"\xF5U\x0B\x13\xA9s\xA7y\x06\xD0\x18\xA2\xA8\x9C\x90`\xE0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPV[a\x04\x8C`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7FN,\xA0Q^\xD1\xAE\xF19_f\xB50;\xB5\xD6\xF1\xBF\x9Da\xA3S\xFAS\xF7?\x8A\xC9\x97?\xA9\xF6\x90``\x01a\x03iV[a\x04\xEC`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\xC3 \xA8R\x9B\x15\xB8Q\xAA\xA6\x85\x19\x91\x9A\xC3D\xE1\xCA\xCE\xAFOG\xD1]\xF6\xE5\x81\x81\xDF\xECc\x19\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x05X`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x91\x89\x82\x16\x91\x8B\x16\x90\x7FRN/\xBE\x97b\xDE\xED\x9C/\x1E\x13<\no\xD9\x0E\x99\x1A\x8F\x87N\xDBh\xC5u\r\xC3\xE4>\xAA\xCD\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPPV[a\x05\xD6`@Q` \x01a\x02\x0F\x90a\x0B0V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FOxWI~\x11[\xB4\xE8\xAC\xD8\xD9\xF78.\xB4\xC1Y\xD8t\x83\x8Dch\t\x95#\xABn\xF9\xF61\x84`@Qa\x06%\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[a\x06E`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\xC7\x0F\xD1\xEB@\xD7\xAED\xA5B\\\t\xDE\xCEZ\x02;G'sV\xE0\x05\xB6\xCA\xEF\x16\0\xCF\xC73N\x90`\x80\x01a\x03\xDCV[a\x06\xAC`@Q` \x01a\x02\x0F\x90a\x0B0V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F1\x15\xD1D\x9A{s,\x98l\xBA\x18$N\x89zE\x0Fa\xE1\xBB\x8DX\x9C\xD2\xE6\x9El\x89$\xF9\xF7\x84`@Qa\x06%\x91\x81R` \x01\x90V[a\x07\r`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\x9B\x07\x9D/\x87\xF5C\xB9\xB0lg2\xEAMs\xFD\x08\xF6\x9C/*Q\xDD\xE3\x14\xAD\x91P0*\xF7T\x90`\x80\x01a\x03\xDCV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF1\x91\x90a\x0BTV[a\x08\x1BW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x08\x12\x92\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x086W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x08TW`\0\x80\xFD[a\x08]\x87a\x08\x1FV[\x95Pa\x08k` \x88\x01a\x08\x1FV[\x94Pa\x08y`@\x88\x01a\x08\x1FV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x08\xB2W`\0\x80\xFD[a\x08\xBB\x86a\x08\x1FV[\x94Pa\x08\xC9` \x87\x01a\x08\x1FV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x08\xF5W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\x11W`\0\x80\xFD[a\t\x1A\x87a\x08\x1FV[\x95Pa\t(` \x88\x01a\x08\x1FV[\x94P`@\x87\x015\x93P``\x87\x015a\t?\x81a\x08\xE7V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\tyW`\0\x80\xFD[a\t\x82\x8Ba\x08\x1FV[\x99Pa\t\x90` \x8C\x01a\x08\x1FV[\x98Pa\t\x9E`@\x8C\x01a\x08\x1FV[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x81\x015\x98`\xA0\x82\x015\x98P`\xC0\x82\x015\x97P`\xE0\x82\x015\x96Pa\x01\0\x82\x015\x95Pa\x01 \x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\t\xF5W`\0\x80\xFD[a\t\xFE\x86a\x08\x1FV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x015\x94P\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\n<W`\0\x80\xFD[a\nE\x89a\x08\x1FV[\x97Pa\nS` \x8A\x01a\x08\x1FV[\x96Pa\na`@\x8A\x01a\x08\x1FV[\x97\x9A\x96\x99P\x96\x97``\x81\x015\x97P`\x80\x81\x015\x96`\xA0\x82\x015\x96P`\xC0\x82\x015\x95P`\xE0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\n\xA6W`\0\x80\xFD[a\n\xAF\x85a\x08\x1FV[\x93Pa\n\xBD` \x86\x01a\x08\x1FV[\x92Pa\n\xCB`@\x86\x01a\x08\x1FV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\n\xF4W`\0\x80\xFD[a\n\xFD\x87a\x08\x1FV[\x95Pa\x0B\x0B` \x88\x01a\x08\x1FV[\x95\x98\x95\x97PPPP`@\x84\x015\x93``\x81\x015\x93`\x80\x82\x015\x93P`\xA0\x90\x91\x015\x91PV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0BfW`\0\x80\xFD[\x81Qa\x0Bq\x81a\x08\xE7V[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x0B\xB4W` \x81\x86\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x0B\x97V[P`\0``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ~i| @\xEF\x82\xED\xD3\xAF*+\xF44\xAB\xF1bQ\xED25M=J\xFC\x83\xCF\x90g\xD0\x1E\xBEdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static EVENTEMITTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c{\xA5\x9A\x8B\x11a\0\x8CW\x80c\x93$Y[\x11a\0fW\x80c\x93$Y[\x14a\x01\xB1W\x80c\x94\xE0\xDD\x1D\x14a\x01\xC4W\x80c\xA2P\xAB\x9A\x14a\x01\xD7W\x80c\xF0\x02\xE0\xBD\x14a\x01\xEAW`\0\x80\xFD[\x80c{\xA5\x9A\x8B\x14a\x01xW\x80c|$\xDA\xB7\x14a\x01\x8BW\x80c\x87\xA4T\xEC\x14a\x01\x9EW`\0\x80\xFD[\x80c\x05J\x90\xC5\x14a\0\xD4W\x80c%\xEC\xCA\x95\x14a\0\xE9W\x80c2J\xC7e\x14a\0\xFCW\x80cJJ{\x04\x14a\x01\x0FW\x80c[c\x81n\x14a\x01RW\x80cf\xE4\xD1z\x14a\x01eW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x08;V[a\x01\xFDV[\0[a\0\xE7a\0\xF76`\x04a\x08;V[a\x02\xAAV[a\0\xE7a\x01\n6`\x04a\x08\x9AV[a\x03\rV[a\x016\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x01`6`\x04a\x08\xF8V[a\x03xV[a\0\xE7a\x01s6`\x04a\tYV[a\x03\xECV[a\0\xE7a\x01\x866`\x04a\x08\x9AV[a\x04zV[a\0\xE7a\x01\x996`\x04a\t\xDDV[a\x04\xDAV[a\0\xE7a\x01\xAC6`\x04a\n\x1FV[a\x05FV[a\0\xE7a\x01\xBF6`\x04a\n\x90V[a\x05\xC4V[a\0\xE7a\x01\xD26`\x04a\n\xDBV[a\x063V[a\0\xE7a\x01\xE56`\x04a\n\x90V[a\x06\x9AV[a\0\xE7a\x01\xF86`\x04a\n\xDBV[a\x06\xFBV[a\x02M`@Q` \x01a\x02\x0F\x90a\x0B0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x07bV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\xDEt\xDC\xB4\xB9\x89M\x99\xAF\xEEI\xAB\xAA[\x8E\xEE\xCE\x83\x99B,@<Q(\xB2\x95\x8F\r\xD6\x18\x05\x90``\x01[`@Q\x80\x91\x03\x90\xA4PPPPPPV[a\x02\xBC`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\xF7\xE4'\x8E\x88\x05\xEE \x18SD\xA74\x9A\xAA\x91\xCD\xCF_1o\xBFf\x89OaJ3\xDE\xBD\x06y\x90``\x01a\x02\x9AV[a\x03\x1F`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7F\xB1IG\x02\x01`\r\x81\xE1i\x8A\x9D\0\x1D \xF8\xD2\xA06\xD2\xC0\xF4%Cke\xA1T\xE9^\xB5:\x90``\x01[`@Q\x80\x91\x03\x90\xA3PPPPPV[a\x03\x8A`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R\x84\x15\x15` \x82\x01R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\\\xA7\xF1\xE4/\xBA\xE6\xC2p$\xA86M\x7F\x80\xE1\xEC\xE7\x02\xFC\xCA\x05\xB4\r\xE6\x9E.\x8C\xBC\xA2\xFA\xC9\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x03\xFE`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x8B\x82\x16\x91\x8D\x16\x90\x7FS8U\xCB\x8A\x87\xF0\x99_l3\x90f^\xB3}\xA8\"\xF5U\x0B\x13\xA9s\xA7y\x06\xD0\x18\xA2\xA8\x9C\x90`\xE0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPV[a\x04\x8C`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7FN,\xA0Q^\xD1\xAE\xF19_f\xB50;\xB5\xD6\xF1\xBF\x9Da\xA3S\xFAS\xF7?\x8A\xC9\x97?\xA9\xF6\x90``\x01a\x03iV[a\x04\xEC`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\xC3 \xA8R\x9B\x15\xB8Q\xAA\xA6\x85\x19\x91\x9A\xC3D\xE1\xCA\xCE\xAFOG\xD1]\xF6\xE5\x81\x81\xDF\xECc\x19\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x05X`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x91\x89\x82\x16\x91\x8B\x16\x90\x7FRN/\xBE\x97b\xDE\xED\x9C/\x1E\x13<\no\xD9\x0E\x99\x1A\x8F\x87N\xDBh\xC5u\r\xC3\xE4>\xAA\xCD\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA4PPPPPPPPV[a\x05\xD6`@Q` \x01a\x02\x0F\x90a\x0B0V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FOxWI~\x11[\xB4\xE8\xAC\xD8\xD9\xF78.\xB4\xC1Y\xD8t\x83\x8Dch\t\x95#\xABn\xF9\xF61\x84`@Qa\x06%\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[a\x06E`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\xC7\x0F\xD1\xEB@\xD7\xAED\xA5B\\\t\xDE\xCEZ\x02;G'sV\xE0\x05\xB6\xCA\xEF\x16\0\xCF\xC73N\x90`\x80\x01a\x03\xDCV[a\x06\xAC`@Q` \x01a\x02\x0F\x90a\x0B0V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x7F1\x15\xD1D\x9A{s,\x98l\xBA\x18$N\x89zE\x0Fa\xE1\xBB\x8DX\x9C\xD2\xE6\x9El\x89$\xF9\xF7\x84`@Qa\x06%\x91\x81R` \x01\x90V[a\x07\r`@Q` \x01a\x02\x0F\x90a\x0B0V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\x9B\x07\x9D/\x87\xF5C\xB9\xB0lg2\xEAMs\xFD\x08\xF6\x9C/*Q\xDD\xE3\x14\xAD\x91P0*\xF7T\x90`\x80\x01a\x03\xDCV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xF1\x91\x90a\x0BTV[a\x08\x1BW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x08\x12\x92\x91\x90a\x0BxV[`@Q\x80\x91\x03\x90\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x086W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x08TW`\0\x80\xFD[a\x08]\x87a\x08\x1FV[\x95Pa\x08k` \x88\x01a\x08\x1FV[\x94Pa\x08y`@\x88\x01a\x08\x1FV[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x08\xB2W`\0\x80\xFD[a\x08\xBB\x86a\x08\x1FV[\x94Pa\x08\xC9` \x87\x01a\x08\x1FV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x08\xF5W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\x11W`\0\x80\xFD[a\t\x1A\x87a\x08\x1FV[\x95Pa\t(` \x88\x01a\x08\x1FV[\x94P`@\x87\x015\x93P``\x87\x015a\t?\x81a\x08\xE7V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\tyW`\0\x80\xFD[a\t\x82\x8Ba\x08\x1FV[\x99Pa\t\x90` \x8C\x01a\x08\x1FV[\x98Pa\t\x9E`@\x8C\x01a\x08\x1FV[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x81\x015\x98`\xA0\x82\x015\x98P`\xC0\x82\x015\x97P`\xE0\x82\x015\x96Pa\x01\0\x82\x015\x95Pa\x01 \x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\t\xF5W`\0\x80\xFD[a\t\xFE\x86a\x08\x1FV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x015\x94P\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\n<W`\0\x80\xFD[a\nE\x89a\x08\x1FV[\x97Pa\nS` \x8A\x01a\x08\x1FV[\x96Pa\na`@\x8A\x01a\x08\x1FV[\x97\x9A\x96\x99P\x96\x97``\x81\x015\x97P`\x80\x81\x015\x96`\xA0\x82\x015\x96P`\xC0\x82\x015\x95P`\xE0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\n\xA6W`\0\x80\xFD[a\n\xAF\x85a\x08\x1FV[\x93Pa\n\xBD` \x86\x01a\x08\x1FV[\x92Pa\n\xCB`@\x86\x01a\x08\x1FV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\n\xF4W`\0\x80\xFD[a\n\xFD\x87a\x08\x1FV[\x95Pa\x0B\x0B` \x88\x01a\x08\x1FV[\x95\x98\x95\x97PPPP`@\x84\x015\x93``\x81\x015\x93`\x80\x82\x015\x93P`\xA0\x90\x91\x015\x91PV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0BfW`\0\x80\xFD[\x81Qa\x0Bq\x81a\x08\xE7V[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x0B\xB4W` \x81\x86\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x0B\x97V[P`\0``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ~i| @\xEF\x82\xED\xD3\xAF*+\xF44\xAB\xF1bQ\xED25M=J\xFC\x83\xCF\x90g\xD0\x1E\xBEdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static EVENTEMITTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct EventEmitter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EventEmitter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EventEmitter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EventEmitter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EventEmitter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EventEmitter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EventEmitter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EVENTEMITTER_ABI.clone(),
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
                EVENTEMITTER_ABI.clone(),
                EVENTEMITTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `emitBorrow` (0xf002e0bd) function
        pub fn emit_borrow(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            borrow_amount: ::ethers::core::types::U256,
            borrow_rate: ::ethers::core::types::U256,
            collateral: ::ethers::core::types::U256,
            debt_scaled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 2, 224, 189],
                    (
                        underlying_asset,
                        account,
                        borrow_amount,
                        borrow_rate,
                        collateral,
                        debt_scaled,
                    ),
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
        ///Calls the contract's `emitClosePosition` (0x87a454ec) function
        pub fn emit_close_position(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            underlying_asset_usd: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            collateral_amount: ::ethers::core::types::U256,
            debt_amount: ::ethers::core::types::U256,
            remain_amount_usd: ::ethers::core::types::U256,
            collateral_usd: ::ethers::core::types::U256,
            debt_scaled_usd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [135, 164, 84, 236],
                    (
                        underlying_asset,
                        underlying_asset_usd,
                        account,
                        collateral_amount,
                        debt_amount,
                        remain_amount_usd,
                        collateral_usd,
                        debt_scaled_usd,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDeposit` (0x7ba59a8b) function
        pub fn emit_deposit(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            deposit_amount: ::ethers::core::types::U256,
            collateral: ::ethers::core::types::U256,
            debt_scaled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 165, 154, 139],
                    (underlying_asset, account, deposit_amount, collateral, debt_scaled),
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
        ///Calls the contract's `emitRedeem` (0x25ecca95) function
        pub fn emit_redeem(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            redeem_amount: ::ethers::core::types::U256,
            collateral: ::ethers::core::types::U256,
            debt_scaled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [37, 236, 202, 149],
                    (
                        underlying_asset,
                        account,
                        to,
                        redeem_amount,
                        collateral,
                        debt_scaled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitRepay` (0x5b63816e) function
        pub fn emit_repay(
            &self,
            underlying_asset: ::ethers::core::types::Address,
            repayer: ::ethers::core::types::Address,
            repay_amount: ::ethers::core::types::U256,
            use_collateral: bool,
            collateral: ::ethers::core::types::U256,
            debt_scaled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [91, 99, 129, 110],
                    (
                        underlying_asset,
                        repayer,
                        repay_amount,
                        use_collateral,
                        collateral,
                        debt_scaled,
                    ),
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
        ///Calls the contract's `emitSwap` (0x66e4d17a) function
        pub fn emit_swap(
            &self,
            underlying_asset_in: ::ethers::core::types::Address,
            underlying_asset_out: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            amount_out: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
            collateral_in: ::ethers::core::types::U256,
            debt_scaled_in: ::ethers::core::types::U256,
            collateral_out: ::ethers::core::types::U256,
            debt_scaled_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 228, 209, 122],
                    (
                        underlying_asset_in,
                        underlying_asset_out,
                        account,
                        amount_in,
                        amount_out,
                        fee,
                        collateral_in,
                        debt_scaled_in,
                        collateral_out,
                        debt_scaled_out,
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
        ///Gets the contract's `Borrow` event
        pub fn borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BorrowFilter> {
            self.0.event()
        }
        ///Gets the contract's `Close` event
        pub fn close_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CloseFilter> {
            self.0.event()
        }
        ///Gets the contract's `ClosePosition` event
        pub fn close_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClosePositionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Liquidation` event
        pub fn liquidation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolUpdated` event
        pub fn pool_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionLiquidation` event
        pub fn position_liquidation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionLiquidationFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Redeem` event
        pub fn redeem_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RedeemFilter> {
            self.0.event()
        }
        ///Gets the contract's `Repay` event
        pub fn repay_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RepayFilter> {
            self.0.event()
        }
        ///Gets the contract's `Supply` event
        pub fn supply_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SupplyFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EventEmitterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EventEmitter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Borrow",
        abi = "Borrow(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub borrower: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub borrow_rate: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Close", abi = "Close(address,address,uint256,uint256,uint256)")]
    pub struct CloseFilter {
        #[ethevent(indexed)]
        pub pool_usd: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount_usd_start_close: ::ethers::core::types::U256,
        pub amount_usd_after_repay_and_sell_collateral: ::ethers::core::types::U256,
        pub amount_usd_after_buy_collateral_and_repay: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ClosePosition",
        abi = "ClosePosition(address,address,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ClosePositionFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_usd: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub collateral: ::ethers::core::types::U256,
        pub debt: ::ethers::core::types::U256,
        pub remain_usd: ::ethers::core::types::U256,
        pub collateral_usd: ::ethers::core::types::U256,
        pub debt_scaled_usd: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Deposit",
        abi = "Deposit(address,address,uint256,uint256,uint256)"
    )]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub depositer: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Liquidation",
        abi = "Liquidation(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct LiquidationFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub health_factor: ::ethers::core::types::U256,
        pub health_factor_liquidation_threshold: ::ethers::core::types::U256,
        pub total_collateral_usd: ::ethers::core::types::U256,
        pub total_debt_usd: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PoolUpdated",
        abi = "PoolUpdated(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct PoolUpdatedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub liquidity_rate: ::ethers::core::types::U256,
        pub borrow_rate: ::ethers::core::types::U256,
        pub liquidity_index: ::ethers::core::types::U256,
        pub borrow_index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PositionLiquidation",
        abi = "PositionLiquidation(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct PositionLiquidationFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub collateral: ::ethers::core::types::U256,
        pub debt: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Redeem",
        abi = "Redeem(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct RedeemFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub redeemer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Repay",
        abi = "Repay(address,address,uint256,bool,uint256,uint256)"
    )]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub repayer: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub use_collateral: bool,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Supply", abi = "Supply(address,address,address,uint256)")]
    pub struct SupplyFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub supplier: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_in: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_out: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
        pub collateral_in: ::ethers::core::types::U256,
        pub debt_scaled_in: ::ethers::core::types::U256,
        pub collateral_out: ::ethers::core::types::U256,
        pub debt_scaled_out: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub withdrawer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum EventEmitterEvents {
        BorrowFilter(BorrowFilter),
        CloseFilter(CloseFilter),
        ClosePositionFilter(ClosePositionFilter),
        DepositFilter(DepositFilter),
        LiquidationFilter(LiquidationFilter),
        PoolUpdatedFilter(PoolUpdatedFilter),
        PositionLiquidationFilter(PositionLiquidationFilter),
        RedeemFilter(RedeemFilter),
        RepayFilter(RepayFilter),
        SupplyFilter(SupplyFilter),
        SwapFilter(SwapFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for EventEmitterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(EventEmitterEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = CloseFilter::decode_log(log) {
                return Ok(EventEmitterEvents::CloseFilter(decoded));
            }
            if let Ok(decoded) = ClosePositionFilter::decode_log(log) {
                return Ok(EventEmitterEvents::ClosePositionFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(EventEmitterEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = LiquidationFilter::decode_log(log) {
                return Ok(EventEmitterEvents::LiquidationFilter(decoded));
            }
            if let Ok(decoded) = PoolUpdatedFilter::decode_log(log) {
                return Ok(EventEmitterEvents::PoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PositionLiquidationFilter::decode_log(log) {
                return Ok(EventEmitterEvents::PositionLiquidationFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(EventEmitterEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(EventEmitterEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = SupplyFilter::decode_log(log) {
                return Ok(EventEmitterEvents::SupplyFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(EventEmitterEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(EventEmitterEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EventEmitterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BorrowFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClosePositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionLiquidationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BorrowFilter> for EventEmitterEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<CloseFilter> for EventEmitterEvents {
        fn from(value: CloseFilter) -> Self {
            Self::CloseFilter(value)
        }
    }
    impl ::core::convert::From<ClosePositionFilter> for EventEmitterEvents {
        fn from(value: ClosePositionFilter) -> Self {
            Self::ClosePositionFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for EventEmitterEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationFilter> for EventEmitterEvents {
        fn from(value: LiquidationFilter) -> Self {
            Self::LiquidationFilter(value)
        }
    }
    impl ::core::convert::From<PoolUpdatedFilter> for EventEmitterEvents {
        fn from(value: PoolUpdatedFilter) -> Self {
            Self::PoolUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PositionLiquidationFilter> for EventEmitterEvents {
        fn from(value: PositionLiquidationFilter) -> Self {
            Self::PositionLiquidationFilter(value)
        }
    }
    impl ::core::convert::From<RedeemFilter> for EventEmitterEvents {
        fn from(value: RedeemFilter) -> Self {
            Self::RedeemFilter(value)
        }
    }
    impl ::core::convert::From<RepayFilter> for EventEmitterEvents {
        fn from(value: RepayFilter) -> Self {
            Self::RepayFilter(value)
        }
    }
    impl ::core::convert::From<SupplyFilter> for EventEmitterEvents {
        fn from(value: SupplyFilter) -> Self {
            Self::SupplyFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for EventEmitterEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for EventEmitterEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `emitBorrow` function with signature `emitBorrow(address,address,uint256,uint256,uint256,uint256)` and selector `0xf002e0bd`
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
        name = "emitBorrow",
        abi = "emitBorrow(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct EmitBorrowCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub borrow_amount: ::ethers::core::types::U256,
        pub borrow_rate: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `emitClosePosition` function with signature `emitClosePosition(address,address,address,uint256,uint256,uint256,uint256,uint256)` and selector `0x87a454ec`
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
        abi = "emitClosePosition(address,address,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct EmitClosePositionCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub underlying_asset_usd: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub collateral_amount: ::ethers::core::types::U256,
        pub debt_amount: ::ethers::core::types::U256,
        pub remain_amount_usd: ::ethers::core::types::U256,
        pub collateral_usd: ::ethers::core::types::U256,
        pub debt_scaled_usd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitDeposit` function with signature `emitDeposit(address,address,uint256,uint256,uint256)` and selector `0x7ba59a8b`
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
        name = "emitDeposit",
        abi = "emitDeposit(address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitDepositCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub deposit_amount: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `emitRedeem` function with signature `emitRedeem(address,address,address,uint256,uint256,uint256)` and selector `0x25ecca95`
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
        name = "emitRedeem",
        abi = "emitRedeem(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct EmitRedeemCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub redeem_amount: ::ethers::core::types::U256,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitRepay` function with signature `emitRepay(address,address,uint256,bool,uint256,uint256)` and selector `0x5b63816e`
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
        name = "emitRepay",
        abi = "emitRepay(address,address,uint256,bool,uint256,uint256)"
    )]
    pub struct EmitRepayCall {
        pub underlying_asset: ::ethers::core::types::Address,
        pub repayer: ::ethers::core::types::Address,
        pub repay_amount: ::ethers::core::types::U256,
        pub use_collateral: bool,
        pub collateral: ::ethers::core::types::U256,
        pub debt_scaled: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `emitSwap` function with signature `emitSwap(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x66e4d17a`
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
        abi = "emitSwap(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct EmitSwapCall {
        pub underlying_asset_in: ::ethers::core::types::Address,
        pub underlying_asset_out: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
        pub collateral_in: ::ethers::core::types::U256,
        pub debt_scaled_in: ::ethers::core::types::U256,
        pub collateral_out: ::ethers::core::types::U256,
        pub debt_scaled_out: ::ethers::core::types::U256,
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
    pub enum EventEmitterCalls {
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
        RoleStore(RoleStoreCall),
    }
    impl ::ethers::core::abi::AbiDecode for EventEmitterCalls {
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
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EventEmitterCalls {
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
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EventEmitterCalls {
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
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmitBorrowCall> for EventEmitterCalls {
        fn from(value: EmitBorrowCall) -> Self {
            Self::EmitBorrow(value)
        }
    }
    impl ::core::convert::From<EmitCloseCall> for EventEmitterCalls {
        fn from(value: EmitCloseCall) -> Self {
            Self::EmitClose(value)
        }
    }
    impl ::core::convert::From<EmitClosePositionCall> for EventEmitterCalls {
        fn from(value: EmitClosePositionCall) -> Self {
            Self::EmitClosePosition(value)
        }
    }
    impl ::core::convert::From<EmitDepositCall> for EventEmitterCalls {
        fn from(value: EmitDepositCall) -> Self {
            Self::EmitDeposit(value)
        }
    }
    impl ::core::convert::From<EmitLiquidationCall> for EventEmitterCalls {
        fn from(value: EmitLiquidationCall) -> Self {
            Self::EmitLiquidation(value)
        }
    }
    impl ::core::convert::From<EmitPoolUpdatedCall> for EventEmitterCalls {
        fn from(value: EmitPoolUpdatedCall) -> Self {
            Self::EmitPoolUpdated(value)
        }
    }
    impl ::core::convert::From<EmitPositionLiquidationCall> for EventEmitterCalls {
        fn from(value: EmitPositionLiquidationCall) -> Self {
            Self::EmitPositionLiquidation(value)
        }
    }
    impl ::core::convert::From<EmitRedeemCall> for EventEmitterCalls {
        fn from(value: EmitRedeemCall) -> Self {
            Self::EmitRedeem(value)
        }
    }
    impl ::core::convert::From<EmitRepayCall> for EventEmitterCalls {
        fn from(value: EmitRepayCall) -> Self {
            Self::EmitRepay(value)
        }
    }
    impl ::core::convert::From<EmitSupplyCall> for EventEmitterCalls {
        fn from(value: EmitSupplyCall) -> Self {
            Self::EmitSupply(value)
        }
    }
    impl ::core::convert::From<EmitSwapCall> for EventEmitterCalls {
        fn from(value: EmitSwapCall) -> Self {
            Self::EmitSwap(value)
        }
    }
    impl ::core::convert::From<EmitWithdrawCall> for EventEmitterCalls {
        fn from(value: EmitWithdrawCall) -> Self {
            Self::EmitWithdraw(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for EventEmitterCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
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
