pub use role_store::*;
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
pub mod role_store {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getRoleCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleCount"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roleKey"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMembers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMembers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roleKey"),
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
                    ::std::borrow::ToOwned::to_owned("getRoles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoles"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roleKey"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roleKey"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("roleKey"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ThereMustBeAtLeastOneRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ThereMustBeAtLeastOneRoleAdmin",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ThereMustBeAtLeastOneTimelockMultiSig",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ThereMustBeAtLeastOneTimelockMultiSig",
                            ),
                            inputs: ::std::vec![],
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ROLESTORE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0d3`@Q` \x01a\0C\x90` \x80\x82R`\n\x90\x82\x01Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\0i` \x1B` \x1CV[a\x013V[a\0t`\0\x82a\0\xBEV[P`\0\x81\x81R`\x02` R`@\x90 a\0\x8D\x90\x83a\0\xD3V[P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0a\0\xCA\x83\x83a\0\xE4V[\x90P[\x92\x91PPV[`\0a\0\xCA\x83`\x01`\x01`\xA0\x1B\x03\x84\x16[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x01+WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\0\xCDV[P`\0a\0\xCDV[a\t4\x80a\x01B`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x83\xD33\x19\x11a\0[W\x80c\x83\xD33\x19\x14a\0\xE0W\x80c\xAB'B\xDC\x14a\0\xF6W\x80c\xACJ\xB3\xFB\x14a\x01\tW\x80c\xCA\x15\xC8s\x14a\x01RW`\0\x80\xFD[\x80c \x8D\xD1\xFF\x14a\0\x82W\x80c*\x86\x1FW\x14a\0\x97W\x80c\x82\x1C\x18\x98\x14a\0\xC0W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x07\x15V[a\x01eV[\0[a\0\xAAa\0\xA56`\x04a\x07MV[a\x01\xEFV[`@Qa\0\xB7\x91\x90a\x07yV[`@Q\x80\x91\x03\x90\xF3[a\0\xD3a\0\xCE6`\x04a\x07\xC5V[a\x02\x13V[`@Qa\0\xB7\x91\x90a\x07\xE7V[a\0\xE8a\x02*V[`@Q\x90\x81R` \x01a\0\xB7V[a\0\x95a\x01\x046`\x04a\x07\x15V[a\x02;V[a\x01Ba\x01\x176`\x04a\x07\x15V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\0\xB7V[a\0\xE8a\x01`6`\x04a\x08\x1FV[a\x02wV[a\x01\xB93`@Q` \x01a\x01x\x90a\x088V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[a\x01\xE1W3`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD8\x91\x90a\x08bV[`@Q\x80\x91\x03\x90\xFD[a\x01\xEB\x82\x82a\x02\x8EV[PPV[`\0\x83\x81R`\x02` R`@\x90 ``\x90a\x02\x0B\x90\x84\x84a\x03\x97V[\x94\x93PPPPV[``a\x02!`\0\x84\x84a\x04[V[\x90P[\x92\x91PPV[`\0a\x026`\0a\x05\x08V[\x90P\x90V[a\x02N3`@Q` \x01a\x01x\x90a\x088V[a\x02mW3`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD8\x91\x90a\x08bV[a\x01\xEB\x82\x82a\x05\x12V[`\0\x81\x81R`\x02` R`@\x81 a\x02$\x90a\x05\x08V[`\0\x81\x81R`\x02` R`@\x90 a\x02\xA6\x90\x83a\x05gV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16\x90U`\x02\x90\x91R\x90 a\x02\xE2\x90a\x05\x08V[`\0\x03a\x01\xEBW`@Q` \x01a\x02\xF8\x90a\x088V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x03a\x03-W`@Qc[\xC1\xE4E`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q` \x01a\x03b\x90` \x80\x82R`\x11\x90\x82\x01RpTIMELOCK_MULTISIG`x\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x03a\x01\xEBW`@Qc\x02\x82\xB5\xB7`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0a\x03\xA4\x85a\x05\x08V[\x90P\x80\x83\x11\x15a\x03\xB2W\x80\x92P[`\0a\x03\xBE\x85\x85a\x08\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD6Wa\x03\xD6a\x08\xBCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x04QWa\x04\x16\x87\x82a\x05|V[\x82a\x04!\x88\x84a\x08\x9BV[\x81Q\x81\x10a\x041Wa\x041a\x08\xD2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x04\x04V[P\x95\x94PPPPPV[```\0a\x04h\x85a\x05\x08V[\x90P\x80\x83\x11\x15a\x04vW\x80\x92P[`\0a\x04\x82\x85\x85a\x08\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x9AWa\x04\x9Aa\x08\xBCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x04QWa\x04\xDA\x87\x82a\x05|V[\x82a\x04\xE5\x88\x84a\x08\x9BV[\x81Q\x81\x10a\x04\xF5Wa\x04\xF5a\x08\xD2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xC8V[`\0a\x02$\x82T\x90V[a\x05\x1D`\0\x82a\x05\x88V[P`\0\x81\x81R`\x02` R`@\x90 a\x056\x90\x83a\x05\x94V[P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0a\x02!\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x05\xA9V[`\0a\x02!\x83\x83a\x06\x9CV[`\0a\x02!\x83\x83a\x06\xC6V[`\0a\x02!\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xC6V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x06\x92W`\0a\x05\xCD`\x01\x83a\x08\x9BV[\x85T\x90\x91P`\0\x90a\x05\xE1\x90`\x01\x90a\x08\x9BV[\x90P\x81\x81\x14a\x06FW`\0\x86`\0\x01\x82\x81T\x81\x10a\x06\x01Wa\x06\x01a\x08\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x06$Wa\x06$a\x08\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x06WWa\x06Wa\x08\xE8V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02$V[`\0\x91PPa\x02$V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x06\xB3Wa\x06\xB3a\x08\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x07\rWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02$V[P`\0a\x02$V[`\0\x80`@\x83\x85\x03\x12\x15a\x07(W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07?W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07bW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\x93V[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xD8W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07\xBAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x08\x01V[`\0` \x82\x84\x03\x12\x15a\x081W`\0\x80\xFD[P5\x91\x90PV[` \x81R`\0a\x02$` \x83\x01`\n\x81Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R`@` \x82\x01\x81\x90R`\n\x90\x82\x01Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B``\x82\x01R`\0`\x80\x82\x01a\x02!V[\x81\x81\x03\x81\x81\x11\x15a\x02$WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 R\xE51\xD0\0mi}\x04\xFFI\xA4\xF7P+j\xE6\x0C\xB5\xAE6\x01Hy\x8D\xF4cd_\x8ES\xA4dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ROLESTORE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x83\xD33\x19\x11a\0[W\x80c\x83\xD33\x19\x14a\0\xE0W\x80c\xAB'B\xDC\x14a\0\xF6W\x80c\xACJ\xB3\xFB\x14a\x01\tW\x80c\xCA\x15\xC8s\x14a\x01RW`\0\x80\xFD[\x80c \x8D\xD1\xFF\x14a\0\x82W\x80c*\x86\x1FW\x14a\0\x97W\x80c\x82\x1C\x18\x98\x14a\0\xC0W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x07\x15V[a\x01eV[\0[a\0\xAAa\0\xA56`\x04a\x07MV[a\x01\xEFV[`@Qa\0\xB7\x91\x90a\x07yV[`@Q\x80\x91\x03\x90\xF3[a\0\xD3a\0\xCE6`\x04a\x07\xC5V[a\x02\x13V[`@Qa\0\xB7\x91\x90a\x07\xE7V[a\0\xE8a\x02*V[`@Q\x90\x81R` \x01a\0\xB7V[a\0\x95a\x01\x046`\x04a\x07\x15V[a\x02;V[a\x01Ba\x01\x176`\x04a\x07\x15V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\0\xB7V[a\0\xE8a\x01`6`\x04a\x08\x1FV[a\x02wV[a\x01\xB93`@Q` \x01a\x01x\x90a\x088V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T`\xFF\x16\x90V[a\x01\xE1W3`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD8\x91\x90a\x08bV[`@Q\x80\x91\x03\x90\xFD[a\x01\xEB\x82\x82a\x02\x8EV[PPV[`\0\x83\x81R`\x02` R`@\x90 ``\x90a\x02\x0B\x90\x84\x84a\x03\x97V[\x94\x93PPPPV[``a\x02!`\0\x84\x84a\x04[V[\x90P[\x92\x91PPV[`\0a\x026`\0a\x05\x08V[\x90P\x90V[a\x02N3`@Q` \x01a\x01x\x90a\x088V[a\x02mW3`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD8\x91\x90a\x08bV[a\x01\xEB\x82\x82a\x05\x12V[`\0\x81\x81R`\x02` R`@\x81 a\x02$\x90a\x05\x08V[`\0\x81\x81R`\x02` R`@\x90 a\x02\xA6\x90\x83a\x05gV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16\x90U`\x02\x90\x91R\x90 a\x02\xE2\x90a\x05\x08V[`\0\x03a\x01\xEBW`@Q` \x01a\x02\xF8\x90a\x088V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x03a\x03-W`@Qc[\xC1\xE4E`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q` \x01a\x03b\x90` \x80\x82R`\x11\x90\x82\x01RpTIMELOCK_MULTISIG`x\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81\x03a\x01\xEBW`@Qc\x02\x82\xB5\xB7`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\0a\x03\xA4\x85a\x05\x08V[\x90P\x80\x83\x11\x15a\x03\xB2W\x80\x92P[`\0a\x03\xBE\x85\x85a\x08\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD6Wa\x03\xD6a\x08\xBCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xFFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x04QWa\x04\x16\x87\x82a\x05|V[\x82a\x04!\x88\x84a\x08\x9BV[\x81Q\x81\x10a\x041Wa\x041a\x08\xD2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x04\x04V[P\x95\x94PPPPPV[```\0a\x04h\x85a\x05\x08V[\x90P\x80\x83\x11\x15a\x04vW\x80\x92P[`\0a\x04\x82\x85\x85a\x08\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x9AWa\x04\x9Aa\x08\xBCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xC3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84[\x84\x81\x10\x15a\x04QWa\x04\xDA\x87\x82a\x05|V[\x82a\x04\xE5\x88\x84a\x08\x9BV[\x81Q\x81\x10a\x04\xF5Wa\x04\xF5a\x08\xD2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xC8V[`\0a\x02$\x82T\x90V[a\x05\x1D`\0\x82a\x05\x88V[P`\0\x81\x81R`\x02` R`@\x90 a\x056\x90\x83a\x05\x94V[P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0a\x02!\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x05\xA9V[`\0a\x02!\x83\x83a\x06\x9CV[`\0a\x02!\x83\x83a\x06\xC6V[`\0a\x02!\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x06\xC6V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x06\x92W`\0a\x05\xCD`\x01\x83a\x08\x9BV[\x85T\x90\x91P`\0\x90a\x05\xE1\x90`\x01\x90a\x08\x9BV[\x90P\x81\x81\x14a\x06FW`\0\x86`\0\x01\x82\x81T\x81\x10a\x06\x01Wa\x06\x01a\x08\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x06$Wa\x06$a\x08\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x06WWa\x06Wa\x08\xE8V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02$V[`\0\x91PPa\x02$V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x06\xB3Wa\x06\xB3a\x08\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x07\rWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02$V[P`\0a\x02$V[`\0\x80`@\x83\x85\x03\x12\x15a\x07(W`\0\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07?W`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x07bW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x07\x93V[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xD8W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x07\xBAW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x08\x01V[`\0` \x82\x84\x03\x12\x15a\x081W`\0\x80\xFD[P5\x91\x90PV[` \x81R`\0a\x02$` \x83\x01`\n\x81Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R`@` \x82\x01\x81\x90R`\n\x90\x82\x01Ri)'\xA6\"\xAF\xA0\xA2&\xA4\xA7`\xB1\x1B``\x82\x01R`\0`\x80\x82\x01a\x02!V[\x81\x81\x03\x81\x81\x11\x15a\x02$WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 R\xE51\xD0\0mi}\x04\xFFI\xA4\xF7P+j\xE6\x0C\xB5\xAE6\x01Hy\x8D\xF4cd_\x8ES\xA4dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ROLESTORE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RoleStore<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RoleStore<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RoleStore<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RoleStore<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RoleStore<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RoleStore)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RoleStore<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROLESTORE_ABI.clone(),
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
                ROLESTORE_ABI.clone(),
                ROLESTORE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getRoleCount` (0x83d33319) function
        pub fn get_role_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 211, 51, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMembers` (0x2a861f57) function
        pub fn get_role_members(
            &self,
            role_key: [u8; 32],
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([42, 134, 31, 87], (role_key, start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoles` (0x821c1898) function
        pub fn get_roles(
            &self,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([130, 28, 24, 152], (start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0xab2742dc) function
        pub fn grant_role(
            &self,
            account: ::ethers::core::types::Address,
            role_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 39, 66, 220], (account, role_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0xac4ab3fb) function
        pub fn has_role(
            &self,
            account: ::ethers::core::types::Address,
            role_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 74, 179, 251], (account, role_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0x208dd1ff) function
        pub fn revoke_role(
            &self,
            account: ::ethers::core::types::Address,
            role_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 141, 209, 255], (account, role_key))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RoleStore<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ThereMustBeAtLeastOneRoleAdmin` with signature `ThereMustBeAtLeastOneRoleAdmin()` and selector `0xb783c88a`
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
        name = "ThereMustBeAtLeastOneRoleAdmin",
        abi = "ThereMustBeAtLeastOneRoleAdmin()"
    )]
    pub struct ThereMustBeAtLeastOneRoleAdmin;
    ///Custom Error type `ThereMustBeAtLeastOneTimelockMultiSig` with signature `ThereMustBeAtLeastOneTimelockMultiSig()` and selector `0x282b5b70`
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
        name = "ThereMustBeAtLeastOneTimelockMultiSig",
        abi = "ThereMustBeAtLeastOneTimelockMultiSig()"
    )]
    pub struct ThereMustBeAtLeastOneTimelockMultiSig;
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
    pub enum RoleStoreErrors {
        ThereMustBeAtLeastOneRoleAdmin(ThereMustBeAtLeastOneRoleAdmin),
        ThereMustBeAtLeastOneTimelockMultiSig(ThereMustBeAtLeastOneTimelockMultiSig),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RoleStoreErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ThereMustBeAtLeastOneRoleAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ThereMustBeAtLeastOneRoleAdmin(decoded));
            }
            if let Ok(decoded) = <ThereMustBeAtLeastOneTimelockMultiSig as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ThereMustBeAtLeastOneTimelockMultiSig(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoleStoreErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ThereMustBeAtLeastOneRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ThereMustBeAtLeastOneTimelockMultiSig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RoleStoreErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ThereMustBeAtLeastOneRoleAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ThereMustBeAtLeastOneTimelockMultiSig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RoleStoreErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ThereMustBeAtLeastOneRoleAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ThereMustBeAtLeastOneTimelockMultiSig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RoleStoreErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ThereMustBeAtLeastOneRoleAdmin> for RoleStoreErrors {
        fn from(value: ThereMustBeAtLeastOneRoleAdmin) -> Self {
            Self::ThereMustBeAtLeastOneRoleAdmin(value)
        }
    }
    impl ::core::convert::From<ThereMustBeAtLeastOneTimelockMultiSig>
    for RoleStoreErrors {
        fn from(value: ThereMustBeAtLeastOneTimelockMultiSig) -> Self {
            Self::ThereMustBeAtLeastOneTimelockMultiSig(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for RoleStoreErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    ///Container type for all input parameters for the `getRoleCount` function with signature `getRoleCount()` and selector `0x83d33319`
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
    #[ethcall(name = "getRoleCount", abi = "getRoleCount()")]
    pub struct GetRoleCountCall;
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMembers` function with signature `getRoleMembers(bytes32,uint256,uint256)` and selector `0x2a861f57`
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
    #[ethcall(name = "getRoleMembers", abi = "getRoleMembers(bytes32,uint256,uint256)")]
    pub struct GetRoleMembersCall {
        pub role_key: [u8; 32],
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoles` function with signature `getRoles(uint256,uint256)` and selector `0x821c1898`
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
    #[ethcall(name = "getRoles", abi = "getRoles(uint256,uint256)")]
    pub struct GetRolesCall {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(address,bytes32)` and selector `0xab2742dc`
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
    #[ethcall(name = "grantRole", abi = "grantRole(address,bytes32)")]
    pub struct GrantRoleCall {
        pub account: ::ethers::core::types::Address,
        pub role_key: [u8; 32],
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(address,bytes32)` and selector `0xac4ab3fb`
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
    #[ethcall(name = "hasRole", abi = "hasRole(address,bytes32)")]
    pub struct HasRoleCall {
        pub account: ::ethers::core::types::Address,
        pub role_key: [u8; 32],
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(address,bytes32)` and selector `0x208dd1ff`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(address,bytes32)")]
    pub struct RevokeRoleCall {
        pub account: ::ethers::core::types::Address,
        pub role_key: [u8; 32],
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
    pub enum RoleStoreCalls {
        GetRoleCount(GetRoleCountCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetRoleMembers(GetRoleMembersCall),
        GetRoles(GetRolesCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RevokeRole(RevokeRoleCall),
    }
    impl ::ethers::core::abi::AbiDecode for RoleStoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetRoleCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleCount(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) = <GetRoleMembersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMembers(decoded));
            }
            if let Ok(decoded) = <GetRolesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoles(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoleStoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetRoleCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMembers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RoleStoreCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetRoleCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleMembers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoles(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetRoleCountCall> for RoleStoreCalls {
        fn from(value: GetRoleCountCall) -> Self {
            Self::GetRoleCount(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for RoleStoreCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GetRoleMembersCall> for RoleStoreCalls {
        fn from(value: GetRoleMembersCall) -> Self {
            Self::GetRoleMembers(value)
        }
    }
    impl ::core::convert::From<GetRolesCall> for RoleStoreCalls {
        fn from(value: GetRolesCall) -> Self {
            Self::GetRoles(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for RoleStoreCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for RoleStoreCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for RoleStoreCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    ///Container type for all return fields from the `getRoleCount` function with signature `getRoleCount()` and selector `0x83d33319`
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
    pub struct GetRoleCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRoleMembers` function with signature `getRoleMembers(bytes32,uint256,uint256)` and selector `0x2a861f57`
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
    pub struct GetRoleMembersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getRoles` function with signature `getRoles(uint256,uint256)` and selector `0x821c1898`
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
    pub struct GetRolesReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(address,bytes32)` and selector `0xac4ab3fb`
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
    pub struct HasRoleReturn(pub bool);
}
