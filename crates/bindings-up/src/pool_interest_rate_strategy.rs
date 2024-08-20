pub use pool_interest_rate_strategy::*;
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
pub mod pool_interest_rate_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("optimalUsageRatio"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rateBase"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rateSlope1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rateSlope2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_EXCESS_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_EXCESS_USAGE_RATIO",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("OPTIMAL_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMAL_USAGE_RATIO",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("calculateInterestRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInterestRates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct InterestUtils.CalculateInterestRatesParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getOptimalUsageRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOptimalUsageRatio",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRateSlope1"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRateSlope2"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getRatebase"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRatebase"),
                            inputs: ::std::vec![],
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static POOLINTERESTRATESTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x0798\x03\x80a\x079\x839\x81\x01`@\x81\x90Ra\x000\x91a\0\x95V[\x83k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10\x15a\0dW`@Qc\\\x86\xF9\xF9`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x84\x90Ra\0\x7F\x84k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\0\xCBV[`\xA0Ra\x01\0\x92\x90\x92R`\xC0R`\xE0RPa\0\xF2V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\0\xABW`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x81\x81\x03\x81\x81\x11\x15a\0\xECWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x05\xC7a\x01r`\09`\0\x81\x81a\x01\x06\x01Ra\x01\xE9\x01R`\0\x81\x81`\xE0\x01Ra\x02\xCF\x01R`\0\x81\x81`\x84\x01R\x81\x81a\x02\xFA\x01Ra\x03g\x01R`\0\x81\x81a\x01}\x01Ra\x02s\x01R`\0\x81\x81`\xBC\x01R\x81\x81a\x01T\x01R\x81\x81a\x02B\x01R\x81\x81a\x02\x94\x01Ra\x03>\x01Ra\x05\xC7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x87\xF0@\x9D\x11a\0[W\x80c\x87\xF0@\x9D\x14a\x01\x04W\x80c\x9D\xADa\x99\x14a\x01*W\x80c\xA8`.\x86\x14a\x01RW\x80c\xA9\xC6\"\xF8\x14a\x01xW`\0\x80\xFD[\x80c-\xD9\x03{\x14a\0\x82W\x80cT\xC3e\xC6\x14a\0\xB7W\x80cu\0[\xB0\x14a\0\xDEW[`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[a\x01=a\x0186`\x04a\x04\xC0V[a\x01\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xAEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x01\xDB`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[` \x80\x85\x01Q\x90\x82\x01\x81\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01R\x15a\x02@W\x83Q\x80\x82R` \x82\x01Qa\x02%\x91a\x05eV[`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x02:\x91a\x03\xFBV[`\x80\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x80\x01Q\x11\x15a\x039W`\0a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\x80\x01Qa\x02\xC2\x91\x90a\x05~V[\x90a\x03\xFBV[\x90Pa\x02\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x04:V[a\x03\x1E\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05eV[\x82`@\x01\x81\x81Qa\x03/\x91\x90a\x05eV[\x90RPa\x03\xA9\x90PV[a\x03\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xC2\x83`\x80\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81`@\x01\x81\x81Qa\x03\xA5\x91\x90a\x05eV[\x90RP[` \x81\x01Q\x15a\x03\xE7Wa\x03\xE1\x84`@\x01Qa'\x10a\x03\xC8\x91\x90a\x05~V[`\x80\x83\x01Q`@\x84\x01Qa\x03\xDB\x91a\x04:V[\x90a\x04~V[``\x82\x01R[\x80``\x01Q\x81`@\x01Q\x92P\x92PP\x91P\x91V[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x04\x1FW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x04\\W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a\x04\x96W`\0\x80\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xBBW`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x80\x15a\x04\xD3W`\0\x80\xFD[`\0\x90P`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\x06WcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01\x81\x90R\x91Pa\x051``\x85\x01a\x04\xA4V[``\x82\x01Ra\x05B`\x80\x85\x01a\x04\xA4V[`\x80\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05xWa\x05xa\x05OV[\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x05xWa\x05xa\x05OV\xFE\xA2dipfsX\"\x12 \xFC\xC3\x83\xED(*\xB2\xAB|\xE5t\xA8X\xDC\\\xB0C\xB8|\x1A\x94c\x15\xF4p\xDD\xCES\"3\x91\x04dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static POOLINTERESTRATESTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x87\xF0@\x9D\x11a\0[W\x80c\x87\xF0@\x9D\x14a\x01\x04W\x80c\x9D\xADa\x99\x14a\x01*W\x80c\xA8`.\x86\x14a\x01RW\x80c\xA9\xC6\"\xF8\x14a\x01xW`\0\x80\xFD[\x80c-\xD9\x03{\x14a\0\x82W\x80cT\xC3e\xC6\x14a\0\xB7W\x80cu\0[\xB0\x14a\0\xDEW[`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[a\x01=a\x0186`\x04a\x04\xC0V[a\x01\x9FV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xAEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0\xA4V[a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x80a\x01\xDB`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[` \x80\x85\x01Q\x90\x82\x01\x81\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x83\x01R\x15a\x02@W\x83Q\x80\x82R` \x82\x01Qa\x02%\x91a\x05eV[`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x02:\x91a\x03\xFBV[`\x80\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x80\x01Q\x11\x15a\x039W`\0a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\x80\x01Qa\x02\xC2\x91\x90a\x05~V[\x90a\x03\xFBV[\x90Pa\x02\xF4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x04:V[a\x03\x1E\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05eV[\x82`@\x01\x81\x81Qa\x03/\x91\x90a\x05eV[\x90RPa\x03\xA9\x90PV[a\x03\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\xC2\x83`\x80\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81`@\x01\x81\x81Qa\x03\xA5\x91\x90a\x05eV[\x90RP[` \x81\x01Q\x15a\x03\xE7Wa\x03\xE1\x84`@\x01Qa'\x10a\x03\xC8\x91\x90a\x05~V[`\x80\x83\x01Q`@\x84\x01Qa\x03\xDB\x91a\x04:V[\x90a\x04~V[``\x82\x01R[\x80``\x01Q\x81`@\x01Q\x92P\x92PP\x91P\x91V[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x04\x1FW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x04\\W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a\x04\x96W`\0\x80\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xBBW`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x80\x15a\x04\xD3W`\0\x80\xFD[`\0\x90P`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x05\x06WcNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[`@\x90\x81R\x845\x82R` \x80\x86\x015\x90\x83\x01R\x84\x81\x015\x90\x82\x01\x81\x90R\x91Pa\x051``\x85\x01a\x04\xA4V[``\x82\x01Ra\x05B`\x80\x85\x01a\x04\xA4V[`\x80\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x05xWa\x05xa\x05OV[\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x05xWa\x05xa\x05OV\xFE\xA2dipfsX\"\x12 \xFC\xC3\x83\xED(*\xB2\xAB|\xE5t\xA8X\xDC\\\xB0C\xB8|\x1A\x94c\x15\xF4p\xDD\xCES\"3\x91\x04dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static POOLINTERESTRATESTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PoolInterestRateStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoolInterestRateStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoolInterestRateStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoolInterestRateStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoolInterestRateStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PoolInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoolInterestRateStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POOLINTERESTRATESTRATEGY_ABI.clone(),
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
                POOLINTERESTRATESTRATEGY_ABI.clone(),
                POOLINTERESTRATESTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_EXCESS_USAGE_RATIO` (0xa9c622f8) function
        pub fn max_excess_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 198, 34, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMAL_USAGE_RATIO` (0x54c365c6) function
        pub fn optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 195, 101, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateInterestRates` (0x9dad6199) function
        pub fn calculate_interest_rates(
            &self,
            params: CalculateInterestRatesParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 173, 97, 153], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOptimalUsageRatio` (0xa8602e86) function
        pub fn get_optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 96, 46, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRateSlope1` (0x2dd9037b) function
        pub fn get_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 217, 3, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRateSlope2` (0x75005bb0) function
        pub fn get_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 0, 91, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRatebase` (0x87f0409d) function
        pub fn get_ratebase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([135, 240, 64, 157], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PoolInterestRateStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
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
    #[ethcall(name = "MAX_EXCESS_USAGE_RATIO", abi = "MAX_EXCESS_USAGE_RATIO()")]
    pub struct MaxExcessUsageRatioCall;
    ///Container type for all input parameters for the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
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
    #[ethcall(name = "OPTIMAL_USAGE_RATIO", abi = "OPTIMAL_USAGE_RATIO()")]
    pub struct OptimalUsageRatioCall;
    ///Container type for all input parameters for the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,address,address))` and selector `0x9dad6199`
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
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    ///Container type for all input parameters for the `getOptimalUsageRatio` function with signature `getOptimalUsageRatio()` and selector `0xa8602e86`
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
    #[ethcall(name = "getOptimalUsageRatio", abi = "getOptimalUsageRatio()")]
    pub struct GetOptimalUsageRatioCall;
    ///Container type for all input parameters for the `getRateSlope1` function with signature `getRateSlope1()` and selector `0x2dd9037b`
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
    #[ethcall(name = "getRateSlope1", abi = "getRateSlope1()")]
    pub struct GetRateSlope1Call;
    ///Container type for all input parameters for the `getRateSlope2` function with signature `getRateSlope2()` and selector `0x75005bb0`
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
    #[ethcall(name = "getRateSlope2", abi = "getRateSlope2()")]
    pub struct GetRateSlope2Call;
    ///Container type for all input parameters for the `getRatebase` function with signature `getRatebase()` and selector `0x87f0409d`
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
    #[ethcall(name = "getRatebase", abi = "getRatebase()")]
    pub struct GetRatebaseCall;
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
    pub enum PoolInterestRateStrategyCalls {
        MaxExcessUsageRatio(MaxExcessUsageRatioCall),
        OptimalUsageRatio(OptimalUsageRatioCall),
        CalculateInterestRates(CalculateInterestRatesCall),
        GetOptimalUsageRatio(GetOptimalUsageRatioCall),
        GetRateSlope1(GetRateSlope1Call),
        GetRateSlope2(GetRateSlope2Call),
        GetRatebase(GetRatebaseCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolInterestRateStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxExcessUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxExcessUsageRatio(decoded));
            }
            if let Ok(decoded) = <OptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OptimalUsageRatio(decoded));
            }
            if let Ok(decoded) = <CalculateInterestRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateInterestRates(decoded));
            }
            if let Ok(decoded) = <GetOptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOptimalUsageRatio(decoded));
            }
            if let Ok(decoded) = <GetRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRateSlope1(decoded));
            }
            if let Ok(decoded) = <GetRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRateSlope2(decoded));
            }
            if let Ok(decoded) = <GetRatebaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRatebase(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxExcessUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInterestRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRatebase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoolInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxExcessUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimalUsageRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateInterestRates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOptimalUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRateSlope1(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRateSlope2(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRatebase(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxExcessUsageRatioCall>
    for PoolInterestRateStrategyCalls {
        fn from(value: MaxExcessUsageRatioCall) -> Self {
            Self::MaxExcessUsageRatio(value)
        }
    }
    impl ::core::convert::From<OptimalUsageRatioCall> for PoolInterestRateStrategyCalls {
        fn from(value: OptimalUsageRatioCall) -> Self {
            Self::OptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<CalculateInterestRatesCall>
    for PoolInterestRateStrategyCalls {
        fn from(value: CalculateInterestRatesCall) -> Self {
            Self::CalculateInterestRates(value)
        }
    }
    impl ::core::convert::From<GetOptimalUsageRatioCall>
    for PoolInterestRateStrategyCalls {
        fn from(value: GetOptimalUsageRatioCall) -> Self {
            Self::GetOptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<GetRateSlope1Call> for PoolInterestRateStrategyCalls {
        fn from(value: GetRateSlope1Call) -> Self {
            Self::GetRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetRateSlope2Call> for PoolInterestRateStrategyCalls {
        fn from(value: GetRateSlope2Call) -> Self {
            Self::GetRateSlope2(value)
        }
    }
    impl ::core::convert::From<GetRatebaseCall> for PoolInterestRateStrategyCalls {
        fn from(value: GetRatebaseCall) -> Self {
            Self::GetRatebase(value)
        }
    }
    ///Container type for all return fields from the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
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
    pub struct MaxExcessUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
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
    pub struct OptimalUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,address,address))` and selector `0x9dad6199`
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
    pub struct CalculateInterestRatesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getOptimalUsageRatio` function with signature `getOptimalUsageRatio()` and selector `0xa8602e86`
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
    pub struct GetOptimalUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRateSlope1` function with signature `getRateSlope1()` and selector `0x2dd9037b`
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
    pub struct GetRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRateSlope2` function with signature `getRateSlope2()` and selector `0x75005bb0`
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
    pub struct GetRateSlope2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRatebase` function with signature `getRatebase()` and selector `0x87f0409d`
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
    pub struct GetRatebaseReturn(pub ::ethers::core::types::U256);
}
