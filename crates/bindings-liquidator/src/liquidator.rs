pub use liquidator::*;
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
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_dataStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_exchangeRouter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reader"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_liquidationHandler"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LiquidationParams"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("toString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x19\xB38\x03\x80a\x19\xB3\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01-V[3\x80a\0UW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0^\x81a\0\xC1V[P`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x17\x90\x91U`\x04\x80T\x82\x16\x95\x87\x16\x95\x90\x95\x17\x90\x94U`\x02\x80T\x85\x16\x93\x86\x16\x93\x90\x93\x17\x90\x92U`\x03\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\x05\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90Ua\x01\x92V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01(W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x01EW`\0\x80\xFD[a\x01N\x86a\x01\x11V[\x94Pa\x01\\` \x87\x01a\x01\x11V[\x93Pa\x01j`@\x87\x01a\x01\x11V[\x92Pa\x01x``\x87\x01a\x01\x11V[\x91Pa\x01\x86`\x80\x87\x01a\x01\x11V[\x90P\x92\x95P\x92\x95\x90\x93PV[a\x18\x12\x80a\x01\xA1`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80ci\0\xA3\xAE\x14a\0gW\x80cqP\x18\xA6\x14a\0\x90W\x80c\x8D\xA5\xCB[\x14a\0\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\0\xB5W\x80c\xFAF\x1E3\x14a\0\xC8W\x80c\xFB(\xB4<\x14a\0\xDBW[`\0\x80\xFD[a\0za\0u6`\x04a\x12uV[a\0\xEEV[`@Qa\0\x87\x91\x90a\x12\xD4V[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\x01 V[\0[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x87V[a\0\x98a\0\xC36`\x04a\x13\x03V[a\x014V[a\0\x98a\0\xD66`\x04a\x13 V[a\x01wV[a\0\x98a\0\xE96`\x04a\x13\xA3V[a\x03UV[``a\x01\x1A\x82`@Q` \x01a\x01\x06\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\r\x12V[\x92\x91PPV[a\x01(a\x0F!V[a\x012`\0a\x0FNV[V[a\x01<a\x0F!V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01kW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x01t\x81a\x0FNV[PV[`\0\x80\x80a\x01\x87\x84\x86\x01\x86a\x13\xF6V[`\x01T\x92\x95P\x90\x93P\x91Pa\x01\xAF\x90`\x01`\x01`\xA0\x1B\x03\x16a\x01\xAA\x85\x85\x85a\x0F\x9EV[a\x10\tV[`\0\x87\x13\x15a\x02\x94W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x1A\x91\x90a\x14=V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8E\x91\x90a\x14jV[Pa\x03LV[`\0\x86\x13\x15a\x034W3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xFF\x91\x90a\x14=V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x02KV[\x86\x15\x80\x15a\x03@WP\x85\x15[a\x03LWa\x03La\x14\x85V[PPPPPPPV[a\x03]a\x0F!V[a\x03|`@Q\x80``\x01`@R\x80`2\x81R` \x01a\x17\xAB`2\x919PV[a\x03\x84a\x11rV[a\x03\x94`@\x83\x01` \x84\x01a\x13\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFE\x91\x90a\x14\x9BV[\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81RkunclaimedFee`\xA0\x1B` \x90\x91\x01R`\x03T`\x04T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91cm|H\x92\x91\x16a\x04J` \x86\x01\x86a\x13\x03V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB9\x91\x90a\x14\xCAV[``\x82\x01\x81\x90R`@\x01Q\x15a\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FLiquidator: The health factor is`D\x82\x01R\x7F higher than the liquidation thr`d\x82\x01Re\x19\\\xDA\x1B\xDB\x19`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x01bV[`\0`\x80\x82\x01R[a\x05[``\x83\x01\x83a\x15RV[\x90P\x81`\x80\x01Q\x10\x15a\tgWa\x05u``\x83\x01\x83a\x15RV[\x82`\x80\x01Q\x81\x81\x10a\x05\x89Wa\x05\x89a\x15\xA3V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\x9F\x91\x90a\x15\xB9V[`\xA0\x82\x01R`\x03T`\x04T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\x1D\xE3\x97\x03\x91\x16a\x05\xCA` \x86\x01\x86a\x13\x03V[`\xA0\x85\x01QQ`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x91\x90\x91\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06I\x91\x90a\x14\x9BV[a\x01 \x82\x01\x81\x90R`\xA0\x82\x01QQ`\x05T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCD\x91\x90a\x14jV[P`\xA0\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x16a\x06\xED`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tOWa\x07*a\x07\x0E`@\x84\x01` \x85\x01a\x13\x03V[`\xA0\x80\x84\x01QQ\x90a\x07%\x90\x86\x01`\x80\x87\x01a\x16#V[a\x10cV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xE0\x83\x01R`\xA0\x82\x01QQ\x16a\x07Q`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x90\x91\x10a\x01\0\x83\x01\x81\x90R`\xE0\x83\x01Qa\x01 \x84\x01Q\x92\x16\x91c\x12\x8A\xCB\x08\x910\x91a\x07\x87\x90a\x16TV[\x85a\x01\0\x01Qa\x07\xB5Wa\x07\xB0`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x16pV[a\x07\xC5V[a\x07\xC5d\x01\0\x02v\xA3`\x01a\x16\x8FV[a\x07\xD5`@\x89\x01` \x8A\x01a\x13\x03V[`\xA0\x80\x89\x01QQ\x90a\x07\xEC\x90\x8B\x01`\x80\x8C\x01a\x16#V[`@Q` \x01a\x07\xFE\x93\x92\x91\x90a\x16\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08-\x95\x94\x93\x92\x91\x90a\x16\xD6V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08o\x91\x90a\x17\x1CV[PP`\xA0\x81\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE0\x91\x90a\x14\x9BV[`@\x82\x01\x81\x90Ra\x01 \x82\x01Q\x14a\tOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FLiquidator: Do not swap to get e`D\x82\x01Rk\x03s{\xAB;A\x03\x0B\x9B\x9B+\xA1`\xA5\x1B`d\x82\x01R`\x84\x01a\x01bV[`\x80\x81\x01\x80Q\x90a\t_\x82a\x17@V[\x90RPa\x05NV[`@Q\x80` \x01`@R\x80\x83`\0\x01` \x81\x01\x90a\t\x85\x91\x90a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91Ra\x01@\x83\x01\x82\x90R`\x02T`@Qc\xB8\\Q\xD5`\xE0\x1B\x81R\x92Q\x82\x16`\x04\x84\x01R\x16\x90c\xB8\\Q\xD5\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xEDW=`\0\x80>=`\0\xFD[PP`\0`\x80\x84\x01RPP[a\n\x06`@\x83\x01\x83a\x15RV[\x90P\x81`\x80\x01Q\x10\x15a\x0B\xF4Wa\n `@\x83\x01\x83a\x15RV[\x82`\x80\x01Q\x81\x81\x10a\n4Wa\n4a\x15\xA3V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\nJ\x91\x90a\x15\xB9V[`\xC0\x82\x01\x81\x90RQ`\x01`\x01`\xA0\x1B\x03\x16a\nk`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xDCWa\n\xA2a\n\x8C`@\x84\x01` \x85\x01a\x13\x03V[`\xC0\x83\x01QQa\x07%`\xA0\x86\x01`\x80\x87\x01a\x16#V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\n\xC0`@\x83\x01` \x84\x01a\x13\x03V[`\xC0\x82\x01Q\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90\x83\x16\x10a\x01\0\x84\x01\x81\x90R`\xE0\x84\x01Q` \x90\x92\x01Q\x91\x90\x92\x16\x91c\x12\x8A\xCB\x08\x910\x91\x90\x81a\x0B Wa\x0B\x1B`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x16pV[a\x0B0V[a\x0B0d\x01\0\x02v\xA3`\x01a\x16\x8FV[a\x0B@`@\x89\x01` \x8A\x01a\x13\x03V[`\xC0\x88\x01QQa\x0BV`\xA0\x8B\x01`\x80\x8C\x01a\x16#V[`@Q` \x01a\x0Bh\x93\x92\x91\x90a\x16\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x97\x95\x94\x93\x92\x91\x90a\x16\xD6V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD9\x91\x90a\x17\x1CV[PP[`\x80\x81\x01\x80Q\x90a\x0B\xEC\x82a\x17@V[\x90RPa\t\xF9V[a\x0C\x04`@\x83\x01` \x84\x01a\x13\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cn\x91\x90a\x14\x9BV[` \x82\x01\x81\x90R\x81Q`\xA0\x84\x015\x91a\x0C\x87\x91\x90a\x17YV[a\x0C\x91\x91\x90a\x17YV[a\x01`\x82\x01\x81\x90R`\0\x12a\r\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FLiquidator: there is no profit o`D\x82\x01R\x7Ff this liquidation action\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01bV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90`\0\x90a\rM\x90`\x02a\x17\x80V[a\rX\x90`\x02a\x17\x97V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rpWa\rpa\x14\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\x9AW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\xB5Wa\r\xB5a\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r\xE4Wa\r\xE4a\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0[\x84Q\x81\x10\x15a\x0F\x19W\x82`\x04\x86\x83\x81Q\x81\x10a\x0E\x1CWa\x0E\x1Ca\x15\xA3V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x0EDWa\x0EDa\x15\xA3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x0E_\x83`\x02a\x17\x80V[a\x0Ej\x90`\x02a\x17\x97V[\x81Q\x81\x10a\x0EzWa\x0Eza\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x0E\xA4Wa\x0E\xA4a\x15\xA3V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x0E\xC4Wa\x0E\xC4a\x15\xA3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x0E\xDF\x83`\x02a\x17\x80V[a\x0E\xEA\x90`\x03a\x17\x97V[\x81Q\x81\x10a\x0E\xFAWa\x0E\xFAa\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\r\xFEV[P\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x012W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01bV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x0F\xD9W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x10\x15\x83\x83a\x10\x8EV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x10^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01bV[PPPV[`\x01T`\0\x90a\x10\x86\x90`\x01`\x01`\xA0\x1B\x03\x16a\x10\x81\x86\x86\x86a\x0F\x9EV[a\x10\x8EV[\x94\x93PPPPV[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x10\xB6W`\0\x80\xFD[\x82\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x10\xD8\x93\x92\x91\x90a\x16\xAEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 a\x11S\x93\x92\x90\x91\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT\x91\x01`\x01`\x01`\xF8\x1B\x03\x19\x81R``\x93\x90\x93\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01\x91\x90\x91R`5\x82\x01R`U\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`@Q\x80a\x01\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01a\x11\xC7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0\x81R` \x01a\x11\xF9`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x12$`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x81R` \x01a\x12h`@Q\x80` \x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[\x81R` \x01`\0\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a\x12\x87W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x12\xB4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x12\x98V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12\xE7` \x83\x01\x84a\x12\x8EV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x15W`\0\x80\xFD[\x815a\x12\xE7\x81a\x12\xEEV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x136W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13[W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x13lW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x83W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x13\x95W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xB5W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xCCW`\0\x80\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x12\xE7W`\0\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xF1W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\x0BW`\0\x80\xFD[\x835a\x14\x16\x81a\x12\xEEV[\x92P` \x84\x015a\x14&\x81a\x12\xEEV[\x91Pa\x144`@\x85\x01a\x13\xDEV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14OW`\0\x80\xFD[\x81Qa\x12\xE7\x81a\x12\xEEV[\x80Q\x80\x15\x15\x81\x14a\x13\xF1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14|W`\0\x80\xFD[a\x12\xE7\x82a\x14ZV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xADW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\xA0\x82\x84\x03\x12\x80\x15a\x14\xDDW`\0\x80\xFD[P`@Q`\0\x90`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\x10WcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@\x90\x81R\x84Q\x82R` \x80\x86\x01Q\x90\x83\x01Ra\x15.\x90\x85\x01a\x14ZV[`@\x82\x01R``\x84\x81\x01Q\x90\x82\x01R`\x80\x93\x84\x01Q\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15iW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x84W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x15\x9CW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x80\x15a\x15\xCCW`\0\x80\xFD[P`@\x80Q`\0\x91\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xFEWcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@R\x835a\x16\x0C\x81a\x12\xEEV[\x81R` \x93\x84\x015\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x165W`\0\x80\xFD[a\x12\xE7\x82a\x13\xDEV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x16iWa\x16ia\x16>V[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01Rb\xFF\xFF\xFF\x90\x91\x16`@\x82\x01R``\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x17\x11\x90\x83\x01\x84a\x12\x8EV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17/W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0`\x01\x82\x01a\x17RWa\x17Ra\x16>V[P`\x01\x01\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x17yWa\x17ya\x16>V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\x1AWa\x01\x1Aa\x16>V[\x80\x82\x01\x80\x82\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V\xFE--------------------liquidate---------------------\xA2dipfsX\"\x12 \xF7.[t\x12>e\xF3\x11\xFF\x14\xAC\x01\x12\x10\xD5\x0F\xBC5\x11\xE2V\x8EAm?\xA6n\x8D\x1D>\\dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80ci\0\xA3\xAE\x14a\0gW\x80cqP\x18\xA6\x14a\0\x90W\x80c\x8D\xA5\xCB[\x14a\0\x9AW\x80c\xF2\xFD\xE3\x8B\x14a\0\xB5W\x80c\xFAF\x1E3\x14a\0\xC8W\x80c\xFB(\xB4<\x14a\0\xDBW[`\0\x80\xFD[a\0za\0u6`\x04a\x12uV[a\0\xEEV[`@Qa\0\x87\x91\x90a\x12\xD4V[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\x01 V[\0[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x87V[a\0\x98a\0\xC36`\x04a\x13\x03V[a\x014V[a\0\x98a\0\xD66`\x04a\x13 V[a\x01wV[a\0\x98a\0\xE96`\x04a\x13\xA3V[a\x03UV[``a\x01\x1A\x82`@Q` \x01a\x01\x06\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\r\x12V[\x92\x91PPV[a\x01(a\x0F!V[a\x012`\0a\x0FNV[V[a\x01<a\x0F!V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01kW`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x01t\x81a\x0FNV[PV[`\0\x80\x80a\x01\x87\x84\x86\x01\x86a\x13\xF6V[`\x01T\x92\x95P\x90\x93P\x91Pa\x01\xAF\x90`\x01`\x01`\xA0\x1B\x03\x16a\x01\xAA\x85\x85\x85a\x0F\x9EV[a\x10\tV[`\0\x87\x13\x15a\x02\x94W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x1A\x91\x90a\x14=V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8E\x91\x90a\x14jV[Pa\x03LV[`\0\x86\x13\x15a\x034W3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xFF\x91\x90a\x14=V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x02KV[\x86\x15\x80\x15a\x03@WP\x85\x15[a\x03LWa\x03La\x14\x85V[PPPPPPPV[a\x03]a\x0F!V[a\x03|`@Q\x80``\x01`@R\x80`2\x81R` \x01a\x17\xAB`2\x919PV[a\x03\x84a\x11rV[a\x03\x94`@\x83\x01` \x84\x01a\x13\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFE\x91\x90a\x14\x9BV[\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81RkunclaimedFee`\xA0\x1B` \x90\x91\x01R`\x03T`\x04T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91cm|H\x92\x91\x16a\x04J` \x86\x01\x86a\x13\x03V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB9\x91\x90a\x14\xCAV[``\x82\x01\x81\x90R`@\x01Q\x15a\x05FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FLiquidator: The health factor is`D\x82\x01R\x7F higher than the liquidation thr`d\x82\x01Re\x19\\\xDA\x1B\xDB\x19`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x01bV[`\0`\x80\x82\x01R[a\x05[``\x83\x01\x83a\x15RV[\x90P\x81`\x80\x01Q\x10\x15a\tgWa\x05u``\x83\x01\x83a\x15RV[\x82`\x80\x01Q\x81\x81\x10a\x05\x89Wa\x05\x89a\x15\xA3V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\x9F\x91\x90a\x15\xB9V[`\xA0\x82\x01R`\x03T`\x04T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\x1D\xE3\x97\x03\x91\x16a\x05\xCA` \x86\x01\x86a\x13\x03V[`\xA0\x85\x01QQ`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x91\x90\x91\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06I\x91\x90a\x14\x9BV[a\x01 \x82\x01\x81\x90R`\xA0\x82\x01QQ`\x05T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCD\x91\x90a\x14jV[P`\xA0\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x16a\x06\xED`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\tOWa\x07*a\x07\x0E`@\x84\x01` \x85\x01a\x13\x03V[`\xA0\x80\x84\x01QQ\x90a\x07%\x90\x86\x01`\x80\x87\x01a\x16#V[a\x10cV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xE0\x83\x01R`\xA0\x82\x01QQ\x16a\x07Q`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x90\x91\x10a\x01\0\x83\x01\x81\x90R`\xE0\x83\x01Qa\x01 \x84\x01Q\x92\x16\x91c\x12\x8A\xCB\x08\x910\x91a\x07\x87\x90a\x16TV[\x85a\x01\0\x01Qa\x07\xB5Wa\x07\xB0`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x16pV[a\x07\xC5V[a\x07\xC5d\x01\0\x02v\xA3`\x01a\x16\x8FV[a\x07\xD5`@\x89\x01` \x8A\x01a\x13\x03V[`\xA0\x80\x89\x01QQ\x90a\x07\xEC\x90\x8B\x01`\x80\x8C\x01a\x16#V[`@Q` \x01a\x07\xFE\x93\x92\x91\x90a\x16\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08-\x95\x94\x93\x92\x91\x90a\x16\xD6V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08o\x91\x90a\x17\x1CV[PP`\xA0\x81\x01QQ`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE0\x91\x90a\x14\x9BV[`@\x82\x01\x81\x90Ra\x01 \x82\x01Q\x14a\tOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FLiquidator: Do not swap to get e`D\x82\x01Rk\x03s{\xAB;A\x03\x0B\x9B\x9B+\xA1`\xA5\x1B`d\x82\x01R`\x84\x01a\x01bV[`\x80\x81\x01\x80Q\x90a\t_\x82a\x17@V[\x90RPa\x05NV[`@Q\x80` \x01`@R\x80\x83`\0\x01` \x81\x01\x90a\t\x85\x91\x90a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x91Ra\x01@\x83\x01\x82\x90R`\x02T`@Qc\xB8\\Q\xD5`\xE0\x1B\x81R\x92Q\x82\x16`\x04\x84\x01R\x16\x90c\xB8\\Q\xD5\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xD9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xEDW=`\0\x80>=`\0\xFD[PP`\0`\x80\x84\x01RPP[a\n\x06`@\x83\x01\x83a\x15RV[\x90P\x81`\x80\x01Q\x10\x15a\x0B\xF4Wa\n `@\x83\x01\x83a\x15RV[\x82`\x80\x01Q\x81\x81\x10a\n4Wa\n4a\x15\xA3V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\nJ\x91\x90a\x15\xB9V[`\xC0\x82\x01\x81\x90RQ`\x01`\x01`\xA0\x1B\x03\x16a\nk`@\x84\x01` \x85\x01a\x13\x03V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xDCWa\n\xA2a\n\x8C`@\x84\x01` \x85\x01a\x13\x03V[`\xC0\x83\x01QQa\x07%`\xA0\x86\x01`\x80\x87\x01a\x16#V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\n\xC0`@\x83\x01` \x84\x01a\x13\x03V[`\xC0\x82\x01Q\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90\x83\x16\x10a\x01\0\x84\x01\x81\x90R`\xE0\x84\x01Q` \x90\x92\x01Q\x91\x90\x92\x16\x91c\x12\x8A\xCB\x08\x910\x91\x90\x81a\x0B Wa\x0B\x1B`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x16pV[a\x0B0V[a\x0B0d\x01\0\x02v\xA3`\x01a\x16\x8FV[a\x0B@`@\x89\x01` \x8A\x01a\x13\x03V[`\xC0\x88\x01QQa\x0BV`\xA0\x8B\x01`\x80\x8C\x01a\x16#V[`@Q` \x01a\x0Bh\x93\x92\x91\x90a\x16\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x97\x95\x94\x93\x92\x91\x90a\x16\xD6V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD9\x91\x90a\x17\x1CV[PP[`\x80\x81\x01\x80Q\x90a\x0B\xEC\x82a\x17@V[\x90RPa\t\xF9V[a\x0C\x04`@\x83\x01` \x84\x01a\x13\x03V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cn\x91\x90a\x14\x9BV[` \x82\x01\x81\x90R\x81Q`\xA0\x84\x015\x91a\x0C\x87\x91\x90a\x17YV[a\x0C\x91\x91\x90a\x17YV[a\x01`\x82\x01\x81\x90R`\0\x12a\r\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FLiquidator: there is no profit o`D\x82\x01R\x7Ff this liquidation action\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01bV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90`\0\x90a\rM\x90`\x02a\x17\x80V[a\rX\x90`\x02a\x17\x97V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rpWa\rpa\x14\xB4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\x9AW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\r\xB5Wa\r\xB5a\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\r\xE4Wa\r\xE4a\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0[\x84Q\x81\x10\x15a\x0F\x19W\x82`\x04\x86\x83\x81Q\x81\x10a\x0E\x1CWa\x0E\x1Ca\x15\xA3V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x0EDWa\x0EDa\x15\xA3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x0E_\x83`\x02a\x17\x80V[a\x0Ej\x90`\x02a\x17\x97V[\x81Q\x81\x10a\x0EzWa\x0Eza\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x0E\xA4Wa\x0E\xA4a\x15\xA3V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x0E\xC4Wa\x0E\xC4a\x15\xA3V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x0E\xDF\x83`\x02a\x17\x80V[a\x0E\xEA\x90`\x03a\x17\x97V[\x81Q\x81\x10a\x0E\xFAWa\x0E\xFAa\x15\xA3V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\r\xFEV[P\x93\x92PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x012W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01bV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x0F\xD9W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0a\x10\x15\x83\x83a\x10\x8EV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x10^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01bV[PPPV[`\x01T`\0\x90a\x10\x86\x90`\x01`\x01`\xA0\x1B\x03\x16a\x10\x81\x86\x86\x86a\x0F\x9EV[a\x10\x8EV[\x94\x93PPPPV[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x10\xB6W`\0\x80\xFD[\x82\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q` \x01a\x10\xD8\x93\x92\x91\x90a\x16\xAEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 a\x11S\x93\x92\x90\x91\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT\x91\x01`\x01`\x01`\xF8\x1B\x03\x19\x81R``\x93\x90\x93\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x84\x01R`\x15\x83\x01\x91\x90\x91R`5\x82\x01R`U\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`@Q\x80a\x01\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01a\x11\xC7`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0\x81R` \x01a\x11\xF9`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[\x81R` \x01a\x12$`@Q\x80`@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x81R` \x01a\x12h`@Q\x80` \x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[\x81R` \x01`\0\x81RP\x90V[`\0` \x82\x84\x03\x12\x15a\x12\x87W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x12\xB4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x12\x98V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12\xE7` \x83\x01\x84a\x12\x8EV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01tW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x15W`\0\x80\xFD[\x815a\x12\xE7\x81a\x12\xEEV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x136W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13[W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x13lW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x83W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x13\x95W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0` \x82\x84\x03\x12\x15a\x13\xB5W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xCCW`\0\x80\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x12\xE7W`\0\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x13\xF1W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\x0BW`\0\x80\xFD[\x835a\x14\x16\x81a\x12\xEEV[\x92P` \x84\x015a\x14&\x81a\x12\xEEV[\x91Pa\x144`@\x85\x01a\x13\xDEV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x14OW`\0\x80\xFD[\x81Qa\x12\xE7\x81a\x12\xEEV[\x80Q\x80\x15\x15\x81\x14a\x13\xF1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x14|W`\0\x80\xFD[a\x12\xE7\x82a\x14ZV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x14\xADW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\xA0\x82\x84\x03\x12\x80\x15a\x14\xDDW`\0\x80\xFD[P`@Q`\0\x90`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\x10WcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@\x90\x81R\x84Q\x82R` \x80\x86\x01Q\x90\x83\x01Ra\x15.\x90\x85\x01a\x14ZV[`@\x82\x01R``\x84\x81\x01Q\x90\x82\x01R`\x80\x93\x84\x01Q\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15iW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\x84W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x15\x9CW`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x80\x15a\x15\xCCW`\0\x80\xFD[P`@\x80Q`\0\x91\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xFEWcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@R\x835a\x16\x0C\x81a\x12\xEEV[\x81R` \x93\x84\x015\x93\x81\x01\x93\x90\x93RP\x90\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x165W`\0\x80\xFD[a\x12\xE7\x82a\x13\xDEV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x16iWa\x16ia\x16>V[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01Rb\xFF\xFF\xFF\x90\x91\x16`@\x82\x01R``\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x17\x11\x90\x83\x01\x84a\x12\x8EV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17/W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0`\x01\x82\x01a\x17RWa\x17Ra\x16>V[P`\x01\x01\x90V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x17yWa\x17ya\x16>V[P\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\x1AWa\x01\x1Aa\x16>V[\x80\x82\x01\x80\x82\x11\x15a\x01\x1AWa\x01\x1Aa\x16>V\xFE--------------------liquidate---------------------\xA2dipfsX\"\x12 \xF7.[t\x12>e\xF3\x11\xFF\x14\xAC\x01\x12\x10\xD5\x0F\xBC5\x11\xE2V\x8EAm?\xA6n\x8D\x1D>\\dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
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
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `liquidate` (0xfb28b43c) function
        pub fn liquidate(
            &self,
            params: LiquidationParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 40, 180, 60], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x6900a3ae) function
        pub fn to_string(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([105, 0, 163, 174], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
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
    pub enum LiquidatorErrors {
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner> for LiquidatorErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount> for LiquidatorErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate((address,address,(address,uint256)[],(address,uint256)[],uint24,uint256))` and selector `0xfb28b43c`
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
        name = "liquidate",
        abi = "liquidate((address,address,(address,uint256)[],(address,uint256)[],uint24,uint256))"
    )]
    pub struct LiquidateCall {
        pub params: LiquidationParams,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    #[ethcall(name = "toString", abi = "toString(uint256)")]
    pub struct ToStringCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum LiquidatorCalls {
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        ToString(ToStringCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ToStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for LiquidatorCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ToStringCall> for LiquidatorCalls {
        fn from(value: ToStringCall) -> Self {
            Self::ToString(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    pub struct ToStringReturn(pub ::std::string::String);
    ///`Asset(address,uint256)`
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
    pub struct Asset {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///`LiquidationParams(address,address,(address,uint256)[],(address,uint256)[],uint24,uint256)`
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
        pub usd_token: ::ethers::core::types::Address,
        pub collaterals: ::std::vec::Vec<Asset>,
        pub debts: ::std::vec::Vec<Asset>,
        pub uniswap_fee: u32,
        pub gas_fee_usd: ::ethers::core::types::U256,
    }
}
