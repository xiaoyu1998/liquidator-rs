pub use keys::*;
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
pub mod keys {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACCOUNT_POSITION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ACCOUNT_POSITION"),
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
                    ::std::borrow::ToOwned::to_owned("ACCOUNT_POSITION_LIST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ACCOUNT_POSITION_LIST",
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
                    ::std::borrow::ToOwned::to_owned("CLAIMABLE_FEE_AMOUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CLAIMABLE_FEE_AMOUNT",
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
                    ::std::borrow::ToOwned::to_owned(
                        "DEBT_MULTIPLIER_FACTOR_FOR_REDEEM",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEBT_MULTIPLIER_FACTOR_FOR_REDEEM",
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
                    ::std::borrow::ToOwned::to_owned("DEX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEX"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD",
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
                    ::std::borrow::ToOwned::to_owned(
                        "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
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
                    ::std::borrow::ToOwned::to_owned("HOLDING_ADDRESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("HOLDING_ADDRESS"),
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
                    ::std::borrow::ToOwned::to_owned("NATIVE_TOKEN_TRANSFER_GAS_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NATIVE_TOKEN_TRANSFER_GAS_LIMIT",
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
                    ::std::borrow::ToOwned::to_owned("ORACLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ORACLE"),
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
                    ::std::borrow::ToOwned::to_owned("ORACLE_DECIMALS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ORACLE_DECIMALS"),
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
                    ::std::borrow::ToOwned::to_owned("POOL_LIST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL_LIST"),
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
                    ::std::borrow::ToOwned::to_owned("POOL_SALT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL_SALT"),
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
                    ::std::borrow::ToOwned::to_owned("POSITION_LIST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POSITION_LIST"),
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
                    ::std::borrow::ToOwned::to_owned("REENTRANCY_GUARD_STATUS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "REENTRANCY_GUARD_STATUS",
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
                    ::std::borrow::ToOwned::to_owned("TOKEN_TRANSFER_GAS_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN_TRANSFER_GAS_LIMIT",
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
                    ::std::borrow::ToOwned::to_owned("TREASURY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TREASURY"),
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
                    ::std::borrow::ToOwned::to_owned("USER_INITIATED_CANCEL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USER_INITIATED_CANCEL",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("WNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WNT"),
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
    pub static KEYS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x06\x97a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01+W`\x005`\xE0\x1C\x80c\x99\x04\xEC\x9C\x11a\0\xB7W\x80c\xC1\x97\xEB\x9E\x11a\0{W\x80c\xC1\x97\xEB\x9E\x14a\x01\xB3W\x80c\xC3R\\(\x14a\x01\xF4W\x80c\xC5\xA9v'\x14a\x01\xFCW\x80c\xECe\x01\xBD\x14a\x02\x04W\x80c\xF3\xA1\x1D\xA7\x14a\x02\x0CW`\0\x80\xFD[\x80c\x99\x04\xEC\x9C\x14a\x01\x8BW\x80c\x9B\xFE\x0C\xCF\x14a\x01\x93W\x80c\xA1X\x9E\xF8\x14a\x01\x9BW\x80c\xAE\xD8\xB2\x93\x14a\x01\xA3W\x80c\xB8\xB4\xA5C\x14a\x01\xABW`\0\x80\xFD[\x80c8\x01?\x02\x11a\0\xFEW\x80c8\x01?\x02\x14a\x01cW\x80csL\x0B\xEA\x14a\x01kW\x80c{\xF4>G\x14a\x01sW\x80c\x80\x93Z\xA9\x14a\x01{W\x80c\x84\xAF\xBA9\x14a\x01\x83W`\0\x80\xFD[\x80c\x04\x14)\xB8\x14a\x010W\x80c\x1E\xA1U\x02\x14a\x01KW\x80c-,Ue\x14a\x01SW\x80c5\xBFtW\x14a\x01[W[`\0\x80\xFD[a\x018a\x02\x14V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x02fV[a\x018a\x02\x99V[a\x018a\x02\xC5V[a\x018a\x02\xF2V[a\x018a\x03\x1CV[a\x018a\x03mV[a\x018a\x03\x94V[a\x018a\x03\xBBV[a\x018a\x03\xEFV[a\x018a\x040V[a\x018a\x04qV[a\x018a\x04\x9EV[a\x018a\x04\xE9V[a\x01\xE7`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x15T\xD1T\x97\xD2S\x92U\x12PU\x11Q\x17\xD0\xD0S\x90\xD1S`Z\x1B\x81RP\x81V[`@Qa\x01B\x91\x90a\x06\x13V[a\x018a\x05\x1AV[a\x018a\x05gV[a\x018a\x05\x9FV[a\x018a\x05\xD2V[`@Q` \x01a\x02M\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x02M\x90` \x80\x82R`\x0F\x90\x82\x01RnORACLE_DECIMALS`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x06\x90\x82\x01ReORACLE`\xD0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`'\x90\x82\x01R\x7FHEALTH_FACTOR_COLLATERAL_RATE_TH`@\x82\x01Rf\x14\x91T\xD2\x13\xD3\x11`\xCA\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x03\x90\x82\x01Rb\x08\x88\xAB`\xEB\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x10\x90\x82\x01Ro \xA1\xA1\xA7\xAA\xA7*/\xA8'\xA9\xA4\xAA$\xA7\xA7`\x81\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x18\x90\x82\x01R\x7FTOKEN_TRANSFER_GAS_LIMIT\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD4\xD0S\x15`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`!\x90\x82\x01R\x7FDEBT_MULTIPLIER_FACTOR_FOR_REDEE`@\x82\x01R`M`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`#\x90\x82\x01R\x7FHEALTH_FACTOR_LIQUIDATION_THRESH`@\x82\x01Rb\x13\xD3\x11`\xEA\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x14\x90\x82\x01Rs\x10\xD3\x10RSPP\x93\x11W\xD1\x91QW\xD0SS\xD5S\x95`b\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x0F\x90\x82\x01RnHOLDING_ADDRESS`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FNATIVE_TOKEN_TRANSFER_GAS_LIMIT\0`@\x82\x01R``\x01\x90V[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x06AW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x06$V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 x\xB2b\xDA\xCBD\xE2\xF6\x8Ez\xCF\xD6~\xBEnx)\r\x90T{\xE7\xFA\x86\xD2\xACH\xCDq\xA4\xC6EdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static KEYS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x01+W`\x005`\xE0\x1C\x80c\x99\x04\xEC\x9C\x11a\0\xB7W\x80c\xC1\x97\xEB\x9E\x11a\0{W\x80c\xC1\x97\xEB\x9E\x14a\x01\xB3W\x80c\xC3R\\(\x14a\x01\xF4W\x80c\xC5\xA9v'\x14a\x01\xFCW\x80c\xECe\x01\xBD\x14a\x02\x04W\x80c\xF3\xA1\x1D\xA7\x14a\x02\x0CW`\0\x80\xFD[\x80c\x99\x04\xEC\x9C\x14a\x01\x8BW\x80c\x9B\xFE\x0C\xCF\x14a\x01\x93W\x80c\xA1X\x9E\xF8\x14a\x01\x9BW\x80c\xAE\xD8\xB2\x93\x14a\x01\xA3W\x80c\xB8\xB4\xA5C\x14a\x01\xABW`\0\x80\xFD[\x80c8\x01?\x02\x11a\0\xFEW\x80c8\x01?\x02\x14a\x01cW\x80csL\x0B\xEA\x14a\x01kW\x80c{\xF4>G\x14a\x01sW\x80c\x80\x93Z\xA9\x14a\x01{W\x80c\x84\xAF\xBA9\x14a\x01\x83W`\0\x80\xFD[\x80c\x04\x14)\xB8\x14a\x010W\x80c\x1E\xA1U\x02\x14a\x01KW\x80c-,Ue\x14a\x01SW\x80c5\xBFtW\x14a\x01[W[`\0\x80\xFD[a\x018a\x02\x14V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x02fV[a\x018a\x02\x99V[a\x018a\x02\xC5V[a\x018a\x02\xF2V[a\x018a\x03\x1CV[a\x018a\x03mV[a\x018a\x03\x94V[a\x018a\x03\xBBV[a\x018a\x03\xEFV[a\x018a\x040V[a\x018a\x04qV[a\x018a\x04\x9EV[a\x018a\x04\xE9V[a\x01\xE7`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x15T\xD1T\x97\xD2S\x92U\x12PU\x11Q\x17\xD0\xD0S\x90\xD1S`Z\x1B\x81RP\x81V[`@Qa\x01B\x91\x90a\x06\x13V[a\x018a\x05\x1AV[a\x018a\x05gV[a\x018a\x05\x9FV[a\x018a\x05\xD2V[`@Q` \x01a\x02M\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x81V[`@Q` \x01a\x02M\x90` \x80\x82R`\x0F\x90\x82\x01RnORACLE_DECIMALS`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x06\x90\x82\x01ReORACLE`\xD0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`'\x90\x82\x01R\x7FHEALTH_FACTOR_COLLATERAL_RATE_TH`@\x82\x01Rf\x14\x91T\xD2\x13\xD3\x11`\xCA\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x03\x90\x82\x01Rb\x08\x88\xAB`\xEB\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x10\x90\x82\x01Ro \xA1\xA1\xA7\xAA\xA7*/\xA8'\xA9\xA4\xAA$\xA7\xA7`\x81\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x18\x90\x82\x01R\x7FTOKEN_TRANSFER_GAS_LIMIT\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD4\xD0S\x15`\xBA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`!\x90\x82\x01R\x7FDEBT_MULTIPLIER_FACTOR_FOR_REDEE`@\x82\x01R`M`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`#\x90\x82\x01R\x7FHEALTH_FACTOR_LIQUIDATION_THRESH`@\x82\x01Rb\x13\xD3\x11`\xEA\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x14\x90\x82\x01Rs\x10\xD3\x10RSPP\x93\x11W\xD1\x91QW\xD0SS\xD5S\x95`b\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x0F\x90\x82\x01RnHOLDING_ADDRESS`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x01a\x02M\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FNATIVE_TOKEN_TRANSFER_GAS_LIMIT\0`@\x82\x01R``\x01\x90V[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x06AW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x06$V[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 x\xB2b\xDA\xCBD\xE2\xF6\x8Ez\xCF\xD6~\xBEnx)\r\x90T{\xE7\xFA\x86\xD2\xACH\xCDq\xA4\xC6EdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static KEYS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Keys<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Keys<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Keys<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Keys<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Keys<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Keys)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Keys<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KEYS_ABI.clone(),
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
                KEYS_ABI.clone(),
                KEYS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACCOUNT_POSITION` (0x84afba39) function
        pub fn account_position(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([132, 175, 186, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ACCOUNT_POSITION_LIST` (0x041429b8) function
        pub fn account_position_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([4, 20, 41, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CLAIMABLE_FEE_AMOUNT` (0xc5a97627) function
        pub fn claimable_fee_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 169, 118, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEBT_MULTIPLIER_FACTOR_FOR_REDEEM` (0xaed8b293) function
        pub fn debt_multiplier_factor_for_redeem(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([174, 216, 178, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEX` (0x80935aa9) function
        pub fn dex(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([128, 147, 90, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD` (0x734c0bea) function
        pub fn health_factor_collateral_rate_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([115, 76, 11, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` (0xc3525c28) function
        pub fn health_factor_liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([195, 82, 92, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `HOLDING_ADDRESS` (0xec6501bd) function
        pub fn holding_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([236, 101, 1, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NATIVE_TOKEN_TRANSFER_GAS_LIMIT` (0xf3a11da7) function
        pub fn native_token_transfer_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([243, 161, 29, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ORACLE` (0x38013f02) function
        pub fn oracle(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([56, 1, 63, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ORACLE_DECIMALS` (0x1ea15502) function
        pub fn oracle_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([30, 161, 85, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_LIST` (0x35bf7457) function
        pub fn pool_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([53, 191, 116, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_SALT` (0xa1589ef8) function
        pub fn pool_salt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([161, 88, 158, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POSITION_LIST` (0xb8b4a543) function
        pub fn position_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 180, 165, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REENTRANCY_GUARD_STATUS` (0x9bfe0ccf) function
        pub fn reentrancy_guard_status(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 254, 12, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TOKEN_TRANSFER_GAS_LIMIT` (0x9904ec9c) function
        pub fn token_transfer_gas_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([153, 4, 236, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TREASURY` (0x2d2c5565) function
        pub fn treasury(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([45, 44, 85, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `USER_INITIATED_CANCEL` (0xc197eb9e) function
        pub fn user_initiated_cancel(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([193, 151, 235, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WNT` (0x7bf43e47) function
        pub fn wnt(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([123, 244, 62, 71], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Keys<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `ACCOUNT_POSITION` function with signature `ACCOUNT_POSITION()` and selector `0x84afba39`
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
    #[ethcall(name = "ACCOUNT_POSITION", abi = "ACCOUNT_POSITION()")]
    pub struct AccountPositionCall;
    ///Container type for all input parameters for the `ACCOUNT_POSITION_LIST` function with signature `ACCOUNT_POSITION_LIST()` and selector `0x041429b8`
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
    #[ethcall(name = "ACCOUNT_POSITION_LIST", abi = "ACCOUNT_POSITION_LIST()")]
    pub struct AccountPositionListCall;
    ///Container type for all input parameters for the `CLAIMABLE_FEE_AMOUNT` function with signature `CLAIMABLE_FEE_AMOUNT()` and selector `0xc5a97627`
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
    #[ethcall(name = "CLAIMABLE_FEE_AMOUNT", abi = "CLAIMABLE_FEE_AMOUNT()")]
    pub struct ClaimableFeeAmountCall;
    ///Container type for all input parameters for the `DEBT_MULTIPLIER_FACTOR_FOR_REDEEM` function with signature `DEBT_MULTIPLIER_FACTOR_FOR_REDEEM()` and selector `0xaed8b293`
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
        name = "DEBT_MULTIPLIER_FACTOR_FOR_REDEEM",
        abi = "DEBT_MULTIPLIER_FACTOR_FOR_REDEEM()"
    )]
    pub struct DebtMultiplierFactorForRedeemCall;
    ///Container type for all input parameters for the `DEX` function with signature `DEX()` and selector `0x80935aa9`
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
    #[ethcall(name = "DEX", abi = "DEX()")]
    pub struct DexCall;
    ///Container type for all input parameters for the `HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD` function with signature `HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD()` and selector `0x734c0bea`
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
        name = "HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD",
        abi = "HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD()"
    )]
    pub struct HealthFactorCollateralRateThresholdCall;
    ///Container type for all input parameters for the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0xc3525c28`
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
        name = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
        abi = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD()"
    )]
    pub struct HealthFactorLiquidationThresholdCall;
    ///Container type for all input parameters for the `HOLDING_ADDRESS` function with signature `HOLDING_ADDRESS()` and selector `0xec6501bd`
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
    #[ethcall(name = "HOLDING_ADDRESS", abi = "HOLDING_ADDRESS()")]
    pub struct HoldingAddressCall;
    ///Container type for all input parameters for the `NATIVE_TOKEN_TRANSFER_GAS_LIMIT` function with signature `NATIVE_TOKEN_TRANSFER_GAS_LIMIT()` and selector `0xf3a11da7`
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
        name = "NATIVE_TOKEN_TRANSFER_GAS_LIMIT",
        abi = "NATIVE_TOKEN_TRANSFER_GAS_LIMIT()"
    )]
    pub struct NativeTokenTransferGasLimitCall;
    ///Container type for all input parameters for the `ORACLE` function with signature `ORACLE()` and selector `0x38013f02`
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
    #[ethcall(name = "ORACLE", abi = "ORACLE()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `ORACLE_DECIMALS` function with signature `ORACLE_DECIMALS()` and selector `0x1ea15502`
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
    #[ethcall(name = "ORACLE_DECIMALS", abi = "ORACLE_DECIMALS()")]
    pub struct OracleDecimalsCall;
    ///Container type for all input parameters for the `POOL_LIST` function with signature `POOL_LIST()` and selector `0x35bf7457`
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
    #[ethcall(name = "POOL_LIST", abi = "POOL_LIST()")]
    pub struct PoolListCall;
    ///Container type for all input parameters for the `POOL_SALT` function with signature `POOL_SALT()` and selector `0xa1589ef8`
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
    #[ethcall(name = "POOL_SALT", abi = "POOL_SALT()")]
    pub struct PoolSaltCall;
    ///Container type for all input parameters for the `POSITION_LIST` function with signature `POSITION_LIST()` and selector `0xb8b4a543`
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
    #[ethcall(name = "POSITION_LIST", abi = "POSITION_LIST()")]
    pub struct PositionListCall;
    ///Container type for all input parameters for the `REENTRANCY_GUARD_STATUS` function with signature `REENTRANCY_GUARD_STATUS()` and selector `0x9bfe0ccf`
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
    #[ethcall(name = "REENTRANCY_GUARD_STATUS", abi = "REENTRANCY_GUARD_STATUS()")]
    pub struct ReentrancyGuardStatusCall;
    ///Container type for all input parameters for the `TOKEN_TRANSFER_GAS_LIMIT` function with signature `TOKEN_TRANSFER_GAS_LIMIT()` and selector `0x9904ec9c`
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
    #[ethcall(name = "TOKEN_TRANSFER_GAS_LIMIT", abi = "TOKEN_TRANSFER_GAS_LIMIT()")]
    pub struct TokenTransferGasLimitCall;
    ///Container type for all input parameters for the `TREASURY` function with signature `TREASURY()` and selector `0x2d2c5565`
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
    #[ethcall(name = "TREASURY", abi = "TREASURY()")]
    pub struct TreasuryCall;
    ///Container type for all input parameters for the `USER_INITIATED_CANCEL` function with signature `USER_INITIATED_CANCEL()` and selector `0xc197eb9e`
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
    #[ethcall(name = "USER_INITIATED_CANCEL", abi = "USER_INITIATED_CANCEL()")]
    pub struct UserInitiatedCancelCall;
    ///Container type for all input parameters for the `WNT` function with signature `WNT()` and selector `0x7bf43e47`
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
    #[ethcall(name = "WNT", abi = "WNT()")]
    pub struct WntCall;
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
    pub enum KeysCalls {
        AccountPosition(AccountPositionCall),
        AccountPositionList(AccountPositionListCall),
        ClaimableFeeAmount(ClaimableFeeAmountCall),
        DebtMultiplierFactorForRedeem(DebtMultiplierFactorForRedeemCall),
        Dex(DexCall),
        HealthFactorCollateralRateThreshold(HealthFactorCollateralRateThresholdCall),
        HealthFactorLiquidationThreshold(HealthFactorLiquidationThresholdCall),
        HoldingAddress(HoldingAddressCall),
        NativeTokenTransferGasLimit(NativeTokenTransferGasLimitCall),
        Oracle(OracleCall),
        OracleDecimals(OracleDecimalsCall),
        PoolList(PoolListCall),
        PoolSalt(PoolSaltCall),
        PositionList(PositionListCall),
        ReentrancyGuardStatus(ReentrancyGuardStatusCall),
        TokenTransferGasLimit(TokenTransferGasLimitCall),
        Treasury(TreasuryCall),
        UserInitiatedCancel(UserInitiatedCancelCall),
        Wnt(WntCall),
    }
    impl ::ethers::core::abi::AbiDecode for KeysCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountPosition(decoded));
            }
            if let Ok(decoded) = <AccountPositionListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountPositionList(decoded));
            }
            if let Ok(decoded) = <ClaimableFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimableFeeAmount(decoded));
            }
            if let Ok(decoded) = <DebtMultiplierFactorForRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtMultiplierFactorForRedeem(decoded));
            }
            if let Ok(decoded) = <DexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Dex(decoded));
            }
            if let Ok(decoded) = <HealthFactorCollateralRateThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorCollateralRateThreshold(decoded));
            }
            if let Ok(decoded) = <HealthFactorLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HealthFactorLiquidationThreshold(decoded));
            }
            if let Ok(decoded) = <HoldingAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HoldingAddress(decoded));
            }
            if let Ok(decoded) = <NativeTokenTransferGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NativeTokenTransferGasLimit(decoded));
            }
            if let Ok(decoded) = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded) = <OracleDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OracleDecimals(decoded));
            }
            if let Ok(decoded) = <PoolListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolList(decoded));
            }
            if let Ok(decoded) = <PoolSaltCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PoolSalt(decoded));
            }
            if let Ok(decoded) = <PositionListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionList(decoded));
            }
            if let Ok(decoded) = <ReentrancyGuardStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReentrancyGuardStatus(decoded));
            }
            if let Ok(decoded) = <TokenTransferGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransferGasLimit(decoded));
            }
            if let Ok(decoded) = <TreasuryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Treasury(decoded));
            }
            if let Ok(decoded) = <UserInitiatedCancelCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserInitiatedCancel(decoded));
            }
            if let Ok(decoded) = <WntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Wnt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KeysCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AccountPositionList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimableFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtMultiplierFactorForRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HealthFactorCollateralRateThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HealthFactorLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HoldingAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTokenTransferGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolSalt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PositionList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyGuardStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Treasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserInitiatedCancel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wnt(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for KeysCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccountPositionList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimableFeeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtMultiplierFactorForRedeem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Dex(element) => ::core::fmt::Display::fmt(element, f),
                Self::HealthFactorCollateralRateThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HealthFactorLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HoldingAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTokenTransferGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolList(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolSalt(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionList(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyGuardStatus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenTransferGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Treasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserInitiatedCancel(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Wnt(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountPositionCall> for KeysCalls {
        fn from(value: AccountPositionCall) -> Self {
            Self::AccountPosition(value)
        }
    }
    impl ::core::convert::From<AccountPositionListCall> for KeysCalls {
        fn from(value: AccountPositionListCall) -> Self {
            Self::AccountPositionList(value)
        }
    }
    impl ::core::convert::From<ClaimableFeeAmountCall> for KeysCalls {
        fn from(value: ClaimableFeeAmountCall) -> Self {
            Self::ClaimableFeeAmount(value)
        }
    }
    impl ::core::convert::From<DebtMultiplierFactorForRedeemCall> for KeysCalls {
        fn from(value: DebtMultiplierFactorForRedeemCall) -> Self {
            Self::DebtMultiplierFactorForRedeem(value)
        }
    }
    impl ::core::convert::From<DexCall> for KeysCalls {
        fn from(value: DexCall) -> Self {
            Self::Dex(value)
        }
    }
    impl ::core::convert::From<HealthFactorCollateralRateThresholdCall> for KeysCalls {
        fn from(value: HealthFactorCollateralRateThresholdCall) -> Self {
            Self::HealthFactorCollateralRateThreshold(value)
        }
    }
    impl ::core::convert::From<HealthFactorLiquidationThresholdCall> for KeysCalls {
        fn from(value: HealthFactorLiquidationThresholdCall) -> Self {
            Self::HealthFactorLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<HoldingAddressCall> for KeysCalls {
        fn from(value: HoldingAddressCall) -> Self {
            Self::HoldingAddress(value)
        }
    }
    impl ::core::convert::From<NativeTokenTransferGasLimitCall> for KeysCalls {
        fn from(value: NativeTokenTransferGasLimitCall) -> Self {
            Self::NativeTokenTransferGasLimit(value)
        }
    }
    impl ::core::convert::From<OracleCall> for KeysCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<OracleDecimalsCall> for KeysCalls {
        fn from(value: OracleDecimalsCall) -> Self {
            Self::OracleDecimals(value)
        }
    }
    impl ::core::convert::From<PoolListCall> for KeysCalls {
        fn from(value: PoolListCall) -> Self {
            Self::PoolList(value)
        }
    }
    impl ::core::convert::From<PoolSaltCall> for KeysCalls {
        fn from(value: PoolSaltCall) -> Self {
            Self::PoolSalt(value)
        }
    }
    impl ::core::convert::From<PositionListCall> for KeysCalls {
        fn from(value: PositionListCall) -> Self {
            Self::PositionList(value)
        }
    }
    impl ::core::convert::From<ReentrancyGuardStatusCall> for KeysCalls {
        fn from(value: ReentrancyGuardStatusCall) -> Self {
            Self::ReentrancyGuardStatus(value)
        }
    }
    impl ::core::convert::From<TokenTransferGasLimitCall> for KeysCalls {
        fn from(value: TokenTransferGasLimitCall) -> Self {
            Self::TokenTransferGasLimit(value)
        }
    }
    impl ::core::convert::From<TreasuryCall> for KeysCalls {
        fn from(value: TreasuryCall) -> Self {
            Self::Treasury(value)
        }
    }
    impl ::core::convert::From<UserInitiatedCancelCall> for KeysCalls {
        fn from(value: UserInitiatedCancelCall) -> Self {
            Self::UserInitiatedCancel(value)
        }
    }
    impl ::core::convert::From<WntCall> for KeysCalls {
        fn from(value: WntCall) -> Self {
            Self::Wnt(value)
        }
    }
    ///Container type for all return fields from the `ACCOUNT_POSITION` function with signature `ACCOUNT_POSITION()` and selector `0x84afba39`
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
    pub struct AccountPositionReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ACCOUNT_POSITION_LIST` function with signature `ACCOUNT_POSITION_LIST()` and selector `0x041429b8`
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
    pub struct AccountPositionListReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CLAIMABLE_FEE_AMOUNT` function with signature `CLAIMABLE_FEE_AMOUNT()` and selector `0xc5a97627`
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
    pub struct ClaimableFeeAmountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEBT_MULTIPLIER_FACTOR_FOR_REDEEM` function with signature `DEBT_MULTIPLIER_FACTOR_FOR_REDEEM()` and selector `0xaed8b293`
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
    pub struct DebtMultiplierFactorForRedeemReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEX` function with signature `DEX()` and selector `0x80935aa9`
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
    pub struct DexReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD` function with signature `HEALTH_FACTOR_COLLATERAL_RATE_THRESHOLD()` and selector `0x734c0bea`
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
    pub struct HealthFactorCollateralRateThresholdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `0xc3525c28`
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
    pub struct HealthFactorLiquidationThresholdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `HOLDING_ADDRESS` function with signature `HOLDING_ADDRESS()` and selector `0xec6501bd`
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
    pub struct HoldingAddressReturn(pub [u8; 32]);
    ///Container type for all return fields from the `NATIVE_TOKEN_TRANSFER_GAS_LIMIT` function with signature `NATIVE_TOKEN_TRANSFER_GAS_LIMIT()` and selector `0xf3a11da7`
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
    pub struct NativeTokenTransferGasLimitReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ORACLE` function with signature `ORACLE()` and selector `0x38013f02`
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
    pub struct OracleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `ORACLE_DECIMALS` function with signature `ORACLE_DECIMALS()` and selector `0x1ea15502`
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
    pub struct OracleDecimalsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POOL_LIST` function with signature `POOL_LIST()` and selector `0x35bf7457`
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
    pub struct PoolListReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POOL_SALT` function with signature `POOL_SALT()` and selector `0xa1589ef8`
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
    pub struct PoolSaltReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POSITION_LIST` function with signature `POSITION_LIST()` and selector `0xb8b4a543`
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
    pub struct PositionListReturn(pub [u8; 32]);
    ///Container type for all return fields from the `REENTRANCY_GUARD_STATUS` function with signature `REENTRANCY_GUARD_STATUS()` and selector `0x9bfe0ccf`
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
    pub struct ReentrancyGuardStatusReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TOKEN_TRANSFER_GAS_LIMIT` function with signature `TOKEN_TRANSFER_GAS_LIMIT()` and selector `0x9904ec9c`
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
    pub struct TokenTransferGasLimitReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TREASURY` function with signature `TREASURY()` and selector `0x2d2c5565`
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
    pub struct TreasuryReturn(pub [u8; 32]);
    ///Container type for all return fields from the `USER_INITIATED_CANCEL` function with signature `USER_INITIATED_CANCEL()` and selector `0xc197eb9e`
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
    pub struct UserInitiatedCancelReturn(pub ::std::string::String);
    ///Container type for all return fields from the `WNT` function with signature `WNT()` and selector `0x7bf43e47`
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
    pub struct WntReturn(pub [u8; 32]);
}
