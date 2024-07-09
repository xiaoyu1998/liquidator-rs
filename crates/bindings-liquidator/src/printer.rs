pub use printer::*;
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
pub mod printer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("toString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toString"),
                            inputs: ::std::vec![
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static PRINTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x05\x02a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0VW`\x005`\xE0\x1C\x80cV\xCAb>\x14a\0[W\x80ci\0\xA3\xAE\x14a\0\x84W\x80cq\xAA\xD1\r\x14a\0\x97W\x80c\xB1\x1A\x19\xE8\x14a\0\x84W[`\0\x80\xFD[a\0na\0i6`\x04a\x03\x10V[a\0\xAAV[`@Qa\0{\x91\x90a\x03@V[`@Q\x80\x91\x03\x90\xF3[a\0na\0\x926`\x04a\x03\x8EV[a\0\xE9V[a\0na\0\xA56`\x04a\x03\xBDV[a\x01\x01V[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\0\xE3\x90`4\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x01\x01V[\x92\x91PPV[``a\0\xE3\x82`@Q` \x01a\0\xCF\x91\x81R` \x01\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90`\0\x90a\x01<\x90`\x02a\x04\x8CV[a\x01G\x90`\x02a\x04\xA3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01_Wa\x01_a\x03\xA7V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01\x89W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x01\xA4Wa\x01\xA4a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x01\xD3Wa\x01\xD3a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0[\x84Q\x81\x10\x15a\x03\x08W\x82`\x04\x86\x83\x81Q\x81\x10a\x02\x0BWa\x02\x0Ba\x04\xB6V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x023Wa\x023a\x04\xB6V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x02N\x83`\x02a\x04\x8CV[a\x02Y\x90`\x02a\x04\xA3V[\x81Q\x81\x10a\x02iWa\x02ia\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x02\x93Wa\x02\x93a\x04\xB6V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x02\xB3Wa\x02\xB3a\x04\xB6V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x02\xCE\x83`\x02a\x04\x8CV[a\x02\xD9\x90`\x03a\x04\xA3V[\x81Q\x81\x10a\x02\xE9Wa\x02\xE9a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x01\xEDV[P\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\"W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x039W`\0\x80\xFD[\x93\x92PPPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x03nW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x03QV[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x03\xA0W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x03\xF7W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x11Wa\x04\x11a\x03\xA7V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04@Wa\x04@a\x03\xA7V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04XW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\0\xE3Wa\0\xE3a\x04vV[\x80\x82\x01\x80\x82\x11\x15a\0\xE3Wa\0\xE3a\x04vV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x88\x8E\x17V\xE3W\x8CA\x99\xFC\xD3\x8C\xFA\x1A3\x03\xE2x\xDE\xD6Rsc>\xFB\xF3\xC6\xDF\xF6\xC3\xC1\x98dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static PRINTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0VW`\x005`\xE0\x1C\x80cV\xCAb>\x14a\0[W\x80ci\0\xA3\xAE\x14a\0\x84W\x80cq\xAA\xD1\r\x14a\0\x97W\x80c\xB1\x1A\x19\xE8\x14a\0\x84W[`\0\x80\xFD[a\0na\0i6`\x04a\x03\x10V[a\0\xAAV[`@Qa\0{\x91\x90a\x03@V[`@Q\x80\x91\x03\x90\xF3[a\0na\0\x926`\x04a\x03\x8EV[a\0\xE9V[a\0na\0\xA56`\x04a\x03\xBDV[a\x01\x01V[`@Q``\x82\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x83\x01R\x90a\0\xE3\x90`4\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x01\x01V[\x92\x91PPV[``a\0\xE3\x82`@Q` \x01a\0\xCF\x91\x81R` \x01\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90`\0\x90a\x01<\x90`\x02a\x04\x8CV[a\x01G\x90`\x02a\x04\xA3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01_Wa\x01_a\x03\xA7V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x01\x89W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x01\xA4Wa\x01\xA4a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x01\xD3Wa\x01\xD3a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0[\x84Q\x81\x10\x15a\x03\x08W\x82`\x04\x86\x83\x81Q\x81\x10a\x02\x0BWa\x02\x0Ba\x04\xB6V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x023Wa\x023a\x04\xB6V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x02N\x83`\x02a\x04\x8CV[a\x02Y\x90`\x02a\x04\xA3V[\x81Q\x81\x10a\x02iWa\x02ia\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x02\x93Wa\x02\x93a\x04\xB6V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x02\xB3Wa\x02\xB3a\x04\xB6V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x02\xCE\x83`\x02a\x04\x8CV[a\x02\xD9\x90`\x03a\x04\xA3V[\x81Q\x81\x10a\x02\xE9Wa\x02\xE9a\x04\xB6V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x01\xEDV[P\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\"W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x039W`\0\x80\xFD[\x93\x92PPPV[` \x81R`\0\x82Q\x80` \x84\x01R`\0[\x81\x81\x10\x15a\x03nW` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x03QV[P`\0`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x03\xA0W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x03\xF7W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x11Wa\x04\x11a\x03\xA7V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04@Wa\x04@a\x03\xA7V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04XW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\0\xE3Wa\0\xE3a\x04vV[\x80\x82\x01\x80\x82\x11\x15a\0\xE3Wa\0\xE3a\x04vV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x88\x8E\x17V\xE3W\x8CA\x99\xFC\xD3\x8C\xFA\x1A3\x03\xE2x\xDE\xD6Rsc>\xFB\xF3\xC6\xDF\xF6\xC3\xC1\x98dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static PRINTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Printer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Printer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Printer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Printer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Printer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Printer)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Printer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PRINTER_ABI.clone(),
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
                PRINTER_ABI.clone(),
                PRINTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `toString` (0x56ca623e) function
        pub fn to_string_0(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 202, 98, 62], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x6900a3ae) function
        pub fn to_string_1(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([105, 0, 163, 174], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x71aad10d) function
        pub fn to_string_2(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([113, 170, 209, 13], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xb11a19e8) function
        pub fn to_string_3(
            &self,
            value: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([177, 26, 25, 232], value)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Printer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(address)` and selector `0x56ca623e`
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
    #[ethcall(name = "toString", abi = "toString(address)")]
    pub struct ToString0Call {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    #[ethcall(name = "toString", abi = "toString(uint256)")]
    pub struct ToString1Call {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes)` and selector `0x71aad10d`
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
    #[ethcall(name = "toString", abi = "toString(bytes)")]
    pub struct ToString2Call {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes32)` and selector `0xb11a19e8`
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
    #[ethcall(name = "toString", abi = "toString(bytes32)")]
    pub struct ToString3Call {
        pub value: [u8; 32],
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
    pub enum PrinterCalls {
        ToString0(ToString0Call),
        ToString1(ToString1Call),
        ToString2(ToString2Call),
        ToString3(ToString3Call),
    }
    impl ::ethers::core::abi::AbiDecode for PrinterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ToString0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString0(decoded));
            }
            if let Ok(decoded) = <ToString1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString1(decoded));
            }
            if let Ok(decoded) = <ToString2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString2(decoded));
            }
            if let Ok(decoded) = <ToString3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString3(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PrinterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ToString0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ToString3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PrinterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ToString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString3(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ToString0Call> for PrinterCalls {
        fn from(value: ToString0Call) -> Self {
            Self::ToString0(value)
        }
    }
    impl ::core::convert::From<ToString1Call> for PrinterCalls {
        fn from(value: ToString1Call) -> Self {
            Self::ToString1(value)
        }
    }
    impl ::core::convert::From<ToString2Call> for PrinterCalls {
        fn from(value: ToString2Call) -> Self {
            Self::ToString2(value)
        }
    }
    impl ::core::convert::From<ToString3Call> for PrinterCalls {
        fn from(value: ToString3Call) -> Self {
            Self::ToString3(value)
        }
    }
    ///Container type for all return fields from the `toString` function with signature `toString(address)` and selector `0x56ca623e`
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
    pub struct ToString0Return(pub ::std::string::String);
    ///Container type for all return fields from the `toString` function with signature `toString(uint256)` and selector `0x6900a3ae`
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
    pub struct ToString1Return(pub ::std::string::String);
    ///Container type for all return fields from the `toString` function with signature `toString(bytes)` and selector `0x71aad10d`
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
    pub struct ToString2Return(pub ::std::string::String);
    ///Container type for all return fields from the `toString` function with signature `toString(bytes32)` and selector `0xb11a19e8`
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
    pub struct ToString3Return(pub ::std::string::String);
}
