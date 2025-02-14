///Module containing a contract's types and functions.
/**

```solidity
library ReaderPoolUtils {
    struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLoan; uint256 actualTradable; }
    struct AssetInfo { address token; string symbol; uint256 decimals; uint256 borrowIndex; }
    struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; uint256 unclaimedFee; uint256 memeMaxDepositAmount; uint256 averagePrice; }
    struct GetPoolInfo { AssetInfo[2] assets; uint256 priceDecimals; uint256 price; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ReaderPoolUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLoan; uint256 actualTradable; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Asset {
        pub token: alloy::sol_types::private::Address,
        pub symbol: alloy::sol_types::private::String,
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowApy: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateralWithDebt: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub poolBalance: alloy::sol_types::private::primitives::aliases::U256,
        pub priceLiquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub avaiableLoan: alloy::sol_types::private::primitives::aliases::U256,
        pub actualTradable: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Asset> for UnderlyingRustTuple<'_> {
            fn from(value: Asset) -> Self {
                (
                    value.token,
                    value.symbol,
                    value.decimals,
                    value.borrowIndex,
                    value.borrowApy,
                    value.totalCollateral,
                    value.totalCollateralWithDebt,
                    value.totalDebtScaled,
                    value.poolBalance,
                    value.priceLiquidity,
                    value.avaiableLoan,
                    value.actualTradable,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Asset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    symbol: tuple.1,
                    decimals: tuple.2,
                    borrowIndex: tuple.3,
                    borrowApy: tuple.4,
                    totalCollateral: tuple.5,
                    totalCollateralWithDebt: tuple.6,
                    totalDebtScaled: tuple.7,
                    poolBalance: tuple.8,
                    priceLiquidity: tuple.9,
                    avaiableLoan: tuple.10,
                    actualTradable: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Asset {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Asset {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.symbol,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowApy),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.totalCollateralWithDebt,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalDebtScaled),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.poolBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.priceLiquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.avaiableLoan),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualTradable),
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
        impl alloy_sol_types::SolType for Asset {
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
        impl alloy_sol_types::SolStruct for Asset {
            const NAME: &'static str = "Asset";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Asset(address token,string symbol,uint256 decimals,uint256 borrowIndex,uint256 borrowApy,uint256 totalCollateral,uint256 totalCollateralWithDebt,uint256 totalDebtScaled,uint256 poolBalance,uint256 priceLiquidity,uint256 avaiableLoan,uint256 actualTradable)",
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
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.symbol,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.decimals)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowApy)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalCollateral,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalCollateralWithDebt,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalDebtScaled,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.poolBalance)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.priceLiquidity,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.avaiableLoan)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.actualTradable,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Asset {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.symbol,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.decimals,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowApy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalCollateral,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalCollateralWithDebt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalDebtScaled,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.poolBalance,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.priceLiquidity,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avaiableLoan,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.actualTradable,
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
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.symbol,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.decimals,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowApy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalCollateral,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalCollateralWithDebt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalDebtScaled,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.poolBalance,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.priceLiquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avaiableLoan,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.actualTradable,
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
struct AssetInfo { address token; string symbol; uint256 decimals; uint256 borrowIndex; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AssetInfo {
        pub token: alloy::sol_types::private::Address,
        pub symbol: alloy::sol_types::private::String,
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowIndex: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<AssetInfo> for UnderlyingRustTuple<'_> {
            fn from(value: AssetInfo) -> Self {
                (value.token, value.symbol, value.decimals, value.borrowIndex)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    symbol: tuple.1,
                    decimals: tuple.2,
                    borrowIndex: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AssetInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AssetInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.symbol,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowIndex),
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
        impl alloy_sol_types::SolType for AssetInfo {
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
        impl alloy_sol_types::SolStruct for AssetInfo {
            const NAME: &'static str = "AssetInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AssetInfo(address token,string symbol,uint256 decimals,uint256 borrowIndex)",
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
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.symbol,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.decimals)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowIndex)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AssetInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.symbol,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.decimals,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowIndex,
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
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.symbol,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.decimals,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowIndex,
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
struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; uint256 unclaimedFee; uint256 memeMaxDepositAmount; uint256 averagePrice; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GetPool {
        pub assets: [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
        pub bank: alloy::sol_types::private::Address,
        pub interestRateStrategy: alloy::sol_types::private::Address,
        pub configuration: alloy::sol_types::private::primitives::aliases::U256,
        pub lastUpdateTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub priceDecimals: alloy::sol_types::private::primitives::aliases::U256,
        pub price: alloy::sol_types::private::primitives::aliases::U256,
        pub source: alloy::sol_types::private::Address,
        pub shortEnabled: bool,
        pub createdTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub unclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
        pub memeMaxDepositAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub averagePrice: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::FixedArray<Asset, 2usize>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            bool,
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
        impl ::core::convert::From<GetPool> for UnderlyingRustTuple<'_> {
            fn from(value: GetPool) -> Self {
                (
                    value.assets,
                    value.bank,
                    value.interestRateStrategy,
                    value.configuration,
                    value.lastUpdateTimestamp,
                    value.priceDecimals,
                    value.price,
                    value.source,
                    value.shortEnabled,
                    value.createdTimestamp,
                    value.unclaimedFee,
                    value.memeMaxDepositAmount,
                    value.averagePrice,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GetPool {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    assets: tuple.0,
                    bank: tuple.1,
                    interestRateStrategy: tuple.2,
                    configuration: tuple.3,
                    lastUpdateTimestamp: tuple.4,
                    priceDecimals: tuple.5,
                    price: tuple.6,
                    source: tuple.7,
                    shortEnabled: tuple.8,
                    createdTimestamp: tuple.9,
                    unclaimedFee: tuple.10,
                    memeMaxDepositAmount: tuple.11,
                    averagePrice: tuple.12,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GetPool {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GetPool {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.bank,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.interestRateStrategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.configuration),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastUpdateTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.priceDecimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.price),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.shortEnabled,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.unclaimedFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeMaxDepositAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.averagePrice),
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
        impl alloy_sol_types::SolType for GetPool {
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
        impl alloy_sol_types::SolStruct for GetPool {
            const NAME: &'static str = "GetPool";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GetPool(Asset[2] assets,address bank,address interestRateStrategy,uint256 configuration,uint256 lastUpdateTimestamp,uint256 priceDecimals,uint256 price,address source,bool shortEnabled,uint256 createdTimestamp,uint256 unclaimedFee,uint256 memeMaxDepositAmount,uint256 averagePrice)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Asset as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Asset as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.assets)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.bank,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.interestRateStrategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.configuration)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lastUpdateTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.priceDecimals)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.price)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.source,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.shortEnabled,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.createdTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.unclaimedFee)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.memeMaxDepositAmount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.averagePrice)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GetPool {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.assets,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.bank,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.interestRateStrategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.configuration,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastUpdateTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.priceDecimals,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.price)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.source,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.shortEnabled,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.createdTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.unclaimedFee,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.memeMaxDepositAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.averagePrice,
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
                <alloy::sol_types::sol_data::FixedArray<
                    Asset,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.assets,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.bank,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.interestRateStrategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.configuration,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastUpdateTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.priceDecimals,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.price,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.source,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.shortEnabled,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.createdTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.unclaimedFee,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.memeMaxDepositAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.averagePrice,
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
struct GetPoolInfo { AssetInfo[2] assets; uint256 priceDecimals; uint256 price; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GetPoolInfo {
        pub assets: [<AssetInfo as alloy::sol_types::SolType>::RustType; 2usize],
        pub priceDecimals: alloy::sol_types::private::primitives::aliases::U256,
        pub price: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::FixedArray<AssetInfo, 2usize>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [<AssetInfo as alloy::sol_types::SolType>::RustType; 2usize],
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
        impl ::core::convert::From<GetPoolInfo> for UnderlyingRustTuple<'_> {
            fn from(value: GetPoolInfo) -> Self {
                (value.assets, value.priceDecimals, value.price)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GetPoolInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    assets: tuple.0,
                    priceDecimals: tuple.1,
                    price: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GetPoolInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GetPoolInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        AssetInfo,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.priceDecimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.price),
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
        impl alloy_sol_types::SolType for GetPoolInfo {
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
        impl alloy_sol_types::SolStruct for GetPoolInfo {
            const NAME: &'static str = "GetPoolInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GetPoolInfo(AssetInfo[2] assets,uint256 priceDecimals,uint256 price)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<AssetInfo as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <AssetInfo as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedArray<
                        AssetInfo,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.assets)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.priceDecimals)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.price)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GetPoolInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        AssetInfo,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.assets,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.priceDecimals,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.price)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedArray<
                    AssetInfo,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.assets,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.priceDecimals,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.price,
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
    /**Creates a new wrapper around an on-chain [`ReaderPoolUtils`](self) contract instance.

See the [wrapper's documentation](`ReaderPoolUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ReaderPoolUtilsInstance<T, P, N> {
        ReaderPoolUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ReaderPoolUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ReaderPoolUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ReaderPoolUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ReaderPoolUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ReaderPoolUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReaderPoolUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ReaderPoolUtils`](self) contract instance.

See the [wrapper's documentation](`ReaderPoolUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ReaderPoolUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ReaderPoolUtilsInstance<T, P, N> {
            ReaderPoolUtilsInstance {
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
    > ReaderPoolUtilsInstance<T, P, N> {
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
    > ReaderPoolUtilsInstance<T, P, N> {
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
library ReaderPositionUtils {
    struct Asset { address token; string symbol; uint256 decimals; uint256 balance; uint256 debt; int256 netWorth; uint256 maxRedeemAmount; uint256 borrowApy; int256 equity; int256 equityValue; }
    struct GetPosition { Asset[2] assets; uint256 id; address account; uint256 marginLevel; uint256 entryPrice; uint256 IndexPrice; int256 pnl; uint256 liquidationPrice; int256 toLiquidationPrice; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ReaderPositionUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Asset { address token; string symbol; uint256 decimals; uint256 balance; uint256 debt; int256 netWorth; uint256 maxRedeemAmount; uint256 borrowApy; int256 equity; int256 equityValue; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Asset {
        pub token: alloy::sol_types::private::Address,
        pub symbol: alloy::sol_types::private::String,
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
        pub debt: alloy::sol_types::private::primitives::aliases::U256,
        pub netWorth: alloy::sol_types::private::primitives::aliases::I256,
        pub maxRedeemAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowApy: alloy::sol_types::private::primitives::aliases::U256,
        pub equity: alloy::sol_types::private::primitives::aliases::I256,
        pub equityValue: alloy::sol_types::private::primitives::aliases::I256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Int<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::String,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::primitives::aliases::I256,
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
        impl ::core::convert::From<Asset> for UnderlyingRustTuple<'_> {
            fn from(value: Asset) -> Self {
                (
                    value.token,
                    value.symbol,
                    value.decimals,
                    value.balance,
                    value.debt,
                    value.netWorth,
                    value.maxRedeemAmount,
                    value.borrowApy,
                    value.equity,
                    value.equityValue,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Asset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    symbol: tuple.1,
                    decimals: tuple.2,
                    balance: tuple.3,
                    debt: tuple.4,
                    netWorth: tuple.5,
                    maxRedeemAmount: tuple.6,
                    borrowApy: tuple.7,
                    equity: tuple.8,
                    equityValue: tuple.9,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Asset {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Asset {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.symbol,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.debt),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.netWorth),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxRedeemAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowApy),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.equity),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.equityValue),
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
        impl alloy_sol_types::SolType for Asset {
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
        impl alloy_sol_types::SolStruct for Asset {
            const NAME: &'static str = "Asset";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Asset(address token,string symbol,uint256 decimals,uint256 balance,uint256 debt,int256 netWorth,uint256 maxRedeemAmount,uint256 borrowApy,int256 equity,int256 equityValue)",
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
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.symbol,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.decimals)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.balance)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.debt)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.netWorth)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxRedeemAmount,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowApy)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.equity)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.equityValue)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Asset {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.symbol,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.decimals,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balance,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.debt)
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.netWorth,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxRedeemAmount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowApy,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.equity,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.equityValue,
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
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.symbol,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.decimals,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balance,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.debt,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.netWorth,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxRedeemAmount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowApy,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.equity,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.equityValue,
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
struct GetPosition { Asset[2] assets; uint256 id; address account; uint256 marginLevel; uint256 entryPrice; uint256 IndexPrice; int256 pnl; uint256 liquidationPrice; int256 toLiquidationPrice; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GetPosition {
        pub assets: [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub account: alloy::sol_types::private::Address,
        pub marginLevel: alloy::sol_types::private::primitives::aliases::U256,
        pub entryPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub IndexPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub pnl: alloy::sol_types::private::primitives::aliases::I256,
        pub liquidationPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub toLiquidationPrice: alloy::sol_types::private::primitives::aliases::I256,
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
            alloy::sol_types::sol_data::FixedArray<Asset, 2usize>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Int<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::I256,
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
        impl ::core::convert::From<GetPosition> for UnderlyingRustTuple<'_> {
            fn from(value: GetPosition) -> Self {
                (
                    value.assets,
                    value.id,
                    value.account,
                    value.marginLevel,
                    value.entryPrice,
                    value.IndexPrice,
                    value.pnl,
                    value.liquidationPrice,
                    value.toLiquidationPrice,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GetPosition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    assets: tuple.0,
                    id: tuple.1,
                    account: tuple.2,
                    marginLevel: tuple.3,
                    entryPrice: tuple.4,
                    IndexPrice: tuple.5,
                    pnl: tuple.6,
                    liquidationPrice: tuple.7,
                    toLiquidationPrice: tuple.8,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for GetPosition {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for GetPosition {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.assets),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.marginLevel),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.entryPrice),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.IndexPrice),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.pnl),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidationPrice),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.toLiquidationPrice),
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
        impl alloy_sol_types::SolType for GetPosition {
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
        impl alloy_sol_types::SolStruct for GetPosition {
            const NAME: &'static str = "GetPosition";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "GetPosition(Asset[2] assets,uint256 id,address account,uint256 marginLevel,uint256 entryPrice,uint256 IndexPrice,int256 pnl,uint256 liquidationPrice,int256 toLiquidationPrice)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Asset as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Asset as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.assets)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.account,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.marginLevel)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.entryPrice)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.IndexPrice)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pnl)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidationPrice,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.toLiquidationPrice,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for GetPosition {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        Asset,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.assets,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.account,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.marginLevel,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.entryPrice,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.IndexPrice,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.pnl)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidationPrice,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.toLiquidationPrice,
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
                <alloy::sol_types::sol_data::FixedArray<
                    Asset,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.assets,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.account,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.marginLevel,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.entryPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.IndexPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.pnl, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidationPrice,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.toLiquidationPrice,
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
    /**Creates a new wrapper around an on-chain [`ReaderPositionUtils`](self) contract instance.

See the [wrapper's documentation](`ReaderPositionUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ReaderPositionUtilsInstance<T, P, N> {
        ReaderPositionUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ReaderPositionUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ReaderPositionUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ReaderPositionUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ReaderPositionUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ReaderPositionUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReaderPositionUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ReaderPositionUtils`](self) contract instance.

See the [wrapper's documentation](`ReaderPositionUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ReaderPositionUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ReaderPositionUtilsInstance<T, P, N> {
            ReaderPositionUtilsInstance {
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
    > ReaderPositionUtilsInstance<T, P, N> {
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
    > ReaderPositionUtilsInstance<T, P, N> {
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
library ReaderPoolUtils {
    struct Asset {
        address token;
        string symbol;
        uint256 decimals;
        uint256 borrowIndex;
        uint256 borrowApy;
        uint256 totalCollateral;
        uint256 totalCollateralWithDebt;
        uint256 totalDebtScaled;
        uint256 poolBalance;
        uint256 priceLiquidity;
        uint256 avaiableLoan;
        uint256 actualTradable;
    }
    struct AssetInfo {
        address token;
        string symbol;
        uint256 decimals;
        uint256 borrowIndex;
    }
    struct GetPool {
        Asset[2] assets;
        address bank;
        address interestRateStrategy;
        uint256 configuration;
        uint256 lastUpdateTimestamp;
        uint256 priceDecimals;
        uint256 price;
        address source;
        bool shortEnabled;
        uint256 createdTimestamp;
        uint256 unclaimedFee;
        uint256 memeMaxDepositAmount;
        uint256 averagePrice;
    }
    struct GetPoolInfo {
        AssetInfo[2] assets;
        uint256 priceDecimals;
        uint256 price;
    }
}

library ReaderPositionUtils {
    struct Asset {
        address token;
        string symbol;
        uint256 decimals;
        uint256 balance;
        uint256 debt;
        int256 netWorth;
        uint256 maxRedeemAmount;
        uint256 borrowApy;
        int256 equity;
        int256 equityValue;
    }
    struct GetPosition {
        Asset[2] assets;
        uint256 id;
        address account;
        uint256 marginLevel;
        uint256 entryPrice;
        uint256 IndexPrice;
        int256 pnl;
        uint256 liquidationPrice;
        int256 toLiquidationPrice;
    }
}

interface Reader {
    error EmptyPool(bytes32 key);
    error MathOverflowedMulDiv();
    error SingleTokenInOutSwapOnly();

    function calcAmountIn(address dataStore, address token0, address token1, uint256 amountOut, uint8 tokenOutIndex) external view returns (uint256, uint256, int256);
    function calcAmountOut(address dataStore, address token0, address token1, uint256 amountIn, uint8 tokenInIndex) external view returns (uint256, uint256, int256);
    function calcLiquidityOut(address dataStore, address token0, address token1, uint256 amount0, uint256 amount1) external view returns (uint256);
    function calcTokenPairOut(address dataStore, address token0, address token1, uint256 liquidity) external view returns (uint256, uint256);
    function getDefaultInterestRateStrategy(address dataStore) external view returns (address);
    function getDefaultPoolConfiguration(address dataStore) external view returns (uint256);
    function getMarginLevelThreshold(address dataStore) external view returns (uint256);
    function getPools(address dataStore) external view returns (ReaderPoolUtils.GetPool[] memory);
    function getPools(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPool[] memory);
    function getPoolsCount(address dataStore) external view returns (uint256);
    function getPoolsInfo(address dataStore) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPositions(address dataStore, address account) external view returns (ReaderPositionUtils.GetPosition[] memory);
    function getTokenBase(address dataStore) external view returns (address);
    function getTreasury(address dataStore) external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "calcAmountIn",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
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
        "name": "amountOut",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "tokenOutIndex",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calcAmountOut",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
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
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "tokenInIndex",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calcLiquidityOut",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
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
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calcTokenPairOut",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
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
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDefaultInterestRateStrategy",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDefaultPoolConfiguration",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMarginLevelThreshold",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPools",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPoolUtils.GetPool[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPoolUtils.Asset[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowApy",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalCollateral",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalCollateralWithDebt",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalDebtScaled",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "poolBalance",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "priceLiquidity",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "avaiableLoan",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "actualTradable",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "bank",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "interestRateStrategy",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "configuration",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "lastUpdateTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "priceDecimals",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "price",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "source",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "shortEnabled",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "createdTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unclaimedFee",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeMaxDepositAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "averagePrice",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPools",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPoolUtils.GetPool[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPoolUtils.Asset[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowApy",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalCollateral",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalCollateralWithDebt",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "totalDebtScaled",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "poolBalance",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "priceLiquidity",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "avaiableLoan",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "actualTradable",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "bank",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "interestRateStrategy",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "configuration",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "lastUpdateTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "priceDecimals",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "price",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "source",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "shortEnabled",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "createdTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "unclaimedFee",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeMaxDepositAmount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "averagePrice",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPoolsCount",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPoolsInfo",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPoolUtils.GetPoolInfo[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPoolUtils.AssetInfo[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "priceDecimals",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "price",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPoolsInfo",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "end",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPoolUtils.GetPoolInfo[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPoolUtils.AssetInfo[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "priceDecimals",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "price",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPoolsInfo",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "poolKeys",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPoolUtils.GetPoolInfo[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPoolUtils.AssetInfo[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "priceDecimals",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "price",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPositions",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct ReaderPositionUtils.GetPosition[]",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct ReaderPositionUtils.Asset[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "symbol",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "decimals",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "balance",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "debt",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "netWorth",
                "type": "int256",
                "internalType": "int256"
              },
              {
                "name": "maxRedeemAmount",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowApy",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "equity",
                "type": "int256",
                "internalType": "int256"
              },
              {
                "name": "equityValue",
                "type": "int256",
                "internalType": "int256"
              }
            ]
          },
          {
            "name": "id",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "account",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "marginLevel",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "entryPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "IndexPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "pnl",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "liquidationPrice",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "toLiquidationPrice",
            "type": "int256",
            "internalType": "int256"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTokenBase",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTreasury",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
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
    "name": "MathOverflowedMulDiv",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SingleTokenInOutSwapOnly",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod Reader {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b50616d818061001c5f395ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c8063739118a411610093578063d28b0a1511610063578063d28b0a1514610269578063e335adb71461027c578063eed074281461028f578063f68a7131146102a2575f5ffd5b8063739118a4146101f557806378f212d1146102155780638f6c7a3c14610228578063c2bdeda11461023b575f5ffd5b806350ed592d116100ce57806350ed592d1461018e57806357b91ca6146101af5780635a6f5710146101c25780635c39f467146101d5575f5ffd5b80631a308175146100ff57806328a0ccf414610128578063317b50ec14610153578063382fc72e1461017b575b5f5ffd5b61011261010d366004616278565b6102b5565b60405161011f91906162c1565b60405180910390f35b61013b610136366004616278565b6102d5565b6040516001600160a01b03909116815260200161011f565b6101666101613660046163ac565b6102e5565b6040805192835260208301919091520161011f565b6101126101893660046163fa565b610300565b6101a161019c366004616278565b610315565b60405190815260200161011f565b6101a16101bd366004616278565b61031f565b6101a16101d0366004616278565b610329565b6101e86101e3366004616278565b610333565b60405161011f9190616502565b610208610203366004616621565b61034c565b60405161011f919061670b565b61013b610223366004616278565b610367565b6101e86102363660046163fa565b610371565b61024e6102493660046167df565b61037e565b6040805193845260208401929092529082015260600161011f565b61024e6102773660046167df565b61039f565b61013b61028a366004616278565b6103af565b61011261029d3660046168ab565b6103b9565b6101a16102b0366004616956565b6103c5565b60605f6102c1836103df565b90506102ce835f83610471565b9392505050565b5f6102df8261048b565b92915050565b5f5f6102f38686868661053c565b9150915094509492505050565b606061030d848484610471565b949350505050565b5f6102df82610578565b5f6102df826105c9565b5f6102df826103df565b60605f61033f836103df565b90506102ce835f8361061a565b60605f61035984846106e8565b905061030d84845f8461075e565b5f6102df8261082e565b606061030d84848461061a565b5f5f5f61038e888888888861086a565b925092509250955095509592505050565b5f5f5f61038e888888888861095a565b5f6102df826109d3565b60606102ce8383610a24565b5f6103d38686868686610ae2565b90505b95945050505050565b5f816001600160a01b031663f3903b9f6040516020016103fe906169ad565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043291815260200190565b602060405180830381865afa15801561044d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df91906169d0565b60605f61047f858585610b15565b90506103d68582610a24565b5f816001600160a01b03166321f8a7216040516020016104c9906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016104fd91815260200190565b602060405180830381865afa158015610518573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df91906169e7565b5f5f5f6105498686610bb6565b90505f6105568883610c5d565b90505f5f6105658a8489611ed5565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f56040516020016103fe9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f610628858585610b15565b90505f815167ffffffffffffffff81111561064557610645616843565b60405190808252806020026020018201604052801561067e57816020015b61066b615cc5565b8152602001906001900390816106635790505b5090505f5b82518110156106de575f83828151811061069f5761069f616a02565b602002602001015190505f6106b4898361210e565b9050808484815181106106c9576106c9616a02565b60209081029190910101525050600101610683565b5095945050505050565b5f826001600160a01b031663f3903b9f610701846125ae565b6040518263ffffffff1660e01b815260040161071f91815260200190565b602060405180830381865afa15801561073a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102ce91906169d0565b60605f61076d86868686612632565b90505f815167ffffffffffffffff81111561078a5761078a616843565b6040519080825280602002602001820160405280156107c357816020015b6107b0615d44565b8152602001906001900390816107a85790505b5090505f5b8251811015610823575f8382815181106107e4576107e4616a02565b602002602001015190505f6107f98a836126ba565b90508084848151811061080e5761080e616a02565b602090810291909101015250506001016107c8565b509695505050505050565b5f816001600160a01b03166321f8a7216040516020016104c990602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6108788888610bb6565b90505f6108858a83610c5d565b90505f808060ff89166108bb5761089d8d8b86612cc9565b9295509193506108b491508590505f858d82612dc1565b90506108eb565b5f1960ff8a16016108eb576108d18d8b86612e8e565b9295509193506108e89150859050845f808e612dc1565b90505b5f6108f585612f65565b90505f82821161090e576109098284616a2a565b610918565b6109188383616a2a565b90505f6109258284613000565b90505f84841161093d5761093882616a3d565b61093f565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f6109688888610bb6565b90505f6109758a83610c5d565b90505f808060ff89166109a55761098e8d8b865f61303b565b9295509193506108b491508590508b5f8087612dc1565b5f1960ff8a16016108eb576109bc8d8b865f61313e565b9295509193506108e891508590505f8c8682612dc1565b5f816001600160a01b03166321f8a7216040516020016104c9906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60605f825167ffffffffffffffff811115610a4157610a41616843565b604051908082528060200260200182016040528015610a7a57816020015b610a67615d97565b815260200190600190039081610a5f5790505b5090505f5b8351811015610ada575f848281518110610a9b57610a9b616a02565b602002602001015190505f610ab08783613226565b905080848481518110610ac557610ac5616a02565b60209081029190910101525050600101610a7f565b509392505050565b5f5f610aee8686610bb6565b90505f610afb8883610c5d565b9050610b098186865f6134b3565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001610b35906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015610b8f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261030d9190810190616a57565b5f816001600160a01b0316836001600160a01b031610610bd7578183610bda565b82825b6040519194509250610c07906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610c65615dbc565b82610c6e615dbc565b816001600160a01b03166391d4403c604051602001610c8c906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610ce0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d049190616ade565b610d115791506102df9050565b816001600160a01b03166321f8a72185604051602001610d51906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610d81929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610db591815260200190565b602060405180830381865afa158015610dd0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df491906169e7565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610e72929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610ea691815260200190565b602060405180830381865afa158015610ec1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee591906169d0565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610f3b906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610f6b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f9f91815260200190565b602060405180830381865afa158015610fba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fde91906169d0565b81515f60200201516040018181525050816001600160a01b031663bd02d0f58560405160200161103f9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161106f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110a391815260200190565b602060405180830381865afa1580156110be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110e291906169d0565b81515f60200201516060018181525050816001600160a01b031663bd02d0f58560405160200161114d9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b6040516020818303038152906040528051906020012060405160200161117d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016111b191815260200190565b602060405180830381865afa1580156111cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111f091906169d0565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016112519060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611281929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016112b591815260200190565b602060405180830381865afa1580156112d0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f491906169d0565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611371929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016113a591815260200190565b602060405180830381865afa1580156113c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113e491906169d0565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611459929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161148d91815260200190565b602060405180830381865afa1580156114a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114cc91906169e7565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161157691815260200190565b602060405180830381865afa158015611591573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115b591906169d0565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161160c90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161163c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161167091815260200190565b602060405180830381865afa15801561168b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116af91906169d0565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016117119060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611741929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161177591815260200190565b602060405180830381865afa158015611790573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117b491906169d0565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016118209060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b60405160208183030381529060405280519060200120604051602001611850929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161188491815260200190565b602060405180830381865afa15801561189f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118c391906169d0565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016119259060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611955929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161198991815260200190565b602060405180830381865afa1580156119a4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119c891906169d0565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001611a2190602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a51929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a8591815260200190565b602060405180830381865afa158015611aa0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ac491906169d0565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001611b1290602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611b42929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b7691815260200190565b602060405180830381865afa158015611b91573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bb591906169e7565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611c23906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611c53929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611c8791815260200190565b602060405180830381865afa158015611ca2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cc691906169e7565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611d29906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d59929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611d8d91815260200190565b602060405180830381865afa158015611da8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dcc91906169d0565b60608201526040516001600160a01b0383169063bd02d0f5908690611e25906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611e55929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611e8991815260200190565b602060405180830381865afa158015611ea4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ec891906169d0565b6080820152949350505050565b5f5f5f5f611ee1615df0565b611eea886135ec565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f34573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5891906169d0565b816020018181525050611f72875f5f84610140015161363d565b506080840152506040820152610140810151611f949088906001905f9061363d565b5060a084015250606082015260408101516020820151611fb5918891613722565b61010082015260608101516020820151611fd0918891613722565b816101200181815250506120106040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b81525082602001516137e1565b612048604051806040016040528060128152602001710766172732e707269636552657365727665360741b81525082604001516137e1565b61208060405180604001604052806012815260200171766172732e7072696365526573657276653160701b81525082606001516137e1565b6120b36040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b8152508261010001516137e1565b6120e66040518060400160405280600c81526020016b766172732e616d6f756e743160a01b8152508261012001516137e1565b80610100015181610120015182608001518360a0015194509450945094505093509350935093565b612116615cc5565b61211e615e3f565b6121288484610c5d565b81526121338461380e565b6020820152612141846135ec565b61018082018190528151602083015161215c925f919061363d565b608085015260a0840152604083015260608201528051602082015161018083015161218a929160019161363d565b61012085015261014084015260e083015261010082015280516121ac90613851565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa15801561222b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526122529190810190616afd565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561229f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122c39190616b91565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156123a3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123ca9190810190616afd565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801561241a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061243e9190616b91565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061251e90612f65565b815260200161252d8686613a60565b6001600160a01b03168152602001612554835f015160600151660800000000000016151590565b151581526020016125658686613b50565b81528251515160c00151602082015282516040909101906125899087905f80613c52565b815260200161259b86845f0151613d4d565b90526101a0909101819052905092915050565b5f6040516020016125e8906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61264c866125ae565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612693573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103d69190810190616a57565b6126c2615d44565b6126ca615ead565b6126d48484613d58565b808252518051516126ed9160015b602002015151610bb6565b604082018190526126ff908590610c5d565b602082018190528151612713918691613d6a565b505050506060820152602081015161272a90612f65565b610300820152805180515160209081015160e08401528083015151510151905161276091905f5b60200201516040015190613dba565b60c0820152602081015160600151612778905f613dfb565b60a082015260e081015160c08201516127919190613e29565b610100820181905260a08201516127a89190613e4a565b61012082015260e081015160c08201516101008301516127c9929190613e66565b610140820152602081015181516103008301516127ea92879290915f613e83565b61016082015261014081015161018082015260e081015160c0820151610120830151612817929190613e66565b6101a0820152602081015160600151612831906001613dfb565b6101c08201528051805160209081015181015161020084015280830151518101510151905161286291906001612751565b6101e0820181905261020082015161287991613e29565b61022082018190526101c08201516128919190613e4a565b61024082018190526103008201516128a99190613dba565b6102608201526102008101516101e08201516102208301516128cc929190613e66565b610280820152602081015181516103008301516128ee92879290916001613e83565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161291d929190613e66565b6102e0820152805161292e90614018565b60808201528051516020015160e00151600214612a26576129588161030001518260800151613e29565b610320820181905261024082015161296f91613dba565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516129a09361405a565b61036082018190526103a082015260e081015160a0820151612a079186916129c89190613e4a565b6129da8460c001518560a00151613e4a565b6129ee856102000151866101c00151613e4a565b612a02866101e00151876101c00151613e4a565b614074565b6103c08201819052610300820151612a1f9190614134565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612a9a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612ac19190810190616afd565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612b1657612b16616a02565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612b6557612b65616a02565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612bc5573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612bec9190810190616afd565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612cd5615f8a565b612cde886135ec565b6101c08201819052612cf59087905f90819061363d565b5060408401525081526101c0810151612d149087906001905f9061363d565b5060608481019190915260208401929092525086015160381c61ffff166101408201819052612d54908890612d4c9061271090614183565b612710613722565b610180820181905281516020830151612d7792612d72908390614183565b613722565b608082018190526020820151612d8d9190614183565b60e0820181905260408201516060830151610140840151612daf908b906141dd565b94509450945094505093509350935093565b5f5f5f5f86118015612dd1575083155b15612de0575083905084612e15565b5f87118015612ded575084155b15612dfc575085905082612e15565b604051636331fab160e01b815260040160405180910390fd5b5f612e2489606001515f613dfb565b90505f612e368a606001516001613dfb565b90505f612e5485676765c793fa10079d601b1b612d7286600a616c8f565b90505f612e7285676765c793fa10079d601b1b612d7286600a616c8f565b9050612e7e8282613000565b9c9b505050505050505050505050565b5f5f5f5f612e9a615f8a565b612ea3886135ec565b6101c08201819052612eba9087905f90819061363d565b5060408401525081526101c0810151612ed99087906001905f9061363d565b50606084015250602082018190528151612ef791612d72818b614183565b61010082018190528151612f0b9190614183565b610120820152606086015160381c61ffff166101408201819052610120820151612f3e9161271090612d72908290614183565b6101a0820181905260408201516060830151610140840151610120850151612daf916141dd565b5f5f612f73835f5f5f61363d565b50505090505f612f868460015f5f61363d565b5050509050805f03612f9b57505f9392505050565b5f612faa85606001515f613dfb565b90505f612fbc86606001516001613dfb565b90505f612fda85676765c793fa10079d601b1b612d7286600a616c8f565b90505f612ff885676765c793fa10079d601b1b612d7286600a616c8f565b9050610b0982825b5f8115676765c793fa10079d601b1b60028404190484111715613021575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f613047615f8a565b613050896135ec565b6101c082018190526130679088905f90819061363d565b5060408401525081526101c08101516130869088906001905f9061363d565b50606084015250602082015285156130ad5787815f018181516130a99190616a2a565b9052505b606087015160381c61ffff1661014082018190526130d4908990612d4c9061271090614183565b6101608201819052815160208301516130f292612d72908390614201565b60808201819052602082015161310791614183565b60c0820181905260408201516060830151610140840151613129908c906141dd565b9450945094509450505b945094509450949050565b5f5f5f5f61314a615f8a565b613153896135ec565b6101c0820181905261316a9088905f90819061363d565b5060408401525081526101c08101516131899088906001905f9061363d565b50606084015250602082015285156131b15787816020018181516131ad9190616a2a565b9052505b805160208201516131c79190612d72818c614201565b6080820181905281516131d991614183565b60a0820152606087015160381c61ffff16610140820181905260a082015161320891612d4c9061271090614183565b6040820151606083015161014084015160a0850151613129916141dd565b61322e615d97565b613236615ff1565b6132408484610c5d565b8152604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156132b3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526132da9190810190616afd565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015613327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334b9190616b91565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156133cb573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526133f29190810190616afd565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613442573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134669190616b91565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016134a186845f0151613d4d565b90526020909101819052905092915050565b5f6134bc615df0565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061352091906169d0565b6020820152613531865f808061363d565b50505060c08201526135468660015f8061363d565b50505060e0820152821561358157848160c0018181516135669190616a2a565b90525060e08101805185919061357d908390616a2a565b9052505b80602001515f036135b1576135aa6103e86135a461359f8888614255565b6142bb565b90614183565b81526135e2565b6135df6135c78683602001518460c00151613722565b6135da8684602001518560e00151613722565b61439b565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff166002811061365a5761365a616a02565b602002015190505f61366c8a8a6143b0565b9050805f03613688575f5f5f5f95509550955095505050613133565b5f613697838c6080015161449e565b90505f6136a4828a613dba565b90505f89156136c9578184106136c3576136be8483614183565b6136cb565b5f6136cb565b5f5b90505f6136d8858d613dba565b90505f8c156136fd578482106136f7576136f28286614183565b6136ff565b5f6136ff565b5f5b905061370b8587616c9a565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f036137565783828161374c5761374c616cad565b04925050506102ce565b8084116137765760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b61380a604051806040016040528060068152602001652573202d257360d01b81525083836144ce565b5050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6138866040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613890845f61451a565b6020830152815260608401516138a6905f613dfb565b6060820181905281516138cb91676765c793fa10079d601b1b90612d7290600a616c8f565b604082015260208101515f036138e6575f6080820152613986565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561395c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061398091906169d0565b60808201525b61399184600161451a565b602083018190529082525f036139ac575f60a0820152613a4c565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613a22573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a4691906169d0565b60a08201525b80608001518160a001519250925050915091565b5f5f613a6c8484614560565b9050806001600160a01b03166321f8a72184604051602001613aad906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613add929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b1191815260200190565b602060405180830381865afa158015613b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030d91906169e7565b5f5f613b5c8484614560565b9050806001600160a01b031663bd02d0f584604051602001613baf9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613bdf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c1391815260200190565b602060405180830381865afa158015613c2e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030d91906169d0565b5f5f613c5f8560016143b0565b90508215613c7457613c718482616a2a565b90505b5f613c7e8761461b565b90505f613c8b8383613dba565b875160200151606001519091505f908210613cb95787516020015160600151613cb49083616a2a565b613cbb565b5f5b9050613cea6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b815250856137e1565b613d1a6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b815250846137e1565b610b09604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b815250836137e1565b5f6102ce838361465f565b613d60616011565b6102ce8383614678565b5f5f5f5f5f613d798888613d4d565b9050613d878787835f615896565b909350915081613d98575f19613da2565b613da28383613000565b9450613dad886105c9565b9350939792965093509350565b5f81156b019d971e4fe8401e740000001983900484111517613dda575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff851601613e1b575060ff60601b19905060605b90198416901c905092915050565b5f818311613e4057613e3b8383616a2a565b6102ce565b6102ce8284616a2a565b5f6102ce83676765c793fa10079d601b1b612d7285600a616c8f565b5f828411613e7c57613e7782616a3d565b61030d565b5092915050565b5f613ebd6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613ec98686865f615896565b60208301528082521580613efa5750845160ff841660028110613eee57613eee616a02565b6020020151602001515f145b15613f08575f9150506103d6565b613f1187615a02565b604082018190526020820151613f2691613dba565b6080820181905281511015613f3e575f9150506103d6565b60808101518151613f4f9190616a2a565b816060018181525050613f66866060015184613dfb565b60a082018190526060820151613f9291613f8190600a616c8f565b676765c793fa10079d601b1b613722565b60c08201525f1960ff841601613fb75760c0810151613fb19085613000565b60c08201525b845160ff841660028110613fcd57613fcd616a02565b6020020151602001518160c00151111561400a57845160ff841660028110613ff757613ff7616a02565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901614038575051602001516060015190565b81516020015160e00151614053575051602001516080015190565b505f919050565b5f84151583851114610ada5761406f82616a3d565b6103d6565b5f5f61407f876105c9565b90505f61408c8287613dba565b90505f6140998386613dba565b90505f6140a68984616cc1565b90505f6140b38389616cc1565b90505f6140bf83615a48565b90505f6140cb83615a48565b90505f841380156140db57505f83125b806140ef57505f841280156140ef57505f83135b15614103575f9750505050505050506103d6565b805f03614119575f9750505050505050506103d6565b6141238282613000565b9d9c50505050505050505050505050565b5f815f0361414357505f6102df565b5f82841161415a576141558484616a2a565b614164565b6141648385616a2a565b90505f6141718285613000565b905083851161030d5761406f81616a3d565b5f8261418f8382616a2a565b91508111156102df5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f811561138819839004841115176141f3575f5ffd5b506127109102611388010490565b5f8261420d8382616c9a565b91508110156102df5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016141d4565b5f8115806142785750828261426a8183616ce0565b92506142769083616cf7565b145b6102df5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016141d4565b5f815f036142ca57505f919050565b5f60016142d684615a5d565b901c6001901b905060018184816142ef576142ef616cad565b048201901c9050600181848161430757614307616cad565b048201901c9050600181848161431f5761431f616cad565b048201901c9050600181848161433757614337616cad565b048201901c9050600181848161434f5761434f616cad565b048201901c9050600181848161436757614367616cad565b048201901c9050600181848161437f5761437f616cad565b048201901c90506102ce8182858161439957614399616cad565b045b5f8183106143a957816102ce565b5090919050565b5f5f835f01518360ff16600281106143ca576143ca616a02565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614423573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061444791906169d0565b9050805f0361445a575f925050506102df565b606082015160c083015161446e9082616c9a565b82106144925760c08301516144838284616a2a565b61448d9190616a2a565b614494565b5f5b9695505050505050565b5f8260a001515f036144b157505f6102df565b5f6144bc8484615af0565b60a085015190915061030d9082613dba565b6145158383836040516024016144e693929190616d16565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052615b33565b505050565b5f5f5f614547855f01518560ff166002811061453857614538616a02565b6020020151866080015161449e565b90505f61455486866143b0565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001614583906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156145d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145fb9190616ade565b6102ce57604051637357d91f60e01b8152600481018490526024016141d4565b5f816001600160a01b031663bd02d0f56040516020016103fe9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f561070184615b3f565b614680616011565b82614689616011565b816001600160a01b03166391d4403c6040516020016146c9906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561471d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147419190616ade565b61474e5791506102df9050565b816001600160a01b031663bd02d0f585604051602001614788906020808252600690820152651413d4d7d25160d21b604082015260600190565b604051602081830303815290604052805190602001206040516020016147b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147ec91815260200190565b602060405180830381865afa158015614807573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061482b91906169d0565b816020018181525050816001600160a01b03166321f8a72185604051602001614873906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b604051602081830303815290604052805190602001206040516020016148a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148d791815260200190565b602060405180830381865afa1580156148f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491691906169e7565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614972906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b604051602081830303815290604052805190602001206040516020016149a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149d691815260200190565b602060405180830381865afa1580156149f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a1591906169e7565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614a90929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ac491815260200190565b602060405180830381865afa158015614adf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b0391906169d0565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614b579060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614b87929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bbb91815260200190565b602060405180830381865afa158015614bd6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614bfa91906169d0565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614c54906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c84929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cb891815260200190565b602060405180830381865afa158015614cd3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614cf791906169d0565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001614d50906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d80929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614db491815260200190565b602060405180830381865afa158015614dcf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614df391906169d0565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614e79929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ead91815260200190565b602060405180830381865afa158015614ec8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614eec91906169d0565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614f46906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f76929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614faa91815260200190565b602060405180830381865afa158015614fc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fe991906169d0565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161505c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161509091815260200190565b602060405180830381865afa1580156150ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150cf91906169d0565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615143929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161517791815260200190565b602060405180830381865afa158015615192573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151b691906169e7565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525d91815260200190565b602060405180830381865afa158015615278573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061529c91906169d0565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016152f19060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615321929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161535591815260200190565b602060405180830381865afa158015615370573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061539491906169d0565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016153ef90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161541f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161545391815260200190565b602060405180830381865afa15801561546e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061549291906169d0565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016154ec90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161551c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161555091815260200190565b602060405180830381865afa15801561556b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061558f91906169d0565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016155f19060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615621929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161565591815260200190565b602060405180830381865afa158015615670573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061569491906169d0565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016156ef90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161571f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161575391815260200190565b602060405180830381865afa15801561576e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061579291906169d0565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f5856040516020016157e1906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615811929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161584591815260200190565b602060405180830381865afa158015615860573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061588491906169d0565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615939575f5f6158c18a8a5f615bda565b915091505f6158dd5f8c60600151613dfb90919063ffffffff16565b90505f6158fb84676765c793fa10079d601b1b612d7285600a616c8f565b90505f61591984676765c793fa10079d601b1b612d7286600a616c8f565b90506159258288616c9a565b96506159318187616c9a565b955050505050505b865160200151516001600160a01b038681169116146159f5575f5f6159608a8a6001615bda565b915091505f61597d60018c60600151613dfb90919063ffffffff16565b90505f61599b84676765c793fa10079d601b1b612d7285600a616c8f565b90505f6159b984676765c793fa10079d601b1b612d7286600a616c8f565b90505f6159c6838d613dba565b90505f6159d3838e613dba565b90506159df828a616c9a565b98506159eb8189616c9a565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615a5957815f036102df565b5090565b5f80608083901c15615a7157608092831c92015b604083901c15615a8357604092831c92015b602083901c15615a9557602092831c92015b601083901c15615aa757601092831c92015b600883901c15615ab957600892831c92015b600483901c15615acb57600492831c92015b600283901c15615add57600292831c92015b600183901c156102df5760010192915050565b5f428203615b03575060208201516102df565b5f615b12846040015184615c72565b9050615b2b846020015182613dba90919063ffffffff16565b9150506102df565b615b3c81615ca6565b50565b80518051515f918291615b539160016126e2565b905080604051602001615b8c90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b60405160208183030381529060405280519060200120604051602001615bbc929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff1660028110615bf557615bf5616a02565b60200201516040015190505f615c2b875f01518660ff1660028110615c1c57615c1c616a02565b60200201518860800151615af0565b90508115615c4257615c3d8282613dba565b615c44565b5f5b865190935060ff861660028110615c5d57615c5d616a02565b60200201516020015193505050935093915050565b5f80615c7e8342616a2a565b615c889085616ce0565b6301e133809004905061030d81676765c793fa10079d601b1b616c9a565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101a00160405280615cd9616037565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b604051806101200160405280615d586160be565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280615daa616139565b81526020015f81526020015f81525090565b6040518060a00160405280615dcf61618f565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c00160405280615e53615dbc565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615ea8615cc5565b905290565b604051806104000160405280615ec1616011565b8152602001615ece615dbc565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060400160405280616004615dbc565b8152602001615ea8615d97565b60405180606001604052806160246161f6565b81525f6020820181905260409091015290565b60405180604001604052806002905b6160a86040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816160465790505090565b60405180604001604052806002905b6161236040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816160cd5790505090565b60405180604001604052806002905b61617960405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816161485790505090565b60405180604001604052806002905b6161e06040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161619e5790505090565b60405180604001604052806002905b61624e6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816162055790505090565b6001600160a01b0381168114615b3c575f5ffd5b5f60208284031215616288575f5ffd5b81356102ce81616264565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057868503603f1901845281518051606080885260a08801919088015f5b600281101561637357898403605f19018252825180516001600160a01b0316855260208082015160809187018290529061634790870182616293565b60408381015190880152606092830151929096019190915250602092830192919091019060010161630b565b505050602082810151888201526040928301519290970191909152949384019391909101906001016162e7565b50929695505050505050565b5f5f5f5f608085870312156163bf575f5ffd5b84356163ca81616264565b935060208501356163da81616264565b925060408501356163ea81616264565b9396929550929360600135925050565b5f5f5f6060848603121561640c575f5ffd5b833561641781616264565b95602085013595506040909401359392505050565b5f8260408101835f5b60028110156164f7578383038752815180516001600160a01b031684526020810151610180602086015261646d610180860182616293565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616435565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057603f19878603018452815180516101a087526165506101a088018261642c565b9050602082015161656c60208901826001600160a01b03169052565b50604082015161658760408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e08201516165ca60e08901826001600160a01b03169052565b506101008201516165e061010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616528565b5f5f60408385031215616632575f5ffd5b823561663d81616264565b9150602083013561664d81616264565b809150509250929050565b5f8260408101835f5b60028110156164f7578383038752815180516001600160a01b0316845260208101516101406020860152616699610140860182616293565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616661565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057603f19878603018452815180516101208752616759610120880182616658565b905060208201516020880152604082015161677f60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616731565b60ff81168114615b3c575f5ffd5b5f5f5f5f5f60a086880312156167f3575f5ffd5b85356167fe81616264565b9450602086013561680e81616264565b9350604086013561681e81616264565b9250606086013591506080860135616835816167d1565b809150509295509295909350565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561688057616880616843565b604052919050565b5f67ffffffffffffffff8211156168a1576168a1616843565b5060051b60200190565b5f5f604083850312156168bc575f5ffd5b82356168c781616264565b9150602083013567ffffffffffffffff8111156168e2575f5ffd5b8301601f810185136168f2575f5ffd5b803561690561690082616888565b616857565b8082825260208201915060208360051b850101925087831115616926575f5ffd5b6020840193505b8284101561694857833582526020938401939091019061692d565b809450505050509250929050565b5f5f5f5f5f60a0868803121561696a575f5ffd5b853561697581616264565b9450602086013561698581616264565b9350604086013561699581616264565b94979396509394606081013594506080013592915050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f602082840312156169e0575f5ffd5b5051919050565b5f602082840312156169f7575f5ffd5b81516102ce81616264565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156102df576102df616a16565b5f600160ff1b8201616a5157616a51616a16565b505f0390565b5f60208284031215616a67575f5ffd5b815167ffffffffffffffff811115616a7d575f5ffd5b8201601f81018413616a8d575f5ffd5b8051616a9b61690082616888565b8082825260208201915060208360051b850101925086831115616abc575f5ffd5b6020840193505b82841015614494578351825260209384019390910190616ac3565b5f60208284031215616aee575f5ffd5b815180151581146102ce575f5ffd5b5f60208284031215616b0d575f5ffd5b815167ffffffffffffffff811115616b23575f5ffd5b8201601f81018413616b33575f5ffd5b805167ffffffffffffffff811115616b4d57616b4d616843565b616b60601f8201601f1916602001616857565b818152856020838501011115616b74575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215616ba1575f5ffd5b81516102ce816167d1565b6001815b6001841115616be757808504811115616bcb57616bcb616a16565b6001841615616bd957908102905b60019390931c928002616bb0565b935093915050565b5f82616bfd575060016102df565b81616c0957505f6102df565b8160018114616c1f5760028114616c2957616c45565b60019150506102df565b60ff841115616c3a57616c3a616a16565b50506001821b6102df565b5060208310610133831016604e8410600b8410161715616c68575081810a6102df565b616c745f198484616bac565b805f1904821115616c8757616c87616a16565b029392505050565b5f6102ce8383616bef565b808201808211156102df576102df616a16565b634e487b7160e01b5f52601260045260245ffd5b8181035f831280158383131683831282161715613e7c57613e7c616a16565b80820281158282048414176102df576102df616a16565b5f82616d1157634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f616d286060830186616293565b8281036020840152616d3a8186616293565b91505082604083015294935050505056fea2646970667358221220fccb42db6896b4ab1d781fc86b4ff7de89e45921b9ac5832e846131a8f3cacc864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pam\x81\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x93W\x80c\xD2\x8B\n\x15\x11a\0cW\x80c\xD2\x8B\n\x15\x14a\x02iW\x80c\xE35\xAD\xB7\x14a\x02|W\x80c\xEE\xD0t(\x14a\x02\x8FW\x80c\xF6\x8Aq1\x14a\x02\xA2W__\xFD[\x80cs\x91\x18\xA4\x14a\x01\xF5W\x80cx\xF2\x12\xD1\x14a\x02\x15W\x80c\x8Flz<\x14a\x02(W\x80c\xC2\xBD\xED\xA1\x14a\x02;W__\xFD[\x80cP\xEDY-\x11a\0\xCEW\x80cP\xEDY-\x14a\x01\x8EW\x80cW\xB9\x1C\xA6\x14a\x01\xAFW\x80cZoW\x10\x14a\x01\xC2W\x80c\\9\xF4g\x14a\x01\xD5W__\xFD[\x80c\x1A0\x81u\x14a\0\xFFW\x80c(\xA0\xCC\xF4\x14a\x01(W\x80c1{P\xEC\x14a\x01SW\x80c8/\xC7.\x14a\x01{W[__\xFD[a\x01\x12a\x01\r6`\x04abxV[a\x02\xB5V[`@Qa\x01\x1F\x91\x90ab\xC1V[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x0166`\x04abxV[a\x02\xD5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1FV[a\x01fa\x01a6`\x04ac\xACV[a\x02\xE5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[a\x01\x12a\x01\x896`\x04ac\xFAV[a\x03\0V[a\x01\xA1a\x01\x9C6`\x04abxV[a\x03\x15V[`@Q\x90\x81R` \x01a\x01\x1FV[a\x01\xA1a\x01\xBD6`\x04abxV[a\x03\x1FV[a\x01\xA1a\x01\xD06`\x04abxV[a\x03)V[a\x01\xE8a\x01\xE36`\x04abxV[a\x033V[`@Qa\x01\x1F\x91\x90ae\x02V[a\x02\x08a\x02\x036`\x04af!V[a\x03LV[`@Qa\x01\x1F\x91\x90ag\x0BV[a\x01;a\x02#6`\x04abxV[a\x03gV[a\x01\xE8a\x0266`\x04ac\xFAV[a\x03qV[a\x02Na\x02I6`\x04ag\xDFV[a\x03~V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x1FV[a\x02Na\x02w6`\x04ag\xDFV[a\x03\x9FV[a\x01;a\x02\x8A6`\x04abxV[a\x03\xAFV[a\x01\x12a\x02\x9D6`\x04ah\xABV[a\x03\xB9V[a\x01\xA1a\x02\xB06`\x04aiVV[a\x03\xC5V[``_a\x02\xC1\x83a\x03\xDFV[\x90Pa\x02\xCE\x83_\x83a\x04qV[\x93\x92PPPV[_a\x02\xDF\x82a\x04\x8BV[\x92\x91PPV[__a\x02\xF3\x86\x86\x86\x86a\x05<V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03\r\x84\x84\x84a\x04qV[\x94\x93PPPPV[_a\x02\xDF\x82a\x05xV[_a\x02\xDF\x82a\x05\xC9V[_a\x02\xDF\x82a\x03\xDFV[``_a\x03?\x83a\x03\xDFV[\x90Pa\x02\xCE\x83_\x83a\x06\x1AV[``_a\x03Y\x84\x84a\x06\xE8V[\x90Pa\x03\r\x84\x84_\x84a\x07^V[_a\x02\xDF\x82a\x08.V[``a\x03\r\x84\x84\x84a\x06\x1AV[___a\x03\x8E\x88\x88\x88\x88\x88a\x08jV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[___a\x03\x8E\x88\x88\x88\x88\x88a\tZV[_a\x02\xDF\x82a\t\xD3V[``a\x02\xCE\x83\x83a\n$V[_a\x03\xD3\x86\x86\x86\x86\x86a\n\xE2V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x03\xFE\x90ai\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x042\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90ai\xD0V[``_a\x04\x7F\x85\x85\x85a\x0B\x15V[\x90Pa\x03\xD6\x85\x82a\n$V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90ai\xE7V[___a\x05I\x86\x86a\x0B\xB6V[\x90P_a\x05V\x88\x83a\x0C]V[\x90P__a\x05e\x8A\x84\x89a\x1E\xD5V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x06(\x85\x85\x85a\x0B\x15V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06EWa\x06EahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06~W\x81` \x01[a\x06ka\\\xC5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06cW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xDEW_\x83\x82\x81Q\x81\x10a\x06\x9FWa\x06\x9Faj\x02V[` \x02` \x01\x01Q\x90P_a\x06\xB4\x89\x83a!\x0EV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\xC9Wa\x06\xC9aj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\x83V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x07\x01\x84a%\xAEV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCE\x91\x90ai\xD0V[``_a\x07m\x86\x86\x86\x86a&2V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x8AWa\x07\x8AahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC3W\x81` \x01[a\x07\xB0a]DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xA8W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x08#W_\x83\x82\x81Q\x81\x10a\x07\xE4Wa\x07\xE4aj\x02V[` \x02` \x01\x01Q\x90P_a\x07\xF9\x8A\x83a&\xBAV[\x90P\x80\x84\x84\x81Q\x81\x10a\x08\x0EWa\x08\x0Eaj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x07\xC8V[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x08x\x88\x88a\x0B\xB6V[\x90P_a\x08\x85\x8A\x83a\x0C]V[\x90P_\x80\x80`\xFF\x89\x16a\x08\xBBWa\x08\x9D\x8D\x8B\x86a,\xC9V[\x92\x95P\x91\x93Pa\x08\xB4\x91P\x85\x90P_\x85\x8D\x82a-\xC1V[\x90Pa\x08\xEBV[_\x19`\xFF\x8A\x16\x01a\x08\xEBWa\x08\xD1\x8D\x8B\x86a.\x8EV[\x92\x95P\x91\x93Pa\x08\xE8\x91P\x85\x90P\x84_\x80\x8Ea-\xC1V[\x90P[_a\x08\xF5\x85a/eV[\x90P_\x82\x82\x11a\t\x0EWa\t\t\x82\x84aj*V[a\t\x18V[a\t\x18\x83\x83aj*V[\x90P_a\t%\x82\x84a0\0V[\x90P_\x84\x84\x11a\t=Wa\t8\x82aj=V[a\t?V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\th\x88\x88a\x0B\xB6V[\x90P_a\tu\x8A\x83a\x0C]V[\x90P_\x80\x80`\xFF\x89\x16a\t\xA5Wa\t\x8E\x8D\x8B\x86_a0;V[\x92\x95P\x91\x93Pa\x08\xB4\x91P\x85\x90P\x8B_\x80\x87a-\xC1V[_\x19`\xFF\x8A\x16\x01a\x08\xEBWa\t\xBC\x8D\x8B\x86_a1>V[\x92\x95P\x91\x93Pa\x08\xE8\x91P\x85\x90P_\x8C\x86\x82a-\xC1V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nAWa\nAahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nzW\x81` \x01[a\nga]\x97V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n_W\x90P[P\x90P_[\x83Q\x81\x10\x15a\n\xDAW_\x84\x82\x81Q\x81\x10a\n\x9BWa\n\x9Baj\x02V[` \x02` \x01\x01Q\x90P_a\n\xB0\x87\x83a2&V[\x90P\x80\x84\x84\x81Q\x81\x10a\n\xC5Wa\n\xC5aj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\n\x7FV[P\x93\x92PPPV[__a\n\xEE\x86\x86a\x0B\xB6V[\x90P_a\n\xFB\x88\x83a\x0C]V[\x90Pa\x0B\t\x81\x86\x86_a4\xB3V[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x0B5\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x8FW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\r\x91\x90\x81\x01\x90ajWV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0B\xD7W\x81\x83a\x0B\xDAV[\x82\x82[`@Q\x91\x94P\x92Pa\x0C\x07\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x0Cea]\xBCV[\x82a\x0Cna]\xBCV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x0C\x8C\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x04\x91\x90aj\xDEV[a\r\x11W\x91Pa\x02\xDF\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\rQ\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF4\x91\x90ai\xE7V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0Er\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE5\x91\x90ai\xD0V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0F;\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0Fk\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xDE\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10?\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xA3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE2\x91\x90ai\xD0V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11M\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11}\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xF0\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12Q\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF4\x91\x90ai\xD0V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xA5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE4\x91\x90ai\xD0V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14Y\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCC\x91\x90ai\xE7V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15v\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB5\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\x0C\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16p\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xAF\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\x11\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17u\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB4\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x18 \x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC3\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19%\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19U\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC8\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1A!\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1AQ\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x85\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xC4\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1B\x12\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1BB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Bv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB5\x91\x90ai\xE7V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1C#\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1CS\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x87\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC6\x91\x90ai\xE7V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1D)\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1DY\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xCC\x91\x90ai\xD0V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1E%\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1EU\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xC8\x91\x90ai\xD0V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1E\xE1a]\xF0V[a\x1E\xEA\x88a5\xECV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FX\x91\x90ai\xD0V[\x81` \x01\x81\x81RPPa\x1Fr\x87__\x84a\x01@\x01Qa6=V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\x1F\x94\x90\x88\x90`\x01\x90_\x90a6=V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1F\xB5\x91\x88\x91a7\"V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1F\xD0\x91\x88\x91a7\"V[\x81a\x01 \x01\x81\x81RPPa \x10`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa7\xE1V[a H`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa7\xE1V[a \x80`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa7\xE1V[a \xB3`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa7\xE1V[a \xE6`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa7\xE1V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[a!\x16a\\\xC5V[a!\x1Ea^?V[a!(\x84\x84a\x0C]V[\x81Ra!3\x84a8\x0EV[` \x82\x01Ra!A\x84a5\xECV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa!\\\x92_\x91\x90a6=V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa!\x8A\x92\x91`\x01\x91a6=V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa!\xAC\x90a8QV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\"+W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"R\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\"\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xC3\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\xA3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xCA\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$>\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a%\x1E\x90a/eV[\x81R` \x01a%-\x86\x86a:`V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a%T\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a%e\x86\x86a;PV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a%\x89\x90\x87\x90_\x80a<RV[\x81R` \x01a%\x9B\x86\x84_\x01Qa=MV[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a%\xE8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a&L\x86a%\xAEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x93W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xD6\x91\x90\x81\x01\x90ajWV[a&\xC2a]DV[a&\xCAa^\xADV[a&\xD4\x84\x84a=XV[\x80\x82RQ\x80QQa&\xED\x91`\x01[` \x02\x01QQa\x0B\xB6V[`@\x82\x01\x81\x90Ra&\xFF\x90\x85\x90a\x0C]V[` \x82\x01\x81\x90R\x81Qa'\x13\x91\x86\x91a=jV[PPPP``\x82\x01R` \x81\x01Qa'*\x90a/eV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa'`\x91\x90_[` \x02\x01Q`@\x01Q\x90a=\xBAV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa'x\x90_a=\xFBV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa'\x91\x91\x90a>)V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa'\xA8\x91\x90a>JV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa'\xC9\x92\x91\x90a>fV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa'\xEA\x92\x87\x92\x90\x91_a>\x83V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa(\x17\x92\x91\x90a>fV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa(1\x90`\x01a=\xFBV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa(b\x91\x90`\x01a'QV[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa(y\x91a>)V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa(\x91\x91\x90a>JV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa(\xA9\x91\x90a=\xBAV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa(\xCC\x92\x91\x90a>fV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa(\xEE\x92\x87\x92\x90\x91`\x01a>\x83V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa)\x1D\x92\x91\x90a>fV[a\x02\xE0\x82\x01R\x80Qa).\x90a@\x18V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a*&Wa)X\x81a\x03\0\x01Q\x82`\x80\x01Qa>)V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa)o\x91a=\xBAV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa)\xA0\x93a@ZV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa*\x07\x91\x86\x91a)\xC8\x91\x90a>JV[a)\xDA\x84`\xC0\x01Q\x85`\xA0\x01Qa>JV[a)\xEE\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa>JV[a*\x02\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa>JV[a@tV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa*\x1F\x91\x90aA4V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*\xC1\x91\x90\x81\x01\x90aj\xFDV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a+\x16Wa+\x16aj\x02V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a+eWa+eaj\x02V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xC5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xEC\x91\x90\x81\x01\x90aj\xFDV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a,\xD5a_\x8AV[a,\xDE\x88a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra,\xF5\x90\x87\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa-\x14\x90\x87\x90`\x01\x90_\x90a6=V[P``\x84\x81\x01\x91\x90\x91R` \x84\x01\x92\x90\x92RP\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra-T\x90\x88\x90a-L\x90a'\x10\x90aA\x83V[a'\x10a7\"V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa-w\x92a-r\x90\x83\x90aA\x83V[a7\"V[`\x80\x82\x01\x81\x90R` \x82\x01Qa-\x8D\x91\x90aA\x83V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa-\xAF\x90\x8B\x90aA\xDDV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a-\xD1WP\x83\x15[\x15a-\xE0WP\x83\x90P\x84a.\x15V[_\x87\x11\x80\x15a-\xEDWP\x84\x15[\x15a-\xFCWP\x85\x90P\x82a.\x15V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a.$\x89``\x01Q_a=\xFBV[\x90P_a.6\x8A``\x01Q`\x01a=\xFBV[\x90P_a.T\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_a.r\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90Pa.~\x82\x82a0\0V[\x9C\x9BPPPPPPPPPPPPV[____a.\x9Aa_\x8AV[a.\xA3\x88a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra.\xBA\x90\x87\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa.\xD9\x90\x87\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01\x81\x90R\x81Qa.\xF7\x91a-r\x81\x8BaA\x83V[a\x01\0\x82\x01\x81\x90R\x81Qa/\x0B\x91\x90aA\x83V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa/>\x91a'\x10\x90a-r\x90\x82\x90aA\x83V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa-\xAF\x91aA\xDDV[__a/s\x83___a6=V[PPP\x90P_a/\x86\x84`\x01__a6=V[PPP\x90P\x80_\x03a/\x9BWP_\x93\x92PPPV[_a/\xAA\x85``\x01Q_a=\xFBV[\x90P_a/\xBC\x86``\x01Q`\x01a=\xFBV[\x90P_a/\xDA\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_a/\xF8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90Pa\x0B\t\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a0!W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a0Ga_\x8AV[a0P\x89a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra0g\x90\x88\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa0\x86\x90\x88\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01R\x85\x15a0\xADW\x87\x81_\x01\x81\x81Qa0\xA9\x91\x90aj*V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra0\xD4\x90\x89\x90a-L\x90a'\x10\x90aA\x83V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\xF2\x92a-r\x90\x83\x90aB\x01V[`\x80\x82\x01\x81\x90R` \x82\x01Qa1\x07\x91aA\x83V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa1)\x90\x8C\x90aA\xDDV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a1Ja_\x8AV[a1S\x89a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra1j\x90\x88\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa1\x89\x90\x88\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01R\x85\x15a1\xB1W\x87\x81` \x01\x81\x81Qa1\xAD\x91\x90aj*V[\x90RP[\x80Q` \x82\x01Qa1\xC7\x91\x90a-r\x81\x8CaB\x01V[`\x80\x82\x01\x81\x90R\x81Qa1\xD9\x91aA\x83V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa2\x08\x91a-L\x90a'\x10\x90aA\x83V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa1)\x91aA\xDDV[a2.a]\x97V[a26a_\xF1V[a2@\x84\x84a\x0C]V[\x81R`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a2\xB3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xDA\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a3'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3K\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a3\xCBW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra3\xF2\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4f\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a4\xA1\x86\x84_\x01Qa=MV[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a4\xBCa]\xF0V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5 \x91\x90ai\xD0V[` \x82\x01Ra51\x86_\x80\x80a6=V[PPP`\xC0\x82\x01Ra5F\x86`\x01_\x80a6=V[PPP`\xE0\x82\x01R\x82\x15a5\x81W\x84\x81`\xC0\x01\x81\x81Qa5f\x91\x90aj*V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a5}\x90\x83\x90aj*V[\x90RP[\x80` \x01Q_\x03a5\xB1Wa5\xAAa\x03\xE8a5\xA4a5\x9F\x88\x88aBUV[aB\xBBV[\x90aA\x83V[\x81Ra5\xE2V[a5\xDFa5\xC7\x86\x83` \x01Q\x84`\xC0\x01Qa7\"V[a5\xDA\x86\x84` \x01Q\x85`\xE0\x01Qa7\"V[aC\x9BV[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a6ZWa6Zaj\x02V[` \x02\x01Q\x90P_a6l\x8A\x8AaC\xB0V[\x90P\x80_\x03a6\x88W____\x95P\x95P\x95P\x95PPPa13V[_a6\x97\x83\x8C`\x80\x01QaD\x9EV[\x90P_a6\xA4\x82\x8Aa=\xBAV[\x90P_\x89\x15a6\xC9W\x81\x84\x10a6\xC3Wa6\xBE\x84\x83aA\x83V[a6\xCBV[_a6\xCBV[_[\x90P_a6\xD8\x85\x8Da=\xBAV[\x90P_\x8C\x15a6\xFDW\x84\x82\x10a6\xF7Wa6\xF2\x82\x86aA\x83V[a6\xFFV[_a6\xFFV[_[\x90Pa7\x0B\x85\x87al\x9AV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a7VW\x83\x82\x81a7LWa7Lal\xADV[\x04\x92PPPa\x02\xCEV[\x80\x84\x11a7vW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a8\n`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aD\xCEV[PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a8\x86`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a8\x90\x84_aE\x1AV[` \x83\x01R\x81R``\x84\x01Qa8\xA6\x90_a=\xFBV[``\x82\x01\x81\x90R\x81Qa8\xCB\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a-r\x90`\nal\x8FV[`@\x82\x01R` \x81\x01Q_\x03a8\xE6W_`\x80\x82\x01Ra9\x86V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x80\x91\x90ai\xD0V[`\x80\x82\x01R[a9\x91\x84`\x01aE\x1AV[` \x83\x01\x81\x90R\x90\x82R_\x03a9\xACW_`\xA0\x82\x01Ra:LV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:F\x91\x90ai\xD0V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a:l\x84\x84aE`V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a:\xAD\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\r\x91\x90ai\xE7V[__a;\\\x84\x84aE`V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a;\xAF\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\r\x91\x90ai\xD0V[__a<_\x85`\x01aC\xB0V[\x90P\x82\x15a<tWa<q\x84\x82aj*V[\x90P[_a<~\x87aF\x1BV[\x90P_a<\x8B\x83\x83a=\xBAV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10a<\xB9W\x87Q` \x01Q``\x01Qa<\xB4\x90\x83aj*V[a<\xBBV[_[\x90Pa<\xEA`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a7\xE1V[a=\x1A`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a7\xE1V[a\x0B\t`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a7\xE1V[_a\x02\xCE\x83\x83aF_V[a=`a`\x11V[a\x02\xCE\x83\x83aFxV[_____a=y\x88\x88a=MV[\x90Pa=\x87\x87\x87\x83_aX\x96V[\x90\x93P\x91P\x81a=\x98W_\x19a=\xA2V[a=\xA2\x83\x83a0\0V[\x94Pa=\xAD\x88a\x05\xC9V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a=\xDAW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a>\x1BWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a>@Wa>;\x83\x83aj*V[a\x02\xCEV[a\x02\xCE\x82\x84aj*V[_a\x02\xCE\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[_\x82\x84\x11a>|Wa>w\x82aj=V[a\x03\rV[P\x92\x91PPV[_a>\xBD`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a>\xC9\x86\x86\x86_aX\x96V[` \x83\x01R\x80\x82R\x15\x80a>\xFAWP\x84Q`\xFF\x84\x16`\x02\x81\x10a>\xEEWa>\xEEaj\x02V[` \x02\x01Q` \x01Q_\x14[\x15a?\x08W_\x91PPa\x03\xD6V[a?\x11\x87aZ\x02V[`@\x82\x01\x81\x90R` \x82\x01Qa?&\x91a=\xBAV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a?>W_\x91PPa\x03\xD6V[`\x80\x81\x01Q\x81Qa?O\x91\x90aj*V[\x81``\x01\x81\x81RPPa?f\x86``\x01Q\x84a=\xFBV[`\xA0\x82\x01\x81\x90R``\x82\x01Qa?\x92\x91a?\x81\x90`\nal\x8FV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba7\"V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a?\xB7W`\xC0\x81\x01Qa?\xB1\x90\x85a0\0V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a?\xCDWa?\xCDaj\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a@\nW\x84Q`\xFF\x84\x16`\x02\x81\x10a?\xF7Wa?\xF7aj\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a@8WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa@SWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\n\xDAWa@o\x82aj=V[a\x03\xD6V[__a@\x7F\x87a\x05\xC9V[\x90P_a@\x8C\x82\x87a=\xBAV[\x90P_a@\x99\x83\x86a=\xBAV[\x90P_a@\xA6\x89\x84al\xC1V[\x90P_a@\xB3\x83\x89al\xC1V[\x90P_a@\xBF\x83aZHV[\x90P_a@\xCB\x83aZHV[\x90P_\x84\x13\x80\x15a@\xDBWP_\x83\x12[\x80a@\xEFWP_\x84\x12\x80\x15a@\xEFWP_\x83\x13[\x15aA\x03W_\x97PPPPPPPPa\x03\xD6V[\x80_\x03aA\x19W_\x97PPPPPPPPa\x03\xD6V[aA#\x82\x82a0\0V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aACWP_a\x02\xDFV[_\x82\x84\x11aAZWaAU\x84\x84aj*V[aAdV[aAd\x83\x85aj*V[\x90P_aAq\x82\x85a0\0V[\x90P\x83\x85\x11a\x03\rWa@o\x81aj=V[_\x82aA\x8F\x83\x82aj*V[\x91P\x81\x11\x15a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aA\xF3W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aB\r\x83\x82al\x9AV[\x91P\x81\x10\x15a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01aA\xD4V[_\x81\x15\x80aBxWP\x82\x82aBj\x81\x83al\xE0V[\x92PaBv\x90\x83al\xF7V[\x14[a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01aA\xD4V[_\x81_\x03aB\xCAWP_\x91\x90PV[_`\x01aB\xD6\x84aZ]V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aB\xEFWaB\xEFal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x07WaC\x07al\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x1FWaC\x1Fal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC7WaC7al\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aCOWaCOal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aCgWaCgal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x7FWaC\x7Fal\xADV[\x04\x82\x01\x90\x1C\x90Pa\x02\xCE\x81\x82\x85\x81aC\x99WaC\x99al\xADV[\x04[_\x81\x83\x10aC\xA9W\x81a\x02\xCEV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aC\xCAWaC\xCAaj\x02V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDG\x91\x90ai\xD0V[\x90P\x80_\x03aDZW_\x92PPPa\x02\xDFV[``\x82\x01Q`\xC0\x83\x01QaDn\x90\x82al\x9AV[\x82\x10aD\x92W`\xC0\x83\x01QaD\x83\x82\x84aj*V[aD\x8D\x91\x90aj*V[aD\x94V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aD\xB1WP_a\x02\xDFV[_aD\xBC\x84\x84aZ\xF0V[`\xA0\x85\x01Q\x90\x91Pa\x03\r\x90\x82a=\xBAV[aE\x15\x83\x83\x83`@Q`$\x01aD\xE6\x93\x92\x91\x90am\x16V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra[3V[PPPV[___aEG\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aE8WaE8aj\x02V[` \x02\x01Q\x86`\x80\x01QaD\x9EV[\x90P_aET\x86\x86aC\xB0V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aE\x83\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xFB\x91\x90aj\xDEV[a\x02\xCEW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01aA\xD4V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x07\x01\x84a[?V[aF\x80a`\x11V[\x82aF\x89a`\x11V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aF\xC9\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGA\x91\x90aj\xDEV[aGNW\x91Pa\x02\xDF\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\x88\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH+\x91\x90ai\xD0V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aHs\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x16\x91\x90ai\xE7V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aIr\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x15\x91\x90ai\xE7V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xC4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x03\x91\x90ai\xD0V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aKW\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xBB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\xFA\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aLT\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x84\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xB8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xF7\x91\x90ai\xD0V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aMP\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x80\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xB4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xF3\x91\x90ai\xD0V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNy\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xAD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xEC\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOF\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aOv\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xAA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xE9\x91\x90ai\xD0V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\x90\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xCF\x91\x90ai\xD0V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQw\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\x92W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xB6\x91\x90ai\xE7V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR]\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRxW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x9C\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\xF1\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aSpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x94\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aS\xEF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT\x1F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTS\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aTnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\x92\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\xEC\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aUkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x8F\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aU\xF1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x94\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xEF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW\x1F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWS\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aWnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\x92\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xE1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\x11\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x84\x91\x90ai\xD0V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aY9W__aX\xC1\x8A\x8A_a[\xDAV[\x91P\x91P_aX\xDD_\x8C``\x01Qa=\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aX\xFB\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[\x90P_aY\x19\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90PaY%\x82\x88al\x9AV[\x96PaY1\x81\x87al\x9AV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aY\xF5W__aY`\x8A\x8A`\x01a[\xDAV[\x91P\x91P_aY}`\x01\x8C``\x01Qa=\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aY\x9B\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[\x90P_aY\xB9\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_aY\xC6\x83\x8Da=\xBAV[\x90P_aY\xD3\x83\x8Ea=\xBAV[\x90PaY\xDF\x82\x8Aal\x9AV[\x98PaY\xEB\x81\x89al\x9AV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aZYW\x81_\x03a\x02\xDFV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aZqW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aZ\x83W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aZ\x95W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aZ\xA7W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aZ\xB9W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aZ\xCBW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aZ\xDDW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x02\xDFW`\x01\x01\x92\x91PPV[_B\x82\x03a[\x03WP` \x82\x01Qa\x02\xDFV[_a[\x12\x84`@\x01Q\x84a\\rV[\x90Pa[+\x84` \x01Q\x82a=\xBA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x02\xDFV[a[<\x81a\\\xA6V[PV[\x80Q\x80QQ_\x91\x82\x91a[S\x91`\x01a&\xE2V[\x90P\x80`@Q` \x01a[\x8C\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10a[\xF5Wa[\xF5aj\x02V[` \x02\x01Q`@\x01Q\x90P_a\\+\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10a\\\x1CWa\\\x1Caj\x02V[` \x02\x01Q\x88`\x80\x01QaZ\xF0V[\x90P\x81\x15a\\BWa\\=\x82\x82a=\xBAV[a\\DV[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10a\\]Wa\\]aj\x02V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80a\\~\x83Baj*V[a\\\x88\x90\x85al\xE0V[c\x01\xE13\x80\x90\x04\x90Pa\x03\r\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bal\x9AV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01\xA0\x01`@R\x80a\\\xD9a`7V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80a]Xa`\xBEV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a]\xAAaa9V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80a]\xCFaa\x8FV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80a^Sa]\xBCV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a^\xA8a\\\xC5V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80a^\xC1a`\x11V[\x81R` \x01a^\xCEa]\xBCV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80a`\x04a]\xBCV[\x81R` \x01a^\xA8a]\x97V[`@Q\x80``\x01`@R\x80a`$aa\xF6V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a`\xA8`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a`FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aa#`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a`\xCDW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aay`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aaHW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aa\xE0`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aa\x9EW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[abN`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ab\x05W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a[<W__\xFD[_` \x82\x84\x03\x12\x15ab\x88W__\xFD[\x815a\x02\xCE\x81abdV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15acsW\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90acG\x90\x87\x01\x82ab\x93V[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ac\x0BV[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ab\xE7V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15ac\xBFW__\xFD[\x845ac\xCA\x81abdV[\x93P` \x85\x015ac\xDA\x81abdV[\x92P`@\x85\x015ac\xEA\x81abdV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ad\x0CW__\xFD[\x835ad\x17\x81abdV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ad\xF7W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Radma\x01\x80\x86\x01\x82ab\x93V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pad5V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87RaePa\x01\xA0\x88\x01\x82ad,V[\x90P` \x82\x01Qael` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qae\x87`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qae\xCA`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qae\xE0a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ae(V[__`@\x83\x85\x03\x12\x15af2W__\xFD[\x825af=\x81abdV[\x91P` \x83\x015afM\x81abdV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ad\xF7W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Raf\x99a\x01@\x86\x01\x82ab\x93V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PafaV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RagYa\x01 \x88\x01\x82afXV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qag\x7F`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ag1V[`\xFF\x81\x16\x81\x14a[<W__\xFD[_____`\xA0\x86\x88\x03\x12\x15ag\xF3W__\xFD[\x855ag\xFE\x81abdV[\x94P` \x86\x015ah\x0E\x81abdV[\x93P`@\x86\x015ah\x1E\x81abdV[\x92P``\x86\x015\x91P`\x80\x86\x015ah5\x81ag\xD1V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15ah\x80Wah\x80ahCV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah\xA1Wah\xA1ahCV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ah\xBCW__\xFD[\x825ah\xC7\x81abdV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\xE2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13ah\xF2W__\xFD[\x805ai\x05ai\0\x82ah\x88V[ahWV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ai&W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aiHW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ai-V[\x80\x94PPPPP\x92P\x92\x90PV[_____`\xA0\x86\x88\x03\x12\x15aijW__\xFD[\x855aiu\x81abdV[\x94P` \x86\x015ai\x85\x81abdV[\x93P`@\x86\x015ai\x95\x81abdV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15ai\xE0W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ai\xF7W__\xFD[\x81Qa\x02\xCE\x81abdV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xDFWa\x02\xDFaj\x16V[_`\x01`\xFF\x1B\x82\x01ajQWajQaj\x16V[P_\x03\x90V[_` \x82\x84\x03\x12\x15ajgW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aj}W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aj\x8DW__\xFD[\x80Qaj\x9Bai\0\x82ah\x88V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aj\xBCW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aD\x94W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aj\xC3V[_` \x82\x84\x03\x12\x15aj\xEEW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\xCEW__\xFD[_` \x82\x84\x03\x12\x15ak\rW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ak#W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ak3W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15akMWakMahCV[ak``\x1F\x82\x01`\x1F\x19\x16` \x01ahWV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aktW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ak\xA1W__\xFD[\x81Qa\x02\xCE\x81ag\xD1V[`\x01\x81[`\x01\x84\x11\x15ak\xE7W\x80\x85\x04\x81\x11\x15ak\xCBWak\xCBaj\x16V[`\x01\x84\x16\x15ak\xD9W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ak\xB0V[\x93P\x93\x91PPV[_\x82ak\xFDWP`\x01a\x02\xDFV[\x81al\tWP_a\x02\xDFV[\x81`\x01\x81\x14al\x1FW`\x02\x81\x14al)WalEV[`\x01\x91PPa\x02\xDFV[`\xFF\x84\x11\x15al:Wal:aj\x16V[PP`\x01\x82\x1Ba\x02\xDFV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15alhWP\x81\x81\na\x02\xDFV[alt_\x19\x84\x84ak\xACV[\x80_\x19\x04\x82\x11\x15al\x87Wal\x87aj\x16V[\x02\x93\x92PPPV[_a\x02\xCE\x83\x83ak\xEFV[\x80\x82\x01\x80\x82\x11\x15a\x02\xDFWa\x02\xDFaj\x16V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a>|Wa>|aj\x16V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xDFWa\x02\xDFaj\x16V[_\x82am\x11WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_am(``\x83\x01\x86ab\x93V[\x82\x81\x03` \x84\x01Ram:\x81\x86ab\x93V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xFC\xCBB\xDBh\x96\xB4\xAB\x1Dx\x1F\xC8kO\xF7\xDE\x89\xE4Y!\xB9\xACX2\xE8F\x13\x1A\x8F<\xAC\xC8dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c8063739118a411610093578063d28b0a1511610063578063d28b0a1514610269578063e335adb71461027c578063eed074281461028f578063f68a7131146102a2575f5ffd5b8063739118a4146101f557806378f212d1146102155780638f6c7a3c14610228578063c2bdeda11461023b575f5ffd5b806350ed592d116100ce57806350ed592d1461018e57806357b91ca6146101af5780635a6f5710146101c25780635c39f467146101d5575f5ffd5b80631a308175146100ff57806328a0ccf414610128578063317b50ec14610153578063382fc72e1461017b575b5f5ffd5b61011261010d366004616278565b6102b5565b60405161011f91906162c1565b60405180910390f35b61013b610136366004616278565b6102d5565b6040516001600160a01b03909116815260200161011f565b6101666101613660046163ac565b6102e5565b6040805192835260208301919091520161011f565b6101126101893660046163fa565b610300565b6101a161019c366004616278565b610315565b60405190815260200161011f565b6101a16101bd366004616278565b61031f565b6101a16101d0366004616278565b610329565b6101e86101e3366004616278565b610333565b60405161011f9190616502565b610208610203366004616621565b61034c565b60405161011f919061670b565b61013b610223366004616278565b610367565b6101e86102363660046163fa565b610371565b61024e6102493660046167df565b61037e565b6040805193845260208401929092529082015260600161011f565b61024e6102773660046167df565b61039f565b61013b61028a366004616278565b6103af565b61011261029d3660046168ab565b6103b9565b6101a16102b0366004616956565b6103c5565b60605f6102c1836103df565b90506102ce835f83610471565b9392505050565b5f6102df8261048b565b92915050565b5f5f6102f38686868661053c565b9150915094509492505050565b606061030d848484610471565b949350505050565b5f6102df82610578565b5f6102df826105c9565b5f6102df826103df565b60605f61033f836103df565b90506102ce835f8361061a565b60605f61035984846106e8565b905061030d84845f8461075e565b5f6102df8261082e565b606061030d84848461061a565b5f5f5f61038e888888888861086a565b925092509250955095509592505050565b5f5f5f61038e888888888861095a565b5f6102df826109d3565b60606102ce8383610a24565b5f6103d38686868686610ae2565b90505b95945050505050565b5f816001600160a01b031663f3903b9f6040516020016103fe906169ad565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043291815260200190565b602060405180830381865afa15801561044d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df91906169d0565b60605f61047f858585610b15565b90506103d68582610a24565b5f816001600160a01b03166321f8a7216040516020016104c9906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016104fd91815260200190565b602060405180830381865afa158015610518573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df91906169e7565b5f5f5f6105498686610bb6565b90505f6105568883610c5d565b90505f5f6105658a8489611ed5565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f56040516020016103fe9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f610628858585610b15565b90505f815167ffffffffffffffff81111561064557610645616843565b60405190808252806020026020018201604052801561067e57816020015b61066b615cc5565b8152602001906001900390816106635790505b5090505f5b82518110156106de575f83828151811061069f5761069f616a02565b602002602001015190505f6106b4898361210e565b9050808484815181106106c9576106c9616a02565b60209081029190910101525050600101610683565b5095945050505050565b5f826001600160a01b031663f3903b9f610701846125ae565b6040518263ffffffff1660e01b815260040161071f91815260200190565b602060405180830381865afa15801561073a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102ce91906169d0565b60605f61076d86868686612632565b90505f815167ffffffffffffffff81111561078a5761078a616843565b6040519080825280602002602001820160405280156107c357816020015b6107b0615d44565b8152602001906001900390816107a85790505b5090505f5b8251811015610823575f8382815181106107e4576107e4616a02565b602002602001015190505f6107f98a836126ba565b90508084848151811061080e5761080e616a02565b602090810291909101015250506001016107c8565b509695505050505050565b5f816001600160a01b03166321f8a7216040516020016104c990602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6108788888610bb6565b90505f6108858a83610c5d565b90505f808060ff89166108bb5761089d8d8b86612cc9565b9295509193506108b491508590505f858d82612dc1565b90506108eb565b5f1960ff8a16016108eb576108d18d8b86612e8e565b9295509193506108e89150859050845f808e612dc1565b90505b5f6108f585612f65565b90505f82821161090e576109098284616a2a565b610918565b6109188383616a2a565b90505f6109258284613000565b90505f84841161093d5761093882616a3d565b61093f565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f6109688888610bb6565b90505f6109758a83610c5d565b90505f808060ff89166109a55761098e8d8b865f61303b565b9295509193506108b491508590508b5f8087612dc1565b5f1960ff8a16016108eb576109bc8d8b865f61313e565b9295509193506108e891508590505f8c8682612dc1565b5f816001600160a01b03166321f8a7216040516020016104c9906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60605f825167ffffffffffffffff811115610a4157610a41616843565b604051908082528060200260200182016040528015610a7a57816020015b610a67615d97565b815260200190600190039081610a5f5790505b5090505f5b8351811015610ada575f848281518110610a9b57610a9b616a02565b602002602001015190505f610ab08783613226565b905080848481518110610ac557610ac5616a02565b60209081029190910101525050600101610a7f565b509392505050565b5f5f610aee8686610bb6565b90505f610afb8883610c5d565b9050610b098186865f6134b3565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001610b35906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015610b8f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261030d9190810190616a57565b5f816001600160a01b0316836001600160a01b031610610bd7578183610bda565b82825b6040519194509250610c07906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610c65615dbc565b82610c6e615dbc565b816001600160a01b03166391d4403c604051602001610c8c906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610ce0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d049190616ade565b610d115791506102df9050565b816001600160a01b03166321f8a72185604051602001610d51906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610d81929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610db591815260200190565b602060405180830381865afa158015610dd0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df491906169e7565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610e72929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610ea691815260200190565b602060405180830381865afa158015610ec1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee591906169d0565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610f3b906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610f6b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f9f91815260200190565b602060405180830381865afa158015610fba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fde91906169d0565b81515f60200201516040018181525050816001600160a01b031663bd02d0f58560405160200161103f9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161106f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110a391815260200190565b602060405180830381865afa1580156110be573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110e291906169d0565b81515f60200201516060018181525050816001600160a01b031663bd02d0f58560405160200161114d9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b6040516020818303038152906040528051906020012060405160200161117d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016111b191815260200190565b602060405180830381865afa1580156111cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111f091906169d0565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016112519060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611281929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016112b591815260200190565b602060405180830381865afa1580156112d0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f491906169d0565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611371929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016113a591815260200190565b602060405180830381865afa1580156113c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113e491906169d0565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611459929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161148d91815260200190565b602060405180830381865afa1580156114a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114cc91906169e7565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161157691815260200190565b602060405180830381865afa158015611591573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115b591906169d0565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161160c90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161163c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161167091815260200190565b602060405180830381865afa15801561168b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116af91906169d0565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016117119060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611741929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161177591815260200190565b602060405180830381865afa158015611790573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117b491906169d0565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016118209060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b60405160208183030381529060405280519060200120604051602001611850929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161188491815260200190565b602060405180830381865afa15801561189f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118c391906169d0565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016119259060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611955929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161198991815260200190565b602060405180830381865afa1580156119a4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119c891906169d0565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001611a2190602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a51929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a8591815260200190565b602060405180830381865afa158015611aa0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ac491906169d0565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001611b1290602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611b42929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b7691815260200190565b602060405180830381865afa158015611b91573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bb591906169e7565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611c23906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611c53929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611c8791815260200190565b602060405180830381865afa158015611ca2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cc691906169e7565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611d29906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d59929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611d8d91815260200190565b602060405180830381865afa158015611da8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dcc91906169d0565b60608201526040516001600160a01b0383169063bd02d0f5908690611e25906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611e55929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611e8991815260200190565b602060405180830381865afa158015611ea4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ec891906169d0565b6080820152949350505050565b5f5f5f5f611ee1615df0565b611eea886135ec565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f34573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5891906169d0565b816020018181525050611f72875f5f84610140015161363d565b506080840152506040820152610140810151611f949088906001905f9061363d565b5060a084015250606082015260408101516020820151611fb5918891613722565b61010082015260608101516020820151611fd0918891613722565b816101200181815250506120106040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b81525082602001516137e1565b612048604051806040016040528060128152602001710766172732e707269636552657365727665360741b81525082604001516137e1565b61208060405180604001604052806012815260200171766172732e7072696365526573657276653160701b81525082606001516137e1565b6120b36040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b8152508261010001516137e1565b6120e66040518060400160405280600c81526020016b766172732e616d6f756e743160a01b8152508261012001516137e1565b80610100015181610120015182608001518360a0015194509450945094505093509350935093565b612116615cc5565b61211e615e3f565b6121288484610c5d565b81526121338461380e565b6020820152612141846135ec565b61018082018190528151602083015161215c925f919061363d565b608085015260a0840152604083015260608201528051602082015161018083015161218a929160019161363d565b61012085015261014084015260e083015261010082015280516121ac90613851565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa15801561222b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526122529190810190616afd565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561229f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122c39190616b91565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156123a3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123ca9190810190616afd565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801561241a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061243e9190616b91565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061251e90612f65565b815260200161252d8686613a60565b6001600160a01b03168152602001612554835f015160600151660800000000000016151590565b151581526020016125658686613b50565b81528251515160c00151602082015282516040909101906125899087905f80613c52565b815260200161259b86845f0151613d4d565b90526101a0909101819052905092915050565b5f6040516020016125e8906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61264c866125ae565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612693573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103d69190810190616a57565b6126c2615d44565b6126ca615ead565b6126d48484613d58565b808252518051516126ed9160015b602002015151610bb6565b604082018190526126ff908590610c5d565b602082018190528151612713918691613d6a565b505050506060820152602081015161272a90612f65565b610300820152805180515160209081015160e08401528083015151510151905161276091905f5b60200201516040015190613dba565b60c0820152602081015160600151612778905f613dfb565b60a082015260e081015160c08201516127919190613e29565b610100820181905260a08201516127a89190613e4a565b61012082015260e081015160c08201516101008301516127c9929190613e66565b610140820152602081015181516103008301516127ea92879290915f613e83565b61016082015261014081015161018082015260e081015160c0820151610120830151612817929190613e66565b6101a0820152602081015160600151612831906001613dfb565b6101c08201528051805160209081015181015161020084015280830151518101510151905161286291906001612751565b6101e0820181905261020082015161287991613e29565b61022082018190526101c08201516128919190613e4a565b61024082018190526103008201516128a99190613dba565b6102608201526102008101516101e08201516102208301516128cc929190613e66565b610280820152602081015181516103008301516128ee92879290916001613e83565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161291d929190613e66565b6102e0820152805161292e90614018565b60808201528051516020015160e00151600214612a26576129588161030001518260800151613e29565b610320820181905261024082015161296f91613dba565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516129a09361405a565b61036082018190526103a082015260e081015160a0820151612a079186916129c89190613e4a565b6129da8460c001518560a00151613e4a565b6129ee856102000151866101c00151613e4a565b612a02866101e00151876101c00151613e4a565b614074565b6103c08201819052610300820151612a1f9190614134565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612a9a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612ac19190810190616afd565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612b1657612b16616a02565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612b6557612b65616a02565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612bc5573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612bec9190810190616afd565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612cd5615f8a565b612cde886135ec565b6101c08201819052612cf59087905f90819061363d565b5060408401525081526101c0810151612d149087906001905f9061363d565b5060608481019190915260208401929092525086015160381c61ffff166101408201819052612d54908890612d4c9061271090614183565b612710613722565b610180820181905281516020830151612d7792612d72908390614183565b613722565b608082018190526020820151612d8d9190614183565b60e0820181905260408201516060830151610140840151612daf908b906141dd565b94509450945094505093509350935093565b5f5f5f5f86118015612dd1575083155b15612de0575083905084612e15565b5f87118015612ded575084155b15612dfc575085905082612e15565b604051636331fab160e01b815260040160405180910390fd5b5f612e2489606001515f613dfb565b90505f612e368a606001516001613dfb565b90505f612e5485676765c793fa10079d601b1b612d7286600a616c8f565b90505f612e7285676765c793fa10079d601b1b612d7286600a616c8f565b9050612e7e8282613000565b9c9b505050505050505050505050565b5f5f5f5f612e9a615f8a565b612ea3886135ec565b6101c08201819052612eba9087905f90819061363d565b5060408401525081526101c0810151612ed99087906001905f9061363d565b50606084015250602082018190528151612ef791612d72818b614183565b61010082018190528151612f0b9190614183565b610120820152606086015160381c61ffff166101408201819052610120820151612f3e9161271090612d72908290614183565b6101a0820181905260408201516060830151610140840151610120850151612daf916141dd565b5f5f612f73835f5f5f61363d565b50505090505f612f868460015f5f61363d565b5050509050805f03612f9b57505f9392505050565b5f612faa85606001515f613dfb565b90505f612fbc86606001516001613dfb565b90505f612fda85676765c793fa10079d601b1b612d7286600a616c8f565b90505f612ff885676765c793fa10079d601b1b612d7286600a616c8f565b9050610b0982825b5f8115676765c793fa10079d601b1b60028404190484111715613021575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f613047615f8a565b613050896135ec565b6101c082018190526130679088905f90819061363d565b5060408401525081526101c08101516130869088906001905f9061363d565b50606084015250602082015285156130ad5787815f018181516130a99190616a2a565b9052505b606087015160381c61ffff1661014082018190526130d4908990612d4c9061271090614183565b6101608201819052815160208301516130f292612d72908390614201565b60808201819052602082015161310791614183565b60c0820181905260408201516060830151610140840151613129908c906141dd565b9450945094509450505b945094509450949050565b5f5f5f5f61314a615f8a565b613153896135ec565b6101c0820181905261316a9088905f90819061363d565b5060408401525081526101c08101516131899088906001905f9061363d565b50606084015250602082015285156131b15787816020018181516131ad9190616a2a565b9052505b805160208201516131c79190612d72818c614201565b6080820181905281516131d991614183565b60a0820152606087015160381c61ffff16610140820181905260a082015161320891612d4c9061271090614183565b6040820151606083015161014084015160a0850151613129916141dd565b61322e615d97565b613236615ff1565b6132408484610c5d565b8152604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156132b3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526132da9190810190616afd565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015613327573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334b9190616b91565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156133cb573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526133f29190810190616afd565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613442573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134669190616b91565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016134a186845f0151613d4d565b90526020909101819052905092915050565b5f6134bc615df0565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061352091906169d0565b6020820152613531865f808061363d565b50505060c08201526135468660015f8061363d565b50505060e0820152821561358157848160c0018181516135669190616a2a565b90525060e08101805185919061357d908390616a2a565b9052505b80602001515f036135b1576135aa6103e86135a461359f8888614255565b6142bb565b90614183565b81526135e2565b6135df6135c78683602001518460c00151613722565b6135da8684602001518560e00151613722565b61439b565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff166002811061365a5761365a616a02565b602002015190505f61366c8a8a6143b0565b9050805f03613688575f5f5f5f95509550955095505050613133565b5f613697838c6080015161449e565b90505f6136a4828a613dba565b90505f89156136c9578184106136c3576136be8483614183565b6136cb565b5f6136cb565b5f5b90505f6136d8858d613dba565b90505f8c156136fd578482106136f7576136f28286614183565b6136ff565b5f6136ff565b5f5b905061370b8587616c9a565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f036137565783828161374c5761374c616cad565b04925050506102ce565b8084116137765760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b61380a604051806040016040528060068152602001652573202d257360d01b81525083836144ce565b5050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6138866040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613890845f61451a565b6020830152815260608401516138a6905f613dfb565b6060820181905281516138cb91676765c793fa10079d601b1b90612d7290600a616c8f565b604082015260208101515f036138e6575f6080820152613986565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561395c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061398091906169d0565b60808201525b61399184600161451a565b602083018190529082525f036139ac575f60a0820152613a4c565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613a22573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a4691906169d0565b60a08201525b80608001518160a001519250925050915091565b5f5f613a6c8484614560565b9050806001600160a01b03166321f8a72184604051602001613aad906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613add929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b1191815260200190565b602060405180830381865afa158015613b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030d91906169e7565b5f5f613b5c8484614560565b9050806001600160a01b031663bd02d0f584604051602001613baf9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613bdf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c1391815260200190565b602060405180830381865afa158015613c2e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030d91906169d0565b5f5f613c5f8560016143b0565b90508215613c7457613c718482616a2a565b90505b5f613c7e8761461b565b90505f613c8b8383613dba565b875160200151606001519091505f908210613cb95787516020015160600151613cb49083616a2a565b613cbb565b5f5b9050613cea6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b815250856137e1565b613d1a6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b815250846137e1565b610b09604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b815250836137e1565b5f6102ce838361465f565b613d60616011565b6102ce8383614678565b5f5f5f5f5f613d798888613d4d565b9050613d878787835f615896565b909350915081613d98575f19613da2565b613da28383613000565b9450613dad886105c9565b9350939792965093509350565b5f81156b019d971e4fe8401e740000001983900484111517613dda575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff851601613e1b575060ff60601b19905060605b90198416901c905092915050565b5f818311613e4057613e3b8383616a2a565b6102ce565b6102ce8284616a2a565b5f6102ce83676765c793fa10079d601b1b612d7285600a616c8f565b5f828411613e7c57613e7782616a3d565b61030d565b5092915050565b5f613ebd6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613ec98686865f615896565b60208301528082521580613efa5750845160ff841660028110613eee57613eee616a02565b6020020151602001515f145b15613f08575f9150506103d6565b613f1187615a02565b604082018190526020820151613f2691613dba565b6080820181905281511015613f3e575f9150506103d6565b60808101518151613f4f9190616a2a565b816060018181525050613f66866060015184613dfb565b60a082018190526060820151613f9291613f8190600a616c8f565b676765c793fa10079d601b1b613722565b60c08201525f1960ff841601613fb75760c0810151613fb19085613000565b60c08201525b845160ff841660028110613fcd57613fcd616a02565b6020020151602001518160c00151111561400a57845160ff841660028110613ff757613ff7616a02565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901614038575051602001516060015190565b81516020015160e00151614053575051602001516080015190565b505f919050565b5f84151583851114610ada5761406f82616a3d565b6103d6565b5f5f61407f876105c9565b90505f61408c8287613dba565b90505f6140998386613dba565b90505f6140a68984616cc1565b90505f6140b38389616cc1565b90505f6140bf83615a48565b90505f6140cb83615a48565b90505f841380156140db57505f83125b806140ef57505f841280156140ef57505f83135b15614103575f9750505050505050506103d6565b805f03614119575f9750505050505050506103d6565b6141238282613000565b9d9c50505050505050505050505050565b5f815f0361414357505f6102df565b5f82841161415a576141558484616a2a565b614164565b6141648385616a2a565b90505f6141718285613000565b905083851161030d5761406f81616a3d565b5f8261418f8382616a2a565b91508111156102df5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f811561138819839004841115176141f3575f5ffd5b506127109102611388010490565b5f8261420d8382616c9a565b91508110156102df5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016141d4565b5f8115806142785750828261426a8183616ce0565b92506142769083616cf7565b145b6102df5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016141d4565b5f815f036142ca57505f919050565b5f60016142d684615a5d565b901c6001901b905060018184816142ef576142ef616cad565b048201901c9050600181848161430757614307616cad565b048201901c9050600181848161431f5761431f616cad565b048201901c9050600181848161433757614337616cad565b048201901c9050600181848161434f5761434f616cad565b048201901c9050600181848161436757614367616cad565b048201901c9050600181848161437f5761437f616cad565b048201901c90506102ce8182858161439957614399616cad565b045b5f8183106143a957816102ce565b5090919050565b5f5f835f01518360ff16600281106143ca576143ca616a02565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614423573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061444791906169d0565b9050805f0361445a575f925050506102df565b606082015160c083015161446e9082616c9a565b82106144925760c08301516144838284616a2a565b61448d9190616a2a565b614494565b5f5b9695505050505050565b5f8260a001515f036144b157505f6102df565b5f6144bc8484615af0565b60a085015190915061030d9082613dba565b6145158383836040516024016144e693929190616d16565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052615b33565b505050565b5f5f5f614547855f01518560ff166002811061453857614538616a02565b6020020151866080015161449e565b90505f61455486866143b0565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001614583906169ad565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156145d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145fb9190616ade565b6102ce57604051637357d91f60e01b8152600481018490526024016141d4565b5f816001600160a01b031663bd02d0f56040516020016103fe9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f561070184615b3f565b614680616011565b82614689616011565b816001600160a01b03166391d4403c6040516020016146c9906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561471d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147419190616ade565b61474e5791506102df9050565b816001600160a01b031663bd02d0f585604051602001614788906020808252600690820152651413d4d7d25160d21b604082015260600190565b604051602081830303815290604052805190602001206040516020016147b8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147ec91815260200190565b602060405180830381865afa158015614807573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061482b91906169d0565b816020018181525050816001600160a01b03166321f8a72185604051602001614873906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b604051602081830303815290604052805190602001206040516020016148a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148d791815260200190565b602060405180830381865afa1580156148f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061491691906169e7565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614972906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b604051602081830303815290604052805190602001206040516020016149a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149d691815260200190565b602060405180830381865afa1580156149f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a1591906169e7565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614a90929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ac491815260200190565b602060405180830381865afa158015614adf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b0391906169d0565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614b579060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614b87929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bbb91815260200190565b602060405180830381865afa158015614bd6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614bfa91906169d0565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614c54906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c84929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cb891815260200190565b602060405180830381865afa158015614cd3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614cf791906169d0565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001614d50906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d80929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614db491815260200190565b602060405180830381865afa158015614dcf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614df391906169d0565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614e79929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ead91815260200190565b602060405180830381865afa158015614ec8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614eec91906169d0565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614f46906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f76929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614faa91815260200190565b602060405180830381865afa158015614fc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fe991906169d0565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161505c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161509091815260200190565b602060405180830381865afa1580156150ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150cf91906169d0565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615143929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161517791815260200190565b602060405180830381865afa158015615192573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151b691906169e7565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525d91815260200190565b602060405180830381865afa158015615278573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061529c91906169d0565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016152f19060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615321929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161535591815260200190565b602060405180830381865afa158015615370573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061539491906169d0565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016153ef90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161541f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161545391815260200190565b602060405180830381865afa15801561546e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061549291906169d0565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016154ec90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161551c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161555091815260200190565b602060405180830381865afa15801561556b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061558f91906169d0565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016155f19060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615621929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161565591815260200190565b602060405180830381865afa158015615670573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061569491906169d0565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016156ef90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161571f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161575391815260200190565b602060405180830381865afa15801561576e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061579291906169d0565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f5856040516020016157e1906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615811929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161584591815260200190565b602060405180830381865afa158015615860573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061588491906169d0565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615939575f5f6158c18a8a5f615bda565b915091505f6158dd5f8c60600151613dfb90919063ffffffff16565b90505f6158fb84676765c793fa10079d601b1b612d7285600a616c8f565b90505f61591984676765c793fa10079d601b1b612d7286600a616c8f565b90506159258288616c9a565b96506159318187616c9a565b955050505050505b865160200151516001600160a01b038681169116146159f5575f5f6159608a8a6001615bda565b915091505f61597d60018c60600151613dfb90919063ffffffff16565b90505f61599b84676765c793fa10079d601b1b612d7285600a616c8f565b90505f6159b984676765c793fa10079d601b1b612d7286600a616c8f565b90505f6159c6838d613dba565b90505f6159d3838e613dba565b90506159df828a616c9a565b98506159eb8189616c9a565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f56040516020016103fe906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615a5957815f036102df565b5090565b5f80608083901c15615a7157608092831c92015b604083901c15615a8357604092831c92015b602083901c15615a9557602092831c92015b601083901c15615aa757601092831c92015b600883901c15615ab957600892831c92015b600483901c15615acb57600492831c92015b600283901c15615add57600292831c92015b600183901c156102df5760010192915050565b5f428203615b03575060208201516102df565b5f615b12846040015184615c72565b9050615b2b846020015182613dba90919063ffffffff16565b9150506102df565b615b3c81615ca6565b50565b80518051515f918291615b539160016126e2565b905080604051602001615b8c90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b60405160208183030381529060405280519060200120604051602001615bbc929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff1660028110615bf557615bf5616a02565b60200201516040015190505f615c2b875f01518660ff1660028110615c1c57615c1c616a02565b60200201518860800151615af0565b90508115615c4257615c3d8282613dba565b615c44565b5f5b865190935060ff861660028110615c5d57615c5d616a02565b60200201516020015193505050935093915050565b5f80615c7e8342616a2a565b615c889085616ce0565b6301e133809004905061030d81676765c793fa10079d601b1b616c9a565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101a00160405280615cd9616037565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b604051806101200160405280615d586160be565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280615daa616139565b81526020015f81526020015f81525090565b6040518060a00160405280615dcf61618f565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c00160405280615e53615dbc565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615ea8615cc5565b905290565b604051806104000160405280615ec1616011565b8152602001615ece615dbc565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060400160405280616004615dbc565b8152602001615ea8615d97565b60405180606001604052806160246161f6565b81525f6020820181905260409091015290565b60405180604001604052806002905b6160a86040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816160465790505090565b60405180604001604052806002905b6161236040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816160cd5790505090565b60405180604001604052806002905b61617960405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816161485790505090565b60405180604001604052806002905b6161e06040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161619e5790505090565b60405180604001604052806002905b61624e6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816162055790505090565b6001600160a01b0381168114615b3c575f5ffd5b5f60208284031215616288575f5ffd5b81356102ce81616264565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057868503603f1901845281518051606080885260a08801919088015f5b600281101561637357898403605f19018252825180516001600160a01b0316855260208082015160809187018290529061634790870182616293565b60408381015190880152606092830151929096019190915250602092830192919091019060010161630b565b505050602082810151888201526040928301519290970191909152949384019391909101906001016162e7565b50929695505050505050565b5f5f5f5f608085870312156163bf575f5ffd5b84356163ca81616264565b935060208501356163da81616264565b925060408501356163ea81616264565b9396929550929360600135925050565b5f5f5f6060848603121561640c575f5ffd5b833561641781616264565b95602085013595506040909401359392505050565b5f8260408101835f5b60028110156164f7578383038752815180516001600160a01b031684526020810151610180602086015261646d610180860182616293565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616435565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057603f19878603018452815180516101a087526165506101a088018261642c565b9050602082015161656c60208901826001600160a01b03169052565b50604082015161658760408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e08201516165ca60e08901826001600160a01b03169052565b506101008201516165e061010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616528565b5f5f60408385031215616632575f5ffd5b823561663d81616264565b9150602083013561664d81616264565b809150509250929050565b5f8260408101835f5b60028110156164f7578383038752815180516001600160a01b0316845260208101516101406020860152616699610140860182616293565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616661565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156163a057603f19878603018452815180516101208752616759610120880182616658565b905060208201516020880152604082015161677f60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616731565b60ff81168114615b3c575f5ffd5b5f5f5f5f5f60a086880312156167f3575f5ffd5b85356167fe81616264565b9450602086013561680e81616264565b9350604086013561681e81616264565b9250606086013591506080860135616835816167d1565b809150509295509295909350565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561688057616880616843565b604052919050565b5f67ffffffffffffffff8211156168a1576168a1616843565b5060051b60200190565b5f5f604083850312156168bc575f5ffd5b82356168c781616264565b9150602083013567ffffffffffffffff8111156168e2575f5ffd5b8301601f810185136168f2575f5ffd5b803561690561690082616888565b616857565b8082825260208201915060208360051b850101925087831115616926575f5ffd5b6020840193505b8284101561694857833582526020938401939091019061692d565b809450505050509250929050565b5f5f5f5f5f60a0868803121561696a575f5ffd5b853561697581616264565b9450602086013561698581616264565b9350604086013561699581616264565b94979396509394606081013594506080013592915050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f602082840312156169e0575f5ffd5b5051919050565b5f602082840312156169f7575f5ffd5b81516102ce81616264565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156102df576102df616a16565b5f600160ff1b8201616a5157616a51616a16565b505f0390565b5f60208284031215616a67575f5ffd5b815167ffffffffffffffff811115616a7d575f5ffd5b8201601f81018413616a8d575f5ffd5b8051616a9b61690082616888565b8082825260208201915060208360051b850101925086831115616abc575f5ffd5b6020840193505b82841015614494578351825260209384019390910190616ac3565b5f60208284031215616aee575f5ffd5b815180151581146102ce575f5ffd5b5f60208284031215616b0d575f5ffd5b815167ffffffffffffffff811115616b23575f5ffd5b8201601f81018413616b33575f5ffd5b805167ffffffffffffffff811115616b4d57616b4d616843565b616b60601f8201601f1916602001616857565b818152856020838501011115616b74575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215616ba1575f5ffd5b81516102ce816167d1565b6001815b6001841115616be757808504811115616bcb57616bcb616a16565b6001841615616bd957908102905b60019390931c928002616bb0565b935093915050565b5f82616bfd575060016102df565b81616c0957505f6102df565b8160018114616c1f5760028114616c2957616c45565b60019150506102df565b60ff841115616c3a57616c3a616a16565b50506001821b6102df565b5060208310610133831016604e8410600b8410161715616c68575081810a6102df565b616c745f198484616bac565b805f1904821115616c8757616c87616a16565b029392505050565b5f6102ce8383616bef565b808201808211156102df576102df616a16565b634e487b7160e01b5f52601260045260245ffd5b8181035f831280158383131683831282161715613e7c57613e7c616a16565b80820281158282048414176102df576102df616a16565b5f82616d1157634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f616d286060830186616293565b8281036020840152616d3a8186616293565b91505082604083015294935050505056fea2646970667358221220fccb42db6896b4ab1d781fc86b4ff7de89e45921b9ac5832e846131a8f3cacc864736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x93W\x80c\xD2\x8B\n\x15\x11a\0cW\x80c\xD2\x8B\n\x15\x14a\x02iW\x80c\xE35\xAD\xB7\x14a\x02|W\x80c\xEE\xD0t(\x14a\x02\x8FW\x80c\xF6\x8Aq1\x14a\x02\xA2W__\xFD[\x80cs\x91\x18\xA4\x14a\x01\xF5W\x80cx\xF2\x12\xD1\x14a\x02\x15W\x80c\x8Flz<\x14a\x02(W\x80c\xC2\xBD\xED\xA1\x14a\x02;W__\xFD[\x80cP\xEDY-\x11a\0\xCEW\x80cP\xEDY-\x14a\x01\x8EW\x80cW\xB9\x1C\xA6\x14a\x01\xAFW\x80cZoW\x10\x14a\x01\xC2W\x80c\\9\xF4g\x14a\x01\xD5W__\xFD[\x80c\x1A0\x81u\x14a\0\xFFW\x80c(\xA0\xCC\xF4\x14a\x01(W\x80c1{P\xEC\x14a\x01SW\x80c8/\xC7.\x14a\x01{W[__\xFD[a\x01\x12a\x01\r6`\x04abxV[a\x02\xB5V[`@Qa\x01\x1F\x91\x90ab\xC1V[`@Q\x80\x91\x03\x90\xF3[a\x01;a\x0166`\x04abxV[a\x02\xD5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1FV[a\x01fa\x01a6`\x04ac\xACV[a\x02\xE5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[a\x01\x12a\x01\x896`\x04ac\xFAV[a\x03\0V[a\x01\xA1a\x01\x9C6`\x04abxV[a\x03\x15V[`@Q\x90\x81R` \x01a\x01\x1FV[a\x01\xA1a\x01\xBD6`\x04abxV[a\x03\x1FV[a\x01\xA1a\x01\xD06`\x04abxV[a\x03)V[a\x01\xE8a\x01\xE36`\x04abxV[a\x033V[`@Qa\x01\x1F\x91\x90ae\x02V[a\x02\x08a\x02\x036`\x04af!V[a\x03LV[`@Qa\x01\x1F\x91\x90ag\x0BV[a\x01;a\x02#6`\x04abxV[a\x03gV[a\x01\xE8a\x0266`\x04ac\xFAV[a\x03qV[a\x02Na\x02I6`\x04ag\xDFV[a\x03~V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x1FV[a\x02Na\x02w6`\x04ag\xDFV[a\x03\x9FV[a\x01;a\x02\x8A6`\x04abxV[a\x03\xAFV[a\x01\x12a\x02\x9D6`\x04ah\xABV[a\x03\xB9V[a\x01\xA1a\x02\xB06`\x04aiVV[a\x03\xC5V[``_a\x02\xC1\x83a\x03\xDFV[\x90Pa\x02\xCE\x83_\x83a\x04qV[\x93\x92PPPV[_a\x02\xDF\x82a\x04\x8BV[\x92\x91PPV[__a\x02\xF3\x86\x86\x86\x86a\x05<V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03\r\x84\x84\x84a\x04qV[\x94\x93PPPPV[_a\x02\xDF\x82a\x05xV[_a\x02\xDF\x82a\x05\xC9V[_a\x02\xDF\x82a\x03\xDFV[``_a\x03?\x83a\x03\xDFV[\x90Pa\x02\xCE\x83_\x83a\x06\x1AV[``_a\x03Y\x84\x84a\x06\xE8V[\x90Pa\x03\r\x84\x84_\x84a\x07^V[_a\x02\xDF\x82a\x08.V[``a\x03\r\x84\x84\x84a\x06\x1AV[___a\x03\x8E\x88\x88\x88\x88\x88a\x08jV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[___a\x03\x8E\x88\x88\x88\x88\x88a\tZV[_a\x02\xDF\x82a\t\xD3V[``a\x02\xCE\x83\x83a\n$V[_a\x03\xD3\x86\x86\x86\x86\x86a\n\xE2V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x03\xFE\x90ai\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x042\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90ai\xD0V[``_a\x04\x7F\x85\x85\x85a\x0B\x15V[\x90Pa\x03\xD6\x85\x82a\n$V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90ai\xE7V[___a\x05I\x86\x86a\x0B\xB6V[\x90P_a\x05V\x88\x83a\x0C]V[\x90P__a\x05e\x8A\x84\x89a\x1E\xD5V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x06(\x85\x85\x85a\x0B\x15V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06EWa\x06EahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06~W\x81` \x01[a\x06ka\\\xC5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06cW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xDEW_\x83\x82\x81Q\x81\x10a\x06\x9FWa\x06\x9Faj\x02V[` \x02` \x01\x01Q\x90P_a\x06\xB4\x89\x83a!\x0EV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\xC9Wa\x06\xC9aj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\x83V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x07\x01\x84a%\xAEV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCE\x91\x90ai\xD0V[``_a\x07m\x86\x86\x86\x86a&2V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x8AWa\x07\x8AahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC3W\x81` \x01[a\x07\xB0a]DV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xA8W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x08#W_\x83\x82\x81Q\x81\x10a\x07\xE4Wa\x07\xE4aj\x02V[` \x02` \x01\x01Q\x90P_a\x07\xF9\x8A\x83a&\xBAV[\x90P\x80\x84\x84\x81Q\x81\x10a\x08\x0EWa\x08\x0Eaj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x07\xC8V[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x08x\x88\x88a\x0B\xB6V[\x90P_a\x08\x85\x8A\x83a\x0C]V[\x90P_\x80\x80`\xFF\x89\x16a\x08\xBBWa\x08\x9D\x8D\x8B\x86a,\xC9V[\x92\x95P\x91\x93Pa\x08\xB4\x91P\x85\x90P_\x85\x8D\x82a-\xC1V[\x90Pa\x08\xEBV[_\x19`\xFF\x8A\x16\x01a\x08\xEBWa\x08\xD1\x8D\x8B\x86a.\x8EV[\x92\x95P\x91\x93Pa\x08\xE8\x91P\x85\x90P\x84_\x80\x8Ea-\xC1V[\x90P[_a\x08\xF5\x85a/eV[\x90P_\x82\x82\x11a\t\x0EWa\t\t\x82\x84aj*V[a\t\x18V[a\t\x18\x83\x83aj*V[\x90P_a\t%\x82\x84a0\0V[\x90P_\x84\x84\x11a\t=Wa\t8\x82aj=V[a\t?V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\th\x88\x88a\x0B\xB6V[\x90P_a\tu\x8A\x83a\x0C]V[\x90P_\x80\x80`\xFF\x89\x16a\t\xA5Wa\t\x8E\x8D\x8B\x86_a0;V[\x92\x95P\x91\x93Pa\x08\xB4\x91P\x85\x90P\x8B_\x80\x87a-\xC1V[_\x19`\xFF\x8A\x16\x01a\x08\xEBWa\t\xBC\x8D\x8B\x86_a1>V[\x92\x95P\x91\x93Pa\x08\xE8\x91P\x85\x90P_\x8C\x86\x82a-\xC1V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x04\xC9\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nAWa\nAahCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nzW\x81` \x01[a\nga]\x97V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n_W\x90P[P\x90P_[\x83Q\x81\x10\x15a\n\xDAW_\x84\x82\x81Q\x81\x10a\n\x9BWa\n\x9Baj\x02V[` \x02` \x01\x01Q\x90P_a\n\xB0\x87\x83a2&V[\x90P\x80\x84\x84\x81Q\x81\x10a\n\xC5Wa\n\xC5aj\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\n\x7FV[P\x93\x92PPPV[__a\n\xEE\x86\x86a\x0B\xB6V[\x90P_a\n\xFB\x88\x83a\x0C]V[\x90Pa\x0B\t\x81\x86\x86_a4\xB3V[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x0B5\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x8FW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\r\x91\x90\x81\x01\x90ajWV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0B\xD7W\x81\x83a\x0B\xDAV[\x82\x82[`@Q\x91\x94P\x92Pa\x0C\x07\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x0Cea]\xBCV[\x82a\x0Cna]\xBCV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x0C\x8C\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x04\x91\x90aj\xDEV[a\r\x11W\x91Pa\x02\xDF\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\rQ\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF4\x91\x90ai\xE7V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0Er\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE5\x91\x90ai\xD0V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0F;\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0Fk\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xDE\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10?\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10o\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xA3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE2\x91\x90ai\xD0V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11M\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11}\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xF0\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12Q\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\x81\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xD0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF4\x91\x90ai\xD0V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xA5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE4\x91\x90ai\xD0V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14Y\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCC\x91\x90ai\xE7V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15v\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB5\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\x0C\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16<\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16p\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xAF\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\x11\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17u\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB4\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x18 \x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC3\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19%\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19U\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC8\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1A!\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1AQ\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x85\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xC4\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1B\x12\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1BB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Bv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xB5\x91\x90ai\xE7V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1C#\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1CS\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x87\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC6\x91\x90ai\xE7V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1D)\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1DY\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xCC\x91\x90ai\xD0V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1E%\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1EU\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x89\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xA4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xC8\x91\x90ai\xD0V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1E\xE1a]\xF0V[a\x1E\xEA\x88a5\xECV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FX\x91\x90ai\xD0V[\x81` \x01\x81\x81RPPa\x1Fr\x87__\x84a\x01@\x01Qa6=V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\x1F\x94\x90\x88\x90`\x01\x90_\x90a6=V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1F\xB5\x91\x88\x91a7\"V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1F\xD0\x91\x88\x91a7\"V[\x81a\x01 \x01\x81\x81RPPa \x10`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa7\xE1V[a H`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa7\xE1V[a \x80`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa7\xE1V[a \xB3`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa7\xE1V[a \xE6`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa7\xE1V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[a!\x16a\\\xC5V[a!\x1Ea^?V[a!(\x84\x84a\x0C]V[\x81Ra!3\x84a8\x0EV[` \x82\x01Ra!A\x84a5\xECV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa!\\\x92_\x91\x90a6=V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa!\x8A\x92\x91`\x01\x91a6=V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa!\xAC\x90a8QV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\"+W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"R\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\"\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xC3\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a#\xA3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xCA\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$>\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a%\x1E\x90a/eV[\x81R` \x01a%-\x86\x86a:`V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a%T\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a%e\x86\x86a;PV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a%\x89\x90\x87\x90_\x80a<RV[\x81R` \x01a%\x9B\x86\x84_\x01Qa=MV[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a%\xE8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a&L\x86a%\xAEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x93W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xD6\x91\x90\x81\x01\x90ajWV[a&\xC2a]DV[a&\xCAa^\xADV[a&\xD4\x84\x84a=XV[\x80\x82RQ\x80QQa&\xED\x91`\x01[` \x02\x01QQa\x0B\xB6V[`@\x82\x01\x81\x90Ra&\xFF\x90\x85\x90a\x0C]V[` \x82\x01\x81\x90R\x81Qa'\x13\x91\x86\x91a=jV[PPPP``\x82\x01R` \x81\x01Qa'*\x90a/eV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa'`\x91\x90_[` \x02\x01Q`@\x01Q\x90a=\xBAV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa'x\x90_a=\xFBV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa'\x91\x91\x90a>)V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa'\xA8\x91\x90a>JV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa'\xC9\x92\x91\x90a>fV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa'\xEA\x92\x87\x92\x90\x91_a>\x83V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa(\x17\x92\x91\x90a>fV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa(1\x90`\x01a=\xFBV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa(b\x91\x90`\x01a'QV[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa(y\x91a>)V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa(\x91\x91\x90a>JV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa(\xA9\x91\x90a=\xBAV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa(\xCC\x92\x91\x90a>fV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa(\xEE\x92\x87\x92\x90\x91`\x01a>\x83V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa)\x1D\x92\x91\x90a>fV[a\x02\xE0\x82\x01R\x80Qa).\x90a@\x18V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a*&Wa)X\x81a\x03\0\x01Q\x82`\x80\x01Qa>)V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa)o\x91a=\xBAV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa)\xA0\x93a@ZV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa*\x07\x91\x86\x91a)\xC8\x91\x90a>JV[a)\xDA\x84`\xC0\x01Q\x85`\xA0\x01Qa>JV[a)\xEE\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa>JV[a*\x02\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa>JV[a@tV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa*\x1F\x91\x90aA4V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*\xC1\x91\x90\x81\x01\x90aj\xFDV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a+\x16Wa+\x16aj\x02V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a+eWa+eaj\x02V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xC5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xEC\x91\x90\x81\x01\x90aj\xFDV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a,\xD5a_\x8AV[a,\xDE\x88a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra,\xF5\x90\x87\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa-\x14\x90\x87\x90`\x01\x90_\x90a6=V[P``\x84\x81\x01\x91\x90\x91R` \x84\x01\x92\x90\x92RP\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra-T\x90\x88\x90a-L\x90a'\x10\x90aA\x83V[a'\x10a7\"V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa-w\x92a-r\x90\x83\x90aA\x83V[a7\"V[`\x80\x82\x01\x81\x90R` \x82\x01Qa-\x8D\x91\x90aA\x83V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa-\xAF\x90\x8B\x90aA\xDDV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a-\xD1WP\x83\x15[\x15a-\xE0WP\x83\x90P\x84a.\x15V[_\x87\x11\x80\x15a-\xEDWP\x84\x15[\x15a-\xFCWP\x85\x90P\x82a.\x15V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a.$\x89``\x01Q_a=\xFBV[\x90P_a.6\x8A``\x01Q`\x01a=\xFBV[\x90P_a.T\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_a.r\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90Pa.~\x82\x82a0\0V[\x9C\x9BPPPPPPPPPPPPV[____a.\x9Aa_\x8AV[a.\xA3\x88a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra.\xBA\x90\x87\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa.\xD9\x90\x87\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01\x81\x90R\x81Qa.\xF7\x91a-r\x81\x8BaA\x83V[a\x01\0\x82\x01\x81\x90R\x81Qa/\x0B\x91\x90aA\x83V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa/>\x91a'\x10\x90a-r\x90\x82\x90aA\x83V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa-\xAF\x91aA\xDDV[__a/s\x83___a6=V[PPP\x90P_a/\x86\x84`\x01__a6=V[PPP\x90P\x80_\x03a/\x9BWP_\x93\x92PPPV[_a/\xAA\x85``\x01Q_a=\xFBV[\x90P_a/\xBC\x86``\x01Q`\x01a=\xFBV[\x90P_a/\xDA\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_a/\xF8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90Pa\x0B\t\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a0!W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a0Ga_\x8AV[a0P\x89a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra0g\x90\x88\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa0\x86\x90\x88\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01R\x85\x15a0\xADW\x87\x81_\x01\x81\x81Qa0\xA9\x91\x90aj*V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra0\xD4\x90\x89\x90a-L\x90a'\x10\x90aA\x83V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\xF2\x92a-r\x90\x83\x90aB\x01V[`\x80\x82\x01\x81\x90R` \x82\x01Qa1\x07\x91aA\x83V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa1)\x90\x8C\x90aA\xDDV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a1Ja_\x8AV[a1S\x89a5\xECV[a\x01\xC0\x82\x01\x81\x90Ra1j\x90\x88\x90_\x90\x81\x90a6=V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa1\x89\x90\x88\x90`\x01\x90_\x90a6=V[P``\x84\x01RP` \x82\x01R\x85\x15a1\xB1W\x87\x81` \x01\x81\x81Qa1\xAD\x91\x90aj*V[\x90RP[\x80Q` \x82\x01Qa1\xC7\x91\x90a-r\x81\x8CaB\x01V[`\x80\x82\x01\x81\x90R\x81Qa1\xD9\x91aA\x83V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa2\x08\x91a-L\x90a'\x10\x90aA\x83V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa1)\x91aA\xDDV[a2.a]\x97V[a26a_\xF1V[a2@\x84\x84a\x0C]V[\x81R`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a2\xB3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xDA\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a3'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3K\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a3\xCBW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra3\xF2\x91\x90\x81\x01\x90aj\xFDV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4f\x91\x90ak\x91V[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a4\xA1\x86\x84_\x01Qa=MV[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a4\xBCa]\xF0V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5 \x91\x90ai\xD0V[` \x82\x01Ra51\x86_\x80\x80a6=V[PPP`\xC0\x82\x01Ra5F\x86`\x01_\x80a6=V[PPP`\xE0\x82\x01R\x82\x15a5\x81W\x84\x81`\xC0\x01\x81\x81Qa5f\x91\x90aj*V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a5}\x90\x83\x90aj*V[\x90RP[\x80` \x01Q_\x03a5\xB1Wa5\xAAa\x03\xE8a5\xA4a5\x9F\x88\x88aBUV[aB\xBBV[\x90aA\x83V[\x81Ra5\xE2V[a5\xDFa5\xC7\x86\x83` \x01Q\x84`\xC0\x01Qa7\"V[a5\xDA\x86\x84` \x01Q\x85`\xE0\x01Qa7\"V[aC\x9BV[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a6ZWa6Zaj\x02V[` \x02\x01Q\x90P_a6l\x8A\x8AaC\xB0V[\x90P\x80_\x03a6\x88W____\x95P\x95P\x95P\x95PPPa13V[_a6\x97\x83\x8C`\x80\x01QaD\x9EV[\x90P_a6\xA4\x82\x8Aa=\xBAV[\x90P_\x89\x15a6\xC9W\x81\x84\x10a6\xC3Wa6\xBE\x84\x83aA\x83V[a6\xCBV[_a6\xCBV[_[\x90P_a6\xD8\x85\x8Da=\xBAV[\x90P_\x8C\x15a6\xFDW\x84\x82\x10a6\xF7Wa6\xF2\x82\x86aA\x83V[a6\xFFV[_a6\xFFV[_[\x90Pa7\x0B\x85\x87al\x9AV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a7VW\x83\x82\x81a7LWa7Lal\xADV[\x04\x92PPPa\x02\xCEV[\x80\x84\x11a7vW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a8\n`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aD\xCEV[PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a8\x86`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a8\x90\x84_aE\x1AV[` \x83\x01R\x81R``\x84\x01Qa8\xA6\x90_a=\xFBV[``\x82\x01\x81\x90R\x81Qa8\xCB\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a-r\x90`\nal\x8FV[`@\x82\x01R` \x81\x01Q_\x03a8\xE6W_`\x80\x82\x01Ra9\x86V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x80\x91\x90ai\xD0V[`\x80\x82\x01R[a9\x91\x84`\x01aE\x1AV[` \x83\x01\x81\x90R\x90\x82R_\x03a9\xACW_`\xA0\x82\x01Ra:LV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:F\x91\x90ai\xD0V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a:l\x84\x84aE`V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a:\xAD\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\r\x91\x90ai\xE7V[__a;\\\x84\x84aE`V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a;\xAF\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\r\x91\x90ai\xD0V[__a<_\x85`\x01aC\xB0V[\x90P\x82\x15a<tWa<q\x84\x82aj*V[\x90P[_a<~\x87aF\x1BV[\x90P_a<\x8B\x83\x83a=\xBAV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10a<\xB9W\x87Q` \x01Q``\x01Qa<\xB4\x90\x83aj*V[a<\xBBV[_[\x90Pa<\xEA`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a7\xE1V[a=\x1A`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a7\xE1V[a\x0B\t`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a7\xE1V[_a\x02\xCE\x83\x83aF_V[a=`a`\x11V[a\x02\xCE\x83\x83aFxV[_____a=y\x88\x88a=MV[\x90Pa=\x87\x87\x87\x83_aX\x96V[\x90\x93P\x91P\x81a=\x98W_\x19a=\xA2V[a=\xA2\x83\x83a0\0V[\x94Pa=\xAD\x88a\x05\xC9V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a=\xDAW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a>\x1BWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a>@Wa>;\x83\x83aj*V[a\x02\xCEV[a\x02\xCE\x82\x84aj*V[_a\x02\xCE\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[_\x82\x84\x11a>|Wa>w\x82aj=V[a\x03\rV[P\x92\x91PPV[_a>\xBD`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a>\xC9\x86\x86\x86_aX\x96V[` \x83\x01R\x80\x82R\x15\x80a>\xFAWP\x84Q`\xFF\x84\x16`\x02\x81\x10a>\xEEWa>\xEEaj\x02V[` \x02\x01Q` \x01Q_\x14[\x15a?\x08W_\x91PPa\x03\xD6V[a?\x11\x87aZ\x02V[`@\x82\x01\x81\x90R` \x82\x01Qa?&\x91a=\xBAV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a?>W_\x91PPa\x03\xD6V[`\x80\x81\x01Q\x81Qa?O\x91\x90aj*V[\x81``\x01\x81\x81RPPa?f\x86``\x01Q\x84a=\xFBV[`\xA0\x82\x01\x81\x90R``\x82\x01Qa?\x92\x91a?\x81\x90`\nal\x8FV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba7\"V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a?\xB7W`\xC0\x81\x01Qa?\xB1\x90\x85a0\0V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a?\xCDWa?\xCDaj\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a@\nW\x84Q`\xFF\x84\x16`\x02\x81\x10a?\xF7Wa?\xF7aj\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a@8WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa@SWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\n\xDAWa@o\x82aj=V[a\x03\xD6V[__a@\x7F\x87a\x05\xC9V[\x90P_a@\x8C\x82\x87a=\xBAV[\x90P_a@\x99\x83\x86a=\xBAV[\x90P_a@\xA6\x89\x84al\xC1V[\x90P_a@\xB3\x83\x89al\xC1V[\x90P_a@\xBF\x83aZHV[\x90P_a@\xCB\x83aZHV[\x90P_\x84\x13\x80\x15a@\xDBWP_\x83\x12[\x80a@\xEFWP_\x84\x12\x80\x15a@\xEFWP_\x83\x13[\x15aA\x03W_\x97PPPPPPPPa\x03\xD6V[\x80_\x03aA\x19W_\x97PPPPPPPPa\x03\xD6V[aA#\x82\x82a0\0V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aACWP_a\x02\xDFV[_\x82\x84\x11aAZWaAU\x84\x84aj*V[aAdV[aAd\x83\x85aj*V[\x90P_aAq\x82\x85a0\0V[\x90P\x83\x85\x11a\x03\rWa@o\x81aj=V[_\x82aA\x8F\x83\x82aj*V[\x91P\x81\x11\x15a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aA\xF3W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aB\r\x83\x82al\x9AV[\x91P\x81\x10\x15a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01aA\xD4V[_\x81\x15\x80aBxWP\x82\x82aBj\x81\x83al\xE0V[\x92PaBv\x90\x83al\xF7V[\x14[a\x02\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01aA\xD4V[_\x81_\x03aB\xCAWP_\x91\x90PV[_`\x01aB\xD6\x84aZ]V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aB\xEFWaB\xEFal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x07WaC\x07al\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x1FWaC\x1Fal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC7WaC7al\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aCOWaCOal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aCgWaCgal\xADV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aC\x7FWaC\x7Fal\xADV[\x04\x82\x01\x90\x1C\x90Pa\x02\xCE\x81\x82\x85\x81aC\x99WaC\x99al\xADV[\x04[_\x81\x83\x10aC\xA9W\x81a\x02\xCEV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aC\xCAWaC\xCAaj\x02V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDG\x91\x90ai\xD0V[\x90P\x80_\x03aDZW_\x92PPPa\x02\xDFV[``\x82\x01Q`\xC0\x83\x01QaDn\x90\x82al\x9AV[\x82\x10aD\x92W`\xC0\x83\x01QaD\x83\x82\x84aj*V[aD\x8D\x91\x90aj*V[aD\x94V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aD\xB1WP_a\x02\xDFV[_aD\xBC\x84\x84aZ\xF0V[`\xA0\x85\x01Q\x90\x91Pa\x03\r\x90\x82a=\xBAV[aE\x15\x83\x83\x83`@Q`$\x01aD\xE6\x93\x92\x91\x90am\x16V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra[3V[PPPV[___aEG\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aE8WaE8aj\x02V[` \x02\x01Q\x86`\x80\x01QaD\x9EV[\x90P_aET\x86\x86aC\xB0V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aE\x83\x90ai\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xFB\x91\x90aj\xDEV[a\x02\xCEW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01aA\xD4V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x07\x01\x84a[?V[aF\x80a`\x11V[\x82aF\x89a`\x11V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aF\xC9\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGA\x91\x90aj\xDEV[aGNW\x91Pa\x02\xDF\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\x88\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH+\x91\x90ai\xD0V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aHs\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x16\x91\x90ai\xE7V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aIr\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x15\x91\x90ai\xE7V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\x90\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xC4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x03\x91\x90ai\xD0V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aKW\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xBB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xD6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\xFA\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aLT\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x84\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xB8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xF7\x91\x90ai\xD0V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aMP\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x80\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xB4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xF3\x91\x90ai\xD0V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNy\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xAD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xEC\x91\x90ai\xD0V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOF\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aOv\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xAA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xE9\x91\x90ai\xD0V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\\\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\x90\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xCF\x91\x90ai\xD0V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQw\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\x92W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xB6\x91\x90ai\xE7V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR]\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRxW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x9C\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\xF1\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aSpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x94\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aS\xEF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT\x1F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTS\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aTnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\x92\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\xEC\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aUkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x8F\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aU\xF1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVU\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVpW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x94\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xEF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW\x1F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWS\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aWnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\x92\x91\x90ai\xD0V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xE1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\x11\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x84\x91\x90ai\xD0V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aY9W__aX\xC1\x8A\x8A_a[\xDAV[\x91P\x91P_aX\xDD_\x8C``\x01Qa=\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aX\xFB\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[\x90P_aY\x19\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90PaY%\x82\x88al\x9AV[\x96PaY1\x81\x87al\x9AV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aY\xF5W__aY`\x8A\x8A`\x01a[\xDAV[\x91P\x91P_aY}`\x01\x8C``\x01Qa=\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aY\x9B\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x85`\nal\x8FV[\x90P_aY\xB9\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba-r\x86`\nal\x8FV[\x90P_aY\xC6\x83\x8Da=\xBAV[\x90P_aY\xD3\x83\x8Ea=\xBAV[\x90PaY\xDF\x82\x8Aal\x9AV[\x98PaY\xEB\x81\x89al\x9AV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x03\xFE\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aZYW\x81_\x03a\x02\xDFV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aZqW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aZ\x83W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aZ\x95W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aZ\xA7W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aZ\xB9W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aZ\xCBW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aZ\xDDW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x02\xDFW`\x01\x01\x92\x91PPV[_B\x82\x03a[\x03WP` \x82\x01Qa\x02\xDFV[_a[\x12\x84`@\x01Q\x84a\\rV[\x90Pa[+\x84` \x01Q\x82a=\xBA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x02\xDFV[a[<\x81a\\\xA6V[PV[\x80Q\x80QQ_\x91\x82\x91a[S\x91`\x01a&\xE2V[\x90P\x80`@Q` \x01a[\x8C\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10a[\xF5Wa[\xF5aj\x02V[` \x02\x01Q`@\x01Q\x90P_a\\+\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10a\\\x1CWa\\\x1Caj\x02V[` \x02\x01Q\x88`\x80\x01QaZ\xF0V[\x90P\x81\x15a\\BWa\\=\x82\x82a=\xBAV[a\\DV[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10a\\]Wa\\]aj\x02V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80a\\~\x83Baj*V[a\\\x88\x90\x85al\xE0V[c\x01\xE13\x80\x90\x04\x90Pa\x03\r\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bal\x9AV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01\xA0\x01`@R\x80a\\\xD9a`7V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80a]Xa`\xBEV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80a]\xAAaa9V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80a]\xCFaa\x8FV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80a^Sa]\xBCV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a^\xA8a\\\xC5V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80a^\xC1a`\x11V[\x81R` \x01a^\xCEa]\xBCV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80a`\x04a]\xBCV[\x81R` \x01a^\xA8a]\x97V[`@Q\x80``\x01`@R\x80a`$aa\xF6V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a`\xA8`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a`FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aa#`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a`\xCDW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aay`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aaHW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aa\xE0`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aa\x9EW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[abN`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ab\x05W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a[<W__\xFD[_` \x82\x84\x03\x12\x15ab\x88W__\xFD[\x815a\x02\xCE\x81abdV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15acsW\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90acG\x90\x87\x01\x82ab\x93V[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ac\x0BV[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ab\xE7V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15ac\xBFW__\xFD[\x845ac\xCA\x81abdV[\x93P` \x85\x015ac\xDA\x81abdV[\x92P`@\x85\x015ac\xEA\x81abdV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ad\x0CW__\xFD[\x835ad\x17\x81abdV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ad\xF7W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Radma\x01\x80\x86\x01\x82ab\x93V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pad5V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87RaePa\x01\xA0\x88\x01\x82ad,V[\x90P` \x82\x01Qael` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qae\x87`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qae\xCA`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qae\xE0a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ae(V[__`@\x83\x85\x03\x12\x15af2W__\xFD[\x825af=\x81abdV[\x91P` \x83\x015afM\x81abdV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ad\xF7W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Raf\x99a\x01@\x86\x01\x82ab\x93V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PafaV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15ac\xA0W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RagYa\x01 \x88\x01\x82afXV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qag\x7F`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ag1V[`\xFF\x81\x16\x81\x14a[<W__\xFD[_____`\xA0\x86\x88\x03\x12\x15ag\xF3W__\xFD[\x855ag\xFE\x81abdV[\x94P` \x86\x015ah\x0E\x81abdV[\x93P`@\x86\x015ah\x1E\x81abdV[\x92P``\x86\x015\x91P`\x80\x86\x015ah5\x81ag\xD1V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15ah\x80Wah\x80ahCV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah\xA1Wah\xA1ahCV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ah\xBCW__\xFD[\x825ah\xC7\x81abdV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\xE2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13ah\xF2W__\xFD[\x805ai\x05ai\0\x82ah\x88V[ahWV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ai&W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aiHW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ai-V[\x80\x94PPPPP\x92P\x92\x90PV[_____`\xA0\x86\x88\x03\x12\x15aijW__\xFD[\x855aiu\x81abdV[\x94P` \x86\x015ai\x85\x81abdV[\x93P`@\x86\x015ai\x95\x81abdV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15ai\xE0W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ai\xF7W__\xFD[\x81Qa\x02\xCE\x81abdV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xDFWa\x02\xDFaj\x16V[_`\x01`\xFF\x1B\x82\x01ajQWajQaj\x16V[P_\x03\x90V[_` \x82\x84\x03\x12\x15ajgW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aj}W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aj\x8DW__\xFD[\x80Qaj\x9Bai\0\x82ah\x88V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aj\xBCW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aD\x94W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aj\xC3V[_` \x82\x84\x03\x12\x15aj\xEEW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\xCEW__\xFD[_` \x82\x84\x03\x12\x15ak\rW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ak#W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ak3W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15akMWakMahCV[ak``\x1F\x82\x01`\x1F\x19\x16` \x01ahWV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aktW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ak\xA1W__\xFD[\x81Qa\x02\xCE\x81ag\xD1V[`\x01\x81[`\x01\x84\x11\x15ak\xE7W\x80\x85\x04\x81\x11\x15ak\xCBWak\xCBaj\x16V[`\x01\x84\x16\x15ak\xD9W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ak\xB0V[\x93P\x93\x91PPV[_\x82ak\xFDWP`\x01a\x02\xDFV[\x81al\tWP_a\x02\xDFV[\x81`\x01\x81\x14al\x1FW`\x02\x81\x14al)WalEV[`\x01\x91PPa\x02\xDFV[`\xFF\x84\x11\x15al:Wal:aj\x16V[PP`\x01\x82\x1Ba\x02\xDFV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15alhWP\x81\x81\na\x02\xDFV[alt_\x19\x84\x84ak\xACV[\x80_\x19\x04\x82\x11\x15al\x87Wal\x87aj\x16V[\x02\x93\x92PPPV[_a\x02\xCE\x83\x83ak\xEFV[\x80\x82\x01\x80\x82\x11\x15a\x02\xDFWa\x02\xDFaj\x16V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a>|Wa>|aj\x16V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xDFWa\x02\xDFaj\x16V[_\x82am\x11WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_am(``\x83\x01\x86ab\x93V[\x82\x81\x03` \x84\x01Ram:\x81\x86ab\x93V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xFC\xCBB\xDBh\x96\xB4\xAB\x1Dx\x1F\xC8kO\xF7\xDE\x89\xE4Y!\xB9\xACX2\xE8F\x13\x1A\x8F<\xAC\xC8dsolcC\0\x08\x1C\x003",
    );
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
    /**Function with signature `calcAmountIn(address,address,address,uint256,uint8)` and selector `0xc2bdeda1`.
```solidity
function calcAmountIn(address dataStore, address token0, address token1, uint256 amountOut, uint8 tokenOutIndex) external view returns (uint256, uint256, int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcAmountInCall {
        pub dataStore: alloy::sol_types::private::Address,
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub amountOut: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenOutIndex: u8,
    }
    ///Container type for the return parameters of the [`calcAmountIn(address,address,address,uint256,uint8)`](calcAmountInCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcAmountInReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
        pub _2: alloy::sol_types::private::primitives::aliases::I256,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calcAmountInCall> for UnderlyingRustTuple<'_> {
                fn from(value: calcAmountInCall) -> Self {
                    (
                        value.dataStore,
                        value.token0,
                        value.token1,
                        value.amountOut,
                        value.tokenOutIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calcAmountInCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        token0: tuple.1,
                        token1: tuple.2,
                        amountOut: tuple.3,
                        tokenOutIndex: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<calcAmountInReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calcAmountInReturn) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calcAmountInReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calcAmountInCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calcAmountInReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calcAmountIn(address,address,address,uint256,uint8)";
            const SELECTOR: [u8; 4] = [194u8, 189u8, 237u8, 161u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOut),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenOutIndex),
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
    /**Function with signature `calcAmountOut(address,address,address,uint256,uint8)` and selector `0xd28b0a15`.
```solidity
function calcAmountOut(address dataStore, address token0, address token1, uint256 amountIn, uint8 tokenInIndex) external view returns (uint256, uint256, int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcAmountOutCall {
        pub dataStore: alloy::sol_types::private::Address,
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenInIndex: u8,
    }
    ///Container type for the return parameters of the [`calcAmountOut(address,address,address,uint256,uint8)`](calcAmountOutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcAmountOutReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
        pub _2: alloy::sol_types::private::primitives::aliases::I256,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calcAmountOutCall> for UnderlyingRustTuple<'_> {
                fn from(value: calcAmountOutCall) -> Self {
                    (
                        value.dataStore,
                        value.token0,
                        value.token1,
                        value.amountIn,
                        value.tokenInIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calcAmountOutCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        token0: tuple.1,
                        token1: tuple.2,
                        amountIn: tuple.3,
                        tokenInIndex: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<calcAmountOutReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calcAmountOutReturn) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calcAmountOutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calcAmountOutCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calcAmountOutReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Int<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calcAmountOut(address,address,address,uint256,uint8)";
            const SELECTOR: [u8; 4] = [210u8, 139u8, 10u8, 21u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenInIndex),
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
    /**Function with signature `calcLiquidityOut(address,address,address,uint256,uint256)` and selector `0xf68a7131`.
```solidity
function calcLiquidityOut(address dataStore, address token0, address token1, uint256 amount0, uint256 amount1) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcLiquidityOutCall {
        pub dataStore: alloy::sol_types::private::Address,
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calcLiquidityOut(address,address,address,uint256,uint256)`](calcLiquidityOutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcLiquidityOutReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calcLiquidityOutCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calcLiquidityOutCall) -> Self {
                    (
                        value.dataStore,
                        value.token0,
                        value.token1,
                        value.amount0,
                        value.amount1,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calcLiquidityOutCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        token0: tuple.1,
                        token1: tuple.2,
                        amount0: tuple.3,
                        amount1: tuple.4,
                    }
                }
            }
        }
        {
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
            impl ::core::convert::From<calcLiquidityOutReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calcLiquidityOutReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calcLiquidityOutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calcLiquidityOutCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calcLiquidityOutReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calcLiquidityOut(address,address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [246u8, 138u8, 113u8, 49u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
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
    /**Function with signature `calcTokenPairOut(address,address,address,uint256)` and selector `0x317b50ec`.
```solidity
function calcTokenPairOut(address dataStore, address token0, address token1, uint256 liquidity) external view returns (uint256, uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcTokenPairOutCall {
        pub dataStore: alloy::sol_types::private::Address,
        pub token0: alloy::sol_types::private::Address,
        pub token1: alloy::sol_types::private::Address,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calcTokenPairOut(address,address,address,uint256)`](calcTokenPairOutCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calcTokenPairOutReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<calcTokenPairOutCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calcTokenPairOutCall) -> Self {
                    (value.dataStore, value.token0, value.token1, value.liquidity)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calcTokenPairOutCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        token0: tuple.1,
                        token1: tuple.2,
                        liquidity: tuple.3,
                    }
                }
            }
        }
        {
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
            impl ::core::convert::From<calcTokenPairOutReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calcTokenPairOutReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calcTokenPairOutReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calcTokenPairOutCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calcTokenPairOutReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calcTokenPairOut(address,address,address,uint256)";
            const SELECTOR: [u8; 4] = [49u8, 123u8, 80u8, 236u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
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
    /**Function with signature `getDefaultInterestRateStrategy(address)` and selector `0xe335adb7`.
```solidity
function getDefaultInterestRateStrategy(address dataStore) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultInterestRateStrategyCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDefaultInterestRateStrategy(address)`](getDefaultInterestRateStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultInterestRateStrategyReturn {
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
            impl ::core::convert::From<getDefaultInterestRateStrategyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultInterestRateStrategyCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultInterestRateStrategyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
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
            impl ::core::convert::From<getDefaultInterestRateStrategyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultInterestRateStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultInterestRateStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDefaultInterestRateStrategyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDefaultInterestRateStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDefaultInterestRateStrategy(address)";
            const SELECTOR: [u8; 4] = [227u8, 53u8, 173u8, 183u8];
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
                        &self.dataStore,
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
    /**Function with signature `getDefaultPoolConfiguration(address)` and selector `0x50ed592d`.
```solidity
function getDefaultPoolConfiguration(address dataStore) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultPoolConfigurationCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDefaultPoolConfiguration(address)`](getDefaultPoolConfigurationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultPoolConfigurationReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getDefaultPoolConfigurationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultPoolConfigurationCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultPoolConfigurationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<getDefaultPoolConfigurationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultPoolConfigurationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultPoolConfigurationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDefaultPoolConfigurationCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDefaultPoolConfigurationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDefaultPoolConfiguration(address)";
            const SELECTOR: [u8; 4] = [80u8, 237u8, 89u8, 45u8];
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
                        &self.dataStore,
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
    /**Function with signature `getMarginLevelThreshold(address)` and selector `0x57b91ca6`.
```solidity
function getMarginLevelThreshold(address dataStore) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMarginLevelThresholdCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMarginLevelThreshold(address)`](getMarginLevelThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMarginLevelThresholdReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getMarginLevelThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMarginLevelThresholdCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMarginLevelThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<getMarginLevelThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMarginLevelThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMarginLevelThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMarginLevelThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMarginLevelThresholdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMarginLevelThreshold(address)";
            const SELECTOR: [u8; 4] = [87u8, 185u8, 28u8, 166u8];
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
                        &self.dataStore,
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
    /**Function with signature `getPools(address)` and selector `0x5c39f467`.
```solidity
function getPools(address dataStore) external view returns (ReaderPoolUtils.GetPool[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools_0Call {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getPools(address)`](getPools_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools_0Return {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPoolUtils::GetPool as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<getPools_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPools_0Call) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPoolUtils::GetPool as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPools_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getPools_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPools_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPools_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPool>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPools(address)";
            const SELECTOR: [u8; 4] = [92u8, 57u8, 244u8, 103u8];
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
                        &self.dataStore,
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
    /**Function with signature `getPools(address,uint256,uint256)` and selector `0x8f6c7a3c`.
```solidity
function getPools(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPool[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools_1Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPools(address,uint256,uint256)`](getPools_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools_1Return {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPoolUtils::GetPool as alloy::sol_types::SolType>::RustType,
        >,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getPools_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPools_1Call) -> Self {
                    (value.dataStore, value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        start: tuple.1,
                        end: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPoolUtils::GetPool as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPools_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getPools_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPools_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPools_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPool>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPools(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [143u8, 108u8, 122u8, 60u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.end),
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
    /**Function with signature `getPoolsCount(address)` and selector `0x5a6f5710`.
```solidity
function getPoolsCount(address dataStore) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsCountCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getPoolsCount(address)`](getPoolsCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsCountReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getPoolsCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsCountCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolsCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
                }
            }
        }
        {
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
            impl ::core::convert::From<getPoolsCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolsCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolsCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolsCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPoolsCount(address)";
            const SELECTOR: [u8; 4] = [90u8, 111u8, 87u8, 16u8];
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
                        &self.dataStore,
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
    /**Function with signature `getPoolsInfo(address)` and selector `0x1a308175`.
```solidity
function getPoolsInfo(address dataStore) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_0Call {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getPoolsInfo(address)`](getPoolsInfo_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_0Return {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<getPoolsInfo_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_0Call) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolsInfo_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPoolsInfo_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPoolsInfo_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolsInfo_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolsInfo_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPoolsInfo(address)";
            const SELECTOR: [u8; 4] = [26u8, 48u8, 129u8, 117u8];
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
                        &self.dataStore,
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
    /**Function with signature `getPoolsInfo(address,uint256,uint256)` and selector `0x382fc72e`.
```solidity
function getPoolsInfo(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_1Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub end: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPoolsInfo(address,uint256,uint256)`](getPoolsInfo_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_1Return {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
        >,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getPoolsInfo_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_1Call) -> Self {
                    (value.dataStore, value.start, value.end)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolsInfo_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        start: tuple.1,
                        end: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPoolsInfo_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPoolsInfo_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolsInfo_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolsInfo_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPoolsInfo(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [56u8, 47u8, 199u8, 46u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.end),
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
    /**Function with signature `getPoolsInfo(address,bytes32[])` and selector `0xeed07428`.
```solidity
function getPoolsInfo(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_2Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub poolKeys: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getPoolsInfo(address,bytes32[])`](getPoolsInfo_2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPoolsInfo_2Return {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
        >,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getPoolsInfo_2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_2Call) -> Self {
                    (value.dataStore, value.poolKeys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPoolsInfo_2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        poolKeys: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPoolUtils::GetPoolInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPoolsInfo_2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPoolsInfo_2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPoolsInfo_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPoolsInfo_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPoolsInfo_2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPoolInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPoolsInfo(address,bytes32[])";
            const SELECTOR: [u8; 4] = [238u8, 208u8, 116u8, 40u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.poolKeys),
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
    /**Function with signature `getPositions(address,address)` and selector `0x739118a4`.
```solidity
function getPositions(address dataStore, address account) external view returns (ReaderPositionUtils.GetPosition[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionsCall {
        pub dataStore: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getPositions(address,address)`](getPositionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositionsReturn {
        pub _0: alloy::sol_types::private::Vec<
            <ReaderPositionUtils::GetPosition as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<getPositionsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionsCall) -> Self {
                    (value.dataStore, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        account: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPositionUtils::GetPosition>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ReaderPositionUtils::GetPosition as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getPositionsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPositionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositionsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositionsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPositionUtils::GetPosition>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPositions(address,address)";
            const SELECTOR: [u8; 4] = [115u8, 145u8, 24u8, 164u8];
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
                        &self.dataStore,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
    /**Function with signature `getTokenBase(address)` and selector `0x28a0ccf4`.
```solidity
function getTokenBase(address dataStore) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTokenBaseCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getTokenBase(address)`](getTokenBaseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTokenBaseReturn {
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
            impl ::core::convert::From<getTokenBaseCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTokenBaseCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTokenBaseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
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
            impl ::core::convert::From<getTokenBaseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTokenBaseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTokenBaseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTokenBaseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTokenBaseReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTokenBase(address)";
            const SELECTOR: [u8; 4] = [40u8, 160u8, 204u8, 244u8];
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
                        &self.dataStore,
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
    /**Function with signature `getTreasury(address)` and selector `0x78f212d1`.
```solidity
function getTreasury(address dataStore) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTreasuryCall {
        pub dataStore: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getTreasury(address)`](getTreasuryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTreasuryReturn {
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
            impl ::core::convert::From<getTreasuryCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTreasuryCall) -> Self {
                    (value.dataStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTreasuryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataStore: tuple.0 }
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
            impl ::core::convert::From<getTreasuryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTreasuryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTreasuryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTreasuryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTreasuryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTreasury(address)";
            const SELECTOR: [u8; 4] = [120u8, 242u8, 18u8, 209u8];
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
                        &self.dataStore,
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
    ///Container for all the [`Reader`](self) function calls.
    pub enum ReaderCalls {
        calcAmountIn(calcAmountInCall),
        calcAmountOut(calcAmountOutCall),
        calcLiquidityOut(calcLiquidityOutCall),
        calcTokenPairOut(calcTokenPairOutCall),
        getDefaultInterestRateStrategy(getDefaultInterestRateStrategyCall),
        getDefaultPoolConfiguration(getDefaultPoolConfigurationCall),
        getMarginLevelThreshold(getMarginLevelThresholdCall),
        getPools_0(getPools_0Call),
        getPools_1(getPools_1Call),
        getPoolsCount(getPoolsCountCall),
        getPoolsInfo_0(getPoolsInfo_0Call),
        getPoolsInfo_1(getPoolsInfo_1Call),
        getPoolsInfo_2(getPoolsInfo_2Call),
        getPositions(getPositionsCall),
        getTokenBase(getTokenBaseCall),
        getTreasury(getTreasuryCall),
    }
    #[automatically_derived]
    impl ReaderCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [26u8, 48u8, 129u8, 117u8],
            [40u8, 160u8, 204u8, 244u8],
            [49u8, 123u8, 80u8, 236u8],
            [56u8, 47u8, 199u8, 46u8],
            [80u8, 237u8, 89u8, 45u8],
            [87u8, 185u8, 28u8, 166u8],
            [90u8, 111u8, 87u8, 16u8],
            [92u8, 57u8, 244u8, 103u8],
            [115u8, 145u8, 24u8, 164u8],
            [120u8, 242u8, 18u8, 209u8],
            [143u8, 108u8, 122u8, 60u8],
            [194u8, 189u8, 237u8, 161u8],
            [210u8, 139u8, 10u8, 21u8],
            [227u8, 53u8, 173u8, 183u8],
            [238u8, 208u8, 116u8, 40u8],
            [246u8, 138u8, 113u8, 49u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ReaderCalls {
        const NAME: &'static str = "ReaderCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::calcAmountIn(_) => {
                    <calcAmountInCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calcAmountOut(_) => {
                    <calcAmountOutCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calcLiquidityOut(_) => {
                    <calcLiquidityOutCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calcTokenPairOut(_) => {
                    <calcTokenPairOutCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDefaultInterestRateStrategy(_) => {
                    <getDefaultInterestRateStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDefaultPoolConfiguration(_) => {
                    <getDefaultPoolConfigurationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMarginLevelThreshold(_) => {
                    <getMarginLevelThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPools_0(_) => {
                    <getPools_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPools_1(_) => {
                    <getPools_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPoolsCount(_) => {
                    <getPoolsCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPoolsInfo_0(_) => {
                    <getPoolsInfo_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPoolsInfo_1(_) => {
                    <getPoolsInfo_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPoolsInfo_2(_) => {
                    <getPoolsInfo_2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPositions(_) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTokenBase(_) => {
                    <getTokenBaseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTreasury(_) => {
                    <getTreasuryCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ReaderCalls>] = &[
                {
                    fn getPoolsInfo_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPoolsInfo_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPoolsInfo_0)
                    }
                    getPoolsInfo_0
                },
                {
                    fn getTokenBase(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getTokenBaseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getTokenBase)
                    }
                    getTokenBase
                },
                {
                    fn calcTokenPairOut(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <calcTokenPairOutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::calcTokenPairOut)
                    }
                    calcTokenPairOut
                },
                {
                    fn getPoolsInfo_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPoolsInfo_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPoolsInfo_1)
                    }
                    getPoolsInfo_1
                },
                {
                    fn getDefaultPoolConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getDefaultPoolConfigurationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getDefaultPoolConfiguration)
                    }
                    getDefaultPoolConfiguration
                },
                {
                    fn getMarginLevelThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getMarginLevelThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getMarginLevelThreshold)
                    }
                    getMarginLevelThreshold
                },
                {
                    fn getPoolsCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPoolsCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPoolsCount)
                    }
                    getPoolsCount
                },
                {
                    fn getPools_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPools_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPools_0)
                    }
                    getPools_0
                },
                {
                    fn getPositions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPositionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPositions)
                    }
                    getPositions
                },
                {
                    fn getTreasury(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getTreasuryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getTreasury)
                    }
                    getTreasury
                },
                {
                    fn getPools_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPools_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPools_1)
                    }
                    getPools_1
                },
                {
                    fn calcAmountIn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <calcAmountInCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::calcAmountIn)
                    }
                    calcAmountIn
                },
                {
                    fn calcAmountOut(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <calcAmountOutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::calcAmountOut)
                    }
                    calcAmountOut
                },
                {
                    fn getDefaultInterestRateStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getDefaultInterestRateStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getDefaultInterestRateStrategy)
                    }
                    getDefaultInterestRateStrategy
                },
                {
                    fn getPoolsInfo_2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPoolsInfo_2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPoolsInfo_2)
                    }
                    getPoolsInfo_2
                },
                {
                    fn calcLiquidityOut(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <calcLiquidityOutCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::calcLiquidityOut)
                    }
                    calcLiquidityOut
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
                Self::calcAmountIn(inner) => {
                    <calcAmountInCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calcAmountOut(inner) => {
                    <calcAmountOutCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calcLiquidityOut(inner) => {
                    <calcLiquidityOutCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calcTokenPairOut(inner) => {
                    <calcTokenPairOutCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDefaultInterestRateStrategy(inner) => {
                    <getDefaultInterestRateStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDefaultPoolConfiguration(inner) => {
                    <getDefaultPoolConfigurationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMarginLevelThreshold(inner) => {
                    <getMarginLevelThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPools_0(inner) => {
                    <getPools_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPools_1(inner) => {
                    <getPools_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPoolsCount(inner) => {
                    <getPoolsCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPoolsInfo_0(inner) => {
                    <getPoolsInfo_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPoolsInfo_1(inner) => {
                    <getPoolsInfo_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPoolsInfo_2(inner) => {
                    <getPoolsInfo_2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPositions(inner) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTokenBase(inner) => {
                    <getTokenBaseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTreasury(inner) => {
                    <getTreasuryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::calcAmountIn(inner) => {
                    <calcAmountInCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calcAmountOut(inner) => {
                    <calcAmountOutCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calcLiquidityOut(inner) => {
                    <calcLiquidityOutCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calcTokenPairOut(inner) => {
                    <calcTokenPairOutCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDefaultInterestRateStrategy(inner) => {
                    <getDefaultInterestRateStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDefaultPoolConfiguration(inner) => {
                    <getDefaultPoolConfigurationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMarginLevelThreshold(inner) => {
                    <getMarginLevelThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPools_0(inner) => {
                    <getPools_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPools_1(inner) => {
                    <getPools_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPoolsCount(inner) => {
                    <getPoolsCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPoolsInfo_0(inner) => {
                    <getPoolsInfo_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPoolsInfo_1(inner) => {
                    <getPoolsInfo_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPoolsInfo_2(inner) => {
                    <getPoolsInfo_2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPositions(inner) => {
                    <getPositionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTokenBase(inner) => {
                    <getTokenBaseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTreasury(inner) => {
                    <getTreasuryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Reader`](self) custom errors.
    pub enum ReaderErrors {
        EmptyPool(EmptyPool),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        SingleTokenInOutSwapOnly(SingleTokenInOutSwapOnly),
    }
    #[automatically_derived]
    impl ReaderErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [34u8, 123u8, 193u8, 83u8],
            [99u8, 49u8, 250u8, 177u8],
            [115u8, 87u8, 217u8, 31u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ReaderErrors {
        const NAME: &'static str = "ReaderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::MathOverflowedMulDiv(_) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SingleTokenInOutSwapOnly(_) => {
                    <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ReaderErrors>] = &[
                {
                    fn MathOverflowedMulDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderErrors> {
                        <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderErrors::MathOverflowedMulDiv)
                    }
                    MathOverflowedMulDiv
                },
                {
                    fn SingleTokenInOutSwapOnly(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderErrors> {
                        <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderErrors::SingleTokenInOutSwapOnly)
                    }
                    SingleTokenInOutSwapOnly
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderErrors::EmptyPool)
                    }
                    EmptyPool
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
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SingleTokenInOutSwapOnly(inner) => {
                    <SingleTokenInOutSwapOnly as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Reader`](self) contract instance.

See the [wrapper's documentation](`ReaderInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ReaderInstance<T, P, N> {
        ReaderInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ReaderInstance<T, P, N>>,
    > {
        ReaderInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        ReaderInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Reader`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Reader`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ReaderInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ReaderInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ReaderInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReaderInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Reader`](self) contract instance.

See the [wrapper's documentation](`ReaderInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ReaderInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> ReaderInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ReaderInstance<T, P, N> {
            ReaderInstance {
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
    > ReaderInstance<T, P, N> {
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
        ///Creates a new call builder for the [`calcAmountIn`] function.
        pub fn calcAmountIn(
            &self,
            dataStore: alloy::sol_types::private::Address,
            token0: alloy::sol_types::private::Address,
            token1: alloy::sol_types::private::Address,
            amountOut: alloy::sol_types::private::primitives::aliases::U256,
            tokenOutIndex: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, calcAmountInCall, N> {
            self.call_builder(
                &calcAmountInCall {
                    dataStore,
                    token0,
                    token1,
                    amountOut,
                    tokenOutIndex,
                },
            )
        }
        ///Creates a new call builder for the [`calcAmountOut`] function.
        pub fn calcAmountOut(
            &self,
            dataStore: alloy::sol_types::private::Address,
            token0: alloy::sol_types::private::Address,
            token1: alloy::sol_types::private::Address,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            tokenInIndex: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, calcAmountOutCall, N> {
            self.call_builder(
                &calcAmountOutCall {
                    dataStore,
                    token0,
                    token1,
                    amountIn,
                    tokenInIndex,
                },
            )
        }
        ///Creates a new call builder for the [`calcLiquidityOut`] function.
        pub fn calcLiquidityOut(
            &self,
            dataStore: alloy::sol_types::private::Address,
            token0: alloy::sol_types::private::Address,
            token1: alloy::sol_types::private::Address,
            amount0: alloy::sol_types::private::primitives::aliases::U256,
            amount1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calcLiquidityOutCall, N> {
            self.call_builder(
                &calcLiquidityOutCall {
                    dataStore,
                    token0,
                    token1,
                    amount0,
                    amount1,
                },
            )
        }
        ///Creates a new call builder for the [`calcTokenPairOut`] function.
        pub fn calcTokenPairOut(
            &self,
            dataStore: alloy::sol_types::private::Address,
            token0: alloy::sol_types::private::Address,
            token1: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calcTokenPairOutCall, N> {
            self.call_builder(
                &calcTokenPairOutCall {
                    dataStore,
                    token0,
                    token1,
                    liquidity,
                },
            )
        }
        ///Creates a new call builder for the [`getDefaultInterestRateStrategy`] function.
        pub fn getDefaultInterestRateStrategy(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getDefaultInterestRateStrategyCall,
            N,
        > {
            self.call_builder(
                &getDefaultInterestRateStrategyCall {
                    dataStore,
                },
            )
        }
        ///Creates a new call builder for the [`getDefaultPoolConfiguration`] function.
        pub fn getDefaultPoolConfiguration(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDefaultPoolConfigurationCall, N> {
            self.call_builder(
                &getDefaultPoolConfigurationCall {
                    dataStore,
                },
            )
        }
        ///Creates a new call builder for the [`getMarginLevelThreshold`] function.
        pub fn getMarginLevelThreshold(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMarginLevelThresholdCall, N> {
            self.call_builder(
                &getMarginLevelThresholdCall {
                    dataStore,
                },
            )
        }
        ///Creates a new call builder for the [`getPools_0`] function.
        pub fn getPools_0(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPools_0Call, N> {
            self.call_builder(&getPools_0Call { dataStore })
        }
        ///Creates a new call builder for the [`getPools_1`] function.
        pub fn getPools_1(
            &self,
            dataStore: alloy::sol_types::private::Address,
            start: alloy::sol_types::private::primitives::aliases::U256,
            end: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPools_1Call, N> {
            self.call_builder(
                &getPools_1Call {
                    dataStore,
                    start,
                    end,
                },
            )
        }
        ///Creates a new call builder for the [`getPoolsCount`] function.
        pub fn getPoolsCount(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolsCountCall, N> {
            self.call_builder(&getPoolsCountCall { dataStore })
        }
        ///Creates a new call builder for the [`getPoolsInfo_0`] function.
        pub fn getPoolsInfo_0(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolsInfo_0Call, N> {
            self.call_builder(&getPoolsInfo_0Call { dataStore })
        }
        ///Creates a new call builder for the [`getPoolsInfo_1`] function.
        pub fn getPoolsInfo_1(
            &self,
            dataStore: alloy::sol_types::private::Address,
            start: alloy::sol_types::private::primitives::aliases::U256,
            end: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolsInfo_1Call, N> {
            self.call_builder(
                &getPoolsInfo_1Call {
                    dataStore,
                    start,
                    end,
                },
            )
        }
        ///Creates a new call builder for the [`getPoolsInfo_2`] function.
        pub fn getPoolsInfo_2(
            &self,
            dataStore: alloy::sol_types::private::Address,
            poolKeys: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPoolsInfo_2Call, N> {
            self.call_builder(
                &getPoolsInfo_2Call {
                    dataStore,
                    poolKeys,
                },
            )
        }
        ///Creates a new call builder for the [`getPositions`] function.
        pub fn getPositions(
            &self,
            dataStore: alloy::sol_types::private::Address,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositionsCall, N> {
            self.call_builder(
                &getPositionsCall {
                    dataStore,
                    account,
                },
            )
        }
        ///Creates a new call builder for the [`getTokenBase`] function.
        pub fn getTokenBase(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTokenBaseCall, N> {
            self.call_builder(&getTokenBaseCall { dataStore })
        }
        ///Creates a new call builder for the [`getTreasury`] function.
        pub fn getTreasury(
            &self,
            dataStore: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTreasuryCall, N> {
            self.call_builder(&getTreasuryCall { dataStore })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ReaderInstance<T, P, N> {
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
