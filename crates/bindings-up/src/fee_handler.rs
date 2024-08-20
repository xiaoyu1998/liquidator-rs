pub use fee_handler::*;
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
pub mod fee_handler {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_dataStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract DataStore"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_eventEmitter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract EventEmitter"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claimFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssets"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dataStore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dataStore"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract DataStore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eventEmitter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eventEmitter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract EventEmitter"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("EmptyPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyUnclaimedFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("EmptyUnclaimedFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EmptyUnderlyingAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EmptyUnderlyingAsset",
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
    pub static FEEHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct FeeHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FeeHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FeeHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FeeHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FeeHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FeeHandler)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FeeHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FEEHANDLER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `claimFees` (0x725acc6b) function
        pub fn claim_fees(
            &self,
            underlying_assets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 90, 204, 107], underlying_assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dataStore` (0x660d0d67) function
        pub fn data_store(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([102, 13, 13, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eventEmitter` (0x9ff78c30) function
        pub fn event_emitter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 247, 140, 48], ())
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
    for FeeHandler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EmptyPool` with signature `EmptyPool(address)` and selector `0x00ee0bb5`
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
    #[etherror(name = "EmptyPool", abi = "EmptyPool(address)")]
    pub struct EmptyPool {
        pub key: ::ethers::core::types::Address,
    }
    ///Custom Error type `EmptyUnclaimedFee` with signature `EmptyUnclaimedFee(address)` and selector `0xb48f6a0f`
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
    #[etherror(name = "EmptyUnclaimedFee", abi = "EmptyUnclaimedFee(address)")]
    pub struct EmptyUnclaimedFee {
        pub poool: ::ethers::core::types::Address,
    }
    ///Custom Error type `EmptyUnderlyingAsset` with signature `EmptyUnderlyingAsset()` and selector `0x30aaec23`
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
    #[etherror(name = "EmptyUnderlyingAsset", abi = "EmptyUnderlyingAsset()")]
    pub struct EmptyUnderlyingAsset;
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
    pub enum FeeHandlerErrors {
        EmptyPool(EmptyPool),
        EmptyUnclaimedFee(EmptyUnclaimedFee),
        EmptyUnderlyingAsset(EmptyUnderlyingAsset),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FeeHandlerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EmptyPool as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyPool(decoded));
            }
            if let Ok(decoded) = <EmptyUnclaimedFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyUnclaimedFee(decoded));
            }
            if let Ok(decoded) = <EmptyUnderlyingAsset as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EmptyUnderlyingAsset(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeeHandlerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EmptyPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyUnclaimedFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmptyUnderlyingAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FeeHandlerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EmptyPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EmptyUnclaimedFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyUnderlyingAsset as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FeeHandlerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmptyPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyUnclaimedFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyUnderlyingAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FeeHandlerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EmptyPool> for FeeHandlerErrors {
        fn from(value: EmptyPool) -> Self {
            Self::EmptyPool(value)
        }
    }
    impl ::core::convert::From<EmptyUnclaimedFee> for FeeHandlerErrors {
        fn from(value: EmptyUnclaimedFee) -> Self {
            Self::EmptyUnclaimedFee(value)
        }
    }
    impl ::core::convert::From<EmptyUnderlyingAsset> for FeeHandlerErrors {
        fn from(value: EmptyUnderlyingAsset) -> Self {
            Self::EmptyUnderlyingAsset(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for FeeHandlerErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    ///Container type for all input parameters for the `claimFees` function with signature `claimFees(address[])` and selector `0x725acc6b`
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
    #[ethcall(name = "claimFees", abi = "claimFees(address[])")]
    pub struct ClaimFeesCall {
        pub underlying_assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    #[ethcall(name = "dataStore", abi = "dataStore()")]
    pub struct DataStoreCall;
    ///Container type for all input parameters for the `eventEmitter` function with signature `eventEmitter()` and selector `0x9ff78c30`
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
    #[ethcall(name = "eventEmitter", abi = "eventEmitter()")]
    pub struct EventEmitterCall;
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
    pub enum FeeHandlerCalls {
        ClaimFees(ClaimFeesCall),
        DataStore(DataStoreCall),
        EventEmitter(EventEmitterCall),
        RoleStore(RoleStoreCall),
    }
    impl ::ethers::core::abi::AbiDecode for FeeHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimFees(decoded));
            }
            if let Ok(decoded) = <DataStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DataStore(decoded));
            }
            if let Ok(decoded) = <EventEmitterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EventEmitter(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RoleStore(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeeHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClaimFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DataStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EventEmitter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoleStore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FeeHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClaimFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::DataStore(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventEmitter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimFeesCall> for FeeHandlerCalls {
        fn from(value: ClaimFeesCall) -> Self {
            Self::ClaimFees(value)
        }
    }
    impl ::core::convert::From<DataStoreCall> for FeeHandlerCalls {
        fn from(value: DataStoreCall) -> Self {
            Self::DataStore(value)
        }
    }
    impl ::core::convert::From<EventEmitterCall> for FeeHandlerCalls {
        fn from(value: EventEmitterCall) -> Self {
            Self::EventEmitter(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for FeeHandlerCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    ///Container type for all return fields from the `dataStore` function with signature `dataStore()` and selector `0x660d0d67`
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
    pub struct DataStoreReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `eventEmitter` function with signature `eventEmitter()` and selector `0x9ff78c30`
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
    pub struct EventEmitterReturn(pub ::ethers::core::types::Address);
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
