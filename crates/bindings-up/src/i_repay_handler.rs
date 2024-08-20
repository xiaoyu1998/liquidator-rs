pub use i_repay_handler::*;
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
pub mod i_repay_handler {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("executeRepay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRepay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepayUtils.RepayParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("executeRepaySubstitute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeRepaySubstitute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepaySubstituteUtils.RepaySubstituteParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static IREPAYHANDLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IRepayHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IRepayHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IRepayHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IRepayHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IRepayHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IRepayHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IRepayHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IREPAYHANDLER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `executeRepay` (0xeb683c4d) function
        pub fn execute_repay(
            &self,
            account: ::ethers::core::types::Address,
            params: RepayParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 104, 60, 77], (account, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRepaySubstitute` (0x15c59a01) function
        pub fn execute_repay_substitute(
            &self,
            account: ::ethers::core::types::Address,
            params: RepaySubstituteParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 197, 154, 1], (account, params))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IRepayHandler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `executeRepay` function with signature `executeRepay(address,(address,uint256))` and selector `0xeb683c4d`
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
    #[ethcall(name = "executeRepay", abi = "executeRepay(address,(address,uint256))")]
    pub struct ExecuteRepayCall {
        pub account: ::ethers::core::types::Address,
        pub params: RepayParams,
    }
    ///Container type for all input parameters for the `executeRepaySubstitute` function with signature `executeRepaySubstitute(address,(address,uint256,address))` and selector `0x15c59a01`
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
        name = "executeRepaySubstitute",
        abi = "executeRepaySubstitute(address,(address,uint256,address))"
    )]
    pub struct ExecuteRepaySubstituteCall {
        pub account: ::ethers::core::types::Address,
        pub params: RepaySubstituteParams,
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
    pub enum IRepayHandlerCalls {
        ExecuteRepay(ExecuteRepayCall),
        ExecuteRepaySubstitute(ExecuteRepaySubstituteCall),
    }
    impl ::ethers::core::abi::AbiDecode for IRepayHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteRepayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepay(decoded));
            }
            if let Ok(decoded) = <ExecuteRepaySubstituteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepaySubstitute(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IRepayHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteRepay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRepaySubstitute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IRepayHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecuteRepay(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRepaySubstitute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ExecuteRepayCall> for IRepayHandlerCalls {
        fn from(value: ExecuteRepayCall) -> Self {
            Self::ExecuteRepay(value)
        }
    }
    impl ::core::convert::From<ExecuteRepaySubstituteCall> for IRepayHandlerCalls {
        fn from(value: ExecuteRepaySubstituteCall) -> Self {
            Self::ExecuteRepaySubstitute(value)
        }
    }
}
