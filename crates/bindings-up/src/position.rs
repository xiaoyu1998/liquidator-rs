pub use position::*;
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
pub mod position {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PositionTypeLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PositionTypeLong"),
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
                    ::std::borrow::ToOwned::to_owned("PositionTypeNone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PositionTypeNone"),
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
                    ::std::borrow::ToOwned::to_owned("PositionTypeShort"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PositionTypeShort"),
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
    pub static POSITION_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA9`7`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`*WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`GW`\x005`\xE0\x1C\x80c-\x81\x86\xA4\x14`LW\x80cB\xE4\xF8-\x14`eW\x80c\xBE\x08>[\x14`lW[`\0\x80\xFD[`S`\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`S`\x01\x81V[`S`\x02\x81V\xFE\xA2dipfsX\"\x12 Pf\x1A\x9D\xDF\xB5!\x93m\x16|\x84ED\xDA\xC5/=\xCE\xC5%U\xD4\x03\xF0\xD0\xECd\xD4p|\xB6dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static POSITION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`GW`\x005`\xE0\x1C\x80c-\x81\x86\xA4\x14`LW\x80cB\xE4\xF8-\x14`eW\x80c\xBE\x08>[\x14`lW[`\0\x80\xFD[`S`\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`S`\x01\x81V[`S`\x02\x81V\xFE\xA2dipfsX\"\x12 Pf\x1A\x9D\xDF\xB5!\x93m\x16|\x84ED\xDA\xC5/=\xCE\xC5%U\xD4\x03\xF0\xD0\xECd\xD4p|\xB6dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static POSITION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Position<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Position<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Position<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Position<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Position<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Position)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Position<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POSITION_ABI.clone(),
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
                POSITION_ABI.clone(),
                POSITION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `PositionTypeLong` (0x42e4f82d) function
        pub fn position_type_long(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([66, 228, 248, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PositionTypeNone` (0xbe083e5b) function
        pub fn position_type_none(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([190, 8, 62, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PositionTypeShort` (0x2d8186a4) function
        pub fn position_type_short(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 129, 134, 164], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Position<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `PositionTypeLong` function with signature `PositionTypeLong()` and selector `0x42e4f82d`
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
    #[ethcall(name = "PositionTypeLong", abi = "PositionTypeLong()")]
    pub struct PositionTypeLongCall;
    ///Container type for all input parameters for the `PositionTypeNone` function with signature `PositionTypeNone()` and selector `0xbe083e5b`
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
    #[ethcall(name = "PositionTypeNone", abi = "PositionTypeNone()")]
    pub struct PositionTypeNoneCall;
    ///Container type for all input parameters for the `PositionTypeShort` function with signature `PositionTypeShort()` and selector `0x2d8186a4`
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
    #[ethcall(name = "PositionTypeShort", abi = "PositionTypeShort()")]
    pub struct PositionTypeShortCall;
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
    pub enum PositionCalls {
        PositionTypeLong(PositionTypeLongCall),
        PositionTypeNone(PositionTypeNoneCall),
        PositionTypeShort(PositionTypeShortCall),
    }
    impl ::ethers::core::abi::AbiDecode for PositionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PositionTypeLongCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionTypeLong(decoded));
            }
            if let Ok(decoded) = <PositionTypeNoneCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionTypeNone(decoded));
            }
            if let Ok(decoded) = <PositionTypeShortCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionTypeShort(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PositionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::PositionTypeLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PositionTypeNone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PositionTypeShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PositionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PositionTypeLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionTypeNone(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionTypeShort(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PositionTypeLongCall> for PositionCalls {
        fn from(value: PositionTypeLongCall) -> Self {
            Self::PositionTypeLong(value)
        }
    }
    impl ::core::convert::From<PositionTypeNoneCall> for PositionCalls {
        fn from(value: PositionTypeNoneCall) -> Self {
            Self::PositionTypeNone(value)
        }
    }
    impl ::core::convert::From<PositionTypeShortCall> for PositionCalls {
        fn from(value: PositionTypeShortCall) -> Self {
            Self::PositionTypeShort(value)
        }
    }
    ///Container type for all return fields from the `PositionTypeLong` function with signature `PositionTypeLong()` and selector `0x42e4f82d`
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
    pub struct PositionTypeLongReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PositionTypeNone` function with signature `PositionTypeNone()` and selector `0xbe083e5b`
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
    pub struct PositionTypeNoneReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PositionTypeShort` function with signature `PositionTypeShort()` and selector `0x2d8186a4`
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
    pub struct PositionTypeShortReturn(pub ::ethers::core::types::U256);
}
