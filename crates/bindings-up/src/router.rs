pub use router::*;
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
pub mod router {
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
                    ::std::borrow::ToOwned::to_owned("pluginTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pluginTransfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
    pub static ROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x06\x818\x03\x80a\x06\x81\x839\x81\x01`@\x81\x90R`,\x91`<V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`jV[`\0` \x82\x84\x03\x12\x15`MW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`cW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x05\xF6a\0\x8B`\09`\0\x81\x81`U\x01Ra\x01?\x01Ra\x05\xF6`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x1B\x82xx\x14a\0;W\x80cJJ{\x04\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\xA9V[a\0\x93V[\0[a\0w\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x08`@Q` \x01a\0\xC7\x90` \x80\x82R`\r\x90\x82\x01Rl)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\r\x81R` \x01l)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B\x81RPa\x01#V[a\x01\x1D`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84\x84a\x01\xE0V[PPPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB2\x91\x90a\x04\xF4V[a\x01\xDCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD3\x92\x91\x90a\x05mV[`@Q\x80\x91\x03\x90\xFD[PPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90R\x83Q\x80\x85\x01\x90\x94R\x80\x84R\x7FSafeERC20: low-level call failed\x90\x84\x01Ra\x01\x1D\x92\x87\x92\x91`\0\x91a\x02x\x91\x85\x16\x90\x84\x90a\x02\xFDV[\x90P\x80Q`\0\x14\x80a\x02\x99WP\x80\x80` \x01\x90Q\x81\x01\x90a\x02\x99\x91\x90a\x04\xF4V[a\x02\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\xD3V[PPPV[``a\x03\x0C\x84\x84`\0\x85a\x03\x14V[\x94\x93PPPPV[``\x82G\x10\x15a\x03uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\xD3V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x03\x91\x91\x90a\x05\x91V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xD3V[``\x91P[P\x91P\x91Pa\x03\xE4\x87\x83\x83\x87a\x03\xEFV[\x97\x96PPPPPPPV[``\x83\x15a\x04^W\x82Q`\0\x03a\x04WW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[P\x81a\x03\x0CV[a\x03\x0C\x83\x83\x81Q\x15a\x04sW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD3\x91\x90a\x05\xADV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xA4W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x04\xBFW`\0\x80\xFD[a\x04\xC8\x85a\x04\x8DV[\x93Pa\x04\xD6` \x86\x01a\x04\x8DV[\x92Pa\x04\xE4`@\x86\x01a\x04\x8DV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a\x05\x06W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\x16W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x058W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05 V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05Y\x81` \x86\x01` \x86\x01a\x05\x1DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x03\x0C\x90\x83\x01\x84a\x05AV[`\0\x82Qa\x05\xA3\x81\x84` \x87\x01a\x05\x1DV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x05\x16` \x83\x01\x84a\x05AV\xFE\xA2dipfsX\"\x12 ^6\xFAZ\xE9\x98p\x82\xE8\xD5\x7F\x84N\x8C\xD6\xCCF\x8A\xCA?\x838\x94\r\xB4>\x10l\xDB\xA8(\xD5dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static ROUTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x1B\x82xx\x14a\0;W\x80cJJ{\x04\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\xA9V[a\0\x93V[\0[a\0w\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\x08`@Q` \x01a\0\xC7\x90` \x80\x82R`\r\x90\x82\x01Rl)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\r\x81R` \x01l)'\xAA\xAA\"\xA9/\xA8&*\xA3\xA4\xA7`\x99\x1B\x81RPa\x01#V[a\x01\x1D`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84\x84a\x01\xE0V[PPPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xB2\x91\x90a\x04\xF4V[a\x01\xDCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\xD3\x92\x91\x90a\x05mV[`@Q\x80\x91\x03\x90\xFD[PPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90R\x83Q\x80\x85\x01\x90\x94R\x80\x84R\x7FSafeERC20: low-level call failed\x90\x84\x01Ra\x01\x1D\x92\x87\x92\x91`\0\x91a\x02x\x91\x85\x16\x90\x84\x90a\x02\xFDV[\x90P\x80Q`\0\x14\x80a\x02\x99WP\x80\x80` \x01\x90Q\x81\x01\x90a\x02\x99\x91\x90a\x04\xF4V[a\x02\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\xD3V[PPPV[``a\x03\x0C\x84\x84`\0\x85a\x03\x14V[\x94\x93PPPPV[``\x82G\x10\x15a\x03uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\xD3V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x03\x91\x91\x90a\x05\x91V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xD3V[``\x91P[P\x91P\x91Pa\x03\xE4\x87\x83\x83\x87a\x03\xEFV[\x97\x96PPPPPPPV[``\x83\x15a\x04^W\x82Q`\0\x03a\x04WW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\xD3V[P\x81a\x03\x0CV[a\x03\x0C\x83\x83\x81Q\x15a\x04sW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD3\x91\x90a\x05\xADV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xA4W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x04\xBFW`\0\x80\xFD[a\x04\xC8\x85a\x04\x8DV[\x93Pa\x04\xD6` \x86\x01a\x04\x8DV[\x92Pa\x04\xE4`@\x86\x01a\x04\x8DV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a\x05\x06W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\x16W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x058W\x81\x81\x01Q\x83\x82\x01R` \x01a\x05 V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05Y\x81` \x86\x01` \x86\x01a\x05\x1DV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x03\x0C\x90\x83\x01\x84a\x05AV[`\0\x82Qa\x05\xA3\x81\x84` \x87\x01a\x05\x1DV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x05\x16` \x83\x01\x84a\x05AV\xFE\xA2dipfsX\"\x12 ^6\xFAZ\xE9\x98p\x82\xE8\xD5\x7F\x84N\x8C\xD6\xCCF\x8A\xCA?\x838\x94\r\xB4>\x10l\xDB\xA8(\xD5dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static ROUTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Router<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Router<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Router<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Router<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Router<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Router)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Router<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROUTER_ABI.clone(),
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
                ROUTER_ABI.clone(),
                ROUTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `pluginTransfer` (0x1b827878) function
        pub fn plugin_transfer(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 130, 120, 120], (token, account, receiver, amount))
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Router<M> {
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
    ///Container type for all input parameters for the `pluginTransfer` function with signature `pluginTransfer(address,address,address,uint256)` and selector `0x1b827878`
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
        name = "pluginTransfer",
        abi = "pluginTransfer(address,address,address,uint256)"
    )]
    pub struct PluginTransferCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum RouterCalls {
        PluginTransfer(PluginTransferCall),
        RoleStore(RoleStoreCall),
    }
    impl ::ethers::core::abi::AbiDecode for RouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PluginTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PluginTransfer(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PluginTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PluginTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PluginTransferCall> for RouterCalls {
        fn from(value: PluginTransferCall) -> Self {
            Self::PluginTransfer(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for RouterCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
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
}
