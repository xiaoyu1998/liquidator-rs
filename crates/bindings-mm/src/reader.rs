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
    error EmptyAmount();
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
    function getPools2(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPool[] memory);
    function getPoolsCount(address dataStore) external view returns (uint256);
    function getPoolsInfo(address dataStore) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPositions(address dataStore, address account) external view returns (ReaderPositionUtils.GetPosition[] memory);
    function getPositions2(address dataStore, bytes32[] memory positionKeys) external view returns (ReaderPositionUtils.GetPosition[] memory);
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
    "name": "getPools2",
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
    "name": "getPositions2",
    "inputs": [
      {
        "name": "dataStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionKeys",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
    ///0x6080604052348015600e575f5ffd5b506173398061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610111575f3560e01c8063739118a41161009e578063c2bdeda11161006e578063c2bdeda114610277578063d28b0a15146102a5578063e335adb7146102b8578063eed07428146102cb578063f68a7131146102de575f5ffd5b8063739118a41461021e57806378f212d11461023e5780638f6c7a3c14610251578063a6a100a014610264575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c457806357b91ca6146101e55780635a6f5710146101f85780635c39f4671461020b575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b610128610123366004616830565b6102f1565b6040516101359190616879565b60405180910390f35b61015161014c366004616830565b610311565b6040516001600160a01b039091168152602001610135565b61017c610177366004616964565b610321565b60408051928352602083019190915201610135565b61012861019f3660046169b2565b61033c565b6101b76101b2366004616a4c565b610351565b6040516101359190616bcd565b6101d76101d2366004616830565b61035d565b604051908152602001610135565b6101d76101f3366004616830565b610367565b6101d7610206366004616830565b610371565b6101b7610219366004616830565b61037b565b61023161022c366004616cec565b610394565b6040516101359190616dd6565b61015161024c366004616830565b6103af565b6101b761025f3660046169b2565b6103b9565b610231610272366004616a4c565b6103c6565b61028a610285366004616eaa565b6103d2565b60408051938452602084019290925290820152606001610135565b61028a6102b3366004616eaa565b6103f4565b6101516102c6366004616830565b610404565b6101286102d9366004616a4c565b61040e565b6101d76102ec366004616f0e565b61041a565b60605f6102fd83610434565b905061030a835f836104c6565b9392505050565b5f61031b826104e0565b92915050565b5f5f61032f86868686610591565b9150915094509492505050565b60606103498484846104c6565b949350505050565b606061030a83836105cd565b5f61031b8261068b565b5f61031b826106dc565b5f61031b82610434565b60605f61038783610434565b905061030a835f8361072d565b60605f6103a184846107fb565b905061034984845f84610871565b5f61031b82610941565b606061034984848461072d565b606061030a838361097d565b5f5f5f6103e28888888888610a33565b9250925092505b955095509592505050565b5f5f5f6103e28888888888610b3d565b5f61031b82610bb6565b606061030a8383610c07565b5f6104288686868686610cbd565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161045390616f65565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161048791815260200190565b602060405180830381865afa1580156104a2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061031b9190616f88565b60605f6104d4858585610cf0565b905061042b8582610d91565b5f816001600160a01b03166321f8a72160405160200161051e906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161055291815260200190565b602060405180830381865afa15801561056d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061031b9190616f9f565b5f5f5f61059e8686610e47565b90505f6105ab8883610eee565b90505f5f6105ba8a8489612166565b50919c909b509950505050505050505050565b60605f825167ffffffffffffffff8111156105ea576105ea6169e4565b60405190808252806020026020018201604052801561062357816020015b610610616277565b8152602001906001900390816106085790505b5090505f5b8351811015610683575f84828151811061064457610644616fba565b602002602001015190505f61065987836123bb565b90508084848151811061066e5761066e616fba565b60209081029190910101525050600101610628565b509392505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f56040516020016104539060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f61073b858585610cf0565b90505f815167ffffffffffffffff811115610758576107586169e4565b60405190808252806020026020018201604052801561079157816020015b61077e616277565b8152602001906001900390816107765790505b5090505f5b82518110156107f1575f8382815181106107b2576107b2616fba565b602002602001015190505f6107c789836123bb565b9050808484815181106107dc576107dc616fba565b60209081029190910101525050600101610796565b5095945050505050565b5f826001600160a01b031663f3903b9f6108148461285b565b6040518263ffffffff1660e01b815260040161083291815260200190565b602060405180830381865afa15801561084d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030a9190616f88565b60605f610880868686866128df565b90505f815167ffffffffffffffff81111561089d5761089d6169e4565b6040519080825280602002602001820160405280156108d657816020015b6108c36162f6565b8152602001906001900390816108bb5790505b5090505f5b8251811015610936575f8382815181106108f7576108f7616fba565b602002602001015190505f61090c8a83612967565b90508084848151811061092157610921616fba565b602090810291909101015250506001016108db565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161051e90602080825260089082015267545245415355525960c01b604082015260600190565b60605f825167ffffffffffffffff81111561099a5761099a6169e4565b6040519080825280602002602001820160405280156109d357816020015b6109c06162f6565b8152602001906001900390816109b85790505b5090505f5b8351811015610683575f8482815181106109f4576109f4616fba565b602002602001015190505f610a098783612967565b905080848481518110610a1e57610a1e616fba565b602090810291909101015250506001016109d8565b5f5f5f5f610a418888610e47565b90505f610a4e8a83610eee565b90505f808060ff8916610a8457610a668d8b86612f8c565b929550919350610a7d91508590505f858d826130a6565b9050610ab4565b5f1960ff8a1601610ab457610a9a8d8b8661328b565b929550919350610ab19150859050845f808e6130a6565b90505b805f03610ace575f5f5f97509750975050505050506103e9565b5f610ad88561338d565b90505f828211610af157610aec8284616fe2565b610afb565b610afb8383616fe2565b90505f610b088284613432565b90505f848411610b2057610b1b82616ff5565b610b22565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f610b4b8888610e47565b90505f610b588a83610eee565b90505f808060ff8916610b8857610b718d8b865f61346d565b929550919350610a7d91508590508b5f80876130a6565b5f1960ff8a1601610ab457610b9f8d8b865f613597565b929550919350610ab191508590505f8c86826130a6565b5f816001600160a01b03166321f8a72160405160200161051e906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60605f825167ffffffffffffffff811115610c2457610c246169e4565b604051908082528060200260200182016040528015610c5d57816020015b610c4a616349565b815260200190600190039081610c425790505b5090505f5b8351811015610683575f848281518110610c7e57610c7e616fba565b602002602001015190505f610c9387836136a6565b905080848481518110610ca857610ca8616fba565b60209081029190910101525050600101610c62565b5f5f610cc98686610e47565b90505f610cd68883610eee565b9050610ce48186865f613966565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001610d1090616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015610d6a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610349919081019061700f565b60605f825167ffffffffffffffff811115610dae57610dae6169e4565b604051908082528060200260200182016040528015610de757816020015b610dd4616349565b815260200190600190039081610dcc5790505b5090505f5b8351811015610683575f848281518110610e0857610e08616fba565b602002602001015190505f610e1d87836136a6565b905080848481518110610e3257610e32616fba565b60209081029190910101525050600101610dec565b5f816001600160a01b0316836001600160a01b031610610e68578183610e6b565b82825b6040519194509250610e98906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610ef661636e565b82610eff61636e565b816001600160a01b03166391d4403c604051602001610f1d90616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610f71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f959190617096565b610fa257915061031b9050565b816001600160a01b03166321f8a72185604051602001610fe2906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611012929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161104691815260200190565b602060405180830381865afa158015611061573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110859190616f9f565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611103929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113791815260200190565b602060405180830381865afa158015611152573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111769190616f88565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016111cc906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016111fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161123091815260200190565b602060405180830381865afa15801561124b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061126f9190616f88565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016112d09060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611300929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161133491815260200190565b602060405180830381865afa15801561134f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113739190616f88565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016113de9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b6040516020818303038152906040528051906020012060405160200161140e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161144291815260200190565b602060405180830381865afa15801561145d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114819190616f88565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016114e29060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611512929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161154691815260200190565b602060405180830381865afa158015611561573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115859190616f88565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611602929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163691815260200190565b602060405180830381865afa158015611651573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116759190616f88565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016116ea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171e91815260200190565b602060405180830381865afa158015611739573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061175d9190616f9f565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161180791815260200190565b602060405180830381865afa158015611822573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118469190616f88565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161189d90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016118cd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161190191815260200190565b602060405180830381865afa15801561191c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119409190616f88565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016119a29060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016119d2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a0691815260200190565b602060405180830381865afa158015611a21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a459190616f88565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001611ab19060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b60405160208183030381529060405280519060200120604051602001611ae1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b1591815260200190565b602060405180830381865afa158015611b30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b549190616f88565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001611bb69060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611be6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611c1a91815260200190565b602060405180830381865afa158015611c35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c599190616f88565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001611cb290602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ce2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611d1691815260200190565b602060405180830381865afa158015611d31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d559190616f88565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001611da390602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611dd3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611e0791815260200190565b602060405180830381865afa158015611e22573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e469190616f9f565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611eb4906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611ee4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611f1891815260200190565b602060405180830381865afa158015611f33573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f579190616f9f565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611fba906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161201e91815260200190565b602060405180830381865afa158015612039573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061205d9190616f88565b60608201526040516001600160a01b0383169063bd02d0f59086906120b6906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016120e6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161211a91815260200190565b602060405180830381865afa158015612135573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121599190616f88565b6080820152949350505050565b5f5f5f5f6121726163a2565b61217b88613ac0565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e99190616f88565b816020018181525050612203875f5f846101400151613b11565b5060808401525060408201526101408101516122259088906001905f90613b11565b5060a084015250606082015260208101515f0361224e575f5f5f5f9450945094509450506123b2565b6122618682604001518360200151613bf6565b6101008201526060810151602082015161227c918891613bf6565b816101200181815250506122bc6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613cb5565b6122f4604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613cb5565b61232c60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613cb5565b61235f6040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613cb5565b6123926040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613cb5565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b6123c3616277565b6123cb6163f1565b6123d58484610eee565b81526123e084613ce2565b60208201526123ee84613ac0565b610180820181905281516020830151612409925f9190613b11565b608085015260a084015260408301526060820152805160208201516101808301516124379291600191613b11565b61012085015261014084015260e0830152610100820152805161245990613d25565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa1580156124d8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124ff91908101906170b5565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561254c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125709190617149565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612650573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261267791908101906170b5565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156126c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126eb9190617149565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906127cb9061338d565b81526020016127da8686613f34565b6001600160a01b03168152602001612801835f015160600151660800000000000016151590565b151581526020016128128686614024565b81528251515160c00151602082015282516040909101906128369087905f80614126565b815260200161284886845f0151614221565b90526101a0909101819052905092915050565b5f604051602001612895906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a6128f98661285b565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612940573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261042b919081019061700f565b61296f6162f6565b61297761645f565b612981848461422c565b8082525180515161299a9160015b602002015151610e47565b604082018190526129ac908590610eee565b6020820181905281516129c091869161423e565b50505050606082015260208101516129d79061338d565b61030082015260208101516129ed908590614221565b610320820152805180515160209081015160e084015280830151515101519051612a2391905f5b6020020151604001519061428e565b60c0820152602081015160600151612a3b905f6142cf565b60a082015260e081015160c0820151612a5491906142fd565b610100820181905260a0820151612a6b919061431e565b61012082015260e081015160c0820151610100830151612a8c92919061433a565b61014082015260208101518151610320830151612aad92879290915f614357565b61016082015261014081015161018082015260e081015160c0820151610120830151612ada92919061433a565b6101a0820152602081015160600151612af49060016142cf565b6101c082015280518051602090810151810151610200840152808301515181015101519051612b2591906001612a14565b6101e08201819052610200820151612b3c916142fd565b61022082018190526101c0820151612b54919061431e565b6102408201819052610300820151612b6c919061428e565b6102608201526102008101516101e0820151610220830151612b8f92919061433a565b61028082015260208101518151610320830151612bb192879290916001614357565b6102a08201526102808101516102c08201526102008101516101e0820151610260830151612be092919061433a565b6102e08201528051612bf1906145c0565b60808201528051516020015160e00151600214612ce957612c1b81610300015182608001516142fd565b6103408201819052610240820151612c329161428e565b61036082018190526080820151610300830151116103a083018190526102008301516101e0840151612c6393614602565b61038082018190526103c082015260e081015160a0820151612cca918691612c8b919061431e565b612c9d8460c001518560a0015161431e565b612cb1856102000151866101c0015161431e565b612cc5866101e00151876101c0015161431e565b61461c565b6103e08201819052610300820151612ce291906146dc565b6104008201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612d5d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d8491908101906170b5565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612dd957612dd9616fba565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612e2857612e28616fba565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612e88573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612eaf91908101906170b5565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103c001518152602001826103e00151815260200182610400015181525091505092915050565b5f5f5f5f612f98616542565b612fa188613ac0565b6101c08201819052612fb89087905f908190613b11565b5060408401525081526101c0810151612fd79087906001905f90613b11565b50606084015250602082015280511580612ff357506020810151155b1561300a575f5f5f5f9450945094509450506123b2565b606086015160381c61ffff166101408201819052613039908890613031906127109061472b565b612710613bf6565b61018082018190528151602083015161305c9261305790839061472b565b613bf6565b608082018190526020820151613072919061472b565b60e0820181905260408201516060830151610140840151613094908b90614780565b94509450945094505093509350935093565b5f5f5f5f861180156130b6575083155b156130c55750839050846130fa565b5f871180156130d2575084155b156130e15750859050826130fa565b604051636331fab160e01b815260040160405180910390fd5b811580613105575080155b15613123576040516301a2868b60e31b815260040160405180910390fd5b61314f6040518060400160405280600a815260200169616d6f756e744261736560b01b81525083613cb5565b61317b6040518060400160405280600a815260200169616d6f756e744d656d6560b01b81525082613cb5565b5f61318a89606001515f6142cf565b90505f61319c8a6060015160016142cf565b90505f6131ba85676765c793fa10079d601b1b61305786600a617247565b90505f6131d885676765c793fa10079d601b1b61305786600a617247565b905061320e6040518060400160405280601281526020017118985cd9505b5bdd5b9d10591a9d5cdd195960721b81525083613cb5565b613242604051806040016040528060128152602001711b595b59505b5bdd5b9d10591a9d5cdd195960721b81525082613cb5565b604080518082019091526005815264707269636560d81b60208201526132719061326c8484613432565b613cb5565b61327b8282613432565b9c9b505050505050505050505050565b5f5f5f5f613297616542565b6132a088613ac0565b6101c082018190526132b79087905f908190613b11565b5060408401525081526101c08101516132d69087906001905f90613b11565b506060840152506020820152805115806132f257506020810151155b15613309575f5f5f5f9450945094509450506123b2565b8051602082015161331f9190613057818b61472b565b61010082018190528151613333919061472b565b610120820152606086015160381c61ffff166101408201819052610120820151613366916127109061305790829061472b565b6101a082018190526040820151606083015161014084015161012085015161309491614780565b5f5f61339b835f5f5f613b11565b50505090505f6133ae8460015f5f613b11565b5050509050815f14806133bf575080155b156133cd57505f9392505050565b5f6133dc85606001515f6142cf565b90505f6133ee866060015160016142cf565b90505f61340c85676765c793fa10079d601b1b61305786600a617247565b90505f61342a85676765c793fa10079d601b1b61305786600a617247565b9050610ce482825b5f8115676765c793fa10079d601b1b60028404190484111715613453575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f613479616542565b61348289613ac0565b6101c082018190526134999088905f908190613b11565b5060408401525081526101c08101516134b89088906001905f90613b11565b506060840152506020820152805115806134d457506020810151155b156134eb575f5f5f5f94509450945094505061358c565b85156135065787815f018181516135029190616fe2565b9052505b606087015160381c61ffff16610140820181905261352d908990613031906127109061472b565b61016082018190528151602083015161354b926130579083906147a4565b6080820181905260208201516135609161472b565b60c0820181905260408201516060830151610140840151613582908c90614780565b9450945094509450505b945094509450949050565b5f5f5f5f6135a3616542565b6135ac89613ac0565b6101c082018190526135c39088905f908190613b11565b5060408401525081526101c08101516135e29088906001905f90613b11565b506060840152506020820152805115806135fe57506020810151155b15613615575f5f5f5f94509450945094505061358c565b851561363157878160200181815161362d9190616fe2565b9052505b805160208201516136479190613057818c6147a4565b6080820181905281516136599161472b565b60a0820152606087015160381c61ffff16610140820181905260a082015161368891613031906127109061472b565b6040820151606083015161014084015160a085015161358291614780565b6136ae616349565b6136b66165a9565b6136c08484610eee565b808252602001516001600160a01b03166136f557604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa158015613766573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261378d91908101906170b5565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa1580156137da573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137fe9190617149565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa15801561387e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526138a591908101906170b5565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156138f5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139199190617149565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff16815260200161395486845f0151614221565b90526020909101819052905092915050565b5f61396f6163a2565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156139af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139d39190616f88565b60208201526139e4865f8080613b11565b50505060c08201526139f98660015f80613b11565b50505060e08201528215613a3457848160c001818151613a199190616fe2565b90525060e081018051859190613a30908390616fe2565b9052505b80602001515f03613a6457613a5d6103e8613a57613a5288886147f8565b61485e565b9061472b565b8152613ab6565b60c08101511580613a77575060e0810151155b15613a85575f915050610349565b613ab3613a9b8683602001518460c00151613bf6565b613aae8684602001518560e00151613bf6565b61493e565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff1660028110613b2e57613b2e616fba565b602002015190505f613b408a8a614953565b9050805f03613b5c575f5f5f5f9550955095509550505061358c565b5f613b6b838c60800151614a41565b90505f613b78828a61428e565b90505f8915613b9d57818410613b9757613b92848361472b565b613b9f565b5f613b9f565b5f5b90505f613bac858d61428e565b90505f8c15613bd157848210613bcb57613bc6828661472b565b613bd3565b5f613bd3565b5f5b9050613bdf8587617252565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613c2a57838281613c2057613c20617265565b049250505061030a565b808411613c4a5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b613cde604051806040016040528060068152602001652573202d257360d01b8152508383614a71565b5050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f613d5a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613d64845f614abd565b602083015281526060840151613d7a905f6142cf565b606082018190528151613d9f91676765c793fa10079d601b1b9061305790600a617247565b604082015260208101515f03613dba575f6080820152613e5a565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613e30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e549190616f88565b60808201525b613e65846001614abd565b602083018190529082525f03613e80575f60a0820152613f20565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613ef6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f1a9190616f88565b60a08201525b80608001518160a001519250925050915091565b5f5f613f408484614b03565b9050806001600160a01b03166321f8a72184604051602001613f81906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613fb1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613fe591815260200190565b602060405180830381865afa158015614000573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103499190616f9f565b5f5f6140308484614b03565b9050806001600160a01b031663bd02d0f5846040516020016140839060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016140b3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140e791815260200190565b602060405180830381865afa158015614102573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103499190616f88565b5f5f614133856001614953565b90508215614148576141458482616fe2565b90505b5f61415287614bbe565b90505f61415f838361428e565b875160200151606001519091505f90821061418d57875160200151606001516141889083616fe2565b61418f565b5f5b90506141be6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525085613cb5565b6141ee6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525084613cb5565b610ce4604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525083613cb5565b5f61030a8383614c02565b6142346165c9565b61030a8383614c1b565b5f5f5f5f5f61424d8888614221565b905061425b8787835f615e39565b90935091508161426c575f19614276565b6142768383613432565b9450614281886106dc565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176142ae575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff8516016142ef575060ff60601b19905060605b90198416901c905092915050565b5f8183116143145761430f8383616fe2565b61030a565b61030a8284616fe2565b5f61030a83676765c793fa10079d601b1b61305785600a617247565b5f8284116143505761434b82616ff5565b610349565b5092915050565b5f6143916040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61439d8686865f615e39565b60208084019190915290825260408051808201909152600f81526e1d985c9ccb98dbdb1b185d195c985b608a1b9181019190915281516143dd9190613cb5565b61440c604051806040016040528060098152602001681d985c9ccb9919589d60ba1b8152508260200151613cb5565b80602001515f0361443e57845160ff84166002811061442d5761442d616fba565b60200201516020015191505061042b565b805115806144695750845160ff84166002811061445d5761445d616fba565b6020020151602001515f145b15614477575f91505061042b565b61448087615fa5565b6040820181905260208201516144959161428e565b608082019081526040805180820190915260118152701d985c9ccb98591a9d5cdd19591119589d607a1b602082015290516144d09190613cb5565b6080810151815110156144e6575f91505061042b565b608081015181516144f79190616fe2565b81606001818152505061450e8660600151846142cf565b60a08201819052606082015161453a9161452990600a617247565b676765c793fa10079d601b1b613bf6565b60c08201525f1960ff84160161455f5760c08101516145599085613432565b60c08201525b845160ff84166002811061457557614575616fba565b6020020151602001518160c0015111156145b257845160ff84166002811061459f5761459f616fba565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f19016145e0575051602001516060015190565b81516020015160e001516145fb575051602001516080015190565b505f919050565b5f841515838511146106835761461782616ff5565b61042b565b5f5f614627876106dc565b90505f614634828761428e565b90505f614641838661428e565b90505f61464e8984617279565b90505f61465b8389617279565b90505f61466783615feb565b90505f61467383615feb565b90505f8413801561468357505f83125b8061469757505f8412801561469757505f83135b156146ab575f97505050505050505061042b565b805f036146c1575f97505050505050505061042b565b6146cb8282613432565b9d9c50505050505050505050505050565b5f815f036146eb57505f61031b565b5f828411614702576146fd8484616fe2565b61470c565b61470c8385616fe2565b90505f6147198285613432565b90508385116103495761461781616ff5565b5f826147378382616fe2565b915081111561031b5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064016136ec565b5f81156113881983900484111517614796575f5ffd5b506127109102611388010490565b5f826147b08382617252565b915081101561031b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016136ec565b5f81158061481b5750828261480d8183617298565b925061481990836172af565b145b61031b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016136ec565b5f815f0361486d57505f919050565b5f600161487984616000565b901c6001901b9050600181848161489257614892617265565b048201901c905060018184816148aa576148aa617265565b048201901c905060018184816148c2576148c2617265565b048201901c905060018184816148da576148da617265565b048201901c905060018184816148f2576148f2617265565b048201901c9050600181848161490a5761490a617265565b048201901c9050600181848161492257614922617265565b048201901c905061030a8182858161493c5761493c617265565b045b5f81831061494c578161030a565b5090919050565b5f5f835f01518360ff166002811061496d5761496d616fba565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156149c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149ea9190616f88565b9050805f036149fd575f9250505061031b565b606082015160c0830151614a119082617252565b8210614a355760c0830151614a268284616fe2565b614a309190616fe2565b614a37565b5f5b9695505050505050565b5f8260a001515f03614a5457505f61031b565b5f614a5f8484616093565b60a0850151909150610349908261428e565b614ab8838383604051602401614a89939291906172ce565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b1790526160d6565b505050565b5f5f5f614aea855f01518560ff1660028110614adb57614adb616fba565b60200201518660800151614a41565b90505f614af78686614953565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001614b2690616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015614b7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b9e9190617096565b61030a57604051637357d91f60e01b8152600481018490526024016136ec565b5f816001600160a01b031663bd02d0f56040516020016104539060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5610814846160e2565b614c236165c9565b82614c2c6165c9565b816001600160a01b03166391d4403c604051602001614c6c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614cc0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ce49190617096565b614cf157915061031b9050565b816001600160a01b031663bd02d0f585604051602001614d2b906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d5b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d8f91815260200190565b602060405180830381865afa158015614daa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614dce9190616f88565b816020018181525050816001600160a01b03166321f8a72185604051602001614e16906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e46929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614e7a91815260200190565b602060405180830381865afa158015614e95573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614eb99190616f9f565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614f15906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f45929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614f7991815260200190565b602060405180830381865afa158015614f94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fb89190616f9f565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615033929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161506791815260200190565b602060405180830381865afa158015615082573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150a69190616f88565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016150fa9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161512a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161515e91815260200190565b602060405180830381865afa158015615179573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061519d9190616f88565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016151f7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615227929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525b91815260200190565b602060405180830381865afa158015615276573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061529a9190616f88565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016152f3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615323929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161535791815260200190565b602060405180830381865afa158015615372573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153969190616f88565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161541c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161545091815260200190565b602060405180830381865afa15801561546b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061548f9190616f88565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016154e9906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615519929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161554d91815260200190565b602060405180830381865afa158015615568573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061558c9190616f88565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016155ff929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161563391815260200190565b602060405180830381865afa15801561564e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156729190616f88565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016156e6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161571a91815260200190565b602060405180830381865afa158015615735573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157599190616f9f565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161580091815260200190565b602060405180830381865afa15801561581b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061583f9190616f88565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016158949060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016158c4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016158f891815260200190565b602060405180830381865afa158015615913573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159379190616f88565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161599290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016159c2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016159f691815260200190565b602060405180830381865afa158015615a11573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a359190616f88565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001615a8f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001615abf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615af391815260200190565b602060405180830381865afa158015615b0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b329190616f88565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001615b949060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615bc4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615bf891815260200190565b602060405180830381865afa158015615c13573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c379190616f88565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615c9290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615cc2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615cf691815260200190565b602060405180830381865afa158015615d11573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d359190616f88565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615d84906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615db4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615de891815260200190565b602060405180830381865afa158015615e03573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e279190616f88565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615edc575f5f615e648a8a5f61617d565b915091505f615e805f8c606001516142cf90919063ffffffff16565b90505f615e9e84676765c793fa10079d601b1b61305785600a617247565b90505f615ebc84676765c793fa10079d601b1b61305786600a617247565b9050615ec88288617252565b9650615ed48187617252565b955050505050505b865160200151516001600160a01b03868116911614615f98575f5f615f038a8a600161617d565b915091505f615f2060018c606001516142cf90919063ffffffff16565b90505f615f3e84676765c793fa10079d601b1b61305785600a617247565b90505f615f5c84676765c793fa10079d601b1b61305786600a617247565b90505f615f69838d61428e565b90505f615f76838e61428e565b9050615f82828a617252565b9850615f8e8189617252565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615ffc57815f0361031b565b5090565b5f80608083901c1561601457608092831c92015b604083901c1561602657604092831c92015b602083901c1561603857602092831c92015b601083901c1561604a57601092831c92015b600883901c1561605c57600892831c92015b600483901c1561606e57600492831c92015b600283901c1561608057600292831c92015b600183901c1561031b5760010192915050565b5f4282036160a65750602082015161031b565b5f6160b5846040015184616224565b90506160ce84602001518261428e90919063ffffffff16565b91505061031b565b6160df81616258565b50565b80518051515f9182916160f691600161298f565b90508060405160200161612f90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161615f929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff166002811061619857616198616fba565b6020020151604001519050805f036161b2575f91506161fa565b5f6161dd875f01518660ff16600281106161ce576161ce616fba565b60200201518860800151616093565b905081156161f4576161ef828261428e565b6161f6565b5f5b9250505b845160ff85166002811061621057616210616fba565b602002015160200151925050935093915050565b5f806162308342616fe2565b61623a9085617298565b6301e133809004905061034981676765c793fa10079d601b1b617252565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101a0016040528061628b6165ef565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b60405180610120016040528061630a616676565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806060016040528061635c6166f1565b81526020015f81526020015f81525090565b6040518060a00160405280616381616747565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c0016040528061640561636e565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f815260200161645a616277565b905290565b6040518061042001604052806164736165c9565b815260200161648061636e565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806165bc61636e565b815260200161645a616349565b60405180606001604052806165dc6167ae565b81525f6020820181905260409091015290565b60405180604001604052806002905b6166606040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165fe5790505090565b60405180604001604052806002905b6166db6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166855790505090565b60405180604001604052806002905b61673160405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816167005790505090565b60405180604001604052806002905b6167986040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167565790505090565b60405180604001604052806002905b6168066040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167bd5790505090565b6001600160a01b03811681146160df575f5ffd5b5f60208284031215616840575f5ffd5b813561030a8161681c565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857868503603f1901845281518051606080885260a08801919088015f5b600281101561692b57898403605f19018252825180516001600160a01b031685526020808201516080918701829052906168ff9087018261684b565b6040838101519088015260609283015192909601919091525060209283019291909101906001016168c3565b5050506020828101518882015260409283015192909701919091529493840193919091019060010161689f565b50929695505050505050565b5f5f5f5f60808587031215616977575f5ffd5b84356169828161681c565b935060208501356169928161681c565b925060408501356169a28161681c565b9396929550929360600135925050565b5f5f5f606084860312156169c4575f5ffd5b83356169cf8161681c565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616a2157616a216169e4565b604052919050565b5f67ffffffffffffffff821115616a4257616a426169e4565b5060051b60200190565b5f5f60408385031215616a5d575f5ffd5b8235616a688161681c565b9150602083013567ffffffffffffffff811115616a83575f5ffd5b8301601f81018513616a93575f5ffd5b8035616aa6616aa182616a29565b6169f8565b8082825260208201915060208360051b850101925087831115616ac7575f5ffd5b6020840193505b82841015616ae9578335825260209384019390910190616ace565b809450505050509250929050565b5f8260408101835f5b6002811015616bc2578383038752815180516001600160a01b0316845260208101516101806020860152616b3861018086018261684b565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616b00565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857603f19878603018452815180516101a08752616c1b6101a0880182616af7565b90506020820151616c3760208901826001600160a01b03169052565b506040820151616c5260408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616c9560e08901826001600160a01b03169052565b50610100820151616cab61010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616bf3565b5f5f60408385031215616cfd575f5ffd5b8235616d088161681c565b91506020830135616d188161681c565b809150509250929050565b5f8260408101835f5b6002811015616bc2578383038752815180516001600160a01b0316845260208101516101406020860152616d6461014086018261684b565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616d2c565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857603f19878603018452815180516101208752616e24610120880182616d23565b9050602082015160208801526040820151616e4a60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616dfc565b60ff811681146160df575f5ffd5b5f5f5f5f5f60a08688031215616ebe575f5ffd5b8535616ec98161681c565b94506020860135616ed98161681c565b93506040860135616ee98161681c565b9250606086013591506080860135616f0081616e9c565b809150509295509295909350565b5f5f5f5f5f60a08688031215616f22575f5ffd5b8535616f2d8161681c565b94506020860135616f3d8161681c565b93506040860135616f4d8161681c565b94979396509394606081013594506080013592915050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f60208284031215616f98575f5ffd5b5051919050565b5f60208284031215616faf575f5ffd5b815161030a8161681c565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561031b5761031b616fce565b5f600160ff1b820161700957617009616fce565b505f0390565b5f6020828403121561701f575f5ffd5b815167ffffffffffffffff811115617035575f5ffd5b8201601f81018413617045575f5ffd5b8051617053616aa182616a29565b8082825260208201915060208360051b850101925086831115617074575f5ffd5b6020840193505b82841015614a3757835182526020938401939091019061707b565b5f602082840312156170a6575f5ffd5b8151801515811461030a575f5ffd5b5f602082840312156170c5575f5ffd5b815167ffffffffffffffff8111156170db575f5ffd5b8201601f810184136170eb575f5ffd5b805167ffffffffffffffff811115617105576171056169e4565b617118601f8201601f19166020016169f8565b81815285602083850101111561712c575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215617159575f5ffd5b815161030a81616e9c565b6001815b600184111561719f5780850481111561718357617183616fce565b600184161561719157908102905b60019390931c928002617168565b935093915050565b5f826171b55750600161031b565b816171c157505f61031b565b81600181146171d757600281146171e1576171fd565b600191505061031b565b60ff8411156171f2576171f2616fce565b50506001821b61031b565b5060208310610133831016604e8410600b8410161715617220575081810a61031b565b61722c5f198484617164565b805f190482111561723f5761723f616fce565b029392505050565b5f61030a83836171a7565b8082018082111561031b5761031b616fce565b634e487b7160e01b5f52601260045260245ffd5b8181035f83128015838313168383128216171561435057614350616fce565b808202811582820484141761031b5761031b616fce565b5f826172c957634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f6172e0606083018661684b565b82810360208401526172f2818661684b565b91505082604083015294935050505056fea26469706673582212207dc01edc39a8e8d8ff7f610b342fea9607114612ab9bb022eb0be4136f36dfc064736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pas9\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02wW\x80c\xD2\x8B\n\x15\x14a\x02\xA5W\x80c\xE35\xAD\xB7\x14a\x02\xB8W\x80c\xEE\xD0t(\x14a\x02\xCBW\x80c\xF6\x8Aq1\x14a\x02\xDEW__\xFD[\x80cs\x91\x18\xA4\x14a\x02\x1EW\x80cx\xF2\x12\xD1\x14a\x02>W\x80c\x8Flz<\x14a\x02QW\x80c\xA6\xA1\0\xA0\x14a\x02dW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cW\xB9\x1C\xA6\x14a\x01\xE5W\x80cZoW\x10\x14a\x01\xF8W\x80c\\9\xF4g\x14a\x02\x0BW__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04ah0V[a\x02\xF1V[`@Qa\x015\x91\x90ahyV[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04ah0V[a\x03\x11V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04aidV[a\x03!V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ai\xB2V[a\x03<V[a\x01\xB7a\x01\xB26`\x04ajLV[a\x03QV[`@Qa\x015\x91\x90ak\xCDV[a\x01\xD7a\x01\xD26`\x04ah0V[a\x03]V[`@Q\x90\x81R` \x01a\x015V[a\x01\xD7a\x01\xF36`\x04ah0V[a\x03gV[a\x01\xD7a\x02\x066`\x04ah0V[a\x03qV[a\x01\xB7a\x02\x196`\x04ah0V[a\x03{V[a\x021a\x02,6`\x04al\xECV[a\x03\x94V[`@Qa\x015\x91\x90am\xD6V[a\x01Qa\x02L6`\x04ah0V[a\x03\xAFV[a\x01\xB7a\x02_6`\x04ai\xB2V[a\x03\xB9V[a\x021a\x02r6`\x04ajLV[a\x03\xC6V[a\x02\x8Aa\x02\x856`\x04an\xAAV[a\x03\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x8Aa\x02\xB36`\x04an\xAAV[a\x03\xF4V[a\x01Qa\x02\xC66`\x04ah0V[a\x04\x04V[a\x01(a\x02\xD96`\x04ajLV[a\x04\x0EV[a\x01\xD7a\x02\xEC6`\x04ao\x0EV[a\x04\x1AV[``_a\x02\xFD\x83a\x044V[\x90Pa\x03\n\x83_\x83a\x04\xC6V[\x93\x92PPPV[_a\x03\x1B\x82a\x04\xE0V[\x92\x91PPV[__a\x03/\x86\x86\x86\x86a\x05\x91V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03I\x84\x84\x84a\x04\xC6V[\x94\x93PPPPV[``a\x03\n\x83\x83a\x05\xCDV[_a\x03\x1B\x82a\x06\x8BV[_a\x03\x1B\x82a\x06\xDCV[_a\x03\x1B\x82a\x044V[``_a\x03\x87\x83a\x044V[\x90Pa\x03\n\x83_\x83a\x07-V[``_a\x03\xA1\x84\x84a\x07\xFBV[\x90Pa\x03I\x84\x84_\x84a\x08qV[_a\x03\x1B\x82a\tAV[``a\x03I\x84\x84\x84a\x07-V[``a\x03\n\x83\x83a\t}V[___a\x03\xE2\x88\x88\x88\x88\x88a\n3V[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x03\xE2\x88\x88\x88\x88\x88a\x0B=V[_a\x03\x1B\x82a\x0B\xB6V[``a\x03\n\x83\x83a\x0C\x07V[_a\x04(\x86\x86\x86\x86\x86a\x0C\xBDV[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04S\x90aoeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x87\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1B\x91\x90ao\x88V[``_a\x04\xD4\x85\x85\x85a\x0C\xF0V[\x90Pa\x04+\x85\x82a\r\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05R\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1B\x91\x90ao\x9FV[___a\x05\x9E\x86\x86a\x0EGV[\x90P_a\x05\xAB\x88\x83a\x0E\xEEV[\x90P__a\x05\xBA\x8A\x84\x89a!fV[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xEAWa\x05\xEAai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06#W\x81` \x01[a\x06\x10abwV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x08W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x06DWa\x06Dao\xBAV[` \x02` \x01\x01Q\x90P_a\x06Y\x87\x83a#\xBBV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06nWa\x06nao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06(V[P\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x07;\x85\x85\x85a\x0C\xF0V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07XWa\x07Xai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x91W\x81` \x01[a\x07~abwV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07vW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x07\xF1W_\x83\x82\x81Q\x81\x10a\x07\xB2Wa\x07\xB2ao\xBAV[` \x02` \x01\x01Q\x90P_a\x07\xC7\x89\x83a#\xBBV[\x90P\x80\x84\x84\x81Q\x81\x10a\x07\xDCWa\x07\xDCao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x07\x96V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x08\x14\x84a([V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x082\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\n\x91\x90ao\x88V[``_a\x08\x80\x86\x86\x86\x86a(\xDFV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9DWa\x08\x9Dai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD6W\x81` \x01[a\x08\xC3ab\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xBBW\x90P[P\x90P_[\x82Q\x81\x10\x15a\t6W_\x83\x82\x81Q\x81\x10a\x08\xF7Wa\x08\xF7ao\xBAV[` \x02` \x01\x01Q\x90P_a\t\x0C\x8A\x83a)gV[\x90P\x80\x84\x84\x81Q\x81\x10a\t!Wa\t!ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08\xDBV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x9AWa\t\x9Aai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xD3W\x81` \x01[a\t\xC0ab\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xB8W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\t\xF4Wa\t\xF4ao\xBAV[` \x02` \x01\x01Q\x90P_a\n\t\x87\x83a)gV[\x90P\x80\x84\x84\x81Q\x81\x10a\n\x1EWa\n\x1Eao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\t\xD8V[____a\nA\x88\x88a\x0EGV[\x90P_a\nN\x8A\x83a\x0E\xEEV[\x90P_\x80\x80`\xFF\x89\x16a\n\x84Wa\nf\x8D\x8B\x86a/\x8CV[\x92\x95P\x91\x93Pa\n}\x91P\x85\x90P_\x85\x8D\x82a0\xA6V[\x90Pa\n\xB4V[_\x19`\xFF\x8A\x16\x01a\n\xB4Wa\n\x9A\x8D\x8B\x86a2\x8BV[\x92\x95P\x91\x93Pa\n\xB1\x91P\x85\x90P\x84_\x80\x8Ea0\xA6V[\x90P[\x80_\x03a\n\xCEW___\x97P\x97P\x97PPPPPPa\x03\xE9V[_a\n\xD8\x85a3\x8DV[\x90P_\x82\x82\x11a\n\xF1Wa\n\xEC\x82\x84ao\xE2V[a\n\xFBV[a\n\xFB\x83\x83ao\xE2V[\x90P_a\x0B\x08\x82\x84a42V[\x90P_\x84\x84\x11a\x0B Wa\x0B\x1B\x82ao\xF5V[a\x0B\"V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x0BK\x88\x88a\x0EGV[\x90P_a\x0BX\x8A\x83a\x0E\xEEV[\x90P_\x80\x80`\xFF\x89\x16a\x0B\x88Wa\x0Bq\x8D\x8B\x86_a4mV[\x92\x95P\x91\x93Pa\n}\x91P\x85\x90P\x8B_\x80\x87a0\xA6V[_\x19`\xFF\x8A\x16\x01a\n\xB4Wa\x0B\x9F\x8D\x8B\x86_a5\x97V[\x92\x95P\x91\x93Pa\n\xB1\x91P\x85\x90P_\x8C\x86\x82a0\xA6V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C$Wa\x0C$ai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C]W\x81` \x01[a\x0CJacIV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0CBW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x0C~Wa\x0C~ao\xBAV[` \x02` \x01\x01Q\x90P_a\x0C\x93\x87\x83a6\xA6V[\x90P\x80\x84\x84\x81Q\x81\x10a\x0C\xA8Wa\x0C\xA8ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0CbV[__a\x0C\xC9\x86\x86a\x0EGV[\x90P_a\x0C\xD6\x88\x83a\x0E\xEEV[\x90Pa\x0C\xE4\x81\x86\x86_a9fV[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\r\x10\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rjW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03I\x91\x90\x81\x01\x90ap\x0FV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xAEWa\r\xAEai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xE7W\x81` \x01[a\r\xD4acIV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x0E\x08Wa\x0E\x08ao\xBAV[` \x02` \x01\x01Q\x90P_a\x0E\x1D\x87\x83a6\xA6V[\x90P\x80\x84\x84\x81Q\x81\x10a\x0E2Wa\x0E2ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\r\xECV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0EhW\x81\x83a\x0EkV[\x82\x82[`@Q\x91\x94P\x92Pa\x0E\x98\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x0E\xF6acnV[\x82a\x0E\xFFacnV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x0F\x1D\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x95\x91\x90ap\x96V[a\x0F\xA2W\x91Pa\x03\x1B\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x0F\xE2\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x12\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x85\x91\x90ao\x9FV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x117\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11RW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11v\x91\x90ao\x88V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\xCC\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x120\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12o\x91\x90ao\x88V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xD0\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x134\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13s\x91\x90ao\x88V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xDE\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x81\x91\x90ao\x88V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xE2\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x12\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x85\x91\x90ao\x88V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\x02\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x166\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16u\x91\x90ao\x88V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x179W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17]\x91\x90ao\x9FV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18F\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x18\x9D\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x01\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19@\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xA2\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AE\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1A\xB1\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BT\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1B\xB6\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CY\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1C\xB2\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DU\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1D\xA3\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EF\x91\x90ao\x9FV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1E\xB4\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xE4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x18\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FW\x91\x90ao\x9FV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1F\xBA\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a ]\x91\x90ao\x88V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a \xB6\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Y\x91\x90ao\x88V[`\x80\x82\x01R\x94\x93PPPPV[____a!rac\xA2V[a!{\x88a:\xC0V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE9\x91\x90ao\x88V[\x81` \x01\x81\x81RPPa\"\x03\x87__\x84a\x01@\x01Qa;\x11V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"%\x90\x88\x90`\x01\x90_\x90a;\x11V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"NW____\x94P\x94P\x94P\x94PPa#\xB2V[a\"a\x86\x82`@\x01Q\x83` \x01Qa;\xF6V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"|\x91\x88\x91a;\xF6V[\x81a\x01 \x01\x81\x81RPPa\"\xBC`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa<\xB5V[a\"\xF4`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa<\xB5V[a#,`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa<\xB5V[a#_`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa<\xB5V[a#\x92`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa<\xB5V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[a#\xC3abwV[a#\xCBac\xF1V[a#\xD5\x84\x84a\x0E\xEEV[\x81Ra#\xE0\x84a<\xE2V[` \x82\x01Ra#\xEE\x84a:\xC0V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa$\t\x92_\x91\x90a;\x11V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa$7\x92\x91`\x01\x91a;\x11V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa$Y\x90a=%V[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xD8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xFF\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a%LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%p\x91\x90aqIV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&PW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&w\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xEB\x91\x90aqIV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a'\xCB\x90a3\x8DV[\x81R` \x01a'\xDA\x86\x86a?4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a(\x01\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a(\x12\x86\x86a@$V[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a(6\x90\x87\x90_\x80aA&V[\x81R` \x01a(H\x86\x84_\x01QaB!V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a(\x95\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a(\xF9\x86a([V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)@W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04+\x91\x90\x81\x01\x90ap\x0FV[a)oab\xF6V[a)wad_V[a)\x81\x84\x84aB,V[\x80\x82RQ\x80QQa)\x9A\x91`\x01[` \x02\x01QQa\x0EGV[`@\x82\x01\x81\x90Ra)\xAC\x90\x85\x90a\x0E\xEEV[` \x82\x01\x81\x90R\x81Qa)\xC0\x91\x86\x91aB>V[PPPP``\x82\x01R` \x81\x01Qa)\xD7\x90a3\x8DV[a\x03\0\x82\x01R` \x81\x01Qa)\xED\x90\x85\x90aB!V[a\x03 \x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa*#\x91\x90_[` \x02\x01Q`@\x01Q\x90aB\x8EV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa*;\x90_aB\xCFV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa*T\x91\x90aB\xFDV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa*k\x91\x90aC\x1EV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa*\x8C\x92\x91\x90aC:V[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03 \x83\x01Qa*\xAD\x92\x87\x92\x90\x91_aCWV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa*\xDA\x92\x91\x90aC:V[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa*\xF4\x90`\x01aB\xCFV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa+%\x91\x90`\x01a*\x14V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa+<\x91aB\xFDV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa+T\x91\x90aC\x1EV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa+l\x91\x90aB\x8EV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa+\x8F\x92\x91\x90aC:V[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03 \x83\x01Qa+\xB1\x92\x87\x92\x90\x91`\x01aCWV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa+\xE0\x92\x91\x90aC:V[a\x02\xE0\x82\x01R\x80Qa+\xF1\x90aE\xC0V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a,\xE9Wa,\x1B\x81a\x03\0\x01Q\x82`\x80\x01QaB\xFDV[a\x03@\x82\x01\x81\x90Ra\x02@\x82\x01Qa,2\x91aB\x8EV[a\x03`\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\xA0\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa,c\x93aF\x02V[a\x03\x80\x82\x01\x81\x90Ra\x03\xC0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa,\xCA\x91\x86\x91a,\x8B\x91\x90aC\x1EV[a,\x9D\x84`\xC0\x01Q\x85`\xA0\x01QaC\x1EV[a,\xB1\x85a\x02\0\x01Q\x86a\x01\xC0\x01QaC\x1EV[a,\xC5\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01QaC\x1EV[aF\x1CV[a\x03\xE0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa,\xE2\x91\x90aF\xDCV[a\x04\0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-]W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x84\x91\x90\x81\x01\x90ap\xB5V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a-\xD9Wa-\xD9ao\xBAV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a.(Wa.(ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x88W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.\xAF\x91\x90\x81\x01\x90ap\xB5V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81R` \x01\x82a\x04\0\x01Q\x81RP\x91PP\x92\x91PPV[____a/\x98aeBV[a/\xA1\x88a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra/\xB8\x90\x87\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa/\xD7\x90\x87\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a/\xF3WP` \x81\x01Q\x15[\x15a0\nW____\x94P\x94P\x94P\x94PPa#\xB2V[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra09\x90\x88\x90a01\x90a'\x10\x90aG+V[a'\x10a;\xF6V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\\\x92a0W\x90\x83\x90aG+V[a;\xF6V[`\x80\x82\x01\x81\x90R` \x82\x01Qa0r\x91\x90aG+V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0\x94\x90\x8B\x90aG\x80V[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a0\xB6WP\x83\x15[\x15a0\xC5WP\x83\x90P\x84a0\xFAV[_\x87\x11\x80\x15a0\xD2WP\x84\x15[\x15a0\xE1WP\x85\x90P\x82a0\xFAV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80a1\x05WP\x80\x15[\x15a1#W`@Qc\x01\xA2\x86\x8B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a1O`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountBase`\xB0\x1B\x81RP\x83a<\xB5V[a1{`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountMeme`\xB0\x1B\x81RP\x82a<\xB5V[_a1\x8A\x89``\x01Q_aB\xCFV[\x90P_a1\x9C\x8A``\x01Q`\x01aB\xCFV[\x90P_a1\xBA\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a1\xD8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa2\x0E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\x98\\\xD9P[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x83a<\xB5V[a2B`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x1BY[YP[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x82a<\xB5V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra2q\x90a2l\x84\x84a42V[a<\xB5V[a2{\x82\x82a42V[\x9C\x9BPPPPPPPPPPPPV[____a2\x97aeBV[a2\xA0\x88a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra2\xB7\x90\x87\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa2\xD6\x90\x87\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a2\xF2WP` \x81\x01Q\x15[\x15a3\tW____\x94P\x94P\x94P\x94PPa#\xB2V[\x80Q` \x82\x01Qa3\x1F\x91\x90a0W\x81\x8BaG+V[a\x01\0\x82\x01\x81\x90R\x81Qa33\x91\x90aG+V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa3f\x91a'\x10\x90a0W\x90\x82\x90aG+V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0\x94\x91aG\x80V[__a3\x9B\x83___a;\x11V[PPP\x90P_a3\xAE\x84`\x01__a;\x11V[PPP\x90P\x81_\x14\x80a3\xBFWP\x80\x15[\x15a3\xCDWP_\x93\x92PPPV[_a3\xDC\x85``\x01Q_aB\xCFV[\x90P_a3\xEE\x86``\x01Q`\x01aB\xCFV[\x90P_a4\x0C\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a4*\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa\x0C\xE4\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a4SW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a4yaeBV[a4\x82\x89a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra4\x99\x90\x88\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4\xB8\x90\x88\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a4\xD4WP` \x81\x01Q\x15[\x15a4\xEBW____\x94P\x94P\x94P\x94PPa5\x8CV[\x85\x15a5\x06W\x87\x81_\x01\x81\x81Qa5\x02\x91\x90ao\xE2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra5-\x90\x89\x90a01\x90a'\x10\x90aG+V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa5K\x92a0W\x90\x83\x90aG\xA4V[`\x80\x82\x01\x81\x90R` \x82\x01Qa5`\x91aG+V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa5\x82\x90\x8C\x90aG\x80V[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a5\xA3aeBV[a5\xAC\x89a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra5\xC3\x90\x88\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa5\xE2\x90\x88\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a5\xFEWP` \x81\x01Q\x15[\x15a6\x15W____\x94P\x94P\x94P\x94PPa5\x8CV[\x85\x15a61W\x87\x81` \x01\x81\x81Qa6-\x91\x90ao\xE2V[\x90RP[\x80Q` \x82\x01Qa6G\x91\x90a0W\x81\x8CaG\xA4V[`\x80\x82\x01\x81\x90R\x81Qa6Y\x91aG+V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa6\x88\x91a01\x90a'\x10\x90aG+V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa5\x82\x91aG\x80V[a6\xAEacIV[a6\xB6ae\xA9V[a6\xC0\x84\x84a\x0E\xEEV[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a6\xF5W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7fW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\x8D\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xFE\x91\x90aqIV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a8~W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra8\xA5\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x19\x91\x90aqIV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a9T\x86\x84_\x01QaB!V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a9oac\xA2V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xD3\x91\x90ao\x88V[` \x82\x01Ra9\xE4\x86_\x80\x80a;\x11V[PPP`\xC0\x82\x01Ra9\xF9\x86`\x01_\x80a;\x11V[PPP`\xE0\x82\x01R\x82\x15a:4W\x84\x81`\xC0\x01\x81\x81Qa:\x19\x91\x90ao\xE2V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a:0\x90\x83\x90ao\xE2V[\x90RP[\x80` \x01Q_\x03a:dWa:]a\x03\xE8a:Wa:R\x88\x88aG\xF8V[aH^V[\x90aG+V[\x81Ra:\xB6V[`\xC0\x81\x01Q\x15\x80a:wWP`\xE0\x81\x01Q\x15[\x15a:\x85W_\x91PPa\x03IV[a:\xB3a:\x9B\x86\x83` \x01Q\x84`\xC0\x01Qa;\xF6V[a:\xAE\x86\x84` \x01Q\x85`\xE0\x01Qa;\xF6V[aI>V[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a;.Wa;.ao\xBAV[` \x02\x01Q\x90P_a;@\x8A\x8AaISV[\x90P\x80_\x03a;\\W____\x95P\x95P\x95P\x95PPPa5\x8CV[_a;k\x83\x8C`\x80\x01QaJAV[\x90P_a;x\x82\x8AaB\x8EV[\x90P_\x89\x15a;\x9DW\x81\x84\x10a;\x97Wa;\x92\x84\x83aG+V[a;\x9FV[_a;\x9FV[_[\x90P_a;\xAC\x85\x8DaB\x8EV[\x90P_\x8C\x15a;\xD1W\x84\x82\x10a;\xCBWa;\xC6\x82\x86aG+V[a;\xD3V[_a;\xD3V[_[\x90Pa;\xDF\x85\x87arRV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a<*W\x83\x82\x81a< Wa< areV[\x04\x92PPPa\x03\nV[\x80\x84\x11a<JW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a<\xDE`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aJqV[PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a=Z`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a=d\x84_aJ\xBDV[` \x83\x01R\x81R``\x84\x01Qa=z\x90_aB\xCFV[``\x82\x01\x81\x90R\x81Qa=\x9F\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a0W\x90`\narGV[`@\x82\x01R` \x81\x01Q_\x03a=\xBAW_`\x80\x82\x01Ra>ZV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>T\x91\x90ao\x88V[`\x80\x82\x01R[a>e\x84`\x01aJ\xBDV[` \x83\x01\x81\x90R\x90\x82R_\x03a>\x80W_`\xA0\x82\x01Ra? V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x1A\x91\x90ao\x88V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a?@\x84\x84aK\x03V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a?\x81\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03I\x91\x90ao\x9FV[__a@0\x84\x84aK\x03V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a@\x83\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03I\x91\x90ao\x88V[__aA3\x85`\x01aISV[\x90P\x82\x15aAHWaAE\x84\x82ao\xE2V[\x90P[_aAR\x87aK\xBEV[\x90P_aA_\x83\x83aB\x8EV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10aA\x8DW\x87Q` \x01Q``\x01QaA\x88\x90\x83ao\xE2V[aA\x8FV[_[\x90PaA\xBE`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a<\xB5V[aA\xEE`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a<\xB5V[a\x0C\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a<\xB5V[_a\x03\n\x83\x83aL\x02V[aB4ae\xC9V[a\x03\n\x83\x83aL\x1BV[_____aBM\x88\x88aB!V[\x90PaB[\x87\x87\x83_a^9V[\x90\x93P\x91P\x81aBlW_\x19aBvV[aBv\x83\x83a42V[\x94PaB\x81\x88a\x06\xDCV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aB\xAEW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aB\xEFWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11aC\x14WaC\x0F\x83\x83ao\xE2V[a\x03\nV[a\x03\n\x82\x84ao\xE2V[_a\x03\n\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[_\x82\x84\x11aCPWaCK\x82ao\xF5V[a\x03IV[P\x92\x91PPV[_aC\x91`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC\x9D\x86\x86\x86_a^9V[` \x80\x84\x01\x91\x90\x91R\x90\x82R`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn\x1D\x98\\\x9C\xCB\x98\xDB\xDB\x1B\x18]\x19\\\x98[`\x8A\x1B\x91\x81\x01\x91\x90\x91R\x81QaC\xDD\x91\x90a<\xB5V[aD\x0C`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x1D\x98\\\x9C\xCB\x99\x19X\x9D`\xBA\x1B\x81RP\x82` \x01Qa<\xB5V[\x80` \x01Q_\x03aD>W\x84Q`\xFF\x84\x16`\x02\x81\x10aD-WaD-ao\xBAV[` \x02\x01Q` \x01Q\x91PPa\x04+V[\x80Q\x15\x80aDiWP\x84Q`\xFF\x84\x16`\x02\x81\x10aD]WaD]ao\xBAV[` \x02\x01Q` \x01Q_\x14[\x15aDwW_\x91PPa\x04+V[aD\x80\x87a_\xA5V[`@\x82\x01\x81\x90R` \x82\x01QaD\x95\x91aB\x8EV[`\x80\x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x11\x81Rp\x1D\x98\\\x9C\xCB\x98Y\x1A\x9D\\\xDD\x19Y\x11\x19X\x9D`z\x1B` \x82\x01R\x90QaD\xD0\x91\x90a<\xB5V[`\x80\x81\x01Q\x81Q\x10\x15aD\xE6W_\x91PPa\x04+V[`\x80\x81\x01Q\x81QaD\xF7\x91\x90ao\xE2V[\x81``\x01\x81\x81RPPaE\x0E\x86``\x01Q\x84aB\xCFV[`\xA0\x82\x01\x81\x90R``\x82\x01QaE:\x91aE)\x90`\narGV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xF6V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01aE_W`\xC0\x81\x01QaEY\x90\x85a42V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10aEuWaEuao\xBAV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15aE\xB2W\x84Q`\xFF\x84\x16`\x02\x81\x10aE\x9FWaE\x9Fao\xBAV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01aE\xE0WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01QaE\xFBWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x06\x83WaF\x17\x82ao\xF5V[a\x04+V[__aF'\x87a\x06\xDCV[\x90P_aF4\x82\x87aB\x8EV[\x90P_aFA\x83\x86aB\x8EV[\x90P_aFN\x89\x84aryV[\x90P_aF[\x83\x89aryV[\x90P_aFg\x83a_\xEBV[\x90P_aFs\x83a_\xEBV[\x90P_\x84\x13\x80\x15aF\x83WP_\x83\x12[\x80aF\x97WP_\x84\x12\x80\x15aF\x97WP_\x83\x13[\x15aF\xABW_\x97PPPPPPPPa\x04+V[\x80_\x03aF\xC1W_\x97PPPPPPPPa\x04+V[aF\xCB\x82\x82a42V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aF\xEBWP_a\x03\x1BV[_\x82\x84\x11aG\x02WaF\xFD\x84\x84ao\xE2V[aG\x0CV[aG\x0C\x83\x85ao\xE2V[\x90P_aG\x19\x82\x85a42V[\x90P\x83\x85\x11a\x03IWaF\x17\x81ao\xF5V[_\x82aG7\x83\x82ao\xE2V[\x91P\x81\x11\x15a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a6\xECV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aG\x96W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aG\xB0\x83\x82arRV[\x91P\x81\x10\x15a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a6\xECV[_\x81\x15\x80aH\x1BWP\x82\x82aH\r\x81\x83ar\x98V[\x92PaH\x19\x90\x83ar\xAFV[\x14[a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a6\xECV[_\x81_\x03aHmWP_\x91\x90PV[_`\x01aHy\x84a`\0V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aH\x92WaH\x92areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xAAWaH\xAAareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xC2WaH\xC2areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xDAWaH\xDAareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xF2WaH\xF2areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\nWaI\nareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\"WaI\"areV[\x04\x82\x01\x90\x1C\x90Pa\x03\n\x81\x82\x85\x81aI<WaI<areV[\x04[_\x81\x83\x10aILW\x81a\x03\nV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aImWaImao\xBAV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xEA\x91\x90ao\x88V[\x90P\x80_\x03aI\xFDW_\x92PPPa\x03\x1BV[``\x82\x01Q`\xC0\x83\x01QaJ\x11\x90\x82arRV[\x82\x10aJ5W`\xC0\x83\x01QaJ&\x82\x84ao\xE2V[aJ0\x91\x90ao\xE2V[aJ7V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aJTWP_a\x03\x1BV[_aJ_\x84\x84a`\x93V[`\xA0\x85\x01Q\x90\x91Pa\x03I\x90\x82aB\x8EV[aJ\xB8\x83\x83\x83`@Q`$\x01aJ\x89\x93\x92\x91\x90ar\xCEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra`\xD6V[PPPV[___aJ\xEA\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aJ\xDBWaJ\xDBao\xBAV[` \x02\x01Q\x86`\x80\x01QaJAV[\x90P_aJ\xF7\x86\x86aISV[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aK&\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x9E\x91\x90ap\x96V[a\x03\nW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a6\xECV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x08\x14\x84a`\xE2V[aL#ae\xC9V[\x82aL,ae\xC9V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aLl\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xE4\x91\x90ap\x96V[aL\xF1W\x91Pa\x03\x1B\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM+\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x8F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xCE\x91\x90ao\x88V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aN\x16\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aNz\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x95W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xB9\x91\x90ao\x9FV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aO\x15\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aOE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aOy\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xB8\x91\x90ao\x9FV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aPg\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xA6\x91\x90ao\x88V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aP\xFA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQyW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x9D\x91\x90ao\x88V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQ\xF7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR[\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRvW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x9A\x91\x90ao\x88V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\xF3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSW\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aSrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x96\x91\x90ao\x88V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aTkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\x8F\x91\x90ao\x88V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\xE9\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUM\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aUhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x8C\x91\x90ao\x88V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aV3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVNW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aVr\x91\x90ao\x88V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aW\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aWY\x91\x90ao\x9FV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX\0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX?\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\x94\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\xC4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY7\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\x92\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aY\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZ\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ5\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\x8F\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZ\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[2\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\x94\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xC4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\7\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\\\x92\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]5\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a]\x84\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a]\xB4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\xE8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^'\x91\x90ao\x88V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a^\xDCW__a^d\x8A\x8A_aa}V[\x91P\x91P_a^\x80_\x8C``\x01QaB\xCF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a^\x9E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[\x90P_a^\xBC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa^\xC8\x82\x88arRV[\x96Pa^\xD4\x81\x87arRV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a_\x98W__a_\x03\x8A\x8A`\x01aa}V[\x91P\x91P_a_ `\x01\x8C``\x01QaB\xCF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a_>\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[\x90P_a_\\\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a_i\x83\x8DaB\x8EV[\x90P_a_v\x83\x8EaB\x8EV[\x90Pa_\x82\x82\x8AarRV[\x98Pa_\x8E\x81\x89arRV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a_\xFCW\x81_\x03a\x03\x1BV[P\x90V[_\x80`\x80\x83\x90\x1C\x15a`\x14W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a`&W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a`8W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a`JW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a`\\W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a`nW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a`\x80W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03\x1BW`\x01\x01\x92\x91PPV[_B\x82\x03a`\xA6WP` \x82\x01Qa\x03\x1BV[_a`\xB5\x84`@\x01Q\x84ab$V[\x90Pa`\xCE\x84` \x01Q\x82aB\x8E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03\x1BV[a`\xDF\x81abXV[PV[\x80Q\x80QQ_\x91\x82\x91a`\xF6\x91`\x01a)\x8FV[\x90P\x80`@Q` \x01aa/\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa_\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aa\x98Waa\x98ao\xBAV[` \x02\x01Q`@\x01Q\x90P\x80_\x03aa\xB2W_\x91Paa\xFAV[_aa\xDD\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aa\xCEWaa\xCEao\xBAV[` \x02\x01Q\x88`\x80\x01Qa`\x93V[\x90P\x81\x15aa\xF4Waa\xEF\x82\x82aB\x8EV[aa\xF6V[_[\x92PP[\x84Q`\xFF\x85\x16`\x02\x81\x10ab\x10Wab\x10ao\xBAV[` \x02\x01Q` \x01Q\x92PP\x93P\x93\x91PPV[_\x80ab0\x83Bao\xE2V[ab:\x90\x85ar\x98V[c\x01\xE13\x80\x90\x04\x90Pa\x03I\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BarRV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01\xA0\x01`@R\x80ab\x8Bae\xEFV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80ac\nafvV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ac\\af\xF1V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80ac\x81agGV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ad\x05acnV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01adZabwV[\x90R\x90V[`@Q\x80a\x04 \x01`@R\x80adsae\xC9V[\x81R` \x01ad\x80acnV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80ae\xBCacnV[\x81R` \x01adZacIV[`@Q\x80``\x01`@R\x80ae\xDCag\xAEV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af``@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xFEW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xDB`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81af\x85W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag1`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\0W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag\x98`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81agVW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ah\x06`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\xBDW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a`\xDFW__\xFD[_` \x82\x84\x03\x12\x15ah@W__\xFD[\x815a\x03\n\x81ah\x1CV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15ai+W\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ah\xFF\x90\x87\x01\x82ahKV[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ah\xC3V[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ah\x9FV[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15aiwW__\xFD[\x845ai\x82\x81ah\x1CV[\x93P` \x85\x015ai\x92\x81ah\x1CV[\x92P`@\x85\x015ai\xA2\x81ah\x1CV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ai\xC4W__\xFD[\x835ai\xCF\x81ah\x1CV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aj!Waj!ai\xE4V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ajBWajBai\xE4V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15aj]W__\xFD[\x825ajh\x81ah\x1CV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aj\x83W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aj\x93W__\xFD[\x805aj\xA6aj\xA1\x82aj)V[ai\xF8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aj\xC7W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aj\xE9W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aj\xCEV[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ak\xC2W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Rak8a\x01\x80\x86\x01\x82ahKV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pak\0V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Ral\x1Ba\x01\xA0\x88\x01\x82aj\xF7V[\x90P` \x82\x01Qal7` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QalR`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qal\x95`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qal\xABa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ak\xF3V[__`@\x83\x85\x03\x12\x15al\xFDW__\xFD[\x825am\x08\x81ah\x1CV[\x91P` \x83\x015am\x18\x81ah\x1CV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ak\xC2W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ramda\x01@\x86\x01\x82ahKV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pam,V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ran$a\x01 \x88\x01\x82am#V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QanJ`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01am\xFCV[`\xFF\x81\x16\x81\x14a`\xDFW__\xFD[_____`\xA0\x86\x88\x03\x12\x15an\xBEW__\xFD[\x855an\xC9\x81ah\x1CV[\x94P` \x86\x015an\xD9\x81ah\x1CV[\x93P`@\x86\x015an\xE9\x81ah\x1CV[\x92P``\x86\x015\x91P`\x80\x86\x015ao\0\x81an\x9CV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15ao\"W__\xFD[\x855ao-\x81ah\x1CV[\x94P` \x86\x015ao=\x81ah\x1CV[\x93P`@\x86\x015aoM\x81ah\x1CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15ao\x98W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ao\xAFW__\xFD[\x81Qa\x03\n\x81ah\x1CV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x1BWa\x03\x1Bao\xCEV[_`\x01`\xFF\x1B\x82\x01ap\tWap\tao\xCEV[P_\x03\x90V[_` \x82\x84\x03\x12\x15ap\x1FW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13apEW__\xFD[\x80QapSaj\xA1\x82aj)V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aptW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aJ7W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ap{V[_` \x82\x84\x03\x12\x15ap\xA6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\nW__\xFD[_` \x82\x84\x03\x12\x15ap\xC5W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\xDBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ap\xEBW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\x05Waq\x05ai\xE4V[aq\x18`\x1F\x82\x01`\x1F\x19\x16` \x01ai\xF8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aq,W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aqYW__\xFD[\x81Qa\x03\n\x81an\x9CV[`\x01\x81[`\x01\x84\x11\x15aq\x9FW\x80\x85\x04\x81\x11\x15aq\x83Waq\x83ao\xCEV[`\x01\x84\x16\x15aq\x91W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aqhV[\x93P\x93\x91PPV[_\x82aq\xB5WP`\x01a\x03\x1BV[\x81aq\xC1WP_a\x03\x1BV[\x81`\x01\x81\x14aq\xD7W`\x02\x81\x14aq\xE1Waq\xFDV[`\x01\x91PPa\x03\x1BV[`\xFF\x84\x11\x15aq\xF2Waq\xF2ao\xCEV[PP`\x01\x82\x1Ba\x03\x1BV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ar WP\x81\x81\na\x03\x1BV[ar,_\x19\x84\x84aqdV[\x80_\x19\x04\x82\x11\x15ar?War?ao\xCEV[\x02\x93\x92PPPV[_a\x03\n\x83\x83aq\xA7V[\x80\x82\x01\x80\x82\x11\x15a\x03\x1BWa\x03\x1Bao\xCEV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aCPWaCPao\xCEV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x1BWa\x03\x1Bao\xCEV[_\x82ar\xC9WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_ar\xE0``\x83\x01\x86ahKV[\x82\x81\x03` \x84\x01Rar\xF2\x81\x86ahKV[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 }\xC0\x1E\xDC9\xA8\xE8\xD8\xFF\x7Fa\x0B4/\xEA\x96\x07\x11F\x12\xAB\x9B\xB0\"\xEB\x0B\xE4\x13o6\xDF\xC0dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610111575f3560e01c8063739118a41161009e578063c2bdeda11161006e578063c2bdeda114610277578063d28b0a15146102a5578063e335adb7146102b8578063eed07428146102cb578063f68a7131146102de575f5ffd5b8063739118a41461021e57806378f212d11461023e5780638f6c7a3c14610251578063a6a100a014610264575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c457806357b91ca6146101e55780635a6f5710146101f85780635c39f4671461020b575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b610128610123366004616830565b6102f1565b6040516101359190616879565b60405180910390f35b61015161014c366004616830565b610311565b6040516001600160a01b039091168152602001610135565b61017c610177366004616964565b610321565b60408051928352602083019190915201610135565b61012861019f3660046169b2565b61033c565b6101b76101b2366004616a4c565b610351565b6040516101359190616bcd565b6101d76101d2366004616830565b61035d565b604051908152602001610135565b6101d76101f3366004616830565b610367565b6101d7610206366004616830565b610371565b6101b7610219366004616830565b61037b565b61023161022c366004616cec565b610394565b6040516101359190616dd6565b61015161024c366004616830565b6103af565b6101b761025f3660046169b2565b6103b9565b610231610272366004616a4c565b6103c6565b61028a610285366004616eaa565b6103d2565b60408051938452602084019290925290820152606001610135565b61028a6102b3366004616eaa565b6103f4565b6101516102c6366004616830565b610404565b6101286102d9366004616a4c565b61040e565b6101d76102ec366004616f0e565b61041a565b60605f6102fd83610434565b905061030a835f836104c6565b9392505050565b5f61031b826104e0565b92915050565b5f5f61032f86868686610591565b9150915094509492505050565b60606103498484846104c6565b949350505050565b606061030a83836105cd565b5f61031b8261068b565b5f61031b826106dc565b5f61031b82610434565b60605f61038783610434565b905061030a835f8361072d565b60605f6103a184846107fb565b905061034984845f84610871565b5f61031b82610941565b606061034984848461072d565b606061030a838361097d565b5f5f5f6103e28888888888610a33565b9250925092505b955095509592505050565b5f5f5f6103e28888888888610b3d565b5f61031b82610bb6565b606061030a8383610c07565b5f6104288686868686610cbd565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161045390616f65565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161048791815260200190565b602060405180830381865afa1580156104a2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061031b9190616f88565b60605f6104d4858585610cf0565b905061042b8582610d91565b5f816001600160a01b03166321f8a72160405160200161051e906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161055291815260200190565b602060405180830381865afa15801561056d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061031b9190616f9f565b5f5f5f61059e8686610e47565b90505f6105ab8883610eee565b90505f5f6105ba8a8489612166565b50919c909b509950505050505050505050565b60605f825167ffffffffffffffff8111156105ea576105ea6169e4565b60405190808252806020026020018201604052801561062357816020015b610610616277565b8152602001906001900390816106085790505b5090505f5b8351811015610683575f84828151811061064457610644616fba565b602002602001015190505f61065987836123bb565b90508084848151811061066e5761066e616fba565b60209081029190910101525050600101610628565b509392505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f56040516020016104539060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f61073b858585610cf0565b90505f815167ffffffffffffffff811115610758576107586169e4565b60405190808252806020026020018201604052801561079157816020015b61077e616277565b8152602001906001900390816107765790505b5090505f5b82518110156107f1575f8382815181106107b2576107b2616fba565b602002602001015190505f6107c789836123bb565b9050808484815181106107dc576107dc616fba565b60209081029190910101525050600101610796565b5095945050505050565b5f826001600160a01b031663f3903b9f6108148461285b565b6040518263ffffffff1660e01b815260040161083291815260200190565b602060405180830381865afa15801561084d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030a9190616f88565b60605f610880868686866128df565b90505f815167ffffffffffffffff81111561089d5761089d6169e4565b6040519080825280602002602001820160405280156108d657816020015b6108c36162f6565b8152602001906001900390816108bb5790505b5090505f5b8251811015610936575f8382815181106108f7576108f7616fba565b602002602001015190505f61090c8a83612967565b90508084848151811061092157610921616fba565b602090810291909101015250506001016108db565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161051e90602080825260089082015267545245415355525960c01b604082015260600190565b60605f825167ffffffffffffffff81111561099a5761099a6169e4565b6040519080825280602002602001820160405280156109d357816020015b6109c06162f6565b8152602001906001900390816109b85790505b5090505f5b8351811015610683575f8482815181106109f4576109f4616fba565b602002602001015190505f610a098783612967565b905080848481518110610a1e57610a1e616fba565b602090810291909101015250506001016109d8565b5f5f5f5f610a418888610e47565b90505f610a4e8a83610eee565b90505f808060ff8916610a8457610a668d8b86612f8c565b929550919350610a7d91508590505f858d826130a6565b9050610ab4565b5f1960ff8a1601610ab457610a9a8d8b8661328b565b929550919350610ab19150859050845f808e6130a6565b90505b805f03610ace575f5f5f97509750975050505050506103e9565b5f610ad88561338d565b90505f828211610af157610aec8284616fe2565b610afb565b610afb8383616fe2565b90505f610b088284613432565b90505f848411610b2057610b1b82616ff5565b610b22565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f610b4b8888610e47565b90505f610b588a83610eee565b90505f808060ff8916610b8857610b718d8b865f61346d565b929550919350610a7d91508590508b5f80876130a6565b5f1960ff8a1601610ab457610b9f8d8b865f613597565b929550919350610ab191508590505f8c86826130a6565b5f816001600160a01b03166321f8a72160405160200161051e906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60605f825167ffffffffffffffff811115610c2457610c246169e4565b604051908082528060200260200182016040528015610c5d57816020015b610c4a616349565b815260200190600190039081610c425790505b5090505f5b8351811015610683575f848281518110610c7e57610c7e616fba565b602002602001015190505f610c9387836136a6565b905080848481518110610ca857610ca8616fba565b60209081029190910101525050600101610c62565b5f5f610cc98686610e47565b90505f610cd68883610eee565b9050610ce48186865f613966565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001610d1090616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015610d6a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610349919081019061700f565b60605f825167ffffffffffffffff811115610dae57610dae6169e4565b604051908082528060200260200182016040528015610de757816020015b610dd4616349565b815260200190600190039081610dcc5790505b5090505f5b8351811015610683575f848281518110610e0857610e08616fba565b602002602001015190505f610e1d87836136a6565b905080848481518110610e3257610e32616fba565b60209081029190910101525050600101610dec565b5f816001600160a01b0316836001600160a01b031610610e68578183610e6b565b82825b6040519194509250610e98906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610ef661636e565b82610eff61636e565b816001600160a01b03166391d4403c604051602001610f1d90616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610f71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f959190617096565b610fa257915061031b9050565b816001600160a01b03166321f8a72185604051602001610fe2906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611012929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161104691815260200190565b602060405180830381865afa158015611061573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110859190616f9f565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611103929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113791815260200190565b602060405180830381865afa158015611152573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111769190616f88565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016111cc906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016111fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161123091815260200190565b602060405180830381865afa15801561124b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061126f9190616f88565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016112d09060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611300929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161133491815260200190565b602060405180830381865afa15801561134f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113739190616f88565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016113de9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b6040516020818303038152906040528051906020012060405160200161140e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161144291815260200190565b602060405180830381865afa15801561145d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114819190616f88565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016114e29060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611512929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161154691815260200190565b602060405180830381865afa158015611561573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115859190616f88565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001611602929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163691815260200190565b602060405180830381865afa158015611651573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116759190616f88565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016116ea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171e91815260200190565b602060405180830381865afa158015611739573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061175d9190616f9f565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161180791815260200190565b602060405180830381865afa158015611822573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118469190616f88565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161189d90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016118cd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161190191815260200190565b602060405180830381865afa15801561191c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119409190616f88565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016119a29060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016119d2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a0691815260200190565b602060405180830381865afa158015611a21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a459190616f88565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001611ab19060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b60405160208183030381529060405280519060200120604051602001611ae1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b1591815260200190565b602060405180830381865afa158015611b30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b549190616f88565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001611bb69060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611be6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611c1a91815260200190565b602060405180830381865afa158015611c35573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c599190616f88565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001611cb290602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ce2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611d1691815260200190565b602060405180830381865afa158015611d31573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d559190616f88565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001611da390602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611dd3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611e0791815260200190565b602060405180830381865afa158015611e22573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e469190616f9f565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611eb4906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611ee4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611f1891815260200190565b602060405180830381865afa158015611f33573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f579190616f9f565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611fba906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161201e91815260200190565b602060405180830381865afa158015612039573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061205d9190616f88565b60608201526040516001600160a01b0383169063bd02d0f59086906120b6906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016120e6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161211a91815260200190565b602060405180830381865afa158015612135573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121599190616f88565b6080820152949350505050565b5f5f5f5f6121726163a2565b61217b88613ac0565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e99190616f88565b816020018181525050612203875f5f846101400151613b11565b5060808401525060408201526101408101516122259088906001905f90613b11565b5060a084015250606082015260208101515f0361224e575f5f5f5f9450945094509450506123b2565b6122618682604001518360200151613bf6565b6101008201526060810151602082015161227c918891613bf6565b816101200181815250506122bc6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151613cb5565b6122f4604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151613cb5565b61232c60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151613cb5565b61235f6040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151613cb5565b6123926040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151613cb5565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b6123c3616277565b6123cb6163f1565b6123d58484610eee565b81526123e084613ce2565b60208201526123ee84613ac0565b610180820181905281516020830151612409925f9190613b11565b608085015260a084015260408301526060820152805160208201516101808301516124379291600191613b11565b61012085015261014084015260e0830152610100820152805161245990613d25565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa1580156124d8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124ff91908101906170b5565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561254c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125709190617149565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612650573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261267791908101906170b5565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156126c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126eb9190617149565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906127cb9061338d565b81526020016127da8686613f34565b6001600160a01b03168152602001612801835f015160600151660800000000000016151590565b151581526020016128128686614024565b81528251515160c00151602082015282516040909101906128369087905f80614126565b815260200161284886845f0151614221565b90526101a0909101819052905092915050565b5f604051602001612895906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a6128f98661285b565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612940573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261042b919081019061700f565b61296f6162f6565b61297761645f565b612981848461422c565b8082525180515161299a9160015b602002015151610e47565b604082018190526129ac908590610eee565b6020820181905281516129c091869161423e565b50505050606082015260208101516129d79061338d565b61030082015260208101516129ed908590614221565b610320820152805180515160209081015160e084015280830151515101519051612a2391905f5b6020020151604001519061428e565b60c0820152602081015160600151612a3b905f6142cf565b60a082015260e081015160c0820151612a5491906142fd565b610100820181905260a0820151612a6b919061431e565b61012082015260e081015160c0820151610100830151612a8c92919061433a565b61014082015260208101518151610320830151612aad92879290915f614357565b61016082015261014081015161018082015260e081015160c0820151610120830151612ada92919061433a565b6101a0820152602081015160600151612af49060016142cf565b6101c082015280518051602090810151810151610200840152808301515181015101519051612b2591906001612a14565b6101e08201819052610200820151612b3c916142fd565b61022082018190526101c0820151612b54919061431e565b6102408201819052610300820151612b6c919061428e565b6102608201526102008101516101e0820151610220830151612b8f92919061433a565b61028082015260208101518151610320830151612bb192879290916001614357565b6102a08201526102808101516102c08201526102008101516101e0820151610260830151612be092919061433a565b6102e08201528051612bf1906145c0565b60808201528051516020015160e00151600214612ce957612c1b81610300015182608001516142fd565b6103408201819052610240820151612c329161428e565b61036082018190526080820151610300830151116103a083018190526102008301516101e0840151612c6393614602565b61038082018190526103c082015260e081015160a0820151612cca918691612c8b919061431e565b612c9d8460c001518560a0015161431e565b612cb1856102000151866101c0015161431e565b612cc5866101e00151876101c0015161431e565b61461c565b6103e08201819052610300820151612ce291906146dc565b6104008201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612d5d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d8491908101906170b5565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612dd957612dd9616fba565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612e2857612e28616fba565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612e88573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612eaf91908101906170b5565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103c001518152602001826103e00151815260200182610400015181525091505092915050565b5f5f5f5f612f98616542565b612fa188613ac0565b6101c08201819052612fb89087905f908190613b11565b5060408401525081526101c0810151612fd79087906001905f90613b11565b50606084015250602082015280511580612ff357506020810151155b1561300a575f5f5f5f9450945094509450506123b2565b606086015160381c61ffff166101408201819052613039908890613031906127109061472b565b612710613bf6565b61018082018190528151602083015161305c9261305790839061472b565b613bf6565b608082018190526020820151613072919061472b565b60e0820181905260408201516060830151610140840151613094908b90614780565b94509450945094505093509350935093565b5f5f5f5f861180156130b6575083155b156130c55750839050846130fa565b5f871180156130d2575084155b156130e15750859050826130fa565b604051636331fab160e01b815260040160405180910390fd5b811580613105575080155b15613123576040516301a2868b60e31b815260040160405180910390fd5b61314f6040518060400160405280600a815260200169616d6f756e744261736560b01b81525083613cb5565b61317b6040518060400160405280600a815260200169616d6f756e744d656d6560b01b81525082613cb5565b5f61318a89606001515f6142cf565b90505f61319c8a6060015160016142cf565b90505f6131ba85676765c793fa10079d601b1b61305786600a617247565b90505f6131d885676765c793fa10079d601b1b61305786600a617247565b905061320e6040518060400160405280601281526020017118985cd9505b5bdd5b9d10591a9d5cdd195960721b81525083613cb5565b613242604051806040016040528060128152602001711b595b59505b5bdd5b9d10591a9d5cdd195960721b81525082613cb5565b604080518082019091526005815264707269636560d81b60208201526132719061326c8484613432565b613cb5565b61327b8282613432565b9c9b505050505050505050505050565b5f5f5f5f613297616542565b6132a088613ac0565b6101c082018190526132b79087905f908190613b11565b5060408401525081526101c08101516132d69087906001905f90613b11565b506060840152506020820152805115806132f257506020810151155b15613309575f5f5f5f9450945094509450506123b2565b8051602082015161331f9190613057818b61472b565b61010082018190528151613333919061472b565b610120820152606086015160381c61ffff166101408201819052610120820151613366916127109061305790829061472b565b6101a082018190526040820151606083015161014084015161012085015161309491614780565b5f5f61339b835f5f5f613b11565b50505090505f6133ae8460015f5f613b11565b5050509050815f14806133bf575080155b156133cd57505f9392505050565b5f6133dc85606001515f6142cf565b90505f6133ee866060015160016142cf565b90505f61340c85676765c793fa10079d601b1b61305786600a617247565b90505f61342a85676765c793fa10079d601b1b61305786600a617247565b9050610ce482825b5f8115676765c793fa10079d601b1b60028404190484111715613453575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f613479616542565b61348289613ac0565b6101c082018190526134999088905f908190613b11565b5060408401525081526101c08101516134b89088906001905f90613b11565b506060840152506020820152805115806134d457506020810151155b156134eb575f5f5f5f94509450945094505061358c565b85156135065787815f018181516135029190616fe2565b9052505b606087015160381c61ffff16610140820181905261352d908990613031906127109061472b565b61016082018190528151602083015161354b926130579083906147a4565b6080820181905260208201516135609161472b565b60c0820181905260408201516060830151610140840151613582908c90614780565b9450945094509450505b945094509450949050565b5f5f5f5f6135a3616542565b6135ac89613ac0565b6101c082018190526135c39088905f908190613b11565b5060408401525081526101c08101516135e29088906001905f90613b11565b506060840152506020820152805115806135fe57506020810151155b15613615575f5f5f5f94509450945094505061358c565b851561363157878160200181815161362d9190616fe2565b9052505b805160208201516136479190613057818c6147a4565b6080820181905281516136599161472b565b60a0820152606087015160381c61ffff16610140820181905260a082015161368891613031906127109061472b565b6040820151606083015161014084015160a085015161358291614780565b6136ae616349565b6136b66165a9565b6136c08484610eee565b808252602001516001600160a01b03166136f557604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa158015613766573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261378d91908101906170b5565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa1580156137da573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137fe9190617149565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa15801561387e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526138a591908101906170b5565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156138f5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139199190617149565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff16815260200161395486845f0151614221565b90526020909101819052905092915050565b5f61396f6163a2565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156139af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139d39190616f88565b60208201526139e4865f8080613b11565b50505060c08201526139f98660015f80613b11565b50505060e08201528215613a3457848160c001818151613a199190616fe2565b90525060e081018051859190613a30908390616fe2565b9052505b80602001515f03613a6457613a5d6103e8613a57613a5288886147f8565b61485e565b9061472b565b8152613ab6565b60c08101511580613a77575060e0810151155b15613a85575f915050610349565b613ab3613a9b8683602001518460c00151613bf6565b613aae8684602001518560e00151613bf6565b61493e565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff1660028110613b2e57613b2e616fba565b602002015190505f613b408a8a614953565b9050805f03613b5c575f5f5f5f9550955095509550505061358c565b5f613b6b838c60800151614a41565b90505f613b78828a61428e565b90505f8915613b9d57818410613b9757613b92848361472b565b613b9f565b5f613b9f565b5f5b90505f613bac858d61428e565b90505f8c15613bd157848210613bcb57613bc6828661472b565b613bd3565b5f613bd3565b5f5b9050613bdf8587617252565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613c2a57838281613c2057613c20617265565b049250505061030a565b808411613c4a5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b613cde604051806040016040528060068152602001652573202d257360d01b8152508383614a71565b5050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f613d5a6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613d64845f614abd565b602083015281526060840151613d7a905f6142cf565b606082018190528151613d9f91676765c793fa10079d601b1b9061305790600a617247565b604082015260208101515f03613dba575f6080820152613e5a565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613e30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e549190616f88565b60808201525b613e65846001614abd565b602083018190529082525f03613e80575f60a0820152613f20565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613ef6573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f1a9190616f88565b60a08201525b80608001518160a001519250925050915091565b5f5f613f408484614b03565b9050806001600160a01b03166321f8a72184604051602001613f81906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613fb1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613fe591815260200190565b602060405180830381865afa158015614000573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103499190616f9f565b5f5f6140308484614b03565b9050806001600160a01b031663bd02d0f5846040516020016140839060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016140b3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140e791815260200190565b602060405180830381865afa158015614102573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103499190616f88565b5f5f614133856001614953565b90508215614148576141458482616fe2565b90505b5f61415287614bbe565b90505f61415f838361428e565b875160200151606001519091505f90821061418d57875160200151606001516141889083616fe2565b61418f565b5f5b90506141be6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525085613cb5565b6141ee6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525084613cb5565b610ce4604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525083613cb5565b5f61030a8383614c02565b6142346165c9565b61030a8383614c1b565b5f5f5f5f5f61424d8888614221565b905061425b8787835f615e39565b90935091508161426c575f19614276565b6142768383613432565b9450614281886106dc565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176142ae575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff8516016142ef575060ff60601b19905060605b90198416901c905092915050565b5f8183116143145761430f8383616fe2565b61030a565b61030a8284616fe2565b5f61030a83676765c793fa10079d601b1b61305785600a617247565b5f8284116143505761434b82616ff5565b610349565b5092915050565b5f6143916040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61439d8686865f615e39565b60208084019190915290825260408051808201909152600f81526e1d985c9ccb98dbdb1b185d195c985b608a1b9181019190915281516143dd9190613cb5565b61440c604051806040016040528060098152602001681d985c9ccb9919589d60ba1b8152508260200151613cb5565b80602001515f0361443e57845160ff84166002811061442d5761442d616fba565b60200201516020015191505061042b565b805115806144695750845160ff84166002811061445d5761445d616fba565b6020020151602001515f145b15614477575f91505061042b565b61448087615fa5565b6040820181905260208201516144959161428e565b608082019081526040805180820190915260118152701d985c9ccb98591a9d5cdd19591119589d607a1b602082015290516144d09190613cb5565b6080810151815110156144e6575f91505061042b565b608081015181516144f79190616fe2565b81606001818152505061450e8660600151846142cf565b60a08201819052606082015161453a9161452990600a617247565b676765c793fa10079d601b1b613bf6565b60c08201525f1960ff84160161455f5760c08101516145599085613432565b60c08201525b845160ff84166002811061457557614575616fba565b6020020151602001518160c0015111156145b257845160ff84166002811061459f5761459f616fba565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f19016145e0575051602001516060015190565b81516020015160e001516145fb575051602001516080015190565b505f919050565b5f841515838511146106835761461782616ff5565b61042b565b5f5f614627876106dc565b90505f614634828761428e565b90505f614641838661428e565b90505f61464e8984617279565b90505f61465b8389617279565b90505f61466783615feb565b90505f61467383615feb565b90505f8413801561468357505f83125b8061469757505f8412801561469757505f83135b156146ab575f97505050505050505061042b565b805f036146c1575f97505050505050505061042b565b6146cb8282613432565b9d9c50505050505050505050505050565b5f815f036146eb57505f61031b565b5f828411614702576146fd8484616fe2565b61470c565b61470c8385616fe2565b90505f6147198285613432565b90508385116103495761461781616ff5565b5f826147378382616fe2565b915081111561031b5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064016136ec565b5f81156113881983900484111517614796575f5ffd5b506127109102611388010490565b5f826147b08382617252565b915081101561031b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016136ec565b5f81158061481b5750828261480d8183617298565b925061481990836172af565b145b61031b5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016136ec565b5f815f0361486d57505f919050565b5f600161487984616000565b901c6001901b9050600181848161489257614892617265565b048201901c905060018184816148aa576148aa617265565b048201901c905060018184816148c2576148c2617265565b048201901c905060018184816148da576148da617265565b048201901c905060018184816148f2576148f2617265565b048201901c9050600181848161490a5761490a617265565b048201901c9050600181848161492257614922617265565b048201901c905061030a8182858161493c5761493c617265565b045b5f81831061494c578161030a565b5090919050565b5f5f835f01518360ff166002811061496d5761496d616fba565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156149c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149ea9190616f88565b9050805f036149fd575f9250505061031b565b606082015160c0830151614a119082617252565b8210614a355760c0830151614a268284616fe2565b614a309190616fe2565b614a37565b5f5b9695505050505050565b5f8260a001515f03614a5457505f61031b565b5f614a5f8484616093565b60a0850151909150610349908261428e565b614ab8838383604051602401614a89939291906172ce565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b1790526160d6565b505050565b5f5f5f614aea855f01518560ff1660028110614adb57614adb616fba565b60200201518660800151614a41565b90505f614af78686614953565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001614b2690616f65565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015614b7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b9e9190617096565b61030a57604051637357d91f60e01b8152600481018490526024016136ec565b5f816001600160a01b031663bd02d0f56040516020016104539060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5610814846160e2565b614c236165c9565b82614c2c6165c9565b816001600160a01b03166391d4403c604051602001614c6c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614cc0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ce49190617096565b614cf157915061031b9050565b816001600160a01b031663bd02d0f585604051602001614d2b906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d5b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614d8f91815260200190565b602060405180830381865afa158015614daa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614dce9190616f88565b816020018181525050816001600160a01b03166321f8a72185604051602001614e16906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e46929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614e7a91815260200190565b602060405180830381865afa158015614e95573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614eb99190616f9f565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614f15906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f45929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614f7991815260200190565b602060405180830381865afa158015614f94573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614fb89190616f9f565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615033929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161506791815260200190565b602060405180830381865afa158015615082573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150a69190616f88565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016150fa9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161512a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161515e91815260200190565b602060405180830381865afa158015615179573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061519d9190616f88565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016151f7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615227929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161525b91815260200190565b602060405180830381865afa158015615276573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061529a9190616f88565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016152f3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615323929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161535791815260200190565b602060405180830381865afa158015615372573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153969190616f88565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161541c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161545091815260200190565b602060405180830381865afa15801561546b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061548f9190616f88565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016154e9906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615519929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161554d91815260200190565b602060405180830381865afa158015615568573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061558c9190616f88565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016155ff929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161563391815260200190565b602060405180830381865afa15801561564e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156729190616f88565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016156e6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161571a91815260200190565b602060405180830381865afa158015615735573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157599190616f9f565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161580091815260200190565b602060405180830381865afa15801561581b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061583f9190616f88565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016158949060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016158c4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016158f891815260200190565b602060405180830381865afa158015615913573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159379190616f88565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161599290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016159c2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016159f691815260200190565b602060405180830381865afa158015615a11573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a359190616f88565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001615a8f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001615abf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615af391815260200190565b602060405180830381865afa158015615b0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b329190616f88565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001615b949060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615bc4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615bf891815260200190565b602060405180830381865afa158015615c13573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c379190616f88565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615c9290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615cc2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615cf691815260200190565b602060405180830381865afa158015615d11573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d359190616f88565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615d84906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615db4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615de891815260200190565b602060405180830381865afa158015615e03573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e279190616f88565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615edc575f5f615e648a8a5f61617d565b915091505f615e805f8c606001516142cf90919063ffffffff16565b90505f615e9e84676765c793fa10079d601b1b61305785600a617247565b90505f615ebc84676765c793fa10079d601b1b61305786600a617247565b9050615ec88288617252565b9650615ed48187617252565b955050505050505b865160200151516001600160a01b03868116911614615f98575f5f615f038a8a600161617d565b915091505f615f2060018c606001516142cf90919063ffffffff16565b90505f615f3e84676765c793fa10079d601b1b61305785600a617247565b90505f615f5c84676765c793fa10079d601b1b61305786600a617247565b90505f615f69838d61428e565b90505f615f76838e61428e565b9050615f82828a617252565b9850615f8e8189617252565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f5604051602001610453906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615ffc57815f0361031b565b5090565b5f80608083901c1561601457608092831c92015b604083901c1561602657604092831c92015b602083901c1561603857602092831c92015b601083901c1561604a57601092831c92015b600883901c1561605c57600892831c92015b600483901c1561606e57600492831c92015b600283901c1561608057600292831c92015b600183901c1561031b5760010192915050565b5f4282036160a65750602082015161031b565b5f6160b5846040015184616224565b90506160ce84602001518261428e90919063ffffffff16565b91505061031b565b6160df81616258565b50565b80518051515f9182916160f691600161298f565b90508060405160200161612f90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161615f929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff166002811061619857616198616fba565b6020020151604001519050805f036161b2575f91506161fa565b5f6161dd875f01518660ff16600281106161ce576161ce616fba565b60200201518860800151616093565b905081156161f4576161ef828261428e565b6161f6565b5f5b9250505b845160ff85166002811061621057616210616fba565b602002015160200151925050935093915050565b5f806162308342616fe2565b61623a9085617298565b6301e133809004905061034981676765c793fa10079d601b1b617252565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b604051806101a0016040528061628b6165ef565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b60405180610120016040528061630a616676565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806060016040528061635c6166f1565b81526020015f81526020015f81525090565b6040518060a00160405280616381616747565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c0016040528061640561636e565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f815260200161645a616277565b905290565b6040518061042001604052806164736165c9565b815260200161648061636e565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806165bc61636e565b815260200161645a616349565b60405180606001604052806165dc6167ae565b81525f6020820181905260409091015290565b60405180604001604052806002905b6166606040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165fe5790505090565b60405180604001604052806002905b6166db6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166855790505090565b60405180604001604052806002905b61673160405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816167005790505090565b60405180604001604052806002905b6167986040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167565790505090565b60405180604001604052806002905b6168066040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167bd5790505090565b6001600160a01b03811681146160df575f5ffd5b5f60208284031215616840575f5ffd5b813561030a8161681c565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857868503603f1901845281518051606080885260a08801919088015f5b600281101561692b57898403605f19018252825180516001600160a01b031685526020808201516080918701829052906168ff9087018261684b565b6040838101519088015260609283015192909601919091525060209283019291909101906001016168c3565b5050506020828101518882015260409283015192909701919091529493840193919091019060010161689f565b50929695505050505050565b5f5f5f5f60808587031215616977575f5ffd5b84356169828161681c565b935060208501356169928161681c565b925060408501356169a28161681c565b9396929550929360600135925050565b5f5f5f606084860312156169c4575f5ffd5b83356169cf8161681c565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616a2157616a216169e4565b604052919050565b5f67ffffffffffffffff821115616a4257616a426169e4565b5060051b60200190565b5f5f60408385031215616a5d575f5ffd5b8235616a688161681c565b9150602083013567ffffffffffffffff811115616a83575f5ffd5b8301601f81018513616a93575f5ffd5b8035616aa6616aa182616a29565b6169f8565b8082825260208201915060208360051b850101925087831115616ac7575f5ffd5b6020840193505b82841015616ae9578335825260209384019390910190616ace565b809450505050509250929050565b5f8260408101835f5b6002811015616bc2578383038752815180516001600160a01b0316845260208101516101806020860152616b3861018086018261684b565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616b00565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857603f19878603018452815180516101a08752616c1b6101a0880182616af7565b90506020820151616c3760208901826001600160a01b03169052565b506040820151616c5260408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616c9560e08901826001600160a01b03169052565b50610100820151616cab61010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616bf3565b5f5f60408385031215616cfd575f5ffd5b8235616d088161681c565b91506020830135616d188161681c565b809150509250929050565b5f8260408101835f5b6002811015616bc2578383038752815180516001600160a01b0316845260208101516101406020860152616d6461014086018261684b565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616d2c565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561695857603f19878603018452815180516101208752616e24610120880182616d23565b9050602082015160208801526040820151616e4a60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616dfc565b60ff811681146160df575f5ffd5b5f5f5f5f5f60a08688031215616ebe575f5ffd5b8535616ec98161681c565b94506020860135616ed98161681c565b93506040860135616ee98161681c565b9250606086013591506080860135616f0081616e9c565b809150509295509295909350565b5f5f5f5f5f60a08688031215616f22575f5ffd5b8535616f2d8161681c565b94506020860135616f3d8161681c565b93506040860135616f4d8161681c565b94979396509394606081013594506080013592915050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f60208284031215616f98575f5ffd5b5051919050565b5f60208284031215616faf575f5ffd5b815161030a8161681c565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561031b5761031b616fce565b5f600160ff1b820161700957617009616fce565b505f0390565b5f6020828403121561701f575f5ffd5b815167ffffffffffffffff811115617035575f5ffd5b8201601f81018413617045575f5ffd5b8051617053616aa182616a29565b8082825260208201915060208360051b850101925086831115617074575f5ffd5b6020840193505b82841015614a3757835182526020938401939091019061707b565b5f602082840312156170a6575f5ffd5b8151801515811461030a575f5ffd5b5f602082840312156170c5575f5ffd5b815167ffffffffffffffff8111156170db575f5ffd5b8201601f810184136170eb575f5ffd5b805167ffffffffffffffff811115617105576171056169e4565b617118601f8201601f19166020016169f8565b81815285602083850101111561712c575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215617159575f5ffd5b815161030a81616e9c565b6001815b600184111561719f5780850481111561718357617183616fce565b600184161561719157908102905b60019390931c928002617168565b935093915050565b5f826171b55750600161031b565b816171c157505f61031b565b81600181146171d757600281146171e1576171fd565b600191505061031b565b60ff8411156171f2576171f2616fce565b50506001821b61031b565b5060208310610133831016604e8410600b8410161715617220575081810a61031b565b61722c5f198484617164565b805f190482111561723f5761723f616fce565b029392505050565b5f61030a83836171a7565b8082018082111561031b5761031b616fce565b634e487b7160e01b5f52601260045260245ffd5b8181035f83128015838313168383128216171561435057614350616fce565b808202811582820484141761031b5761031b616fce565b5f826172c957634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f6172e0606083018661684b565b82810360208401526172f2818661684b565b91505082604083015294935050505056fea26469706673582212207dc01edc39a8e8d8ff7f610b342fea9607114612ab9bb022eb0be4136f36dfc064736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02wW\x80c\xD2\x8B\n\x15\x14a\x02\xA5W\x80c\xE35\xAD\xB7\x14a\x02\xB8W\x80c\xEE\xD0t(\x14a\x02\xCBW\x80c\xF6\x8Aq1\x14a\x02\xDEW__\xFD[\x80cs\x91\x18\xA4\x14a\x02\x1EW\x80cx\xF2\x12\xD1\x14a\x02>W\x80c\x8Flz<\x14a\x02QW\x80c\xA6\xA1\0\xA0\x14a\x02dW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cW\xB9\x1C\xA6\x14a\x01\xE5W\x80cZoW\x10\x14a\x01\xF8W\x80c\\9\xF4g\x14a\x02\x0BW__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04ah0V[a\x02\xF1V[`@Qa\x015\x91\x90ahyV[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04ah0V[a\x03\x11V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04aidV[a\x03!V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ai\xB2V[a\x03<V[a\x01\xB7a\x01\xB26`\x04ajLV[a\x03QV[`@Qa\x015\x91\x90ak\xCDV[a\x01\xD7a\x01\xD26`\x04ah0V[a\x03]V[`@Q\x90\x81R` \x01a\x015V[a\x01\xD7a\x01\xF36`\x04ah0V[a\x03gV[a\x01\xD7a\x02\x066`\x04ah0V[a\x03qV[a\x01\xB7a\x02\x196`\x04ah0V[a\x03{V[a\x021a\x02,6`\x04al\xECV[a\x03\x94V[`@Qa\x015\x91\x90am\xD6V[a\x01Qa\x02L6`\x04ah0V[a\x03\xAFV[a\x01\xB7a\x02_6`\x04ai\xB2V[a\x03\xB9V[a\x021a\x02r6`\x04ajLV[a\x03\xC6V[a\x02\x8Aa\x02\x856`\x04an\xAAV[a\x03\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x8Aa\x02\xB36`\x04an\xAAV[a\x03\xF4V[a\x01Qa\x02\xC66`\x04ah0V[a\x04\x04V[a\x01(a\x02\xD96`\x04ajLV[a\x04\x0EV[a\x01\xD7a\x02\xEC6`\x04ao\x0EV[a\x04\x1AV[``_a\x02\xFD\x83a\x044V[\x90Pa\x03\n\x83_\x83a\x04\xC6V[\x93\x92PPPV[_a\x03\x1B\x82a\x04\xE0V[\x92\x91PPV[__a\x03/\x86\x86\x86\x86a\x05\x91V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03I\x84\x84\x84a\x04\xC6V[\x94\x93PPPPV[``a\x03\n\x83\x83a\x05\xCDV[_a\x03\x1B\x82a\x06\x8BV[_a\x03\x1B\x82a\x06\xDCV[_a\x03\x1B\x82a\x044V[``_a\x03\x87\x83a\x044V[\x90Pa\x03\n\x83_\x83a\x07-V[``_a\x03\xA1\x84\x84a\x07\xFBV[\x90Pa\x03I\x84\x84_\x84a\x08qV[_a\x03\x1B\x82a\tAV[``a\x03I\x84\x84\x84a\x07-V[``a\x03\n\x83\x83a\t}V[___a\x03\xE2\x88\x88\x88\x88\x88a\n3V[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x03\xE2\x88\x88\x88\x88\x88a\x0B=V[_a\x03\x1B\x82a\x0B\xB6V[``a\x03\n\x83\x83a\x0C\x07V[_a\x04(\x86\x86\x86\x86\x86a\x0C\xBDV[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04S\x90aoeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x87\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1B\x91\x90ao\x88V[``_a\x04\xD4\x85\x85\x85a\x0C\xF0V[\x90Pa\x04+\x85\x82a\r\x91V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05R\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1B\x91\x90ao\x9FV[___a\x05\x9E\x86\x86a\x0EGV[\x90P_a\x05\xAB\x88\x83a\x0E\xEEV[\x90P__a\x05\xBA\x8A\x84\x89a!fV[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xEAWa\x05\xEAai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06#W\x81` \x01[a\x06\x10abwV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\x08W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x06DWa\x06Dao\xBAV[` \x02` \x01\x01Q\x90P_a\x06Y\x87\x83a#\xBBV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06nWa\x06nao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06(V[P\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x07;\x85\x85\x85a\x0C\xF0V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07XWa\x07Xai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\x91W\x81` \x01[a\x07~abwV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07vW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x07\xF1W_\x83\x82\x81Q\x81\x10a\x07\xB2Wa\x07\xB2ao\xBAV[` \x02` \x01\x01Q\x90P_a\x07\xC7\x89\x83a#\xBBV[\x90P\x80\x84\x84\x81Q\x81\x10a\x07\xDCWa\x07\xDCao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x07\x96V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x08\x14\x84a([V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x082\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\n\x91\x90ao\x88V[``_a\x08\x80\x86\x86\x86\x86a(\xDFV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x9DWa\x08\x9Dai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD6W\x81` \x01[a\x08\xC3ab\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xBBW\x90P[P\x90P_[\x82Q\x81\x10\x15a\t6W_\x83\x82\x81Q\x81\x10a\x08\xF7Wa\x08\xF7ao\xBAV[` \x02` \x01\x01Q\x90P_a\t\x0C\x8A\x83a)gV[\x90P\x80\x84\x84\x81Q\x81\x10a\t!Wa\t!ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x08\xDBV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x9AWa\t\x9Aai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xD3W\x81` \x01[a\t\xC0ab\xF6V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\xB8W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\t\xF4Wa\t\xF4ao\xBAV[` \x02` \x01\x01Q\x90P_a\n\t\x87\x83a)gV[\x90P\x80\x84\x84\x81Q\x81\x10a\n\x1EWa\n\x1Eao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\t\xD8V[____a\nA\x88\x88a\x0EGV[\x90P_a\nN\x8A\x83a\x0E\xEEV[\x90P_\x80\x80`\xFF\x89\x16a\n\x84Wa\nf\x8D\x8B\x86a/\x8CV[\x92\x95P\x91\x93Pa\n}\x91P\x85\x90P_\x85\x8D\x82a0\xA6V[\x90Pa\n\xB4V[_\x19`\xFF\x8A\x16\x01a\n\xB4Wa\n\x9A\x8D\x8B\x86a2\x8BV[\x92\x95P\x91\x93Pa\n\xB1\x91P\x85\x90P\x84_\x80\x8Ea0\xA6V[\x90P[\x80_\x03a\n\xCEW___\x97P\x97P\x97PPPPPPa\x03\xE9V[_a\n\xD8\x85a3\x8DV[\x90P_\x82\x82\x11a\n\xF1Wa\n\xEC\x82\x84ao\xE2V[a\n\xFBV[a\n\xFB\x83\x83ao\xE2V[\x90P_a\x0B\x08\x82\x84a42V[\x90P_\x84\x84\x11a\x0B Wa\x0B\x1B\x82ao\xF5V[a\x0B\"V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x0BK\x88\x88a\x0EGV[\x90P_a\x0BX\x8A\x83a\x0E\xEEV[\x90P_\x80\x80`\xFF\x89\x16a\x0B\x88Wa\x0Bq\x8D\x8B\x86_a4mV[\x92\x95P\x91\x93Pa\n}\x91P\x85\x90P\x8B_\x80\x87a0\xA6V[_\x19`\xFF\x8A\x16\x01a\n\xB4Wa\x0B\x9F\x8D\x8B\x86_a5\x97V[\x92\x95P\x91\x93Pa\n\xB1\x91P\x85\x90P_\x8C\x86\x82a0\xA6V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05\x1E\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C$Wa\x0C$ai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C]W\x81` \x01[a\x0CJacIV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0CBW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x0C~Wa\x0C~ao\xBAV[` \x02` \x01\x01Q\x90P_a\x0C\x93\x87\x83a6\xA6V[\x90P\x80\x84\x84\x81Q\x81\x10a\x0C\xA8Wa\x0C\xA8ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0CbV[__a\x0C\xC9\x86\x86a\x0EGV[\x90P_a\x0C\xD6\x88\x83a\x0E\xEEV[\x90Pa\x0C\xE4\x81\x86\x86_a9fV[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\r\x10\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rjW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03I\x91\x90\x81\x01\x90ap\x0FV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xAEWa\r\xAEai\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xE7W\x81` \x01[a\r\xD4acIV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x83W_\x84\x82\x81Q\x81\x10a\x0E\x08Wa\x0E\x08ao\xBAV[` \x02` \x01\x01Q\x90P_a\x0E\x1D\x87\x83a6\xA6V[\x90P\x80\x84\x84\x81Q\x81\x10a\x0E2Wa\x0E2ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\r\xECV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x0EhW\x81\x83a\x0EkV[\x82\x82[`@Q\x91\x94P\x92Pa\x0E\x98\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x0E\xF6acnV[\x82a\x0E\xFFacnV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x0F\x1D\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x95\x91\x90ap\x96V[a\x0F\xA2W\x91Pa\x03\x1B\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x0F\xE2\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x12\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x85\x91\x90ao\x9FV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x117\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11RW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11v\x91\x90ao\x88V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\xCC\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x120\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12o\x91\x90ao\x88V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xD0\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x134\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13s\x91\x90ao\x88V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xDE\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x0E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x81\x91\x90ao\x88V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xE2\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x12\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15aW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x85\x91\x90ao\x88V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\x02\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x166\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16u\x91\x90ao\x88V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x179W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17]\x91\x90ao\x9FV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18F\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x18\x9D\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x01\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19@\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xA2\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xD2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AE\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1A\xB1\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BT\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1B\xB6\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CY\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1C\xB2\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DU\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1D\xA3\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EF\x91\x90ao\x9FV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x1E\xB4\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xE4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\x18\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FW\x91\x90ao\x9FV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1F\xBA\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a 9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a ]\x91\x90ao\x88V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a \xB6\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Y\x91\x90ao\x88V[`\x80\x82\x01R\x94\x93PPPPV[____a!rac\xA2V[a!{\x88a:\xC0V[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE9\x91\x90ao\x88V[\x81` \x01\x81\x81RPPa\"\x03\x87__\x84a\x01@\x01Qa;\x11V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"%\x90\x88\x90`\x01\x90_\x90a;\x11V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"NW____\x94P\x94P\x94P\x94PPa#\xB2V[a\"a\x86\x82`@\x01Q\x83` \x01Qa;\xF6V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"|\x91\x88\x91a;\xF6V[\x81a\x01 \x01\x81\x81RPPa\"\xBC`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa<\xB5V[a\"\xF4`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa<\xB5V[a#,`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa<\xB5V[a#_`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa<\xB5V[a#\x92`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa<\xB5V[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[a#\xC3abwV[a#\xCBac\xF1V[a#\xD5\x84\x84a\x0E\xEEV[\x81Ra#\xE0\x84a<\xE2V[` \x82\x01Ra#\xEE\x84a:\xC0V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa$\t\x92_\x91\x90a;\x11V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa$7\x92\x91`\x01\x91a;\x11V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa$Y\x90a=%V[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xD8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xFF\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a%LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%p\x91\x90aqIV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&PW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&w\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xEB\x91\x90aqIV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a'\xCB\x90a3\x8DV[\x81R` \x01a'\xDA\x86\x86a?4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a(\x01\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a(\x12\x86\x86a@$V[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a(6\x90\x87\x90_\x80aA&V[\x81R` \x01a(H\x86\x84_\x01QaB!V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a(\x95\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a(\xF9\x86a([V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)@W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04+\x91\x90\x81\x01\x90ap\x0FV[a)oab\xF6V[a)wad_V[a)\x81\x84\x84aB,V[\x80\x82RQ\x80QQa)\x9A\x91`\x01[` \x02\x01QQa\x0EGV[`@\x82\x01\x81\x90Ra)\xAC\x90\x85\x90a\x0E\xEEV[` \x82\x01\x81\x90R\x81Qa)\xC0\x91\x86\x91aB>V[PPPP``\x82\x01R` \x81\x01Qa)\xD7\x90a3\x8DV[a\x03\0\x82\x01R` \x81\x01Qa)\xED\x90\x85\x90aB!V[a\x03 \x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa*#\x91\x90_[` \x02\x01Q`@\x01Q\x90aB\x8EV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa*;\x90_aB\xCFV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa*T\x91\x90aB\xFDV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa*k\x91\x90aC\x1EV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa*\x8C\x92\x91\x90aC:V[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03 \x83\x01Qa*\xAD\x92\x87\x92\x90\x91_aCWV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa*\xDA\x92\x91\x90aC:V[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa*\xF4\x90`\x01aB\xCFV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa+%\x91\x90`\x01a*\x14V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa+<\x91aB\xFDV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa+T\x91\x90aC\x1EV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa+l\x91\x90aB\x8EV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa+\x8F\x92\x91\x90aC:V[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03 \x83\x01Qa+\xB1\x92\x87\x92\x90\x91`\x01aCWV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa+\xE0\x92\x91\x90aC:V[a\x02\xE0\x82\x01R\x80Qa+\xF1\x90aE\xC0V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a,\xE9Wa,\x1B\x81a\x03\0\x01Q\x82`\x80\x01QaB\xFDV[a\x03@\x82\x01\x81\x90Ra\x02@\x82\x01Qa,2\x91aB\x8EV[a\x03`\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\xA0\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa,c\x93aF\x02V[a\x03\x80\x82\x01\x81\x90Ra\x03\xC0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa,\xCA\x91\x86\x91a,\x8B\x91\x90aC\x1EV[a,\x9D\x84`\xC0\x01Q\x85`\xA0\x01QaC\x1EV[a,\xB1\x85a\x02\0\x01Q\x86a\x01\xC0\x01QaC\x1EV[a,\xC5\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01QaC\x1EV[aF\x1CV[a\x03\xE0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa,\xE2\x91\x90aF\xDCV[a\x04\0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-]W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x84\x91\x90\x81\x01\x90ap\xB5V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a-\xD9Wa-\xD9ao\xBAV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a.(Wa.(ao\xBAV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x88W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.\xAF\x91\x90\x81\x01\x90ap\xB5V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81R` \x01\x82a\x04\0\x01Q\x81RP\x91PP\x92\x91PPV[____a/\x98aeBV[a/\xA1\x88a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra/\xB8\x90\x87\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa/\xD7\x90\x87\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a/\xF3WP` \x81\x01Q\x15[\x15a0\nW____\x94P\x94P\x94P\x94PPa#\xB2V[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra09\x90\x88\x90a01\x90a'\x10\x90aG+V[a'\x10a;\xF6V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\\\x92a0W\x90\x83\x90aG+V[a;\xF6V[`\x80\x82\x01\x81\x90R` \x82\x01Qa0r\x91\x90aG+V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0\x94\x90\x8B\x90aG\x80V[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a0\xB6WP\x83\x15[\x15a0\xC5WP\x83\x90P\x84a0\xFAV[_\x87\x11\x80\x15a0\xD2WP\x84\x15[\x15a0\xE1WP\x85\x90P\x82a0\xFAV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15\x80a1\x05WP\x80\x15[\x15a1#W`@Qc\x01\xA2\x86\x8B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a1O`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountBase`\xB0\x1B\x81RP\x83a<\xB5V[a1{`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iamountMeme`\xB0\x1B\x81RP\x82a<\xB5V[_a1\x8A\x89``\x01Q_aB\xCFV[\x90P_a1\x9C\x8A``\x01Q`\x01aB\xCFV[\x90P_a1\xBA\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a1\xD8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa2\x0E`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\x98\\\xD9P[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x83a<\xB5V[a2B`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x1BY[YP[[\xDD[\x9D\x10Y\x1A\x9D\\\xDD\x19Y`r\x1B\x81RP\x82a<\xB5V[`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdprice`\xD8\x1B` \x82\x01Ra2q\x90a2l\x84\x84a42V[a<\xB5V[a2{\x82\x82a42V[\x9C\x9BPPPPPPPPPPPPV[____a2\x97aeBV[a2\xA0\x88a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra2\xB7\x90\x87\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa2\xD6\x90\x87\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a2\xF2WP` \x81\x01Q\x15[\x15a3\tW____\x94P\x94P\x94P\x94PPa#\xB2V[\x80Q` \x82\x01Qa3\x1F\x91\x90a0W\x81\x8BaG+V[a\x01\0\x82\x01\x81\x90R\x81Qa33\x91\x90aG+V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa3f\x91a'\x10\x90a0W\x90\x82\x90aG+V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0\x94\x91aG\x80V[__a3\x9B\x83___a;\x11V[PPP\x90P_a3\xAE\x84`\x01__a;\x11V[PPP\x90P\x81_\x14\x80a3\xBFWP\x80\x15[\x15a3\xCDWP_\x93\x92PPPV[_a3\xDC\x85``\x01Q_aB\xCFV[\x90P_a3\xEE\x86``\x01Q`\x01aB\xCFV[\x90P_a4\x0C\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a4*\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa\x0C\xE4\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a4SW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a4yaeBV[a4\x82\x89a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra4\x99\x90\x88\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4\xB8\x90\x88\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a4\xD4WP` \x81\x01Q\x15[\x15a4\xEBW____\x94P\x94P\x94P\x94PPa5\x8CV[\x85\x15a5\x06W\x87\x81_\x01\x81\x81Qa5\x02\x91\x90ao\xE2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra5-\x90\x89\x90a01\x90a'\x10\x90aG+V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa5K\x92a0W\x90\x83\x90aG\xA4V[`\x80\x82\x01\x81\x90R` \x82\x01Qa5`\x91aG+V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa5\x82\x90\x8C\x90aG\x80V[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a5\xA3aeBV[a5\xAC\x89a:\xC0V[a\x01\xC0\x82\x01\x81\x90Ra5\xC3\x90\x88\x90_\x90\x81\x90a;\x11V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa5\xE2\x90\x88\x90`\x01\x90_\x90a;\x11V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a5\xFEWP` \x81\x01Q\x15[\x15a6\x15W____\x94P\x94P\x94P\x94PPa5\x8CV[\x85\x15a61W\x87\x81` \x01\x81\x81Qa6-\x91\x90ao\xE2V[\x90RP[\x80Q` \x82\x01Qa6G\x91\x90a0W\x81\x8CaG\xA4V[`\x80\x82\x01\x81\x90R\x81Qa6Y\x91aG+V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa6\x88\x91a01\x90a'\x10\x90aG+V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa5\x82\x91aG\x80V[a6\xAEacIV[a6\xB6ae\xA9V[a6\xC0\x84\x84a\x0E\xEEV[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a6\xF5W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7fW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\x8D\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xFE\x91\x90aqIV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a8~W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra8\xA5\x91\x90\x81\x01\x90ap\xB5V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x19\x91\x90aqIV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a9T\x86\x84_\x01QaB!V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a9oac\xA2V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xD3\x91\x90ao\x88V[` \x82\x01Ra9\xE4\x86_\x80\x80a;\x11V[PPP`\xC0\x82\x01Ra9\xF9\x86`\x01_\x80a;\x11V[PPP`\xE0\x82\x01R\x82\x15a:4W\x84\x81`\xC0\x01\x81\x81Qa:\x19\x91\x90ao\xE2V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a:0\x90\x83\x90ao\xE2V[\x90RP[\x80` \x01Q_\x03a:dWa:]a\x03\xE8a:Wa:R\x88\x88aG\xF8V[aH^V[\x90aG+V[\x81Ra:\xB6V[`\xC0\x81\x01Q\x15\x80a:wWP`\xE0\x81\x01Q\x15[\x15a:\x85W_\x91PPa\x03IV[a:\xB3a:\x9B\x86\x83` \x01Q\x84`\xC0\x01Qa;\xF6V[a:\xAE\x86\x84` \x01Q\x85`\xE0\x01Qa;\xF6V[aI>V[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a;.Wa;.ao\xBAV[` \x02\x01Q\x90P_a;@\x8A\x8AaISV[\x90P\x80_\x03a;\\W____\x95P\x95P\x95P\x95PPPa5\x8CV[_a;k\x83\x8C`\x80\x01QaJAV[\x90P_a;x\x82\x8AaB\x8EV[\x90P_\x89\x15a;\x9DW\x81\x84\x10a;\x97Wa;\x92\x84\x83aG+V[a;\x9FV[_a;\x9FV[_[\x90P_a;\xAC\x85\x8DaB\x8EV[\x90P_\x8C\x15a;\xD1W\x84\x82\x10a;\xCBWa;\xC6\x82\x86aG+V[a;\xD3V[_a;\xD3V[_[\x90Pa;\xDF\x85\x87arRV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a<*W\x83\x82\x81a< Wa< areV[\x04\x92PPPa\x03\nV[\x80\x84\x11a<JW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a<\xDE`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83aJqV[PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a=Z`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a=d\x84_aJ\xBDV[` \x83\x01R\x81R``\x84\x01Qa=z\x90_aB\xCFV[``\x82\x01\x81\x90R\x81Qa=\x9F\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a0W\x90`\narGV[`@\x82\x01R` \x81\x01Q_\x03a=\xBAW_`\x80\x82\x01Ra>ZV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>T\x91\x90ao\x88V[`\x80\x82\x01R[a>e\x84`\x01aJ\xBDV[` \x83\x01\x81\x90R\x90\x82R_\x03a>\x80W_`\xA0\x82\x01Ra? V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x1A\x91\x90ao\x88V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a?@\x84\x84aK\x03V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a?\x81\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\xE5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03I\x91\x90ao\x9FV[__a@0\x84\x84aK\x03V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a@\x83\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03I\x91\x90ao\x88V[__aA3\x85`\x01aISV[\x90P\x82\x15aAHWaAE\x84\x82ao\xE2V[\x90P[_aAR\x87aK\xBEV[\x90P_aA_\x83\x83aB\x8EV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10aA\x8DW\x87Q` \x01Q``\x01QaA\x88\x90\x83ao\xE2V[aA\x8FV[_[\x90PaA\xBE`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a<\xB5V[aA\xEE`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a<\xB5V[a\x0C\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a<\xB5V[_a\x03\n\x83\x83aL\x02V[aB4ae\xC9V[a\x03\n\x83\x83aL\x1BV[_____aBM\x88\x88aB!V[\x90PaB[\x87\x87\x83_a^9V[\x90\x93P\x91P\x81aBlW_\x19aBvV[aBv\x83\x83a42V[\x94PaB\x81\x88a\x06\xDCV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aB\xAEW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aB\xEFWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11aC\x14WaC\x0F\x83\x83ao\xE2V[a\x03\nV[a\x03\n\x82\x84ao\xE2V[_a\x03\n\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[_\x82\x84\x11aCPWaCK\x82ao\xF5V[a\x03IV[P\x92\x91PPV[_aC\x91`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aC\x9D\x86\x86\x86_a^9V[` \x80\x84\x01\x91\x90\x91R\x90\x82R`@\x80Q\x80\x82\x01\x90\x91R`\x0F\x81Rn\x1D\x98\\\x9C\xCB\x98\xDB\xDB\x1B\x18]\x19\\\x98[`\x8A\x1B\x91\x81\x01\x91\x90\x91R\x81QaC\xDD\x91\x90a<\xB5V[aD\x0C`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x1D\x98\\\x9C\xCB\x99\x19X\x9D`\xBA\x1B\x81RP\x82` \x01Qa<\xB5V[\x80` \x01Q_\x03aD>W\x84Q`\xFF\x84\x16`\x02\x81\x10aD-WaD-ao\xBAV[` \x02\x01Q` \x01Q\x91PPa\x04+V[\x80Q\x15\x80aDiWP\x84Q`\xFF\x84\x16`\x02\x81\x10aD]WaD]ao\xBAV[` \x02\x01Q` \x01Q_\x14[\x15aDwW_\x91PPa\x04+V[aD\x80\x87a_\xA5V[`@\x82\x01\x81\x90R` \x82\x01QaD\x95\x91aB\x8EV[`\x80\x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x90\x91R`\x11\x81Rp\x1D\x98\\\x9C\xCB\x98Y\x1A\x9D\\\xDD\x19Y\x11\x19X\x9D`z\x1B` \x82\x01R\x90QaD\xD0\x91\x90a<\xB5V[`\x80\x81\x01Q\x81Q\x10\x15aD\xE6W_\x91PPa\x04+V[`\x80\x81\x01Q\x81QaD\xF7\x91\x90ao\xE2V[\x81``\x01\x81\x81RPPaE\x0E\x86``\x01Q\x84aB\xCFV[`\xA0\x82\x01\x81\x90R``\x82\x01QaE:\x91aE)\x90`\narGV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xF6V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01aE_W`\xC0\x81\x01QaEY\x90\x85a42V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10aEuWaEuao\xBAV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15aE\xB2W\x84Q`\xFF\x84\x16`\x02\x81\x10aE\x9FWaE\x9Fao\xBAV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01aE\xE0WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01QaE\xFBWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x06\x83WaF\x17\x82ao\xF5V[a\x04+V[__aF'\x87a\x06\xDCV[\x90P_aF4\x82\x87aB\x8EV[\x90P_aFA\x83\x86aB\x8EV[\x90P_aFN\x89\x84aryV[\x90P_aF[\x83\x89aryV[\x90P_aFg\x83a_\xEBV[\x90P_aFs\x83a_\xEBV[\x90P_\x84\x13\x80\x15aF\x83WP_\x83\x12[\x80aF\x97WP_\x84\x12\x80\x15aF\x97WP_\x83\x13[\x15aF\xABW_\x97PPPPPPPPa\x04+V[\x80_\x03aF\xC1W_\x97PPPPPPPPa\x04+V[aF\xCB\x82\x82a42V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aF\xEBWP_a\x03\x1BV[_\x82\x84\x11aG\x02WaF\xFD\x84\x84ao\xE2V[aG\x0CV[aG\x0C\x83\x85ao\xE2V[\x90P_aG\x19\x82\x85a42V[\x90P\x83\x85\x11a\x03IWaF\x17\x81ao\xF5V[_\x82aG7\x83\x82ao\xE2V[\x91P\x81\x11\x15a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a6\xECV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aG\x96W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aG\xB0\x83\x82arRV[\x91P\x81\x10\x15a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a6\xECV[_\x81\x15\x80aH\x1BWP\x82\x82aH\r\x81\x83ar\x98V[\x92PaH\x19\x90\x83ar\xAFV[\x14[a\x03\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a6\xECV[_\x81_\x03aHmWP_\x91\x90PV[_`\x01aHy\x84a`\0V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aH\x92WaH\x92areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xAAWaH\xAAareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xC2WaH\xC2areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xDAWaH\xDAareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\xF2WaH\xF2areV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\nWaI\nareV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aI\"WaI\"areV[\x04\x82\x01\x90\x1C\x90Pa\x03\n\x81\x82\x85\x81aI<WaI<areV[\x04[_\x81\x83\x10aILW\x81a\x03\nV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aImWaImao\xBAV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xEA\x91\x90ao\x88V[\x90P\x80_\x03aI\xFDW_\x92PPPa\x03\x1BV[``\x82\x01Q`\xC0\x83\x01QaJ\x11\x90\x82arRV[\x82\x10aJ5W`\xC0\x83\x01QaJ&\x82\x84ao\xE2V[aJ0\x91\x90ao\xE2V[aJ7V[_[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aJTWP_a\x03\x1BV[_aJ_\x84\x84a`\x93V[`\xA0\x85\x01Q\x90\x91Pa\x03I\x90\x82aB\x8EV[aJ\xB8\x83\x83\x83`@Q`$\x01aJ\x89\x93\x92\x91\x90ar\xCEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra`\xD6V[PPPV[___aJ\xEA\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aJ\xDBWaJ\xDBao\xBAV[` \x02\x01Q\x86`\x80\x01QaJAV[\x90P_aJ\xF7\x86\x86aISV[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aK&\x90aoeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aKzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x9E\x91\x90ap\x96V[a\x03\nW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a6\xECV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x08\x14\x84a`\xE2V[aL#ae\xC9V[\x82aL,ae\xC9V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aLl\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\xE4\x91\x90ap\x96V[aL\xF1W\x91Pa\x03\x1B\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aM+\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM[\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\x8F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xCE\x91\x90ao\x88V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aN\x16\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aNF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aNz\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x95W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xB9\x91\x90ao\x9FV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aO\x15\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aOE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aOy\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xB8\x91\x90ao\x9FV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aPg\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xA6\x91\x90ao\x88V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aP\xFA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQyW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x9D\x91\x90ao\x88V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQ\xF7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR[\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aRvW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x9A\x91\x90ao\x88V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aR\xF3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aSW\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aSrW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x96\x91\x90ao\x88V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aT\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aTP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aTkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\x8F\x91\x90ao\x88V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aT\xE9\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUM\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aUhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x8C\x91\x90ao\x88V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aV3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVNW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aVr\x91\x90ao\x88V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV\xE6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aW\x1A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aWY\x91\x90ao\x9FV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX\0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX?\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\x94\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\xC4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY7\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\x92\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aY\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZ\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ5\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\x8F\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZ\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[2\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\x94\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xC4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[\xF8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\\x13W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\7\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\\\x92\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\xC2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\\xF6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]5\x91\x90ao\x88V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a]\x84\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a]\xB4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\xE8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^'\x91\x90ao\x88V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a^\xDCW__a^d\x8A\x8A_aa}V[\x91P\x91P_a^\x80_\x8C``\x01QaB\xCF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a^\x9E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[\x90P_a^\xBC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90Pa^\xC8\x82\x88arRV[\x96Pa^\xD4\x81\x87arRV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a_\x98W__a_\x03\x8A\x8A`\x01aa}V[\x91P\x91P_a_ `\x01\x8C``\x01QaB\xCF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a_>\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x85`\narGV[\x90P_a_\\\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0W\x86`\narGV[\x90P_a_i\x83\x8DaB\x8EV[\x90P_a_v\x83\x8EaB\x8EV[\x90Pa_\x82\x82\x8AarRV[\x98Pa_\x8E\x81\x89arRV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04S\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a_\xFCW\x81_\x03a\x03\x1BV[P\x90V[_\x80`\x80\x83\x90\x1C\x15a`\x14W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a`&W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a`8W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a`JW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a`\\W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a`nW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a`\x80W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03\x1BW`\x01\x01\x92\x91PPV[_B\x82\x03a`\xA6WP` \x82\x01Qa\x03\x1BV[_a`\xB5\x84`@\x01Q\x84ab$V[\x90Pa`\xCE\x84` \x01Q\x82aB\x8E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03\x1BV[a`\xDF\x81abXV[PV[\x80Q\x80QQ_\x91\x82\x91a`\xF6\x91`\x01a)\x8FV[\x90P\x80`@Q` \x01aa/\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa_\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aa\x98Waa\x98ao\xBAV[` \x02\x01Q`@\x01Q\x90P\x80_\x03aa\xB2W_\x91Paa\xFAV[_aa\xDD\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aa\xCEWaa\xCEao\xBAV[` \x02\x01Q\x88`\x80\x01Qa`\x93V[\x90P\x81\x15aa\xF4Waa\xEF\x82\x82aB\x8EV[aa\xF6V[_[\x92PP[\x84Q`\xFF\x85\x16`\x02\x81\x10ab\x10Wab\x10ao\xBAV[` \x02\x01Q` \x01Q\x92PP\x93P\x93\x91PPV[_\x80ab0\x83Bao\xE2V[ab:\x90\x85ar\x98V[c\x01\xE13\x80\x90\x04\x90Pa\x03I\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BarRV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[`@Q\x80a\x01\xA0\x01`@R\x80ab\x8Bae\xEFV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80ac\nafvV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ac\\af\xF1V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80ac\x81agGV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ad\x05acnV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01adZabwV[\x90R\x90V[`@Q\x80a\x04 \x01`@R\x80adsae\xC9V[\x81R` \x01ad\x80acnV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80ae\xBCacnV[\x81R` \x01adZacIV[`@Q\x80``\x01`@R\x80ae\xDCag\xAEV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af``@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xFEW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xDB`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81af\x85W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag1`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\0W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag\x98`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81agVW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ah\x06`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\xBDW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a`\xDFW__\xFD[_` \x82\x84\x03\x12\x15ah@W__\xFD[\x815a\x03\n\x81ah\x1CV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15ai+W\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ah\xFF\x90\x87\x01\x82ahKV[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ah\xC3V[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ah\x9FV[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15aiwW__\xFD[\x845ai\x82\x81ah\x1CV[\x93P` \x85\x015ai\x92\x81ah\x1CV[\x92P`@\x85\x015ai\xA2\x81ah\x1CV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ai\xC4W__\xFD[\x835ai\xCF\x81ah\x1CV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aj!Waj!ai\xE4V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ajBWajBai\xE4V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15aj]W__\xFD[\x825ajh\x81ah\x1CV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aj\x83W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aj\x93W__\xFD[\x805aj\xA6aj\xA1\x82aj)V[ai\xF8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aj\xC7W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aj\xE9W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aj\xCEV[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ak\xC2W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Rak8a\x01\x80\x86\x01\x82ahKV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pak\0V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Ral\x1Ba\x01\xA0\x88\x01\x82aj\xF7V[\x90P` \x82\x01Qal7` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QalR`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qal\x95`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qal\xABa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ak\xF3V[__`@\x83\x85\x03\x12\x15al\xFDW__\xFD[\x825am\x08\x81ah\x1CV[\x91P` \x83\x015am\x18\x81ah\x1CV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ak\xC2W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ramda\x01@\x86\x01\x82ahKV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pam,V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aiXW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ran$a\x01 \x88\x01\x82am#V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QanJ`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01am\xFCV[`\xFF\x81\x16\x81\x14a`\xDFW__\xFD[_____`\xA0\x86\x88\x03\x12\x15an\xBEW__\xFD[\x855an\xC9\x81ah\x1CV[\x94P` \x86\x015an\xD9\x81ah\x1CV[\x93P`@\x86\x015an\xE9\x81ah\x1CV[\x92P``\x86\x015\x91P`\x80\x86\x015ao\0\x81an\x9CV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15ao\"W__\xFD[\x855ao-\x81ah\x1CV[\x94P` \x86\x015ao=\x81ah\x1CV[\x93P`@\x86\x015aoM\x81ah\x1CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15ao\x98W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ao\xAFW__\xFD[\x81Qa\x03\n\x81ah\x1CV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x1BWa\x03\x1Bao\xCEV[_`\x01`\xFF\x1B\x82\x01ap\tWap\tao\xCEV[P_\x03\x90V[_` \x82\x84\x03\x12\x15ap\x1FW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13apEW__\xFD[\x80QapSaj\xA1\x82aj)V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aptW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aJ7W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ap{V[_` \x82\x84\x03\x12\x15ap\xA6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\nW__\xFD[_` \x82\x84\x03\x12\x15ap\xC5W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\xDBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ap\xEBW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\x05Waq\x05ai\xE4V[aq\x18`\x1F\x82\x01`\x1F\x19\x16` \x01ai\xF8V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aq,W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aqYW__\xFD[\x81Qa\x03\n\x81an\x9CV[`\x01\x81[`\x01\x84\x11\x15aq\x9FW\x80\x85\x04\x81\x11\x15aq\x83Waq\x83ao\xCEV[`\x01\x84\x16\x15aq\x91W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02aqhV[\x93P\x93\x91PPV[_\x82aq\xB5WP`\x01a\x03\x1BV[\x81aq\xC1WP_a\x03\x1BV[\x81`\x01\x81\x14aq\xD7W`\x02\x81\x14aq\xE1Waq\xFDV[`\x01\x91PPa\x03\x1BV[`\xFF\x84\x11\x15aq\xF2Waq\xF2ao\xCEV[PP`\x01\x82\x1Ba\x03\x1BV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ar WP\x81\x81\na\x03\x1BV[ar,_\x19\x84\x84aqdV[\x80_\x19\x04\x82\x11\x15ar?War?ao\xCEV[\x02\x93\x92PPPV[_a\x03\n\x83\x83aq\xA7V[\x80\x82\x01\x80\x82\x11\x15a\x03\x1BWa\x03\x1Bao\xCEV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aCPWaCPao\xCEV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x1BWa\x03\x1Bao\xCEV[_\x82ar\xC9WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_ar\xE0``\x83\x01\x86ahKV[\x82\x81\x03` \x84\x01Rar\xF2\x81\x86ahKV[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 }\xC0\x1E\xDC9\xA8\xE8\xD8\xFF\x7Fa\x0B4/\xEA\x96\x07\x11F\x12\xAB\x9B\xB0\"\xEB\x0B\xE4\x13o6\xDF\xC0dsolcC\0\x08\x1C\x003",
    );
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
    /**Function with signature `getPools2(address,bytes32[])` and selector `0x50376aaa`.
```solidity
function getPools2(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPool[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools2Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub poolKeys: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getPools2(address,bytes32[])`](getPools2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPools2Return {
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
            impl ::core::convert::From<getPools2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPools2Call) -> Self {
                    (value.dataStore, value.poolKeys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools2Call {
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
            impl ::core::convert::From<getPools2Return> for UnderlyingRustTuple<'_> {
                fn from(value: getPools2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPools2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPools2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPools2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPoolUtils::GetPool>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPools2(address,bytes32[])";
            const SELECTOR: [u8; 4] = [80u8, 55u8, 106u8, 170u8];
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
    /**Function with signature `getPositions2(address,bytes32[])` and selector `0xa6a100a0`.
```solidity
function getPositions2(address dataStore, bytes32[] memory positionKeys) external view returns (ReaderPositionUtils.GetPosition[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions2Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub positionKeys: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getPositions2(address,bytes32[])`](getPositions2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions2Return {
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
            impl ::core::convert::From<getPositions2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPositions2Call) -> Self {
                    (value.dataStore, value.positionKeys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositions2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataStore: tuple.0,
                        positionKeys: tuple.1,
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
            impl ::core::convert::From<getPositions2Return> for UnderlyingRustTuple<'_> {
                fn from(value: getPositions2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositions2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositions2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositions2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPositionUtils::GetPosition>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPositions2(address,bytes32[])";
            const SELECTOR: [u8; 4] = [166u8, 161u8, 0u8, 160u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.positionKeys),
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
        getPools2(getPools2Call),
        getPoolsCount(getPoolsCountCall),
        getPoolsInfo_0(getPoolsInfo_0Call),
        getPoolsInfo_1(getPoolsInfo_1Call),
        getPoolsInfo_2(getPoolsInfo_2Call),
        getPositions(getPositionsCall),
        getPositions2(getPositions2Call),
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
            [80u8, 55u8, 106u8, 170u8],
            [80u8, 237u8, 89u8, 45u8],
            [87u8, 185u8, 28u8, 166u8],
            [90u8, 111u8, 87u8, 16u8],
            [92u8, 57u8, 244u8, 103u8],
            [115u8, 145u8, 24u8, 164u8],
            [120u8, 242u8, 18u8, 209u8],
            [143u8, 108u8, 122u8, 60u8],
            [166u8, 161u8, 0u8, 160u8],
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
        const COUNT: usize = 18usize;
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
                Self::getPools2(_) => {
                    <getPools2Call as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getPositions2(_) => {
                    <getPositions2Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn getPools2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPools2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPools2)
                    }
                    getPools2
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
                    fn getPositions2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPositions2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPositions2)
                    }
                    getPositions2
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
                Self::getPools2(inner) => {
                    <getPools2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::getPositions2(inner) => {
                    <getPositions2Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getPools2(inner) => {
                    <getPools2Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getPositions2(inner) => {
                    <getPositions2Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
        EmptyAmount(EmptyAmount),
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
            [13u8, 20u8, 52u8, 88u8],
            [34u8, 123u8, 193u8, 83u8],
            [99u8, 49u8, 250u8, 177u8],
            [115u8, 87u8, 217u8, 31u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ReaderErrors {
        const NAME: &'static str = "ReaderErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyAmount(_) => {
                    <EmptyAmount as alloy_sol_types::SolError>::SELECTOR
                }
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
                    fn EmptyAmount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderErrors> {
                        <EmptyAmount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderErrors::EmptyAmount)
                    }
                    EmptyAmount
                },
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
                Self::EmptyAmount(inner) => {
                    <EmptyAmount as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
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
                Self::EmptyAmount(inner) => {
                    <EmptyAmount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
        ///Creates a new call builder for the [`getPools2`] function.
        pub fn getPools2(
            &self,
            dataStore: alloy::sol_types::private::Address,
            poolKeys: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPools2Call, N> {
            self.call_builder(
                &getPools2Call {
                    dataStore,
                    poolKeys,
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
        ///Creates a new call builder for the [`getPositions2`] function.
        pub fn getPositions2(
            &self,
            dataStore: alloy::sol_types::private::Address,
            positionKeys: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositions2Call, N> {
            self.call_builder(
                &getPositions2Call {
                    dataStore,
                    positionKeys,
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
