pub use errors::*;
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
pub mod errors {
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
                    ::std::borrow::ToOwned::to_owned("CollateralCanNotCoverDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralCanNotCoverDebt",
                            ),
                            inputs: ::std::vec![
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
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CompactedArrayOutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CompactedArrayOutOfBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("compactedValues"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slotIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("label"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CouldNotSendNativeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CouldNotSendNativeToken",
                            ),
                            inputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DebtTokenOperationNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DebtTokenOperationNotSupported",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DexEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DexEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyAccount"),
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
                    ::std::borrow::ToOwned::to_owned("EmptyBurnAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyBurnAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyDepositAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EmptyDepositAmounts",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyHoldingAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EmptyHoldingAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyMintAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyMintAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
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
                    ::std::borrow::ToOwned::to_owned("EmptyPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyPositions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
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
                    ::std::borrow::ToOwned::to_owned("EmptyRepayAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyRepayAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptySupplyAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptySupplyAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptySwapAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptySwapAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyTokenTranferGasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EmptyTokenTranferGasLimit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("EmptyWithdrawAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EmptyWithdrawAmounts",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ErrorStep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ErrorStep"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("step"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "HealthFactorHigherThanLiquidationThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HealthFactorHigherThanLiquidationThreshold",
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
                                        "healthFactorCollateralRateThreshold",
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
                    ::std::borrow::ToOwned::to_owned(
                        "HealthFactorLowerThanCollateralRateThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HealthFactorLowerThanCollateralRateThreshold",
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
                                        "healthFactorCollateralRateThreshold",
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
                    ::std::borrow::ToOwned::to_owned("InsufficientAvailableLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAvailableLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientCollateralAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientCollateralAmount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToRemove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBalance"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "InsufficientCollateralAmountForRepay",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientCollateralAmountForRepay",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("collateralAmount"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientCollateralForSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientCollateralForSwap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapNeedamountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientDexLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientDexLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("avaiableDexAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientUserBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientUserBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("userBalance"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidNativeTokenSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidNativeTokenSender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidOptimalUsageRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOptimalUsageRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("optimalUsageRatio"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidOracleDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOracleDecimals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracleDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("MaxOracleDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidOraclePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOraclePrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPoolIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPoolIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolKeyId"),
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
                    ::std::borrow::ToOwned::to_owned("OracleEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OracleEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PoolAlreadyExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolToken"),
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
                (
                    ::std::borrow::ToOwned::to_owned("PositionNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PositionNotFound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SelfTransferNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SelfTransferNotSupported",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("SupplyCapacityExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SupplyCapacityExceeded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalSupplyAddUnclaimedFeeAddAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapPoolsNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SwapPoolsNotMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool1"),
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
                    ::std::borrow::ToOwned::to_owned("ThereMustBeAtLeastOneRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ThereMustBeAtLeastOneRoleAdmin",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ThereMustBeAtLeastOneTimelockMultiSig",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ThereMustBeAtLeastOneTimelockMultiSig",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenCanNotSwapWithSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenCanNotSwapWithSelf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("TokenNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenNotMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("TokenTransferError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenTransferError"),
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
                (
                    ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UsdCollateralCanNotCoverDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UsdCollateralCanNotCoverDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "usdCollateralAmount",
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
                                        "usdCollateralAmountNeeded",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
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
                    ::std::borrow::ToOwned::to_owned("UsdNotHaveLongOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UsdNotHaveLongOperation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UsdNotHaveShortOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UsdNotHaveShortOperation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UserDoNotHaveDebtInPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UserDoNotHaveDebtInPool",
                            ),
                            inputs: ::std::vec![
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
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`V`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x82V\x99b\xC7\x04:\x05E\xAD\xCDO'\x03\xB0\x8ERS\x10&\x1Cdb\x04**R\xAD0\x83\xFE\xCDdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ERRORS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x82V\x99b\xC7\x04:\x05E\xAD\xCDO'\x03\xB0\x8ERS\x10&\x1Cdb\x04**R\xAD0\x83\xFE\xCDdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ERRORS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Errors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Errors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Errors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Errors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Errors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Errors)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Errors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERRORS_ABI.clone(),
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
                ERRORS_ABI.clone(),
                ERRORS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Errors<M> {
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
    ///Custom Error type `CollateralCanNotCoverDebt` with signature `CollateralCanNotCoverDebt(uint256,uint256)` and selector `0xac3fb11d`
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
        name = "CollateralCanNotCoverDebt",
        abi = "CollateralCanNotCoverDebt(uint256,uint256)"
    )]
    pub struct CollateralCanNotCoverDebt {
        pub collateral_amount: ::ethers::core::types::U256,
        pub debt_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `CompactedArrayOutOfBounds` with signature `CompactedArrayOutOfBounds(uint256[],uint256,uint256,string)` and selector `0xbdec9c0d`
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
        name = "CompactedArrayOutOfBounds",
        abi = "CompactedArrayOutOfBounds(uint256[],uint256,uint256,string)"
    )]
    pub struct CompactedArrayOutOfBounds {
        pub compacted_values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub index: ::ethers::core::types::U256,
        pub slot_index: ::ethers::core::types::U256,
        pub label: ::std::string::String,
    }
    ///Custom Error type `CouldNotSendNativeToken` with signature `CouldNotSendNativeToken(address,uint256)` and selector `0x321c1666`
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
        name = "CouldNotSendNativeToken",
        abi = "CouldNotSendNativeToken(address,uint256)"
    )]
    pub struct CouldNotSendNativeToken {
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `DebtTokenOperationNotSupported` with signature `DebtTokenOperationNotSupported()` and selector `0xe8964514`
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
        name = "DebtTokenOperationNotSupported",
        abi = "DebtTokenOperationNotSupported()"
    )]
    pub struct DebtTokenOperationNotSupported;
    ///Custom Error type `DexEmpty` with signature `DexEmpty()` and selector `0x9f424ee8`
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
    #[etherror(name = "DexEmpty", abi = "DexEmpty()")]
    pub struct DexEmpty;
    ///Custom Error type `EmptyAccount` with signature `EmptyAccount()` and selector `0xdd7016a2`
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
    #[etherror(name = "EmptyAccount", abi = "EmptyAccount()")]
    pub struct EmptyAccount;
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
    ///Custom Error type `EmptyBurnAmounts` with signature `EmptyBurnAmounts()` and selector `0xf8badfd8`
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
    #[etherror(name = "EmptyBurnAmounts", abi = "EmptyBurnAmounts()")]
    pub struct EmptyBurnAmounts;
    ///Custom Error type `EmptyDepositAmounts` with signature `EmptyDepositAmounts()` and selector `0x01af8c24`
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
    #[etherror(name = "EmptyDepositAmounts", abi = "EmptyDepositAmounts()")]
    pub struct EmptyDepositAmounts;
    ///Custom Error type `EmptyHoldingAddress` with signature `EmptyHoldingAddress()` and selector `0xe9b78bd4`
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
    #[etherror(name = "EmptyHoldingAddress", abi = "EmptyHoldingAddress()")]
    pub struct EmptyHoldingAddress;
    ///Custom Error type `EmptyMintAmounts` with signature `EmptyMintAmounts()` and selector `0x75cf7209`
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
    #[etherror(name = "EmptyMintAmounts", abi = "EmptyMintAmounts()")]
    pub struct EmptyMintAmounts;
    ///Custom Error type `EmptyOracle` with signature `EmptyOracle(address)` and selector `0xadb29466`
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
    #[etherror(name = "EmptyOracle", abi = "EmptyOracle(address)")]
    pub struct EmptyOracle {
        pub underlying_asset: ::ethers::core::types::Address,
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
    ///Custom Error type `EmptyPositions` with signature `EmptyPositions(address)` and selector `0x73df2cd0`
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
    #[etherror(name = "EmptyPositions", abi = "EmptyPositions(address)")]
    pub struct EmptyPositions {
        pub account: ::ethers::core::types::Address,
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
    ///Custom Error type `EmptyRepayAmount` with signature `EmptyRepayAmount()` and selector `0x018439d6`
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
    #[etherror(name = "EmptyRepayAmount", abi = "EmptyRepayAmount()")]
    pub struct EmptyRepayAmount;
    ///Custom Error type `EmptySupplyAmounts` with signature `EmptySupplyAmounts()` and selector `0xd52fec05`
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
    #[etherror(name = "EmptySupplyAmounts", abi = "EmptySupplyAmounts()")]
    pub struct EmptySupplyAmounts;
    ///Custom Error type `EmptySwapAmount` with signature `EmptySwapAmount()` and selector `0xe043049d`
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
    #[etherror(name = "EmptySwapAmount", abi = "EmptySwapAmount()")]
    pub struct EmptySwapAmount;
    ///Custom Error type `EmptyTokenTranferGasLimit` with signature `EmptyTokenTranferGasLimit(address)` and selector `0x9fc297fa`
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
        name = "EmptyTokenTranferGasLimit",
        abi = "EmptyTokenTranferGasLimit(address)"
    )]
    pub struct EmptyTokenTranferGasLimit {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `EmptyWithdrawAmounts` with signature `EmptyWithdrawAmounts()` and selector `0xfd9e68c4`
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
    #[etherror(name = "EmptyWithdrawAmounts", abi = "EmptyWithdrawAmounts()")]
    pub struct EmptyWithdrawAmounts;
    ///Custom Error type `ErrorStep` with signature `ErrorStep(address,uint256)` and selector `0xe8d2ae39`
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
    #[etherror(name = "ErrorStep", abi = "ErrorStep(address,uint256)")]
    pub struct ErrorStep {
        pub account: ::ethers::core::types::Address,
        pub step: ::ethers::core::types::U256,
    }
    ///Custom Error type `HealthFactorHigherThanLiquidationThreshold` with signature `HealthFactorHigherThanLiquidationThreshold(uint256,uint256)` and selector `0xb7ae955e`
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
        name = "HealthFactorHigherThanLiquidationThreshold",
        abi = "HealthFactorHigherThanLiquidationThreshold(uint256,uint256)"
    )]
    pub struct HealthFactorHigherThanLiquidationThreshold {
        pub health_factor: ::ethers::core::types::U256,
        pub health_factor_collateral_rate_threshold: ::ethers::core::types::U256,
    }
    ///Custom Error type `HealthFactorLowerThanCollateralRateThreshold` with signature `HealthFactorLowerThanCollateralRateThreshold(uint256,uint256)` and selector `0xc486d352`
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
        name = "HealthFactorLowerThanCollateralRateThreshold",
        abi = "HealthFactorLowerThanCollateralRateThreshold(uint256,uint256)"
    )]
    pub struct HealthFactorLowerThanCollateralRateThreshold {
        pub health_factor: ::ethers::core::types::U256,
        pub health_factor_collateral_rate_threshold: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `InsufficientAvailableLiquidity` with signature `InsufficientAvailableLiquidity(uint256,uint256)` and selector `0xaf9a97d4`
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
        name = "InsufficientAvailableLiquidity",
        abi = "InsufficientAvailableLiquidity(uint256,uint256)"
    )]
    pub struct InsufficientAvailableLiquidity {
        pub amount: ::ethers::core::types::U256,
        pub available_liquidity: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientCollateralAmount` with signature `InsufficientCollateralAmount(uint256,uint256)` and selector `0x1f4824b4`
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
        name = "InsufficientCollateralAmount",
        abi = "InsufficientCollateralAmount(uint256,uint256)"
    )]
    pub struct InsufficientCollateralAmount {
        pub amount_to_remove: ::ethers::core::types::U256,
        pub amount_balance: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientCollateralAmountForRepay` with signature `InsufficientCollateralAmountForRepay(uint256,uint256)` and selector `0x86c4ec21`
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
        name = "InsufficientCollateralAmountForRepay",
        abi = "InsufficientCollateralAmountForRepay(uint256,uint256)"
    )]
    pub struct InsufficientCollateralAmountForRepay {
        pub repay_amount: ::ethers::core::types::U256,
        pub collateral_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientCollateralForSwap` with signature `InsufficientCollateralForSwap(uint256,uint256)` and selector `0xf3a6116b`
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
        name = "InsufficientCollateralForSwap",
        abi = "InsufficientCollateralForSwap(uint256,uint256)"
    )]
    pub struct InsufficientCollateralForSwap {
        pub swap_needamount_in: ::ethers::core::types::U256,
        pub collateral_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientDexLiquidity` with signature `InsufficientDexLiquidity(uint256,uint256)` and selector `0xc7eaec87`
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
        name = "InsufficientDexLiquidity",
        abi = "InsufficientDexLiquidity(uint256,uint256)"
    )]
    pub struct InsufficientDexLiquidity {
        pub avaiable_dex_amount: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
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
    ///Custom Error type `InsufficientUserBalance` with signature `InsufficientUserBalance(uint256,uint256)` and selector `0x5f504d20`
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
        name = "InsufficientUserBalance",
        abi = "InsufficientUserBalance(uint256,uint256)"
    )]
    pub struct InsufficientUserBalance {
        pub amount: ::ethers::core::types::U256,
        pub user_balance: ::ethers::core::types::U256,
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
    ///Custom Error type `InvalidNativeTokenSender` with signature `InvalidNativeTokenSender(address)` and selector `0xe71a51be`
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
        name = "InvalidNativeTokenSender",
        abi = "InvalidNativeTokenSender(address)"
    )]
    pub struct InvalidNativeTokenSender {
        pub msg_sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidOptimalUsageRate` with signature `InvalidOptimalUsageRate(uint256)` and selector `0xb90df3f2`
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
        name = "InvalidOptimalUsageRate",
        abi = "InvalidOptimalUsageRate(uint256)"
    )]
    pub struct InvalidOptimalUsageRate {
        pub optimal_usage_ratio: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidOracleDecimals` with signature `InvalidOracleDecimals(uint256,uint256)` and selector `0xe25421ba`
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
        name = "InvalidOracleDecimals",
        abi = "InvalidOracleDecimals(uint256,uint256)"
    )]
    pub struct InvalidOracleDecimals {
        pub oracle_decimals: ::ethers::core::types::U256,
        pub max_oracle_decimals: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidOraclePrice` with signature `InvalidOraclePrice(address,int256)` and selector `0xa0bdf75a`
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
    #[etherror(name = "InvalidOraclePrice", abi = "InvalidOraclePrice(address,int256)")]
    pub struct InvalidOraclePrice {
        pub underlying_asset: ::ethers::core::types::Address,
        pub price: ::ethers::core::types::I256,
    }
    ///Custom Error type `InvalidPoolIndex` with signature `InvalidPoolIndex(uint256)` and selector `0x782cbada`
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
    #[etherror(name = "InvalidPoolIndex", abi = "InvalidPoolIndex(uint256)")]
    pub struct InvalidPoolIndex {
        pub pool_key_id: ::ethers::core::types::U256,
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
    ///Custom Error type `OracleEmpty` with signature `OracleEmpty()` and selector `0xd3637d35`
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
    #[etherror(name = "OracleEmpty", abi = "OracleEmpty()")]
    pub struct OracleEmpty;
    ///Custom Error type `PoolAlreadyExists` with signature `PoolAlreadyExists(address,address)` and selector `0x791ffc82`
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
    #[etherror(name = "PoolAlreadyExists", abi = "PoolAlreadyExists(address,address)")]
    pub struct PoolAlreadyExists {
        pub key: ::ethers::core::types::Address,
        pub pool_token: ::ethers::core::types::Address,
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
    ///Custom Error type `PositionNotFound` with signature `PositionNotFound(bytes32)` and selector `0x426cfff0`
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
    #[etherror(name = "PositionNotFound", abi = "PositionNotFound(bytes32)")]
    pub struct PositionNotFound {
        pub key: [u8; 32],
    }
    ///Custom Error type `SelfTransferNotSupported` with signature `SelfTransferNotSupported(address)` and selector `0xe70f9152`
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
        name = "SelfTransferNotSupported",
        abi = "SelfTransferNotSupported(address)"
    )]
    pub struct SelfTransferNotSupported {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `SupplyCapacityExceeded` with signature `SupplyCapacityExceeded(uint256,uint256)` and selector `0xe321f56b`
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
        name = "SupplyCapacityExceeded",
        abi = "SupplyCapacityExceeded(uint256,uint256)"
    )]
    pub struct SupplyCapacityExceeded {
        pub total_supply_add_unclaimed_fee_add_amount: ::ethers::core::types::U256,
        pub supply_capacity: ::ethers::core::types::U256,
    }
    ///Custom Error type `SwapPoolsNotMatch` with signature `SwapPoolsNotMatch(address,address)` and selector `0xe9aae252`
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
    #[etherror(name = "SwapPoolsNotMatch", abi = "SwapPoolsNotMatch(address,address)")]
    pub struct SwapPoolsNotMatch {
        pub pool_0: ::ethers::core::types::Address,
        pub pool_1: ::ethers::core::types::Address,
    }
    ///Custom Error type `ThereMustBeAtLeastOneRoleAdmin` with signature `ThereMustBeAtLeastOneRoleAdmin()` and selector `0xb783c88a`
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
        name = "ThereMustBeAtLeastOneRoleAdmin",
        abi = "ThereMustBeAtLeastOneRoleAdmin()"
    )]
    pub struct ThereMustBeAtLeastOneRoleAdmin;
    ///Custom Error type `ThereMustBeAtLeastOneTimelockMultiSig` with signature `ThereMustBeAtLeastOneTimelockMultiSig()` and selector `0x282b5b70`
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
        name = "ThereMustBeAtLeastOneTimelockMultiSig",
        abi = "ThereMustBeAtLeastOneTimelockMultiSig()"
    )]
    pub struct ThereMustBeAtLeastOneTimelockMultiSig;
    ///Custom Error type `TokenCanNotSwapWithSelf` with signature `TokenCanNotSwapWithSelf(address)` and selector `0xecfbebe4`
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
        name = "TokenCanNotSwapWithSelf",
        abi = "TokenCanNotSwapWithSelf(address)"
    )]
    pub struct TokenCanNotSwapWithSelf {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `TokenNotMatch` with signature `TokenNotMatch(address,address,address,address)` and selector `0x94604de9`
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
        name = "TokenNotMatch",
        abi = "TokenNotMatch(address,address,address,address)"
    )]
    pub struct TokenNotMatch {
        pub pool: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `TokenTransferError` with signature `TokenTransferError(address,address,uint256)` and selector `0x979dc780`
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
        name = "TokenTransferError",
        abi = "TokenTransferError(address,address,uint256)"
    )]
    pub struct TokenTransferError {
        pub token: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    ///Custom Error type `UnderlyAssetEmpty` with signature `UnderlyAssetEmpty()` and selector `0x08689653`
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
    #[etherror(name = "UnderlyAssetEmpty", abi = "UnderlyAssetEmpty()")]
    pub struct UnderlyAssetEmpty;
    ///Custom Error type `UsdCollateralCanNotCoverDebt` with signature `UsdCollateralCanNotCoverDebt(uint256,uint256,uint256,address)` and selector `0xb2a4b254`
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
        name = "UsdCollateralCanNotCoverDebt",
        abi = "UsdCollateralCanNotCoverDebt(uint256,uint256,uint256,address)"
    )]
    pub struct UsdCollateralCanNotCoverDebt {
        pub usd_collateral_amount: ::ethers::core::types::U256,
        pub usd_collateral_amount_needed: ::ethers::core::types::U256,
        pub debt_amount: ::ethers::core::types::U256,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Custom Error type `UsdNotHaveLongOperation` with signature `UsdNotHaveLongOperation()` and selector `0xea28cc60`
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
    #[etherror(name = "UsdNotHaveLongOperation", abi = "UsdNotHaveLongOperation()")]
    pub struct UsdNotHaveLongOperation;
    ///Custom Error type `UsdNotHaveShortOperation` with signature `UsdNotHaveShortOperation()` and selector `0x26601d80`
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
    #[etherror(name = "UsdNotHaveShortOperation", abi = "UsdNotHaveShortOperation()")]
    pub struct UsdNotHaveShortOperation;
    ///Custom Error type `UserDoNotHaveDebtInPool` with signature `UserDoNotHaveDebtInPool(address,address)` and selector `0x4a520d24`
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
        name = "UserDoNotHaveDebtInPool",
        abi = "UserDoNotHaveDebtInPool(address,address)"
    )]
    pub struct UserDoNotHaveDebtInPool {
        pub account: ::ethers::core::types::Address,
        pub pool_key: ::ethers::core::types::Address,
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
    pub enum ErrorsErrors {
        BorrowCapacityExceeded(BorrowCapacityExceeded),
        CollateralBalanceIsZero(CollateralBalanceIsZero),
        CollateralCanNotCoverDebt(CollateralCanNotCoverDebt),
        CompactedArrayOutOfBounds(CompactedArrayOutOfBounds),
        CouldNotSendNativeToken(CouldNotSendNativeToken),
        DebtTokenOperationNotSupported(DebtTokenOperationNotSupported),
        DexEmpty(DexEmpty),
        EmptyAccount(EmptyAccount),
        EmptyBorrowAmounts(EmptyBorrowAmounts),
        EmptyBurnAmounts(EmptyBurnAmounts),
        EmptyDepositAmounts(EmptyDepositAmounts),
        EmptyHoldingAddress(EmptyHoldingAddress),
        EmptyMintAmounts(EmptyMintAmounts),
        EmptyOracle(EmptyOracle),
        EmptyPosition(EmptyPosition),
        EmptyPositions(EmptyPositions),
        EmptyReceiver(EmptyReceiver),
        EmptyRedeemAmount(EmptyRedeemAmount),
        EmptyRepayAmount(EmptyRepayAmount),
        EmptySupplyAmounts(EmptySupplyAmounts),
        EmptySwapAmount(EmptySwapAmount),
        EmptyTokenTranferGasLimit(EmptyTokenTranferGasLimit),
        EmptyWithdrawAmounts(EmptyWithdrawAmounts),
        ErrorStep(ErrorStep),
        HealthFactorHigherThanLiquidationThreshold(
            HealthFactorHigherThanLiquidationThreshold,
        ),
        HealthFactorLowerThanCollateralRateThreshold(
            HealthFactorLowerThanCollateralRateThreshold,
        ),
        HealthFactorLowerThanLiquidationThreshold(
            HealthFactorLowerThanLiquidationThreshold,
        ),
        InsufficientAvailableLiquidity(InsufficientAvailableLiquidity),
        InsufficientCollateralAmount(InsufficientCollateralAmount),
        InsufficientCollateralAmountForRepay(InsufficientCollateralAmountForRepay),
        InsufficientCollateralForSwap(InsufficientCollateralForSwap),
        InsufficientDexLiquidity(InsufficientDexLiquidity),
        InsufficientLiquidityForBorrow(InsufficientLiquidityForBorrow),
        InsufficientUserBalance(InsufficientUserBalance),
        InvalidBorrowCapacity(InvalidBorrowCapacity),
        InvalidDecimals(InvalidDecimals),
        InvalidFeeFactor(InvalidFeeFactor),
        InvalidNativeTokenSender(InvalidNativeTokenSender),
        InvalidOptimalUsageRate(InvalidOptimalUsageRate),
        InvalidOracleDecimals(InvalidOracleDecimals),
        InvalidOraclePrice(InvalidOraclePrice),
        InvalidPoolIndex(InvalidPoolIndex),
        InvalidSupplyCapacity(InvalidSupplyCapacity),
        OracleEmpty(OracleEmpty),
        PoolAlreadyExists(PoolAlreadyExists),
        PoolIsFrozen(PoolIsFrozen),
        PoolIsInactive(PoolIsInactive),
        PoolIsNotBorrowing(PoolIsNotBorrowing),
        PoolIsPaused(PoolIsPaused),
        PoolNotFound(PoolNotFound),
        PositionNotFound(PositionNotFound),
        SelfTransferNotSupported(SelfTransferNotSupported),
        SupplyCapacityExceeded(SupplyCapacityExceeded),
        SwapPoolsNotMatch(SwapPoolsNotMatch),
        ThereMustBeAtLeastOneRoleAdmin(ThereMustBeAtLeastOneRoleAdmin),
        ThereMustBeAtLeastOneTimelockMultiSig(ThereMustBeAtLeastOneTimelockMultiSig),
        TokenCanNotSwapWithSelf(TokenCanNotSwapWithSelf),
        TokenNotMatch(TokenNotMatch),
        TokenTransferError(TokenTransferError),
        Unauthorized(Unauthorized),
        UnderlyAssetEmpty(UnderlyAssetEmpty),
        UsdCollateralCanNotCoverDebt(UsdCollateralCanNotCoverDebt),
        UsdNotHaveLongOperation(UsdNotHaveLongOperation),
        UsdNotHaveShortOperation(UsdNotHaveShortOperation),
        UserDoNotHaveDebtInPool(UserDoNotHaveDebtInPool),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ErrorsErrors {
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
            if let Ok(decoded) = <CollateralCanNotCoverDebt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralCanNotCoverDebt(decoded));
            }
            if let Ok(decoded) = <CompactedArrayOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompactedArrayOutOfBounds(decoded));
            }
            if let Ok(decoded) = <CouldNotSendNativeToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CouldNotSendNativeToken(decoded));
            }
            if let Ok(decoded) = <DebtTokenOperationNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtTokenOperationNotSupported(decoded));
            }
            if let Ok(decoded) = <DexEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DexEmpty(decoded));
            }
            if let Ok(decoded) = <EmptyAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyAccount(decoded));
            }
            if let Ok(decoded) = <EmptyBorrowAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyBorrowAmounts(decoded));
            }
            if let Ok(decoded) = <EmptyBurnAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyBurnAmounts(decoded));
            }
            if let Ok(decoded) = <EmptyDepositAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyDepositAmounts(decoded));
            }
            if let Ok(decoded) = <EmptyHoldingAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyHoldingAddress(decoded));
            }
            if let Ok(decoded) = <EmptyMintAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyMintAmounts(decoded));
            }
            if let Ok(decoded) = <EmptyOracle as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyOracle(decoded));
            }
            if let Ok(decoded) = <EmptyPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyPosition(decoded));
            }
            if let Ok(decoded) = <EmptyPositions as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyPositions(decoded));
            }
            if let Ok(decoded) = <EmptyReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyReceiver(decoded));
            }
            if let Ok(decoded) = <EmptyRedeemAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyRedeemAmount(decoded));
            }
            if let Ok(decoded) = <EmptyRepayAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyRepayAmount(decoded));
            }
            if let Ok(decoded) = <EmptySupplyAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptySupplyAmounts(decoded));
            }
            if let Ok(decoded) = <EmptySwapAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptySwapAmount(decoded));
            }
            if let Ok(decoded) = <EmptyTokenTranferGasLimit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyTokenTranferGasLimit(decoded));
            }
            if let Ok(decoded) = <EmptyWithdrawAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyWithdrawAmounts(decoded));
            }
            if let Ok(decoded) = <ErrorStep as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ErrorStep(decoded));
            }
            if let Ok(decoded) = <HealthFactorHigherThanLiquidationThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorHigherThanLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <HealthFactorLowerThanCollateralRateThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorLowerThanCollateralRateThreshold(decoded));
            }
            if let Ok(decoded) = <HealthFactorLowerThanLiquidationThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorLowerThanLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <InsufficientAvailableLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientAvailableLiquidity(decoded));
            }
            if let Ok(decoded) = <InsufficientCollateralAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientCollateralAmount(decoded));
            }
            if let Ok(decoded) = <InsufficientCollateralAmountForRepay as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientCollateralAmountForRepay(decoded));
            }
            if let Ok(decoded) = <InsufficientCollateralForSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientCollateralForSwap(decoded));
            }
            if let Ok(decoded) = <InsufficientDexLiquidity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientDexLiquidity(decoded));
            }
            if let Ok(decoded) = <InsufficientLiquidityForBorrow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientLiquidityForBorrow(decoded));
            }
            if let Ok(decoded) = <InsufficientUserBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientUserBalance(decoded));
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
            if let Ok(decoded) = <InvalidNativeTokenSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidNativeTokenSender(decoded));
            }
            if let Ok(decoded) = <InvalidOptimalUsageRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOptimalUsageRate(decoded));
            }
            if let Ok(decoded) = <InvalidOracleDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOracleDecimals(decoded));
            }
            if let Ok(decoded) = <InvalidOraclePrice as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOraclePrice(decoded));
            }
            if let Ok(decoded) = <InvalidPoolIndex as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPoolIndex(decoded));
            }
            if let Ok(decoded) = <InvalidSupplyCapacity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSupplyCapacity(decoded));
            }
            if let Ok(decoded) = <OracleEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OracleEmpty(decoded));
            }
            if let Ok(decoded) = <PoolAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolAlreadyExists(decoded));
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
            if let Ok(decoded) = <PositionNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionNotFound(decoded));
            }
            if let Ok(decoded) = <SelfTransferNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfTransferNotSupported(decoded));
            }
            if let Ok(decoded) = <SupplyCapacityExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupplyCapacityExceeded(decoded));
            }
            if let Ok(decoded) = <SwapPoolsNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapPoolsNotMatch(decoded));
            }
            if let Ok(decoded) = <ThereMustBeAtLeastOneRoleAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ThereMustBeAtLeastOneRoleAdmin(decoded));
            }
            if let Ok(decoded) = <ThereMustBeAtLeastOneTimelockMultiSig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ThereMustBeAtLeastOneTimelockMultiSig(decoded));
            }
            if let Ok(decoded) = <TokenCanNotSwapWithSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenCanNotSwapWithSelf(decoded));
            }
            if let Ok(decoded) = <TokenNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenNotMatch(decoded));
            }
            if let Ok(decoded) = <TokenTransferError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransferError(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded) = <UnderlyAssetEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyAssetEmpty(decoded));
            }
            if let Ok(decoded) = <UsdCollateralCanNotCoverDebt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsdCollateralCanNotCoverDebt(decoded));
            }
            if let Ok(decoded) = <UsdNotHaveLongOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsdNotHaveLongOperation(decoded));
            }
            if let Ok(decoded) = <UsdNotHaveShortOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UsdNotHaveShortOperation(decoded));
            }
            if let Ok(decoded) = <UserDoNotHaveDebtInPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserDoNotHaveDebtInPool(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BorrowCapacityExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollateralCanNotCoverDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompactedArrayOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CouldNotSendNativeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtTokenOperationNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DexEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyBorrowAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyBurnAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyDepositAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyHoldingAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyMintAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyRedeemAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyRepayAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptySupplyAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptySwapAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyTokenTranferGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyWithdrawAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ErrorStep(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorHigherThanLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLowerThanCollateralRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAvailableLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientCollateralAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientCollateralAmountForRepay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientCollateralForSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientDexLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientLiquidityForBorrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientUserBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBorrowCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNativeTokenSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOptimalUsageRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOracleDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOraclePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPoolIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSupplyCapacity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OracleEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAlreadyExists(element) => {
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
                Self::PositionNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfTransferNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyCapacityExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapPoolsNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ThereMustBeAtLeastOneRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ThereMustBeAtLeastOneTimelockMultiSig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenCanNotSwapWithSelf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyAssetEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsdCollateralCanNotCoverDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsdNotHaveLongOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsdNotHaveShortOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserDoNotHaveDebtInPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ErrorsErrors {
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
                    == <CollateralCanNotCoverDebt as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CompactedArrayOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CouldNotSendNativeToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DebtTokenOperationNotSupported as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DexEmpty as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EmptyAccount as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EmptyBorrowAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyBurnAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyDepositAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyHoldingAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyMintAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyOracle as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EmptyPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyPositions as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyRedeemAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyRepayAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptySupplyAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptySwapAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyTokenTranferGasLimit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyWithdrawAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ErrorStep as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <HealthFactorHigherThanLiquidationThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HealthFactorLowerThanCollateralRateThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <HealthFactorLowerThanLiquidationThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAvailableLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientCollateralAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientCollateralAmountForRepay as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientCollateralForSwap as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientDexLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientLiquidityForBorrow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientUserBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                    == <InvalidNativeTokenSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOptimalUsageRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOracleDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOraclePrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPoolIndex as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSupplyCapacity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OracleEmpty as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PoolAlreadyExists as ::ethers::contract::EthError>::selector() => {
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
                _ if selector
                    == <PositionNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SelfTransferNotSupported as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SupplyCapacityExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapPoolsNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ThereMustBeAtLeastOneRoleAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ThereMustBeAtLeastOneTimelockMultiSig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenCanNotSwapWithSelf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnderlyAssetEmpty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UsdCollateralCanNotCoverDebt as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UsdNotHaveLongOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UsdNotHaveShortOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UserDoNotHaveDebtInPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BorrowCapacityExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralBalanceIsZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralCanNotCoverDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompactedArrayOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CouldNotSendNativeToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtTokenOperationNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DexEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyAccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyBorrowAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptyBurnAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyDepositAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptyHoldingAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptyMintAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyRedeemAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyRepayAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptySupplyAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptySwapAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyTokenTranferGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmptyWithdrawAmounts(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ErrorStep(element) => ::core::fmt::Display::fmt(element, f),
                Self::HealthFactorHigherThanLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorLowerThanCollateralRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorLowerThanLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAvailableLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientCollateralAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientCollateralAmountForRepay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientCollateralForSwap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientDexLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientLiquidityForBorrow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientUserBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidBorrowCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNativeTokenSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOptimalUsageRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOracleDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOraclePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPoolIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSupplyCapacity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OracleEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsInactive(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolIsNotBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolIsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfTransferNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupplyCapacityExceeded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapPoolsNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ThereMustBeAtLeastOneRoleAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ThereMustBeAtLeastOneTimelockMultiSig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenCanNotSwapWithSelf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenTransferError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyAssetEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsdCollateralCanNotCoverDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UsdNotHaveLongOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UsdNotHaveShortOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserDoNotHaveDebtInPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BorrowCapacityExceeded> for ErrorsErrors {
        fn from(value: BorrowCapacityExceeded) -> Self {
            Self::BorrowCapacityExceeded(value)
        }
    }
    impl ::core::convert::From<CollateralBalanceIsZero> for ErrorsErrors {
        fn from(value: CollateralBalanceIsZero) -> Self {
            Self::CollateralBalanceIsZero(value)
        }
    }
    impl ::core::convert::From<CollateralCanNotCoverDebt> for ErrorsErrors {
        fn from(value: CollateralCanNotCoverDebt) -> Self {
            Self::CollateralCanNotCoverDebt(value)
        }
    }
    impl ::core::convert::From<CompactedArrayOutOfBounds> for ErrorsErrors {
        fn from(value: CompactedArrayOutOfBounds) -> Self {
            Self::CompactedArrayOutOfBounds(value)
        }
    }
    impl ::core::convert::From<CouldNotSendNativeToken> for ErrorsErrors {
        fn from(value: CouldNotSendNativeToken) -> Self {
            Self::CouldNotSendNativeToken(value)
        }
    }
    impl ::core::convert::From<DebtTokenOperationNotSupported> for ErrorsErrors {
        fn from(value: DebtTokenOperationNotSupported) -> Self {
            Self::DebtTokenOperationNotSupported(value)
        }
    }
    impl ::core::convert::From<DexEmpty> for ErrorsErrors {
        fn from(value: DexEmpty) -> Self {
            Self::DexEmpty(value)
        }
    }
    impl ::core::convert::From<EmptyAccount> for ErrorsErrors {
        fn from(value: EmptyAccount) -> Self {
            Self::EmptyAccount(value)
        }
    }
    impl ::core::convert::From<EmptyBorrowAmounts> for ErrorsErrors {
        fn from(value: EmptyBorrowAmounts) -> Self {
            Self::EmptyBorrowAmounts(value)
        }
    }
    impl ::core::convert::From<EmptyBurnAmounts> for ErrorsErrors {
        fn from(value: EmptyBurnAmounts) -> Self {
            Self::EmptyBurnAmounts(value)
        }
    }
    impl ::core::convert::From<EmptyDepositAmounts> for ErrorsErrors {
        fn from(value: EmptyDepositAmounts) -> Self {
            Self::EmptyDepositAmounts(value)
        }
    }
    impl ::core::convert::From<EmptyHoldingAddress> for ErrorsErrors {
        fn from(value: EmptyHoldingAddress) -> Self {
            Self::EmptyHoldingAddress(value)
        }
    }
    impl ::core::convert::From<EmptyMintAmounts> for ErrorsErrors {
        fn from(value: EmptyMintAmounts) -> Self {
            Self::EmptyMintAmounts(value)
        }
    }
    impl ::core::convert::From<EmptyOracle> for ErrorsErrors {
        fn from(value: EmptyOracle) -> Self {
            Self::EmptyOracle(value)
        }
    }
    impl ::core::convert::From<EmptyPosition> for ErrorsErrors {
        fn from(value: EmptyPosition) -> Self {
            Self::EmptyPosition(value)
        }
    }
    impl ::core::convert::From<EmptyPositions> for ErrorsErrors {
        fn from(value: EmptyPositions) -> Self {
            Self::EmptyPositions(value)
        }
    }
    impl ::core::convert::From<EmptyReceiver> for ErrorsErrors {
        fn from(value: EmptyReceiver) -> Self {
            Self::EmptyReceiver(value)
        }
    }
    impl ::core::convert::From<EmptyRedeemAmount> for ErrorsErrors {
        fn from(value: EmptyRedeemAmount) -> Self {
            Self::EmptyRedeemAmount(value)
        }
    }
    impl ::core::convert::From<EmptyRepayAmount> for ErrorsErrors {
        fn from(value: EmptyRepayAmount) -> Self {
            Self::EmptyRepayAmount(value)
        }
    }
    impl ::core::convert::From<EmptySupplyAmounts> for ErrorsErrors {
        fn from(value: EmptySupplyAmounts) -> Self {
            Self::EmptySupplyAmounts(value)
        }
    }
    impl ::core::convert::From<EmptySwapAmount> for ErrorsErrors {
        fn from(value: EmptySwapAmount) -> Self {
            Self::EmptySwapAmount(value)
        }
    }
    impl ::core::convert::From<EmptyTokenTranferGasLimit> for ErrorsErrors {
        fn from(value: EmptyTokenTranferGasLimit) -> Self {
            Self::EmptyTokenTranferGasLimit(value)
        }
    }
    impl ::core::convert::From<EmptyWithdrawAmounts> for ErrorsErrors {
        fn from(value: EmptyWithdrawAmounts) -> Self {
            Self::EmptyWithdrawAmounts(value)
        }
    }
    impl ::core::convert::From<ErrorStep> for ErrorsErrors {
        fn from(value: ErrorStep) -> Self {
            Self::ErrorStep(value)
        }
    }
    impl ::core::convert::From<HealthFactorHigherThanLiquidationThreshold>
    for ErrorsErrors {
        fn from(value: HealthFactorHigherThanLiquidationThreshold) -> Self {
            Self::HealthFactorHigherThanLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<HealthFactorLowerThanCollateralRateThreshold>
    for ErrorsErrors {
        fn from(value: HealthFactorLowerThanCollateralRateThreshold) -> Self {
            Self::HealthFactorLowerThanCollateralRateThreshold(value)
        }
    }
    impl ::core::convert::From<HealthFactorLowerThanLiquidationThreshold>
    for ErrorsErrors {
        fn from(value: HealthFactorLowerThanLiquidationThreshold) -> Self {
            Self::HealthFactorLowerThanLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<InsufficientAvailableLiquidity> for ErrorsErrors {
        fn from(value: InsufficientAvailableLiquidity) -> Self {
            Self::InsufficientAvailableLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientCollateralAmount> for ErrorsErrors {
        fn from(value: InsufficientCollateralAmount) -> Self {
            Self::InsufficientCollateralAmount(value)
        }
    }
    impl ::core::convert::From<InsufficientCollateralAmountForRepay> for ErrorsErrors {
        fn from(value: InsufficientCollateralAmountForRepay) -> Self {
            Self::InsufficientCollateralAmountForRepay(value)
        }
    }
    impl ::core::convert::From<InsufficientCollateralForSwap> for ErrorsErrors {
        fn from(value: InsufficientCollateralForSwap) -> Self {
            Self::InsufficientCollateralForSwap(value)
        }
    }
    impl ::core::convert::From<InsufficientDexLiquidity> for ErrorsErrors {
        fn from(value: InsufficientDexLiquidity) -> Self {
            Self::InsufficientDexLiquidity(value)
        }
    }
    impl ::core::convert::From<InsufficientLiquidityForBorrow> for ErrorsErrors {
        fn from(value: InsufficientLiquidityForBorrow) -> Self {
            Self::InsufficientLiquidityForBorrow(value)
        }
    }
    impl ::core::convert::From<InsufficientUserBalance> for ErrorsErrors {
        fn from(value: InsufficientUserBalance) -> Self {
            Self::InsufficientUserBalance(value)
        }
    }
    impl ::core::convert::From<InvalidBorrowCapacity> for ErrorsErrors {
        fn from(value: InvalidBorrowCapacity) -> Self {
            Self::InvalidBorrowCapacity(value)
        }
    }
    impl ::core::convert::From<InvalidDecimals> for ErrorsErrors {
        fn from(value: InvalidDecimals) -> Self {
            Self::InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidFeeFactor> for ErrorsErrors {
        fn from(value: InvalidFeeFactor) -> Self {
            Self::InvalidFeeFactor(value)
        }
    }
    impl ::core::convert::From<InvalidNativeTokenSender> for ErrorsErrors {
        fn from(value: InvalidNativeTokenSender) -> Self {
            Self::InvalidNativeTokenSender(value)
        }
    }
    impl ::core::convert::From<InvalidOptimalUsageRate> for ErrorsErrors {
        fn from(value: InvalidOptimalUsageRate) -> Self {
            Self::InvalidOptimalUsageRate(value)
        }
    }
    impl ::core::convert::From<InvalidOracleDecimals> for ErrorsErrors {
        fn from(value: InvalidOracleDecimals) -> Self {
            Self::InvalidOracleDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidOraclePrice> for ErrorsErrors {
        fn from(value: InvalidOraclePrice) -> Self {
            Self::InvalidOraclePrice(value)
        }
    }
    impl ::core::convert::From<InvalidPoolIndex> for ErrorsErrors {
        fn from(value: InvalidPoolIndex) -> Self {
            Self::InvalidPoolIndex(value)
        }
    }
    impl ::core::convert::From<InvalidSupplyCapacity> for ErrorsErrors {
        fn from(value: InvalidSupplyCapacity) -> Self {
            Self::InvalidSupplyCapacity(value)
        }
    }
    impl ::core::convert::From<OracleEmpty> for ErrorsErrors {
        fn from(value: OracleEmpty) -> Self {
            Self::OracleEmpty(value)
        }
    }
    impl ::core::convert::From<PoolAlreadyExists> for ErrorsErrors {
        fn from(value: PoolAlreadyExists) -> Self {
            Self::PoolAlreadyExists(value)
        }
    }
    impl ::core::convert::From<PoolIsFrozen> for ErrorsErrors {
        fn from(value: PoolIsFrozen) -> Self {
            Self::PoolIsFrozen(value)
        }
    }
    impl ::core::convert::From<PoolIsInactive> for ErrorsErrors {
        fn from(value: PoolIsInactive) -> Self {
            Self::PoolIsInactive(value)
        }
    }
    impl ::core::convert::From<PoolIsNotBorrowing> for ErrorsErrors {
        fn from(value: PoolIsNotBorrowing) -> Self {
            Self::PoolIsNotBorrowing(value)
        }
    }
    impl ::core::convert::From<PoolIsPaused> for ErrorsErrors {
        fn from(value: PoolIsPaused) -> Self {
            Self::PoolIsPaused(value)
        }
    }
    impl ::core::convert::From<PoolNotFound> for ErrorsErrors {
        fn from(value: PoolNotFound) -> Self {
            Self::PoolNotFound(value)
        }
    }
    impl ::core::convert::From<PositionNotFound> for ErrorsErrors {
        fn from(value: PositionNotFound) -> Self {
            Self::PositionNotFound(value)
        }
    }
    impl ::core::convert::From<SelfTransferNotSupported> for ErrorsErrors {
        fn from(value: SelfTransferNotSupported) -> Self {
            Self::SelfTransferNotSupported(value)
        }
    }
    impl ::core::convert::From<SupplyCapacityExceeded> for ErrorsErrors {
        fn from(value: SupplyCapacityExceeded) -> Self {
            Self::SupplyCapacityExceeded(value)
        }
    }
    impl ::core::convert::From<SwapPoolsNotMatch> for ErrorsErrors {
        fn from(value: SwapPoolsNotMatch) -> Self {
            Self::SwapPoolsNotMatch(value)
        }
    }
    impl ::core::convert::From<ThereMustBeAtLeastOneRoleAdmin> for ErrorsErrors {
        fn from(value: ThereMustBeAtLeastOneRoleAdmin) -> Self {
            Self::ThereMustBeAtLeastOneRoleAdmin(value)
        }
    }
    impl ::core::convert::From<ThereMustBeAtLeastOneTimelockMultiSig> for ErrorsErrors {
        fn from(value: ThereMustBeAtLeastOneTimelockMultiSig) -> Self {
            Self::ThereMustBeAtLeastOneTimelockMultiSig(value)
        }
    }
    impl ::core::convert::From<TokenCanNotSwapWithSelf> for ErrorsErrors {
        fn from(value: TokenCanNotSwapWithSelf) -> Self {
            Self::TokenCanNotSwapWithSelf(value)
        }
    }
    impl ::core::convert::From<TokenNotMatch> for ErrorsErrors {
        fn from(value: TokenNotMatch) -> Self {
            Self::TokenNotMatch(value)
        }
    }
    impl ::core::convert::From<TokenTransferError> for ErrorsErrors {
        fn from(value: TokenTransferError) -> Self {
            Self::TokenTransferError(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for ErrorsErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnderlyAssetEmpty> for ErrorsErrors {
        fn from(value: UnderlyAssetEmpty) -> Self {
            Self::UnderlyAssetEmpty(value)
        }
    }
    impl ::core::convert::From<UsdCollateralCanNotCoverDebt> for ErrorsErrors {
        fn from(value: UsdCollateralCanNotCoverDebt) -> Self {
            Self::UsdCollateralCanNotCoverDebt(value)
        }
    }
    impl ::core::convert::From<UsdNotHaveLongOperation> for ErrorsErrors {
        fn from(value: UsdNotHaveLongOperation) -> Self {
            Self::UsdNotHaveLongOperation(value)
        }
    }
    impl ::core::convert::From<UsdNotHaveShortOperation> for ErrorsErrors {
        fn from(value: UsdNotHaveShortOperation) -> Self {
            Self::UsdNotHaveShortOperation(value)
        }
    }
    impl ::core::convert::From<UserDoNotHaveDebtInPool> for ErrorsErrors {
        fn from(value: UserDoNotHaveDebtInPool) -> Self {
            Self::UserDoNotHaveDebtInPool(value)
        }
    }
}
