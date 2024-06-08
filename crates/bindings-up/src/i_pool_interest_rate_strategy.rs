pub use i_pool_interest_rate_strategy::*;
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
pub mod i_pool_interest_rate_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("calculateInterestRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInterestRates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct InterestUtils.CalculateInterestRatesParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getOptimalUsageRatio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOptimalUsageRatio",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRateSlope1"),
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
                    ::std::borrow::ToOwned::to_owned("getRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRateSlope2"),
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
                    ::std::borrow::ToOwned::to_owned("getRatebase"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRatebase"),
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
    pub static IPOOLINTERESTRATESTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPoolInterestRateStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPoolInterestRateStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPoolInterestRateStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPoolInterestRateStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPoolInterestRateStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPoolInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPoolInterestRateStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOOLINTERESTRATESTRATEGY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `calculateInterestRates` (0x9dad6199) function
        pub fn calculate_interest_rates(
            &self,
            params: CalculateInterestRatesParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 173, 97, 153], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOptimalUsageRatio` (0xa8602e86) function
        pub fn get_optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 96, 46, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRateSlope1` (0x2dd9037b) function
        pub fn get_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 217, 3, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRateSlope2` (0x75005bb0) function
        pub fn get_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 0, 91, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRatebase` (0x87f0409d) function
        pub fn get_ratebase(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([135, 240, 64, 157], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPoolInterestRateStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,address,address))` and selector `0x9dad6199`
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
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    ///Container type for all input parameters for the `getOptimalUsageRatio` function with signature `getOptimalUsageRatio()` and selector `0xa8602e86`
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
    #[ethcall(name = "getOptimalUsageRatio", abi = "getOptimalUsageRatio()")]
    pub struct GetOptimalUsageRatioCall;
    ///Container type for all input parameters for the `getRateSlope1` function with signature `getRateSlope1()` and selector `0x2dd9037b`
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
    #[ethcall(name = "getRateSlope1", abi = "getRateSlope1()")]
    pub struct GetRateSlope1Call;
    ///Container type for all input parameters for the `getRateSlope2` function with signature `getRateSlope2()` and selector `0x75005bb0`
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
    #[ethcall(name = "getRateSlope2", abi = "getRateSlope2()")]
    pub struct GetRateSlope2Call;
    ///Container type for all input parameters for the `getRatebase` function with signature `getRatebase()` and selector `0x87f0409d`
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
    #[ethcall(name = "getRatebase", abi = "getRatebase()")]
    pub struct GetRatebaseCall;
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
    pub enum IPoolInterestRateStrategyCalls {
        CalculateInterestRates(CalculateInterestRatesCall),
        GetOptimalUsageRatio(GetOptimalUsageRatioCall),
        GetRateSlope1(GetRateSlope1Call),
        GetRateSlope2(GetRateSlope2Call),
        GetRatebase(GetRatebaseCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolInterestRateStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CalculateInterestRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateInterestRates(decoded));
            }
            if let Ok(decoded) = <GetOptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOptimalUsageRatio(decoded));
            }
            if let Ok(decoded) = <GetRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRateSlope1(decoded));
            }
            if let Ok(decoded) = <GetRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRateSlope2(decoded));
            }
            if let Ok(decoded) = <GetRatebaseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRatebase(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPoolInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CalculateInterestRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRatebase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPoolInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateInterestRates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOptimalUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRateSlope1(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRateSlope2(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRatebase(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateInterestRatesCall>
    for IPoolInterestRateStrategyCalls {
        fn from(value: CalculateInterestRatesCall) -> Self {
            Self::CalculateInterestRates(value)
        }
    }
    impl ::core::convert::From<GetOptimalUsageRatioCall>
    for IPoolInterestRateStrategyCalls {
        fn from(value: GetOptimalUsageRatioCall) -> Self {
            Self::GetOptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<GetRateSlope1Call> for IPoolInterestRateStrategyCalls {
        fn from(value: GetRateSlope1Call) -> Self {
            Self::GetRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetRateSlope2Call> for IPoolInterestRateStrategyCalls {
        fn from(value: GetRateSlope2Call) -> Self {
            Self::GetRateSlope2(value)
        }
    }
    impl ::core::convert::From<GetRatebaseCall> for IPoolInterestRateStrategyCalls {
        fn from(value: GetRatebaseCall) -> Self {
            Self::GetRatebase(value)
        }
    }
    ///Container type for all return fields from the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,address,address))` and selector `0x9dad6199`
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
    pub struct CalculateInterestRatesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getOptimalUsageRatio` function with signature `getOptimalUsageRatio()` and selector `0xa8602e86`
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
    pub struct GetOptimalUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRateSlope1` function with signature `getRateSlope1()` and selector `0x2dd9037b`
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
    pub struct GetRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRateSlope2` function with signature `getRateSlope2()` and selector `0x75005bb0`
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
    pub struct GetRateSlope2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRatebase` function with signature `getRatebase()` and selector `0x87f0409d`
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
    pub struct GetRatebaseReturn(pub ::ethers::core::types::U256);
}
