pub use multicall_3::*;
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
pub mod multicall_3 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("aggregate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aggregate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Multicall3.Call[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("aggregate3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aggregate3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Call3[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("aggregate3Value"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aggregate3Value"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Call3Value[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("blockAndAggregate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blockAndAggregate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Multicall3.Call[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBasefee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBasefee"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("basefee"),
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
                    ::std::borrow::ToOwned::to_owned("getBlockHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBlockHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
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
                    ::std::borrow::ToOwned::to_owned("getBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBlockNumber"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainid"),
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
                    ::std::borrow::ToOwned::to_owned("getCurrentBlockCoinbase"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentBlockCoinbase",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("coinbase"),
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
                    ::std::borrow::ToOwned::to_owned("getCurrentBlockGasLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentBlockGasLimit",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gaslimit"),
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
                    ::std::borrow::ToOwned::to_owned("getCurrentBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentBlockTimestamp",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
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
                    ::std::borrow::ToOwned::to_owned("getEthBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEthBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                    ::std::borrow::ToOwned::to_owned("getLastBlockHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLastBlockHash"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
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
                    ::std::borrow::ToOwned::to_owned("tryAggregate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tryAggregate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requireSuccess"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Multicall3.Call[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tryBlockAndAggregate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tryBlockAndAggregate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requireSuccess"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calls"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Multicall3.Call[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Multicall3.Result[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
    pub static MULTICALL3_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x0C\xBA\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xE8W`\x005`\xE0\x1C\x80cB\xCB\xB1\\\x11a\0\x8AW\x80c\xA8\xB0WN\x11a\0YW\x80c\xA8\xB0WN\x14a\x02\x11W\x80c\xBC\xE3\x8B\xD7\x14a\x02,W\x80c\xC3\x07\x7F\xA9\x14a\x02?W\x80c\xEE\x82\xAC^\x14a\x02RW`\0\x80\xFD[\x80cB\xCB\xB1\\\x14a\x01\xB0W\x80cM#\x01\xCC\x14a\x01\xC3W\x80c\x82\xADV\xCB\x14a\x01\xEBW\x80c\x86\xD5\x16\xE8\x14a\x01\xFEW`\0\x80\xFD[\x80c'\xE8mn\x11a\0\xC6W\x80c'\xE8mn\x14a\x01PW\x80c4\x08\xE4p\x14a\x01hW\x80c9\x95B\xE9\x14a\x01{W\x80c>d\xA6\x96\x14a\x01\x9DW`\0\x80\xFD[\x80c\x0F(\xC9}\x14a\0\xEDW\x80c\x17M\xEAq\x14a\x01\x0FW\x80c%-\xBAB\x14a\x01/W[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[PB[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\"a\x01\x1D6`\x04a\tnV[a\x02qV[`@Qa\x01\x06\x91\x90a\nkV[a\x01Ba\x01=6`\x04a\tnV[a\x04_V[`@Qa\x01\x06\x92\x91\x90a\n\x85V[4\x80\x15a\x01\\W`\0\x80\xFD[PC`\0\x19\x01@a\0\xFCV[4\x80\x15a\x01tW`\0\x80\xFD[PFa\0\xFCV[a\x01\x8Ea\x01\x896`\x04a\n\xF1V[a\x05\xD3V[`@Qa\x01\x06\x93\x92\x91\x90a\x0BKV[4\x80\x15a\x01\xA9W`\0\x80\xFD[PHa\0\xFCV[4\x80\x15a\x01\xBCW`\0\x80\xFD[PCa\0\xFCV[4\x80\x15a\x01\xCFW`\0\x80\xFD[Pa\0\xFCa\x01\xDE6`\x04a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x161\x90V[a\x01\"a\x01\xF96`\x04a\tnV[a\x05\xEEV[4\x80\x15a\x02\nW`\0\x80\xFD[PEa\0\xFCV[4\x80\x15a\x02\x1DW`\0\x80\xFD[P`@QA\x81R` \x01a\x01\x06V[a\x01\"a\x02:6`\x04a\n\xF1V[a\x07pV[a\x01\x8Ea\x02M6`\x04a\tnV[a\t\x03V[4\x80\x15a\x02^W`\0\x80\xFD[Pa\0\xFCa\x02m6`\x04a\x0B\x9CV[@\x90V[```\0\x82\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x8FWa\x02\x8Fa\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xD5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\xADW\x90P[P\x92P6`\0[\x82\x81\x10\x15a\x04\x01W`\0\x85\x82\x81Q\x81\x10a\x02\xF8Wa\x02\xF8a\x0B\xCBV[` \x02` \x01\x01Q\x90P\x87\x87\x83\x81\x81\x10a\x03\x14Wa\x03\x14a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x03&\x91\x90a\x0B\xE1V[`@\x81\x015\x95\x86\x01\x95\x90\x93Pa\x03?` \x85\x01\x85a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16\x81a\x03V``\x87\x01\x87a\x0C\x01V[`@Qa\x03d\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xA6V[``\x91P[P` \x80\x85\x01\x91\x90\x91R\x90\x15\x15\x80\x84R\x90\x85\x015\x17a\x03\xF7WbF\x1B\xCD`\xE5\x1B`\0R` `\x04R`\x17`$Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`DR`\x84`\0\xFD[PP`\x01\x01a\x02\xDCV[P\x824\x14a\x04VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FMulticall3: value mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPP\x92\x91PPV[C``\x82\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04|Wa\x04|a\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xAFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x9AW\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x05\xC9W`\0\x87\x87\x83\x81\x81\x10a\x04\xD2Wa\x04\xD2a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x04\xE4\x91\x90a\x0CXV[\x92Pa\x04\xF3` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x05\t` \x85\x01\x85a\x0C\x01V[`@Qa\x05\x17\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05TW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05YV[``\x91P[P\x86\x84\x81Q\x81\x10a\x05lWa\x05la\x0B\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x90P\x80a\x05\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`D\x82\x01R`d\x01a\x04MV[P`\x01\x01a\x04\xB6V[PPP\x92P\x92\x90PV[C\x80@``a\x05\xE3\x86\x86\x86a\x07pV[\x90P\x93P\x93P\x93\x90PV[``\x81\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\nWa\x06\na\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06PW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06(W\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x04VW`\0\x84\x82\x81Q\x81\x10a\x06sWa\x06sa\x0B\xCBV[` \x02` \x01\x01Q\x90P\x86\x86\x83\x81\x81\x10a\x06\x8FWa\x06\x8Fa\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x06\xA1\x91\x90a\x0CnV[\x92Pa\x06\xB0` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x06\xC6`@\x85\x01\x85a\x0C\x01V[`@Qa\x06\xD4\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x16V[``\x91P[P` \x80\x84\x01\x91\x90\x91R\x90\x15\x15\x80\x83R\x90\x84\x015\x17a\x07gWbF\x1B\xCD`\xE5\x1B`\0R` `\x04R`\x17`$Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`DR`d`\0\xFD[P`\x01\x01a\x06WV[``\x81\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x8CWa\x07\x8Ca\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xAAW\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x08\xF9W`\0\x84\x82\x81Q\x81\x10a\x07\xF5Wa\x07\xF5a\x0B\xCBV[` \x02` \x01\x01Q\x90P\x86\x86\x83\x81\x81\x10a\x08\x11Wa\x08\x11a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x08#\x91\x90a\x0CXV[\x92Pa\x082` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x08H` \x85\x01\x85a\x0C\x01V[`@Qa\x08V\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x98V[``\x91P[P` \x83\x01R\x15\x15\x81R\x87\x15a\x08\xF0W\x80Qa\x08\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`D\x82\x01R`d\x01a\x04MV[P`\x01\x01a\x07\xD9V[PPP\x93\x92PPPV[`\0\x80``a\t\x14`\x01\x86\x86a\x05\xD3V[\x91\x97\x90\x96P\x90\x94P\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\t4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tLW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\tgW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x98W`\0\x80\xFD[a\t\xA4\x85\x82\x86\x01a\t\"V[\x90\x96\x90\x95P\x93PPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\t\xD6W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\t\xBAV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a\n_W`\x1F\x19\x85\x84\x03\x01\x88R\x81Q\x80Q\x15\x15\x84R` \x81\x01Q\x90P`@` \x85\x01Ra\nH`@\x85\x01\x82a\t\xB0V[` \x99\x8A\x01\x99\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01a\n\x14V[P\x90\x96\x95PPPPPPV[` \x81R`\0a\n~` \x83\x01\x84a\t\xF6V[\x93\x92PPPV[`\0`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\n\xE4W`_\x19\x87\x86\x03\x01\x84Ra\n\xCF\x85\x83Qa\t\xB0V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\n\xB3V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0B\x06W`\0\x80\xFD[\x835\x80\x15\x15\x81\x14a\x0B\x16W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B2W`\0\x80\xFD[a\x0B>\x86\x82\x87\x01a\t\"V[\x94\x97\x90\x96P\x93\x94PPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0Bj``\x83\x01\x84a\t\xF6V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x85W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n~W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xAEW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0C\x18W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\tgW`\0\x80\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x97\x86\xE95\xF7BY\x067|\xC2\x7F\xFF\xEC\xE40\x0CQ\xDE\xE6\x05\xBC\x1A\x80f\x024A\xB9\x12\x03\xB9dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MULTICALL3_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xE8W`\x005`\xE0\x1C\x80cB\xCB\xB1\\\x11a\0\x8AW\x80c\xA8\xB0WN\x11a\0YW\x80c\xA8\xB0WN\x14a\x02\x11W\x80c\xBC\xE3\x8B\xD7\x14a\x02,W\x80c\xC3\x07\x7F\xA9\x14a\x02?W\x80c\xEE\x82\xAC^\x14a\x02RW`\0\x80\xFD[\x80cB\xCB\xB1\\\x14a\x01\xB0W\x80cM#\x01\xCC\x14a\x01\xC3W\x80c\x82\xADV\xCB\x14a\x01\xEBW\x80c\x86\xD5\x16\xE8\x14a\x01\xFEW`\0\x80\xFD[\x80c'\xE8mn\x11a\0\xC6W\x80c'\xE8mn\x14a\x01PW\x80c4\x08\xE4p\x14a\x01hW\x80c9\x95B\xE9\x14a\x01{W\x80c>d\xA6\x96\x14a\x01\x9DW`\0\x80\xFD[\x80c\x0F(\xC9}\x14a\0\xEDW\x80c\x17M\xEAq\x14a\x01\x0FW\x80c%-\xBAB\x14a\x01/W[`\0\x80\xFD[4\x80\x15a\0\xF9W`\0\x80\xFD[PB[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\"a\x01\x1D6`\x04a\tnV[a\x02qV[`@Qa\x01\x06\x91\x90a\nkV[a\x01Ba\x01=6`\x04a\tnV[a\x04_V[`@Qa\x01\x06\x92\x91\x90a\n\x85V[4\x80\x15a\x01\\W`\0\x80\xFD[PC`\0\x19\x01@a\0\xFCV[4\x80\x15a\x01tW`\0\x80\xFD[PFa\0\xFCV[a\x01\x8Ea\x01\x896`\x04a\n\xF1V[a\x05\xD3V[`@Qa\x01\x06\x93\x92\x91\x90a\x0BKV[4\x80\x15a\x01\xA9W`\0\x80\xFD[PHa\0\xFCV[4\x80\x15a\x01\xBCW`\0\x80\xFD[PCa\0\xFCV[4\x80\x15a\x01\xCFW`\0\x80\xFD[Pa\0\xFCa\x01\xDE6`\x04a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x161\x90V[a\x01\"a\x01\xF96`\x04a\tnV[a\x05\xEEV[4\x80\x15a\x02\nW`\0\x80\xFD[PEa\0\xFCV[4\x80\x15a\x02\x1DW`\0\x80\xFD[P`@QA\x81R` \x01a\x01\x06V[a\x01\"a\x02:6`\x04a\n\xF1V[a\x07pV[a\x01\x8Ea\x02M6`\x04a\tnV[a\t\x03V[4\x80\x15a\x02^W`\0\x80\xFD[Pa\0\xFCa\x02m6`\x04a\x0B\x9CV[@\x90V[```\0\x82\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x8FWa\x02\x8Fa\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xD5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\xADW\x90P[P\x92P6`\0[\x82\x81\x10\x15a\x04\x01W`\0\x85\x82\x81Q\x81\x10a\x02\xF8Wa\x02\xF8a\x0B\xCBV[` \x02` \x01\x01Q\x90P\x87\x87\x83\x81\x81\x10a\x03\x14Wa\x03\x14a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x03&\x91\x90a\x0B\xE1V[`@\x81\x015\x95\x86\x01\x95\x90\x93Pa\x03?` \x85\x01\x85a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16\x81a\x03V``\x87\x01\x87a\x0C\x01V[`@Qa\x03d\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xA6V[``\x91P[P` \x80\x85\x01\x91\x90\x91R\x90\x15\x15\x80\x84R\x90\x85\x015\x17a\x03\xF7WbF\x1B\xCD`\xE5\x1B`\0R` `\x04R`\x17`$Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`DR`\x84`\0\xFD[PP`\x01\x01a\x02\xDCV[P\x824\x14a\x04VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FMulticall3: value mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPP\x92\x91PPV[C``\x82\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04|Wa\x04|a\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xAFW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\x9AW\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x05\xC9W`\0\x87\x87\x83\x81\x81\x10a\x04\xD2Wa\x04\xD2a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x04\xE4\x91\x90a\x0CXV[\x92Pa\x04\xF3` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x05\t` \x85\x01\x85a\x0C\x01V[`@Qa\x05\x17\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x05TW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05YV[``\x91P[P\x86\x84\x81Q\x81\x10a\x05lWa\x05la\x0B\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x90P\x80a\x05\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`D\x82\x01R`d\x01a\x04MV[P`\x01\x01a\x04\xB6V[PPP\x92P\x92\x90PV[C\x80@``a\x05\xE3\x86\x86\x86a\x07pV[\x90P\x93P\x93P\x93\x90PV[``\x81\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\nWa\x06\na\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06PW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06(W\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x04VW`\0\x84\x82\x81Q\x81\x10a\x06sWa\x06sa\x0B\xCBV[` \x02` \x01\x01Q\x90P\x86\x86\x83\x81\x81\x10a\x06\x8FWa\x06\x8Fa\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x06\xA1\x91\x90a\x0CnV[\x92Pa\x06\xB0` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x06\xC6`@\x85\x01\x85a\x0C\x01V[`@Qa\x06\xD4\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\x11W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x16V[``\x91P[P` \x80\x84\x01\x91\x90\x91R\x90\x15\x15\x80\x83R\x90\x84\x015\x17a\x07gWbF\x1B\xCD`\xE5\x1B`\0R` `\x04R`\x17`$Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`DR`d`\0\xFD[P`\x01\x01a\x06WV[``\x81\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x8CWa\x07\x8Ca\x0B\xB5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xAAW\x90P[P\x91P6`\0[\x82\x81\x10\x15a\x08\xF9W`\0\x84\x82\x81Q\x81\x10a\x07\xF5Wa\x07\xF5a\x0B\xCBV[` \x02` \x01\x01Q\x90P\x86\x86\x83\x81\x81\x10a\x08\x11Wa\x08\x11a\x0B\xCBV[\x90P` \x02\x81\x01\x90a\x08#\x91\x90a\x0CXV[\x92Pa\x082` \x84\x01\x84a\x0BsV[`\x01`\x01`\xA0\x1B\x03\x16a\x08H` \x85\x01\x85a\x0C\x01V[`@Qa\x08V\x92\x91\x90a\x0CHV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x98V[``\x91P[P` \x83\x01R\x15\x15\x81R\x87\x15a\x08\xF0W\x80Qa\x08\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv\x13][\x1D\x1AX\xD8[\x1B\x0C\xCE\x88\x18\xD8[\x1B\x08\x19\x98Z[\x19Y`J\x1B`D\x82\x01R`d\x01a\x04MV[P`\x01\x01a\x07\xD9V[PPP\x93\x92PPPV[`\0\x80``a\t\x14`\x01\x86\x86a\x05\xD3V[\x91\x97\x90\x96P\x90\x94P\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\t4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\tLW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\tgW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\t\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x98W`\0\x80\xFD[a\t\xA4\x85\x82\x86\x01a\t\"V[\x90\x96\x90\x95P\x93PPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\t\xD6W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\t\xBAV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a\n_W`\x1F\x19\x85\x84\x03\x01\x88R\x81Q\x80Q\x15\x15\x84R` \x81\x01Q\x90P`@` \x85\x01Ra\nH`@\x85\x01\x82a\t\xB0V[` \x99\x8A\x01\x99\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01a\n\x14V[P\x90\x96\x95PPPPPPV[` \x81R`\0a\n~` \x83\x01\x84a\t\xF6V[\x93\x92PPPV[`\0`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\n\xE4W`_\x19\x87\x86\x03\x01\x84Ra\n\xCF\x85\x83Qa\t\xB0V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\n\xB3V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0B\x06W`\0\x80\xFD[\x835\x80\x15\x15\x81\x14a\x0B\x16W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B2W`\0\x80\xFD[a\x0B>\x86\x82\x87\x01a\t\"V[\x94\x97\x90\x96P\x93\x94PPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0Bj``\x83\x01\x84a\t\xF6V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x85W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n~W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\xAEW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0C\x18W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\tgW`\0\x80\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12a\x0B\xF7W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x97\x86\xE95\xF7BY\x067|\xC2\x7F\xFF\xEC\xE40\x0CQ\xDE\xE6\x05\xBC\x1A\x80f\x024A\xB9\x12\x03\xB9dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MULTICALL3_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Multicall3<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Multicall3<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Multicall3<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Multicall3<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Multicall3<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Multicall3)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Multicall3<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTICALL3_ABI.clone(),
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
                MULTICALL3_ABI.clone(),
                MULTICALL3_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `aggregate` (0x252dba42) function
        pub fn aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::ethers::core::types::Bytes>),
        > {
            self.0
                .method_hash([37, 45, 186, 66], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregate3` (0x82ad56cb) function
        pub fn aggregate_3(
            &self,
            calls: ::std::vec::Vec<Call3>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([130, 173, 86, 203], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregate3Value` (0x174dea71) function
        pub fn aggregate_3_value(
            &self,
            calls: ::std::vec::Vec<Call3Value>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([23, 77, 234, 113], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockAndAggregate` (0xc3077fa9) function
        pub fn block_and_aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([195, 7, 127, 169], calls)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBasefee` (0x3e64a696) function
        pub fn get_basefee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 100, 166, 150], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockHash` (0xee82ac5e) function
        pub fn get_block_hash(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 130, 172, 94], block_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBlockNumber` (0x42cbb15c) function
        pub fn get_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockCoinbase` (0xa8b0574e) function
        pub fn get_current_block_coinbase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 176, 87, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockGasLimit` (0x86d516e8) function
        pub fn get_current_block_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([134, 213, 22, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function
        pub fn get_current_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEthBalance` (0x4d2301cc) function
        pub fn get_eth_balance(
            &self,
            addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastBlockHash` (0x27e86d6e) function
        pub fn get_last_block_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 232, 109, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryAggregate` (0xbce38bd7) function
        pub fn try_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([188, 227, 139, 215], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryBlockAndAggregate` (0x399542e9) function
        pub fn try_block_and_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([57, 149, 66, 233], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Multicall3<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `0x252dba42`
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
    #[ethcall(name = "aggregate", abi = "aggregate((address,bytes)[])")]
    pub struct AggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `aggregate3` function with signature `aggregate3((address,bool,bytes)[])` and selector `0x82ad56cb`
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
    #[ethcall(name = "aggregate3", abi = "aggregate3((address,bool,bytes)[])")]
    pub struct Aggregate3Call {
        pub calls: ::std::vec::Vec<Call3>,
    }
    ///Container type for all input parameters for the `aggregate3Value` function with signature `aggregate3Value((address,bool,uint256,bytes)[])` and selector `0x174dea71`
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
        name = "aggregate3Value",
        abi = "aggregate3Value((address,bool,uint256,bytes)[])"
    )]
    pub struct Aggregate3ValueCall {
        pub calls: ::std::vec::Vec<Call3Value>,
    }
    ///Container type for all input parameters for the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `0xc3077fa9`
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
    #[ethcall(name = "blockAndAggregate", abi = "blockAndAggregate((address,bytes)[])")]
    pub struct BlockAndAggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `getBasefee` function with signature `getBasefee()` and selector `0x3e64a696`
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
    #[ethcall(name = "getBasefee", abi = "getBasefee()")]
    pub struct GetBasefeeCall;
    ///Container type for all input parameters for the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `0xee82ac5e`
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
    #[ethcall(name = "getBlockHash", abi = "getBlockHash(uint256)")]
    pub struct GetBlockHashCall {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `0xa8b0574e`
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
    #[ethcall(name = "getCurrentBlockCoinbase", abi = "getCurrentBlockCoinbase()")]
    pub struct GetCurrentBlockCoinbaseCall;
    ///Container type for all input parameters for the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `0x86d516e8`
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
    #[ethcall(name = "getCurrentBlockGasLimit", abi = "getCurrentBlockGasLimit()")]
    pub struct GetCurrentBlockGasLimitCall;
    ///Container type for all input parameters for the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
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
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    ///Container type for all input parameters for the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
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
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `0x27e86d6e`
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
    #[ethcall(name = "getLastBlockHash", abi = "getLastBlockHash()")]
    pub struct GetLastBlockHashCall;
    ///Container type for all input parameters for the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `0xbce38bd7`
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
    #[ethcall(name = "tryAggregate", abi = "tryAggregate(bool,(address,bytes)[])")]
    pub struct TryAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    ///Container type for all input parameters for the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `0x399542e9`
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
        name = "tryBlockAndAggregate",
        abi = "tryBlockAndAggregate(bool,(address,bytes)[])"
    )]
    pub struct TryBlockAndAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
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
    pub enum Multicall3Calls {
        Aggregate(AggregateCall),
        Aggregate3(Aggregate3Call),
        Aggregate3Value(Aggregate3ValueCall),
        BlockAndAggregate(BlockAndAggregateCall),
        GetBasefee(GetBasefeeCall),
        GetBlockHash(GetBlockHashCall),
        GetBlockNumber(GetBlockNumberCall),
        GetChainId(GetChainIdCall),
        GetCurrentBlockCoinbase(GetCurrentBlockCoinbaseCall),
        GetCurrentBlockGasLimit(GetCurrentBlockGasLimitCall),
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        GetLastBlockHash(GetLastBlockHashCall),
        TryAggregate(TryAggregateCall),
        TryBlockAndAggregate(TryBlockAndAggregateCall),
    }
    impl ::ethers::core::abi::AbiDecode for Multicall3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aggregate(decoded));
            }
            if let Ok(decoded) = <Aggregate3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aggregate3(decoded));
            }
            if let Ok(decoded) = <Aggregate3ValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aggregate3Value(decoded));
            }
            if let Ok(decoded) = <BlockAndAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlockAndAggregate(decoded));
            }
            if let Ok(decoded) = <GetBasefeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBasefee(decoded));
            }
            if let Ok(decoded) = <GetBlockHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBlockHash(decoded));
            }
            if let Ok(decoded) = <GetBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) = <GetCurrentBlockCoinbaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentBlockCoinbase(decoded));
            }
            if let Ok(decoded) = <GetCurrentBlockGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentBlockGasLimit(decoded));
            }
            if let Ok(decoded) = <GetCurrentBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <GetEthBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEthBalance(decoded));
            }
            if let Ok(decoded) = <GetLastBlockHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLastBlockHash(decoded));
            }
            if let Ok(decoded) = <TryAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryAggregate(decoded));
            }
            if let Ok(decoded) = <TryBlockAndAggregateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TryBlockAndAggregate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Multicall3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Aggregate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Aggregate3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Aggregate3Value(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlockAndAggregate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBasefee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBlockHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentBlockCoinbase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentBlockGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEthBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLastBlockHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryAggregate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TryBlockAndAggregate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for Multicall3Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Aggregate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Aggregate3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Aggregate3Value(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockAndAggregate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBasefee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentBlockCoinbase(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentBlockGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentBlockTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEthBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLastBlockHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryAggregate(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryBlockAndAggregate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AggregateCall> for Multicall3Calls {
        fn from(value: AggregateCall) -> Self {
            Self::Aggregate(value)
        }
    }
    impl ::core::convert::From<Aggregate3Call> for Multicall3Calls {
        fn from(value: Aggregate3Call) -> Self {
            Self::Aggregate3(value)
        }
    }
    impl ::core::convert::From<Aggregate3ValueCall> for Multicall3Calls {
        fn from(value: Aggregate3ValueCall) -> Self {
            Self::Aggregate3Value(value)
        }
    }
    impl ::core::convert::From<BlockAndAggregateCall> for Multicall3Calls {
        fn from(value: BlockAndAggregateCall) -> Self {
            Self::BlockAndAggregate(value)
        }
    }
    impl ::core::convert::From<GetBasefeeCall> for Multicall3Calls {
        fn from(value: GetBasefeeCall) -> Self {
            Self::GetBasefee(value)
        }
    }
    impl ::core::convert::From<GetBlockHashCall> for Multicall3Calls {
        fn from(value: GetBlockHashCall) -> Self {
            Self::GetBlockHash(value)
        }
    }
    impl ::core::convert::From<GetBlockNumberCall> for Multicall3Calls {
        fn from(value: GetBlockNumberCall) -> Self {
            Self::GetBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for Multicall3Calls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetCurrentBlockCoinbaseCall> for Multicall3Calls {
        fn from(value: GetCurrentBlockCoinbaseCall) -> Self {
            Self::GetCurrentBlockCoinbase(value)
        }
    }
    impl ::core::convert::From<GetCurrentBlockGasLimitCall> for Multicall3Calls {
        fn from(value: GetCurrentBlockGasLimitCall) -> Self {
            Self::GetCurrentBlockGasLimit(value)
        }
    }
    impl ::core::convert::From<GetCurrentBlockTimestampCall> for Multicall3Calls {
        fn from(value: GetCurrentBlockTimestampCall) -> Self {
            Self::GetCurrentBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<GetEthBalanceCall> for Multicall3Calls {
        fn from(value: GetEthBalanceCall) -> Self {
            Self::GetEthBalance(value)
        }
    }
    impl ::core::convert::From<GetLastBlockHashCall> for Multicall3Calls {
        fn from(value: GetLastBlockHashCall) -> Self {
            Self::GetLastBlockHash(value)
        }
    }
    impl ::core::convert::From<TryAggregateCall> for Multicall3Calls {
        fn from(value: TryAggregateCall) -> Self {
            Self::TryAggregate(value)
        }
    }
    impl ::core::convert::From<TryBlockAndAggregateCall> for Multicall3Calls {
        fn from(value: TryBlockAndAggregateCall) -> Self {
            Self::TryBlockAndAggregate(value)
        }
    }
    ///Container type for all return fields from the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `0x252dba42`
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
    pub struct AggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub return_data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `aggregate3` function with signature `aggregate3((address,bool,bytes)[])` and selector `0x82ad56cb`
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
    pub struct Aggregate3Return {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `aggregate3Value` function with signature `aggregate3Value((address,bool,uint256,bytes)[])` and selector `0x174dea71`
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
    pub struct Aggregate3ValueReturn {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `0xc3077fa9`
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
    pub struct BlockAndAggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `getBasefee` function with signature `getBasefee()` and selector `0x3e64a696`
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
    pub struct GetBasefeeReturn {
        pub basefee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `0xee82ac5e`
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
    pub struct GetBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `0x42cbb15c`
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
    pub struct GetBlockNumberReturn {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn {
        pub chainid: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `0xa8b0574e`
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
    pub struct GetCurrentBlockCoinbaseReturn {
        pub coinbase: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `0x86d516e8`
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
    pub struct GetCurrentBlockGasLimitReturn {
        pub gaslimit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `0x0f28c97d`
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
    pub struct GetCurrentBlockTimestampReturn {
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getEthBalance` function with signature `getEthBalance(address)` and selector `0x4d2301cc`
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
    pub struct GetEthBalanceReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `0x27e86d6e`
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
    pub struct GetLastBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    ///Container type for all return fields from the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `0xbce38bd7`
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
    pub struct TryAggregateReturn {
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///Container type for all return fields from the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `0x399542e9`
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
    pub struct TryBlockAndAggregateReturn {
        pub block_number: ::ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    ///`Call(address,bytes)`
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
    pub struct Call {
        pub target: ::ethers::core::types::Address,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Call3(address,bool,bytes)`
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
    pub struct Call3 {
        pub target: ::ethers::core::types::Address,
        pub allow_failure: bool,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Call3Value(address,bool,uint256,bytes)`
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
    pub struct Call3Value {
        pub target: ::ethers::core::types::Address,
        pub allow_failure: bool,
        pub value: ::ethers::core::types::U256,
        pub call_data: ::ethers::core::types::Bytes,
    }
    ///`Result(bool,bytes)`
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
    pub struct Result {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
    }
}
