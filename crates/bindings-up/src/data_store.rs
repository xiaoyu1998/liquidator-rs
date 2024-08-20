pub use data_store::*;
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
pub mod data_store {
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
                    ::std::borrow::ToOwned::to_owned("addAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("addBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("addUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addressArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("addressValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addressValues"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("applyBoundedDeltaToUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "applyBoundedDeltaToUint",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("applyDeltaToInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("applyDeltaToInt"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("applyDeltaToUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("applyDeltaToUint"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("errorMessage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("applyDeltaToUint"),
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
                    ::std::borrow::ToOwned::to_owned("boolArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("boolArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("boolValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("boolValues"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bytes32ArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bytes32ArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("bytes32Values"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bytes32Values"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("containsAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("containsAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("containsBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("containsBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("containsUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("containsUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decrementInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decrementInt"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decrementUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decrementUint"),
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
                    ::std::borrow::ToOwned::to_owned("getAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getAddressArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddressArray"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAddressCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddressCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getAddressValuesAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddressValuesAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBool"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBoolArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBoolArray"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getBytes32Array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBytes32Array"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBytes32Count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBytes32Count"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getBytes32ValuesAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBytes32ValuesAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInt"),
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
                    ::std::borrow::ToOwned::to_owned("getIntArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getIntArray"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getString"),
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
                    ::std::borrow::ToOwned::to_owned("getStringArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStringArray"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUint"),
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
                    ::std::borrow::ToOwned::to_owned("getUintArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUintArray"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUintCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUintCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getUintValuesAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUintValuesAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("incrementInt"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("incrementUint"),
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
                    ::std::borrow::ToOwned::to_owned("intArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("intValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intValues"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("removeAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAddress"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeAddressArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeAddressArray"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBool"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeBoolArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBoolArray"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBytes32"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeBytes32Array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBytes32Array"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeInt"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeIntArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeIntArray"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeString"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeStringArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeStringArray"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeUint"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setKey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeUintArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeUintArray"),
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
                (
                    ::std::borrow::ToOwned::to_owned("setAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddress"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAddressArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddressArray"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("setBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBool"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBoolArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBoolArray"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
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
                    ::std::borrow::ToOwned::to_owned("setBytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBytes32"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBytes32Array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBytes32Array"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                    ::std::borrow::ToOwned::to_owned("setInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setInt"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setIntArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setIntArray"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
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
                    ::std::borrow::ToOwned::to_owned("setString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setString"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStringArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setStringArray"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
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
                    ::std::borrow::ToOwned::to_owned("setUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUint"),
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
                    ::std::borrow::ToOwned::to_owned("setUintArray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUintArray"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                    ::std::borrow::ToOwned::to_owned("stringArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stringArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("stringValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stringValues"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("uintArrayValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uintArrayValues"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("uintValues"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uintValues"),
                            inputs: ::std::vec![
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
    pub static DATASTORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa*\x1D8\x03\x80a*\x1D\x839\x81\x01`@\x81\x90R`,\x91`<V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`jV[`\0` \x82\x84\x03\x12\x15`MW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa)\x91a\0\x8C`\09`\0\x81\x81a\x06,\x01Ra\x18\xA0\x01Ra)\x91`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04(W`\x005`\xE0\x1C\x80c\x99!\xC3\xCC\x11a\x02+W\x80c\xCB\xB0\x93\xDD\x11a\x010W\x80c\xE4\xE3lN\x11a\0\xB8W\x80c\xECg,\xF6\x11a\0\x87W\x80c\xECg,\xF6\x14a\nCW\x80c\xF0i\x05*\x14a\nVW\x80c\xF1\\\xAE\xAC\x14a\niW\x80c\xF3\x90;\x9F\x14a\n|W\x80c\xF5\x1F\xC0\xD9\x14a\n\x8FW`\0\x80\xFD[\x80c\xE4\xE3lN\x14a\t8W\x80c\xE6$a\xCE\x14a\n\nW\x80c\xE7\xE4\x14\x8E\x14a\n\x1DW\x80c\xE9\x8A\xAB\xC1\x14a\n0W`\0\x80\xFD[\x80c\xD5(R\xAF\x11a\0\xFFW\x80c\xD5(R\xAF\x14a\t\x91W\x80c\xDC\x97\xD9b\x14a\t\xB1W\x80c\xDD\x03\x19\x97\x14a\t\xD1W\x80c\xE2\x08\xA7\r\x14a\t\xE4W\x80c\xE2\xA4\x85:\x14a\t\xF7W`\0\x80\xFD[\x80c\xCB\xB0\x93\xDD\x14a\t8W\x80c\xCCP\xEA\xDD\x14a\tKW\x80c\xCFj\x87\"\x14a\t^W\x80c\xD3\x8E\xEB\xC7\x14a\tqW`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x11a\x01\xB3W\x80c\xC1\xDC\x91\x82\x11a\x01\x82W\x80c\xC1\xDC\x91\x82\x14a\x08\xD9W\x80c\xC4\xF0\x0F\xDE\x14a\x08\xECW\x80c\xC7i\xD1\xA1\x14a\x08\xFFW\x80c\xC8\x0FLb\x14a\t\x12W\x80c\xCADm\xD9\x14a\t%W`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x14a\x08\x80W\x80c\xBEC\xCA\xA3\x14a\x08\xA0W\x80c\xBFI\x8D\xD3\x14a\x08\xB3W\x80c\xBF\x7F\x03Z\x14a\x08\xC6W`\0\x80\xFD[\x80c\xA9\xFC\xF7k\x11a\x01\xFAW\x80c\xA9\xFC\xF7k\x14a\x08!W\x80c\xAB\xFD\xCC\xED\x14a\x084W\x80c\xAD\xB3S\xDC\x14a\x08GW\x80c\xB3H\xE69\x14a\x08ZW\x80c\xB82\n\x08\x14a\x08mW`\0\x80\xFD[\x80c\x99!\xC3\xCC\x14a\x07\xC8W\x80c\x9F\xAFo\xB6\x14a\x07\xDBW\x80c\x9F\xE7\xAC\x12\x14a\x07\xEEW\x80c\xA6\xEDV>\x14a\x08\x01W`\0\x80\xFD[\x80cJJ{\x04\x11a\x031W\x80ct=\xF3%\x11a\x02\xB9W\x80c\x88\x02\x1Ar\x11a\x02\x88W\x80c\x88\x02\x1Ar\x14a\x07iW\x80c\x8C\xA4\x98\xB0\x14a\x07|W\x80c\x91\xD4@<\x14a\x07\x8FW\x80c\x93&o\x9A\x14a\x07\xA2W\x80c\x98ny\x1A\x14a\x07\xB5W`\0\x80\xFD[\x80ct=\xF3%\x14a\x07\0W\x80cz\xE1\xCF\xCA\x14a\x07 W\x80c\x80\xAA\xCD\xCD\x14a\x07CW\x80c\x86\xACk\xDF\x14a\x07VW`\0\x80\xFD[\x80cc9sM\x11a\x03\0W\x80cc9sM\x14a\x06\x94W\x80cir\x1DA\x14a\x06\xA7W\x80cn\x89\x95P\x14a\x06\xBAW\x80co\xAET\xF0\x14a\x06\xDAW\x80cp&\xD4,\x14a\x06\xEDW`\0\x80\xFD[\x80cJJ{\x04\x14a\x06'W\x80cN\x91\xDB\x08\x14a\x06NW\x80cYH\xF73\x14a\x06aW\x80c^\xB0}\xBD\x14a\x06\x81W`\0\x80\xFD[\x80c2\xF8[\xBF\x11a\x03\xB4W\x80c=\xBA\xCD\x1A\x11a\x03\x83W\x80c=\xBA\xCD\x1A\x14a\x05\xB8W\x80c>I\xBE\xD0\x14a\x05\xCBW\x80cB\xC3\xBD\x96\x14a\x05\xDEW\x80cD\xA2B\xB1\x14a\x05\xF1W\x80cI\x9E\xA5\x0E\x14a\x06\x14W`\0\x80\xFD[\x80c2\xF8[\xBF\x14a\x05lW\x80c4\r\xBA\xB3\x14a\x05\x7FW\x80c5\xD4\xD4\x07\x14a\x05\x92W\x80c5\xEA\x80Y\x14a\x05\xA5W`\0\x80\xFD[\x80c\"S\x8D\xAE\x11a\x03\xFBW\x80c\"S\x8D\xAE\x14a\x04\xD8W\x80c\"\xF8td\x14a\x05\x01W\x80c&\0HF\x14a\x05\x14W\x80c-(\x99\xB6\x14a\x05)W\x80c1\x0B\x88\x82\x14a\x05IW`\0\x80\xFD[\x80c\x01g}\xA2\x14a\x04-W\x80c\x06_!\xA7\x14a\x04VW\x80c\x11k\xB9)\x14a\x04wW\x80c!\xF8\xA7!\x14a\x04\x97W[`\0\x80\xFD[a\x04@a\x04;6`\x04a qV[a\n\xA2V[`@Qa\x04M\x91\x90a \xD0V[`@Q\x80\x91\x03\x90\xF3[a\x04ia\x04d6`\x04a qV[a\x0B\x8EV[`@Q\x90\x81R` \x01a\x04MV[a\x04\x8Aa\x04\x856`\x04a qV[a\x0B\xABV[`@Qa\x04M\x91\x90a!5V[a\x04\xC0a\x04\xA56`\x04a qV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04MV[a\x04\xC0a\x04\xE66`\x04a qV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xC0a\x05\x0F6`\x04a!zV[a\x0C-V[a\x05'a\x05\"6`\x04a\"\x05V[a\x0CeV[\0[a\x05<a\x0576`\x04a qV[a\x0C\xD9V[`@Qa\x04M\x91\x90a\"\xABV[a\x05\\a\x05W6`\x04a!zV[a\r:V[`@Q\x90\x15\x15\x81R` \x01a\x04MV[a\x04ia\x05z6`\x04a#RV[a\rYV[a\x04ia\x05\x8D6`\x04a!zV[a\r\xEAV[a\x05'a\x05\xA06`\x04a#\xAFV[a\x0E2V[a\x04ia\x05\xB36`\x04a qV[a\x0EcV[a\x04ia\x05\xC66`\x04a!zV[a\x0EzV[a\x04ia\x05\xD96`\x04a!zV[a\x0E\xC3V[a\x05'a\x05\xEC6`\x04a qV[a\x0E\xEDV[a\x05\\a\x05\xFF6`\x04a qV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05'a\x06\"6`\x04a qV[a\x0F\x10V[a\x04\xC0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04ia\x06\\6`\x04a!zV[a\x0F<V[a\x06ta\x06o6`\x04a qV[a\x0FfV[`@Qa\x04M\x91\x90a$KV[a\x05'a\x06\x8F6`\x04a\"\x05V[a\x0F\xD1V[a\x04ia\x06\xA26`\x04a!zV[a\x10\x02V[a\x05'a\x06\xB56`\x04a$\xA8V[a\x103V[a\x06\xCDa\x06\xC86`\x04a$\xD4V[a\x10]V[`@Qa\x04M\x91\x90a%\x1AV[a\x04ia\x06\xE86`\x04a!zV[a\x10\x91V[a\x05<a\x06\xFB6`\x04a%-V[a\x10\xD9V[a\x04ia\x07\x0E6`\x04a qV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x05\\a\x07.6`\x04a qV[`\0\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x90V[a\x05\\a\x07Q6`\x04a!zV[a\x10\xFDV[a\x05<a\x07d6`\x04a qV[a\x11@V[a\x05'a\x07w6`\x04a%YV[a\x11\xA0V[a\x04ia\x07\x8A6`\x04a!zV[a\x11\xD1V[a\x05\\a\x07\x9D6`\x04a!zV[a\x126V[a\x05'a\x07\xB06`\x04a!zV[a\x12NV[a\x06\xCDa\x07\xC36`\x04a qV[a\x12xV[a\x05'a\x07\xD66`\x04a!zV[a\x13\x10V[a\x05'a\x07\xE96`\x04a qV[a\x13:V[a\x05'a\x07\xFC6`\x04a qV[a\x13jV[a\x04ia\x08\x0F6`\x04a qV[`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x05'a\x08/6`\x04a\"\x05V[a\x13\x94V[a\x05\\a\x08B6`\x04a&\x1AV[a\x13\xC5V[a\x05'a\x08U6`\x04a!zV[a\x13\xF8V[a\x05'a\x08h6`\x04a$\xA8V[a\x14\"V[a\x06\xCDa\x08{6`\x04a!zV[a\x14LV[a\x04ia\x08\x8E6`\x04a qV[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x05'a\x08\xAE6`\x04a qV[a\x15\x05V[a\x04ia\x08\xC16`\x04a!zV[a\x15.V[a\x05'a\x08\xD46`\x04a qV[a\x15JV[a\x05'a\x08\xE76`\x04a qV[a\x15sV[a\x04ia\x08\xFA6`\x04a!zV[a\x15\x9CV[a\x05\\a\t\r6`\x04a$\xA8V[a\x15\xB8V[a\x05'a\t 6`\x04a!zV[a\x15\xD0V[a\x04\xC0a\t36`\x04a$\xA8V[a\x15\xFAV[a\x04ia\tF6`\x04a!zV[a\x16:V[a\x05'a\tY6`\x04a qV[a\x16hV[a\x05'a\tl6`\x04a qV[a\x16\x91V[a\x04ia\t\x7F6`\x04a qV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\x04ia\t\x9F6`\x04a qV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x04ia\t\xBF6`\x04a qV[`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x05<a\t\xDF6`\x04a qV[a\x16\xB4V[a\x05'a\t\xF26`\x04a qV[a\x17\x14V[a\x04ia\n\x056`\x04a!zV[a\x17=V[a\x05'a\n\x186`\x04a qV[a\x17gV[a\x06ta\n+6`\x04a%-V[a\x17\x8AV[a\x04ia\n>6`\x04a!zV[a\x17\xA6V[a\x05'a\nQ6`\x04a&JV[a\x17\xD4V[a\x05<a\nd6`\x04a%-V[a\x18\x05V[a\x06\xCDa\nw6`\x04a qV[a\x18!V[a\x04ia\n\x8A6`\x04a qV[a\x18:V[a\x05'a\n\x9D6`\x04a qV[a\x18QV[```\n`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x83W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xF6\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\"\x90a&\xE4V[\x80\x15a\x0BoW\x80`\x1F\x10a\x0BDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BoV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BRW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xD7V[PPPP\x90P\x91\x90PV[`\0\x81\x81R`\x0E` R`@\x81 a\x0B\xA5\x90a\x18zV[\x92\x91PPV[`\0\x81\x81R`\t` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82Ta\x01\0\x83\x90\n\x90\x04`\xFF\x16\x15\x15\x81R` `\x01\x92\x83\x01\x81\x81\x04\x94\x85\x01\x94\x90\x93\x03\x90\x92\x02\x91\x01\x80\x84\x11a\x0B\xF0W\x90P[PPPPP\x90P\x91\x90PV[`\x08` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0CIW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[a\x0C\xB5`@Q` \x01a\x0Cw\x90a'\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x18\x84V[`\0\x82\x81R`\x0B` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[PPPV[`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[`\0\x82\x81R`\x0E` R`@\x81 a\rR\x90\x83a\x198V[\x93\x92PPPV[`\0a\rm`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x84\x81R` \x81\x90R`@\x81 T\x90\x84\x12\x80\x15a\r\x9AWP\x80a\r\x98a\r\x93\x86a'XV[a\x19PV[\x11[\x15a\r\xC2W\x82`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xB9\x91\x90a%\x1AV[`@Q\x80\x91\x03\x90\xFD[`\0a\r\xCE\x82\x86a\x19\xA6V[`\0\x87\x81R` \x81\x90R`@\x90 \x81\x90U\x92PPP\x93\x92PPPV[`\0a\r\xFE`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 Ta\x0E\x18\x90\x84\x90a'tV[`\0\x85\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x92\x91PPV[a\x0ED`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\t` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1EaV[`\0\x81\x81R`\r` R`@\x81 a\x0B\xA5\x90a\x18zV[`\0a\x0E\x8E`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 T\x90a\x0E\xA8\x84\x83a'tV[`\0\x86\x81R` \x81\x90R`@\x90 \x81\x90U\x92PPP\x92\x91PPV[`\0a\x0E\xD7`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x01` R`@\x90\x91 \x81\x90U\x90V[a\x0E\xFF`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R` \x81\x90R`@\x81 UV[a\x0F\"`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x07` R`@\x81 a\x0F9\x91a\x1F\x01V[PV[`\0a\x0FP`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x05` R`@\x90\x91 \x81\x90U\x90V[`\0\x81\x81R`\x08` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0F\xA8WPPPPP\x90P\x91\x90PV[a\x0F\xE3`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x06` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[`\x07` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[a\x10E`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\r` R`@\x90 a\x0C\xD4\x90\x82a\x19\xDDV[``a\x10q`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x04` R`@\x90 a\x10\x89\x83\x82a'\xD5V[P\x90\x92\x91PPV[`\0a\x10\xA5`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x01` R`@\x81 Ta\x10\xBF\x90\x84\x90a(\x93V[`\0\x85\x81R`\x01` R`@\x90 \x81\x90U\x91PP\x92\x91PPV[`\0\x83\x81R`\x0E` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x19\xF2V[\x94\x93PPPPV[`\t` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11\x19W`\0\x80\xFD[\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x81\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[a\x11\xB2`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\n` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1F\x1FV[`\0a\x11\xE5`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 T\x90\x83\x12\x80\x15a\x12\rWP\x80a\x12\x0Ba\r\x93\x85a'XV[\x11[\x15a\x12*WPP`\0\x82\x81R` \x81\x90R`@\x81 \x81\x90Ua\x0B\xA5V[`\0a\x0E\xA8\x82\x85a\x19\xA6V[`\0\x82\x81R`\x0C` R`@\x81 a\rR\x90\x83a\x198V[a\x12``@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0E` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xCCV[`\0\x81\x81R`\x04` R`@\x90 \x80T``\x91\x90a\x12\x95\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xC1\x90a&\xE4V[\x80\x15a\x0C!W\x80`\x1F\x10a\x12\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xF1WP\x93\x96\x95PPPPPPV[a\x13\"`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0C` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xCCV[a\x13L`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[a\x13|`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90UV[a\x13\xA6`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x07` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[`\0a\x13\xD9`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x03` R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x15\x15\x17\x90U\x90V[a\x14\n`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0E` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xD8V[a\x144`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\r` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xE4V[`\n` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x14hW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PP\x80Ta\x14\x84\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xB0\x90a&\xE4V[\x80\x15a\x14\xFDW\x80`\x1F\x10a\x14\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x15\x17`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x06` R`@\x81 a\x0F9\x91a\x1F\x01V[`\x0B` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[a\x15\\`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x0B` R`@\x81 a\x0F9\x91a\x1F\x01V[a\x15\x85`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x08` R`@\x81 a\x0F9\x91a\x1F\x01V[`\x06` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[`\0\x82\x81R`\r` R`@\x81 a\rR\x90\x83a\x1A\xF9V[a\x15\xE2`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0C` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xD8V[`\0a\x16\x0E`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x02` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x90V[`\0a\x16N`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x01` R`@\x81 Ta\x10\xBF\x90\x84\x90a(\xB3V[a\x16z`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x04` R`@\x81 a\x0F9\x91a\x1FqV[a\x16\xA3`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x05` R`@\x81 UV[`\0\x81\x81R`\x0B` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[a\x17&`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\n` R`@\x81 a\x0F9\x91a\x1F\xABV[`\0a\x17Q`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R` \x82\x90R`@\x90\x91 \x81\x90U\x90V[a\x17y`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x01` R`@\x81 UV[`\0\x83\x81R`\r` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x1B\x1BV[`\0a\x17\xBA`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 Ta\x0E\x18\x90\x84\x90a(\xDBV[a\x17\xE6`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x08` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1F\xC9V[`\0\x83\x81R`\x0C` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x1B\xD4V[`\x04` R`\0\x90\x81R`@\x90 \x80Ta\x14\x84\x90a&\xE4V[`\0\x81\x81R`\x0C` R`@\x81 a\x0B\xA5\x90a\x18zV[a\x18c`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\t` R`@\x81 a\x0F9\x91a \x1EV[`\0a\x0B\xA5\x82T\x90V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x13\x91\x90a(\xEEV[a\x194W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\r\xB9\x92\x91\x90a)\x0BV[PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\rRV[`\0\x80\x82\x12\x15a\x19\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FSafeCast: value must be positive`D\x82\x01R`d\x01a\r\xB9V[P\x90V[`\0\x80\x82\x13\x15a\x19\xCAWa\x19\xB9\x82a\x1C\x80V[a\x19\xC3\x90\x84a'tV[\x90Pa\x0B\xA5V[a\x19\xD3\x82a\x1C\x80V[a\rR\x90\x84a(\xDBV[`\0a\rR\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1C\x93V[``a\x19\xFD\x84a\x18zV[\x83\x10a\x1A\x18WP`@\x80Q`\0\x81R` \x81\x01\x90\x91Ra\rRV[`\0a\x1A#\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1A1W\x80\x92P[`\0a\x1A=\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1ATWa\x1ATa!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1A\x94\x87\x82a\x1D\x8DV[\x82a\x1A\x9F\x88\x84a(\xDBV[\x81Q\x81\x10a\x1A\xAFWa\x1A\xAFa)/V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x82V[P\x95\x94PPPPPV[`\0a\rR\x83\x83a\x1C\x93V[`\0a\rR\x83\x83a\x1D\x99V[`\0a\rR\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1D\x99V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\rRV[```\0a\x1B(\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1B6W\x80\x92P[`\0a\x1BB\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BYWa\x1BYa!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x82W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1B\x99\x87\x82a\x1D\x8DV[\x82a\x1B\xA4\x88\x84a(\xDBV[\x81Q\x81\x10a\x1B\xB4Wa\x1B\xB4a)/V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1B\x87V[```\0a\x1B\xE1\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1B\xEFW\x80\x92P[`\0a\x1B\xFB\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x12Wa\x1C\x12a!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C;W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1CR\x87\x82a\x1D\xE8V[\x82a\x1C]\x88\x84a(\xDBV[\x81Q\x81\x10a\x1CmWa\x1Cma)/V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1C@V[`\0\x80\x82\x12\x15a\x19\xA2W\x81`\0\x03a\x0B\xA5V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1D|W`\0a\x1C\xB7`\x01\x83a(\xDBV[\x85T\x90\x91P`\0\x90a\x1C\xCB\x90`\x01\x90a(\xDBV[\x90P\x81\x81\x14a\x1D0W`\0\x86`\0\x01\x82\x81T\x81\x10a\x1C\xEBWa\x1C\xEBa)/V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x1D\x0EWa\x1D\x0Ea)/V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1DAWa\x1DAa)EV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x0B\xA5V[`\0\x91PPa\x0B\xA5V[P\x92\x91PPV[`\0a\rR\x83\x83a\x1D\xF0V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1D\xE0WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0B\xA5V[P`\0a\x0B\xA5V[`\0a\rR\x83\x83[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1E\x07Wa\x1E\x07a)/V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1EUW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1E:V[Pa\x19\xA2\x92\x91Pa ?V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01`\0[\x83\x82\x11\x15a\x1E\xC7W\x83Q\x83\x82a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x92` \x01\x92`\x01\x01` \x81`\0\x01\x04\x92\x83\x01\x92`\x01\x03\x02a\x1E\x8AV[\x80\x15a\x1E\xF4W\x82\x81a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x01` \x81`\0\x01\x04\x92\x83\x01\x92`\x01\x03\x02a\x1E\xC7V[PPa\x19\xA2\x92\x91Pa ?V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a ?V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1FeW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1FeW\x82Q\x82\x90a\x1FU\x90\x82a'\xD5V[P\x91` \x01\x91\x90`\x01\x01\x90a\x1F?V[Pa\x19\xA2\x92\x91Pa TV[P\x80Ta\x1F}\x90a&\xE4V[`\0\x82U\x80`\x1F\x10a\x1F\x8DWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a ?V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a TV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1EUW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x1F\xE9V[P\x80T`\0\x82U`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90[[\x80\x82\x11\x15a\x19\xA2W`\0\x81U`\x01\x01a @V[\x80\x82\x11\x15a\x19\xA2W`\0a h\x82\x82a\x1FqV[P`\x01\x01a TV[`\0` \x82\x84\x03\x12\x15a \x83W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a \xB0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a \x94V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a!)W`?\x19\x87\x86\x03\x01\x84Ra!\x14\x85\x83Qa \x8AV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a \xF8V[P\x92\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q\x15\x15\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a!OV[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a!\x8DW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!\xDAWa!\xDAa!\x9CV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a!\xFBWa!\xFBa!\x9CV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\"\x18W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"FW`\0\x80\xFD[\x805a\"Ya\"T\x82a!\xE2V[a!\xB2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\"{W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\"\x82V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"\xC5V[`\0\x82`\x1F\x83\x01\x12a\"\xF4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#\rWa#\ra!\x9CV[a# `\x1F\x82\x01`\x1F\x19\x16` \x01a!\xB2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a#5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#gW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x8BW`\0\x80\xFD[a#\x97\x86\x82\x87\x01a\"\xE3V[\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x0F9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\xC2W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\xF0W`\0\x80\xFD[\x805a#\xFEa\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a$ W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DW\x835a$:\x81a#\xA1V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a$'V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$eV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xA3W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xBBW`\0\x80\xFD[\x825\x91Pa$\xCB` \x84\x01a$\x8CV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x04W`\0\x80\xFD[a%\x10\x85\x82\x86\x01a\"\xE3V[\x91PP\x92P\x92\x90PV[` \x81R`\0a\rR` \x83\x01\x84a \x8AV[`\0\x80`\0``\x84\x86\x03\x12\x15a%BW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a%lW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x89W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a%\x9AW`\0\x80\xFD[\x805a%\xA8a\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a%\xCAW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a&\x0BW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xEDW`\0\x80\xFD[a%\xFC\x8A` \x83\x89\x01\x01a\"\xE3V[\x84RP` \x92\x83\x01\x92\x01a%\xCFV[P\x80\x94PPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&-W`\0\x80\xFD[\x825\x91P` \x83\x015a&?\x81a#\xA1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&]W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&zW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\x8BW`\0\x80\xFD[\x805a&\x99a\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a&\xBBW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DWa&\xD3\x84a$\x8CV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a&\xC2V[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xF8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a'mWa'ma'BV[P`\0\x03\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xA5Wa\x0B\xA5a'BV[`\x1F\x82\x11\x15a\x0C\xD4W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a'\xAEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\xCEW`\0\x81U`\x01\x01a'\xBAV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xEEWa'\xEEa!\x9CV[a(\x02\x81a'\xFC\x84Ta&\xE4V[\x84a'\x87V[` `\x1F\x82\x11`\x01\x81\x14a(6W`\0\x83\x15a(\x1EWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua'\xCEV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a(fW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a(FV[P\x84\x82\x10\x15a(\x84W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D\x86Wa\x1D\x86a'BV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xD3Wa(\xD3a'BV[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xA5Wa\x0B\xA5a'BV[`\0` \x82\x84\x03\x12\x15a)\0W`\0\x80\xFD[\x81Qa\rR\x81a#\xA1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x10\xF5\x90\x83\x01\x84a \x8AV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xEA~\xA7Y\xCB\xAE\xEB\x92D\xDDQ\x86\xC3-\xA0\xC9\x0F\xF7,R\x87\xE5\x16\xD2y\x1A\x19b\x13\xDD\x02\xC7dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static DATASTORE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04(W`\x005`\xE0\x1C\x80c\x99!\xC3\xCC\x11a\x02+W\x80c\xCB\xB0\x93\xDD\x11a\x010W\x80c\xE4\xE3lN\x11a\0\xB8W\x80c\xECg,\xF6\x11a\0\x87W\x80c\xECg,\xF6\x14a\nCW\x80c\xF0i\x05*\x14a\nVW\x80c\xF1\\\xAE\xAC\x14a\niW\x80c\xF3\x90;\x9F\x14a\n|W\x80c\xF5\x1F\xC0\xD9\x14a\n\x8FW`\0\x80\xFD[\x80c\xE4\xE3lN\x14a\t8W\x80c\xE6$a\xCE\x14a\n\nW\x80c\xE7\xE4\x14\x8E\x14a\n\x1DW\x80c\xE9\x8A\xAB\xC1\x14a\n0W`\0\x80\xFD[\x80c\xD5(R\xAF\x11a\0\xFFW\x80c\xD5(R\xAF\x14a\t\x91W\x80c\xDC\x97\xD9b\x14a\t\xB1W\x80c\xDD\x03\x19\x97\x14a\t\xD1W\x80c\xE2\x08\xA7\r\x14a\t\xE4W\x80c\xE2\xA4\x85:\x14a\t\xF7W`\0\x80\xFD[\x80c\xCB\xB0\x93\xDD\x14a\t8W\x80c\xCCP\xEA\xDD\x14a\tKW\x80c\xCFj\x87\"\x14a\t^W\x80c\xD3\x8E\xEB\xC7\x14a\tqW`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x11a\x01\xB3W\x80c\xC1\xDC\x91\x82\x11a\x01\x82W\x80c\xC1\xDC\x91\x82\x14a\x08\xD9W\x80c\xC4\xF0\x0F\xDE\x14a\x08\xECW\x80c\xC7i\xD1\xA1\x14a\x08\xFFW\x80c\xC8\x0FLb\x14a\t\x12W\x80c\xCADm\xD9\x14a\t%W`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x14a\x08\x80W\x80c\xBEC\xCA\xA3\x14a\x08\xA0W\x80c\xBFI\x8D\xD3\x14a\x08\xB3W\x80c\xBF\x7F\x03Z\x14a\x08\xC6W`\0\x80\xFD[\x80c\xA9\xFC\xF7k\x11a\x01\xFAW\x80c\xA9\xFC\xF7k\x14a\x08!W\x80c\xAB\xFD\xCC\xED\x14a\x084W\x80c\xAD\xB3S\xDC\x14a\x08GW\x80c\xB3H\xE69\x14a\x08ZW\x80c\xB82\n\x08\x14a\x08mW`\0\x80\xFD[\x80c\x99!\xC3\xCC\x14a\x07\xC8W\x80c\x9F\xAFo\xB6\x14a\x07\xDBW\x80c\x9F\xE7\xAC\x12\x14a\x07\xEEW\x80c\xA6\xEDV>\x14a\x08\x01W`\0\x80\xFD[\x80cJJ{\x04\x11a\x031W\x80ct=\xF3%\x11a\x02\xB9W\x80c\x88\x02\x1Ar\x11a\x02\x88W\x80c\x88\x02\x1Ar\x14a\x07iW\x80c\x8C\xA4\x98\xB0\x14a\x07|W\x80c\x91\xD4@<\x14a\x07\x8FW\x80c\x93&o\x9A\x14a\x07\xA2W\x80c\x98ny\x1A\x14a\x07\xB5W`\0\x80\xFD[\x80ct=\xF3%\x14a\x07\0W\x80cz\xE1\xCF\xCA\x14a\x07 W\x80c\x80\xAA\xCD\xCD\x14a\x07CW\x80c\x86\xACk\xDF\x14a\x07VW`\0\x80\xFD[\x80cc9sM\x11a\x03\0W\x80cc9sM\x14a\x06\x94W\x80cir\x1DA\x14a\x06\xA7W\x80cn\x89\x95P\x14a\x06\xBAW\x80co\xAET\xF0\x14a\x06\xDAW\x80cp&\xD4,\x14a\x06\xEDW`\0\x80\xFD[\x80cJJ{\x04\x14a\x06'W\x80cN\x91\xDB\x08\x14a\x06NW\x80cYH\xF73\x14a\x06aW\x80c^\xB0}\xBD\x14a\x06\x81W`\0\x80\xFD[\x80c2\xF8[\xBF\x11a\x03\xB4W\x80c=\xBA\xCD\x1A\x11a\x03\x83W\x80c=\xBA\xCD\x1A\x14a\x05\xB8W\x80c>I\xBE\xD0\x14a\x05\xCBW\x80cB\xC3\xBD\x96\x14a\x05\xDEW\x80cD\xA2B\xB1\x14a\x05\xF1W\x80cI\x9E\xA5\x0E\x14a\x06\x14W`\0\x80\xFD[\x80c2\xF8[\xBF\x14a\x05lW\x80c4\r\xBA\xB3\x14a\x05\x7FW\x80c5\xD4\xD4\x07\x14a\x05\x92W\x80c5\xEA\x80Y\x14a\x05\xA5W`\0\x80\xFD[\x80c\"S\x8D\xAE\x11a\x03\xFBW\x80c\"S\x8D\xAE\x14a\x04\xD8W\x80c\"\xF8td\x14a\x05\x01W\x80c&\0HF\x14a\x05\x14W\x80c-(\x99\xB6\x14a\x05)W\x80c1\x0B\x88\x82\x14a\x05IW`\0\x80\xFD[\x80c\x01g}\xA2\x14a\x04-W\x80c\x06_!\xA7\x14a\x04VW\x80c\x11k\xB9)\x14a\x04wW\x80c!\xF8\xA7!\x14a\x04\x97W[`\0\x80\xFD[a\x04@a\x04;6`\x04a qV[a\n\xA2V[`@Qa\x04M\x91\x90a \xD0V[`@Q\x80\x91\x03\x90\xF3[a\x04ia\x04d6`\x04a qV[a\x0B\x8EV[`@Q\x90\x81R` \x01a\x04MV[a\x04\x8Aa\x04\x856`\x04a qV[a\x0B\xABV[`@Qa\x04M\x91\x90a!5V[a\x04\xC0a\x04\xA56`\x04a qV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04MV[a\x04\xC0a\x04\xE66`\x04a qV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xC0a\x05\x0F6`\x04a!zV[a\x0C-V[a\x05'a\x05\"6`\x04a\"\x05V[a\x0CeV[\0[a\x05<a\x0576`\x04a qV[a\x0C\xD9V[`@Qa\x04M\x91\x90a\"\xABV[a\x05\\a\x05W6`\x04a!zV[a\r:V[`@Q\x90\x15\x15\x81R` \x01a\x04MV[a\x04ia\x05z6`\x04a#RV[a\rYV[a\x04ia\x05\x8D6`\x04a!zV[a\r\xEAV[a\x05'a\x05\xA06`\x04a#\xAFV[a\x0E2V[a\x04ia\x05\xB36`\x04a qV[a\x0EcV[a\x04ia\x05\xC66`\x04a!zV[a\x0EzV[a\x04ia\x05\xD96`\x04a!zV[a\x0E\xC3V[a\x05'a\x05\xEC6`\x04a qV[a\x0E\xEDV[a\x05\\a\x05\xFF6`\x04a qV[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05'a\x06\"6`\x04a qV[a\x0F\x10V[a\x04\xC0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04ia\x06\\6`\x04a!zV[a\x0F<V[a\x06ta\x06o6`\x04a qV[a\x0FfV[`@Qa\x04M\x91\x90a$KV[a\x05'a\x06\x8F6`\x04a\"\x05V[a\x0F\xD1V[a\x04ia\x06\xA26`\x04a!zV[a\x10\x02V[a\x05'a\x06\xB56`\x04a$\xA8V[a\x103V[a\x06\xCDa\x06\xC86`\x04a$\xD4V[a\x10]V[`@Qa\x04M\x91\x90a%\x1AV[a\x04ia\x06\xE86`\x04a!zV[a\x10\x91V[a\x05<a\x06\xFB6`\x04a%-V[a\x10\xD9V[a\x04ia\x07\x0E6`\x04a qV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x05\\a\x07.6`\x04a qV[`\0\x90\x81R`\x03` R`@\x90 T`\xFF\x16\x90V[a\x05\\a\x07Q6`\x04a!zV[a\x10\xFDV[a\x05<a\x07d6`\x04a qV[a\x11@V[a\x05'a\x07w6`\x04a%YV[a\x11\xA0V[a\x04ia\x07\x8A6`\x04a!zV[a\x11\xD1V[a\x05\\a\x07\x9D6`\x04a!zV[a\x126V[a\x05'a\x07\xB06`\x04a!zV[a\x12NV[a\x06\xCDa\x07\xC36`\x04a qV[a\x12xV[a\x05'a\x07\xD66`\x04a!zV[a\x13\x10V[a\x05'a\x07\xE96`\x04a qV[a\x13:V[a\x05'a\x07\xFC6`\x04a qV[a\x13jV[a\x04ia\x08\x0F6`\x04a qV[`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x05'a\x08/6`\x04a\"\x05V[a\x13\x94V[a\x05\\a\x08B6`\x04a&\x1AV[a\x13\xC5V[a\x05'a\x08U6`\x04a!zV[a\x13\xF8V[a\x05'a\x08h6`\x04a$\xA8V[a\x14\"V[a\x06\xCDa\x08{6`\x04a!zV[a\x14LV[a\x04ia\x08\x8E6`\x04a qV[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x05'a\x08\xAE6`\x04a qV[a\x15\x05V[a\x04ia\x08\xC16`\x04a!zV[a\x15.V[a\x05'a\x08\xD46`\x04a qV[a\x15JV[a\x05'a\x08\xE76`\x04a qV[a\x15sV[a\x04ia\x08\xFA6`\x04a!zV[a\x15\x9CV[a\x05\\a\t\r6`\x04a$\xA8V[a\x15\xB8V[a\x05'a\t 6`\x04a!zV[a\x15\xD0V[a\x04\xC0a\t36`\x04a$\xA8V[a\x15\xFAV[a\x04ia\tF6`\x04a!zV[a\x16:V[a\x05'a\tY6`\x04a qV[a\x16hV[a\x05'a\tl6`\x04a qV[a\x16\x91V[a\x04ia\t\x7F6`\x04a qV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\x04ia\t\x9F6`\x04a qV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[a\x04ia\t\xBF6`\x04a qV[`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x05<a\t\xDF6`\x04a qV[a\x16\xB4V[a\x05'a\t\xF26`\x04a qV[a\x17\x14V[a\x04ia\n\x056`\x04a!zV[a\x17=V[a\x05'a\n\x186`\x04a qV[a\x17gV[a\x06ta\n+6`\x04a%-V[a\x17\x8AV[a\x04ia\n>6`\x04a!zV[a\x17\xA6V[a\x05'a\nQ6`\x04a&JV[a\x17\xD4V[a\x05<a\nd6`\x04a%-V[a\x18\x05V[a\x06\xCDa\nw6`\x04a qV[a\x18!V[a\x04ia\n\x8A6`\x04a qV[a\x18:V[a\x05'a\n\x9D6`\x04a qV[a\x18QV[```\n`\0\x83\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\x83W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\xF6\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\"\x90a&\xE4V[\x80\x15a\x0BoW\x80`\x1F\x10a\x0BDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BoV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BRW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xD7V[PPPP\x90P\x91\x90PV[`\0\x81\x81R`\x0E` R`@\x81 a\x0B\xA5\x90a\x18zV[\x92\x91PPV[`\0\x81\x81R`\t` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82Ta\x01\0\x83\x90\n\x90\x04`\xFF\x16\x15\x15\x81R` `\x01\x92\x83\x01\x81\x81\x04\x94\x85\x01\x94\x90\x93\x03\x90\x92\x02\x91\x01\x80\x84\x11a\x0B\xF0W\x90P[PPPPP\x90P\x91\x90PV[`\x08` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x0CIW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[a\x0C\xB5`@Q` \x01a\x0Cw\x90a'\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x18\x84V[`\0\x82\x81R`\x0B` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[PPPV[`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[`\0\x82\x81R`\x0E` R`@\x81 a\rR\x90\x83a\x198V[\x93\x92PPPV[`\0a\rm`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x84\x81R` \x81\x90R`@\x81 T\x90\x84\x12\x80\x15a\r\x9AWP\x80a\r\x98a\r\x93\x86a'XV[a\x19PV[\x11[\x15a\r\xC2W\x82`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\r\xB9\x91\x90a%\x1AV[`@Q\x80\x91\x03\x90\xFD[`\0a\r\xCE\x82\x86a\x19\xA6V[`\0\x87\x81R` \x81\x90R`@\x90 \x81\x90U\x92PPP\x93\x92PPPV[`\0a\r\xFE`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 Ta\x0E\x18\x90\x84\x90a'tV[`\0\x85\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x92\x91PPV[a\x0ED`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\t` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1EaV[`\0\x81\x81R`\r` R`@\x81 a\x0B\xA5\x90a\x18zV[`\0a\x0E\x8E`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 T\x90a\x0E\xA8\x84\x83a'tV[`\0\x86\x81R` \x81\x90R`@\x90 \x81\x90U\x92PPP\x92\x91PPV[`\0a\x0E\xD7`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x01` R`@\x90\x91 \x81\x90U\x90V[a\x0E\xFF`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R` \x81\x90R`@\x81 UV[a\x0F\"`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x07` R`@\x81 a\x0F9\x91a\x1F\x01V[PV[`\0a\x0FP`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x05` R`@\x90\x91 \x81\x90U\x90V[`\0\x81\x81R`\x08` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0F\xA8WPPPPP\x90P\x91\x90PV[a\x0F\xE3`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x06` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[`\x07` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PPT\x81V[a\x10E`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\r` R`@\x90 a\x0C\xD4\x90\x82a\x19\xDDV[``a\x10q`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x04` R`@\x90 a\x10\x89\x83\x82a'\xD5V[P\x90\x92\x91PPV[`\0a\x10\xA5`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x01` R`@\x81 Ta\x10\xBF\x90\x84\x90a(\x93V[`\0\x85\x81R`\x01` R`@\x90 \x81\x90U\x91PP\x92\x91PPV[`\0\x83\x81R`\x0E` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x19\xF2V[\x94\x93PPPPV[`\t` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11\x19W`\0\x80\xFD[\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[`\0\x81\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[a\x11\xB2`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\n` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1F\x1FV[`\0a\x11\xE5`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 T\x90\x83\x12\x80\x15a\x12\rWP\x80a\x12\x0Ba\r\x93\x85a'XV[\x11[\x15a\x12*WPP`\0\x82\x81R` \x81\x90R`@\x81 \x81\x90Ua\x0B\xA5V[`\0a\x0E\xA8\x82\x85a\x19\xA6V[`\0\x82\x81R`\x0C` R`@\x81 a\rR\x90\x83a\x198V[a\x12``@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0E` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xCCV[`\0\x81\x81R`\x04` R`@\x90 \x80T``\x91\x90a\x12\x95\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xC1\x90a&\xE4V[\x80\x15a\x0C!W\x80`\x1F\x10a\x12\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xF1WP\x93\x96\x95PPPPPPV[a\x13\"`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0C` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xCCV[a\x13L`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[a\x13|`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90UV[a\x13\xA6`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x07` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1E\x1AV[`\0a\x13\xD9`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x03` R`@\x90\x91 \x80T`\xFF\x19\x16\x82\x15\x15\x17\x90U\x90V[a\x14\n`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0E` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xD8V[a\x144`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\r` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xE4V[`\n` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x14hW`\0\x80\xFD[\x90`\0R` `\0 \x01`\0\x91P\x91PP\x80Ta\x14\x84\x90a&\xE4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xB0\x90a&\xE4V[\x80\x15a\x14\xFDW\x80`\x1F\x10a\x14\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[a\x15\x17`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x06` R`@\x81 a\x0F9\x91a\x1F\x01V[`\x0B` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[a\x15\\`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x0B` R`@\x81 a\x0F9\x91a\x1F\x01V[a\x15\x85`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x08` R`@\x81 a\x0F9\x91a\x1F\x01V[`\x06` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\x1EW`\0\x80\xFD[`\0\x82\x81R`\r` R`@\x81 a\rR\x90\x83a\x1A\xF9V[a\x15\xE2`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x0C` R`@\x90 a\x0C\xD4\x90\x82a\x1A\xD8V[`\0a\x16\x0E`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R`\x02` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90U\x90V[`\0a\x16N`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R`\x01` R`@\x81 Ta\x10\xBF\x90\x84\x90a(\xB3V[a\x16z`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\x04` R`@\x81 a\x0F9\x91a\x1FqV[a\x16\xA3`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x05` R`@\x81 UV[`\0\x81\x81R`\x0B` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x0C!W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\x1BWPPPPP\x90P\x91\x90PV[a\x17&`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\n` R`@\x81 a\x0F9\x91a\x1F\xABV[`\0a\x17Q`@Q` \x01a\x0Cw\x90a'\x1EV[P`\0\x91\x82R` \x82\x90R`@\x90\x91 \x81\x90U\x90V[a\x17y`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x90\x81R`\x01` R`@\x81 UV[`\0\x83\x81R`\r` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x1B\x1BV[`\0a\x17\xBA`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x83\x81R` \x81\x90R`@\x81 Ta\x0E\x18\x90\x84\x90a(\xDBV[a\x17\xE6`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x82\x81R`\x08` \x90\x81R`@\x90\x91 \x82Qa\x0C\xD4\x92\x84\x01\x90a\x1F\xC9V[`\0\x83\x81R`\x0C` R`@\x90 ``\x90a\x10\xF5\x90\x84\x84a\x1B\xD4V[`\x04` R`\0\x90\x81R`@\x90 \x80Ta\x14\x84\x90a&\xE4V[`\0\x81\x81R`\x0C` R`@\x81 a\x0B\xA5\x90a\x18zV[a\x18c`@Q` \x01a\x0Cw\x90a'\x1EV[`\0\x81\x81R`\t` R`@\x81 a\x0F9\x91a \x1EV[`\0a\x0B\xA5\x82T\x90V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x13\x91\x90a(\xEEV[a\x194W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\r\xB9\x92\x91\x90a)\x0BV[PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\rRV[`\0\x80\x82\x12\x15a\x19\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FSafeCast: value must be positive`D\x82\x01R`d\x01a\r\xB9V[P\x90V[`\0\x80\x82\x13\x15a\x19\xCAWa\x19\xB9\x82a\x1C\x80V[a\x19\xC3\x90\x84a'tV[\x90Pa\x0B\xA5V[a\x19\xD3\x82a\x1C\x80V[a\rR\x90\x84a(\xDBV[`\0a\rR\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1C\x93V[``a\x19\xFD\x84a\x18zV[\x83\x10a\x1A\x18WP`@\x80Q`\0\x81R` \x81\x01\x90\x91Ra\rRV[`\0a\x1A#\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1A1W\x80\x92P[`\0a\x1A=\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1ATWa\x1ATa!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1A\x94\x87\x82a\x1D\x8DV[\x82a\x1A\x9F\x88\x84a(\xDBV[\x81Q\x81\x10a\x1A\xAFWa\x1A\xAFa)/V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\x82V[P\x95\x94PPPPPV[`\0a\rR\x83\x83a\x1C\x93V[`\0a\rR\x83\x83a\x1D\x99V[`\0a\rR\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1D\x99V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\rRV[```\0a\x1B(\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1B6W\x80\x92P[`\0a\x1BB\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BYWa\x1BYa!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x82W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1B\x99\x87\x82a\x1D\x8DV[\x82a\x1B\xA4\x88\x84a(\xDBV[\x81Q\x81\x10a\x1B\xB4Wa\x1B\xB4a)/V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1B\x87V[```\0a\x1B\xE1\x85a\x18zV[\x90P\x80\x83\x11\x15a\x1B\xEFW\x80\x92P[`\0a\x1B\xFB\x85\x85a(\xDBV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x12Wa\x1C\x12a!\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C;W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x1A\xC2Wa\x1CR\x87\x82a\x1D\xE8V[\x82a\x1C]\x88\x84a(\xDBV[\x81Q\x81\x10a\x1CmWa\x1Cma)/V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1C@V[`\0\x80\x82\x12\x15a\x19\xA2W\x81`\0\x03a\x0B\xA5V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1D|W`\0a\x1C\xB7`\x01\x83a(\xDBV[\x85T\x90\x91P`\0\x90a\x1C\xCB\x90`\x01\x90a(\xDBV[\x90P\x81\x81\x14a\x1D0W`\0\x86`\0\x01\x82\x81T\x81\x10a\x1C\xEBWa\x1C\xEBa)/V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x1D\x0EWa\x1D\x0Ea)/V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1DAWa\x1DAa)EV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x0B\xA5V[`\0\x91PPa\x0B\xA5V[P\x92\x91PPV[`\0a\rR\x83\x83a\x1D\xF0V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1D\xE0WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x0B\xA5V[P`\0a\x0B\xA5V[`\0a\rR\x83\x83[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1E\x07Wa\x1E\x07a)/V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1EUW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1E:V[Pa\x19\xA2\x92\x91Pa ?V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01`\0[\x83\x82\x11\x15a\x1E\xC7W\x83Q\x83\x82a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x92` \x01\x92`\x01\x01` \x81`\0\x01\x04\x92\x83\x01\x92`\x01\x03\x02a\x1E\x8AV[\x80\x15a\x1E\xF4W\x82\x81a\x01\0\n\x81T\x90`\xFF\x02\x19\x16\x90U`\x01\x01` \x81`\0\x01\x04\x92\x83\x01\x92`\x01\x03\x02a\x1E\xC7V[PPa\x19\xA2\x92\x91Pa ?V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a ?V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1FeW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1FeW\x82Q\x82\x90a\x1FU\x90\x82a'\xD5V[P\x91` \x01\x91\x90`\x01\x01\x90a\x1F?V[Pa\x19\xA2\x92\x91Pa TV[P\x80Ta\x1F}\x90a&\xE4V[`\0\x82U\x80`\x1F\x10a\x1F\x8DWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a ?V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90a TV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x1EUW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1EUW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a\x1F\xE9V[P\x80T`\0\x82U`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x0F9\x91\x90[[\x80\x82\x11\x15a\x19\xA2W`\0\x81U`\x01\x01a @V[\x80\x82\x11\x15a\x19\xA2W`\0a h\x82\x82a\x1FqV[P`\x01\x01a TV[`\0` \x82\x84\x03\x12\x15a \x83W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a \xB0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a \x94V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a!)W`?\x19\x87\x86\x03\x01\x84Ra!\x14\x85\x83Qa \x8AV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a \xF8V[P\x92\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q\x15\x15\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a!OV[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a!\x8DW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!\xDAWa!\xDAa!\x9CV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a!\xFBWa!\xFBa!\x9CV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\"\x18W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"FW`\0\x80\xFD[\x805a\"Ya\"T\x82a!\xE2V[a!\xB2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\"{W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\"\x82V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"\xC5V[`\0\x82`\x1F\x83\x01\x12a\"\xF4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#\rWa#\ra!\x9CV[a# `\x1F\x82\x01`\x1F\x19\x16` \x01a!\xB2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a#5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a#gW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x8BW`\0\x80\xFD[a#\x97\x86\x82\x87\x01a\"\xE3V[\x91PP\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x0F9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a#\xC2W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\xF0W`\0\x80\xFD[\x805a#\xFEa\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a$ W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DW\x835a$:\x81a#\xA1V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a$'V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!oW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$eV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xA3W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xBBW`\0\x80\xFD[\x825\x91Pa$\xCB` \x84\x01a$\x8CV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xE7W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x04W`\0\x80\xFD[a%\x10\x85\x82\x86\x01a\"\xE3V[\x91PP\x92P\x92\x90PV[` \x81R`\0a\rR` \x83\x01\x84a \x8AV[`\0\x80`\0``\x84\x86\x03\x12\x15a%BW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a%lW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x89W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a%\x9AW`\0\x80\xFD[\x805a%\xA8a\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a%\xCAW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a&\x0BW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xEDW`\0\x80\xFD[a%\xFC\x8A` \x83\x89\x01\x01a\"\xE3V[\x84RP` \x92\x83\x01\x92\x01a%\xCFV[P\x80\x94PPPPP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&-W`\0\x80\xFD[\x825\x91P` \x83\x015a&?\x81a#\xA1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a&]W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&zW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\x8BW`\0\x80\xFD[\x805a&\x99a\"T\x82a!\xE2V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a&\xBBW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\"\x9DWa&\xD3\x84a$\x8CV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a&\xC2V[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xF8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a'mWa'ma'BV[P`\0\x03\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0B\xA5Wa\x0B\xA5a'BV[`\x1F\x82\x11\x15a\x0C\xD4W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a'\xAEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'\xCEW`\0\x81U`\x01\x01a'\xBAV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xEEWa'\xEEa!\x9CV[a(\x02\x81a'\xFC\x84Ta&\xE4V[\x84a'\x87V[` `\x1F\x82\x11`\x01\x81\x14a(6W`\0\x83\x15a(\x1EWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua'\xCEV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a(fW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a(FV[P\x84\x82\x10\x15a(\x84W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D\x86Wa\x1D\x86a'BV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xD3Wa(\xD3a'BV[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x0B\xA5Wa\x0B\xA5a'BV[`\0` \x82\x84\x03\x12\x15a)\0W`\0\x80\xFD[\x81Qa\rR\x81a#\xA1V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x10\xF5\x90\x83\x01\x84a \x8AV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xEA~\xA7Y\xCB\xAE\xEB\x92D\xDDQ\x86\xC3-\xA0\xC9\x0F\xF7,R\x87\xE5\x16\xD2y\x1A\x19b\x13\xDD\x02\xC7dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static DATASTORE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DataStore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DataStore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DataStore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DataStore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DataStore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DataStore)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DataStore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DATASTORE_ABI.clone(),
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
                DATASTORE_ABI.clone(),
                DATASTORE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addAddress` (0xb348e639) function
        pub fn add_address(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 72, 230, 57], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBytes32` (0xc80f4c62) function
        pub fn add_bytes_32(
            &self,
            set_key: [u8; 32],
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 15, 76, 98], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addUint` (0xadb353dc) function
        pub fn add_uint(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 179, 83, 220], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addressArrayValues` (0x22f87464) function
        pub fn address_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([34, 248, 116, 100], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addressValues` (0x22538dae) function
        pub fn address_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([34, 83, 141, 174], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyBoundedDeltaToUint` (0x8ca498b0) function
        pub fn apply_bounded_delta_to_uint(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 164, 152, 176], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyDeltaToInt` (0xe4e36c4e) function
        pub fn apply_delta_to_int(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 227, 108, 78], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyDeltaToUint` (0x32f85bbf) function
        pub fn apply_delta_to_uint_with_key_and_value(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
            error_message: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([50, 248, 91, 191], (key, value, error_message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyDeltaToUint` (0x3dbacd1a) function
        pub fn apply_delta_to_uint(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([61, 186, 205, 26], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `boolArrayValues` (0x80aacdcd) function
        pub fn bool_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([128, 170, 205, 205], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `boolValues` (0x44a242b1) function
        pub fn bool_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 162, 66, 177], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bytes32ArrayValues` (0xbf498dd3) function
        pub fn bytes_32_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([191, 73, 141, 211], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bytes32Values` (0xd52852af) function
        pub fn bytes_32_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 40, 82, 175], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `containsAddress` (0xc769d1a1) function
        pub fn contains_address(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 105, 209, 161], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `containsBytes32` (0x91d4403c) function
        pub fn contains_bytes_32(
            &self,
            set_key: [u8; 32],
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 212, 64, 60], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `containsUint` (0x310b8882) function
        pub fn contains_uint(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 11, 136, 130], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decrementInt` (0x6fae54f0) function
        pub fn decrement_int(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([111, 174, 84, 240], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decrementUint` (0xe98aabc1) function
        pub fn decrement_uint(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 138, 171, 193], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddress` (0x21f8a721) function
        pub fn get_address(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 248, 167, 33], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressArray` (0x5948f733) function
        pub fn get_address_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([89, 72, 247, 51], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressCount` (0x35ea8059) function
        pub fn get_address_count(
            &self,
            set_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 234, 128, 89], set_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressValuesAt` (0xe7e4148e) function
        pub fn get_address_values_at(
            &self,
            set_key: [u8; 32],
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([231, 228, 20, 142], (set_key, start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBool` (0x7ae1cfca) function
        pub fn get_bool(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 225, 207, 202], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBoolArray` (0x116bb929) function
        pub fn get_bool_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([17, 107, 185, 41], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBytes32` (0xa6ed563e) function
        pub fn get_bytes_32(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 237, 86, 62], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBytes32Array` (0xdd031997) function
        pub fn get_bytes_32_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([221, 3, 25, 151], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBytes32Count` (0xf3903b9f) function
        pub fn get_bytes_32_count(
            &self,
            set_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 144, 59, 159], set_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBytes32ValuesAt` (0xf069052a) function
        pub fn get_bytes_32_values_at(
            &self,
            set_key: [u8; 32],
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([240, 105, 5, 42], (set_key, start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInt` (0xdc97d962) function
        pub fn get_int(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([220, 151, 217, 98], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getIntArray` (0x2d2899b6) function
        pub fn get_int_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([45, 40, 153, 182], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getString` (0x986e791a) function
        pub fn get_string(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([152, 110, 121, 26], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStringArray` (0x01677da2) function
        pub fn get_string_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([1, 103, 125, 162], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUint` (0xbd02d0f5) function
        pub fn get_uint(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 2, 208, 245], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUintArray` (0x86ac6bdf) function
        pub fn get_uint_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([134, 172, 107, 223], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUintCount` (0x065f21a7) function
        pub fn get_uint_count(
            &self,
            set_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([6, 95, 33, 167], set_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUintValuesAt` (0x7026d42c) function
        pub fn get_uint_values_at(
            &self,
            set_key: [u8; 32],
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([112, 38, 212, 44], (set_key, start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementInt` (0xcbb093dd) function
        pub fn increment_int(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([203, 176, 147, 221], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementUint` (0x340dbab3) function
        pub fn increment_uint(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 13, 186, 179], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intArrayValues` (0x6339734d) function
        pub fn int_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([99, 57, 115, 77], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intValues` (0x743df325) function
        pub fn int_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([116, 61, 243, 37], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAddress` (0x69721d41) function
        pub fn remove_address_with_set_key(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 114, 29, 65], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAddress` (0x9faf6fb6) function
        pub fn remove_address(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 175, 111, 182], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAddressArray` (0xc1dc9182) function
        pub fn remove_address_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 220, 145, 130], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBool` (0x9fe7ac12) function
        pub fn remove_bool(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 231, 172, 18], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBoolArray` (0xf51fc0d9) function
        pub fn remove_bool_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 31, 192, 217], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBytes32` (0x9921c3cc) function
        pub fn remove_bytes_32_with_set_key(
            &self,
            set_key: [u8; 32],
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 33, 195, 204], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBytes32` (0xcf6a8722) function
        pub fn remove_bytes_32(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 106, 135, 34], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBytes32Array` (0xbf7f035a) function
        pub fn remove_bytes_32_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 127, 3, 90], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeInt` (0xe62461ce) function
        pub fn remove_int(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 36, 97, 206], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeIntArray` (0x499ea50e) function
        pub fn remove_int_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 158, 165, 14], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeString` (0xcc50eadd) function
        pub fn remove_string(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 80, 234, 221], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStringArray` (0xe208a70d) function
        pub fn remove_string_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 8, 167, 13], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeUint` (0x42c3bd96) function
        pub fn remove_uint(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 195, 189, 150], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeUint` (0x93266f9a) function
        pub fn remove_uint_with_set_key(
            &self,
            set_key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 38, 111, 154], (set_key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeUintArray` (0xbe43caa3) function
        pub fn remove_uint_array(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 67, 202, 163], key)
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
        ///Calls the contract's `setAddress` (0xca446dd9) function
        pub fn set_address(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([202, 68, 109, 217], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddressArray` (0xec672cf6) function
        pub fn set_address_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 103, 44, 246], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBool` (0xabfdcced) function
        pub fn set_bool(
            &self,
            key: [u8; 32],
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([171, 253, 204, 237], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBoolArray` (0x35d4d407) function
        pub fn set_bool_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 212, 212, 7], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBytes32` (0x4e91db08) function
        pub fn set_bytes_32(
            &self,
            key: [u8; 32],
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([78, 145, 219, 8], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBytes32Array` (0x26004846) function
        pub fn set_bytes_32_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 0, 72, 70], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInt` (0x3e49bed0) function
        pub fn set_int(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([62, 73, 190, 208], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIntArray` (0xa9fcf76b) function
        pub fn set_int_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<::ethers::core::types::I256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 252, 247, 107], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setString` (0x6e899550) function
        pub fn set_string(
            &self,
            key: [u8; 32],
            value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([110, 137, 149, 80], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStringArray` (0x88021a72) function
        pub fn set_string_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<::std::string::String>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 2, 26, 114], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUint` (0xe2a4853a) function
        pub fn set_uint(
            &self,
            key: [u8; 32],
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([226, 164, 133, 58], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUintArray` (0x5eb07dbd) function
        pub fn set_uint_array(
            &self,
            key: [u8; 32],
            value: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 176, 125, 189], (key, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stringArrayValues` (0xb8320a08) function
        pub fn string_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([184, 50, 10, 8], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stringValues` (0xf15caeac) function
        pub fn string_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([241, 92, 174, 172], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uintArrayValues` (0xc4f00fde) function
        pub fn uint_array_values(
            &self,
            p0: [u8; 32],
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 240, 15, 222], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uintValues` (0xd38eebc7) function
        pub fn uint_values(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([211, 142, 235, 199], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DataStore<M> {
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
    ///Container type for all input parameters for the `addAddress` function with signature `addAddress(bytes32,address)` and selector `0xb348e639`
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
    #[ethcall(name = "addAddress", abi = "addAddress(bytes32,address)")]
    pub struct AddAddressCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addBytes32` function with signature `addBytes32(bytes32,bytes32)` and selector `0xc80f4c62`
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
    #[ethcall(name = "addBytes32", abi = "addBytes32(bytes32,bytes32)")]
    pub struct AddBytes32Call {
        pub set_key: [u8; 32],
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `addUint` function with signature `addUint(bytes32,uint256)` and selector `0xadb353dc`
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
    #[ethcall(name = "addUint", abi = "addUint(bytes32,uint256)")]
    pub struct AddUintCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addressArrayValues` function with signature `addressArrayValues(bytes32,uint256)` and selector `0x22f87464`
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
    #[ethcall(name = "addressArrayValues", abi = "addressArrayValues(bytes32,uint256)")]
    pub struct AddressArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `addressValues` function with signature `addressValues(bytes32)` and selector `0x22538dae`
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
    #[ethcall(name = "addressValues", abi = "addressValues(bytes32)")]
    pub struct AddressValuesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `applyBoundedDeltaToUint` function with signature `applyBoundedDeltaToUint(bytes32,int256)` and selector `0x8ca498b0`
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
        name = "applyBoundedDeltaToUint",
        abi = "applyBoundedDeltaToUint(bytes32,int256)"
    )]
    pub struct ApplyBoundedDeltaToUintCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `applyDeltaToInt` function with signature `applyDeltaToInt(bytes32,int256)` and selector `0xe4e36c4e`
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
    #[ethcall(name = "applyDeltaToInt", abi = "applyDeltaToInt(bytes32,int256)")]
    pub struct ApplyDeltaToIntCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `applyDeltaToUint` function with signature `applyDeltaToUint(bytes32,int256,string)` and selector `0x32f85bbf`
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
        name = "applyDeltaToUint",
        abi = "applyDeltaToUint(bytes32,int256,string)"
    )]
    pub struct ApplyDeltaToUintWithKeyAndValueCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
        pub error_message: ::std::string::String,
    }
    ///Container type for all input parameters for the `applyDeltaToUint` function with signature `applyDeltaToUint(bytes32,uint256)` and selector `0x3dbacd1a`
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
    #[ethcall(name = "applyDeltaToUint", abi = "applyDeltaToUint(bytes32,uint256)")]
    pub struct ApplyDeltaToUintCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `boolArrayValues` function with signature `boolArrayValues(bytes32,uint256)` and selector `0x80aacdcd`
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
    #[ethcall(name = "boolArrayValues", abi = "boolArrayValues(bytes32,uint256)")]
    pub struct BoolArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `boolValues` function with signature `boolValues(bytes32)` and selector `0x44a242b1`
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
    #[ethcall(name = "boolValues", abi = "boolValues(bytes32)")]
    pub struct BoolValuesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `bytes32ArrayValues` function with signature `bytes32ArrayValues(bytes32,uint256)` and selector `0xbf498dd3`
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
    #[ethcall(name = "bytes32ArrayValues", abi = "bytes32ArrayValues(bytes32,uint256)")]
    pub struct Bytes32ArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `bytes32Values` function with signature `bytes32Values(bytes32)` and selector `0xd52852af`
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
    #[ethcall(name = "bytes32Values", abi = "bytes32Values(bytes32)")]
    pub struct Bytes32ValuesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `containsAddress` function with signature `containsAddress(bytes32,address)` and selector `0xc769d1a1`
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
    #[ethcall(name = "containsAddress", abi = "containsAddress(bytes32,address)")]
    pub struct ContainsAddressCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `containsBytes32` function with signature `containsBytes32(bytes32,bytes32)` and selector `0x91d4403c`
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
    #[ethcall(name = "containsBytes32", abi = "containsBytes32(bytes32,bytes32)")]
    pub struct ContainsBytes32Call {
        pub set_key: [u8; 32],
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `containsUint` function with signature `containsUint(bytes32,uint256)` and selector `0x310b8882`
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
    #[ethcall(name = "containsUint", abi = "containsUint(bytes32,uint256)")]
    pub struct ContainsUintCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decrementInt` function with signature `decrementInt(bytes32,int256)` and selector `0x6fae54f0`
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
    #[ethcall(name = "decrementInt", abi = "decrementInt(bytes32,int256)")]
    pub struct DecrementIntCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `decrementUint` function with signature `decrementUint(bytes32,uint256)` and selector `0xe98aabc1`
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
    #[ethcall(name = "decrementUint", abi = "decrementUint(bytes32,uint256)")]
    pub struct DecrementUintCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
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
    #[ethcall(name = "getAddress", abi = "getAddress(bytes32)")]
    pub struct GetAddressCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getAddressArray` function with signature `getAddressArray(bytes32)` and selector `0x5948f733`
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
    #[ethcall(name = "getAddressArray", abi = "getAddressArray(bytes32)")]
    pub struct GetAddressArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getAddressCount` function with signature `getAddressCount(bytes32)` and selector `0x35ea8059`
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
    #[ethcall(name = "getAddressCount", abi = "getAddressCount(bytes32)")]
    pub struct GetAddressCountCall {
        pub set_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getAddressValuesAt` function with signature `getAddressValuesAt(bytes32,uint256,uint256)` and selector `0xe7e4148e`
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
        name = "getAddressValuesAt",
        abi = "getAddressValuesAt(bytes32,uint256,uint256)"
    )]
    pub struct GetAddressValuesAtCall {
        pub set_key: [u8; 32],
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBool` function with signature `getBool(bytes32)` and selector `0x7ae1cfca`
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
    #[ethcall(name = "getBool", abi = "getBool(bytes32)")]
    pub struct GetBoolCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBoolArray` function with signature `getBoolArray(bytes32)` and selector `0x116bb929`
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
    #[ethcall(name = "getBoolArray", abi = "getBoolArray(bytes32)")]
    pub struct GetBoolArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBytes32` function with signature `getBytes32(bytes32)` and selector `0xa6ed563e`
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
    #[ethcall(name = "getBytes32", abi = "getBytes32(bytes32)")]
    pub struct GetBytes32Call {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBytes32Array` function with signature `getBytes32Array(bytes32)` and selector `0xdd031997`
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
    #[ethcall(name = "getBytes32Array", abi = "getBytes32Array(bytes32)")]
    pub struct GetBytes32ArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBytes32Count` function with signature `getBytes32Count(bytes32)` and selector `0xf3903b9f`
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
    #[ethcall(name = "getBytes32Count", abi = "getBytes32Count(bytes32)")]
    pub struct GetBytes32CountCall {
        pub set_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBytes32ValuesAt` function with signature `getBytes32ValuesAt(bytes32,uint256,uint256)` and selector `0xf069052a`
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
        name = "getBytes32ValuesAt",
        abi = "getBytes32ValuesAt(bytes32,uint256,uint256)"
    )]
    pub struct GetBytes32ValuesAtCall {
        pub set_key: [u8; 32],
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInt` function with signature `getInt(bytes32)` and selector `0xdc97d962`
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
    #[ethcall(name = "getInt", abi = "getInt(bytes32)")]
    pub struct GetIntCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getIntArray` function with signature `getIntArray(bytes32)` and selector `0x2d2899b6`
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
    #[ethcall(name = "getIntArray", abi = "getIntArray(bytes32)")]
    pub struct GetIntArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getString` function with signature `getString(bytes32)` and selector `0x986e791a`
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
    #[ethcall(name = "getString", abi = "getString(bytes32)")]
    pub struct GetStringCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getStringArray` function with signature `getStringArray(bytes32)` and selector `0x01677da2`
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
    #[ethcall(name = "getStringArray", abi = "getStringArray(bytes32)")]
    pub struct GetStringArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUint` function with signature `getUint(bytes32)` and selector `0xbd02d0f5`
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
    #[ethcall(name = "getUint", abi = "getUint(bytes32)")]
    pub struct GetUintCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUintArray` function with signature `getUintArray(bytes32)` and selector `0x86ac6bdf`
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
    #[ethcall(name = "getUintArray", abi = "getUintArray(bytes32)")]
    pub struct GetUintArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUintCount` function with signature `getUintCount(bytes32)` and selector `0x065f21a7`
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
    #[ethcall(name = "getUintCount", abi = "getUintCount(bytes32)")]
    pub struct GetUintCountCall {
        pub set_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUintValuesAt` function with signature `getUintValuesAt(bytes32,uint256,uint256)` and selector `0x7026d42c`
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
        name = "getUintValuesAt",
        abi = "getUintValuesAt(bytes32,uint256,uint256)"
    )]
    pub struct GetUintValuesAtCall {
        pub set_key: [u8; 32],
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `incrementInt` function with signature `incrementInt(bytes32,int256)` and selector `0xcbb093dd`
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
    #[ethcall(name = "incrementInt", abi = "incrementInt(bytes32,int256)")]
    pub struct IncrementIntCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `incrementUint` function with signature `incrementUint(bytes32,uint256)` and selector `0x340dbab3`
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
    #[ethcall(name = "incrementUint", abi = "incrementUint(bytes32,uint256)")]
    pub struct IncrementUintCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intArrayValues` function with signature `intArrayValues(bytes32,uint256)` and selector `0x6339734d`
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
    #[ethcall(name = "intArrayValues", abi = "intArrayValues(bytes32,uint256)")]
    pub struct IntArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `intValues` function with signature `intValues(bytes32)` and selector `0x743df325`
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
    #[ethcall(name = "intValues", abi = "intValues(bytes32)")]
    pub struct IntValuesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `removeAddress` function with signature `removeAddress(bytes32,address)` and selector `0x69721d41`
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
    #[ethcall(name = "removeAddress", abi = "removeAddress(bytes32,address)")]
    pub struct RemoveAddressWithSetKeyCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeAddress` function with signature `removeAddress(bytes32)` and selector `0x9faf6fb6`
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
    #[ethcall(name = "removeAddress", abi = "removeAddress(bytes32)")]
    pub struct RemoveAddressCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeAddressArray` function with signature `removeAddressArray(bytes32)` and selector `0xc1dc9182`
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
    #[ethcall(name = "removeAddressArray", abi = "removeAddressArray(bytes32)")]
    pub struct RemoveAddressArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeBool` function with signature `removeBool(bytes32)` and selector `0x9fe7ac12`
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
    #[ethcall(name = "removeBool", abi = "removeBool(bytes32)")]
    pub struct RemoveBoolCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeBoolArray` function with signature `removeBoolArray(bytes32)` and selector `0xf51fc0d9`
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
    #[ethcall(name = "removeBoolArray", abi = "removeBoolArray(bytes32)")]
    pub struct RemoveBoolArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeBytes32` function with signature `removeBytes32(bytes32,bytes32)` and selector `0x9921c3cc`
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
    #[ethcall(name = "removeBytes32", abi = "removeBytes32(bytes32,bytes32)")]
    pub struct RemoveBytes32WithSetKeyCall {
        pub set_key: [u8; 32],
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `removeBytes32` function with signature `removeBytes32(bytes32)` and selector `0xcf6a8722`
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
    #[ethcall(name = "removeBytes32", abi = "removeBytes32(bytes32)")]
    pub struct RemoveBytes32Call {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeBytes32Array` function with signature `removeBytes32Array(bytes32)` and selector `0xbf7f035a`
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
    #[ethcall(name = "removeBytes32Array", abi = "removeBytes32Array(bytes32)")]
    pub struct RemoveBytes32ArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeInt` function with signature `removeInt(bytes32)` and selector `0xe62461ce`
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
    #[ethcall(name = "removeInt", abi = "removeInt(bytes32)")]
    pub struct RemoveIntCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeIntArray` function with signature `removeIntArray(bytes32)` and selector `0x499ea50e`
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
    #[ethcall(name = "removeIntArray", abi = "removeIntArray(bytes32)")]
    pub struct RemoveIntArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeString` function with signature `removeString(bytes32)` and selector `0xcc50eadd`
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
    #[ethcall(name = "removeString", abi = "removeString(bytes32)")]
    pub struct RemoveStringCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeStringArray` function with signature `removeStringArray(bytes32)` and selector `0xe208a70d`
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
    #[ethcall(name = "removeStringArray", abi = "removeStringArray(bytes32)")]
    pub struct RemoveStringArrayCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeUint` function with signature `removeUint(bytes32)` and selector `0x42c3bd96`
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
    #[ethcall(name = "removeUint", abi = "removeUint(bytes32)")]
    pub struct RemoveUintCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `removeUint` function with signature `removeUint(bytes32,uint256)` and selector `0x93266f9a`
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
    #[ethcall(name = "removeUint", abi = "removeUint(bytes32,uint256)")]
    pub struct RemoveUintWithSetKeyCall {
        pub set_key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeUintArray` function with signature `removeUintArray(bytes32)` and selector `0xbe43caa3`
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
    #[ethcall(name = "removeUintArray", abi = "removeUintArray(bytes32)")]
    pub struct RemoveUintArrayCall {
        pub key: [u8; 32],
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
    ///Container type for all input parameters for the `setAddress` function with signature `setAddress(bytes32,address)` and selector `0xca446dd9`
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
    #[ethcall(name = "setAddress", abi = "setAddress(bytes32,address)")]
    pub struct SetAddressCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAddressArray` function with signature `setAddressArray(bytes32,address[])` and selector `0xec672cf6`
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
    #[ethcall(name = "setAddressArray", abi = "setAddressArray(bytes32,address[])")]
    pub struct SetAddressArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setBool` function with signature `setBool(bytes32,bool)` and selector `0xabfdcced`
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
    #[ethcall(name = "setBool", abi = "setBool(bytes32,bool)")]
    pub struct SetBoolCall {
        pub key: [u8; 32],
        pub value: bool,
    }
    ///Container type for all input parameters for the `setBoolArray` function with signature `setBoolArray(bytes32,bool[])` and selector `0x35d4d407`
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
    #[ethcall(name = "setBoolArray", abi = "setBoolArray(bytes32,bool[])")]
    pub struct SetBoolArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `setBytes32` function with signature `setBytes32(bytes32,bytes32)` and selector `0x4e91db08`
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
    #[ethcall(name = "setBytes32", abi = "setBytes32(bytes32,bytes32)")]
    pub struct SetBytes32Call {
        pub key: [u8; 32],
        pub value: [u8; 32],
    }
    ///Container type for all input parameters for the `setBytes32Array` function with signature `setBytes32Array(bytes32,bytes32[])` and selector `0x26004846`
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
    #[ethcall(name = "setBytes32Array", abi = "setBytes32Array(bytes32,bytes32[])")]
    pub struct SetBytes32ArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `setInt` function with signature `setInt(bytes32,int256)` and selector `0x3e49bed0`
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
    #[ethcall(name = "setInt", abi = "setInt(bytes32,int256)")]
    pub struct SetIntCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `setIntArray` function with signature `setIntArray(bytes32,int256[])` and selector `0xa9fcf76b`
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
    #[ethcall(name = "setIntArray", abi = "setIntArray(bytes32,int256[])")]
    pub struct SetIntArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all input parameters for the `setString` function with signature `setString(bytes32,string)` and selector `0x6e899550`
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
    #[ethcall(name = "setString", abi = "setString(bytes32,string)")]
    pub struct SetStringCall {
        pub key: [u8; 32],
        pub value: ::std::string::String,
    }
    ///Container type for all input parameters for the `setStringArray` function with signature `setStringArray(bytes32,string[])` and selector `0x88021a72`
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
    #[ethcall(name = "setStringArray", abi = "setStringArray(bytes32,string[])")]
    pub struct SetStringArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all input parameters for the `setUint` function with signature `setUint(bytes32,uint256)` and selector `0xe2a4853a`
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
    #[ethcall(name = "setUint", abi = "setUint(bytes32,uint256)")]
    pub struct SetUintCall {
        pub key: [u8; 32],
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setUintArray` function with signature `setUintArray(bytes32,uint256[])` and selector `0x5eb07dbd`
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
    #[ethcall(name = "setUintArray", abi = "setUintArray(bytes32,uint256[])")]
    pub struct SetUintArrayCall {
        pub key: [u8; 32],
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `stringArrayValues` function with signature `stringArrayValues(bytes32,uint256)` and selector `0xb8320a08`
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
    #[ethcall(name = "stringArrayValues", abi = "stringArrayValues(bytes32,uint256)")]
    pub struct StringArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `stringValues` function with signature `stringValues(bytes32)` and selector `0xf15caeac`
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
    #[ethcall(name = "stringValues", abi = "stringValues(bytes32)")]
    pub struct StringValuesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `uintArrayValues` function with signature `uintArrayValues(bytes32,uint256)` and selector `0xc4f00fde`
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
    #[ethcall(name = "uintArrayValues", abi = "uintArrayValues(bytes32,uint256)")]
    pub struct UintArrayValuesCall(pub [u8; 32], pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `uintValues` function with signature `uintValues(bytes32)` and selector `0xd38eebc7`
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
    #[ethcall(name = "uintValues", abi = "uintValues(bytes32)")]
    pub struct UintValuesCall(pub [u8; 32]);
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
    pub enum DataStoreCalls {
        AddAddress(AddAddressCall),
        AddBytes32(AddBytes32Call),
        AddUint(AddUintCall),
        AddressArrayValues(AddressArrayValuesCall),
        AddressValues(AddressValuesCall),
        ApplyBoundedDeltaToUint(ApplyBoundedDeltaToUintCall),
        ApplyDeltaToInt(ApplyDeltaToIntCall),
        ApplyDeltaToUintWithKeyAndValue(ApplyDeltaToUintWithKeyAndValueCall),
        ApplyDeltaToUint(ApplyDeltaToUintCall),
        BoolArrayValues(BoolArrayValuesCall),
        BoolValues(BoolValuesCall),
        Bytes32ArrayValues(Bytes32ArrayValuesCall),
        Bytes32Values(Bytes32ValuesCall),
        ContainsAddress(ContainsAddressCall),
        ContainsBytes32(ContainsBytes32Call),
        ContainsUint(ContainsUintCall),
        DecrementInt(DecrementIntCall),
        DecrementUint(DecrementUintCall),
        GetAddress(GetAddressCall),
        GetAddressArray(GetAddressArrayCall),
        GetAddressCount(GetAddressCountCall),
        GetAddressValuesAt(GetAddressValuesAtCall),
        GetBool(GetBoolCall),
        GetBoolArray(GetBoolArrayCall),
        GetBytes32(GetBytes32Call),
        GetBytes32Array(GetBytes32ArrayCall),
        GetBytes32Count(GetBytes32CountCall),
        GetBytes32ValuesAt(GetBytes32ValuesAtCall),
        GetInt(GetIntCall),
        GetIntArray(GetIntArrayCall),
        GetString(GetStringCall),
        GetStringArray(GetStringArrayCall),
        GetUint(GetUintCall),
        GetUintArray(GetUintArrayCall),
        GetUintCount(GetUintCountCall),
        GetUintValuesAt(GetUintValuesAtCall),
        IncrementInt(IncrementIntCall),
        IncrementUint(IncrementUintCall),
        IntArrayValues(IntArrayValuesCall),
        IntValues(IntValuesCall),
        RemoveAddressWithSetKey(RemoveAddressWithSetKeyCall),
        RemoveAddress(RemoveAddressCall),
        RemoveAddressArray(RemoveAddressArrayCall),
        RemoveBool(RemoveBoolCall),
        RemoveBoolArray(RemoveBoolArrayCall),
        RemoveBytes32WithSetKey(RemoveBytes32WithSetKeyCall),
        RemoveBytes32(RemoveBytes32Call),
        RemoveBytes32Array(RemoveBytes32ArrayCall),
        RemoveInt(RemoveIntCall),
        RemoveIntArray(RemoveIntArrayCall),
        RemoveString(RemoveStringCall),
        RemoveStringArray(RemoveStringArrayCall),
        RemoveUint(RemoveUintCall),
        RemoveUintWithSetKey(RemoveUintWithSetKeyCall),
        RemoveUintArray(RemoveUintArrayCall),
        RoleStore(RoleStoreCall),
        SetAddress(SetAddressCall),
        SetAddressArray(SetAddressArrayCall),
        SetBool(SetBoolCall),
        SetBoolArray(SetBoolArrayCall),
        SetBytes32(SetBytes32Call),
        SetBytes32Array(SetBytes32ArrayCall),
        SetInt(SetIntCall),
        SetIntArray(SetIntArrayCall),
        SetString(SetStringCall),
        SetStringArray(SetStringArrayCall),
        SetUint(SetUintCall),
        SetUintArray(SetUintArrayCall),
        StringArrayValues(StringArrayValuesCall),
        StringValues(StringValuesCall),
        UintArrayValues(UintArrayValuesCall),
        UintValues(UintValuesCall),
    }
    impl ::ethers::core::abi::AbiDecode for DataStoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAddress(decoded));
            }
            if let Ok(decoded) = <AddBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddBytes32(decoded));
            }
            if let Ok(decoded) = <AddUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddUint(decoded));
            }
            if let Ok(decoded) = <AddressArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressArrayValues(decoded));
            }
            if let Ok(decoded) = <AddressValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressValues(decoded));
            }
            if let Ok(decoded) = <ApplyBoundedDeltaToUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApplyBoundedDeltaToUint(decoded));
            }
            if let Ok(decoded) = <ApplyDeltaToIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApplyDeltaToInt(decoded));
            }
            if let Ok(decoded) = <ApplyDeltaToUintWithKeyAndValueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApplyDeltaToUintWithKeyAndValue(decoded));
            }
            if let Ok(decoded) = <ApplyDeltaToUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApplyDeltaToUint(decoded));
            }
            if let Ok(decoded) = <BoolArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BoolArrayValues(decoded));
            }
            if let Ok(decoded) = <BoolValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BoolValues(decoded));
            }
            if let Ok(decoded) = <Bytes32ArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bytes32ArrayValues(decoded));
            }
            if let Ok(decoded) = <Bytes32ValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bytes32Values(decoded));
            }
            if let Ok(decoded) = <ContainsAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContainsAddress(decoded));
            }
            if let Ok(decoded) = <ContainsBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContainsBytes32(decoded));
            }
            if let Ok(decoded) = <ContainsUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContainsUint(decoded));
            }
            if let Ok(decoded) = <DecrementIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecrementInt(decoded));
            }
            if let Ok(decoded) = <DecrementUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecrementUint(decoded));
            }
            if let Ok(decoded) = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddress(decoded));
            }
            if let Ok(decoded) = <GetAddressArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddressArray(decoded));
            }
            if let Ok(decoded) = <GetAddressCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddressCount(decoded));
            }
            if let Ok(decoded) = <GetAddressValuesAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddressValuesAt(decoded));
            }
            if let Ok(decoded) = <GetBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBool(decoded));
            }
            if let Ok(decoded) = <GetBoolArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBoolArray(decoded));
            }
            if let Ok(decoded) = <GetBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes32(decoded));
            }
            if let Ok(decoded) = <GetBytes32ArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes32Array(decoded));
            }
            if let Ok(decoded) = <GetBytes32CountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes32Count(decoded));
            }
            if let Ok(decoded) = <GetBytes32ValuesAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes32ValuesAt(decoded));
            }
            if let Ok(decoded) = <GetIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInt(decoded));
            }
            if let Ok(decoded) = <GetIntArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetIntArray(decoded));
            }
            if let Ok(decoded) = <GetStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetString(decoded));
            }
            if let Ok(decoded) = <GetStringArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStringArray(decoded));
            }
            if let Ok(decoded) = <GetUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUint(decoded));
            }
            if let Ok(decoded) = <GetUintArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUintArray(decoded));
            }
            if let Ok(decoded) = <GetUintCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUintCount(decoded));
            }
            if let Ok(decoded) = <GetUintValuesAtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUintValuesAt(decoded));
            }
            if let Ok(decoded) = <IncrementIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncrementInt(decoded));
            }
            if let Ok(decoded) = <IncrementUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncrementUint(decoded));
            }
            if let Ok(decoded) = <IntArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntArrayValues(decoded));
            }
            if let Ok(decoded) = <IntValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntValues(decoded));
            }
            if let Ok(decoded) = <RemoveAddressWithSetKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAddressWithSetKey(decoded));
            }
            if let Ok(decoded) = <RemoveAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAddress(decoded));
            }
            if let Ok(decoded) = <RemoveAddressArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAddressArray(decoded));
            }
            if let Ok(decoded) = <RemoveBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBool(decoded));
            }
            if let Ok(decoded) = <RemoveBoolArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBoolArray(decoded));
            }
            if let Ok(decoded) = <RemoveBytes32WithSetKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBytes32WithSetKey(decoded));
            }
            if let Ok(decoded) = <RemoveBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBytes32(decoded));
            }
            if let Ok(decoded) = <RemoveBytes32ArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBytes32Array(decoded));
            }
            if let Ok(decoded) = <RemoveIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveInt(decoded));
            }
            if let Ok(decoded) = <RemoveIntArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveIntArray(decoded));
            }
            if let Ok(decoded) = <RemoveStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveString(decoded));
            }
            if let Ok(decoded) = <RemoveStringArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveStringArray(decoded));
            }
            if let Ok(decoded) = <RemoveUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveUint(decoded));
            }
            if let Ok(decoded) = <RemoveUintWithSetKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveUintWithSetKey(decoded));
            }
            if let Ok(decoded) = <RemoveUintArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveUintArray(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <SetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAddress(decoded));
            }
            if let Ok(decoded) = <SetAddressArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAddressArray(decoded));
            }
            if let Ok(decoded) = <SetBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBool(decoded));
            }
            if let Ok(decoded) = <SetBoolArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBoolArray(decoded));
            }
            if let Ok(decoded) = <SetBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBytes32(decoded));
            }
            if let Ok(decoded) = <SetBytes32ArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBytes32Array(decoded));
            }
            if let Ok(decoded) = <SetIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetInt(decoded));
            }
            if let Ok(decoded) = <SetIntArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetIntArray(decoded));
            }
            if let Ok(decoded) = <SetStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetString(decoded));
            }
            if let Ok(decoded) = <SetStringArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStringArray(decoded));
            }
            if let Ok(decoded) = <SetUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUint(decoded));
            }
            if let Ok(decoded) = <SetUintArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUintArray(decoded));
            }
            if let Ok(decoded) = <StringArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringArrayValues(decoded));
            }
            if let Ok(decoded) = <StringValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StringValues(decoded));
            }
            if let Ok(decoded) = <UintArrayValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UintArrayValues(decoded));
            }
            if let Ok(decoded) = <UintValuesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UintValues(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DataStoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApplyBoundedDeltaToUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApplyDeltaToInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApplyDeltaToUintWithKeyAndValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApplyDeltaToUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BoolArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BoolValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bytes32ArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bytes32Values(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContainsAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContainsBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContainsUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecrementInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecrementUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressValuesAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBoolArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32Array(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32Count(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32ValuesAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetIntArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStringArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetUintArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUintCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUintValuesAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncrementInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncrementUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAddressWithSetKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAddressArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBoolArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBytes32WithSetKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBytes32Array(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveInt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveIntArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStringArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUintWithSetKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUintArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddressArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBoolArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBytes32Array(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetIntArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStringArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUintArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UintArrayValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UintValues(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DataStoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressArrayValues(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApplyBoundedDeltaToUint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApplyDeltaToInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApplyDeltaToUintWithKeyAndValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApplyDeltaToUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::BoolArrayValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::BoolValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bytes32ArrayValues(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bytes32Values(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContainsAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContainsBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContainsUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecrementInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecrementUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressValuesAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBoolArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32Array(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32Count(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32ValuesAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetIntArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetString(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStringArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUintArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUintCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUintValuesAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntArrayValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAddressWithSetKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAddressArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBoolArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBytes32WithSetKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBytes32Array(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveIntArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveString(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStringArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveUintWithSetKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveUintArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddressArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBoolArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBytes32Array(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIntArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetString(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStringArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUintArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::StringArrayValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::StringValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::UintArrayValues(element) => ::core::fmt::Display::fmt(element, f),
                Self::UintValues(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAddressCall> for DataStoreCalls {
        fn from(value: AddAddressCall) -> Self {
            Self::AddAddress(value)
        }
    }
    impl ::core::convert::From<AddBytes32Call> for DataStoreCalls {
        fn from(value: AddBytes32Call) -> Self {
            Self::AddBytes32(value)
        }
    }
    impl ::core::convert::From<AddUintCall> for DataStoreCalls {
        fn from(value: AddUintCall) -> Self {
            Self::AddUint(value)
        }
    }
    impl ::core::convert::From<AddressArrayValuesCall> for DataStoreCalls {
        fn from(value: AddressArrayValuesCall) -> Self {
            Self::AddressArrayValues(value)
        }
    }
    impl ::core::convert::From<AddressValuesCall> for DataStoreCalls {
        fn from(value: AddressValuesCall) -> Self {
            Self::AddressValues(value)
        }
    }
    impl ::core::convert::From<ApplyBoundedDeltaToUintCall> for DataStoreCalls {
        fn from(value: ApplyBoundedDeltaToUintCall) -> Self {
            Self::ApplyBoundedDeltaToUint(value)
        }
    }
    impl ::core::convert::From<ApplyDeltaToIntCall> for DataStoreCalls {
        fn from(value: ApplyDeltaToIntCall) -> Self {
            Self::ApplyDeltaToInt(value)
        }
    }
    impl ::core::convert::From<ApplyDeltaToUintWithKeyAndValueCall> for DataStoreCalls {
        fn from(value: ApplyDeltaToUintWithKeyAndValueCall) -> Self {
            Self::ApplyDeltaToUintWithKeyAndValue(value)
        }
    }
    impl ::core::convert::From<ApplyDeltaToUintCall> for DataStoreCalls {
        fn from(value: ApplyDeltaToUintCall) -> Self {
            Self::ApplyDeltaToUint(value)
        }
    }
    impl ::core::convert::From<BoolArrayValuesCall> for DataStoreCalls {
        fn from(value: BoolArrayValuesCall) -> Self {
            Self::BoolArrayValues(value)
        }
    }
    impl ::core::convert::From<BoolValuesCall> for DataStoreCalls {
        fn from(value: BoolValuesCall) -> Self {
            Self::BoolValues(value)
        }
    }
    impl ::core::convert::From<Bytes32ArrayValuesCall> for DataStoreCalls {
        fn from(value: Bytes32ArrayValuesCall) -> Self {
            Self::Bytes32ArrayValues(value)
        }
    }
    impl ::core::convert::From<Bytes32ValuesCall> for DataStoreCalls {
        fn from(value: Bytes32ValuesCall) -> Self {
            Self::Bytes32Values(value)
        }
    }
    impl ::core::convert::From<ContainsAddressCall> for DataStoreCalls {
        fn from(value: ContainsAddressCall) -> Self {
            Self::ContainsAddress(value)
        }
    }
    impl ::core::convert::From<ContainsBytes32Call> for DataStoreCalls {
        fn from(value: ContainsBytes32Call) -> Self {
            Self::ContainsBytes32(value)
        }
    }
    impl ::core::convert::From<ContainsUintCall> for DataStoreCalls {
        fn from(value: ContainsUintCall) -> Self {
            Self::ContainsUint(value)
        }
    }
    impl ::core::convert::From<DecrementIntCall> for DataStoreCalls {
        fn from(value: DecrementIntCall) -> Self {
            Self::DecrementInt(value)
        }
    }
    impl ::core::convert::From<DecrementUintCall> for DataStoreCalls {
        fn from(value: DecrementUintCall) -> Self {
            Self::DecrementUint(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for DataStoreCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetAddressArrayCall> for DataStoreCalls {
        fn from(value: GetAddressArrayCall) -> Self {
            Self::GetAddressArray(value)
        }
    }
    impl ::core::convert::From<GetAddressCountCall> for DataStoreCalls {
        fn from(value: GetAddressCountCall) -> Self {
            Self::GetAddressCount(value)
        }
    }
    impl ::core::convert::From<GetAddressValuesAtCall> for DataStoreCalls {
        fn from(value: GetAddressValuesAtCall) -> Self {
            Self::GetAddressValuesAt(value)
        }
    }
    impl ::core::convert::From<GetBoolCall> for DataStoreCalls {
        fn from(value: GetBoolCall) -> Self {
            Self::GetBool(value)
        }
    }
    impl ::core::convert::From<GetBoolArrayCall> for DataStoreCalls {
        fn from(value: GetBoolArrayCall) -> Self {
            Self::GetBoolArray(value)
        }
    }
    impl ::core::convert::From<GetBytes32Call> for DataStoreCalls {
        fn from(value: GetBytes32Call) -> Self {
            Self::GetBytes32(value)
        }
    }
    impl ::core::convert::From<GetBytes32ArrayCall> for DataStoreCalls {
        fn from(value: GetBytes32ArrayCall) -> Self {
            Self::GetBytes32Array(value)
        }
    }
    impl ::core::convert::From<GetBytes32CountCall> for DataStoreCalls {
        fn from(value: GetBytes32CountCall) -> Self {
            Self::GetBytes32Count(value)
        }
    }
    impl ::core::convert::From<GetBytes32ValuesAtCall> for DataStoreCalls {
        fn from(value: GetBytes32ValuesAtCall) -> Self {
            Self::GetBytes32ValuesAt(value)
        }
    }
    impl ::core::convert::From<GetIntCall> for DataStoreCalls {
        fn from(value: GetIntCall) -> Self {
            Self::GetInt(value)
        }
    }
    impl ::core::convert::From<GetIntArrayCall> for DataStoreCalls {
        fn from(value: GetIntArrayCall) -> Self {
            Self::GetIntArray(value)
        }
    }
    impl ::core::convert::From<GetStringCall> for DataStoreCalls {
        fn from(value: GetStringCall) -> Self {
            Self::GetString(value)
        }
    }
    impl ::core::convert::From<GetStringArrayCall> for DataStoreCalls {
        fn from(value: GetStringArrayCall) -> Self {
            Self::GetStringArray(value)
        }
    }
    impl ::core::convert::From<GetUintCall> for DataStoreCalls {
        fn from(value: GetUintCall) -> Self {
            Self::GetUint(value)
        }
    }
    impl ::core::convert::From<GetUintArrayCall> for DataStoreCalls {
        fn from(value: GetUintArrayCall) -> Self {
            Self::GetUintArray(value)
        }
    }
    impl ::core::convert::From<GetUintCountCall> for DataStoreCalls {
        fn from(value: GetUintCountCall) -> Self {
            Self::GetUintCount(value)
        }
    }
    impl ::core::convert::From<GetUintValuesAtCall> for DataStoreCalls {
        fn from(value: GetUintValuesAtCall) -> Self {
            Self::GetUintValuesAt(value)
        }
    }
    impl ::core::convert::From<IncrementIntCall> for DataStoreCalls {
        fn from(value: IncrementIntCall) -> Self {
            Self::IncrementInt(value)
        }
    }
    impl ::core::convert::From<IncrementUintCall> for DataStoreCalls {
        fn from(value: IncrementUintCall) -> Self {
            Self::IncrementUint(value)
        }
    }
    impl ::core::convert::From<IntArrayValuesCall> for DataStoreCalls {
        fn from(value: IntArrayValuesCall) -> Self {
            Self::IntArrayValues(value)
        }
    }
    impl ::core::convert::From<IntValuesCall> for DataStoreCalls {
        fn from(value: IntValuesCall) -> Self {
            Self::IntValues(value)
        }
    }
    impl ::core::convert::From<RemoveAddressWithSetKeyCall> for DataStoreCalls {
        fn from(value: RemoveAddressWithSetKeyCall) -> Self {
            Self::RemoveAddressWithSetKey(value)
        }
    }
    impl ::core::convert::From<RemoveAddressCall> for DataStoreCalls {
        fn from(value: RemoveAddressCall) -> Self {
            Self::RemoveAddress(value)
        }
    }
    impl ::core::convert::From<RemoveAddressArrayCall> for DataStoreCalls {
        fn from(value: RemoveAddressArrayCall) -> Self {
            Self::RemoveAddressArray(value)
        }
    }
    impl ::core::convert::From<RemoveBoolCall> for DataStoreCalls {
        fn from(value: RemoveBoolCall) -> Self {
            Self::RemoveBool(value)
        }
    }
    impl ::core::convert::From<RemoveBoolArrayCall> for DataStoreCalls {
        fn from(value: RemoveBoolArrayCall) -> Self {
            Self::RemoveBoolArray(value)
        }
    }
    impl ::core::convert::From<RemoveBytes32WithSetKeyCall> for DataStoreCalls {
        fn from(value: RemoveBytes32WithSetKeyCall) -> Self {
            Self::RemoveBytes32WithSetKey(value)
        }
    }
    impl ::core::convert::From<RemoveBytes32Call> for DataStoreCalls {
        fn from(value: RemoveBytes32Call) -> Self {
            Self::RemoveBytes32(value)
        }
    }
    impl ::core::convert::From<RemoveBytes32ArrayCall> for DataStoreCalls {
        fn from(value: RemoveBytes32ArrayCall) -> Self {
            Self::RemoveBytes32Array(value)
        }
    }
    impl ::core::convert::From<RemoveIntCall> for DataStoreCalls {
        fn from(value: RemoveIntCall) -> Self {
            Self::RemoveInt(value)
        }
    }
    impl ::core::convert::From<RemoveIntArrayCall> for DataStoreCalls {
        fn from(value: RemoveIntArrayCall) -> Self {
            Self::RemoveIntArray(value)
        }
    }
    impl ::core::convert::From<RemoveStringCall> for DataStoreCalls {
        fn from(value: RemoveStringCall) -> Self {
            Self::RemoveString(value)
        }
    }
    impl ::core::convert::From<RemoveStringArrayCall> for DataStoreCalls {
        fn from(value: RemoveStringArrayCall) -> Self {
            Self::RemoveStringArray(value)
        }
    }
    impl ::core::convert::From<RemoveUintCall> for DataStoreCalls {
        fn from(value: RemoveUintCall) -> Self {
            Self::RemoveUint(value)
        }
    }
    impl ::core::convert::From<RemoveUintWithSetKeyCall> for DataStoreCalls {
        fn from(value: RemoveUintWithSetKeyCall) -> Self {
            Self::RemoveUintWithSetKey(value)
        }
    }
    impl ::core::convert::From<RemoveUintArrayCall> for DataStoreCalls {
        fn from(value: RemoveUintArrayCall) -> Self {
            Self::RemoveUintArray(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for DataStoreCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for DataStoreCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<SetAddressArrayCall> for DataStoreCalls {
        fn from(value: SetAddressArrayCall) -> Self {
            Self::SetAddressArray(value)
        }
    }
    impl ::core::convert::From<SetBoolCall> for DataStoreCalls {
        fn from(value: SetBoolCall) -> Self {
            Self::SetBool(value)
        }
    }
    impl ::core::convert::From<SetBoolArrayCall> for DataStoreCalls {
        fn from(value: SetBoolArrayCall) -> Self {
            Self::SetBoolArray(value)
        }
    }
    impl ::core::convert::From<SetBytes32Call> for DataStoreCalls {
        fn from(value: SetBytes32Call) -> Self {
            Self::SetBytes32(value)
        }
    }
    impl ::core::convert::From<SetBytes32ArrayCall> for DataStoreCalls {
        fn from(value: SetBytes32ArrayCall) -> Self {
            Self::SetBytes32Array(value)
        }
    }
    impl ::core::convert::From<SetIntCall> for DataStoreCalls {
        fn from(value: SetIntCall) -> Self {
            Self::SetInt(value)
        }
    }
    impl ::core::convert::From<SetIntArrayCall> for DataStoreCalls {
        fn from(value: SetIntArrayCall) -> Self {
            Self::SetIntArray(value)
        }
    }
    impl ::core::convert::From<SetStringCall> for DataStoreCalls {
        fn from(value: SetStringCall) -> Self {
            Self::SetString(value)
        }
    }
    impl ::core::convert::From<SetStringArrayCall> for DataStoreCalls {
        fn from(value: SetStringArrayCall) -> Self {
            Self::SetStringArray(value)
        }
    }
    impl ::core::convert::From<SetUintCall> for DataStoreCalls {
        fn from(value: SetUintCall) -> Self {
            Self::SetUint(value)
        }
    }
    impl ::core::convert::From<SetUintArrayCall> for DataStoreCalls {
        fn from(value: SetUintArrayCall) -> Self {
            Self::SetUintArray(value)
        }
    }
    impl ::core::convert::From<StringArrayValuesCall> for DataStoreCalls {
        fn from(value: StringArrayValuesCall) -> Self {
            Self::StringArrayValues(value)
        }
    }
    impl ::core::convert::From<StringValuesCall> for DataStoreCalls {
        fn from(value: StringValuesCall) -> Self {
            Self::StringValues(value)
        }
    }
    impl ::core::convert::From<UintArrayValuesCall> for DataStoreCalls {
        fn from(value: UintArrayValuesCall) -> Self {
            Self::UintArrayValues(value)
        }
    }
    impl ::core::convert::From<UintValuesCall> for DataStoreCalls {
        fn from(value: UintValuesCall) -> Self {
            Self::UintValues(value)
        }
    }
    ///Container type for all return fields from the `addressArrayValues` function with signature `addressArrayValues(bytes32,uint256)` and selector `0x22f87464`
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
    pub struct AddressArrayValuesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addressValues` function with signature `addressValues(bytes32)` and selector `0x22538dae`
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
    pub struct AddressValuesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `applyBoundedDeltaToUint` function with signature `applyBoundedDeltaToUint(bytes32,int256)` and selector `0x8ca498b0`
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
    pub struct ApplyBoundedDeltaToUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `applyDeltaToInt` function with signature `applyDeltaToInt(bytes32,int256)` and selector `0xe4e36c4e`
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
    pub struct ApplyDeltaToIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `applyDeltaToUint` function with signature `applyDeltaToUint(bytes32,int256,string)` and selector `0x32f85bbf`
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
    pub struct ApplyDeltaToUintWithKeyAndValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `applyDeltaToUint` function with signature `applyDeltaToUint(bytes32,uint256)` and selector `0x3dbacd1a`
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
    pub struct ApplyDeltaToUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `boolArrayValues` function with signature `boolArrayValues(bytes32,uint256)` and selector `0x80aacdcd`
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
    pub struct BoolArrayValuesReturn(pub bool);
    ///Container type for all return fields from the `boolValues` function with signature `boolValues(bytes32)` and selector `0x44a242b1`
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
    pub struct BoolValuesReturn(pub bool);
    ///Container type for all return fields from the `bytes32ArrayValues` function with signature `bytes32ArrayValues(bytes32,uint256)` and selector `0xbf498dd3`
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
    pub struct Bytes32ArrayValuesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `bytes32Values` function with signature `bytes32Values(bytes32)` and selector `0xd52852af`
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
    pub struct Bytes32ValuesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `containsAddress` function with signature `containsAddress(bytes32,address)` and selector `0xc769d1a1`
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
    pub struct ContainsAddressReturn(pub bool);
    ///Container type for all return fields from the `containsBytes32` function with signature `containsBytes32(bytes32,bytes32)` and selector `0x91d4403c`
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
    pub struct ContainsBytes32Return(pub bool);
    ///Container type for all return fields from the `containsUint` function with signature `containsUint(bytes32,uint256)` and selector `0x310b8882`
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
    pub struct ContainsUintReturn(pub bool);
    ///Container type for all return fields from the `decrementInt` function with signature `decrementInt(bytes32,int256)` and selector `0x6fae54f0`
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
    pub struct DecrementIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `decrementUint` function with signature `decrementUint(bytes32,uint256)` and selector `0xe98aabc1`
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
    pub struct DecrementUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
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
    pub struct GetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAddressArray` function with signature `getAddressArray(bytes32)` and selector `0x5948f733`
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
    pub struct GetAddressArrayReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getAddressCount` function with signature `getAddressCount(bytes32)` and selector `0x35ea8059`
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
    pub struct GetAddressCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAddressValuesAt` function with signature `getAddressValuesAt(bytes32,uint256,uint256)` and selector `0xe7e4148e`
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
    pub struct GetAddressValuesAtReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getBool` function with signature `getBool(bytes32)` and selector `0x7ae1cfca`
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
    pub struct GetBoolReturn(pub bool);
    ///Container type for all return fields from the `getBoolArray` function with signature `getBoolArray(bytes32)` and selector `0x116bb929`
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
    pub struct GetBoolArrayReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `getBytes32` function with signature `getBytes32(bytes32)` and selector `0xa6ed563e`
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
    pub struct GetBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `getBytes32Array` function with signature `getBytes32Array(bytes32)` and selector `0xdd031997`
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
    pub struct GetBytes32ArrayReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getBytes32Count` function with signature `getBytes32Count(bytes32)` and selector `0xf3903b9f`
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
    pub struct GetBytes32CountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBytes32ValuesAt` function with signature `getBytes32ValuesAt(bytes32,uint256,uint256)` and selector `0xf069052a`
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
    pub struct GetBytes32ValuesAtReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getInt` function with signature `getInt(bytes32)` and selector `0xdc97d962`
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
    pub struct GetIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getIntArray` function with signature `getIntArray(bytes32)` and selector `0x2d2899b6`
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
    pub struct GetIntArrayReturn(pub ::std::vec::Vec<::ethers::core::types::I256>);
    ///Container type for all return fields from the `getString` function with signature `getString(bytes32)` and selector `0x986e791a`
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
    pub struct GetStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getStringArray` function with signature `getStringArray(bytes32)` and selector `0x01677da2`
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
    pub struct GetStringArrayReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `getUint` function with signature `getUint(bytes32)` and selector `0xbd02d0f5`
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
    pub struct GetUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUintArray` function with signature `getUintArray(bytes32)` and selector `0x86ac6bdf`
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
    pub struct GetUintArrayReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getUintCount` function with signature `getUintCount(bytes32)` and selector `0x065f21a7`
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
    pub struct GetUintCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUintValuesAt` function with signature `getUintValuesAt(bytes32,uint256,uint256)` and selector `0x7026d42c`
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
    pub struct GetUintValuesAtReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `incrementInt` function with signature `incrementInt(bytes32,int256)` and selector `0xcbb093dd`
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
    pub struct IncrementIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `incrementUint` function with signature `incrementUint(bytes32,uint256)` and selector `0x340dbab3`
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
    pub struct IncrementUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `intArrayValues` function with signature `intArrayValues(bytes32,uint256)` and selector `0x6339734d`
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
    pub struct IntArrayValuesReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `intValues` function with signature `intValues(bytes32)` and selector `0x743df325`
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
    pub struct IntValuesReturn(pub ::ethers::core::types::I256);
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
    ///Container type for all return fields from the `setAddress` function with signature `setAddress(bytes32,address)` and selector `0xca446dd9`
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
    pub struct SetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setBool` function with signature `setBool(bytes32,bool)` and selector `0xabfdcced`
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
    pub struct SetBoolReturn(pub bool);
    ///Container type for all return fields from the `setBytes32` function with signature `setBytes32(bytes32,bytes32)` and selector `0x4e91db08`
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
    pub struct SetBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `setInt` function with signature `setInt(bytes32,int256)` and selector `0x3e49bed0`
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
    pub struct SetIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `setString` function with signature `setString(bytes32,string)` and selector `0x6e899550`
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
    pub struct SetStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `setUint` function with signature `setUint(bytes32,uint256)` and selector `0xe2a4853a`
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
    pub struct SetUintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stringArrayValues` function with signature `stringArrayValues(bytes32,uint256)` and selector `0xb8320a08`
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
    pub struct StringArrayValuesReturn(pub ::std::string::String);
    ///Container type for all return fields from the `stringValues` function with signature `stringValues(bytes32)` and selector `0xf15caeac`
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
    pub struct StringValuesReturn(pub ::std::string::String);
    ///Container type for all return fields from the `uintArrayValues` function with signature `uintArrayValues(bytes32,uint256)` and selector `0xc4f00fde`
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
    pub struct UintArrayValuesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `uintValues` function with signature `uintValues(bytes32)` and selector `0xd38eebc7`
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
    pub struct UintValuesReturn(pub ::ethers::core::types::U256);
}
