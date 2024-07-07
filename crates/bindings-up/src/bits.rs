pub use bits::*;
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
pub mod bits {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BITMASK_16"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BITMASK_16"),
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
                    ::std::borrow::ToOwned::to_owned("BITMASK_32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BITMASK_32"),
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
                    ::std::borrow::ToOwned::to_owned("BITMASK_64"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BITMASK_64"),
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
                    ::std::borrow::ToOwned::to_owned("BITMASK_8"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BITMASK_8"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BITS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC5a\08`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`+WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`QW`\x005`\xE0\x1C\x80c\x80|\x97\x82\x14`VW\x80c\x98H\x98\xE5\x14`rW\x80c\xE3&Fm\x14`\x80W\x80c\xF9l^R\x14`\x87W[`\0\x80\xFD[``c\xFF\xFF\xFF\xFF\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[``g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81V[```\xFF\x81V[``a\xFF\xFF\x81V\xFE\xA2dipfsX\"\x12 :\x19\x06\xF6tgc\x01\x01\xC6e\x16\xAAu;\x94\x84\xE6\x19\0\x8E\x85\x12\xE6\x05u6;WU\xC6\x90dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static BITS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`QW`\x005`\xE0\x1C\x80c\x80|\x97\x82\x14`VW\x80c\x98H\x98\xE5\x14`rW\x80c\xE3&Fm\x14`\x80W\x80c\xF9l^R\x14`\x87W[`\0\x80\xFD[``c\xFF\xFF\xFF\xFF\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[``g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81V[```\xFF\x81V[``a\xFF\xFF\x81V\xFE\xA2dipfsX\"\x12 :\x19\x06\xF6tgc\x01\x01\xC6e\x16\xAAu;\x94\x84\xE6\x19\0\x8E\x85\x12\xE6\x05u6;WU\xC6\x90dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static BITS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Bits<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Bits<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Bits<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Bits<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Bits<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Bits)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Bits<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BITS_ABI.clone(),
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
                BITS_ABI.clone(),
                BITS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BITMASK_16` (0xf96c5e52) function
        pub fn bitmask_16(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 108, 94, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BITMASK_32` (0x807c9782) function
        pub fn bitmask_32(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 124, 151, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BITMASK_64` (0x984898e5) function
        pub fn bitmask_64(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 72, 152, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BITMASK_8` (0xe326466d) function
        pub fn bitmask_8(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 38, 70, 109], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Bits<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `BITMASK_16` function with signature `BITMASK_16()` and selector `0xf96c5e52`
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
    #[ethcall(name = "BITMASK_16", abi = "BITMASK_16()")]
    pub struct Bitmask16Call;
    ///Container type for all input parameters for the `BITMASK_32` function with signature `BITMASK_32()` and selector `0x807c9782`
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
    #[ethcall(name = "BITMASK_32", abi = "BITMASK_32()")]
    pub struct Bitmask32Call;
    ///Container type for all input parameters for the `BITMASK_64` function with signature `BITMASK_64()` and selector `0x984898e5`
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
    #[ethcall(name = "BITMASK_64", abi = "BITMASK_64()")]
    pub struct Bitmask64Call;
    ///Container type for all input parameters for the `BITMASK_8` function with signature `BITMASK_8()` and selector `0xe326466d`
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
    #[ethcall(name = "BITMASK_8", abi = "BITMASK_8()")]
    pub struct Bitmask8Call;
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
    pub enum BitsCalls {
        Bitmask16(Bitmask16Call),
        Bitmask32(Bitmask32Call),
        Bitmask64(Bitmask64Call),
        Bitmask8(Bitmask8Call),
    }
    impl ::ethers::core::abi::AbiDecode for BitsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Bitmask16Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bitmask16(decoded));
            }
            if let Ok(decoded) = <Bitmask32Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bitmask32(decoded));
            }
            if let Ok(decoded) = <Bitmask64Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bitmask64(decoded));
            }
            if let Ok(decoded) = <Bitmask8Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bitmask8(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BitsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Bitmask16(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bitmask32(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bitmask64(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bitmask8(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BitsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Bitmask16(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bitmask32(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bitmask64(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bitmask8(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Bitmask16Call> for BitsCalls {
        fn from(value: Bitmask16Call) -> Self {
            Self::Bitmask16(value)
        }
    }
    impl ::core::convert::From<Bitmask32Call> for BitsCalls {
        fn from(value: Bitmask32Call) -> Self {
            Self::Bitmask32(value)
        }
    }
    impl ::core::convert::From<Bitmask64Call> for BitsCalls {
        fn from(value: Bitmask64Call) -> Self {
            Self::Bitmask64(value)
        }
    }
    impl ::core::convert::From<Bitmask8Call> for BitsCalls {
        fn from(value: Bitmask8Call) -> Self {
            Self::Bitmask8(value)
        }
    }
    ///Container type for all return fields from the `BITMASK_16` function with signature `BITMASK_16()` and selector `0xf96c5e52`
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
    pub struct Bitmask16Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BITMASK_32` function with signature `BITMASK_32()` and selector `0x807c9782`
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
    pub struct Bitmask32Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BITMASK_64` function with signature `BITMASK_64()` and selector `0x984898e5`
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
    pub struct Bitmask64Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BITMASK_8` function with signature `BITMASK_8()` and selector `0xe326466d`
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
    pub struct Bitmask8Return(pub ::ethers::core::types::U256);
}
