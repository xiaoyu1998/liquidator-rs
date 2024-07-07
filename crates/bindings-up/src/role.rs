pub use role::*;
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
pub mod role {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CONFIG_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CONFIG_KEEPER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CONTROLLER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("CONTROLLER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FEE_DISTRIBUTION_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FEE_DISTRIBUTION_KEEPER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FEE_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FEE_KEEPER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GOV_TOKEN_CONTROLLER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GOV_TOKEN_CONTROLLER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LIMITED_CONFIG_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LIMITED_CONFIG_KEEPER",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LIQUIDATION_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("LIQUIDATION_KEEPER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL_KEEPER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PRICING_KEEPER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PRICING_KEEPER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ROLE_ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ROLE_ADMIN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ROUTER_PLUGIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ROUTER_PLUGIN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TIMELOCK_ADMIN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TIMELOCK_ADMIN"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TIMELOCK_MULTISIG"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TIMELOCK_MULTISIG"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ROLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x04Ca\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\xD9W`\x005`\xE0\x1C\x80c\xCF\x7F\xB1\xD0\x11a\0\x96W\x80c\xE2\xFFG\xB3\x11a\0pW\x80c\xE2\xFFG\xB3\x14a\x018W\x80c\xE6)\xD4\x8C\x14a\x01@W\x80c\xEE\x0F\xC1!\x14a\x01HW\x80c\xF1<ZK\x14a\x01PW`\0\x80\xFD[\x80c\xCF\x7F\xB1\xD0\x14a\x01 W\x80c\xD3\x91\x01K\x14a\x01(W\x80c\xE0\xFD\xE2\x0C\x14a\x010W`\0\x80\xFD[\x80cDy\xD9{\x14a\0\xDEW\x80cu\xD3\xAD\xBB\x14a\0\xF8W\x80cwO\xB4\xB8\x14a\x01\0W\x80c\x9B\x8BI\xF8\x14a\x01\x08W\x80c\x9E\xCF\xF6\x17\x14a\x01\x10W\x80c\xC6j\xAD\xE1\x14a\x01\x18W[`\0\x80\xFD[a\0\xE6a\x01XV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xE6a\x01\xA6V[a\0\xE6a\x01\xE7V[a\0\xE6a\x02 V[a\0\xE6a\x02QV[a\0\xE6a\x02\x83V[a\0\xE6a\x02\xB4V[a\0\xE6a\x02\xE2V[a\0\xE6a\x03\x10V[a\0\xE6a\x03HV[a\0\xE6a\x03zV[a\0\xE6a\x03\xA9V[a\0\xE6a\x03\xD7V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x11\x90\x82\x01RpTIMELOCK_MULTISIG`x\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x17\x90\x82\x01R\x7FFEE_DISTRIBUTION_KEEPER\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x15\x90\x82\x01Rt&$\xA6\xA4\xAA\"\xA2/\xA1\xA7\xA7#$\xA3\xAF\xA5\xA2\xA2\xA8\"\xA9`Y\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0E\x90\x82\x01Rm()$\xA1\xA4\xA7#\xAF\xA5\xA2\xA2\xA8\"\xA9`\x91\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl!\xA7\xA7#$\xA3\xAF\xA5\xA2\xA2\xA8\"\xA9`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri#\"\xA2\xAF\xA5\xA2\xA2\xA8\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x14\x90\x82\x01Rs#\xA7\xAB/\xAA'\xA5\xA2\xA7/\xA1\xA7\xA7*)'\xA6&\"\xA9`a\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0E\x90\x82\x01Rm*$\xA6\xA2\xA6'\xA1\xA5\xAF\xA0\xA2&\xA4\xA7`\x91\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0B\x90\x82\x01Rj('\xA7\xA6/\xA5\xA2\xA2\xA8\"\xA9`\xA9\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \x8C\xD4i\xDD\xA5\xC1\x9F`\x97[c\x8F)\x92\xDC\xC7\x1DU\xA8DB}\xD5\xBFki\xB8'\xC4\xC88\xAFdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ROLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0\xD9W`\x005`\xE0\x1C\x80c\xCF\x7F\xB1\xD0\x11a\0\x96W\x80c\xE2\xFFG\xB3\x11a\0pW\x80c\xE2\xFFG\xB3\x14a\x018W\x80c\xE6)\xD4\x8C\x14a\x01@W\x80c\xEE\x0F\xC1!\x14a\x01HW\x80c\xF1<ZK\x14a\x01PW`\0\x80\xFD[\x80c\xCF\x7F\xB1\xD0\x14a\x01 W\x80c\xD3\x91\x01K\x14a\x01(W\x80c\xE0\xFD\xE2\x0C\x14a\x010W`\0\x80\xFD[\x80cDy\xD9{\x14a\0\xDEW\x80cu\xD3\xAD\xBB\x14a\0\xF8W\x80cwO\xB4\xB8\x14a\x01\0W\x80c\x9B\x8BI\xF8\x14a\x01\x08W\x80c\x9E\xCF\xF6\x17\x14a\x01\x10W\x80c\xC6j\xAD\xE1\x14a\x01\x18W[`\0\x80\xFD[a\0\xE6a\x01XV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xE6a\x01\xA6V[a\0\xE6a\x01\xE7V[a\0\xE6a\x02 V[a\0\xE6a\x02QV[a\0\xE6a\x02\x83V[a\0\xE6a\x02\xB4V[a\0\xE6a\x02\xE2V[a\0\xE6a\x03\x10V[a\0\xE6a\x03HV[a\0\xE6a\x03zV[a\0\xE6a\x03\xA9V[a\0\xE6a\x03\xD7V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x11\x90\x82\x01RpTIMELOCK_MULTISIG`x\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x17\x90\x82\x01R\x7FFEE_DISTRIBUTION_KEEPER\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x15\x90\x82\x01Rt&$\xA6\xA4\xAA\"\xA2/\xA1\xA7\xA7#$\xA3\xAF\xA5\xA2\xA2\xA8\"\xA9`Y\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0E\x90\x82\x01Rm()$\xA1\xA4\xA7#\xAF\xA5\xA2\xA2\xA8\"\xA9`\x91\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl!\xA7\xA7#$\xA3\xAF\xA5\xA2\xA2\xA8\"\xA9`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri#\"\xA2\xAF\xA5\xA2\xA2\xA8\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x14\x90\x82\x01Rs#\xA7\xAB/\xAA'\xA5\xA2\xA7/\xA1\xA7\xA7*)'\xA6&\"\xA9`a\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0E\x90\x82\x01Rm*$\xA6\xA2\xA6'\xA1\xA5\xAF\xA0\xA2&\xA4\xA7`\x91\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x0B\x90\x82\x01Rj('\xA7\xA6/\xA5\xA2\xA2\xA8\"\xA9`\xA9\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x01\x8D\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \x8C\xD4i\xDD\xA5\xC1\x9F`\x97[c\x8F)\x92\xDC\xC7\x1DU\xA8DB}\xD5\xBFki\xB8'\xC4\xC88\xAFdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ROLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Role<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Role<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Role<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Role<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Role<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Role)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Role<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROLE_ABI.clone(),
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
                ROLE_ABI.clone(),
                ROLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CONFIG_KEEPER` (0xc66aade1) function
        pub fn config_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([198, 106, 173, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CONTROLLER` (0xee0fc121) function
        pub fn controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 15, 193, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FEE_DISTRIBUTION_KEEPER` (0x75d3adbb) function
        pub fn fee_distribution_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([117, 211, 173, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FEE_KEEPER` (0xcf7fb1d0) function
        pub fn fee_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([207, 127, 177, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `GOV_TOKEN_CONTROLLER` (0xe0fde20c) function
        pub fn gov_token_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([224, 253, 226, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIMITED_CONFIG_KEEPER` (0x774fb4b8) function
        pub fn limited_config_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([119, 79, 180, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `LIQUIDATION_KEEPER` (0xf13c5a4b) function
        pub fn liquidation_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 60, 90, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_KEEPER` (0xe629d48c) function
        pub fn pool_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 41, 212, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PRICING_KEEPER` (0x9ecff617) function
        pub fn pricing_keeper(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([158, 207, 246, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ROLE_ADMIN` (0xd391014b) function
        pub fn role_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([211, 145, 1, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ROUTER_PLUGIN` (0x9b8b49f8) function
        pub fn router_plugin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 139, 73, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TIMELOCK_ADMIN` (0xe2ff47b3) function
        pub fn timelock_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([226, 255, 71, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TIMELOCK_MULTISIG` (0x4479d97b) function
        pub fn timelock_multisig(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([68, 121, 217, 123], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Role<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `CONFIG_KEEPER` function with signature `CONFIG_KEEPER()` and selector `0xc66aade1`
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
    #[ethcall(name = "CONFIG_KEEPER", abi = "CONFIG_KEEPER()")]
    pub struct ConfigKeeperCall;
    ///Container type for all input parameters for the `CONTROLLER` function with signature `CONTROLLER()` and selector `0xee0fc121`
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
    #[ethcall(name = "CONTROLLER", abi = "CONTROLLER()")]
    pub struct ControllerCall;
    ///Container type for all input parameters for the `FEE_DISTRIBUTION_KEEPER` function with signature `FEE_DISTRIBUTION_KEEPER()` and selector `0x75d3adbb`
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
    #[ethcall(name = "FEE_DISTRIBUTION_KEEPER", abi = "FEE_DISTRIBUTION_KEEPER()")]
    pub struct FeeDistributionKeeperCall;
    ///Container type for all input parameters for the `FEE_KEEPER` function with signature `FEE_KEEPER()` and selector `0xcf7fb1d0`
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
    #[ethcall(name = "FEE_KEEPER", abi = "FEE_KEEPER()")]
    pub struct FeeKeeperCall;
    ///Container type for all input parameters for the `GOV_TOKEN_CONTROLLER` function with signature `GOV_TOKEN_CONTROLLER()` and selector `0xe0fde20c`
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
    #[ethcall(name = "GOV_TOKEN_CONTROLLER", abi = "GOV_TOKEN_CONTROLLER()")]
    pub struct GovTokenControllerCall;
    ///Container type for all input parameters for the `LIMITED_CONFIG_KEEPER` function with signature `LIMITED_CONFIG_KEEPER()` and selector `0x774fb4b8`
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
    #[ethcall(name = "LIMITED_CONFIG_KEEPER", abi = "LIMITED_CONFIG_KEEPER()")]
    pub struct LimitedConfigKeeperCall;
    ///Container type for all input parameters for the `LIQUIDATION_KEEPER` function with signature `LIQUIDATION_KEEPER()` and selector `0xf13c5a4b`
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
    #[ethcall(name = "LIQUIDATION_KEEPER", abi = "LIQUIDATION_KEEPER()")]
    pub struct LiquidationKeeperCall;
    ///Container type for all input parameters for the `POOL_KEEPER` function with signature `POOL_KEEPER()` and selector `0xe629d48c`
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
    #[ethcall(name = "POOL_KEEPER", abi = "POOL_KEEPER()")]
    pub struct PoolKeeperCall;
    ///Container type for all input parameters for the `PRICING_KEEPER` function with signature `PRICING_KEEPER()` and selector `0x9ecff617`
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
    #[ethcall(name = "PRICING_KEEPER", abi = "PRICING_KEEPER()")]
    pub struct PricingKeeperCall;
    ///Container type for all input parameters for the `ROLE_ADMIN` function with signature `ROLE_ADMIN()` and selector `0xd391014b`
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
    #[ethcall(name = "ROLE_ADMIN", abi = "ROLE_ADMIN()")]
    pub struct RoleAdminCall;
    ///Container type for all input parameters for the `ROUTER_PLUGIN` function with signature `ROUTER_PLUGIN()` and selector `0x9b8b49f8`
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
    #[ethcall(name = "ROUTER_PLUGIN", abi = "ROUTER_PLUGIN()")]
    pub struct RouterPluginCall;
    ///Container type for all input parameters for the `TIMELOCK_ADMIN` function with signature `TIMELOCK_ADMIN()` and selector `0xe2ff47b3`
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
    #[ethcall(name = "TIMELOCK_ADMIN", abi = "TIMELOCK_ADMIN()")]
    pub struct TimelockAdminCall;
    ///Container type for all input parameters for the `TIMELOCK_MULTISIG` function with signature `TIMELOCK_MULTISIG()` and selector `0x4479d97b`
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
    #[ethcall(name = "TIMELOCK_MULTISIG", abi = "TIMELOCK_MULTISIG()")]
    pub struct TimelockMultisigCall;
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
    pub enum RoleCalls {
        ConfigKeeper(ConfigKeeperCall),
        Controller(ControllerCall),
        FeeDistributionKeeper(FeeDistributionKeeperCall),
        FeeKeeper(FeeKeeperCall),
        GovTokenController(GovTokenControllerCall),
        LimitedConfigKeeper(LimitedConfigKeeperCall),
        LiquidationKeeper(LiquidationKeeperCall),
        PoolKeeper(PoolKeeperCall),
        PricingKeeper(PricingKeeperCall),
        RoleAdmin(RoleAdminCall),
        RouterPlugin(RouterPluginCall),
        TimelockAdmin(TimelockAdminCall),
        TimelockMultisig(TimelockMultisigCall),
    }
    impl ::ethers::core::abi::AbiDecode for RoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ConfigKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConfigKeeper(decoded));
            }
            if let Ok(decoded) = <ControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Controller(decoded));
            }
            if let Ok(decoded) = <FeeDistributionKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeDistributionKeeper(decoded));
            }
            if let Ok(decoded) = <FeeKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeKeeper(decoded));
            }
            if let Ok(decoded) = <GovTokenControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GovTokenController(decoded));
            }
            if let Ok(decoded) = <LimitedConfigKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LimitedConfigKeeper(decoded));
            }
            if let Ok(decoded) = <LiquidationKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidationKeeper(decoded));
            }
            if let Ok(decoded) = <PoolKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolKeeper(decoded));
            }
            if let Ok(decoded) = <PricingKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PricingKeeper(decoded));
            }
            if let Ok(decoded) = <RoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleAdmin(decoded));
            }
            if let Ok(decoded) = <RouterPluginCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RouterPlugin(decoded));
            }
            if let Ok(decoded) = <TimelockAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TimelockAdmin(decoded));
            }
            if let Ok(decoded) = <TimelockMultisigCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TimelockMultisig(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConfigKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Controller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeDistributionKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GovTokenController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LimitedConfigKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PricingKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RouterPlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimelockAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimelockMultisig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfigKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::Controller(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeDistributionKeeper(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::GovTokenController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LimitedConfigKeeper(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::PricingKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RouterPlugin(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimelockAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimelockMultisig(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConfigKeeperCall> for RoleCalls {
        fn from(value: ConfigKeeperCall) -> Self {
            Self::ConfigKeeper(value)
        }
    }
    impl ::core::convert::From<ControllerCall> for RoleCalls {
        fn from(value: ControllerCall) -> Self {
            Self::Controller(value)
        }
    }
    impl ::core::convert::From<FeeDistributionKeeperCall> for RoleCalls {
        fn from(value: FeeDistributionKeeperCall) -> Self {
            Self::FeeDistributionKeeper(value)
        }
    }
    impl ::core::convert::From<FeeKeeperCall> for RoleCalls {
        fn from(value: FeeKeeperCall) -> Self {
            Self::FeeKeeper(value)
        }
    }
    impl ::core::convert::From<GovTokenControllerCall> for RoleCalls {
        fn from(value: GovTokenControllerCall) -> Self {
            Self::GovTokenController(value)
        }
    }
    impl ::core::convert::From<LimitedConfigKeeperCall> for RoleCalls {
        fn from(value: LimitedConfigKeeperCall) -> Self {
            Self::LimitedConfigKeeper(value)
        }
    }
    impl ::core::convert::From<LiquidationKeeperCall> for RoleCalls {
        fn from(value: LiquidationKeeperCall) -> Self {
            Self::LiquidationKeeper(value)
        }
    }
    impl ::core::convert::From<PoolKeeperCall> for RoleCalls {
        fn from(value: PoolKeeperCall) -> Self {
            Self::PoolKeeper(value)
        }
    }
    impl ::core::convert::From<PricingKeeperCall> for RoleCalls {
        fn from(value: PricingKeeperCall) -> Self {
            Self::PricingKeeper(value)
        }
    }
    impl ::core::convert::From<RoleAdminCall> for RoleCalls {
        fn from(value: RoleAdminCall) -> Self {
            Self::RoleAdmin(value)
        }
    }
    impl ::core::convert::From<RouterPluginCall> for RoleCalls {
        fn from(value: RouterPluginCall) -> Self {
            Self::RouterPlugin(value)
        }
    }
    impl ::core::convert::From<TimelockAdminCall> for RoleCalls {
        fn from(value: TimelockAdminCall) -> Self {
            Self::TimelockAdmin(value)
        }
    }
    impl ::core::convert::From<TimelockMultisigCall> for RoleCalls {
        fn from(value: TimelockMultisigCall) -> Self {
            Self::TimelockMultisig(value)
        }
    }
    ///Container type for all return fields from the `CONFIG_KEEPER` function with signature `CONFIG_KEEPER()` and selector `0xc66aade1`
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
    pub struct ConfigKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CONTROLLER` function with signature `CONTROLLER()` and selector `0xee0fc121`
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
    pub struct ControllerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FEE_DISTRIBUTION_KEEPER` function with signature `FEE_DISTRIBUTION_KEEPER()` and selector `0x75d3adbb`
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
    pub struct FeeDistributionKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FEE_KEEPER` function with signature `FEE_KEEPER()` and selector `0xcf7fb1d0`
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
    pub struct FeeKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `GOV_TOKEN_CONTROLLER` function with signature `GOV_TOKEN_CONTROLLER()` and selector `0xe0fde20c`
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
    pub struct GovTokenControllerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `LIMITED_CONFIG_KEEPER` function with signature `LIMITED_CONFIG_KEEPER()` and selector `0x774fb4b8`
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
    pub struct LimitedConfigKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `LIQUIDATION_KEEPER` function with signature `LIQUIDATION_KEEPER()` and selector `0xf13c5a4b`
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
    pub struct LiquidationKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POOL_KEEPER` function with signature `POOL_KEEPER()` and selector `0xe629d48c`
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
    pub struct PoolKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PRICING_KEEPER` function with signature `PRICING_KEEPER()` and selector `0x9ecff617`
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
    pub struct PricingKeeperReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ROLE_ADMIN` function with signature `ROLE_ADMIN()` and selector `0xd391014b`
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
    pub struct RoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ROUTER_PLUGIN` function with signature `ROUTER_PLUGIN()` and selector `0x9b8b49f8`
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
    pub struct RouterPluginReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TIMELOCK_ADMIN` function with signature `TIMELOCK_ADMIN()` and selector `0xe2ff47b3`
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
    pub struct TimelockAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TIMELOCK_MULTISIG` function with signature `TIMELOCK_MULTISIG()` and selector `0x4479d97b`
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
    pub struct TimelockMultisigReturn(pub [u8; 32]);
}
