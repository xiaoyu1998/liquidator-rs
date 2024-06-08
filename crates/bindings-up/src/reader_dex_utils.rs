pub use reader_dex_utils::*;
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
pub mod reader_dex_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_getDexPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_getDexPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetB"),
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
                    ::std::borrow::ToOwned::to_owned("_getDexPoolFeeAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_getDexPoolFeeAmount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetB"),
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
                (
                    ::std::borrow::ToOwned::to_owned("_getDexPoolSwapConstantFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_getDexPoolSwapConstantFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
    pub static READERDEXUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct ReaderDexUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReaderDexUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReaderDexUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReaderDexUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReaderDexUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReaderDexUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReaderDexUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    READERDEXUTILS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `_getDexPool` (0xa3bbcb37) function
        pub fn get_dex_pool(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [163, 187, 203, 55],
                    (data_store, underlying_asset_a, underlying_asset_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_getDexPoolFeeAmount` (0xbf41a4bd) function
        pub fn get_dex_pool_fee_amount(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [191, 65, 164, 189],
                    (data_store, underlying_asset_a, underlying_asset_b),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_getDexPoolSwapConstantFee` (0xd932da80) function
        pub fn get_dex_pool_swap_constant_fee(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [217, 50, 218, 128],
                    (data_store, underlying_asset_a, underlying_asset_b, amount_in),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ReaderDexUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `_getDexPool` function with signature `_getDexPool(address,address,address)` and selector `0xa3bbcb37`
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
    #[ethcall(name = "_getDexPool", abi = "_getDexPool(address,address,address)")]
    pub struct GetDexPoolCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_getDexPoolFeeAmount` function with signature `_getDexPoolFeeAmount(address,address,address)` and selector `0xbf41a4bd`
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
        name = "_getDexPoolFeeAmount",
        abi = "_getDexPoolFeeAmount(address,address,address)"
    )]
    pub struct GetDexPoolFeeAmountCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_getDexPoolSwapConstantFee` function with signature `_getDexPoolSwapConstantFee(address,address,address,uint256)` and selector `0xd932da80`
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
        name = "_getDexPoolSwapConstantFee",
        abi = "_getDexPoolSwapConstantFee(address,address,address,uint256)"
    )]
    pub struct GetDexPoolSwapConstantFeeCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
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
    pub enum ReaderDexUtilsCalls {
        GetDexPool(GetDexPoolCall),
        GetDexPoolFeeAmount(GetDexPoolFeeAmountCall),
        GetDexPoolSwapConstantFee(GetDexPoolSwapConstantFeeCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReaderDexUtilsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetDexPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPool(decoded));
            }
            if let Ok(decoded) = <GetDexPoolFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPoolFeeAmount(decoded));
            }
            if let Ok(decoded) = <GetDexPoolSwapConstantFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDexPoolSwapConstantFee(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReaderDexUtilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetDexPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDexPoolFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDexPoolSwapConstantFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReaderDexUtilsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetDexPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDexPoolFeeAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDexPoolSwapConstantFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetDexPoolCall> for ReaderDexUtilsCalls {
        fn from(value: GetDexPoolCall) -> Self {
            Self::GetDexPool(value)
        }
    }
    impl ::core::convert::From<GetDexPoolFeeAmountCall> for ReaderDexUtilsCalls {
        fn from(value: GetDexPoolFeeAmountCall) -> Self {
            Self::GetDexPoolFeeAmount(value)
        }
    }
    impl ::core::convert::From<GetDexPoolSwapConstantFeeCall> for ReaderDexUtilsCalls {
        fn from(value: GetDexPoolSwapConstantFeeCall) -> Self {
            Self::GetDexPoolSwapConstantFee(value)
        }
    }
    ///Container type for all return fields from the `_getDexPool` function with signature `_getDexPool(address,address,address)` and selector `0xa3bbcb37`
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
    pub struct GetDexPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `_getDexPoolFeeAmount` function with signature `_getDexPoolFeeAmount(address,address,address)` and selector `0xbf41a4bd`
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
    pub struct GetDexPoolFeeAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_getDexPoolSwapConstantFee` function with signature `_getDexPoolSwapConstantFee(address,address,address,uint256)` and selector `0xd932da80`
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
    pub struct GetDexPoolSwapConstantFeeReturn(pub ::ethers::core::types::U256);
}
