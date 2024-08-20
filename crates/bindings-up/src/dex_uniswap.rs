pub use dex_uniswap::*;
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
pub mod dex_uniswap {
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
                        name: ::std::borrow::ToOwned::to_owned("_factory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feeAmount"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint24"),
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
                    ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapFee"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("swapExactIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("swapExactOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
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
    pub static DEXUNISWAP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0F=8\x03\x80a\x0F=\x839\x81\x01`@\x81\x90Ra\0/\x91a\0vV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x16`\xA0R`\0\x80Tb\xFF\xFF\xFF\x19\x16b\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\xCBV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0sW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\x8BW`\0\x80\xFD[\x83Qa\0\x96\x81a\0^V[` \x85\x01Q\x90\x93Pa\0\xA7\x81a\0^V[`@\x85\x01Q\x90\x92Pb\xFF\xFF\xFF\x81\x16\x81\x14a\0\xC0W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Qa\x0E2a\x01\x0B`\09`\0\x81\x81`\xA7\x01Ra\x06\r\x01R`\0\x81\x81a\x01I\x01R\x81\x81a\x02\x99\x01R\x81\x81a\x04\xD4\x01Ra\x06\xB5\x01Ra\x0E2`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x9E;\xB5\x9D\x11a\0[W\x80c\x9E;\xB5\x9D\x14a\x01\x1AW\x80c\xBF\\\x98\"\x14a\x01-W\x80c\xC4Z\x01U\x14a\x01DW\x80c\xFAF\x1E3\x14a\x01kW`\0\x80\xFD[\x80c,g\xE2W\x14a\0\x8DW\x80cJJ{\x04\x14a\0\xA2W\x80cS\x1A\xA0>\x14a\0\xE6W\x80c\x8E}i5\x14a\0\xF9W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\n\xEBV[a\x01~V[\0[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC9a\0\xF46`\x04a\x0B[V[a\x02\x8FV[a\x01\x0Ca\x01\x076`\x04a\x0B\x94V[a\x02\xD9V[`@Q\x90\x81R` \x01a\0\xDDV[a\0\xA0a\x01(6`\x04a\n\xEBV[a\x02\xF1V[`\0T`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xDDV[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA0a\x01y6`\x04a\x0B\xADV[a\x03\xB4V[a\x01\xED`@Q` \x01a\x01\xAF\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05\xF1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x90\x88\x16\x10\x91\x90a\x02\x14\x90\x88\x90\x88\x90b\xFF\xFF\xFF\x16a\x06\xAEV[\x90P`\0\x84\x15a\x02$W\x84a\x02]V[\x82a\x02MWa\x02H`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0CFV[a\x02]V[a\x02]d\x01\0\x02v\xA3`\x01a\x0CeV[\x90P\x82\x15a\x02zWa\x02r\x89\x83\x88\x87\x85a\x06\xE9V[PPPa\x02\x87V[a\x02r\x89\x83\x88\x87\x85a\x07\x8BV[PPPPPPV[`\0\x80Ta\x02\xD0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x02\xCB\x90\x86\x90\x86\x90b\xFF\xFF\xFF\x16a\x07\xA6V[a\x08\x11V[\x90P[\x92\x91PPV[`\0\x80Ta\x02\xD3\x90\x83\x90b\xFF\xFF\xFF\x16b\x0FB@a\x08\xFAV[a\x03\"`@Q` \x01a\x01\xAF\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x90\x88\x16\x10\x91\x90a\x03I\x90\x88\x90\x88\x90b\xFF\xFF\xFF\x16a\x06\xAEV[\x90P`\0\x84\x15a\x03YW\x84a\x03\x92V[\x82a\x03\x82Wa\x03}`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0CFV[a\x03\x92V[a\x03\x92d\x01\0\x02v\xA3`\x01a\x0CeV[\x90P\x82\x15a\x03\xA7Wa\x02r\x89\x83\x88\x87\x85a\t\xE4V[a\x02r\x89\x83\x88\x87\x85a\n\x08V[`\0a\x03\xC2\x82\x84\x01\x84a\x0C\x84V[`@\x80Q\x87\x81R` \x81\x01\x87\x90R\x91\x92P\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\x003`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04b\x91\x90a\x0C\xA1V[\x90P`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC8\x91\x90a\x0C\xA1V[`\0T\x90\x91Pa\x05\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x84\x90\x84\x90b\xFF\xFF\xFF\x16a\n#V[P`\0\x87\x13\x15a\x05\x8CW`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x89\x90R\x83\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x86\x91\x90a\x0C\xBEV[Pa\x05\xE8V[`\0\x86\x13\x15a\x05\xD0W`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x82\x16\x90c#\xB8r\xDD\x90`d\x01a\x05CV[\x86\x15\x80\x15a\x05\xDCWP\x85\x15[a\x05\xE8Wa\x05\xE8a\x0C\xE0V[PPPPPPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x80\x91\x90a\x0C\xBEV[a\x06\xAAW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x06\xA1\x92\x91\x90a\r<V[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a\x06\xDF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCB\x86\x86\x86a\x07\xA6V[\x90P[\x93\x92PPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07\x04\x87a\nBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07I\x95\x94\x93\x92\x91\x90a\r`V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE8\x91\x90a\r\xA6V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07\x04\x87a\nBV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07\xE1W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x089W`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\t4W\x83\x82\x81a\t*Wa\t*a\r\xCAV[\x04\x92PPPa\x06\xE2V[\x80\x84\x11a\t{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x06\xA1V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\t\xFF\x87a\nBV[a\x07\x04\x90a\r\xE0V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\t\xFF\x87a\nBV[`\0a\n9\x85a\n4\x86\x86\x86a\x07\xA6V[a\n\xB0V[\x95\x94PPPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\x06\xA1V[P\x90V[`\0a\n\xBC\x83\x83a\x08\x11V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02\xD3W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xE8W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0B\x04W`\0\x80\xFD[\x865a\x0B\x0F\x81a\n\xD3V[\x95P` \x87\x015a\x0B\x1F\x81a\n\xD3V[\x94P`@\x87\x015a\x0B/\x81a\n\xD3V[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015a\x0BM\x81a\n\xD3V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x0BnW`\0\x80\xFD[\x825a\x0By\x81a\n\xD3V[\x91P` \x83\x015a\x0B\x89\x81a\n\xD3V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xA6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0B\xC3W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xE8W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x0B\xF9W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x10W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x0C\"W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02\xD3Wa\x02\xD3a\x0C0V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x02\xD3Wa\x02\xD3a\x0C0V[`\0` \x82\x84\x03\x12\x15a\x0C\x96W`\0\x80\xFD[\x815a\x06\xE2\x81a\n\xD3V[`\0` \x82\x84\x03\x12\x15a\x0C\xB3W`\0\x80\xFD[\x81Qa\x06\xE2\x81a\n\xD3V[`\0` \x82\x84\x03\x12\x15a\x0C\xD0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xE2W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r\x1CW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\0V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x06\xDF\x90\x83\x01\x84a\x0C\xF6V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x9B\x90\x83\x01\x84a\x0C\xF6V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB9W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\r\xF5Wa\r\xF5a\x0C0V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \x9A\xE2x\xC2\xC6\x83\x80X\xF5\xB6\x10\x8E\x08\x9F0\x9DQ{\xAC[Q\xFB\xD1h]\x86\xE8\x11\x86,\x14\rdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static DEXUNISWAP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x9E;\xB5\x9D\x11a\0[W\x80c\x9E;\xB5\x9D\x14a\x01\x1AW\x80c\xBF\\\x98\"\x14a\x01-W\x80c\xC4Z\x01U\x14a\x01DW\x80c\xFAF\x1E3\x14a\x01kW`\0\x80\xFD[\x80c,g\xE2W\x14a\0\x8DW\x80cJJ{\x04\x14a\0\xA2W\x80cS\x1A\xA0>\x14a\0\xE6W\x80c\x8E}i5\x14a\0\xF9W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\n\xEBV[a\x01~V[\0[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC9a\0\xF46`\x04a\x0B[V[a\x02\x8FV[a\x01\x0Ca\x01\x076`\x04a\x0B\x94V[a\x02\xD9V[`@Q\x90\x81R` \x01a\0\xDDV[a\0\xA0a\x01(6`\x04a\n\xEBV[a\x02\xF1V[`\0T`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xDDV[a\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA0a\x01y6`\x04a\x0B\xADV[a\x03\xB4V[a\x01\xED`@Q` \x01a\x01\xAF\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05\xF1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x90\x88\x16\x10\x91\x90a\x02\x14\x90\x88\x90\x88\x90b\xFF\xFF\xFF\x16a\x06\xAEV[\x90P`\0\x84\x15a\x02$W\x84a\x02]V[\x82a\x02MWa\x02H`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0CFV[a\x02]V[a\x02]d\x01\0\x02v\xA3`\x01a\x0CeV[\x90P\x82\x15a\x02zWa\x02r\x89\x83\x88\x87\x85a\x06\xE9V[PPPa\x02\x87V[a\x02r\x89\x83\x88\x87\x85a\x07\x8BV[PPPPPPV[`\0\x80Ta\x02\xD0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x02\xCB\x90\x86\x90\x86\x90b\xFF\xFF\xFF\x16a\x07\xA6V[a\x08\x11V[\x90P[\x92\x91PPV[`\0\x80Ta\x02\xD3\x90\x83\x90b\xFF\xFF\xFF\x16b\x0FB@a\x08\xFAV[a\x03\"`@Q` \x01a\x01\xAF\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x90\x88\x16\x10\x91\x90a\x03I\x90\x88\x90\x88\x90b\xFF\xFF\xFF\x16a\x06\xAEV[\x90P`\0\x84\x15a\x03YW\x84a\x03\x92V[\x82a\x03\x82Wa\x03}`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0CFV[a\x03\x92V[a\x03\x92d\x01\0\x02v\xA3`\x01a\x0CeV[\x90P\x82\x15a\x03\xA7Wa\x02r\x89\x83\x88\x87\x85a\t\xE4V[a\x02r\x89\x83\x88\x87\x85a\n\x08V[`\0a\x03\xC2\x82\x84\x01\x84a\x0C\x84V[`@\x80Q\x87\x81R` \x81\x01\x87\x90R\x91\x92P\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\x003`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04b\x91\x90a\x0C\xA1V[\x90P`\x003`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC8\x91\x90a\x0C\xA1V[`\0T\x90\x91Pa\x05\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x84\x90\x84\x90b\xFF\xFF\xFF\x16a\n#V[P`\0\x87\x13\x15a\x05\x8CW`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x89\x90R\x83\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x86\x91\x90a\x0C\xBEV[Pa\x05\xE8V[`\0\x86\x13\x15a\x05\xD0W`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x82\x16\x90c#\xB8r\xDD\x90`d\x01a\x05CV[\x86\x15\x80\x15a\x05\xDCWP\x85\x15[a\x05\xE8Wa\x05\xE8a\x0C\xE0V[PPPPPPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x80\x91\x90a\x0C\xBEV[a\x06\xAAW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x06\xA1\x92\x91\x90a\r<V[`@Q\x80\x91\x03\x90\xFD[PPV[`\0a\x06\xDF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xCB\x86\x86\x86a\x07\xA6V[\x90P[\x93\x92PPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07\x04\x87a\nBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07I\x95\x94\x93\x92\x91\x90a\r`V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE8\x91\x90a\r\xA6V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07\x04\x87a\nBV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07\xE1W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x089W`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\t4W\x83\x82\x81a\t*Wa\t*a\r\xCAV[\x04\x92PPPa\x06\xE2V[\x80\x84\x11a\t{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x06\xA1V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\t\xFF\x87a\nBV[a\x07\x04\x90a\r\xE0V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\t\xFF\x87a\nBV[`\0a\n9\x85a\n4\x86\x86\x86a\x07\xA6V[a\n\xB0V[\x95\x94PPPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\n\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\x06\xA1V[P\x90V[`\0a\n\xBC\x83\x83a\x08\x11V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02\xD3W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xE8W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0B\x04W`\0\x80\xFD[\x865a\x0B\x0F\x81a\n\xD3V[\x95P` \x87\x015a\x0B\x1F\x81a\n\xD3V[\x94P`@\x87\x015a\x0B/\x81a\n\xD3V[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015a\x0BM\x81a\n\xD3V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15a\x0BnW`\0\x80\xFD[\x825a\x0By\x81a\n\xD3V[\x91P` \x83\x015a\x0B\x89\x81a\n\xD3V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xA6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x0B\xC3W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xE8W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x0B\xF9W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x10W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x0C\"W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x02\xD3Wa\x02\xD3a\x0C0V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x02\xD3Wa\x02\xD3a\x0C0V[`\0` \x82\x84\x03\x12\x15a\x0C\x96W`\0\x80\xFD[\x815a\x06\xE2\x81a\n\xD3V[`\0` \x82\x84\x03\x12\x15a\x0C\xB3W`\0\x80\xFD[\x81Qa\x06\xE2\x81a\n\xD3V[`\0` \x82\x84\x03\x12\x15a\x0C\xD0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xE2W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r\x1CW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\0V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x06\xDF\x90\x83\x01\x84a\x0C\xF6V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\r\x9B\x90\x83\x01\x84a\x0C\xF6V[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB9W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\r\xF5Wa\r\xF5a\x0C0V[P`\0\x03\x90V\xFE\xA2dipfsX\"\x12 \x9A\xE2x\xC2\xC6\x83\x80X\xF5\xB6\x10\x8E\x08\x9F0\x9DQ{\xAC[Q\xFB\xD1h]\x86\xE8\x11\x86,\x14\rdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static DEXUNISWAP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DexUniswap<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DexUniswap<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DexUniswap<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DexUniswap<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DexUniswap<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DexUniswap)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DexUniswap<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEXUNISWAP_ABI.clone(),
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
                DEXUNISWAP_ABI.clone(),
                DEXUNISWAP_BYTECODE.clone().into(),
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
        ///Calls the contract's `getFeeAmount` (0xbf5c9822) function
        pub fn get_fee_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 92, 152, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x531aa03e) function
        pub fn get_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 26, 160, 62], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFee` (0x8e7d6935) function
        pub fn get_swap_fee(
            &self,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 125, 105, 53], amount_in)
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
        ///Calls the contract's `swapExactIn` (0x2c67e257) function
        pub fn swap_exact_in(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 103, 226, 87],
                    (from, token_in, token_out, amount, sqrt_price_limit_x96, to),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactOut` (0x9e3bb59d) function
        pub fn swap_exact_out(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [158, 59, 181, 157],
                    (from, token_in, token_out, amount, sqrt_price_limit_x96, to),
                )
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
        ///Gets the contract's `SwapCallback` event
        pub fn swap_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapCallbackFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapCallbackFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DexUniswap<M> {
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
    #[ethevent(name = "SwapCallback", abi = "SwapCallback(int256,int256)")]
    pub struct SwapCallbackFilter {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
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
    #[ethcall(name = "getFeeAmount", abi = "getFeeAmount()")]
    pub struct GetFeeAmountCall;
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
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
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
    #[ethcall(name = "getSwapFee", abi = "getSwapFee(uint256)")]
    pub struct GetSwapFeeCall {
        pub amount_in: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `swapExactIn` function with signature `swapExactIn(address,address,address,uint256,uint256,address)` and selector `0x2c67e257`
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
        name = "swapExactIn",
        abi = "swapExactIn(address,address,address,uint256,uint256,address)"
    )]
    pub struct SwapExactInCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapExactOut` function with signature `swapExactOut(address,address,address,uint256,uint256,address)` and selector `0x9e3bb59d`
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
        name = "swapExactOut",
        abi = "swapExactOut(address,address,address,uint256,uint256,address)"
    )]
    pub struct SwapExactOutCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
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
    pub enum DexUniswapCalls {
        Factory(FactoryCall),
        GetFeeAmount(GetFeeAmountCall),
        GetPool(GetPoolCall),
        GetSwapFee(GetSwapFeeCall),
        RoleStore(RoleStoreCall),
        SwapExactIn(SwapExactInCall),
        SwapExactOut(SwapExactOutCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for DexUniswapCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFeeAmount(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <SwapExactInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactIn(decoded));
            }
            if let Ok(decoded) = <SwapExactOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactOut(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DexUniswapCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DexUniswapCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FactoryCall> for DexUniswapCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetFeeAmountCall> for DexUniswapCalls {
        fn from(value: GetFeeAmountCall) -> Self {
            Self::GetFeeAmount(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for DexUniswapCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetSwapFeeCall> for DexUniswapCalls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for DexUniswapCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<SwapExactInCall> for DexUniswapCalls {
        fn from(value: SwapExactInCall) -> Self {
            Self::SwapExactIn(value)
        }
    }
    impl ::core::convert::From<SwapExactOutCall> for DexUniswapCalls {
        fn from(value: SwapExactOutCall) -> Self {
            Self::SwapExactOut(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for DexUniswapCalls {
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
    ///Container type for all return fields from the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
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
    pub struct GetFeeAmountReturn(pub u32);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address)` and selector `0x531aa03e`
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
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
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
    pub struct GetSwapFeeReturn(pub ::ethers::core::types::U256);
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
