///Module containing a contract's types and functions.
/**

```solidity
library ReaderPoolUtils {
    struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLiquidity; uint256 loan; }
    struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; }
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
struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLiquidity; uint256 loan; }
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
        pub avaiableLiquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub loan: alloy::sol_types::private::primitives::aliases::U256,
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
                    value.avaiableLiquidity,
                    value.loan,
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
                    avaiableLiquidity: tuple.10,
                    loan: tuple.11,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.avaiableLiquidity),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.loan),
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
                    "Asset(address token,string symbol,uint256 decimals,uint256 borrowIndex,uint256 borrowApy,uint256 totalCollateral,uint256 totalCollateralWithDebt,uint256 totalDebtScaled,uint256 poolBalance,uint256 priceLiquidity,uint256 avaiableLiquidity,uint256 loan)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.avaiableLiquidity,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.loan)
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
                        &rust.avaiableLiquidity,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.loan)
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
                    &rust.avaiableLiquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.loan,
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
struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; }
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
                    "GetPool(Asset[2] assets,address bank,address interestRateStrategy,uint256 configuration,uint256 lastUpdateTimestamp,uint256 priceDecimals,uint256 price,address source,bool shortEnabled,uint256 createdTimestamp)",
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
    struct GetPosition { Asset[2] assets; uint256 id; address account; uint256 marginLevel; uint256 entryPrice; uint256 IndexPrice; int256 pnl; uint256 liquidationPrice; uint256 toLiquidationPrice; }
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
struct GetPosition { Asset[2] assets; uint256 id; address account; uint256 marginLevel; uint256 entryPrice; uint256 IndexPrice; int256 pnl; uint256 liquidationPrice; uint256 toLiquidationPrice; }
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
        pub toLiquidationPrice: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
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
                    <alloy::sol_types::sol_data::Uint<
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
                    "GetPosition(Asset[2] assets,uint256 id,address account,uint256 marginLevel,uint256 entryPrice,uint256 IndexPrice,int256 pnl,uint256 liquidationPrice,uint256 toLiquidationPrice)",
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
                    <alloy::sol_types::sol_data::Uint<
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
                    + <alloy::sol_types::sol_data::Uint<
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
                <alloy::sol_types::sol_data::Uint<
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
        uint256 avaiableLiquidity;
        uint256 loan;
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
        uint256 toLiquidationPrice;
    }
}

interface Reader {
    error EmptyPool(bytes32 key);
    error MathOverflowedMulDiv();
    error SingleTokenInOutSwapOnly();

    function calcAmountOut(address dataStore, address token0, address token1, uint256 amountIn, uint8 tokenInIndex) external view returns (uint256, uint256, int256);
    function calcLiquidityOut(address dataStore, address token0, address token1, uint256 amount0, uint256 amount1) external view returns (uint256);
    function calcTokenPairOut(address dataStore, address token0, address token1, uint256 liquidity) external view returns (uint256, uint256);
    function getDefaultInterestRateStrategy(address dataStore) external view returns (address);
    function getDefaultPoolConfiguration(address dataStore) external view returns (uint256);
    function getPools(address dataStore) external view returns (ReaderPoolUtils.GetPool[] memory);
    function getPools(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPool[] memory);
    function getPoolsCount(address dataStore) external view returns (uint256);
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
                "name": "avaiableLiquidity",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "loan",
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
                "name": "avaiableLiquidity",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "loan",
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
    ///0x6080604052348015600e575f5ffd5b50615c9e8061001c5f395ff3fe608060405234801561000f575f5ffd5b50600436106100a6575f3560e01c8063739118a41161006e578063739118a41461015657806378f212d1146101765780638f6c7a3c14610189578063d28b0a151461019c578063e335adb7146101ca578063f68a7131146101dd575f5ffd5b806328a0ccf4146100aa578063317b50ec146100da57806350ed592d146101025780635a6f5710146101235780635c39f46714610136575b5f5ffd5b6100bd6100b8366004615374565b6101f0565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ed6100e836600461538f565b610200565b604080519283526020830191909152016100d1565b610115610110366004615374565b61021b565b6040519081526020016100d1565b610115610131366004615374565b610225565b610149610144366004615374565b61022f565b6040516100d191906154e1565b6101696101643660046155ee565b61024f565b6040516100d191906156d8565b6100bd610184366004615374565b610272565b61014961019736600461579e565b61027c565b6101af6101aa3660046157de565b610289565b604080519384526020840192909252908201526060016100d1565b6100bd6101d8366004615374565b6102aa565b6101156101eb366004615842565b6102b4565b5f6101fa826102ce565b92915050565b5f5f61020e8686868661037f565b9150915094509492505050565b5f6101fa826103ba565b5f6101fa8261047e565b60605f61023b8361047e565b9050610248835f8361049d565b9392505050565b60605f61025c848461056b565b905061026a84845f846105e1565b949350505050565b5f6101fa826106b1565b606061026a84848461049d565b5f5f5f61029988888888886106ed565b925092509250955095509592505050565b5f6101fa826107dd565b5f6102c2868686868661082e565b90505b95945050505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161034091815260200190565b602060405180830381865afa15801561035b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa9190615899565b5f5f5f61038c8686610861565b90505f6103998883610908565b90505f5f6103a78388611b80565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043f91815260200190565b602060405180830381865afa15801561045a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa91906158b4565b5f816001600160a01b031663f3903b9f60405160200161040b906158cb565b60605f6104ab858585611d2f565b90505f815167ffffffffffffffff8111156104c8576104c86158ee565b60405190808252806020026020018201604052801561050157816020015b6104ee614ea4565b8152602001906001900390816104e65790505b5090505f5b8251811015610561575f83828151811061052257610522615902565b602002602001015190505f6105378983611dd0565b90508084848151811061054c5761054c615902565b60209081029190910101525050600101610506565b5095945050505050565b5f826001600160a01b031663f3903b9f61058484612222565b6040518263ffffffff1660e01b81526004016105a291815260200190565b602060405180830381865afa1580156105bd573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061024891906158b4565b60605f6105f0868686866122a6565b90505f815167ffffffffffffffff81111561060d5761060d6158ee565b60405190808252806020026020018201604052801561064657816020015b610633614f11565b81526020019060019003908161062b5790505b5090505f5b82518110156106a6575f83828151811061066757610667615902565b602002602001015190505f61067c8a8361232e565b90508084848151811061069157610691615902565b6020908102919091010152505060010161064b565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161030c90602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6106fb8888610861565b90505f6107088a83610908565b90505f808060ff891661073e576107208a855f612930565b92955091935061073791508590508b5f8087612a1a565b905061076e565b5f1960ff8a160161076e576107548a855f612ae7565b92955091935061076b91508590505f8c8682612a1a565b90505b5f61077885612bac565b90505f8282116107915761078c828461592a565b61079b565b61079b838361592a565b90505f6107a88284612c41565b90505f8484116107c0576107bb8261593d565b6107c2565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f61083a8686610861565b90505f6108478883610908565b90506108558186865f612c7c565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610882578183610885565b82825b60405191945092506108b2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610910614f64565b82610919614f64565b816001600160a01b03166391d4403c604051602001610937906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561098b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109af9190615957565b6109bc5791506101fa9050565b816001600160a01b03166321f8a721856040516020016109fc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a6091815260200190565b602060405180830381865afa158015610a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9f9190615899565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610b1d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b5191815260200190565b602060405180830381865afa158015610b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b9091906158b4565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610be6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4a91815260200190565b602060405180830381865afa158015610c65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c8991906158b4565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610cea9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4e91815260200190565b602060405180830381865afa158015610d69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8d91906158b4565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610df89060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610e28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e5c91815260200190565b602060405180830381865afa158015610e77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9b91906158b4565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610efc9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610f2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f6091815260200190565b602060405180830381865afa158015610f7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9f91906158b4565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161101c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161105091815260200190565b602060405180830381865afa15801561106b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061108f91906158b4565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611104929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113891815260200190565b602060405180830381865afa158015611153573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111779190615899565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161122191815260200190565b602060405180830381865afa15801561123c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061126091906158b4565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016112b790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016112e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131b91815260200190565b602060405180830381865afa158015611336573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061135a91906158b4565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016113bc9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161142091815260200190565b602060405180830381865afa15801561143b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145f91906158b4565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016114cb9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016114fb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161152f91815260200190565b602060405180830381865afa15801561154a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061156e91906158b4565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016115d09060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611600929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163491815260200190565b602060405180830381865afa15801561164f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061167391906158b4565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016116cc90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161173091815260200190565b602060405180830381865afa15801561174b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061176f91906158b4565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016117bd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ed929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161182191815260200190565b602060405180830381865afa15801561183c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118609190615899565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016118ce906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161193291815260200190565b602060405180830381865afa15801561194d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119719190615899565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016119d4906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a04929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a3891815260200190565b602060405180830381865afa158015611a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a7791906158b4565b60608201526040516001600160a01b0383169063bd02d0f5908690611ad0906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611b00929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b3491815260200190565b602060405180830381865afa158015611b4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b7391906158b4565b6080820152949350505050565b5f5f5f5f611b8c614f98565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf091906158b4565b6020820152611bff875f612daf565b6080840152506040820152611c15876001612daf565b60a084015250606082015260408101516020820151611c35918891612e2d565b61010082015260608101516020820151611c50918891612e2d565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611d4f906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611da9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261026a91908101906159a7565b611dd8614ea4565b611de0614fe1565b611dea8484610908565b808252611df7905f612daf565b60608401526040830152602080830191909152815151805190910151611e28915f5b602002015160a0015190612eec565b60808201528051611e3a906001612daf565b60e084015260c083015260a08201528051516020818101510151611e5f916001611e19565b610100820152604080516103008101825282515151516001600160a01b039081166101808301908152845151515184516395d89b4160e01b81529451939485946101408601948594936101a08801939116916395d89b41916004808201925f929091908290030181865afa158015611ed9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611f009190810190615a4f565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015611f4d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f719190615ae3565b60ff16815286515151602090810151818301528751515160409081015181840152885151516060908101518185015289515151608090810151818601528a51515160a09081015190860152838b015160c0860152828b015160e0860152908a015161010085015289015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa15801561204e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120759190810190615a4f565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120e99190615ae3565b60ff168152865151602090810151810151818301528751518101516040908101518184015288515182015160609081015181850152895151830151608090810151818601528a515184015160a09081015181870152808c015160c080880191909152808d015160e0808901919091528d0151610100808901919091528d0151610120909701969096529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906121c890612bac565b81526020016121d78686612f2d565b6001600160a01b031681526020016121fe835f015160600151660800000000000016151590565b1515815260200161220f868661301d565b9052610120909101819052905092915050565b5f60405160200161225c906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a6122c086612222565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612307573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102c591908101906159a7565b612336614f11565b61233e615037565b612348848461311f565b80825251805151602090910151516123609190610861565b60408201819052612372908590610908565b602082018190528151612386918691613131565b505050506060820152602081015161239d90612bac565b610300820152805180515160209081015160e0840152808301515151015190516123d391905f5b60200201516040015190612eec565b60c08201526020810151606001516123eb905f613180565b60a082015260e081015160c082015161240491906131ae565b610100820181905260a082015161241b91906131cf565b610120820181905260e082015160c0830151612436926131eb565b6101408201526020810151815161030083015161245792879290915f613208565b61016082015260e081015160c08201516101008301516124789291906131eb565b6101808201526101408101516101a082015260208101516060015161249e906001613180565b6101c0820152805180516020908101518101516102008401528083015151810151015190516124cf919060016123c4565b6101e082018190526102008201516124e6916131ae565b61022082018190526101c08201516124fe91906131cf565b61024082018190526103008201516125169190612eec565b61026082018190526102008201516101e0830151612533926131eb565b6102808201526020810151815161030083015161255592879290916001613208565b6102a08201526102008101516101e08201516102208301516125789291906131eb565b6102c08201526102808101516102e082015280516125959061339d565b60808201528051516020015160e0015160021461268d576125bf81610300015182608001516131ae565b61032082018190526102408201516125d691612eec565b610340820181905260808201516103008301511161038083018190526102008301516101e0840151612607936133df565b61036082018190526103a082015260e081015160a082015161266e91869161262f91906131cf565b6126418460c001518560a001516131cf565b612655856102000151866101c001516131cf565b612669866101e00151876101c001516131cf565b613406565b6103c0820181905261030082015161268691906134c6565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612701573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127289190810190615a4f565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061277d5761277d615902565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106127cc576127cc615902565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa15801561282c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128539190810190615a4f565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f61293c615114565b612946875f612daf565b6040840152508152612959876001612daf565b6060840152506020820152851561297f5787815f0181815161297b919061592a565b9052505b606087015160381c61ffff1661012082018190526129ae9089906129a690612710906134df565b612710612e2d565b6101408201819052815160208301516129d1926129cc908390613539565b612e2d565b6080820181905260208201516129e6916134df565b60c0820181905260408201516060830151610120840151612a08908c9061358d565b94509450945094505093509350935093565b5f5f5f5f86118015612a2a575083155b15612a39575083905084612a6e565b5f87118015612a46575084155b15612a55575085905082612a6e565b604051636331fab160e01b815260040160405180910390fd5b5f612a7d89606001515f613180565b90505f612a8f8a606001516001613180565b90505f612aad85676765c793fa10079d601b1b6129cc86600a615be1565b90505f612acb85676765c793fa10079d601b1b6129cc86600a615be1565b9050612ad78282612c41565b9c9b505050505050505050505050565b5f5f5f5f612af3615114565b612afd875f612daf565b6040840152508152612b10876001612daf565b60608401525060208201528515612b37578781602001818151612b33919061592a565b9052505b80516020820151612b4d91906129cc818c613539565b608082018190528151612b5f916134df565b60a0820152606087015160381c61ffff16610120820181905260a0820151612b8e916129a690612710906134df565b6040820151606083015161012084015160a0850151612a089161358d565b5f5f612bb8835f612daf565b505090505f612bc8846001612daf565b50509050805f03612bdc57505f9392505050565b5f612beb85606001515f613180565b90505f612bfd86606001516001613180565b90505f612c1b85676765c793fa10079d601b1b6129cc86600a615be1565b90505f612c3985676765c793fa10079d601b1b6129cc86600a615be1565b905061085582825b5f8115676765c793fa10079d601b1b60028404190484111715612c62575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612c85614f98565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612cc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce991906158b4565b6020820152612cf8865f612daf565b505060c0820152612d0a866001612daf565b505060e08201528215612d4457848160c001818151612d29919061592a565b90525060e081018051859190612d4090839061592a565b9052505b80602001515f03612d7457612d6d6103e8612d67612d6288886135b1565b613617565b906134df565b8152612da5565b612da2612d8a8683602001518460c00151612e2d565b612d9d8684602001518560e00151612e2d565b6136f7565b81525b5195945050505050565b5f5f5f5f855f01518560ff1660028110612dcb57612dcb615902565b602002015190505f612ddd878761370c565b9050805f03612df6575f5f5f9450945094505050612e26565b5f612e058389608001516137de565b9050612e118183615bec565b82612e1c838261592a565b9550955095505050505b9250925092565b5f838302815f1985870982811083820303915050805f03612e6157838281612e5757612e57615bff565b0492505050610248565b808411612e815760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f81156b019d971e4fe8401e740000001983900484111517612f0c575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f612f39848461380e565b9050806001600160a01b03166321f8a72184604051602001612f7a906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612faa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612fde91815260200190565b602060405180830381865afa158015612ff9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a9190615899565b5f5f613029848461380e565b9050806001600160a01b031663bd02d0f58460405160200161307c9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016130ac929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016130e091815260200190565b602060405180830381865afa1580156130fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a91906158b4565b613127615163565b61024883836138c9565b5f5f5f5f5f61313f87612bac565b905061314d8787835f614ae7565b90935091508161315e575f19613168565b6131688383612c41565b945061317388614c53565b9350939792965093509350565b5f60ff60581b1960585f1960ff8516016131a0575060ff60601b19905060605b90198416901c905092915050565b5f8183116131c5576131c0838361592a565b610248565b610248828461592a565b5f61024883676765c793fa10079d601b1b6129cc85600a615be1565b5f828411613201576131fc8261593d565b61026a565b5092915050565b5f6132426040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61324e8686865f614ae7565b6020830152808252158061327f5750845160ff84166002811061327357613273615902565b6020020151602001515f145b1561328d575f9150506102c5565b61329687614ca4565b6040820181905260208201516132ab91612eec565b60808201819052815110156132c3575f9150506102c5565b608081015181516132d4919061592a565b8160600181815250506132eb866060015184613180565b60a0820181905260608201516133179161330690600a615be1565b676765c793fa10079d601b1b612e2d565b60c08201525f1960ff84160161333c5760c08101516133369085612c41565b60c08201525b845160ff84166002811061335257613352615902565b6020020151602001518160c00151111561338f57845160ff84166002811061337c5761337c615902565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f19016133bd575051602001516060015190565b81516020015160e001516133d8575051602001516080015190565b505f919050565b5f8480156133ec57508284115b6133fe576133f98261593d565b6102c5565b509392505050565b5f5f61341187614c53565b90505f61341e8287612eec565b90505f61342b8386612eec565b90505f6134388984615c13565b90505f6134458389615c13565b90505f61345183614cea565b90505f61345d83614cea565b90505f8413801561346d57505f83125b8061348157505f8412801561348157505f83135b15613495575f9750505050505050506102c5565b805f036134ab575f9750505050505050506102c5565b6134b58282612c41565b9d9c50505050505050505050505050565b5f82156134d7576131c08284612c41565b505f92915050565b5f826134eb838261592a565b91508111156101fa5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826135458382615bec565b91508110156101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401613530565b5f811561138819839004841115176135a3575f5ffd5b506127109102611388010490565b5f8115806135d4575082826135c68183615c32565b92506135d29083615c49565b145b6101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b6044820152606401613530565b5f815f0361362657505f919050565b5f600161363284614cff565b901c6001901b9050600181848161364b5761364b615bff565b048201901c9050600181848161366357613663615bff565b048201901c9050600181848161367b5761367b615bff565b048201901c9050600181848161369357613693615bff565b048201901c905060018184816136ab576136ab615bff565b048201901c905060018184816136c3576136c3615bff565b048201901c905060018184816136db576136db615bff565b048201901c9050610248818285816136f5576136f5615bff565b045b5f8183106137055781610248565b5090919050565b5f5f835f01518360ff166002811061372657613726615902565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa15801561377f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a391906158b4565b9050805f036137b6575f925050506101fa565b606082015160c08301516137ca828461592a565b6137d4919061592a565b9695505050505050565b5f8260a001515f036137f157505f6101fa565b5f6137fc8484614d92565b60a085015190915061026a9082612eec565b5f5f839050806001600160a01b03166391d4403c604051602001613831906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015613885573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138a99190615957565b61024857604051637357d91f60e01b815260048101849052602401613530565b6138d1615163565b826138da615163565b816001600160a01b03166391d4403c60405160200161391a906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561396e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139929190615957565b61399f5791506101fa9050565b816001600160a01b031663bd02d0f5856040516020016139d9906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001613a09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a3d91815260200190565b602060405180830381865afa158015613a58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a7c91906158b4565b816020018181525050816001600160a01b03166321f8a72185604051602001613ac4906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613af4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b2891815260200190565b602060405180830381865afa158015613b43573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b679190615899565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613bc3906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613bf3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c2791815260200190565b602060405180830381865afa158015613c42573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c669190615899565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613ce1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d1591815260200190565b602060405180830381865afa158015613d30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d5491906158b4565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613da89060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001613dd8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e0c91815260200190565b602060405180830381865afa158015613e27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e4b91906158b4565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613ea5906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001613ed5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f0991815260200190565b602060405180830381865afa158015613f24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f4891906158b4565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001613fa1906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613fd1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161400591815260200190565b602060405180830381865afa158015614020573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061404491906158b4565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016140ca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140fe91815260200190565b602060405180830381865afa158015614119573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061413d91906158b4565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614197906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016141c7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016141fb91815260200190565b602060405180830381865afa158015614216573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061423a91906158b4565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016142ad929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016142e191815260200190565b602060405180830381865afa1580156142fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061432091906158b4565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614394929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016143c891815260200190565b602060405180830381865afa1580156143e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144079190615899565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144ae91815260200190565b602060405180830381865afa1580156144c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144ed91906158b4565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016145429060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001614572929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016145a691815260200190565b602060405180830381865afa1580156145c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145e591906158b4565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161464090602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614670929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016146a491815260200190565b602060405180830381865afa1580156146bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146e391906158b4565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161473d90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161476d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147a191815260200190565b602060405180830381865afa1580156147bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147e091906158b4565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016148429060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614872929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148a691815260200190565b602060405180830381865afa1580156148c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148e591906158b4565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161494090602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614970929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149a491815260200190565b602060405180830381865afa1580156149bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149e391906158b4565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001614a32906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001614a62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a9691815260200190565b602060405180830381865afa158015614ab1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ad591906158b4565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614614b8a575f5f614b128a8a5f614dd8565b915091505f614b2e5f8c6060015161318090919063ffffffff16565b90505f614b4c84676765c793fa10079d601b1b6129cc85600a615be1565b90505f614b6a84676765c793fa10079d601b1b6129cc86600a615be1565b9050614b768288615bec565b9650614b828187615bec565b955050505050505b865160200151516001600160a01b03868116911614614c46575f5f614bb18a8a6001614dd8565b915091505f614bce60018c6060015161318090919063ffffffff16565b90505f614bec84676765c793fa10079d601b1b6129cc85600a615be1565b90505f614c0a84676765c793fa10079d601b1b6129cc86600a615be1565b90505f614c17838d612eec565b90505f614c24838e612eec565b9050614c30828a615bec565b9850614c3c8189615bec565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161040b9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215614cfb57815f036101fa565b5090565b5f80608083901c15614d1357608092831c92015b604083901c15614d2557604092831c92015b602083901c15614d3757602092831c92015b601083901c15614d4957601092831c92015b600883901c15614d5b57600892831c92015b600483901c15614d6d57600492831c92015b600283901c15614d7f57600292831c92015b600183901c156101fa5760010192915050565b5f428203614da5575060208201516101fa565b5f614db4846040015184614e70565b9050614dcd846020015182612eec90919063ffffffff16565b9150506101fa565b50565b5f5f5f845f01518460ff1660028110614df357614df3615902565b60200201516040015190505f614e29875f01518660ff1660028110614e1a57614e1a615902565b60200201518860800151614d92565b90508115614e4057614e3b8282612eec565b614e42565b5f5b865190935060ff861660028110614e5b57614e5b615902565b60200201516020015193505050935093915050565b5f80614e7c834261592a565b614e869085615c32565b6301e133809004905061026a81676765c793fa10079d601b1b615bec565b604051806101400160405280614eb8615189565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81525090565b604051806101200160405280614f25615210565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614f7761528b565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101400160405280614ff5614f64565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615032614ea4565b905290565b60405180610400016040528061504b615163565b8152602001615058614f64565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806151766152f2565b81525f6020820181905260409091015290565b60405180604001604052806002905b6151fa6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816151985790505090565b60405180604001604052806002905b6152756040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161521f5790505090565b60405180604001604052806002905b6152dc6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161529a5790505090565b60405180604001604052806002905b61534a6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816153015790505090565b6001600160a01b0381168114614dd5575f5ffd5b5f60208284031215615384575f5ffd5b813561024881615360565b5f5f5f5f608085870312156153a2575f5ffd5b84356153ad81615360565b935060208501356153bd81615360565b925060408501356153cd81615360565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b60028110156154d6578383038752815180516001600160a01b031684526020810151610180602086015261544c6101808601826153dd565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615414565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156155e257603f1987860301845281518051610140875261552f61014088018261540b565b9050602082015161554b60208901826001600160a01b03169052565b50604082015161556660408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e08201516155a960e08901826001600160a01b03169052565b506101008201516155bf61010089018215159052565b506101209182015196909101959095526020938401939190910190600101615507565b50929695505050505050565b5f5f604083850312156155ff575f5ffd5b823561560a81615360565b9150602083013561561a81615360565b809150509250929050565b5f8260408101835f5b60028110156154d6578383038752815180516001600160a01b03168452602081015161014060208601526156666101408601826153dd565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e0860152610100820151610100860152610120820151610120860152809450505060208201915060208701965060018101905061562e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156155e257603f19878603018452815180516101208752615726610120880182615625565b905060208201516020880152604082015161574c60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e0808301519088015261010091820151919096015260209384019391909101906001016156fe565b5f5f5f606084860312156157b0575f5ffd5b83356157bb81615360565b95602085013595506040909401359392505050565b60ff81168114614dd5575f5ffd5b5f5f5f5f5f60a086880312156157f2575f5ffd5b85356157fd81615360565b9450602086013561580d81615360565b9350604086013561581d81615360565b9250606086013591506080860135615834816157d0565b809150509295509295909350565b5f5f5f5f5f60a08688031215615856575f5ffd5b853561586181615360565b9450602086013561587181615360565b9350604086013561588181615360565b94979396509394606081013594506080013592915050565b5f602082840312156158a9575f5ffd5b815161024881615360565b5f602082840312156158c4575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156101fa576101fa615916565b5f600160ff1b820161595157615951615916565b505f0390565b5f60208284031215615967575f5ffd5b81518015158114610248575f5ffd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561599f5761599f6158ee565b604052919050565b5f602082840312156159b7575f5ffd5b815167ffffffffffffffff8111156159cd575f5ffd5b8201601f810184136159dd575f5ffd5b805167ffffffffffffffff8111156159f7576159f76158ee565b8060051b615a0760208201615976565b91825260208184018101929081019087841115615a22575f5ffd5b6020850194505b83851015615a44578451825260209485019490910190615a29565b979650505050505050565b5f60208284031215615a5f575f5ffd5b815167ffffffffffffffff811115615a75575f5ffd5b8201601f81018413615a85575f5ffd5b805167ffffffffffffffff811115615a9f57615a9f6158ee565b615ab2601f8201601f1916602001615976565b818152856020838501011115615ac6575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215615af3575f5ffd5b8151610248816157d0565b6001815b6001841115615b3957808504811115615b1d57615b1d615916565b6001841615615b2b57908102905b60019390931c928002615b02565b935093915050565b5f82615b4f575060016101fa565b81615b5b57505f6101fa565b8160018114615b715760028114615b7b57615b97565b60019150506101fa565b60ff841115615b8c57615b8c615916565b50506001821b6101fa565b5060208310610133831016604e8410600b8410161715615bba575081810a6101fa565b615bc65f198484615afe565b805f1904821115615bd957615bd9615916565b029392505050565b5f6102488383615b41565b808201808211156101fa576101fa615916565b634e487b7160e01b5f52601260045260245ffd5b8181035f83128015838313168383128216171561320157613201615916565b80820281158282048414176101fa576101fa615916565b5f82615c6357634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220acc84fdb42849c480344e963c0f9b36fbc0a32dad245d603ec480696065749d064736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\\\x9E\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01VW\x80cx\xF2\x12\xD1\x14a\x01vW\x80c\x8Flz<\x14a\x01\x89W\x80c\xD2\x8B\n\x15\x14a\x01\x9CW\x80c\xE35\xAD\xB7\x14a\x01\xCAW\x80c\xF6\x8Aq1\x14a\x01\xDDW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xAAW\x80c1{P\xEC\x14a\0\xDAW\x80cP\xEDY-\x14a\x01\x02W\x80cZoW\x10\x14a\x01#W\x80c\\9\xF4g\x14a\x016W[__\xFD[a\0\xBDa\0\xB86`\x04aStV[a\x01\xF0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEDa\0\xE86`\x04aS\x8FV[a\x02\0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD1V[a\x01\x15a\x01\x106`\x04aStV[a\x02\x1BV[`@Q\x90\x81R` \x01a\0\xD1V[a\x01\x15a\x0116`\x04aStV[a\x02%V[a\x01Ia\x01D6`\x04aStV[a\x02/V[`@Qa\0\xD1\x91\x90aT\xE1V[a\x01ia\x01d6`\x04aU\xEEV[a\x02OV[`@Qa\0\xD1\x91\x90aV\xD8V[a\0\xBDa\x01\x846`\x04aStV[a\x02rV[a\x01Ia\x01\x976`\x04aW\x9EV[a\x02|V[a\x01\xAFa\x01\xAA6`\x04aW\xDEV[a\x02\x89V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD1V[a\0\xBDa\x01\xD86`\x04aStV[a\x02\xAAV[a\x01\x15a\x01\xEB6`\x04aXBV[a\x02\xB4V[_a\x01\xFA\x82a\x02\xCEV[\x92\x91PPV[__a\x02\x0E\x86\x86\x86\x86a\x03\x7FV[\x91P\x91P\x94P\x94\x92PPPV[_a\x01\xFA\x82a\x03\xBAV[_a\x01\xFA\x82a\x04~V[``_a\x02;\x83a\x04~V[\x90Pa\x02H\x83_\x83a\x04\x9DV[\x93\x92PPPV[``_a\x02\\\x84\x84a\x05kV[\x90Pa\x02j\x84\x84_\x84a\x05\xE1V[\x94\x93PPPPV[_a\x01\xFA\x82a\x06\xB1V[``a\x02j\x84\x84\x84a\x04\x9DV[___a\x02\x99\x88\x88\x88\x88\x88a\x06\xEDV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x01\xFA\x82a\x07\xDDV[_a\x02\xC2\x86\x86\x86\x86\x86a\x08.V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aX\x99V[___a\x03\x8C\x86\x86a\x08aV[\x90P_a\x03\x99\x88\x83a\t\x08V[\x90P__a\x03\xA7\x83\x88a\x1B\x80V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aX\xB4V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04\x0B\x90aX\xCBV[``_a\x04\xAB\x85\x85\x85a\x1D/V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC8Wa\x04\xC8aX\xEEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x01W\x81` \x01[a\x04\xEEaN\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\xE6W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x05aW_\x83\x82\x81Q\x81\x10a\x05\"Wa\x05\"aY\x02V[` \x02` \x01\x01Q\x90P_a\x057\x89\x83a\x1D\xD0V[\x90P\x80\x84\x84\x81Q\x81\x10a\x05LWa\x05LaY\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x06V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x05\x84\x84a\"\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02H\x91\x90aX\xB4V[``_a\x05\xF0\x86\x86\x86\x86a\"\xA6V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\raX\xEEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06FW\x81` \x01[a\x063aO\x11V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06+W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xA6W_\x83\x82\x81Q\x81\x10a\x06gWa\x06gaY\x02V[` \x02` \x01\x01Q\x90P_a\x06|\x8A\x83a#.V[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\x91Wa\x06\x91aY\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06KV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x06\xFB\x88\x88a\x08aV[\x90P_a\x07\x08\x8A\x83a\t\x08V[\x90P_\x80\x80`\xFF\x89\x16a\x07>Wa\x07 \x8A\x85_a)0V[\x92\x95P\x91\x93Pa\x077\x91P\x85\x90P\x8B_\x80\x87a*\x1AV[\x90Pa\x07nV[_\x19`\xFF\x8A\x16\x01a\x07nWa\x07T\x8A\x85_a*\xE7V[\x92\x95P\x91\x93Pa\x07k\x91P\x85\x90P_\x8C\x86\x82a*\x1AV[\x90P[_a\x07x\x85a+\xACV[\x90P_\x82\x82\x11a\x07\x91Wa\x07\x8C\x82\x84aY*V[a\x07\x9BV[a\x07\x9B\x83\x83aY*V[\x90P_a\x07\xA8\x82\x84a,AV[\x90P_\x84\x84\x11a\x07\xC0Wa\x07\xBB\x82aY=V[a\x07\xC2V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08:\x86\x86a\x08aV[\x90P_a\x08G\x88\x83a\t\x08V[\x90Pa\x08U\x81\x86\x86_a,|V[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x82W\x81\x83a\x08\x85V[\x82\x82[`@Q\x91\x94P\x92Pa\x08\xB2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\x10aOdV[\x82a\t\x19aOdV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t7\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aYWV[a\t\xBCW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xFC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90aX\x99V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BQ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x90\x91\x90aX\xB4V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xE6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xEA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90aX\xB4V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\xF8\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\xFC\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9F\x91\x90aX\xB4V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10P\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8F\x91\x90aX\xB4V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x118\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11w\x91\x90aX\x99V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB7\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x136W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Z\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xBC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14 \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xCB\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xD0\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x164\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16s\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\xCC\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17o\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xBD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18`\x91\x90aX\x99V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18\xCE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x192\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19q\x91\x90aX\x99V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xD4\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ASW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Aw\x91\x90aX\xB4V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1A\xD0\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bs\x91\x90aX\xB4V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1B\x8CaO\x98V[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF0\x91\x90aX\xB4V[` \x82\x01Ra\x1B\xFF\x87_a-\xAFV[`\x80\x84\x01RP`@\x82\x01Ra\x1C\x15\x87`\x01a-\xAFV[`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C5\x91\x88\x91a.-V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1CP\x91\x88\x91a.-V[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1DO\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02j\x91\x90\x81\x01\x90aY\xA7V[a\x1D\xD8aN\xA4V[a\x1D\xE0aO\xE1V[a\x1D\xEA\x84\x84a\t\x08V[\x80\x82Ra\x1D\xF7\x90_a-\xAFV[``\x84\x01R`@\x83\x01R` \x80\x83\x01\x91\x90\x91R\x81QQ\x80Q\x90\x91\x01Qa\x1E(\x91_[` \x02\x01Q`\xA0\x01Q\x90a.\xECV[`\x80\x82\x01R\x80Qa\x1E:\x90`\x01a-\xAFV[`\xE0\x84\x01R`\xC0\x83\x01R`\xA0\x82\x01R\x80QQ` \x81\x81\x01Q\x01Qa\x1E_\x91`\x01a\x1E\x19V[a\x01\0\x82\x01R`@\x80Qa\x03\0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\x80\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01@\x86\x01\x94\x85\x94\x93a\x01\xA0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xD9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\0\x91\x90\x81\x01\x90aZOV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fq\x91\x90aZ\xE3V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R\x87QQQ`@\x90\x81\x01Q\x81\x84\x01R\x88QQQ``\x90\x81\x01Q\x81\x85\x01R\x89QQQ`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\xA0\x90\x81\x01Q\x90\x86\x01R\x83\x8B\x01Q`\xC0\x86\x01R\x82\x8B\x01Q`\xE0\x86\x01R\x90\x8A\x01Qa\x01\0\x85\x01R\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a NW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra u\x91\x90\x81\x01\x90aZOV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xE9\x91\x90aZ\xE3V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01R\x87QQ\x81\x01Q`@\x90\x81\x01Q\x81\x84\x01R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R\x80\x8C\x01Q`\xC0\x80\x88\x01\x91\x90\x91R\x80\x8D\x01Q`\xE0\x80\x89\x01\x91\x90\x91R\x8D\x01Qa\x01\0\x80\x89\x01\x91\x90\x91R\x8D\x01Qa\x01 \x90\x97\x01\x96\x90\x96R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a!\xC8\x90a+\xACV[\x81R` \x01a!\xD7\x86\x86a/-V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a!\xFE\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a\"\x0F\x86\x86a0\x1DV[\x90Ra\x01 \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a\"\\\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a\"\xC0\x86a\"\"V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x07W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xC5\x91\x90\x81\x01\x90aY\xA7V[a#6aO\x11V[a#>aP7V[a#H\x84\x84a1\x1FV[\x80\x82RQ\x80QQ` \x90\x91\x01QQa#`\x91\x90a\x08aV[`@\x82\x01\x81\x90Ra#r\x90\x85\x90a\t\x08V[` \x82\x01\x81\x90R\x81Qa#\x86\x91\x86\x91a11V[PPPP``\x82\x01R` \x81\x01Qa#\x9D\x90a+\xACV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa#\xD3\x91\x90_[` \x02\x01Q`@\x01Q\x90a.\xECV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa#\xEB\x90_a1\x80V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa$\x04\x91\x90a1\xAEV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa$\x1B\x91\x90a1\xCFV[a\x01 \x82\x01\x81\x90R`\xE0\x82\x01Q`\xC0\x83\x01Qa$6\x92a1\xEBV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa$W\x92\x87\x92\x90\x91_a2\x08V[a\x01`\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa$x\x92\x91\x90a1\xEBV[a\x01\x80\x82\x01Ra\x01@\x81\x01Qa\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa$\x9E\x90`\x01a1\x80V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa$\xCF\x91\x90`\x01a#\xC4V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa$\xE6\x91a1\xAEV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa$\xFE\x91\x90a1\xCFV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa%\x16\x91\x90a.\xECV[a\x02`\x82\x01\x81\x90Ra\x02\0\x82\x01Qa\x01\xE0\x83\x01Qa%3\x92a1\xEBV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%U\x92\x87\x92\x90\x91`\x01a2\x08V[a\x02\xA0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa%x\x92\x91\x90a1\xEBV[a\x02\xC0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xE0\x82\x01R\x80Qa%\x95\x90a3\x9DV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a&\x8DWa%\xBF\x81a\x03\0\x01Q\x82`\x80\x01Qa1\xAEV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa%\xD6\x91a.\xECV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa&\x07\x93a3\xDFV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa&n\x91\x86\x91a&/\x91\x90a1\xCFV[a&A\x84`\xC0\x01Q\x85`\xA0\x01Qa1\xCFV[a&U\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa1\xCFV[a&i\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa1\xCFV[a4\x06V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&\x86\x91\x90a4\xC6V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\x01W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'(\x91\x90\x81\x01\x90aZOV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a'}Wa'}aY\x02V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a'\xCCWa'\xCCaY\x02V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(,W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(S\x91\x90\x81\x01\x90aZOV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a)<aQ\x14V[a)F\x87_a-\xAFV[`@\x84\x01RP\x81Ra)Y\x87`\x01a-\xAFV[``\x84\x01RP` \x82\x01R\x85\x15a)\x7FW\x87\x81_\x01\x81\x81Qa){\x91\x90aY*V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra)\xAE\x90\x89\x90a)\xA6\x90a'\x10\x90a4\xDFV[a'\x10a.-V[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa)\xD1\x92a)\xCC\x90\x83\x90a59V[a.-V[`\x80\x82\x01\x81\x90R` \x82\x01Qa)\xE6\x91a4\xDFV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa*\x08\x90\x8C\x90a5\x8DV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a**WP\x83\x15[\x15a*9WP\x83\x90P\x84a*nV[_\x87\x11\x80\x15a*FWP\x84\x15[\x15a*UWP\x85\x90P\x82a*nV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*}\x89``\x01Q_a1\x80V[\x90P_a*\x8F\x8A``\x01Q`\x01a1\x80V[\x90P_a*\xAD\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_a*\xCB\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90Pa*\xD7\x82\x82a,AV[\x9C\x9BPPPPPPPPPPPPV[____a*\xF3aQ\x14V[a*\xFD\x87_a-\xAFV[`@\x84\x01RP\x81Ra+\x10\x87`\x01a-\xAFV[``\x84\x01RP` \x82\x01R\x85\x15a+7W\x87\x81` \x01\x81\x81Qa+3\x91\x90aY*V[\x90RP[\x80Q` \x82\x01Qa+M\x91\x90a)\xCC\x81\x8Ca59V[`\x80\x82\x01\x81\x90R\x81Qa+_\x91a4\xDFV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa+\x8E\x91a)\xA6\x90a'\x10\x90a4\xDFV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa*\x08\x91a5\x8DV[__a+\xB8\x83_a-\xAFV[PP\x90P_a+\xC8\x84`\x01a-\xAFV[PP\x90P\x80_\x03a+\xDCWP_\x93\x92PPPV[_a+\xEB\x85``\x01Q_a1\x80V[\x90P_a+\xFD\x86``\x01Q`\x01a1\x80V[\x90P_a,\x1B\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_a,9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90Pa\x08U\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a,bW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a,\x85aO\x98V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE9\x91\x90aX\xB4V[` \x82\x01Ra,\xF8\x86_a-\xAFV[PP`\xC0\x82\x01Ra-\n\x86`\x01a-\xAFV[PP`\xE0\x82\x01R\x82\x15a-DW\x84\x81`\xC0\x01\x81\x81Qa-)\x91\x90aY*V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a-@\x90\x83\x90aY*V[\x90RP[\x80` \x01Q_\x03a-tWa-ma\x03\xE8a-ga-b\x88\x88a5\xB1V[a6\x17V[\x90a4\xDFV[\x81Ra-\xA5V[a-\xA2a-\x8A\x86\x83` \x01Q\x84`\xC0\x01Qa.-V[a-\x9D\x86\x84` \x01Q\x85`\xE0\x01Qa.-V[a6\xF7V[\x81R[Q\x95\x94PPPPPV[____\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a-\xCBWa-\xCBaY\x02V[` \x02\x01Q\x90P_a-\xDD\x87\x87a7\x0CV[\x90P\x80_\x03a-\xF6W___\x94P\x94P\x94PPPa.&V[_a.\x05\x83\x89`\x80\x01Qa7\xDEV[\x90Pa.\x11\x81\x83a[\xECV[\x82a.\x1C\x83\x82aY*V[\x95P\x95P\x95PPPP[\x92P\x92P\x92V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a.aW\x83\x82\x81a.WWa.Wa[\xFFV[\x04\x92PPPa\x02HV[\x80\x84\x11a.\x81W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a/\x0CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__a/9\x84\x84a8\x0EV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a/z\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aX\x99V[__a0)\x84\x84a8\x0EV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a0|\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\xE0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aX\xB4V[a1'aQcV[a\x02H\x83\x83a8\xC9V[_____a1?\x87a+\xACV[\x90Pa1M\x87\x87\x83_aJ\xE7V[\x90\x93P\x91P\x81a1^W_\x19a1hV[a1h\x83\x83a,AV[\x94Pa1s\x88aLSV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a1\xA0WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a1\xC5Wa1\xC0\x83\x83aY*V[a\x02HV[a\x02H\x82\x84aY*V[_a\x02H\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[_\x82\x84\x11a2\x01Wa1\xFC\x82aY=V[a\x02jV[P\x92\x91PPV[_a2B`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a2N\x86\x86\x86_aJ\xE7V[` \x83\x01R\x80\x82R\x15\x80a2\x7FWP\x84Q`\xFF\x84\x16`\x02\x81\x10a2sWa2saY\x02V[` \x02\x01Q` \x01Q_\x14[\x15a2\x8DW_\x91PPa\x02\xC5V[a2\x96\x87aL\xA4V[`@\x82\x01\x81\x90R` \x82\x01Qa2\xAB\x91a.\xECV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a2\xC3W_\x91PPa\x02\xC5V[`\x80\x81\x01Q\x81Qa2\xD4\x91\x90aY*V[\x81``\x01\x81\x81RPPa2\xEB\x86``\x01Q\x84a1\x80V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa3\x17\x91a3\x06\x90`\na[\xE1V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba.-V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a3<W`\xC0\x81\x01Qa36\x90\x85a,AV[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a3RWa3RaY\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a3\x8FW\x84Q`\xFF\x84\x16`\x02\x81\x10a3|Wa3|aY\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a3\xBDWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa3\xD8WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x80\x15a3\xECWP\x82\x84\x11[a3\xFEWa3\xF9\x82aY=V[a\x02\xC5V[P\x93\x92PPPV[__a4\x11\x87aLSV[\x90P_a4\x1E\x82\x87a.\xECV[\x90P_a4+\x83\x86a.\xECV[\x90P_a48\x89\x84a\\\x13V[\x90P_a4E\x83\x89a\\\x13V[\x90P_a4Q\x83aL\xEAV[\x90P_a4]\x83aL\xEAV[\x90P_\x84\x13\x80\x15a4mWP_\x83\x12[\x80a4\x81WP_\x84\x12\x80\x15a4\x81WP_\x83\x13[\x15a4\x95W_\x97PPPPPPPPa\x02\xC5V[\x80_\x03a4\xABW_\x97PPPPPPPPa\x02\xC5V[a4\xB5\x82\x82a,AV[\x9D\x9CPPPPPPPPPPPPPV[_\x82\x15a4\xD7Wa1\xC0\x82\x84a,AV[P_\x92\x91PPV[_\x82a4\xEB\x83\x82aY*V[\x91P\x81\x11\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a5E\x83\x82a[\xECV[\x91P\x81\x10\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a50V[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a5\xA3W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a5\xD4WP\x82\x82a5\xC6\x81\x83a\\2V[\x92Pa5\xD2\x90\x83a\\IV[\x14[a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a50V[_\x81_\x03a6&WP_\x91\x90PV[_`\x01a62\x84aL\xFFV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a6KWa6Ka[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6cWa6ca[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6{Wa6{a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\x93Wa6\x93a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xABWa6\xABa[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xC3Wa6\xC3a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xDBWa6\xDBa[\xFFV[\x04\x82\x01\x90\x1C\x90Pa\x02H\x81\x82\x85\x81a6\xF5Wa6\xF5a[\xFFV[\x04[_\x81\x83\x10a7\x05W\x81a\x02HV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a7&Wa7&aY\x02V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA3\x91\x90aX\xB4V[\x90P\x80_\x03a7\xB6W_\x92PPPa\x01\xFAV[``\x82\x01Q`\xC0\x83\x01Qa7\xCA\x82\x84aY*V[a7\xD4\x91\x90aY*V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a7\xF1WP_a\x01\xFAV[_a7\xFC\x84\x84aM\x92V[`\xA0\x85\x01Q\x90\x91Pa\x02j\x90\x82a.\xECV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a81\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xA9\x91\x90aYWV[a\x02HW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a50V[a8\xD1aQcV[\x82a8\xDAaQcV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a9\x1A\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x92\x91\x90aYWV[a9\x9FW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\xD9\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:|\x91\x90aX\xB4V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a:\xC4\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;(\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;g\x91\x90aX\x99V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a;\xC3\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<f\x91\x90aX\x99V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=T\x91\x90aX\xB4V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a=\xA8\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xD8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>K\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\xA5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?H\x91\x90aX\xB4V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\xA1\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@ W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@D\x91\x90aX\xB4V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA=\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aA\x97\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\xFB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB:\x91\x90aX\xB4V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xAD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xE1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC \x91\x90aX\xB4V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x07\x91\x90aX\x99V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xED\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aEB\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aEr\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xE5\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF@\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aFp\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xE3\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG=\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aGm\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xA1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xE0\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aHB\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aHr\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\xE5\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI@\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aIp\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xE3\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ2\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJb\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xD5\x91\x90aX\xB4V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aK\x8AW__aK\x12\x8A\x8A_aM\xD8V[\x91P\x91P_aK._\x8C``\x01Qa1\x80\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aKL\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[\x90P_aKj\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90PaKv\x82\x88a[\xECV[\x96PaK\x82\x81\x87a[\xECV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aLFW__aK\xB1\x8A\x8A`\x01aM\xD8V[\x91P\x91P_aK\xCE`\x01\x8C``\x01Qa1\x80\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aK\xEC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[\x90P_aL\n\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_aL\x17\x83\x8Da.\xECV[\x90P_aL$\x83\x8Ea.\xECV[\x90PaL0\x82\x8Aa[\xECV[\x98PaL<\x81\x89a[\xECV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aL\xFBW\x81_\x03a\x01\xFAV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aM\x13W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aM%W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aM7W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aMIW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aM[W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aMmW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aM\x7FW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x01\xFAW`\x01\x01\x92\x91PPV[_B\x82\x03aM\xA5WP` \x82\x01Qa\x01\xFAV[_aM\xB4\x84`@\x01Q\x84aNpV[\x90PaM\xCD\x84` \x01Q\x82a.\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x01\xFAV[PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aM\xF3WaM\xF3aY\x02V[` \x02\x01Q`@\x01Q\x90P_aN)\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aN\x1AWaN\x1AaY\x02V[` \x02\x01Q\x88`\x80\x01QaM\x92V[\x90P\x81\x15aN@WaN;\x82\x82a.\xECV[aNBV[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aN[WaN[aY\x02V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aN|\x83BaY*V[aN\x86\x90\x85a\\2V[c\x01\xE13\x80\x90\x04\x90Pa\x02j\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba[\xECV[`@Q\x80a\x01@\x01`@R\x80aN\xB8aQ\x89V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aO%aR\x10V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aOwaR\x8BV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01@\x01`@R\x80aO\xF5aOdV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aP2aN\xA4V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aPKaQcV[\x81R` \x01aPXaOdV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aQvaR\xF2V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aQ\xFA`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aQ\x98W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aRu`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x1FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aR\xDC`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x9AW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aSJ`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aS\x01W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aM\xD5W__\xFD[_` \x82\x84\x03\x12\x15aS\x84W__\xFD[\x815a\x02H\x81aS`V[____`\x80\x85\x87\x03\x12\x15aS\xA2W__\xFD[\x845aS\xAD\x81aS`V[\x93P` \x85\x015aS\xBD\x81aS`V[\x92P`@\x85\x015aS\xCD\x81aS`V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aT\xD6W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01RaTLa\x01\x80\x86\x01\x82aS\xDDV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaT\x14V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU\xE2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01@\x87RaU/a\x01@\x88\x01\x82aT\x0BV[\x90P` \x82\x01QaUK` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QaUf`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01QaU\xA9`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01QaU\xBFa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aU\x07V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aU\xFFW__\xFD[\x825aV\n\x81aS`V[\x91P` \x83\x015aV\x1A\x81aS`V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aT\xD6W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01RaVfa\x01@\x86\x01\x82aS\xDDV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaV.V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU\xE2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RaW&a\x01 \x88\x01\x82aV%V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QaWL`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aV\xFEV[___``\x84\x86\x03\x12\x15aW\xB0W__\xFD[\x835aW\xBB\x81aS`V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14aM\xD5W__\xFD[_____`\xA0\x86\x88\x03\x12\x15aW\xF2W__\xFD[\x855aW\xFD\x81aS`V[\x94P` \x86\x015aX\r\x81aS`V[\x93P`@\x86\x015aX\x1D\x81aS`V[\x92P``\x86\x015\x91P`\x80\x86\x015aX4\x81aW\xD0V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15aXVW__\xFD[\x855aXa\x81aS`V[\x94P` \x86\x015aXq\x81aS`V[\x93P`@\x86\x015aX\x81\x81aS`V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15aX\xA9W__\xFD[\x81Qa\x02H\x81aS`V[_` \x82\x84\x03\x12\x15aX\xC4W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01\xFAWa\x01\xFAaY\x16V[_`\x01`\xFF\x1B\x82\x01aYQWaYQaY\x16V[P_\x03\x90V[_` \x82\x84\x03\x12\x15aYgW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02HW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\x9FWaY\x9FaX\xEEV[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15aY\xB7W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xCDW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aY\xDDW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xF7WaY\xF7aX\xEEV[\x80`\x05\x1BaZ\x07` \x82\x01aYvV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aZ\"W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aZDW\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aZ)V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aZ_W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZuW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZ\x85W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x9FWaZ\x9FaX\xEEV[aZ\xB2`\x1F\x82\x01`\x1F\x19\x16` \x01aYvV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aZ\xC6W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aZ\xF3W__\xFD[\x81Qa\x02H\x81aW\xD0V[`\x01\x81[`\x01\x84\x11\x15a[9W\x80\x85\x04\x81\x11\x15a[\x1DWa[\x1DaY\x16V[`\x01\x84\x16\x15a[+W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a[\x02V[\x93P\x93\x91PPV[_\x82a[OWP`\x01a\x01\xFAV[\x81a[[WP_a\x01\xFAV[\x81`\x01\x81\x14a[qW`\x02\x81\x14a[{Wa[\x97V[`\x01\x91PPa\x01\xFAV[`\xFF\x84\x11\x15a[\x8CWa[\x8CaY\x16V[PP`\x01\x82\x1Ba\x01\xFAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a[\xBAWP\x81\x81\na\x01\xFAV[a[\xC6_\x19\x84\x84aZ\xFEV[\x80_\x19\x04\x82\x11\x15a[\xD9Wa[\xD9aY\x16V[\x02\x93\x92PPPV[_a\x02H\x83\x83a[AV[\x80\x82\x01\x80\x82\x11\x15a\x01\xFAWa\x01\xFAaY\x16V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a2\x01Wa2\x01aY\x16V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\xFAWa\x01\xFAaY\x16V[_\x82a\\cWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAC\xC8O\xDBB\x84\x9CH\x03D\xE9c\xC0\xF9\xB3o\xBC\n2\xDA\xD2E\xD6\x03\xECH\x06\x96\x06WI\xD0dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100a6575f3560e01c8063739118a41161006e578063739118a41461015657806378f212d1146101765780638f6c7a3c14610189578063d28b0a151461019c578063e335adb7146101ca578063f68a7131146101dd575f5ffd5b806328a0ccf4146100aa578063317b50ec146100da57806350ed592d146101025780635a6f5710146101235780635c39f46714610136575b5f5ffd5b6100bd6100b8366004615374565b6101f0565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ed6100e836600461538f565b610200565b604080519283526020830191909152016100d1565b610115610110366004615374565b61021b565b6040519081526020016100d1565b610115610131366004615374565b610225565b610149610144366004615374565b61022f565b6040516100d191906154e1565b6101696101643660046155ee565b61024f565b6040516100d191906156d8565b6100bd610184366004615374565b610272565b61014961019736600461579e565b61027c565b6101af6101aa3660046157de565b610289565b604080519384526020840192909252908201526060016100d1565b6100bd6101d8366004615374565b6102aa565b6101156101eb366004615842565b6102b4565b5f6101fa826102ce565b92915050565b5f5f61020e8686868661037f565b9150915094509492505050565b5f6101fa826103ba565b5f6101fa8261047e565b60605f61023b8361047e565b9050610248835f8361049d565b9392505050565b60605f61025c848461056b565b905061026a84845f846105e1565b949350505050565b5f6101fa826106b1565b606061026a84848461049d565b5f5f5f61029988888888886106ed565b925092509250955095509592505050565b5f6101fa826107dd565b5f6102c2868686868661082e565b90505b95945050505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161034091815260200190565b602060405180830381865afa15801561035b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa9190615899565b5f5f5f61038c8686610861565b90505f6103998883610908565b90505f5f6103a78388611b80565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043f91815260200190565b602060405180830381865afa15801561045a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa91906158b4565b5f816001600160a01b031663f3903b9f60405160200161040b906158cb565b60605f6104ab858585611d2f565b90505f815167ffffffffffffffff8111156104c8576104c86158ee565b60405190808252806020026020018201604052801561050157816020015b6104ee614ea4565b8152602001906001900390816104e65790505b5090505f5b8251811015610561575f83828151811061052257610522615902565b602002602001015190505f6105378983611dd0565b90508084848151811061054c5761054c615902565b60209081029190910101525050600101610506565b5095945050505050565b5f826001600160a01b031663f3903b9f61058484612222565b6040518263ffffffff1660e01b81526004016105a291815260200190565b602060405180830381865afa1580156105bd573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061024891906158b4565b60605f6105f0868686866122a6565b90505f815167ffffffffffffffff81111561060d5761060d6158ee565b60405190808252806020026020018201604052801561064657816020015b610633614f11565b81526020019060019003908161062b5790505b5090505f5b82518110156106a6575f83828151811061066757610667615902565b602002602001015190505f61067c8a8361232e565b90508084848151811061069157610691615902565b6020908102919091010152505060010161064b565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161030c90602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6106fb8888610861565b90505f6107088a83610908565b90505f808060ff891661073e576107208a855f612930565b92955091935061073791508590508b5f8087612a1a565b905061076e565b5f1960ff8a160161076e576107548a855f612ae7565b92955091935061076b91508590505f8c8682612a1a565b90505b5f61077885612bac565b90505f8282116107915761078c828461592a565b61079b565b61079b838361592a565b90505f6107a88284612c41565b90505f8484116107c0576107bb8261593d565b6107c2565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f61083a8686610861565b90505f6108478883610908565b90506108558186865f612c7c565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610882578183610885565b82825b60405191945092506108b2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610910614f64565b82610919614f64565b816001600160a01b03166391d4403c604051602001610937906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561098b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109af9190615957565b6109bc5791506101fa9050565b816001600160a01b03166321f8a721856040516020016109fc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a6091815260200190565b602060405180830381865afa158015610a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9f9190615899565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610b1d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b5191815260200190565b602060405180830381865afa158015610b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b9091906158b4565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610be6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4a91815260200190565b602060405180830381865afa158015610c65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c8991906158b4565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610cea9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4e91815260200190565b602060405180830381865afa158015610d69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8d91906158b4565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610df89060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610e28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e5c91815260200190565b602060405180830381865afa158015610e77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9b91906158b4565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610efc9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610f2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f6091815260200190565b602060405180830381865afa158015610f7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9f91906158b4565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161101c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161105091815260200190565b602060405180830381865afa15801561106b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061108f91906158b4565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611104929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113891815260200190565b602060405180830381865afa158015611153573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111779190615899565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161122191815260200190565b602060405180830381865afa15801561123c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061126091906158b4565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016112b790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016112e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131b91815260200190565b602060405180830381865afa158015611336573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061135a91906158b4565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016113bc9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161142091815260200190565b602060405180830381865afa15801561143b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145f91906158b4565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016114cb9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016114fb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161152f91815260200190565b602060405180830381865afa15801561154a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061156e91906158b4565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016115d09060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611600929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163491815260200190565b602060405180830381865afa15801561164f573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061167391906158b4565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016116cc90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161173091815260200190565b602060405180830381865afa15801561174b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061176f91906158b4565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016117bd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ed929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161182191815260200190565b602060405180830381865afa15801561183c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118609190615899565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016118ce906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161193291815260200190565b602060405180830381865afa15801561194d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119719190615899565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016119d4906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a04929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a3891815260200190565b602060405180830381865afa158015611a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a7791906158b4565b60608201526040516001600160a01b0383169063bd02d0f5908690611ad0906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611b00929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b3491815260200190565b602060405180830381865afa158015611b4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b7391906158b4565b6080820152949350505050565b5f5f5f5f611b8c614f98565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf091906158b4565b6020820152611bff875f612daf565b6080840152506040820152611c15876001612daf565b60a084015250606082015260408101516020820151611c35918891612e2d565b61010082015260608101516020820151611c50918891612e2d565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611d4f906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611da9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261026a91908101906159a7565b611dd8614ea4565b611de0614fe1565b611dea8484610908565b808252611df7905f612daf565b60608401526040830152602080830191909152815151805190910151611e28915f5b602002015160a0015190612eec565b60808201528051611e3a906001612daf565b60e084015260c083015260a08201528051516020818101510151611e5f916001611e19565b610100820152604080516103008101825282515151516001600160a01b039081166101808301908152845151515184516395d89b4160e01b81529451939485946101408601948594936101a08801939116916395d89b41916004808201925f929091908290030181865afa158015611ed9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611f009190810190615a4f565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015611f4d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f719190615ae3565b60ff16815286515151602090810151818301528751515160409081015181840152885151516060908101518185015289515151608090810151818601528a51515160a09081015190860152838b015160c0860152828b015160e0860152908a015161010085015289015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa15801561204e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120759190810190615a4f565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120c5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120e99190615ae3565b60ff168152865151602090810151810151818301528751518101516040908101518184015288515182015160609081015181850152895151830151608090810151818601528a515184015160a09081015181870152808c015160c080880191909152808d015160e0808901919091528d0151610100808901919091528d0151610120909701969096529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906121c890612bac565b81526020016121d78686612f2d565b6001600160a01b031681526020016121fe835f015160600151660800000000000016151590565b1515815260200161220f868661301d565b9052610120909101819052905092915050565b5f60405160200161225c906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a6122c086612222565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612307573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102c591908101906159a7565b612336614f11565b61233e615037565b612348848461311f565b80825251805151602090910151516123609190610861565b60408201819052612372908590610908565b602082018190528151612386918691613131565b505050506060820152602081015161239d90612bac565b610300820152805180515160209081015160e0840152808301515151015190516123d391905f5b60200201516040015190612eec565b60c08201526020810151606001516123eb905f613180565b60a082015260e081015160c082015161240491906131ae565b610100820181905260a082015161241b91906131cf565b610120820181905260e082015160c0830151612436926131eb565b6101408201526020810151815161030083015161245792879290915f613208565b61016082015260e081015160c08201516101008301516124789291906131eb565b6101808201526101408101516101a082015260208101516060015161249e906001613180565b6101c0820152805180516020908101518101516102008401528083015151810151015190516124cf919060016123c4565b6101e082018190526102008201516124e6916131ae565b61022082018190526101c08201516124fe91906131cf565b61024082018190526103008201516125169190612eec565b61026082018190526102008201516101e0830151612533926131eb565b6102808201526020810151815161030083015161255592879290916001613208565b6102a08201526102008101516101e08201516102208301516125789291906131eb565b6102c08201526102808101516102e082015280516125959061339d565b60808201528051516020015160e0015160021461268d576125bf81610300015182608001516131ae565b61032082018190526102408201516125d691612eec565b610340820181905260808201516103008301511161038083018190526102008301516101e0840151612607936133df565b61036082018190526103a082015260e081015160a082015161266e91869161262f91906131cf565b6126418460c001518560a001516131cf565b612655856102000151866101c001516131cf565b612669866101e00151876101c001516131cf565b613406565b6103c0820181905261030082015161268691906134c6565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612701573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127289190810190615a4f565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061277d5761277d615902565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106127cc576127cc615902565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa15801561282c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128539190810190615a4f565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f61293c615114565b612946875f612daf565b6040840152508152612959876001612daf565b6060840152506020820152851561297f5787815f0181815161297b919061592a565b9052505b606087015160381c61ffff1661012082018190526129ae9089906129a690612710906134df565b612710612e2d565b6101408201819052815160208301516129d1926129cc908390613539565b612e2d565b6080820181905260208201516129e6916134df565b60c0820181905260408201516060830151610120840151612a08908c9061358d565b94509450945094505093509350935093565b5f5f5f5f86118015612a2a575083155b15612a39575083905084612a6e565b5f87118015612a46575084155b15612a55575085905082612a6e565b604051636331fab160e01b815260040160405180910390fd5b5f612a7d89606001515f613180565b90505f612a8f8a606001516001613180565b90505f612aad85676765c793fa10079d601b1b6129cc86600a615be1565b90505f612acb85676765c793fa10079d601b1b6129cc86600a615be1565b9050612ad78282612c41565b9c9b505050505050505050505050565b5f5f5f5f612af3615114565b612afd875f612daf565b6040840152508152612b10876001612daf565b60608401525060208201528515612b37578781602001818151612b33919061592a565b9052505b80516020820151612b4d91906129cc818c613539565b608082018190528151612b5f916134df565b60a0820152606087015160381c61ffff16610120820181905260a0820151612b8e916129a690612710906134df565b6040820151606083015161012084015160a0850151612a089161358d565b5f5f612bb8835f612daf565b505090505f612bc8846001612daf565b50509050805f03612bdc57505f9392505050565b5f612beb85606001515f613180565b90505f612bfd86606001516001613180565b90505f612c1b85676765c793fa10079d601b1b6129cc86600a615be1565b90505f612c3985676765c793fa10079d601b1b6129cc86600a615be1565b905061085582825b5f8115676765c793fa10079d601b1b60028404190484111715612c62575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612c85614f98565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612cc5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce991906158b4565b6020820152612cf8865f612daf565b505060c0820152612d0a866001612daf565b505060e08201528215612d4457848160c001818151612d29919061592a565b90525060e081018051859190612d4090839061592a565b9052505b80602001515f03612d7457612d6d6103e8612d67612d6288886135b1565b613617565b906134df565b8152612da5565b612da2612d8a8683602001518460c00151612e2d565b612d9d8684602001518560e00151612e2d565b6136f7565b81525b5195945050505050565b5f5f5f5f855f01518560ff1660028110612dcb57612dcb615902565b602002015190505f612ddd878761370c565b9050805f03612df6575f5f5f9450945094505050612e26565b5f612e058389608001516137de565b9050612e118183615bec565b82612e1c838261592a565b9550955095505050505b9250925092565b5f838302815f1985870982811083820303915050805f03612e6157838281612e5757612e57615bff565b0492505050610248565b808411612e815760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f81156b019d971e4fe8401e740000001983900484111517612f0c575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f612f39848461380e565b9050806001600160a01b03166321f8a72184604051602001612f7a906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612faa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401612fde91815260200190565b602060405180830381865afa158015612ff9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a9190615899565b5f5f613029848461380e565b9050806001600160a01b031663bd02d0f58460405160200161307c9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016130ac929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016130e091815260200190565b602060405180830381865afa1580156130fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a91906158b4565b613127615163565b61024883836138c9565b5f5f5f5f5f61313f87612bac565b905061314d8787835f614ae7565b90935091508161315e575f19613168565b6131688383612c41565b945061317388614c53565b9350939792965093509350565b5f60ff60581b1960585f1960ff8516016131a0575060ff60601b19905060605b90198416901c905092915050565b5f8183116131c5576131c0838361592a565b610248565b610248828461592a565b5f61024883676765c793fa10079d601b1b6129cc85600a615be1565b5f828411613201576131fc8261593d565b61026a565b5092915050565b5f6132426040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61324e8686865f614ae7565b6020830152808252158061327f5750845160ff84166002811061327357613273615902565b6020020151602001515f145b1561328d575f9150506102c5565b61329687614ca4565b6040820181905260208201516132ab91612eec565b60808201819052815110156132c3575f9150506102c5565b608081015181516132d4919061592a565b8160600181815250506132eb866060015184613180565b60a0820181905260608201516133179161330690600a615be1565b676765c793fa10079d601b1b612e2d565b60c08201525f1960ff84160161333c5760c08101516133369085612c41565b60c08201525b845160ff84166002811061335257613352615902565b6020020151602001518160c00151111561338f57845160ff84166002811061337c5761337c615902565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f19016133bd575051602001516060015190565b81516020015160e001516133d8575051602001516080015190565b505f919050565b5f8480156133ec57508284115b6133fe576133f98261593d565b6102c5565b509392505050565b5f5f61341187614c53565b90505f61341e8287612eec565b90505f61342b8386612eec565b90505f6134388984615c13565b90505f6134458389615c13565b90505f61345183614cea565b90505f61345d83614cea565b90505f8413801561346d57505f83125b8061348157505f8412801561348157505f83135b15613495575f9750505050505050506102c5565b805f036134ab575f9750505050505050506102c5565b6134b58282612c41565b9d9c50505050505050505050505050565b5f82156134d7576131c08284612c41565b505f92915050565b5f826134eb838261592a565b91508111156101fa5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826135458382615bec565b91508110156101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401613530565b5f811561138819839004841115176135a3575f5ffd5b506127109102611388010490565b5f8115806135d4575082826135c68183615c32565b92506135d29083615c49565b145b6101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b6044820152606401613530565b5f815f0361362657505f919050565b5f600161363284614cff565b901c6001901b9050600181848161364b5761364b615bff565b048201901c9050600181848161366357613663615bff565b048201901c9050600181848161367b5761367b615bff565b048201901c9050600181848161369357613693615bff565b048201901c905060018184816136ab576136ab615bff565b048201901c905060018184816136c3576136c3615bff565b048201901c905060018184816136db576136db615bff565b048201901c9050610248818285816136f5576136f5615bff565b045b5f8183106137055781610248565b5090919050565b5f5f835f01518360ff166002811061372657613726615902565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa15801561377f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a391906158b4565b9050805f036137b6575f925050506101fa565b606082015160c08301516137ca828461592a565b6137d4919061592a565b9695505050505050565b5f8260a001515f036137f157505f6101fa565b5f6137fc8484614d92565b60a085015190915061026a9082612eec565b5f5f839050806001600160a01b03166391d4403c604051602001613831906158cb565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015613885573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138a99190615957565b61024857604051637357d91f60e01b815260048101849052602401613530565b6138d1615163565b826138da615163565b816001600160a01b03166391d4403c60405160200161391a906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561396e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139929190615957565b61399f5791506101fa9050565b816001600160a01b031663bd02d0f5856040516020016139d9906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001613a09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a3d91815260200190565b602060405180830381865afa158015613a58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a7c91906158b4565b816020018181525050816001600160a01b03166321f8a72185604051602001613ac4906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613af4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b2891815260200190565b602060405180830381865afa158015613b43573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b679190615899565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613bc3906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613bf3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c2791815260200190565b602060405180830381865afa158015613c42573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c669190615899565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613ce1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d1591815260200190565b602060405180830381865afa158015613d30573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d5491906158b4565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613da89060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001613dd8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e0c91815260200190565b602060405180830381865afa158015613e27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e4b91906158b4565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613ea5906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001613ed5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f0991815260200190565b602060405180830381865afa158015613f24573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f4891906158b4565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001613fa1906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613fd1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161400591815260200190565b602060405180830381865afa158015614020573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061404491906158b4565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016140ca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140fe91815260200190565b602060405180830381865afa158015614119573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061413d91906158b4565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614197906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016141c7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016141fb91815260200190565b602060405180830381865afa158015614216573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061423a91906158b4565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016142ad929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016142e191815260200190565b602060405180830381865afa1580156142fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061432091906158b4565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614394929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016143c891815260200190565b602060405180830381865afa1580156143e3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144079190615899565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144ae91815260200190565b602060405180830381865afa1580156144c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144ed91906158b4565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016145429060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001614572929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016145a691815260200190565b602060405180830381865afa1580156145c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906145e591906158b4565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161464090602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614670929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016146a491815260200190565b602060405180830381865afa1580156146bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146e391906158b4565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161473d90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161476d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016147a191815260200190565b602060405180830381865afa1580156147bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147e091906158b4565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016148429060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614872929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016148a691815260200190565b602060405180830381865afa1580156148c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906148e591906158b4565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161494090602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614970929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016149a491815260200190565b602060405180830381865afa1580156149bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149e391906158b4565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001614a32906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001614a62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a9691815260200190565b602060405180830381865afa158015614ab1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ad591906158b4565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614614b8a575f5f614b128a8a5f614dd8565b915091505f614b2e5f8c6060015161318090919063ffffffff16565b90505f614b4c84676765c793fa10079d601b1b6129cc85600a615be1565b90505f614b6a84676765c793fa10079d601b1b6129cc86600a615be1565b9050614b768288615bec565b9650614b828187615bec565b955050505050505b865160200151516001600160a01b03868116911614614c46575f5f614bb18a8a6001614dd8565b915091505f614bce60018c6060015161318090919063ffffffff16565b90505f614bec84676765c793fa10079d601b1b6129cc85600a615be1565b90505f614c0a84676765c793fa10079d601b1b6129cc86600a615be1565b90505f614c17838d612eec565b90505f614c24838e612eec565b9050614c30828a615bec565b9850614c3c8189615bec565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161040b9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215614cfb57815f036101fa565b5090565b5f80608083901c15614d1357608092831c92015b604083901c15614d2557604092831c92015b602083901c15614d3757602092831c92015b601083901c15614d4957601092831c92015b600883901c15614d5b57600892831c92015b600483901c15614d6d57600492831c92015b600283901c15614d7f57600292831c92015b600183901c156101fa5760010192915050565b5f428203614da5575060208201516101fa565b5f614db4846040015184614e70565b9050614dcd846020015182612eec90919063ffffffff16565b9150506101fa565b50565b5f5f5f845f01518460ff1660028110614df357614df3615902565b60200201516040015190505f614e29875f01518660ff1660028110614e1a57614e1a615902565b60200201518860800151614d92565b90508115614e4057614e3b8282612eec565b614e42565b5f5b865190935060ff861660028110614e5b57614e5b615902565b60200201516020015193505050935093915050565b5f80614e7c834261592a565b614e869085615c32565b6301e133809004905061026a81676765c793fa10079d601b1b615bec565b604051806101400160405280614eb8615189565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81525090565b604051806101200160405280614f25615210565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614f7761528b565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101400160405280614ff5614f64565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615032614ea4565b905290565b60405180610400016040528061504b615163565b8152602001615058614f64565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806151766152f2565b81525f6020820181905260409091015290565b60405180604001604052806002905b6151fa6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816151985790505090565b60405180604001604052806002905b6152756040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161521f5790505090565b60405180604001604052806002905b6152dc6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161529a5790505090565b60405180604001604052806002905b61534a6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816153015790505090565b6001600160a01b0381168114614dd5575f5ffd5b5f60208284031215615384575f5ffd5b813561024881615360565b5f5f5f5f608085870312156153a2575f5ffd5b84356153ad81615360565b935060208501356153bd81615360565b925060408501356153cd81615360565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b60028110156154d6578383038752815180516001600160a01b031684526020810151610180602086015261544c6101808601826153dd565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615414565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156155e257603f1987860301845281518051610140875261552f61014088018261540b565b9050602082015161554b60208901826001600160a01b03169052565b50604082015161556660408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e08201516155a960e08901826001600160a01b03169052565b506101008201516155bf61010089018215159052565b506101209182015196909101959095526020938401939190910190600101615507565b50929695505050505050565b5f5f604083850312156155ff575f5ffd5b823561560a81615360565b9150602083013561561a81615360565b809150509250929050565b5f8260408101835f5b60028110156154d6578383038752815180516001600160a01b03168452602081015161014060208601526156666101408601826153dd565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e0860152610100820151610100860152610120820151610120860152809450505060208201915060208701965060018101905061562e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156155e257603f19878603018452815180516101208752615726610120880182615625565b905060208201516020880152604082015161574c60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e0808301519088015261010091820151919096015260209384019391909101906001016156fe565b5f5f5f606084860312156157b0575f5ffd5b83356157bb81615360565b95602085013595506040909401359392505050565b60ff81168114614dd5575f5ffd5b5f5f5f5f5f60a086880312156157f2575f5ffd5b85356157fd81615360565b9450602086013561580d81615360565b9350604086013561581d81615360565b9250606086013591506080860135615834816157d0565b809150509295509295909350565b5f5f5f5f5f60a08688031215615856575f5ffd5b853561586181615360565b9450602086013561587181615360565b9350604086013561588181615360565b94979396509394606081013594506080013592915050565b5f602082840312156158a9575f5ffd5b815161024881615360565b5f602082840312156158c4575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156101fa576101fa615916565b5f600160ff1b820161595157615951615916565b505f0390565b5f60208284031215615967575f5ffd5b81518015158114610248575f5ffd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561599f5761599f6158ee565b604052919050565b5f602082840312156159b7575f5ffd5b815167ffffffffffffffff8111156159cd575f5ffd5b8201601f810184136159dd575f5ffd5b805167ffffffffffffffff8111156159f7576159f76158ee565b8060051b615a0760208201615976565b91825260208184018101929081019087841115615a22575f5ffd5b6020850194505b83851015615a44578451825260209485019490910190615a29565b979650505050505050565b5f60208284031215615a5f575f5ffd5b815167ffffffffffffffff811115615a75575f5ffd5b8201601f81018413615a85575f5ffd5b805167ffffffffffffffff811115615a9f57615a9f6158ee565b615ab2601f8201601f1916602001615976565b818152856020838501011115615ac6575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215615af3575f5ffd5b8151610248816157d0565b6001815b6001841115615b3957808504811115615b1d57615b1d615916565b6001841615615b2b57908102905b60019390931c928002615b02565b935093915050565b5f82615b4f575060016101fa565b81615b5b57505f6101fa565b8160018114615b715760028114615b7b57615b97565b60019150506101fa565b60ff841115615b8c57615b8c615916565b50506001821b6101fa565b5060208310610133831016604e8410600b8410161715615bba575081810a6101fa565b615bc65f198484615afe565b805f1904821115615bd957615bd9615916565b029392505050565b5f6102488383615b41565b808201808211156101fa576101fa615916565b634e487b7160e01b5f52601260045260245ffd5b8181035f83128015838313168383128216171561320157613201615916565b80820281158282048414176101fa576101fa615916565b5f82615c6357634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220acc84fdb42849c480344e963c0f9b36fbc0a32dad245d603ec480696065749d064736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01VW\x80cx\xF2\x12\xD1\x14a\x01vW\x80c\x8Flz<\x14a\x01\x89W\x80c\xD2\x8B\n\x15\x14a\x01\x9CW\x80c\xE35\xAD\xB7\x14a\x01\xCAW\x80c\xF6\x8Aq1\x14a\x01\xDDW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xAAW\x80c1{P\xEC\x14a\0\xDAW\x80cP\xEDY-\x14a\x01\x02W\x80cZoW\x10\x14a\x01#W\x80c\\9\xF4g\x14a\x016W[__\xFD[a\0\xBDa\0\xB86`\x04aStV[a\x01\xF0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEDa\0\xE86`\x04aS\x8FV[a\x02\0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD1V[a\x01\x15a\x01\x106`\x04aStV[a\x02\x1BV[`@Q\x90\x81R` \x01a\0\xD1V[a\x01\x15a\x0116`\x04aStV[a\x02%V[a\x01Ia\x01D6`\x04aStV[a\x02/V[`@Qa\0\xD1\x91\x90aT\xE1V[a\x01ia\x01d6`\x04aU\xEEV[a\x02OV[`@Qa\0\xD1\x91\x90aV\xD8V[a\0\xBDa\x01\x846`\x04aStV[a\x02rV[a\x01Ia\x01\x976`\x04aW\x9EV[a\x02|V[a\x01\xAFa\x01\xAA6`\x04aW\xDEV[a\x02\x89V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD1V[a\0\xBDa\x01\xD86`\x04aStV[a\x02\xAAV[a\x01\x15a\x01\xEB6`\x04aXBV[a\x02\xB4V[_a\x01\xFA\x82a\x02\xCEV[\x92\x91PPV[__a\x02\x0E\x86\x86\x86\x86a\x03\x7FV[\x91P\x91P\x94P\x94\x92PPPV[_a\x01\xFA\x82a\x03\xBAV[_a\x01\xFA\x82a\x04~V[``_a\x02;\x83a\x04~V[\x90Pa\x02H\x83_\x83a\x04\x9DV[\x93\x92PPPV[``_a\x02\\\x84\x84a\x05kV[\x90Pa\x02j\x84\x84_\x84a\x05\xE1V[\x94\x93PPPPV[_a\x01\xFA\x82a\x06\xB1V[``a\x02j\x84\x84\x84a\x04\x9DV[___a\x02\x99\x88\x88\x88\x88\x88a\x06\xEDV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x01\xFA\x82a\x07\xDDV[_a\x02\xC2\x86\x86\x86\x86\x86a\x08.V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aX\x99V[___a\x03\x8C\x86\x86a\x08aV[\x90P_a\x03\x99\x88\x83a\t\x08V[\x90P__a\x03\xA7\x83\x88a\x1B\x80V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aX\xB4V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04\x0B\x90aX\xCBV[``_a\x04\xAB\x85\x85\x85a\x1D/V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC8Wa\x04\xC8aX\xEEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x01W\x81` \x01[a\x04\xEEaN\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\xE6W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x05aW_\x83\x82\x81Q\x81\x10a\x05\"Wa\x05\"aY\x02V[` \x02` \x01\x01Q\x90P_a\x057\x89\x83a\x1D\xD0V[\x90P\x80\x84\x84\x81Q\x81\x10a\x05LWa\x05LaY\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x06V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x05\x84\x84a\"\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02H\x91\x90aX\xB4V[``_a\x05\xF0\x86\x86\x86\x86a\"\xA6V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\raX\xEEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06FW\x81` \x01[a\x063aO\x11V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06+W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xA6W_\x83\x82\x81Q\x81\x10a\x06gWa\x06gaY\x02V[` \x02` \x01\x01Q\x90P_a\x06|\x8A\x83a#.V[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\x91Wa\x06\x91aY\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06KV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x06\xFB\x88\x88a\x08aV[\x90P_a\x07\x08\x8A\x83a\t\x08V[\x90P_\x80\x80`\xFF\x89\x16a\x07>Wa\x07 \x8A\x85_a)0V[\x92\x95P\x91\x93Pa\x077\x91P\x85\x90P\x8B_\x80\x87a*\x1AV[\x90Pa\x07nV[_\x19`\xFF\x8A\x16\x01a\x07nWa\x07T\x8A\x85_a*\xE7V[\x92\x95P\x91\x93Pa\x07k\x91P\x85\x90P_\x8C\x86\x82a*\x1AV[\x90P[_a\x07x\x85a+\xACV[\x90P_\x82\x82\x11a\x07\x91Wa\x07\x8C\x82\x84aY*V[a\x07\x9BV[a\x07\x9B\x83\x83aY*V[\x90P_a\x07\xA8\x82\x84a,AV[\x90P_\x84\x84\x11a\x07\xC0Wa\x07\xBB\x82aY=V[a\x07\xC2V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08:\x86\x86a\x08aV[\x90P_a\x08G\x88\x83a\t\x08V[\x90Pa\x08U\x81\x86\x86_a,|V[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x82W\x81\x83a\x08\x85V[\x82\x82[`@Q\x91\x94P\x92Pa\x08\xB2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\x10aOdV[\x82a\t\x19aOdV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t7\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aYWV[a\t\xBCW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xFC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90aX\x99V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BQ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x90\x91\x90aX\xB4V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xE6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xEA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90aX\xB4V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\xF8\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\xFC\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9F\x91\x90aX\xB4V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10P\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8F\x91\x90aX\xB4V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x118\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11w\x91\x90aX\x99V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB7\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x136W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Z\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xBC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14 \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xCB\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xD0\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x164\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16s\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\xCC\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17o\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xBD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18`\x91\x90aX\x99V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18\xCE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x192\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19q\x91\x90aX\x99V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xD4\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ASW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Aw\x91\x90aX\xB4V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1A\xD0\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bs\x91\x90aX\xB4V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1B\x8CaO\x98V[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF0\x91\x90aX\xB4V[` \x82\x01Ra\x1B\xFF\x87_a-\xAFV[`\x80\x84\x01RP`@\x82\x01Ra\x1C\x15\x87`\x01a-\xAFV[`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C5\x91\x88\x91a.-V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1CP\x91\x88\x91a.-V[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1DO\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02j\x91\x90\x81\x01\x90aY\xA7V[a\x1D\xD8aN\xA4V[a\x1D\xE0aO\xE1V[a\x1D\xEA\x84\x84a\t\x08V[\x80\x82Ra\x1D\xF7\x90_a-\xAFV[``\x84\x01R`@\x83\x01R` \x80\x83\x01\x91\x90\x91R\x81QQ\x80Q\x90\x91\x01Qa\x1E(\x91_[` \x02\x01Q`\xA0\x01Q\x90a.\xECV[`\x80\x82\x01R\x80Qa\x1E:\x90`\x01a-\xAFV[`\xE0\x84\x01R`\xC0\x83\x01R`\xA0\x82\x01R\x80QQ` \x81\x81\x01Q\x01Qa\x1E_\x91`\x01a\x1E\x19V[a\x01\0\x82\x01R`@\x80Qa\x03\0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\x80\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01@\x86\x01\x94\x85\x94\x93a\x01\xA0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xD9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\0\x91\x90\x81\x01\x90aZOV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fq\x91\x90aZ\xE3V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R\x87QQQ`@\x90\x81\x01Q\x81\x84\x01R\x88QQQ``\x90\x81\x01Q\x81\x85\x01R\x89QQQ`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\xA0\x90\x81\x01Q\x90\x86\x01R\x83\x8B\x01Q`\xC0\x86\x01R\x82\x8B\x01Q`\xE0\x86\x01R\x90\x8A\x01Qa\x01\0\x85\x01R\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a NW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra u\x91\x90\x81\x01\x90aZOV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xE9\x91\x90aZ\xE3V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01R\x87QQ\x81\x01Q`@\x90\x81\x01Q\x81\x84\x01R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R\x80\x8C\x01Q`\xC0\x80\x88\x01\x91\x90\x91R\x80\x8D\x01Q`\xE0\x80\x89\x01\x91\x90\x91R\x8D\x01Qa\x01\0\x80\x89\x01\x91\x90\x91R\x8D\x01Qa\x01 \x90\x97\x01\x96\x90\x96R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a!\xC8\x90a+\xACV[\x81R` \x01a!\xD7\x86\x86a/-V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a!\xFE\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a\"\x0F\x86\x86a0\x1DV[\x90Ra\x01 \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a\"\\\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a\"\xC0\x86a\"\"V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x07W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xC5\x91\x90\x81\x01\x90aY\xA7V[a#6aO\x11V[a#>aP7V[a#H\x84\x84a1\x1FV[\x80\x82RQ\x80QQ` \x90\x91\x01QQa#`\x91\x90a\x08aV[`@\x82\x01\x81\x90Ra#r\x90\x85\x90a\t\x08V[` \x82\x01\x81\x90R\x81Qa#\x86\x91\x86\x91a11V[PPPP``\x82\x01R` \x81\x01Qa#\x9D\x90a+\xACV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa#\xD3\x91\x90_[` \x02\x01Q`@\x01Q\x90a.\xECV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa#\xEB\x90_a1\x80V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa$\x04\x91\x90a1\xAEV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa$\x1B\x91\x90a1\xCFV[a\x01 \x82\x01\x81\x90R`\xE0\x82\x01Q`\xC0\x83\x01Qa$6\x92a1\xEBV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa$W\x92\x87\x92\x90\x91_a2\x08V[a\x01`\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa$x\x92\x91\x90a1\xEBV[a\x01\x80\x82\x01Ra\x01@\x81\x01Qa\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa$\x9E\x90`\x01a1\x80V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa$\xCF\x91\x90`\x01a#\xC4V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa$\xE6\x91a1\xAEV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa$\xFE\x91\x90a1\xCFV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa%\x16\x91\x90a.\xECV[a\x02`\x82\x01\x81\x90Ra\x02\0\x82\x01Qa\x01\xE0\x83\x01Qa%3\x92a1\xEBV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%U\x92\x87\x92\x90\x91`\x01a2\x08V[a\x02\xA0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa%x\x92\x91\x90a1\xEBV[a\x02\xC0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xE0\x82\x01R\x80Qa%\x95\x90a3\x9DV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a&\x8DWa%\xBF\x81a\x03\0\x01Q\x82`\x80\x01Qa1\xAEV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa%\xD6\x91a.\xECV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa&\x07\x93a3\xDFV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa&n\x91\x86\x91a&/\x91\x90a1\xCFV[a&A\x84`\xC0\x01Q\x85`\xA0\x01Qa1\xCFV[a&U\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa1\xCFV[a&i\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa1\xCFV[a4\x06V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&\x86\x91\x90a4\xC6V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\x01W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'(\x91\x90\x81\x01\x90aZOV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a'}Wa'}aY\x02V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a'\xCCWa'\xCCaY\x02V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(,W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(S\x91\x90\x81\x01\x90aZOV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a)<aQ\x14V[a)F\x87_a-\xAFV[`@\x84\x01RP\x81Ra)Y\x87`\x01a-\xAFV[``\x84\x01RP` \x82\x01R\x85\x15a)\x7FW\x87\x81_\x01\x81\x81Qa){\x91\x90aY*V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra)\xAE\x90\x89\x90a)\xA6\x90a'\x10\x90a4\xDFV[a'\x10a.-V[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa)\xD1\x92a)\xCC\x90\x83\x90a59V[a.-V[`\x80\x82\x01\x81\x90R` \x82\x01Qa)\xE6\x91a4\xDFV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa*\x08\x90\x8C\x90a5\x8DV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a**WP\x83\x15[\x15a*9WP\x83\x90P\x84a*nV[_\x87\x11\x80\x15a*FWP\x84\x15[\x15a*UWP\x85\x90P\x82a*nV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*}\x89``\x01Q_a1\x80V[\x90P_a*\x8F\x8A``\x01Q`\x01a1\x80V[\x90P_a*\xAD\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_a*\xCB\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90Pa*\xD7\x82\x82a,AV[\x9C\x9BPPPPPPPPPPPPV[____a*\xF3aQ\x14V[a*\xFD\x87_a-\xAFV[`@\x84\x01RP\x81Ra+\x10\x87`\x01a-\xAFV[``\x84\x01RP` \x82\x01R\x85\x15a+7W\x87\x81` \x01\x81\x81Qa+3\x91\x90aY*V[\x90RP[\x80Q` \x82\x01Qa+M\x91\x90a)\xCC\x81\x8Ca59V[`\x80\x82\x01\x81\x90R\x81Qa+_\x91a4\xDFV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa+\x8E\x91a)\xA6\x90a'\x10\x90a4\xDFV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa*\x08\x91a5\x8DV[__a+\xB8\x83_a-\xAFV[PP\x90P_a+\xC8\x84`\x01a-\xAFV[PP\x90P\x80_\x03a+\xDCWP_\x93\x92PPPV[_a+\xEB\x85``\x01Q_a1\x80V[\x90P_a+\xFD\x86``\x01Q`\x01a1\x80V[\x90P_a,\x1B\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_a,9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90Pa\x08U\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a,bW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a,\x85aO\x98V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE9\x91\x90aX\xB4V[` \x82\x01Ra,\xF8\x86_a-\xAFV[PP`\xC0\x82\x01Ra-\n\x86`\x01a-\xAFV[PP`\xE0\x82\x01R\x82\x15a-DW\x84\x81`\xC0\x01\x81\x81Qa-)\x91\x90aY*V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a-@\x90\x83\x90aY*V[\x90RP[\x80` \x01Q_\x03a-tWa-ma\x03\xE8a-ga-b\x88\x88a5\xB1V[a6\x17V[\x90a4\xDFV[\x81Ra-\xA5V[a-\xA2a-\x8A\x86\x83` \x01Q\x84`\xC0\x01Qa.-V[a-\x9D\x86\x84` \x01Q\x85`\xE0\x01Qa.-V[a6\xF7V[\x81R[Q\x95\x94PPPPPV[____\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a-\xCBWa-\xCBaY\x02V[` \x02\x01Q\x90P_a-\xDD\x87\x87a7\x0CV[\x90P\x80_\x03a-\xF6W___\x94P\x94P\x94PPPa.&V[_a.\x05\x83\x89`\x80\x01Qa7\xDEV[\x90Pa.\x11\x81\x83a[\xECV[\x82a.\x1C\x83\x82aY*V[\x95P\x95P\x95PPPP[\x92P\x92P\x92V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a.aW\x83\x82\x81a.WWa.Wa[\xFFV[\x04\x92PPPa\x02HV[\x80\x84\x11a.\x81W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a/\x0CW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[__a/9\x84\x84a8\x0EV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a/z\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aX\x99V[__a0)\x84\x84a8\x0EV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a0|\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\xAC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\xE0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aX\xB4V[a1'aQcV[a\x02H\x83\x83a8\xC9V[_____a1?\x87a+\xACV[\x90Pa1M\x87\x87\x83_aJ\xE7V[\x90\x93P\x91P\x81a1^W_\x19a1hV[a1h\x83\x83a,AV[\x94Pa1s\x88aLSV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a1\xA0WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a1\xC5Wa1\xC0\x83\x83aY*V[a\x02HV[a\x02H\x82\x84aY*V[_a\x02H\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[_\x82\x84\x11a2\x01Wa1\xFC\x82aY=V[a\x02jV[P\x92\x91PPV[_a2B`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a2N\x86\x86\x86_aJ\xE7V[` \x83\x01R\x80\x82R\x15\x80a2\x7FWP\x84Q`\xFF\x84\x16`\x02\x81\x10a2sWa2saY\x02V[` \x02\x01Q` \x01Q_\x14[\x15a2\x8DW_\x91PPa\x02\xC5V[a2\x96\x87aL\xA4V[`@\x82\x01\x81\x90R` \x82\x01Qa2\xAB\x91a.\xECV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a2\xC3W_\x91PPa\x02\xC5V[`\x80\x81\x01Q\x81Qa2\xD4\x91\x90aY*V[\x81``\x01\x81\x81RPPa2\xEB\x86``\x01Q\x84a1\x80V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa3\x17\x91a3\x06\x90`\na[\xE1V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba.-V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a3<W`\xC0\x81\x01Qa36\x90\x85a,AV[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a3RWa3RaY\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a3\x8FW\x84Q`\xFF\x84\x16`\x02\x81\x10a3|Wa3|aY\x02V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a3\xBDWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa3\xD8WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x80\x15a3\xECWP\x82\x84\x11[a3\xFEWa3\xF9\x82aY=V[a\x02\xC5V[P\x93\x92PPPV[__a4\x11\x87aLSV[\x90P_a4\x1E\x82\x87a.\xECV[\x90P_a4+\x83\x86a.\xECV[\x90P_a48\x89\x84a\\\x13V[\x90P_a4E\x83\x89a\\\x13V[\x90P_a4Q\x83aL\xEAV[\x90P_a4]\x83aL\xEAV[\x90P_\x84\x13\x80\x15a4mWP_\x83\x12[\x80a4\x81WP_\x84\x12\x80\x15a4\x81WP_\x83\x13[\x15a4\x95W_\x97PPPPPPPPa\x02\xC5V[\x80_\x03a4\xABW_\x97PPPPPPPPa\x02\xC5V[a4\xB5\x82\x82a,AV[\x9D\x9CPPPPPPPPPPPPPV[_\x82\x15a4\xD7Wa1\xC0\x82\x84a,AV[P_\x92\x91PPV[_\x82a4\xEB\x83\x82aY*V[\x91P\x81\x11\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a5E\x83\x82a[\xECV[\x91P\x81\x10\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a50V[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a5\xA3W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a5\xD4WP\x82\x82a5\xC6\x81\x83a\\2V[\x92Pa5\xD2\x90\x83a\\IV[\x14[a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a50V[_\x81_\x03a6&WP_\x91\x90PV[_`\x01a62\x84aL\xFFV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a6KWa6Ka[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6cWa6ca[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6{Wa6{a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\x93Wa6\x93a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xABWa6\xABa[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xC3Wa6\xC3a[\xFFV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xDBWa6\xDBa[\xFFV[\x04\x82\x01\x90\x1C\x90Pa\x02H\x81\x82\x85\x81a6\xF5Wa6\xF5a[\xFFV[\x04[_\x81\x83\x10a7\x05W\x81a\x02HV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a7&Wa7&aY\x02V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA3\x91\x90aX\xB4V[\x90P\x80_\x03a7\xB6W_\x92PPPa\x01\xFAV[``\x82\x01Q`\xC0\x83\x01Qa7\xCA\x82\x84aY*V[a7\xD4\x91\x90aY*V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a7\xF1WP_a\x01\xFAV[_a7\xFC\x84\x84aM\x92V[`\xA0\x85\x01Q\x90\x91Pa\x02j\x90\x82a.\xECV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a81\x90aX\xCBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xA9\x91\x90aYWV[a\x02HW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a50V[a8\xD1aQcV[\x82a8\xDAaQcV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a9\x1A\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x92\x91\x90aYWV[a9\x9FW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a9\xD9\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:|\x91\x90aX\xB4V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a:\xC4\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;(\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;g\x91\x90aX\x99V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a;\xC3\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<f\x91\x90aX\x99V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=T\x91\x90aX\xB4V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a=\xA8\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xD8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>K\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\xA5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xD5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?$W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?H\x91\x90aX\xB4V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\xA1\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@ W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@D\x91\x90aX\xB4V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA=\x91\x90aX\xB4V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aA\x97\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\xFB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB:\x91\x90aX\xB4V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xAD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xE1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC \x91\x90aX\xB4V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aC\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x07\x91\x90aX\x99V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xED\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aEB\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aEr\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xE5\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF@\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aFp\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xE3\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG=\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aGm\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\xA1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xE0\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aHB\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aHr\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\xA6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH\xE5\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI@\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aIp\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xE3\x91\x90aX\xB4V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ2\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJb\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xD5\x91\x90aX\xB4V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aK\x8AW__aK\x12\x8A\x8A_aM\xD8V[\x91P\x91P_aK._\x8C``\x01Qa1\x80\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aKL\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[\x90P_aKj\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90PaKv\x82\x88a[\xECV[\x96PaK\x82\x81\x87a[\xECV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aLFW__aK\xB1\x8A\x8A`\x01aM\xD8V[\x91P\x91P_aK\xCE`\x01\x8C``\x01Qa1\x80\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aK\xEC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x85`\na[\xE1V[\x90P_aL\n\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xCC\x86`\na[\xE1V[\x90P_aL\x17\x83\x8Da.\xECV[\x90P_aL$\x83\x8Ea.\xECV[\x90PaL0\x82\x8Aa[\xECV[\x98PaL<\x81\x89a[\xECV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aL\xFBW\x81_\x03a\x01\xFAV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aM\x13W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aM%W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aM7W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aMIW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aM[W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aMmW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aM\x7FW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x01\xFAW`\x01\x01\x92\x91PPV[_B\x82\x03aM\xA5WP` \x82\x01Qa\x01\xFAV[_aM\xB4\x84`@\x01Q\x84aNpV[\x90PaM\xCD\x84` \x01Q\x82a.\xEC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x01\xFAV[PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aM\xF3WaM\xF3aY\x02V[` \x02\x01Q`@\x01Q\x90P_aN)\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aN\x1AWaN\x1AaY\x02V[` \x02\x01Q\x88`\x80\x01QaM\x92V[\x90P\x81\x15aN@WaN;\x82\x82a.\xECV[aNBV[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aN[WaN[aY\x02V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aN|\x83BaY*V[aN\x86\x90\x85a\\2V[c\x01\xE13\x80\x90\x04\x90Pa\x02j\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba[\xECV[`@Q\x80a\x01@\x01`@R\x80aN\xB8aQ\x89V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aO%aR\x10V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aOwaR\x8BV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01@\x01`@R\x80aO\xF5aOdV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aP2aN\xA4V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aPKaQcV[\x81R` \x01aPXaOdV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aQvaR\xF2V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aQ\xFA`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aQ\x98W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aRu`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x1FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aR\xDC`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x9AW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aSJ`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aS\x01W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aM\xD5W__\xFD[_` \x82\x84\x03\x12\x15aS\x84W__\xFD[\x815a\x02H\x81aS`V[____`\x80\x85\x87\x03\x12\x15aS\xA2W__\xFD[\x845aS\xAD\x81aS`V[\x93P` \x85\x015aS\xBD\x81aS`V[\x92P`@\x85\x015aS\xCD\x81aS`V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aT\xD6W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01RaTLa\x01\x80\x86\x01\x82aS\xDDV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaT\x14V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU\xE2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01@\x87RaU/a\x01@\x88\x01\x82aT\x0BV[\x90P` \x82\x01QaUK` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QaUf`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01QaU\xA9`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01QaU\xBFa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aU\x07V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aU\xFFW__\xFD[\x825aV\n\x81aS`V[\x91P` \x83\x015aV\x1A\x81aS`V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aT\xD6W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01RaVfa\x01@\x86\x01\x82aS\xDDV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaV.V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aU\xE2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RaW&a\x01 \x88\x01\x82aV%V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QaWL`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aV\xFEV[___``\x84\x86\x03\x12\x15aW\xB0W__\xFD[\x835aW\xBB\x81aS`V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14aM\xD5W__\xFD[_____`\xA0\x86\x88\x03\x12\x15aW\xF2W__\xFD[\x855aW\xFD\x81aS`V[\x94P` \x86\x015aX\r\x81aS`V[\x93P`@\x86\x015aX\x1D\x81aS`V[\x92P``\x86\x015\x91P`\x80\x86\x015aX4\x81aW\xD0V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15aXVW__\xFD[\x855aXa\x81aS`V[\x94P` \x86\x015aXq\x81aS`V[\x93P`@\x86\x015aX\x81\x81aS`V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15aX\xA9W__\xFD[\x81Qa\x02H\x81aS`V[_` \x82\x84\x03\x12\x15aX\xC4W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01\xFAWa\x01\xFAaY\x16V[_`\x01`\xFF\x1B\x82\x01aYQWaYQaY\x16V[P_\x03\x90V[_` \x82\x84\x03\x12\x15aYgW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02HW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\x9FWaY\x9FaX\xEEV[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15aY\xB7W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xCDW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aY\xDDW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aY\xF7WaY\xF7aX\xEEV[\x80`\x05\x1BaZ\x07` \x82\x01aYvV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aZ\"W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aZDW\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aZ)V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aZ_W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZuW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZ\x85W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\x9FWaZ\x9FaX\xEEV[aZ\xB2`\x1F\x82\x01`\x1F\x19\x16` \x01aYvV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aZ\xC6W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aZ\xF3W__\xFD[\x81Qa\x02H\x81aW\xD0V[`\x01\x81[`\x01\x84\x11\x15a[9W\x80\x85\x04\x81\x11\x15a[\x1DWa[\x1DaY\x16V[`\x01\x84\x16\x15a[+W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a[\x02V[\x93P\x93\x91PPV[_\x82a[OWP`\x01a\x01\xFAV[\x81a[[WP_a\x01\xFAV[\x81`\x01\x81\x14a[qW`\x02\x81\x14a[{Wa[\x97V[`\x01\x91PPa\x01\xFAV[`\xFF\x84\x11\x15a[\x8CWa[\x8CaY\x16V[PP`\x01\x82\x1Ba\x01\xFAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a[\xBAWP\x81\x81\na\x01\xFAV[a[\xC6_\x19\x84\x84aZ\xFEV[\x80_\x19\x04\x82\x11\x15a[\xD9Wa[\xD9aY\x16V[\x02\x93\x92PPPV[_a\x02H\x83\x83a[AV[\x80\x82\x01\x80\x82\x11\x15a\x01\xFAWa\x01\xFAaY\x16V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a2\x01Wa2\x01aY\x16V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\xFAWa\x01\xFAaY\x16V[_\x82a\\cWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAC\xC8O\xDBB\x84\x9CH\x03D\xE9c\xC0\xF9\xB3o\xBC\n2\xDA\xD2E\xD6\x03\xECH\x06\x96\x06WI\xD0dsolcC\0\x08\x1C\x003",
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
        calcAmountOut(calcAmountOutCall),
        calcLiquidityOut(calcLiquidityOutCall),
        calcTokenPairOut(calcTokenPairOutCall),
        getDefaultInterestRateStrategy(getDefaultInterestRateStrategyCall),
        getDefaultPoolConfiguration(getDefaultPoolConfigurationCall),
        getPools_0(getPools_0Call),
        getPools_1(getPools_1Call),
        getPoolsCount(getPoolsCountCall),
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
            [40u8, 160u8, 204u8, 244u8],
            [49u8, 123u8, 80u8, 236u8],
            [80u8, 237u8, 89u8, 45u8],
            [90u8, 111u8, 87u8, 16u8],
            [92u8, 57u8, 244u8, 103u8],
            [115u8, 145u8, 24u8, 164u8],
            [120u8, 242u8, 18u8, 209u8],
            [143u8, 108u8, 122u8, 60u8],
            [210u8, 139u8, 10u8, 21u8],
            [227u8, 53u8, 173u8, 183u8],
            [246u8, 138u8, 113u8, 49u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ReaderCalls {
        const NAME: &'static str = "ReaderCalls";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::getPools_0(_) => {
                    <getPools_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPools_1(_) => {
                    <getPools_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPoolsCount(_) => {
                    <getPoolsCountCall as alloy_sol_types::SolCall>::SELECTOR
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
