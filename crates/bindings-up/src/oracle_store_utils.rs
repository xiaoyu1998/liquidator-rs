pub use oracle_store_utils::*;
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
pub mod oracle_store_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
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
                    ::std::borrow::ToOwned::to_owned("getOracleDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOracleDecimals"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOracleDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOracleDecimals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracleDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("MaxOracleDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("OracleEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OracleEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
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
    pub static ORACLESTOREUTILS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x05\xA9a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0VW`\x005`\xE0\x1C\x80c31\x87\"\x14a\0[W\x80c\x83 \x92Y\x14a\0}W\x80c\xD7\x03/-\x14a\0\x9DW\x80c\xD8\x1E\x84#\x14a\0\xC3W[`\0\x80\xFD[\x81\x80\x15a\0gW`\0\x80\xFD[Pa\0{a\0v6`\x04a\x04xV[a\0\xEEV[\0[\x81\x80\x15a\0\x89W`\0\x80\xFD[Pa\0{a\0\x986`\x04a\x04\xB9V[a\x01\xE5V[a\0\xB0a\0\xAB6`\x04a\x05\x04V[a\x02\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD6a\0\xD16`\x04a\x05\x04V[a\x03<V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01\x15W`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[l\x0C\x9F,\x9C\xD0Ft\xED\xEA@\0\0\0\x81\x11\x15a\x01]W`@Qcq*\x10\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rl\x0C\x9F,\x9C\xD0Ft\xED\xEA@\0\0\0`$\x82\x01R`D\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\x01u\x84a\x03\xB5V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDF\x91\x90a\x05=V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\x0CW`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x023W`@Qc\xD3c}5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9a\x02K\x84a\x044V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDF\x91\x90a\x05VV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x02\xD6\x84a\x03\xB5V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xF4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x05=V[\x93\x92PPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!a\x03V\x84a\x044V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x05VV[`\0`@Q` \x01a\x03\xEA\x90` \x80\x82R`\x0F\x90\x82\x01RnORACLE_DECIMALS`\x88\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0`@Q` \x01a\x03\xEA\x90` \x80\x82R`\x06\x90\x82\x01ReORACLE`\xD0\x1B`@\x82\x01R``\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04uW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\x8DW`\0\x80\xFD[\x835a\x04\x98\x81a\x04`V[\x92P` \x84\x015a\x04\xA8\x81a\x04`V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\xCEW`\0\x80\xFD[\x835a\x04\xD9\x81a\x04`V[\x92P` \x84\x015a\x04\xE9\x81a\x04`V[\x91P`@\x84\x015a\x04\xF9\x81a\x04`V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x05\x17W`\0\x80\xFD[\x825a\x05\"\x81a\x04`V[\x91P` \x83\x015a\x052\x81a\x04`V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x05OW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05hW`\0\x80\xFD[\x81Qa\x035\x81a\x04`V\xFE\xA2dipfsX\"\x12 \x9A\xB6vy\xF1\x9B\x8E\xA1\xEAL\xF5W\x7F)\xE3\xA2\xF9\x857\xFCp\xC0\xB1T\xA5\x84\xD9`3\xF4\xDEXdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ORACLESTOREUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0VW`\x005`\xE0\x1C\x80c31\x87\"\x14a\0[W\x80c\x83 \x92Y\x14a\0}W\x80c\xD7\x03/-\x14a\0\x9DW\x80c\xD8\x1E\x84#\x14a\0\xC3W[`\0\x80\xFD[\x81\x80\x15a\0gW`\0\x80\xFD[Pa\0{a\0v6`\x04a\x04xV[a\0\xEEV[\0[\x81\x80\x15a\0\x89W`\0\x80\xFD[Pa\0{a\0\x986`\x04a\x04\xB9V[a\x01\xE5V[a\0\xB0a\0\xAB6`\x04a\x05\x04V[a\x02\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD6a\0\xD16`\x04a\x05\x04V[a\x03<V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBAV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01\x15W`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[l\x0C\x9F,\x9C\xD0Ft\xED\xEA@\0\0\0\x81\x11\x15a\x01]W`@Qcq*\x10\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rl\x0C\x9F,\x9C\xD0Ft\xED\xEA@\0\0\0`$\x82\x01R`D\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\x01u\x84a\x03\xB5V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDF\x91\x90a\x05=V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\x0CW`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x023W`@Qc\xD3c}5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9a\x02K\x84a\x044V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDF\x91\x90a\x05VV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x02\xD6\x84a\x03\xB5V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xF4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x05=V[\x93\x92PPPV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!a\x03V\x84a\x044V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x035\x91\x90a\x05VV[`\0`@Q` \x01a\x03\xEA\x90` \x80\x82R`\x0F\x90\x82\x01RnORACLE_DECIMALS`\x88\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0`@Q` \x01a\x03\xEA\x90` \x80\x82R`\x06\x90\x82\x01ReORACLE`\xD0\x1B`@\x82\x01R``\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04uW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\x8DW`\0\x80\xFD[\x835a\x04\x98\x81a\x04`V[\x92P` \x84\x015a\x04\xA8\x81a\x04`V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\xCEW`\0\x80\xFD[\x835a\x04\xD9\x81a\x04`V[\x92P` \x84\x015a\x04\xE9\x81a\x04`V[\x91P`@\x84\x015a\x04\xF9\x81a\x04`V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x05\x17W`\0\x80\xFD[\x825a\x05\"\x81a\x04`V[\x91P` \x83\x015a\x052\x81a\x04`V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x05OW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05hW`\0\x80\xFD[\x81Qa\x035\x81a\x04`V\xFE\xA2dipfsX\"\x12 \x9A\xB6vy\xF1\x9B\x8E\xA1\xEAL\xF5W\x7F)\xE3\xA2\xF9\x857\xFCp\xC0\xB1T\xA5\x84\xD9`3\xF4\xDEXdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ORACLESTOREUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OracleStoreUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OracleStoreUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OracleStoreUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OracleStoreUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OracleStoreUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OracleStoreUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OracleStoreUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ORACLESTOREUTILS_ABI.clone(),
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
                ORACLESTOREUTILS_ABI.clone(),
                ORACLESTOREUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `get` (0xd81e8423) function
        pub fn get(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([216, 30, 132, 35], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOracleDecimals` (0xd7032f2d) function
        pub fn get_oracle_decimals(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 3, 47, 45], (data_store, underlying_asset))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OracleStoreUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidOracleDecimals` with signature `InvalidOracleDecimals(uint256,uint256)` and selector `0xe25421ba`
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
        name = "InvalidOracleDecimals",
        abi = "InvalidOracleDecimals(uint256,uint256)"
    )]
    pub struct InvalidOracleDecimals {
        pub oracle_decimals: ::ethers::core::types::U256,
        pub max_oracle_decimals: ::ethers::core::types::U256,
    }
    ///Custom Error type `OracleEmpty` with signature `OracleEmpty()` and selector `0xd3637d35`
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
    #[etherror(name = "OracleEmpty", abi = "OracleEmpty()")]
    pub struct OracleEmpty;
    ///Custom Error type `UnderlyAssetEmpty` with signature `UnderlyAssetEmpty()` and selector `0x08689653`
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
    #[etherror(name = "UnderlyAssetEmpty", abi = "UnderlyAssetEmpty()")]
    pub struct UnderlyAssetEmpty;
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
    pub enum OracleStoreUtilsErrors {
        InvalidOracleDecimals(InvalidOracleDecimals),
        OracleEmpty(OracleEmpty),
        UnderlyAssetEmpty(UnderlyAssetEmpty),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for OracleStoreUtilsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidOracleDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOracleDecimals(decoded));
            }
            if let Ok(decoded) = <OracleEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OracleEmpty(decoded));
            }
            if let Ok(decoded) = <UnderlyAssetEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyAssetEmpty(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OracleStoreUtilsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidOracleDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OracleEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyAssetEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for OracleStoreUtilsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidOracleDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OracleEmpty as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnderlyAssetEmpty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for OracleStoreUtilsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidOracleDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OracleEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyAssetEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for OracleStoreUtilsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidOracleDecimals> for OracleStoreUtilsErrors {
        fn from(value: InvalidOracleDecimals) -> Self {
            Self::InvalidOracleDecimals(value)
        }
    }
    impl ::core::convert::From<OracleEmpty> for OracleStoreUtilsErrors {
        fn from(value: OracleEmpty) -> Self {
            Self::OracleEmpty(value)
        }
    }
    impl ::core::convert::From<UnderlyAssetEmpty> for OracleStoreUtilsErrors {
        fn from(value: UnderlyAssetEmpty) -> Self {
            Self::UnderlyAssetEmpty(value)
        }
    }
    ///Container type for all input parameters for the `get` function with signature `get(address,address)` and selector `0xd81e8423`
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
    #[ethcall(name = "get", abi = "get(address,address)")]
    pub struct GetCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOracleDecimals` function with signature `getOracleDecimals(address,address)` and selector `0xd7032f2d`
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
    #[ethcall(name = "getOracleDecimals", abi = "getOracleDecimals(address,address)")]
    pub struct GetOracleDecimalsCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
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
    pub enum OracleStoreUtilsCalls {
        Get(GetCall),
        GetOracleDecimals(GetOracleDecimalsCall),
    }
    impl ::ethers::core::abi::AbiDecode for OracleStoreUtilsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Get(decoded));
            }
            if let Ok(decoded) = <GetOracleDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOracleDecimals(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OracleStoreUtilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Get(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOracleDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OracleStoreUtilsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Get(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOracleDecimals(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCall> for OracleStoreUtilsCalls {
        fn from(value: GetCall) -> Self {
            Self::Get(value)
        }
    }
    impl ::core::convert::From<GetOracleDecimalsCall> for OracleStoreUtilsCalls {
        fn from(value: GetOracleDecimalsCall) -> Self {
            Self::GetOracleDecimals(value)
        }
    }
    ///Container type for all return fields from the `get` function with signature `get(address,address)` and selector `0xd81e8423`
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
    pub struct GetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getOracleDecimals` function with signature `getOracleDecimals(address,address)` and selector `0xd7032f2d`
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
    pub struct GetOracleDecimalsReturn(pub ::ethers::core::types::U256);
}
