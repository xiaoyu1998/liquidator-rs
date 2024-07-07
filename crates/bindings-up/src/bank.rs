pub use bank::*;
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
pub mod bank {
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
    pub static BANK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\tH8\x03\x80a\tH\x839\x81\x01`@\x81\x90Ra\0/\x91a\0^V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x98V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\0qW`\0\x80\xFD[\x82Qa\0|\x81a\0FV[` \x84\x01Q\x90\x92Pa\0\x8D\x81a\0FV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x08\x7Fa\0\xC9`\09`\0\x81\x81`D\x01Ra\x01#\x01R`\0\x81\x81`\xD3\x01Ra\x02\x93\x01Ra\x08\x7F`\0\xF3\xFE`\x80`@R`\x046\x10a\08W`\x005`\xE0\x1C\x80c\x07\x8D;y\x14a\0\xA1W\x80cJJ{\x04\x14a\0\xC1W\x80cf\r\rg\x14a\x01\x11W`\0\x80\xFD[6a\0\x9CW`\0a\0h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01EV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\0\x9AW`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0\x9Aa\0\xBC6`\x04a\x06:V[a\x01\xF8V[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x01}\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF2\x91\x90a\x06{V[\x92\x91PPV[a\x02g`@Q` \x01a\x02)\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x02wV[a\x02r\x83\x83\x83a\x03+V[PPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x06\x91\x90a\x06\x9FV[a\x03'W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\0\x91\x92\x91\x90a\x07\x11V[PPV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x03_W`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\0\x91V[a\x02r\x83\x83\x83\x80`\0\x03a\x03rWPPPV[a\x03{\x82a\x04\x17V[`\0\x80a\x03\x89\x85\x85\x85a\x04>V[\x91P\x91P\x81\x15a\x03\x9AWPPPPPV[`\0a\x03\xA5\x82a\x05\x95V[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x03\xD9\x92\x91\x90a\x07=V[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\0\x91V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x14W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x04\xA1\x90\x85\x90a\x07kV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x04\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xE3V[``\x91P[P\x91P\x91P\x81\x15a\x05\x84W\x80Q`\0\x03a\x05AW`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x05AW`\0`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\x05\x8DV[`\0\x81Q\x11\x80\x15a\x05cWP\x80\x80` \x01\x90Q\x81\x01\x90a\x05a\x91\x90a\x06\x9FV[\x15[\x15a\x05vW`\0\x94P\x92Pa\x05\x8D\x91PPV[`\x01\x94P\x92Pa\x05\x8D\x91PPV[`\0\x94P\x92PPP[\x93P\x93\x91PPV[```\0`D\x83Q\x10\x15a\x05\xBCWPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x05\xC9\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x06\x08W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x05\xFD\x91\x90a\x07\x9DV[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x14W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06OW`\0\x80\xFD[\x835a\x06Z\x81a\x06%V[\x92P` \x84\x015a\x06j\x81a\x06%V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x06\x8DW`\0\x80\xFD[\x81Qa\x06\x98\x81a\x06%V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\x98W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x06\xDCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xC4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xFD\x81` \x86\x01` \x86\x01a\x06\xC1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x075\x90\x83\x01\x84a\x06\xE5V[\x94\x93PPPPV[`@\x81R`\0a\x07P`@\x83\x01\x85a\x06\xE5V[\x82\x81\x03` \x84\x01Ra\x07b\x81\x85a\x06\xE5V[\x95\x94PPPPPV[`\0\x82Qa\x07}\x81\x84` \x87\x01a\x06\xC1V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xAFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xC6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x07\xD7W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF1Wa\x07\xF1a\x07\x87V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08 Wa\x08 a\x07\x87V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x088W`\0\x80\xFD[a\x07b\x82` \x83\x01` \x86\x01a\x06\xC1V\xFE\xA2dipfsX\"\x12 \xFA=^\xDE\xFBn\x9Am\xE8\xD8\x162\xF4\xF2\xED\xC1\x0F\xB6\xBF\x1C\x9F\xCF\xCC\x9DmZ4\xC1C\xB2\x9A1dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static BANK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\08W`\x005`\xE0\x1C\x80c\x07\x8D;y\x14a\0\xA1W\x80cJJ{\x04\x14a\0\xC1W\x80cf\r\rg\x14a\x01\x11W`\0\x80\xFD[6a\0\x9CW`\0a\0h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01EV[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\0\x9AW`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0\x9Aa\0\xBC6`\x04a\x06:V[a\x01\xF8V[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x1DW`\0\x80\xFD[Pa\0\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x01}\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF2\x91\x90a\x06{V[\x92\x91PPV[a\x02g`@Q` \x01a\x02)\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x02wV[a\x02r\x83\x83\x83a\x03+V[PPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x06\x91\x90a\x06\x9FV[a\x03'W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\0\x91\x92\x91\x90a\x07\x11V[PPV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x03_W`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\0\x91V[a\x02r\x83\x83\x83\x80`\0\x03a\x03rWPPPV[a\x03{\x82a\x04\x17V[`\0\x80a\x03\x89\x85\x85\x85a\x04>V[\x91P\x91P\x81\x15a\x03\x9AWPPPPPV[`\0a\x03\xA5\x82a\x05\x95V[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x03\xD9\x92\x91\x90a\x07=V[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\0\x91V[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x14W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x04\xA1\x90\x85\x90a\x07kV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x04\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\xE3V[``\x91P[P\x91P\x91P\x81\x15a\x05\x84W\x80Q`\0\x03a\x05AW`\x01`\x01`\xA0\x1B\x03\x88\x16;a\x05AW`\0`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\x05\x8DV[`\0\x81Q\x11\x80\x15a\x05cWP\x80\x80` \x01\x90Q\x81\x01\x90a\x05a\x91\x90a\x06\x9FV[\x15[\x15a\x05vW`\0\x94P\x92Pa\x05\x8D\x91PPV[`\x01\x94P\x92Pa\x05\x8D\x91PPV[`\0\x94P\x92PPP[\x93P\x93\x91PPV[```\0`D\x83Q\x10\x15a\x05\xBCWPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x05\xC9\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x06\x08W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x05\xFD\x91\x90a\x07\x9DV[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x14W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06OW`\0\x80\xFD[\x835a\x06Z\x81a\x06%V[\x92P` \x84\x015a\x06j\x81a\x06%V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\x06\x8DW`\0\x80\xFD[\x81Qa\x06\x98\x81a\x06%V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\x98W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x06\xDCW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xC4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xFD\x81` \x86\x01` \x86\x01a\x06\xC1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x075\x90\x83\x01\x84a\x06\xE5V[\x94\x93PPPPV[`@\x81R`\0a\x07P`@\x83\x01\x85a\x06\xE5V[\x82\x81\x03` \x84\x01Ra\x07b\x81\x85a\x06\xE5V[\x95\x94PPPPPV[`\0\x82Qa\x07}\x81\x84` \x87\x01a\x06\xC1V[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xAFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xC6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x07\xD7W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF1Wa\x07\xF1a\x07\x87V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08 Wa\x08 a\x07\x87V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x088W`\0\x80\xFD[a\x07b\x82` \x83\x01` \x86\x01a\x06\xC1V\xFE\xA2dipfsX\"\x12 \xFA=^\xDE\xFBn\x9Am\xE8\xD8\x162\xF4\xF2\xED\xC1\x0F\xB6\xBF\x1C\x9F\xCF\xCC\x9DmZ4\xC1C\xB2\x9A1dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static BANK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Bank<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Bank<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Bank<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Bank<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Bank<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Bank)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Bank<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BANK_ABI.clone(),
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
                BANK_ABI.clone(),
                BANK_BYTECODE.clone().into(),
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
    for Bank<M> {
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
    pub enum BankErrors {
        EmptyReceiver(EmptyReceiver),
        InvalidNativeTokenSender(InvalidNativeTokenSender),
        SelfTransferNotSupported(SelfTransferNotSupported),
        TokenTransferError(TokenTransferError),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BankErrors {
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
    impl ::ethers::core::abi::AbiEncode for BankErrors {
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
    impl ::ethers::contract::ContractRevert for BankErrors {
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
    impl ::core::fmt::Display for BankErrors {
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
    impl ::core::convert::From<::std::string::String> for BankErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EmptyReceiver> for BankErrors {
        fn from(value: EmptyReceiver) -> Self {
            Self::EmptyReceiver(value)
        }
    }
    impl ::core::convert::From<InvalidNativeTokenSender> for BankErrors {
        fn from(value: InvalidNativeTokenSender) -> Self {
            Self::InvalidNativeTokenSender(value)
        }
    }
    impl ::core::convert::From<SelfTransferNotSupported> for BankErrors {
        fn from(value: SelfTransferNotSupported) -> Self {
            Self::SelfTransferNotSupported(value)
        }
    }
    impl ::core::convert::From<TokenTransferError> for BankErrors {
        fn from(value: TokenTransferError) -> Self {
            Self::TokenTransferError(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for BankErrors {
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
    pub enum BankCalls {
        DataStore(DataStoreCall),
        RoleStore(RoleStoreCall),
        TransferOut(TransferOutCall),
    }
    impl ::ethers::core::abi::AbiDecode for BankCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <TransferOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOut(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BankCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BankCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOut(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DataStoreCall> for BankCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for BankCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<TransferOutCall> for BankCalls {
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
