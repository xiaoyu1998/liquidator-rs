///Module containing a contract's types and functions.
/**

```solidity
library LiquidityUtils {
    struct AddParams { address token0; address token1; address to; }
    struct RemoveParams { address token0; address token1; uint256 liquidity; address to; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod LiquidityUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct AddParams { address token0; address token1; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AddParams {
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AddParams> for UnderlyingRustTuple<'_> {
            fn from(value: AddParams) -> Self {
                (value.token0, value.token1, value.to)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AddParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token0: tuple.0,
                    token1: tuple.1,
                    to: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AddParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AddParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AddParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for AddParams {
            const NAME: &'static str = "AddParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AddParams(address token0,address token1,address to)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AddParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token1,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token0,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token1,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct RemoveParams { address token0; address token1; uint256 liquidity; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RemoveParams {
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RemoveParams> for UnderlyingRustTuple<'_> {
            fn from(value: RemoveParams) -> Self {
                (value.token0, value.token1, value.liquidity, value.to)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RemoveParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token0: tuple.0,
                    token1: tuple.1,
                    liquidity: tuple.2,
                    to: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RemoveParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RemoveParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RemoveParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RemoveParams {
            const NAME: &'static str = "RemoveParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RemoveParams(address token0,address token1,uint256 liquidity,address to)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.liquidity)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.to,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RemoveParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidity,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.to,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token0,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token1,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.to,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LiquidityUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidityUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LiquidityUtilsInstance<T, P, N> {
        LiquidityUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`LiquidityUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LiquidityUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LiquidityUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LiquidityUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LiquidityUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`LiquidityUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidityUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LiquidityUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LiquidityUtilsInstance<T, P, N> {
            LiquidityUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library LiquidityUtils {
    struct AddParams {
        address token0;
        address token1;
        address to;
    }
    struct RemoveParams {
        address token0;
        address token1;
        uint256 liquidity;
        address to;
    }
}

interface LiquidityHandler {
    error EmptyAddAmounts();
    error EmptyPool(bytes32 key);
    error EmptyRemoveAmounts();
    error InsufficientUserBalance(uint256 balance, uint256 liquidity);
    error MathOverflowedMulDiv();
    error Reserve0Insufficient(uint256 amount0, uint256 availableReserve0);
    error Reserve1Insufficient(uint256 amount1, uint256 availableReserve1);
    error Unauthorized(address msgSender, string role);

    constructor(address _roleStore, address _dataStore, address _eventEmitter);

    function dataStore() external view returns (address);
    function eventEmitter() external view returns (address);
    function executeAdd(address account, LiquidityUtils.AddParams memory AddParams) external;
    function executeRemove(address account, LiquidityUtils.RemoveParams memory removeParams) external;
    function roleStore() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_roleStore",
        "type": "address",
        "internalType": "contract RoleStore"
      },
      {
        "name": "_dataStore",
        "type": "address",
        "internalType": "contract DataStore"
      },
      {
        "name": "_eventEmitter",
        "type": "address",
        "internalType": "contract EventEmitter"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "dataStore",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract DataStore"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eventEmitter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EventEmitter"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "executeAdd",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "AddParams",
        "type": "tuple",
        "internalType": "struct LiquidityUtils.AddParams",
        "components": [
          {
            "name": "token0",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token1",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "executeRemove",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "removeParams",
        "type": "tuple",
        "internalType": "struct LiquidityUtils.RemoveParams",
        "components": [
          {
            "name": "token0",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "token1",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "liquidity",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "to",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "roleStore",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RoleStore"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "EmptyAddAmounts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyPool",
    "inputs": [
      {
        "name": "key",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "EmptyRemoveAmounts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientUserBalance",
    "inputs": [
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MathOverflowedMulDiv",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Reserve0Insufficient",
    "inputs": [
      {
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableReserve0",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "Reserve1Insufficient",
    "inputs": [
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableReserve1",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "Unauthorized",
    "inputs": [
      {
        "name": "msgSender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "role",
        "type": "string",
        "internalType": "string"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod LiquidityHandler {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b506040516151e03803806151e083398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c0516150d561010b5f395f818160f0015281816101c601526102de01525f8181605e015261055601525f818160b601528181610197015281816102af0152818161037f01528181610485015261077601526150d55ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614cb8565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614cf6565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614d26565b6001600160a01b031681526020018360200160208101906102269190614d26565b6001600160a01b031681526020016102446060850160408601614d26565b6001600160a01b03169052905061025b83826105e8565b50610264610774565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614d26565b6001600160a01b0316815260200183602001602081019061033e9190614d26565b6001600160a01b031681526040808501356020830152016103656080850160608601614d26565b6001600160a01b03169052905061025b838261082d565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614d41565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614d78565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614d41565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614d78565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614d8f565b61026457338160405163a35b150b60e01b815260040161047a929190614ddc565b5f5f610600835f015184604001518560600151610a59565b915091505f61060e83610a9c565b905061061e845f01518483610b4c565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614d78565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614d78565b90506107138282610f55565b610724868484848b60800151610f7e565b865161073090876110bc565b61073e876020015187611101565b865161074b908688611125565b61076a87602001518989604001518a606001518b60800151878761233a565b5050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b290614d41565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610806573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082a9190614d78565b50565b610835614b61565b61084b825f015183604001518460600151610a59565b6020830152808252608083015161086391859161238e565b81518151608084015161087792919061244b565b60c0850181905260a08501829052610120850183905261010085018490526108a193929190612684565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108f9575f5ffd5b505af115801561090b573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610972575f5ffd5b505af1158015610984573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109ec575f5ffd5b505af11580156109fe573d5f5f3e3d5ffd5b50505060208301518251610a129250611101565b815160208201518251610a26929190611125565b610a548260200151848460400151856060015186608001518760a001518761010001518861012001516126da565b505050565b610a61614bba565b5f5f610a6d8585612759565b90505f610a7a8783612801565b9050610a868183613a03565b610a8f81613a31565b925090505b935093915050565b5f5f610aaa835f5f5f613add565b50505090505f610abd8460015f5f613add565b5050509050805f03610ad257505f9392505050565b5f610ae185606001515f613bc6565b90505f610af386606001516001613bc6565b90505f610b1685676765c793fa10079d601b1b610b1186600a614eee565b613bf4565b90505f610b3485676765c793fa10079d601b1b610b1186600a614eee565b9050610b408282613cb4565b98975050505050505050565b610b7c6040518060400160405280600f81526020016e75706461746554776170507269636560881b815250613cef565b5f610b878484613d13565b90505f610b9964010000000042614f0d565b90508163ffffffff165f03610c4357610bb3858583613d89565b610bbe85855f613e0d565b610bc9858583613e4c565b610bd485855f613e64565b610bdf858585613e7c565b610c156040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b8152508263ffffffff16613e94565b610c3c60405180604001604052806005815260200164707269636560d81b81525084613e94565b5050505050565b5f610c4e8383614f20565b9050610c836040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b8152508263ffffffff16613e94565b63ffffffff811615610d92575f610c9a8787613ebd565b90505f610cad63ffffffff841687614f3c565b610cb79083614f53565b9050610ce060405180604001604052806005815260200164707269636560d81b81525087613e94565b610d136040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b8152508463ffffffff16613e94565b610d48604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b81525083613e94565b610d796040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082613e94565b610d84888883613e0d565b610d8f888887613d89565b50505b5f610d9d8787613ed6565b90505f610daa8285614f20565b90505f610db689613eef565b9050610de6604051806040016040528060068152602001651411549253d160d21b8152508263ffffffff16613e94565b610e2b6040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508463ffffffff16613e94565b610e66604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508363ffffffff16613e94565b8063ffffffff168263ffffffff161115610f4a575f610e858a8a613ebd565b90505f610e928b8b613fa1565b90505f63ffffffff8516610ea68385614f66565b610eb09190614f79565b9050879550829150610ef76040518060400160405280601b81526020017f707269636543756d756c61746976654c6173744279506572696f64000000000081525083613e94565b610f256040518060400160405280600c81526020016b70726963654176657261676560a01b81525082613e94565b610f308c8c88613e4c565b610f3b8c8c84613e64565b610f468c8c83613e7c565b5050505b505050505050505050565b811580610f60575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f610f8c8685856001613fba565b90505f856001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fef9190614d78565b9050805f03611055576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038716906340c10f19906044015f604051808303815f87803b15801561103e575f5ffd5b505af1158015611050573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038481166004830152602482018490528716906340c10f19906044015b5f604051808303815f87803b15801561109d575f5ffd5b505af11580156110af573d5f5f3e3d5ffd5b5050505050505050505050565b6060810151660800000000000016610264575f5f6110da84846140f3565b915091508181106110fb5760608301516110f5906001614154565b60608401525b50505050565b61110a8161417e565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611165906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156111b5575f5ffd5b505af11580156111c7573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161120b906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161123b929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561129e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c29190614fa0565b50806001600160a01b031663e2a4853a8460405160200161130a906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161133a929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611397573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113bb9190614d78565b50806001600160a01b031663e2a4853a84604051602001611402906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611432929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561148e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114b29190614d78565b50806001600160a01b031663e2a4853a846040516020016114fe906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161152e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561158b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115af9190614d78565b50806001600160a01b031663e2a4853a846040516020016115cf90614fbb565b604051602081830303815290604052805190602001206040516020016115ff929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561165c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116809190614d78565b50806001600160a01b031663e2a4853a846040516020016116cd906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561175a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061177e9190614d78565b50806001600160a01b031663e2a4853a846040516020016117c7906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016117f7929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611854573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118789190614d78565b50806001600160a01b031663ca446dd9846040516020016118b9906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016118e9929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561194f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119739190614fa0565b50806001600160a01b031663e2a4853a846040516020016119bb90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016119eb929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e9190614d78565b50806001600160a01b031663e2a4853a84604051602001611ab590602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ae5929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b689190614d78565b50806001600160a01b031663e2a4853a84604051602001611bb490602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611be4929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c689190614d78565b50806001600160a01b031663e2a4853a84604051602001611c8890614ffc565b60405160208183030381529060405280519060200120604051602001611cb8929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d3c9190614d78565b50806001600160a01b031663e2a4853a84604051602001611d8990602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611db9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e19573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e3d9190614d78565b50806001600160a01b031663e2a4853a84604051602001611e8690602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611eb6929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f16573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f3a9190614d78565b50806001600160a01b031663ca446dd984604051602001611f7890602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fa8929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611ff29291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561200e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120329190614fa0565b50806001600160a01b031663ca446dd984604051602001612084906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016120b4929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b1683526120ff926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561211b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061213f9190614fa0565b50806001600160a01b031663e2a4853a84604051602001612186906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016121b6929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b81526004016121f7929190918252602082015260400190565b6020604051808303815f875af1158015612213573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122379190614d78565b50806001600160a01b031663e2a4853a84604051602001612289906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016122b9929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b81526004016122fa929190918252602082015260400190565b6020604051808303815f875af1158015612316573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c3c9190614d78565b604051639ed486eb60e01b81526001600160a01b0387811660048301528681166024830152858116604483015284811660648301526084820184905260a48201839052881690639ed486eb9060c401611086565b805f036123ae57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa1580156123fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241e9190614d78565b905080831115610c3c576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f612457614bee565b6124608861438d565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156124aa573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124ce9190614d78565b8160200181815250506124e8875f5f846101400151613add565b50608084015250604082015261014081015161250a9088906001905f90613add565b5060a08401525060608201526040810151602082015161252b918891613bf4565b61010082015260608101516020820151612546918891613bf4565b816101200181815250506125866040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613e94565b6125be604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613e94565b6125f660405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613e94565b6126296040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613e94565b61265c6040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613e94565b80610100015181610120015182608001518360a0015194509450945094505093509350935093565b818411156126af57604051631fc107c160e01b8152600481018590526024810183905260440161047a565b808311156110fb57604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4015f604051808303815f87803b158015612747575f5ffd5b505af1158015610f46573d5f5f3e3d5ffd5b5f816001600160a01b0316836001600160a01b03161061277a57818361277d565b82825b60405191945092506127aa906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612809614bba565b82612812614bba565b816001600160a01b03166391d4403c60405160200161284e906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156128a2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128c69190614d8f565b6128d35791506127fb9050565b816001600160a01b03166321f8a72185604051602001612913906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612943929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161297791815260200190565b602060405180830381865afa158015612992573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b69190614fa0565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612a34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612a6891815260200190565b602060405180830381865afa158015612a83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aa79190614d78565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612afd906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b2d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612b6191815260200190565b602060405180830381865afa158015612b7c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ba09190614d78565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612bfb906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c2b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612c5f91815260200190565b602060405180830381865afa158015612c7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9e9190614d78565b815151606001526040516001600160a01b0383169063bd02d0f5908690612cc790602001614fbb565b60405160208183030381529060405280519060200120604051602001612cf7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612d2b91815260200190565b602060405180830381865afa158015612d46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6a9190614d78565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612dc6906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612df6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e2a91815260200190565b602060405180830381865afa158015612e45573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e699190614d78565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612ee6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f1a91815260200190565b602060405180830381865afa158015612f35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f599190614d78565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001612fce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161300291815260200190565b602060405180830381865afa15801561301d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130419190614fa0565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016130eb91815260200190565b602060405180830381865afa158015613106573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061312a9190614d78565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161318190602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016131b1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016131e591815260200190565b602060405180830381865afa158015613200573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132249190614d78565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161328090602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016132b0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016132e491815260200190565b602060405180830381865afa1580156132ff573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133239190614d78565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161335390614ffc565b60405160208183030381529060405280519060200120604051602001613383929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133b791815260200190565b602060405180830381865afa1580156133d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133f69190614d78565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161345390602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613483929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134b791815260200190565b602060405180830381865afa1580156134d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134f69190614d78565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161354f90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161357f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135b391815260200190565b602060405180830381865afa1580156135ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135f29190614d78565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161364090602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613670929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136a491815260200190565b602060405180830381865afa1580156136bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e39190614fa0565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613751906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613781929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137b591815260200190565b602060405180830381865afa1580156137d0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137f49190614fa0565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001613857906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613887929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138bb91815260200190565b602060405180830381865afa1580156138d6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138fa9190614d78565b60608201526040516001600160a01b0383169063bd02d0f5908690613953906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613983929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016139b791815260200190565b602060405180830381865afa1580156139d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139f69190614d78565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613a43575050565b81515160a0015115613a995781515f90613a6c90825b60200201516040015184608001516143de565b8351909150613a909082905f5b60200201516020015161441a90919063ffffffff16565b83515160200152505b81516020015160a0015115613ad55781515f90613ab7906001613a59565b8351909150613ac99082906001613a79565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613afa57613afa614f8c565b602002015190505f613b0c8a8a61445b565b9050805f03613b28575f5f5f5f95509550955095505050613bbb565b5f613b37838c60800151614549565b90505f613b44828a61441a565b90505f8915613b6957818410613b6357613b5e8483614579565b613b6b565b5f613b6b565b5f5b90505f613b78858d61441a565b90505f8c15613b9d57848210613b9757613b928286614579565b613b9f565b5f613b9f565b5f5b9050613bab8587614f53565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613be6575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613c2857838281613c1e57613c1e614ef9565b0492505050613cad565b808411613c485760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613cd5575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61082a60405180604001604052806002815260200161257360f01b815250826145ce565b5f826001600160a01b031663bd02d0f5613d2c84614613565b6040518263ffffffff1660e01b8152600401613d4a91815260200190565b602060405180830381865afa158015613d65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cad9190614d78565b826001600160a01b031663e2a4853a613da184614613565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613de9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110fb9190614d78565b826001600160a01b031663e2a4853a613e25846146c6565b836040518363ffffffff1660e01b8152600401613dcd929190918252602082015260400190565b826001600160a01b031663e2a4853a613da18461471a565b826001600160a01b031663e2a4853a613e258461477b565b826001600160a01b031663e2a4853a613e25846147e1565b610264604051806040016040528060068152602001652573202d257360d01b815250838361482a565b5f826001600160a01b031663bd02d0f5613d2c846146c6565b5f826001600160a01b031663bd02d0f5613d2c8461471a565b5f816001600160a01b031663bd02d0f5604051602001613f2e906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f6291815260200190565b602060405180830381865afa158015613f7d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127fb9190614d78565b5f826001600160a01b031663bd02d0f5613d2c8461477b565b5f613fc3614bee565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015614003573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140279190614d78565b6020820152614038865f8080613add565b50505060c082015261404d8660015f80613add565b50505060e0820152821561408857848160c00181815161406d9190614f66565b90525060e081018051859190614084908390614f66565b9052505b80602001515f036140b8576140b16103e86140ab6140a68888614871565b6148d7565b90614579565b81526140e9565b6140e66140ce8683602001518460c00151613bf4565b6140e18684602001518560e00151613bf4565b6149b7565b81525b5195945050505050565b5f5f5f61410484606001515f613bc6565b90505f614110866149cc565b90505f6141338261412285600a614eee565b676765c793fa10079d601b1b613bf4565b90505f614142875f5f5f613add565b50939a91995090975050505050505050565b5f603382614162575f614165565b60015b60ff16901b660800000000000019841617905092915050565b5f5f6141b36040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6141bd845f614a1d565b6020830152815260608401516141d3905f613bc6565b6060820181905281516141f891676765c793fa10079d601b1b90610b1190600a614eee565b604082015260208101515f03614213575f60808201526142b3565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614289573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142ad9190614d78565b60808201525b6142be846001614a1d565b602083018190529082525f036142d9575f60a0820152614379565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561434f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143739190614d78565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001613f2e906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f806143ea8342614f66565b6143f49085614f3c565b6301e133809004905061441281676765c793fa10079d601b1b614f53565b949350505050565b5f81156b019d971e4fe8401e74000000198390048411151761443a575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff166002811061447557614475614f8c565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156144ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144f29190614d78565b9050805f03614505575f925050506127fb565b606082015160c08301516145199082614f53565b821061453d5760c083015161452e8284614f66565b6145389190614f66565b61453f565b5f5b9695505050505050565b5f8260a001515f0361455c57505f6127fb565b5f6145678484614a63565b60a0850151909150614412908261441a565b5f826145858382614f66565b91508111156127fb5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b61026482826040516024016145e492919061503d565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614aa6565b80515f90819061463490825b60200201515184516001602002015151612759565b905080604051602001614678906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016146a8929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906146d6908261461f565b905080604051602001614678906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061472a908261461f565b9050806040516020016146789060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f90819061478b908261461f565b9050806040516020016146789060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906147f1908261461f565b90508060405160200161467890602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a548383836040516024016148429392919061506a565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614aa6565b5f811580614894575082826148868183614f3c565b92506148929083614f79565b145b6127fb5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f036148e657505f919050565b5f60016148f284614aaf565b901c6001901b9050600181848161490b5761490b614ef9565b048201901c9050600181848161492357614923614ef9565b048201901c9050600181848161493b5761493b614ef9565b048201901c9050600181848161495357614953614ef9565b048201901c9050600181848161496b5761496b614ef9565b048201901c9050600181848161498357614983614ef9565b048201901c9050600181848161499b5761499b614ef9565b048201901c9050613cad818285816149b5576149b5614ef9565b045b5f8183106149c55781613cad565b5090919050565b5f816001600160a01b031663bd02d0f5604051602001613f2e9060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614a4a855f01518560ff1660028110614a3b57614a3b614f8c565b60200201518660800151614549565b90505f614a57868661445b565b96919550909350505050565b5f428203614a76575060208201516127fb565b5f614a858460400151846143de565b9050614a9e84602001518261441a90919063ffffffff16565b9150506127fb565b61082a81614b42565b5f80608083901c15614ac357608092831c92015b604083901c15614ad557604092831c92015b602083901c15614ae757602092831c92015b601083901c15614af957601092831c92015b600883901c15614b0b57600892831c92015b600483901c15614b1d57600492831c92015b600283901c15614b2f57600292831c92015b600183901c156127fb5760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614b75614bba565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614bcd614c3d565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614c8e6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614c4c5790505090565b6001600160a01b038116811461082a575f5ffd5b5f5f8284036080811215614cca575f5ffd5b8335614cd581614ca4565b92506060601f1982011215614ce8575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614d08575f5ffd5b8335614d1381614ca4565b92506080601f1982011215614ce8575f5ffd5b5f60208284031215614d36575f5ffd5b8135613cad81614ca4565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614d88575f5ffd5b5051919050565b5f60208284031215614d9f575f5ffd5b81518015158114613cad575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061441290830184614dae565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9457808504811115614e3257614e32614dff565b6001841615614e4057908102905b60019390931c928002614e17565b5f82614e5c575060016127fb565b81614e6857505f6127fb565b8160018114614e7e5760028114614e8857614ea4565b60019150506127fb565b60ff841115614e9957614e99614dff565b50506001821b6127fb565b5060208310610133831016604e8410600b8410161715614ec7575081810a6127fb565b614ed35f198484614e13565b805f1904821115614ee657614ee6614dff565b029392505050565b5f613cad8383614e4e565b634e487b7160e01b5f52601260045260245ffd5b5f82614f1b57614f1b614ef9565b500690565b63ffffffff82811682821603908111156127fb576127fb614dff565b80820281158282048414176127fb576127fb614dff565b808201808211156127fb576127fb614dff565b818103818111156127fb576127fb614dff565b5f82614f8757614f87614ef9565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614fb0575f5ffd5b8151613cad81614ca4565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61504f6040830185614dae565b82810360208401526150618185614dae565b95945050505050565b606081525f61507c6060830186614dae565b828103602084015261508e8186614dae565b91505082604083015294935050505056fea26469706673582212202f39307b89e0f8c664297857c7c61e43ff1e24164a31d8b37933ef7483dbc9a764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaQ\xE08\x03\x80aQ\xE0\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QaP\xD5a\x01\x0B_9_\x81\x81`\xF0\x01R\x81\x81a\x01\xC6\x01Ra\x02\xDE\x01R_\x81\x81`^\x01Ra\x05V\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\xAF\x01R\x81\x81a\x03\x7F\x01R\x81\x81a\x04\x85\x01Ra\x07v\x01RaP\xD5_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aL\xB8V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aL\xF6V[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07tV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x08-V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aMAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aMxV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aMAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aMxV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aM\x8FV[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aM\xDCV[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\nYV[\x91P\x91P_a\x06\x0E\x83a\n\x9CV[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0BLV[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aMxV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aMxV[\x90Pa\x07\x13\x82\x82a\x0FUV[a\x07$\x86\x84\x84\x84\x8B`\x80\x01Qa\x0F~V[\x86Qa\x070\x90\x87a\x10\xBCV[a\x07>\x87` \x01Q\x87a\x11\x01V[\x86Qa\x07K\x90\x86\x88a\x11%V[a\x07j\x87` \x01Q\x89\x89`@\x01Q\x8A``\x01Q\x8B`\x80\x01Q\x87\x87a#:V[PPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB2\x90aMAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08*\x91\x90aMxV[PV[a\x085aKaV[a\x08K\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\nYV[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08c\x91\x85\x91a#\x8EV[\x81Q\x81Q`\x80\x84\x01Qa\x08w\x92\x91\x90a$KV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA1\x93\x92\x91\x90a&\x84V[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xF9W__\xFD[PZ\xF1\x15\x80\x15a\t\x0BW=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\trW__\xFD[PZ\xF1\x15\x80\x15a\t\x84W=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xECW__\xFD[PZ\xF1\x15\x80\x15a\t\xFEW=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x12\x92Pa\x11\x01V[\x81Q` \x82\x01Q\x82Qa\n&\x92\x91\x90a\x11%V[a\nT\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa&\xDAV[PPPV[a\naaK\xBAV[__a\nm\x85\x85a'YV[\x90P_a\nz\x87\x83a(\x01V[\x90Pa\n\x86\x81\x83a:\x03V[a\n\x8F\x81a:1V[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xAA\x83___a:\xDDV[PPP\x90P_a\n\xBD\x84`\x01__a:\xDDV[PPP\x90P\x80_\x03a\n\xD2WP_\x93\x92PPPV[_a\n\xE1\x85``\x01Q_a;\xC6V[\x90P_a\n\xF3\x86``\x01Q`\x01a;\xC6V[\x90P_a\x0B\x16\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x11\x86`\naN\xEEV[a;\xF4V[\x90P_a\x0B4\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x11\x86`\naN\xEEV[\x90Pa\x0B@\x82\x82a<\xB4V[\x98\x97PPPPPPPPV[a\x0B|`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nupdateTwapPrice`\x88\x1B\x81RPa<\xEFV[_a\x0B\x87\x84\x84a=\x13V[\x90P_a\x0B\x99d\x01\0\0\0\0BaO\rV[\x90P\x81c\xFF\xFF\xFF\xFF\x16_\x03a\x0CCWa\x0B\xB3\x85\x85\x83a=\x89V[a\x0B\xBE\x85\x85_a>\rV[a\x0B\xC9\x85\x85\x83a>LV[a\x0B\xD4\x85\x85_a>dV[a\x0B\xDF\x85\x85\x85a>|V[a\x0C\x15`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0C<`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x84a>\x94V[PPPPPV[_a\x0CN\x83\x83aO V[\x90Pa\x0C\x83`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[c\xFF\xFF\xFF\xFF\x81\x16\x15a\r\x92W_a\x0C\x9A\x87\x87a>\xBDV[\x90P_a\x0C\xADc\xFF\xFF\xFF\xFF\x84\x16\x87aO<V[a\x0C\xB7\x90\x83aOSV[\x90Pa\x0C\xE0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x87a>\x94V[a\r\x13`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x84c\xFF\xFF\xFF\xFF\x16a>\x94V[a\rH`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x83a>\x94V[a\ry`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82a>\x94V[a\r\x84\x88\x88\x83a>\rV[a\r\x8F\x88\x88\x87a=\x89V[PP[_a\r\x9D\x87\x87a>\xD6V[\x90P_a\r\xAA\x82\x85aO V[\x90P_a\r\xB6\x89a>\xEFV[\x90Pa\r\xE6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x14\x11T\x92S\xD1`\xD2\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0E+`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x84c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0Ef`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x83c\xFF\xFF\xFF\xFF\x16a>\x94V[\x80c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0FJW_a\x0E\x85\x8A\x8Aa>\xBDV[\x90P_a\x0E\x92\x8B\x8Ba?\xA1V[\x90P_c\xFF\xFF\xFF\xFF\x85\x16a\x0E\xA6\x83\x85aOfV[a\x0E\xB0\x91\x90aOyV[\x90P\x87\x95P\x82\x91Pa\x0E\xF7`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FpriceCumulativeLastByPeriod\0\0\0\0\0\x81RP\x83a>\x94V[a\x0F%`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a>\x94V[a\x0F0\x8C\x8C\x88a>LV[a\x0F;\x8C\x8C\x84a>dV[a\x0FF\x8C\x8C\x83a>|V[PPP[PPPPPPPPPV[\x81\x15\x80a\x0F`WP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x8C\x86\x85\x85`\x01a?\xBAV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEF\x91\x90aMxV[\x90P\x80_\x03a\x10UW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10>W__\xFD[PZ\xF1\x15\x80\x15a\x10PW=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x10\xAFW=__>=_\xFD[PPPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x10\xDA\x84\x84a@\xF3V[\x91P\x91P\x81\x81\x10a\x10\xFBW``\x83\x01Qa\x10\xF5\x90`\x01aATV[``\x84\x01R[PPPPV[a\x11\n\x81aA~V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x11e\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xB5W__\xFD[PZ\xF1\x15\x80\x15a\x11\xC7W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x12\x0B\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x13\n\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13:\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBB\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x02\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x142\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB2\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\xFE\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15.\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xAF\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\xCF\x90aO\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x80\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xCD\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17~\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xC7\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18x\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x18\xB9\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19s\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xBB\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xB5\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bh\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xB4\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ch\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\x88\x90aO\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D<\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x89\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E=\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x86\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F:\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1Fx\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xF2\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 2\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a \x84\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xB4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra \xFF\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!?\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\x86\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"7\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x89\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C<\x91\x90aMxV[`@Qc\x9E\xD4\x86\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90c\x9E\xD4\x86\xEB\x90`\xC4\x01a\x10\x86V[\x80_\x03a#\xAEW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1E\x91\x90aMxV[\x90P\x80\x83\x11\x15a\x0C<W`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a$WaK\xEEV[a$`\x88aC\x8DV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xCE\x91\x90aMxV[\x81` \x01\x81\x81RPPa$\xE8\x87__\x84a\x01@\x01Qa:\xDDV[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa%\n\x90\x88\x90`\x01\x90_\x90a:\xDDV[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa%+\x91\x88\x91a;\xF4V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa%F\x91\x88\x91a;\xF4V[\x81a\x01 \x01\x81\x81RPPa%\x86`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa>\x94V[a%\xBE`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa>\x94V[a%\xF6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa>\x94V[a&)`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa>\x94V[a&\\`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa>\x94V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a&\xAFW`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x10\xFBW`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'GW__\xFD[PZ\xF1\x15\x80\x15a\x0FFW=__>=_\xFD[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a'zW\x81\x83a'}V[\x82\x82[`@Q\x91\x94P\x92Pa'\xAA\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a(\taK\xBAV[\x82a(\x12aK\xBAV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a(N\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC6\x91\x90aM\x8FV[a(\xD3W\x91Pa'\xFB\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a)\x13\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)w\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x92W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB6\x91\x90aO\xA0V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xA7\x91\x90aMxV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a*\xFD\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+a\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA0\x91\x90aMxV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a+\xFB\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,_\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9E\x91\x90aMxV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a,\xC7\x90` \x01aO\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-j\x91\x90aMxV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-\xC6\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.*\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.i\x91\x90aMxV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/Y\x91\x90aMxV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0A\x91\x90aO\xA0V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1*\x91\x90aMxV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a1\x81\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2$\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\x80\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xB0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xE4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3#\x91\x90aMxV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3S\x90aO\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xF6\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4S\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xF6\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5O\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xF2\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a6@\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE3\x91\x90aO\xA0V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7Q\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xF4\x91\x90aO\xA0V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a8W\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xBB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xFA\x91\x90aMxV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a9S\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xF6\x91\x90aMxV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a:CWPPV[\x81QQ`\xA0\x01Q\x15a:\x99W\x81Q_\x90a:l\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaC\xDEV[\x83Q\x90\x91Pa:\x90\x90\x82\x90_[` \x02\x01Q` \x01QaD\x1A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a:\xD5W\x81Q_\x90a:\xB7\x90`\x01a:YV[\x83Q\x90\x91Pa:\xC9\x90\x82\x90`\x01a:yV[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a:\xFAWa:\xFAaO\x8CV[` \x02\x01Q\x90P_a;\x0C\x8A\x8AaD[V[\x90P\x80_\x03a;(W____\x95P\x95P\x95P\x95PPPa;\xBBV[_a;7\x83\x8C`\x80\x01QaEIV[\x90P_a;D\x82\x8AaD\x1AV[\x90P_\x89\x15a;iW\x81\x84\x10a;cWa;^\x84\x83aEyV[a;kV[_a;kV[_[\x90P_a;x\x85\x8DaD\x1AV[\x90P_\x8C\x15a;\x9DW\x84\x82\x10a;\x97Wa;\x92\x82\x86aEyV[a;\x9FV[_a;\x9FV[_[\x90Pa;\xAB\x85\x87aOSV[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a;\xE6WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a<(W\x83\x82\x81a<\x1EWa<\x1EaN\xF9V[\x04\x92PPPa<\xADV[\x80\x84\x11a<HW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a<\xD5W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x08*`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aE\xCEV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aF\x13V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xAD\x91\x90aMxV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a=\xA1\x84aF\x13V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFB\x91\x90aMxV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aF\xC6V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a=\xA1\x84aG\x1AV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aG{V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aG\xE1V[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aH*V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aF\xC6V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aG\x1AV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xFB\x91\x90aMxV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aG{V[_a?\xC3aK\xEEV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@'\x91\x90aMxV[` \x82\x01Ra@8\x86_\x80\x80a:\xDDV[PPP`\xC0\x82\x01Ra@M\x86`\x01_\x80a:\xDDV[PPP`\xE0\x82\x01R\x82\x15a@\x88W\x84\x81`\xC0\x01\x81\x81Qa@m\x91\x90aOfV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a@\x84\x90\x83\x90aOfV[\x90RP[\x80` \x01Q_\x03a@\xB8Wa@\xB1a\x03\xE8a@\xABa@\xA6\x88\x88aHqV[aH\xD7V[\x90aEyV[\x81Ra@\xE9V[a@\xE6a@\xCE\x86\x83` \x01Q\x84`\xC0\x01Qa;\xF4V[a@\xE1\x86\x84` \x01Q\x85`\xE0\x01Qa;\xF4V[aI\xB7V[\x81R[Q\x95\x94PPPPPV[___aA\x04\x84``\x01Q_a;\xC6V[\x90P_aA\x10\x86aI\xCCV[\x90P_aA3\x82aA\"\x85`\naN\xEEV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xF4V[\x90P_aAB\x87___a:\xDDV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aAbW_aAeV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aA\xB3`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\xBD\x84_aJ\x1DV[` \x83\x01R\x81R``\x84\x01QaA\xD3\x90_a;\xC6V[``\x82\x01\x81\x90R\x81QaA\xF8\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B\x11\x90`\naN\xEEV[`@\x82\x01R` \x81\x01Q_\x03aB\x13W_`\x80\x82\x01RaB\xB3V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xAD\x91\x90aMxV[`\x80\x82\x01R[aB\xBE\x84`\x01aJ\x1DV[` \x83\x01\x81\x90R\x90\x82R_\x03aB\xD9W_`\xA0\x82\x01RaCyV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aCs\x91\x90aMxV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aC\xEA\x83BaOfV[aC\xF4\x90\x85aO<V[c\x01\xE13\x80\x90\x04\x90PaD\x12\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaOSV[\x94\x93PPPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aD:W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aDuWaDuaO\x8CV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xF2\x91\x90aMxV[\x90P\x80_\x03aE\x05W_\x92PPPa'\xFBV[``\x82\x01Q`\xC0\x83\x01QaE\x19\x90\x82aOSV[\x82\x10aE=W`\xC0\x83\x01QaE.\x82\x84aOfV[aE8\x91\x90aOfV[aE?V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aE\\WP_a'\xFBV[_aEg\x84\x84aJcV[`\xA0\x85\x01Q\x90\x91PaD\x12\x90\x82aD\x1AV[_\x82aE\x85\x83\x82aOfV[\x91P\x81\x11\x15a'\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aE\xE4\x92\x91\x90aP=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaJ\xA6V[\x80Q_\x90\x81\x90aF4\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa'YV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aF\xD6\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aG*\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aG\x8B\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aG\xF1\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nT\x83\x83\x83`@Q`$\x01aHB\x93\x92\x91\x90aPjV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaJ\xA6V[_\x81\x15\x80aH\x94WP\x82\x82aH\x86\x81\x83aO<V[\x92PaH\x92\x90\x83aOyV[\x14[a'\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aH\xE6WP_\x91\x90PV[_`\x01aH\xF2\x84aJ\xAFV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aI\x0BWaI\x0BaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI#WaI#aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI;WaI;aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aISWaISaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aIkWaIkaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\x83WaI\x83aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\x9BWaI\x9BaN\xF9V[\x04\x82\x01\x90\x1C\x90Pa<\xAD\x81\x82\x85\x81aI\xB5WaI\xB5aN\xF9V[\x04[_\x81\x83\x10aI\xC5W\x81a<\xADV[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aJJ\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aJ;WaJ;aO\x8CV[` \x02\x01Q\x86`\x80\x01QaEIV[\x90P_aJW\x86\x86aD[V[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aJvWP` \x82\x01Qa'\xFBV[_aJ\x85\x84`@\x01Q\x84aC\xDEV[\x90PaJ\x9E\x84` \x01Q\x82aD\x1A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa'\xFBV[a\x08*\x81aKBV[_\x80`\x80\x83\x90\x1C\x15aJ\xC3W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aJ\xD5W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aJ\xE7W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aJ\xF9W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aK\x0BW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aK\x1DW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aK/W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a'\xFBW`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aKuaK\xBAV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aK\xCDaL=V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aL\x8E`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aLLW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08*W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aL\xCAW__\xFD[\x835aL\xD5\x81aL\xA4V[\x92P```\x1F\x19\x82\x01\x12\x15aL\xE8W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aM\x08W__\xFD[\x835aM\x13\x81aL\xA4V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aL\xE8W__\xFD[_` \x82\x84\x03\x12\x15aM6W__\xFD[\x815a<\xAD\x81aL\xA4V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aM\x88W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aM\x9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a<\xADW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aD\x12\x90\x83\x01\x84aM\xAEV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x94W\x80\x85\x04\x81\x11\x15aN2WaN2aM\xFFV[`\x01\x84\x16\x15aN@W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aN\x17V[_\x82aN\\WP`\x01a'\xFBV[\x81aNhWP_a'\xFBV[\x81`\x01\x81\x14aN~W`\x02\x81\x14aN\x88WaN\xA4V[`\x01\x91PPa'\xFBV[`\xFF\x84\x11\x15aN\x99WaN\x99aM\xFFV[PP`\x01\x82\x1Ba'\xFBV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aN\xC7WP\x81\x81\na'\xFBV[aN\xD3_\x19\x84\x84aN\x13V[\x80_\x19\x04\x82\x11\x15aN\xE6WaN\xE6aM\xFFV[\x02\x93\x92PPPV[_a<\xAD\x83\x83aNNV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aO\x1BWaO\x1BaN\xF9V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a'\xFBWa'\xFBaM\xFFV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a'\xFBWa'\xFBaM\xFFV[\x80\x82\x01\x80\x82\x11\x15a'\xFBWa'\xFBaM\xFFV[\x81\x81\x03\x81\x81\x11\x15a'\xFBWa'\xFBaM\xFFV[_\x82aO\x87WaO\x87aN\xF9V[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\xB0W__\xFD[\x81Qa<\xAD\x81aL\xA4V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aPO`@\x83\x01\x85aM\xAEV[\x82\x81\x03` \x84\x01RaPa\x81\x85aM\xAEV[\x95\x94PPPPPV[``\x81R_aP|``\x83\x01\x86aM\xAEV[\x82\x81\x03` \x84\x01RaP\x8E\x81\x86aM\xAEV[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 /90{\x89\xE0\xF8\xC6d)xW\xC7\xC6\x1EC\xFF\x1E$\x16J1\xD8\xB3y3\xEFt\x83\xDB\xC9\xA7dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614cb8565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614cf6565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614d26565b6001600160a01b031681526020018360200160208101906102269190614d26565b6001600160a01b031681526020016102446060850160408601614d26565b6001600160a01b03169052905061025b83826105e8565b50610264610774565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614d26565b6001600160a01b0316815260200183602001602081019061033e9190614d26565b6001600160a01b031681526040808501356020830152016103656080850160608601614d26565b6001600160a01b03169052905061025b838261082d565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614d41565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614d78565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614d41565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614d78565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614d8f565b61026457338160405163a35b150b60e01b815260040161047a929190614ddc565b5f5f610600835f015184604001518560600151610a59565b915091505f61060e83610a9c565b905061061e845f01518483610b4c565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614d78565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614d78565b90506107138282610f55565b610724868484848b60800151610f7e565b865161073090876110bc565b61073e876020015187611101565b865161074b908688611125565b61076a87602001518989604001518a606001518b60800151878761233a565b5050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b290614d41565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610806573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082a9190614d78565b50565b610835614b61565b61084b825f015183604001518460600151610a59565b6020830152808252608083015161086391859161238e565b81518151608084015161087792919061244b565b60c0850181905260a08501829052610120850183905261010085018490526108a193929190612684565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108f9575f5ffd5b505af115801561090b573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610972575f5ffd5b505af1158015610984573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109ec575f5ffd5b505af11580156109fe573d5f5f3e3d5ffd5b50505060208301518251610a129250611101565b815160208201518251610a26929190611125565b610a548260200151848460400151856060015186608001518760a001518761010001518861012001516126da565b505050565b610a61614bba565b5f5f610a6d8585612759565b90505f610a7a8783612801565b9050610a868183613a03565b610a8f81613a31565b925090505b935093915050565b5f5f610aaa835f5f5f613add565b50505090505f610abd8460015f5f613add565b5050509050805f03610ad257505f9392505050565b5f610ae185606001515f613bc6565b90505f610af386606001516001613bc6565b90505f610b1685676765c793fa10079d601b1b610b1186600a614eee565b613bf4565b90505f610b3485676765c793fa10079d601b1b610b1186600a614eee565b9050610b408282613cb4565b98975050505050505050565b610b7c6040518060400160405280600f81526020016e75706461746554776170507269636560881b815250613cef565b5f610b878484613d13565b90505f610b9964010000000042614f0d565b90508163ffffffff165f03610c4357610bb3858583613d89565b610bbe85855f613e0d565b610bc9858583613e4c565b610bd485855f613e64565b610bdf858585613e7c565b610c156040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b8152508263ffffffff16613e94565b610c3c60405180604001604052806005815260200164707269636560d81b81525084613e94565b5050505050565b5f610c4e8383614f20565b9050610c836040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b8152508263ffffffff16613e94565b63ffffffff811615610d92575f610c9a8787613ebd565b90505f610cad63ffffffff841687614f3c565b610cb79083614f53565b9050610ce060405180604001604052806005815260200164707269636560d81b81525087613e94565b610d136040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b8152508463ffffffff16613e94565b610d48604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b81525083613e94565b610d796040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082613e94565b610d84888883613e0d565b610d8f888887613d89565b50505b5f610d9d8787613ed6565b90505f610daa8285614f20565b90505f610db689613eef565b9050610de6604051806040016040528060068152602001651411549253d160d21b8152508263ffffffff16613e94565b610e2b6040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508463ffffffff16613e94565b610e66604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508363ffffffff16613e94565b8063ffffffff168263ffffffff161115610f4a575f610e858a8a613ebd565b90505f610e928b8b613fa1565b90505f63ffffffff8516610ea68385614f66565b610eb09190614f79565b9050879550829150610ef76040518060400160405280601b81526020017f707269636543756d756c61746976654c6173744279506572696f64000000000081525083613e94565b610f256040518060400160405280600c81526020016b70726963654176657261676560a01b81525082613e94565b610f308c8c88613e4c565b610f3b8c8c84613e64565b610f468c8c83613e7c565b5050505b505050505050505050565b811580610f60575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f610f8c8685856001613fba565b90505f856001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610fcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fef9190614d78565b9050805f03611055576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038716906340c10f19906044015f604051808303815f87803b15801561103e575f5ffd5b505af1158015611050573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038481166004830152602482018490528716906340c10f19906044015b5f604051808303815f87803b15801561109d575f5ffd5b505af11580156110af573d5f5f3e3d5ffd5b5050505050505050505050565b6060810151660800000000000016610264575f5f6110da84846140f3565b915091508181106110fb5760608301516110f5906001614154565b60608401525b50505050565b61110a8161417e565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611165906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156111b5575f5ffd5b505af11580156111c7573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161120b906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161123b929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561129e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c29190614fa0565b50806001600160a01b031663e2a4853a8460405160200161130a906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161133a929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611397573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113bb9190614d78565b50806001600160a01b031663e2a4853a84604051602001611402906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611432929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561148e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114b29190614d78565b50806001600160a01b031663e2a4853a846040516020016114fe906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161152e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561158b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115af9190614d78565b50806001600160a01b031663e2a4853a846040516020016115cf90614fbb565b604051602081830303815290604052805190602001206040516020016115ff929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561165c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116809190614d78565b50806001600160a01b031663e2a4853a846040516020016116cd906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561175a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061177e9190614d78565b50806001600160a01b031663e2a4853a846040516020016117c7906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016117f7929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611854573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118789190614d78565b50806001600160a01b031663ca446dd9846040516020016118b9906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016118e9929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561194f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119739190614fa0565b50806001600160a01b031663e2a4853a846040516020016119bb90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016119eb929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e9190614d78565b50806001600160a01b031663e2a4853a84604051602001611ab590602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ae5929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b689190614d78565b50806001600160a01b031663e2a4853a84604051602001611bb490602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611be4929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c689190614d78565b50806001600160a01b031663e2a4853a84604051602001611c8890614ffc565b60405160208183030381529060405280519060200120604051602001611cb8929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d3c9190614d78565b50806001600160a01b031663e2a4853a84604051602001611d8990602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611db9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e19573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e3d9190614d78565b50806001600160a01b031663e2a4853a84604051602001611e8690602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611eb6929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f16573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f3a9190614d78565b50806001600160a01b031663ca446dd984604051602001611f7890602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fa8929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611ff29291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561200e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120329190614fa0565b50806001600160a01b031663ca446dd984604051602001612084906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016120b4929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b1683526120ff926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561211b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061213f9190614fa0565b50806001600160a01b031663e2a4853a84604051602001612186906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016121b6929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b81526004016121f7929190918252602082015260400190565b6020604051808303815f875af1158015612213573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122379190614d78565b50806001600160a01b031663e2a4853a84604051602001612289906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016122b9929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b81526004016122fa929190918252602082015260400190565b6020604051808303815f875af1158015612316573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c3c9190614d78565b604051639ed486eb60e01b81526001600160a01b0387811660048301528681166024830152858116604483015284811660648301526084820184905260a48201839052881690639ed486eb9060c401611086565b805f036123ae57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa1580156123fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241e9190614d78565b905080831115610c3c576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f612457614bee565b6124608861438d565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156124aa573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124ce9190614d78565b8160200181815250506124e8875f5f846101400151613add565b50608084015250604082015261014081015161250a9088906001905f90613add565b5060a08401525060608201526040810151602082015161252b918891613bf4565b61010082015260608101516020820151612546918891613bf4565b816101200181815250506125866040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613e94565b6125be604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613e94565b6125f660405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613e94565b6126296040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613e94565b61265c6040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613e94565b80610100015181610120015182608001518360a0015194509450945094505093509350935093565b818411156126af57604051631fc107c160e01b8152600481018590526024810183905260440161047a565b808311156110fb57604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4015f604051808303815f87803b158015612747575f5ffd5b505af1158015610f46573d5f5f3e3d5ffd5b5f816001600160a01b0316836001600160a01b03161061277a57818361277d565b82825b60405191945092506127aa906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612809614bba565b82612812614bba565b816001600160a01b03166391d4403c60405160200161284e906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156128a2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128c69190614d8f565b6128d35791506127fb9050565b816001600160a01b03166321f8a72185604051602001612913906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612943929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161297791815260200190565b602060405180830381865afa158015612992573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b69190614fa0565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612a34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612a6891815260200190565b602060405180830381865afa158015612a83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aa79190614d78565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612afd906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b2d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612b6191815260200190565b602060405180830381865afa158015612b7c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ba09190614d78565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612bfb906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c2b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612c5f91815260200190565b602060405180830381865afa158015612c7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9e9190614d78565b815151606001526040516001600160a01b0383169063bd02d0f5908690612cc790602001614fbb565b60405160208183030381529060405280519060200120604051602001612cf7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612d2b91815260200190565b602060405180830381865afa158015612d46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6a9190614d78565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612dc6906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612df6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e2a91815260200190565b602060405180830381865afa158015612e45573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e699190614d78565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612ee6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f1a91815260200190565b602060405180830381865afa158015612f35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f599190614d78565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001612fce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161300291815260200190565b602060405180830381865afa15801561301d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130419190614fa0565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016130eb91815260200190565b602060405180830381865afa158015613106573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061312a9190614d78565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161318190602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016131b1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016131e591815260200190565b602060405180830381865afa158015613200573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132249190614d78565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161328090602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016132b0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016132e491815260200190565b602060405180830381865afa1580156132ff573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133239190614d78565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161335390614ffc565b60405160208183030381529060405280519060200120604051602001613383929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133b791815260200190565b602060405180830381865afa1580156133d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133f69190614d78565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161345390602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613483929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134b791815260200190565b602060405180830381865afa1580156134d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134f69190614d78565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161354f90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161357f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135b391815260200190565b602060405180830381865afa1580156135ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135f29190614d78565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161364090602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613670929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136a491815260200190565b602060405180830381865afa1580156136bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e39190614fa0565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613751906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613781929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137b591815260200190565b602060405180830381865afa1580156137d0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137f49190614fa0565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001613857906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613887929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138bb91815260200190565b602060405180830381865afa1580156138d6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138fa9190614d78565b60608201526040516001600160a01b0383169063bd02d0f5908690613953906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613983929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016139b791815260200190565b602060405180830381865afa1580156139d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139f69190614d78565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613a43575050565b81515160a0015115613a995781515f90613a6c90825b60200201516040015184608001516143de565b8351909150613a909082905f5b60200201516020015161441a90919063ffffffff16565b83515160200152505b81516020015160a0015115613ad55781515f90613ab7906001613a59565b8351909150613ac99082906001613a79565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613afa57613afa614f8c565b602002015190505f613b0c8a8a61445b565b9050805f03613b28575f5f5f5f95509550955095505050613bbb565b5f613b37838c60800151614549565b90505f613b44828a61441a565b90505f8915613b6957818410613b6357613b5e8483614579565b613b6b565b5f613b6b565b5f5b90505f613b78858d61441a565b90505f8c15613b9d57848210613b9757613b928286614579565b613b9f565b5f613b9f565b5f5b9050613bab8587614f53565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613be6575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613c2857838281613c1e57613c1e614ef9565b0492505050613cad565b808411613c485760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613cd5575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61082a60405180604001604052806002815260200161257360f01b815250826145ce565b5f826001600160a01b031663bd02d0f5613d2c84614613565b6040518263ffffffff1660e01b8152600401613d4a91815260200190565b602060405180830381865afa158015613d65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cad9190614d78565b826001600160a01b031663e2a4853a613da184614613565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613de9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110fb9190614d78565b826001600160a01b031663e2a4853a613e25846146c6565b836040518363ffffffff1660e01b8152600401613dcd929190918252602082015260400190565b826001600160a01b031663e2a4853a613da18461471a565b826001600160a01b031663e2a4853a613e258461477b565b826001600160a01b031663e2a4853a613e25846147e1565b610264604051806040016040528060068152602001652573202d257360d01b815250838361482a565b5f826001600160a01b031663bd02d0f5613d2c846146c6565b5f826001600160a01b031663bd02d0f5613d2c8461471a565b5f816001600160a01b031663bd02d0f5604051602001613f2e906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f6291815260200190565b602060405180830381865afa158015613f7d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127fb9190614d78565b5f826001600160a01b031663bd02d0f5613d2c8461477b565b5f613fc3614bee565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015614003573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140279190614d78565b6020820152614038865f8080613add565b50505060c082015261404d8660015f80613add565b50505060e0820152821561408857848160c00181815161406d9190614f66565b90525060e081018051859190614084908390614f66565b9052505b80602001515f036140b8576140b16103e86140ab6140a68888614871565b6148d7565b90614579565b81526140e9565b6140e66140ce8683602001518460c00151613bf4565b6140e18684602001518560e00151613bf4565b6149b7565b81525b5195945050505050565b5f5f5f61410484606001515f613bc6565b90505f614110866149cc565b90505f6141338261412285600a614eee565b676765c793fa10079d601b1b613bf4565b90505f614142875f5f5f613add565b50939a91995090975050505050505050565b5f603382614162575f614165565b60015b60ff16901b660800000000000019841617905092915050565b5f5f6141b36040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6141bd845f614a1d565b6020830152815260608401516141d3905f613bc6565b6060820181905281516141f891676765c793fa10079d601b1b90610b1190600a614eee565b604082015260208101515f03614213575f60808201526142b3565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614289573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142ad9190614d78565b60808201525b6142be846001614a1d565b602083018190529082525f036142d9575f60a0820152614379565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561434f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143739190614d78565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001613f2e906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f806143ea8342614f66565b6143f49085614f3c565b6301e133809004905061441281676765c793fa10079d601b1b614f53565b949350505050565b5f81156b019d971e4fe8401e74000000198390048411151761443a575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff166002811061447557614475614f8c565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156144ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144f29190614d78565b9050805f03614505575f925050506127fb565b606082015160c08301516145199082614f53565b821061453d5760c083015161452e8284614f66565b6145389190614f66565b61453f565b5f5b9695505050505050565b5f8260a001515f0361455c57505f6127fb565b5f6145678484614a63565b60a0850151909150614412908261441a565b5f826145858382614f66565b91508111156127fb5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b61026482826040516024016145e492919061503d565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614aa6565b80515f90819061463490825b60200201515184516001602002015151612759565b905080604051602001614678906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016146a8929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906146d6908261461f565b905080604051602001614678906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061472a908261461f565b9050806040516020016146789060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f90819061478b908261461f565b9050806040516020016146789060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906147f1908261461f565b90508060405160200161467890602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a548383836040516024016148429392919061506a565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614aa6565b5f811580614894575082826148868183614f3c565b92506148929083614f79565b145b6127fb5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f036148e657505f919050565b5f60016148f284614aaf565b901c6001901b9050600181848161490b5761490b614ef9565b048201901c9050600181848161492357614923614ef9565b048201901c9050600181848161493b5761493b614ef9565b048201901c9050600181848161495357614953614ef9565b048201901c9050600181848161496b5761496b614ef9565b048201901c9050600181848161498357614983614ef9565b048201901c9050600181848161499b5761499b614ef9565b048201901c9050613cad818285816149b5576149b5614ef9565b045b5f8183106149c55781613cad565b5090919050565b5f816001600160a01b031663bd02d0f5604051602001613f2e9060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614a4a855f01518560ff1660028110614a3b57614a3b614f8c565b60200201518660800151614549565b90505f614a57868661445b565b96919550909350505050565b5f428203614a76575060208201516127fb565b5f614a858460400151846143de565b9050614a9e84602001518261441a90919063ffffffff16565b9150506127fb565b61082a81614b42565b5f80608083901c15614ac357608092831c92015b604083901c15614ad557604092831c92015b602083901c15614ae757602092831c92015b601083901c15614af957601092831c92015b600883901c15614b0b57600892831c92015b600483901c15614b1d57600492831c92015b600283901c15614b2f57600292831c92015b600183901c156127fb5760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614b75614bba565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614bcd614c3d565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614c8e6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614c4c5790505090565b6001600160a01b038116811461082a575f5ffd5b5f5f8284036080811215614cca575f5ffd5b8335614cd581614ca4565b92506060601f1982011215614ce8575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614d08575f5ffd5b8335614d1381614ca4565b92506080601f1982011215614ce8575f5ffd5b5f60208284031215614d36575f5ffd5b8135613cad81614ca4565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614d88575f5ffd5b5051919050565b5f60208284031215614d9f575f5ffd5b81518015158114613cad575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061441290830184614dae565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9457808504811115614e3257614e32614dff565b6001841615614e4057908102905b60019390931c928002614e17565b5f82614e5c575060016127fb565b81614e6857505f6127fb565b8160018114614e7e5760028114614e8857614ea4565b60019150506127fb565b60ff841115614e9957614e99614dff565b50506001821b6127fb565b5060208310610133831016604e8410600b8410161715614ec7575081810a6127fb565b614ed35f198484614e13565b805f1904821115614ee657614ee6614dff565b029392505050565b5f613cad8383614e4e565b634e487b7160e01b5f52601260045260245ffd5b5f82614f1b57614f1b614ef9565b500690565b63ffffffff82811682821603908111156127fb576127fb614dff565b80820281158282048414176127fb576127fb614dff565b808201808211156127fb576127fb614dff565b818103818111156127fb576127fb614dff565b5f82614f8757614f87614ef9565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614fb0575f5ffd5b8151613cad81614ca4565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61504f6040830185614dae565b82810360208401526150618185614dae565b95945050505050565b606081525f61507c6060830186614dae565b828103602084015261508e8186614dae565b91505082604083015294935050505056fea26469706673582212202f39307b89e0f8c664297857c7c61e43ff1e24164a31d8b37933ef7483dbc9a764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aL\xB8V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aL\xF6V[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07tV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aM&V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x08-V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aMAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aMxV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aMAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aMxV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aM\x8FV[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aM\xDCV[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\nYV[\x91P\x91P_a\x06\x0E\x83a\n\x9CV[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0BLV[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aMxV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aMxV[\x90Pa\x07\x13\x82\x82a\x0FUV[a\x07$\x86\x84\x84\x84\x8B`\x80\x01Qa\x0F~V[\x86Qa\x070\x90\x87a\x10\xBCV[a\x07>\x87` \x01Q\x87a\x11\x01V[\x86Qa\x07K\x90\x86\x88a\x11%V[a\x07j\x87` \x01Q\x89\x89`@\x01Q\x8A``\x01Q\x8B`\x80\x01Q\x87\x87a#:V[PPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB2\x90aMAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08*\x91\x90aMxV[PV[a\x085aKaV[a\x08K\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\nYV[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08c\x91\x85\x91a#\x8EV[\x81Q\x81Q`\x80\x84\x01Qa\x08w\x92\x91\x90a$KV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA1\x93\x92\x91\x90a&\x84V[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xF9W__\xFD[PZ\xF1\x15\x80\x15a\t\x0BW=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\trW__\xFD[PZ\xF1\x15\x80\x15a\t\x84W=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xECW__\xFD[PZ\xF1\x15\x80\x15a\t\xFEW=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x12\x92Pa\x11\x01V[\x81Q` \x82\x01Q\x82Qa\n&\x92\x91\x90a\x11%V[a\nT\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa&\xDAV[PPPV[a\naaK\xBAV[__a\nm\x85\x85a'YV[\x90P_a\nz\x87\x83a(\x01V[\x90Pa\n\x86\x81\x83a:\x03V[a\n\x8F\x81a:1V[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xAA\x83___a:\xDDV[PPP\x90P_a\n\xBD\x84`\x01__a:\xDDV[PPP\x90P\x80_\x03a\n\xD2WP_\x93\x92PPPV[_a\n\xE1\x85``\x01Q_a;\xC6V[\x90P_a\n\xF3\x86``\x01Q`\x01a;\xC6V[\x90P_a\x0B\x16\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x11\x86`\naN\xEEV[a;\xF4V[\x90P_a\x0B4\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B\x11\x86`\naN\xEEV[\x90Pa\x0B@\x82\x82a<\xB4V[\x98\x97PPPPPPPPV[a\x0B|`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nupdateTwapPrice`\x88\x1B\x81RPa<\xEFV[_a\x0B\x87\x84\x84a=\x13V[\x90P_a\x0B\x99d\x01\0\0\0\0BaO\rV[\x90P\x81c\xFF\xFF\xFF\xFF\x16_\x03a\x0CCWa\x0B\xB3\x85\x85\x83a=\x89V[a\x0B\xBE\x85\x85_a>\rV[a\x0B\xC9\x85\x85\x83a>LV[a\x0B\xD4\x85\x85_a>dV[a\x0B\xDF\x85\x85\x85a>|V[a\x0C\x15`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0C<`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x84a>\x94V[PPPPPV[_a\x0CN\x83\x83aO V[\x90Pa\x0C\x83`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[c\xFF\xFF\xFF\xFF\x81\x16\x15a\r\x92W_a\x0C\x9A\x87\x87a>\xBDV[\x90P_a\x0C\xADc\xFF\xFF\xFF\xFF\x84\x16\x87aO<V[a\x0C\xB7\x90\x83aOSV[\x90Pa\x0C\xE0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x87a>\x94V[a\r\x13`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x84c\xFF\xFF\xFF\xFF\x16a>\x94V[a\rH`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x83a>\x94V[a\ry`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82a>\x94V[a\r\x84\x88\x88\x83a>\rV[a\r\x8F\x88\x88\x87a=\x89V[PP[_a\r\x9D\x87\x87a>\xD6V[\x90P_a\r\xAA\x82\x85aO V[\x90P_a\r\xB6\x89a>\xEFV[\x90Pa\r\xE6`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x14\x11T\x92S\xD1`\xD2\x1B\x81RP\x82c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0E+`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x84c\xFF\xFF\xFF\xFF\x16a>\x94V[a\x0Ef`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x83c\xFF\xFF\xFF\xFF\x16a>\x94V[\x80c\xFF\xFF\xFF\xFF\x16\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0FJW_a\x0E\x85\x8A\x8Aa>\xBDV[\x90P_a\x0E\x92\x8B\x8Ba?\xA1V[\x90P_c\xFF\xFF\xFF\xFF\x85\x16a\x0E\xA6\x83\x85aOfV[a\x0E\xB0\x91\x90aOyV[\x90P\x87\x95P\x82\x91Pa\x0E\xF7`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FpriceCumulativeLastByPeriod\0\0\0\0\0\x81RP\x83a>\x94V[a\x0F%`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a>\x94V[a\x0F0\x8C\x8C\x88a>LV[a\x0F;\x8C\x8C\x84a>dV[a\x0FF\x8C\x8C\x83a>|V[PPP[PPPPPPPPPV[\x81\x15\x80a\x0F`WP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x8C\x86\x85\x85`\x01a?\xBAV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEF\x91\x90aMxV[\x90P\x80_\x03a\x10UW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10>W__\xFD[PZ\xF1\x15\x80\x15a\x10PW=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x87\x16\x90c@\xC1\x0F\x19\x90`D\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x10\xAFW=__>=_\xFD[PPPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x10\xDA\x84\x84a@\xF3V[\x91P\x91P\x81\x81\x10a\x10\xFBW``\x83\x01Qa\x10\xF5\x90`\x01aATV[``\x84\x01R[PPPPV[a\x11\n\x81aA~V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x11e\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xB5W__\xFD[PZ\xF1\x15\x80\x15a\x11\xC7W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x12\x0B\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x13\n\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13:\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBB\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x02\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x142\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xB2\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\xFE\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15.\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xAF\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\xCF\x90aO\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x80\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xCD\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17~\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xC7\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18x\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x18\xB9\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19s\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xBB\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xB5\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bh\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xB4\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ch\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\x88\x90aO\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D<\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x89\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E=\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x86\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F:\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1Fx\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xF2\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a 2\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a \x84\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xB4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra \xFF\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!?\x91\x90aO\xA0V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\x86\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"7\x91\x90aMxV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x89\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C<\x91\x90aMxV[`@Qc\x9E\xD4\x86\xEB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90c\x9E\xD4\x86\xEB\x90`\xC4\x01a\x10\x86V[\x80_\x03a#\xAEW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1E\x91\x90aMxV[\x90P\x80\x83\x11\x15a\x0C<W`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a$WaK\xEEV[a$`\x88aC\x8DV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xCE\x91\x90aMxV[\x81` \x01\x81\x81RPPa$\xE8\x87__\x84a\x01@\x01Qa:\xDDV[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa%\n\x90\x88\x90`\x01\x90_\x90a:\xDDV[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa%+\x91\x88\x91a;\xF4V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa%F\x91\x88\x91a;\xF4V[\x81a\x01 \x01\x81\x81RPPa%\x86`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa>\x94V[a%\xBE`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa>\x94V[a%\xF6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa>\x94V[a&)`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa>\x94V[a&\\`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa>\x94V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a&\xAFW`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x10\xFBW`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'GW__\xFD[PZ\xF1\x15\x80\x15a\x0FFW=__>=_\xFD[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a'zW\x81\x83a'}V[\x82\x82[`@Q\x91\x94P\x92Pa'\xAA\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a(\taK\xBAV[\x82a(\x12aK\xBAV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a(N\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC6\x91\x90aM\x8FV[a(\xD3W\x91Pa'\xFB\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a)\x13\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)w\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x92W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB6\x91\x90aO\xA0V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xA7\x91\x90aMxV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a*\xFD\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+a\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA0\x91\x90aMxV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a+\xFB\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,_\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9E\x91\x90aMxV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a,\xC7\x90` \x01aO\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-j\x91\x90aMxV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-\xC6\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.*\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.i\x91\x90aMxV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/Y\x91\x90aMxV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0A\x91\x90aO\xA0V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1*\x91\x90aMxV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a1\x81\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2$\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\x80\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xB0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xE4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3#\x91\x90aMxV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3S\x90aO\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xF6\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4S\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xF6\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5O\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xF2\x91\x90aMxV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a6@\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE3\x91\x90aO\xA0V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7Q\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xF4\x91\x90aO\xA0V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a8W\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xBB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xFA\x91\x90aMxV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a9S\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xF6\x91\x90aMxV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a:CWPPV[\x81QQ`\xA0\x01Q\x15a:\x99W\x81Q_\x90a:l\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaC\xDEV[\x83Q\x90\x91Pa:\x90\x90\x82\x90_[` \x02\x01Q` \x01QaD\x1A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a:\xD5W\x81Q_\x90a:\xB7\x90`\x01a:YV[\x83Q\x90\x91Pa:\xC9\x90\x82\x90`\x01a:yV[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a:\xFAWa:\xFAaO\x8CV[` \x02\x01Q\x90P_a;\x0C\x8A\x8AaD[V[\x90P\x80_\x03a;(W____\x95P\x95P\x95P\x95PPPa;\xBBV[_a;7\x83\x8C`\x80\x01QaEIV[\x90P_a;D\x82\x8AaD\x1AV[\x90P_\x89\x15a;iW\x81\x84\x10a;cWa;^\x84\x83aEyV[a;kV[_a;kV[_[\x90P_a;x\x85\x8DaD\x1AV[\x90P_\x8C\x15a;\x9DW\x84\x82\x10a;\x97Wa;\x92\x82\x86aEyV[a;\x9FV[_a;\x9FV[_[\x90Pa;\xAB\x85\x87aOSV[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a;\xE6WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a<(W\x83\x82\x81a<\x1EWa<\x1EaN\xF9V[\x04\x92PPPa<\xADV[\x80\x84\x11a<HW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a<\xD5W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x08*`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aE\xCEV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aF\x13V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xAD\x91\x90aMxV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a=\xA1\x84aF\x13V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\xE9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFB\x91\x90aMxV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aF\xC6V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a=\xA1\x84aG\x1AV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aG{V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>%\x84aG\xE1V[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aH*V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aF\xC6V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aG\x1AV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xFB\x91\x90aMxV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a=,\x84aG{V[_a?\xC3aK\xEEV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@'\x91\x90aMxV[` \x82\x01Ra@8\x86_\x80\x80a:\xDDV[PPP`\xC0\x82\x01Ra@M\x86`\x01_\x80a:\xDDV[PPP`\xE0\x82\x01R\x82\x15a@\x88W\x84\x81`\xC0\x01\x81\x81Qa@m\x91\x90aOfV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a@\x84\x90\x83\x90aOfV[\x90RP[\x80` \x01Q_\x03a@\xB8Wa@\xB1a\x03\xE8a@\xABa@\xA6\x88\x88aHqV[aH\xD7V[\x90aEyV[\x81Ra@\xE9V[a@\xE6a@\xCE\x86\x83` \x01Q\x84`\xC0\x01Qa;\xF4V[a@\xE1\x86\x84` \x01Q\x85`\xE0\x01Qa;\xF4V[aI\xB7V[\x81R[Q\x95\x94PPPPPV[___aA\x04\x84``\x01Q_a;\xC6V[\x90P_aA\x10\x86aI\xCCV[\x90P_aA3\x82aA\"\x85`\naN\xEEV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xF4V[\x90P_aAB\x87___a:\xDDV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aAbW_aAeV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aA\xB3`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\xBD\x84_aJ\x1DV[` \x83\x01R\x81R``\x84\x01QaA\xD3\x90_a;\xC6V[``\x82\x01\x81\x90R\x81QaA\xF8\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B\x11\x90`\naN\xEEV[`@\x82\x01R` \x81\x01Q_\x03aB\x13W_`\x80\x82\x01RaB\xB3V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xAD\x91\x90aMxV[`\x80\x82\x01R[aB\xBE\x84`\x01aJ\x1DV[` \x83\x01\x81\x90R\x90\x82R_\x03aB\xD9W_`\xA0\x82\x01RaCyV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aCs\x91\x90aMxV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aC\xEA\x83BaOfV[aC\xF4\x90\x85aO<V[c\x01\xE13\x80\x90\x04\x90PaD\x12\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaOSV[\x94\x93PPPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aD:W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aDuWaDuaO\x8CV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xF2\x91\x90aMxV[\x90P\x80_\x03aE\x05W_\x92PPPa'\xFBV[``\x82\x01Q`\xC0\x83\x01QaE\x19\x90\x82aOSV[\x82\x10aE=W`\xC0\x83\x01QaE.\x82\x84aOfV[aE8\x91\x90aOfV[aE?V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aE\\WP_a'\xFBV[_aEg\x84\x84aJcV[`\xA0\x85\x01Q\x90\x91PaD\x12\x90\x82aD\x1AV[_\x82aE\x85\x83\x82aOfV[\x91P\x81\x11\x15a'\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aE\xE4\x92\x91\x90aP=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaJ\xA6V[\x80Q_\x90\x81\x90aF4\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa'YV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aF\xD6\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aG*\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aG\x8B\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aG\xF1\x90\x82aF\x1FV[\x90P\x80`@Q` \x01aFx\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nT\x83\x83\x83`@Q`$\x01aHB\x93\x92\x91\x90aPjV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaJ\xA6V[_\x81\x15\x80aH\x94WP\x82\x82aH\x86\x81\x83aO<V[\x92PaH\x92\x90\x83aOyV[\x14[a'\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aH\xE6WP_\x91\x90PV[_`\x01aH\xF2\x84aJ\xAFV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aI\x0BWaI\x0BaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI#WaI#aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI;WaI;aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aISWaISaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aIkWaIkaN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\x83WaI\x83aN\xF9V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\x9BWaI\x9BaN\xF9V[\x04\x82\x01\x90\x1C\x90Pa<\xAD\x81\x82\x85\x81aI\xB5WaI\xB5aN\xF9V[\x04[_\x81\x83\x10aI\xC5W\x81a<\xADV[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a?.\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aJJ\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aJ;WaJ;aO\x8CV[` \x02\x01Q\x86`\x80\x01QaEIV[\x90P_aJW\x86\x86aD[V[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aJvWP` \x82\x01Qa'\xFBV[_aJ\x85\x84`@\x01Q\x84aC\xDEV[\x90PaJ\x9E\x84` \x01Q\x82aD\x1A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa'\xFBV[a\x08*\x81aKBV[_\x80`\x80\x83\x90\x1C\x15aJ\xC3W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aJ\xD5W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aJ\xE7W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aJ\xF9W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aK\x0BW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aK\x1DW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aK/W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a'\xFBW`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aKuaK\xBAV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aK\xCDaL=V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aL\x8E`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aLLW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08*W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aL\xCAW__\xFD[\x835aL\xD5\x81aL\xA4V[\x92P```\x1F\x19\x82\x01\x12\x15aL\xE8W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aM\x08W__\xFD[\x835aM\x13\x81aL\xA4V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aL\xE8W__\xFD[_` \x82\x84\x03\x12\x15aM6W__\xFD[\x815a<\xAD\x81aL\xA4V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aM\x88W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aM\x9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a<\xADW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aD\x12\x90\x83\x01\x84aM\xAEV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x94W\x80\x85\x04\x81\x11\x15aN2WaN2aM\xFFV[`\x01\x84\x16\x15aN@W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aN\x17V[_\x82aN\\WP`\x01a'\xFBV[\x81aNhWP_a'\xFBV[\x81`\x01\x81\x14aN~W`\x02\x81\x14aN\x88WaN\xA4V[`\x01\x91PPa'\xFBV[`\xFF\x84\x11\x15aN\x99WaN\x99aM\xFFV[PP`\x01\x82\x1Ba'\xFBV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aN\xC7WP\x81\x81\na'\xFBV[aN\xD3_\x19\x84\x84aN\x13V[\x80_\x19\x04\x82\x11\x15aN\xE6WaN\xE6aM\xFFV[\x02\x93\x92PPPV[_a<\xAD\x83\x83aNNV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aO\x1BWaO\x1BaN\xF9V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a'\xFBWa'\xFBaM\xFFV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a'\xFBWa'\xFBaM\xFFV[\x80\x82\x01\x80\x82\x11\x15a'\xFBWa'\xFBaM\xFFV[\x81\x81\x03\x81\x81\x11\x15a'\xFBWa'\xFBaM\xFFV[_\x82aO\x87WaO\x87aN\xF9V[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\xB0W__\xFD[\x81Qa<\xAD\x81aL\xA4V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aPO`@\x83\x01\x85aM\xAEV[\x82\x81\x03` \x84\x01RaPa\x81\x85aM\xAEV[\x95\x94PPPPPV[``\x81R_aP|``\x83\x01\x86aM\xAEV[\x82\x81\x03` \x84\x01RaP\x8E\x81\x86aM\xAEV[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 /90{\x89\xE0\xF8\xC6d)xW\xC7\xC6\x1EC\xFF\x1E$\x16J1\xD8\xB3y3\xEFt\x83\xDB\xC9\xA7dsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `EmptyAddAmounts()` and selector `0x6977ca0c`.
```solidity
error EmptyAddAmounts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyAddAmounts {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyAddAmounts> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyAddAmounts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyAddAmounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyAddAmounts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyAddAmounts()";
            const SELECTOR: [u8; 4] = [105u8, 119u8, 202u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `EmptyPool(bytes32)` and selector `0x7357d91f`.
```solidity
error EmptyPool(bytes32 key);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyPool {
        pub key: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyPool> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyPool) -> Self {
                (value.key,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyPool {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { key: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyPool {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyPool(bytes32)";
            const SELECTOR: [u8; 4] = [115u8, 87u8, 217u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.key),
                )
            }
        }
    };
    /**Custom error with signature `EmptyRemoveAmounts()` and selector `0x51865911`.
```solidity
error EmptyRemoveAmounts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyRemoveAmounts {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyRemoveAmounts> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyRemoveAmounts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyRemoveAmounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyRemoveAmounts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyRemoveAmounts()";
            const SELECTOR: [u8; 4] = [81u8, 134u8, 89u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientUserBalance(uint256,uint256)` and selector `0x5f504d20`.
```solidity
error InsufficientUserBalance(uint256 balance, uint256 liquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientUserBalance {
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientUserBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientUserBalance) -> Self {
                (value.balance, value.liquidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientUserBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balance: tuple.0,
                    liquidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientUserBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientUserBalance(uint256,uint256)";
            const SELECTOR: [u8; 4] = [95u8, 80u8, 77u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                )
            }
        }
    };
    /**Custom error with signature `MathOverflowedMulDiv()` and selector `0x227bc153`.
```solidity
error MathOverflowedMulDiv();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MathOverflowedMulDiv {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MathOverflowedMulDiv> for UnderlyingRustTuple<'_> {
            fn from(value: MathOverflowedMulDiv) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MathOverflowedMulDiv {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MathOverflowedMulDiv {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MathOverflowedMulDiv()";
            const SELECTOR: [u8; 4] = [34u8, 123u8, 193u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `Reserve0Insufficient(uint256,uint256)` and selector `0x1fc107c1`.
```solidity
error Reserve0Insufficient(uint256 amount0, uint256 availableReserve0);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reserve0Insufficient {
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub availableReserve0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Reserve0Insufficient> for UnderlyingRustTuple<'_> {
            fn from(value: Reserve0Insufficient) -> Self {
                (value.amount0, value.availableReserve0)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reserve0Insufficient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount0: tuple.0,
                    availableReserve0: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reserve0Insufficient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reserve0Insufficient(uint256,uint256)";
            const SELECTOR: [u8; 4] = [31u8, 193u8, 7u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.availableReserve0),
                )
            }
        }
    };
    /**Custom error with signature `Reserve1Insufficient(uint256,uint256)` and selector `0x0e793baf`.
```solidity
error Reserve1Insufficient(uint256 amount1, uint256 availableReserve1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Reserve1Insufficient {
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
        pub availableReserve1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Reserve1Insufficient> for UnderlyingRustTuple<'_> {
            fn from(value: Reserve1Insufficient) -> Self {
                (value.amount1, value.availableReserve1)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Reserve1Insufficient {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount1: tuple.0,
                    availableReserve1: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Reserve1Insufficient {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Reserve1Insufficient(uint256,uint256)";
            const SELECTOR: [u8; 4] = [14u8, 121u8, 59u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.availableReserve1),
                )
            }
        }
    };
    /**Custom error with signature `Unauthorized(address,string)` and selector `0xa35b150b`.
```solidity
error Unauthorized(address msgSender, string role);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Unauthorized {
        pub msgSender: alloy::sol_types::private::Address,
        pub role: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Unauthorized> for UnderlyingRustTuple<'_> {
            fn from(value: Unauthorized) -> Self {
                (value.msgSender, value.role)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Unauthorized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    msgSender: tuple.0,
                    role: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Unauthorized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Unauthorized(address,string)";
            const SELECTOR: [u8; 4] = [163u8, 91u8, 21u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.msgSender,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.role,
                    ),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _roleStore, address _dataStore, address _eventEmitter);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _roleStore: alloy::sol_types::private::Address,
        pub _dataStore: alloy::sol_types::private::Address,
        pub _eventEmitter: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._roleStore, value._dataStore, value._eventEmitter)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _roleStore: tuple.0,
                        _dataStore: tuple.1,
                        _eventEmitter: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._roleStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eventEmitter,
                    ),
                )
            }
        }
    };
    /**Function with signature `dataStore()` and selector `0x660d0d67`.
```solidity
function dataStore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dataStoreCall {}
    ///Container type for the return parameters of the [`dataStore()`](dataStoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct dataStoreReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dataStoreCall> for UnderlyingRustTuple<'_> {
                fn from(value: dataStoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dataStoreCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<dataStoreReturn> for UnderlyingRustTuple<'_> {
                fn from(value: dataStoreReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for dataStoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for dataStoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = dataStoreReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "dataStore()";
            const SELECTOR: [u8; 4] = [102u8, 13u8, 13u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eventEmitter()` and selector `0x9ff78c30`.
```solidity
function eventEmitter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventEmitterCall {}
    ///Container type for the return parameters of the [`eventEmitter()`](eventEmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eventEmitterReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eventEmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: eventEmitterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventEmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eventEmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eventEmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eventEmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eventEmitterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eventEmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eventEmitter()";
            const SELECTOR: [u8; 4] = [159u8, 247u8, 140u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `executeAdd(address,(address,address,address))` and selector `0x5ecd44e8`.
```solidity
function executeAdd(address account, LiquidityUtils.AddParams memory AddParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeAddCall {
        pub account: alloy::sol_types::private::Address,
        pub AddParams: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeAdd(address,(address,address,address))`](executeAddCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeAddReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::AddParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeAddCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeAddCall) -> Self {
                    (value.account, value.AddParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        AddParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeAddReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeAddReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeAddReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeAddCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::AddParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeAddReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeAdd(address,(address,address,address))";
            const SELECTOR: [u8; 4] = [94u8, 205u8, 68u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <LiquidityUtils::AddParams as alloy_sol_types::SolType>::tokenize(
                        &self.AddParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `executeRemove(address,(address,address,uint256,address))` and selector `0x7d237c99`.
```solidity
function executeRemove(address account, LiquidityUtils.RemoveParams memory removeParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRemoveCall {
        pub account: alloy::sol_types::private::Address,
        pub removeParams: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeRemove(address,(address,address,uint256,address))`](executeRemoveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRemoveReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::RemoveParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeRemoveCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeRemoveCall) -> Self {
                    (value.account, value.removeParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRemoveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        removeParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeRemoveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeRemoveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRemoveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeRemoveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                LiquidityUtils::RemoveParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeRemoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeRemove(address,(address,address,uint256,address))";
            const SELECTOR: [u8; 4] = [125u8, 35u8, 124u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <LiquidityUtils::RemoveParams as alloy_sol_types::SolType>::tokenize(
                        &self.removeParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `roleStore()` and selector `0x4a4a7b04`.
```solidity
function roleStore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roleStoreCall {}
    ///Container type for the return parameters of the [`roleStore()`](roleStoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct roleStoreReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<roleStoreCall> for UnderlyingRustTuple<'_> {
                fn from(value: roleStoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roleStoreCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<roleStoreReturn> for UnderlyingRustTuple<'_> {
                fn from(value: roleStoreReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for roleStoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for roleStoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = roleStoreReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "roleStore()";
            const SELECTOR: [u8; 4] = [74u8, 74u8, 123u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`LiquidityHandler`](self) function calls.
    pub enum LiquidityHandlerCalls {
        dataStore(dataStoreCall),
        eventEmitter(eventEmitterCall),
        executeAdd(executeAddCall),
        executeRemove(executeRemoveCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl LiquidityHandlerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [74u8, 74u8, 123u8, 4u8],
            [94u8, 205u8, 68u8, 232u8],
            [102u8, 13u8, 13u8, 103u8],
            [125u8, 35u8, 124u8, 153u8],
            [159u8, 247u8, 140u8, 48u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LiquidityHandlerCalls {
        const NAME: &'static str = "LiquidityHandlerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::dataStore(_) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventEmitter(_) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeAdd(_) => {
                    <executeAddCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeRemove(_) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::roleStore(_) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<LiquidityHandlerCalls>] = &[
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn executeAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <executeAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::executeAdd)
                    }
                    executeAdd
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn executeRemove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <executeRemoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::executeRemove)
                    }
                    executeRemove
                },
                {
                    fn eventEmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerCalls> {
                        <eventEmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerCalls::eventEmitter)
                    }
                    eventEmitter
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeAdd(inner) => {
                    <executeAddCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::executeRemove(inner) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeAdd(inner) => {
                    <executeAddCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeRemove(inner) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`LiquidityHandler`](self) custom errors.
    pub enum LiquidityHandlerErrors {
        EmptyAddAmounts(EmptyAddAmounts),
        EmptyPool(EmptyPool),
        EmptyRemoveAmounts(EmptyRemoveAmounts),
        InsufficientUserBalance(InsufficientUserBalance),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        Reserve0Insufficient(Reserve0Insufficient),
        Reserve1Insufficient(Reserve1Insufficient),
        Unauthorized(Unauthorized),
    }
    #[automatically_derived]
    impl LiquidityHandlerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [14u8, 121u8, 59u8, 175u8],
            [31u8, 193u8, 7u8, 193u8],
            [34u8, 123u8, 193u8, 83u8],
            [81u8, 134u8, 89u8, 17u8],
            [95u8, 80u8, 77u8, 32u8],
            [105u8, 119u8, 202u8, 12u8],
            [115u8, 87u8, 217u8, 31u8],
            [163u8, 91u8, 21u8, 11u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for LiquidityHandlerErrors {
        const NAME: &'static str = "LiquidityHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyAddAmounts(_) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyRemoveAmounts(_) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientUserBalance(_) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MathOverflowedMulDiv(_) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve0Insufficient(_) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve1Insufficient(_) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<LiquidityHandlerErrors>] = &[
                {
                    fn Reserve1Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Reserve1Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Reserve1Insufficient)
                    }
                    Reserve1Insufficient
                },
                {
                    fn Reserve0Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Reserve0Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Reserve0Insufficient)
                    }
                    Reserve0Insufficient
                },
                {
                    fn MathOverflowedMulDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::MathOverflowedMulDiv)
                    }
                    MathOverflowedMulDiv
                },
                {
                    fn EmptyRemoveAmounts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyRemoveAmounts)
                    }
                    EmptyRemoveAmounts
                },
                {
                    fn InsufficientUserBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <InsufficientUserBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::InsufficientUserBalance)
                    }
                    InsufficientUserBalance
                },
                {
                    fn EmptyAddAmounts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyAddAmounts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyAddAmounts)
                    }
                    EmptyAddAmounts
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::EmptyPool)
                    }
                    EmptyPool
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<LiquidityHandlerErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(LiquidityHandlerErrors::Unauthorized)
                    }
                    Unauthorized
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::EmptyAddAmounts(inner) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyRemoveAmounts(inner) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientUserBalance(inner) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Reserve0Insufficient(inner) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Reserve1Insufficient(inner) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EmptyAddAmounts(inner) => {
                    <EmptyAddAmounts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyRemoveAmounts(inner) => {
                    <EmptyRemoveAmounts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientUserBalance(inner) => {
                    <InsufficientUserBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Reserve0Insufficient(inner) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Reserve1Insufficient(inner) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`LiquidityHandler`](self) contract instance.

See the [wrapper's documentation](`LiquidityHandlerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LiquidityHandlerInstance<T, P, N> {
        LiquidityHandlerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _eventEmitter: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<LiquidityHandlerInstance<T, P, N>>,
    > {
        LiquidityHandlerInstance::<
            T,
            P,
            N,
        >::deploy(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _eventEmitter: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        LiquidityHandlerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**A [`LiquidityHandler`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LiquidityHandler`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LiquidityHandlerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LiquidityHandlerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LiquidityHandlerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`LiquidityHandler`](self) contract instance.

See the [wrapper's documentation](`LiquidityHandlerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _eventEmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<LiquidityHandlerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _roleStore,
                _dataStore,
                _eventEmitter,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _eventEmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _roleStore,
                            _dataStore,
                            _eventEmitter,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> LiquidityHandlerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LiquidityHandlerInstance<T, P, N> {
            LiquidityHandlerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`dataStore`] function.
        pub fn dataStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, dataStoreCall, N> {
            self.call_builder(&dataStoreCall {})
        }
        ///Creates a new call builder for the [`eventEmitter`] function.
        pub fn eventEmitter(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eventEmitterCall, N> {
            self.call_builder(&eventEmitterCall {})
        }
        ///Creates a new call builder for the [`executeAdd`] function.
        pub fn executeAdd(
            &self,
            account: alloy::sol_types::private::Address,
            AddParams: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeAddCall, N> {
            self.call_builder(
                &executeAddCall {
                    account,
                    AddParams,
                },
            )
        }
        ///Creates a new call builder for the [`executeRemove`] function.
        pub fn executeRemove(
            &self,
            account: alloy::sol_types::private::Address,
            removeParams: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeRemoveCall, N> {
            self.call_builder(
                &executeRemoveCall {
                    account,
                    removeParams,
                },
            )
        }
        ///Creates a new call builder for the [`roleStore`] function.
        pub fn roleStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, roleStoreCall, N> {
            self.call_builder(&roleStoreCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidityHandlerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
