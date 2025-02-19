///Module containing a contract's types and functions.
/**

```solidity
library BorrowUtils {
    struct BorrowParams { uint256 positionId; uint8 tokenIndex; uint256 borrowAmount; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod BorrowUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct BorrowParams { uint256 positionId; uint8 tokenIndex; uint256 borrowAmount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BorrowParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub borrowAmount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u8,
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
        impl ::core::convert::From<BorrowParams> for UnderlyingRustTuple<'_> {
            fn from(value: BorrowParams) -> Self {
                (value.positionId, value.tokenIndex, value.borrowAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BorrowParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    tokenIndex: tuple.1,
                    borrowAmount: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BorrowParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BorrowParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowAmount),
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
        impl alloy_sol_types::SolType for BorrowParams {
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
        impl alloy_sol_types::SolStruct for BorrowParams {
            const NAME: &'static str = "BorrowParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BorrowParams(uint256 positionId,uint8 tokenIndex,uint256 borrowAmount)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowAmount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BorrowParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowAmount,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowAmount,
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
    /**Creates a new wrapper around an on-chain [`BorrowUtils`](self) contract instance.

See the [wrapper's documentation](`BorrowUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BorrowUtilsInstance<T, P, N> {
        BorrowUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`BorrowUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BorrowUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BorrowUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BorrowUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BorrowUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BorrowUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BorrowUtils`](self) contract instance.

See the [wrapper's documentation](`BorrowUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BorrowUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BorrowUtilsInstance<T, P, N> {
            BorrowUtilsInstance {
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
    > BorrowUtilsInstance<T, P, N> {
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
    > BorrowUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library CloseUtils {
    struct CloseParams { uint256 positionId; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod CloseUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct CloseParams { uint256 positionId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CloseParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<CloseParams> for UnderlyingRustTuple<'_> {
            fn from(value: CloseParams) -> Self {
                (value.positionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CloseParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { positionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CloseParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CloseParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
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
        impl alloy_sol_types::SolType for CloseParams {
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
        impl alloy_sol_types::SolStruct for CloseParams {
            const NAME: &'static str = "CloseParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CloseParams(uint256 positionId)",
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CloseParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
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
    /**Creates a new wrapper around an on-chain [`CloseUtils`](self) contract instance.

See the [wrapper's documentation](`CloseUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CloseUtilsInstance<T, P, N> {
        CloseUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`CloseUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`CloseUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CloseUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for CloseUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CloseUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > CloseUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`CloseUtils`](self) contract instance.

See the [wrapper's documentation](`CloseUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> CloseUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CloseUtilsInstance<T, P, N> {
            CloseUtilsInstance {
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
    > CloseUtilsInstance<T, P, N> {
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
    > CloseUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library DepositUtils {
    struct DepositParams { uint256 positionId; address token0; address token1; uint8 tokenIndex; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod DepositUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct DepositParams { uint256 positionId; address token0; address token1; uint8 tokenIndex; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DepositParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub tokenIndex: u8,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<8>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            u8,
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
        impl ::core::convert::From<DepositParams> for UnderlyingRustTuple<'_> {
            fn from(value: DepositParams) -> Self {
                (value.positionId, value.token0, value.token1, value.tokenIndex)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DepositParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    token0: tuple.1,
                    token1: tuple.2,
                    tokenIndex: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DepositParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DepositParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
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
        impl alloy_sol_types::SolType for DepositParams {
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
        impl alloy_sol_types::SolStruct for DepositParams {
            const NAME: &'static str = "DepositParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DepositParams(uint256 positionId,address token0,address token1,uint8 tokenIndex)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token0,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndex)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DepositParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token0,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndex,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
                    out,
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndex,
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
    /**Creates a new wrapper around an on-chain [`DepositUtils`](self) contract instance.

See the [wrapper's documentation](`DepositUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DepositUtilsInstance<T, P, N> {
        DepositUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`DepositUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DepositUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DepositUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DepositUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DepositUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DepositUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DepositUtils`](self) contract instance.

See the [wrapper's documentation](`DepositUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> DepositUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DepositUtilsInstance<T, P, N> {
            DepositUtilsInstance {
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
    > DepositUtilsInstance<T, P, N> {
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
    > DepositUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library LiquidationUtils {
    struct LiquidationParams { address account; uint256 positionId; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod LiquidationUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct LiquidationParams { address account; uint256 positionId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LiquidationParams {
        pub account: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<LiquidationParams> for UnderlyingRustTuple<'_> {
            fn from(value: LiquidationParams) -> Self {
                (value.account, value.positionId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LiquidationParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    account: tuple.0,
                    positionId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for LiquidationParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for LiquidationParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
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
        impl alloy_sol_types::SolType for LiquidationParams {
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
        impl alloy_sol_types::SolStruct for LiquidationParams {
            const NAME: &'static str = "LiquidationParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "LiquidationParams(address account,uint256 positionId)",
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
                            &self.account,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for LiquidationParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.account,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
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
                    &rust.account,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
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
    /**Creates a new wrapper around an on-chain [`LiquidationUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidationUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> LiquidationUtilsInstance<T, P, N> {
        LiquidationUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`LiquidationUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`LiquidationUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct LiquidationUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for LiquidationUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("LiquidationUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > LiquidationUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`LiquidationUtils`](self) contract instance.

See the [wrapper's documentation](`LiquidationUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> LiquidationUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> LiquidationUtilsInstance<T, P, N> {
            LiquidationUtilsInstance {
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
    > LiquidationUtilsInstance<T, P, N> {
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
    > LiquidationUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library RepayUtils {
    struct RepayParams { uint256 positionId; uint8 tokenIndex; uint256 repayAmount; uint8 repayOption; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod RepayUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct RepayParams { uint256 positionId; uint8 tokenIndex; uint256 repayAmount; uint8 repayOption; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RepayParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub repayAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub repayOption: u8,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<8>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
            u8,
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
        impl ::core::convert::From<RepayParams> for UnderlyingRustTuple<'_> {
            fn from(value: RepayParams) -> Self {
                (
                    value.positionId,
                    value.tokenIndex,
                    value.repayAmount,
                    value.repayOption,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RepayParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    tokenIndex: tuple.1,
                    repayAmount: tuple.2,
                    repayOption: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RepayParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RepayParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.repayAmount),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.repayOption),
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
        impl alloy_sol_types::SolType for RepayParams {
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
        impl alloy_sol_types::SolStruct for RepayParams {
            const NAME: &'static str = "RepayParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RepayParams(uint256 positionId,uint8 tokenIndex,uint256 repayAmount,uint8 repayOption)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.repayAmount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.repayOption)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RepayParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.repayAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.repayOption,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.repayAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.repayOption,
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
    /**Creates a new wrapper around an on-chain [`RepayUtils`](self) contract instance.

See the [wrapper's documentation](`RepayUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RepayUtilsInstance<T, P, N> {
        RepayUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`RepayUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`RepayUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RepayUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RepayUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RepayUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RepayUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`RepayUtils`](self) contract instance.

See the [wrapper's documentation](`RepayUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> RepayUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RepayUtilsInstance<T, P, N> {
            RepayUtilsInstance {
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
    > RepayUtilsInstance<T, P, N> {
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
    > RepayUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library SwapUtils {
    struct SwapInPositionParams { uint256 positionId; uint256 amount0In; uint256 amount1In; uint256 amount0Out; uint256 amount1Out; }
    struct SwapParams { address token0; address token1; uint256 amount0In; uint256 amount1In; uint256 amount0Out; uint256 amount1Out; address to; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod SwapUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SwapInPositionParams { uint256 positionId; uint256 amount0In; uint256 amount1In; uint256 amount0Out; uint256 amount1Out; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SwapInPositionParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub amount0In: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1In: alloy::sol_types::private::primitives::aliases::U256,
        pub amount0Out: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1Out: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<SwapInPositionParams> for UnderlyingRustTuple<'_> {
            fn from(value: SwapInPositionParams) -> Self {
                (
                    value.positionId,
                    value.amount0In,
                    value.amount1In,
                    value.amount0Out,
                    value.amount1Out,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapInPositionParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    amount0In: tuple.1,
                    amount1In: tuple.2,
                    amount0Out: tuple.3,
                    amount1Out: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SwapInPositionParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SwapInPositionParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0In),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1In),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0Out),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1Out),
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
        impl alloy_sol_types::SolType for SwapInPositionParams {
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
        impl alloy_sol_types::SolStruct for SwapInPositionParams {
            const NAME: &'static str = "SwapInPositionParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SwapInPositionParams(uint256 positionId,uint256 amount0In,uint256 amount1In,uint256 amount0Out,uint256 amount1Out)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount0In)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount1In)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount0Out)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount1Out)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SwapInPositionParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount0In,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount1In,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount0Out,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount1Out,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount0In,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount1In,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount0Out,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount1Out,
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
struct SwapParams { address token0; address token1; uint256 amount0In; uint256 amount1In; uint256 amount0Out; uint256 amount1Out; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SwapParams {
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub amount0In: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1In: alloy::sol_types::private::primitives::aliases::U256,
        pub amount0Out: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1Out: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<SwapParams> for UnderlyingRustTuple<'_> {
            fn from(value: SwapParams) -> Self {
                (
                    value.token0,
                    value.token1,
                    value.amount0In,
                    value.amount1In,
                    value.amount0Out,
                    value.amount1Out,
                    value.to,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token0: tuple.0,
                    token1: tuple.1,
                    amount0In: tuple.2,
                    amount1In: tuple.3,
                    amount0Out: tuple.4,
                    amount1Out: tuple.5,
                    to: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SwapParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SwapParams {
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0In),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1In),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0Out),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1Out),
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
        impl alloy_sol_types::SolType for SwapParams {
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
        impl alloy_sol_types::SolStruct for SwapParams {
            const NAME: &'static str = "SwapParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SwapParams(address token0,address token1,uint256 amount0In,uint256 amount1In,uint256 amount0Out,uint256 amount1Out,address to)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount0In)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount1In)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount0Out)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount1Out)
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
        impl alloy_sol_types::EventTopic for SwapParams {
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
                        &rust.amount0In,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount1In,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount0Out,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount1Out,
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
                    &rust.amount0In,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount1In,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount0Out,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount1Out,
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
    /**Creates a new wrapper around an on-chain [`SwapUtils`](self) contract instance.

See the [wrapper's documentation](`SwapUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SwapUtilsInstance<T, P, N> {
        SwapUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`SwapUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SwapUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SwapUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SwapUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SwapUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SwapUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SwapUtils`](self) contract instance.

See the [wrapper's documentation](`SwapUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> SwapUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SwapUtilsInstance<T, P, N> {
            SwapUtilsInstance {
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
    > SwapUtilsInstance<T, P, N> {
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
    > SwapUtilsInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library WithdrawUtils {
    struct WithdrawParams { uint256 positionId; uint8 tokenIndex; uint256 withdrawAmount; address to; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod WithdrawUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct WithdrawParams { uint256 positionId; uint8 tokenIndex; uint256 withdrawAmount; address to; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u8,
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
        impl ::core::convert::From<WithdrawParams> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawParams) -> Self {
                (value.positionId, value.tokenIndex, value.withdrawAmount, value.to)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    tokenIndex: tuple.1,
                    withdrawAmount: tuple.2,
                    to: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for WithdrawParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for WithdrawParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawAmount),
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
        impl alloy_sol_types::SolType for WithdrawParams {
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
        impl alloy_sol_types::SolStruct for WithdrawParams {
            const NAME: &'static str = "WithdrawParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "WithdrawParams(uint256 positionId,uint8 tokenIndex,uint256 withdrawAmount,address to)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawAmount,
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
        impl alloy_sol_types::EventTopic for WithdrawParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawAmount,
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.positionId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawAmount,
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
    /**Creates a new wrapper around an on-chain [`WithdrawUtils`](self) contract instance.

See the [wrapper's documentation](`WithdrawUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WithdrawUtilsInstance<T, P, N> {
        WithdrawUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`WithdrawUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`WithdrawUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WithdrawUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WithdrawUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WithdrawUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WithdrawUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`WithdrawUtils`](self) contract instance.

See the [wrapper's documentation](`WithdrawUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> WithdrawUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WithdrawUtilsInstance<T, P, N> {
            WithdrawUtilsInstance {
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
    > WithdrawUtilsInstance<T, P, N> {
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
    > WithdrawUtilsInstance<T, P, N> {
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
library BorrowUtils {
    struct BorrowParams {
        uint256 positionId;
        uint8 tokenIndex;
        uint256 borrowAmount;
    }
}

library CloseUtils {
    struct CloseParams {
        uint256 positionId;
    }
}

library DepositUtils {
    struct DepositParams {
        uint256 positionId;
        address token0;
        address token1;
        uint8 tokenIndex;
    }
}

library LiquidationUtils {
    struct LiquidationParams {
        address account;
        uint256 positionId;
    }
}

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

library RepayUtils {
    struct RepayParams {
        uint256 positionId;
        uint8 tokenIndex;
        uint256 repayAmount;
        uint8 repayOption;
    }
}

library SwapUtils {
    struct SwapInPositionParams {
        uint256 positionId;
        uint256 amount0In;
        uint256 amount1In;
        uint256 amount0Out;
        uint256 amount1Out;
    }
    struct SwapParams {
        address token0;
        address token1;
        uint256 amount0In;
        uint256 amount1In;
        uint256 amount0Out;
        uint256 amount1Out;
        address to;
    }
}

library WithdrawUtils {
    struct WithdrawParams {
        uint256 positionId;
        uint8 tokenIndex;
        uint256 withdrawAmount;
        address to;
    }
}

interface ExchangeRouter {
    error EmptyReceiver();
    error ReentrancyGuardReentrantCall();
    error Unauthorized(address msgSender, string role);

    constructor(address _router, address _roleStore, address _dataStore, address _liquidityHandler, address _swapHandler, address _depositHandler, address _borrowHandler, address _repayHandler, address _withdrawHandler, address _liquidationHandler, address _closeHandler);

    function borrowHandler() external view returns (address);
    function closeHandler() external view returns (address);
    function dataStore() external view returns (address);
    function depositHandler() external view returns (address);
    function executeAdd(LiquidityUtils.AddParams memory params) external payable;
    function executeBorrow(BorrowUtils.BorrowParams memory params) external payable;
    function executeClose(CloseUtils.CloseParams memory params) external payable;
    function executeDeposit(DepositUtils.DepositParams memory params) external payable;
    function executeLiquidation(LiquidationUtils.LiquidationParams memory params) external payable;
    function executeLiquidationBatch(LiquidationUtils.LiquidationParams[] memory params) external payable;
    function executeRemove(LiquidityUtils.RemoveParams memory params) external payable;
    function executeRepay(RepayUtils.RepayParams memory params) external payable;
    function executeSwap(SwapUtils.SwapParams memory params) external payable;
    function executeSwapInPosition(SwapUtils.SwapInPositionParams memory params) external payable;
    function executeWithdraw(WithdrawUtils.WithdrawParams memory params) external payable;
    function liquidationHandler() external view returns (address);
    function liquidityHandler() external view returns (address);
    function multicall(bytes[] memory data) external payable returns (bytes[] memory results);
    function repayHandler() external view returns (address);
    function roleStore() external view returns (address);
    function router() external view returns (address);
    function sendTokens(address token, address receiver, uint256 amount) external payable;
    function swapHandler() external view returns (address);
    function withdrawHandler() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_router",
        "type": "address",
        "internalType": "contract Router"
      },
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
        "name": "_liquidityHandler",
        "type": "address",
        "internalType": "contract ILiquidityHandler"
      },
      {
        "name": "_swapHandler",
        "type": "address",
        "internalType": "contract ISwapHandler"
      },
      {
        "name": "_depositHandler",
        "type": "address",
        "internalType": "contract IDepositHandler"
      },
      {
        "name": "_borrowHandler",
        "type": "address",
        "internalType": "contract IBorrowHandler"
      },
      {
        "name": "_repayHandler",
        "type": "address",
        "internalType": "contract IRepayHandler"
      },
      {
        "name": "_withdrawHandler",
        "type": "address",
        "internalType": "contract IWithdrawHandler"
      },
      {
        "name": "_liquidationHandler",
        "type": "address",
        "internalType": "contract ILiquidationHandler"
      },
      {
        "name": "_closeHandler",
        "type": "address",
        "internalType": "contract ICloseHandler"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "borrowHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBorrowHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "closeHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ICloseHandler"
      }
    ],
    "stateMutability": "view"
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
    "name": "depositHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDepositHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "executeAdd",
    "inputs": [
      {
        "name": "params",
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeBorrow",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct BorrowUtils.BorrowParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "tokenIndex",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "borrowAmount",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeClose",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct CloseUtils.CloseParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeDeposit",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct DepositUtils.DepositParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          },
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
            "name": "tokenIndex",
            "type": "uint8",
            "internalType": "uint8"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeLiquidation",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct LiquidationUtils.LiquidationParams",
        "components": [
          {
            "name": "account",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeLiquidationBatch",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct LiquidationUtils.LiquidationParams[]",
        "components": [
          {
            "name": "account",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeRemove",
    "inputs": [
      {
        "name": "params",
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeRepay",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct RepayUtils.RepayParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "tokenIndex",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "repayAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "repayOption",
            "type": "uint8",
            "internalType": "uint8"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeSwap",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct SwapUtils.SwapParams",
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
            "name": "amount0In",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount1In",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount0Out",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount1Out",
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeSwapInPosition",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct SwapUtils.SwapInPositionParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount0In",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount1In",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount0Out",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "amount1Out",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "executeWithdraw",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct WithdrawUtils.WithdrawParams",
        "components": [
          {
            "name": "positionId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "tokenIndex",
            "type": "uint8",
            "internalType": "uint8"
          },
          {
            "name": "withdrawAmount",
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
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "liquidationHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ILiquidationHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "liquidityHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ILiquidityHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "multicall",
    "inputs": [
      {
        "name": "data",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "outputs": [
      {
        "name": "results",
        "type": "bytes[]",
        "internalType": "bytes[]"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "repayHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IRepayHandler"
      }
    ],
    "stateMutability": "view"
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
    "type": "function",
    "name": "router",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract Router"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "sendTokens",
    "inputs": [
      {
        "name": "token",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "swapHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISwapHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "withdrawHandler",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IWithdrawHandler"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "EmptyReceiver",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
    "inputs": []
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
pub mod ExchangeRouter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101e0604052348015610010575f5ffd5b506040516116b23803806116b283398101604081905261002f916100aa565b60015f556001600160a01b03998a1660805299891660a05296881660c05294871660e052928616610100529085166101205284166101405283166101605282166101805281166101a052166101c052610183565b6001600160a01b0381168114610097575f5ffd5b50565b80516100a581610083565b919050565b5f5f5f5f5f5f5f5f5f5f5f6101608c8e0312156100c5575f5ffd5b8b516100d081610083565b60208d0151909b506100e181610083565b60408d0151909a506100f281610083565b60608d015190995061010381610083565b60808d015190985061011481610083565b60a08d015190975061012581610083565b60c08d015190965061013681610083565b945061014460e08d0161009a565b93506101536101008d0161009a565b92506101626101208d0161009a565b91506101716101408d0161009a565b90509295989b509295989b9093969950565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c05161145b6102575f395f81816104370152610a7401525f81816103be015281816107250152610a1601525f818161015c015261090001525f81816102d9015261060201525f818161031f015261065a01525f81816102a601526105aa01525f818161026001528181610552015261098501525f81816103f1015281816104c00152610b7c01525f61021a01525f818161047d0152610af301525f81816101e70152610bf7015261145b5ff3fe608060405260043610610147575f3560e01c8063a82ed4ce116100b3578063d25adeb31161006d578063d25adeb3146103ad578063d9e08811146103e0578063e6d66ac814610413578063ed27afaf14610426578063f3e01a4e14610459578063f887ea401461046c575f5ffd5b8063a82ed4ce1461030e578063ac9650d814610341578063ad0c993414610361578063b184067514610374578063b610321514610387578063c56d4a2a1461039a575f5ffd5b80637b123bc8116101045780637b123bc81461023c5780638a53aaac1461024f578063907e7454146102825780639c8b2cfb146102955780639d451d0c146102c8578063a76718d2146102fb575f5ffd5b8063083473ef1461014b57806321b108cc1461019b57806334f5db7e146101b057806347570cdc146101c35780634a4a7b04146101d6578063660d0d6714610209575b5f5ffd5b348015610156575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6101ae6101a9366004610d9b565b61049f565b005b6101ae6101be366004610dbc565b610531565b6101ae6101d1366004610de5565b610589565b3480156101e1575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b348015610214575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae61024a366004610de5565b6105e1565b34801561025a575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610290366004610d9b565b610639565b3480156102a0575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102d3575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610309366004610dff565b610691565b348015610319575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b61035461034f366004610e70565b6107d0565b6040516101929190610eff565b6101ae61036f366004610de5565b6108df565b6101ae610382366004610f62565b610937565b6101ae610395366004610f73565b6109bc565b6101ae6103a8366004610f84565b610a4d565b3480156103b8575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156103eb575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610421366004610fb0565b610aab565b348015610431575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610467366004610de5565b610b5b565b348015610477575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6104a7610bb3565b604051630bd9a89d60e31b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ecd44e8906104f79084908690600401610fea565b5f604051808303815f87803b15801561050e575f5ffd5b505af1158015610520573d5f5f3e3d5ffd5b505050505061052e60015f55565b50565b610539610bb3565b604051636ce213a160e11b815233906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063d9c42742906104f79084908690600401611044565b610591610bb3565b604051630f4e5ba160e01b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690630f4e5ba1906104f790849086906004016110e4565b6105e9610bb3565b6040516325f8c62b60e21b815233906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906397e318ac906104f79084908690600401611147565b610641610bb3565b60405163134494cd60e31b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639a24a668906104f79084908690600401611194565b610699610bb3565b6107186040516020016106d2906020808252601290820152712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b604082015260600190565b60405160208183030381529060405280519060200120604051806040016040528060128152602001712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b815250610bdb565b335f5b828110156107c1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166383f3d2eb83868685818110610765576107656111d8565b9050604002016040518363ffffffff1660e01b81526004016107889291906111ec565b5f604051808303815f87803b15801561079f575f5ffd5b505af11580156107b1573d5f5f3e3d5ffd5b50506001909201915061071b9050565b50506107cc60015f55565b5050565b60608167ffffffffffffffff8111156107eb576107eb611221565b60405190808252806020026020018201604052801561081e57816020015b60608152602001906001900390816108095790505b5090505f5b828110156108d8575f8030868685818110610840576108406111d8565b90506020028101906108529190611235565b60405161086092919061127f565b5f60405180830381855af49150503d805f8114610898576040519150601f19603f3d011682016040523d82523d5f602084013e61089d565b606091505b5091509150816108b0576108b081610c92565b808484815181106108c3576108c36111d8565b60209081029190910101525050600101610823565b5092915050565b6108e7610bb3565b60405163133dd6f560e11b815233906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063267badea906104f7908490869060040161128e565b61093f610bb3565b604080516352b5de3d60e01b81523360048201819052833560248301526020840135604483015291830135606482015260608301356084820152608083013560a48201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906352b5de3d9060c4016104f7565b6109c4610bb3565b6109fd6040516020016106d2906020808252601290820152712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b604082015260600190565b6040516383f3d2eb60e01b815233906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906383f3d2eb906104f790849086906004016111ec565b610a55610bb3565b6040516319e28a9960e31b8152336004820181905282356024830152907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063cf1454c8906044016104f7565b610ab3610bb3565b610abc82610cca565b6040516303704f0f60e31b81526001600160a01b0384811660048301523360248301819052848216604484015260648301849052917f000000000000000000000000000000000000000000000000000000000000000090911690631b827878906084015f604051808303815f87803b158015610b36575f5ffd5b505af1158015610b48573d5f5f3e3d5ffd5b5050505050610b5660015f55565b505050565b610b63610bb3565b604051637d237c9960e01b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690637d237c99906104f790849086906004016112e2565b60025f5403610bd557604051633ee5aeb560e01b815260040160405180910390fd5b60025f55565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610c44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c689190611316565b6107cc57338160405163a35b150b60e01b8152600401610c89929190611335565b60405180910390fd5b5f5f610c9d83610cf1565b915091508015610cc1578160405162461bcd60e51b8152600401610c899190611360565b610b5683610d7c565b6001600160a01b03811661052e5760405163d551823d60e01b815260040160405180910390fd5b60605f604483511015610d1657505060408051602081019091525f8082529092909150565b5f610d22846020015190565b90506307b9e43360e51b6001600160e01b0319821601610d615760048401935083806020019051810190610d569190611372565b946001945092505050565b5f60405180602001604052805f815250909250925050915091565b80518060208301fd5b5f60608284031215610d95575f5ffd5b50919050565b5f60608284031215610dab575f5ffd5b610db58383610d85565b9392505050565b5f60e0828403128015610dcd575f5ffd5b509092915050565b5f60808284031215610d95575f5ffd5b5f60808284031215610df5575f5ffd5b610db58383610dd5565b5f5f60208385031215610e10575f5ffd5b823567ffffffffffffffff811115610e26575f5ffd5b8301601f81018513610e36575f5ffd5b803567ffffffffffffffff811115610e4c575f5ffd5b8560208260061b8401011115610e60575f5ffd5b6020919091019590945092505050565b5f5f60208385031215610e81575f5ffd5b823567ffffffffffffffff811115610e97575f5ffd5b8301601f81018513610ea7575f5ffd5b803567ffffffffffffffff811115610ebd575f5ffd5b8560208260051b8401011115610e60575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015610f5657603f19878603018452610f41858351610ed1565b94506020938401939190910190600101610f25565b50929695505050505050565b5f60a0828403128015610dcd575f5ffd5b5f6040828403128015610dcd575f5ffd5b5f6020828403128015610dcd575f5ffd5b80356001600160a01b0381168114610fab575f5ffd5b919050565b5f5f5f60608486031215610fc2575f5ffd5b610fcb84610f95565b9250610fd960208501610f95565b929592945050506040919091013590565b6001600160a01b038381168252608082019061100584610f95565b16602083015260018060a01b0361101e60208501610f95565b16604083015260018060a01b0361103760408501610f95565b1660608301529392505050565b6001600160a01b03838116825261010082019061106084610f95565b16602083015260018060a01b0361107960208501610f95565b1660408381019190915283013560608084019190915283013560808084019190915283013560a08084019190915283013560c0808401919091526001600160a01b03906110c7908501610f95565b1660e08301529392505050565b803560ff81168114610fab575f5ffd5b6001600160a01b038381168252823560208084019190915260a08301919061110d908501610f95565b16604083015260018060a01b0361112660408501610f95565b16606083015260ff61113a606085016110d4565b1660808301529392505050565b6001600160a01b0383168152813560208083019190915260a082019060ff906111719085016110d4565b1660408301525f604084013590508060608401525060ff61113a606085016110d4565b6001600160a01b03831681528135602080830191909152608082019060ff906111be9085016110d4565b166040838101919091529290920135606090910152919050565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b038381168252606082019061120784610f95565b166020838101919091529290920135604090910152919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f8335601e1984360301811261124a575f5ffd5b83018035915067ffffffffffffffff821115611264575f5ffd5b602001915036819003821315611278575f5ffd5b9250929050565b818382375f9101908152919050565b6001600160a01b0383168152813560208083019190915260a082019060ff906112b89085016110d4565b166040838101919091528301356060808401919091526001600160a01b039061113a908501610f95565b6001600160a01b03838116825260a08201906112fd84610f95565b16602083015260018060a01b036112b860208501610f95565b5f60208284031215611326575f5ffd5b81518015158114610db5575f5ffd5b6001600160a01b03831681526040602082018190525f9061135890830184610ed1565b949350505050565b602081525f610db56020830184610ed1565b5f60208284031215611382575f5ffd5b815167ffffffffffffffff811115611398575f5ffd5b8201601f810184136113a8575f5ffd5b805167ffffffffffffffff8111156113c2576113c2611221565b604051601f8201601f19908116603f0116810167ffffffffffffffff811182821017156113f1576113f1611221565b604052818152828201602001861015611408575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea264697066735822122092825a85af0dca125dbaf19b788c888cb76d784beca2681dc70c484cd9e3b78364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xE0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa\x16\xB28\x03\x80a\x16\xB2\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xAAV[`\x01_U`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16`\x80R\x99\x89\x16`\xA0R\x96\x88\x16`\xC0R\x94\x87\x16`\xE0R\x92\x86\x16a\x01\0R\x90\x85\x16a\x01 R\x84\x16a\x01@R\x83\x16a\x01`R\x82\x16a\x01\x80R\x81\x16a\x01\xA0R\x16a\x01\xC0Ra\x01\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x97W__\xFD[PV[\x80Qa\0\xA5\x81a\0\x83V[\x91\x90PV[___________a\x01`\x8C\x8E\x03\x12\x15a\0\xC5W__\xFD[\x8BQa\0\xD0\x81a\0\x83V[` \x8D\x01Q\x90\x9BPa\0\xE1\x81a\0\x83V[`@\x8D\x01Q\x90\x9APa\0\xF2\x81a\0\x83V[``\x8D\x01Q\x90\x99Pa\x01\x03\x81a\0\x83V[`\x80\x8D\x01Q\x90\x98Pa\x01\x14\x81a\0\x83V[`\xA0\x8D\x01Q\x90\x97Pa\x01%\x81a\0\x83V[`\xC0\x8D\x01Q\x90\x96Pa\x016\x81a\0\x83V[\x94Pa\x01D`\xE0\x8D\x01a\0\x9AV[\x93Pa\x01Sa\x01\0\x8D\x01a\0\x9AV[\x92Pa\x01ba\x01 \x8D\x01a\0\x9AV[\x91Pa\x01qa\x01@\x8D\x01a\0\x9AV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x14[a\x02W_9_\x81\x81a\x047\x01Ra\nt\x01R_\x81\x81a\x03\xBE\x01R\x81\x81a\x07%\x01Ra\n\x16\x01R_\x81\x81a\x01\\\x01Ra\t\0\x01R_\x81\x81a\x02\xD9\x01Ra\x06\x02\x01R_\x81\x81a\x03\x1F\x01Ra\x06Z\x01R_\x81\x81a\x02\xA6\x01Ra\x05\xAA\x01R_\x81\x81a\x02`\x01R\x81\x81a\x05R\x01Ra\t\x85\x01R_\x81\x81a\x03\xF1\x01R\x81\x81a\x04\xC0\x01Ra\x0B|\x01R_a\x02\x1A\x01R_\x81\x81a\x04}\x01Ra\n\xF3\x01R_\x81\x81a\x01\xE7\x01Ra\x0B\xF7\x01Ra\x14[_\xF3\xFE`\x80`@R`\x046\x10a\x01GW_5`\xE0\x1C\x80c\xA8.\xD4\xCE\x11a\0\xB3W\x80c\xD2Z\xDE\xB3\x11a\0mW\x80c\xD2Z\xDE\xB3\x14a\x03\xADW\x80c\xD9\xE0\x88\x11\x14a\x03\xE0W\x80c\xE6\xD6j\xC8\x14a\x04\x13W\x80c\xED'\xAF\xAF\x14a\x04&W\x80c\xF3\xE0\x1AN\x14a\x04YW\x80c\xF8\x87\xEA@\x14a\x04lW__\xFD[\x80c\xA8.\xD4\xCE\x14a\x03\x0EW\x80c\xAC\x96P\xD8\x14a\x03AW\x80c\xAD\x0C\x994\x14a\x03aW\x80c\xB1\x84\x06u\x14a\x03tW\x80c\xB6\x102\x15\x14a\x03\x87W\x80c\xC5mJ*\x14a\x03\x9AW__\xFD[\x80c{\x12;\xC8\x11a\x01\x04W\x80c{\x12;\xC8\x14a\x02<W\x80c\x8AS\xAA\xAC\x14a\x02OW\x80c\x90~tT\x14a\x02\x82W\x80c\x9C\x8B,\xFB\x14a\x02\x95W\x80c\x9DE\x1D\x0C\x14a\x02\xC8W\x80c\xA7g\x18\xD2\x14a\x02\xFBW__\xFD[\x80c\x084s\xEF\x14a\x01KW\x80c!\xB1\x08\xCC\x14a\x01\x9BW\x80c4\xF5\xDB~\x14a\x01\xB0W\x80cGW\x0C\xDC\x14a\x01\xC3W\x80cJJ{\x04\x14a\x01\xD6W\x80cf\r\rg\x14a\x02\tW[__\xFD[4\x80\x15a\x01VW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xAEa\x01\xA96`\x04a\r\x9BV[a\x04\x9FV[\0[a\x01\xAEa\x01\xBE6`\x04a\r\xBCV[a\x051V[a\x01\xAEa\x01\xD16`\x04a\r\xE5V[a\x05\x89V[4\x80\x15a\x01\xE1W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\x14W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x02J6`\x04a\r\xE5V[a\x05\xE1V[4\x80\x15a\x02ZW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x02\x906`\x04a\r\x9BV[a\x069V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xD3W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x03\t6`\x04a\r\xFFV[a\x06\x91V[4\x80\x15a\x03\x19W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ta\x03O6`\x04a\x0EpV[a\x07\xD0V[`@Qa\x01\x92\x91\x90a\x0E\xFFV[a\x01\xAEa\x03o6`\x04a\r\xE5V[a\x08\xDFV[a\x01\xAEa\x03\x826`\x04a\x0FbV[a\t7V[a\x01\xAEa\x03\x956`\x04a\x0FsV[a\t\xBCV[a\x01\xAEa\x03\xA86`\x04a\x0F\x84V[a\nMV[4\x80\x15a\x03\xB8W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xEBW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x04!6`\x04a\x0F\xB0V[a\n\xABV[4\x80\x15a\x041W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x04g6`\x04a\r\xE5V[a\x0B[V[4\x80\x15a\x04wW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xA7a\x0B\xB3V[`@Qc\x0B\xD9\xA8\x9D`\xE3\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^\xCDD\xE8\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x0F\xEAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x0EW__\xFD[PZ\xF1\x15\x80\x15a\x05 W=__>=_\xFD[PPPPPa\x05.`\x01_UV[PV[a\x059a\x0B\xB3V[`@Qcl\xE2\x13\xA1`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD9\xC4'B\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x10DV[a\x05\x91a\x0B\xB3V[`@Qc\x0FN[\xA1`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x0FN[\xA1\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x10\xE4V[a\x05\xE9a\x0B\xB3V[`@Qc%\xF8\xC6+`\xE2\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x97\xE3\x18\xAC\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11GV[a\x06Aa\x0B\xB3V[`@Qc\x13D\x94\xCD`\xE3\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9A$\xA6h\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11\x94V[a\x06\x99a\x0B\xB3V[a\x07\x18`@Q` \x01a\x06\xD2\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B\x81RPa\x0B\xDBV[3_[\x82\x81\x10\x15a\x07\xC1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x83\xF3\xD2\xEB\x83\x86\x86\x85\x81\x81\x10a\x07eWa\x07ea\x11\xD8V[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x88\x92\x91\x90a\x11\xECV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x9FW__\xFD[PZ\xF1\x15\x80\x15a\x07\xB1W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x07\x1B\x90PV[PPa\x07\xCC`\x01_UV[PPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xEBWa\x07\xEBa\x12!V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x1EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\tW\x90P[P\x90P_[\x82\x81\x10\x15a\x08\xD8W_\x800\x86\x86\x85\x81\x81\x10a\x08@Wa\x08@a\x11\xD8V[\x90P` \x02\x81\x01\x90a\x08R\x91\x90a\x125V[`@Qa\x08`\x92\x91\x90a\x12\x7FV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x08\x98W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\x9DV[``\x91P[P\x91P\x91P\x81a\x08\xB0Wa\x08\xB0\x81a\x0C\x92V[\x80\x84\x84\x81Q\x81\x10a\x08\xC3Wa\x08\xC3a\x11\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08#V[P\x92\x91PPV[a\x08\xE7a\x0B\xB3V[`@Qc\x13=\xD6\xF5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&{\xAD\xEA\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x12\x8EV[a\t?a\x0B\xB3V[`@\x80QcR\xB5\xDE=`\xE0\x1B\x81R3`\x04\x82\x01\x81\x90R\x835`$\x83\x01R` \x84\x015`D\x83\x01R\x91\x83\x015`d\x82\x01R``\x83\x015`\x84\x82\x01R`\x80\x83\x015`\xA4\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cR\xB5\xDE=\x90`\xC4\x01a\x04\xF7V[a\t\xC4a\x0B\xB3V[a\t\xFD`@Q` \x01a\x06\xD2\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V[`@Qc\x83\xF3\xD2\xEB`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x83\xF3\xD2\xEB\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11\xECV[a\nUa\x0B\xB3V[`@Qc\x19\xE2\x8A\x99`\xE3\x1B\x81R3`\x04\x82\x01\x81\x90R\x825`$\x83\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCF\x14T\xC8\x90`D\x01a\x04\xF7V[a\n\xB3a\x0B\xB3V[a\n\xBC\x82a\x0C\xCAV[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B6W__\xFD[PZ\xF1\x15\x80\x15a\x0BHW=__>=_\xFD[PPPPPa\x0BV`\x01_UV[PPPV[a\x0Bca\x0B\xB3V[`@Qc}#|\x99`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c}#|\x99\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x12\xE2V[`\x02_T\x03a\x0B\xD5W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ch\x91\x90a\x13\x16V[a\x07\xCCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x0C\x89\x92\x91\x90a\x135V[`@Q\x80\x91\x03\x90\xFD[__a\x0C\x9D\x83a\x0C\xF1V[\x91P\x91P\x80\x15a\x0C\xC1W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C\x89\x91\x90a\x13`V[a\x0BV\x83a\r|V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05.W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_`D\x83Q\x10\x15a\r\x16WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x90\x92\x90\x91PV[_a\r\"\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\raW`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\rV\x91\x90a\x13rV[\x94`\x01\x94P\x92PPPV[_`@Q\x80` \x01`@R\x80_\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[_``\x82\x84\x03\x12\x15a\r\x95W__\xFD[P\x91\x90PV[_``\x82\x84\x03\x12\x15a\r\xABW__\xFD[a\r\xB5\x83\x83a\r\x85V[\x93\x92PPPV[_`\xE0\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[P\x90\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\r\x95W__\xFD[_`\x80\x82\x84\x03\x12\x15a\r\xF5W__\xFD[a\r\xB5\x83\x83a\r\xD5V[__` \x83\x85\x03\x12\x15a\x0E\x10W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E&W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E6W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ELW__\xFD[\x85` \x82`\x06\x1B\x84\x01\x01\x11\x15a\x0E`W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__` \x83\x85\x03\x12\x15a\x0E\x81W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x97W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E\xA7W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xBDW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x0E`W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x0FVW`?\x19\x87\x86\x03\x01\x84Ra\x0FA\x85\x83Qa\x0E\xD1V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x0F%V[P\x92\x96\x95PPPPPPV[_`\xA0\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[_`@\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[_` \x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xABW__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x0F\xC2W__\xFD[a\x0F\xCB\x84a\x0F\x95V[\x92Pa\x0F\xD9` \x85\x01a\x0F\x95V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\x80\x82\x01\x90a\x10\x05\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10\x1E` \x85\x01a\x0F\x95V[\x16`@\x83\x01R`\x01\x80`\xA0\x1B\x03a\x107`@\x85\x01a\x0F\x95V[\x16``\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82Ra\x01\0\x82\x01\x90a\x10`\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10y` \x85\x01a\x0F\x95V[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R\x83\x015`\x80\x80\x84\x01\x91\x90\x91R\x83\x015`\xA0\x80\x84\x01\x91\x90\x91R\x83\x015`\xC0\x80\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90a\x10\xC7\x90\x85\x01a\x0F\x95V[\x16`\xE0\x83\x01R\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\xABW__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R\x825` \x80\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90a\x11\r\x90\x85\x01a\x0F\x95V[\x16`@\x83\x01R`\x01\x80`\xA0\x1B\x03a\x11&`@\x85\x01a\x0F\x95V[\x16``\x83\x01R`\xFFa\x11:``\x85\x01a\x10\xD4V[\x16`\x80\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x90`\xFF\x90a\x11q\x90\x85\x01a\x10\xD4V[\x16`@\x83\x01R_`@\x84\x015\x90P\x80``\x84\x01RP`\xFFa\x11:``\x85\x01a\x10\xD4V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\x80\x82\x01\x90`\xFF\x90a\x11\xBE\x90\x85\x01a\x10\xD4V[\x16`@\x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015``\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R``\x82\x01\x90a\x12\x07\x84a\x0F\x95V[\x16` \x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015`@\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12JW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12dW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x12xW__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x90`\xFF\x90a\x12\xB8\x90\x85\x01a\x10\xD4V[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90a\x11:\x90\x85\x01a\x0F\x95V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xA0\x82\x01\x90a\x12\xFD\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x12\xB8` \x85\x01a\x0F\x95V[_` \x82\x84\x03\x12\x15a\x13&W__\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xB5W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x13X\x90\x83\x01\x84a\x0E\xD1V[\x94\x93PPPPV[` \x81R_a\r\xB5` \x83\x01\x84a\x0E\xD1V[_` \x82\x84\x03\x12\x15a\x13\x82W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x98W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13\xA8W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xC2Wa\x13\xC2a\x12!V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\xF1Wa\x13\xF1a\x12!V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x14\x08W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x92\x82Z\x85\xAF\r\xCA\x12]\xBA\xF1\x9Bx\x8C\x88\x8C\xB7mxK\xEC\xA2h\x1D\xC7\x0CHL\xD9\xE3\xB7\x83dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610147575f3560e01c8063a82ed4ce116100b3578063d25adeb31161006d578063d25adeb3146103ad578063d9e08811146103e0578063e6d66ac814610413578063ed27afaf14610426578063f3e01a4e14610459578063f887ea401461046c575f5ffd5b8063a82ed4ce1461030e578063ac9650d814610341578063ad0c993414610361578063b184067514610374578063b610321514610387578063c56d4a2a1461039a575f5ffd5b80637b123bc8116101045780637b123bc81461023c5780638a53aaac1461024f578063907e7454146102825780639c8b2cfb146102955780639d451d0c146102c8578063a76718d2146102fb575f5ffd5b8063083473ef1461014b57806321b108cc1461019b57806334f5db7e146101b057806347570cdc146101c35780634a4a7b04146101d6578063660d0d6714610209575b5f5ffd5b348015610156575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6101ae6101a9366004610d9b565b61049f565b005b6101ae6101be366004610dbc565b610531565b6101ae6101d1366004610de5565b610589565b3480156101e1575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b348015610214575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae61024a366004610de5565b6105e1565b34801561025a575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610290366004610d9b565b610639565b3480156102a0575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102d3575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610309366004610dff565b610691565b348015610319575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b61035461034f366004610e70565b6107d0565b6040516101929190610eff565b6101ae61036f366004610de5565b6108df565b6101ae610382366004610f62565b610937565b6101ae610395366004610f73565b6109bc565b6101ae6103a8366004610f84565b610a4d565b3480156103b8575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b3480156103eb575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610421366004610fb0565b610aab565b348015610431575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6101ae610467366004610de5565b610b5b565b348015610477575f5ffd5b5061017e7f000000000000000000000000000000000000000000000000000000000000000081565b6104a7610bb3565b604051630bd9a89d60e31b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ecd44e8906104f79084908690600401610fea565b5f604051808303815f87803b15801561050e575f5ffd5b505af1158015610520573d5f5f3e3d5ffd5b505050505061052e60015f55565b50565b610539610bb3565b604051636ce213a160e11b815233906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063d9c42742906104f79084908690600401611044565b610591610bb3565b604051630f4e5ba160e01b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690630f4e5ba1906104f790849086906004016110e4565b6105e9610bb3565b6040516325f8c62b60e21b815233906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906397e318ac906104f79084908690600401611147565b610641610bb3565b60405163134494cd60e31b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639a24a668906104f79084908690600401611194565b610699610bb3565b6107186040516020016106d2906020808252601290820152712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b604082015260600190565b60405160208183030381529060405280519060200120604051806040016040528060128152602001712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b815250610bdb565b335f5b828110156107c1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166383f3d2eb83868685818110610765576107656111d8565b9050604002016040518363ffffffff1660e01b81526004016107889291906111ec565b5f604051808303815f87803b15801561079f575f5ffd5b505af11580156107b1573d5f5f3e3d5ffd5b50506001909201915061071b9050565b50506107cc60015f55565b5050565b60608167ffffffffffffffff8111156107eb576107eb611221565b60405190808252806020026020018201604052801561081e57816020015b60608152602001906001900390816108095790505b5090505f5b828110156108d8575f8030868685818110610840576108406111d8565b90506020028101906108529190611235565b60405161086092919061127f565b5f60405180830381855af49150503d805f8114610898576040519150601f19603f3d011682016040523d82523d5f602084013e61089d565b606091505b5091509150816108b0576108b081610c92565b808484815181106108c3576108c36111d8565b60209081029190910101525050600101610823565b5092915050565b6108e7610bb3565b60405163133dd6f560e11b815233906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063267badea906104f7908490869060040161128e565b61093f610bb3565b604080516352b5de3d60e01b81523360048201819052833560248301526020840135604483015291830135606482015260608301356084820152608083013560a48201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906352b5de3d9060c4016104f7565b6109c4610bb3565b6109fd6040516020016106d2906020808252601290820152712624a8aaa4a220aa24a7a72fa5a2a2a822a960711b604082015260600190565b6040516383f3d2eb60e01b815233906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906383f3d2eb906104f790849086906004016111ec565b610a55610bb3565b6040516319e28a9960e31b8152336004820181905282356024830152907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063cf1454c8906044016104f7565b610ab3610bb3565b610abc82610cca565b6040516303704f0f60e31b81526001600160a01b0384811660048301523360248301819052848216604484015260648301849052917f000000000000000000000000000000000000000000000000000000000000000090911690631b827878906084015f604051808303815f87803b158015610b36575f5ffd5b505af1158015610b48573d5f5f3e3d5ffd5b5050505050610b5660015f55565b505050565b610b63610bb3565b604051637d237c9960e01b815233906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690637d237c99906104f790849086906004016112e2565b60025f5403610bd557604051633ee5aeb560e01b815260040160405180910390fd5b60025f55565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610c44573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c689190611316565b6107cc57338160405163a35b150b60e01b8152600401610c89929190611335565b60405180910390fd5b5f5f610c9d83610cf1565b915091508015610cc1578160405162461bcd60e51b8152600401610c899190611360565b610b5683610d7c565b6001600160a01b03811661052e5760405163d551823d60e01b815260040160405180910390fd5b60605f604483511015610d1657505060408051602081019091525f8082529092909150565b5f610d22846020015190565b90506307b9e43360e51b6001600160e01b0319821601610d615760048401935083806020019051810190610d569190611372565b946001945092505050565b5f60405180602001604052805f815250909250925050915091565b80518060208301fd5b5f60608284031215610d95575f5ffd5b50919050565b5f60608284031215610dab575f5ffd5b610db58383610d85565b9392505050565b5f60e0828403128015610dcd575f5ffd5b509092915050565b5f60808284031215610d95575f5ffd5b5f60808284031215610df5575f5ffd5b610db58383610dd5565b5f5f60208385031215610e10575f5ffd5b823567ffffffffffffffff811115610e26575f5ffd5b8301601f81018513610e36575f5ffd5b803567ffffffffffffffff811115610e4c575f5ffd5b8560208260061b8401011115610e60575f5ffd5b6020919091019590945092505050565b5f5f60208385031215610e81575f5ffd5b823567ffffffffffffffff811115610e97575f5ffd5b8301601f81018513610ea7575f5ffd5b803567ffffffffffffffff811115610ebd575f5ffd5b8560208260051b8401011115610e60575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015610f5657603f19878603018452610f41858351610ed1565b94506020938401939190910190600101610f25565b50929695505050505050565b5f60a0828403128015610dcd575f5ffd5b5f6040828403128015610dcd575f5ffd5b5f6020828403128015610dcd575f5ffd5b80356001600160a01b0381168114610fab575f5ffd5b919050565b5f5f5f60608486031215610fc2575f5ffd5b610fcb84610f95565b9250610fd960208501610f95565b929592945050506040919091013590565b6001600160a01b038381168252608082019061100584610f95565b16602083015260018060a01b0361101e60208501610f95565b16604083015260018060a01b0361103760408501610f95565b1660608301529392505050565b6001600160a01b03838116825261010082019061106084610f95565b16602083015260018060a01b0361107960208501610f95565b1660408381019190915283013560608084019190915283013560808084019190915283013560a08084019190915283013560c0808401919091526001600160a01b03906110c7908501610f95565b1660e08301529392505050565b803560ff81168114610fab575f5ffd5b6001600160a01b038381168252823560208084019190915260a08301919061110d908501610f95565b16604083015260018060a01b0361112660408501610f95565b16606083015260ff61113a606085016110d4565b1660808301529392505050565b6001600160a01b0383168152813560208083019190915260a082019060ff906111719085016110d4565b1660408301525f604084013590508060608401525060ff61113a606085016110d4565b6001600160a01b03831681528135602080830191909152608082019060ff906111be9085016110d4565b166040838101919091529290920135606090910152919050565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b038381168252606082019061120784610f95565b166020838101919091529290920135604090910152919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f8335601e1984360301811261124a575f5ffd5b83018035915067ffffffffffffffff821115611264575f5ffd5b602001915036819003821315611278575f5ffd5b9250929050565b818382375f9101908152919050565b6001600160a01b0383168152813560208083019190915260a082019060ff906112b89085016110d4565b166040838101919091528301356060808401919091526001600160a01b039061113a908501610f95565b6001600160a01b03838116825260a08201906112fd84610f95565b16602083015260018060a01b036112b860208501610f95565b5f60208284031215611326575f5ffd5b81518015158114610db5575f5ffd5b6001600160a01b03831681526040602082018190525f9061135890830184610ed1565b949350505050565b602081525f610db56020830184610ed1565b5f60208284031215611382575f5ffd5b815167ffffffffffffffff811115611398575f5ffd5b8201601f810184136113a8575f5ffd5b805167ffffffffffffffff8111156113c2576113c2611221565b604051601f8201601f19908116603f0116810167ffffffffffffffff811182821017156113f1576113f1611221565b604052818152828201602001861015611408575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea264697066735822122092825a85af0dca125dbaf19b788c888cb76d784beca2681dc70c484cd9e3b78364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01GW_5`\xE0\x1C\x80c\xA8.\xD4\xCE\x11a\0\xB3W\x80c\xD2Z\xDE\xB3\x11a\0mW\x80c\xD2Z\xDE\xB3\x14a\x03\xADW\x80c\xD9\xE0\x88\x11\x14a\x03\xE0W\x80c\xE6\xD6j\xC8\x14a\x04\x13W\x80c\xED'\xAF\xAF\x14a\x04&W\x80c\xF3\xE0\x1AN\x14a\x04YW\x80c\xF8\x87\xEA@\x14a\x04lW__\xFD[\x80c\xA8.\xD4\xCE\x14a\x03\x0EW\x80c\xAC\x96P\xD8\x14a\x03AW\x80c\xAD\x0C\x994\x14a\x03aW\x80c\xB1\x84\x06u\x14a\x03tW\x80c\xB6\x102\x15\x14a\x03\x87W\x80c\xC5mJ*\x14a\x03\x9AW__\xFD[\x80c{\x12;\xC8\x11a\x01\x04W\x80c{\x12;\xC8\x14a\x02<W\x80c\x8AS\xAA\xAC\x14a\x02OW\x80c\x90~tT\x14a\x02\x82W\x80c\x9C\x8B,\xFB\x14a\x02\x95W\x80c\x9DE\x1D\x0C\x14a\x02\xC8W\x80c\xA7g\x18\xD2\x14a\x02\xFBW__\xFD[\x80c\x084s\xEF\x14a\x01KW\x80c!\xB1\x08\xCC\x14a\x01\x9BW\x80c4\xF5\xDB~\x14a\x01\xB0W\x80cGW\x0C\xDC\x14a\x01\xC3W\x80cJJ{\x04\x14a\x01\xD6W\x80cf\r\rg\x14a\x02\tW[__\xFD[4\x80\x15a\x01VW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xAEa\x01\xA96`\x04a\r\x9BV[a\x04\x9FV[\0[a\x01\xAEa\x01\xBE6`\x04a\r\xBCV[a\x051V[a\x01\xAEa\x01\xD16`\x04a\r\xE5V[a\x05\x89V[4\x80\x15a\x01\xE1W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\x14W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x02J6`\x04a\r\xE5V[a\x05\xE1V[4\x80\x15a\x02ZW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x02\x906`\x04a\r\x9BV[a\x069V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xD3W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x03\t6`\x04a\r\xFFV[a\x06\x91V[4\x80\x15a\x03\x19W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ta\x03O6`\x04a\x0EpV[a\x07\xD0V[`@Qa\x01\x92\x91\x90a\x0E\xFFV[a\x01\xAEa\x03o6`\x04a\r\xE5V[a\x08\xDFV[a\x01\xAEa\x03\x826`\x04a\x0FbV[a\t7V[a\x01\xAEa\x03\x956`\x04a\x0FsV[a\t\xBCV[a\x01\xAEa\x03\xA86`\x04a\x0F\x84V[a\nMV[4\x80\x15a\x03\xB8W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xEBW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x04!6`\x04a\x0F\xB0V[a\n\xABV[4\x80\x15a\x041W__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xAEa\x04g6`\x04a\r\xE5V[a\x0B[V[4\x80\x15a\x04wW__\xFD[Pa\x01~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xA7a\x0B\xB3V[`@Qc\x0B\xD9\xA8\x9D`\xE3\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c^\xCDD\xE8\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x0F\xEAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x0EW__\xFD[PZ\xF1\x15\x80\x15a\x05 W=__>=_\xFD[PPPPPa\x05.`\x01_UV[PV[a\x059a\x0B\xB3V[`@Qcl\xE2\x13\xA1`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD9\xC4'B\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x10DV[a\x05\x91a\x0B\xB3V[`@Qc\x0FN[\xA1`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x0FN[\xA1\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x10\xE4V[a\x05\xE9a\x0B\xB3V[`@Qc%\xF8\xC6+`\xE2\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x97\xE3\x18\xAC\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11GV[a\x06Aa\x0B\xB3V[`@Qc\x13D\x94\xCD`\xE3\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9A$\xA6h\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11\x94V[a\x06\x99a\x0B\xB3V[a\x07\x18`@Q` \x01a\x06\xD2\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B\x81RPa\x0B\xDBV[3_[\x82\x81\x10\x15a\x07\xC1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x83\xF3\xD2\xEB\x83\x86\x86\x85\x81\x81\x10a\x07eWa\x07ea\x11\xD8V[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x88\x92\x91\x90a\x11\xECV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x9FW__\xFD[PZ\xF1\x15\x80\x15a\x07\xB1W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x07\x1B\x90PV[PPa\x07\xCC`\x01_UV[PPV[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xEBWa\x07\xEBa\x12!V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x1EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\tW\x90P[P\x90P_[\x82\x81\x10\x15a\x08\xD8W_\x800\x86\x86\x85\x81\x81\x10a\x08@Wa\x08@a\x11\xD8V[\x90P` \x02\x81\x01\x90a\x08R\x91\x90a\x125V[`@Qa\x08`\x92\x91\x90a\x12\x7FV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x08\x98W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\x9DV[``\x91P[P\x91P\x91P\x81a\x08\xB0Wa\x08\xB0\x81a\x0C\x92V[\x80\x84\x84\x81Q\x81\x10a\x08\xC3Wa\x08\xC3a\x11\xD8V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08#V[P\x92\x91PPV[a\x08\xE7a\x0B\xB3V[`@Qc\x13=\xD6\xF5`\xE1\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&{\xAD\xEA\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x12\x8EV[a\t?a\x0B\xB3V[`@\x80QcR\xB5\xDE=`\xE0\x1B\x81R3`\x04\x82\x01\x81\x90R\x835`$\x83\x01R` \x84\x015`D\x83\x01R\x91\x83\x015`d\x82\x01R``\x83\x015`\x84\x82\x01R`\x80\x83\x015`\xA4\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cR\xB5\xDE=\x90`\xC4\x01a\x04\xF7V[a\t\xC4a\x0B\xB3V[a\t\xFD`@Q` \x01a\x06\xD2\x90` \x80\x82R`\x12\x90\x82\x01Rq&$\xA8\xAA\xA4\xA2 \xAA$\xA7\xA7/\xA5\xA2\xA2\xA8\"\xA9`q\x1B`@\x82\x01R``\x01\x90V[`@Qc\x83\xF3\xD2\xEB`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x83\xF3\xD2\xEB\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x11\xECV[a\nUa\x0B\xB3V[`@Qc\x19\xE2\x8A\x99`\xE3\x1B\x81R3`\x04\x82\x01\x81\x90R\x825`$\x83\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCF\x14T\xC8\x90`D\x01a\x04\xF7V[a\n\xB3a\x0B\xB3V[a\n\xBC\x82a\x0C\xCAV[`@Qc\x03pO\x0F`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R3`$\x83\x01\x81\x90R\x84\x82\x16`D\x84\x01R`d\x83\x01\x84\x90R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1B\x82xx\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B6W__\xFD[PZ\xF1\x15\x80\x15a\x0BHW=__>=_\xFD[PPPPPa\x0BV`\x01_UV[PPPV[a\x0Bca\x0B\xB3V[`@Qc}#|\x99`\xE0\x1B\x81R3\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c}#|\x99\x90a\x04\xF7\x90\x84\x90\x86\x90`\x04\x01a\x12\xE2V[`\x02_T\x03a\x0B\xD5W`@Qc>\xE5\xAE\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_UV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ch\x91\x90a\x13\x16V[a\x07\xCCW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x0C\x89\x92\x91\x90a\x135V[`@Q\x80\x91\x03\x90\xFD[__a\x0C\x9D\x83a\x0C\xF1V[\x91P\x91P\x80\x15a\x0C\xC1W\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C\x89\x91\x90a\x13`V[a\x0BV\x83a\r|V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05.W`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_`D\x83Q\x10\x15a\r\x16WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x90\x92\x90\x91PV[_a\r\"\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\raW`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\rV\x91\x90a\x13rV[\x94`\x01\x94P\x92PPPV[_`@Q\x80` \x01`@R\x80_\x81RP\x90\x92P\x92PP\x91P\x91V[\x80Q\x80` \x83\x01\xFD[_``\x82\x84\x03\x12\x15a\r\x95W__\xFD[P\x91\x90PV[_``\x82\x84\x03\x12\x15a\r\xABW__\xFD[a\r\xB5\x83\x83a\r\x85V[\x93\x92PPPV[_`\xE0\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[P\x90\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15a\r\x95W__\xFD[_`\x80\x82\x84\x03\x12\x15a\r\xF5W__\xFD[a\r\xB5\x83\x83a\r\xD5V[__` \x83\x85\x03\x12\x15a\x0E\x10W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E&W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E6W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ELW__\xFD[\x85` \x82`\x06\x1B\x84\x01\x01\x11\x15a\x0E`W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__` \x83\x85\x03\x12\x15a\x0E\x81W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x97W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E\xA7W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xBDW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x0E`W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x0FVW`?\x19\x87\x86\x03\x01\x84Ra\x0FA\x85\x83Qa\x0E\xD1V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x0F%V[P\x92\x96\x95PPPPPPV[_`\xA0\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[_`@\x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[_` \x82\x84\x03\x12\x80\x15a\r\xCDW__\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xABW__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x0F\xC2W__\xFD[a\x0F\xCB\x84a\x0F\x95V[\x92Pa\x0F\xD9` \x85\x01a\x0F\x95V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\x80\x82\x01\x90a\x10\x05\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10\x1E` \x85\x01a\x0F\x95V[\x16`@\x83\x01R`\x01\x80`\xA0\x1B\x03a\x107`@\x85\x01a\x0F\x95V[\x16``\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82Ra\x01\0\x82\x01\x90a\x10`\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x10y` \x85\x01a\x0F\x95V[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R\x83\x015`\x80\x80\x84\x01\x91\x90\x91R\x83\x015`\xA0\x80\x84\x01\x91\x90\x91R\x83\x015`\xC0\x80\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90a\x10\xC7\x90\x85\x01a\x0F\x95V[\x16`\xE0\x83\x01R\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\xABW__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R\x825` \x80\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90a\x11\r\x90\x85\x01a\x0F\x95V[\x16`@\x83\x01R`\x01\x80`\xA0\x1B\x03a\x11&`@\x85\x01a\x0F\x95V[\x16``\x83\x01R`\xFFa\x11:``\x85\x01a\x10\xD4V[\x16`\x80\x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x90`\xFF\x90a\x11q\x90\x85\x01a\x10\xD4V[\x16`@\x83\x01R_`@\x84\x015\x90P\x80``\x84\x01RP`\xFFa\x11:``\x85\x01a\x10\xD4V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\x80\x82\x01\x90`\xFF\x90a\x11\xBE\x90\x85\x01a\x10\xD4V[\x16`@\x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015``\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R``\x82\x01\x90a\x12\x07\x84a\x0F\x95V[\x16` \x83\x81\x01\x91\x90\x91R\x92\x90\x92\x015`@\x90\x91\x01R\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12JW__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12dW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x12xW__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R\x815` \x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x90`\xFF\x90a\x12\xB8\x90\x85\x01a\x10\xD4V[\x16`@\x83\x81\x01\x91\x90\x91R\x83\x015``\x80\x84\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90a\x11:\x90\x85\x01a\x0F\x95V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`\xA0\x82\x01\x90a\x12\xFD\x84a\x0F\x95V[\x16` \x83\x01R`\x01\x80`\xA0\x1B\x03a\x12\xB8` \x85\x01a\x0F\x95V[_` \x82\x84\x03\x12\x15a\x13&W__\xFD[\x81Q\x80\x15\x15\x81\x14a\r\xB5W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x13X\x90\x83\x01\x84a\x0E\xD1V[\x94\x93PPPPV[` \x81R_a\r\xB5` \x83\x01\x84a\x0E\xD1V[_` \x82\x84\x03\x12\x15a\x13\x82W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x98W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13\xA8W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xC2Wa\x13\xC2a\x12!V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\xF1Wa\x13\xF1a\x12!V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x14\x08W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x92\x82Z\x85\xAF\r\xCA\x12]\xBA\xF1\x9Bx\x8C\x88\x8C\xB7mxK\xEC\xA2h\x1D\xC7\x0CHL\xD9\xE3\xB7\x83dsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `EmptyReceiver()` and selector `0xd551823d`.
```solidity
error EmptyReceiver();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyReceiver {}
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
        impl ::core::convert::From<EmptyReceiver> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyReceiver) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyReceiver {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyReceiver {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyReceiver()";
            const SELECTOR: [u8; 4] = [213u8, 81u8, 130u8, 61u8];
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
    /**Custom error with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`.
```solidity
error ReentrancyGuardReentrantCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReentrancyGuardReentrantCall {}
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
        impl ::core::convert::From<ReentrancyGuardReentrantCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReentrancyGuardReentrantCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReentrancyGuardReentrantCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReentrancyGuardReentrantCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReentrancyGuardReentrantCall()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 174u8, 181u8];
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
constructor(address _router, address _roleStore, address _dataStore, address _liquidityHandler, address _swapHandler, address _depositHandler, address _borrowHandler, address _repayHandler, address _withdrawHandler, address _liquidationHandler, address _closeHandler);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _router: alloy::sol_types::private::Address,
        pub _roleStore: alloy::sol_types::private::Address,
        pub _dataStore: alloy::sol_types::private::Address,
        pub _liquidityHandler: alloy::sol_types::private::Address,
        pub _swapHandler: alloy::sol_types::private::Address,
        pub _depositHandler: alloy::sol_types::private::Address,
        pub _borrowHandler: alloy::sol_types::private::Address,
        pub _repayHandler: alloy::sol_types::private::Address,
        pub _withdrawHandler: alloy::sol_types::private::Address,
        pub _liquidationHandler: alloy::sol_types::private::Address,
        pub _closeHandler: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
                    (
                        value._router,
                        value._roleStore,
                        value._dataStore,
                        value._liquidityHandler,
                        value._swapHandler,
                        value._depositHandler,
                        value._borrowHandler,
                        value._repayHandler,
                        value._withdrawHandler,
                        value._liquidationHandler,
                        value._closeHandler,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _router: tuple.0,
                        _roleStore: tuple.1,
                        _dataStore: tuple.2,
                        _liquidityHandler: tuple.3,
                        _swapHandler: tuple.4,
                        _depositHandler: tuple.5,
                        _borrowHandler: tuple.6,
                        _repayHandler: tuple.7,
                        _withdrawHandler: tuple.8,
                        _liquidationHandler: tuple.9,
                        _closeHandler: tuple.10,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                        &self._router,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._roleStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._liquidityHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._swapHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._depositHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._borrowHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._repayHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._withdrawHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._liquidationHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._closeHandler,
                    ),
                )
            }
        }
    };
    /**Function with signature `borrowHandler()` and selector `0xa82ed4ce`.
```solidity
function borrowHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct borrowHandlerCall {}
    ///Container type for the return parameters of the [`borrowHandler()`](borrowHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct borrowHandlerReturn {
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
            impl ::core::convert::From<borrowHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: borrowHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for borrowHandlerCall {
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
            impl ::core::convert::From<borrowHandlerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: borrowHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for borrowHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for borrowHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = borrowHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "borrowHandler()";
            const SELECTOR: [u8; 4] = [168u8, 46u8, 212u8, 206u8];
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
    /**Function with signature `closeHandler()` and selector `0xed27afaf`.
```solidity
function closeHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeHandlerCall {}
    ///Container type for the return parameters of the [`closeHandler()`](closeHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct closeHandlerReturn {
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
            impl ::core::convert::From<closeHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: closeHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeHandlerCall {
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
            impl ::core::convert::From<closeHandlerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: closeHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for closeHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for closeHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = closeHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "closeHandler()";
            const SELECTOR: [u8; 4] = [237u8, 39u8, 175u8, 175u8];
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
    /**Function with signature `depositHandler()` and selector `0x9c8b2cfb`.
```solidity
function depositHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositHandlerCall {}
    ///Container type for the return parameters of the [`depositHandler()`](depositHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositHandlerReturn {
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
            impl ::core::convert::From<depositHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositHandlerCall {
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
            impl ::core::convert::From<depositHandlerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositHandler()";
            const SELECTOR: [u8; 4] = [156u8, 139u8, 44u8, 251u8];
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
    /**Function with signature `executeAdd((address,address,address))` and selector `0x21b108cc`.
```solidity
function executeAdd(LiquidityUtils.AddParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeAddCall {
        pub params: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeAdd((address,address,address))`](executeAddCall) function.
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
            type UnderlyingSolTuple<'a> = (LiquidityUtils::AddParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            type Parameters<'a> = (LiquidityUtils::AddParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeAddReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeAdd((address,address,address))";
            const SELECTOR: [u8; 4] = [33u8, 177u8, 8u8, 204u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LiquidityUtils::AddParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeBorrow((uint256,uint8,uint256))` and selector `0x907e7454`.
```solidity
function executeBorrow(BorrowUtils.BorrowParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBorrowCall {
        pub params: <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeBorrow((uint256,uint8,uint256))`](executeBorrowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBorrowReturn {}
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
            type UnderlyingSolTuple<'a> = (BorrowUtils::BorrowParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeBorrowCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeBorrowCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBorrowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeBorrowReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeBorrowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBorrowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeBorrowCall {
            type Parameters<'a> = (BorrowUtils::BorrowParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeBorrowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeBorrow((uint256,uint8,uint256))";
            const SELECTOR: [u8; 4] = [144u8, 126u8, 116u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BorrowUtils::BorrowParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeClose((uint256))` and selector `0xc56d4a2a`.
```solidity
function executeClose(CloseUtils.CloseParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCloseCall {
        pub params: <CloseUtils::CloseParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeClose((uint256))`](executeCloseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCloseReturn {}
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
            type UnderlyingSolTuple<'a> = (CloseUtils::CloseParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <CloseUtils::CloseParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeCloseCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCloseCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCloseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeCloseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeCloseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCloseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCloseCall {
            type Parameters<'a> = (CloseUtils::CloseParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeCloseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeClose((uint256))";
            const SELECTOR: [u8; 4] = [197u8, 109u8, 74u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <CloseUtils::CloseParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeDeposit((uint256,address,address,uint8))` and selector `0x47570cdc`.
```solidity
function executeDeposit(DepositUtils.DepositParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeDepositCall {
        pub params: <DepositUtils::DepositParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeDeposit((uint256,address,address,uint8))`](executeDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeDepositReturn {}
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
            type UnderlyingSolTuple<'a> = (DepositUtils::DepositParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <DepositUtils::DepositParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeDepositCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeDepositCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeDepositReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeDepositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeDepositCall {
            type Parameters<'a> = (DepositUtils::DepositParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeDeposit((uint256,address,address,uint8))";
            const SELECTOR: [u8; 4] = [71u8, 87u8, 12u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <DepositUtils::DepositParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeLiquidation((address,uint256))` and selector `0xb6103215`.
```solidity
function executeLiquidation(LiquidationUtils.LiquidationParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeLiquidationCall {
        pub params: <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeLiquidation((address,uint256))`](executeLiquidationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeLiquidationReturn {}
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
            type UnderlyingSolTuple<'a> = (LiquidationUtils::LiquidationParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeLiquidationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeLiquidationCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeLiquidationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeLiquidationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeLiquidationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeLiquidationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeLiquidationCall {
            type Parameters<'a> = (LiquidationUtils::LiquidationParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeLiquidationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeLiquidation((address,uint256))";
            const SELECTOR: [u8; 4] = [182u8, 16u8, 50u8, 21u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LiquidationUtils::LiquidationParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeLiquidationBatch((address,uint256)[])` and selector `0xa76718d2`.
```solidity
function executeLiquidationBatch(LiquidationUtils.LiquidationParams[] memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeLiquidationBatchCall {
        pub params: alloy::sol_types::private::Vec<
            <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`executeLiquidationBatch((address,uint256)[])`](executeLiquidationBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeLiquidationBatchReturn {}
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
                alloy::sol_types::sol_data::Array<LiquidationUtils::LiquidationParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<executeLiquidationBatchCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeLiquidationBatchCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeLiquidationBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeLiquidationBatchReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeLiquidationBatchReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeLiquidationBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeLiquidationBatchCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<LiquidationUtils::LiquidationParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeLiquidationBatchReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeLiquidationBatch((address,uint256)[])";
            const SELECTOR: [u8; 4] = [167u8, 103u8, 24u8, 210u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        LiquidationUtils::LiquidationParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
    /**Function with signature `executeRemove((address,address,uint256,address))` and selector `0xf3e01a4e`.
```solidity
function executeRemove(LiquidityUtils.RemoveParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRemoveCall {
        pub params: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeRemove((address,address,uint256,address))`](executeRemoveCall) function.
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
            type UnderlyingSolTuple<'a> = (LiquidityUtils::RemoveParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRemoveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            type Parameters<'a> = (LiquidityUtils::RemoveParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeRemoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeRemove((address,address,uint256,address))";
            const SELECTOR: [u8; 4] = [243u8, 224u8, 26u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <LiquidityUtils::RemoveParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeRepay((uint256,uint8,uint256,uint8))` and selector `0x7b123bc8`.
```solidity
function executeRepay(RepayUtils.RepayParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRepayCall {
        pub params: <RepayUtils::RepayParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeRepay((uint256,uint8,uint256,uint8))`](executeRepayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeRepayReturn {}
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
            type UnderlyingSolTuple<'a> = (RepayUtils::RepayParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <RepayUtils::RepayParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeRepayCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeRepayCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRepayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeRepayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeRepayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeRepayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeRepayCall {
            type Parameters<'a> = (RepayUtils::RepayParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeRepayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeRepay((uint256,uint8,uint256,uint8))";
            const SELECTOR: [u8; 4] = [123u8, 18u8, 59u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <RepayUtils::RepayParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeSwap((address,address,uint256,uint256,uint256,uint256,address))` and selector `0x34f5db7e`.
```solidity
function executeSwap(SwapUtils.SwapParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapCall {
        pub params: <SwapUtils::SwapParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeSwap((address,address,uint256,uint256,uint256,uint256,address))`](executeSwapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapReturn {}
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
            type UnderlyingSolTuple<'a> = (SwapUtils::SwapParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <SwapUtils::SwapParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeSwapCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeSwapCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeSwapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeSwapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeSwapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeSwapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeSwapCall {
            type Parameters<'a> = (SwapUtils::SwapParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeSwapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeSwap((address,address,uint256,uint256,uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [52u8, 245u8, 219u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <SwapUtils::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeSwapInPosition((uint256,uint256,uint256,uint256,uint256))` and selector `0xb1840675`.
```solidity
function executeSwapInPosition(SwapUtils.SwapInPositionParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapInPositionCall {
        pub params: <SwapUtils::SwapInPositionParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeSwapInPosition((uint256,uint256,uint256,uint256,uint256))`](executeSwapInPositionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapInPositionReturn {}
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
            type UnderlyingSolTuple<'a> = (SwapUtils::SwapInPositionParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <SwapUtils::SwapInPositionParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeSwapInPositionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeSwapInPositionCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeSwapInPositionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeSwapInPositionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeSwapInPositionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeSwapInPositionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeSwapInPositionCall {
            type Parameters<'a> = (SwapUtils::SwapInPositionParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeSwapInPositionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeSwapInPosition((uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [177u8, 132u8, 6u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <SwapUtils::SwapInPositionParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `executeWithdraw((uint256,uint8,uint256,address))` and selector `0xad0c9934`.
```solidity
function executeWithdraw(WithdrawUtils.WithdrawParams memory params) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeWithdrawCall {
        pub params: <WithdrawUtils::WithdrawParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeWithdraw((uint256,uint8,uint256,address))`](executeWithdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeWithdrawReturn {}
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
            type UnderlyingSolTuple<'a> = (WithdrawUtils::WithdrawParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <WithdrawUtils::WithdrawParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeWithdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeWithdrawCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeWithdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<executeWithdrawReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeWithdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeWithdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeWithdrawCall {
            type Parameters<'a> = (WithdrawUtils::WithdrawParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeWithdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeWithdraw((uint256,uint8,uint256,address))";
            const SELECTOR: [u8; 4] = [173u8, 12u8, 153u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <WithdrawUtils::WithdrawParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `liquidationHandler()` and selector `0xd25adeb3`.
```solidity
function liquidationHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct liquidationHandlerCall {}
    ///Container type for the return parameters of the [`liquidationHandler()`](liquidationHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct liquidationHandlerReturn {
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
            impl ::core::convert::From<liquidationHandlerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: liquidationHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for liquidationHandlerCall {
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
            impl ::core::convert::From<liquidationHandlerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: liquidationHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for liquidationHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for liquidationHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = liquidationHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "liquidationHandler()";
            const SELECTOR: [u8; 4] = [210u8, 90u8, 222u8, 179u8];
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
    /**Function with signature `liquidityHandler()` and selector `0xd9e08811`.
```solidity
function liquidityHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct liquidityHandlerCall {}
    ///Container type for the return parameters of the [`liquidityHandler()`](liquidityHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct liquidityHandlerReturn {
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
            impl ::core::convert::From<liquidityHandlerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: liquidityHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for liquidityHandlerCall {
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
            impl ::core::convert::From<liquidityHandlerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: liquidityHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for liquidityHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for liquidityHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = liquidityHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "liquidityHandler()";
            const SELECTOR: [u8; 4] = [217u8, 224u8, 136u8, 17u8];
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
    /**Function with signature `multicall(bytes[])` and selector `0xac9650d8`.
```solidity
function multicall(bytes[] memory data) external payable returns (bytes[] memory results);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallCall {
        pub data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
    }
    ///Container type for the return parameters of the [`multicall(bytes[])`](multicallCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multicallReturn {
        pub results: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallCall> for UnderlyingRustTuple<'_> {
                fn from(value: multicallCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
            impl ::core::convert::From<multicallReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multicallReturn) -> Self {
                    (value.results,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multicallReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { results: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multicallCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = multicallReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multicall(bytes[])";
            const SELECTOR: [u8; 4] = [172u8, 150u8, 80u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.data),
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
    /**Function with signature `repayHandler()` and selector `0x9d451d0c`.
```solidity
function repayHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct repayHandlerCall {}
    ///Container type for the return parameters of the [`repayHandler()`](repayHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct repayHandlerReturn {
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
            impl ::core::convert::From<repayHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: repayHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for repayHandlerCall {
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
            impl ::core::convert::From<repayHandlerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: repayHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for repayHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for repayHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = repayHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "repayHandler()";
            const SELECTOR: [u8; 4] = [157u8, 69u8, 29u8, 12u8];
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
    /**Function with signature `router()` and selector `0xf887ea40`.
```solidity
function router() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct routerCall {}
    ///Container type for the return parameters of the [`router()`](routerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct routerReturn {
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
            impl ::core::convert::From<routerCall> for UnderlyingRustTuple<'_> {
                fn from(value: routerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for routerCall {
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
            impl ::core::convert::From<routerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: routerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for routerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for routerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = routerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "router()";
            const SELECTOR: [u8; 4] = [248u8, 135u8, 234u8, 64u8];
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
    /**Function with signature `sendTokens(address,address,uint256)` and selector `0xe6d66ac8`.
```solidity
function sendTokens(address token, address receiver, uint256 amount) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendTokensCall {
        pub token: alloy::sol_types::private::Address,
        pub receiver: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`sendTokens(address,address,uint256)`](sendTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sendTokensReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<sendTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: sendTokensCall) -> Self {
                    (value.token, value.receiver, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sendTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        token: tuple.0,
                        receiver: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<sendTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: sendTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for sendTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sendTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sendTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sendTokens(address,address,uint256)";
            const SELECTOR: [u8; 4] = [230u8, 214u8, 106u8, 200u8];
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
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiver,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
    /**Function with signature `swapHandler()` and selector `0x8a53aaac`.
```solidity
function swapHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapHandlerCall {}
    ///Container type for the return parameters of the [`swapHandler()`](swapHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapHandlerReturn {
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
            impl ::core::convert::From<swapHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapHandlerCall {
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
            impl ::core::convert::From<swapHandlerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: swapHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapHandler()";
            const SELECTOR: [u8; 4] = [138u8, 83u8, 170u8, 172u8];
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
    /**Function with signature `withdrawHandler()` and selector `0x083473ef`.
```solidity
function withdrawHandler() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawHandlerCall {}
    ///Container type for the return parameters of the [`withdrawHandler()`](withdrawHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawHandlerReturn {
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
            impl ::core::convert::From<withdrawHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawHandlerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawHandlerCall {
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
            impl ::core::convert::From<withdrawHandlerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawHandlerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawHandlerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawHandlerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawHandler()";
            const SELECTOR: [u8; 4] = [8u8, 52u8, 115u8, 239u8];
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
    ///Container for all the [`ExchangeRouter`](self) function calls.
    pub enum ExchangeRouterCalls {
        borrowHandler(borrowHandlerCall),
        closeHandler(closeHandlerCall),
        dataStore(dataStoreCall),
        depositHandler(depositHandlerCall),
        executeAdd(executeAddCall),
        executeBorrow(executeBorrowCall),
        executeClose(executeCloseCall),
        executeDeposit(executeDepositCall),
        executeLiquidation(executeLiquidationCall),
        executeLiquidationBatch(executeLiquidationBatchCall),
        executeRemove(executeRemoveCall),
        executeRepay(executeRepayCall),
        executeSwap(executeSwapCall),
        executeSwapInPosition(executeSwapInPositionCall),
        executeWithdraw(executeWithdrawCall),
        liquidationHandler(liquidationHandlerCall),
        liquidityHandler(liquidityHandlerCall),
        multicall(multicallCall),
        repayHandler(repayHandlerCall),
        roleStore(roleStoreCall),
        router(routerCall),
        sendTokens(sendTokensCall),
        swapHandler(swapHandlerCall),
        withdrawHandler(withdrawHandlerCall),
    }
    #[automatically_derived]
    impl ExchangeRouterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 52u8, 115u8, 239u8],
            [33u8, 177u8, 8u8, 204u8],
            [52u8, 245u8, 219u8, 126u8],
            [71u8, 87u8, 12u8, 220u8],
            [74u8, 74u8, 123u8, 4u8],
            [102u8, 13u8, 13u8, 103u8],
            [123u8, 18u8, 59u8, 200u8],
            [138u8, 83u8, 170u8, 172u8],
            [144u8, 126u8, 116u8, 84u8],
            [156u8, 139u8, 44u8, 251u8],
            [157u8, 69u8, 29u8, 12u8],
            [167u8, 103u8, 24u8, 210u8],
            [168u8, 46u8, 212u8, 206u8],
            [172u8, 150u8, 80u8, 216u8],
            [173u8, 12u8, 153u8, 52u8],
            [177u8, 132u8, 6u8, 117u8],
            [182u8, 16u8, 50u8, 21u8],
            [197u8, 109u8, 74u8, 42u8],
            [210u8, 90u8, 222u8, 179u8],
            [217u8, 224u8, 136u8, 17u8],
            [230u8, 214u8, 106u8, 200u8],
            [237u8, 39u8, 175u8, 175u8],
            [243u8, 224u8, 26u8, 78u8],
            [248u8, 135u8, 234u8, 64u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ExchangeRouterCalls {
        const NAME: &'static str = "ExchangeRouterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::borrowHandler(_) => {
                    <borrowHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::closeHandler(_) => {
                    <closeHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::dataStore(_) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositHandler(_) => {
                    <depositHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeAdd(_) => {
                    <executeAddCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeBorrow(_) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeClose(_) => {
                    <executeCloseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeDeposit(_) => {
                    <executeDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeLiquidation(_) => {
                    <executeLiquidationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeLiquidationBatch(_) => {
                    <executeLiquidationBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeRemove(_) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeRepay(_) => {
                    <executeRepayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeSwap(_) => {
                    <executeSwapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeSwapInPosition(_) => {
                    <executeSwapInPositionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeWithdraw(_) => {
                    <executeWithdrawCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::liquidationHandler(_) => {
                    <liquidationHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::liquidityHandler(_) => {
                    <liquidityHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multicall(_) => {
                    <multicallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::repayHandler(_) => {
                    <repayHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::roleStore(_) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::router(_) => <routerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sendTokens(_) => {
                    <sendTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapHandler(_) => {
                    <swapHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawHandler(_) => {
                    <withdrawHandlerCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ExchangeRouterCalls>] = &[
                {
                    fn withdrawHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <withdrawHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::withdrawHandler)
                    }
                    withdrawHandler
                },
                {
                    fn executeAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeAdd)
                    }
                    executeAdd
                },
                {
                    fn executeSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeSwap)
                    }
                    executeSwap
                },
                {
                    fn executeDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeDeposit)
                    }
                    executeDeposit
                },
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn executeRepay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeRepayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeRepay)
                    }
                    executeRepay
                },
                {
                    fn swapHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <swapHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::swapHandler)
                    }
                    swapHandler
                },
                {
                    fn executeBorrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeBorrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeBorrow)
                    }
                    executeBorrow
                },
                {
                    fn depositHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <depositHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::depositHandler)
                    }
                    depositHandler
                },
                {
                    fn repayHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <repayHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::repayHandler)
                    }
                    repayHandler
                },
                {
                    fn executeLiquidationBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeLiquidationBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeLiquidationBatch)
                    }
                    executeLiquidationBatch
                },
                {
                    fn borrowHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <borrowHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::borrowHandler)
                    }
                    borrowHandler
                },
                {
                    fn multicall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <multicallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::multicall)
                    }
                    multicall
                },
                {
                    fn executeWithdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeWithdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeWithdraw)
                    }
                    executeWithdraw
                },
                {
                    fn executeSwapInPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeSwapInPositionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeSwapInPosition)
                    }
                    executeSwapInPosition
                },
                {
                    fn executeLiquidation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeLiquidationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeLiquidation)
                    }
                    executeLiquidation
                },
                {
                    fn executeClose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeCloseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeClose)
                    }
                    executeClose
                },
                {
                    fn liquidationHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <liquidationHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::liquidationHandler)
                    }
                    liquidationHandler
                },
                {
                    fn liquidityHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <liquidityHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::liquidityHandler)
                    }
                    liquidityHandler
                },
                {
                    fn sendTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <sendTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::sendTokens)
                    }
                    sendTokens
                },
                {
                    fn closeHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <closeHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::closeHandler)
                    }
                    closeHandler
                },
                {
                    fn executeRemove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <executeRemoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::executeRemove)
                    }
                    executeRemove
                },
                {
                    fn router(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterCalls> {
                        <routerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterCalls::router)
                    }
                    router
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
                Self::borrowHandler(inner) => {
                    <borrowHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::closeHandler(inner) => {
                    <closeHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositHandler(inner) => {
                    <depositHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeAdd(inner) => {
                    <executeAddCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::executeBorrow(inner) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeClose(inner) => {
                    <executeCloseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeDeposit(inner) => {
                    <executeDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeLiquidation(inner) => {
                    <executeLiquidationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeLiquidationBatch(inner) => {
                    <executeLiquidationBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeRemove(inner) => {
                    <executeRemoveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeRepay(inner) => {
                    <executeRepayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeSwap(inner) => {
                    <executeSwapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeSwapInPosition(inner) => {
                    <executeSwapInPositionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeWithdraw(inner) => {
                    <executeWithdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::liquidationHandler(inner) => {
                    <liquidationHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::liquidityHandler(inner) => {
                    <liquidityHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::repayHandler(inner) => {
                    <repayHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::router(inner) => {
                    <routerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sendTokens(inner) => {
                    <sendTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::swapHandler(inner) => {
                    <swapHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawHandler(inner) => {
                    <withdrawHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::borrowHandler(inner) => {
                    <borrowHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::closeHandler(inner) => {
                    <closeHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositHandler(inner) => {
                    <depositHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::executeBorrow(inner) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeClose(inner) => {
                    <executeCloseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeDeposit(inner) => {
                    <executeDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeLiquidation(inner) => {
                    <executeLiquidationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeLiquidationBatch(inner) => {
                    <executeLiquidationBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::executeRepay(inner) => {
                    <executeRepayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeSwap(inner) => {
                    <executeSwapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeSwapInPosition(inner) => {
                    <executeSwapInPositionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeWithdraw(inner) => {
                    <executeWithdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::liquidationHandler(inner) => {
                    <liquidationHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::liquidityHandler(inner) => {
                    <liquidityHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multicall(inner) => {
                    <multicallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::repayHandler(inner) => {
                    <repayHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::router(inner) => {
                    <routerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sendTokens(inner) => {
                    <sendTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapHandler(inner) => {
                    <swapHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawHandler(inner) => {
                    <withdrawHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ExchangeRouter`](self) custom errors.
    pub enum ExchangeRouterErrors {
        EmptyReceiver(EmptyReceiver),
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        Unauthorized(Unauthorized),
    }
    #[automatically_derived]
    impl ExchangeRouterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [62u8, 229u8, 174u8, 181u8],
            [163u8, 91u8, 21u8, 11u8],
            [213u8, 81u8, 130u8, 61u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ExchangeRouterErrors {
        const NAME: &'static str = "ExchangeRouterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyReceiver(_) => {
                    <EmptyReceiver as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ExchangeRouterErrors>] = &[
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn EmptyReceiver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExchangeRouterErrors> {
                        <EmptyReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExchangeRouterErrors::EmptyReceiver)
                    }
                    EmptyReceiver
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
                Self::EmptyReceiver(inner) => {
                    <EmptyReceiver as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyReceiver(inner) => {
                    <EmptyReceiver as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
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
    /**Creates a new wrapper around an on-chain [`ExchangeRouter`](self) contract instance.

See the [wrapper's documentation](`ExchangeRouterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ExchangeRouterInstance<T, P, N> {
        ExchangeRouterInstance::<T, P, N>::new(address, provider)
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
        _router: alloy::sol_types::private::Address,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _liquidityHandler: alloy::sol_types::private::Address,
        _swapHandler: alloy::sol_types::private::Address,
        _depositHandler: alloy::sol_types::private::Address,
        _borrowHandler: alloy::sol_types::private::Address,
        _repayHandler: alloy::sol_types::private::Address,
        _withdrawHandler: alloy::sol_types::private::Address,
        _liquidationHandler: alloy::sol_types::private::Address,
        _closeHandler: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ExchangeRouterInstance<T, P, N>>,
    > {
        ExchangeRouterInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _router,
            _roleStore,
            _dataStore,
            _liquidityHandler,
            _swapHandler,
            _depositHandler,
            _borrowHandler,
            _repayHandler,
            _withdrawHandler,
            _liquidationHandler,
            _closeHandler,
        )
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
        _router: alloy::sol_types::private::Address,
        _roleStore: alloy::sol_types::private::Address,
        _dataStore: alloy::sol_types::private::Address,
        _liquidityHandler: alloy::sol_types::private::Address,
        _swapHandler: alloy::sol_types::private::Address,
        _depositHandler: alloy::sol_types::private::Address,
        _borrowHandler: alloy::sol_types::private::Address,
        _repayHandler: alloy::sol_types::private::Address,
        _withdrawHandler: alloy::sol_types::private::Address,
        _liquidationHandler: alloy::sol_types::private::Address,
        _closeHandler: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ExchangeRouterInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _router,
            _roleStore,
            _dataStore,
            _liquidityHandler,
            _swapHandler,
            _depositHandler,
            _borrowHandler,
            _repayHandler,
            _withdrawHandler,
            _liquidationHandler,
            _closeHandler,
        )
    }
    /**A [`ExchangeRouter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ExchangeRouter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ExchangeRouterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ExchangeRouterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ExchangeRouterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ExchangeRouterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ExchangeRouter`](self) contract instance.

See the [wrapper's documentation](`ExchangeRouterInstance`) for more details.*/
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
            _router: alloy::sol_types::private::Address,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _liquidityHandler: alloy::sol_types::private::Address,
            _swapHandler: alloy::sol_types::private::Address,
            _depositHandler: alloy::sol_types::private::Address,
            _borrowHandler: alloy::sol_types::private::Address,
            _repayHandler: alloy::sol_types::private::Address,
            _withdrawHandler: alloy::sol_types::private::Address,
            _liquidationHandler: alloy::sol_types::private::Address,
            _closeHandler: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ExchangeRouterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _router,
                _roleStore,
                _dataStore,
                _liquidityHandler,
                _swapHandler,
                _depositHandler,
                _borrowHandler,
                _repayHandler,
                _withdrawHandler,
                _liquidationHandler,
                _closeHandler,
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
            _router: alloy::sol_types::private::Address,
            _roleStore: alloy::sol_types::private::Address,
            _dataStore: alloy::sol_types::private::Address,
            _liquidityHandler: alloy::sol_types::private::Address,
            _swapHandler: alloy::sol_types::private::Address,
            _depositHandler: alloy::sol_types::private::Address,
            _borrowHandler: alloy::sol_types::private::Address,
            _repayHandler: alloy::sol_types::private::Address,
            _withdrawHandler: alloy::sol_types::private::Address,
            _liquidationHandler: alloy::sol_types::private::Address,
            _closeHandler: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _router,
                            _roleStore,
                            _dataStore,
                            _liquidityHandler,
                            _swapHandler,
                            _depositHandler,
                            _borrowHandler,
                            _repayHandler,
                            _withdrawHandler,
                            _liquidationHandler,
                            _closeHandler,
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
    impl<T, P: ::core::clone::Clone, N> ExchangeRouterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ExchangeRouterInstance<T, P, N> {
            ExchangeRouterInstance {
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
    > ExchangeRouterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`borrowHandler`] function.
        pub fn borrowHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, borrowHandlerCall, N> {
            self.call_builder(&borrowHandlerCall {})
        }
        ///Creates a new call builder for the [`closeHandler`] function.
        pub fn closeHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, closeHandlerCall, N> {
            self.call_builder(&closeHandlerCall {})
        }
        ///Creates a new call builder for the [`dataStore`] function.
        pub fn dataStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, dataStoreCall, N> {
            self.call_builder(&dataStoreCall {})
        }
        ///Creates a new call builder for the [`depositHandler`] function.
        pub fn depositHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositHandlerCall, N> {
            self.call_builder(&depositHandlerCall {})
        }
        ///Creates a new call builder for the [`executeAdd`] function.
        pub fn executeAdd(
            &self,
            params: <LiquidityUtils::AddParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeAddCall, N> {
            self.call_builder(&executeAddCall { params })
        }
        ///Creates a new call builder for the [`executeBorrow`] function.
        pub fn executeBorrow(
            &self,
            params: <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeBorrowCall, N> {
            self.call_builder(&executeBorrowCall { params })
        }
        ///Creates a new call builder for the [`executeClose`] function.
        pub fn executeClose(
            &self,
            params: <CloseUtils::CloseParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeCloseCall, N> {
            self.call_builder(&executeCloseCall { params })
        }
        ///Creates a new call builder for the [`executeDeposit`] function.
        pub fn executeDeposit(
            &self,
            params: <DepositUtils::DepositParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeDepositCall, N> {
            self.call_builder(&executeDepositCall { params })
        }
        ///Creates a new call builder for the [`executeLiquidation`] function.
        pub fn executeLiquidation(
            &self,
            params: <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeLiquidationCall, N> {
            self.call_builder(&executeLiquidationCall { params })
        }
        ///Creates a new call builder for the [`executeLiquidationBatch`] function.
        pub fn executeLiquidationBatch(
            &self,
            params: alloy::sol_types::private::Vec<
                <LiquidationUtils::LiquidationParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeLiquidationBatchCall, N> {
            self.call_builder(
                &executeLiquidationBatchCall {
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`executeRemove`] function.
        pub fn executeRemove(
            &self,
            params: <LiquidityUtils::RemoveParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeRemoveCall, N> {
            self.call_builder(&executeRemoveCall { params })
        }
        ///Creates a new call builder for the [`executeRepay`] function.
        pub fn executeRepay(
            &self,
            params: <RepayUtils::RepayParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeRepayCall, N> {
            self.call_builder(&executeRepayCall { params })
        }
        ///Creates a new call builder for the [`executeSwap`] function.
        pub fn executeSwap(
            &self,
            params: <SwapUtils::SwapParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeSwapCall, N> {
            self.call_builder(&executeSwapCall { params })
        }
        ///Creates a new call builder for the [`executeSwapInPosition`] function.
        pub fn executeSwapInPosition(
            &self,
            params: <SwapUtils::SwapInPositionParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeSwapInPositionCall, N> {
            self.call_builder(
                &executeSwapInPositionCall {
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`executeWithdraw`] function.
        pub fn executeWithdraw(
            &self,
            params: <WithdrawUtils::WithdrawParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeWithdrawCall, N> {
            self.call_builder(&executeWithdrawCall { params })
        }
        ///Creates a new call builder for the [`liquidationHandler`] function.
        pub fn liquidationHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, liquidationHandlerCall, N> {
            self.call_builder(&liquidationHandlerCall {})
        }
        ///Creates a new call builder for the [`liquidityHandler`] function.
        pub fn liquidityHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, liquidityHandlerCall, N> {
            self.call_builder(&liquidityHandlerCall {})
        }
        ///Creates a new call builder for the [`multicall`] function.
        pub fn multicall(
            &self,
            data: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        ) -> alloy_contract::SolCallBuilder<T, &P, multicallCall, N> {
            self.call_builder(&multicallCall { data })
        }
        ///Creates a new call builder for the [`repayHandler`] function.
        pub fn repayHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, repayHandlerCall, N> {
            self.call_builder(&repayHandlerCall {})
        }
        ///Creates a new call builder for the [`roleStore`] function.
        pub fn roleStore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, roleStoreCall, N> {
            self.call_builder(&roleStoreCall {})
        }
        ///Creates a new call builder for the [`router`] function.
        pub fn router(&self) -> alloy_contract::SolCallBuilder<T, &P, routerCall, N> {
            self.call_builder(&routerCall {})
        }
        ///Creates a new call builder for the [`sendTokens`] function.
        pub fn sendTokens(
            &self,
            token: alloy::sol_types::private::Address,
            receiver: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, sendTokensCall, N> {
            self.call_builder(
                &sendTokensCall {
                    token,
                    receiver,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`swapHandler`] function.
        pub fn swapHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapHandlerCall, N> {
            self.call_builder(&swapHandlerCall {})
        }
        ///Creates a new call builder for the [`withdrawHandler`] function.
        pub fn withdrawHandler(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawHandlerCall, N> {
            self.call_builder(&withdrawHandlerCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ExchangeRouterInstance<T, P, N> {
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
