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
    error EmptyAmount();
    error EmptyPool(bytes32 key);
    error EmptyPosition();
    error EmptySwapInAmount();
    error EmptySwapOutAmount();
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
    "name": "EmptyAmount",
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
    "name": "EmptySwapOutAmount",
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
    ///0x60e060405234801561000f575f5ffd5b5060405161847d38038061847d83398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c05161837261010b5f395f818160dd015281816101c601526102b201525f8181605e015261054c01525f818160b60152818161019701528181610283015281816103750152818161047b01526108e001526183725ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004617f01565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617f3f565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b506102376108de565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f19190617f70565b6001600160a01b031681526020018360200160208101906103129190617f70565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b9190617f70565b6001600160a01b03169052905061022e8382610997565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b190617f8b565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104249190617fc2565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b790617f8b565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102379190617fc2565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd9190617fd9565b61023757338160405163a35b150b60e01b8152600401610470929190618026565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610dbe565b61061c617c1a565b61062e83835f01518460400151610de2565b606083015260408201819052825190518051516020919091015151610654929190610e03565b6020830152808252606083015160a0808401829052608085015160c08086018290528651928701519087015161068f9593949291905f610e44565b6102408601526101a0850152610180840152610100830181905260e08301829052604083015160a084015160c08501516106ce94929391929091611117565b6106e0815f01518261024001516111a4565b6102608201528051604082015160a083015160e08401516107109392915f916107099190618079565b5f5f6111d3565b610733815f0151826040015160018460c001518561010001516107099190618079565b60a08101511561074b578151815161074b9190611457565b61076c815f01518260a001518360c001518460e0015185610100015161149c565b61016082018190528251825161078192611669565b6107ad815f015182604001518360a001518460c001518560e00151866101000151876101600151611b69565b6107c3825f015182606001518360400151611bc3565b6107d48260200151825f0151612e66565b8151602082015182516107e8929190612e8a565b610809815f01518260a001518360c001518460e0015185610100015161405f565b610220850181905261020085018290526001600160a01b039283166101e08601819052939092166101c085018190526020868101516040888101516102608901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108999792968b9693949193929091614147565b60208281015160408381015151805180519185015180518489015183880151938601519783015192909501516108d9978b966004969593949093926141fe565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161091c90617f8b565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109949190617fc2565b50565b6109c36040518060400160405280600b81526020016a065786563757465537761760ac1b815250610dbe565b6109cb617cc8565b6109e1825f015183604001518460600151610e03565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af1158015610a3f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a639190617fc2565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610aba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ade9190617fc2565b60808201526060810151158015610af757506080810151155b15610b1557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511115610b3057608082015160608201525b8160a0015181608001511115610b4b5760a082015160808201525b610b71825f0151825f0151836060015184608001518660c001518760e001516001610e44565b6101c0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610ba5916111a4565b6101e082015260a081015115610c2f576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c18575f5ffd5b505af1158015610c2a573d5f5f3e3d5ffd5b505050505b60c081015115610cb457604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c9d575f5ffd5b505af1158015610caf573d5f5f3e3d5ffd5b505050505b610cc58260200151825f0151612e66565b606081015115610cdd5781518151610cdd9190611457565b815160208201518251610cf1929190612e8a565b610d11815f0151826060015183608001518460a001518560c0015161405f565b6101a08501526101808401526001600160a01b039081166101608401521661014082015280516060820151608083015160a084015160c0850151610d58949392919061149c565b610120820181905282518251610d6d92611669565b6108d9826020015184836101400151846101600151627a1200866101800151876101a00151886101e0015160405180608001604052805f81526020015f81526020015f81526020015f815250614147565b61099460405180604001604052806002815260200161257360f01b8152508261429c565b610dea617d57565b5f610df68585856142e1565b915091505b935093915050565b610e0b617d7d565b5f5f610e17858561430e565b90505f610e2487836143b6565b9050610e3081836155b8565b610e39816155e6565b969095509350505050565b5f5f5f5f5f610ea16040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8b118015610eaf57505f8a115b15610ecd57604051636331fab160e01b815260040160405180910390fd5b5f8a118015610eda575087155b15610fd657610eeb8d8b8e8a615692565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e082018190525f03610f4c576040516317a6ef6360e31b815260040160405180910390fd5b8051891115610f7b5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610f92919061809f565b1115610fd1576101008101518151610faa919061809f565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b6110e4565b5f8b118015610fe3575088155b156110cb57610ff48d8c8e8a6157be565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e082018190525f0361105a576040516317a6ef6360e31b815260040160405180910390fd5b878160200151101561108f57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610fd15760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805f0151816020015182604001518360600151846101000151955095509550955095505097509750975097509792505050565b5f83118015611124575080155b15611173578451602090810151015183111561116e578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b61119d565b5f84118015611180575081155b1561119d578451516020015184111561119d57845184905f611141565b5050505050565b60608201515f9060481c61ffff16816111bd84836158cf565b90506111c985826158f3565b9150505b92915050565b5f6111dd84615910565b90505f841261124b578551819060ff8716600281106111fe576111fe618051565b6020020151602001818151611213919061809f565b9052508651819060ff87166002811061122e5761122e618051565b6020020151606001818151611243919061809f565b9052506112ac565b8551819060ff87166002811061126357611263618051565b602002015160200181815161127891906180b2565b9052508651819060ff87166002811061129357611293618051565b60200201516060018181516112a891906180b2565b9052505b811561132f5785515f9060ff8716600281106112ca576112ca618051565b602002015160400151905080885f01518760ff16600281106112ee576112ee618051565b602002015160a00181815161130391906180b2565b90525086515f9060ff88166002811061131e5761131e618051565b6020020151604001525061144f9050565b825f0361133c575061144f565b5f61134684615910565b90505f61137c895f01518860ff166002811061136457611364618051565b6020020151602001518361592590919063ffffffff16565b90505f85126113ea578751819060ff89166002811061139d5761139d618051565b60200201516040018181516113b2919061809f565b9052508851819060ff8916600281106113cd576113cd618051565b602002015160a0018181516113e2919061809f565b90525061144b565b8751819060ff89166002811061140257611402618051565b602002015160400181815161141791906180b2565b9052508851819060ff89166002811061143257611432618051565b602002015160a00181815161144791906180b2565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f6114758484615960565b915091508181106114965760608301516114909060016159c1565b60608401525b50505050565b5f5f5f5f861180156114ac575083155b156114bb5750839050846114d3565b5f871180156114c8575084155b156110cb5750859050825b8115806114de575080155b156114fc576040516301a2868b60e31b815260040160405180910390fd5b6115286040518060400160405280600a815260200169616d6f756e744261736560b01b815250836159eb565b6115546040518060400160405280600a815260200169616d6f756e744d656d6560b01b815250826159eb565b5f61156389606001515f615a14565b90505f6115758a606001516001615a14565b90505f61159885676765c793fa10079d601b1b61159386600a6181a0565b615a42565b90505f6115b685676765c793fa10079d601b1b61159386600a6181a0565b90506115ec6040518060400160405280601281526020017118985cd9505b5bdd5b9d10591a9d5cdd195960721b815250836159eb565b611620604051806040016040528060128152602001711b595b59505b5bdd5b9d10591a9d5cdd195960721b815250826159eb565b604080518082019091526005815264707269636560d81b602082015261164f9061164a8484615925565b6159eb565b6116598282615925565b9c9b505050505050505050505050565b805f0361167557505050565b61169660405180606001604052806025815260200161831860259139610dbe565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101919091526116f08484615b02565b63ffffffff168152611707640100000000426181bf565b63ffffffff90811660208301528151165f036117bd5761172c84848360200151615b78565b61173784845f615bfc565b61174684848360200151615c3b565b61175184845f615c53565b61175c848484615c6b565b6117966040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff166159eb565b61149660405180604001604052806005815260200164707269636560d81b815250836159eb565b805160208201516117ce91906181d2565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b6020820152905161180a92166159eb565b604081015163ffffffff1615611496576118248484615c83565b6060820152604081015161183e9063ffffffff16836181ee565b816060015161184d919061809f565b6080820152604080518082019091526005815264707269636560d81b602082015261187890836159eb565b6118af6040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff166159eb565b6118e8604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b81525082606001516159eb565b61191d6040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082608001516159eb565b61192c84848360800151615bfc565b61193b84848360200151615b78565b6119458484615c9c565b63ffffffff1660a08201819052602082015161196191906181d2565b63ffffffff1660c082015261197584615cb5565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b602082015290516119ad92166159eb565b6119f66040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff166159eb565b611a35604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff166159eb565b8060e0015163ffffffff168160c0015163ffffffff16111561149657611a5b8484615d67565b610100820181905260c0820151608083015163ffffffff90911691611a7f916180b2565b611a899190618205565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151611ad391906159eb565b611b086040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082608001516159eb565b611b3b6040518060400160405280600c81526020016b70726963654176657261676560a01b8152508261012001516159eb565b611b4a84848360200151615c3b565b611b5984848360800151615c53565b6114968484836101200151615c6b565b5f611b748584618079565b90505f811315611b9e57865160200151611b999083611b9284615910565b6001615d80565b611bb9565b865160200151611bb99083611bb284615910565b6001615e9e565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611c07906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611c57575f5ffd5b505af1158015611c69573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62611c898460400151615fb1565b856040518363ffffffff1660e01b8152600401611cb0929190918252602082015260400190565b5f604051808303815f87803b158015611cc7575f5ffd5b505af1158015611cd9573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611d17906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d47929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611d88929190918252602082015260400190565b6020604051808303815f875af1158015611da4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dc89190617fc2565b50806001600160a01b031663ca446dd984604051602001611e08906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611e38929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611e83926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015611e9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ec39190618218565b50806001600160a01b031663ca446dd984604051602001611f03906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611f33929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fba9190618218565b50806001600160a01b031663e2a4853a84604051602001611fff9060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161202f929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561208c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120b09190617fc2565b50806001600160a01b031663e2a4853a846040516020016120f59060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001612125929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612181573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121a59190617fc2565b50806001600160a01b031663e2a4853a846040516020016121f0906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001612220929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561227d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122a19190617fc2565b50806001600160a01b031663e2a4853a846040516020016122eb906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161231b929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612378573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061239c9190617fc2565b50806001600160a01b031663e2a4853a846040516020016123e8906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612418929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612475573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124999190617fc2565b50806001600160a01b031663e2a4853a846040516020016124e4906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001612514929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612571573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125959190617fc2565b50806001600160a01b031663e2a4853a846040516020016125d4906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612604929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612662573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126869190617fc2565b50806001600160a01b031663ca446dd9846040516020016126c6906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016126f6929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561275c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127809190618218565b50806001600160a01b031663e2a4853a846040516020016127c59060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016127f5929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612854573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128789190617fc2565b50806001600160a01b031663e2a4853a846040516020016128bd9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016128ed929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561294c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129709190617fc2565b50806001600160a01b031663e2a4853a846040516020016129bb90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016129eb929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612a4b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a6f9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612ab990602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ae9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612b49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b6d9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612bb990602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612be9929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612c49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c6d9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612cb890602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ce8929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612d48573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6c9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612dab906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ddb929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612e26929190918252602082015260400190565b6020604051808303815f875af1158015612e42573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061119d9190617fc2565b612e6f81616035565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001612eca906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612f1a575f5ffd5b505af1158015612f2c573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001612f70906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612fa0929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015613003573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130279190618218565b50806001600160a01b031663e2a4853a8460405160200161306f906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161309f929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156130fc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131209190617fc2565b50806001600160a01b031663e2a4853a84604051602001613167906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613197929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156131f3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132179190617fc2565b50806001600160a01b031663e2a4853a84604051602001613263906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613293929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156132f0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133149190617fc2565b50806001600160a01b031663e2a4853a8460405160200161333490618233565b60405160208183030381529060405280519060200120604051602001613364929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156133c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e59190617fc2565b50806001600160a01b031663e2a4853a84604051602001613432906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001613462929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156134bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134e39190617fc2565b50806001600160a01b031663e2a4853a8460405160200161352c906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b6040516020818303038152906040528051906020012060405160200161355c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156135b9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135dd9190617fc2565b50806001600160a01b031663ca446dd98460405160200161361e906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b6040516020818303038152906040528051906020012060405160200161364e929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156136b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136d89190618218565b50806001600160a01b031663e2a4853a8460405160200161372090602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001613750929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156137af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137d39190617fc2565b50806001600160a01b031663e2a4853a8460405160200161381a90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161384a929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156138a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138cd9190617fc2565b50806001600160a01b031663e2a4853a8460405160200161391990602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613949929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156139a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139cd9190617fc2565b50806001600160a01b031663e2a4853a846040516020016139ed90618274565b60405160208183030381529060405280519060200120604051602001613a1d929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613a7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613aa19190617fc2565b50806001600160a01b031663e2a4853a84604051602001613aee90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b1e929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613b7e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ba29190617fc2565b50806001600160a01b031663e2a4853a84604051602001613beb90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c1b929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613c7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c9f9190617fc2565b50806001600160a01b031663ca446dd984604051602001613cdd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613d0d929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401613d579291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613d73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d979190618218565b50806001600160a01b031663ca446dd984604051602001613de9906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613e19929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613e64926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613e80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ea49190618218565b50806001600160a01b031663e2a4853a84604051602001613eeb906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f1b929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613f5c929190918252602082015260400190565b6020604051808303815f875af1158015613f78573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f9c9190617fc2565b50806001600160a01b031663e2a4853a84604051602001613fee906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161401e929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612e26929190918252602082015260400190565b5f5f5f5f61409c60405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f881180156140a9575085155b156140e0578951602090810151516001600160a01b0390811683528b51515116908201526040810188905260608101879052614122565b5f891180156140ed575086155b1561412257895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b1580156141dd575f5ffd5b505af11580156141ef573d5f5f3e3d5ffd5b50505050505050505050505050565b6040516304e6bdd160e11b81526001600160a01b038a81166004830152602482018a9052888116604483015287811660648301526084820187905260a4820186905260c4820185905260e4820184905261010482018390528b16906309cd7ba290610124015f604051808303815f87803b15801561427a575f5ffd5b505af115801561428c573d5f5f3e3d5ffd5b5050505050505050505050505050565b61023782826040516024016142b29291906182b5565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052616244565b6142e9617d57565b5f5f6142f5868561624d565b90505f61430286836162b3565b9050610e3987826162c5565b5f816001600160a01b0316836001600160a01b03161061432f578183614332565b82825b604051919450925061435f906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b6143be617d7d565b826143c7617d7d565b816001600160a01b03166391d4403c604051602001614403906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614457573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061447b9190617fd9565b6144885791506111cd9050565b816001600160a01b03166321f8a721856040516020016144c8906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016144f8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161452c91815260200190565b602060405180830381865afa158015614547573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061456b9190618218565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016145e9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161461d91815260200190565b602060405180830381865afa158015614638573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061465c9190617fc2565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016146b2906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016146e2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161471691815260200190565b602060405180830381865afa158015614731573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147559190617fc2565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016147b0906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016147e0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161481491815260200190565b602060405180830381865afa15801561482f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148539190617fc2565b815151606001526040516001600160a01b0383169063bd02d0f590869061487c90602001618233565b604051602081830303815290604052805190602001206040516020016148ac929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148e091815260200190565b602060405180830381865afa1580156148fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491f9190617fc2565b81515f60200201516080018181525050816001600160a01b031663bd02d0f58560405160200161497b906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016149ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149df91815260200190565b602060405180830381865afa1580156149fa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a1e9190617fc2565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614a9b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614acf91815260200190565b602060405180830381865afa158015614aea573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b0e9190617fc2565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614b83929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bb791815260200190565b602060405180830381865afa158015614bd2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614bf69190618218565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ca091815260200190565b602060405180830381865afa158015614cbb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614cdf9190617fc2565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614d3690602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d66929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d9a91815260200190565b602060405180830381865afa158015614db5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614dd99190617fc2565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614e3590602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e65929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614e9991815260200190565b602060405180830381865afa158015614eb4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ed89190617fc2565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614f0890618274565b60405160208183030381529060405280519060200120604051602001614f38929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614f6c91815260200190565b602060405180830381865afa158015614f87573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fab9190617fc2565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161500890602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001615038929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161506c91815260200190565b602060405180830381865afa158015615087573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150ab9190617fc2565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161510490602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001615134929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161516891815260200190565b602060405180830381865afa158015615183573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151a79190617fc2565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016151f590602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001615225929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525991815260200190565b602060405180830381865afa158015615274573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152989190618218565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615306906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615336929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161536a91815260200190565b602060405180830381865afa158015615385573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153a99190618218565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161540c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b6040516020818303038152906040528051906020012060405160200161543c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161547091815260200190565b602060405180830381865afa15801561548b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154af9190617fc2565b60608201526040516001600160a01b0383169063bd02d0f5908690615508906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615538929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161556c91815260200190565b602060405180830381865afa158015615587573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155ab9190617fc2565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b608081015142908190036155f8575050565b81515160a001511561564e5781515f9061562190825b602002015160400151846080015161633f565b83519091506156459082905f5b60200201516020015161637390919063ffffffff16565b83515160200152505b81516020015160a001511561568a5781515f9061566c90600161560e565b835190915061567e908290600161562e565b83516020908101510152505b608090910152565b5f5f5f5f61569e617db1565b6156a7896163b4565b6101c082018190526156be9088905f908190616405565b5060408401525081526101c08101516156dd9088906001905f90616405565b506060840152506020820152805115806156f957506020810151155b15615710575f5f5f5f9450945094509450506157b3565b851561572c57878160200181815161572891906180b2565b9052505b805160208201516157429190611593818c6164ea565b6080820181905281516157549161653e565b60a0820152606087015160381c61ffff16610140820181905260a082015161578b91615783906127109061653e565b612710615a42565b6040820151606083015161014084015160a08501516157a9916158cf565b9450945094509450505b945094509450949050565b5f5f5f5f6157ca617db1565b6157d3896163b4565b6101c082018190526157ea9088905f908190616405565b5060408401525081526101c08101516158099088906001905f90616405565b5060608401525060208201528051158061582557506020810151155b1561583c575f5f5f5f9450945094509450506157b3565b85156158575787815f0181815161585391906180b2565b9052505b606087015160381c61ffff16610140820181905261587e908990615783906127109061653e565b61016082018190528151602083015161589c926115939083906164ea565b6080820181905260208201516158b19161653e565b60c08201819052604082015160608301516101408401516157a9908c905b5f811561138819839004841115176158e5575f5ffd5b506127109102611388010490565b81515160c001805182919061590990839061809f565b9052505050565b5f5f82121561592157815f036111cd565b5090565b5f8115676765c793fa10079d601b1b60028404190484111715615946575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f61597184606001515f615a14565b90505f61597d86616593565b90505f6159a08261598f85600a6181a0565b676765c793fa10079d601b1b615a42565b90505f6159af875f5f5f616405565b50939a91995090975050505050505050565b5f6033826159cf575f6159d2565b60015b60ff16901b660800000000000019841617905092915050565b610237604051806040016040528060068152602001652573202d257360d01b81525083836165e4565b5f60ff60581b1960585f1960ff851601615a34575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03615a7657838281615a6c57615a6c6181ab565b0492505050615afb565b808411615a965760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f826001600160a01b031663bd02d0f5615b1b8461662b565b6040518263ffffffff1660e01b8152600401615b3991815260200190565b602060405180830381865afa158015615b54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615afb9190617fc2565b826001600160a01b031663e2a4853a615b908461662b565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015615bd8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114969190617fc2565b826001600160a01b031663e2a4853a615c14846166de565b836040518363ffffffff1660e01b8152600401615bbc929190918252602082015260400190565b826001600160a01b031663e2a4853a615b9084616732565b826001600160a01b031663e2a4853a615c1484616793565b826001600160a01b031663e2a4853a615c14846167f9565b5f826001600160a01b031663bd02d0f5615b1b846166de565b5f826001600160a01b031663bd02d0f5615b1b84616732565b5f816001600160a01b031663bd02d0f5604051602001615cf4906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d2891815260200190565b602060405180830381865afa158015615d43573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111cd9190617fc2565b5f826001600160a01b031663bd02d0f5615b1b84616793565b60e084015160011901615da757600160e085015260a0840182905260608401839052611496565b60e08401515f1901615e195760a084018051908390615dc6828461809f565b9052508115615e13575f615dda8585616373565b6060870151615de99084616373565b615df3919061809f565b9050615e0c8660a001518261592590919063ffffffff16565b6060870152505b50611496565b60e084015161149657818460c001511115615e4857818460c001818151615e4091906180b2565b905250611496565b818460c0015103615e6c57600260e08501525f60c085018190526080850152611496565b600160e085015260c0840151615e8290836180b2565b60a0850152505060608201525f60c08201819052608090910152565b60e084015160011901615ec4575f60e085015260c0840182905260808401839052611496565b60e0840151615f325760c084018051908390615ee0828461809f565b9052508115615e13575f615ef48585616373565b6080870151615f039084616373565b615f0d919061809f565b9050615f268660c001518261592590919063ffffffff16565b60808701525050611496565b60e08401515f190161149657818460a001511115615f5c57818460a001818151615e4091906180b2565b818460a0015103615f8057600260e08501525f60a085018190526060850152611496565b5f60e085015260a0840151615f9590836180b2565b60c0850152505060808201525f60a08201819052606090910152565b5f604051602001615feb906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61606a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b616074845f616842565b60208301528152606084015161608a905f615a14565b6060820181905281516160af91676765c793fa10079d601b1b9061159390600a6181a0565b604082015260208101515f036160ca575f608082015261616a565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015616140573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906161649190617fc2565b60808201525b616175846001616842565b602083018190529082525f03616190575f60a0820152616230565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015616206573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061622a9190617fc2565b60a08201525b80608001518160a001519250925050915091565b61099481616888565b5f60405160200161627a906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0385169082015260608101839052608001614398565b6162bb617d57565b615afb83836168a7565b60408101516001600160a01b03166162f057604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f8061634b83426180b2565b61635590856181ee565b6301e13380900490506111c981676765c793fa10079d601b1b61809f565b5f81156b019d971e4fe8401e740000001983900484111517616393575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f816001600160a01b031663bd02d0f5604051602001615cf4906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff166002811061642257616422618051565b602002015190505f6164348a8a617ab9565b9050805f03616450575f5f5f5f955095509550955050506157b3565b5f61645f838c60800151617ba7565b90505f61646c828a616373565b90505f89156164915781841061648b57616486848361653e565b616493565b5f616493565b5f5b90505f6164a0858d616373565b90505f8c156164c5578482106164bf576164ba828661653e565b6164c7565b5f6164c7565b5f5b90506164d3858761809f565b9f959e50919c50909a509298505050505050505050565b5f826164f6838261809f565b91508110156111cd5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f8261654a83826180b2565b91508111156111cd5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f816001600160a01b031663bd02d0f5604051602001615cf49060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b6108d98383836040516024016165fc939291906182e2565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052616244565b80515f90819061664c90825b6020020151518451600160200201515161430e565b905080604051602001616690906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016166c0929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906166ee9082616637565b905080604051602001616690906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f9081906167429082616637565b9050806040516020016166909060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906167a39082616637565b9050806040516020016166909060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906168099082616637565b90508060405160200161669090602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b5f5f5f61686f855f01518560ff166002811061686057616860618051565b60200201518660800151617ba7565b90505f61687c8686617ab9565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b6168af617d57565b826168b8617d57565b816001600160a01b03166391d4403c6040516020016168f8906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561694c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906169709190617fd9565b61697d5791506111cd9050565b816001600160a01b031663bd02d0f5856040516020016169b7906020808252600690820152651413d4d7d25160d21b604082015260600190565b604051602081830303815290604052805190602001206040516020016169e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616a1b91815260200190565b602060405180830381865afa158015616a36573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616a5a9190617fc2565b816020018181525050816001600160a01b03166321f8a72185604051602001616aa2906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616ad2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616b0691815260200190565b602060405180830381865afa158015616b21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616b459190618218565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001616ba1906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616bd1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616c0591815260200190565b602060405180830381865afa158015616c20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616c449190618218565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616cbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616cf391815260200190565b602060405180830381865afa158015616d0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616d329190617fc2565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001616d869060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001616db6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616dea91815260200190565b602060405180830381865afa158015616e05573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616e299190617fc2565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001616e83906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616eb3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616ee791815260200190565b602060405180830381865afa158015616f02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616f269190617fc2565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001616f7f906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616faf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616fe391815260200190565b602060405180830381865afa158015616ffe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906170229190617fc2565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016170a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016170d691815260200190565b602060405180830381865afa1580156170f1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906171159190617fc2565b81515f60200201516080018181525050816001600160a01b031663bd02d0f58560405160200161716f906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161719f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016171d391815260200190565b602060405180830381865afa1580156171ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906172129190617fc2565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001617285929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016172b991815260200190565b602060405180830381865afa1580156172d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906172f89190617fc2565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a7219086906080016040516020818303038152906040528051906020012060405160200161736c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016173a091815260200190565b602060405180830381865afa1580156173bb573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906173df9190618218565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161748691815260200190565b602060405180830381865afa1580156174a1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906174c59190617fc2565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161751a9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161754a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161757e91815260200190565b602060405180830381865afa158015617599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906175bd9190617fc2565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161761890602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001617648929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161767c91815260200190565b602060405180830381865afa158015617697573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906176bb9190617fc2565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161771590602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001617745929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161777991815260200190565b602060405180830381865afa158015617794573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906177b89190617fc2565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161781490602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001617844929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161787891815260200190565b602060405180830381865afa158015617893573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906178b79190617fc2565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161791290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001617942929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161797691815260200190565b602060405180830381865afa158015617991573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906179b59190617fc2565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001617a04906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001617a34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401617a6891815260200190565b602060405180830381865afa158015617a83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617aa79190617fc2565b81516020015160e00152949350505050565b5f5f835f01518360ff1660028110617ad357617ad3618051565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015617b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617b509190617fc2565b9050805f03617b63575f925050506111cd565b606082015160c0830151617b77908261809f565b8210617b9b5760c0830151617b8c82846180b2565b617b9691906180b2565b617b9d565b5f5b9695505050505050565b5f8260a001515f03617bba57505f6111cd565b5f617bc58484617bd7565b60a08501519091506111c99082616373565b5f428203617bea575060208201516111cd565b5f617bf984604001518461633f565b9050617c1284602001518261637390919063ffffffff16565b9150506111cd565b604051806102800160405280617c2e617d7d565b81526020015f8152602001617c41617d57565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806102000160405280617cdc617d7d565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280617d6a617e18565b81525f6020820181905260409091015290565b6040518060a00160405280617d90617e86565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b617e706040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617e275790505090565b60405180604001604052806002905b617ed76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617e955790505090565b6001600160a01b0381168114610994575f5ffd5b5f5f82840360c0811215617f13575f5ffd5b8335617f1e81617eed565b925060a0601f1982011215617f31575f5ffd5b506020830190509250929050565b5f5f828403610100811215617f52575f5ffd5b8335617f5d81617eed565b925060e0601f1982011215617f31575f5ffd5b5f60208284031215617f80575f5ffd5b8135615afb81617eed565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215617fd2575f5ffd5b5051919050565b5f60208284031215617fe9575f5ffd5b81518015158114615afb575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061804990830184617ff8565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f83128015838313168383128216171561809857618098618065565b5092915050565b808201808211156111cd576111cd618065565b818103818111156111cd576111cd618065565b6001815b6001841115610dfb578085048111156180e4576180e4618065565b60018416156180f257908102905b60019390931c9280026180c9565b5f8261810e575060016111cd565b8161811a57505f6111cd565b8160018114618130576002811461813a57618156565b60019150506111cd565b60ff84111561814b5761814b618065565b50506001821b6111cd565b5060208310610133831016604e8410600b8410161715618179575081810a6111cd565b6181855f1984846180c5565b805f190482111561819857618198618065565b029392505050565b5f615afb8383618100565b634e487b7160e01b5f52601260045260245ffd5b5f826181cd576181cd6181ab565b500690565b63ffffffff82811682821603908111156111cd576111cd618065565b80820281158282048414176111cd576111cd618065565b5f82618213576182136181ab565b500490565b5f60208284031215618228575f5ffd5b8151615afb81617eed565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6182c76040830185617ff8565b82810360208401526182d98185617ff8565b95945050505050565b606081525f6182f46060830186617ff8565b82810360208401526183068186617ff8565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da264697066735822122025b53b816a0eb226e84738b84654305ba0eacfe0b06ca9a86d3a051390cc3ca664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x84}8\x03\x80a\x84}\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x83ra\x01\x0B_9_\x81\x81`\xDD\x01R\x81\x81a\x01\xC6\x01Ra\x02\xB2\x01R_\x81\x81`^\x01Ra\x05L\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\x83\x01R\x81\x81a\x03u\x01R\x81\x81a\x04{\x01Ra\x08\xE0\x01Ra\x83r_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04a\x7F\x01V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04a\x7F?V[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08\xDEV[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x97V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90a\x7F\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a\x7F\xC2V[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90a\x7F\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90a\x7F\xC2V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90a\x7F\xD9V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90a\x80&V[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\xBEV[a\x06\x1Ca|\x1AV[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r\xE2V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\x0E\x03V[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x86Q\x92\x87\x01Q\x90\x87\x01Qa\x06\x8F\x95\x93\x94\x92\x91\x90_a\x0EDV[a\x02@\x86\x01Ra\x01\xA0\x85\x01Ra\x01\x80\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCE\x94\x92\x93\x91\x92\x90\x91a\x11\x17V[a\x06\xE0\x81_\x01Q\x82a\x02@\x01Qa\x11\xA4V[a\x02`\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x10\x93\x92\x91_\x91a\x07\t\x91\x90a\x80yV[__a\x11\xD3V[a\x073\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\t\x91\x90a\x80yV[`\xA0\x81\x01Q\x15a\x07KW\x81Q\x81Qa\x07K\x91\x90a\x14WV[a\x07l\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa\x14\x9CV[a\x01`\x82\x01\x81\x90R\x82Q\x82Qa\x07\x81\x92a\x16iV[a\x07\xAD\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87a\x01`\x01Qa\x1BiV[a\x07\xC3\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x1B\xC3V[a\x07\xD4\x82` \x01Q\x82_\x01Qa.fV[\x81Q` \x82\x01Q\x82Qa\x07\xE8\x92\x91\x90a.\x8AV[a\x08\t\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa@_V[a\x02 \x85\x01\x81\x90Ra\x02\0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xE0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xC0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02`\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08\x99\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91aAGV[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80Q\x80Q\x91\x85\x01Q\x80Q\x84\x89\x01Q\x83\x88\x01Q\x93\x86\x01Q\x97\x83\x01Q\x92\x90\x95\x01Qa\x08\xD9\x97\x8B\x96`\x04\x96\x95\x93\x94\x90\x93\x92aA\xFEV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\t\x1C\x90a\x7F\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90a\x7F\xC2V[PV[a\t\xC3`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\xBEV[a\t\xCBa|\xC8V[a\t\xE1\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\x0E\x03V[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a\x7F\xC2V[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDE\x91\x90a\x7F\xC2V[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\n\xF7WP`\x80\x81\x01Q\x15[\x15a\x0B\x15W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x11\x15a\x0B0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x11\x15a\x0BKW`\xA0\x82\x01Q`\x80\x82\x01R[a\x0Bq\x82_\x01Q\x82_\x01Q\x83``\x01Q\x84`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`\x01a\x0EDV[a\x01\xC0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B\xA5\x91a\x11\xA4V[a\x01\xE0\x82\x01R`\xA0\x81\x01Q\x15a\x0C/W`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x18W__\xFD[PZ\xF1\x15\x80\x15a\x0C*W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C\xB4W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xAFW=__>=_\xFD[PPPP[a\x0C\xC5\x82` \x01Q\x82_\x01Qa.fV[``\x81\x01Q\x15a\x0C\xDDW\x81Q\x81Qa\x0C\xDD\x91\x90a\x14WV[\x81Q` \x82\x01Q\x82Qa\x0C\xF1\x92\x91\x90a.\x8AV[a\r\x11\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa@_V[a\x01\xA0\x85\x01Ra\x01\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x84\x01R\x16a\x01@\x82\x01R\x80Q``\x82\x01Q`\x80\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\rX\x94\x93\x92\x91\x90a\x14\x9CV[a\x01 \x82\x01\x81\x90R\x82Q\x82Qa\rm\x92a\x16iV[a\x08\xD9\x82` \x01Q\x84\x83a\x01@\x01Q\x84a\x01`\x01Qbz\x12\0\x86a\x01\x80\x01Q\x87a\x01\xA0\x01Q\x88a\x01\xE0\x01Q`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPaAGV[a\t\x94`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aB\x9CV[a\r\xEAa}WV[_a\r\xF6\x85\x85\x85aB\xE1V[\x91P\x91P[\x93P\x93\x91PPV[a\x0E\x0Ba}}V[__a\x0E\x17\x85\x85aC\x0EV[\x90P_a\x0E$\x87\x83aC\xB6V[\x90Pa\x0E0\x81\x83aU\xB8V[a\x0E9\x81aU\xE6V[\x96\x90\x95P\x93PPPPV[_____a\x0E\xA1`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8B\x11\x80\x15a\x0E\xAFWP_\x8A\x11[\x15a\x0E\xCDW`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x8A\x11\x80\x15a\x0E\xDAWP\x87\x15[\x15a\x0F\xD6Wa\x0E\xEB\x8D\x8B\x8E\x8AaV\x92V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R_\x03a\x0FLW`@Qc\x17\xA6\xEFc`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q\x89\x11\x15a\x0F{W\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0F\x92\x91\x90a\x80\x9FV[\x11\x15a\x0F\xD1Wa\x01\0\x81\x01Q\x81Qa\x0F\xAA\x91\x90a\x80\x9FV[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x10\xE4V[_\x8B\x11\x80\x15a\x0F\xE3WP\x88\x15[\x15a\x10\xCBWa\x0F\xF4\x8D\x8C\x8E\x8AaW\xBEV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R_\x03a\x10ZW`@Qc\x17\xA6\xEFc`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87\x81` \x01Q\x10\x15a\x10\x8FW` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0F\xD1W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84a\x01\0\x01Q\x95P\x95P\x95P\x95P\x95PP\x97P\x97P\x97P\x97P\x97\x92PPPV[_\x83\x11\x80\x15a\x11$WP\x80\x15[\x15a\x11sW\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x11nW\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x11\x9DV[_\x84\x11\x80\x15a\x11\x80WP\x81\x15[\x15a\x11\x9DW\x84QQ` \x01Q\x84\x11\x15a\x11\x9DW\x84Q\x84\x90_a\x11AV[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x11\xBD\x84\x83aX\xCFV[\x90Pa\x11\xC9\x85\x82aX\xF3V[\x91PP[\x92\x91PPV[_a\x11\xDD\x84aY\x10V[\x90P_\x84\x12a\x12KW\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xFEWa\x11\xFEa\x80QV[` \x02\x01Q` \x01\x81\x81Qa\x12\x13\x91\x90a\x80\x9FV[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12.Wa\x12.a\x80QV[` \x02\x01Q``\x01\x81\x81Qa\x12C\x91\x90a\x80\x9FV[\x90RPa\x12\xACV[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12cWa\x12ca\x80QV[` \x02\x01Q` \x01\x81\x81Qa\x12x\x91\x90a\x80\xB2V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12\x93Wa\x12\x93a\x80QV[` \x02\x01Q``\x01\x81\x81Qa\x12\xA8\x91\x90a\x80\xB2V[\x90RP[\x81\x15a\x13/W\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x12\xCAWa\x12\xCAa\x80QV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x12\xEEWa\x12\xEEa\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\x03\x91\x90a\x80\xB2V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x13\x1EWa\x13\x1Ea\x80QV[` \x02\x01Q`@\x01RPa\x14O\x90PV[\x82_\x03a\x13<WPa\x14OV[_a\x13F\x84aY\x10V[\x90P_a\x13|\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x13dWa\x13da\x80QV[` \x02\x01Q` \x01Q\x83aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x13\xEAW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\x9DWa\x13\x9Da\x80QV[` \x02\x01Q`@\x01\x81\x81Qa\x13\xB2\x91\x90a\x80\x9FV[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\xCDWa\x13\xCDa\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\xE2\x91\x90a\x80\x9FV[\x90RPa\x14KV[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x14\x02Wa\x14\x02a\x80QV[` \x02\x01Q`@\x01\x81\x81Qa\x14\x17\x91\x90a\x80\xB2V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x142Wa\x142a\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x14G\x91\x90a\x80\xB2V[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x14u\x84\x84aY`V[\x91P\x91P\x81\x81\x10a\x14\x96W``\x83\x01Qa\x14\x90\x90`\x01aY\xC1V[``\x84\x01R[PPPPV[____\x86\x11\x80\x15a\x14\xACWP\x83\x15[\x15a\x14\xBBWP\x83\x90P\x84a\x14\xD3V[_\x87\x11\x80\x15a\x14\xC8WP\x84\x15[\x15a\x10\xCBWP\x85\x90P\x82[\x81\x15\x80a\x14\xDEWP\x80\x15[\x15a\x14\xFCW`@Qc\x01\xA2\x86\x8B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15(`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountBase`\xB0\x1B\x81RP\x83aY\xEBV[a\x15T`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountMeme`\xB0\x1B\x81RP\x82aY\xEBV[_a\x15c\x89``\x01Q_aZ\x14V[\x90P_a\x15u\x8A``\x01Q`\x01aZ\x14V[\x90P_a\x15\x98\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x15\x93\x86`\na\x81\xA0V[aZBV[\x90P_a\x15\xB6\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x15\x93\x86`\na\x81\xA0V[\x90Pa\x15\xEC`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\x98\\\xD9P[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x83aY\xEBV[a\x16 `@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x1BY[YP[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x82aY\xEBV[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x16O\x90a\x16J\x84\x84aY%V[aY\xEBV[a\x16Y\x82\x82aY%V[\x9C\x9BPPPPPPPPPPPPV[\x80_\x03a\x16uWPPPV[a\x16\x96`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x83\x18`%\x919a\r\xBEV[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x16\xF0\x84\x84a[\x02V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x17\x07d\x01\0\0\0\0Ba\x81\xBFV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x17\xBDWa\x17,\x84\x84\x83` \x01Qa[xV[a\x177\x84\x84_a[\xFCV[a\x17F\x84\x84\x83` \x01Qa\\;V[a\x17Q\x84\x84_a\\SV[a\x17\\\x84\x84\x84a\\kV[a\x17\x96`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x14\x96`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83aY\xEBV[\x80Q` \x82\x01Qa\x17\xCE\x91\x90a\x81\xD2V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x18\n\x92\x16aY\xEBV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x14\x96Wa\x18$\x84\x84a\\\x83V[``\x82\x01R`@\x81\x01Qa\x18>\x90c\xFF\xFF\xFF\xFF\x16\x83a\x81\xEEV[\x81``\x01Qa\x18M\x91\x90a\x80\x9FV[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x18x\x90\x83aY\xEBV[a\x18\xAF`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x18\xE8`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01QaY\xEBV[a\x19\x1D`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaY\xEBV[a\x19,\x84\x84\x83`\x80\x01Qa[\xFCV[a\x19;\x84\x84\x83` \x01Qa[xV[a\x19E\x84\x84a\\\x9CV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x19a\x91\x90a\x81\xD2V[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x19u\x84a\\\xB5V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x19\xAD\x92\x16aY\xEBV[a\x19\xF6`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x1A5`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\x96Wa\x1A[\x84\x84a]gV[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x1A\x7F\x91a\x80\xB2V[a\x1A\x89\x91\x90a\x82\x05V[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x1A\xD3\x91\x90aY\xEBV[a\x1B\x08`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaY\xEBV[a\x1B;`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01QaY\xEBV[a\x1BJ\x84\x84\x83` \x01Qa\\;V[a\x1BY\x84\x84\x83`\x80\x01Qa\\SV[a\x14\x96\x84\x84\x83a\x01 \x01Qa\\kV[_a\x1Bt\x85\x84a\x80yV[\x90P_\x81\x13\x15a\x1B\x9EW\x86Q` \x01Qa\x1B\x99\x90\x83a\x1B\x92\x84aY\x10V[`\x01a]\x80V[a\x1B\xB9V[\x86Q` \x01Qa\x1B\xB9\x90\x83a\x1B\xB2\x84aY\x10V[`\x01a^\x9EV[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1C\x07\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1CWW__\xFD[PZ\xF1\x15\x80\x15a\x1CiW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x1C\x89\x84`@\x01Qa_\xB1V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xB0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xD9W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x17\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1DG\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x88\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC8\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1E\x08\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x1E\x83\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xC3\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1F\x03\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBA\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\xFF\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a /\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB0\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \xF5\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xA5\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\xF0\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\" \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xA1\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x9C\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\xE8\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$uW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$\xE4\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\x14\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x95\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%\xD4\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x86\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a&\xC6\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x80\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'\xC5\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(x\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(\xBD\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)p\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)\xBB\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*o\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*\xB9\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+m\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+\xB9\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,m\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,\xB8\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-HW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-l\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-\xAB\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\xDB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x9D\x91\x90a\x7F\xC2V[a.o\x81a`5V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a.\xCA\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x1AW__\xFD[PZ\xF1\x15\x80\x15a/,W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a/p\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0'\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0o\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1 \x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1g\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x97\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x17\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2c\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\x93\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x14\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a34\x90a\x823V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE5\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a42\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xE3\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a5,\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xDD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a6\x1E\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xD8\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7 \x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xD3\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a8\x1A\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8J\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xCD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9\x19\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9I\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xCD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9\xED\x90a\x82tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a:}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xA1\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a:\xEE\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\x1E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xA2\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a;\xEB\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a<{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x9F\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a<\xDD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=W\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=sW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\x97\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a=\xE9\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra>d\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a>\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xA4\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a>\xEB\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a?xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x9C\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a?\xEE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\x1E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a@\x9C`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a@\xA9WP\x85\x15[\x15a@\xE0W\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90RaA\"V[_\x89\x11\x80\x15a@\xEDWP\x86\x15[\x15aA\"W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aA\xDDW__\xFD[PZ\xF1\x15\x80\x15aA\xEFW=__>=_\xFD[PPPPPPPPPPPPPV[`@Qc\x04\xE6\xBD\xD1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x88\x81\x16`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R`\xE4\x82\x01\x84\x90Ra\x01\x04\x82\x01\x83\x90R\x8B\x16\x90c\t\xCD{\xA2\x90a\x01$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aBzW__\xFD[PZ\xF1\x15\x80\x15aB\x8CW=__>=_\xFD[PPPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01aB\xB2\x92\x91\x90a\x82\xB5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RabDV[aB\xE9a}WV[__aB\xF5\x86\x85abMV[\x90P_aC\x02\x86\x83ab\xB3V[\x90Pa\x0E9\x87\x82ab\xC5V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aC/W\x81\x83aC2V[\x82\x82[`@Q\x91\x94P\x92PaC_\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[aC\xBEa}}V[\x82aC\xC7a}}V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aD\x03\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDWW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD{\x91\x90a\x7F\xD9V[aD\x88W\x91Pa\x11\xCD\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aD\xC8\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE,\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aEGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEk\x91\x90a\x82\x18V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\\\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xB2\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGU\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\xB0\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xE0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\x14\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHS\x91\x90a\x7F\xC2V[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aH|\x90` \x01a\x823V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xE0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x1F\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI{\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x1E\x91\x90a\x7F\xC2V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\x9B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xCF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x0E\x91\x90a\x7F\xC2V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\xF6\x91\x90a\x82\x18V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xA0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xDF\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM6\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aMf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xD9\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN5\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNe\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xD8\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aO\x08\x90a\x82tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aOl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xAB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aP\x08\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aPl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xAB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQ\x04\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQh\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xA7\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aQ\xF5\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aRY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRtW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x98\x91\x90a\x82\x18V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aS\x06\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xA9\x91\x90a\x82\x18V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\x0C\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTp\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xAF\x91\x90a\x7F\xC2V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aU\x08\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xAB\x91\x90a\x7F\xC2V[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aU\xF8WPPV[\x81QQ`\xA0\x01Q\x15aVNW\x81Q_\x90aV!\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01Qac?V[\x83Q\x90\x91PaVE\x90\x82\x90_[` \x02\x01Q` \x01Qacs\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aV\x8AW\x81Q_\x90aVl\x90`\x01aV\x0EV[\x83Q\x90\x91PaV~\x90\x82\x90`\x01aV.V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aV\x9Ea}\xB1V[aV\xA7\x89ac\xB4V[a\x01\xC0\x82\x01\x81\x90RaV\xBE\x90\x88\x90_\x90\x81\x90ad\x05V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaV\xDD\x90\x88\x90`\x01\x90_\x90ad\x05V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aV\xF9WP` \x81\x01Q\x15[\x15aW\x10W____\x94P\x94P\x94P\x94PPaW\xB3V[\x85\x15aW,W\x87\x81` \x01\x81\x81QaW(\x91\x90a\x80\xB2V[\x90RP[\x80Q` \x82\x01QaWB\x91\x90a\x15\x93\x81\x8Cad\xEAV[`\x80\x82\x01\x81\x90R\x81QaWT\x91ae>V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01QaW\x8B\x91aW\x83\x90a'\x10\x90ae>V[a'\x10aZBV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01QaW\xA9\x91aX\xCFV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____aW\xCAa}\xB1V[aW\xD3\x89ac\xB4V[a\x01\xC0\x82\x01\x81\x90RaW\xEA\x90\x88\x90_\x90\x81\x90ad\x05V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaX\t\x90\x88\x90`\x01\x90_\x90ad\x05V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aX%WP` \x81\x01Q\x15[\x15aX<W____\x94P\x94P\x94P\x94PPaW\xB3V[\x85\x15aXWW\x87\x81_\x01\x81\x81QaXS\x91\x90a\x80\xB2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90RaX~\x90\x89\x90aW\x83\x90a'\x10\x90ae>V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01QaX\x9C\x92a\x15\x93\x90\x83\x90ad\xEAV[`\x80\x82\x01\x81\x90R` \x82\x01QaX\xB1\x91ae>V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01QaW\xA9\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aX\xE5W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aY\t\x90\x83\x90a\x80\x9FV[\x90RPPPV[__\x82\x12\x15aY!W\x81_\x03a\x11\xCDV[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aYFW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aYq\x84``\x01Q_aZ\x14V[\x90P_aY}\x86ae\x93V[\x90P_aY\xA0\x82aY\x8F\x85`\na\x81\xA0V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaZBV[\x90P_aY\xAF\x87___ad\x05V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aY\xCFW_aY\xD2V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[a\x027`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83ae\xE4V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aZ4WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aZvW\x83\x82\x81aZlWaZla\x81\xABV[\x04\x92PPPaZ\xFBV[\x80\x84\x11aZ\x96W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84af+V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\xFB\x91\x90a\x7F\xC2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a[\x90\x84af+V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a[\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x96\x91\x90a\x7F\xC2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84af\xDEV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a[\x90\x84ag2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84ag\x93V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84ag\xF9V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84af\xDEV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84ag2V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a](\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xCD\x91\x90a\x7F\xC2V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84ag\x93V[`\xE0\x84\x01Q`\x01\x19\x01a]\xA7W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x14\x96V[`\xE0\x84\x01Q_\x19\x01a^\x19W`\xA0\x84\x01\x80Q\x90\x83\x90a]\xC6\x82\x84a\x80\x9FV[\x90RP\x81\x15a^\x13W_a]\xDA\x85\x85acsV[``\x87\x01Qa]\xE9\x90\x84acsV[a]\xF3\x91\x90a\x80\x9FV[\x90Pa^\x0C\x86`\xA0\x01Q\x82aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x14\x96V[`\xE0\x84\x01Qa\x14\x96W\x81\x84`\xC0\x01Q\x11\x15a^HW\x81\x84`\xC0\x01\x81\x81Qa^@\x91\x90a\x80\xB2V[\x90RPa\x14\x96V[\x81\x84`\xC0\x01Q\x03a^lW`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x14\x96V[`\x01`\xE0\x85\x01R`\xC0\x84\x01Qa^\x82\x90\x83a\x80\xB2V[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01a^\xC4W_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x14\x96V[`\xE0\x84\x01Qa_2W`\xC0\x84\x01\x80Q\x90\x83\x90a^\xE0\x82\x84a\x80\x9FV[\x90RP\x81\x15a^\x13W_a^\xF4\x85\x85acsV[`\x80\x87\x01Qa_\x03\x90\x84acsV[a_\r\x91\x90a\x80\x9FV[\x90Pa_&\x86`\xC0\x01Q\x82aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x14\x96V[`\xE0\x84\x01Q_\x19\x01a\x14\x96W\x81\x84`\xA0\x01Q\x11\x15a_\\W\x81\x84`\xA0\x01\x81\x81Qa^@\x91\x90a\x80\xB2V[\x81\x84`\xA0\x01Q\x03a_\x80W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x14\x96V[_`\xE0\x85\x01R`\xA0\x84\x01Qa_\x95\x90\x83a\x80\xB2V[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01a_\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__a`j`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a`t\x84_ahBV[` \x83\x01R\x81R``\x84\x01Qa`\x8A\x90_aZ\x14V[``\x82\x01\x81\x90R\x81Qa`\xAF\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x15\x93\x90`\na\x81\xA0V[`@\x82\x01R` \x81\x01Q_\x03a`\xCAW_`\x80\x82\x01RaajV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aa@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aad\x91\x90a\x7F\xC2V[`\x80\x82\x01R[aau\x84`\x01ahBV[` \x83\x01\x81\x90R\x90\x82R_\x03aa\x90W_`\xA0\x82\x01Rab0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ab\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ab*\x91\x90a\x7F\xC2V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[a\t\x94\x81ah\x88V[_`@Q` \x01abz\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01aC\x98V[ab\xBBa}WV[aZ\xFB\x83\x83ah\xA7V[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16ab\xF0W`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80acK\x83Ba\x80\xB2V[acU\x90\x85a\x81\xEEV[c\x01\xE13\x80\x90\x04\x90Pa\x11\xC9\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x80\x9FV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17ac\x93W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10ad\"Wad\"a\x80QV[` \x02\x01Q\x90P_ad4\x8A\x8Aaz\xB9V[\x90P\x80_\x03adPW____\x95P\x95P\x95P\x95PPPaW\xB3V[_ad_\x83\x8C`\x80\x01Qa{\xA7V[\x90P_adl\x82\x8AacsV[\x90P_\x89\x15ad\x91W\x81\x84\x10ad\x8BWad\x86\x84\x83ae>V[ad\x93V[_ad\x93V[_[\x90P_ad\xA0\x85\x8DacsV[\x90P_\x8C\x15ad\xC5W\x84\x82\x10ad\xBFWad\xBA\x82\x86ae>V[ad\xC7V[_ad\xC7V[_[\x90Pad\xD3\x85\x87a\x80\x9FV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x82ad\xF6\x83\x82a\x80\x9FV[\x91P\x81\x10\x15a\x11\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x82aeJ\x83\x82a\x80\xB2V[\x91P\x81\x11\x15a\x11\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[a\x08\xD9\x83\x83\x83`@Q`$\x01ae\xFC\x93\x92\x91\x90a\x82\xE2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RabDV[\x80Q_\x90\x81\x90afL\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQaC\x0EV[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01af\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90af\xEE\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90agB\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ag\xA3\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ah\t\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[___aho\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10ah`Wah`a\x80QV[` \x02\x01Q\x86`\x80\x01Qa{\xA7V[\x90P_ah|\x86\x86az\xB9V[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[ah\xAFa}WV[\x82ah\xB8a}WV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01ah\xF8\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aiLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aip\x91\x90a\x7F\xD9V[ai}W\x91Pa\x11\xCD\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ai\xB7\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ai\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ajZ\x91\x90a\x7F\xC2V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aj\xA2\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aj\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ak\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90akE\x91\x90a\x82\x18V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01ak\xA1\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ak\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90alD\x91\x90a\x82\x18V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am2\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01am\x86\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01am\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01am\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15an\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90an)\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01an\x83\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01an\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01an\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ao\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ao&\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ao\x7F\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ao\xAF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ao\xE3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ao\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ap\"\x91\x90a\x7F\xC2V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ap\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ap\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ap\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aq\x15\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aqo\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aq\xD3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aq\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ar\x12\x91\x90a\x7F\xC2V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ar\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ar\xB9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ar\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ar\xF8\x91\x90a\x7F\xC2V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01asl\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01as\xA0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15as\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90as\xDF\x91\x90a\x82\x18V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01at\x86\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15at\xA1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90at\xC5\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01au\x1A\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01auJ\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au~\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15au\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90au\xBD\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01av\x18\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01avH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01av|\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15av\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90av\xBB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aw\x15\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01awE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01awy\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aw\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aw\xB8\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ax\x14\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01axD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01axx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ax\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ax\xB7\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ay\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ayB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ayv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay\xB5\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01az\x04\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01az4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01azh\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15az\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90az\xA7\x91\x90a\x7F\xC2V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10az\xD3Waz\xD3a\x80QV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a{,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a{P\x91\x90a\x7F\xC2V[\x90P\x80_\x03a{cW_\x92PPPa\x11\xCDV[``\x82\x01Q`\xC0\x83\x01Qa{w\x90\x82a\x80\x9FV[\x82\x10a{\x9BW`\xC0\x83\x01Qa{\x8C\x82\x84a\x80\xB2V[a{\x96\x91\x90a\x80\xB2V[a{\x9DV[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a{\xBAWP_a\x11\xCDV[_a{\xC5\x84\x84a{\xD7V[`\xA0\x85\x01Q\x90\x91Pa\x11\xC9\x90\x82acsV[_B\x82\x03a{\xEAWP` \x82\x01Qa\x11\xCDV[_a{\xF9\x84`@\x01Q\x84ac?V[\x90Pa|\x12\x84` \x01Q\x82acs\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x11\xCDV[`@Q\x80a\x02\x80\x01`@R\x80a|.a}}V[\x81R` \x01_\x81R` \x01a|Aa}WV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x02\0\x01`@R\x80a|\xDCa}}V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a}ja~\x18V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80a}\x90a~\x86V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a~p`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a~'W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a~\xD7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a~\x95W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x94W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15a\x7F\x13W__\xFD[\x835a\x7F\x1E\x81a~\xEDV[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15a\x7F1W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15a\x7FRW__\xFD[\x835a\x7F]\x81a~\xEDV[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15a\x7F1W__\xFD[_` \x82\x84\x03\x12\x15a\x7F\x80W__\xFD[\x815aZ\xFB\x81a~\xEDV[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x7F\xD2W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x7F\xE9W__\xFD[\x81Q\x80\x15\x15\x81\x14aZ\xFBW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x80I\x90\x83\x01\x84a\x7F\xF8V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x80\x98Wa\x80\x98a\x80eV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[\x81\x81\x03\x81\x81\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[`\x01\x81[`\x01\x84\x11\x15a\r\xFBW\x80\x85\x04\x81\x11\x15a\x80\xE4Wa\x80\xE4a\x80eV[`\x01\x84\x16\x15a\x80\xF2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x80\xC9V[_\x82a\x81\x0EWP`\x01a\x11\xCDV[\x81a\x81\x1AWP_a\x11\xCDV[\x81`\x01\x81\x14a\x810W`\x02\x81\x14a\x81:Wa\x81VV[`\x01\x91PPa\x11\xCDV[`\xFF\x84\x11\x15a\x81KWa\x81Ka\x80eV[PP`\x01\x82\x1Ba\x11\xCDV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x81yWP\x81\x81\na\x11\xCDV[a\x81\x85_\x19\x84\x84a\x80\xC5V[\x80_\x19\x04\x82\x11\x15a\x81\x98Wa\x81\x98a\x80eV[\x02\x93\x92PPPV[_aZ\xFB\x83\x83a\x81\0V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x81\xCDWa\x81\xCDa\x81\xABV[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11\xCDWa\x11\xCDa\x80eV[_\x82a\x82\x13Wa\x82\x13a\x81\xABV[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x82(W__\xFD[\x81QaZ\xFB\x81a~\xEDV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a\x82\xC7`@\x83\x01\x85a\x7F\xF8V[\x82\x81\x03` \x84\x01Ra\x82\xD9\x81\x85a\x7F\xF8V[\x95\x94PPPPPV[``\x81R_a\x82\xF4``\x83\x01\x86a\x7F\xF8V[\x82\x81\x03` \x84\x01Ra\x83\x06\x81\x86a\x7F\xF8V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 %\xB5;\x81j\x0E\xB2&\xE8G8\xB8FT0[\xA0\xEA\xCF\xE0\xB0l\xA9\xA8m:\x05\x13\x90\xCC<\xA6dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa366004617f01565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617f3f565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b506102376108de565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f19190617f70565b6001600160a01b031681526020018360200160208101906103129190617f70565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b9190617f70565b6001600160a01b03169052905061022e8382610997565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b190617f8b565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104249190617fc2565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b790617f8b565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102379190617fc2565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd9190617fd9565b61023757338160405163a35b150b60e01b8152600401610470929190618026565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610dbe565b61061c617c1a565b61062e83835f01518460400151610de2565b606083015260408201819052825190518051516020919091015151610654929190610e03565b6020830152808252606083015160a0808401829052608085015160c08086018290528651928701519087015161068f9593949291905f610e44565b6102408601526101a0850152610180840152610100830181905260e08301829052604083015160a084015160c08501516106ce94929391929091611117565b6106e0815f01518261024001516111a4565b6102608201528051604082015160a083015160e08401516107109392915f916107099190618079565b5f5f6111d3565b610733815f0151826040015160018460c001518561010001516107099190618079565b60a08101511561074b578151815161074b9190611457565b61076c815f01518260a001518360c001518460e0015185610100015161149c565b61016082018190528251825161078192611669565b6107ad815f015182604001518360a001518460c001518560e00151866101000151876101600151611b69565b6107c3825f015182606001518360400151611bc3565b6107d48260200151825f0151612e66565b8151602082015182516107e8929190612e8a565b610809815f01518260a001518360c001518460e0015185610100015161405f565b610220850181905261020085018290526001600160a01b039283166101e08601819052939092166101c085018190526020868101516040888101516102608901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108999792968b9693949193929091614147565b60208281015160408381015151805180519185015180518489015183880151938601519783015192909501516108d9978b966004969593949093926141fe565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161091c90617f8b565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af1158015610970573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109949190617fc2565b50565b6109c36040518060400160405280600b81526020016a065786563757465537761760ac1b815250610dbe565b6109cb617cc8565b6109e1825f015183604001518460600151610e03565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af1158015610a3f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a639190617fc2565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610aba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ade9190617fc2565b60808201526060810151158015610af757506080810151155b15610b1557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511115610b3057608082015160608201525b8160a0015181608001511115610b4b5760a082015160808201525b610b71825f0151825f0151836060015184608001518660c001518760e001516001610e44565b6101c0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610ba5916111a4565b6101e082015260a081015115610c2f576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c18575f5ffd5b505af1158015610c2a573d5f5f3e3d5ffd5b505050505b60c081015115610cb457604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c9d575f5ffd5b505af1158015610caf573d5f5f3e3d5ffd5b505050505b610cc58260200151825f0151612e66565b606081015115610cdd5781518151610cdd9190611457565b815160208201518251610cf1929190612e8a565b610d11815f0151826060015183608001518460a001518560c0015161405f565b6101a08501526101808401526001600160a01b039081166101608401521661014082015280516060820151608083015160a084015160c0850151610d58949392919061149c565b610120820181905282518251610d6d92611669565b6108d9826020015184836101400151846101600151627a1200866101800151876101a00151886101e0015160405180608001604052805f81526020015f81526020015f81526020015f815250614147565b61099460405180604001604052806002815260200161257360f01b8152508261429c565b610dea617d57565b5f610df68585856142e1565b915091505b935093915050565b610e0b617d7d565b5f5f610e17858561430e565b90505f610e2487836143b6565b9050610e3081836155b8565b610e39816155e6565b969095509350505050565b5f5f5f5f5f610ea16040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8b118015610eaf57505f8a115b15610ecd57604051636331fab160e01b815260040160405180910390fd5b5f8a118015610eda575087155b15610fd657610eeb8d8b8e8a615692565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e082018190525f03610f4c576040516317a6ef6360e31b815260040160405180910390fd5b8051891115610f7b5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610f92919061809f565b1115610fd1576101008101518151610faa919061809f565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b6110e4565b5f8b118015610fe3575088155b156110cb57610ff48d8c8e8a6157be565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e082018190525f0361105a576040516317a6ef6360e31b815260040160405180910390fd5b878160200151101561108f57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610fd15760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805f0151816020015182604001518360600151846101000151955095509550955095505097509750975097509792505050565b5f83118015611124575080155b15611173578451602090810151015183111561116e578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b61119d565b5f84118015611180575081155b1561119d578451516020015184111561119d57845184905f611141565b5050505050565b60608201515f9060481c61ffff16816111bd84836158cf565b90506111c985826158f3565b9150505b92915050565b5f6111dd84615910565b90505f841261124b578551819060ff8716600281106111fe576111fe618051565b6020020151602001818151611213919061809f565b9052508651819060ff87166002811061122e5761122e618051565b6020020151606001818151611243919061809f565b9052506112ac565b8551819060ff87166002811061126357611263618051565b602002015160200181815161127891906180b2565b9052508651819060ff87166002811061129357611293618051565b60200201516060018181516112a891906180b2565b9052505b811561132f5785515f9060ff8716600281106112ca576112ca618051565b602002015160400151905080885f01518760ff16600281106112ee576112ee618051565b602002015160a00181815161130391906180b2565b90525086515f9060ff88166002811061131e5761131e618051565b6020020151604001525061144f9050565b825f0361133c575061144f565b5f61134684615910565b90505f61137c895f01518860ff166002811061136457611364618051565b6020020151602001518361592590919063ffffffff16565b90505f85126113ea578751819060ff89166002811061139d5761139d618051565b60200201516040018181516113b2919061809f565b9052508851819060ff8916600281106113cd576113cd618051565b602002015160a0018181516113e2919061809f565b90525061144b565b8751819060ff89166002811061140257611402618051565b602002015160400181815161141791906180b2565b9052508851819060ff89166002811061143257611432618051565b602002015160a00181815161144791906180b2565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f6114758484615960565b915091508181106114965760608301516114909060016159c1565b60608401525b50505050565b5f5f5f5f861180156114ac575083155b156114bb5750839050846114d3565b5f871180156114c8575084155b156110cb5750859050825b8115806114de575080155b156114fc576040516301a2868b60e31b815260040160405180910390fd5b6115286040518060400160405280600a815260200169616d6f756e744261736560b01b815250836159eb565b6115546040518060400160405280600a815260200169616d6f756e744d656d6560b01b815250826159eb565b5f61156389606001515f615a14565b90505f6115758a606001516001615a14565b90505f61159885676765c793fa10079d601b1b61159386600a6181a0565b615a42565b90505f6115b685676765c793fa10079d601b1b61159386600a6181a0565b90506115ec6040518060400160405280601281526020017118985cd9505b5bdd5b9d10591a9d5cdd195960721b815250836159eb565b611620604051806040016040528060128152602001711b595b59505b5bdd5b9d10591a9d5cdd195960721b815250826159eb565b604080518082019091526005815264707269636560d81b602082015261164f9061164a8484615925565b6159eb565b6116598282615925565b9c9b505050505050505050505050565b805f0361167557505050565b61169660405180606001604052806025815260200161831860259139610dbe565b60408051610140810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101919091526116f08484615b02565b63ffffffff168152611707640100000000426181bf565b63ffffffff90811660208301528151165f036117bd5761172c84848360200151615b78565b61173784845f615bfc565b61174684848360200151615c3b565b61175184845f615c53565b61175c848484615c6b565b6117966040518060400160405280600e81526020016d0626c6f636b54696d655374616d760941b815250826020015163ffffffff166159eb565b61149660405180604001604052806005815260200164707269636560d81b815250836159eb565b805160208201516117ce91906181d2565b63ffffffff90811660408084019182528051808201909152600b81526a1d1a5b59515b185c1cd95960aa1b6020820152905161180a92166159eb565b604081015163ffffffff1615611496576118248484615c83565b6060820152604081015161183e9063ffffffff16836181ee565b816060015161184d919061809f565b6080820152604080518082019091526005815264707269636560d81b602082015261187890836159eb565b6118af6040518060400160405280600b81526020016a1d1a5b59515b185c1cd95960aa1b815250826040015163ffffffff166159eb565b6118e8604051806040016040528060138152602001721c1c9a58d950dd5b5d5b185d1a5d9953185cdd606a1b81525082606001516159eb565b61191d6040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082608001516159eb565b61192c84848360800151615bfc565b61193b84848360200151615b78565b6119458484615c9c565b63ffffffff1660a08201819052602082015161196191906181d2565b63ffffffff1660c082015261197584615cb5565b63ffffffff90811660e083019081526040805180820190915260068152651411549253d160d21b602082015290516119ad92166159eb565b6119f66040518060400160405280601b81526020017f6c61737455706461746554696d657374616d704279506572696f6400000000008152508260a0015163ffffffff166159eb565b611a35604051806040016040528060138152602001721d1a5b59515b185c1cd959109e54195c9a5bd9606a1b8152508260c0015163ffffffff166159eb565b8060e0015163ffffffff168160c0015163ffffffff16111561149657611a5b8484615d67565b610100820181905260c0820151608083015163ffffffff90911691611a7f916180b2565b611a899190618205565b61012082015260408051808201909152601b81527f707269636543756d756c61746976654c6173744279506572696f6400000000006020820152610100820151611ad391906159eb565b611b086040518060400160405280600f81526020016e707269636543756d756c617469766560881b81525082608001516159eb565b611b3b6040518060400160405280600c81526020016b70726963654176657261676560a01b8152508261012001516159eb565b611b4a84848360200151615c3b565b611b5984848360800151615c53565b6114968484836101200151615c6b565b5f611b748584618079565b90505f811315611b9e57865160200151611b999083611b9284615910565b6001615d80565b611bb9565b865160200151611bb99083611bb284615910565b6001615e9e565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611c07906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611c57575f5ffd5b505af1158015611c69573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62611c898460400151615fb1565b856040518363ffffffff1660e01b8152600401611cb0929190918252602082015260400190565b5f604051808303815f87803b158015611cc7575f5ffd5b505af1158015611cd9573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611d17906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d47929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611d88929190918252602082015260400190565b6020604051808303815f875af1158015611da4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dc89190617fc2565b50806001600160a01b031663ca446dd984604051602001611e08906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611e38929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611e83926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015611e9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ec39190618218565b50806001600160a01b031663ca446dd984604051602001611f03906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611f33929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fba9190618218565b50806001600160a01b031663e2a4853a84604051602001611fff9060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161202f929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561208c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120b09190617fc2565b50806001600160a01b031663e2a4853a846040516020016120f59060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001612125929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612181573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121a59190617fc2565b50806001600160a01b031663e2a4853a846040516020016121f0906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001612220929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561227d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122a19190617fc2565b50806001600160a01b031663e2a4853a846040516020016122eb906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161231b929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612378573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061239c9190617fc2565b50806001600160a01b031663e2a4853a846040516020016123e8906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612418929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612475573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124999190617fc2565b50806001600160a01b031663e2a4853a846040516020016124e4906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001612514929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612571573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125959190617fc2565b50806001600160a01b031663e2a4853a846040516020016125d4906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612604929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612662573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126869190617fc2565b50806001600160a01b031663ca446dd9846040516020016126c6906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016126f6929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561275c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127809190618218565b50806001600160a01b031663e2a4853a846040516020016127c59060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016127f5929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612854573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128789190617fc2565b50806001600160a01b031663e2a4853a846040516020016128bd9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016128ed929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561294c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129709190617fc2565b50806001600160a01b031663e2a4853a846040516020016129bb90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016129eb929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612a4b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a6f9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612ab990602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ae9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612b49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b6d9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612bb990602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612be9929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612c49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c6d9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612cb890602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ce8929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612d48573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6c9190617fc2565b50806001600160a01b031663e2a4853a84604051602001612dab906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612ddb929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612e26929190918252602082015260400190565b6020604051808303815f875af1158015612e42573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061119d9190617fc2565b612e6f81616035565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001612eca906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612f1a575f5ffd5b505af1158015612f2c573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001612f70906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001612fa0929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015613003573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130279190618218565b50806001600160a01b031663e2a4853a8460405160200161306f906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161309f929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156130fc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131209190617fc2565b50806001600160a01b031663e2a4853a84604051602001613167906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613197929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156131f3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132179190617fc2565b50806001600160a01b031663e2a4853a84604051602001613263906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613293929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156132f0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133149190617fc2565b50806001600160a01b031663e2a4853a8460405160200161333490618233565b60405160208183030381529060405280519060200120604051602001613364929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156133c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e59190617fc2565b50806001600160a01b031663e2a4853a84604051602001613432906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001613462929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156134bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134e39190617fc2565b50806001600160a01b031663e2a4853a8460405160200161352c906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b6040516020818303038152906040528051906020012060405160200161355c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156135b9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135dd9190617fc2565b50806001600160a01b031663ca446dd98460405160200161361e906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b6040516020818303038152906040528051906020012060405160200161364e929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156136b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136d89190618218565b50806001600160a01b031663e2a4853a8460405160200161372090602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001613750929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156137af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137d39190617fc2565b50806001600160a01b031663e2a4853a8460405160200161381a90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161384a929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156138a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138cd9190617fc2565b50806001600160a01b031663e2a4853a8460405160200161391990602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613949929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156139a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139cd9190617fc2565b50806001600160a01b031663e2a4853a846040516020016139ed90618274565b60405160208183030381529060405280519060200120604051602001613a1d929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613a7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613aa19190617fc2565b50806001600160a01b031663e2a4853a84604051602001613aee90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b1e929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613b7e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ba29190617fc2565b50806001600160a01b031663e2a4853a84604051602001613beb90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c1b929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613c7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c9f9190617fc2565b50806001600160a01b031663ca446dd984604051602001613cdd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613d0d929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401613d579291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613d73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d979190618218565b50806001600160a01b031663ca446dd984604051602001613de9906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613e19929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613e64926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613e80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ea49190618218565b50806001600160a01b031663e2a4853a84604051602001613eeb906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f1b929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613f5c929190918252602082015260400190565b6020604051808303815f875af1158015613f78573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f9c9190617fc2565b50806001600160a01b031663e2a4853a84604051602001613fee906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161401e929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612e26929190918252602082015260400190565b5f5f5f5f61409c60405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f881180156140a9575085155b156140e0578951602090810151516001600160a01b0390811683528b51515116908201526040810188905260608101879052614122565b5f891180156140ed575086155b1561412257895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b1580156141dd575f5ffd5b505af11580156141ef573d5f5f3e3d5ffd5b50505050505050505050505050565b6040516304e6bdd160e11b81526001600160a01b038a81166004830152602482018a9052888116604483015287811660648301526084820187905260a4820186905260c4820185905260e4820184905261010482018390528b16906309cd7ba290610124015f604051808303815f87803b15801561427a575f5ffd5b505af115801561428c573d5f5f3e3d5ffd5b5050505050505050505050505050565b61023782826040516024016142b29291906182b5565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052616244565b6142e9617d57565b5f5f6142f5868561624d565b90505f61430286836162b3565b9050610e3987826162c5565b5f816001600160a01b0316836001600160a01b03161061432f578183614332565b82825b604051919450925061435f906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b6143be617d7d565b826143c7617d7d565b816001600160a01b03166391d4403c604051602001614403906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614457573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061447b9190617fd9565b6144885791506111cd9050565b816001600160a01b03166321f8a721856040516020016144c8906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016144f8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161452c91815260200190565b602060405180830381865afa158015614547573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061456b9190618218565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016145e9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161461d91815260200190565b602060405180830381865afa158015614638573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061465c9190617fc2565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016146b2906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016146e2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161471691815260200190565b602060405180830381865afa158015614731573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147559190617fc2565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016147b0906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016147e0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161481491815260200190565b602060405180830381865afa15801561482f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148539190617fc2565b815151606001526040516001600160a01b0383169063bd02d0f590869061487c90602001618233565b604051602081830303815290604052805190602001206040516020016148ac929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148e091815260200190565b602060405180830381865afa1580156148fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491f9190617fc2565b81515f60200201516080018181525050816001600160a01b031663bd02d0f58560405160200161497b906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016149ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149df91815260200190565b602060405180830381865afa1580156149fa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a1e9190617fc2565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614a9b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614acf91815260200190565b602060405180830381865afa158015614aea573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b0e9190617fc2565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614b83929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bb791815260200190565b602060405180830381865afa158015614bd2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614bf69190618218565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ca091815260200190565b602060405180830381865afa158015614cbb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614cdf9190617fc2565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614d3690602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d66929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d9a91815260200190565b602060405180830381865afa158015614db5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614dd99190617fc2565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614e3590602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e65929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614e9991815260200190565b602060405180830381865afa158015614eb4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ed89190617fc2565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614f0890618274565b60405160208183030381529060405280519060200120604051602001614f38929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614f6c91815260200190565b602060405180830381865afa158015614f87573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fab9190617fc2565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161500890602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001615038929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161506c91815260200190565b602060405180830381865afa158015615087573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150ab9190617fc2565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161510490602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001615134929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161516891815260200190565b602060405180830381865afa158015615183573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151a79190617fc2565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016151f590602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001615225929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525991815260200190565b602060405180830381865afa158015615274573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152989190618218565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615306906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615336929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161536a91815260200190565b602060405180830381865afa158015615385573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153a99190618218565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f58560405160200161540c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b6040516020818303038152906040528051906020012060405160200161543c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161547091815260200190565b602060405180830381865afa15801561548b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154af9190617fc2565b60608201526040516001600160a01b0383169063bd02d0f5908690615508906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615538929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161556c91815260200190565b602060405180830381865afa158015615587573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155ab9190617fc2565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b608081015142908190036155f8575050565b81515160a001511561564e5781515f9061562190825b602002015160400151846080015161633f565b83519091506156459082905f5b60200201516020015161637390919063ffffffff16565b83515160200152505b81516020015160a001511561568a5781515f9061566c90600161560e565b835190915061567e908290600161562e565b83516020908101510152505b608090910152565b5f5f5f5f61569e617db1565b6156a7896163b4565b6101c082018190526156be9088905f908190616405565b5060408401525081526101c08101516156dd9088906001905f90616405565b506060840152506020820152805115806156f957506020810151155b15615710575f5f5f5f9450945094509450506157b3565b851561572c57878160200181815161572891906180b2565b9052505b805160208201516157429190611593818c6164ea565b6080820181905281516157549161653e565b60a0820152606087015160381c61ffff16610140820181905260a082015161578b91615783906127109061653e565b612710615a42565b6040820151606083015161014084015160a08501516157a9916158cf565b9450945094509450505b945094509450949050565b5f5f5f5f6157ca617db1565b6157d3896163b4565b6101c082018190526157ea9088905f908190616405565b5060408401525081526101c08101516158099088906001905f90616405565b5060608401525060208201528051158061582557506020810151155b1561583c575f5f5f5f9450945094509450506157b3565b85156158575787815f0181815161585391906180b2565b9052505b606087015160381c61ffff16610140820181905261587e908990615783906127109061653e565b61016082018190528151602083015161589c926115939083906164ea565b6080820181905260208201516158b19161653e565b60c08201819052604082015160608301516101408401516157a9908c905b5f811561138819839004841115176158e5575f5ffd5b506127109102611388010490565b81515160c001805182919061590990839061809f565b9052505050565b5f5f82121561592157815f036111cd565b5090565b5f8115676765c793fa10079d601b1b60028404190484111715615946575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f61597184606001515f615a14565b90505f61597d86616593565b90505f6159a08261598f85600a6181a0565b676765c793fa10079d601b1b615a42565b90505f6159af875f5f5f616405565b50939a91995090975050505050505050565b5f6033826159cf575f6159d2565b60015b60ff16901b660800000000000019841617905092915050565b610237604051806040016040528060068152602001652573202d257360d01b81525083836165e4565b5f60ff60581b1960585f1960ff851601615a34575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f03615a7657838281615a6c57615a6c6181ab565b0492505050615afb565b808411615a965760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150505b9392505050565b5f826001600160a01b031663bd02d0f5615b1b8461662b565b6040518263ffffffff1660e01b8152600401615b3991815260200190565b602060405180830381865afa158015615b54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615afb9190617fc2565b826001600160a01b031663e2a4853a615b908461662b565b6040516001600160e01b031960e084901b168152600481019190915263ffffffff841660248201526044015b6020604051808303815f875af1158015615bd8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114969190617fc2565b826001600160a01b031663e2a4853a615c14846166de565b836040518363ffffffff1660e01b8152600401615bbc929190918252602082015260400190565b826001600160a01b031663e2a4853a615b9084616732565b826001600160a01b031663e2a4853a615c1484616793565b826001600160a01b031663e2a4853a615c14846167f9565b5f826001600160a01b031663bd02d0f5615b1b846166de565b5f826001600160a01b031663bd02d0f5615b1b84616732565b5f816001600160a01b031663bd02d0f5604051602001615cf4906020808252600b908201526a1515d05417d411549253d160aa1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d2891815260200190565b602060405180830381865afa158015615d43573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111cd9190617fc2565b5f826001600160a01b031663bd02d0f5615b1b84616793565b60e084015160011901615da757600160e085015260a0840182905260608401839052611496565b60e08401515f1901615e195760a084018051908390615dc6828461809f565b9052508115615e13575f615dda8585616373565b6060870151615de99084616373565b615df3919061809f565b9050615e0c8660a001518261592590919063ffffffff16565b6060870152505b50611496565b60e084015161149657818460c001511115615e4857818460c001818151615e4091906180b2565b905250611496565b818460c0015103615e6c57600260e08501525f60c085018190526080850152611496565b600160e085015260c0840151615e8290836180b2565b60a0850152505060608201525f60c08201819052608090910152565b60e084015160011901615ec4575f60e085015260c0840182905260808401839052611496565b60e0840151615f325760c084018051908390615ee0828461809f565b9052508115615e13575f615ef48585616373565b6080870151615f039084616373565b615f0d919061809f565b9050615f268660c001518261592590919063ffffffff16565b60808701525050611496565b60e08401515f190161149657818460a001511115615f5c57818460a001818151615e4091906180b2565b818460a0015103615f8057600260e08501525f60a085018190526060850152611496565b5f60e085015260a0840151615f9590836180b2565b60c0850152505060808201525f60a08201819052606090910152565b5f604051602001615feb906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61606a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b616074845f616842565b60208301528152606084015161608a905f615a14565b6060820181905281516160af91676765c793fa10079d601b1b9061159390600a6181a0565b604082015260208101515f036160ca575f608082015261616a565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015616140573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906161649190617fc2565b60808201525b616175846001616842565b602083018190529082525f03616190575f60a0820152616230565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015616206573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061622a9190617fc2565b60a08201525b80608001518160a001519250925050915091565b61099481616888565b5f60405160200161627a906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0385169082015260608101839052608001614398565b6162bb617d57565b615afb83836168a7565b60408101516001600160a01b03166162f057604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f8061634b83426180b2565b61635590856181ee565b6301e13380900490506111c981676765c793fa10079d601b1b61809f565b5f81156b019d971e4fe8401e740000001983900484111517616393575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f816001600160a01b031663bd02d0f5604051602001615cf4906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff166002811061642257616422618051565b602002015190505f6164348a8a617ab9565b9050805f03616450575f5f5f5f955095509550955050506157b3565b5f61645f838c60800151617ba7565b90505f61646c828a616373565b90505f89156164915781841061648b57616486848361653e565b616493565b5f616493565b5f5b90505f6164a0858d616373565b90505f8c156164c5578482106164bf576164ba828661653e565b6164c7565b5f6164c7565b5f5b90506164d3858761809f565b9f959e50919c50909a509298505050505050505050565b5f826164f6838261809f565b91508110156111cd5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f8261654a83826180b2565b91508111156111cd5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f816001600160a01b031663bd02d0f5604051602001615cf49060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b6108d98383836040516024016165fc939291906182e2565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052616244565b80515f90819061664c90825b6020020151518451600160200201515161430e565b905080604051602001616690906020808252601a908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d50000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016166c0929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b80515f9081906166ee9082616637565b905080604051602001616690906020808252601f908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b455900604082015260600190565b80515f9081906167429082616637565b9050806040516020016166909060208082526024908201527f545741505f4c4153545f424c4f434b5f54494d455f5354414d505f42595f5045604082015263149253d160e21b606082015260800190565b80515f9081906167a39082616637565b9050806040516020016166909060208082526029908201527f545741505f4c4153545f50524943455f43554d4d554c41544956455f4b45595f604082015268109657d411549253d160ba1b606082015260800190565b80515f9081906168099082616637565b90508060405160200161669090602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b5f5f5f61686f855f01518560ff166002811061686057616860618051565b60200201518660800151617ba7565b90505f61687c8686617ab9565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b6168af617d57565b826168b8617d57565b816001600160a01b03166391d4403c6040516020016168f8906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561694c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906169709190617fd9565b61697d5791506111cd9050565b816001600160a01b031663bd02d0f5856040516020016169b7906020808252600690820152651413d4d7d25160d21b604082015260600190565b604051602081830303815290604052805190602001206040516020016169e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616a1b91815260200190565b602060405180830381865afa158015616a36573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616a5a9190617fc2565b816020018181525050816001600160a01b03166321f8a72185604051602001616aa2906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616ad2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616b0691815260200190565b602060405180830381865afa158015616b21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616b459190618218565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001616ba1906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616bd1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616c0591815260200190565b602060405180830381865afa158015616c20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616c449190618218565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001616cbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616cf391815260200190565b602060405180830381865afa158015616d0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616d329190617fc2565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001616d869060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001616db6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616dea91815260200190565b602060405180830381865afa158015616e05573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616e299190617fc2565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001616e83906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616eb3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616ee791815260200190565b602060405180830381865afa158015616f02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616f269190617fc2565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001616f7f906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616faf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616fe391815260200190565b602060405180830381865afa158015616ffe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906170229190617fc2565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016170a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016170d691815260200190565b602060405180830381865afa1580156170f1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906171159190617fc2565b81515f60200201516080018181525050816001600160a01b031663bd02d0f58560405160200161716f906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161719f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016171d391815260200190565b602060405180830381865afa1580156171ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906172129190617fc2565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001617285929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016172b991815260200190565b602060405180830381865afa1580156172d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906172f89190617fc2565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a7219086906080016040516020818303038152906040528051906020012060405160200161736c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016173a091815260200190565b602060405180830381865afa1580156173bb573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906173df9190618218565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161748691815260200190565b602060405180830381865afa1580156174a1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906174c59190617fc2565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161751a9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161754a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161757e91815260200190565b602060405180830381865afa158015617599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906175bd9190617fc2565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161761890602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001617648929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161767c91815260200190565b602060405180830381865afa158015617697573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906176bb9190617fc2565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161771590602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001617745929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161777991815260200190565b602060405180830381865afa158015617794573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906177b89190617fc2565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161781490602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001617844929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161787891815260200190565b602060405180830381865afa158015617893573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906178b79190617fc2565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161791290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001617942929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161797691815260200190565b602060405180830381865afa158015617991573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906179b59190617fc2565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001617a04906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001617a34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401617a6891815260200190565b602060405180830381865afa158015617a83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617aa79190617fc2565b81516020015160e00152949350505050565b5f5f835f01518360ff1660028110617ad357617ad3618051565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015617b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190617b509190617fc2565b9050805f03617b63575f925050506111cd565b606082015160c0830151617b77908261809f565b8210617b9b5760c0830151617b8c82846180b2565b617b9691906180b2565b617b9d565b5f5b9695505050505050565b5f8260a001515f03617bba57505f6111cd565b5f617bc58484617bd7565b60a08501519091506111c99082616373565b5f428203617bea575060208201516111cd565b5f617bf984604001518461633f565b9050617c1284602001518261637390919063ffffffff16565b9150506111cd565b604051806102800160405280617c2e617d7d565b81526020015f8152602001617c41617d57565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806102000160405280617cdc617d7d565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280617d6a617e18565b81525f6020820181905260409091015290565b6040518060a00160405280617d90617e86565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b617e706040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617e275790505090565b60405180604001604052806002905b617ed76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081617e955790505090565b6001600160a01b0381168114610994575f5ffd5b5f5f82840360c0811215617f13575f5ffd5b8335617f1e81617eed565b925060a0601f1982011215617f31575f5ffd5b506020830190509250929050565b5f5f828403610100811215617f52575f5ffd5b8335617f5d81617eed565b925060e0601f1982011215617f31575f5ffd5b5f60208284031215617f80575f5ffd5b8135615afb81617eed565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215617fd2575f5ffd5b5051919050565b5f60208284031215617fe9575f5ffd5b81518015158114615afb575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061804990830184617ff8565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f83128015838313168383128216171561809857618098618065565b5092915050565b808201808211156111cd576111cd618065565b818103818111156111cd576111cd618065565b6001815b6001841115610dfb578085048111156180e4576180e4618065565b60018416156180f257908102905b60019390931c9280026180c9565b5f8261810e575060016111cd565b8161811a57505f6111cd565b8160018114618130576002811461813a57618156565b60019150506111cd565b60ff84111561814b5761814b618065565b50506001821b6111cd565b5060208310610133831016604e8410600b8410161715618179575081810a6111cd565b6181855f1984846180c5565b805f190482111561819857618198618065565b029392505050565b5f615afb8383618100565b634e487b7160e01b5f52601260045260245ffd5b5f826181cd576181cd6181ab565b500690565b63ffffffff82811682821603908111156111cd576111cd618065565b80820281158282048414176111cd576111cd618065565b5f82618213576182136181ab565b500490565b5f60208284031215618228575f5ffd5b8151615afb81617eed565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6182c76040830185617ff8565b82810360208401526182d98185617ff8565b95945050505050565b606081525f6182f46060830186617ff8565b82810360208401526183068186617ff8565b91505082604083015294935050505056fe2d2d2d2d2d2d2d2d2d2d2d7570646174655477617050726963652d2d2d2d2d2d2d2d2d2d2da264697066735822122025b53b816a0eb226e84738b84654305ba0eacfe0b06ca9a86d3a051390cc3ca664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04a\x7F\x01V[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04a\x7F?V[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08\xDEV[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90a\x7FpV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x97V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90a\x7F\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90a\x7F\xC2V[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90a\x7F\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90a\x7F\xC2V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90a\x7F\xD9V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90a\x80&V[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\xBEV[a\x06\x1Ca|\x1AV[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r\xE2V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\x0E\x03V[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x86Q\x92\x87\x01Q\x90\x87\x01Qa\x06\x8F\x95\x93\x94\x92\x91\x90_a\x0EDV[a\x02@\x86\x01Ra\x01\xA0\x85\x01Ra\x01\x80\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCE\x94\x92\x93\x91\x92\x90\x91a\x11\x17V[a\x06\xE0\x81_\x01Q\x82a\x02@\x01Qa\x11\xA4V[a\x02`\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x10\x93\x92\x91_\x91a\x07\t\x91\x90a\x80yV[__a\x11\xD3V[a\x073\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\t\x91\x90a\x80yV[`\xA0\x81\x01Q\x15a\x07KW\x81Q\x81Qa\x07K\x91\x90a\x14WV[a\x07l\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa\x14\x9CV[a\x01`\x82\x01\x81\x90R\x82Q\x82Qa\x07\x81\x92a\x16iV[a\x07\xAD\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Q\x87a\x01`\x01Qa\x1BiV[a\x07\xC3\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x1B\xC3V[a\x07\xD4\x82` \x01Q\x82_\x01Qa.fV[\x81Q` \x82\x01Q\x82Qa\x07\xE8\x92\x91\x90a.\x8AV[a\x08\t\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa@_V[a\x02 \x85\x01\x81\x90Ra\x02\0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xE0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xC0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02`\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08\x99\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91aAGV[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80Q\x80Q\x91\x85\x01Q\x80Q\x84\x89\x01Q\x83\x88\x01Q\x93\x86\x01Q\x97\x83\x01Q\x92\x90\x95\x01Qa\x08\xD9\x97\x8B\x96`\x04\x96\x95\x93\x94\x90\x93\x92aA\xFEV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\t\x1C\x90a\x7F\x8BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x94\x91\x90a\x7F\xC2V[PV[a\t\xC3`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\xBEV[a\t\xCBa|\xC8V[a\t\xE1\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\x0E\x03V[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n?W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nc\x91\x90a\x7F\xC2V[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDE\x91\x90a\x7F\xC2V[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\n\xF7WP`\x80\x81\x01Q\x15[\x15a\x0B\x15W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x11\x15a\x0B0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x11\x15a\x0BKW`\xA0\x82\x01Q`\x80\x82\x01R[a\x0Bq\x82_\x01Q\x82_\x01Q\x83``\x01Q\x84`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`\x01a\x0EDV[a\x01\xC0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B\xA5\x91a\x11\xA4V[a\x01\xE0\x82\x01R`\xA0\x81\x01Q\x15a\x0C/W`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x18W__\xFD[PZ\xF1\x15\x80\x15a\x0C*W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C\xB4W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x9DW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xAFW=__>=_\xFD[PPPP[a\x0C\xC5\x82` \x01Q\x82_\x01Qa.fV[``\x81\x01Q\x15a\x0C\xDDW\x81Q\x81Qa\x0C\xDD\x91\x90a\x14WV[\x81Q` \x82\x01Q\x82Qa\x0C\xF1\x92\x91\x90a.\x8AV[a\r\x11\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa@_V[a\x01\xA0\x85\x01Ra\x01\x80\x84\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x84\x01R\x16a\x01@\x82\x01R\x80Q``\x82\x01Q`\x80\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\rX\x94\x93\x92\x91\x90a\x14\x9CV[a\x01 \x82\x01\x81\x90R\x82Q\x82Qa\rm\x92a\x16iV[a\x08\xD9\x82` \x01Q\x84\x83a\x01@\x01Q\x84a\x01`\x01Qbz\x12\0\x86a\x01\x80\x01Q\x87a\x01\xA0\x01Q\x88a\x01\xE0\x01Q`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RPaAGV[a\t\x94`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82aB\x9CV[a\r\xEAa}WV[_a\r\xF6\x85\x85\x85aB\xE1V[\x91P\x91P[\x93P\x93\x91PPV[a\x0E\x0Ba}}V[__a\x0E\x17\x85\x85aC\x0EV[\x90P_a\x0E$\x87\x83aC\xB6V[\x90Pa\x0E0\x81\x83aU\xB8V[a\x0E9\x81aU\xE6V[\x96\x90\x95P\x93PPPPV[_____a\x0E\xA1`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8B\x11\x80\x15a\x0E\xAFWP_\x8A\x11[\x15a\x0E\xCDW`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x8A\x11\x80\x15a\x0E\xDAWP\x87\x15[\x15a\x0F\xD6Wa\x0E\xEB\x8D\x8B\x8E\x8AaV\x92V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R_\x03a\x0FLW`@Qc\x17\xA6\xEFc`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q\x89\x11\x15a\x0F{W\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0F\x92\x91\x90a\x80\x9FV[\x11\x15a\x0F\xD1Wa\x01\0\x81\x01Q\x81Qa\x0F\xAA\x91\x90a\x80\x9FV[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x10\xE4V[_\x8B\x11\x80\x15a\x0F\xE3WP\x88\x15[\x15a\x10\xCBWa\x0F\xF4\x8D\x8C\x8E\x8AaW\xBEV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R_\x03a\x10ZW`@Qc\x17\xA6\xEFc`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x87\x81` \x01Q\x10\x15a\x10\x8FW` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0F\xD1W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x01Q\x81` \x01Q\x82`@\x01Q\x83``\x01Q\x84a\x01\0\x01Q\x95P\x95P\x95P\x95P\x95PP\x97P\x97P\x97P\x97P\x97\x92PPPV[_\x83\x11\x80\x15a\x11$WP\x80\x15[\x15a\x11sW\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x11nW\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x11\x9DV[_\x84\x11\x80\x15a\x11\x80WP\x81\x15[\x15a\x11\x9DW\x84QQ` \x01Q\x84\x11\x15a\x11\x9DW\x84Q\x84\x90_a\x11AV[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x11\xBD\x84\x83aX\xCFV[\x90Pa\x11\xC9\x85\x82aX\xF3V[\x91PP[\x92\x91PPV[_a\x11\xDD\x84aY\x10V[\x90P_\x84\x12a\x12KW\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11\xFEWa\x11\xFEa\x80QV[` \x02\x01Q` \x01\x81\x81Qa\x12\x13\x91\x90a\x80\x9FV[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12.Wa\x12.a\x80QV[` \x02\x01Q``\x01\x81\x81Qa\x12C\x91\x90a\x80\x9FV[\x90RPa\x12\xACV[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12cWa\x12ca\x80QV[` \x02\x01Q` \x01\x81\x81Qa\x12x\x91\x90a\x80\xB2V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x12\x93Wa\x12\x93a\x80QV[` \x02\x01Q``\x01\x81\x81Qa\x12\xA8\x91\x90a\x80\xB2V[\x90RP[\x81\x15a\x13/W\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x12\xCAWa\x12\xCAa\x80QV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x12\xEEWa\x12\xEEa\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\x03\x91\x90a\x80\xB2V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x13\x1EWa\x13\x1Ea\x80QV[` \x02\x01Q`@\x01RPa\x14O\x90PV[\x82_\x03a\x13<WPa\x14OV[_a\x13F\x84aY\x10V[\x90P_a\x13|\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x13dWa\x13da\x80QV[` \x02\x01Q` \x01Q\x83aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x13\xEAW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\x9DWa\x13\x9Da\x80QV[` \x02\x01Q`@\x01\x81\x81Qa\x13\xB2\x91\x90a\x80\x9FV[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x13\xCDWa\x13\xCDa\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\xE2\x91\x90a\x80\x9FV[\x90RPa\x14KV[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x14\x02Wa\x14\x02a\x80QV[` \x02\x01Q`@\x01\x81\x81Qa\x14\x17\x91\x90a\x80\xB2V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x142Wa\x142a\x80QV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x14G\x91\x90a\x80\xB2V[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x14u\x84\x84aY`V[\x91P\x91P\x81\x81\x10a\x14\x96W``\x83\x01Qa\x14\x90\x90`\x01aY\xC1V[``\x84\x01R[PPPPV[____\x86\x11\x80\x15a\x14\xACWP\x83\x15[\x15a\x14\xBBWP\x83\x90P\x84a\x14\xD3V[_\x87\x11\x80\x15a\x14\xC8WP\x84\x15[\x15a\x10\xCBWP\x85\x90P\x82[\x81\x15\x80a\x14\xDEWP\x80\x15[\x15a\x14\xFCW`@Qc\x01\xA2\x86\x8B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15(`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountBase`\xB0\x1B\x81RP\x83aY\xEBV[a\x15T`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountMeme`\xB0\x1B\x81RP\x82aY\xEBV[_a\x15c\x89``\x01Q_aZ\x14V[\x90P_a\x15u\x8A``\x01Q`\x01aZ\x14V[\x90P_a\x15\x98\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x15\x93\x86`\na\x81\xA0V[aZBV[\x90P_a\x15\xB6\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x15\x93\x86`\na\x81\xA0V[\x90Pa\x15\xEC`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\x98\\\xD9P[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x83aY\xEBV[a\x16 `@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x1BY[YP[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x82aY\xEBV[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x16O\x90a\x16J\x84\x84aY%V[aY\xEBV[a\x16Y\x82\x82aY%V[\x9C\x9BPPPPPPPPPPPPV[\x80_\x03a\x16uWPPPV[a\x16\x96`@Q\x80``\x01`@R\x80`%\x81R` \x01a\x83\x18`%\x919a\r\xBEV[`@\x80Qa\x01@\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91Ra\x16\xF0\x84\x84a[\x02V[c\xFF\xFF\xFF\xFF\x16\x81Ra\x17\x07d\x01\0\0\0\0Ba\x81\xBFV[c\xFF\xFF\xFF\xFF\x90\x81\x16` \x83\x01R\x81Q\x16_\x03a\x17\xBDWa\x17,\x84\x84\x83` \x01Qa[xV[a\x177\x84\x84_a[\xFCV[a\x17F\x84\x84\x83` \x01Qa\\;V[a\x17Q\x84\x84_a\\SV[a\x17\\\x84\x84\x84a\\kV[a\x17\x96`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x06&\xC6\xF66\xB5F\x96\xD6U7F\x16\xD7`\x94\x1B\x81RP\x82` \x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x14\x96`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dprice`\xD8\x1B\x81RP\x83aY\xEBV[\x80Q` \x82\x01Qa\x17\xCE\x91\x90a\x81\xD2V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x80\x84\x01\x91\x82R\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B` \x82\x01R\x90Qa\x18\n\x92\x16aY\xEBV[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15a\x14\x96Wa\x18$\x84\x84a\\\x83V[``\x82\x01R`@\x81\x01Qa\x18>\x90c\xFF\xFF\xFF\xFF\x16\x83a\x81\xEEV[\x81``\x01Qa\x18M\x91\x90a\x80\x9FV[`\x80\x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra\x18x\x90\x83aY\xEBV[a\x18\xAF`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x1D\x1A[YQ[\x18\\\x1C\xD9Y`\xAA\x1B\x81RP\x82`@\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x18\xE8`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1C\x1C\x9AX\xD9P\xDD[][\x18]\x1A]\x99S\x18\\\xDD`j\x1B\x81RP\x82``\x01QaY\xEBV[a\x19\x1D`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaY\xEBV[a\x19,\x84\x84\x83`\x80\x01Qa[\xFCV[a\x19;\x84\x84\x83` \x01Qa[xV[a\x19E\x84\x84a\\\x9CV[c\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01\x81\x90R` \x82\x01Qa\x19a\x91\x90a\x81\xD2V[c\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01Ra\x19u\x84a\\\xB5V[c\xFF\xFF\xFF\xFF\x90\x81\x16`\xE0\x83\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x06\x81Re\x14\x11T\x92S\xD1`\xD2\x1B` \x82\x01R\x90Qa\x19\xAD\x92\x16aY\xEBV[a\x19\xF6`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FlastUpdateTimestampByPeriod\0\0\0\0\0\x81RP\x82`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[a\x1A5`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x1D\x1A[YQ[\x18\\\x1C\xD9Y\x10\x9ET\x19\\\x9A[\xD9`j\x1B\x81RP\x82`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16aY\xEBV[\x80`\xE0\x01Qc\xFF\xFF\xFF\xFF\x16\x81`\xC0\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\x96Wa\x1A[\x84\x84a]gV[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Q`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x91a\x1A\x7F\x91a\x80\xB2V[a\x1A\x89\x91\x90a\x82\x05V[a\x01 \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\x1B\x81R\x7FpriceCumulativeLastByPeriod\0\0\0\0\0` \x82\x01Ra\x01\0\x82\x01Qa\x1A\xD3\x91\x90aY\xEBV[a\x1B\x08`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01npriceCumulative`\x88\x1B\x81RP\x82`\x80\x01QaY\xEBV[a\x1B;`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kpriceAverage`\xA0\x1B\x81RP\x82a\x01 \x01QaY\xEBV[a\x1BJ\x84\x84\x83` \x01Qa\\;V[a\x1BY\x84\x84\x83`\x80\x01Qa\\SV[a\x14\x96\x84\x84\x83a\x01 \x01Qa\\kV[_a\x1Bt\x85\x84a\x80yV[\x90P_\x81\x13\x15a\x1B\x9EW\x86Q` \x01Qa\x1B\x99\x90\x83a\x1B\x92\x84aY\x10V[`\x01a]\x80V[a\x1B\xB9V[\x86Q` \x01Qa\x1B\xB9\x90\x83a\x1B\xB2\x84aY\x10V[`\x01a^\x9EV[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1C\x07\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1CWW__\xFD[PZ\xF1\x15\x80\x15a\x1CiW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x1C\x89\x84`@\x01Qa_\xB1V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xB0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xD9W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\x17\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1DG\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x88\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC8\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1E\x08\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x1E\x83\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xC3\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1F\x03\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBA\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\xFF\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a /\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB0\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \xF5\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xA5\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\xF0\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\" \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xA1\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x9C\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\xE8\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$uW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$\xE4\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\x14\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x95\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%\xD4\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x86\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a&\xC6\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x80\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'\xC5\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(x\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(\xBD\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)p\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)\xBB\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*o\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*\xB9\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+m\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+\xB9\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,IW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,m\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,\xB8\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-HW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-l\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-\xAB\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\xDB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x9D\x91\x90a\x7F\xC2V[a.o\x81a`5V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a.\xCA\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x1AW__\xFD[PZ\xF1\x15\x80\x15a/,W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a/p\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0'\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0o\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1 \x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1g\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1\x97\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x17\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2c\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2\x93\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x14\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a34\x90a\x823V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE5\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a42\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xE3\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a5,\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xDD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a6\x1E\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xD8\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7 \x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xD3\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a8\x1A\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8J\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xCD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9\x19\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9I\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a9\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xCD\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a9\xED\x90a\x82tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a:}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xA1\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a:\xEE\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\x1E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a;~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xA2\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a;\xEB\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a<{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x9F\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a<\xDD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=W\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a=sW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\x97\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a=\xE9\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra>d\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a>\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xA4\x91\x90a\x82\x18V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a>\xEB\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a?xW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x9C\x91\x90a\x7F\xC2V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a?\xEE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\x1E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a@\x9C`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a@\xA9WP\x85\x15[\x15a@\xE0W\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90RaA\"V[_\x89\x11\x80\x15a@\xEDWP\x86\x15[\x15aA\"W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aA\xDDW__\xFD[PZ\xF1\x15\x80\x15aA\xEFW=__>=_\xFD[PPPPPPPPPPPPPV[`@Qc\x04\xE6\xBD\xD1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x8A\x90R\x88\x81\x16`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R`\xE4\x82\x01\x84\x90Ra\x01\x04\x82\x01\x83\x90R\x8B\x16\x90c\t\xCD{\xA2\x90a\x01$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15aBzW__\xFD[PZ\xF1\x15\x80\x15aB\x8CW=__>=_\xFD[PPPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01aB\xB2\x92\x91\x90a\x82\xB5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RabDV[aB\xE9a}WV[__aB\xF5\x86\x85abMV[\x90P_aC\x02\x86\x83ab\xB3V[\x90Pa\x0E9\x87\x82ab\xC5V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10aC/W\x81\x83aC2V[\x82\x82[`@Q\x91\x94P\x92PaC_\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[aC\xBEa}}V[\x82aC\xC7a}}V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aD\x03\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDWW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD{\x91\x90a\x7F\xD9V[aD\x88W\x91Pa\x11\xCD\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aD\xC8\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE,\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aEGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEk\x91\x90a\x82\x18V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\\\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xB2\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGU\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\xB0\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xE0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\x14\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHS\x91\x90a\x7F\xC2V[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aH|\x90` \x01a\x823V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xE0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x1F\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI{\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x1E\x91\x90a\x7F\xC2V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\x9B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xCF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x0E\x91\x90a\x7F\xC2V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xB7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\xF6\x91\x90a\x82\x18V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xA0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xDF\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM6\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aMf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xD9\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN5\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNe\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xD8\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aO\x08\x90a\x82tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aOl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xAB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aP\x08\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aPl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xAB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQ\x04\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQh\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xA7\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aQ\xF5\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aRY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRtW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x98\x91\x90a\x82\x18V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aS\x06\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xA9\x91\x90a\x82\x18V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\x0C\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTp\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xAF\x91\x90a\x7F\xC2V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aU\x08\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUl\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xAB\x91\x90a\x7F\xC2V[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aU\xF8WPPV[\x81QQ`\xA0\x01Q\x15aVNW\x81Q_\x90aV!\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01Qac?V[\x83Q\x90\x91PaVE\x90\x82\x90_[` \x02\x01Q` \x01Qacs\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aV\x8AW\x81Q_\x90aVl\x90`\x01aV\x0EV[\x83Q\x90\x91PaV~\x90\x82\x90`\x01aV.V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aV\x9Ea}\xB1V[aV\xA7\x89ac\xB4V[a\x01\xC0\x82\x01\x81\x90RaV\xBE\x90\x88\x90_\x90\x81\x90ad\x05V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaV\xDD\x90\x88\x90`\x01\x90_\x90ad\x05V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aV\xF9WP` \x81\x01Q\x15[\x15aW\x10W____\x94P\x94P\x94P\x94PPaW\xB3V[\x85\x15aW,W\x87\x81` \x01\x81\x81QaW(\x91\x90a\x80\xB2V[\x90RP[\x80Q` \x82\x01QaWB\x91\x90a\x15\x93\x81\x8Cad\xEAV[`\x80\x82\x01\x81\x90R\x81QaWT\x91ae>V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01QaW\x8B\x91aW\x83\x90a'\x10\x90ae>V[a'\x10aZBV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01QaW\xA9\x91aX\xCFV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____aW\xCAa}\xB1V[aW\xD3\x89ac\xB4V[a\x01\xC0\x82\x01\x81\x90RaW\xEA\x90\x88\x90_\x90\x81\x90ad\x05V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01QaX\t\x90\x88\x90`\x01\x90_\x90ad\x05V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80aX%WP` \x81\x01Q\x15[\x15aX<W____\x94P\x94P\x94P\x94PPaW\xB3V[\x85\x15aXWW\x87\x81_\x01\x81\x81QaXS\x91\x90a\x80\xB2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90RaX~\x90\x89\x90aW\x83\x90a'\x10\x90ae>V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01QaX\x9C\x92a\x15\x93\x90\x83\x90ad\xEAV[`\x80\x82\x01\x81\x90R` \x82\x01QaX\xB1\x91ae>V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01QaW\xA9\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aX\xE5W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aY\t\x90\x83\x90a\x80\x9FV[\x90RPPPV[__\x82\x12\x15aY!W\x81_\x03a\x11\xCDV[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aYFW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aYq\x84``\x01Q_aZ\x14V[\x90P_aY}\x86ae\x93V[\x90P_aY\xA0\x82aY\x8F\x85`\na\x81\xA0V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaZBV[\x90P_aY\xAF\x87___ad\x05V[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aY\xCFW_aY\xD2V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[a\x027`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83ae\xE4V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aZ4WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aZvW\x83\x82\x81aZlWaZla\x81\xABV[\x04\x92PPPaZ\xFBV[\x80\x84\x11aZ\x96W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP[\x93\x92PPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84af+V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[TW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\xFB\x91\x90a\x7F\xC2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a[\x90\x84af+V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a[\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x96\x91\x90a\x7F\xC2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84af\xDEV[\x83`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a[\x90\x84ag2V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84ag\x93V[\x82`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:a\\\x14\x84ag\xF9V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84af\xDEV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84ag2V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x15\x15\xD0T\x17\xD4\x11T\x92S\xD1`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a](\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xCD\x91\x90a\x7F\xC2V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a[\x1B\x84ag\x93V[`\xE0\x84\x01Q`\x01\x19\x01a]\xA7W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x14\x96V[`\xE0\x84\x01Q_\x19\x01a^\x19W`\xA0\x84\x01\x80Q\x90\x83\x90a]\xC6\x82\x84a\x80\x9FV[\x90RP\x81\x15a^\x13W_a]\xDA\x85\x85acsV[``\x87\x01Qa]\xE9\x90\x84acsV[a]\xF3\x91\x90a\x80\x9FV[\x90Pa^\x0C\x86`\xA0\x01Q\x82aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x14\x96V[`\xE0\x84\x01Qa\x14\x96W\x81\x84`\xC0\x01Q\x11\x15a^HW\x81\x84`\xC0\x01\x81\x81Qa^@\x91\x90a\x80\xB2V[\x90RPa\x14\x96V[\x81\x84`\xC0\x01Q\x03a^lW`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x14\x96V[`\x01`\xE0\x85\x01R`\xC0\x84\x01Qa^\x82\x90\x83a\x80\xB2V[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01a^\xC4W_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x14\x96V[`\xE0\x84\x01Qa_2W`\xC0\x84\x01\x80Q\x90\x83\x90a^\xE0\x82\x84a\x80\x9FV[\x90RP\x81\x15a^\x13W_a^\xF4\x85\x85acsV[`\x80\x87\x01Qa_\x03\x90\x84acsV[a_\r\x91\x90a\x80\x9FV[\x90Pa_&\x86`\xC0\x01Q\x82aY%\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x14\x96V[`\xE0\x84\x01Q_\x19\x01a\x14\x96W\x81\x84`\xA0\x01Q\x11\x15a_\\W\x81\x84`\xA0\x01\x81\x81Qa^@\x91\x90a\x80\xB2V[\x81\x84`\xA0\x01Q\x03a_\x80W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x14\x96V[_`\xE0\x85\x01R`\xA0\x84\x01Qa_\x95\x90\x83a\x80\xB2V[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01a_\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__a`j`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a`t\x84_ahBV[` \x83\x01R\x81R``\x84\x01Qa`\x8A\x90_aZ\x14V[``\x82\x01\x81\x90R\x81Qa`\xAF\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a\x15\x93\x90`\na\x81\xA0V[`@\x82\x01R` \x81\x01Q_\x03a`\xCAW_`\x80\x82\x01RaajV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aa@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aad\x91\x90a\x7F\xC2V[`\x80\x82\x01R[aau\x84`\x01ahBV[` \x83\x01\x81\x90R\x90\x82R_\x03aa\x90W_`\xA0\x82\x01Rab0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ab\x06W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ab*\x91\x90a\x7F\xC2V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[a\t\x94\x81ah\x88V[_`@Q` \x01abz\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01aC\x98V[ab\xBBa}WV[aZ\xFB\x83\x83ah\xA7V[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16ab\xF0W`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80acK\x83Ba\x80\xB2V[acU\x90\x85a\x81\xEEV[c\x01\xE13\x80\x90\x04\x90Pa\x11\xC9\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\x80\x9FV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17ac\x93W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10ad\"Wad\"a\x80QV[` \x02\x01Q\x90P_ad4\x8A\x8Aaz\xB9V[\x90P\x80_\x03adPW____\x95P\x95P\x95P\x95PPPaW\xB3V[_ad_\x83\x8C`\x80\x01Qa{\xA7V[\x90P_adl\x82\x8AacsV[\x90P_\x89\x15ad\x91W\x81\x84\x10ad\x8BWad\x86\x84\x83ae>V[ad\x93V[_ad\x93V[_[\x90P_ad\xA0\x85\x8DacsV[\x90P_\x8C\x15ad\xC5W\x84\x82\x10ad\xBFWad\xBA\x82\x86ae>V[ad\xC7V[_ad\xC7V[_[\x90Pad\xD3\x85\x87a\x80\x9FV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x82ad\xF6\x83\x82a\x80\x9FV[\x91P\x81\x10\x15a\x11\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x82aeJ\x83\x82a\x80\xB2V[\x91P\x81\x11\x15a\x11\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\\\xF4\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[a\x08\xD9\x83\x83\x83`@Q`$\x01ae\xFC\x93\x92\x91\x90a\x82\xE2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RabDV[\x80Q_\x90\x81\x90afL\x90\x82[` \x02\x01QQ\x84Q`\x01` \x02\x01QQaC\x0EV[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01af\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[\x80Q_\x90\x81\x90af\xEE\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY\0`@\x82\x01R``\x01\x90V[\x80Q_\x90\x81\x90agB\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`$\x90\x82\x01R\x7FTWAP_LAST_BLOCK_TIME_STAMP_BY_PE`@\x82\x01Rc\x14\x92S\xD1`\xE2\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ag\xA3\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`)\x90\x82\x01R\x7FTWAP_LAST_PRICE_CUMMULATIVE_KEY_`@\x82\x01Rh\x10\x96W\xD4\x11T\x92S\xD1`\xBA\x1B``\x82\x01R`\x80\x01\x90V[\x80Q_\x90\x81\x90ah\t\x90\x82af7V[\x90P\x80`@Q` \x01af\x90\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[___aho\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10ah`Wah`a\x80QV[` \x02\x01Q\x86`\x80\x01Qa{\xA7V[\x90P_ah|\x86\x86az\xB9V[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[ah\xAFa}WV[\x82ah\xB8a}WV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01ah\xF8\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aiLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aip\x91\x90a\x7F\xD9V[ai}W\x91Pa\x11\xCD\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ai\xB7\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ai\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ajZ\x91\x90a\x7F\xC2V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aj\xA2\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aj\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ak\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90akE\x91\x90a\x82\x18V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01ak\xA1\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ak\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90alD\x91\x90a\x82\x18V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am2\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01am\x86\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01am\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01am\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15an\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90an)\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01an\x83\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01an\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01an\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ao\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ao&\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ao\x7F\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ao\xAF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ao\xE3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ao\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ap\"\x91\x90a\x7F\xC2V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ap\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ap\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ap\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aq\x15\x91\x90a\x7F\xC2V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aqo\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aq\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aq\xD3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aq\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ar\x12\x91\x90a\x7F\xC2V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ar\x85\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ar\xB9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ar\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ar\xF8\x91\x90a\x7F\xC2V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01asl\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01as\xA0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15as\xBBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90as\xDF\x91\x90a\x82\x18V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01at\x86\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15at\xA1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90at\xC5\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01au\x1A\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01auJ\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01au~\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15au\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90au\xBD\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01av\x18\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01avH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01av|\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15av\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90av\xBB\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aw\x15\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01awE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01awy\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aw\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aw\xB8\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ax\x14\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01axD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01axx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ax\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ax\xB7\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ay\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ayB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ayv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ay\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ay\xB5\x91\x90a\x7F\xC2V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01az\x04\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01az4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01azh\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15az\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90az\xA7\x91\x90a\x7F\xC2V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10az\xD3Waz\xD3a\x80QV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a{,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a{P\x91\x90a\x7F\xC2V[\x90P\x80_\x03a{cW_\x92PPPa\x11\xCDV[``\x82\x01Q`\xC0\x83\x01Qa{w\x90\x82a\x80\x9FV[\x82\x10a{\x9BW`\xC0\x83\x01Qa{\x8C\x82\x84a\x80\xB2V[a{\x96\x91\x90a\x80\xB2V[a{\x9DV[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a{\xBAWP_a\x11\xCDV[_a{\xC5\x84\x84a{\xD7V[`\xA0\x85\x01Q\x90\x91Pa\x11\xC9\x90\x82acsV[_B\x82\x03a{\xEAWP` \x82\x01Qa\x11\xCDV[_a{\xF9\x84`@\x01Q\x84ac?V[\x90Pa|\x12\x84` \x01Q\x82acs\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x11\xCDV[`@Q\x80a\x02\x80\x01`@R\x80a|.a}}V[\x81R` \x01_\x81R` \x01a|Aa}WV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x02\0\x01`@R\x80a|\xDCa}}V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a}ja~\x18V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80a}\x90a~\x86V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a~p`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a~'W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a~\xD7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a~\x95W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x94W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15a\x7F\x13W__\xFD[\x835a\x7F\x1E\x81a~\xEDV[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15a\x7F1W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15a\x7FRW__\xFD[\x835a\x7F]\x81a~\xEDV[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15a\x7F1W__\xFD[_` \x82\x84\x03\x12\x15a\x7F\x80W__\xFD[\x815aZ\xFB\x81a~\xEDV[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x7F\xD2W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x7F\xE9W__\xFD[\x81Q\x80\x15\x15\x81\x14aZ\xFBW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x80I\x90\x83\x01\x84a\x7F\xF8V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x80\x98Wa\x80\x98a\x80eV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[\x81\x81\x03\x81\x81\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[`\x01\x81[`\x01\x84\x11\x15a\r\xFBW\x80\x85\x04\x81\x11\x15a\x80\xE4Wa\x80\xE4a\x80eV[`\x01\x84\x16\x15a\x80\xF2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x80\xC9V[_\x82a\x81\x0EWP`\x01a\x11\xCDV[\x81a\x81\x1AWP_a\x11\xCDV[\x81`\x01\x81\x14a\x810W`\x02\x81\x14a\x81:Wa\x81VV[`\x01\x91PPa\x11\xCDV[`\xFF\x84\x11\x15a\x81KWa\x81Ka\x80eV[PP`\x01\x82\x1Ba\x11\xCDV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x81yWP\x81\x81\na\x11\xCDV[a\x81\x85_\x19\x84\x84a\x80\xC5V[\x80_\x19\x04\x82\x11\x15a\x81\x98Wa\x81\x98a\x80eV[\x02\x93\x92PPPV[_aZ\xFB\x83\x83a\x81\0V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x81\xCDWa\x81\xCDa\x81\xABV[P\x06\x90V[c\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11\xCDWa\x11\xCDa\x80eV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11\xCDWa\x11\xCDa\x80eV[_\x82a\x82\x13Wa\x82\x13a\x81\xABV[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x82(W__\xFD[\x81QaZ\xFB\x81a~\xEDV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a\x82\xC7`@\x83\x01\x85a\x7F\xF8V[\x82\x81\x03` \x84\x01Ra\x82\xD9\x81\x85a\x7F\xF8V[\x95\x94PPPPPV[``\x81R_a\x82\xF4``\x83\x01\x86a\x7F\xF8V[\x82\x81\x03` \x84\x01Ra\x83\x06\x81\x86a\x7F\xF8V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE-----------updateTwapPrice-----------\xA2dipfsX\"\x12 %\xB5;\x81j\x0E\xB2&\xE8G8\xB8FT0[\xA0\xEA\xCF\xE0\xB0l\xA9\xA8m:\x05\x13\x90\xCC<\xA6dsolcC\0\x08\x1C\x003",
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
    /**Custom error with signature `EmptyAmount()` and selector `0x0d143458`.
```solidity
error EmptyAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyAmount {}
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
        impl ::core::convert::From<EmptyAmount> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyAmount()";
            const SELECTOR: [u8; 4] = [13u8, 20u8, 52u8, 88u8];
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
    /**Custom error with signature `EmptySwapOutAmount()` and selector `0xbd377b18`.
```solidity
error EmptySwapOutAmount();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptySwapOutAmount {}
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
        impl ::core::convert::From<EmptySwapOutAmount> for UnderlyingRustTuple<'_> {
            fn from(value: EmptySwapOutAmount) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptySwapOutAmount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptySwapOutAmount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptySwapOutAmount()";
            const SELECTOR: [u8; 4] = [189u8, 55u8, 123u8, 24u8];
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
        EmptyAmount(EmptyAmount),
        EmptyPool(EmptyPool),
        EmptyPosition(EmptyPosition),
        EmptySwapInAmount(EmptySwapInAmount),
        EmptySwapOutAmount(EmptySwapOutAmount),
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
            [13u8, 20u8, 52u8, 88u8],
            [14u8, 121u8, 59u8, 175u8],
            [31u8, 193u8, 7u8, 193u8],
            [34u8, 123u8, 193u8, 83u8],
            [37u8, 199u8, 21u8, 126u8],
            [77u8, 251u8, 191u8, 243u8],
            [99u8, 49u8, 250u8, 177u8],
            [103u8, 26u8, 189u8, 7u8],
            [115u8, 87u8, 217u8, 31u8],
            [163u8, 91u8, 21u8, 11u8],
            [189u8, 55u8, 123u8, 24u8],
            [207u8, 15u8, 21u8, 130u8],
            [234u8, 29u8, 104u8, 146u8],
            [249u8, 56u8, 26u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SwapHandlerErrors {
        const NAME: &'static str = "SwapHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 14usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccountNotMatch(_) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyAmount(_) => {
                    <EmptyAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyPosition(_) => {
                    <EmptyPosition as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptySwapInAmount(_) => {
                    <EmptySwapInAmount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptySwapOutAmount(_) => {
                    <EmptySwapOutAmount as alloy_sol_types::SolError>::SELECTOR
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
                    fn EmptyAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <EmptyAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::EmptyAmount)
                    }
                    EmptyAmount
                },
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
                    fn EmptySwapOutAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SwapHandlerErrors> {
                        <EmptySwapOutAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SwapHandlerErrors::EmptySwapOutAmount)
                    }
                    EmptySwapOutAmount
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
                Self::EmptyAmount(inner) => {
                    <EmptyAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::EmptySwapOutAmount(inner) => {
                    <EmptySwapOutAmount as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyAmount(inner) => {
                    <EmptyAmount as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptySwapOutAmount(inner) => {
                    <EmptySwapOutAmount as alloy_sol_types::SolError>::abi_encode_raw(
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
