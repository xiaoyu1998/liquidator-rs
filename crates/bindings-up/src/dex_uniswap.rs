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
    pub use super::super::shared_types::*;
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDex2.SwapParams2"),
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
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDex2.SwapParams2"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEXUNISWAP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x0E\x9A8\x03\x80a\x0E\x9A\x839\x81\x01`@\x81\x90R`,\x91`<V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`jV[`\0` \x82\x84\x03\x12\x15`MW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0E\x0Fa\0\x8B`\09`\0\x81\x81`k\x01Ra\x06\x87\x01Ra\x0E\x0F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c<\xC8\xC0\xEF\x14a\0QW\x80c\xC4Z\x01U\x14a\0fW\x80c\xF6\xFCf\x9E\x14a\0\xA9W\x80c\xFAF\x1E3\x14a\0\xBCW[`\0\x80\xFD[a\0da\0_6`\x04a\n!V[a\0\xCFV[\0[a\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\xB76`\x04a\n!V[a\x01\xCFV[a\0da\0\xCA6`\x04a\n\xCFV[a\x02\xC8V[` \x82\x01Q\x82Q`@\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10\x92`\0\x92a\0\xFA\x92\x90\x91\x90a\x06\x80V[\x90P\x81\x15a\x01lWa\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x01,W\x88`\x80\x01Qa\x06\xBEV[\x86a\x01UWa\x01P`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x06\xBEV[a\x01Pd\x01\0\x02v\xA3`\x01a\x0B\x8DV[PPPPPV[a\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x01\x96W\x88`\x80\x01Qa\x07rV[\x86a\x01\xBFWa\x01\xBA`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07rV[a\x01\xBAd\x01\0\x02v\xA3`\x01a\x0B\x8DV[` \x82\x01Q\x82Q`@\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10\x92`\0\x92a\x01\xFA\x92\x90\x91\x90a\x06\x80V[\x90P\x81\x15a\x02eWa\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x02,W\x88`\x80\x01Qa\x07\x8DV[\x86a\x02UWa\x02P`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07\x8DV[a\x02Pd\x01\0\x02v\xA3`\x01a\x0B\x8DV[a\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x02\x8FW\x88`\x80\x01Qa\x07\xA8V[\x86a\x02\xB8Wa\x02\xB3`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07\xA8V[a\x02\xB3d\x01\0\x02v\xA3`\x01a\x0B\x8DV[`\0a\x02\xD6\x82\x84\x01\x84a\x0B\xACV[\x90Pa\x02\xF7`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\r\x92`H\x919PV[a\x03\"`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount0Delta`\xA0\x1B\x81RPPV[a\x03M`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount1Delta`\xA0\x1B\x81RPPV[a\x04:`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD1\x91\x90a\x0B\xD0V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x046\x91\x90\x81\x01\x90a\x0C\x11V[PPV[a\x04\x9A`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xADW=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x85\x13\x15a\x05\xC0W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05>\x91\x90a\x0B\xD0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBA\x91\x90a\x0C\xA7V[Pa\x01eV[`\0\x84\x13\x15a\x06hW3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06+\x91\x90a\x0B\xD0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01a\x05wV[\x84\x15\x80\x15a\x06tWP\x83\x15[a\x01eWa\x01ea\x0C\xC9V[`\0a\x06\xB6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xB1\x86\x86\x86a\x07\xC6V[a\x081V[\x94\x93PPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x06\xD9\x87a\t\x1AV[a\x06\xE2\x90a\x0C\xDFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07'\x95\x94\x93\x92\x91\x90a\r'V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07i\x91\x90a\rmV[PPPPPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x06\xD9\x87a\t\x1AV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x06\xE2\x87a\t\x1AV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x06\xE2\x87a\t\x1AV[PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08\x01W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08YW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\t\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xC3W`\0\x80\xFD[\x805a\t\xAC\x81a\t\x8CV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xEAWa\t\xEAa\t\xB1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x19Wa\n\x19a\t\xB1V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a\n7W`\0\x80\xFD[\x845a\nB\x81a\t\x8CV[\x93P`\xA0`\x1F\x19\x82\x01\x12\x15a\nVW`\0\x80\xFD[Pa\n_a\t\xC7V[` \x85\x015a\nm\x81a\t\x8CV[\x81R`@\x85\x015a\n}\x81a\t\x8CV[` \x82\x01R``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\n\x98W`\0\x80\xFD[`@\x82\x01R`\x80\x85\x015``\x82\x01Ra\n\xB3`\xA0\x86\x01a\t\xA1V[`\x80\x82\x01R\x91Pa\n\xC6`\xC0\x85\x01a\t\xA1V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n\xE5W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\nW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x0B\x1BW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B2W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x0BDW`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0B\x87Wa\x0B\x87a\x0BRV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0B\x87Wa\x0B\x87a\x0BRV[`\0` \x82\x84\x03\x12\x15a\x0B\xBEW`\0\x80\xFD[\x815a\x0B\xC9\x81a\t\x8CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xE2W`\0\x80\xFD[\x81Qa\x0B\xC9\x81a\t\x8CV[`\0[\x83\x81\x10\x15a\x0C\x08W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xF0V[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x0C#W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C:W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0CKW`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CeWa\x0Cea\t\xB1V[a\x0Cx`\x1F\x82\x01`\x1F\x19\x16` \x01a\t\xF0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0C\x8DW`\0\x80\xFD[a\x0C\x9E\x82` \x83\x01` \x86\x01a\x0B\xEDV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B\xC9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C\xF4Wa\x0C\xF4a\x0BRV[P`\0\x03\x90V[`\0\x81Q\x80\x84Ra\r\x13\x81` \x86\x01` \x86\x01a\x0B\xEDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\rb\x90\x83\x01\x84a\x0C\xFBV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\x80W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV\xFE-------------------------uniswapV3SwapCallback--------------------------\xA2dipfsX\"\x12 \xB3m\xF2\xD5K\xDA\x81\x97|\xAB\x0E\xDF/\x83F\xD1B\xC1j\x88\xFC\x1C<o \x8Dz\xD4p\x14\xAD\xC4dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static DEXUNISWAP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c<\xC8\xC0\xEF\x14a\0QW\x80c\xC4Z\x01U\x14a\0fW\x80c\xF6\xFCf\x9E\x14a\0\xA9W\x80c\xFAF\x1E3\x14a\0\xBCW[`\0\x80\xFD[a\0da\0_6`\x04a\n!V[a\0\xCFV[\0[a\0\x8D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\xB76`\x04a\n!V[a\x01\xCFV[a\0da\0\xCA6`\x04a\n\xCFV[a\x02\xC8V[` \x82\x01Q\x82Q`@\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10\x92`\0\x92a\0\xFA\x92\x90\x91\x90a\x06\x80V[\x90P\x81\x15a\x01lWa\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x01,W\x88`\x80\x01Qa\x06\xBEV[\x86a\x01UWa\x01P`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x06\xBEV[a\x01Pd\x01\0\x02v\xA3`\x01a\x0B\x8DV[PPPPPV[a\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x01\x96W\x88`\x80\x01Qa\x07rV[\x86a\x01\xBFWa\x01\xBA`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07rV[a\x01\xBAd\x01\0\x02v\xA3`\x01a\x0B\x8DV[` \x82\x01Q\x82Q`@\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10\x92`\0\x92a\x01\xFA\x92\x90\x91\x90a\x06\x80V[\x90P\x81\x15a\x02eWa\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x02,W\x88`\x80\x01Qa\x07\x8DV[\x86a\x02UWa\x02P`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07\x8DV[a\x02Pd\x01\0\x02v\xA3`\x01a\x0B\x8DV[a\x01e\x85\x82\x86``\x01Q\x86\x88`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x14a\x02\x8FW\x88`\x80\x01Qa\x07\xA8V[\x86a\x02\xB8Wa\x02\xB3`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0BhV[a\x07\xA8V[a\x02\xB3d\x01\0\x02v\xA3`\x01a\x0B\x8DV[`\0a\x02\xD6\x82\x84\x01\x84a\x0B\xACV[\x90Pa\x02\xF7`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\r\x92`H\x919PV[a\x03\"`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount0Delta`\xA0\x1B\x81RPPV[a\x03M`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount1Delta`\xA0\x1B\x81RPPV[a\x04:`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD1\x91\x90a\x0B\xD0V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x046\x91\x90\x81\x01\x90a\x0C\x11V[PPV[a\x04\x9A`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xADW=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x85\x13\x15a\x05\xC0W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05>\x91\x90a\x0B\xD0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBA\x91\x90a\x0C\xA7V[Pa\x01eV[`\0\x84\x13\x15a\x06hW3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06+\x91\x90a\x0B\xD0V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01a\x05wV[\x84\x15\x80\x15a\x06tWP\x83\x15[a\x01eWa\x01ea\x0C\xC9V[`\0a\x06\xB6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06\xB1\x86\x86\x86a\x07\xC6V[a\x081V[\x94\x93PPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x06\xD9\x87a\t\x1AV[a\x06\xE2\x90a\x0C\xDFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07'\x95\x94\x93\x92\x91\x90a\r'V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07i\x91\x90a\rmV[PPPPPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x06\xD9\x87a\t\x1AV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x06\xE2\x87a\t\x1AV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x06\xE2\x87a\t\x1AV[PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x08\x01W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08YW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\t\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xC3W`\0\x80\xFD[\x805a\t\xAC\x81a\t\x8CV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xEAWa\t\xEAa\t\xB1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x19Wa\n\x19a\t\xB1V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a\n7W`\0\x80\xFD[\x845a\nB\x81a\t\x8CV[\x93P`\xA0`\x1F\x19\x82\x01\x12\x15a\nVW`\0\x80\xFD[Pa\n_a\t\xC7V[` \x85\x015a\nm\x81a\t\x8CV[\x81R`@\x85\x015a\n}\x81a\t\x8CV[` \x82\x01R``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14a\n\x98W`\0\x80\xFD[`@\x82\x01R`\x80\x85\x015``\x82\x01Ra\n\xB3`\xA0\x86\x01a\t\xA1V[`\x80\x82\x01R\x91Pa\n\xC6`\xC0\x85\x01a\t\xA1V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n\xE5W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\nW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x0B\x1BW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B2W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x0BDW`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0B\x87Wa\x0B\x87a\x0BRV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0B\x87Wa\x0B\x87a\x0BRV[`\0` \x82\x84\x03\x12\x15a\x0B\xBEW`\0\x80\xFD[\x815a\x0B\xC9\x81a\t\x8CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xE2W`\0\x80\xFD[\x81Qa\x0B\xC9\x81a\t\x8CV[`\0[\x83\x81\x10\x15a\x0C\x08W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xF0V[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x0C#W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C:W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0CKW`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CeWa\x0Cea\t\xB1V[a\x0Cx`\x1F\x82\x01`\x1F\x19\x16` \x01a\t\xF0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x0C\x8DW`\0\x80\xFD[a\x0C\x9E\x82` \x83\x01` \x86\x01a\x0B\xEDV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\xB9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B\xC9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C\xF4Wa\x0C\xF4a\x0BRV[P`\0\x03\x90V[`\0\x81Q\x80\x84Ra\r\x13\x81` \x86\x01` \x86\x01a\x0B\xEDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\rb\x90\x83\x01\x84a\x0C\xFBV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\x80W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV\xFE-------------------------uniswapV3SwapCallback--------------------------\xA2dipfsX\"\x12 \xB3m\xF2\xD5K\xDA\x81\x97|\xAB\x0E\xDF/\x83F\xD1B\xC1j\x88\xFC\x1C<o \x8Dz\xD4p\x14\xAD\xC4dsolcC\0\x08\x1A\x003";
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
        ///Calls the contract's `swapExactIn` (0xf6fc669e) function
        pub fn swap_exact_in(
            &self,
            from: ::ethers::core::types::Address,
            params: SwapParams2,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 252, 102, 158], (from, params, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactOut` (0x3cc8c0ef) function
        pub fn swap_exact_out(
            &self,
            from: ::ethers::core::types::Address,
            params: SwapParams2,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 200, 192, 239], (from, params, to))
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
    ///Container type for all input parameters for the `swapExactIn` function with signature `swapExactIn(address,(address,address,uint24,uint256,uint160),address)` and selector `0xf6fc669e`
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
        abi = "swapExactIn(address,(address,address,uint24,uint256,uint160),address)"
    )]
    pub struct SwapExactInCall {
        pub from: ::ethers::core::types::Address,
        pub params: SwapParams2,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapExactOut` function with signature `swapExactOut(address,(address,address,uint24,uint256,uint160),address)` and selector `0x3cc8c0ef`
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
        abi = "swapExactOut(address,(address,address,uint24,uint256,uint160),address)"
    )]
    pub struct SwapExactOutCall {
        pub from: ::ethers::core::types::Address,
        pub params: SwapParams2,
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
}
