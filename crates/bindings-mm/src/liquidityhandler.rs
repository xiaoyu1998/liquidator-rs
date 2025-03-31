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
    ///0x60e060405234801561000f575f5ffd5b5060405161536838038061536883398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c05161525d61010b5f395f818160f0015281816101c601526102de01525f8181605e015261055601525f818160b601528181610197015281816102af0152818161037f01528181610485015261077c015261525d5ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614e1b565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614e59565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614e89565b6001600160a01b031681526020018360200160208101906102269190614e89565b6001600160a01b031681526020016102446060850160408601614e89565b6001600160a01b03169052905061025b83826105e8565b5061026461077a565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614e89565b6001600160a01b0316815260200183602001602081019061033e9190614e89565b6001600160a01b031681526040808501356020830152016103656080850160608601614e89565b6001600160a01b03169052905061025b8382610833565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614ea4565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614edb565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614ea4565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614edb565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614ef2565b61026457338160405163a35b150b60e01b815260040161047a929190614f3f565b5f5f610600835f015184604001518560600151610a5f565b915091505f61060e83610aa2565b905061061e845f01518483610b5c565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614edb565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614edb565b90506107138282611062565b5f610725878585858c6080015161108b565b9050610734885f0151886111cc565b610742886020015188611210565b875161074f908789611234565b61076f88602001518a8a604001518b60600151858d608001518989612450565b505050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b890614ea4565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af115801561080c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108309190614edb565b50565b61083b614cc4565b610851825f015183604001518460600151610a5f565b602083015280825260808301516108699185916124de565b81518151608084015161087d92919061259b565b60c0850181905260a08501829052610120850183905261010085018490526108a7939291906127f0565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108ff575f5ffd5b505af1158015610911573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610978575f5ffd5b505af115801561098a573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109f2575f5ffd5b505af1158015610a04573d5f5f3e3d5ffd5b50505060208301518251610a189250611210565b815160208201518251610a2c929190611234565b610a5a8260200151848460400151856060015186608001518760a00151876101000151886101200151612846565b505050565b610a67614d1d565b5f5f610a7385856128a1565b90505f610a808783612949565b9050610a8c8183613b4b565b610a9581613b79565b925090505b935093915050565b5f5f610ab0835f5f5f613c25565b50505090505f610ac38460015f5f613c25565b5050509050815f1480610ad4575080155b15610ae257505f9392505050565b5f610af185606001515f613d0e565b90505f610b0386606001516001613d0e565b90505f610b2685676765c793fa10079d601b1b610b2186600a615051565b613d3c565b90505f610b4485676765c793fa10079d601b1b610b2186600a615051565b9050610b508282613dfc565b98975050505050505050565b805f03610b6857505050565b610b8960405180606001604052806025815260200161520360259139613e37565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152610be38484613e5b565b63ffffffff168152610bfa64010000000042615070565b63ffffffff90811660208301528151165f03610cb657610c1f84848360200151613ed1565b610c2a84845f613f55565b610c3984848360200151613f94565b610c4484845f613fac565b610c4f848484613fc4565b610c896040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16613fdc565b610cb060405180604001604052806005815260200164707269636560d81b81525083613fdc565b50505050565b80516020820151610cc79190615083565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b60208201529051610d039216613fdc565b604081015163ffffffff1615610cb057610d1d8484614005565b60608201526040810151610d379063ffffffff168361509f565b8160600151610d4691906150b6565b6080820152604080518082019091526005815264707269636560d81b6020820152610d719083613fdc565b610da86040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16613fdc565b610de1604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151613fdc565b610e166040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fdc565b610e2584848360800151613f55565b610e3484848360200151613ed1565b610e3e848461401e565b63ffffffff1660a082018190526020820151610e5a9190615083565b63ffffffff1660c0820152610e6e84614037565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b60208201529051610ea69216613fdc565b610eef6040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16613fdc565b610f2e604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16613fdc565b8060e0015163ffffffff168160c0015163ffffffff161115610cb057610f5484846140e9565b610100820181905260c0820151608083015163ffffffff90911691610f78916150c9565b610f8291906150dc565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151610fcc9190613fdc565b6110016040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fdc565b6110346040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151613fdc565b61104384848360200151613f94565b61105284848360800151613fac565b610cb08484836101200151613fc4565b81158061106d575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f5f61109a8786866001614102565b90505f866001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110fd9190614edb565b9050805f03611163576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038816906340c10f19906044015f604051808303815f87803b15801561114c575f5ffd5b505af115801561115e573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038581166004830152602482018490528816906340c10f19906044015f604051808303815f87803b1580156111aa575f5ffd5b505af11580156111bc573d5f5f3e3d5ffd5b50939a9950505050505050505050565b6060810151660800000000000016610264575f5f6111ea848461425e565b91509150818110610cb05760608301516112059060016142bf565b606084015250505050565b611219816142e9565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611274906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156112c4575f5ffd5b505af11580156112d6573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161131a906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161134a929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156113ad573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113d19190615103565b50806001600160a01b031663e2a4853a84604051602001611419906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611449929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156114a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114ca9190614edb565b50806001600160a01b031663e2a4853a84604051602001611511906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611541929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561159d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115c19190614edb565b50806001600160a01b031663e2a4853a8460405160200161160d906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161163d929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561169a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116be9190614edb565b50806001600160a01b031663e2a4853a846040516020016116de9061511e565b6040516020818303038152906040528051906020012060405160200161170e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561176b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061178f9190614edb565b50806001600160a01b031663e2a4853a846040516020016117dc906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161180c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061188d9190614edb565b50806001600160a01b031663e2a4853a846040516020016118d6906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001611906929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611963573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119879190614edb565b50806001600160a01b031663ca446dd9846040516020016119c8906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016119f8929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611a5e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a829190615103565b50806001600160a01b031663e2a4853a84604051602001611aca90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001611afa929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b59573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b7d9190614edb565b50806001600160a01b031663e2a4853a84604051602001611bc490602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611bf4929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611c53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c779190614edb565b50806001600160a01b031663e2a4853a84604051602001611cc390602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cf3929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d779190614edb565b50806001600160a01b031663e2a4853a84604051602001611d979061515f565b60405160208183030381529060405280519060200120604051602001611dc7929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e4b9190614edb565b50806001600160a01b031663e2a4853a84604051602001611e9890602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ec8929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f4c9190614edb565b50806001600160a01b031663e2a4853a84604051602001611f9590602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fc5929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612025573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120499190614edb565b50806001600160a01b031663ca446dd98460405160200161208790602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016120b7929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016121019291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561211d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121419190615103565b50806001600160a01b031663ca446dd984604051602001612193906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016121c3929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b16835261220e926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561222a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224e9190615103565b50806001600160a01b031663e2a4853a84604051602001612295906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016122c5929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612306929190918252602082015260400190565b6020604051808303815f875af1158015612322573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123469190614edb565b50806001600160a01b031663e2a4853a84604051602001612398906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016123c8929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612409929190918252602082015260400190565b6020604051808303815f875af1158015612425573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124499190614edb565b5050505050565b604051630cecd96d60e21b81526001600160a01b0388811660048301528781166024830152868116604483015284811660648301526084820184905260a4820183905260c482018690528916906333b365b49060e4015b5f604051808303815f87803b1580156124be575f5ffd5b505af11580156124d0573d5f5f3e3d5ffd5b505050505050505050505050565b805f036124fe57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa15801561254a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061256e9190614edb565b905080831115612449576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f6125a7614d51565b6125b0886144f8565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061261e9190614edb565b816020018181525050612638875f5f846101400151613c25565b50608084015250604082015261014081015161265a9088906001905f90613c25565b5060a084015250606082015260208101515f03612683575f5f5f5f9450945094509450506127e7565b6126968682604001518360200151613d3c565b610100820152606081015160208201516126b1918891613d3c565b816101200181815250506126f16040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613fdc565b612729604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613fdc565b61276160405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613fdc565b6127946040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613fdc565b6127c76040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613fdc565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b8184111561281b57604051631fc107c160e01b8152600481018590526024810183905260440161047a565b80831115610cb057604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4016124a7565b5f816001600160a01b0316836001600160a01b0316106128c25781836128c5565b82825b60405191945092506128f2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612951614d1d565b8261295a614d1d565b816001600160a01b03166391d4403c604051602001612996906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156129ea573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a0e9190614ef2565b612a1b5791506129439050565b816001600160a01b03166321f8a72185604051602001612a5b906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a8b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612abf91815260200190565b602060405180830381865afa158015612ada573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612afe9190615103565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612b7c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612bb091815260200190565b602060405180830381865afa158015612bcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bef9190614edb565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612c45906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c75929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612ca991815260200190565b602060405180830381865afa158015612cc4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce89190614edb565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612d43906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d73929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612da791815260200190565b602060405180830381865afa158015612dc2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612de69190614edb565b815151606001526040516001600160a01b0383169063bd02d0f5908690612e0f9060200161511e565b60405160208183030381529060405280519060200120604051602001612e3f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e7391815260200190565b602060405180830381865afa158015612e8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612eb29190614edb565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612f0e906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f3e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f7291815260200190565b602060405180830381865afa158015612f8d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fb19190614edb565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161302e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161306291815260200190565b602060405180830381865afa15801561307d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130a19190614edb565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613116929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161314a91815260200190565b602060405180830381865afa158015613165573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131899190615103565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161323391815260200190565b602060405180830381865afa15801561324e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132729190614edb565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016132c990602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016132f9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161332d91815260200190565b602060405180830381865afa158015613348573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336c9190614edb565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016133c890602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016133f8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161342c91815260200190565b602060405180830381865afa158015613447573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061346b9190614edb565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161349b9061515f565b604051602081830303815290604052805190602001206040516020016134cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134ff91815260200190565b602060405180830381865afa15801561351a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061353e9190614edb565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161359b90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016135cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135ff91815260200190565b602060405180830381865afa15801561361a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061363e9190614edb565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161369790602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016136c7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136fb91815260200190565b602060405180830381865afa158015613716573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061373a9190614edb565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161378890602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016137b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137ec91815260200190565b602060405180830381865afa158015613807573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061382b9190615103565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613899906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016138c9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138fd91815260200190565b602060405180830381865afa158015613918573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061393c9190615103565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161399f906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016139cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a0391815260200190565b602060405180830381865afa158015613a1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a429190614edb565b60608201526040516001600160a01b0383169063bd02d0f5908690613a9b906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613acb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aff91815260200190565b602060405180830381865afa158015613b1a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b3e9190614edb565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613b8b575050565b81515160a0015115613be15781515f90613bb490825b6020020151604001518460800151614549565b8351909150613bd89082905f5b60200201516020015161457d90919063ffffffff16565b83515160200152505b81516020015160a0015115613c1d5781515f90613bff906001613ba1565b8351909150613c119082906001613bc1565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613c4257613c426150ef565b602002015190505f613c548a8a6145be565b9050805f03613c70575f5f5f5f95509550955095505050613d03565b5f613c7f838c608001516146ac565b90505f613c8c828a61457d565b90505f8915613cb157818410613cab57613ca684836146dc565b613cb3565b5f613cb3565b5f5b90505f613cc0858d61457d565b90505f8c15613ce557848210613cdf57613cda82866146dc565b613ce7565b5f613ce7565b5f5b9050613cf385876150b6565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613d2e575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613d7057838281613d6657613d6661505c565b0492505050613df5565b808411613d905760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613e1d575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61083060405180604001604052806002815260200161257360f01b81525082614731565b5f826001600160a01b031663bd02d0f5613e7484614776565b6040518263ffffffff1660e01b8152600401613e9291815260200190565b602060405180830381865afa158015613ead573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613df59190614edb565b826001600160a01b031663e2a4853a613ee984614776565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613f31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cb09190614edb565b826001600160a01b031663e2a4853a613f6d84614829565b836040518363ffffffff1660e01b8152600401613f15929190918252602082015260400190565b826001600160a01b031663e2a4853a613ee98461487d565b826001600160a01b031663e2a4853a613f6d846148de565b826001600160a01b031663e2a4853a613f6d84614944565b610264604051806040016040528060068152602001652573202d257360d01b815250838361498d565b5f826001600160a01b031663bd02d0f5613e7484614829565b5f826001600160a01b031663bd02d0f5613e748461487d565b5f816001600160a01b031663bd02d0f5604051602001614076906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140aa91815260200190565b602060405180830381865afa1580156140c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129439190614edb565b5f826001600160a01b031663bd02d0f5613e74846148de565b5f61410b614d51565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561414b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061416f9190614edb565b6020820152614180865f8080613c25565b50505060c08201526141958660015f80613c25565b50505060e082015282156141d057848160c0018181516141b591906150c9565b90525060e0810180518591906141cc9083906150c9565b9052505b80602001515f03614200576141f96103e86141f36141ee88886149d4565b614a3a565b906146dc565b8152614252565b60c08101511580614213575060e0810151155b15614221575f915050614256565b61424f6142378683602001518460c00151613d3c565b61424a8684602001518560e00151613d3c565b614b1a565b81525b5190505b949350505050565b5f5f5f61426f84606001515f613d0e565b90505f61427b86614b2f565b90505f61429e8261428d85600a615051565b676765c793fa10079d601b1b613d3c565b90505f6142ad875f5f5f613c25565b50939a91995090975050505050505050565b5f6033826142cd575f6142d0565b60015b60ff16901b660800000000000019841617905092915050565b5f5f61431e6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614328845f614b80565b60208301528152606084015161433e905f613d0e565b60608201819052815161436391676765c793fa10079d601b1b90610b2190600a615051565b604082015260208101515f0361437e575f608082015261441e565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156143f4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144189190614edb565b60808201525b614429846001614b80565b602083018190529082525f03614444575f60a08201526144e4565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156144ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144de9190614edb565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001614076906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f8061455583426150c9565b61455f908561509f565b6301e133809004905061425681676765c793fa10079d601b1b6150b6565b5f81156b019d971e4fe8401e74000000198390048411151761459d575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff16600281106145d8576145d86150ef565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614631573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146559190614edb565b9050805f03614668575f92505050612943565b606082015160c083015161467c90826150b6565b82106146a05760c083015161469182846150c9565b61469b91906150c9565b6146a2565b5f5b9695505050505050565b5f8260a001515f036146bf57505f612943565b5f6146ca8484614bc6565b60a0850151909150614256908261457d565b5f826146e883826150c9565b91508111156129435760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b61026482826040516024016147479291906151a0565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614c09565b80515f90819061479790825b602002015151845160016020020151516128a1565b9050806040516020016147db906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161480b929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906148399082614782565b9050806040516020016147db906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061488d9082614782565b9050806040516020016147db9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906148ee9082614782565b9050806040516020016147db9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906149549082614782565b9050806040516020016147db90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a5a8383836040516024016149a5939291906151cd565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614c09565b5f8115806149f7575082826149e9818361509f565b92506149f590836150dc565b145b6129435760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f03614a4957505f919050565b5f6001614a5584614c12565b901c6001901b90506001818481614a6e57614a6e61505c565b048201901c90506001818481614a8657614a8661505c565b048201901c90506001818481614a9e57614a9e61505c565b048201901c90506001818481614ab657614ab661505c565b048201901c90506001818481614ace57614ace61505c565b048201901c90506001818481614ae657614ae661505c565b048201901c90506001818481614afe57614afe61505c565b048201901c9050613df581828581614b1857614b1861505c565b045b5f818310614b285781613df5565b5090919050565b5f816001600160a01b031663bd02d0f56040516020016140769060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614bad855f01518560ff1660028110614b9e57614b9e6150ef565b602002015186608001516146ac565b90505f614bba86866145be565b96919550909350505050565b5f428203614bd957506020820151612943565b5f614be8846040015184614549565b9050614c0184602001518261457d90919063ffffffff16565b915050612943565b61083081614ca5565b5f80608083901c15614c2657608092831c92015b604083901c15614c3857604092831c92015b602083901c15614c4a57602092831c92015b601083901c15614c5c57601092831c92015b600883901c15614c6e57600892831c92015b600483901c15614c8057600492831c92015b600283901c15614c9257600292831c92015b600183901c156129435760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614cd8614d1d565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614d30614da0565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614df16040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614daf5790505090565b6001600160a01b0381168114610830575f5ffd5b5f5f8284036080811215614e2d575f5ffd5b8335614e3881614e07565b92506060601f1982011215614e4b575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614e6b575f5ffd5b8335614e7681614e07565b92506080601f1982011215614e4b575f5ffd5b5f60208284031215614e99575f5ffd5b8135613df581614e07565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614eeb575f5ffd5b5051919050565b5f60208284031215614f02575f5ffd5b81518015158114613df5575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061425690830184614f11565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9a57808504811115614f9557614f95614f62565b6001841615614fa357908102905b60019390931c928002614f7a565b5f82614fbf57506001612943565b81614fcb57505f612943565b8160018114614fe15760028114614feb57615007565b6001915050612943565b60ff841115614ffc57614ffc614f62565b50506001821b612943565b5060208310610133831016604e8410600b841016171561502a575081810a612943565b6150365f198484614f76565b805f190482111561504957615049614f62565b029392505050565b5f613df58383614fb1565b634e487b7160e01b5f52601260045260245ffd5b5f8261507e5761507e61505c565b500690565b63ffffffff828116828216039081111561294357612943614f62565b808202811582820484141761294357612943614f62565b8082018082111561294357612943614f62565b8181038181111561294357612943614f62565b5f826150ea576150ea61505c565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215615113575f5ffd5b8151613df581614e07565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6151b26040830185614f11565b82810360208401526151c48185614f11565b95945050505050565b606081525f6151df6060830186614f11565b82810360208401526151f18186614f11565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da2646970667358221220256a93aab88bf78d1151d4a48875e9e5112742a25ab4812279baafa54c11f00964736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaSh8\x03\x80aSh\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QaR]a\x01\x0B_9_\x81\x81`\xF0\x01R\x81\x81a\x01\xC6\x01Ra\x02\xDE\x01R_\x81\x81`^\x01Ra\x05V\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\xAF\x01R\x81\x81a\x03\x7F\x01R\x81\x81a\x04\x85\x01Ra\x07|\x01RaR]_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aN\x1BV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aNYV[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07zV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x083V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aN\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aN\xDBV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aN\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aN\xDBV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aN\xF2V[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aO?V[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\n_V[\x91P\x91P_a\x06\x0E\x83a\n\xA2V[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0B\\V[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aN\xDBV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aN\xDBV[\x90Pa\x07\x13\x82\x82a\x10bV[_a\x07%\x87\x85\x85\x85\x8C`\x80\x01Qa\x10\x8BV[\x90Pa\x074\x88_\x01Q\x88a\x11\xCCV[a\x07B\x88` \x01Q\x88a\x12\x10V[\x87Qa\x07O\x90\x87\x89a\x124V[a\x07o\x88` \x01Q\x8A\x8A`@\x01Q\x8B``\x01Q\x85\x8D`\x80\x01Q\x89\x89a$PV[PPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB8\x90aN\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x080\x91\x90aN\xDBV[PV[a\x08;aL\xC4V[a\x08Q\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\n_V[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08i\x91\x85\x91a$\xDEV[\x81Q\x81Q`\x80\x84\x01Qa\x08}\x92\x91\x90a%\x9BV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA7\x93\x92\x91\x90a'\xF0V[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xFFW__\xFD[PZ\xF1\x15\x80\x15a\t\x11W=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\txW__\xFD[PZ\xF1\x15\x80\x15a\t\x8AW=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xF2W__\xFD[PZ\xF1\x15\x80\x15a\n\x04W=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x18\x92Pa\x12\x10V[\x81Q` \x82\x01Q\x82Qa\n,\x92\x91\x90a\x124V[a\nZ\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa(FV[PPPV[a\ngaM\x1DV[__a\ns\x85\x85a(\xA1V[\x90P_a\n\x80\x87\x83a)IV[\x90Pa\n\x8C\x81\x83a;KV[a\n\x95\x81a;yV[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xB0\x83___a<%V[PPP\x90P_a\n\xC3\x84`\x01__a<%V[PPP\x90P\x81_\x14\x80a\n\xD4WP\x80\x15[\x15a\n\xE2WP_\x93\x92PPPV[_a\n\xF1\x85``\x01Q_a=\x0EV[\x90P_a\x0B\x03\x86``\x01Q`\x01a=\x0EV[\x90P_a\x0B&\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B!\x86`\naPQV[a=<V[\x90P_a\x0BD\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B!\x86`\naPQV[\x90Pa\x0BP\x82\x82a=\xFCV[\x98\x97PPPPPPPPV[\x80_\x03a\x0BhWPPPV[a\x0B\x89`@Q\x80``\x01`@R\x80`%\x81R` \x01aR\x03`%\x919a>7V[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x0B\xE3\x84\x84a>[V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x0B\xFAd\x01\0\0\0\0BaPpV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x0C\xB6Wa\x0C\x1F\x84\x84\x83` \x01Qa>\xD1V[a\x0C*\x84\x84_a?UV[a\x0C9\x84\x84\x83` \x01Qa?\x94V[a\x0CD\x84\x84_a?\xACV[a\x0CO\x84\x84\x84a?\xC4V[a\x0C\x89`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\x0C\xB0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83a?\xDCV[PPPPV[\x80Q` \x82\x01Qa\x0C\xC7\x91\x90aP\x83V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\r\x03\x92\x16a?\xDCV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x0C\xB0Wa\r\x1D\x84\x84a@\x05V[``\x82\x01R`@\x81\x01Qa\r7\x90c\xFF\xFF\xFF\xFF\x16\x83aP\x9FV[\x81``\x01Qa\rF\x91\x90aP\xB6V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\rq\x90\x83a?\xDCV[a\r\xA8`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\r\xE1`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01Qa?\xDCV[a\x0E\x16`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xDCV[a\x0E%\x84\x84\x83`\x80\x01Qa?UV[a\x0E4\x84\x84\x83` \x01Qa>\xD1V[a\x0E>\x84\x84a@\x1EV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x0EZ\x91\x90aP\x83V[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x0En\x84a@7V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x0E\xA6\x92\x16a?\xDCV[a\x0E\xEF`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\x0F.`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xB0Wa\x0FT\x84\x84a@\xE9V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0Fx\x91aP\xC9V[a\x0F\x82\x91\x90aP\xDCV[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x0F\xCC\x91\x90a?\xDCV[a\x10\x01`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xDCV[a\x104`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xDCV[a\x10C\x84\x84\x83` \x01Qa?\x94V[a\x10R\x84\x84\x83`\x80\x01Qa?\xACV[a\x0C\xB0\x84\x84\x83a\x01 \x01Qa?\xC4V[\x81\x15\x80a\x10mWP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x10\x9A\x87\x86\x86`\x01aA\x02V[\x90P_\x86`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFD\x91\x90aN\xDBV[\x90P\x80_\x03a\x11cW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11LW__\xFD[PZ\xF1\x15\x80\x15a\x11^W=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xAAW__\xFD[PZ\xF1\x15\x80\x15a\x11\xBCW=__>=_\xFD[P\x93\x9A\x99PPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x11\xEA\x84\x84aB^V[\x91P\x91P\x81\x81\x10a\x0C\xB0W``\x83\x01Qa\x12\x05\x90`\x01aB\xBFV[``\x84\x01RPPPPV[a\x12\x19\x81aB\xE9V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x12t\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xC4W__\xFD[PZ\xF1\x15\x80\x15a\x12\xD6W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x13\x1A\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13J\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD1\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x19\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14I\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCA\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\x11\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC1\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\r\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16=\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xDE\x90aQ\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8F\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xDC\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x0C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x8D\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xD6\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x87\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x19\xC8\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x82\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xCA\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BYW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B}\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xC4\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cw\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xC3\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dw\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x97\x90aQ_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EK\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x98\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FL\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x95\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a %W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a I\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a \x87\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xB7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\x01\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!A\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a!\x93\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\"\x0E\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"N\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x95\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#F\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x98\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$I\x91\x90aN\xDBV[PPPPPV[`@Qc\x0C\xEC\xD9m`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R`\xC4\x82\x01\x86\x90R\x89\x16\x90c3\xB3e\xB4\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\xBEW__\xFD[PZ\xF1\x15\x80\x15a$\xD0W=__>=_\xFD[PPPPPPPPPPPPV[\x80_\x03a$\xFEW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%n\x91\x90aN\xDBV[\x90P\x80\x83\x11\x15a$IW`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a%\xA7aMQV[a%\xB0\x88aD\xF8V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1E\x91\x90aN\xDBV[\x81` \x01\x81\x81RPPa&8\x87__\x84a\x01@\x01Qa<%V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa&Z\x90\x88\x90`\x01\x90_\x90a<%V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a&\x83W____\x94P\x94P\x94P\x94PPa'\xE7V[a&\x96\x86\x82`@\x01Q\x83` \x01Qa=<V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa&\xB1\x91\x88\x91a=<V[\x81a\x01 \x01\x81\x81RPPa&\xF1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa?\xDCV[a')`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa?\xDCV[a'a`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa?\xDCV[a'\x94`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa?\xDCV[a'\xC7`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xDCV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a(\x1BW`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x0C\xB0W`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01a$\xA7V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a(\xC2W\x81\x83a(\xC5V[\x82\x82[`@Q\x91\x94P\x92Pa(\xF2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a)QaM\x1DV[\x82a)ZaM\x1DV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a)\x96\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\x0E\x91\x90aN\xF2V[a*\x1BW\x91Pa)C\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a*[\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\x8B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xBF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xFE\x91\x90aQ\x03V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xB0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xEF\x91\x90aN\xDBV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a,E\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xA9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE8\x91\x90aN\xDBV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-C\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-s\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xA7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xE6\x91\x90aN\xDBV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a.\x0F\x90` \x01aQ\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.?\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.s\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xB2\x91\x90aN\xDBV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a/\x0E\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/>\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/r\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xB1\x91\x90aN\xDBV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0.\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xA1\x91\x90aN\xDBV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\x89\x91\x90aQ\x03V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a23\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2r\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\xC9\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xF9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3-\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3HW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3l\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3\xC8\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4,\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4k\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4\x9B\x90aQ_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5>\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5\x9B\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6>\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a6\x97\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xFB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7:\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7\x88\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8+\x91\x90aQ\x03V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a8\x99\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xC9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9<\x91\x90aQ\x03V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\x9F\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:B\x91\x90aN\xDBV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a:\x9B\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;>\x91\x90aN\xDBV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a;\x8BWPPV[\x81QQ`\xA0\x01Q\x15a;\xE1W\x81Q_\x90a;\xB4\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaEIV[\x83Q\x90\x91Pa;\xD8\x90\x82\x90_[` \x02\x01Q` \x01QaE}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a<\x1DW\x81Q_\x90a;\xFF\x90`\x01a;\xA1V[\x83Q\x90\x91Pa<\x11\x90\x82\x90`\x01a;\xC1V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a<BWa<BaP\xEFV[` \x02\x01Q\x90P_a<T\x8A\x8AaE\xBEV[\x90P\x80_\x03a<pW____\x95P\x95P\x95P\x95PPPa=\x03V[_a<\x7F\x83\x8C`\x80\x01QaF\xACV[\x90P_a<\x8C\x82\x8AaE}V[\x90P_\x89\x15a<\xB1W\x81\x84\x10a<\xABWa<\xA6\x84\x83aF\xDCV[a<\xB3V[_a<\xB3V[_[\x90P_a<\xC0\x85\x8DaE}V[\x90P_\x8C\x15a<\xE5W\x84\x82\x10a<\xDFWa<\xDA\x82\x86aF\xDCV[a<\xE7V[_a<\xE7V[_[\x90Pa<\xF3\x85\x87aP\xB6V[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=.WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a=pW\x83\x82\x81a=fWa=faP\\V[\x04\x92PPPa=\xF5V[\x80\x84\x11a=\x90W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a>\x1DW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x080`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aG1V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aGvV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x92\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xF5\x91\x90aN\xDBV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xE9\x84aGvV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a?1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xB0\x91\x90aN\xDBV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aH)V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xE9\x84aH}V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aH\xDEV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aIDV[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aI\x8DV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH)V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH}V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xAA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)C\x91\x90aN\xDBV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH\xDEV[_aA\x0BaMQV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAKW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aAo\x91\x90aN\xDBV[` \x82\x01RaA\x80\x86_\x80\x80a<%V[PPP`\xC0\x82\x01RaA\x95\x86`\x01_\x80a<%V[PPP`\xE0\x82\x01R\x82\x15aA\xD0W\x84\x81`\xC0\x01\x81\x81QaA\xB5\x91\x90aP\xC9V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90aA\xCC\x90\x83\x90aP\xC9V[\x90RP[\x80` \x01Q_\x03aB\0WaA\xF9a\x03\xE8aA\xF3aA\xEE\x88\x88aI\xD4V[aJ:V[\x90aF\xDCV[\x81RaBRV[`\xC0\x81\x01Q\x15\x80aB\x13WP`\xE0\x81\x01Q\x15[\x15aB!W_\x91PPaBVV[aBOaB7\x86\x83` \x01Q\x84`\xC0\x01Qa=<V[aBJ\x86\x84` \x01Q\x85`\xE0\x01Qa=<V[aK\x1AV[\x81R[Q\x90P[\x94\x93PPPPV[___aBo\x84``\x01Q_a=\x0EV[\x90P_aB{\x86aK/V[\x90P_aB\x9E\x82aB\x8D\x85`\naPQV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba=<V[\x90P_aB\xAD\x87___a<%V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aB\xCDW_aB\xD0V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aC\x1E`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC(\x84_aK\x80V[` \x83\x01R\x81R``\x84\x01QaC>\x90_a=\x0EV[``\x82\x01\x81\x90R\x81QaCc\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B!\x90`\naPQV[`@\x82\x01R` \x81\x01Q_\x03aC~W_`\x80\x82\x01RaD\x1EV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x18\x91\x90aN\xDBV[`\x80\x82\x01R[aD)\x84`\x01aK\x80V[` \x83\x01\x81\x90R\x90\x82R_\x03aDDW_`\xA0\x82\x01RaD\xE4V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xDE\x91\x90aN\xDBV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aEU\x83BaP\xC9V[aE_\x90\x85aP\x9FV[c\x01\xE13\x80\x90\x04\x90PaBV\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xB6V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aE\x9DW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aE\xD8WaE\xD8aP\xEFV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFU\x91\x90aN\xDBV[\x90P\x80_\x03aFhW_\x92PPPa)CV[``\x82\x01Q`\xC0\x83\x01QaF|\x90\x82aP\xB6V[\x82\x10aF\xA0W`\xC0\x83\x01QaF\x91\x82\x84aP\xC9V[aF\x9B\x91\x90aP\xC9V[aF\xA2V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aF\xBFWP_a)CV[_aF\xCA\x84\x84aK\xC6V[`\xA0\x85\x01Q\x90\x91PaBV\x90\x82aE}V[_\x82aF\xE8\x83\x82aP\xC9V[\x91P\x81\x11\x15a)CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aGG\x92\x91\x90aQ\xA0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaL\tV[\x80Q_\x90\x81\x90aG\x97\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa(\xA1V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aH9\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aH\x8D\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aH\xEE\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aIT\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nZ\x83\x83\x83`@Q`$\x01aI\xA5\x93\x92\x91\x90aQ\xCDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaL\tV[_\x81\x15\x80aI\xF7WP\x82\x82aI\xE9\x81\x83aP\x9FV[\x92PaI\xF5\x90\x83aP\xDCV[\x14[a)CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aJIWP_\x91\x90PV[_`\x01aJU\x84aL\x12V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aJnWaJnaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x86WaJ\x86aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x9EWaJ\x9EaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xB6WaJ\xB6aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xCEWaJ\xCEaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xE6WaJ\xE6aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xFEWaJ\xFEaP\\V[\x04\x82\x01\x90\x1C\x90Pa=\xF5\x81\x82\x85\x81aK\x18WaK\x18aP\\V[\x04[_\x81\x83\x10aK(W\x81a=\xF5V[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aK\xAD\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aK\x9EWaK\x9EaP\xEFV[` \x02\x01Q\x86`\x80\x01QaF\xACV[\x90P_aK\xBA\x86\x86aE\xBEV[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aK\xD9WP` \x82\x01Qa)CV[_aK\xE8\x84`@\x01Q\x84aEIV[\x90PaL\x01\x84` \x01Q\x82aE}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa)CV[a\x080\x81aL\xA5V[_\x80`\x80\x83\x90\x1C\x15aL&W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aL8W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aLJW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aL\\W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aLnW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aL\x80W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aL\x92W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a)CW`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aL\xD8aM\x1DV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aM0aM\xA0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aM\xF1`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aM\xAFW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x080W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aN-W__\xFD[\x835aN8\x81aN\x07V[\x92P```\x1F\x19\x82\x01\x12\x15aNKW__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aNkW__\xFD[\x835aNv\x81aN\x07V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aNKW__\xFD[_` \x82\x84\x03\x12\x15aN\x99W__\xFD[\x815a=\xF5\x81aN\x07V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aN\xEBW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aO\x02W__\xFD[\x81Q\x80\x15\x15\x81\x14a=\xF5W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aBV\x90\x83\x01\x84aO\x11V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x9AW\x80\x85\x04\x81\x11\x15aO\x95WaO\x95aObV[`\x01\x84\x16\x15aO\xA3W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aOzV[_\x82aO\xBFWP`\x01a)CV[\x81aO\xCBWP_a)CV[\x81`\x01\x81\x14aO\xE1W`\x02\x81\x14aO\xEBWaP\x07V[`\x01\x91PPa)CV[`\xFF\x84\x11\x15aO\xFCWaO\xFCaObV[PP`\x01\x82\x1Ba)CV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aP*WP\x81\x81\na)CV[aP6_\x19\x84\x84aOvV[\x80_\x19\x04\x82\x11\x15aPIWaPIaObV[\x02\x93\x92PPPV[_a=\xF5\x83\x83aO\xB1V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP~WaP~aP\\V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)CWa)CaObV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)CWa)CaObV[\x80\x82\x01\x80\x82\x11\x15a)CWa)CaObV[\x81\x81\x03\x81\x81\x11\x15a)CWa)CaObV[_\x82aP\xEAWaP\xEAaP\\V[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aQ\x13W__\xFD[\x81Qa=\xF5\x81aN\x07V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aQ\xB2`@\x83\x01\x85aO\x11V[\x82\x81\x03` \x84\x01RaQ\xC4\x81\x85aO\x11V[\x95\x94PPPPPV[``\x81R_aQ\xDF``\x83\x01\x86aO\x11V[\x82\x81\x03` \x84\x01RaQ\xF1\x81\x86aO\x11V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 %j\x93\xAA\xB8\x8B\xF7\x8D\x11Q\xD4\xA4\x88u\xE9\xE5\x11'B\xA2Z\xB4\x81\"y\xBA\xAF\xA5L\x11\xF0\tdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b04146100595780635ecd44e81461009c578063660d0d67146100b15780637d237c99146100d85780639ff78c30146100eb575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004614e1b565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af6100e6366004614e59565b610268565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b61011a61037c565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b81525061053a565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102059190614e89565b6001600160a01b031681526020018360200160208101906102269190614e89565b6001600160a01b031681526020016102446060850160408601614e89565b6001600160a01b03169052905061025b83826105e8565b5061026461077a565b5050565b61027061037c565b6102a160405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518060c001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f01602081019061031d9190614e89565b6001600160a01b0316815260200183602001602081019061033e9190614e89565b6001600160a01b031681526040808501356020830152016103656080850160608601614e89565b6001600160a01b03169052905061025b8382610833565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103bb90614ea4565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103ef91815260200190565b602060405180830381865afa15801561040a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042e9190614edb565b905080156104835760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104c190614ea4565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af1158015610516573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190614edb565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa1580156105a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105c79190614ef2565b61026457338160405163a35b150b60e01b815260040161047a929190614f3f565b5f5f610600835f015184604001518560600151610a5f565b915091505f61060e83610aa2565b905061061e845f01518483610b5c565b6020830151604085810151905163352f9aed60e01b81526001600160a01b0391821660048201525f9183169063352f9aed906024016020604051808303815f875af115801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190614edb565b606087015160405163352f9aed60e01b81526001600160a01b0391821660048201529192505f919084169063352f9aed906024016020604051808303815f875af11580156106e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107079190614edb565b90506107138282611062565b5f610725878585858c6080015161108b565b9050610734885f0151886111cc565b610742886020015188611210565b875161074f908789611234565b61076f88602001518a8a604001518b60600151858d608001518989612450565b505050505050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016107b890614ea4565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af115801561080c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108309190614edb565b50565b61083b614cc4565b610851825f015183604001518460600151610a5f565b602083015280825260808301516108699185916124de565b81518151608084015161087d92919061259b565b60c0850181905260a08501829052610120850183905261010085018490526108a7939291906127f0565b8051602001516001600160a01b03908116604080840182905260808501519051632770a7eb60e21b81529286166004840152602483015290639dc29fac906044015f604051808303815f87803b1580156108ff575f5ffd5b505af1158015610911573d5f5f3e3d5ffd5b505050506040818101518382015160a0850151610100850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610978575f5ffd5b505af115801561098a573d5f5f3e3d5ffd5b50505050604081810151606084015160a0850151610120850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b1580156109f2575f5ffd5b505af1158015610a04573d5f5f3e3d5ffd5b50505060208301518251610a189250611210565b815160208201518251610a2c929190611234565b610a5a8260200151848460400151856060015186608001518760a00151876101000151886101200151612846565b505050565b610a67614d1d565b5f5f610a7385856128a1565b90505f610a808783612949565b9050610a8c8183613b4b565b610a9581613b79565b925090505b935093915050565b5f5f610ab0835f5f5f613c25565b50505090505f610ac38460015f5f613c25565b5050509050815f1480610ad4575080155b15610ae257505f9392505050565b5f610af185606001515f613d0e565b90505f610b0386606001516001613d0e565b90505f610b2685676765c793fa10079d601b1b610b2186600a615051565b613d3c565b90505f610b4485676765c793fa10079d601b1b610b2186600a615051565b9050610b508282613dfc565b98975050505050505050565b805f03610b6857505050565b610b8960405180606001604052806025815260200161520360259139613e37565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152610be38484613e5b565b63ffffffff168152610bfa64010000000042615070565b63ffffffff90811660208301528151165f03610cb657610c1f84848360200151613ed1565b610c2a84845f613f55565b610c3984848360200151613f94565b610c4484845f613fac565b610c4f848484613fc4565b610c896040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16613fdc565b610cb060405180604001604052806005815260200164707269636560d81b81525083613fdc565b50505050565b80516020820151610cc79190615083565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b60208201529051610d039216613fdc565b604081015163ffffffff1615610cb057610d1d8484614005565b60608201526040810151610d379063ffffffff168361509f565b8160600151610d4691906150b6565b6080820152604080518082019091526005815264707269636560d81b6020820152610d719083613fdc565b610da86040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16613fdc565b610de1604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151613fdc565b610e166040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fdc565b610e2584848360800151613f55565b610e3484848360200151613ed1565b610e3e848461401e565b63ffffffff1660a082018190526020820151610e5a9190615083565b63ffffffff1660c0820152610e6e84614037565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b60208201529051610ea69216613fdc565b610eef6040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16613fdc565b610f2e604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16613fdc565b8060e0015163ffffffff168160c0015163ffffffff161115610cb057610f5484846140e9565b610100820181905260c0820151608083015163ffffffff90911691610f78916150c9565b610f8291906150dc565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151610fcc9190613fdc565b6110016040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151613fdc565b6110346040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151613fdc565b61104384848360200151613f94565b61105284848360800151613fac565b610cb08484836101200151613fc4565b81158061106d575080155b1561026457604051631a5df28360e21b815260040160405180910390fd5b5f5f61109a8786866001614102565b90505f866001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156110d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110fd9190614edb565b9050805f03611163576040516340c10f1960e01b81525f60048201526103e860248201526001600160a01b038816906340c10f19906044015f604051808303815f87803b15801561114c575f5ffd5b505af115801561115e573d5f5f3e3d5ffd5b505050505b6040516340c10f1960e01b81526001600160a01b038581166004830152602482018490528816906340c10f19906044015f604051808303815f87803b1580156111aa575f5ffd5b505af11580156111bc573d5f5f3e3d5ffd5b50939a9950505050505050505050565b6060810151660800000000000016610264575f5f6111ea848461425e565b91509150818110610cb05760608301516112059060016142bf565b606084015250505050565b611219816142e9565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611274906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b1580156112c4575f5ffd5b505af11580156112d6573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161131a906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161134a929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156113ad573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113d19190615103565b50806001600160a01b031663e2a4853a84604051602001611419906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611449929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156114a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114ca9190614edb565b50806001600160a01b031663e2a4853a84604051602001611511906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611541929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561159d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115c19190614edb565b50806001600160a01b031663e2a4853a8460405160200161160d906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161163d929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561169a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116be9190614edb565b50806001600160a01b031663e2a4853a846040516020016116de9061511e565b6040516020818303038152906040528051906020012060405160200161170e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561176b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061178f9190614edb565b50806001600160a01b031663e2a4853a846040516020016117dc906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161180c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061188d9190614edb565b50806001600160a01b031663e2a4853a846040516020016118d6906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001611906929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611963573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119879190614edb565b50806001600160a01b031663ca446dd9846040516020016119c8906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016119f8929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611a5e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a829190615103565b50806001600160a01b031663e2a4853a84604051602001611aca90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001611afa929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611b59573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b7d9190614edb565b50806001600160a01b031663e2a4853a84604051602001611bc490602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611bf4929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611c53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c779190614edb565b50806001600160a01b031663e2a4853a84604051602001611cc390602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cf3929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d779190614edb565b50806001600160a01b031663e2a4853a84604051602001611d979061515f565b60405160208183030381529060405280519060200120604051602001611dc7929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e4b9190614edb565b50806001600160a01b031663e2a4853a84604051602001611e9890602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ec8929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f4c9190614edb565b50806001600160a01b031663e2a4853a84604051602001611f9590602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fc5929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612025573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120499190614edb565b50806001600160a01b031663ca446dd98460405160200161208790602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016120b7929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016121019291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561211d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121419190615103565b50806001600160a01b031663ca446dd984604051602001612193906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016121c3929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b16835261220e926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561222a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061224e9190615103565b50806001600160a01b031663e2a4853a84604051602001612295906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016122c5929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612306929190918252602082015260400190565b6020604051808303815f875af1158015612322573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123469190614edb565b50806001600160a01b031663e2a4853a84604051602001612398906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016123c8929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612409929190918252602082015260400190565b6020604051808303815f875af1158015612425573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124499190614edb565b5050505050565b604051630cecd96d60e21b81526001600160a01b0388811660048301528781166024830152868116604483015284811660648301526084820184905260a4820183905260c482018690528916906333b365b49060e4015b5f604051808303815f87803b1580156124be575f5ffd5b505af11580156124d0573d5f5f3e3d5ffd5b505050505050505050505050565b805f036124fe57604051635186591160e01b815260040160405180910390fd5b60208201516040516370a0823160e01b81526001600160a01b0385811660048301525f91908316906370a0823190602401602060405180830381865afa15801561254a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061256e9190614edb565b905080831115612449576040516302fa826960e51b8152600481018290526024810184905260440161047a565b5f5f5f5f6125a7614d51565b6125b0886144f8565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156125fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061261e9190614edb565b816020018181525050612638875f5f846101400151613c25565b50608084015250604082015261014081015161265a9088906001905f90613c25565b5060a084015250606082015260208101515f03612683575f5f5f5f9450945094509450506127e7565b6126968682604001518360200151613d3c565b610100820152606081015160208201516126b1918891613d3c565b816101200181815250506126f16040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613fdc565b612729604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613fdc565b61276160405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613fdc565b6127946040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613fdc565b6127c76040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613fdc565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b8184111561281b57604051631fc107c160e01b8152600481018590526024810183905260440161047a565b80831115610cb057604051630e793baf60e01b8152600481018490526024810182905260440161047a565b604051631495739160e11b81526001600160a01b0388811660048301528781166024830152868116604483015260648201869052848116608483015260a4820184905260c4820183905289169063292ae7229060e4016124a7565b5f816001600160a01b0316836001600160a01b0316106128c25781836128c5565b82825b60405191945092506128f2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080016040516020818303038152906040528051906020012090505b92915050565b612951614d1d565b8261295a614d1d565b816001600160a01b03166391d4403c604051602001612996906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156129ea573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a0e9190614ef2565b612a1b5791506129439050565b816001600160a01b03166321f8a72185604051602001612a5b906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a8b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612abf91815260200190565b602060405180830381865afa158015612ada573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612afe9190615103565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001612b7c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612bb091815260200190565b602060405180830381865afa158015612bcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bef9190614edb565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001612c45906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c75929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612ca991815260200190565b602060405180830381865afa158015612cc4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce89190614edb565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001612d43906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d73929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612da791815260200190565b602060405180830381865afa158015612dc2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612de69190614edb565b815151606001526040516001600160a01b0383169063bd02d0f5908690612e0f9060200161511e565b60405160208183030381529060405280519060200120604051602001612e3f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612e7391815260200190565b602060405180830381865afa158015612e8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612eb29190614edb565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001612f0e906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f3e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612f7291815260200190565b602060405180830381865afa158015612f8d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fb19190614edb565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161302e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161306291815260200190565b602060405180830381865afa15801561307d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130a19190614edb565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613116929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161314a91815260200190565b602060405180830381865afa158015613165573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131899190615103565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161323391815260200190565b602060405180830381865afa15801561324e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132729190614edb565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016132c990602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016132f9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161332d91815260200190565b602060405180830381865afa158015613348573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336c9190614edb565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016133c890602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016133f8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161342c91815260200190565b602060405180830381865afa158015613447573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061346b9190614edb565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161349b9061515f565b604051602081830303815290604052805190602001206040516020016134cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134ff91815260200190565b602060405180830381865afa15801561351a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061353e9190614edb565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161359b90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016135cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016135ff91815260200190565b602060405180830381865afa15801561361a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061363e9190614edb565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161369790602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016136c7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016136fb91815260200190565b602060405180830381865afa158015613716573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061373a9190614edb565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161378890602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016137b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016137ec91815260200190565b602060405180830381865afa158015613807573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061382b9190615103565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613899906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016138c9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016138fd91815260200190565b602060405180830381865afa158015613918573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061393c9190615103565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161399f906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016139cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a0391815260200190565b602060405180830381865afa158015613a1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a429190614edb565b60608201526040516001600160a01b0383169063bd02d0f5908690613a9b906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613acb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aff91815260200190565b602060405180830381865afa158015613b1a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b3e9190614edb565b6080820152949350505050565b60208201516001600160a01b031661026457604051637357d91f60e01b81526004810182905260240161047a565b60808101514290819003613b8b575050565b81515160a0015115613be15781515f90613bb490825b6020020151604001518460800151614549565b8351909150613bd89082905f5b60200201516020015161457d90919063ffffffff16565b83515160200152505b81516020015160a0015115613c1d5781515f90613bff906001613ba1565b8351909150613c119082906001613bc1565b83516020908101510152505b608090910152565b5f5f5f5f5f885f01518860ff1660028110613c4257613c426150ef565b602002015190505f613c548a8a6145be565b9050805f03613c70575f5f5f5f95509550955095505050613d03565b5f613c7f838c608001516146ac565b90505f613c8c828a61457d565b90505f8915613cb157818410613cab57613ca684836146dc565b613cb3565b5f613cb3565b5f5b90505f613cc0858d61457d565b90505f8c15613ce557848210613cdf57613cda82866146dc565b613ce7565b5f613ce7565b5f5b9050613cf385876150b6565b9a50949850909650929450505050505b945094509450949050565b5f60ff60581b1960585f1960ff851601613d2e575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03613d7057838281613d6657613d6661505c565b0492505050613df5565b808411613d905760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f8115676765c793fa10079d601b1b60028404190484111715613e1d575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b61083060405180604001604052806002815260200161257360f01b81525082614731565b5f826001600160a01b031663bd02d0f5613e7484614776565b6040518263ffffffff1660e01b8152600401613e9291815260200190565b602060405180830381865afa158015613ead573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613df59190614edb565b826001600160a01b031663e2a4853a613ee984614776565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015613f31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cb09190614edb565b826001600160a01b031663e2a4853a613f6d84614829565b836040518363ffffffff1660e01b8152600401613f15929190918252602082015260400190565b826001600160a01b031663e2a4853a613ee98461487d565b826001600160a01b031663e2a4853a613f6d846148de565b826001600160a01b031663e2a4853a613f6d84614944565b610264604051806040016040528060068152602001652573202d257360d01b815250838361498d565b5f826001600160a01b031663bd02d0f5613e7484614829565b5f826001600160a01b031663bd02d0f5613e748461487d565b5f816001600160a01b031663bd02d0f5604051602001614076906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140aa91815260200190565b602060405180830381865afa1580156140c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129439190614edb565b5f826001600160a01b031663bd02d0f5613e74846148de565b5f61410b614d51565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561414b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061416f9190614edb565b6020820152614180865f8080613c25565b50505060c08201526141958660015f80613c25565b50505060e082015282156141d057848160c0018181516141b591906150c9565b90525060e0810180518591906141cc9083906150c9565b9052505b80602001515f03614200576141f96103e86141f36141ee88886149d4565b614a3a565b906146dc565b8152614252565b60c08101511580614213575060e0810151155b15614221575f915050614256565b61424f6142378683602001518460c00151613d3c565b61424a8684602001518560e00151613d3c565b614b1a565b81525b5190505b949350505050565b5f5f5f61426f84606001515f613d0e565b90505f61427b86614b2f565b90505f61429e8261428d85600a615051565b676765c793fa10079d601b1b613d3c565b90505f6142ad875f5f5f613c25565b50939a91995090975050505050505050565b5f6033826142cd575f6142d0565b60015b60ff16901b660800000000000019841617905092915050565b5f5f61431e6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614328845f614b80565b60208301528152606084015161433e905f613d0e565b60608201819052815161436391676765c793fa10079d601b1b90610b2190600a615051565b604082015260208101515f0361437e575f608082015261441e565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156143f4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144189190614edb565b60808201525b614429846001614b80565b602083018190529082525f03614444575f60a08201526144e4565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa1580156144ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144de9190614edb565b60a08201525b80608001518160a001519250925050915091565b5f816001600160a01b031663bd02d0f5604051602001614076906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f8061455583426150c9565b61455f908561509f565b6301e133809004905061425681676765c793fa10079d601b1b6150b6565b5f81156b019d971e4fe8401e74000000198390048411151761459d575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f835f01518360ff16600281106145d8576145d86150ef565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614631573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146559190614edb565b9050805f03614668575f92505050612943565b606082015160c083015161467c90826150b6565b82106146a05760c083015161469182846150c9565b61469b91906150c9565b6146a2565b5f5b9695505050505050565b5f8260a001515f036146bf57505f612943565b5f6146ca8484614bc6565b60a0850151909150614256908261457d565b5f826146e883826150c9565b91508111156129435760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161047a565b61026482826040516024016147479291906151a0565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614c09565b80515f90819061479790825b602002015151845160016020020151516128a1565b9050806040516020016147db906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161480b929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906148399082614782565b9050806040516020016147db906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f90819061488d9082614782565b9050806040516020016147db9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906148ee9082614782565b9050806040516020016147db9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906149549082614782565b9050806040516020016147db90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b610a5a8383836040516024016149a5939291906151cd565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614c09565b5f8115806149f7575082826149e9818361509f565b92506149f590836150dc565b145b6129435760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161047a565b5f815f03614a4957505f919050565b5f6001614a5584614c12565b901c6001901b90506001818481614a6e57614a6e61505c565b048201901c90506001818481614a8657614a8661505c565b048201901c90506001818481614a9e57614a9e61505c565b048201901c90506001818481614ab657614ab661505c565b048201901c90506001818481614ace57614ace61505c565b048201901c90506001818481614ae657614ae661505c565b048201901c90506001818481614afe57614afe61505c565b048201901c9050613df581828581614b1857614b1861505c565b045b5f818310614b285781613df5565b5090919050565b5f816001600160a01b031663bd02d0f56040516020016140769060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f5f5f614bad855f01518560ff1660028110614b9e57614b9e6150ef565b602002015186608001516146ac565b90505f614bba86866145be565b96919550909350505050565b5f428203614bd957506020820151612943565b5f614be8846040015184614549565b9050614c0184602001518261457d90919063ffffffff16565b915050612943565b61083081614ca5565b5f80608083901c15614c2657608092831c92015b604083901c15614c3857604092831c92015b602083901c15614c4a57602092831c92015b601083901c15614c5c57601092831c92015b600883901c15614c6e57600892831c92015b600483901c15614c8057600492831c92015b600283901c15614c9257600292831c92015b600183901c156129435760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101400160405280614cd8614d1d565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614d30614da0565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b614df16040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081614daf5790505090565b6001600160a01b0381168114610830575f5ffd5b5f5f8284036080811215614e2d575f5ffd5b8335614e3881614e07565b92506060601f1982011215614e4b575f5ffd5b506020830190509250929050565b5f5f82840360a0811215614e6b575f5ffd5b8335614e7681614e07565b92506080601f1982011215614e4b575f5ffd5b5f60208284031215614e99575f5ffd5b8135613df581614e07565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215614eeb575f5ffd5b5051919050565b5f60208284031215614f02575f5ffd5b81518015158114613df5575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061425690830184614f11565b634e487b7160e01b5f52601160045260245ffd5b6001815b6001841115610a9a57808504811115614f9557614f95614f62565b6001841615614fa357908102905b60019390931c928002614f7a565b5f82614fbf57506001612943565b81614fcb57505f612943565b8160018114614fe15760028114614feb57615007565b6001915050612943565b60ff841115614ffc57614ffc614f62565b50506001821b612943565b5060208310610133831016604e8410600b841016171561502a575081810a612943565b6150365f198484614f76565b805f190482111561504957615049614f62565b029392505050565b5f613df58383614fb1565b634e487b7160e01b5f52601260045260245ffd5b5f8261507e5761507e61505c565b500690565b63ffffffff828116828216039081111561294357612943614f62565b808202811582820484141761294357612943614f62565b8082018082111561294357612943614f62565b8181038181111561294357612943614f62565b5f826150ea576150ea61505c565b500490565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215615113575f5ffd5b8151613df581614e07565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6151b26040830185614f11565b82810360208401526151c48185614f11565b95945050505050565b606081525f6151df6060830186614f11565b82810360208401526151f18186614f11565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da2646970667358221220256a93aab88bf78d1151d4a48875e9e5112742a25ab4812279baafa54c11f00964736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80c^\xCDD\xE8\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c}#|\x99\x14a\0\xD8W\x80c\x9F\xF7\x8C0\x14a\0\xEBW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aN\x1BV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\0\xE66`\x04aNYV[a\x02hV[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x1Aa\x03|V[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x05:V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\x05\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x02&\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x02D``\x85\x01`@\x86\x01aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x05\xE8V[Pa\x02da\x07zV[PPV[a\x02pa\x03|V[a\x02\xA1`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80`\xC0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x03\x1D\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03>\x91\x90aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`@\x80\x85\x015` \x83\x01R\x01a\x03e`\x80\x85\x01``\x86\x01aN\x89V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02[\x83\x82a\x083V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xBB\x90aN\xA4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90aN\xDBV[\x90P\x80\x15a\x04\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xC1\x90aN\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90aN\xDBV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90aN\xF2V[a\x02dW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04z\x92\x91\x90aO?V[__a\x06\0\x83_\x01Q\x84`@\x01Q\x85``\x01Qa\n_V[\x91P\x91P_a\x06\x0E\x83a\n\xA2V[\x90Pa\x06\x1E\x84_\x01Q\x84\x83a\x0B\\V[` \x83\x01Q`@\x85\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x91\x83\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90aN\xDBV[``\x87\x01Q`@Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90aN\xDBV[\x90Pa\x07\x13\x82\x82a\x10bV[_a\x07%\x87\x85\x85\x85\x8C`\x80\x01Qa\x10\x8BV[\x90Pa\x074\x88_\x01Q\x88a\x11\xCCV[a\x07B\x88` \x01Q\x88a\x12\x10V[\x87Qa\x07O\x90\x87\x89a\x124V[a\x07o\x88` \x01Q\x8A\x8A`@\x01Q\x8B``\x01Q\x85\x8D`\x80\x01Q\x89\x89a$PV[PPPPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x07\xB8\x90aN\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x080\x91\x90aN\xDBV[PV[a\x08;aL\xC4V[a\x08Q\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\n_V[` \x83\x01R\x80\x82R`\x80\x83\x01Qa\x08i\x91\x85\x91a$\xDEV[\x81Q\x81Q`\x80\x84\x01Qa\x08}\x92\x91\x90a%\x9BV[`\xC0\x85\x01\x81\x90R`\xA0\x85\x01\x82\x90Ra\x01 \x85\x01\x83\x90Ra\x01\0\x85\x01\x84\x90Ra\x08\xA7\x93\x92\x91\x90a'\xF0V[\x80Q` \x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R`\x80\x85\x01Q\x90Qc'p\xA7\xEB`\xE2\x1B\x81R\x92\x86\x16`\x04\x84\x01R`$\x83\x01R\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xFFW__\xFD[PZ\xF1\x15\x80\x15a\t\x11W=__>=_\xFD[PPPP`@\x81\x81\x01Q\x83\x82\x01Q`\xA0\x85\x01Qa\x01\0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\txW__\xFD[PZ\xF1\x15\x80\x15a\t\x8AW=__>=_\xFD[PPPP`@\x81\x81\x01Q``\x84\x01Q`\xA0\x85\x01Qa\x01 \x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xF2W__\xFD[PZ\xF1\x15\x80\x15a\n\x04W=__>=_\xFD[PPP` \x83\x01Q\x82Qa\n\x18\x92Pa\x12\x10V[\x81Q` \x82\x01Q\x82Qa\n,\x92\x91\x90a\x124V[a\nZ\x82` \x01Q\x84\x84`@\x01Q\x85``\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x87a\x01\0\x01Q\x88a\x01 \x01Qa(FV[PPPV[a\ngaM\x1DV[__a\ns\x85\x85a(\xA1V[\x90P_a\n\x80\x87\x83a)IV[\x90Pa\n\x8C\x81\x83a;KV[a\n\x95\x81a;yV[\x92P\x90P[\x93P\x93\x91PPV[__a\n\xB0\x83___a<%V[PPP\x90P_a\n\xC3\x84`\x01__a<%V[PPP\x90P\x81_\x14\x80a\n\xD4WP\x80\x15[\x15a\n\xE2WP_\x93\x92PPPV[_a\n\xF1\x85``\x01Q_a=\x0EV[\x90P_a\x0B\x03\x86``\x01Q`\x01a=\x0EV[\x90P_a\x0B&\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B!\x86`\naPQV[a=<V[\x90P_a\x0BD\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x0B!\x86`\naPQV[\x90Pa\x0BP\x82\x82a=\xFCV[\x98\x97PPPPPPPPV[\x80_\x03a\x0BhWPPPV[a\x0B\x89`@Q\x80``\x01`@R\x80`%\x81R` \x01aR\x03`%\x919a>7V[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x0B\xE3\x84\x84a>[V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x0B\xFAd\x01\0\0\0\0BaPpV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x0C\xB6Wa\x0C\x1F\x84\x84\x83` \x01Qa>\xD1V[a\x0C*\x84\x84_a?UV[a\x0C9\x84\x84\x83` \x01Qa?\x94V[a\x0CD\x84\x84_a?\xACV[a\x0CO\x84\x84\x84a?\xC4V[a\x0C\x89`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\x0C\xB0`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83a?\xDCV[PPPPV[\x80Q` \x82\x01Qa\x0C\xC7\x91\x90aP\x83V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\r\x03\x92\x16a?\xDCV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x0C\xB0Wa\r\x1D\x84\x84a@\x05V[``\x82\x01R`@\x81\x01Qa\r7\x90c\xFF\xFF\xFF\xFF\x16\x83aP\x9FV[\x81``\x01Qa\rF\x91\x90aP\xB6V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\rq\x90\x83a?\xDCV[a\r\xA8`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\r\xE1`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01Qa?\xDCV[a\x0E\x16`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xDCV[a\x0E%\x84\x84\x83`\x80\x01Qa?UV[a\x0E4\x84\x84\x83` \x01Qa>\xD1V[a\x0E>\x84\x84a@\x1EV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x0EZ\x91\x90aP\x83V[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x0En\x84a@7V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x0E\xA6\x92\x16a?\xDCV[a\x0E\xEF`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[a\x0F.`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16a?\xDCV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xB0Wa\x0FT\x84\x84a@\xE9V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x0Fx\x91aP\xC9V[a\x0F\x82\x91\x90aP\xDCV[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x0F\xCC\x91\x90a?\xDCV[a\x10\x01`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01Qa?\xDCV[a\x104`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xDCV[a\x10C\x84\x84\x83` \x01Qa?\x94V[a\x10R\x84\x84\x83`\x80\x01Qa?\xACV[a\x0C\xB0\x84\x84\x83a\x01 \x01Qa?\xC4V[\x81\x15\x80a\x10mWP\x80\x15[\x15a\x02dW`@Qc\x1A]\xF2\x83`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x10\x9A\x87\x86\x86`\x01aA\x02V[\x90P_\x86`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xFD\x91\x90aN\xDBV[\x90P\x80_\x03a\x11cW`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R_`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11LW__\xFD[PZ\xF1\x15\x80\x15a\x11^W=__>=_\xFD[PPPP[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x88\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xAAW__\xFD[PZ\xF1\x15\x80\x15a\x11\xBCW=__>=_\xFD[P\x93\x9A\x99PPPPPPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02dW__a\x11\xEA\x84\x84aB^V[\x91P\x91P\x81\x81\x10a\x0C\xB0W``\x83\x01Qa\x12\x05\x90`\x01aB\xBFV[``\x84\x01RPPPPV[a\x12\x19\x81aB\xE9V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x12t\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xC4W__\xFD[PZ\xF1\x15\x80\x15a\x12\xD6W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x13\x1A\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13J\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD1\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\x19\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14I\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCA\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15\x11\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC1\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\r\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16=\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xBE\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xDE\x90aQ\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8F\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xDC\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x0C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x8D\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xD6\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x87\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x19\xC8\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x82\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xCA\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BYW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B}\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xC4\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cw\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xC3\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dw\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x97\x90aQ_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EK\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x98\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FL\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x95\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a %W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a I\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a \x87\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xB7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\x01\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!A\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a!\x93\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\"\x0E\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"N\x91\x90aQ\x03V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\x95\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x06\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#F\x91\x90aN\xDBV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x98\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$I\x91\x90aN\xDBV[PPPPPV[`@Qc\x0C\xEC\xD9m`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R\x84\x81\x16`d\x83\x01R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R`\xC4\x82\x01\x86\x90R\x89\x16\x90c3\xB3e\xB4\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a$\xBEW__\xFD[PZ\xF1\x15\x80\x15a$\xD0W=__>=_\xFD[PPPPPPPPPPPPV[\x80_\x03a$\xFEW`@QcQ\x86Y\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R_\x91\x90\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%n\x91\x90aN\xDBV[\x90P\x80\x83\x11\x15a$IW`@Qc\x02\xFA\x82i`\xE5\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x04zV[____a%\xA7aMQV[a%\xB0\x88aD\xF8V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1E\x91\x90aN\xDBV[\x81` \x01\x81\x81RPPa&8\x87__\x84a\x01@\x01Qa<%V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa&Z\x90\x88\x90`\x01\x90_\x90a<%V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a&\x83W____\x94P\x94P\x94P\x94PPa'\xE7V[a&\x96\x86\x82`@\x01Q\x83` \x01Qa=<V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa&\xB1\x91\x88\x91a=<V[\x81a\x01 \x01\x81\x81RPPa&\xF1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa?\xDCV[a')`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa?\xDCV[a'a`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa?\xDCV[a'\x94`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa?\xDCV[a'\xC7`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa?\xDCV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[\x81\x84\x11\x15a(\x1BW`@Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x83\x90R`D\x01a\x04zV[\x80\x83\x11\x15a\x0C\xB0W`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x04zV[`@Qc\x14\x95s\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x87\x81\x16`$\x83\x01R\x86\x81\x16`D\x83\x01R`d\x82\x01\x86\x90R\x84\x81\x16`\x84\x83\x01R`\xA4\x82\x01\x84\x90R`\xC4\x82\x01\x83\x90R\x89\x16\x90c)*\xE7\"\x90`\xE4\x01a$\xA7V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a(\xC2W\x81\x83a(\xC5V[\x82\x82[`@Q\x91\x94P\x92Pa(\xF2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a)QaM\x1DV[\x82a)ZaM\x1DV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a)\x96\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\x0E\x91\x90aN\xF2V[a*\x1BW\x91Pa)C\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a*[\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\x8B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xBF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xFE\x91\x90aQ\x03V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xB0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xEF\x91\x90aN\xDBV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a,E\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xA9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE8\x91\x90aN\xDBV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a-C\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-s\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xA7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xE6\x91\x90aN\xDBV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a.\x0F\x90` \x01aQ\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.?\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.s\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xB2\x91\x90aN\xDBV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a/\x0E\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/>\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/r\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xB1\x91\x90aN\xDBV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0.\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0b\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xA1\x91\x90aN\xDBV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\x89\x91\x90aQ\x03V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a23\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2NW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2r\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a2\xC9\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xF9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3-\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3HW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3l\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a3\xC8\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4,\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4k\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a4\x9B\x90aQ_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5>\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a5\x9B\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6>\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a6\x97\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\xFB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7:\x91\x90aN\xDBV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a7\x88\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8+\x91\x90aQ\x03V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a8\x99\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xC9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9<\x91\x90aQ\x03V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\x9F\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:B\x91\x90aN\xDBV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a:\x9B\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;>\x91\x90aN\xDBV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02dW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04zV[`\x80\x81\x01QB\x90\x81\x90\x03a;\x8BWPPV[\x81QQ`\xA0\x01Q\x15a;\xE1W\x81Q_\x90a;\xB4\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaEIV[\x83Q\x90\x91Pa;\xD8\x90\x82\x90_[` \x02\x01Q` \x01QaE}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15a<\x1DW\x81Q_\x90a;\xFF\x90`\x01a;\xA1V[\x83Q\x90\x91Pa<\x11\x90\x82\x90`\x01a;\xC1V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a<BWa<BaP\xEFV[` \x02\x01Q\x90P_a<T\x8A\x8AaE\xBEV[\x90P\x80_\x03a<pW____\x95P\x95P\x95P\x95PPPa=\x03V[_a<\x7F\x83\x8C`\x80\x01QaF\xACV[\x90P_a<\x8C\x82\x8AaE}V[\x90P_\x89\x15a<\xB1W\x81\x84\x10a<\xABWa<\xA6\x84\x83aF\xDCV[a<\xB3V[_a<\xB3V[_[\x90P_a<\xC0\x85\x8DaE}V[\x90P_\x8C\x15a<\xE5W\x84\x82\x10a<\xDFWa<\xDA\x82\x86aF\xDCV[a<\xE7V[_a<\xE7V[_[\x90Pa<\xF3\x85\x87aP\xB6V[\x9AP\x94\x98P\x90\x96P\x92\x94PPPPP[\x94P\x94P\x94P\x94\x90PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=.WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a=pW\x83\x82\x81a=fWa=faP\\V[\x04\x92PPPa=\xF5V[\x80\x84\x11a=\x90W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a>\x1DW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[a\x080`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aG1V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aGvV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x92\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xF5\x91\x90aN\xDBV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xE9\x84aGvV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a?1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xB0\x91\x90aN\xDBV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aH)V[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a>\xE9\x84aH}V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aH\xDEV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a?m\x84aIDV[a\x02d`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aI\x8DV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH)V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH}V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xAA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)C\x91\x90aN\xDBV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a>t\x84aH\xDEV[_aA\x0BaMQV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAKW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aAo\x91\x90aN\xDBV[` \x82\x01RaA\x80\x86_\x80\x80a<%V[PPP`\xC0\x82\x01RaA\x95\x86`\x01_\x80a<%V[PPP`\xE0\x82\x01R\x82\x15aA\xD0W\x84\x81`\xC0\x01\x81\x81QaA\xB5\x91\x90aP\xC9V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90aA\xCC\x90\x83\x90aP\xC9V[\x90RP[\x80` \x01Q_\x03aB\0WaA\xF9a\x03\xE8aA\xF3aA\xEE\x88\x88aI\xD4V[aJ:V[\x90aF\xDCV[\x81RaBRV[`\xC0\x81\x01Q\x15\x80aB\x13WP`\xE0\x81\x01Q\x15[\x15aB!W_\x91PPaBVV[aBOaB7\x86\x83` \x01Q\x84`\xC0\x01Qa=<V[aBJ\x86\x84` \x01Q\x85`\xE0\x01Qa=<V[aK\x1AV[\x81R[Q\x90P[\x94\x93PPPPV[___aBo\x84``\x01Q_a=\x0EV[\x90P_aB{\x86aK/V[\x90P_aB\x9E\x82aB\x8D\x85`\naPQV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba=<V[\x90P_aB\xAD\x87___a<%V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aB\xCDW_aB\xD0V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[__aC\x1E`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC(\x84_aK\x80V[` \x83\x01R\x81R``\x84\x01QaC>\x90_a=\x0EV[``\x82\x01\x81\x90R\x81QaCc\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x0B!\x90`\naPQV[`@\x82\x01R` \x81\x01Q_\x03aC~W_`\x80\x82\x01RaD\x1EV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x18\x91\x90aN\xDBV[`\x80\x82\x01R[aD)\x84`\x01aK\x80V[` \x83\x01\x81\x90R\x90\x82R_\x03aDDW_`\xA0\x82\x01RaD\xE4V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xDE\x91\x90aN\xDBV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_\x80aEU\x83BaP\xC9V[aE_\x90\x85aP\x9FV[c\x01\xE13\x80\x90\x04\x90PaBV\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xB6V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aE\x9DW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aE\xD8WaE\xD8aP\xEFV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFU\x91\x90aN\xDBV[\x90P\x80_\x03aFhW_\x92PPPa)CV[``\x82\x01Q`\xC0\x83\x01QaF|\x90\x82aP\xB6V[\x82\x10aF\xA0W`\xC0\x83\x01QaF\x91\x82\x84aP\xC9V[aF\x9B\x91\x90aP\xC9V[aF\xA2V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aF\xBFWP_a)CV[_aF\xCA\x84\x84aK\xC6V[`\xA0\x85\x01Q\x90\x91PaBV\x90\x82aE}V[_\x82aF\xE8\x83\x82aP\xC9V[\x91P\x81\x11\x15a)CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04zV[a\x02d\x82\x82`@Q`$\x01aGG\x92\x91\x90aQ\xA0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaL\tV[\x80Q_\x90\x81\x90aG\x97\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQa(\xA1V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90aH9\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90aH\x8D\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aH\xEE\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90aIT\x90\x82aG\x82V[\x90P\x80`@Q` \x01aG\xDB\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\nZ\x83\x83\x83`@Q`$\x01aI\xA5\x93\x92\x91\x90aQ\xCDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaL\tV[_\x81\x15\x80aI\xF7WP\x82\x82aI\xE9\x81\x83aP\x9FV[\x92PaI\xF5\x90\x83aP\xDCV[\x14[a)CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a\x04zV[_\x81_\x03aJIWP_\x91\x90PV[_`\x01aJU\x84aL\x12V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aJnWaJnaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x86WaJ\x86aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\x9EWaJ\x9EaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xB6WaJ\xB6aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xCEWaJ\xCEaP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xE6WaJ\xE6aP\\V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aJ\xFEWaJ\xFEaP\\V[\x04\x82\x01\x90\x1C\x90Pa=\xF5\x81\x82\x85\x81aK\x18WaK\x18aP\\V[\x04[_\x81\x83\x10aK(W\x81a=\xF5V[P\x90\x91\x90PV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a@v\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aK\xAD\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aK\x9EWaK\x9EaP\xEFV[` \x02\x01Q\x86`\x80\x01QaF\xACV[\x90P_aK\xBA\x86\x86aE\xBEV[\x96\x91\x95P\x90\x93PPPPV[_B\x82\x03aK\xD9WP` \x82\x01Qa)CV[_aK\xE8\x84`@\x01Q\x84aEIV[\x90PaL\x01\x84` \x01Q\x82aE}\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa)CV[a\x080\x81aL\xA5V[_\x80`\x80\x83\x90\x1C\x15aL&W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aL8W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aLJW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aL\\W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aLnW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aL\x80W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aL\x92W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a)CW`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01@\x01`@R\x80aL\xD8aM\x1DV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aM0aM\xA0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aM\xF1`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aM\xAFW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x080W__\xFD[__\x82\x84\x03`\x80\x81\x12\x15aN-W__\xFD[\x835aN8\x81aN\x07V[\x92P```\x1F\x19\x82\x01\x12\x15aNKW__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03`\xA0\x81\x12\x15aNkW__\xFD[\x835aNv\x81aN\x07V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15aNKW__\xFD[_` \x82\x84\x03\x12\x15aN\x99W__\xFD[\x815a=\xF5\x81aN\x07V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aN\xEBW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aO\x02W__\xFD[\x81Q\x80\x15\x15\x81\x14a=\xF5W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aBV\x90\x83\x01\x84aO\x11V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\x9AW\x80\x85\x04\x81\x11\x15aO\x95WaO\x95aObV[`\x01\x84\x16\x15aO\xA3W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aOzV[_\x82aO\xBFWP`\x01a)CV[\x81aO\xCBWP_a)CV[\x81`\x01\x81\x14aO\xE1W`\x02\x81\x14aO\xEBWaP\x07V[`\x01\x91PPa)CV[`\xFF\x84\x11\x15aO\xFCWaO\xFCaObV[PP`\x01\x82\x1Ba)CV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aP*WP\x81\x81\na)CV[aP6_\x19\x84\x84aOvV[\x80_\x19\x04\x82\x11\x15aPIWaPIaObV[\x02\x93\x92PPPV[_a=\xF5\x83\x83aO\xB1V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aP~WaP~aP\\V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a)CWa)CaObV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a)CWa)CaObV[\x80\x82\x01\x80\x82\x11\x15a)CWa)CaObV[\x81\x81\x03\x81\x81\x11\x15a)CWa)CaObV[_\x82aP\xEAWaP\xEAaP\\V[P\x04\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aQ\x13W__\xFD[\x81Qa=\xF5\x81aN\x07V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_aQ\xB2`@\x83\x01\x85aO\x11V[\x82\x81\x03` \x84\x01RaQ\xC4\x81\x85aO\x11V[\x95\x94PPPPPV[``\x81R_aQ\xDF``\x83\x01\x86aO\x11V[\x82\x81\x03` \x84\x01RaQ\xF1\x81\x86aO\x11V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 %j\x93\xAA\xB8\x8B\xF7\x8D\x11Q\xD4\xA4\x88u\xE9\xE5\x11'B\xA2Z\xB4\x81\"y\xBA\xAF\xA5L\x11\xF0\tdsolcC\0\x08\x1C\x003",
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
