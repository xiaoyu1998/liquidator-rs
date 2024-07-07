pub use base_router::*;
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
pub mod base_router {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_router"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Router"),
                        ),
                    },
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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
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
                    ::std::borrow::ToOwned::to_owned("router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Router"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendTokens"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASEROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08b8\x03\x80a\x08b\x839\x81\x01`@\x81\x90Ra\0/\x91a\0iV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xB6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0fW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0~W`\0\x80\xFD[\x83Qa\0\x89\x81a\0QV[` \x85\x01Q\x90\x93Pa\0\x9A\x81a\0QV[`@\x85\x01Q\x90\x92Pa\0\xAB\x81a\0QV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x07xa\0\xEA`\09`\0`\xB2\x01R`\0\x81\x81a\x01\x1B\x01Ra\x02\x99\x01R`\0`a\x01Ra\x07x`\0\xF3\xFE`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80cJJ{\x04\x14a\0OW\x80cf\r\rg\x14a\0\xA0W\x80c\xAC\x96P\xD8\x14a\0\xD4W\x80c\xE6\xD6j\xC8\x14a\0\xF4W\x80c\xF8\x87\xEA@\x14a\x01\tW[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xACW`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xE7a\0\xE26`\x04a\x04eV[a\x01=V[`@Qa\0\x97\x91\x90a\x05,V[a\x01\x07a\x01\x026`\x04a\x05\xADV[a\x02QV[\0[4\x80\x15a\x01\x15W`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01XWa\x01Xa\x05\xE9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01vW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02JW`\0\x800\x86\x86\x85\x81\x81\x10a\x01\xAFWa\x01\xAFa\x05\xFFV[\x90P` \x02\x81\x01\x90a\x01\xC1\x91\x90a\x06\x15V[`@Qa\x01\xCF\x92\x91\x90a\x06cV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x0FV[``\x91P[P\x91P\x91P\x81a\x02\"Wa\x02\"\x81a\x03\x07V[\x80\x84\x84\x81Q\x81\x10a\x025Wa\x025a\x05\xFFV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x01\x91V[P\x92\x91PPV[a\x02Ya\x03IV[a\x02b\x82a\x03\xA2V[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF3W=`\0\x80>=`\0\xFD[PPPPPa\x03\x02`\x01`\0UV[PPPV[`\0\x80a\x03\x13\x83a\x03\xCCV[\x91P\x91P\x80\x15a\x03@W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x037\x91\x90a\x06sV[`@Q\x80\x91\x03\x90\xFD[a\x03\x02\x83a\x04\\V[`\x02`\0T\x03a\x03\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x037V[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xC9W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[```\0`D\x83Q\x10\x15a\x03\xF3WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x04\0\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x04?W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x044\x91\x90a\x06\x8DV[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x04xW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x04\xA0W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xB7W`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x04\xCCW`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a\x04\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\x18\x81` \x86\x01` \x86\x01a\x04\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x05\x85W`?\x19\x87\x86\x03\x01\x84Ra\x05p\x85\x83Qa\x05\0V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x05TV[P\x92\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA8W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\xC2W`\0\x80\xFD[a\x05\xCB\x84a\x05\x91V[\x92Pa\x05\xD9` \x85\x01a\x05\x91V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06,W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06GW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06\\W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R`\0a\x06\x86` \x83\x01\x84a\x05\0V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x9FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xB6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x06\xC7W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE1Wa\x06\xE1a\x05\xE9V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x10Wa\x07\x10a\x05\xE9V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x07(W`\0\x80\xFD[a\x079\x82` \x83\x01` \x86\x01a\x04\xDCV[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 RK\x1E\x8F@\x04\x10:UY\xAB\x9C\xBA\xABDuS&\r\xE1\xB2]\xDB\xED%\xA4C?\xEB\xFB\xCBVdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static BASEROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80cJJ{\x04\x14a\0OW\x80cf\r\rg\x14a\0\xA0W\x80c\xAC\x96P\xD8\x14a\0\xD4W\x80c\xE6\xD6j\xC8\x14a\0\xF4W\x80c\xF8\x87\xEA@\x14a\x01\tW[`\0\x80\xFD[4\x80\x15a\0[W`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xACW`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xE7a\0\xE26`\x04a\x04eV[a\x01=V[`@Qa\0\x97\x91\x90a\x05,V[a\x01\x07a\x01\x026`\x04a\x05\xADV[a\x02QV[\0[4\x80\x15a\x01\x15W`\0\x80\xFD[Pa\0\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01XWa\x01Xa\x05\xE9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8BW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01vW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02JW`\0\x800\x86\x86\x85\x81\x81\x10a\x01\xAFWa\x01\xAFa\x05\xFFV[\x90P` \x02\x81\x01\x90a\x01\xC1\x91\x90a\x06\x15V[`@Qa\x01\xCF\x92\x91\x90a\x06cV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x02\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\x0FV[``\x91P[P\x91P\x91P\x81a\x02\"Wa\x02\"\x81a\x03\x07V[\x80\x84\x84\x81Q\x81\x10a\x025Wa\x025a\x05\xFFV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x01\x91V[P\x92\x91PPV[a\x02Ya\x03IV[a\x02b\x82a\x03\xA2V[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF3W=`\0\x80>=`\0\xFD[PPPPPa\x03\x02`\x01`\0UV[PPPV[`\0\x80a\x03\x13\x83a\x03\xCCV[\x91P\x91P\x80\x15a\x03@W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x037\x91\x90a\x06sV[`@Q\x80\x91\x03\x90\xFD[a\x03\x02\x83a\x04\\V[`\x02`\0T\x03a\x03\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x037V[`\x02`\0UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03\xC9W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[```\0`D\x83Q\x10\x15a\x03\xF3WPP`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x90\x92\x90\x91PV[`\0a\x04\0\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x04?W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x044\x91\x90a\x06\x8DV[\x94`\x01\x94P\x92PPPV[`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x04xW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x04\xA0W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xB7W`\0\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x04\xCCW`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a\x04\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\x18\x81` \x86\x01` \x86\x01a\x04\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x05\x85W`?\x19\x87\x86\x03\x01\x84Ra\x05p\x85\x83Qa\x05\0V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x05TV[P\x92\x96\x95PPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA8W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x05\xC2W`\0\x80\xFD[a\x05\xCB\x84a\x05\x91V[\x92Pa\x05\xD9` \x85\x01a\x05\x91V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06,W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06GW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06\\W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[` \x81R`\0a\x06\x86` \x83\x01\x84a\x05\0V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\x9FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xB6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x06\xC7W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE1Wa\x06\xE1a\x05\xE9V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x10Wa\x07\x10a\x05\xE9V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x07(W`\0\x80\xFD[a\x079\x82` \x83\x01` \x86\x01a\x04\xDCV[\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 RK\x1E\x8F@\x04\x10:UY\xAB\x9C\xBA\xABDuS&\r\xE1\xB2]\xDB\xED%\xA4C?\xEB\xFB\xCBVdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static BASEROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BaseRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BaseRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BaseRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BaseRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BaseRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BaseRouter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BaseRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASEROUTER_ABI.clone(),
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
                BASEROUTER_ABI.clone(),
                BASEROUTER_BYTECODE.clone().into(),
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
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
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
        ///Calls the contract's `router` (0xf887ea40) function
        pub fn router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTokens` (0xe6d66ac8) function
        pub fn send_tokens(
            &self,
            token: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 214, 106, 200], (token, receiver, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BaseRouter<M> {
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
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    ///Container type for all input parameters for the `router` function with signature `router()` and selector `0xf887ea40`
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
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    ///Container type for all input parameters for the `sendTokens` function with signature `sendTokens(address,address,uint256)` and selector `0xe6d66ac8`
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
    #[ethcall(name = "sendTokens", abi = "sendTokens(address,address,uint256)")]
    pub struct SendTokensCall {
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
    pub enum BaseRouterCalls {
        DataStore(DataStoreCall),
        Multicall(MulticallCall),
        RoleStore(RoleStoreCall),
        Router(RouterCall),
        SendTokens(SendTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for BaseRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            if let Ok(decoded) = <RouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Router(decoded));
            }
            if let Ok(decoded) = <SendTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BaseRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Router(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BaseRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DataStoreCall> for BaseRouterCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for BaseRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for BaseRouterCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    impl ::core::convert::From<RouterCall> for BaseRouterCalls {
        fn from(value: RouterCall) -> Self {
            Self::Router(value)
        }
    }
    impl ::core::convert::From<SendTokensCall> for BaseRouterCalls {
        fn from(value: SendTokensCall) -> Self {
            Self::SendTokens(value)
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
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
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
    ///Container type for all return fields from the `router` function with signature `router()` and selector `0xf887ea40`
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
    pub struct RouterReturn(pub ::ethers::core::types::Address);
}
