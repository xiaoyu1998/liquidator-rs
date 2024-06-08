pub use quoter::*;
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
pub mod quoter {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
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
                    ::std::borrow::ToOwned::to_owned("quoteExactInputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteExactInputSingle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("startSqrtPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteExactOutputSingle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quoteExactOutputSingle",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("startSqrtPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
    pub static QUOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x0F38\x03\x80a\x0F3\x839\x81\x01`@\x81\x90R`,\x91`<V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`jV[`\0` \x82\x84\x03\x12\x15`MW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0E\xA1a\0\x92`\09`\0\x81\x81`\x83\x01R\x81\x81a\x055\x01Ra\x05\xED\x01Ra\x0E\xA1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c0\xD0\x7F!\x14a\0QW\x80c\xC4Z\x01U\x14a\0~W\x80c\xF7r\x9DC\x14a\0\xBDW\x80c\xFAF\x1E3\x14a\0\xD0W[`\0\x80\xFD[a\0da\0_6`\x04a\n%V[a\0\xE5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0uV[a\0da\0\xCB6`\x04a\n%V[a\x03#V[a\0\xE3a\0\xDE6`\x04a\x0B\x04V[a\x05\x04V[\0[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x88\x16\x10\x81R\x81\x90a\x01\x1C\x88\x88\x88a\x05\xE6V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90R`@\x80Qc8P\xC7\xBD`\xE0\x1B\x81R\x90Qc8P\xC7\xBD\x91`\x04\x80\x82\x01\x92`\xE0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x8B\x91\x90a\x0B\xAEV[PPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`@\x86\x01RPPP\x84\x16`\0\x03a\x01\xB0W`\0\x85\x90U[\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83`\0\x01Qa\x01\xD2\x89a\x06$V[a\x01\xDB\x90a\x0ChV[`\x01`\x01`\xA0\x1B\x03\x89\x16\x15a\x01\xF0W\x88a\x02*V[\x85Qa\x02\x1AWa\x02\x15`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0C\x84V[a\x02*V[a\x02*d\x01\0\x02v\xA3`\x01a\x0C\xA3V[\x8C\x8C\x8F`@Q` \x01a\x02?\x93\x92\x91\x90a\x0C\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02n\x95\x94\x93\x92\x91\x90a\rMV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x02\xA8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x02\xA5\x91\x81\x01\x90a\r\x93V[`\x01[a\x03\x15W=\x80\x80\x15a\x02\xD6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xDBV[``\x91P[P\x84`\x01`\x01`\xA0\x1B\x03\x16`\0\x03a\x02\xF2W`\0\x80U[a\x02\xFB\x81a\x06:V[`@\x90\x92\x01Q\x91\x93PP`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\x03\x19V[PPP[\x95P\x95\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x88\x16\x10\x81R\x81\x90a\x03Z\x88\x88\x88a\x05\xE6V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90R`@\x80Qc8P\xC7\xBD`\xE0\x1B\x81R\x90Qc8P\xC7\xBD\x91`\x04\x80\x82\x01\x92`\xE0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC9\x91\x90a\x0B\xAEV[PPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`@\x86\x01RPPP` \x82\x01Q\x82Q\x91\x16\x90c\x12\x8A\xCB\x08\x900\x90a\x03\xFA\x89a\x06$V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x15a\x04\x0FW\x88a\x04IV[\x85Qa\x049Wa\x044`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0C\x84V[a\x04IV[a\x04Id\x01\0\x02v\xA3`\x01a\x0C\xA3V[\x8D\x8C\x8E`@Q` \x01a\x04^\x93\x92\x91\x90a\x0C\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8D\x95\x94\x93\x92\x91\x90a\rMV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x04\xC7WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04\xC4\x91\x81\x01\x90a\r\x93V[`\x01[a\x03\x15W=\x80\x80\x15a\x04\xF5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xFAV[``\x91P[Pa\x02\xFB\x81a\x06:V[`\0\x83\x13\x80a\x05\x13WP`\0\x82\x13[a\x05\x1CW`\0\x80\xFD[`\0\x80`\0a\x05*\x84a\x06\xDCV[\x92P\x92P\x92Pa\x05\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x07\x18V[P`\0\x80`\0\x80\x89\x13a\x05\x8EW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8Aa\x05\x89\x90a\x0ChV[a\x05\xAEV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89a\x05\xAE\x90a\x0ChV[\x92P\x92P\x92P\x82\x15a\x05\xC5W`@Q\x81\x81R` \x81\xFD[`\0T\x15a\x05\xDBW`\0T\x81\x14a\x05\xDBW`\0\x80\xFD[`@Q\x82\x81R` \x81\xFD[`\0a\x06\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x17\x86\x86\x86a\x077V[a\x07\xA2V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x066W`\0\x80\xFD[P\x90V[`\0\x81Q` \x14a\x06\xC2W`D\x82Q\x10\x15a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x06\xA9\x91\x90a\r\xB7V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x86\x91\x90a\x0E%V[\x81\x80` \x01\x90Q\x81\x01\x90a\x06\xD6\x91\x90a\x0E?V[\x92\x91PPV[`\0\x80\x80a\x06\xEA\x84\x82a\x08\x8BV[\x92Pa\x06\xF7\x84`\x14a\t?V[\x90Pa\x07\x0Fa\x07\x08`\x03`\x14a\x0EXV[\x85\x90a\x08\x8BV[\x91P\x91\x93\x90\x92PV[`\0a\x07.\x85a\x07)\x86\x86\x86a\x077V[a\t\xEAV[\x95\x94PPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07rW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x07\xCAW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0\x81a\x08\x99\x81`\x14a\x0EXV[\x10\x15a\x08\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R`d\x01a\x06\x86V[a\x08\xE7\x82`\x14a\x0EXV[\x83Q\x10\x15a\t/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x06\x86V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81a\tM\x81`\x03a\x0EXV[\x10\x15a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x06\x86V[a\t\x9A\x82`\x03a\x0EXV[\x83Q\x10\x15a\t\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x06\x86V[P\x01`\x03\x01Q\x90V[`\0a\t\xF6\x83\x83a\x07\xA2V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x06\xD6W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\"W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\n=W`\0\x80\xFD[\x855a\nH\x81a\n\rV[\x94P` \x86\x015a\nX\x81a\n\rV[\x93P`@\x86\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\npW`\0\x80\xFD[\x92P``\x86\x015\x91P`\x80\x86\x015a\n\x87\x81a\n\rV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xD4Wa\n\xD4a\n\x95V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xF6Wa\n\xF6a\n\x95V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\x19W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B>W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0BOW`\0\x80\xFD[\x805a\x0Bba\x0B]\x82a\n\xDCV[a\n\xABV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x0BwW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x0B\xA9W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xC9W`\0\x80\xFD[\x87Qa\x0B\xD4\x81a\n\rV[\x80\x97PP` \x88\x01Q\x80`\x02\x0B\x81\x14a\x0B\xECW`\0\x80\xFD[\x95Pa\x0B\xFA`@\x89\x01a\x0B\x97V[\x94Pa\x0C\x08``\x89\x01a\x0B\x97V[\x93Pa\x0C\x16`\x80\x89\x01a\x0B\x97V[\x92P`\xA0\x88\x01Q`\xFF\x81\x16\x81\x14a\x0C,W`\0\x80\xFD[`\xC0\x89\x01Q\x90\x92P\x80\x15\x15\x81\x14a\x0CBW`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C}Wa\x0C}a\x0CRV[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\0[\x83\x81\x10\x15a\r\x18W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r9\x81` \x86\x01` \x86\x01a\x0C\xFDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x88\x90\x83\x01\x84a\r!V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xA6W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\r\xC9W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\r\xF1W`\0\x80\xFD[\x80Qa\r\xFFa\x0B]\x82a\n\xDCV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0E\x14W`\0\x80\xFD[a\x07.\x82` \x83\x01` \x86\x01a\x0C\xFDV[` \x81R`\0a\x0E8` \x83\x01\x84a\r!V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0EQW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV\xFE\xA2dipfsX\"\x12 \xC1\xDC\xF4\xF30\x15\xE9D\xAD)E\n\xD4\xB2<\x0EW\x12\xB4\xE5X)\xB6\xD7\x11\xD7\xDB\xBCZ\x14\xD8\x1BdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static QUOTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c0\xD0\x7F!\x14a\0QW\x80c\xC4Z\x01U\x14a\0~W\x80c\xF7r\x9DC\x14a\0\xBDW\x80c\xFAF\x1E3\x14a\0\xD0W[`\0\x80\xFD[a\0da\0_6`\x04a\n%V[a\0\xE5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0uV[a\0da\0\xCB6`\x04a\n%V[a\x03#V[a\0\xE3a\0\xDE6`\x04a\x0B\x04V[a\x05\x04V[\0[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x88\x16\x10\x81R\x81\x90a\x01\x1C\x88\x88\x88a\x05\xE6V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90R`@\x80Qc8P\xC7\xBD`\xE0\x1B\x81R\x90Qc8P\xC7\xBD\x91`\x04\x80\x82\x01\x92`\xE0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x01gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x8B\x91\x90a\x0B\xAEV[PPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`@\x86\x01RPPP\x84\x16`\0\x03a\x01\xB0W`\0\x85\x90U[\x80` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x080\x83`\0\x01Qa\x01\xD2\x89a\x06$V[a\x01\xDB\x90a\x0ChV[`\x01`\x01`\xA0\x1B\x03\x89\x16\x15a\x01\xF0W\x88a\x02*V[\x85Qa\x02\x1AWa\x02\x15`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0C\x84V[a\x02*V[a\x02*d\x01\0\x02v\xA3`\x01a\x0C\xA3V[\x8C\x8C\x8F`@Q` \x01a\x02?\x93\x92\x91\x90a\x0C\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02n\x95\x94\x93\x92\x91\x90a\rMV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x02\xA8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x02\xA5\x91\x81\x01\x90a\r\x93V[`\x01[a\x03\x15W=\x80\x80\x15a\x02\xD6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\xDBV[``\x91P[P\x84`\x01`\x01`\xA0\x1B\x03\x16`\0\x03a\x02\xF2W`\0\x80U[a\x02\xFB\x81a\x06:V[`@\x90\x92\x01Q\x91\x93PP`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\x03\x19V[PPP[\x95P\x95\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x88\x16\x10\x81R\x81\x90a\x03Z\x88\x88\x88a\x05\xE6V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90R`@\x80Qc8P\xC7\xBD`\xE0\x1B\x81R\x90Qc8P\xC7\xBD\x91`\x04\x80\x82\x01\x92`\xE0\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xC9\x91\x90a\x0B\xAEV[PPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`@\x86\x01RPPP` \x82\x01Q\x82Q\x91\x16\x90c\x12\x8A\xCB\x08\x900\x90a\x03\xFA\x89a\x06$V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x15a\x04\x0FW\x88a\x04IV[\x85Qa\x049Wa\x044`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0C\x84V[a\x04IV[a\x04Id\x01\0\x02v\xA3`\x01a\x0C\xA3V[\x8D\x8C\x8E`@Q` \x01a\x04^\x93\x92\x91\x90a\x0C\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8D\x95\x94\x93\x92\x91\x90a\rMV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x04\xC7WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x04\xC4\x91\x81\x01\x90a\r\x93V[`\x01[a\x03\x15W=\x80\x80\x15a\x04\xF5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xFAV[``\x91P[Pa\x02\xFB\x81a\x06:V[`\0\x83\x13\x80a\x05\x13WP`\0\x82\x13[a\x05\x1CW`\0\x80\xFD[`\0\x80`\0a\x05*\x84a\x06\xDCV[\x92P\x92P\x92Pa\x05\\\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x84\x84a\x07\x18V[P`\0\x80`\0\x80\x89\x13a\x05\x8EW\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10\x88\x8Aa\x05\x89\x90a\x0ChV[a\x05\xAEV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x10\x89\x89a\x05\xAE\x90a\x0ChV[\x92P\x92P\x92P\x82\x15a\x05\xC5W`@Q\x81\x81R` \x81\xFD[`\0T\x15a\x05\xDBW`\0T\x81\x14a\x05\xDBW`\0\x80\xFD[`@Q\x82\x81R` \x81\xFD[`\0a\x06\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\x17\x86\x86\x86a\x077V[a\x07\xA2V[\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x10a\x066W`\0\x80\xFD[P\x90V[`\0\x81Q` \x14a\x06\xC2W`D\x82Q\x10\x15a\x06\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro*\xB72\xBC82\xB1\xBA2\xB2\x102\xB997\xB9`\x81\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x04\x82\x01\x91P\x81\x80` \x01\x90Q\x81\x01\x90a\x06\xA9\x91\x90a\r\xB7V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x86\x91\x90a\x0E%V[\x81\x80` \x01\x90Q\x81\x01\x90a\x06\xD6\x91\x90a\x0E?V[\x92\x91PPV[`\0\x80\x80a\x06\xEA\x84\x82a\x08\x8BV[\x92Pa\x06\xF7\x84`\x14a\t?V[\x90Pa\x07\x0Fa\x07\x08`\x03`\x14a\x0EXV[\x85\x90a\x08\x8BV[\x91P\x91\x93\x90\x92PV[`\0a\x07.\x85a\x07)\x86\x86\x86a\x077V[a\t\xEAV[\x95\x94PPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07rW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x07\xCAW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0\x81a\x08\x99\x81`\x14a\x0EXV[\x10\x15a\x08\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqtoAddress_overflow`p\x1B`D\x82\x01R`d\x01a\x06\x86V[a\x08\xE7\x82`\x14a\x0EXV[\x83Q\x10\x15a\t/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x06\x86V[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[`\0\x81a\tM\x81`\x03a\x0EXV[\x10\x15a\t\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x06\x86V[a\t\x9A\x82`\x03a\x0EXV[\x83Q\x10\x15a\t\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x06\x86V[P\x01`\x03\x01Q\x90V[`\0a\t\xF6\x83\x83a\x07\xA2V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x06\xD6W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\"W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\n=W`\0\x80\xFD[\x855a\nH\x81a\n\rV[\x94P` \x86\x015a\nX\x81a\n\rV[\x93P`@\x86\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\npW`\0\x80\xFD[\x92P``\x86\x015\x91P`\x80\x86\x015a\n\x87\x81a\n\rV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xD4Wa\n\xD4a\n\x95V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xF6Wa\n\xF6a\n\x95V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\x19W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B>W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0BOW`\0\x80\xFD[\x805a\x0Bba\x0B]\x82a\n\xDCV[a\n\xABV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x0BwW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x0B\xA9W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xC9W`\0\x80\xFD[\x87Qa\x0B\xD4\x81a\n\rV[\x80\x97PP` \x88\x01Q\x80`\x02\x0B\x81\x14a\x0B\xECW`\0\x80\xFD[\x95Pa\x0B\xFA`@\x89\x01a\x0B\x97V[\x94Pa\x0C\x08``\x89\x01a\x0B\x97V[\x93Pa\x0C\x16`\x80\x89\x01a\x0B\x97V[\x92P`\xA0\x88\x01Q`\xFF\x81\x16\x81\x14a\x0C,W`\0\x80\xFD[`\xC0\x89\x01Q\x90\x92P\x80\x15\x15\x81\x14a\x0CBW`\0\x80\xFD[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C}Wa\x0C}a\x0CRV[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV[``\x93\x84\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16\x82R`\xE8\x93\x90\x93\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16`\x14\x82\x01R\x92\x1B\x16`\x17\x82\x01R`+\x01\x90V[`\0[\x83\x81\x10\x15a\r\x18W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r9\x81` \x86\x01` \x86\x01a\x0C\xFDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x88\x90\x83\x01\x84a\r!V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xA6W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\r\xC9W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\r\xF1W`\0\x80\xFD[\x80Qa\r\xFFa\x0B]\x82a\n\xDCV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0E\x14W`\0\x80\xFD[a\x07.\x82` \x83\x01` \x86\x01a\x0C\xFDV[` \x81R`\0a\x0E8` \x83\x01\x84a\r!V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0EQW`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x06\xD6Wa\x06\xD6a\x0CRV\xFE\xA2dipfsX\"\x12 \xC1\xDC\xF4\xF30\x15\xE9D\xAD)E\n\xD4\xB2<\x0EW\x12\xB4\xE5X)\xB6\xD7\x11\xD7\xDB\xBCZ\x14\xD8\x1BdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static QUOTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Quoter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Quoter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Quoter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Quoter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Quoter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Quoter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Quoter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    QUOTER_ABI.clone(),
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
                QUOTER_ABI.clone(),
                QUOTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactInputSingle` (0xf7729d43) function
        pub fn quote_exact_input_single(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            fee: u32,
            amount_in: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [247, 114, 157, 67],
                    (token_in, token_out, fee, amount_in, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteExactOutputSingle` (0x30d07f21) function
        pub fn quote_exact_output_single(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            fee: u32,
            amount_out: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [48, 208, 127, 33],
                    (token_in, token_out, fee, amount_out, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, path))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Quoter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `quoteExactInputSingle` function with signature `quoteExactInputSingle(address,address,uint24,uint256,uint160)` and selector `0xf7729d43`
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
        name = "quoteExactInputSingle",
        abi = "quoteExactInputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactInputSingleCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub amount_in: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle(address,address,uint24,uint256,uint160)` and selector `0x30d07f21`
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
        name = "quoteExactOutputSingle",
        abi = "quoteExactOutputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactOutputSingleCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub amount_out: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
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
        pub path: ::ethers::core::types::Bytes,
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
    pub enum QuoterCalls {
        Factory(FactoryCall),
        QuoteExactInputSingle(QuoteExactInputSingleCall),
        QuoteExactOutputSingle(QuoteExactOutputSingleCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuoterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <QuoteExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactInputSingle(decoded));
            }
            if let Ok(decoded) = <QuoteExactOutputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteExactOutputSingle(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QuoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuoteExactInputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteExactOutputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for QuoterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteExactInputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuoteExactOutputSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FactoryCall> for QuoterCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<QuoteExactInputSingleCall> for QuoterCalls {
        fn from(value: QuoteExactInputSingleCall) -> Self {
            Self::QuoteExactInputSingle(value)
        }
    }
    impl ::core::convert::From<QuoteExactOutputSingleCall> for QuoterCalls {
        fn from(value: QuoteExactOutputSingleCall) -> Self {
            Self::QuoteExactOutputSingle(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for QuoterCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quoteExactInputSingle` function with signature `quoteExactInputSingle(address,address,uint24,uint256,uint160)` and selector `0xf7729d43`
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
    pub struct QuoteExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
        pub start_sqrt_price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quoteExactOutputSingle` function with signature `quoteExactOutputSingle(address,address,uint24,uint256,uint160)` and selector `0x30d07f21`
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
    pub struct QuoteExactOutputSingleReturn {
        pub amount_in: ::ethers::core::types::U256,
        pub start_sqrt_price: ::ethers::core::types::U256,
    }
}
