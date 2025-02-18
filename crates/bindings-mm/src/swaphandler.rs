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
/**

Generated by the following Solidity interface...
```solidity
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

interface SwapHandler {
    error AccountNotMatch(address accountInPosition, address account);
    error EmptyPool(bytes32 key);
    error EmptyPosition();
    error EmptySwapInAmount();
    error InsufficientSwapCollateral(uint256 amountIn, uint256 collateral);
    error MathOverflowedMulDiv();
    error RequestedAmount1ExceedsPriceLimit(uint256 amount1Out, uint256 amount1OutMax);
    error RequestedAmountOExceedsPriceLimit(uint256 amount0Out, uint256 amount0OutMax);
    error Reserve0Insufficient(uint256 amount0, uint256 availableReserve0);
    error Reserve1Insufficient(uint256 amount1, uint256 availableReserve1);
    error SingleTokenInOutSwapOnly();
    error Unauthorized(address msgSender, string role);

    constructor(address _roleStore, address _dataStore, address _eventEmitter);

    function dataStore() external view returns (address);
    function eventEmitter() external view returns (address);
    function executeSwap(address account, SwapUtils.SwapParams memory swapParams) external;
    function executeSwapInPosition(address account, SwapUtils.SwapInPositionParams memory swapParams) external;
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
    "name": "executeSwap",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "swapParams",
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
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "executeSwapInPosition",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "swapParams",
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
    "name": "AccountNotMatch",
    "inputs": [
      {
        "name": "accountInPosition",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
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
    "name": "EmptyPosition",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptySwapInAmount",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientSwapCollateral",
    "inputs": [
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "collateral",
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
    "name": "RequestedAmount1ExceedsPriceLimit",
    "inputs": [
      {
        "name": "amount1Out",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1OutMax",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "RequestedAmountOExceedsPriceLimit",
    "inputs": [
      {
        "name": "amount0Out",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount0OutMax",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "SingleTokenInOutSwapOnly",
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
pub mod SwapHandler {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b506040516182d53803806182d583398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c0516181ca61010b5f395f818160dd015281816101c601526102b201525f8181605e015261054c01525f818160b60152818161019701528181610283015281816103750152818161047b01526108e001526181ca5ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004617d87565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617dc5565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b506102376108de565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f19190617df6565b6001600160a01b031681526020018360200160208101906103129190617df6565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b9190617df6565b6001600160a01b03169052905061022e8382610997565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b190617e11565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104249190617e48565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b790617e11565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102379190617e48565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd9190617e5f565b61023757338160405163a35b150b60e01b8152600401610470929190617eac565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610dbe565b61061c617aa0565b61062e83835f01518460400151610de2565b606083015260408201819052825190518051516020919091015151610654929190610e03565b6020830152808252606083015160a0808401829052608085015160c08086018290528651928701519087015161068f9593949291905f610e44565b6102408601526101a0850152610180840152610100830181905260e08301829052604083015160a084015160c08501516106ce949293919290916110a6565b6106e0815f0151826102400151611133565b6102608201528051604082015160a083015160e08401516107109392915f916107099190617eff565b5f5f611162565b610733815f0151826040015160018460c001518561010001516107099190617eff565b60a08101511561074b578151815161074b91906113e6565b61076c815f01518260a001518360c001518460e0015185610100015161142b565b610160820181905282518251610781926114f7565b6107ad815f015182604001518360a001518460c001518560e001518661010001518761016001516119ef565b6107c3825f015182606001518360400151611a49565b6107d48260200151825f0151612cec565b8151602082015182516107e8929190612d10565b610809815f01518260a001518360c001518460e00151856101000151613ee5565b610220850181905261020085018290526001600160a01b039283166101e08601819052939092166101c085018190526020868101516040888101516102608901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108999792968b9693949193929091613fcd565b60208281015160408381015151805180519185015180518489015183880151938601519783015192909501516108d9978b96600496959394909392614084565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161091c90617e11565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109949190617e48565b50565b6109c36040518060400160405280600b81526020016a065786563757465537761760ac1b815250610dbe565b6109cb617b4e565b6109e1825f015183604001518460600151610e03565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af1158015610a3f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a639190617e48565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610aba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ade9190617e48565b60808201526060810151158015610af757506080810151155b15610b1557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511015610b3057608082015160608201525b8160a0015181608001511015610b4b5760a082015160808201525b610b71825f0151825f0151836060015184608001518660c001518760e001516001610e44565b6101c0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610ba591611133565b6101e082015260a081015115610c2f576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c18575f5ffd5b505af1158015610c2a573d5f5f3e3d5ffd5b505050505b60c081015115610cb457604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c9d575f5ffd5b505af1158015610caf573d5f5f3e3d5ffd5b505050505b610cc58260200151825f0151612cec565b606081015115610cdd5781518151610cdd91906113e6565b815160208201518251610cf1929190612d10565b610d11815f0151826060015183608001518460a001518560c00151613ee5565b6101a08501526101808401526001600160a01b039081166101608401521661014082015280516060820151608083015160a084015160c0850151610d58949392919061142b565b610120820181905282518251610d6d926114f7565b6108d9826020015184836101400151846101600151627a1200866101800151876101a00151886101e0015160405180608001604052805f81526020015f81526020015f81526020015f815250613fcd565b61099460405180604001604052806002815260200161257360f01b81525082614122565b610dea617bdd565b5f610df6858585614167565b915091505b935093915050565b610e0b617c03565b5f5f610e178585614194565b90505f610e24878361423c565b9050610e30818361543e565b610e398161546c565b969095509350505050565b5f5f5f5f5f610ea16040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8a118015610eae575087155b15610f8957610ebf8d8b8e8a615518565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e08201819052891115610f2e5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610f459190617f25565b1115610f84576101008101518151610f5d9190617f25565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b611073565b5f8b118015610f96575088155b1561105a57610fa78d8c8e8a615644565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e0820181905288111561101e57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610f845760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805f0151816020015182604001518360600151846101000151955095509550955095505097509750975097509792505050565b5f831180156110b3575080155b1561110257845160209081015101518311156110fd578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b61112c565b5f8411801561110f575081155b1561112c578451516020015184111561112c57845184905f6110d0565b5050505050565b60608201515f9060481c61ffff168161114c8483615755565b90506111588582615779565b9150505b92915050565b5f61116c84615796565b90505f84126111da578551819060ff87166002811061118d5761118d617ed7565b60200201516020018181516111a29190617f25565b9052508651819060ff8716600281106111bd576111bd617ed7565b60200201516060018181516111d29190617f25565b90525061123b565b8551819060ff8716600281106111f2576111f2617ed7565b60200201516020018181516112079190617f38565b9052508651819060ff87166002811061122257611222617ed7565b60200201516060018181516112379190617f38565b9052505b81156112be5785515f9060ff87166002811061125957611259617ed7565b602002015160400151905080885f01518760ff166002811061127d5761127d617ed7565b602002015160a0018181516112929190617f38565b90525086515f9060ff8816600281106112ad576112ad617ed7565b602002015160400152506113de9050565b825f036112cb57506113de565b5f6112d584615796565b90505f61130b895f01518860ff16600281106112f3576112f3617ed7565b602002015160200151836157ab90919063ffffffff16565b90505f8512611379578751819060ff89166002811061132c5761132c617ed7565b60200201516040018181516113419190617f25565b9052508851819060ff89166002811061135c5761135c617ed7565b602002015160a0018181516113719190617f25565b9052506113da565b8751819060ff89166002811061139157611391617ed7565b60200201516040018181516113a69190617f38565b9052508851819060ff8916600281106113c1576113c1617ed7565b602002015160a0018181516113d69190617f38565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f61140484846157e6565b9150915081811061142557606083015161141f906001615847565b60608401525b50505050565b5f5f5f5f8611801561143b575083155b1561144a575083905084611462565b5f87118015611457575084155b1561105a5750859050825b5f61147189606001515f615871565b90505f6114838a606001516001615871565b90505f6114a685676765c793fa10079d601b1b6114a186600a618026565b61589f565b90505f6114c485676765c793fa10079d601b1b6114a186600a618026565b9050805f036114db575f96505050505050506114ee565b6114e582826157ab565b96505050505050505b95945050505050565b6115276040518060400160405280600f81526020016e75706461746554776170507269636560881b815250610dbe565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152611581848461595f565b63ffffffff16815261159864010000000042618045565b63ffffffff90811660208301528151165f0361164e576115bd848483602001516159d5565b6115c884845f615a59565b6115d784848360200151615a98565b6115e284845f615ab0565b6115ed848484615ac8565b6116276040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16615ae0565b61142560405180604001604052806005815260200164707269636560d81b81525083615ae0565b8051602082015161165f9190618058565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b6020820152905161169b9216615ae0565b604081015163ffffffff16156117cb576116b58484615b09565b606082015260408101516116cf9063ffffffff1683618074565b81606001516116de9190617f25565b6080820152604080518082019091526005815264707269636560d81b60208201526117099083615ae0565b6117406040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16615ae0565b611779604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151615ae0565b6117ae6040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151615ae0565b6117bd84848360800151615a59565b6117cb8484835f01516159d5565b6117d58484615b22565b63ffffffff1660a0820181905260208201516117f19190618058565b63ffffffff1660c082015261180584615b3b565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b6020820152905161183d9216615ae0565b6118866040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16615ae0565b6118c5604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16615ae0565b8060e0015163ffffffff168160c0015163ffffffff161115611425576118eb8484615b09565b60808201526118fa8484615bed565b610100820181905260c0820151608083015163ffffffff9091169161191e91617f38565b611928919061808b565b61012082015260208082015163ffffffff1660a08301526080820151610100830190815260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f640000000000928101929092525161198d9190615ae0565b6119c06040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151615ae0565b6119cf84848360a00151615a98565b6119df8484836101000151615ab0565b6114258484836101200151615ac8565b5f6119fa8584617eff565b90505f811315611a2457865160200151611a1f9083611a1884615796565b6001615c06565b611a3f565b865160200151611a3f9083611a3884615796565b6001615d24565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611a8d906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611add575f5ffd5b505af1158015611aef573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62611b0f8460400151615e37565b856040518363ffffffff1660e01b8152600401611b36929190918252602082015260400190565b5f604051808303815f87803b158015611b4d575f5ffd5b505af1158015611b5f573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611b9d906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611bcd929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611c0e929190918252602082015260400190565b6020604051808303815f875af1158015611c2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c4e9190617e48565b50806001600160a01b031663ca446dd984604051602001611c8e906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cbe929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611d09926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015611d25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d49919061809e565b50806001600160a01b031663ca446dd984604051602001611d89906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611db9929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611e1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e40919061809e565b50806001600160a01b031663e2a4853a84604051602001611e859060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611eb5929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611f12573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f369190617e48565b50806001600160a01b031663e2a4853a84604051602001611f7b9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fab929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612007573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061202b9190617e48565b50806001600160a01b031663e2a4853a84604051602001612076906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016120a6929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612103573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121279190617e48565b50806001600160a01b031663e2a4853a84604051602001612171906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016121a1929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156121fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122229190617e48565b50806001600160a01b031663e2a4853a8460405160200161226e906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161229e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156122fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231f9190617e48565b50806001600160a01b031663e2a4853a8460405160200161236a906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161239a929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156123f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241b9190617e48565b50806001600160a01b031663e2a4853a8460405160200161245a906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161248a929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156124e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061250c9190617e48565b50806001600160a01b031663ca446dd98460405160200161254c906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161257c929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156125e2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612606919061809e565b50806001600160a01b031663e2a4853a8460405160200161264b9060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161267b929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156126da573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126fe9190617e48565b50806001600160a01b031663e2a4853a846040516020016127439060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001612773929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156127d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f69190617e48565b50806001600160a01b031663e2a4853a8460405160200161284190602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612871929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128d1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128f59190617e48565b50806001600160a01b031663e2a4853a8460405160200161293f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161296f929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156129cf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129f39190617e48565b50806001600160a01b031663e2a4853a84604051602001612a3f90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a6f929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612acf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612af39190617e48565b50806001600160a01b031663e2a4853a84604051602001612b3e90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b6e929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612bce573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf29190617e48565b50806001600160a01b031663e2a4853a84604051602001612c31906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c61929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612cac929190918252602082015260400190565b6020604051808303815f875af1158015612cc8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061112c9190617e48565b612cf581615ebb565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001612d50906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612da0575f5ffd5b505af1158015612db2573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001612df6906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e26929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612e89573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ead919061809e565b50806001600160a01b031663e2a4853a84604051602001612ef5906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f25929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612f82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa69190617e48565b50806001600160a01b031663e2a4853a84604051602001612fed906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b6040516020818303038152906040528051906020012060405160200161301d929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015613079573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061309d9190617e48565b50806001600160a01b031663e2a4853a846040516020016130e9906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613119929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613176573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061319a9190617e48565b50806001600160a01b031663e2a4853a846040516020016131ba906180b9565b604051602081830303815290604052805190602001206040516020016131ea929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613247573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061326b9190617e48565b50806001600160a01b031663e2a4853a846040516020016132b8906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016132e8929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613345573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133699190617e48565b50806001600160a01b031663e2a4853a846040516020016133b2906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016133e2929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561343f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134639190617e48565b50806001600160a01b031663ca446dd9846040516020016134a4906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016134d4929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561353a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061355e919061809e565b50806001600160a01b031663e2a4853a846040516020016135a690602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016135d6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015613635573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136599190617e48565b50806001600160a01b031663e2a4853a846040516020016136a090602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016136d0929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561372f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137539190617e48565b50806001600160a01b031663e2a4853a8460405160200161379f90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016137cf929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561382f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138539190617e48565b50806001600160a01b031663e2a4853a84604051602001613873906180fa565b604051602081830303815290604052805190602001206040516020016138a3929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613903573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139279190617e48565b50806001600160a01b031663e2a4853a8460405160200161397490602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016139a4929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613a04573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a289190617e48565b50806001600160a01b031663e2a4853a84604051602001613a7190602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613aa1929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b259190617e48565b50806001600160a01b031663ca446dd984604051602001613b6390602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b93929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401613bdd9291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613bf9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c1d919061809e565b50806001600160a01b031663ca446dd984604051602001613c6f906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613c9f929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613cea926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613d06573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d2a919061809e565b50806001600160a01b031663e2a4853a84604051602001613d71906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613da1929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613de2929190918252602082015260400190565b6020604051808303815f875af1158015613dfe573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e229190617e48565b50806001600160a01b031663e2a4853a84604051602001613e74906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613ea4929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612cac929190918252602082015260400190565b5f5f5f5f613f2260405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f88118015613f2f575085155b15613f66578951602090810151516001600160a01b0390811683528b51515116908201526040810188905260608101879052613fa8565b5f89118015613f73575086155b15613fa857895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b158015614063575f5ffd5b505af1158015614075573d5f5f3e3d5ffd5b50505050505050505050505050565b6040516304e6bdd160e11b81526001600160a01b038a81166004830152602482018a9052888116604483015287811660648301526084820187905260a4820186905260c4820185905260e4820184905261010482018390528b16906309cd7ba290610124015f604051808303815f87803b158015614100575f5ffd5b505af1158015614112573d5f5f3e3d5ffd5b5050505050505050505050505050565b610237828260405160240161413892919061813b565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b1790526160ca565b61416f617bdd565b5f5f61417b86856160d3565b90505f6141888683616139565b9050610e39878261614b565b5f816001600160a01b0316836001600160a01b0316106141b55781836141b8565b82825b60405191945092506141e5906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b614244617c03565b8261424d617c03565b816001600160a01b03166391d4403c604051602001614289906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156142dd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143019190617e5f565b61430e57915061115c9050565b816001600160a01b03166321f8a7218560405160200161434e906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161437e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016143b291815260200190565b602060405180830381865afa1580156143cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143f1919061809e565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161446f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144a391815260200190565b602060405180830381865afa1580156144be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144e29190617e48565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614538906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001614568929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161459c91815260200190565b602060405180830381865afa1580156145b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145db9190617e48565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614636906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614666929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161469a91815260200190565b602060405180830381865afa1580156146b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146d99190617e48565b815151606001526040516001600160a01b0383169063bd02d0f5908690614702906020016180b9565b60405160208183030381529060405280519060200120604051602001614732929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161476691815260200190565b602060405180830381865afa158015614781573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147a59190617e48565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614801906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001614831929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161486591815260200190565b602060405180830381865afa158015614880573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148a49190617e48565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614921929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161495591815260200190565b602060405180830381865afa158015614970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149949190617e48565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614a09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a3d91815260200190565b602060405180830381865afa158015614a58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a7c919061809e565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b2691815260200190565b602060405180830381865afa158015614b41573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b659190617e48565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614bbc90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614c2091815260200190565b602060405180830381865afa158015614c3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c5f9190617e48565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614cbb90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ceb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d1f91815260200190565b602060405180830381865afa158015614d3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d5e9190617e48565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614d8e906180fa565b60405160208183030381529060405280519060200120604051602001614dbe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614df291815260200190565b602060405180830381865afa158015614e0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e319190617e48565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001614e8e90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ebe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ef291815260200190565b602060405180830381865afa158015614f0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f319190617e48565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001614f8a90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001614fba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fee91815260200190565b602060405180830381865afa158015615009573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061502d9190617e48565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161507b90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016150ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150df91815260200190565b602060405180830381865afa1580156150fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061511e919061809e565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a7218560405160200161518c906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016151bc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151f091815260200190565b602060405180830381865afa15801561520b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061522f919061809e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001615292906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016152c2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152f691815260200190565b602060405180830381865afa158015615311573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153359190617e48565b60608201526040516001600160a01b0383169063bd02d0f590869061538e906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016153be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153f291815260200190565b602060405180830381865afa15801561540d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154319190617e48565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b6080810151429081900361547e575050565b81515160a00151156154d45781515f906154a790825b60200201516040015184608001516161c5565b83519091506154cb9082905f5b6020020151602001516161f990919063ffffffff16565b83515160200152505b81516020015160a00151156155105781515f906154f2906001615494565b835190915061550490829060016154b4565b83516020908101510152505b608090910152565b5f5f5f5f615524617c37565b61552d8961623a565b6101c082018190526155449088905f90819061628b565b5060408401525081526101c08101516155639088906001905f9061628b565b5060608401525060208201528051158061557f57506020810151155b15615596575f5f5f5f945094509450945050615639565b85156155b25787816020018181516155ae9190617f38565b9052505b805160208201516155c891906114a1818c616370565b6080820181905281516155da916163c4565b60a0820152606087015160381c61ffff16610140820181905260a08201516156119161560990612710906163c4565b61271061589f565b6040820151606083015161014084015160a085015161562f91615755565b9450945094509450505b945094509450949050565b5f5f5f5f615650617c37565b6156598961623a565b6101c082018190526156709088905f90819061628b565b5060408401525081526101c081015161568f9088906001905f9061628b565b506060840152506020820152805115806156ab57506020810151155b156156c2575f5f5f5f945094509450945050615639565b85156156dd5787815f018181516156d99190617f38565b9052505b606087015160381c61ffff16610140820181905261570490899061560990612710906163c4565b610160820181905281516020830151615722926114a1908390616370565b608082018190526020820151615737916163c4565b60c082018190526040820151606083015161014084015161562f908c905b5f8115611388198390048411151761576b575f5ffd5b506127109102611388010490565b81515160c001805182919061578f908390617f25565b9052505050565b5f5f8212156157a757815f0361115c565b5090565b5f8115676765c793fa10079d601b1b600284041904841117156157cc575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f6157f784606001515f615871565b90505f61580386616419565b90505f6158268261581585600a618026565b676765c793fa10079d601b1b61589f565b90505f615835875f5f5f61628b565b50939a91995090975050505050505050565b5f603382615855575f615858565b60015b60ff16901b660800000000000019841617905092915050565b5f60ff60581b1960585f1960ff851601615891575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f036158d3578382816158c9576158c9618031565b0492505050615958565b8084116158f35760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f826001600160a01b031663bd02d0f56159788461646a565b6040518263ffffffff1660e01b815260040161599691815260200190565b602060405180830381865afa1580156159b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159589190617e48565b826001600160a01b031663e2a4853a6159ed8461646a565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015615a35573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114259190617e48565b826001600160a01b031663e2a4853a615a718461651d565b836040518363ffffffff1660e01b8152600401615a19929190918252602082015260400190565b826001600160a01b031663e2a4853a6159ed84616571565b826001600160a01b031663e2a4853a615a71846165d2565b826001600160a01b031663e2a4853a615a7184616638565b610237604051806040016040528060068152602001652573202d257360d01b8152508383616681565b5f826001600160a01b031663bd02d0f56159788461651d565b5f826001600160a01b031663bd02d0f561597884616571565b5f816001600160a01b031663bd02d0f5604051602001615b7a906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615bae91815260200190565b602060405180830381865afa158015615bc9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061115c9190617e48565b5f826001600160a01b031663bd02d0f5615978846165d2565b60e084015160011901615c2d57600160e085015260a0840182905260608401839052611425565b60e08401515f1901615c9f5760a084018051908390615c4c8284617f25565b9052508115615c99575f615c6085856161f9565b6060870151615c6f90846161f9565b615c799190617f25565b9050615c928660a00151826157ab90919063ffffffff16565b6060870152505b50611425565b60e084015161142557818460c001511115615cce57818460c001818151615cc69190617f38565b905250611425565b818460c0015103615cf257600260e08501525f60c085018190526080850152611425565b600160e085015260c0840151615d089083617f38565b60a0850152505060608201525f60c08201819052608090910152565b60e084015160011901615d4a575f60e085015260c0840182905260808401839052611425565b60e0840151615db85760c084018051908390615d668284617f25565b9052508115615c99575f615d7a85856161f9565b6080870151615d8990846161f9565b615d939190617f25565b9050615dac8660c00151826157ab90919063ffffffff16565b60808701525050611425565b60e08401515f190161142557818460a001511115615de257818460a001818151615cc69190617f38565b818460a0015103615e0657600260e08501525f60a085018190526060850152611425565b5f60e085015260a0840151615e1b9083617f38565b60c0850152505060808201525f60a08201819052606090910152565b5f604051602001615e71906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f615ef06040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b615efa845f6166c8565b602083015281526060840151615f10905f615871565b606082018190528151615f3591676765c793fa10079d601b1b906114a190600a618026565b604082015260208101515f03615f50575f6080820152615ff0565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015615fc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615fea9190617e48565b60808201525b615ffb8460016166c8565b602083018190529082525f03616016575f60a08201526160b6565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561608c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906160b09190617e48565b60a08201525b80608001518160a001519250925050915091565b6109948161670e565b5f604051602001616100906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b038516908201526060810183905260800161421e565b616141617bdd565b615958838361672d565b60408101516001600160a01b031661617657604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f806161d18342617f38565b6161db9085618074565b6301e133809004905061115881676765c793fa10079d601b1b617f25565b5f81156b019d971e4fe8401e740000001983900484111517616219575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f816001600160a01b031663bd02d0f5604051602001615b7a906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff16600281106162a8576162a8617ed7565b602002015190505f6162ba8a8a61793f565b9050805f036162d6575f5f5f5f95509550955095505050615639565b5f6162e5838c60800151617a2d565b90505f6162f2828a6161f9565b90505f8915616317578184106163115761630c84836163c4565b616319565b5f616319565b5f5b90505f616326858d6161f9565b90505f8c1561634b578482106163455761634082866163c4565b61634d565b5f61634d565b5f5b90506163598587617f25565b9f959e50919c50909a509298505050505050505050565b5f8261637c8382617f25565b915081101561115c5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f826163d08382617f38565b915081111561115c5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f816001600160a01b031663bd02d0f5604051602001615b7a9060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b80515f90819061648b90825b60200201515184516001602002015151614194565b9050806040516020016164cf906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016164ff929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f90819061652d9082616476565b9050806040516020016164cf906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f9081906165819082616476565b9050806040516020016164cf9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906165e29082616476565b9050806040516020016164cf9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906166489082616476565b9050806040516020016164cf90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b6108d98383836040516024016166999392919061815f565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b1790526160ca565b5f5f5f6166f5855f01518560ff16600281106166e6576166e6617ed7565b60200201518660800151617a2d565b90505f616702868661793f565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b616735617bdd565b8261673e617bdd565b816001600160a01b03166391d4403c60405160200161677e906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156167d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906167f69190617e5f565b61680357915061115c9050565b816001600160a01b031663bd02d0f58560405160200161683d906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161686d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016168a191815260200190565b602060405180830381865afa1580156168bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906168e09190617e48565b816020018181525050816001600160a01b03166321f8a72185604051602001616928906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616958929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161698c91815260200190565b602060405180830381865afa1580156169a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906169cb919061809e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001616a27906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616a57929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616a8b91815260200190565b602060405180830381865afa158015616aa6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616aca919061809e565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616b45929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616b7991815260200190565b602060405180830381865afa158015616b94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616bb89190617e48565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001616c0c9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001616c3c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616c7091815260200190565b602060405180830381865afa158015616c8b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616caf9190617e48565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001616d09906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616d39929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616d6d91815260200190565b602060405180830381865afa158015616d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616dac9190617e48565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001616e05906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616e35929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616e6991815260200190565b602060405180830381865afa158015616e84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616ea89190617e48565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616f28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616f5c91815260200190565b602060405180830381865afa158015616f77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616f9b9190617e48565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001616ff5906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001617025929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161705991815260200190565b602060405180830381865afa158015617074573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906170989190617e48565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161710b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161713f91815260200190565b602060405180830381865afa15801561715a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061717e9190617e48565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016171f2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161722691815260200190565b602060405180830381865afa158015617241573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617265919061809e565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161730c91815260200190565b602060405180830381865afa158015617327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061734b9190617e48565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016173a09060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016173d0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161740491815260200190565b602060405180830381865afa15801561741f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906174439190617e48565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161749e90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016174ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161750291815260200190565b602060405180830381865afa15801561751d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906175419190617e48565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161759b90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016175cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016175ff91815260200190565b602060405180830381865afa15801561761a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061763e9190617e48565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161769a90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016176ca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016176fe91815260200190565b602060405180830381865afa158015617719573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061773d9190617e48565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161779890602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016177c8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016177fc91815260200190565b602060405180830381865afa158015617817573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061783b9190617e48565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f58560405160200161788a906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016178ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016178ee91815260200190565b602060405180830381865afa158015617909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061792d9190617e48565b81516020015160e00152949350505050565b5f5f835f01518360ff166002811061795957617959617ed7565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156179b2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906179d69190617e48565b9050805f036179e9575f9250505061115c565b606082015160c08301516179fd9082617f25565b8210617a215760c0830151617a128284617f38565b617a1c9190617f38565b617a23565b5f5b9695505050505050565b5f8260a001515f03617a4057505f61115c565b5f617a4b8484617a5d565b60a085015190915061115890826161f9565b5f428203617a705750602082015161115c565b5f617a7f8460400151846161c5565b9050617a988460200151826161f990919063ffffffff16565b91505061115c565b604051806102800160405280617ab4617c03565b81526020015f8152602001617ac7617bdd565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806102000160405280617b62617c03565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280617bf0617c9e565b81525f6020820181905260409091015290565b6040518060a00160405280617c16617d0c565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b617cf66040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617cad5790505090565b60405180604001604052806002905b617d5d6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617d1b5790505090565b6001600160a01b0381168114610994575f5ffd5b5f5f82840360c0811215617d99575f5ffd5b8335617da481617d73565b925060a0601f1982011215617db7575f5ffd5b506020830190509250929050565b5f5f828403610100811215617dd8575f5ffd5b8335617de381617d73565b925060e0601f1982011215617db7575f5ffd5b5f60208284031215617e06575f5ffd5b813561595881617d73565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215617e58575f5ffd5b5051919050565b5f60208284031215617e6f575f5ffd5b81518015158114615958575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90617ecf90830184617e7e565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f831280158383131683831282161715617f1e57617f1e617eeb565b5092915050565b8082018082111561115c5761115c617eeb565b8181038181111561115c5761115c617eeb565b6001815b6001841115610dfb57808504811115617f6a57617f6a617eeb565b6001841615617f7857908102905b60019390931c928002617f4f565b5f82617f945750600161115c565b81617fa057505f61115c565b8160018114617fb65760028114617fc057617fdc565b600191505061115c565b60ff841115617fd157617fd1617eeb565b50506001821b61115c565b5060208310610133831016604e8410600b8410161715617fff575081810a61115c565b61800b5f198484617f4b565b805f190482111561801e5761801e617eeb565b029392505050565b5f6159588383617f86565b634e487b7160e01b5f52601260045260245ffd5b5f8261805357618053618031565b500690565b63ffffffff828116828216039081111561115c5761115c617eeb565b808202811582820484141761115c5761115c617eeb565b5f8261809957618099618031565b500490565b5f602082840312156180ae575f5ffd5b815161595881617d73565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61814d6040830185617e7e565b82810360208401526114ee8185617e7e565b606081525f6181716060830186617e7e565b82810360208401526181838186617e7e565b91505082604083015294935050505056fea26469706673582212201d1fe5e07a1921b59f774ad1db097c7c6867b1e45df290a4321ad85f0f6b13f464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x82\xD58\x03\x80a\x82\xD5\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x81\xCAa\x01\x0B_9_\x81\x81`\xDD\x01R\x81\x81a\x01\xC6\x01Ra\x02\xB2\x01R_\x81\x81`^\x01Ra\x05L\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\x83\x01R\x81\x81a\x03u\x01R\x81\x81a\x04{\x01Ra\x08\xE0\x01Ra\x81\xCA_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04a}\x87V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04a}\xC5V[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08\xDEV[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x97V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90a~\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a~HV[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90a~\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90a~HV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90a~_V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90a~\xACV[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\xBEV[a\x06\x1Caz\xA0V[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r\xE2V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\x0E\x03V[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x86Q\x92\x87\x01Q\x90\x87\x01Qa\x06\x8F\x95\x93\x94\x92\x91\x90_a\x0EDV[a\x02@\x86\x01Ra\x01\xA0\x85\x01Ra\x01\x80\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCE\x94\x92\x93\x91\x92\x90\x91a\x10\xA6V[a\x06\xE0\x81_\x01Q\x82a\x02@\x01Qa\x113V[a\x02`\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x10\x93\x92\x91_\x91a\x07\t\x91\x90a~\xFFV[__a\x11bV[a\x073\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\t\x91\x90a~\xFFV[`\xA0\x81\x01Q\x15a\x07KW\x81Q\x81Qa\x07K\x91\x90a\x13\xE6V[a\x07l\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa\x14+V[a\x01`\x82\x01\x81\x90R\x82Q\x82Qa\x07\x81\x92a\x14\xF7V[a\x07\xAD\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87a\x01`\x01Qa\x19\xEFV[a\x07\xC3\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x1AIV[a\x07\xD4\x82` \x01Q\x82_\x01Qa,\xECV[\x81Q` \x82\x01Q\x82Qa\x07\xE8\x92\x91\x90a-\x10V[a\x08\t\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa>\xE5V[a\x02 \x85\x01\x81\x90Ra\x02\0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xE0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xC0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02`\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08\x99\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91a?\xCDV[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80Q\x80Q\x91\x85\x01Q\x80Q\x84\x89\x01Q\x83\x88\x01Q\x93\x86\x01Q\x97\x83\x01Q\x92\x90\x95\x01Qa\x08\xD9\x97\x8B\x96`\x04\x96\x95\x93\x94\x90\x93\x92a@\x84V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\t\x1C\x90a~\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90a~HV[PV[a\t\xC3`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\xBEV[a\t\xCBa{NV[a\t\xE1\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\x0E\x03V[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a~HV[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDE\x91\x90a~HV[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\n\xF7WP`\x80\x81\x01Q\x15[\x15a\x0B\x15W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x10\x15a\x0B0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x10\x15a\x0BKW`\xA0\x82\x01Q`\x80\x82\x01R[a\x0Bq\x82_\x01Q\x82_\x01Q\x83``\x01Q\x84`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`\x01a\x0EDV[a\x01\xC0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B\xA5\x91a\x113V[a\x01\xE0\x82\x01R`\xA0\x81\x01Q\x15a\x0C/W`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x18W__\xFD[PZ\xF1\x15\x80\x15a\x0C*W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C\xB4W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xAFW=__>=_\xFD[PPPP[a\x0C\xC5\x82` \x01Q\x82_\x01Qa,\xECV[``\x81\x01Q\x15a\x0C\xDDW\x81Q\x81Qa\x0C\xDD\x91\x90a\x13\xE6V[\x81Q` \x82\x01Q\x82Qa\x0C\xF1\x92\x91\x90a-\x10V[a\r\x11\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa>\xE5V[a\x01\xA0\x85\x01Ra\x01\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x84\x01R\x16a\x01@\x82\x01R\x80Q``\x82\x01Q`\x80\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\rX\x94\x93\x92\x91\x90a\x14+V[a\x01 \x82\x01\x81\x90R\x82Q\x82Qa\rm\x92a\x14\xF7V[a\x08\xD9\x82` \x01Q\x84\x83a\x01@\x01Q\x84a\x01`\x01Qbz\x12\0\x86a\x01\x80\x01Q\x87a\x01\xA0\x01Q\x88a\x01\xE0\x01Q`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPa?\xCDV[a\t\x94`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aA\"V[a\r\xEAa{\xDDV[_a\r\xF6\x85\x85\x85aAgV[\x91P\x91P[\x93P\x93\x91PPV[a\x0E\x0Ba|\x03V[__a\x0E\x17\x85\x85aA\x94V[\x90P_a\x0E$\x87\x83aB<V[\x90Pa\x0E0\x81\x83aT>V[a\x0E9\x81aTlV[\x96\x90\x95P\x93PPPPV[_____a\x0E\xA1`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8A\x11\x80\x15a\x0E\xAEWP\x87\x15[\x15a\x0F\x89Wa\x0E\xBF\x8D\x8B\x8E\x8AaU\x18V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R\x89\x11\x15a\x0F.W\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0FE\x91\x90a\x7F%V[\x11\x15a\x0F\x84Wa\x01\0\x81\x01Q\x81Qa\x0F]\x91\x90a\x7F%V[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x10sV[_\x8B\x11\x80\x15a\x0F\x96WP\x88\x15[\x15a\x10ZWa\x0F\xA7\x8D\x8C\x8E\x8AaVDV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R\x88\x11\x15a\x10\x1EW` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0F\x84W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84a\x01\0\x01Q\x95P\x95P\x95P\x95P\x95PP\x97P\x97P\x97P\x97P\x97\x92PPPV[_\x83\x11\x80\x15a\x10\xB3WP\x80\x15[\x15a\x11\x02W\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x10\xFDW\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x11,V[_\x84\x11\x80\x15a\x11\x0FWP\x81\x15[\x15a\x11,W\x84QQ` \x01Q\x84\x11\x15a\x11,W\x84Q\x84\x90_a\x10\xD0V[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x11L\x84\x83aWUV[\x90Pa\x11X\x85\x82aWyV[\x91PP[\x92\x91PPV[_a\x11l\x84aW\x96V[\x90P_\x84\x12a\x11\xDAW\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\x8DWa\x11\x8Da~\xD7V[` \x02\x01Q` \x01\x81\x81Qa\x11\xA2\x91\x90a\x7F%V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xBDWa\x11\xBDa~\xD7V[` \x02\x01Q``\x01\x81\x81Qa\x11\xD2\x91\x90a\x7F%V[\x90RPa\x12;V[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xF2Wa\x11\xF2a~\xD7V[` \x02\x01Q` \x01\x81\x81Qa\x12\x07\x91\x90a\x7F8V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12\"Wa\x12\"a~\xD7V[` \x02\x01Q``\x01\x81\x81Qa\x127\x91\x90a\x7F8V[\x90RP[\x81\x15a\x12\xBEW\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x12YWa\x12Ya~\xD7V[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x12}Wa\x12}a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x12\x92\x91\x90a\x7F8V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x12\xADWa\x12\xADa~\xD7V[` \x02\x01Q`@\x01RPa\x13\xDE\x90PV[\x82_\x03a\x12\xCBWPa\x13\xDEV[_a\x12\xD5\x84aW\x96V[\x90P_a\x13\x0B\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x12\xF3Wa\x12\xF3a~\xD7V[` \x02\x01Q` \x01Q\x83aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x13yW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13,Wa\x13,a~\xD7V[` \x02\x01Q`@\x01\x81\x81Qa\x13A\x91\x90a\x7F%V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\\Wa\x13\\a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13q\x91\x90a\x7F%V[\x90RPa\x13\xDAV[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\x91Wa\x13\x91a~\xD7V[` \x02\x01Q`@\x01\x81\x81Qa\x13\xA6\x91\x90a\x7F8V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\xC1Wa\x13\xC1a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\xD6\x91\x90a\x7F8V[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x14\x04\x84\x84aW\xE6V[\x91P\x91P\x81\x81\x10a\x14%W``\x83\x01Qa\x14\x1F\x90`\x01aXGV[``\x84\x01R[PPPPV[____\x86\x11\x80\x15a\x14;WP\x83\x15[\x15a\x14JWP\x83\x90P\x84a\x14bV[_\x87\x11\x80\x15a\x14WWP\x84\x15[\x15a\x10ZWP\x85\x90P\x82[_a\x14q\x89``\x01Q_aXqV[\x90P_a\x14\x83\x8A``\x01Q`\x01aXqV[\x90P_a\x14\xA6\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x14\xA1\x86`\na\x80&V[aX\x9FV[\x90P_a\x14\xC4\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x14\xA1\x86`\na\x80&V[\x90P\x80_\x03a\x14\xDBW_\x96PPPPPPPa\x14\xEEV[a\x14\xE5\x82\x82aW\xABV[\x96PPPPPPP[\x95\x94PPPPPV[a\x15'`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nupdateTwapPrice`\x88\x1B\x81RPa\r\xBEV[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x15\x81\x84\x84aY_V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x15\x98d\x01\0\0\0\0Ba\x80EV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x16NWa\x15\xBD\x84\x84\x83` \x01QaY\xD5V[a\x15\xC8\x84\x84_aZYV[a\x15\xD7\x84\x84\x83` \x01QaZ\x98V[a\x15\xE2\x84\x84_aZ\xB0V[a\x15\xED\x84\x84\x84aZ\xC8V[a\x16'`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x14%`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83aZ\xE0V[\x80Q` \x82\x01Qa\x16_\x91\x90a\x80XV[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x16\x9B\x92\x16aZ\xE0V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x17\xCBWa\x16\xB5\x84\x84a[\tV[``\x82\x01R`@\x81\x01Qa\x16\xCF\x90c\xFF\xFF\xFF\xFF\x16\x83a\x80tV[\x81``\x01Qa\x16\xDE\x91\x90a\x7F%V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x17\t\x90\x83aZ\xE0V[a\x17@`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x17y`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01QaZ\xE0V[a\x17\xAE`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaZ\xE0V[a\x17\xBD\x84\x84\x83`\x80\x01QaZYV[a\x17\xCB\x84\x84\x83_\x01QaY\xD5V[a\x17\xD5\x84\x84a[\"V[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x17\xF1\x91\x90a\x80XV[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x18\x05\x84a[;V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x18=\x92\x16aZ\xE0V[a\x18\x86`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x18\xC5`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14%Wa\x18\xEB\x84\x84a[\tV[`\x80\x82\x01Ra\x18\xFA\x84\x84a[\xEDV[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x19\x1E\x91a\x7F8V[a\x19(\x91\x90a\x80\x8BV[a\x01 \x82\x01R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16`\xA0\x83\x01R`\x80\x82\x01Qa\x01\0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0\x92\x81\x01\x92\x90\x92RQa\x19\x8D\x91\x90aZ\xE0V[a\x19\xC0`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01QaZ\xE0V[a\x19\xCF\x84\x84\x83`\xA0\x01QaZ\x98V[a\x19\xDF\x84\x84\x83a\x01\0\x01QaZ\xB0V[a\x14%\x84\x84\x83a\x01 \x01QaZ\xC8V[_a\x19\xFA\x85\x84a~\xFFV[\x90P_\x81\x13\x15a\x1A$W\x86Q` \x01Qa\x1A\x1F\x90\x83a\x1A\x18\x84aW\x96V[`\x01a\\\x06V[a\x1A?V[\x86Q` \x01Qa\x1A?\x90\x83a\x1A8\x84aW\x96V[`\x01a]$V[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1A\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xDDW__\xFD[PZ\xF1\x15\x80\x15a\x1A\xEFW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x1B\x0F\x84`@\x01Qa^7V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BMW__\xFD[PZ\xF1\x15\x80\x15a\x1B_W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\x9D\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CN\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1C\x8E\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x1D\t\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DI\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1D\x89\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E@\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x85\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xB5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F{\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a +\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a v\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!'\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!q\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\"\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"n\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1F\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#j\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x9A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1B\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$Z\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\x8A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x0C\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a%L\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x06\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&K\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&{\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFE\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'C\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a's\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(A\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xF5\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)?\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xF3\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*?\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xF3\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+>\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF2\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,a\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11,\x91\x90a~HV[a,\xF5\x81a^\xBBV[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a-P\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xA0W__\xFD[PZ\xF1\x15\x80\x15a-\xB2W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a-\xF6\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xAD\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.\xF5\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a/\xED\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x9D\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0\xE9\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1vW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\x9A\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1\xBA\x90a\x80\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2k\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2\xB8\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3i\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a3\xB2\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4c\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a4\xA4\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5^\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a5\xA6\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xD6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a65W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6Y\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a6\xA0\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xD0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7S\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7\x9F\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8S\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a8s\x90a\x80\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9'\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9t\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a:\x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:(\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a:q\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;%\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a;c\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\x93\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\xDD\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x1D\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a<o\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra<\xEA\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=*\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a=q\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\"\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a>t\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a?\"`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a?/WP\x85\x15[\x15a?fW\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90Ra?\xA8V[_\x89\x11\x80\x15a?sWP\x86\x15[\x15a?\xA8W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a@cW__\xFD[PZ\xF1\x15\x80\x15a@uW=__>=_\xFD[PPPPPPPPPPPPPV[`@Qc\x04\xE6\xBD\xD1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x88\x81\x16`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R`\xE4\x82\x01\x84\x90Ra\x01\x04\x82\x01\x83\x90R\x8B\x16\x90c\t\xCD{\xA2\x90a\x01$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aA\0W__\xFD[PZ\xF1\x15\x80\x15aA\x12W=__>=_\xFD[PPPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01aA8\x92\x91\x90a\x81;V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra`\xCAV[aAoa{\xDDV[__aA{\x86\x85a`\xD3V[\x90P_aA\x88\x86\x83aa9V[\x90Pa\x0E9\x87\x82aaKV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aA\xB5W\x81\x83aA\xB8V[\x82\x82[`@Q\x91\x94P\x92PaA\xE5\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[aBDa|\x03V[\x82aBMa|\x03V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aB\x89\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x01\x91\x90a~_V[aC\x0EW\x91Pa\x11\\\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aCN\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC\xB2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\xF1\x91\x90a\x80\x9EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aDo\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xA3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xE2\x91\x90a~HV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE8\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aEh\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x9C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xDB\x91\x90a~HV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF6\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aFf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xD9\x91\x90a~HV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aG\x02\x90` \x01a\x80\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aGf\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xA5\x91\x90a~HV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\x01\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aHe\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\xA4\x91\x90a~HV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aIU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aIpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x94\x91\x90a~HV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJXW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ|\x91\x90a\x80\x9EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKe\x91\x90a~HV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK\xBC\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL_\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aL\xBB\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM^\x91\x90a~HV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM\x8E\x90a\x80\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN1\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN\x8E\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO1\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aO\x8A\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP-\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aP{\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x1E\x91\x90a\x80\x9EV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aQ\x8C\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR/\x91\x90a\x80\x9EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\x92\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS5\x91\x90a~HV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aS\x8E\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT1\x91\x90a~HV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aT~WPPV[\x81QQ`\xA0\x01Q\x15aT\xD4W\x81Q_\x90aT\xA7\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01Qaa\xC5V[\x83Q\x90\x91PaT\xCB\x90\x82\x90_[` \x02\x01Q` \x01Qaa\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aU\x10W\x81Q_\x90aT\xF2\x90`\x01aT\x94V[\x83Q\x90\x91PaU\x04\x90\x82\x90`\x01aT\xB4V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aU$a|7V[aU-\x89ab:V[a\x01\xC0\x82\x01\x81\x90RaUD\x90\x88\x90_\x90\x81\x90ab\x8BV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaUc\x90\x88\x90`\x01\x90_\x90ab\x8BV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aU\x7FWP` \x81\x01Q\x15[\x15aU\x96W____\x94P\x94P\x94P\x94PPaV9V[\x85\x15aU\xB2W\x87\x81` \x01\x81\x81QaU\xAE\x91\x90a\x7F8V[\x90RP[\x80Q` \x82\x01QaU\xC8\x91\x90a\x14\xA1\x81\x8CacpV[`\x80\x82\x01\x81\x90R\x81QaU\xDA\x91ac\xC4V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01QaV\x11\x91aV\t\x90a'\x10\x90ac\xC4V[a'\x10aX\x9FV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01QaV/\x91aWUV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____aVPa|7V[aVY\x89ab:V[a\x01\xC0\x82\x01\x81\x90RaVp\x90\x88\x90_\x90\x81\x90ab\x8BV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaV\x8F\x90\x88\x90`\x01\x90_\x90ab\x8BV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aV\xABWP` \x81\x01Q\x15[\x15aV\xC2W____\x94P\x94P\x94P\x94PPaV9V[\x85\x15aV\xDDW\x87\x81_\x01\x81\x81QaV\xD9\x91\x90a\x7F8V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90RaW\x04\x90\x89\x90aV\t\x90a'\x10\x90ac\xC4V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01QaW\"\x92a\x14\xA1\x90\x83\x90acpV[`\x80\x82\x01\x81\x90R` \x82\x01QaW7\x91ac\xC4V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01QaV/\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aWkW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aW\x8F\x90\x83\x90a\x7F%V[\x90RPPPV[__\x82\x12\x15aW\xA7W\x81_\x03a\x11\\V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aW\xCCW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aW\xF7\x84``\x01Q_aXqV[\x90P_aX\x03\x86ad\x19V[\x90P_aX&\x82aX\x15\x85`\na\x80&V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaX\x9FV[\x90P_aX5\x87___ab\x8BV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aXUW_aXXV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aX\x91WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aX\xD3W\x83\x82\x81aX\xC9WaX\xC9a\x801V[\x04\x92PPPaYXV[\x80\x84\x11aX\xF3W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84adjV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aY\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aYX\x91\x90a~HV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aY\xED\x84adjV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15aZ5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14%\x91\x90a~HV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84ae\x1DV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZ\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aY\xED\x84aeqV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84ae\xD2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84af8V[a\x027`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83af\x81V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84ae\x1DV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84aeqV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\\\x91\x90a~HV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84ae\xD2V[`\xE0\x84\x01Q`\x01\x19\x01a\\-W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x14%V[`\xE0\x84\x01Q_\x19\x01a\\\x9FW`\xA0\x84\x01\x80Q\x90\x83\x90a\\L\x82\x84a\x7F%V[\x90RP\x81\x15a\\\x99W_a\\`\x85\x85aa\xF9V[``\x87\x01Qa\\o\x90\x84aa\xF9V[a\\y\x91\x90a\x7F%V[\x90Pa\\\x92\x86`\xA0\x01Q\x82aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x14%V[`\xE0\x84\x01Qa\x14%W\x81\x84`\xC0\x01Q\x11\x15a\\\xCEW\x81\x84`\xC0\x01\x81\x81Qa\\\xC6\x91\x90a\x7F8V[\x90RPa\x14%V[\x81\x84`\xC0\x01Q\x03a\\\xF2W`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x14%V[`\x01`\xE0\x85\x01R`\xC0\x84\x01Qa]\x08\x90\x83a\x7F8V[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01a]JW_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x14%V[`\xE0\x84\x01Qa]\xB8W`\xC0\x84\x01\x80Q\x90\x83\x90a]f\x82\x84a\x7F%V[\x90RP\x81\x15a\\\x99W_a]z\x85\x85aa\xF9V[`\x80\x87\x01Qa]\x89\x90\x84aa\xF9V[a]\x93\x91\x90a\x7F%V[\x90Pa]\xAC\x86`\xC0\x01Q\x82aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x14%V[`\xE0\x84\x01Q_\x19\x01a\x14%W\x81\x84`\xA0\x01Q\x11\x15a]\xE2W\x81\x84`\xA0\x01\x81\x81Qa\\\xC6\x91\x90a\x7F8V[\x81\x84`\xA0\x01Q\x03a^\x06W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x14%V[_`\xE0\x85\x01R`\xA0\x84\x01Qa^\x1B\x90\x83a\x7F8V[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01a^q\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__a^\xF0`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a^\xFA\x84_af\xC8V[` \x83\x01R\x81R``\x84\x01Qa_\x10\x90_aXqV[``\x82\x01\x81\x90R\x81Qa_5\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x14\xA1\x90`\na\x80&V[`@\x82\x01R` \x81\x01Q_\x03a_PW_`\x80\x82\x01Ra_\xF0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_\xEA\x91\x90a~HV[`\x80\x82\x01R[a_\xFB\x84`\x01af\xC8V[` \x83\x01\x81\x90R\x90\x82R_\x03a`\x16W_`\xA0\x82\x01Ra`\xB6V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`\xB0\x91\x90a~HV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[a\t\x94\x81ag\x0EV[_`@Q` \x01aa\0\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01aB\x1EV[aaAa{\xDDV[aYX\x83\x83ag-V[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aavW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80aa\xD1\x83Ba\x7F8V[aa\xDB\x90\x85a\x80tV[c\x01\xE13\x80\x90\x04\x90Pa\x11X\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x7F%V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17ab\x19W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10ab\xA8Wab\xA8a~\xD7V[` \x02\x01Q\x90P_ab\xBA\x8A\x8Aay?V[\x90P\x80_\x03ab\xD6W____\x95P\x95P\x95P\x95PPPaV9V[_ab\xE5\x83\x8C`\x80\x01Qaz-V[\x90P_ab\xF2\x82\x8Aaa\xF9V[\x90P_\x89\x15ac\x17W\x81\x84\x10ac\x11Wac\x0C\x84\x83ac\xC4V[ac\x19V[_ac\x19V[_[\x90P_ac&\x85\x8Daa\xF9V[\x90P_\x8C\x15acKW\x84\x82\x10acEWac@\x82\x86ac\xC4V[acMV[_acMV[_[\x90PacY\x85\x87a\x7F%V[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x82ac|\x83\x82a\x7F%V[\x91P\x81\x10\x15a\x11\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x82ac\xD0\x83\x82a\x7F8V[\x91P\x81\x11\x15a\x11\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90ad\x8B\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQaA\x94V[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ad\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90ae-\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90ae\x81\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ae\xE2\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90afH\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\x08\xD9\x83\x83\x83`@Q`$\x01af\x99\x93\x92\x91\x90a\x81_V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra`\xCAV[___af\xF5\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10af\xE6Waf\xE6a~\xD7V[` \x02\x01Q\x86`\x80\x01Qaz-V[\x90P_ag\x02\x86\x86ay?V[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[ag5a{\xDDV[\x82ag>a{\xDDV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01ag~\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ag\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ag\xF6\x91\x90a~_V[ah\x03W\x91Pa\x11\\\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ah=\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ahm\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ah\xA1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ah\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ah\xE0\x91\x90a~HV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01ai(\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aiX\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ai\x8C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ai\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ai\xCB\x91\x90a\x80\x9EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aj'\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ajW\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aj\xCA\x91\x90a\x80\x9EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01akE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aky\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ak\xB8\x91\x90a~HV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01al\x0C\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01alp\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\xAF\x91\x90a~HV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01am\t\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01am9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01amm\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am\xAC\x91\x90a~HV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01an\x05\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01an5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ani\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15an\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90an\xA8\x91\x90a~HV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ao(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ao\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aowW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ao\x9B\x91\x90a~HV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ao\xF5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ap%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01apY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aptW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ap\x98\x91\x90a~HV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aq?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aqZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aq~\x91\x90a~HV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ar&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15arAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90are\x91\x90a\x80\x9EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01as\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15as'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90asK\x91\x90a~HV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01as\xA0\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01as\xD0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01at\x04\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15at\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90atC\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01at\x9E\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01at\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15au\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90auA\x91\x90a~HV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01au\x9B\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01au\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15av\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90av>\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01av\x9A\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01av\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01av\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aw\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aw=\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aw\x98\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aw\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aw\xFC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ax\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ax;\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ax\x8A\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ax\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ax\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay-\x91\x90a~HV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10ayYWayYa~\xD7V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay\xD6\x91\x90a~HV[\x90P\x80_\x03ay\xE9W_\x92PPPa\x11\\V[``\x82\x01Q`\xC0\x83\x01Qay\xFD\x90\x82a\x7F%V[\x82\x10az!W`\xC0\x83\x01Qaz\x12\x82\x84a\x7F8V[az\x1C\x91\x90a\x7F8V[az#V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03az@WP_a\x11\\V[_azK\x84\x84az]V[`\xA0\x85\x01Q\x90\x91Pa\x11X\x90\x82aa\xF9V[_B\x82\x03azpWP` \x82\x01Qa\x11\\V[_az\x7F\x84`@\x01Q\x84aa\xC5V[\x90Paz\x98\x84` \x01Q\x82aa\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x11\\V[`@Q\x80a\x02\x80\x01`@R\x80az\xB4a|\x03V[\x81R` \x01_\x81R` \x01az\xC7a{\xDDV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x02\0\x01`@R\x80a{ba|\x03V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a{\xF0a|\x9EV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80a|\x16a}\x0CV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a|\xF6`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a|\xADW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a}]`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a}\x1BW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x94W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15a}\x99W__\xFD[\x835a}\xA4\x81a}sV[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15a}\xB7W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15a}\xD8W__\xFD[\x835a}\xE3\x81a}sV[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15a}\xB7W__\xFD[_` \x82\x84\x03\x12\x15a~\x06W__\xFD[\x815aYX\x81a}sV[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a~XW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a~oW__\xFD[\x81Q\x80\x15\x15\x81\x14aYXW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a~\xCF\x90\x83\x01\x84a~~V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x7F\x1EWa\x7F\x1Ea~\xEBV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x11\\Wa\x11\\a~\xEBV[\x81\x81\x03\x81\x81\x11\x15a\x11\\Wa\x11\\a~\xEBV[`\x01\x81[`\x01\x84\x11\x15a\r\xFBW\x80\x85\x04\x81\x11\x15a\x7FjWa\x7Fja~\xEBV[`\x01\x84\x16\x15a\x7FxW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x7FOV[_\x82a\x7F\x94WP`\x01a\x11\\V[\x81a\x7F\xA0WP_a\x11\\V[\x81`\x01\x81\x14a\x7F\xB6W`\x02\x81\x14a\x7F\xC0Wa\x7F\xDCV[`\x01\x91PPa\x11\\V[`\xFF\x84\x11\x15a\x7F\xD1Wa\x7F\xD1a~\xEBV[PP`\x01\x82\x1Ba\x11\\V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x7F\xFFWP\x81\x81\na\x11\\V[a\x80\x0B_\x19\x84\x84a\x7FKV[\x80_\x19\x04\x82\x11\x15a\x80\x1EWa\x80\x1Ea~\xEBV[\x02\x93\x92PPPV[_aYX\x83\x83a\x7F\x86V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x80SWa\x80Sa\x801V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11\\Wa\x11\\a~\xEBV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11\\Wa\x11\\a~\xEBV[_\x82a\x80\x99Wa\x80\x99a\x801V[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x80\xAEW__\xFD[\x81QaYX\x81a}sV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a\x81M`@\x83\x01\x85a~~V[\x82\x81\x03` \x84\x01Ra\x14\xEE\x81\x85a~~V[``\x81R_a\x81q``\x83\x01\x86a~~V[\x82\x81\x03` \x84\x01Ra\x81\x83\x81\x86a~~V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x1D\x1F\xE5\xE0z\x19!\xB5\x9FwJ\xD1\xDB\t||hg\xB1\xE4]\xF2\x90\xA42\x1A\xD8_\x0Fk\x13\xF4dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004617d87565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617dc5565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b506102376108de565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f19190617df6565b6001600160a01b031681526020018360200160208101906103129190617df6565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b9190617df6565b6001600160a01b03169052905061022e8382610997565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b190617e11565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104249190617e48565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b790617e11565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102379190617e48565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd9190617e5f565b61023757338160405163a35b150b60e01b8152600401610470929190617eac565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610dbe565b61061c617aa0565b61062e83835f01518460400151610de2565b606083015260408201819052825190518051516020919091015151610654929190610e03565b6020830152808252606083015160a0808401829052608085015160c08086018290528651928701519087015161068f9593949291905f610e44565b6102408601526101a0850152610180840152610100830181905260e08301829052604083015160a084015160c08501516106ce949293919290916110a6565b6106e0815f0151826102400151611133565b6102608201528051604082015160a083015160e08401516107109392915f916107099190617eff565b5f5f611162565b610733815f0151826040015160018460c001518561010001516107099190617eff565b60a08101511561074b578151815161074b91906113e6565b61076c815f01518260a001518360c001518460e0015185610100015161142b565b610160820181905282518251610781926114f7565b6107ad815f015182604001518360a001518460c001518560e001518661010001518761016001516119ef565b6107c3825f015182606001518360400151611a49565b6107d48260200151825f0151612cec565b8151602082015182516107e8929190612d10565b610809815f01518260a001518360c001518460e00151856101000151613ee5565b610220850181905261020085018290526001600160a01b039283166101e08601819052939092166101c085018190526020868101516040888101516102608901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108999792968b9693949193929091613fcd565b60208281015160408381015151805180519185015180518489015183880151938601519783015192909501516108d9978b96600496959394909392614084565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161091c90617e11565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109949190617e48565b50565b6109c36040518060400160405280600b81526020016a065786563757465537761760ac1b815250610dbe565b6109cb617b4e565b6109e1825f015183604001518460600151610e03565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af1158015610a3f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a639190617e48565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610aba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ade9190617e48565b60808201526060810151158015610af757506080810151155b15610b1557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511015610b3057608082015160608201525b8160a0015181608001511015610b4b5760a082015160808201525b610b71825f0151825f0151836060015184608001518660c001518760e001516001610e44565b6101c0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610ba591611133565b6101e082015260a081015115610c2f576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c18575f5ffd5b505af1158015610c2a573d5f5f3e3d5ffd5b505050505b60c081015115610cb457604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c9d575f5ffd5b505af1158015610caf573d5f5f3e3d5ffd5b505050505b610cc58260200151825f0151612cec565b606081015115610cdd5781518151610cdd91906113e6565b815160208201518251610cf1929190612d10565b610d11815f0151826060015183608001518460a001518560c00151613ee5565b6101a08501526101808401526001600160a01b039081166101608401521661014082015280516060820151608083015160a084015160c0850151610d58949392919061142b565b610120820181905282518251610d6d926114f7565b6108d9826020015184836101400151846101600151627a1200866101800151876101a00151886101e0015160405180608001604052805f81526020015f81526020015f81526020015f815250613fcd565b61099460405180604001604052806002815260200161257360f01b81525082614122565b610dea617bdd565b5f610df6858585614167565b915091505b935093915050565b610e0b617c03565b5f5f610e178585614194565b90505f610e24878361423c565b9050610e30818361543e565b610e398161546c565b969095509350505050565b5f5f5f5f5f610ea16040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8a118015610eae575087155b15610f8957610ebf8d8b8e8a615518565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e08201819052891115610f2e5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610f459190617f25565b1115610f84576101008101518151610f5d9190617f25565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b611073565b5f8b118015610f96575088155b1561105a57610fa78d8c8e8a615644565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e0820181905288111561101e57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610f845760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805f0151816020015182604001518360600151846101000151955095509550955095505097509750975097509792505050565b5f831180156110b3575080155b1561110257845160209081015101518311156110fd578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b61112c565b5f8411801561110f575081155b1561112c578451516020015184111561112c57845184905f6110d0565b5050505050565b60608201515f9060481c61ffff168161114c8483615755565b90506111588582615779565b9150505b92915050565b5f61116c84615796565b90505f84126111da578551819060ff87166002811061118d5761118d617ed7565b60200201516020018181516111a29190617f25565b9052508651819060ff8716600281106111bd576111bd617ed7565b60200201516060018181516111d29190617f25565b90525061123b565b8551819060ff8716600281106111f2576111f2617ed7565b60200201516020018181516112079190617f38565b9052508651819060ff87166002811061122257611222617ed7565b60200201516060018181516112379190617f38565b9052505b81156112be5785515f9060ff87166002811061125957611259617ed7565b602002015160400151905080885f01518760ff166002811061127d5761127d617ed7565b602002015160a0018181516112929190617f38565b90525086515f9060ff8816600281106112ad576112ad617ed7565b602002015160400152506113de9050565b825f036112cb57506113de565b5f6112d584615796565b90505f61130b895f01518860ff16600281106112f3576112f3617ed7565b602002015160200151836157ab90919063ffffffff16565b90505f8512611379578751819060ff89166002811061132c5761132c617ed7565b60200201516040018181516113419190617f25565b9052508851819060ff89166002811061135c5761135c617ed7565b602002015160a0018181516113719190617f25565b9052506113da565b8751819060ff89166002811061139157611391617ed7565b60200201516040018181516113a69190617f38565b9052508851819060ff8916600281106113c1576113c1617ed7565b602002015160a0018181516113d69190617f38565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f61140484846157e6565b9150915081811061142557606083015161141f906001615847565b60608401525b50505050565b5f5f5f5f8611801561143b575083155b1561144a575083905084611462565b5f87118015611457575084155b1561105a5750859050825b5f61147189606001515f615871565b90505f6114838a606001516001615871565b90505f6114a685676765c793fa10079d601b1b6114a186600a618026565b61589f565b90505f6114c485676765c793fa10079d601b1b6114a186600a618026565b9050805f036114db575f96505050505050506114ee565b6114e582826157ab565b96505050505050505b95945050505050565b6115276040518060400160405280600f81526020016e75706461746554776170507269636560881b815250610dbe565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101829052610120810191909152611581848461595f565b63ffffffff16815261159864010000000042618045565b63ffffffff90811660208301528151165f0361164e576115bd848483602001516159d5565b6115c884845f615a59565b6115d784848360200151615a98565b6115e284845f615ab0565b6115ed848484615ac8565b6116276040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff16615ae0565b61142560405180604001604052806005815260200164707269636560d81b81525083615ae0565b8051602082015161165f9190618058565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b6020820152905161169b9216615ae0565b604081015163ffffffff16156117cb576116b58484615b09565b606082015260408101516116cf9063ffffffff1683618074565b81606001516116de9190617f25565b6080820152604080518082019091526005815264707269636560d81b60208201526117099083615ae0565b6117406040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff16615ae0565b611779604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b8152508260600151615ae0565b6117ae6040518060400160405280600f81526020016e707269636543756d756c617469766560881b8152508260800151615ae0565b6117bd84848360800151615a59565b6117cb8484835f01516159d5565b6117d58484615b22565b63ffffffff1660a0820181905260208201516117f19190618058565b63ffffffff1660c082015261180584615b3b565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b6020820152905161183d9216615ae0565b6118866040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff16615ae0565b6118c5604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff16615ae0565b8060e0015163ffffffff168160c0015163ffffffff161115611425576118eb8484615b09565b60808201526118fa8484615bed565b610100820181905260c0820151608083015163ffffffff9091169161191e91617f38565b611928919061808b565b61012082015260208082015163ffffffff1660a08301526080820151610100830190815260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f640000000000928101929092525161198d9190615ae0565b6119c06040518060400160405280600c81526020016b70726963654176657261676560a01b815250826101200151615ae0565b6119cf84848360a00151615a98565b6119df8484836101000151615ab0565b6114258484836101200151615ac8565b5f6119fa8584617eff565b90505f811315611a2457865160200151611a1f9083611a1884615796565b6001615c06565b611a3f565b865160200151611a3f9083611a3884615796565b6001615d24565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611a8d906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611add575f5ffd5b505af1158015611aef573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62611b0f8460400151615e37565b856040518363ffffffff1660e01b8152600401611b36929190918252602082015260400190565b5f604051808303815f87803b158015611b4d575f5ffd5b505af1158015611b5f573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611b9d906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611bcd929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611c0e929190918252602082015260400190565b6020604051808303815f875af1158015611c2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c4e9190617e48565b50806001600160a01b031663ca446dd984604051602001611c8e906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cbe929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611d09926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015611d25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d49919061809e565b50806001600160a01b031663ca446dd984604051602001611d89906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611db9929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611e1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e40919061809e565b50806001600160a01b031663e2a4853a84604051602001611e859060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611eb5929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611f12573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f369190617e48565b50806001600160a01b031663e2a4853a84604051602001611f7b9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fab929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612007573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061202b9190617e48565b50806001600160a01b031663e2a4853a84604051602001612076906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016120a6929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612103573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121279190617e48565b50806001600160a01b031663e2a4853a84604051602001612171906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016121a1929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156121fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122229190617e48565b50806001600160a01b031663e2a4853a8460405160200161226e906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161229e929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156122fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061231f9190617e48565b50806001600160a01b031663e2a4853a8460405160200161236a906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161239a929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156123f7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061241b9190617e48565b50806001600160a01b031663e2a4853a8460405160200161245a906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161248a929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156124e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061250c9190617e48565b50806001600160a01b031663ca446dd98460405160200161254c906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161257c929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156125e2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612606919061809e565b50806001600160a01b031663e2a4853a8460405160200161264b9060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161267b929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156126da573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126fe9190617e48565b50806001600160a01b031663e2a4853a846040516020016127439060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001612773929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156127d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f69190617e48565b50806001600160a01b031663e2a4853a8460405160200161284190602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612871929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128d1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128f59190617e48565b50806001600160a01b031663e2a4853a8460405160200161293f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161296f929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156129cf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129f39190617e48565b50806001600160a01b031663e2a4853a84604051602001612a3f90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a6f929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612acf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612af39190617e48565b50806001600160a01b031663e2a4853a84604051602001612b3e90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b6e929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612bce573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf29190617e48565b50806001600160a01b031663e2a4853a84604051602001612c31906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c61929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612cac929190918252602082015260400190565b6020604051808303815f875af1158015612cc8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061112c9190617e48565b612cf581615ebb565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001612d50906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612da0575f5ffd5b505af1158015612db2573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001612df6906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e26929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612e89573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ead919061809e565b50806001600160a01b031663e2a4853a84604051602001612ef5906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f25929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612f82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa69190617e48565b50806001600160a01b031663e2a4853a84604051602001612fed906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b6040516020818303038152906040528051906020012060405160200161301d929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015613079573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061309d9190617e48565b50806001600160a01b031663e2a4853a846040516020016130e9906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613119929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613176573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061319a9190617e48565b50806001600160a01b031663e2a4853a846040516020016131ba906180b9565b604051602081830303815290604052805190602001206040516020016131ea929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613247573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061326b9190617e48565b50806001600160a01b031663e2a4853a846040516020016132b8906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016132e8929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613345573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133699190617e48565b50806001600160a01b031663e2a4853a846040516020016133b2906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016133e2929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561343f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134639190617e48565b50806001600160a01b031663ca446dd9846040516020016134a4906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016134d4929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561353a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061355e919061809e565b50806001600160a01b031663e2a4853a846040516020016135a690602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016135d6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015613635573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136599190617e48565b50806001600160a01b031663e2a4853a846040516020016136a090602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016136d0929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561372f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137539190617e48565b50806001600160a01b031663e2a4853a8460405160200161379f90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016137cf929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561382f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138539190617e48565b50806001600160a01b031663e2a4853a84604051602001613873906180fa565b604051602081830303815290604052805190602001206040516020016138a3929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613903573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139279190617e48565b50806001600160a01b031663e2a4853a8460405160200161397490602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016139a4929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613a04573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a289190617e48565b50806001600160a01b031663e2a4853a84604051602001613a7190602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613aa1929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613b01573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b259190617e48565b50806001600160a01b031663ca446dd984604051602001613b6390602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b93929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401613bdd9291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613bf9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c1d919061809e565b50806001600160a01b031663ca446dd984604051602001613c6f906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613c9f929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613cea926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613d06573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d2a919061809e565b50806001600160a01b031663e2a4853a84604051602001613d71906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613da1929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613de2929190918252602082015260400190565b6020604051808303815f875af1158015613dfe573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e229190617e48565b50806001600160a01b031663e2a4853a84604051602001613e74906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613ea4929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612cac929190918252602082015260400190565b5f5f5f5f613f2260405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f88118015613f2f575085155b15613f66578951602090810151516001600160a01b0390811683528b51515116908201526040810188905260608101879052613fa8565b5f89118015613f73575086155b15613fa857895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b158015614063575f5ffd5b505af1158015614075573d5f5f3e3d5ffd5b50505050505050505050505050565b6040516304e6bdd160e11b81526001600160a01b038a81166004830152602482018a9052888116604483015287811660648301526084820187905260a4820186905260c4820185905260e4820184905261010482018390528b16906309cd7ba290610124015f604051808303815f87803b158015614100575f5ffd5b505af1158015614112573d5f5f3e3d5ffd5b5050505050505050505050505050565b610237828260405160240161413892919061813b565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b1790526160ca565b61416f617bdd565b5f5f61417b86856160d3565b90505f6141888683616139565b9050610e39878261614b565b5f816001600160a01b0316836001600160a01b0316106141b55781836141b8565b82825b60405191945092506141e5906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b614244617c03565b8261424d617c03565b816001600160a01b03166391d4403c604051602001614289906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156142dd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143019190617e5f565b61430e57915061115c9050565b816001600160a01b03166321f8a7218560405160200161434e906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161437e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016143b291815260200190565b602060405180830381865afa1580156143cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143f1919061809e565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161446f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144a391815260200190565b602060405180830381865afa1580156144be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144e29190617e48565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614538906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001614568929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161459c91815260200190565b602060405180830381865afa1580156145b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145db9190617e48565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614636906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614666929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161469a91815260200190565b602060405180830381865afa1580156146b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146d99190617e48565b815151606001526040516001600160a01b0383169063bd02d0f5908690614702906020016180b9565b60405160208183030381529060405280519060200120604051602001614732929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161476691815260200190565b602060405180830381865afa158015614781573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147a59190617e48565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614801906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001614831929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161486591815260200190565b602060405180830381865afa158015614880573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148a49190617e48565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614921929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161495591815260200190565b602060405180830381865afa158015614970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149949190617e48565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614a09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a3d91815260200190565b602060405180830381865afa158015614a58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a7c919061809e565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b2691815260200190565b602060405180830381865afa158015614b41573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b659190617e48565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614bbc90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614c2091815260200190565b602060405180830381865afa158015614c3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c5f9190617e48565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614cbb90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ceb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d1f91815260200190565b602060405180830381865afa158015614d3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d5e9190617e48565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614d8e906180fa565b60405160208183030381529060405280519060200120604051602001614dbe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614df291815260200190565b602060405180830381865afa158015614e0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e319190617e48565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001614e8e90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ebe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ef291815260200190565b602060405180830381865afa158015614f0d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f319190617e48565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001614f8a90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001614fba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fee91815260200190565b602060405180830381865afa158015615009573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061502d9190617e48565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161507b90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016150ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150df91815260200190565b602060405180830381865afa1580156150fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061511e919061809e565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a7218560405160200161518c906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016151bc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151f091815260200190565b602060405180830381865afa15801561520b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061522f919061809e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001615292906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016152c2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152f691815260200190565b602060405180830381865afa158015615311573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153359190617e48565b60608201526040516001600160a01b0383169063bd02d0f590869061538e906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016153be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153f291815260200190565b602060405180830381865afa15801561540d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154319190617e48565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b6080810151429081900361547e575050565b81515160a00151156154d45781515f906154a790825b60200201516040015184608001516161c5565b83519091506154cb9082905f5b6020020151602001516161f990919063ffffffff16565b83515160200152505b81516020015160a00151156155105781515f906154f2906001615494565b835190915061550490829060016154b4565b83516020908101510152505b608090910152565b5f5f5f5f615524617c37565b61552d8961623a565b6101c082018190526155449088905f90819061628b565b5060408401525081526101c08101516155639088906001905f9061628b565b5060608401525060208201528051158061557f57506020810151155b15615596575f5f5f5f945094509450945050615639565b85156155b25787816020018181516155ae9190617f38565b9052505b805160208201516155c891906114a1818c616370565b6080820181905281516155da916163c4565b60a0820152606087015160381c61ffff16610140820181905260a08201516156119161560990612710906163c4565b61271061589f565b6040820151606083015161014084015160a085015161562f91615755565b9450945094509450505b945094509450949050565b5f5f5f5f615650617c37565b6156598961623a565b6101c082018190526156709088905f90819061628b565b5060408401525081526101c081015161568f9088906001905f9061628b565b506060840152506020820152805115806156ab57506020810151155b156156c2575f5f5f5f945094509450945050615639565b85156156dd5787815f018181516156d99190617f38565b9052505b606087015160381c61ffff16610140820181905261570490899061560990612710906163c4565b610160820181905281516020830151615722926114a1908390616370565b608082018190526020820151615737916163c4565b60c082018190526040820151606083015161014084015161562f908c905b5f8115611388198390048411151761576b575f5ffd5b506127109102611388010490565b81515160c001805182919061578f908390617f25565b9052505050565b5f5f8212156157a757815f0361115c565b5090565b5f8115676765c793fa10079d601b1b600284041904841117156157cc575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f6157f784606001515f615871565b90505f61580386616419565b90505f6158268261581585600a618026565b676765c793fa10079d601b1b61589f565b90505f615835875f5f5f61628b565b50939a91995090975050505050505050565b5f603382615855575f615858565b60015b60ff16901b660800000000000019841617905092915050565b5f60ff60581b1960585f1960ff851601615891575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f036158d3578382816158c9576158c9618031565b0492505050615958565b8084116158f35760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f826001600160a01b031663bd02d0f56159788461646a565b6040518263ffffffff1660e01b815260040161599691815260200190565b602060405180830381865afa1580156159b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159589190617e48565b826001600160a01b031663e2a4853a6159ed8461646a565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015615a35573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114259190617e48565b826001600160a01b031663e2a4853a615a718461651d565b836040518363ffffffff1660e01b8152600401615a19929190918252602082015260400190565b826001600160a01b031663e2a4853a6159ed84616571565b826001600160a01b031663e2a4853a615a71846165d2565b826001600160a01b031663e2a4853a615a7184616638565b610237604051806040016040528060068152602001652573202d257360d01b8152508383616681565b5f826001600160a01b031663bd02d0f56159788461651d565b5f826001600160a01b031663bd02d0f561597884616571565b5f816001600160a01b031663bd02d0f5604051602001615b7a906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615bae91815260200190565b602060405180830381865afa158015615bc9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061115c9190617e48565b5f826001600160a01b031663bd02d0f5615978846165d2565b60e084015160011901615c2d57600160e085015260a0840182905260608401839052611425565b60e08401515f1901615c9f5760a084018051908390615c4c8284617f25565b9052508115615c99575f615c6085856161f9565b6060870151615c6f90846161f9565b615c799190617f25565b9050615c928660a00151826157ab90919063ffffffff16565b6060870152505b50611425565b60e084015161142557818460c001511115615cce57818460c001818151615cc69190617f38565b905250611425565b818460c0015103615cf257600260e08501525f60c085018190526080850152611425565b600160e085015260c0840151615d089083617f38565b60a0850152505060608201525f60c08201819052608090910152565b60e084015160011901615d4a575f60e085015260c0840182905260808401839052611425565b60e0840151615db85760c084018051908390615d668284617f25565b9052508115615c99575f615d7a85856161f9565b6080870151615d8990846161f9565b615d939190617f25565b9050615dac8660c00151826157ab90919063ffffffff16565b60808701525050611425565b60e08401515f190161142557818460a001511115615de257818460a001818151615cc69190617f38565b818460a0015103615e0657600260e08501525f60a085018190526060850152611425565b5f60e085015260a0840151615e1b9083617f38565b60c0850152505060808201525f60a08201819052606090910152565b5f604051602001615e71906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f615ef06040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b615efa845f6166c8565b602083015281526060840151615f10905f615871565b606082018190528151615f3591676765c793fa10079d601b1b906114a190600a618026565b604082015260208101515f03615f50575f6080820152615ff0565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015615fc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615fea9190617e48565b60808201525b615ffb8460016166c8565b602083018190529082525f03616016575f60a08201526160b6565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561608c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906160b09190617e48565b60a08201525b80608001518160a001519250925050915091565b6109948161670e565b5f604051602001616100906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b038516908201526060810183905260800161421e565b616141617bdd565b615958838361672d565b60408101516001600160a01b031661617657604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f806161d18342617f38565b6161db9085618074565b6301e133809004905061115881676765c793fa10079d601b1b617f25565b5f81156b019d971e4fe8401e740000001983900484111517616219575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f816001600160a01b031663bd02d0f5604051602001615b7a906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff16600281106162a8576162a8617ed7565b602002015190505f6162ba8a8a61793f565b9050805f036162d6575f5f5f5f95509550955095505050615639565b5f6162e5838c60800151617a2d565b90505f6162f2828a6161f9565b90505f8915616317578184106163115761630c84836163c4565b616319565b5f616319565b5f5b90505f616326858d6161f9565b90505f8c1561634b578482106163455761634082866163c4565b61634d565b5f61634d565b5f5b90506163598587617f25565b9f959e50919c50909a509298505050505050505050565b5f8261637c8382617f25565b915081101561115c5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f826163d08382617f38565b915081111561115c5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f816001600160a01b031663bd02d0f5604051602001615b7a9060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b80515f90819061648b90825b60200201515184516001602002015151614194565b9050806040516020016164cf906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016164ff929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f90819061652d9082616476565b9050806040516020016164cf906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f9081906165819082616476565b9050806040516020016164cf9060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906165e29082616476565b9050806040516020016164cf9060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906166489082616476565b9050806040516020016164cf90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b6108d98383836040516024016166999392919061815f565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b1790526160ca565b5f5f5f6166f5855f01518560ff16600281106166e6576166e6617ed7565b60200201518660800151617a2d565b90505f616702868661793f565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b616735617bdd565b8261673e617bdd565b816001600160a01b03166391d4403c60405160200161677e906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156167d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906167f69190617e5f565b61680357915061115c9050565b816001600160a01b031663bd02d0f58560405160200161683d906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161686d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016168a191815260200190565b602060405180830381865afa1580156168bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906168e09190617e48565b816020018181525050816001600160a01b03166321f8a72185604051602001616928906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616958929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161698c91815260200190565b602060405180830381865afa1580156169a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906169cb919061809e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001616a27906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616a57929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616a8b91815260200190565b602060405180830381865afa158015616aa6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616aca919061809e565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616b45929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616b7991815260200190565b602060405180830381865afa158015616b94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616bb89190617e48565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001616c0c9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001616c3c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616c7091815260200190565b602060405180830381865afa158015616c8b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616caf9190617e48565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001616d09906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616d39929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616d6d91815260200190565b602060405180830381865afa158015616d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616dac9190617e48565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001616e05906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616e35929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616e6991815260200190565b602060405180830381865afa158015616e84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616ea89190617e48565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616f28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616f5c91815260200190565b602060405180830381865afa158015616f77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616f9b9190617e48565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001616ff5906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001617025929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161705991815260200190565b602060405180830381865afa158015617074573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906170989190617e48565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161710b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161713f91815260200190565b602060405180830381865afa15801561715a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061717e9190617e48565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016171f2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161722691815260200190565b602060405180830381865afa158015617241573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617265919061809e565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161730c91815260200190565b602060405180830381865afa158015617327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061734b9190617e48565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016173a09060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016173d0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161740491815260200190565b602060405180830381865afa15801561741f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906174439190617e48565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161749e90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016174ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161750291815260200190565b602060405180830381865afa15801561751d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906175419190617e48565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161759b90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016175cb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016175ff91815260200190565b602060405180830381865afa15801561761a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061763e9190617e48565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161769a90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016176ca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016176fe91815260200190565b602060405180830381865afa158015617719573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061773d9190617e48565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161779890602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016177c8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016177fc91815260200190565b602060405180830381865afa158015617817573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061783b9190617e48565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f58560405160200161788a906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016178ba929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016178ee91815260200190565b602060405180830381865afa158015617909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061792d9190617e48565b81516020015160e00152949350505050565b5f5f835f01518360ff166002811061795957617959617ed7565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156179b2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906179d69190617e48565b9050805f036179e9575f9250505061115c565b606082015160c08301516179fd9082617f25565b8210617a215760c0830151617a128284617f38565b617a1c9190617f38565b617a23565b5f5b9695505050505050565b5f8260a001515f03617a4057505f61115c565b5f617a4b8484617a5d565b60a085015190915061115890826161f9565b5f428203617a705750602082015161115c565b5f617a7f8460400151846161c5565b9050617a988460200151826161f990919063ffffffff16565b91505061115c565b604051806102800160405280617ab4617c03565b81526020015f8152602001617ac7617bdd565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806102000160405280617b62617c03565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280617bf0617c9e565b81525f6020820181905260409091015290565b6040518060a00160405280617c16617d0c565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b617cf66040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617cad5790505090565b60405180604001604052806002905b617d5d6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617d1b5790505090565b6001600160a01b0381168114610994575f5ffd5b5f5f82840360c0811215617d99575f5ffd5b8335617da481617d73565b925060a0601f1982011215617db7575f5ffd5b506020830190509250929050565b5f5f828403610100811215617dd8575f5ffd5b8335617de381617d73565b925060e0601f1982011215617db7575f5ffd5b5f60208284031215617e06575f5ffd5b813561595881617d73565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215617e58575f5ffd5b5051919050565b5f60208284031215617e6f575f5ffd5b81518015158114615958575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90617ecf90830184617e7e565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f831280158383131683831282161715617f1e57617f1e617eeb565b5092915050565b8082018082111561115c5761115c617eeb565b8181038181111561115c5761115c617eeb565b6001815b6001841115610dfb57808504811115617f6a57617f6a617eeb565b6001841615617f7857908102905b60019390931c928002617f4f565b5f82617f945750600161115c565b81617fa057505f61115c565b8160018114617fb65760028114617fc057617fdc565b600191505061115c565b60ff841115617fd157617fd1617eeb565b50506001821b61115c565b5060208310610133831016604e8410600b8410161715617fff575081810a61115c565b61800b5f198484617f4b565b805f190482111561801e5761801e617eeb565b029392505050565b5f6159588383617f86565b634e487b7160e01b5f52601260045260245ffd5b5f8261805357618053618031565b500690565b63ffffffff828116828216039081111561115c5761115c617eeb565b808202811582820484141761115c5761115c617eeb565b5f8261809957618099618031565b500490565b5f602082840312156180ae575f5ffd5b815161595881617d73565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61814d6040830185617e7e565b82810360208401526114ee8185617e7e565b606081525f6181716060830186617e7e565b82810360208401526181838186617e7e565b91505082604083015294935050505056fea26469706673582212201d1fe5e07a1921b59f774ad1db097c7c6867b1e45df290a4321ad85f0f6b13f464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04a}\x87V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04a}\xC5V[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08\xDEV[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90a}\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x97V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90a~\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a~HV[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90a~\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90a~HV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90a~_V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90a~\xACV[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\xBEV[a\x06\x1Caz\xA0V[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r\xE2V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\x0E\x03V[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x86Q\x92\x87\x01Q\x90\x87\x01Qa\x06\x8F\x95\x93\x94\x92\x91\x90_a\x0EDV[a\x02@\x86\x01Ra\x01\xA0\x85\x01Ra\x01\x80\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCE\x94\x92\x93\x91\x92\x90\x91a\x10\xA6V[a\x06\xE0\x81_\x01Q\x82a\x02@\x01Qa\x113V[a\x02`\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x10\x93\x92\x91_\x91a\x07\t\x91\x90a~\xFFV[__a\x11bV[a\x073\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\t\x91\x90a~\xFFV[`\xA0\x81\x01Q\x15a\x07KW\x81Q\x81Qa\x07K\x91\x90a\x13\xE6V[a\x07l\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa\x14+V[a\x01`\x82\x01\x81\x90R\x82Q\x82Qa\x07\x81\x92a\x14\xF7V[a\x07\xAD\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87a\x01`\x01Qa\x19\xEFV[a\x07\xC3\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x1AIV[a\x07\xD4\x82` \x01Q\x82_\x01Qa,\xECV[\x81Q` \x82\x01Q\x82Qa\x07\xE8\x92\x91\x90a-\x10V[a\x08\t\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa>\xE5V[a\x02 \x85\x01\x81\x90Ra\x02\0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xE0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xC0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02`\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08\x99\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91a?\xCDV[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80Q\x80Q\x91\x85\x01Q\x80Q\x84\x89\x01Q\x83\x88\x01Q\x93\x86\x01Q\x97\x83\x01Q\x92\x90\x95\x01Qa\x08\xD9\x97\x8B\x96`\x04\x96\x95\x93\x94\x90\x93\x92a@\x84V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\t\x1C\x90a~\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90a~HV[PV[a\t\xC3`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\xBEV[a\t\xCBa{NV[a\t\xE1\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\x0E\x03V[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a~HV[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDE\x91\x90a~HV[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\n\xF7WP`\x80\x81\x01Q\x15[\x15a\x0B\x15W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x10\x15a\x0B0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x10\x15a\x0BKW`\xA0\x82\x01Q`\x80\x82\x01R[a\x0Bq\x82_\x01Q\x82_\x01Q\x83``\x01Q\x84`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`\x01a\x0EDV[a\x01\xC0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B\xA5\x91a\x113V[a\x01\xE0\x82\x01R`\xA0\x81\x01Q\x15a\x0C/W`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x18W__\xFD[PZ\xF1\x15\x80\x15a\x0C*W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C\xB4W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xAFW=__>=_\xFD[PPPP[a\x0C\xC5\x82` \x01Q\x82_\x01Qa,\xECV[``\x81\x01Q\x15a\x0C\xDDW\x81Q\x81Qa\x0C\xDD\x91\x90a\x13\xE6V[\x81Q` \x82\x01Q\x82Qa\x0C\xF1\x92\x91\x90a-\x10V[a\r\x11\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa>\xE5V[a\x01\xA0\x85\x01Ra\x01\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x84\x01R\x16a\x01@\x82\x01R\x80Q``\x82\x01Q`\x80\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\rX\x94\x93\x92\x91\x90a\x14+V[a\x01 \x82\x01\x81\x90R\x82Q\x82Qa\rm\x92a\x14\xF7V[a\x08\xD9\x82` \x01Q\x84\x83a\x01@\x01Q\x84a\x01`\x01Qbz\x12\0\x86a\x01\x80\x01Q\x87a\x01\xA0\x01Q\x88a\x01\xE0\x01Q`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPa?\xCDV[a\t\x94`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aA\"V[a\r\xEAa{\xDDV[_a\r\xF6\x85\x85\x85aAgV[\x91P\x91P[\x93P\x93\x91PPV[a\x0E\x0Ba|\x03V[__a\x0E\x17\x85\x85aA\x94V[\x90P_a\x0E$\x87\x83aB<V[\x90Pa\x0E0\x81\x83aT>V[a\x0E9\x81aTlV[\x96\x90\x95P\x93PPPPV[_____a\x0E\xA1`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8A\x11\x80\x15a\x0E\xAEWP\x87\x15[\x15a\x0F\x89Wa\x0E\xBF\x8D\x8B\x8E\x8AaU\x18V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R\x89\x11\x15a\x0F.W\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0FE\x91\x90a\x7F%V[\x11\x15a\x0F\x84Wa\x01\0\x81\x01Q\x81Qa\x0F]\x91\x90a\x7F%V[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x10sV[_\x8B\x11\x80\x15a\x0F\x96WP\x88\x15[\x15a\x10ZWa\x0F\xA7\x8D\x8C\x8E\x8AaVDV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R\x88\x11\x15a\x10\x1EW` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0F\x84W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84a\x01\0\x01Q\x95P\x95P\x95P\x95P\x95PP\x97P\x97P\x97P\x97P\x97\x92PPPV[_\x83\x11\x80\x15a\x10\xB3WP\x80\x15[\x15a\x11\x02W\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x10\xFDW\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x11,V[_\x84\x11\x80\x15a\x11\x0FWP\x81\x15[\x15a\x11,W\x84QQ` \x01Q\x84\x11\x15a\x11,W\x84Q\x84\x90_a\x10\xD0V[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x11L\x84\x83aWUV[\x90Pa\x11X\x85\x82aWyV[\x91PP[\x92\x91PPV[_a\x11l\x84aW\x96V[\x90P_\x84\x12a\x11\xDAW\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\x8DWa\x11\x8Da~\xD7V[` \x02\x01Q` \x01\x81\x81Qa\x11\xA2\x91\x90a\x7F%V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xBDWa\x11\xBDa~\xD7V[` \x02\x01Q``\x01\x81\x81Qa\x11\xD2\x91\x90a\x7F%V[\x90RPa\x12;V[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xF2Wa\x11\xF2a~\xD7V[` \x02\x01Q` \x01\x81\x81Qa\x12\x07\x91\x90a\x7F8V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12\"Wa\x12\"a~\xD7V[` \x02\x01Q``\x01\x81\x81Qa\x127\x91\x90a\x7F8V[\x90RP[\x81\x15a\x12\xBEW\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x12YWa\x12Ya~\xD7V[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x12}Wa\x12}a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x12\x92\x91\x90a\x7F8V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x12\xADWa\x12\xADa~\xD7V[` \x02\x01Q`@\x01RPa\x13\xDE\x90PV[\x82_\x03a\x12\xCBWPa\x13\xDEV[_a\x12\xD5\x84aW\x96V[\x90P_a\x13\x0B\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x12\xF3Wa\x12\xF3a~\xD7V[` \x02\x01Q` \x01Q\x83aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x13yW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13,Wa\x13,a~\xD7V[` \x02\x01Q`@\x01\x81\x81Qa\x13A\x91\x90a\x7F%V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\\Wa\x13\\a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13q\x91\x90a\x7F%V[\x90RPa\x13\xDAV[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\x91Wa\x13\x91a~\xD7V[` \x02\x01Q`@\x01\x81\x81Qa\x13\xA6\x91\x90a\x7F8V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\xC1Wa\x13\xC1a~\xD7V[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\xD6\x91\x90a\x7F8V[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x14\x04\x84\x84aW\xE6V[\x91P\x91P\x81\x81\x10a\x14%W``\x83\x01Qa\x14\x1F\x90`\x01aXGV[``\x84\x01R[PPPPV[____\x86\x11\x80\x15a\x14;WP\x83\x15[\x15a\x14JWP\x83\x90P\x84a\x14bV[_\x87\x11\x80\x15a\x14WWP\x84\x15[\x15a\x10ZWP\x85\x90P\x82[_a\x14q\x89``\x01Q_aXqV[\x90P_a\x14\x83\x8A``\x01Q`\x01aXqV[\x90P_a\x14\xA6\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x14\xA1\x86`\na\x80&V[aX\x9FV[\x90P_a\x14\xC4\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x14\xA1\x86`\na\x80&V[\x90P\x80_\x03a\x14\xDBW_\x96PPPPPPPa\x14\xEEV[a\x14\xE5\x82\x82aW\xABV[\x96PPPPPPP[\x95\x94PPPPPV[a\x15'`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nupdateTwapPrice`\x88\x1B\x81RPa\r\xBEV[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x15\x81\x84\x84aY_V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x15\x98d\x01\0\0\0\0Ba\x80EV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x16NWa\x15\xBD\x84\x84\x83` \x01QaY\xD5V[a\x15\xC8\x84\x84_aZYV[a\x15\xD7\x84\x84\x83` \x01QaZ\x98V[a\x15\xE2\x84\x84_aZ\xB0V[a\x15\xED\x84\x84\x84aZ\xC8V[a\x16'`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x14%`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83aZ\xE0V[\x80Q` \x82\x01Qa\x16_\x91\x90a\x80XV[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x16\x9B\x92\x16aZ\xE0V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x17\xCBWa\x16\xB5\x84\x84a[\tV[``\x82\x01R`@\x81\x01Qa\x16\xCF\x90c\xFF\xFF\xFF\xFF\x16\x83a\x80tV[\x81``\x01Qa\x16\xDE\x91\x90a\x7F%V[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x17\t\x90\x83aZ\xE0V[a\x17@`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x17y`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01QaZ\xE0V[a\x17\xAE`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaZ\xE0V[a\x17\xBD\x84\x84\x83`\x80\x01QaZYV[a\x17\xCB\x84\x84\x83_\x01QaY\xD5V[a\x17\xD5\x84\x84a[\"V[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x17\xF1\x91\x90a\x80XV[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x18\x05\x84a[;V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x18=\x92\x16aZ\xE0V[a\x18\x86`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[a\x18\xC5`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16aZ\xE0V[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14%Wa\x18\xEB\x84\x84a[\tV[`\x80\x82\x01Ra\x18\xFA\x84\x84a[\xEDV[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x19\x1E\x91a\x7F8V[a\x19(\x91\x90a\x80\x8BV[a\x01 \x82\x01R` \x80\x82\x01Qc\xFF\xFF\xFF\xFF\x16`\xA0\x83\x01R`\x80\x82\x01Qa\x01\0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0\x92\x81\x01\x92\x90\x92RQa\x19\x8D\x91\x90aZ\xE0V[a\x19\xC0`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01QaZ\xE0V[a\x19\xCF\x84\x84\x83`\xA0\x01QaZ\x98V[a\x19\xDF\x84\x84\x83a\x01\0\x01QaZ\xB0V[a\x14%\x84\x84\x83a\x01 \x01QaZ\xC8V[_a\x19\xFA\x85\x84a~\xFFV[\x90P_\x81\x13\x15a\x1A$W\x86Q` \x01Qa\x1A\x1F\x90\x83a\x1A\x18\x84aW\x96V[`\x01a\\\x06V[a\x1A?V[\x86Q` \x01Qa\x1A?\x90\x83a\x1A8\x84aW\x96V[`\x01a]$V[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1A\x8D\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xDDW__\xFD[PZ\xF1\x15\x80\x15a\x1A\xEFW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x1B\x0F\x84`@\x01Qa^7V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BMW__\xFD[PZ\xF1\x15\x80\x15a\x1B_W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\x9D\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CN\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1C\x8E\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x1D\t\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DI\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1D\x89\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E@\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\x85\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xB5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F{\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a +\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a v\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!'\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!q\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\"\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"n\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x1F\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#j\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x9A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xF7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x1B\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$Z\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\x8A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x0C\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a%L\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x06\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&K\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&{\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFE\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'C\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a's\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(A\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xF5\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)?\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xF3\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*?\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xF3\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+>\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF2\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,a\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11,\x91\x90a~HV[a,\xF5\x81a^\xBBV[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a-P\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\xA0W__\xFD[PZ\xF1\x15\x80\x15a-\xB2W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a-\xF6\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xAD\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.\xF5\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA6\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a/\xED\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x9D\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0\xE9\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1vW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\x9A\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1\xBA\x90a\x80\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2GW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2k\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2\xB8\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3i\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a3\xB2\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4c\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a4\xA4\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5^\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a5\xA6\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\xD6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a65W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6Y\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a6\xA0\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6\xD0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7S\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7\x9F\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8S\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a8s\x90a\x80\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9'\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9t\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a:\x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:(\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a:q\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;\x01W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;%\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a;c\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\x93\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\xDD\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x1D\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a<o\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra<\xEA\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=*\x91\x90a\x80\x9EV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a=q\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xA1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\"\x91\x90a~HV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a>t\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a?\"`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a?/WP\x85\x15[\x15a?fW\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90Ra?\xA8V[_\x89\x11\x80\x15a?sWP\x86\x15[\x15a?\xA8W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a@cW__\xFD[PZ\xF1\x15\x80\x15a@uW=__>=_\xFD[PPPPPPPPPPPPPV[`@Qc\x04\xE6\xBD\xD1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x88\x81\x16`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R`\xE4\x82\x01\x84\x90Ra\x01\x04\x82\x01\x83\x90R\x8B\x16\x90c\t\xCD{\xA2\x90a\x01$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aA\0W__\xFD[PZ\xF1\x15\x80\x15aA\x12W=__>=_\xFD[PPPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01aA8\x92\x91\x90a\x81;V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra`\xCAV[aAoa{\xDDV[__aA{\x86\x85a`\xD3V[\x90P_aA\x88\x86\x83aa9V[\x90Pa\x0E9\x87\x82aaKV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aA\xB5W\x81\x83aA\xB8V[\x82\x82[`@Q\x91\x94P\x92PaA\xE5\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[aBDa|\x03V[\x82aBMa|\x03V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aB\x89\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x01\x91\x90a~_V[aC\x0EW\x91Pa\x11\\\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aCN\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC\xB2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\xF1\x91\x90a\x80\x9EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aDo\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xA3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xE2\x91\x90a~HV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE8\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aEh\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x9C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xDB\x91\x90a~HV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF6\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aFf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xD9\x91\x90a~HV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aG\x02\x90` \x01a\x80\xB9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aGf\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xA5\x91\x90a~HV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\x01\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aHe\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\xA4\x91\x90a~HV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aIU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aIpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x94\x91\x90a~HV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJXW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ|\x91\x90a\x80\x9EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKe\x91\x90a~HV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK\xBC\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL_\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aL\xBB\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM^\x91\x90a~HV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM\x8E\x90a\x80\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN1\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN\x8E\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO1\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aO\x8A\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP-\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aP{\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x1E\x91\x90a\x80\x9EV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aQ\x8C\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR/\x91\x90a\x80\x9EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\x92\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS5\x91\x90a~HV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aS\x8E\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT1\x91\x90a~HV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aT~WPPV[\x81QQ`\xA0\x01Q\x15aT\xD4W\x81Q_\x90aT\xA7\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01Qaa\xC5V[\x83Q\x90\x91PaT\xCB\x90\x82\x90_[` \x02\x01Q` \x01Qaa\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aU\x10W\x81Q_\x90aT\xF2\x90`\x01aT\x94V[\x83Q\x90\x91PaU\x04\x90\x82\x90`\x01aT\xB4V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aU$a|7V[aU-\x89ab:V[a\x01\xC0\x82\x01\x81\x90RaUD\x90\x88\x90_\x90\x81\x90ab\x8BV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaUc\x90\x88\x90`\x01\x90_\x90ab\x8BV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aU\x7FWP` \x81\x01Q\x15[\x15aU\x96W____\x94P\x94P\x94P\x94PPaV9V[\x85\x15aU\xB2W\x87\x81` \x01\x81\x81QaU\xAE\x91\x90a\x7F8V[\x90RP[\x80Q` \x82\x01QaU\xC8\x91\x90a\x14\xA1\x81\x8CacpV[`\x80\x82\x01\x81\x90R\x81QaU\xDA\x91ac\xC4V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01QaV\x11\x91aV\t\x90a'\x10\x90ac\xC4V[a'\x10aX\x9FV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01QaV/\x91aWUV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____aVPa|7V[aVY\x89ab:V[a\x01\xC0\x82\x01\x81\x90RaVp\x90\x88\x90_\x90\x81\x90ab\x8BV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaV\x8F\x90\x88\x90`\x01\x90_\x90ab\x8BV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aV\xABWP` \x81\x01Q\x15[\x15aV\xC2W____\x94P\x94P\x94P\x94PPaV9V[\x85\x15aV\xDDW\x87\x81_\x01\x81\x81QaV\xD9\x91\x90a\x7F8V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90RaW\x04\x90\x89\x90aV\t\x90a'\x10\x90ac\xC4V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01QaW\"\x92a\x14\xA1\x90\x83\x90acpV[`\x80\x82\x01\x81\x90R` \x82\x01QaW7\x91ac\xC4V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01QaV/\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aWkW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aW\x8F\x90\x83\x90a\x7F%V[\x90RPPPV[__\x82\x12\x15aW\xA7W\x81_\x03a\x11\\V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aW\xCCW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aW\xF7\x84``\x01Q_aXqV[\x90P_aX\x03\x86ad\x19V[\x90P_aX&\x82aX\x15\x85`\na\x80&V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaX\x9FV[\x90P_aX5\x87___ab\x8BV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aXUW_aXXV[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aX\x91WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aX\xD3W\x83\x82\x81aX\xC9WaX\xC9a\x801V[\x04\x92PPPaYXV[\x80\x84\x11aX\xF3W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84adjV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aY\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aYX\x91\x90a~HV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aY\xED\x84adjV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15aZ5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14%\x91\x90a~HV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84ae\x1DV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZ\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aY\xED\x84aeqV[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84ae\xD2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:aZq\x84af8V[a\x027`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83af\x81V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84ae\x1DV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84aeqV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\\\x91\x90a~HV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5aYx\x84ae\xD2V[`\xE0\x84\x01Q`\x01\x19\x01a\\-W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x14%V[`\xE0\x84\x01Q_\x19\x01a\\\x9FW`\xA0\x84\x01\x80Q\x90\x83\x90a\\L\x82\x84a\x7F%V[\x90RP\x81\x15a\\\x99W_a\\`\x85\x85aa\xF9V[``\x87\x01Qa\\o\x90\x84aa\xF9V[a\\y\x91\x90a\x7F%V[\x90Pa\\\x92\x86`\xA0\x01Q\x82aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x14%V[`\xE0\x84\x01Qa\x14%W\x81\x84`\xC0\x01Q\x11\x15a\\\xCEW\x81\x84`\xC0\x01\x81\x81Qa\\\xC6\x91\x90a\x7F8V[\x90RPa\x14%V[\x81\x84`\xC0\x01Q\x03a\\\xF2W`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x14%V[`\x01`\xE0\x85\x01R`\xC0\x84\x01Qa]\x08\x90\x83a\x7F8V[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01a]JW_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x14%V[`\xE0\x84\x01Qa]\xB8W`\xC0\x84\x01\x80Q\x90\x83\x90a]f\x82\x84a\x7F%V[\x90RP\x81\x15a\\\x99W_a]z\x85\x85aa\xF9V[`\x80\x87\x01Qa]\x89\x90\x84aa\xF9V[a]\x93\x91\x90a\x7F%V[\x90Pa]\xAC\x86`\xC0\x01Q\x82aW\xAB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x14%V[`\xE0\x84\x01Q_\x19\x01a\x14%W\x81\x84`\xA0\x01Q\x11\x15a]\xE2W\x81\x84`\xA0\x01\x81\x81Qa\\\xC6\x91\x90a\x7F8V[\x81\x84`\xA0\x01Q\x03a^\x06W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x14%V[_`\xE0\x85\x01R`\xA0\x84\x01Qa^\x1B\x90\x83a\x7F8V[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01a^q\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__a^\xF0`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a^\xFA\x84_af\xC8V[` \x83\x01R\x81R``\x84\x01Qa_\x10\x90_aXqV[``\x82\x01\x81\x90R\x81Qa_5\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x14\xA1\x90`\na\x80&V[`@\x82\x01R` \x81\x01Q_\x03a_PW_`\x80\x82\x01Ra_\xF0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_\xEA\x91\x90a~HV[`\x80\x82\x01R[a_\xFB\x84`\x01af\xC8V[` \x83\x01\x81\x90R\x90\x82R_\x03a`\x16W_`\xA0\x82\x01Ra`\xB6V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`\xB0\x91\x90a~HV[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[a\t\x94\x81ag\x0EV[_`@Q` \x01aa\0\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01aB\x1EV[aaAa{\xDDV[aYX\x83\x83ag-V[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aavW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80aa\xD1\x83Ba\x7F8V[aa\xDB\x90\x85a\x80tV[c\x01\xE13\x80\x90\x04\x90Pa\x11X\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x7F%V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17ab\x19W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10ab\xA8Wab\xA8a~\xD7V[` \x02\x01Q\x90P_ab\xBA\x8A\x8Aay?V[\x90P\x80_\x03ab\xD6W____\x95P\x95P\x95P\x95PPPaV9V[_ab\xE5\x83\x8C`\x80\x01Qaz-V[\x90P_ab\xF2\x82\x8Aaa\xF9V[\x90P_\x89\x15ac\x17W\x81\x84\x10ac\x11Wac\x0C\x84\x83ac\xC4V[ac\x19V[_ac\x19V[_[\x90P_ac&\x85\x8Daa\xF9V[\x90P_\x8C\x15acKW\x84\x82\x10acEWac@\x82\x86ac\xC4V[acMV[_acMV[_[\x90PacY\x85\x87a\x7F%V[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x82ac|\x83\x82a\x7F%V[\x91P\x81\x10\x15a\x11\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x82ac\xD0\x83\x82a\x7F8V[\x91P\x81\x11\x15a\x11\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a[z\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90ad\x8B\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQaA\x94V[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ad\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90ae-\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90ae\x81\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ae\xE2\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90afH\x90\x82advV[\x90P\x80`@Q` \x01ad\xCF\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[a\x08\xD9\x83\x83\x83`@Q`$\x01af\x99\x93\x92\x91\x90a\x81_V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra`\xCAV[___af\xF5\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10af\xE6Waf\xE6a~\xD7V[` \x02\x01Q\x86`\x80\x01Qaz-V[\x90P_ag\x02\x86\x86ay?V[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[ag5a{\xDDV[\x82ag>a{\xDDV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01ag~\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ag\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ag\xF6\x91\x90a~_V[ah\x03W\x91Pa\x11\\\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ah=\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ahm\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ah\xA1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ah\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ah\xE0\x91\x90a~HV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01ai(\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aiX\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ai\x8C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ai\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ai\xCB\x91\x90a\x80\x9EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aj'\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ajW\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aj\xCA\x91\x90a\x80\x9EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01akE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aky\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ak\xB8\x91\x90a~HV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01al\x0C\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01alp\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\xAF\x91\x90a~HV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01am\t\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01am9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01amm\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am\xAC\x91\x90a~HV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01an\x05\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01an5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ani\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15an\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90an\xA8\x91\x90a~HV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ao(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ao\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aowW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ao\x9B\x91\x90a~HV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ao\xF5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ap%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01apY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aptW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ap\x98\x91\x90a~HV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\x0B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aq?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aqZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aq~\x91\x90a~HV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ar&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15arAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90are\x91\x90a\x80\x9EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01as\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15as'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90asK\x91\x90a~HV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01as\xA0\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01as\xD0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01at\x04\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15at\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90atC\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01at\x9E\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01at\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15au\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90auA\x91\x90a~HV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01au\x9B\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01au\xCB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au\xFF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15av\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90av>\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01av\x9A\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01av\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01av\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aw\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aw=\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aw\x98\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aw\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aw\xFC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ax\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ax;\x91\x90a~HV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ax\x8A\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ax\xBA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ax\xEE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay-\x91\x90a~HV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10ayYWayYa~\xD7V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay\xD6\x91\x90a~HV[\x90P\x80_\x03ay\xE9W_\x92PPPa\x11\\V[``\x82\x01Q`\xC0\x83\x01Qay\xFD\x90\x82a\x7F%V[\x82\x10az!W`\xC0\x83\x01Qaz\x12\x82\x84a\x7F8V[az\x1C\x91\x90a\x7F8V[az#V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03az@WP_a\x11\\V[_azK\x84\x84az]V[`\xA0\x85\x01Q\x90\x91Pa\x11X\x90\x82aa\xF9V[_B\x82\x03azpWP` \x82\x01Qa\x11\\V[_az\x7F\x84`@\x01Q\x84aa\xC5V[\x90Paz\x98\x84` \x01Q\x82aa\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x11\\V[`@Q\x80a\x02\x80\x01`@R\x80az\xB4a|\x03V[\x81R` \x01_\x81R` \x01az\xC7a{\xDDV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x02\0\x01`@R\x80a{ba|\x03V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a{\xF0a|\x9EV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80a|\x16a}\x0CV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a|\xF6`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a|\xADW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a}]`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a}\x1BW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x94W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15a}\x99W__\xFD[\x835a}\xA4\x81a}sV[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15a}\xB7W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15a}\xD8W__\xFD[\x835a}\xE3\x81a}sV[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15a}\xB7W__\xFD[_` \x82\x84\x03\x12\x15a~\x06W__\xFD[\x815aYX\x81a}sV[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a~XW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a~oW__\xFD[\x81Q\x80\x15\x15\x81\x14aYXW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a~\xCF\x90\x83\x01\x84a~~V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x7F\x1EWa\x7F\x1Ea~\xEBV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x11\\Wa\x11\\a~\xEBV[\x81\x81\x03\x81\x81\x11\x15a\x11\\Wa\x11\\a~\xEBV[`\x01\x81[`\x01\x84\x11\x15a\r\xFBW\x80\x85\x04\x81\x11\x15a\x7FjWa\x7Fja~\xEBV[`\x01\x84\x16\x15a\x7FxW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x7FOV[_\x82a\x7F\x94WP`\x01a\x11\\V[\x81a\x7F\xA0WP_a\x11\\V[\x81`\x01\x81\x14a\x7F\xB6W`\x02\x81\x14a\x7F\xC0Wa\x7F\xDCV[`\x01\x91PPa\x11\\V[`\xFF\x84\x11\x15a\x7F\xD1Wa\x7F\xD1a~\xEBV[PP`\x01\x82\x1Ba\x11\\V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x7F\xFFWP\x81\x81\na\x11\\V[a\x80\x0B_\x19\x84\x84a\x7FKV[\x80_\x19\x04\x82\x11\x15a\x80\x1EWa\x80\x1Ea~\xEBV[\x02\x93\x92PPPV[_aYX\x83\x83a\x7F\x86V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x80SWa\x80Sa\x801V[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11\\Wa\x11\\a~\xEBV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11\\Wa\x11\\a~\xEBV[_\x82a\x80\x99Wa\x80\x99a\x801V[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x80\xAEW__\xFD[\x81QaYX\x81a}sV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a\x81M`@\x83\x01\x85a~~V[\x82\x81\x03` \x84\x01Ra\x14\xEE\x81\x85a~~V[``\x81R_a\x81q``\x83\x01\x86a~~V[\x82\x81\x03` \x84\x01Ra\x81\x83\x81\x86a~~V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x1D\x1F\xE5\xE0z\x19!\xB5\x9FwJ\xD1\xDB\t||hg\xB1\xE4]\xF2\x90\xA42\x1A\xD8_\x0Fk\x13\xF4dsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `AccountNotMatch(address,address)` and selector `0x25c7157e`.
```solidity
error AccountNotMatch(address accountInPosition, address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccountNotMatch {
        pub accountInPosition: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<AccountNotMatch> for UnderlyingRustTuple<'_> {
            fn from(value: AccountNotMatch) -> Self {
                (value.accountInPosition, value.account)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AccountNotMatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    accountInPosition: tuple.0,
                    account: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccountNotMatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccountNotMatch(address,address)";
            const SELECTOR: [u8; 4] = [37u8, 199u8, 21u8, 126u8];
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
                        &self.accountInPosition,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
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
    /**Custom error with signature `EmptyPosition()` and selector `0x4dfbbff3`.
```solidity
error EmptyPosition();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyPosition {}
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
        impl ::core::convert::From<EmptyPosition> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyPosition) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyPosition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyPosition {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyPosition()";
            const SELECTOR: [u8; 4] = [77u8, 251u8, 191u8, 243u8];
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
    /**Custom error with signature `EmptySwapInAmount()` and selector `0xf9381afa`.
```solidity
error EmptySwapInAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptySwapInAmount {}
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
        impl ::core::convert::From<EmptySwapInAmount> for UnderlyingRustTuple<'_> {
            fn from(value: EmptySwapInAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptySwapInAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptySwapInAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptySwapInAmount()";
            const SELECTOR: [u8; 4] = [249u8, 56u8, 26u8, 250u8];
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
    /**Custom error with signature `InsufficientSwapCollateral(uint256,uint256)` and selector `0x671abd07`.
```solidity
error InsufficientSwapCollateral(uint256 amountIn, uint256 collateral);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSwapCollateral {
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        pub collateral: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InsufficientSwapCollateral>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSwapCollateral) -> Self {
                (value.amountIn, value.collateral)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientSwapCollateral {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountIn: tuple.0,
                    collateral: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSwapCollateral {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSwapCollateral(uint256,uint256)";
            const SELECTOR: [u8; 4] = [103u8, 26u8, 189u8, 7u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.collateral),
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
    /**Custom error with signature `RequestedAmount1ExceedsPriceLimit(uint256,uint256)` and selector `0xea1d6892`.
```solidity
error RequestedAmount1ExceedsPriceLimit(uint256 amount1Out, uint256 amount1OutMax);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestedAmount1ExceedsPriceLimit {
        pub amount1Out: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1OutMax: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<RequestedAmount1ExceedsPriceLimit>
        for UnderlyingRustTuple<'_> {
            fn from(value: RequestedAmount1ExceedsPriceLimit) -> Self {
                (value.amount1Out, value.amount1OutMax)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RequestedAmount1ExceedsPriceLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount1Out: tuple.0,
                    amount1OutMax: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RequestedAmount1ExceedsPriceLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RequestedAmount1ExceedsPriceLimit(uint256,uint256)";
            const SELECTOR: [u8; 4] = [234u8, 29u8, 104u8, 146u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1Out),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1OutMax),
                )
            }
        }
    };
    /**Custom error with signature `RequestedAmountOExceedsPriceLimit(uint256,uint256)` and selector `0xcf0f1582`.
```solidity
error RequestedAmountOExceedsPriceLimit(uint256 amount0Out, uint256 amount0OutMax);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestedAmountOExceedsPriceLimit {
        pub amount0Out: alloy::sol_types::private::primitives::aliases::U256,
        pub amount0OutMax: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<RequestedAmountOExceedsPriceLimit>
        for UnderlyingRustTuple<'_> {
            fn from(value: RequestedAmountOExceedsPriceLimit) -> Self {
                (value.amount0Out, value.amount0OutMax)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RequestedAmountOExceedsPriceLimit {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount0Out: tuple.0,
                    amount0OutMax: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RequestedAmountOExceedsPriceLimit {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RequestedAmountOExceedsPriceLimit(uint256,uint256)";
            const SELECTOR: [u8; 4] = [207u8, 15u8, 21u8, 130u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0Out),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0OutMax),
                )
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
    /**Custom error with signature `SingleTokenInOutSwapOnly()` and selector `0x6331fab1`.
```solidity
error SingleTokenInOutSwapOnly();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SingleTokenInOutSwapOnly {}
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
        impl ::core::convert::From<SingleTokenInOutSwapOnly>
        for UnderlyingRustTuple<'_> {
            fn from(value: SingleTokenInOutSwapOnly) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SingleTokenInOutSwapOnly {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SingleTokenInOutSwapOnly {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SingleTokenInOutSwapOnly()";
            const SELECTOR: [u8; 4] = [99u8, 49u8, 250u8, 177u8];
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
    /**Function with signature `executeSwap(address,(address,address,uint256,uint256,uint256,uint256,address))` and selector `0xd9c42742`.
```solidity
function executeSwap(address account, SwapUtils.SwapParams memory swapParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapCall {
        pub account: alloy::sol_types::private::Address,
        pub swapParams: <SwapUtils::SwapParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeSwap(address,(address,address,uint256,uint256,uint256,uint256,address))`](executeSwapCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                SwapUtils::SwapParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.account, value.swapParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeSwapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        swapParams: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                SwapUtils::SwapParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeSwapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeSwap(address,(address,address,uint256,uint256,uint256,uint256,address))";
            const SELECTOR: [u8; 4] = [217u8, 196u8, 39u8, 66u8];
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
                    <SwapUtils::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self.swapParams,
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
    /**Function with signature `executeSwapInPosition(address,(uint256,uint256,uint256,uint256,uint256))` and selector `0x52b5de3d`.
```solidity
function executeSwapInPosition(address account, SwapUtils.SwapInPositionParams memory swapParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSwapInPositionCall {
        pub account: alloy::sol_types::private::Address,
        pub swapParams: <SwapUtils::SwapInPositionParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeSwapInPosition(address,(uint256,uint256,uint256,uint256,uint256))`](executeSwapInPositionCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                SwapUtils::SwapInPositionParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.account, value.swapParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeSwapInPositionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        swapParams: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                SwapUtils::SwapInPositionParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeSwapInPositionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeSwapInPosition(address,(uint256,uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [82u8, 181u8, 222u8, 61u8];
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
                    <SwapUtils::SwapInPositionParams as alloy_sol_types::SolType>::tokenize(
                        &self.swapParams,
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
    ///Container for all the [`SwapHandler`](self) function calls.
    pub enum SwapHandlerCalls {
        dataStore(dataStoreCall),
        eventEmitter(eventEmitterCall),
        executeSwap(executeSwapCall),
        executeSwapInPosition(executeSwapInPositionCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl SwapHandlerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [74u8, 74u8, 123u8, 4u8],
            [82u8, 181u8, 222u8, 61u8],
            [102u8, 13u8, 13u8, 103u8],
            [159u8, 247u8, 140u8, 48u8],
            [217u8, 196u8, 39u8, 66u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SwapHandlerCalls {
        const NAME: &'static str = "SwapHandlerCalls";
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
                Self::executeSwap(_) => {
                    <executeSwapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeSwapInPosition(_) => {
                    <executeSwapInPositionCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SwapHandlerCalls>] = &[
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn executeSwapInPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerCalls> {
                        <executeSwapInPositionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerCalls::executeSwapInPosition)
                    }
                    executeSwapInPosition
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn eventEmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerCalls> {
                        <eventEmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerCalls::eventEmitter)
                    }
                    eventEmitter
                },
                {
                    fn executeSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerCalls> {
                        <executeSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerCalls::executeSwap)
                    }
                    executeSwap
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
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SwapHandler`](self) custom errors.
    pub enum SwapHandlerErrors {
        AccountNotMatch(AccountNotMatch),
        EmptyPool(EmptyPool),
        EmptyPosition(EmptyPosition),
        EmptySwapInAmount(EmptySwapInAmount),
        InsufficientSwapCollateral(InsufficientSwapCollateral),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        RequestedAmount1ExceedsPriceLimit(RequestedAmount1ExceedsPriceLimit),
        RequestedAmountOExceedsPriceLimit(RequestedAmountOExceedsPriceLimit),
        Reserve0Insufficient(Reserve0Insufficient),
        Reserve1Insufficient(Reserve1Insufficient),
        SingleTokenInOutSwapOnly(SingleTokenInOutSwapOnly),
        Unauthorized(Unauthorized),
    }
    #[automatically_derived]
    impl SwapHandlerErrors {
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
            [37u8, 199u8, 21u8, 126u8],
            [77u8, 251u8, 191u8, 243u8],
            [99u8, 49u8, 250u8, 177u8],
            [103u8, 26u8, 189u8, 7u8],
            [115u8, 87u8, 217u8, 31u8],
            [163u8, 91u8, 21u8, 11u8],
            [207u8, 15u8, 21u8, 130u8],
            [234u8, 29u8, 104u8, 146u8],
            [249u8, 56u8, 26u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SwapHandlerErrors {
        const NAME: &'static str = "SwapHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 12usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccountNotMatch(_) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyPosition(_) => {
                    <EmptyPosition as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptySwapInAmount(_) => {
                    <EmptySwapInAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientSwapCollateral(_) => {
                    <InsufficientSwapCollateral as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MathOverflowedMulDiv(_) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RequestedAmount1ExceedsPriceLimit(_) => {
                    <RequestedAmount1ExceedsPriceLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RequestedAmountOExceedsPriceLimit(_) => {
                    <RequestedAmountOExceedsPriceLimit as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve0Insufficient(_) => {
                    <Reserve0Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Reserve1Insufficient(_) => {
                    <Reserve1Insufficient as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SingleTokenInOutSwapOnly(_) => {
                    <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SwapHandlerErrors>] = &[
                {
                    fn Reserve1Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <Reserve1Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::Reserve1Insufficient)
                    }
                    Reserve1Insufficient
                },
                {
                    fn Reserve0Insufficient(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <Reserve0Insufficient as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::Reserve0Insufficient)
                    }
                    Reserve0Insufficient
                },
                {
                    fn MathOverflowedMulDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::MathOverflowedMulDiv)
                    }
                    MathOverflowedMulDiv
                },
                {
                    fn AccountNotMatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <AccountNotMatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::AccountNotMatch)
                    }
                    AccountNotMatch
                },
                {
                    fn EmptyPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <EmptyPosition as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::EmptyPosition)
                    }
                    EmptyPosition
                },
                {
                    fn SingleTokenInOutSwapOnly(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::SingleTokenInOutSwapOnly)
                    }
                    SingleTokenInOutSwapOnly
                },
                {
                    fn InsufficientSwapCollateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <InsufficientSwapCollateral as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::InsufficientSwapCollateral)
                    }
                    InsufficientSwapCollateral
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::EmptyPool)
                    }
                    EmptyPool
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn RequestedAmountOExceedsPriceLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <RequestedAmountOExceedsPriceLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::RequestedAmountOExceedsPriceLimit)
                    }
                    RequestedAmountOExceedsPriceLimit
                },
                {
                    fn RequestedAmount1ExceedsPriceLimit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <RequestedAmount1ExceedsPriceLimit as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::RequestedAmount1ExceedsPriceLimit)
                    }
                    RequestedAmount1ExceedsPriceLimit
                },
                {
                    fn EmptySwapInAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <EmptySwapInAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::EmptySwapInAmount)
                    }
                    EmptySwapInAmount
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
                Self::AccountNotMatch(inner) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyPosition(inner) => {
                    <EmptyPosition as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptySwapInAmount(inner) => {
                    <EmptySwapInAmount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientSwapCollateral(inner) => {
                    <InsufficientSwapCollateral as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RequestedAmount1ExceedsPriceLimit(inner) => {
                    <RequestedAmount1ExceedsPriceLimit as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RequestedAmountOExceedsPriceLimit(inner) => {
                    <RequestedAmountOExceedsPriceLimit as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::SingleTokenInOutSwapOnly(inner) => {
                    <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AccountNotMatch(inner) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyPosition(inner) => {
                    <EmptyPosition as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptySwapInAmount(inner) => {
                    <EmptySwapInAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientSwapCollateral(inner) => {
                    <InsufficientSwapCollateral as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::RequestedAmount1ExceedsPriceLimit(inner) => {
                    <RequestedAmount1ExceedsPriceLimit as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RequestedAmountOExceedsPriceLimit(inner) => {
                    <RequestedAmountOExceedsPriceLimit as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::SingleTokenInOutSwapOnly(inner) => {
                    <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::abi_encode_raw(
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
    /**Creates a new wrapper around an on-chain [`SwapHandler`](self) contract instance.

See the [wrapper's documentation](`SwapHandlerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SwapHandlerInstance<T, P, N> {
        SwapHandlerInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<SwapHandlerInstance<T, P, N>>,
    > {
        SwapHandlerInstance::<
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
        SwapHandlerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**A [`SwapHandler`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SwapHandler`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SwapHandlerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SwapHandlerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SwapHandlerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SwapHandlerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SwapHandler`](self) contract instance.

See the [wrapper's documentation](`SwapHandlerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SwapHandlerInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SwapHandlerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SwapHandlerInstance<T, P, N> {
            SwapHandlerInstance {
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
    > SwapHandlerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`executeSwap`] function.
        pub fn executeSwap(
            &self,
            account: alloy::sol_types::private::Address,
            swapParams: <SwapUtils::SwapParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeSwapCall, N> {
            self.call_builder(
                &executeSwapCall {
                    account,
                    swapParams,
                },
            )
        }
        ///Creates a new call builder for the [`executeSwapInPosition`] function.
        pub fn executeSwapInPosition(
            &self,
            account: alloy::sol_types::private::Address,
            swapParams: <SwapUtils::SwapInPositionParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeSwapInPositionCall, N> {
            self.call_builder(
                &executeSwapInPositionCall {
                    account,
                    swapParams,
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
    > SwapHandlerInstance<T, P, N> {
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
