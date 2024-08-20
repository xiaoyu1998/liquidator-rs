pub use swap_event_utils::*;
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
pub mod swap_event_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SWAPEVENTUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xEFa\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c\xE2B\xAE\xFC\x14a\0:W[`\0\x80\xFD[\x81\x80\x15a\0FW`\0\x80\xFD[Pa\0Za\0U6`\x04a\x01$V[a\0\\V[\0[`@Qc3rh\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R\x8A\x81\x16`$\x83\x01R\x89\x81\x16`D\x83\x01R`d\x82\x01\x89\x90R`\x84\x82\x01\x88\x90R`\xA4\x82\x01\x87\x90R`\xC4\x82\x01\x86\x90R`\xE4\x82\x01\x85\x90Ra\x01\x04\x82\x01\x84\x90Ra\x01$\x82\x01\x83\x90R\x8C\x16\x90cf\xE4\xD1z\x90a\x01D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xF7W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x1FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a\x01FW`\0\x80\xFD[a\x01O\x8Ca\x01\x08V[\x9APa\x01]` \x8D\x01a\x01\x08V[\x99Pa\x01k`@\x8D\x01a\x01\x08V[\x98Pa\x01y``\x8D\x01a\x01\x08V[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x81\x015\x99`\xA0\x82\x015\x99P`\xC0\x82\x015\x98P`\xE0\x82\x015\x97Pa\x01\0\x82\x015\x96Pa\x01 \x82\x015\x95Pa\x01@\x90\x91\x015\x93P\x91PPV\xFE\xA2dipfsX\"\x12 *I\xE2\xAD\x1C\xD4\x85\x8D\x87\xE3a\xCC\xCAGI\x86\x84\xF0!\xA0TnE{\xD9\xEA\xEC\xF8\xCB@\x0E\xD2dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static SWAPEVENTUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c\xE2B\xAE\xFC\x14a\0:W[`\0\x80\xFD[\x81\x80\x15a\0FW`\0\x80\xFD[Pa\0Za\0U6`\x04a\x01$V[a\0\\V[\0[`@Qc3rh\xBD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\x04\x83\x01R\x8A\x81\x16`$\x83\x01R\x89\x81\x16`D\x83\x01R`d\x82\x01\x89\x90R`\x84\x82\x01\x88\x90R`\xA4\x82\x01\x87\x90R`\xC4\x82\x01\x86\x90R`\xE4\x82\x01\x85\x90Ra\x01\x04\x82\x01\x84\x90Ra\x01$\x82\x01\x83\x90R\x8C\x16\x90cf\xE4\xD1z\x90a\x01D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xF7W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x1FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01`\x8C\x8E\x03\x12\x15a\x01FW`\0\x80\xFD[a\x01O\x8Ca\x01\x08V[\x9APa\x01]` \x8D\x01a\x01\x08V[\x99Pa\x01k`@\x8D\x01a\x01\x08V[\x98Pa\x01y``\x8D\x01a\x01\x08V[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x81\x015\x99`\xA0\x82\x015\x99P`\xC0\x82\x015\x98P`\xE0\x82\x015\x97Pa\x01\0\x82\x015\x96Pa\x01 \x82\x015\x95Pa\x01@\x90\x91\x015\x93P\x91PPV\xFE\xA2dipfsX\"\x12 *I\xE2\xAD\x1C\xD4\x85\x8D\x87\xE3a\xCC\xCAGI\x86\x84\xF0!\xA0TnE{\xD9\xEA\xEC\xF8\xCB@\x0E\xD2dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPEVENTUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SwapEventUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapEventUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapEventUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapEventUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapEventUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SwapEventUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapEventUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SWAPEVENTUTILS_ABI.clone(),
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
                SWAPEVENTUTILS_ABI.clone(),
                SWAPEVENTUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SwapEventUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
