pub use i_dex_2::*;
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
pub mod i_dex_2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenB"),
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
                    ::std::borrow::ToOwned::to_owned("getSwapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapFee"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("swapExactIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("swapExactOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
    pub static IDEX2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IDex2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDex2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDex2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDex2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDex2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IDex2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IDex2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IDEX2_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getFeeAmount` (0xbf5c9822) function
        pub fn get_fee_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 92, 152, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x531aa03e) function
        pub fn get_pool(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 26, 160, 62], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFee` (0x8e7d6935) function
        pub fn get_swap_fee(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 125, 105, 53], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactIn` (0x2c67e257) function
        pub fn swap_exact_in(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 103, 226, 87],
                    (from, token_in, token_out, amount, sqrt_price_limit_x96, to),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactOut` (0x9e3bb59d) function
        pub fn swap_exact_out(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [158, 59, 181, 157],
                    (from, token_in, token_out, amount, sqrt_price_limit_x96, to),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IDex2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
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
    #[ethcall(name = "getFeeAmount", abi = "getFeeAmount()")]
    pub struct GetFeeAmountCall;
    ///Container type for all input parameters for the `getPool` function with signature `getPool(address,address)` and selector `0x531aa03e`
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
    #[ethcall(name = "getPool", abi = "getPool(address,address)")]
    pub struct GetPoolCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
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
    #[ethcall(name = "getSwapFee", abi = "getSwapFee(uint256)")]
    pub struct GetSwapFeeCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `swapExactIn` function with signature `swapExactIn(address,address,address,uint256,uint256,address)` and selector `0x2c67e257`
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
        name = "swapExactIn",
        abi = "swapExactIn(address,address,address,uint256,uint256,address)"
    )]
    pub struct SwapExactInCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapExactOut` function with signature `swapExactOut(address,address,address,uint256,uint256,address)` and selector `0x9e3bb59d`
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
        name = "swapExactOut",
        abi = "swapExactOut(address,address,address,uint256,uint256,address)"
    )]
    pub struct SwapExactOutCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
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
    pub enum IDex2Calls {
        GetFeeAmount(GetFeeAmountCall),
        GetPool(GetPoolCall),
        GetSwapFee(GetSwapFeeCall),
        SwapExactIn(SwapExactInCall),
        SwapExactOut(SwapExactOutCall),
    }
    impl ::ethers::core::abi::AbiDecode for IDex2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFeeAmount(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <SwapExactInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactIn(decoded));
            }
            if let Ok(decoded) = <SwapExactOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactOut(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IDex2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IDex2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetFeeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactOut(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetFeeAmountCall> for IDex2Calls {
        fn from(value: GetFeeAmountCall) -> Self {
            Self::GetFeeAmount(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for IDex2Calls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetSwapFeeCall> for IDex2Calls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<SwapExactInCall> for IDex2Calls {
        fn from(value: SwapExactInCall) -> Self {
            Self::SwapExactIn(value)
        }
    }
    impl ::core::convert::From<SwapExactOutCall> for IDex2Calls {
        fn from(value: SwapExactOutCall) -> Self {
            Self::SwapExactOut(value)
        }
    }
    ///Container type for all return fields from the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
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
    pub struct GetFeeAmountReturn(pub u32);
    ///Container type for all return fields from the `getPool` function with signature `getPool(address,address)` and selector `0x531aa03e`
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
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
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
    pub struct GetSwapFeeReturn(pub ::ethers::core::types::U256);
}
