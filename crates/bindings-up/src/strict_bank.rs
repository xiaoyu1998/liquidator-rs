pub use strict_bank::*;
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
pub mod strict_bank {
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
                        name: ::std::borrow::ToOwned::to_owned("_dataStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract DataStore"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dataStore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dataStore"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract DataStore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordTransferIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recordTransferIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("recordTransferOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recordTransferOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("syncTokenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("syncTokenBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("tokenBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenBalances"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("transferOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("TokenTransferReverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenTransferReverted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("returndata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
                    ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyReceiver"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNativeTokenSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidNativeTokenSender",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
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
                    ::std::borrow::ToOwned::to_owned("SelfTransferNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SelfTransferNotSupported",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                    ::std::borrow::ToOwned::to_owned("TokenTransferError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenTransferError"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
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
                            ],
                        },
                    ],
                ),
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
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STRICTBANK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r38\x03\x80a\r3\x839\x81\x01`@\x81\x90Ra\0/\x91a\0^V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x98V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\0qW`\0\x80\xFD[\x82Qa\0|\x81a\0FV[` \x84\x01Q\x90\x92Pa\0\x8D\x81a\0FV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0Cia\0\xCA`\09`\0\x81\x81`\x80\x01Ra\x01\xBB\x01R`\0\x81\x81a\x01B\x01Ra\x04(\x01Ra\x0Ci`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cR?\xBA\x7F\x11a\0NW\x80cR?\xBA\x7F\x14a\x01|W\x80cf\r\rg\x14a\x01\xA9W\x80c\x8C\x1B_\xDE\x14a\x01\xDDW\x80c\xEB@\x13?\x14a\x01\xFDW`\0\x80\xFD[\x80c\x07\x8D;y\x14a\0\xDDW\x80c5/\x9A\xED\x14a\0\xFDW\x80cJJ{\x04\x14a\x010W`\0\x80\xFD[6a\0\xD8W`\0a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\x1DV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\0\xD6W`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xE9W`\0\x80\xFD[Pa\0\xD6a\0\xF86`\x04a\t\xB1V[a\x02\xD0V[4\x80\x15a\x01\tW`\0\x80\xFD[Pa\x01\x1Da\x01\x186`\x04a\t\xF2V[a\x030V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01<W`\0\x80\xFD[Pa\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x01\x88W`\0\x80\xFD[Pa\x01\x1Da\x01\x976`\x04a\t\xF2V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\x1Da\x01\xF86`\x04a\t\xF2V[a\x03MV[4\x80\x15a\x02\tW`\0\x80\xFD[Pa\x01\x1Da\x02\x186`\x04a\t\xF2V[a\x03jV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x02U\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\n\x16V[\x92\x91PPV[a\x03 `@Q` \x01a\x02\xE2\x90a\n3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x04\x0CV[a\x03+\x83\x83\x83a\x04\xC0V[PPPV[`\0a\x03D`@Q` \x01a\x02\xE2\x90a\n3V[a\x02\xCA\x82a\x05\x08V[`\0a\x03a`@Q` \x01a\x02\xE2\x90a\n3V[a\x02\xCA\x82a\x05\xB4V[`\0a\x03~`@Q` \x01a\x02\xE2\x90a\n3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE9\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x91\x90PV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a\npV[a\x04\xBCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\0\xCD\x92\x91\x90a\n\xE2V[PPV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x04\xF4W`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\0\xCDV[a\x04\xFF\x83\x83\x83a\x06XV[a\x03+\x83a\x07\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x85\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x05\xAC\x82\x82a\x0B\x06V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x061\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x05\xAC\x81\x83a\x0B\x06V[\x80`\0\x03a\x06eWPPPV[a\x06n\x82a\x07\x8BV[`\0\x80a\x06|\x85\x85\x85a\x07\xB5V[\x91P\x91P\x81\x15a\x06\x8DWPPPPPV[`\0a\x06\x98\x82a\t\x0CV[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x06\xCC\x92\x91\x90a\x0B'V[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\0\xCDV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07o\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xB2W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x08\x18\x90\x85\x90a\x0BUV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08ZV[``\x91P[P\x91P\x91P\x81\x15a\x08\xFBW\x80Q`\0\x03a\x08\xB8W`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x08\xB8W`\0`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\t\x04V[`\0\x81Q\x11\x80\x15a\x08\xDAWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xD8\x91\x90a\npV[\x15[\x15a\x08\xEDW`\0\x94P\x92Pa\t\x04\x91PPV[`\x01\x94P\x92Pa\t\x04\x91PPV[`\0\x94P\x92PPP[\x93P\x93\x91PPV[```\0`D\x83Q\x10\x15a\t3WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\t@\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\t\x7FW`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\tt\x91\x90a\x0B\x87V[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xB2W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x835a\t\xD1\x81a\t\x9CV[\x92P` \x84\x015a\t\xE1\x81a\t\x9CV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\x04W`\0\x80\xFD[\x815a\n\x0F\x81a\t\x9CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\n(W`\0\x80\xFD[\x81Qa\n\x0F\x81a\t\x9CV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\niW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x82W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x0FW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\n\xADW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x95V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\xCE\x81` \x86\x01` \x86\x01a\n\x92V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\xAC\x90\x83\x01\x84a\n\xB6V[\x81\x81\x03\x81\x81\x11\x15a\x02\xCAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`@\x81R`\0a\x0B:`@\x83\x01\x85a\n\xB6V[\x82\x81\x03` \x84\x01Ra\x0BL\x81\x85a\n\xB6V[\x95\x94PPPPPV[`\0\x82Qa\x0Bg\x81\x84` \x87\x01a\n\x92V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\x99W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xB0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0B\xC1W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xDBWa\x0B\xDBa\x0BqV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\nWa\x0C\na\x0BqV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x0C\"W`\0\x80\xFD[a\x0BL\x82` \x83\x01` \x86\x01a\n\x92V\xFE\xA2dipfsX\"\x12 \xCC\x7FN\x1B\xE9\\*\xDC\xD9I7\xE2r \x9AC\x01\xB0\xB1\xFA9\x19\xBF\xCD\xAD\xE5\x01\x0C/\xE2N\x1AdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static STRICTBANK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cR?\xBA\x7F\x11a\0NW\x80cR?\xBA\x7F\x14a\x01|W\x80cf\r\rg\x14a\x01\xA9W\x80c\x8C\x1B_\xDE\x14a\x01\xDDW\x80c\xEB@\x13?\x14a\x01\xFDW`\0\x80\xFD[\x80c\x07\x8D;y\x14a\0\xDDW\x80c5/\x9A\xED\x14a\0\xFDW\x80cJJ{\x04\x14a\x010W`\0\x80\xFD[6a\0\xD8W`\0a\0\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02\x1DV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\0\xD6W`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xE9W`\0\x80\xFD[Pa\0\xD6a\0\xF86`\x04a\t\xB1V[a\x02\xD0V[4\x80\x15a\x01\tW`\0\x80\xFD[Pa\x01\x1Da\x01\x186`\x04a\t\xF2V[a\x030V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01<W`\0\x80\xFD[Pa\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x01\x88W`\0\x80\xFD[Pa\x01\x1Da\x01\x976`\x04a\t\xF2V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xE9W`\0\x80\xFD[Pa\x01\x1Da\x01\xF86`\x04a\t\xF2V[a\x03MV[4\x80\x15a\x02\tW`\0\x80\xFD[Pa\x01\x1Da\x02\x186`\x04a\t\xF2V[a\x03jV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x02U\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\n\x16V[\x92\x91PPV[a\x03 `@Q` \x01a\x02\xE2\x90a\n3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x04\x0CV[a\x03+\x83\x83\x83a\x04\xC0V[PPPV[`\0a\x03D`@Q` \x01a\x02\xE2\x90a\n3V[a\x02\xCA\x82a\x05\x08V[`\0a\x03a`@Q` \x01a\x02\xE2\x90a\n3V[a\x02\xCA\x82a\x05\xB4V[`\0a\x03~`@Q` \x01a\x02\xE2\x90a\n3V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE9\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x91\x90PV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a\npV[a\x04\xBCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\0\xCD\x92\x91\x90a\n\xE2V[PPV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x04\xF4W`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\0\xCDV[a\x04\xFF\x83\x83\x83a\x06XV[a\x03+\x83a\x07\x07V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x85\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x05\xAC\x82\x82a\x0B\x06V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x061\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x05\xAC\x81\x83a\x0B\x06V[\x80`\0\x03a\x06eWPPPV[a\x06n\x82a\x07\x8BV[`\0\x80a\x06|\x85\x85\x85a\x07\xB5V[\x91P\x91P\x81\x15a\x06\x8DWPPPPPV[`\0a\x06\x98\x82a\t\x0CV[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x06\xCC\x92\x91\x90a\x0B'V[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\0\xCDV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07o\x91\x90a\nWV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xB2W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x08\x18\x90\x85\x90a\x0BUV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x08UW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08ZV[``\x91P[P\x91P\x91P\x81\x15a\x08\xFBW\x80Q`\0\x03a\x08\xB8W`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x08\xB8W`\0`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\t\x04V[`\0\x81Q\x11\x80\x15a\x08\xDAWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xD8\x91\x90a\npV[\x15[\x15a\x08\xEDW`\0\x94P\x92Pa\t\x04\x91PPV[`\x01\x94P\x92Pa\t\x04\x91PPV[`\0\x94P\x92PPP[\x93P\x93\x91PPV[```\0`D\x83Q\x10\x15a\t3WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\t@\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\t\x7FW`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\tt\x91\x90a\x0B\x87V[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xB2W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xC6W`\0\x80\xFD[\x835a\t\xD1\x81a\t\x9CV[\x92P` \x84\x015a\t\xE1\x81a\t\x9CV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\x04W`\0\x80\xFD[\x815a\n\x0F\x81a\t\x9CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\n(W`\0\x80\xFD[\x81Qa\n\x0F\x81a\t\x9CV[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\niW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x82W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x0FW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\n\xADW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x95V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\xCE\x81` \x86\x01` \x86\x01a\n\x92V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\xAC\x90\x83\x01\x84a\n\xB6V[\x81\x81\x03\x81\x81\x11\x15a\x02\xCAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`@\x81R`\0a\x0B:`@\x83\x01\x85a\n\xB6V[\x82\x81\x03` \x84\x01Ra\x0BL\x81\x85a\n\xB6V[\x95\x94PPPPPV[`\0\x82Qa\x0Bg\x81\x84` \x87\x01a\n\x92V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\x99W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xB0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0B\xC1W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xDBWa\x0B\xDBa\x0BqV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\nWa\x0C\na\x0BqV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x0C\"W`\0\x80\xFD[a\x0BL\x82` \x83\x01` \x86\x01a\n\x92V\xFE\xA2dipfsX\"\x12 \xCC\x7FN\x1B\xE9\\*\xDC\xD9I7\xE2r \x9AC\x01\xB0\xB1\xFA9\x19\xBF\xCD\xAD\xE5\x01\x0C/\xE2N\x1AdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static STRICTBANK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StrictBank<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StrictBank<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StrictBank<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StrictBank<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StrictBank<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StrictBank)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StrictBank<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STRICTBANK_ABI.clone(),
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
                STRICTBANK_ABI.clone(),
                STRICTBANK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dataStore` (0x660d0d67) function
        pub fn data_store(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([102, 13, 13, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordTransferIn` (0x352f9aed) function
        pub fn record_transfer_in(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 47, 154, 237], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordTransferOut` (0x8c1b5fde) function
        pub fn record_transfer_out(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 27, 95, 222], token)
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
        ///Calls the contract's `syncTokenBalance` (0xeb40133f) function
        pub fn sync_token_balance(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 64, 19, 63], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenBalances` (0x523fba7f) function
        pub fn token_balances(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([82, 63, 186, 127], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOut` (0x078d3b79) function
        pub fn transfer_out(
            &self,
            token: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 141, 59, 121], (token, receiver, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TokenTransferReverted` event
        pub fn token_transfer_reverted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenTransferRevertedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenTransferRevertedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StrictBank<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EmptyReceiver` with signature `EmptyReceiver()` and selector `0xd551823d`
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
    #[etherror(name = "EmptyReceiver", abi = "EmptyReceiver()")]
    pub struct EmptyReceiver;
    ///Custom Error type `InvalidNativeTokenSender` with signature `InvalidNativeTokenSender(address)` and selector `0xe71a51be`
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
        name = "InvalidNativeTokenSender",
        abi = "InvalidNativeTokenSender(address)"
    )]
    pub struct InvalidNativeTokenSender {
        pub msg_sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `SelfTransferNotSupported` with signature `SelfTransferNotSupported(address)` and selector `0xe70f9152`
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
        name = "SelfTransferNotSupported",
        abi = "SelfTransferNotSupported(address)"
    )]
    pub struct SelfTransferNotSupported {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `TokenTransferError` with signature `TokenTransferError(address,address,uint256)` and selector `0x979dc780`
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
        name = "TokenTransferError",
        abi = "TokenTransferError(address,address,uint256)"
    )]
    pub struct TokenTransferError {
        pub token: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum StrictBankErrors {
        EmptyReceiver(EmptyReceiver),
        InvalidNativeTokenSender(InvalidNativeTokenSender),
        SelfTransferNotSupported(SelfTransferNotSupported),
        TokenTransferError(TokenTransferError),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for StrictBankErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EmptyReceiver as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyReceiver(decoded));
            }
            if let Ok(decoded) = <InvalidNativeTokenSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidNativeTokenSender(decoded));
            }
            if let Ok(decoded) = <SelfTransferNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelfTransferNotSupported(decoded));
            }
            if let Ok(decoded) = <TokenTransferError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransferError(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StrictBankErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EmptyReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNativeTokenSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfTransferNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for StrictBankErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EmptyReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNativeTokenSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SelfTransferNotSupported as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for StrictBankErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmptyReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNativeTokenSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfTransferNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenTransferError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for StrictBankErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EmptyReceiver> for StrictBankErrors {
        fn from(value: EmptyReceiver) -> Self {
            Self::EmptyReceiver(value)
        }
    }
    impl ::core::convert::From<InvalidNativeTokenSender> for StrictBankErrors {
        fn from(value: InvalidNativeTokenSender) -> Self {
            Self::InvalidNativeTokenSender(value)
        }
    }
    impl ::core::convert::From<SelfTransferNotSupported> for StrictBankErrors {
        fn from(value: SelfTransferNotSupported) -> Self {
            Self::SelfTransferNotSupported(value)
        }
    }
    impl ::core::convert::From<TokenTransferError> for StrictBankErrors {
        fn from(value: TokenTransferError) -> Self {
            Self::TokenTransferError(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for StrictBankErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
        name = "TokenTransferReverted",
        abi = "TokenTransferReverted(string,bytes)"
    )]
    pub struct TokenTransferRevertedFilter {
        pub reason: ::std::string::String,
        pub returndata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    #[ethcall(name = "dataStore", abi = "dataStore()")]
    pub struct DataStoreCall;
    ///Container type for all input parameters for the `recordTransferIn` function with signature `recordTransferIn(address)` and selector `0x352f9aed`
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
    #[ethcall(name = "recordTransferIn", abi = "recordTransferIn(address)")]
    pub struct RecordTransferInCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `recordTransferOut` function with signature `recordTransferOut(address)` and selector `0x8c1b5fde`
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
    #[ethcall(name = "recordTransferOut", abi = "recordTransferOut(address)")]
    pub struct RecordTransferOutCall {
        pub token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `syncTokenBalance` function with signature `syncTokenBalance(address)` and selector `0xeb40133f`
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
    #[ethcall(name = "syncTokenBalance", abi = "syncTokenBalance(address)")]
    pub struct SyncTokenBalanceCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tokenBalances` function with signature `tokenBalances(address)` and selector `0x523fba7f`
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
    #[ethcall(name = "tokenBalances", abi = "tokenBalances(address)")]
    pub struct TokenBalancesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `transferOut` function with signature `transferOut(address,address,uint256)` and selector `0x078d3b79`
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
    #[ethcall(name = "transferOut", abi = "transferOut(address,address,uint256)")]
    pub struct TransferOutCall {
        pub token: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum StrictBankCalls {
        DataStore(DataStoreCall),
        RecordTransferIn(RecordTransferInCall),
        RecordTransferOut(RecordTransferOutCall),
        RoleStore(RoleStoreCall),
        SyncTokenBalance(SyncTokenBalanceCall),
        TokenBalances(TokenBalancesCall),
        TransferOut(TransferOutCall),
    }
    impl ::ethers::core::abi::AbiDecode for StrictBankCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <RecordTransferInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordTransferIn(decoded));
            }
            if let Ok(decoded) = <RecordTransferOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordTransferOut(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <SyncTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SyncTokenBalance(decoded));
            }
            if let Ok(decoded) = <TokenBalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenBalances(decoded));
            }
            if let Ok(decoded) = <TransferOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOut(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StrictBankCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordTransferIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordTransferOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SyncTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StrictBankCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordTransferIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordTransferOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::SyncTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOut(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DataStoreCall> for StrictBankCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<RecordTransferInCall> for StrictBankCalls {
        fn from(value: RecordTransferInCall) -> Self {
            Self::RecordTransferIn(value)
        }
    }
    impl ::core::convert::From<RecordTransferOutCall> for StrictBankCalls {
        fn from(value: RecordTransferOutCall) -> Self {
            Self::RecordTransferOut(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for StrictBankCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<SyncTokenBalanceCall> for StrictBankCalls {
        fn from(value: SyncTokenBalanceCall) -> Self {
            Self::SyncTokenBalance(value)
        }
    }
    impl ::core::convert::From<TokenBalancesCall> for StrictBankCalls {
        fn from(value: TokenBalancesCall) -> Self {
            Self::TokenBalances(value)
        }
    }
    impl ::core::convert::From<TransferOutCall> for StrictBankCalls {
        fn from(value: TransferOutCall) -> Self {
            Self::TransferOut(value)
        }
    }
    ///Container type for all return fields from the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    pub struct DataStoreReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `recordTransferIn` function with signature `recordTransferIn(address)` and selector `0x352f9aed`
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
    pub struct RecordTransferInReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `recordTransferOut` function with signature `recordTransferOut(address)` and selector `0x8c1b5fde`
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
    pub struct RecordTransferOutReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `syncTokenBalance` function with signature `syncTokenBalance(address)` and selector `0xeb40133f`
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
    pub struct SyncTokenBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenBalances` function with signature `tokenBalances(address)` and selector `0x523fba7f`
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
    pub struct TokenBalancesReturn(pub ::ethers::core::types::U256);
}
