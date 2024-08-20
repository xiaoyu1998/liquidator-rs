pub use fee_store_utils::*;
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
pub mod fee_store_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTreasury"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EmptyTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyTreasury"),
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
    pub static FEESTOREUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x03\x01a\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80cx\xF2\x12\xD1\x14a\0EW\x80c\xAD\xFE\x08p\x14a\0tW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x02QV[a\0\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x81\x80\x15a\0\x80W`\0\x80\xFD[Pa\0\x94a\0\x8F6`\x04a\x02uV[a\x01NV[\0[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\0\xD3\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01H\x91\x90a\x02\xAEV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01uW`@Qc3r\xD4-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9`@Q` \x01a\x01\xB0\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x024\x91\x90a\x02\xAEV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02NW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02cW`\0\x80\xFD[\x815a\x02n\x81a\x029V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x88W`\0\x80\xFD[\x825a\x02\x93\x81a\x029V[\x91P` \x83\x015a\x02\xA3\x81a\x029V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xC0W`\0\x80\xFD[\x81Qa\x02n\x81a\x029V\xFE\xA2dipfsX\"\x12 \xC5<\xEC\xF2\xE3q\x1Ch\xE4\x12\xF4\xCCG\xE5+\xECv\x90\xC8\xE97\xACC\xA6\x10\x88h\n\x7F\xA98\xD4dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static FEESTOREUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80cx\xF2\x12\xD1\x14a\0EW\x80c\xAD\xFE\x08p\x14a\0tW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x02QV[a\0\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x81\x80\x15a\0\x80W`\0\x80\xFD[Pa\0\x94a\0\x8F6`\x04a\x02uV[a\x01NV[\0[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\0\xD3\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01H\x91\x90a\x02\xAEV[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01uW`@Qc3r\xD4-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9`@Q` \x01a\x01\xB0\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x024\x91\x90a\x02\xAEV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02NW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x02cW`\0\x80\xFD[\x815a\x02n\x81a\x029V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x88W`\0\x80\xFD[\x825a\x02\x93\x81a\x029V[\x91P` \x83\x015a\x02\xA3\x81a\x029V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xC0W`\0\x80\xFD[\x81Qa\x02n\x81a\x029V\xFE\xA2dipfsX\"\x12 \xC5<\xEC\xF2\xE3q\x1Ch\xE4\x12\xF4\xCCG\xE5+\xECv\x90\xC8\xE97\xACC\xA6\x10\x88h\n\x7F\xA98\xD4dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static FEESTOREUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FeeStoreUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FeeStoreUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FeeStoreUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FeeStoreUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FeeStoreUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FeeStoreUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FeeStoreUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FEESTOREUTILS_ABI.clone(),
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
                FEESTOREUTILS_ABI.clone(),
                FEESTOREUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getTreasury` (0x78f212d1) function
        pub fn get_treasury(
            &self,
            data_store: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([120, 242, 18, 209], data_store)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FeeStoreUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EmptyTreasury` with signature `EmptyTreasury()` and selector `0xcdcb50b4`
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
    #[etherror(name = "EmptyTreasury", abi = "EmptyTreasury()")]
    pub struct EmptyTreasury;
    ///Container type for all input parameters for the `getTreasury` function with signature `getTreasury(address)` and selector `0x78f212d1`
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
    #[ethcall(name = "getTreasury", abi = "getTreasury(address)")]
    pub struct GetTreasuryCall {
        pub data_store: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getTreasury` function with signature `getTreasury(address)` and selector `0x78f212d1`
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
    pub struct GetTreasuryReturn(pub ::ethers::core::types::Address);
}
