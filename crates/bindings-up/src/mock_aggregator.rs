pub use mock_aggregator::*;
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
pub mod mock_aggregator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_decimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initialAnswer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("int256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("description"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("description"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAnswer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAnswer"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoundData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoundData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("answer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startedAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updatedAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("answeredInRound"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTimestamp"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("latestAnswer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestAnswer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestRound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestRound"),
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
                    ::std::borrow::ToOwned::to_owned("latestRoundData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestRoundData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("answer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startedAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updatedAt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("answeredInRound"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("setAnswer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAnswer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_answer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                    ::std::borrow::ToOwned::to_owned("updateRoundData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateRoundData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint80"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_answer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_startedAt"),
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
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AnswerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AnswerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("current"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("updatedAt"),
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
                    ::std::borrow::ToOwned::to_owned("NewRound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewRound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("roundId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("startedBy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("startedAt"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKAGGREGATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08\xE18\x03\x80a\x08\xE1\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x8DV[a\083a\0`V[`\0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B`\xFF\x85\x16\x02\x17\x90Ua\0Y\x81a\0\xB0V[PPa\x01\xE8V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01\x81\x90UB`\x02U`\x03\x80T\x90`\0a\0\xC9\x83a\x01\xC1V[\x90\x91UPP`\x03\x80T`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x90U\x83T\x83R`\x05\x82R\x80\x83 B\x90\x81\x90U\x84T\x84R`\x06\x83R\x92\x81\x90 \x83\x90U\x92T\x92Q\x91\x82R\x83\x91\x7F\x05Y\x88O\xD3\xA4`\xDB0s\xB7\xFC\x89l\xC7y\x86\xF1n7\x82\x10\xDE\xD41\x86\x17[\xF6F\xFC_\x91\x01`@Q\x80\x91\x03\x90\xA3`\0T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x03T\x7F\x01\t\xFCoU\xCF@h\x9F\x02\xFB\xAA\xD7\xAF\x7F\xE7\xBB\xAC\x8A=!\x86`\n\xFC}>\x10\xCA\xC6\x02qB`@Qa\x01\x82\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xA0W`\0\x80\xFD[\x82Q`\xFF\x81\x16\x81\x14a\x01\xB1W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0`\x01\x82\x01a\x01\xE1WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[a\x06\xEA\x80a\x01\xF7`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\x82\x05\xBFj\x11a\0\x97W\x80c\xB5\xABX\xDC\x11a\0fW\x80c\xB5\xABX\xDC\x14a\x02^W\x80c\xB63b\x0C\x14a\x02~W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9EW\x80c\xFE\xAF\x96\x8C\x14a\x02\xB1W`\0\x80\xFD[\x80c\x82\x05\xBFj\x14a\x01\xA9W\x80c\x8D\xA5\xCB[\x14a\x01\xB2W\x80c\x99!<\xD8\x14a\x01\xCDW\x80c\x9Ao\xC8\xF5\x14a\x01\xE0W`\0\x80\xFD[\x80cT\xFDMP\x11a\0\xD3W\x80cT\xFDMP\x14a\x01QW\x80cf\x8A\x0F\x02\x14a\x01YW\x80cqP\x18\xA6\x14a\x01bW\x80cr\x84\xE4\x16\x14a\x01jW`\0\x80\xFD[\x80c1<\xE5g\x14a\0\xFAW\x80cJ\xA2\x01\x1F\x14a\x01%W\x80cP\xD2[\xCD\x14a\x01:W[`\0\x80\xFD[`\0Ta\x01\x0E\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a\x05\xA2V[a\x02\xDCV[\0[a\x01C`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\x1CV[a\x01C`\0\x81V[a\x01C`\x03T\x81V[a\x018a\x03aV[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F/mock/MockAggregator.sol\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x01\x1C\x91\x90a\x05\xDBV[a\x01C`\x02T\x81V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x018a\x01\xDB6`\x04a\x06)V[a\x03uV[a\x02'a\x01\xEE6`\x04a\x06BV[i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 T`\x06\x83R\x81\x84 T`\x05\x90\x93R\x92 T\x92\x93\x91\x92\x90\x91\x84\x90V[`@\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81R` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x01\x1CV[a\x01Ca\x02l6`\x04a\x06)V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[a\x01Ca\x02\x8C6`\x04a\x06)V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x018a\x02\xAC6`\x04a\x06dV[a\x03\x81V[`\x03T`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 T`\x06\x83R\x81\x84 T`\x05\x90\x93R\x92 T\x83a\x02'V[i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x03\x81\x81U`\x01\x85\x90U`\x02\x84\x90U`\0\x82\x81R`\x04` \x90\x81R`@\x80\x83 \x88\x90U\x83T\x83R`\x05\x82R\x80\x83 \x87\x90U\x92T\x82R`\x06\x81R\x90\x82\x90 \x84\x90U\x90QB\x81R\x85\x91\x7F\x05Y\x88O\xD3\xA4`\xDB0s\xB7\xFC\x89l\xC7y\x86\xF1n7\x82\x10\xDE\xD41\x86\x17[\xF6F\xFC_\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\x03ia\x03\xFCV[a\x03s`\0a\x04VV[V[a\x03~\x81a\x04\xA6V[PV[a\x03\x89a\x03\xFCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03~\x81a\x04VV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\xEAV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01\x81\x90UB`\x02U`\x03\x80T\x90`\0a\x04\xBF\x83a\x06\x8DV[\x90\x91UPP`\x03\x80T`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x90U\x83T\x83R`\x05\x82R\x80\x83 B\x90\x81\x90U\x84T\x84R`\x06\x83R\x92\x81\x90 \x83\x90U\x92T\x92Q\x91\x82R\x83\x91\x7F\x05Y\x88O\xD3\xA4`\xDB0s\xB7\xFC\x89l\xC7y\x86\xF1n7\x82\x10\xDE\xD41\x86\x17[\xF6F\xFC_\x91\x01`@Q\x80\x91\x03\x90\xA3`\0T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x03T\x7F\x01\t\xFCoU\xCF@h\x9F\x02\xFB\xAA\xD7\xAF\x7F\xE7\xBB\xAC\x8A=!\x86`\n\xFC}>\x10\xCA\xC6\x02qB`@Qa\x05x\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PV[\x805i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x9DW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xB8W`\0\x80\xFD[a\x05\xC1\x85a\x05\x83V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x06\tW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x05\xECV[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06;W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06TW`\0\x80\xFD[a\x06]\x82a\x05\x83V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06vW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06]W`\0\x80\xFD[`\0`\x01\x82\x01a\x06\xADWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xC5\x0C}\x86\xE2d\xDB\x81\x8E2:@\xD06370\x93\t\x15k\xBAbw\x9EF\xF2k\xAFdeEdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MOCKAGGREGATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\x82\x05\xBFj\x11a\0\x97W\x80c\xB5\xABX\xDC\x11a\0fW\x80c\xB5\xABX\xDC\x14a\x02^W\x80c\xB63b\x0C\x14a\x02~W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9EW\x80c\xFE\xAF\x96\x8C\x14a\x02\xB1W`\0\x80\xFD[\x80c\x82\x05\xBFj\x14a\x01\xA9W\x80c\x8D\xA5\xCB[\x14a\x01\xB2W\x80c\x99!<\xD8\x14a\x01\xCDW\x80c\x9Ao\xC8\xF5\x14a\x01\xE0W`\0\x80\xFD[\x80cT\xFDMP\x11a\0\xD3W\x80cT\xFDMP\x14a\x01QW\x80cf\x8A\x0F\x02\x14a\x01YW\x80cqP\x18\xA6\x14a\x01bW\x80cr\x84\xE4\x16\x14a\x01jW`\0\x80\xFD[\x80c1<\xE5g\x14a\0\xFAW\x80cJ\xA2\x01\x1F\x14a\x01%W\x80cP\xD2[\xCD\x14a\x01:W[`\0\x80\xFD[`\0Ta\x01\x0E\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a\x05\xA2V[a\x02\xDCV[\0[a\x01C`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\x1CV[a\x01C`\0\x81V[a\x01C`\x03T\x81V[a\x018a\x03aV[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F/mock/MockAggregator.sol\0\0\0\0\0\0\0\0` \x82\x01R\x90Qa\x01\x1C\x91\x90a\x05\xDBV[a\x01C`\x02T\x81V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x018a\x01\xDB6`\x04a\x06)V[a\x03uV[a\x02'a\x01\xEE6`\x04a\x06BV[i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 T`\x06\x83R\x81\x84 T`\x05\x90\x93R\x92 T\x92\x93\x91\x92\x90\x91\x84\x90V[`@\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x96\x87\x16\x81R` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R\x90\x91\x16`\x80\x82\x01R`\xA0\x01a\x01\x1CV[a\x01Ca\x02l6`\x04a\x06)V[`\x04` R`\0\x90\x81R`@\x90 T\x81V[a\x01Ca\x02\x8C6`\x04a\x06)V[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x018a\x02\xAC6`\x04a\x06dV[a\x03\x81V[`\x03T`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 T`\x06\x83R\x81\x84 T`\x05\x90\x93R\x92 T\x83a\x02'V[i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x03\x81\x81U`\x01\x85\x90U`\x02\x84\x90U`\0\x82\x81R`\x04` \x90\x81R`@\x80\x83 \x88\x90U\x83T\x83R`\x05\x82R\x80\x83 \x87\x90U\x92T\x82R`\x06\x81R\x90\x82\x90 \x84\x90U\x90QB\x81R\x85\x91\x7F\x05Y\x88O\xD3\xA4`\xDB0s\xB7\xFC\x89l\xC7y\x86\xF1n7\x82\x10\xDE\xD41\x86\x17[\xF6F\xFC_\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[a\x03ia\x03\xFCV[a\x03s`\0a\x04VV[V[a\x03~\x81a\x04\xA6V[PV[a\x03\x89a\x03\xFCV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03~\x81a\x04VV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\xEAV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01\x81\x90UB`\x02U`\x03\x80T\x90`\0a\x04\xBF\x83a\x06\x8DV[\x90\x91UPP`\x03\x80T`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x90U\x83T\x83R`\x05\x82R\x80\x83 B\x90\x81\x90U\x84T\x84R`\x06\x83R\x92\x81\x90 \x83\x90U\x92T\x92Q\x91\x82R\x83\x91\x7F\x05Y\x88O\xD3\xA4`\xDB0s\xB7\xFC\x89l\xC7y\x86\xF1n7\x82\x10\xDE\xD41\x86\x17[\xF6F\xFC_\x91\x01`@Q\x80\x91\x03\x90\xA3`\0T`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x03T\x7F\x01\t\xFCoU\xCF@h\x9F\x02\xFB\xAA\xD7\xAF\x7F\xE7\xBB\xAC\x8A=!\x86`\n\xFC}>\x10\xCA\xC6\x02qB`@Qa\x05x\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PV[\x805i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x9DW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xB8W`\0\x80\xFD[a\x05\xC1\x85a\x05\x83V[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x06\tW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x05\xECV[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06;W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06TW`\0\x80\xFD[a\x06]\x82a\x05\x83V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06vW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06]W`\0\x80\xFD[`\0`\x01\x82\x01a\x06\xADWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xC5\x0C}\x86\xE2d\xDB\x81\x8E2:@\xD06370\x93\t\x15k\xBAbw\x9EF\xF2k\xAFdeEdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKAGGREGATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockAggregator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockAggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockAggregator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockAggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockAggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockAggregator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockAggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKAGGREGATOR_ABI.clone(),
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
                MOCKAGGREGATOR_ABI.clone(),
                MOCKAGGREGATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAnswer` (0xb5ab58dc) function
        pub fn get_answer(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([181, 171, 88, 220], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundData` (0x9a6fc8f5) function
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimestamp` (0xb633620c) function
        pub fn get_timestamp(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestAnswer` (0x50d25bcd) function
        pub fn latest_answer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRound` (0x668a0f02) function
        pub fn latest_round(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRoundData` (0xfeaf968c) function
        pub fn latest_round_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestTimestamp` (0x8205bf6a) function
        pub fn latest_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
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
        ///Calls the contract's `setAnswer` (0x99213cd8) function
        pub fn set_answer(
            &self,
            answer: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 33, 60, 216], answer)
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
        ///Calls the contract's `updateRoundData` (0x4aa2011f) function
        pub fn update_round_data(
            &self,
            round_id: u128,
            answer: ::ethers::core::types::I256,
            timestamp: ::ethers::core::types::U256,
            started_at: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 162, 1, 31], (round_id, answer, timestamp, started_at))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AnswerUpdated` event
        pub fn answer_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AnswerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewRound` event
        pub fn new_round_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewRoundFilter,
        > {
            self.0.event()
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
            MockAggregatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockAggregator<M> {
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: ::ethers::core::types::I256,
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ::ethers::core::types::Address,
        pub started_at: ::ethers::core::types::U256,
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
    pub enum MockAggregatorEvents {
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        NewRoundFilter(NewRoundFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockAggregatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(MockAggregatorEvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(MockAggregatorEvents::NewRoundFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MockAggregatorEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockAggregatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AnswerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewRoundFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AnswerUpdatedFilter> for MockAggregatorEvents {
        fn from(value: AnswerUpdatedFilter) -> Self {
            Self::AnswerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NewRoundFilter> for MockAggregatorEvents {
        fn from(value: NewRoundFilter) -> Self {
            Self::NewRoundFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MockAggregatorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
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
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    ///Container type for all input parameters for the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    ///Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    ///Container type for all input parameters for the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
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
    ///Container type for all input parameters for the `setAnswer` function with signature `setAnswer(int256)` and selector `0x99213cd8`
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
    #[ethcall(name = "setAnswer", abi = "setAnswer(int256)")]
    pub struct SetAnswerCall {
        pub answer: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `updateRoundData` function with signature `updateRoundData(uint80,int256,uint256,uint256)` and selector `0x4aa2011f`
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
        name = "updateRoundData",
        abi = "updateRoundData(uint80,int256,uint256,uint256)"
    )]
    pub struct UpdateRoundDataCall {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub timestamp: ::ethers::core::types::U256,
        pub started_at: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
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
    pub enum MockAggregatorCalls {
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetAnswer(GetAnswerCall),
        GetRoundData(GetRoundDataCall),
        GetTimestamp(GetTimestampCall),
        LatestAnswer(LatestAnswerCall),
        LatestRound(LatestRoundCall),
        LatestRoundData(LatestRoundDataCall),
        LatestTimestamp(LatestTimestampCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetAnswer(SetAnswerCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateRoundData(UpdateRoundDataCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockAggregatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded) = <GetAnswerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAnswer(decoded));
            }
            if let Ok(decoded) = <GetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoundData(decoded));
            }
            if let Ok(decoded) = <GetTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTimestamp(decoded));
            }
            if let Ok(decoded) = <LatestAnswerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestAnswer(decoded));
            }
            if let Ok(decoded) = <LatestRoundCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestRound(decoded));
            }
            if let Ok(decoded) = <LatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestRoundData(decoded));
            }
            if let Ok(decoded) = <LatestTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestTimestamp(decoded));
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
            if let Ok(decoded) = <SetAnswerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAnswer(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateRoundData(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockAggregatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Description(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAnswer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestAnswer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAnswer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockAggregatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockAggregatorCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for MockAggregatorCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<GetAnswerCall> for MockAggregatorCalls {
        fn from(value: GetAnswerCall) -> Self {
            Self::GetAnswer(value)
        }
    }
    impl ::core::convert::From<GetRoundDataCall> for MockAggregatorCalls {
        fn from(value: GetRoundDataCall) -> Self {
            Self::GetRoundData(value)
        }
    }
    impl ::core::convert::From<GetTimestampCall> for MockAggregatorCalls {
        fn from(value: GetTimestampCall) -> Self {
            Self::GetTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestAnswerCall> for MockAggregatorCalls {
        fn from(value: LatestAnswerCall) -> Self {
            Self::LatestAnswer(value)
        }
    }
    impl ::core::convert::From<LatestRoundCall> for MockAggregatorCalls {
        fn from(value: LatestRoundCall) -> Self {
            Self::LatestRound(value)
        }
    }
    impl ::core::convert::From<LatestRoundDataCall> for MockAggregatorCalls {
        fn from(value: LatestRoundDataCall) -> Self {
            Self::LatestRoundData(value)
        }
    }
    impl ::core::convert::From<LatestTimestampCall> for MockAggregatorCalls {
        fn from(value: LatestTimestampCall) -> Self {
            Self::LatestTimestamp(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockAggregatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MockAggregatorCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetAnswerCall> for MockAggregatorCalls {
        fn from(value: SetAnswerCall) -> Self {
            Self::SetAnswer(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockAggregatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateRoundDataCall> for MockAggregatorCalls {
        fn from(value: UpdateRoundDataCall) -> Self {
            Self::UpdateRoundData(value)
        }
    }
    impl ::core::convert::From<VersionCall> for MockAggregatorCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
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
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    pub struct GetAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    pub struct GetTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    pub struct LatestAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    pub struct LatestRoundReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    pub struct LatestTimestampReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}
