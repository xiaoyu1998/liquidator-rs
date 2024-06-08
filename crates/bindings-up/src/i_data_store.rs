pub use i_data_store::*;
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
pub mod i_data_store {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IDATASTORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IDataStore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDataStore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDataStore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDataStore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDataStore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDataStore)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDataStore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDATASTORE_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `getBytes32` (0xa6ed563e) function
        pub fn get_bytes_32(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 237, 86, 62], key)
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
        ///Calls the contract's `getUint` (0xbd02d0f5) function
        pub fn get_uint(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 2, 208, 245], key)
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
        ///Calls the contract's `removeBool` (0x9fe7ac12) function
        pub fn remove_bool(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 231, 172, 18], key)
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
        ///Calls the contract's `removeUint` (0x42c3bd96) function
        pub fn remove_uint(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 195, 189, 150], key)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDataStore<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    pub enum IDataStoreCalls {
        AddAddress(AddAddressCall),
        AddBytes32(AddBytes32Call),
        ContainsAddress(ContainsAddressCall),
        ContainsBytes32(ContainsBytes32Call),
        GetAddress(GetAddressCall),
        GetAddressCount(GetAddressCountCall),
        GetAddressValuesAt(GetAddressValuesAtCall),
        GetBool(GetBoolCall),
        GetBytes32(GetBytes32Call),
        GetBytes32Count(GetBytes32CountCall),
        GetBytes32ValuesAt(GetBytes32ValuesAtCall),
        GetUint(GetUintCall),
        IncrementUint(IncrementUintCall),
        RemoveAddressWithSetKey(RemoveAddressWithSetKeyCall),
        RemoveAddress(RemoveAddressCall),
        RemoveBool(RemoveBoolCall),
        RemoveBytes32WithSetKey(RemoveBytes32WithSetKeyCall),
        RemoveBytes32(RemoveBytes32Call),
        RemoveUint(RemoveUintCall),
        SetAddress(SetAddressCall),
        SetBool(SetBoolCall),
        SetBytes32(SetBytes32Call),
        SetUint(SetUintCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDataStoreCalls {
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
            if let Ok(decoded) = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddress(decoded));
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
            if let Ok(decoded) = <GetBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes32(decoded));
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
            if let Ok(decoded) = <GetUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUint(decoded));
            }
            if let Ok(decoded) = <IncrementUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncrementUint(decoded));
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
            if let Ok(decoded) = <RemoveBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveBool(decoded));
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
            if let Ok(decoded) = <RemoveUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveUint(decoded));
            }
            if let Ok(decoded) = <SetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAddress(decoded));
            }
            if let Ok(decoded) = <SetBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBool(decoded));
            }
            if let Ok(decoded) = <SetBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBytes32(decoded));
            }
            if let Ok(decoded) = <SetUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUint(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDataStoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContainsAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContainsBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressValuesAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32Count(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBytes32ValuesAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAddressWithSetKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBytes32WithSetKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveUint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBytes32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IDataStoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContainsAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContainsBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddressValuesAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32Count(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes32ValuesAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAddressWithSetKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBytes32WithSetKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUint(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAddressCall> for IDataStoreCalls {
        fn from(value: AddAddressCall) -> Self {
            Self::AddAddress(value)
        }
    }
    impl ::core::convert::From<AddBytes32Call> for IDataStoreCalls {
        fn from(value: AddBytes32Call) -> Self {
            Self::AddBytes32(value)
        }
    }
    impl ::core::convert::From<ContainsAddressCall> for IDataStoreCalls {
        fn from(value: ContainsAddressCall) -> Self {
            Self::ContainsAddress(value)
        }
    }
    impl ::core::convert::From<ContainsBytes32Call> for IDataStoreCalls {
        fn from(value: ContainsBytes32Call) -> Self {
            Self::ContainsBytes32(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for IDataStoreCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetAddressCountCall> for IDataStoreCalls {
        fn from(value: GetAddressCountCall) -> Self {
            Self::GetAddressCount(value)
        }
    }
    impl ::core::convert::From<GetAddressValuesAtCall> for IDataStoreCalls {
        fn from(value: GetAddressValuesAtCall) -> Self {
            Self::GetAddressValuesAt(value)
        }
    }
    impl ::core::convert::From<GetBoolCall> for IDataStoreCalls {
        fn from(value: GetBoolCall) -> Self {
            Self::GetBool(value)
        }
    }
    impl ::core::convert::From<GetBytes32Call> for IDataStoreCalls {
        fn from(value: GetBytes32Call) -> Self {
            Self::GetBytes32(value)
        }
    }
    impl ::core::convert::From<GetBytes32CountCall> for IDataStoreCalls {
        fn from(value: GetBytes32CountCall) -> Self {
            Self::GetBytes32Count(value)
        }
    }
    impl ::core::convert::From<GetBytes32ValuesAtCall> for IDataStoreCalls {
        fn from(value: GetBytes32ValuesAtCall) -> Self {
            Self::GetBytes32ValuesAt(value)
        }
    }
    impl ::core::convert::From<GetUintCall> for IDataStoreCalls {
        fn from(value: GetUintCall) -> Self {
            Self::GetUint(value)
        }
    }
    impl ::core::convert::From<IncrementUintCall> for IDataStoreCalls {
        fn from(value: IncrementUintCall) -> Self {
            Self::IncrementUint(value)
        }
    }
    impl ::core::convert::From<RemoveAddressWithSetKeyCall> for IDataStoreCalls {
        fn from(value: RemoveAddressWithSetKeyCall) -> Self {
            Self::RemoveAddressWithSetKey(value)
        }
    }
    impl ::core::convert::From<RemoveAddressCall> for IDataStoreCalls {
        fn from(value: RemoveAddressCall) -> Self {
            Self::RemoveAddress(value)
        }
    }
    impl ::core::convert::From<RemoveBoolCall> for IDataStoreCalls {
        fn from(value: RemoveBoolCall) -> Self {
            Self::RemoveBool(value)
        }
    }
    impl ::core::convert::From<RemoveBytes32WithSetKeyCall> for IDataStoreCalls {
        fn from(value: RemoveBytes32WithSetKeyCall) -> Self {
            Self::RemoveBytes32WithSetKey(value)
        }
    }
    impl ::core::convert::From<RemoveBytes32Call> for IDataStoreCalls {
        fn from(value: RemoveBytes32Call) -> Self {
            Self::RemoveBytes32(value)
        }
    }
    impl ::core::convert::From<RemoveUintCall> for IDataStoreCalls {
        fn from(value: RemoveUintCall) -> Self {
            Self::RemoveUint(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for IDataStoreCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<SetBoolCall> for IDataStoreCalls {
        fn from(value: SetBoolCall) -> Self {
            Self::SetBool(value)
        }
    }
    impl ::core::convert::From<SetBytes32Call> for IDataStoreCalls {
        fn from(value: SetBytes32Call) -> Self {
            Self::SetBytes32(value)
        }
    }
    impl ::core::convert::From<SetUintCall> for IDataStoreCalls {
        fn from(value: SetUintCall) -> Self {
            Self::SetUint(value)
        }
    }
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
}
