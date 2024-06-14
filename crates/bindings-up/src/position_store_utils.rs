pub use position_store_utils::*;
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
pub mod position_store_utils {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACCOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACCOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("ACC_LONG_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACC_LONG_AMOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("ACC_SHORT_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACC_SHORT_AMOUNT"),
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
                    ::std::borrow::ToOwned::to_owned("ENTRY_LONG_PRICE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ENTRY_LONG_PRICE"),
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
                    ::std::borrow::ToOwned::to_owned("ENTRY_SHORT_PRICE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ENTRY_SHORT_PRICE"),
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
                    ::std::borrow::ToOwned::to_owned("HAS_COLLATERAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HAS_COLLATERAL"),
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
                    ::std::borrow::ToOwned::to_owned("HAS_DEBT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HAS_DEBT"),
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
                    ::std::borrow::ToOwned::to_owned("IS_USD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_USD"),
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
                    ::std::borrow::ToOwned::to_owned("POSITION_TYPE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POSITION_TYPE"),
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
                    ::std::borrow::ToOwned::to_owned("UNDERLYING_ASSET"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("UNDERLYING_ASSET"),
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
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStoreAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Position.Props"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDebtMultiplierFactorForRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDebtMultiplierFactorForRedeem",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getHealthFactorCollateralRateThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getHealthFactorCollateralRateThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getHealthFactorLiquidationThreshold",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getHealthFactorLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PositionNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PositionNotFound"),
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
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static POSITIONSTOREUTILS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a%Ea\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01+W`\x005`\xE0\x1C\x80c{\x82\xD7N\x11a\0\xB7W\x80c\xCE\xB8\x92\x10\x11a\0{W\x80c\xCE\xB8\x92\x10\x14a\x02+W\x80c\xE6\x02S\x93\x14a\x02KW\x80c\xE6\xC8\xE3\x06\x14a\x02SW\x80c\xE9\x05$\xF2\x14a\x02sW\x80c\xEF\\\xB5=\x14a\x02{W`\0\x80\xFD[\x80c{\x82\xD7N\x14a\x01\xC8W\x80c\xA6\x1A\x94A\x14a\x01\xE8W\x80c\xB2\xAA\x83\xEC\x14a\x01\xF0W\x80c\xB3\x9DS^\x14a\x02\x10W\x80c\xB3\xB8\xC8\xE5\x14a\x02\x18W`\0\x80\xFD[\x80cC\xD6Rs\x11a\0\xFEW\x80cC\xD6Rs\x14a\x01nW\x80cQ\x14\xEE>\x14a\x01vW\x80cS\xF2\x03l\x14a\x01\x98W\x80cX\x1A*\xE8\x14a\x01\xA0W\x80cp\xB5'\x0E\x14a\x01\xA8W`\0\x80\xFD[\x80c\x011&\xB1\x14a\x010W\x80c\x13|_\xFF\x14a\x01KW\x80c/\xA3p\"\x14a\x01SW\x80c4\x9C\x1Ar\x14a\x01[W[`\0\x80\xFD[a\x018a\x02\x8EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x02\xB6V[a\x018a\x02\xC5V[a\x018a\x01i6`\x04a \x02V[a\x02\xEFV[a\x018a\x03\x8AV[\x81\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x96a\x01\x916`\x04a \x1FV[a\x03\x99V[\0[a\x018a\x048V[a\x018a\x04GV[\x81\x80\x15a\x01\xB4W`\0\x80\xFD[Pa\x01\x96a\x01\xC36`\x04a \x9CV[a\x04VV[a\x01\xDBa\x01\xD66`\x04a \x1FV[a\x0C\xEBV[`@Qa\x01B\x91\x90a!hV[a\x018a\r\x04V[\x81\x80\x15a\x01\xFCW`\0\x80\xFD[Pa\x01\x96a\x02\x0B6`\x04a!\xF1V[a\r\x13V[a\x018a\x151V[a\x018a\x02&6`\x04a \x02V[a\x15@V[\x81\x80\x15a\x027W`\0\x80\xFD[Pa\x01\x96a\x02F6`\x04a\"3V[a\x15`V[a\x018a\x15\xE8V[\x81\x80\x15a\x02_W`\0\x80\xFD[Pa\x01\x96a\x02n6`\x04a \x1FV[a\x15\xF7V[a\x018a\x16\x15V[a\x018a\x02\x896`\x04a\"tV[a\x16$V[`@Q` \x01a\x02\x9D\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x02\x9D\x90a\"\xD8V[`@Q` \x01a\x02\x9D\x90` \x80\x82R`\x06\x90\x82\x01Re\x12T\xD7\xD5T\xD1`\xD2\x1B`@\x82\x01R``\x01\x90V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\x0F\x90a#\x01V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x84\x91\x90a#BV[\x92\x91PPV[`@Q` \x01a\x02\x9D\x90a#[V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03\xB7\x90a#\x83V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x043\x91\x90a#BV[PPPV[`@Q` \x01a\x02\x9D\x90a#\xC6V[`@Q` \x01a\x02\x9D\x90a#\xE8V[`\0\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x04y\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE0W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x05\0\x84`\0\x01Qa\x16\x9DV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x05x\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x05\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x05\xF2\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x065\x91\x90a$WV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x06U\x90a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xCF\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x12\x91\x90a$WV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x072\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x07b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x07\xA4\x92`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE7\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x08\x07\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x087\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBB\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x08\xDB\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tL\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\tkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8F\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\t\xAF\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xA0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\n\x83\x90a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xC0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B7\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xAB\xFD\xCC\xED\x84`@Q` \x01a\x0BW\x90a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xCA\x92\x91\x90\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\r\x91\x90a$\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xAB\xFD\xCC\xED\x84`@Q` \x01a\x0C-\x90a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C]\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x01\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA1\x92\x91\x90\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE4\x91\x90a$\xF2V[PPPPPV[a\x0C\xF3a\x1FxV[a\x0C\xFD\x83\x83a\x17\"V[\x93\x92PPPV[`@Q` \x01a\x02\x9D\x90a$\x9EV[`\0\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\r6\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB0\x91\x90a$\xF2V[a\r\xD4W`@Qc\x04&\xCF\xFF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16c\x99!\xC3\xCC`@Q` \x01a\r\xF2\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EYW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x99!\xC3\xCCa\x0Eu\x84a\x16\x9DV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xB6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xCAW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xAFo\xB6\x84`@Q` \x01a\x0E\xED\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FQ\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FkW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xAFo\xB6\x84`@Q` \x01a\x0F\xA2\x90a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x06\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x10W\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xBB\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xE9W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x11\x0C\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11p\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x8AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\x9EW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x11\xC1\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xF1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12%\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x12v\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xDA\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xF4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x08W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x13+\x90a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x8F\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xBDW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xE7\xAC\x12\x84`@Q` \x01a\x13\xE0\x90a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x10\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14D\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14rW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xE7\xAC\x12\x84`@Q` \x01a\x14\x95\x90a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xF9\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15'W=`\0\x80>=`\0\xFD[PPPPPPPPV[`@Q` \x01a\x02\x9D\x90a$\xC8V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\x0F\x90a#\x83V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\x15x\x84a\x1F%V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE2\x91\x90a#BV[PPPPV[`@Q` \x01a\x02\x9D\x90a$tV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03\xB7\x90a#\x01V[`@Q` \x01a\x02\x9D\x90a$6V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x16>\x84a\x1F%V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFD\x91\x90a#BV[`\0`@Q` \x01a\x16\xD8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x17*a\x1FxV[\x82a\x173a\x1FxV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x17Q\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xCB\x91\x90a$\xF2V[a\x17\xD8W\x91Pa\x03\x84\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xF7\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18[\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x9C\x91\x90a$WV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`@Q\x90\x83\x16\x90c!\xF8\xA7!\x90\x86\x90a\x18\xC4\x90` \x01a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19(\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19i\x91\x90a$WV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xA5\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AJ\x91\x90a#BV[\x81`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1Ar\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x17\x91\x90a#BV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1B>\x90` \x01a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1Bn\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE3\x91\x90a#BV[`\x80\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1C\n\x90` \x01a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C:\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Cn\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xAF\x91\x90a#BV[`\xA0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1C\xD6\x90` \x01a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D:\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D{\x91\x90a#BV[`\xC0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cz\xE1\xCF\xCA\x90\x86\x90a\x1D\xA2\x90` \x01a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EG\x91\x90a$\xF2V[\x15\x15`\xE0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cz\xE1\xCF\xCA\x90\x86\x90a\x1Ep\x90` \x01a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x15\x91\x90a$\xF2V[\x15\x15a\x01\0\x82\x01R\x94\x93PPPPV[`\0`@Q` \x01a\x16\xD8\x90` \x80\x82R`'\x90\x82\x01R\x7FHEALTH_FACTOR_COLLATERAL_RATE_TH`@\x82\x01Rf\x14\x91T\xD2\x13\xD3\x11`\xCA\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80a\x01 \x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x15\x15\x81RP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\xEFW`\0\x80\xFD[PV[\x805a\x1F\xFD\x81a\x1F\xDAV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a \x14W`\0\x80\xFD[\x815a\x0C\xFD\x81a\x1F\xDAV[`\0\x80`@\x83\x85\x03\x12\x15a 2W`\0\x80\xFD[\x825a =\x81a\x1F\xDAV[\x94` \x93\x90\x93\x015\x93PPPV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a }WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x80\x15\x15\x81\x14a\x1F\xEFW`\0\x80\xFD[\x805a\x1F\xFD\x81a \x83V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a \xB3W`\0\x80\xFD[\x845a \xBE\x81a\x1F\xDAV[\x93P` \x85\x015\x92Pa\x01 `?\x19\x82\x01\x12\x15a \xDAW`\0\x80\xFD[Pa \xE3a KV[a \xEF`@\x86\x01a\x1F\xF2V[\x81Ra \xFD``\x86\x01a\x1F\xF2V[` \x82\x01R`\x80\x85\x81\x015`@\x83\x01R`\xA0\x80\x87\x015``\x84\x01R`\xC0\x80\x88\x015\x92\x84\x01\x92\x90\x92R`\xE0\x87\x015\x90\x83\x01Ra\x01\0\x86\x015\x90\x82\x01Ra!Ea\x01 \x86\x01a \x91V[`\xE0\x82\x01Ra!Wa\x01@\x86\x01a \x91V[a\x01\0\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0a\x01 \x82\x01\x90P`\x01\x80`\xA0\x1B\x03\x83Q\x16\x82R`\x01\x80`\xA0\x1B\x03` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Qa!\xD4`\xE0\x84\x01\x82\x15\x15\x90RV[Pa\x01\0\x83\x01Qa!\xEAa\x01\0\x84\x01\x82\x15\x15\x90RV[P\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x06W`\0\x80\xFD[\x835a\"\x11\x81a\x1F\xDAV[\x92P` \x84\x015\x91P`@\x84\x015a\"(\x81a\x1F\xDAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\"HW`\0\x80\xFD[\x835a\"S\x81a\x1F\xDAV[\x92P` \x84\x015a\"c\x81a\x1F\xDAV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\"\x87W`\0\x80\xFD[\x825a\"\x92\x81a\x1F\xDAV[\x91P` \x83\x015a\"\xA2\x81a\x1F\xDAV[\x80\x91PP\x92P\x92\x90PV[` \x80\x82R`\x11\x90\x82\x01RpENTRY_SHORT_PRICE`x\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0F\x90\x82\x01Rn\x10P\xD0\xD7\xD3\x13\xD3\x91\xD7\xD0SS\xD5S\x95`\x8A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FDEBT_MULTIPLIER_FACTOR_FOR_REDEE`@\x82\x01R`M`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a#TW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`\x0E\x90\x82\x01Rm\x12\x10T\xD7\xD0\xD3\xD3\x13\x10U\x11T\x90S`\x92\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FHEALTH_FACTOR_LIQUIDATION_THRESH`@\x82\x01Rb\x13\xD3\x11`\xEA\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x08\x90\x82\x01Rg\x12\x10T\xD7\xD1\x11P\x95`\xC2\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\r\x90\x82\x01RlPOSITION_TYPE`\x98\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x07\x90\x82\x01Rf\x10P\xD0\xD3\xD5S\x95`\xCA\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a$iW`\0\x80\xFD[\x81Qa\x0C\xFD\x81a\x1F\xDAV[` \x80\x82R`\x10\x90\x82\x01Ro\x15S\x91\x11T\x93\x16RS\x91\xD7\xD0T\xD4\xD1U`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01RoENTRY_LONG_PRICE`\x80\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01Ro\x10P\xD0\xD7\xD4\xD2\x13\xD4\x95\x17\xD0SS\xD5S\x95`\x82\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a%\x04W`\0\x80\xFD[\x81Qa\x0C\xFD\x81a \x83V\xFE\xA2dipfsX\"\x12 3\xD4)ay\xD2\x98\xC8\x92\x14\xBC\xD91\xF6ua\x1D:\xEA\x94\x8D4+\xD9\x17\x0Fv\xF3ZK\x08\xC1dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static POSITIONSTOREUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01+W`\x005`\xE0\x1C\x80c{\x82\xD7N\x11a\0\xB7W\x80c\xCE\xB8\x92\x10\x11a\0{W\x80c\xCE\xB8\x92\x10\x14a\x02+W\x80c\xE6\x02S\x93\x14a\x02KW\x80c\xE6\xC8\xE3\x06\x14a\x02SW\x80c\xE9\x05$\xF2\x14a\x02sW\x80c\xEF\\\xB5=\x14a\x02{W`\0\x80\xFD[\x80c{\x82\xD7N\x14a\x01\xC8W\x80c\xA6\x1A\x94A\x14a\x01\xE8W\x80c\xB2\xAA\x83\xEC\x14a\x01\xF0W\x80c\xB3\x9DS^\x14a\x02\x10W\x80c\xB3\xB8\xC8\xE5\x14a\x02\x18W`\0\x80\xFD[\x80cC\xD6Rs\x11a\0\xFEW\x80cC\xD6Rs\x14a\x01nW\x80cQ\x14\xEE>\x14a\x01vW\x80cS\xF2\x03l\x14a\x01\x98W\x80cX\x1A*\xE8\x14a\x01\xA0W\x80cp\xB5'\x0E\x14a\x01\xA8W`\0\x80\xFD[\x80c\x011&\xB1\x14a\x010W\x80c\x13|_\xFF\x14a\x01KW\x80c/\xA3p\"\x14a\x01SW\x80c4\x9C\x1Ar\x14a\x01[W[`\0\x80\xFD[a\x018a\x02\x8EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x02\xB6V[a\x018a\x02\xC5V[a\x018a\x01i6`\x04a \x02V[a\x02\xEFV[a\x018a\x03\x8AV[\x81\x80\x15a\x01\x82W`\0\x80\xFD[Pa\x01\x96a\x01\x916`\x04a \x1FV[a\x03\x99V[\0[a\x018a\x048V[a\x018a\x04GV[\x81\x80\x15a\x01\xB4W`\0\x80\xFD[Pa\x01\x96a\x01\xC36`\x04a \x9CV[a\x04VV[a\x01\xDBa\x01\xD66`\x04a \x1FV[a\x0C\xEBV[`@Qa\x01B\x91\x90a!hV[a\x018a\r\x04V[\x81\x80\x15a\x01\xFCW`\0\x80\xFD[Pa\x01\x96a\x02\x0B6`\x04a!\xF1V[a\r\x13V[a\x018a\x151V[a\x018a\x02&6`\x04a \x02V[a\x15@V[\x81\x80\x15a\x027W`\0\x80\xFD[Pa\x01\x96a\x02F6`\x04a\"3V[a\x15`V[a\x018a\x15\xE8V[\x81\x80\x15a\x02_W`\0\x80\xFD[Pa\x01\x96a\x02n6`\x04a \x1FV[a\x15\xF7V[a\x018a\x16\x15V[a\x018a\x02\x896`\x04a\"tV[a\x16$V[`@Q` \x01a\x02\x9D\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x02\x9D\x90a\"\xD8V[`@Q` \x01a\x02\x9D\x90` \x80\x82R`\x06\x90\x82\x01Re\x12T\xD7\xD5T\xD1`\xD2\x1B`@\x82\x01R``\x01\x90V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\x0F\x90a#\x01V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x84\x91\x90a#BV[\x92\x91PPV[`@Q` \x01a\x02\x9D\x90a#[V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03\xB7\x90a#\x83V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x043\x91\x90a#BV[PPPV[`@Q` \x01a\x02\x9D\x90a#\xC6V[`@Q` \x01a\x02\x9D\x90a#\xE8V[`\0\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x04y\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE0W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x05\0\x84`\0\x01Qa\x16\x9DV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x05x\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x05\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x05\xF2\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x065\x91\x90a$WV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x06U\x90a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x06\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xCF\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x12\x91\x90a$WV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x072\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x07b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x07\xA4\x92`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE7\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x08\x07\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x087\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBB\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x08\xDB\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\tL\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\tkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x8F\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\t\xAF\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xA0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\n\x83\x90a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xC0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B7\x91\x90a#BV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xAB\xFD\xCC\xED\x84`@Q` \x01a\x0BW\x90a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xCA\x92\x91\x90\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\r\x91\x90a$\xF2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xAB\xFD\xCC\xED\x84`@Q` \x01a\x0C-\x90a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C]\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x01\0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA1\x92\x91\x90\x91\x82R\x15\x15` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE4\x91\x90a$\xF2V[PPPPPV[a\x0C\xF3a\x1FxV[a\x0C\xFD\x83\x83a\x17\"V[\x93\x92PPPV[`@Q` \x01a\x02\x9D\x90a$\x9EV[`\0\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\r6\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB0\x91\x90a$\xF2V[a\r\xD4W`@Qc\x04&\xCF\xFF`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16c\x99!\xC3\xCC`@Q` \x01a\r\xF2\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0EEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0EYW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x99!\xC3\xCCa\x0Eu\x84a\x16\x9DV[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xB6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xCAW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xAFo\xB6\x84`@Q` \x01a\x0E\xED\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FQ\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FkW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x7FW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xAFo\xB6\x84`@Q` \x01a\x0F\xA2\x90a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x06\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10 W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x10W\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xBB\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xE9W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x11\x0C\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11p\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x8AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\x9EW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x11\xC1\x90a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xF1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12%\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x12v\x90a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xDA\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xF4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\x08W=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16cB\xC3\xBD\x96\x84`@Q` \x01a\x13+\x90a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x8F\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xBDW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xE7\xAC\x12\x84`@Q` \x01a\x13\xE0\x90a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x10\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14D\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14rW=`\0\x80>=`\0\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xE7\xAC\x12\x84`@Q` \x01a\x14\x95\x90a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xF9\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15'W=`\0\x80>=`\0\xFD[PPPPPPPPV[`@Q` \x01a\x02\x9D\x90a$\xC8V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\x0F\x90a#\x83V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\x15x\x84a\x1F%V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE2\x91\x90a#BV[PPPPV[`@Q` \x01a\x02\x9D\x90a$tV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03\xB7\x90a#\x01V[`@Q` \x01a\x02\x9D\x90a$6V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x16>\x84a\x1F%V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFD\x91\x90a#BV[`\0`@Q` \x01a\x16\xD8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x17*a\x1FxV[\x82a\x173a\x1FxV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x17Q\x90a$\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xCB\x91\x90a$\xF2V[a\x17\xD8W\x91Pa\x03\x84\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xF7\x90a$6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18[\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x9C\x91\x90a$WV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`@Q\x90\x83\x16\x90c!\xF8\xA7!\x90\x86\x90a\x18\xC4\x90` \x01a$tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19(\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19i\x91\x90a$WV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xA5\x90a$\x9EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AJ\x91\x90a#BV[\x81`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1Ar\x90a\"\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x17\x91\x90a#BV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1B>\x90` \x01a\"\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1Bn\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE3\x91\x90a#BV[`\x80\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1C\n\x90` \x01a$\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C:\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Cn\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xAF\x91\x90a#BV[`\xA0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1C\xD6\x90` \x01a#\xE8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D:\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D{\x91\x90a#BV[`\xC0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cz\xE1\xCF\xCA\x90\x86\x90a\x1D\xA2\x90` \x01a#[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EG\x91\x90a$\xF2V[\x15\x15`\xE0\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cz\xE1\xCF\xCA\x90\x86\x90a\x1Ep\x90` \x01a#\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x15\x91\x90a$\xF2V[\x15\x15a\x01\0\x82\x01R\x94\x93PPPPV[`\0`@Q` \x01a\x16\xD8\x90` \x80\x82R`'\x90\x82\x01R\x7FHEALTH_FACTOR_COLLATERAL_RATE_TH`@\x82\x01Rf\x14\x91T\xD2\x13\xD3\x11`\xCA\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80a\x01 \x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x15\x15\x81R` \x01`\0\x15\x15\x81RP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\xEFW`\0\x80\xFD[PV[\x805a\x1F\xFD\x81a\x1F\xDAV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a \x14W`\0\x80\xFD[\x815a\x0C\xFD\x81a\x1F\xDAV[`\0\x80`@\x83\x85\x03\x12\x15a 2W`\0\x80\xFD[\x825a =\x81a\x1F\xDAV[\x94` \x93\x90\x93\x015\x93PPPV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a }WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x80\x15\x15\x81\x14a\x1F\xEFW`\0\x80\xFD[\x805a\x1F\xFD\x81a \x83V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a \xB3W`\0\x80\xFD[\x845a \xBE\x81a\x1F\xDAV[\x93P` \x85\x015\x92Pa\x01 `?\x19\x82\x01\x12\x15a \xDAW`\0\x80\xFD[Pa \xE3a KV[a \xEF`@\x86\x01a\x1F\xF2V[\x81Ra \xFD``\x86\x01a\x1F\xF2V[` \x82\x01R`\x80\x85\x81\x015`@\x83\x01R`\xA0\x80\x87\x015``\x84\x01R`\xC0\x80\x88\x015\x92\x84\x01\x92\x90\x92R`\xE0\x87\x015\x90\x83\x01Ra\x01\0\x86\x015\x90\x82\x01Ra!Ea\x01 \x86\x01a \x91V[`\xE0\x82\x01Ra!Wa\x01@\x86\x01a \x91V[a\x01\0\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0a\x01 \x82\x01\x90P`\x01\x80`\xA0\x1B\x03\x83Q\x16\x82R`\x01\x80`\xA0\x1B\x03` \x84\x01Q\x16` \x83\x01R`@\x83\x01Q`@\x83\x01R``\x83\x01Q``\x83\x01R`\x80\x83\x01Q`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Qa!\xD4`\xE0\x84\x01\x82\x15\x15\x90RV[Pa\x01\0\x83\x01Qa!\xEAa\x01\0\x84\x01\x82\x15\x15\x90RV[P\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\x06W`\0\x80\xFD[\x835a\"\x11\x81a\x1F\xDAV[\x92P` \x84\x015\x91P`@\x84\x015a\"(\x81a\x1F\xDAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\"HW`\0\x80\xFD[\x835a\"S\x81a\x1F\xDAV[\x92P` \x84\x015a\"c\x81a\x1F\xDAV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\"\x87W`\0\x80\xFD[\x825a\"\x92\x81a\x1F\xDAV[\x91P` \x83\x015a\"\xA2\x81a\x1F\xDAV[\x80\x91PP\x92P\x92\x90PV[` \x80\x82R`\x11\x90\x82\x01RpENTRY_SHORT_PRICE`x\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x0F\x90\x82\x01Rn\x10P\xD0\xD7\xD3\x13\xD3\x91\xD7\xD0SS\xD5S\x95`\x8A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FDEBT_MULTIPLIER_FACTOR_FOR_REDEE`@\x82\x01R`M`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a#TW`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`\x0E\x90\x82\x01Rm\x12\x10T\xD7\xD0\xD3\xD3\x13\x10U\x11T\x90S`\x92\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`#\x90\x82\x01R\x7FHEALTH_FACTOR_LIQUIDATION_THRESH`@\x82\x01Rb\x13\xD3\x11`\xEA\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x08\x90\x82\x01Rg\x12\x10T\xD7\xD1\x11P\x95`\xC2\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\r\x90\x82\x01RlPOSITION_TYPE`\x98\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x07\x90\x82\x01Rf\x10P\xD0\xD3\xD5S\x95`\xCA\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a$iW`\0\x80\xFD[\x81Qa\x0C\xFD\x81a\x1F\xDAV[` \x80\x82R`\x10\x90\x82\x01Ro\x15S\x91\x11T\x93\x16RS\x91\xD7\xD0T\xD4\xD1U`\x82\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01RoENTRY_LONG_PRICE`\x80\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x10\x90\x82\x01Ro\x10P\xD0\xD7\xD4\xD2\x13\xD4\x95\x17\xD0SS\xD5S\x95`\x82\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a%\x04W`\0\x80\xFD[\x81Qa\x0C\xFD\x81a \x83V\xFE\xA2dipfsX\"\x12 3\xD4)ay\xD2\x98\xC8\x92\x14\xBC\xD91\xF6ua\x1D:\xEA\x94\x8D4+\xD9\x17\x0Fv\xF3ZK\x08\xC1dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static POSITIONSTOREUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PositionStoreUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PositionStoreUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PositionStoreUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PositionStoreUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PositionStoreUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PositionStoreUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PositionStoreUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POSITIONSTOREUTILS_ABI.clone(),
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
                POSITIONSTOREUTILS_ABI.clone(),
                POSITIONSTOREUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACCOUNT` (0xe90524f2) function
        pub fn account(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([233, 5, 36, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ACC_LONG_AMOUNT` (0x137c5fff) function
        pub fn acc_long_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 124, 95, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ACC_SHORT_AMOUNT` (0xb39d535e) function
        pub fn acc_short_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([179, 157, 83, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ENTRY_LONG_PRICE` (0xa61a9441) function
        pub fn entry_long_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 26, 148, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ENTRY_SHORT_PRICE` (0x013126b1) function
        pub fn entry_short_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([1, 49, 38, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HAS_COLLATERAL` (0x43d65273) function
        pub fn has_collateral(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 214, 82, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HAS_DEBT` (0x53f2036c) function
        pub fn has_debt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([83, 242, 3, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `IS_USD` (0x2fa37022) function
        pub fn is_usd(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 163, 112, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POSITION_TYPE` (0x581a2ae8) function
        pub fn position_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([88, 26, 42, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING_ASSET` (0xe6025393) function
        pub fn underlying_asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 2, 83, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get` (0x7b82d74e) function
        pub fn get(
            &self,
            data_store_address: ::ethers::core::types::Address,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, PositionProps> {
            self.0
                .method_hash([123, 130, 215, 78], (data_store_address, key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDebtMultiplierFactorForRedeem` (0x349c1a72) function
        pub fn get_debt_multiplier_factor_for_redeem(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 156, 26, 114], data_store)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthFactorCollateralRateThreshold` (0xef5cb53d) function
        pub fn get_health_factor_collateral_rate_threshold(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 92, 181, 61], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthFactorLiquidationThreshold` (0xb3b8c8e5) function
        pub fn get_health_factor_liquidation_threshold(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 184, 200, 229], data_store)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PositionStoreUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `PositionNotFound` with signature `PositionNotFound(bytes32)` and selector `0x426cfff0`
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
    #[etherror(name = "PositionNotFound", abi = "PositionNotFound(bytes32)")]
    pub struct PositionNotFound {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `ACCOUNT` function with signature `ACCOUNT()` and selector `0xe90524f2`
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
    #[ethcall(name = "ACCOUNT", abi = "ACCOUNT()")]
    pub struct AccountCall;
    ///Container type for all input parameters for the `ACC_LONG_AMOUNT` function with signature `ACC_LONG_AMOUNT()` and selector `0x137c5fff`
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
    #[ethcall(name = "ACC_LONG_AMOUNT", abi = "ACC_LONG_AMOUNT()")]
    pub struct AccLongAmountCall;
    ///Container type for all input parameters for the `ACC_SHORT_AMOUNT` function with signature `ACC_SHORT_AMOUNT()` and selector `0xb39d535e`
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
    #[ethcall(name = "ACC_SHORT_AMOUNT", abi = "ACC_SHORT_AMOUNT()")]
    pub struct AccShortAmountCall;
    ///Container type for all input parameters for the `ENTRY_LONG_PRICE` function with signature `ENTRY_LONG_PRICE()` and selector `0xa61a9441`
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
    #[ethcall(name = "ENTRY_LONG_PRICE", abi = "ENTRY_LONG_PRICE()")]
    pub struct EntryLongPriceCall;
    ///Container type for all input parameters for the `ENTRY_SHORT_PRICE` function with signature `ENTRY_SHORT_PRICE()` and selector `0x013126b1`
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
    #[ethcall(name = "ENTRY_SHORT_PRICE", abi = "ENTRY_SHORT_PRICE()")]
    pub struct EntryShortPriceCall;
    ///Container type for all input parameters for the `HAS_COLLATERAL` function with signature `HAS_COLLATERAL()` and selector `0x43d65273`
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
    #[ethcall(name = "HAS_COLLATERAL", abi = "HAS_COLLATERAL()")]
    pub struct HasCollateralCall;
    ///Container type for all input parameters for the `HAS_DEBT` function with signature `HAS_DEBT()` and selector `0x53f2036c`
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
    #[ethcall(name = "HAS_DEBT", abi = "HAS_DEBT()")]
    pub struct HasDebtCall;
    ///Container type for all input parameters for the `IS_USD` function with signature `IS_USD()` and selector `0x2fa37022`
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
    #[ethcall(name = "IS_USD", abi = "IS_USD()")]
    pub struct IsUsdCall;
    ///Container type for all input parameters for the `POSITION_TYPE` function with signature `POSITION_TYPE()` and selector `0x581a2ae8`
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
    #[ethcall(name = "POSITION_TYPE", abi = "POSITION_TYPE()")]
    pub struct PositionTypeCall;
    ///Container type for all input parameters for the `UNDERLYING_ASSET` function with signature `UNDERLYING_ASSET()` and selector `0xe6025393`
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
    #[ethcall(name = "UNDERLYING_ASSET", abi = "UNDERLYING_ASSET()")]
    pub struct UnderlyingAssetCall;
    ///Container type for all input parameters for the `get` function with signature `get(address,bytes32)` and selector `0x7b82d74e`
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
    #[ethcall(name = "get", abi = "get(address,bytes32)")]
    pub struct GetCall {
        pub data_store_address: ::ethers::core::types::Address,
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getDebtMultiplierFactorForRedeem` function with signature `getDebtMultiplierFactorForRedeem(address)` and selector `0x349c1a72`
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
        name = "getDebtMultiplierFactorForRedeem",
        abi = "getDebtMultiplierFactorForRedeem(address)"
    )]
    pub struct GetDebtMultiplierFactorForRedeemCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getHealthFactorCollateralRateThreshold` function with signature `getHealthFactorCollateralRateThreshold(address,address)` and selector `0xef5cb53d`
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
        name = "getHealthFactorCollateralRateThreshold",
        abi = "getHealthFactorCollateralRateThreshold(address,address)"
    )]
    pub struct GetHealthFactorCollateralRateThresholdCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getHealthFactorLiquidationThreshold` function with signature `getHealthFactorLiquidationThreshold(address)` and selector `0xb3b8c8e5`
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
        name = "getHealthFactorLiquidationThreshold",
        abi = "getHealthFactorLiquidationThreshold(address)"
    )]
    pub struct GetHealthFactorLiquidationThresholdCall {
        pub data_store: ::ethers::core::types::Address,
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
    pub enum PositionStoreUtilsCalls {
        Account(AccountCall),
        AccLongAmount(AccLongAmountCall),
        AccShortAmount(AccShortAmountCall),
        EntryLongPrice(EntryLongPriceCall),
        EntryShortPrice(EntryShortPriceCall),
        HasCollateral(HasCollateralCall),
        HasDebt(HasDebtCall),
        IsUsd(IsUsdCall),
        PositionType(PositionTypeCall),
        UnderlyingAsset(UnderlyingAssetCall),
        Get(GetCall),
        GetDebtMultiplierFactorForRedeem(GetDebtMultiplierFactorForRedeemCall),
        GetHealthFactorCollateralRateThreshold(
            GetHealthFactorCollateralRateThresholdCall,
        ),
        GetHealthFactorLiquidationThreshold(GetHealthFactorLiquidationThresholdCall),
    }
    impl ::ethers::core::abi::AbiDecode for PositionStoreUtilsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Account(decoded));
            }
            if let Ok(decoded) = <AccLongAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccLongAmount(decoded));
            }
            if let Ok(decoded) = <AccShortAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccShortAmount(decoded));
            }
            if let Ok(decoded) = <EntryLongPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EntryLongPrice(decoded));
            }
            if let Ok(decoded) = <EntryShortPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EntryShortPrice(decoded));
            }
            if let Ok(decoded) = <HasCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasCollateral(decoded));
            }
            if let Ok(decoded) = <HasDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasDebt(decoded));
            }
            if let Ok(decoded) = <IsUsdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsUsd(decoded));
            }
            if let Ok(decoded) = <PositionTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionType(decoded));
            }
            if let Ok(decoded) = <UnderlyingAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyingAsset(decoded));
            }
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <GetDebtMultiplierFactorForRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDebtMultiplierFactorForRedeem(decoded));
            }
            if let Ok(decoded) = <GetHealthFactorCollateralRateThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealthFactorCollateralRateThreshold(decoded));
            }
            if let Ok(decoded) = <GetHealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetHealthFactorLiquidationThreshold(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PositionStoreUtilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Account(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccLongAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccShortAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EntryLongPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EntryShortPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsUsd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PositionType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDebtMultiplierFactorForRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthFactorCollateralRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PositionStoreUtilsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Account(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccLongAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccShortAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryLongPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::EntryShortPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsUsd(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionType(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDebtMultiplierFactorForRedeem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHealthFactorCollateralRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccountCall> for PositionStoreUtilsCalls {
        fn from(value: AccountCall) -> Self {
            Self::Account(value)
        }
    }
    impl ::core::convert::From<AccLongAmountCall> for PositionStoreUtilsCalls {
        fn from(value: AccLongAmountCall) -> Self {
            Self::AccLongAmount(value)
        }
    }
    impl ::core::convert::From<AccShortAmountCall> for PositionStoreUtilsCalls {
        fn from(value: AccShortAmountCall) -> Self {
            Self::AccShortAmount(value)
        }
    }
    impl ::core::convert::From<EntryLongPriceCall> for PositionStoreUtilsCalls {
        fn from(value: EntryLongPriceCall) -> Self {
            Self::EntryLongPrice(value)
        }
    }
    impl ::core::convert::From<EntryShortPriceCall> for PositionStoreUtilsCalls {
        fn from(value: EntryShortPriceCall) -> Self {
            Self::EntryShortPrice(value)
        }
    }
    impl ::core::convert::From<HasCollateralCall> for PositionStoreUtilsCalls {
        fn from(value: HasCollateralCall) -> Self {
            Self::HasCollateral(value)
        }
    }
    impl ::core::convert::From<HasDebtCall> for PositionStoreUtilsCalls {
        fn from(value: HasDebtCall) -> Self {
            Self::HasDebt(value)
        }
    }
    impl ::core::convert::From<IsUsdCall> for PositionStoreUtilsCalls {
        fn from(value: IsUsdCall) -> Self {
            Self::IsUsd(value)
        }
    }
    impl ::core::convert::From<PositionTypeCall> for PositionStoreUtilsCalls {
        fn from(value: PositionTypeCall) -> Self {
            Self::PositionType(value)
        }
    }
    impl ::core::convert::From<UnderlyingAssetCall> for PositionStoreUtilsCalls {
        fn from(value: UnderlyingAssetCall) -> Self {
            Self::UnderlyingAsset(value)
        }
    }
    impl ::core::convert::From<GetCall> for PositionStoreUtilsCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<GetDebtMultiplierFactorForRedeemCall>
    for PositionStoreUtilsCalls {
        fn from(value: GetDebtMultiplierFactorForRedeemCall) -> Self {
            Self::GetDebtMultiplierFactorForRedeem(value)
        }
    }
    impl ::core::convert::From<GetHealthFactorCollateralRateThresholdCall>
    for PositionStoreUtilsCalls {
        fn from(value: GetHealthFactorCollateralRateThresholdCall) -> Self {
            Self::GetHealthFactorCollateralRateThreshold(value)
        }
    }
    impl ::core::convert::From<GetHealthFactorLiquidationThresholdCall>
    for PositionStoreUtilsCalls {
        fn from(value: GetHealthFactorLiquidationThresholdCall) -> Self {
            Self::GetHealthFactorLiquidationThreshold(value)
        }
    }
    ///Container type for all return fields from the `ACCOUNT` function with signature `ACCOUNT()` and selector `0xe90524f2`
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
    pub struct AccountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ACC_LONG_AMOUNT` function with signature `ACC_LONG_AMOUNT()` and selector `0x137c5fff`
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
    pub struct AccLongAmountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ACC_SHORT_AMOUNT` function with signature `ACC_SHORT_AMOUNT()` and selector `0xb39d535e`
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
    pub struct AccShortAmountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ENTRY_LONG_PRICE` function with signature `ENTRY_LONG_PRICE()` and selector `0xa61a9441`
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
    pub struct EntryLongPriceReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ENTRY_SHORT_PRICE` function with signature `ENTRY_SHORT_PRICE()` and selector `0x013126b1`
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
    pub struct EntryShortPriceReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HAS_COLLATERAL` function with signature `HAS_COLLATERAL()` and selector `0x43d65273`
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
    pub struct HasCollateralReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HAS_DEBT` function with signature `HAS_DEBT()` and selector `0x53f2036c`
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
    pub struct HasDebtReturn(pub [u8; 32]);
    ///Container type for all return fields from the `IS_USD` function with signature `IS_USD()` and selector `0x2fa37022`
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
    pub struct IsUsdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POSITION_TYPE` function with signature `POSITION_TYPE()` and selector `0x581a2ae8`
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
    pub struct PositionTypeReturn(pub [u8; 32]);
    ///Container type for all return fields from the `UNDERLYING_ASSET` function with signature `UNDERLYING_ASSET()` and selector `0xe6025393`
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
    pub struct UnderlyingAssetReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get` function with signature `get(address,bytes32)` and selector `0x7b82d74e`
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
    pub struct GetReturn(pub PositionProps);
    ///Container type for all return fields from the `getDebtMultiplierFactorForRedeem` function with signature `getDebtMultiplierFactorForRedeem(address)` and selector `0x349c1a72`
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
    pub struct GetDebtMultiplierFactorForRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getHealthFactorCollateralRateThreshold` function with signature `getHealthFactorCollateralRateThreshold(address,address)` and selector `0xef5cb53d`
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
    pub struct GetHealthFactorCollateralRateThresholdReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getHealthFactorLiquidationThreshold` function with signature `getHealthFactorLiquidationThreshold(address)` and selector `0xb3b8c8e5`
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
    pub struct GetHealthFactorLiquidationThresholdReturn(
        pub ::ethers::core::types::U256,
    );
}
