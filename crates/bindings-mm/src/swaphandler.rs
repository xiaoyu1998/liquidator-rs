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
    ///0x60e060405234801561000f575f5ffd5b506040516175fb3803806175fb83398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c0516174f061010b5f395f818160dd015281816101c601526102b201525f8181605e015261054c01525f818160b60152818161019701528181610283015281816103750152818161047b015261086001526174f05ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa36600461711b565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617159565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b5061023761085e565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f1919061718a565b6001600160a01b03168152602001836020016020810190610312919061718a565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b919061718a565b6001600160a01b03169052905061022e8382610917565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b1906171a5565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042491906171dc565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b7906171a5565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061023791906171dc565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd91906171f3565b61023757338160405163a35b150b60e01b8152600401610470929190617240565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610d00565b61061c616e52565b61062e83835f01518460400151610d24565b606083015260408201819052825190518051516020919091015151610654929190610d45565b6020830152808252606083015160a0808401829052608085015160c0808601829052918601519186015161068b949392905f610d86565b610220860152610180850152610160840152610100830181905260e08301829052604083015160a084015160c08501516106ca94929391929091610fe1565b6106dc815f015182610220015161106e565b6102408201528051604082015160a083015160e084015161070c9392915f916107059190617293565b5f5f61109d565b61072f815f0151826040015160018460c001518561010001516107059190617293565b60a08101511561074757815181516107479190611321565b61076d815f015182604001518360a001518460c001518560e00151866101000151611366565b610783825f0151826060015183604001516113d0565b6107948260200151825f0151612673565b8151602082015182516107a8929190612697565b6107c9815f01518260a001518360c001518460e0015185610100015161386c565b61020085018190526101e085018290526001600160a01b039283166101c08601819052939092166101a085018190526020868101516040888101516102408901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108599792968b9693949193929091613954565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161089c906171a5565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af11580156108f0573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061091491906171dc565b50565b6109436040518060400160405280600b81526020016a065786563757465537761760ac1b815250610d00565b61094b616efa565b610961825f015183604001518460600151610d45565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af11580156109bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109e391906171dc565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610a3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5e91906171dc565b60808201526060810151158015610a7757506080810151155b15610a9557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511015610ab057608082015160608201525b8160a0015181608001511015610acb5760a082015160808201525b610aed815f0151826060015183608001518560c001518660e001516001610d86565b6101a0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610b219161106e565b6101c082015260a081015115610bab576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610b94575f5ffd5b505af1158015610ba6573d5f5f3e3d5ffd5b505050505b60c081015115610c3057604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c19575f5ffd5b505af1158015610c2b573d5f5f3e3d5ffd5b505050505b610c418260200151825f0151612673565b606081015115610c595781518151610c599190611321565b815160208201518251610c6d929190612697565b610c8d815f0151826060015183608001518460a001518560c0015161386c565b610180850181905261016085018290526001600160a01b0392831661014086018190529390921661012085018190526020868101516101c0870151604080516080810182525f80825294810185905290810184905260608101939093526108599691958a959293627a1200939290613954565b61091460405180604001604052806002815260200161257360f01b81525082613a0b565b610d2c616f83565b5f610d38858585613a50565b915091505b935093915050565b610d4d616fa9565b5f5f610d598585613a7d565b90505f610d668783613b25565b9050610d728183614d27565b610d7b81614d55565b969095509350505050565b5f5f5f5f5f610de36040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8a118015610df0575087155b15610eca57610e008a8d89614e01565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e08201819052891115610e6f5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610e8691906172b9565b1115610ec5576101008101518151610e9e91906172b9565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b610fb3565b5f8b118015610ed7575088155b15610f9a57610ee78b8d89614eea565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e08201819052881115610f5e57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610ec55760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805160208201516040830151606084015161010090940151929f919e509c50919a5098509650505050505050565b5f83118015610fee575080155b1561103d5784516020908101510151831115611038578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b611067565b5f8411801561104a575081155b15611067578451516020015184111561106757845184905f61100b565b5050505050565b60608201515f9060481c61ffff16816110878483614fb5565b90506110938582614fd9565b9150505b92915050565b5f6110a784614ff6565b90505f8412611115578551819060ff8716600281106110c8576110c861726b565b60200201516020018181516110dd91906172b9565b9052508651819060ff8716600281106110f8576110f861726b565b602002015160600181815161110d91906172b9565b905250611176565b8551819060ff87166002811061112d5761112d61726b565b602002015160200181815161114291906172cc565b9052508651819060ff87166002811061115d5761115d61726b565b602002015160600181815161117291906172cc565b9052505b81156111f95785515f9060ff8716600281106111945761119461726b565b602002015160400151905080885f01518760ff16600281106111b8576111b861726b565b602002015160a0018181516111cd91906172cc565b90525086515f9060ff8816600281106111e8576111e861726b565b602002015160400152506113199050565b825f036112065750611319565b5f61121084614ff6565b90505f611246895f01518860ff166002811061122e5761122e61726b565b6020020151602001518361500b90919063ffffffff16565b90505f85126112b4578751819060ff8916600281106112675761126761726b565b602002015160400181815161127c91906172b9565b9052508851819060ff8916600281106112975761129761726b565b602002015160a0018181516112ac91906172b9565b905250611315565b8751819060ff8916600281106112cc576112cc61726b565b60200201516040018181516112e191906172cc565b9052508851819060ff8916600281106112fc576112fc61726b565b602002015160a00181815161131191906172cc565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f61133f8484615046565b9150915081811061136057606083015161135a9060016150a6565b60608401525b50505050565b5f6113718483617293565b90505f61138188878787876150d0565b90505f8213156113ab578651602001516113a6908261139f85614ff6565b6001615180565b6113c6565b8651602001516113c690826113bf85614ff6565b600161529e565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611414906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611464575f5ffd5b505af1158015611476573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c6261149684604001516153b1565b856040518363ffffffff1660e01b81526004016114bd929190918252602082015260400190565b5f604051808303815f87803b1580156114d4575f5ffd5b505af11580156114e6573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611524906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611554929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611595929190918252602082015260400190565b6020604051808303815f875af11580156115b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d591906171dc565b50806001600160a01b031663ca446dd984604051602001611615906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611645929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611690926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af11580156116ac573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116d091906172df565b50806001600160a01b031663ca446dd984604051602001611710906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611740929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156117a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117c791906172df565b50806001600160a01b031663e2a4853a8460405160200161180c9060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161183c929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611899573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118bd91906171dc565b50806001600160a01b031663e2a4853a846040516020016119029060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611932929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561198e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119b291906171dc565b50806001600160a01b031663e2a4853a846040516020016119fd906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a2d929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611a8a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611aae91906171dc565b50806001600160a01b031663e2a4853a84604051602001611af8906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611b28929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611b85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ba991906171dc565b50806001600160a01b031663e2a4853a84604051602001611bf5906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611c25929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ca691906171dc565b50806001600160a01b031663e2a4853a84604051602001611cf1906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d21929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d7e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611da291906171dc565b50806001600160a01b031663e2a4853a84604051602001611de1906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611e11929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e6f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e9391906171dc565b50806001600160a01b031663ca446dd984604051602001611ed3906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611f03929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f8d91906172df565b50806001600160a01b031663e2a4853a84604051602001611fd29060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001612002929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612061573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061208591906171dc565b50806001600160a01b031663e2a4853a846040516020016120ca9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016120fa929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612159573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061217d91906171dc565b50806001600160a01b031663e2a4853a846040516020016121c890602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016121f8929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612258573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061227c91906171dc565b50806001600160a01b031663e2a4853a846040516020016122c690602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016122f6929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612356573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061237a91906171dc565b50806001600160a01b031663e2a4853a846040516020016123c690602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016123f6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612456573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061247a91906171dc565b50806001600160a01b031663e2a4853a846040516020016124c590602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016124f5929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612555573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061257991906171dc565b50806001600160a01b031663e2a4853a846040516020016125b8906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016125e8929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612633929190918252602082015260400190565b6020604051808303815f875af115801561264f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061106791906171dc565b61267c81615435565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c626040516020016126d7906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612727575f5ffd5b505af1158015612739573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161277d906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016127ad929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612810573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061283491906172df565b50806001600160a01b031663e2a4853a8460405160200161287c906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016128ac929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061292d91906171dc565b50806001600160a01b031663e2a4853a84604051602001612974906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016129a4929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612a00573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a2491906171dc565b50806001600160a01b031663e2a4853a84604051602001612a70906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612aa0929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612afd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2191906171dc565b50806001600160a01b031663e2a4853a84604051602001612b41906172fa565b60405160208183030381529060405280519060200120604051602001612b71929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612bce573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf291906171dc565b50806001600160a01b031663e2a4853a84604051602001612c3f906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c6f929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612ccc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf091906171dc565b50806001600160a01b031663e2a4853a84604051602001612d39906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d69929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612dc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dea91906171dc565b50806001600160a01b031663ca446dd984604051602001612e2b906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e5b929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612ec1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ee591906172df565b50806001600160a01b031663e2a4853a84604051602001612f2d90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f5d929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612fbc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fe091906171dc565b50806001600160a01b031663e2a4853a8460405160200161302790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001613057929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156130b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130da91906171dc565b50806001600160a01b031663e2a4853a8460405160200161312690602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613156929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156131b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131da91906171dc565b50806001600160a01b031663e2a4853a846040516020016131fa9061733b565b6040516020818303038152906040528051906020012060405160200161322a929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561328a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132ae91906171dc565b50806001600160a01b031663e2a4853a846040516020016132fb90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161332b929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561338b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133af91906171dc565b50806001600160a01b031663e2a4853a846040516020016133f890602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613428929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613488573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134ac91906171dc565b50806001600160a01b031663ca446dd9846040516020016134ea90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161351a929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016135649291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613580573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135a491906172df565b50806001600160a01b031663ca446dd9846040516020016135f6906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613626929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613671926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561368d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136b191906172df565b50806001600160a01b031663e2a4853a846040516020016136f8906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613728929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613769929190918252602082015260400190565b6020604051808303815f875af1158015613785573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a991906171dc565b50806001600160a01b031663e2a4853a846040516020016137fb906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161382b929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612633929190918252602082015260400190565b5f5f5f5f6138a960405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f881180156138b6575085155b156138ed578951602090810151516001600160a01b0390811683528b5151511690820152604081018890526060810187905261392f565b5f891180156138fa575086155b1561392f57895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b1580156139ea575f5ffd5b505af11580156139fc573d5f5f3e3d5ffd5b50505050505050505050505050565b6102378282604051602401613a2192919061737c565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052615617565b613a58616f83565b5f5f613a648685615620565b90505f613a718683615686565b9050610d7b878261569f565b5f816001600160a01b0316836001600160a01b031610613a9e578183613aa1565b82825b6040519194509250613ace906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b613b2d616fa9565b82613b36616fa9565b816001600160a01b03166391d4403c604051602001613b72906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015613bc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bea91906171f3565b613bf75791506110979050565b816001600160a01b03166321f8a72185604051602001613c37906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c67929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c9b91815260200190565b602060405180830381865afa158015613cb6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cda91906172df565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613d58929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d8c91815260200190565b602060405180830381865afa158015613da7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613dcb91906171dc565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613e21906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e51929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e8591815260200190565b602060405180830381865afa158015613ea0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ec491906171dc565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613f1f906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f4f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f8391815260200190565b602060405180830381865afa158015613f9e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fc291906171dc565b815151606001526040516001600160a01b0383169063bd02d0f5908690613feb906020016172fa565b6040516020818303038152906040528051906020012060405160200161401b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161404f91815260200190565b602060405180830381865afa15801561406a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061408e91906171dc565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016140ea906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161411a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161414e91815260200190565b602060405180830381865afa158015614169573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061418d91906171dc565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161420a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161423e91815260200190565b602060405180830381865afa158015614259573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061427d91906171dc565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016142f2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161432691815260200190565b602060405180830381865afa158015614341573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061436591906172df565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161440f91815260200190565b602060405180830381865afa15801561442a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061444e91906171dc565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016144a590602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016144d5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161450991815260200190565b602060405180830381865afa158015614524573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061454891906171dc565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016145a490602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016145d4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161460891815260200190565b602060405180830381865afa158015614623573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061464791906171dc565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016146779061733b565b604051602081830303815290604052805190602001206040516020016146a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016146db91815260200190565b602060405180830381865afa1580156146f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061471a91906171dc565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161477790602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016147a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147db91815260200190565b602060405180830381865afa1580156147f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061481a91906171dc565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161487390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016148a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148d791815260200190565b602060405180830381865afa1580156148f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491691906171dc565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161496490602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001614994929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149c891815260200190565b602060405180830381865afa1580156149e3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a0791906172df565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614a75906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614aa5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ad991815260200190565b602060405180830381865afa158015614af4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b1891906172df565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001614b7b906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bdf91815260200190565b602060405180830381865afa158015614bfa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c1e91906171dc565b60608201526040516001600160a01b0383169063bd02d0f5908690614c77906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614ca7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cdb91815260200190565b602060405180830381865afa158015614cf6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d1a91906171dc565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b60808101514290819003614d67575050565b81515160a0015115614dbd5781515f90614d9090825b6020020151604001518460800151615719565b8351909150614db49082905f5b60200201516020015161574d90919063ffffffff16565b83515160200152505b81516020015160a0015115614df95781515f90614ddb906001614d7d565b8351909150614ded9082906001614d9d565b83516020908101510152505b608090910152565b5f5f5f5f614e0d616fdd565b614e18875f5f61578e565b506040840152508152614e2d8760015f61578e565b5060608401525060208201528515614e55578781602001818151614e5191906172cc565b9052505b80516020820151614e709190614e6b818c61583c565b615890565b608082018190528151614e829161594f565b60a0820152606087015160381c61ffff16610120820181905260a0820151614eb991614eb1906127109061594f565b612710615890565b6040820151606083015161012084015160a0850151614ed791614fb5565b9450945094509450505b93509350935093565b5f5f5f5f614ef6616fdd565b614f01875f5f61578e565b506040840152508152614f168760015f61578e565b5060608401525060208201528515614f3d5787815f01818151614f3991906172cc565b9052505b606087015160381c61ffff166101208201819052614f64908990614eb1906127109061594f565b610140820181905281516020830151614f8292614e6b90839061583c565b608082018190526020820151614f979161594f565b60c0820181905260408201516060830151610120840151614ed7908c905b5f81156113881983900484111517614fcb575f5ffd5b506127109102611388010490565b81515160c0018051829190614fef9083906172b9565b9052505050565b5f5f82121561500757815f03611097565b5090565b5f8115676765c793fa10079d601b1b6002840419048411171561502c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f61505784606001515f6159a4565b90505f615063866159d2565b90505f6150868261507585600a617484565b676765c793fa10079d601b1b615890565b90505f615094875f5f61578e565b50939a91995090975050505050505050565b5f6033826150b4575f6150b7565b60015b60ff16901b660800000000000019841617905092915050565b5f5f5f5f861180156150e0575083155b156150ef575083905084615107565b5f871180156150fc575084155b15610f9a5750859050825b5f61511689606001515f6159a4565b90505f6151288a6060015160016159a4565b90505f61514685676765c793fa10079d601b1b614e6b86600a617484565b90505f61516485676765c793fa10079d601b1b614e6b86600a617484565b9050615170828261500b565b9c9b505050505050505050505050565b60e0840151600119016151a757600160e085015260a0840182905260608401839052611360565b60e08401515f19016152195760a0840180519083906151c682846172b9565b9052508115615213575f6151da858561574d565b60608701516151e9908461574d565b6151f391906172b9565b905061520c8660a001518261500b90919063ffffffff16565b6060870152505b50611360565b60e084015161136057818460c00151111561524857818460c00181815161524091906172cc565b905250611360565b818460c001510361526c57600260e08501525f60c085018190526080850152611360565b600160e085015260c084015161528290836172cc565b60a0850152505060608201525f60c08201819052608090910152565b60e0840151600119016152c4575f60e085015260c0840182905260808401839052611360565b60e08401516153325760c0840180519083906152e082846172b9565b9052508115615213575f6152f4858561574d565b6080870151615303908461574d565b61530d91906172b9565b90506153268660c001518261500b90919063ffffffff16565b60808701525050611360565b60e08401515f190161136057818460a00151111561535c57818460a00181815161524091906172cc565b818460a001510361538057600260e08501525f60a085018190526060850152611360565b5f60e085015260a084015161539590836172cc565b60c0850152505060808201525f60a08201819052606090910152565b5f6040516020016153eb906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61546a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b615474845f615a96565b60208301528152606084015161548a905f6159a4565b6060820181905281516154af91676765c793fa10079d601b1b90614e6b90600a617484565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561552a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061554e91906171dc565b608082015261555e846001615a96565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa1580156155dd573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061560191906171dc565b60a0820181905260809091015194909350915050565b61091481615adc565b5f60405160200161564d906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0385169082015260608101839052608001613b07565b61568e616f83565b6156988383615afb565b9392505050565b60408101516001600160a01b03166156ca57604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f8061572583426172cc565b61572f908561748f565b6301e133809004905061109381676765c793fa10079d601b1b6172b9565b5f81156b019d971e4fe8401e74000000198390048411151761576d575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f5f5f5f875f01518760ff16600281106157ab576157ab61726b565b602002015190505f6157bd8989616d0d565b9050805f036157d9575f5f5f5f95509550955095505050614ee1565b5f6157e8838b60800151616ddf565b90505f6157f5838a61574d565b90505f891561580d57615808828461594f565b61580f565b5f5b905061581b83856172b9565b8461582685826172cc565b919a509850965094505050505093509350935093565b5f8261584883826172b9565b91508110156110975760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f838302815f1985870982811083820303915050805f036158c4578382816158ba576158ba6174a6565b0492505050615698565b8084116158e45760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8261595b83826172cc565b91508111156110975760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f60ff60581b1960585f1960ff8516016159c4575060ff60601b19905060605b90198416901c905092915050565b5f816001600160a01b031663bd02d0f5604051602001615a239060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a5791815260200190565b602060405180830381865afa158015615a72573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061109791906171dc565b5f5f5f615ac3855f01518560ff1660028110615ab457615ab461726b565b60200201518660800151616ddf565b90505f615ad08686616d0d565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b615b03616f83565b82615b0c616f83565b816001600160a01b03166391d4403c604051602001615b4c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015615ba0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615bc491906171f3565b615bd15791506110979050565b816001600160a01b031663bd02d0f585604051602001615c0b906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001615c3b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c6f91815260200190565b602060405180830381865afa158015615c8a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615cae91906171dc565b816020018181525050816001600160a01b03166321f8a72185604051602001615cf6906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615d26929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d5a91815260200190565b602060405180830381865afa158015615d75573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d9991906172df565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615df5906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615e25929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615e5991815260200190565b602060405180830381865afa158015615e74573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e9891906172df565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615f13929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615f4791815260200190565b602060405180830381865afa158015615f62573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615f8691906171dc565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001615fda9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161600a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161603e91815260200190565b602060405180830381865afa158015616059573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061607d91906171dc565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016160d7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616107929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161613b91815260200190565b602060405180830381865afa158015616156573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061617a91906171dc565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016161d3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616203929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161623791815260200190565b602060405180830381865afa158015616252573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061627691906171dc565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016162f6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161632a91815260200190565b602060405180830381865afa158015616345573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061636991906171dc565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016163c3906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016163f3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161642791815260200190565b602060405180830381865afa158015616442573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061646691906171dc565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016164d9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161650d91815260200190565b602060405180830381865afa158015616528573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061654c91906171dc565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016165c0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016165f491815260200190565b602060405180830381865afa15801561660f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061663391906172df565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016166da91815260200190565b602060405180830381865afa1580156166f5573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061671991906171dc565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161676e9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161679e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016167d291815260200190565b602060405180830381865afa1580156167ed573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061681191906171dc565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161686c90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161689c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016168d091815260200190565b602060405180830381865afa1580156168eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061690f91906171dc565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161696990602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001616999929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016169cd91815260200190565b602060405180830381865afa1580156169e8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616a0c91906171dc565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001616a6890602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001616a98929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616acc91815260200190565b602060405180830381865afa158015616ae7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616b0b91906171dc565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001616b6690602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001616b96929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616bca91815260200190565b602060405180830381865afa158015616be5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616c0991906171dc565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001616c58906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001616c88929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616cbc91815260200190565b602060405180830381865afa158015616cd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616cfb91906171dc565b81516020015160e00152949350505050565b5f5f835f01518360ff1660028110616d2757616d2761726b565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015616d80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616da491906171dc565b9050805f03616db7575f92505050611097565b606082015160c0830151616dcb82846172cc565b616dd591906172cc565b9695505050505050565b5f8260a001515f03616df257505f611097565b5f616dfd8484616e0f565b60a0850151909150611093908261574d565b5f428203616e2257506020820151611097565b5f616e31846040015184615719565b9050616e4a84602001518261574d90919063ffffffff16565b915050611097565b604051806102600160405280616e66616fa9565b81526020015f8152602001616e79616f83565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806101e00160405280616f0e616fa9565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280616f96617032565b81525f6020820181905260409091015290565b6040518060a00160405280616fbc6170a0565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061018001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b61708a6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816170415790505090565b60405180604001604052806002905b6170f16040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816170af5790505090565b6001600160a01b0381168114610914575f5ffd5b5f5f82840360c081121561712d575f5ffd5b833561713881617107565b925060a0601f198201121561714b575f5ffd5b506020830190509250929050565b5f5f82840361010081121561716c575f5ffd5b833561717781617107565b925060e0601f198201121561714b575f5ffd5b5f6020828403121561719a575f5ffd5b813561569881617107565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f602082840312156171ec575f5ffd5b5051919050565b5f60208284031215617203575f5ffd5b81518015158114615698575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061726390830184617212565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f8312801583831316838312821617156172b2576172b261727f565b5092915050565b808201808211156110975761109761727f565b818103818111156110975761109761727f565b5f602082840312156172ef575f5ffd5b815161569881617107565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61738e6040830185617212565b82810360208401526173a08185617212565b95945050505050565b6001815b6001841115610d3d578085048111156173c8576173c861727f565b60018416156173d657908102905b60019390931c9280026173ad565b5f826173f257506001611097565b816173fe57505f611097565b8160018114617414576002811461741e5761743a565b6001915050611097565b60ff84111561742f5761742f61727f565b50506001821b611097565b5060208310610133831016604e8410600b841016171561745d575081810a611097565b6174695f1984846173a9565b805f190482111561747c5761747c61727f565b029392505050565b5f61569883836173e4565b80820281158282048414176110975761109761727f565b634e487b7160e01b5f52601260045260245ffdfea2646970667358221220641396967051097bd7696f6f6d5db42e1aa552b613042670911771fee2b6667564736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qau\xFB8\x03\x80au\xFB\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qat\xF0a\x01\x0B_9_\x81\x81`\xDD\x01R\x81\x81a\x01\xC6\x01Ra\x02\xB2\x01R_\x81\x81`^\x01Ra\x05L\x01R_\x81\x81`\xB6\x01R\x81\x81a\x01\x97\x01R\x81\x81a\x02\x83\x01R\x81\x81a\x03u\x01R\x81\x81a\x04{\x01Ra\x08`\x01Rat\xF0_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aq\x1BV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04aqYV[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08^V[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x17V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90aq\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90aq\xDCV[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90aq\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90aq\xDCV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90aq\xF3V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90ar@V[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\0V[a\x06\x1CanRV[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r$V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\rEV[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x91\x86\x01Q\x91\x86\x01Qa\x06\x8B\x94\x93\x92\x90_a\r\x86V[a\x02 \x86\x01Ra\x01\x80\x85\x01Ra\x01`\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCA\x94\x92\x93\x91\x92\x90\x91a\x0F\xE1V[a\x06\xDC\x81_\x01Q\x82a\x02 \x01Qa\x10nV[a\x02@\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x0C\x93\x92\x91_\x91a\x07\x05\x91\x90ar\x93V[__a\x10\x9DV[a\x07/\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\x05\x91\x90ar\x93V[`\xA0\x81\x01Q\x15a\x07GW\x81Q\x81Qa\x07G\x91\x90a\x13!V[a\x07m\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Qa\x13fV[a\x07\x83\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x13\xD0V[a\x07\x94\x82` \x01Q\x82_\x01Qa&sV[\x81Q` \x82\x01Q\x82Qa\x07\xA8\x92\x91\x90a&\x97V[a\x07\xC9\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa8lV[a\x02\0\x85\x01\x81\x90Ra\x01\xE0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xC0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xA0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02@\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08Y\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91a9TV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x08\x9C\x90aq\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x14\x91\x90aq\xDCV[PV[a\tC`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\0V[a\tKan\xFAV[a\ta\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\rEV[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE3\x91\x90aq\xDCV[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n^\x91\x90aq\xDCV[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\nwWP`\x80\x81\x01Q\x15[\x15a\n\x95W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x10\x15a\n\xB0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x10\x15a\n\xCBW`\xA0\x82\x01Q`\x80\x82\x01R[a\n\xED\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x85`\xC0\x01Q\x86`\xE0\x01Q`\x01a\r\x86V[a\x01\xA0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B!\x91a\x10nV[a\x01\xC0\x82\x01R`\xA0\x81\x01Q\x15a\x0B\xABW`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\x94W__\xFD[PZ\xF1\x15\x80\x15a\x0B\xA6W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C0W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x19W__\xFD[PZ\xF1\x15\x80\x15a\x0C+W=__>=_\xFD[PPPP[a\x0CA\x82` \x01Q\x82_\x01Qa&sV[``\x81\x01Q\x15a\x0CYW\x81Q\x81Qa\x0CY\x91\x90a\x13!V[\x81Q` \x82\x01Q\x82Qa\x0Cm\x92\x91\x90a&\x97V[a\x0C\x8D\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa8lV[a\x01\x80\x85\x01\x81\x90Ra\x01`\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01@\x86\x01\x81\x90R\x93\x90\x92\x16a\x01 \x85\x01\x81\x90R` \x86\x81\x01Qa\x01\xC0\x87\x01Q`@\x80Q`\x80\x81\x01\x82R_\x80\x82R\x94\x81\x01\x85\x90R\x90\x81\x01\x84\x90R``\x81\x01\x93\x90\x93Ra\x08Y\x96\x91\x95\x8A\x95\x92\x93bz\x12\0\x93\x92\x90a9TV[a\t\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82a:\x0BV[a\r,ao\x83V[_a\r8\x85\x85\x85a:PV[\x91P\x91P[\x93P\x93\x91PPV[a\rMao\xA9V[__a\rY\x85\x85a:}V[\x90P_a\rf\x87\x83a;%V[\x90Pa\rr\x81\x83aM'V[a\r{\x81aMUV[\x96\x90\x95P\x93PPPPV[_____a\r\xE3`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8A\x11\x80\x15a\r\xF0WP\x87\x15[\x15a\x0E\xCAWa\x0E\0\x8A\x8D\x89aN\x01V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R\x89\x11\x15a\x0EoW\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0E\x86\x91\x90ar\xB9V[\x11\x15a\x0E\xC5Wa\x01\0\x81\x01Q\x81Qa\x0E\x9E\x91\x90ar\xB9V[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x0F\xB3V[_\x8B\x11\x80\x15a\x0E\xD7WP\x88\x15[\x15a\x0F\x9AWa\x0E\xE7\x8B\x8D\x89aN\xEAV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R\x88\x11\x15a\x0F^W` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0E\xC5W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Qa\x01\0\x90\x94\x01Q\x92\x9F\x91\x9EP\x9CP\x91\x9AP\x98P\x96PPPPPPPV[_\x83\x11\x80\x15a\x0F\xEEWP\x80\x15[\x15a\x10=W\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x108W\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x10gV[_\x84\x11\x80\x15a\x10JWP\x81\x15[\x15a\x10gW\x84QQ` \x01Q\x84\x11\x15a\x10gW\x84Q\x84\x90_a\x10\x0BV[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x10\x87\x84\x83aO\xB5V[\x90Pa\x10\x93\x85\x82aO\xD9V[\x91PP[\x92\x91PPV[_a\x10\xA7\x84aO\xF6V[\x90P_\x84\x12a\x11\x15W\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x10\xC8Wa\x10\xC8arkV[` \x02\x01Q` \x01\x81\x81Qa\x10\xDD\x91\x90ar\xB9V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x10\xF8Wa\x10\xF8arkV[` \x02\x01Q``\x01\x81\x81Qa\x11\r\x91\x90ar\xB9V[\x90RPa\x11vV[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11-Wa\x11-arkV[` \x02\x01Q` \x01\x81\x81Qa\x11B\x91\x90ar\xCCV[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11]Wa\x11]arkV[` \x02\x01Q``\x01\x81\x81Qa\x11r\x91\x90ar\xCCV[\x90RP[\x81\x15a\x11\xF9W\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x11\x94Wa\x11\x94arkV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x11\xB8Wa\x11\xB8arkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x11\xCD\x91\x90ar\xCCV[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x11\xE8Wa\x11\xE8arkV[` \x02\x01Q`@\x01RPa\x13\x19\x90PV[\x82_\x03a\x12\x06WPa\x13\x19V[_a\x12\x10\x84aO\xF6V[\x90P_a\x12F\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x12.Wa\x12.arkV[` \x02\x01Q` \x01Q\x83aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x12\xB4W\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12gWa\x12garkV[` \x02\x01Q`@\x01\x81\x81Qa\x12|\x91\x90ar\xB9V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\x97Wa\x12\x97arkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x12\xAC\x91\x90ar\xB9V[\x90RPa\x13\x15V[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\xCCWa\x12\xCCarkV[` \x02\x01Q`@\x01\x81\x81Qa\x12\xE1\x91\x90ar\xCCV[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\xFCWa\x12\xFCarkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\x11\x91\x90ar\xCCV[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x13?\x84\x84aPFV[\x91P\x91P\x81\x81\x10a\x13`W``\x83\x01Qa\x13Z\x90`\x01aP\xA6V[``\x84\x01R[PPPPV[_a\x13q\x84\x83ar\x93V[\x90P_a\x13\x81\x88\x87\x87\x87\x87aP\xD0V[\x90P_\x82\x13\x15a\x13\xABW\x86Q` \x01Qa\x13\xA6\x90\x82a\x13\x9F\x85aO\xF6V[`\x01aQ\x80V[a\x13\xC6V[\x86Q` \x01Qa\x13\xC6\x90\x82a\x13\xBF\x85aO\xF6V[`\x01aR\x9EV[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x14\x14\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14dW__\xFD[PZ\xF1\x15\x80\x15a\x14vW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x14\x96\x84`@\x01QaS\xB1V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xD4W__\xFD[PZ\xF1\x15\x80\x15a\x14\xE6W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15$\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15T\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x95\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD5\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x16\x15\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x16\x90\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\xACW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD0\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x17\x10\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\x0C\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xBD\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\x02\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x192\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xFD\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xAE\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xF8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA9\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xF5\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA6\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xF1\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\xE1\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\x11\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x93\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1E\xD3\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x8D\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\xD2\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \x02\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x85\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \xCA\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!YW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!}\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\xC8\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"|\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\xC6\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#z\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\xC6\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$z\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$\xC5\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%y\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%\xB8\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10g\x91\x90aq\xDCV[a&|\x81aT5V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a&\xD7\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a''W__\xFD[PZ\xF1\x15\x80\x15a'9W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a'}\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\xAD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(4\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(|\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)-\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)t\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*$\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*p\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xFDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+!\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+A\x90ar\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,?\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF0\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-9\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-i\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xEA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a.+\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xE5\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a/-\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/]\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xE0\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0'\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0W\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xDA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1&\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1V\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xDA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1\xFA\x90as;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xAE\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2\xFB\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xAF\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a3\xF8\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xAC\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a4\xEA\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5d\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xA4\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a5\xF6\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra6q\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xB1\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a6\xF8\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7i\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA9\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7\xFB\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a8\xA9`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a8\xB6WP\x85\x15[\x15a8\xEDW\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90Ra9/V[_\x89\x11\x80\x15a8\xFAWP\x86\x15[\x15a9/W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a9\xEAW__\xFD[PZ\xF1\x15\x80\x15a9\xFCW=__>=_\xFD[PPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01a:!\x92\x91\x90as|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaV\x17V[a:Xao\x83V[__a:d\x86\x85aV V[\x90P_a:q\x86\x83aV\x86V[\x90Pa\r{\x87\x82aV\x9FV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a:\x9EW\x81\x83a:\xA1V[\x82\x82[`@Q\x91\x94P\x92Pa:\xCE\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a;-ao\xA9V[\x82a;6ao\xA9V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a;r\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xEA\x91\x90aq\xF3V[a;\xF7W\x91Pa\x10\x97\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a<7\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<g\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x9B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xDA\x91\x90ar\xDFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=X\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x8C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xCB\x91\x90aq\xDCV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>!\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>Q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x85\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xC4\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\x1F\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?O\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x83\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xC2\x91\x90aq\xDCV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a?\xEB\x90` \x01ar\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@O\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x8E\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\xEA\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aAN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\x8D\x91\x90aq\xDCV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB>\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBYW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB}\x91\x90aq\xDCV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aCe\x91\x90ar\xDFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\x0F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDN\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aD\xA5\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEH\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xA4\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFG\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aFw\x90as;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\x1A\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aGw\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\x1A\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aHs\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x16\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aId\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x07\x91\x90ar\xDFV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aJu\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xA5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xD9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x18\x91\x90ar\xDFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK{\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\x1E\x91\x90aq\xDCV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aLw\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x1A\x91\x90aq\xDCV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aMgWPPV[\x81QQ`\xA0\x01Q\x15aM\xBDW\x81Q_\x90aM\x90\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaW\x19V[\x83Q\x90\x91PaM\xB4\x90\x82\x90_[` \x02\x01Q` \x01QaWM\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aM\xF9W\x81Q_\x90aM\xDB\x90`\x01aM}V[\x83Q\x90\x91PaM\xED\x90\x82\x90`\x01aM\x9DV[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aN\rao\xDDV[aN\x18\x87__aW\x8EV[P`@\x84\x01RP\x81RaN-\x87`\x01_aW\x8EV[P``\x84\x01RP` \x82\x01R\x85\x15aNUW\x87\x81` \x01\x81\x81QaNQ\x91\x90ar\xCCV[\x90RP[\x80Q` \x82\x01QaNp\x91\x90aNk\x81\x8CaX<V[aX\x90V[`\x80\x82\x01\x81\x90R\x81QaN\x82\x91aYOV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01QaN\xB9\x91aN\xB1\x90a'\x10\x90aYOV[a'\x10aX\x90V[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01QaN\xD7\x91aO\xB5V[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____aN\xF6ao\xDDV[aO\x01\x87__aW\x8EV[P`@\x84\x01RP\x81RaO\x16\x87`\x01_aW\x8EV[P``\x84\x01RP` \x82\x01R\x85\x15aO=W\x87\x81_\x01\x81\x81QaO9\x91\x90ar\xCCV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90RaOd\x90\x89\x90aN\xB1\x90a'\x10\x90aYOV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01QaO\x82\x92aNk\x90\x83\x90aX<V[`\x80\x82\x01\x81\x90R` \x82\x01QaO\x97\x91aYOV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01QaN\xD7\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aO\xCBW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aO\xEF\x90\x83\x90ar\xB9V[\x90RPPPV[__\x82\x12\x15aP\x07W\x81_\x03a\x10\x97V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aP,W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aPW\x84``\x01Q_aY\xA4V[\x90P_aPc\x86aY\xD2V[\x90P_aP\x86\x82aPu\x85`\nat\x84V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaX\x90V[\x90P_aP\x94\x87__aW\x8EV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aP\xB4W_aP\xB7V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[____\x86\x11\x80\x15aP\xE0WP\x83\x15[\x15aP\xEFWP\x83\x90P\x84aQ\x07V[_\x87\x11\x80\x15aP\xFCWP\x84\x15[\x15a\x0F\x9AWP\x85\x90P\x82[_aQ\x16\x89``\x01Q_aY\xA4V[\x90P_aQ(\x8A``\x01Q`\x01aY\xA4V[\x90P_aQF\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaNk\x86`\nat\x84V[\x90P_aQd\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaNk\x86`\nat\x84V[\x90PaQp\x82\x82aP\x0BV[\x9C\x9BPPPPPPPPPPPPV[`\xE0\x84\x01Q`\x01\x19\x01aQ\xA7W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x13`V[`\xE0\x84\x01Q_\x19\x01aR\x19W`\xA0\x84\x01\x80Q\x90\x83\x90aQ\xC6\x82\x84ar\xB9V[\x90RP\x81\x15aR\x13W_aQ\xDA\x85\x85aWMV[``\x87\x01QaQ\xE9\x90\x84aWMV[aQ\xF3\x91\x90ar\xB9V[\x90PaR\x0C\x86`\xA0\x01Q\x82aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x13`V[`\xE0\x84\x01Qa\x13`W\x81\x84`\xC0\x01Q\x11\x15aRHW\x81\x84`\xC0\x01\x81\x81QaR@\x91\x90ar\xCCV[\x90RPa\x13`V[\x81\x84`\xC0\x01Q\x03aRlW`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x13`V[`\x01`\xE0\x85\x01R`\xC0\x84\x01QaR\x82\x90\x83ar\xCCV[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01aR\xC4W_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x13`V[`\xE0\x84\x01QaS2W`\xC0\x84\x01\x80Q\x90\x83\x90aR\xE0\x82\x84ar\xB9V[\x90RP\x81\x15aR\x13W_aR\xF4\x85\x85aWMV[`\x80\x87\x01QaS\x03\x90\x84aWMV[aS\r\x91\x90ar\xB9V[\x90PaS&\x86`\xC0\x01Q\x82aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x13`V[`\xE0\x84\x01Q_\x19\x01a\x13`W\x81\x84`\xA0\x01Q\x11\x15aS\\W\x81\x84`\xA0\x01\x81\x81QaR@\x91\x90ar\xCCV[\x81\x84`\xA0\x01Q\x03aS\x80W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x13`V[_`\xE0\x85\x01R`\xA0\x84\x01QaS\x95\x90\x83ar\xCCV[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01aS\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__aTj`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aTt\x84_aZ\x96V[` \x83\x01R\x81R``\x84\x01QaT\x8A\x90_aY\xA4V[``\x82\x01\x81\x90R\x81QaT\xAF\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aNk\x90`\nat\x84V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aUN\x91\x90aq\xDCV[`\x80\x82\x01RaU^\x84`\x01aZ\x96V[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x01\x91\x90aq\xDCV[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[a\t\x14\x81aZ\xDCV[_`@Q` \x01aVM\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01a;\x07V[aV\x8Eao\x83V[aV\x98\x83\x83aZ\xFBV[\x93\x92PPPV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aV\xCAW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80aW%\x83Bar\xCCV[aW/\x90\x85at\x8FV[c\x01\xE13\x80\x90\x04\x90Pa\x10\x93\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bar\xB9V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aWmW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10aW\xABWaW\xABarkV[` \x02\x01Q\x90P_aW\xBD\x89\x89am\rV[\x90P\x80_\x03aW\xD9W____\x95P\x95P\x95P\x95PPPaN\xE1V[_aW\xE8\x83\x8B`\x80\x01Qam\xDFV[\x90P_aW\xF5\x83\x8AaWMV[\x90P_\x89\x15aX\rWaX\x08\x82\x84aYOV[aX\x0FV[_[\x90PaX\x1B\x83\x85ar\xB9V[\x84aX&\x85\x82ar\xCCV[\x91\x9AP\x98P\x96P\x94PPPPP\x93P\x93P\x93P\x93V[_\x82aXH\x83\x82ar\xB9V[\x91P\x81\x10\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aX\xC4W\x83\x82\x81aX\xBAWaX\xBAat\xA6V[\x04\x92PPPaV\x98V[\x80\x84\x11aX\xE4W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x82aY[\x83\x82ar\xCCV[\x91P\x81\x11\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aY\xC4WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aZ#\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZW\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x97\x91\x90aq\xDCV[___aZ\xC3\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aZ\xB4WaZ\xB4arkV[` \x02\x01Q\x86`\x80\x01Qam\xDFV[\x90P_aZ\xD0\x86\x86am\rV[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[a[\x03ao\x83V[\x82a[\x0Cao\x83V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a[L\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\xC4\x91\x90aq\xF3V[a[\xD1W\x91Pa\x10\x97\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\\\x0B\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\o\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\\xAE\x91\x90aq\xDCV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\\\xF6\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a]&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]Z\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]uW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]\x99\x91\x90ar\xDFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a]\xF5\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a^%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a^Y\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^\x98\x91\x90ar\xDFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a_G\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_\x86\x91\x90aq\xDCV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a_\xDA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a`\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a`>\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`YW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`}\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a`\xD7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa\x07\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aa;\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aaVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aaz\x91\x90aq\xDCV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aa\xD3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ab7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15abRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90abv\x91\x90aq\xDCV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ac*\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15acEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aci\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ac\xC3\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ac\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ad'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15adBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90adf\x91\x90aq\xDCV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ad\xD9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ae\r\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ae(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aeL\x91\x90aq\xDCV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ae\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ae\xF4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15af\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90af3\x91\x90ar\xDFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01af\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15af\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ag\x19\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01agn\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ag\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ag\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ag\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ah\x11\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ahl\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ah\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ah\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ah\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ai\x0F\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aii\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ai\x99\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ai\xCD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ai\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aj\x0C\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ajh\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aj\x98\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\xCC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ak\x0B\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01akf\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ak\x96\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ak\xCA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\t\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01alX\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al\x88\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\xBC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\xFB\x91\x90aq\xDCV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10am'Wam'arkV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am\xA4\x91\x90aq\xDCV[\x90P\x80_\x03am\xB7W_\x92PPPa\x10\x97V[``\x82\x01Q`\xC0\x83\x01Qam\xCB\x82\x84ar\xCCV[am\xD5\x91\x90ar\xCCV[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03am\xF2WP_a\x10\x97V[_am\xFD\x84\x84an\x0FV[`\xA0\x85\x01Q\x90\x91Pa\x10\x93\x90\x82aWMV[_B\x82\x03an\"WP` \x82\x01Qa\x10\x97V[_an1\x84`@\x01Q\x84aW\x19V[\x90PanJ\x84` \x01Q\x82aWM\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x10\x97V[`@Q\x80a\x02`\x01`@R\x80anfao\xA9V[\x81R` \x01_\x81R` \x01anyao\x83V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80ao\x0Eao\xA9V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ao\x96ap2V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80ao\xBCap\xA0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ap\x8A`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81apAW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ap\xF1`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ap\xAFW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x14W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15aq-W__\xFD[\x835aq8\x81aq\x07V[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15aqKW__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15aqlW__\xFD[\x835aqw\x81aq\x07V[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15aqKW__\xFD[_` \x82\x84\x03\x12\x15aq\x9AW__\xFD[\x815aV\x98\x81aq\x07V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aq\xECW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ar\x03W__\xFD[\x81Q\x80\x15\x15\x81\x14aV\x98W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90arc\x90\x83\x01\x84ar\x12V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15ar\xB2War\xB2ar\x7FV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x10\x97Wa\x10\x97ar\x7FV[\x81\x81\x03\x81\x81\x11\x15a\x10\x97Wa\x10\x97ar\x7FV[_` \x82\x84\x03\x12\x15ar\xEFW__\xFD[\x81QaV\x98\x81aq\x07V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_as\x8E`@\x83\x01\x85ar\x12V[\x82\x81\x03` \x84\x01Ras\xA0\x81\x85ar\x12V[\x95\x94PPPPPV[`\x01\x81[`\x01\x84\x11\x15a\r=W\x80\x85\x04\x81\x11\x15as\xC8Was\xC8ar\x7FV[`\x01\x84\x16\x15as\xD6W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02as\xADV[_\x82as\xF2WP`\x01a\x10\x97V[\x81as\xFEWP_a\x10\x97V[\x81`\x01\x81\x14at\x14W`\x02\x81\x14at\x1EWat:V[`\x01\x91PPa\x10\x97V[`\xFF\x84\x11\x15at/Wat/ar\x7FV[PP`\x01\x82\x1Ba\x10\x97V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15at]WP\x81\x81\na\x10\x97V[ati_\x19\x84\x84as\xA9V[\x80_\x19\x04\x82\x11\x15at|Wat|ar\x7FV[\x02\x93\x92PPPV[_aV\x98\x83\x83as\xE4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\x97Wa\x10\x97ar\x7FV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 d\x13\x96\x96pQ\t{\xD7ioom]\xB4.\x1A\xA5R\xB6\x13\x04&p\x91\x17q\xFE\xE2\xB6fudsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80634a4a7b041461005957806352b5de3d1461009c578063660d0d67146100b15780639ff78c30146100d8578063d9c42742146100ff575b5f5ffd5b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100af6100aa36600461711b565b610112565b005b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100807f000000000000000000000000000000000000000000000000000000000000000081565b6100af61010d366004617159565b61023b565b61011a610372565b61018960405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610530565b5f6040518060e001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200135815260200183604001358152602001836060013581526020018360800135815250905061022e83826105de565b5061023761085e565b5050565b610243610372565b61027460405160200161014b906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f6040518061012001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f0160208101906102f1919061718a565b6001600160a01b03168152602001836020016020810190610312919061718a565b6001600160a01b031681526020018360400135815260200183606001358152602001836080013581526020018360a0013581526020018360c001602081019061035b919061718a565b6001600160a01b03169052905061022e8382610917565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016103b1906171a5565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016103e591815260200190565b602060405180830381865afa158015610400573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061042491906171dc565b905080156104795760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016104b7906171a5565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af115801561050c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061023791906171dc565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610599573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105bd91906171f3565b61023757338160405163a35b150b60e01b8152600401610470929190617240565b6106146040518060400160405280601581526020017432bc32b1baba32a9bbb0b824b72837b9b4ba34b7b760591b815250610d00565b61061c616e52565b61062e83835f01518460400151610d24565b606083015260408201819052825190518051516020919091015151610654929190610d45565b6020830152808252606083015160a0808401829052608085015160c0808601829052918601519186015161068b949392905f610d86565b610220860152610180850152610160840152610100830181905260e08301829052604083015160a084015160c08501516106ca94929391929091610fe1565b6106dc815f015182610220015161106e565b6102408201528051604082015160a083015160e084015161070c9392915f916107059190617293565b5f5f61109d565b61072f815f0151826040015160018460c001518561010001516107059190617293565b60a08101511561074757815181516107479190611321565b61076d815f015182604001518360a001518460c001518560e00151866101000151611366565b610783825f0151826060015183604001516113d0565b6107948260200151825f0151612673565b8151602082015182516107a8929190612697565b6107c9815f01518260a001518360c001518460e0015185610100015161386c565b61020085018190526101e085018290526001600160a01b039283166101c08601819052939092166101a085018190526020868101516040888101516102408901518251608081018452838b018051515187015182528051515185015182880152805151870151870151828601525151909501519092015160608501526108599792968b9693949193929091613954565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161089c906171a5565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af11580156108f0573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061091491906171dc565b50565b6109436040518060400160405280600b81526020016a065786563757465537761760ac1b815250610d00565b61094b616efa565b610961825f015183604001518460600151610d45565b60208381019190915281835201516001600160a01b03908116604080840182905284810151905163352f9aed60e01b8152921660048301529063352f9aed906024016020604051808303815f875af11580156109bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109e391906171dc565b60608083019190915260408083015191840151905163352f9aed60e01b81526001600160a01b03918216600482015291169063352f9aed906024016020604051808303815f875af1158015610a3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5e91906171dc565b60808201526060810151158015610a7757506080810151155b15610a9557604051637c9c0d7d60e11b815260040160405180910390fd5b816080015181606001511015610ab057608082015160608201525b8160a0015181608001511015610acb5760a082015160808201525b610aed815f0151826060015183608001518560c001518660e001516001610d86565b6101a0860181905261010086019190915260e085019190915260c084019190915260a08301919091528151610b219161106e565b6101c082015260a081015115610bab576040808201518382015161010085015160a0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610b94575f5ffd5b505af1158015610ba6573d5f5f3e3d5ffd5b505050505b60c081015115610c3057604080820151606084015161010085015160c0850151935163078d3b7960e01b81526001600160a01b03928316600482015290821660248201526044810193909352169063078d3b79906064015f604051808303815f87803b158015610c19575f5ffd5b505af1158015610c2b573d5f5f3e3d5ffd5b505050505b610c418260200151825f0151612673565b606081015115610c595781518151610c599190611321565b815160208201518251610c6d929190612697565b610c8d815f0151826060015183608001518460a001518560c0015161386c565b610180850181905261016085018290526001600160a01b0392831661014086018190529390921661012085018190526020868101516101c0870151604080516080810182525f80825294810185905290810184905260608101939093526108599691958a959293627a1200939290613954565b61091460405180604001604052806002815260200161257360f01b81525082613a0b565b610d2c616f83565b5f610d38858585613a50565b915091505b935093915050565b610d4d616fa9565b5f5f610d598585613a7d565b90505f610d668783613b25565b9050610d728183614d27565b610d7b81614d55565b969095509350505050565b5f5f5f5f5f610de36040518061012001604052805f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81525090565b5f8a118015610df0575087155b15610eca57610e008a8d89614e01565b610100850152606084015260408301528082528c5160200151516001600160a01b0390811660808401528d5151511660a083015260c082018b905260e08201819052891115610e6f5780516040516367878ac160e11b8152610470918b91600401918252602082015260400190565b60408101516101008201518251610e8691906172b9565b1115610ec5576101008101518151610e9e91906172b9565b6040808301519051631fc107c160e01b815260048101929092526024820152604401610470565b610fb3565b5f8b118015610ed7575088155b15610f9a57610ee78b8d89614eea565b6101008501526060840152604083015260208083018290528d5151516001600160a01b0390811660808501528e5190910151511660a083015260c082018c905260e08201819052881115610f5e57602081015160405163750eb44960e11b8152610470918a91600401918252602082015260400190565b806060015181602001511115610ec55760208101516060820151604051630e793baf60e01b815260048101929092526024820152604401610470565b604051636331fab160e01b815260040160405180910390fd5b805160208201516040830151606084015161010090940151929f919e509c50919a5098509650505050505050565b5f83118015610fee575080155b1561103d5784516020908101510151831115611038578451839060015b60200201516020015160405163671abd0760e01b8152600401610470929190918252602082015260400190565b611067565b5f8411801561104a575081155b15611067578451516020015184111561106757845184905f61100b565b5050505050565b60608201515f9060481c61ffff16816110878483614fb5565b90506110938582614fd9565b9150505b92915050565b5f6110a784614ff6565b90505f8412611115578551819060ff8716600281106110c8576110c861726b565b60200201516020018181516110dd91906172b9565b9052508651819060ff8716600281106110f8576110f861726b565b602002015160600181815161110d91906172b9565b905250611176565b8551819060ff87166002811061112d5761112d61726b565b602002015160200181815161114291906172cc565b9052508651819060ff87166002811061115d5761115d61726b565b602002015160600181815161117291906172cc565b9052505b81156111f95785515f9060ff8716600281106111945761119461726b565b602002015160400151905080885f01518760ff16600281106111b8576111b861726b565b602002015160a0018181516111cd91906172cc565b90525086515f9060ff8816600281106111e8576111e861726b565b602002015160400152506113199050565b825f036112065750611319565b5f61121084614ff6565b90505f611246895f01518860ff166002811061122e5761122e61726b565b6020020151602001518361500b90919063ffffffff16565b90505f85126112b4578751819060ff8916600281106112675761126761726b565b602002015160400181815161127c91906172b9565b9052508851819060ff8916600281106112975761129761726b565b602002015160a0018181516112ac91906172b9565b905250611315565b8751819060ff8916600281106112cc576112cc61726b565b60200201516040018181516112e191906172cc565b9052508851819060ff8916600281106112fc576112fc61726b565b602002015160a00181815161131191906172cc565b9052505b5050505b505050505050565b6060810151660800000000000016610237575f5f61133f8484615046565b9150915081811061136057606083015161135a9060016150a6565b60608401525b50505050565b5f6113718483617293565b90505f61138188878787876150d0565b90505f8213156113ab578651602001516113a6908261139f85614ff6565b6001615180565b6113c6565b8651602001516113c690826113bf85614ff6565b600161529e565b5050505050505050565b5f839050806001600160a01b031663c80f4c62604051602001611414906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611464575f5ffd5b505af1158015611476573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c6261149684604001516153b1565b856040518363ffffffff1660e01b81526004016114bd929190918252602082015260400190565b5f604051808303815f87803b1580156114d4575f5ffd5b505af11580156114e6573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001611524906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001611554929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401611595929190918252602082015260400190565b6020604051808303815f875af11580156115b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d591906171dc565b50806001600160a01b031663ca446dd984604051602001611615906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611645929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352611690926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af11580156116ac573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116d091906172df565b50806001600160a01b031663ca446dd984604051602001611710906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611740929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156117a3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117c791906172df565b50806001600160a01b031663e2a4853a8460405160200161180c9060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161183c929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611899573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118bd91906171dc565b50806001600160a01b031663e2a4853a846040516020016119029060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611932929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561198e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119b291906171dc565b50806001600160a01b031663e2a4853a846040516020016119fd906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a2d929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611a8a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611aae91906171dc565b50806001600160a01b031663e2a4853a84604051602001611af8906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611b28929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611b85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ba991906171dc565b50806001600160a01b031663e2a4853a84604051602001611bf5906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611c25929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ca691906171dc565b50806001600160a01b031663e2a4853a84604051602001611cf1906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d21929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611d7e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611da291906171dc565b50806001600160a01b031663e2a4853a84604051602001611de1906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611e11929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611e6f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e9391906171dc565b50806001600160a01b031663ca446dd984604051602001611ed3906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611f03929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f8d91906172df565b50806001600160a01b031663e2a4853a84604051602001611fd29060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001612002929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612061573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061208591906171dc565b50806001600160a01b031663e2a4853a846040516020016120ca9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016120fa929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612159573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061217d91906171dc565b50806001600160a01b031663e2a4853a846040516020016121c890602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016121f8929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612258573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061227c91906171dc565b50806001600160a01b031663e2a4853a846040516020016122c690602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016122f6929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612356573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061237a91906171dc565b50806001600160a01b031663e2a4853a846040516020016123c690602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016123f6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612456573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061247a91906171dc565b50806001600160a01b031663e2a4853a846040516020016124c590602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016124f5929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612555573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061257991906171dc565b50806001600160a01b031663e2a4853a846040516020016125b8906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016125e8929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401612633929190918252602082015260400190565b6020604051808303815f875af115801561264f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061106791906171dc565b61267c81615435565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c626040516020016126d7906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015612727575f5ffd5b505af1158015612739573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd98460405160200161277d906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016127ad929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612810573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061283491906172df565b50806001600160a01b031663e2a4853a8460405160200161287c906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016128ac929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612909573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061292d91906171dc565b50806001600160a01b031663e2a4853a84604051602001612974906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016129a4929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612a00573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a2491906171dc565b50806001600160a01b031663e2a4853a84604051602001612a70906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001612aa0929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612afd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2191906171dc565b50806001600160a01b031663e2a4853a84604051602001612b41906172fa565b60405160208183030381529060405280519060200120604051602001612b71929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612bce573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf291906171dc565b50806001600160a01b031663e2a4853a84604051602001612c3f906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c6f929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612ccc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf091906171dc565b50806001600160a01b031663e2a4853a84604051602001612d39906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d69929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612dc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dea91906171dc565b50806001600160a01b031663ca446dd984604051602001612e2b906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e5b929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612ec1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ee591906172df565b50806001600160a01b031663e2a4853a84604051602001612f2d90602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001612f5d929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612fbc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fe091906171dc565b50806001600160a01b031663e2a4853a8460405160200161302790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001613057929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156130b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130da91906171dc565b50806001600160a01b031663e2a4853a8460405160200161312690602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613156929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156131b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131da91906171dc565b50806001600160a01b031663e2a4853a846040516020016131fa9061733b565b6040516020818303038152906040528051906020012060405160200161322a929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561328a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132ae91906171dc565b50806001600160a01b031663e2a4853a846040516020016132fb90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161332b929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561338b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133af91906171dc565b50806001600160a01b031663e2a4853a846040516020016133f890602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613428929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015613488573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134ac91906171dc565b50806001600160a01b031663ca446dd9846040516020016134ea90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161351a929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016135649291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015613580573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135a491906172df565b50806001600160a01b031663ca446dd9846040516020016135f6906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613626929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352613671926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af115801561368d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136b191906172df565b50806001600160a01b031663e2a4853a846040516020016136f8906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001613728929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401613769929190918252602082015260400190565b6020604051808303815f875af1158015613785573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a991906171dc565b50806001600160a01b031663e2a4853a846040516020016137fb906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161382b929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612633929190918252602082015260400190565b5f5f5f5f6138a960405180608001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81525090565b5f881180156138b6575085155b156138ed578951602090810151516001600160a01b0390811683528b5151511690820152604081018890526060810187905261392f565b5f891180156138fa575086155b1561392f57895151516001600160a01b0390811682528a51602090810151519091169082015260408101899052606081018690525b805160208201516040830151606090930151919c909b50919950975095505050505050565b604080516311ccb21d60e01b81526001600160a01b038a8116600483015289811660248301528881166044830152606482018890526084820187905260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a16906311ccb21d90610164015f604051808303815f87803b1580156139ea575f5ffd5b505af11580156139fc573d5f5f3e3d5ffd5b50505050505050505050505050565b6102378282604051602401613a2192919061737c565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052615617565b613a58616f83565b5f5f613a648685615620565b90505f613a718683615686565b9050610d7b878261569f565b5f816001600160a01b0316836001600160a01b031610613a9e578183613aa1565b82825b6040519194509250613ace906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b60405160208183030381529060405280519060200120905092915050565b613b2d616fa9565b82613b36616fa9565b816001600160a01b03166391d4403c604051602001613b72906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015613bc6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bea91906171f3565b613bf75791506110979050565b816001600160a01b03166321f8a72185604051602001613c37906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c67929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c9b91815260200190565b602060405180830381865afa158015613cb6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cda91906172df565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613d58929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d8c91815260200190565b602060405180830381865afa158015613da7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613dcb91906171dc565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613e21906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e51929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e8591815260200190565b602060405180830381865afa158015613ea0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613ec491906171dc565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613f1f906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f4f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f8391815260200190565b602060405180830381865afa158015613f9e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fc291906171dc565b815151606001526040516001600160a01b0383169063bd02d0f5908690613feb906020016172fa565b6040516020818303038152906040528051906020012060405160200161401b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161404f91815260200190565b602060405180830381865afa15801561406a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061408e91906171dc565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016140ea906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161411a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161414e91815260200190565b602060405180830381865afa158015614169573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061418d91906171dc565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161420a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161423e91815260200190565b602060405180830381865afa158015614259573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061427d91906171dc565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016142f2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161432691815260200190565b602060405180830381865afa158015614341573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061436591906172df565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161440f91815260200190565b602060405180830381865afa15801561442a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061444e91906171dc565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016144a590602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016144d5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161450991815260200190565b602060405180830381865afa158015614524573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061454891906171dc565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016145a490602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016145d4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161460891815260200190565b602060405180830381865afa158015614623573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061464791906171dc565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016146779061733b565b604051602081830303815290604052805190602001206040516020016146a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016146db91815260200190565b602060405180830381865afa1580156146f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061471a91906171dc565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161477790602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b604051602081830303815290604052805190602001206040516020016147a7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147db91815260200190565b602060405180830381865afa1580156147f6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061481a91906171dc565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161487390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016148a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148d791815260200190565b602060405180830381865afa1580156148f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491691906171dc565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161496490602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001614994929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149c891815260200190565b602060405180830381865afa1580156149e3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a0791906172df565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614a75906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614aa5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ad991815260200190565b602060405180830381865afa158015614af4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b1891906172df565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001614b7b906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bdf91815260200190565b602060405180830381865afa158015614bfa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c1e91906171dc565b60608201526040516001600160a01b0383169063bd02d0f5908690614c77906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614ca7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cdb91815260200190565b602060405180830381865afa158015614cf6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d1a91906171dc565b6080820152949350505050565b60208201516001600160a01b031661023757604051637357d91f60e01b815260048101829052602401610470565b60808101514290819003614d67575050565b81515160a0015115614dbd5781515f90614d9090825b6020020151604001518460800151615719565b8351909150614db49082905f5b60200201516020015161574d90919063ffffffff16565b83515160200152505b81516020015160a0015115614df95781515f90614ddb906001614d7d565b8351909150614ded9082906001614d9d565b83516020908101510152505b608090910152565b5f5f5f5f614e0d616fdd565b614e18875f5f61578e565b506040840152508152614e2d8760015f61578e565b5060608401525060208201528515614e55578781602001818151614e5191906172cc565b9052505b80516020820151614e709190614e6b818c61583c565b615890565b608082018190528151614e829161594f565b60a0820152606087015160381c61ffff16610120820181905260a0820151614eb991614eb1906127109061594f565b612710615890565b6040820151606083015161012084015160a0850151614ed791614fb5565b9450945094509450505b93509350935093565b5f5f5f5f614ef6616fdd565b614f01875f5f61578e565b506040840152508152614f168760015f61578e565b5060608401525060208201528515614f3d5787815f01818151614f3991906172cc565b9052505b606087015160381c61ffff166101208201819052614f64908990614eb1906127109061594f565b610140820181905281516020830151614f8292614e6b90839061583c565b608082018190526020820151614f979161594f565b60c0820181905260408201516060830151610120840151614ed7908c905b5f81156113881983900484111517614fcb575f5ffd5b506127109102611388010490565b81515160c0018051829190614fef9083906172b9565b9052505050565b5f5f82121561500757815f03611097565b5090565b5f8115676765c793fa10079d601b1b6002840419048411171561502c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f61505784606001515f6159a4565b90505f615063866159d2565b90505f6150868261507585600a617484565b676765c793fa10079d601b1b615890565b90505f615094875f5f61578e565b50939a91995090975050505050505050565b5f6033826150b4575f6150b7565b60015b60ff16901b660800000000000019841617905092915050565b5f5f5f5f861180156150e0575083155b156150ef575083905084615107565b5f871180156150fc575084155b15610f9a5750859050825b5f61511689606001515f6159a4565b90505f6151288a6060015160016159a4565b90505f61514685676765c793fa10079d601b1b614e6b86600a617484565b90505f61516485676765c793fa10079d601b1b614e6b86600a617484565b9050615170828261500b565b9c9b505050505050505050505050565b60e0840151600119016151a757600160e085015260a0840182905260608401839052611360565b60e08401515f19016152195760a0840180519083906151c682846172b9565b9052508115615213575f6151da858561574d565b60608701516151e9908461574d565b6151f391906172b9565b905061520c8660a001518261500b90919063ffffffff16565b6060870152505b50611360565b60e084015161136057818460c00151111561524857818460c00181815161524091906172cc565b905250611360565b818460c001510361526c57600260e08501525f60c085018190526080850152611360565b600160e085015260c084015161528290836172cc565b60a0850152505060608201525f60c08201819052608090910152565b60e0840151600119016152c4575f60e085015260c0840182905260808401839052611360565b60e08401516153325760c0840180519083906152e082846172b9565b9052508115615213575f6152f4858561574d565b6080870151615303908461574d565b61530d91906172b9565b90506153268660c001518261500b90919063ffffffff16565b60808701525050611360565b60e08401515f190161136057818460a00151111561535c57818460a00181815161524091906172cc565b818460a001510361538057600260e08501525f60a085018190526060850152611360565b5f60e085015260a084015161539590836172cc565b60c0850152505060808201525f60a08201819052606090910152565b5f6040516020016153eb906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61546a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b615474845f615a96565b60208301528152606084015161548a905f6159a4565b6060820181905281516154af91676765c793fa10079d601b1b90614e6b90600a617484565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561552a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061554e91906171dc565b608082015261555e846001615a96565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa1580156155dd573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061560191906171dc565b60a0820181905260809091015194909350915050565b61091481615adc565b5f60405160200161564d906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0385169082015260608101839052608001613b07565b61568e616f83565b6156988383615afb565b9392505050565b60408101516001600160a01b03166156ca57604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102375760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610470565b5f8061572583426172cc565b61572f908561748f565b6301e133809004905061109381676765c793fa10079d601b1b6172b9565b5f81156b019d971e4fe8401e74000000198390048411151761576d575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f5f5f5f875f01518760ff16600281106157ab576157ab61726b565b602002015190505f6157bd8989616d0d565b9050805f036157d9575f5f5f5f95509550955095505050614ee1565b5f6157e8838b60800151616ddf565b90505f6157f5838a61574d565b90505f891561580d57615808828461594f565b61580f565b5f5b905061581b83856172b9565b8461582685826172cc565b919a509850965094505050505093509350935093565b5f8261584883826172b9565b91508110156110975760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610470565b5f838302815f1985870982811083820303915050805f036158c4578382816158ba576158ba6174a6565b0492505050615698565b8084116158e45760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8261595b83826172cc565b91508111156110975760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610470565b5f60ff60581b1960585f1960ff8516016159c4575060ff60601b19905060605b90198416901c905092915050565b5f816001600160a01b031663bd02d0f5604051602001615a239060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a5791815260200190565b602060405180830381865afa158015615a72573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061109791906171dc565b5f5f5f615ac3855f01518560ff1660028110615ab457615ab461726b565b60200201518660800151616ddf565b90505f615ad08686616d0d565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b615b03616f83565b82615b0c616f83565b816001600160a01b03166391d4403c604051602001615b4c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015615ba0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615bc491906171f3565b615bd15791506110979050565b816001600160a01b031663bd02d0f585604051602001615c0b906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001615c3b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c6f91815260200190565b602060405180830381865afa158015615c8a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615cae91906171dc565b816020018181525050816001600160a01b03166321f8a72185604051602001615cf6906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615d26929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d5a91815260200190565b602060405180830381865afa158015615d75573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d9991906172df565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615df5906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615e25929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615e5991815260200190565b602060405180830381865afa158015615e74573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e9891906172df565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615f13929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615f4791815260200190565b602060405180830381865afa158015615f62573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615f8691906171dc565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001615fda9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161600a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161603e91815260200190565b602060405180830381865afa158015616059573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061607d91906171dc565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016160d7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001616107929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161613b91815260200190565b602060405180830381865afa158015616156573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061617a91906171dc565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016161d3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001616203929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161623791815260200190565b602060405180830381865afa158015616252573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061627691906171dc565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016162f6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161632a91815260200190565b602060405180830381865afa158015616345573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061636991906171dc565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016163c3906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016163f3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161642791815260200190565b602060405180830381865afa158015616442573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061646691906171dc565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016164d9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161650d91815260200190565b602060405180830381865afa158015616528573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061654c91906171dc565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016165c0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016165f491815260200190565b602060405180830381865afa15801561660f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061663391906172df565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016166da91815260200190565b602060405180830381865afa1580156166f5573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061671991906171dc565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161676e9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161679e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016167d291815260200190565b602060405180830381865afa1580156167ed573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061681191906171dc565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161686c90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161689c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016168d091815260200190565b602060405180830381865afa1580156168eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061690f91906171dc565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161696990602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001616999929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016169cd91815260200190565b602060405180830381865afa1580156169e8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616a0c91906171dc565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001616a6890602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001616a98929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616acc91815260200190565b602060405180830381865afa158015616ae7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616b0b91906171dc565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001616b6690602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001616b96929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616bca91815260200190565b602060405180830381865afa158015616be5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616c0991906171dc565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001616c58906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001616c88929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401616cbc91815260200190565b602060405180830381865afa158015616cd7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616cfb91906171dc565b81516020015160e00152949350505050565b5f5f835f01518360ff1660028110616d2757616d2761726b565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015616d80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616da491906171dc565b9050805f03616db7575f92505050611097565b606082015160c0830151616dcb82846172cc565b616dd591906172cc565b9695505050505050565b5f8260a001515f03616df257505f611097565b5f616dfd8484616e0f565b60a0850151909150611093908261574d565b5f428203616e2257506020820151611097565b5f616e31846040015184615719565b9050616e4a84602001518261574d90919063ffffffff16565b915050611097565b604051806102600160405280616e66616fa9565b81526020015f8152602001616e79616f83565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b604051806101e00160405280616f0e616fa9565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280616f96617032565b81525f6020820181905260409091015290565b6040518060a00160405280616fbc6170a0565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061018001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806002905b61708a6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816170415790505090565b60405180604001604052806002905b6170f16040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816170af5790505090565b6001600160a01b0381168114610914575f5ffd5b5f5f82840360c081121561712d575f5ffd5b833561713881617107565b925060a0601f198201121561714b575f5ffd5b506020830190509250929050565b5f5f82840361010081121561716c575f5ffd5b833561717781617107565b925060e0601f198201121561714b575f5ffd5b5f6020828403121561719a575f5ffd5b813561569881617107565b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f602082840312156171ec575f5ffd5b5051919050565b5f60208284031215617203575f5ffd5b81518015158114615698575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f9061726390830184617212565b949350505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181035f8312801583831316838312821617156172b2576172b261727f565b5092915050565b808201808211156110975761109761727f565b818103818111156110975761109761727f565b5f602082840312156172ef575f5ffd5b815161569881617107565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f61738e6040830185617212565b82810360208401526173a08185617212565b95945050505050565b6001815b6001841115610d3d578085048111156173c8576173c861727f565b60018416156173d657908102905b60019390931c9280026173ad565b5f826173f257506001611097565b816173fe57505f611097565b8160018114617414576002811461741e5761743a565b6001915050611097565b60ff84111561742f5761742f61727f565b50506001821b611097565b5060208310610133831016604e8410600b841016171561745d575081810a611097565b6174695f1984846173a9565b805f190482111561747c5761747c61727f565b029392505050565b5f61569883836173e4565b80820281158282048414176110975761109761727f565b634e487b7160e01b5f52601260045260245ffdfea2646970667358221220641396967051097bd7696f6f6d5db42e1aa552b613042670911771fee2b6667564736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80cJJ{\x04\x14a\0YW\x80cR\xB5\xDE=\x14a\0\x9CW\x80cf\r\rg\x14a\0\xB1W\x80c\x9F\xF7\x8C0\x14a\0\xD8W\x80c\xD9\xC4'B\x14a\0\xFFW[__\xFD[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\xAA6`\x04aq\x1BV[a\x01\x12V[\0[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xAFa\x01\r6`\x04aqYV[a\x02;V[a\x01\x1Aa\x03rV[a\x01\x89`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x050V[_`@Q\x80`\xE0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x015\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81RP\x90Pa\x02.\x83\x82a\x05\xDEV[Pa\x027a\x08^V[PPV[a\x02Ca\x03rV[a\x02t`@Q` \x01a\x01K\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_`@Q\x80a\x01 \x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x01` \x81\x01\x90a\x02\xF1\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01` \x81\x01\x90a\x03\x12\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83`@\x015\x81R` \x01\x83``\x015\x81R` \x01\x83`\x80\x015\x81R` \x01\x83`\xA0\x015\x81R` \x01\x83`\xC0\x01` \x81\x01\x90a\x03[\x91\x90aq\x8AV[`\x01`\x01`\xA0\x1B\x03\x16\x90R\x90Pa\x02.\x83\x82a\t\x17V[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xB1\x90aq\xA5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04$\x91\x90aq\xDCV[\x90P\x80\x15a\x04yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x04\xB7\x90aq\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x027\x91\x90aq\xDCV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBD\x91\x90aq\xF3V[a\x027W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90ar@V[a\x06\x14`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t2\xBC2\xB1\xBA\xBA2\xA9\xBB\xB0\xB8$\xB7(7\xB9\xB4\xBA4\xB7\xB7`Y\x1B\x81RPa\r\0V[a\x06\x1CanRV[a\x06.\x83\x83_\x01Q\x84`@\x01Qa\r$V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x06T\x92\x91\x90a\rEV[` \x83\x01R\x80\x82R``\x83\x01Q`\xA0\x80\x84\x01\x82\x90R`\x80\x85\x01Q`\xC0\x80\x86\x01\x82\x90R\x91\x86\x01Q\x91\x86\x01Qa\x06\x8B\x94\x93\x92\x90_a\r\x86V[a\x02 \x86\x01Ra\x01\x80\x85\x01Ra\x01`\x84\x01Ra\x01\0\x83\x01\x81\x90R`\xE0\x83\x01\x82\x90R`@\x83\x01Q`\xA0\x84\x01Q`\xC0\x85\x01Qa\x06\xCA\x94\x92\x93\x91\x92\x90\x91a\x0F\xE1V[a\x06\xDC\x81_\x01Q\x82a\x02 \x01Qa\x10nV[a\x02@\x82\x01R\x80Q`@\x82\x01Q`\xA0\x83\x01Q`\xE0\x84\x01Qa\x07\x0C\x93\x92\x91_\x91a\x07\x05\x91\x90ar\x93V[__a\x10\x9DV[a\x07/\x81_\x01Q\x82`@\x01Q`\x01\x84`\xC0\x01Q\x85a\x01\0\x01Qa\x07\x05\x91\x90ar\x93V[`\xA0\x81\x01Q\x15a\x07GW\x81Q\x81Qa\x07G\x91\x90a\x13!V[a\x07m\x81_\x01Q\x82`@\x01Q\x83`\xA0\x01Q\x84`\xC0\x01Q\x85`\xE0\x01Q\x86a\x01\0\x01Qa\x13fV[a\x07\x83\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\x13\xD0V[a\x07\x94\x82` \x01Q\x82_\x01Qa&sV[\x81Q` \x82\x01Q\x82Qa\x07\xA8\x92\x91\x90a&\x97V[a\x07\xC9\x81_\x01Q\x82`\xA0\x01Q\x83`\xC0\x01Q\x84`\xE0\x01Q\x85a\x01\0\x01Qa8lV[a\x02\0\x85\x01\x81\x90Ra\x01\xE0\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01\xC0\x86\x01\x81\x90R\x93\x90\x92\x16a\x01\xA0\x85\x01\x81\x90R` \x86\x81\x01Q`@\x88\x81\x01Qa\x02@\x89\x01Q\x82Q`\x80\x81\x01\x84R\x83\x8B\x01\x80QQQ\x87\x01Q\x82R\x80QQQ\x85\x01Q\x82\x88\x01R\x80QQ\x87\x01Q\x87\x01Q\x82\x86\x01RQQ\x90\x95\x01Q\x90\x92\x01Q``\x85\x01Ra\x08Y\x97\x92\x96\x8B\x96\x93\x94\x91\x93\x92\x90\x91a9TV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x08\x9C\x90aq\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x14\x91\x90aq\xDCV[PV[a\tC`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x06W\x86V7WFU7v\x17`\xAC\x1B\x81RPa\r\0V[a\tKan\xFAV[a\ta\x82_\x01Q\x83`@\x01Q\x84``\x01Qa\rEV[` \x83\x81\x01\x91\x90\x91R\x81\x83R\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x82\x90R\x84\x81\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE3\x91\x90aq\xDCV[``\x80\x83\x01\x91\x90\x91R`@\x80\x83\x01Q\x91\x84\x01Q\x90Qc5/\x9A\xED`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90c5/\x9A\xED\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n^\x91\x90aq\xDCV[`\x80\x82\x01R``\x81\x01Q\x15\x80\x15a\nwWP`\x80\x81\x01Q\x15[\x15a\n\x95W`@Qc|\x9C\r}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x80\x01Q\x81``\x01Q\x10\x15a\n\xB0W`\x80\x82\x01Q``\x82\x01R[\x81`\xA0\x01Q\x81`\x80\x01Q\x10\x15a\n\xCBW`\xA0\x82\x01Q`\x80\x82\x01R[a\n\xED\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x85`\xC0\x01Q\x86`\xE0\x01Q`\x01a\r\x86V[a\x01\xA0\x86\x01\x81\x90Ra\x01\0\x86\x01\x91\x90\x91R`\xE0\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91R\x81Qa\x0B!\x91a\x10nV[a\x01\xC0\x82\x01R`\xA0\x81\x01Q\x15a\x0B\xABW`@\x80\x82\x01Q\x83\x82\x01Qa\x01\0\x85\x01Q`\xA0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\x94W__\xFD[PZ\xF1\x15\x80\x15a\x0B\xA6W=__>=_\xFD[PPPP[`\xC0\x81\x01Q\x15a\x0C0W`@\x80\x82\x01Q``\x84\x01Qa\x01\0\x85\x01Q`\xC0\x85\x01Q\x93Qc\x07\x8D;y`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x82\x16`$\x82\x01R`D\x81\x01\x93\x90\x93R\x16\x90c\x07\x8D;y\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x19W__\xFD[PZ\xF1\x15\x80\x15a\x0C+W=__>=_\xFD[PPPP[a\x0CA\x82` \x01Q\x82_\x01Qa&sV[``\x81\x01Q\x15a\x0CYW\x81Q\x81Qa\x0CY\x91\x90a\x13!V[\x81Q` \x82\x01Q\x82Qa\x0Cm\x92\x91\x90a&\x97V[a\x0C\x8D\x81_\x01Q\x82``\x01Q\x83`\x80\x01Q\x84`\xA0\x01Q\x85`\xC0\x01Qa8lV[a\x01\x80\x85\x01\x81\x90Ra\x01`\x85\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16a\x01@\x86\x01\x81\x90R\x93\x90\x92\x16a\x01 \x85\x01\x81\x90R` \x86\x81\x01Qa\x01\xC0\x87\x01Q`@\x80Q`\x80\x81\x01\x82R_\x80\x82R\x94\x81\x01\x85\x90R\x90\x81\x01\x84\x90R``\x81\x01\x93\x90\x93Ra\x08Y\x96\x91\x95\x8A\x95\x92\x93bz\x12\0\x93\x92\x90a9TV[a\t\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82a:\x0BV[a\r,ao\x83V[_a\r8\x85\x85\x85a:PV[\x91P\x91P[\x93P\x93\x91PPV[a\rMao\xA9V[__a\rY\x85\x85a:}V[\x90P_a\rf\x87\x83a;%V[\x90Pa\rr\x81\x83aM'V[a\r{\x81aMUV[\x96\x90\x95P\x93PPPPV[_____a\r\xE3`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x8A\x11\x80\x15a\r\xF0WP\x87\x15[\x15a\x0E\xCAWa\x0E\0\x8A\x8D\x89aN\x01V[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R\x80\x82R\x8CQ` \x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x8DQQQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8B\x90R`\xE0\x82\x01\x81\x90R\x89\x11\x15a\x0EoW\x80Q`@Qcg\x87\x8A\xC1`\xE1\x1B\x81Ra\x04p\x91\x8B\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x81\x01Qa\x01\0\x82\x01Q\x82Qa\x0E\x86\x91\x90ar\xB9V[\x11\x15a\x0E\xC5Wa\x01\0\x81\x01Q\x81Qa\x0E\x9E\x91\x90ar\xB9V[`@\x80\x83\x01Q\x90Qc\x1F\xC1\x07\xC1`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[a\x0F\xB3V[_\x8B\x11\x80\x15a\x0E\xD7WP\x88\x15[\x15a\x0F\x9AWa\x0E\xE7\x8B\x8D\x89aN\xEAV[a\x01\0\x85\x01R``\x84\x01R`@\x83\x01R` \x80\x83\x01\x82\x90R\x8DQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x85\x01R\x8EQ\x90\x91\x01QQ\x16`\xA0\x83\x01R`\xC0\x82\x01\x8C\x90R`\xE0\x82\x01\x81\x90R\x88\x11\x15a\x0F^W` \x81\x01Q`@Qcu\x0E\xB4I`\xE1\x1B\x81Ra\x04p\x91\x8A\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[\x80``\x01Q\x81` \x01Q\x11\x15a\x0E\xC5W` \x81\x01Q``\x82\x01Q`@Qc\x0Ey;\xAF`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04pV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q` \x82\x01Q`@\x83\x01Q``\x84\x01Qa\x01\0\x90\x94\x01Q\x92\x9F\x91\x9EP\x9CP\x91\x9AP\x98P\x96PPPPPPPV[_\x83\x11\x80\x15a\x0F\xEEWP\x80\x15[\x15a\x10=W\x84Q` \x90\x81\x01Q\x01Q\x83\x11\x15a\x108W\x84Q\x83\x90`\x01[` \x02\x01Q` \x01Q`@Qcg\x1A\xBD\x07`\xE0\x1B\x81R`\x04\x01a\x04p\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[a\x10gV[_\x84\x11\x80\x15a\x10JWP\x81\x15[\x15a\x10gW\x84QQ` \x01Q\x84\x11\x15a\x10gW\x84Q\x84\x90_a\x10\x0BV[PPPPPV[``\x82\x01Q_\x90`H\x1Ca\xFF\xFF\x16\x81a\x10\x87\x84\x83aO\xB5V[\x90Pa\x10\x93\x85\x82aO\xD9V[\x91PP[\x92\x91PPV[_a\x10\xA7\x84aO\xF6V[\x90P_\x84\x12a\x11\x15W\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x10\xC8Wa\x10\xC8arkV[` \x02\x01Q` \x01\x81\x81Qa\x10\xDD\x91\x90ar\xB9V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x10\xF8Wa\x10\xF8arkV[` \x02\x01Q``\x01\x81\x81Qa\x11\r\x91\x90ar\xB9V[\x90RPa\x11vV[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11-Wa\x11-arkV[` \x02\x01Q` \x01\x81\x81Qa\x11B\x91\x90ar\xCCV[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x11]Wa\x11]arkV[` \x02\x01Q``\x01\x81\x81Qa\x11r\x91\x90ar\xCCV[\x90RP[\x81\x15a\x11\xF9W\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\x11\x94Wa\x11\x94arkV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\x11\xB8Wa\x11\xB8arkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x11\xCD\x91\x90ar\xCCV[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\x11\xE8Wa\x11\xE8arkV[` \x02\x01Q`@\x01RPa\x13\x19\x90PV[\x82_\x03a\x12\x06WPa\x13\x19V[_a\x12\x10\x84aO\xF6V[\x90P_a\x12F\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\x12.Wa\x12.arkV[` \x02\x01Q` \x01Q\x83aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\x12\xB4W\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12gWa\x12garkV[` \x02\x01Q`@\x01\x81\x81Qa\x12|\x91\x90ar\xB9V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\x97Wa\x12\x97arkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x12\xAC\x91\x90ar\xB9V[\x90RPa\x13\x15V[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\xCCWa\x12\xCCarkV[` \x02\x01Q`@\x01\x81\x81Qa\x12\xE1\x91\x90ar\xCCV[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\x12\xFCWa\x12\xFCarkV[` \x02\x01Q`\xA0\x01\x81\x81Qa\x13\x11\x91\x90ar\xCCV[\x90RP[PPP[PPPPPPV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x027W__a\x13?\x84\x84aPFV[\x91P\x91P\x81\x81\x10a\x13`W``\x83\x01Qa\x13Z\x90`\x01aP\xA6V[``\x84\x01R[PPPPV[_a\x13q\x84\x83ar\x93V[\x90P_a\x13\x81\x88\x87\x87\x87\x87aP\xD0V[\x90P_\x82\x13\x15a\x13\xABW\x86Q` \x01Qa\x13\xA6\x90\x82a\x13\x9F\x85aO\xF6V[`\x01aQ\x80V[a\x13\xC6V[\x86Q` \x01Qa\x13\xC6\x90\x82a\x13\xBF\x85aO\xF6V[`\x01aR\x9EV[PPPPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x14\x14\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14dW__\xFD[PZ\xF1\x15\x80\x15a\x14vW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x14\x96\x84`@\x01QaS\xB1V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\xD4W__\xFD[PZ\xF1\x15\x80\x15a\x14\xE6W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x15$\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15T\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x95\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD5\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x16\x15\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\x16\x90\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\xACW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD0\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x17\x10\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17\xA3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC7\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\x0C\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18\x99W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xBD\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\x02\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x192\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xB2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xFD\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xAE\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xF8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA9\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xF5\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA6\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xF1\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\xE1\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\x11\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x93\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1E\xD3\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x8D\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\xD2\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \x02\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x85\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \xCA\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!YW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!}\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\xC8\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"|\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"\xC6\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#z\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\xC6\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$z\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$\xC5\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\xF5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%y\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%\xB8\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10g\x91\x90aq\xDCV[a&|\x81aT5V[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a&\xD7\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a''W__\xFD[PZ\xF1\x15\x80\x15a'9W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a'}\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\xAD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(4\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(|\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)-\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)t\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)\xA4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*$\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*p\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xFDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+!\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+A\x90ar\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF2\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,?\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF0\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-9\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-i\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xEA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a.+\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xE5\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a/-\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/]\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a/\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xE0\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a0'\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0W\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xDA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1&\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a1V\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xDA\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a1\xFA\x90as;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a2\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xAE\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a2\xFB\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a3\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xAF\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a3\xF8\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xAC\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a4\xEA\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5d\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a5\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xA4\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a5\xF6\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a6&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra6q\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xB1\x91\x90ar\xDFV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a6\xF8\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a7(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7i\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a7\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA9\x91\x90aq\xDCV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a7\xFB\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a&3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[____a8\xA9`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81RP\x90V[_\x88\x11\x80\x15a8\xB6WP\x85\x15[\x15a8\xEDW\x89Q` \x90\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x8BQQQ\x16\x90\x82\x01R`@\x81\x01\x88\x90R``\x81\x01\x87\x90Ra9/V[_\x89\x11\x80\x15a8\xFAWP\x86\x15[\x15a9/W\x89QQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x8AQ` \x90\x81\x01QQ\x90\x91\x16\x90\x82\x01R`@\x81\x01\x89\x90R``\x81\x01\x86\x90R[\x80Q` \x82\x01Q`@\x83\x01Q``\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`@\x80Qc\x11\xCC\xB2\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x87\x90R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\x11\xCC\xB2\x1D\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a9\xEAW__\xFD[PZ\xF1\x15\x80\x15a9\xFCW=__>=_\xFD[PPPPPPPPPPPPPV[a\x027\x82\x82`@Q`$\x01a:!\x92\x91\x90as|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaV\x17V[a:Xao\x83V[__a:d\x86\x85aV V[\x90P_a:q\x86\x83aV\x86V[\x90Pa\r{\x87\x82aV\x9FV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a:\x9EW\x81\x83a:\xA1V[\x82\x82[`@Q\x91\x94P\x92Pa:\xCE\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a;-ao\xA9V[\x82a;6ao\xA9V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a;r\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xEA\x91\x90aq\xF3V[a;\xF7W\x91Pa\x10\x97\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a<7\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<g\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x9B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xDA\x91\x90ar\xDFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=X\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x8C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xCB\x91\x90aq\xDCV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>!\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>Q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x85\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xC4\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\x1F\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?O\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\x83\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xC2\x91\x90aq\xDCV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a?\xEB\x90` \x01ar\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\x1B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@O\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x8E\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\xEA\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aAN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\x8D\x91\x90aq\xDCV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB>\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBYW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB}\x91\x90aq\xDCV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC&\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aCe\x91\x90ar\xDFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\x0F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDN\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aD\xA5\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEH\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xA4\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFG\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aFw\x90as;V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\x1A\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aGw\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\x1A\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aHs\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x16\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aId\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x07\x91\x90ar\xDFV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aJu\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xA5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xD9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x18\x91\x90ar\xDFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK{\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\x1E\x91\x90aq\xDCV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aLw\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x1A\x91\x90aq\xDCV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x027W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04pV[`\x80\x81\x01QB\x90\x81\x90\x03aMgWPPV[\x81QQ`\xA0\x01Q\x15aM\xBDW\x81Q_\x90aM\x90\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaW\x19V[\x83Q\x90\x91PaM\xB4\x90\x82\x90_[` \x02\x01Q` \x01QaWM\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aM\xF9W\x81Q_\x90aM\xDB\x90`\x01aM}V[\x83Q\x90\x91PaM\xED\x90\x82\x90`\x01aM\x9DV[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[____aN\rao\xDDV[aN\x18\x87__aW\x8EV[P`@\x84\x01RP\x81RaN-\x87`\x01_aW\x8EV[P``\x84\x01RP` \x82\x01R\x85\x15aNUW\x87\x81` \x01\x81\x81QaNQ\x91\x90ar\xCCV[\x90RP[\x80Q` \x82\x01QaNp\x91\x90aNk\x81\x8CaX<V[aX\x90V[`\x80\x82\x01\x81\x90R\x81QaN\x82\x91aYOV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01QaN\xB9\x91aN\xB1\x90a'\x10\x90aYOV[a'\x10aX\x90V[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01QaN\xD7\x91aO\xB5V[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____aN\xF6ao\xDDV[aO\x01\x87__aW\x8EV[P`@\x84\x01RP\x81RaO\x16\x87`\x01_aW\x8EV[P``\x84\x01RP` \x82\x01R\x85\x15aO=W\x87\x81_\x01\x81\x81QaO9\x91\x90ar\xCCV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90RaOd\x90\x89\x90aN\xB1\x90a'\x10\x90aYOV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01QaO\x82\x92aNk\x90\x83\x90aX<V[`\x80\x82\x01\x81\x90R` \x82\x01QaO\x97\x91aYOV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01QaN\xD7\x90\x8C\x90[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aO\xCBW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[\x81QQ`\xC0\x01\x80Q\x82\x91\x90aO\xEF\x90\x83\x90ar\xB9V[\x90RPPPV[__\x82\x12\x15aP\x07W\x81_\x03a\x10\x97V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aP,W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[___aPW\x84``\x01Q_aY\xA4V[\x90P_aPc\x86aY\xD2V[\x90P_aP\x86\x82aPu\x85`\nat\x84V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaX\x90V[\x90P_aP\x94\x87__aW\x8EV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[_`3\x82aP\xB4W_aP\xB7V[`\x01[`\xFF\x16\x90\x1Bf\x08\0\0\0\0\0\0\x19\x84\x16\x17\x90P\x92\x91PPV[____\x86\x11\x80\x15aP\xE0WP\x83\x15[\x15aP\xEFWP\x83\x90P\x84aQ\x07V[_\x87\x11\x80\x15aP\xFCWP\x84\x15[\x15a\x0F\x9AWP\x85\x90P\x82[_aQ\x16\x89``\x01Q_aY\xA4V[\x90P_aQ(\x8A``\x01Q`\x01aY\xA4V[\x90P_aQF\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaNk\x86`\nat\x84V[\x90P_aQd\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaNk\x86`\nat\x84V[\x90PaQp\x82\x82aP\x0BV[\x9C\x9BPPPPPPPPPPPPV[`\xE0\x84\x01Q`\x01\x19\x01aQ\xA7W`\x01`\xE0\x85\x01R`\xA0\x84\x01\x82\x90R``\x84\x01\x83\x90Ra\x13`V[`\xE0\x84\x01Q_\x19\x01aR\x19W`\xA0\x84\x01\x80Q\x90\x83\x90aQ\xC6\x82\x84ar\xB9V[\x90RP\x81\x15aR\x13W_aQ\xDA\x85\x85aWMV[``\x87\x01QaQ\xE9\x90\x84aWMV[aQ\xF3\x91\x90ar\xB9V[\x90PaR\x0C\x86`\xA0\x01Q\x82aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x87\x01RP[Pa\x13`V[`\xE0\x84\x01Qa\x13`W\x81\x84`\xC0\x01Q\x11\x15aRHW\x81\x84`\xC0\x01\x81\x81QaR@\x91\x90ar\xCCV[\x90RPa\x13`V[\x81\x84`\xC0\x01Q\x03aRlW`\x02`\xE0\x85\x01R_`\xC0\x85\x01\x81\x90R`\x80\x85\x01Ra\x13`V[`\x01`\xE0\x85\x01R`\xC0\x84\x01QaR\x82\x90\x83ar\xCCV[`\xA0\x85\x01RPP``\x82\x01R_`\xC0\x82\x01\x81\x90R`\x80\x90\x91\x01RV[`\xE0\x84\x01Q`\x01\x19\x01aR\xC4W_`\xE0\x85\x01R`\xC0\x84\x01\x82\x90R`\x80\x84\x01\x83\x90Ra\x13`V[`\xE0\x84\x01QaS2W`\xC0\x84\x01\x80Q\x90\x83\x90aR\xE0\x82\x84ar\xB9V[\x90RP\x81\x15aR\x13W_aR\xF4\x85\x85aWMV[`\x80\x87\x01QaS\x03\x90\x84aWMV[aS\r\x91\x90ar\xB9V[\x90PaS&\x86`\xC0\x01Q\x82aP\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x80\x87\x01RPPa\x13`V[`\xE0\x84\x01Q_\x19\x01a\x13`W\x81\x84`\xA0\x01Q\x11\x15aS\\W\x81\x84`\xA0\x01\x81\x81QaR@\x91\x90ar\xCCV[\x81\x84`\xA0\x01Q\x03aS\x80W`\x02`\xE0\x85\x01R_`\xA0\x85\x01\x81\x90R``\x85\x01Ra\x13`V[_`\xE0\x85\x01R`\xA0\x84\x01QaS\x95\x90\x83ar\xCCV[`\xC0\x85\x01RPP`\x80\x82\x01R_`\xA0\x82\x01\x81\x90R``\x90\x91\x01RV[_`@Q` \x01aS\xEB\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__aTj`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aTt\x84_aZ\x96V[` \x83\x01R\x81R``\x84\x01QaT\x8A\x90_aY\xA4V[``\x82\x01\x81\x90R\x81QaT\xAF\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aNk\x90`\nat\x84V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aUN\x91\x90aq\xDCV[`\x80\x82\x01RaU^\x84`\x01aZ\x96V[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x01\x91\x90aq\xDCV[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[a\t\x14\x81aZ\xDCV[_`@Q` \x01aVM\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01a;\x07V[aV\x8Eao\x83V[aV\x98\x83\x83aZ\xFBV[\x93\x92PPPV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aV\xCAW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x04pV[_\x80aW%\x83Bar\xCCV[aW/\x90\x85at\x8FV[c\x01\xE13\x80\x90\x04\x90Pa\x10\x93\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bar\xB9V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aWmW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10aW\xABWaW\xABarkV[` \x02\x01Q\x90P_aW\xBD\x89\x89am\rV[\x90P\x80_\x03aW\xD9W____\x95P\x95P\x95P\x95PPPaN\xE1V[_aW\xE8\x83\x8B`\x80\x01Qam\xDFV[\x90P_aW\xF5\x83\x8AaWMV[\x90P_\x89\x15aX\rWaX\x08\x82\x84aYOV[aX\x0FV[_[\x90PaX\x1B\x83\x85ar\xB9V[\x84aX&\x85\x82ar\xCCV[\x91\x9AP\x98P\x96P\x94PPPPP\x93P\x93P\x93P\x93V[_\x82aXH\x83\x82ar\xB9V[\x91P\x81\x10\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x04pV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aX\xC4W\x83\x82\x81aX\xBAWaX\xBAat\xA6V[\x04\x92PPPaV\x98V[\x80\x84\x11aX\xE4W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x82aY[\x83\x82ar\xCCV[\x91P\x81\x11\x15a\x10\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x04pV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aY\xC4WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aZ#\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZW\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x97\x91\x90aq\xDCV[___aZ\xC3\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aZ\xB4WaZ\xB4arkV[` \x02\x01Q\x86`\x80\x01Qam\xDFV[\x90P_aZ\xD0\x86\x86am\rV[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[a[\x03ao\x83V[\x82a[\x0Cao\x83V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a[L\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\xC4\x91\x90aq\xF3V[a[\xD1W\x91Pa\x10\x97\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\\\x0B\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\o\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\\x8AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\\xAE\x91\x90aq\xDCV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\\\xF6\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a]&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]Z\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]uW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]\x99\x91\x90ar\xDFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a]\xF5\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a^%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a^Y\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^tW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^\x98\x91\x90ar\xDFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a_G\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_\x86\x91\x90aq\xDCV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a_\xDA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a`\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a`>\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`YW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`}\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a`\xD7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa\x07\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aa;\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aaVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aaz\x91\x90aq\xDCV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aa\xD3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ab7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15abRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90abv\x91\x90aq\xDCV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ac*\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15acEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aci\x91\x90aq\xDCV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ac\xC3\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ac\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ad'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15adBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90adf\x91\x90aq\xDCV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ad\xD9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ae\r\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ae(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aeL\x91\x90aq\xDCV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ae\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ae\xF4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15af\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90af3\x91\x90ar\xDFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01af\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15af\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ag\x19\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01agn\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ag\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ag\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ag\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ah\x11\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ahl\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ah\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ah\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ah\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ai\x0F\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aii\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ai\x99\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ai\xCD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ai\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aj\x0C\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ajh\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aj\x98\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aj\xCC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aj\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ak\x0B\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01akf\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ak\x96\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ak\xCA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ak\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\t\x91\x90aq\xDCV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01alX\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01al\x88\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01al\xBC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15al\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90al\xFB\x91\x90aq\xDCV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10am'Wam'arkV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15am\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90am\xA4\x91\x90aq\xDCV[\x90P\x80_\x03am\xB7W_\x92PPPa\x10\x97V[``\x82\x01Q`\xC0\x83\x01Qam\xCB\x82\x84ar\xCCV[am\xD5\x91\x90ar\xCCV[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03am\xF2WP_a\x10\x97V[_am\xFD\x84\x84an\x0FV[`\xA0\x85\x01Q\x90\x91Pa\x10\x93\x90\x82aWMV[_B\x82\x03an\"WP` \x82\x01Qa\x10\x97V[_an1\x84`@\x01Q\x84aW\x19V[\x90PanJ\x84` \x01Q\x82aWM\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x10\x97V[`@Q\x80a\x02`\x01`@R\x80anfao\xA9V[\x81R` \x01_\x81R` \x01anyao\x83V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80ao\x0Eao\xA9V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ao\x96ap2V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80ao\xBCap\xA0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ap\x8A`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81apAW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ap\xF1`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ap\xAFW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x14W__\xFD[__\x82\x84\x03`\xC0\x81\x12\x15aq-W__\xFD[\x835aq8\x81aq\x07V[\x92P`\xA0`\x1F\x19\x82\x01\x12\x15aqKW__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[__\x82\x84\x03a\x01\0\x81\x12\x15aqlW__\xFD[\x835aqw\x81aq\x07V[\x92P`\xE0`\x1F\x19\x82\x01\x12\x15aqKW__\xFD[_` \x82\x84\x03\x12\x15aq\x9AW__\xFD[\x815aV\x98\x81aq\x07V[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aq\xECW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ar\x03W__\xFD[\x81Q\x80\x15\x15\x81\x14aV\x98W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90arc\x90\x83\x01\x84ar\x12V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15ar\xB2War\xB2ar\x7FV[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x10\x97Wa\x10\x97ar\x7FV[\x81\x81\x03\x81\x81\x11\x15a\x10\x97Wa\x10\x97ar\x7FV[_` \x82\x84\x03\x12\x15ar\xEFW__\xFD[\x81QaV\x98\x81aq\x07V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_as\x8E`@\x83\x01\x85ar\x12V[\x82\x81\x03` \x84\x01Ras\xA0\x81\x85ar\x12V[\x95\x94PPPPPV[`\x01\x81[`\x01\x84\x11\x15a\r=W\x80\x85\x04\x81\x11\x15as\xC8Was\xC8ar\x7FV[`\x01\x84\x16\x15as\xD6W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02as\xADV[_\x82as\xF2WP`\x01a\x10\x97V[\x81as\xFEWP_a\x10\x97V[\x81`\x01\x81\x14at\x14W`\x02\x81\x14at\x1EWat:V[`\x01\x91PPa\x10\x97V[`\xFF\x84\x11\x15at/Wat/ar\x7FV[PP`\x01\x82\x1Ba\x10\x97V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15at]WP\x81\x81\na\x10\x97V[ati_\x19\x84\x84as\xA9V[\x80_\x19\x04\x82\x11\x15at|Wat|ar\x7FV[\x02\x93\x92PPPV[_aV\x98\x83\x83as\xE4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\x97Wa\x10\x97ar\x7FV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 d\x13\x96\x96pQ\t{\xD7ioom]\xB4.\x1A\xA5R\xB6\x13\x04&p\x91\x17q\xFE\xE2\xB6fudsolcC\0\x08\x1C\x003",
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
