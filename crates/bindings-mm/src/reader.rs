///Module containing a contract's types and functions.
/**

```solidity
library ReaderPoolUtils {
    struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLoan; uint256 actualTradable; }
    struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; uint256 unclaimedFee; }
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
struct GetPool { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; uint256 priceDecimals; uint256 price; address source; bool shortEnabled; uint256 createdTimestamp; uint256 unclaimedFee; }
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
                    "GetPool(Asset[2] assets,address bank,address interestRateStrategy,uint256 configuration,uint256 lastUpdateTimestamp,uint256 priceDecimals,uint256 price,address source,bool shortEnabled,uint256 createdTimestamp,uint256 unclaimedFee)",
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

    function calcAmountOut(address dataStore, address token0, address token1, uint256 amountIn, uint8 tokenInIndex) external view returns (uint256, uint256, int256);
    function calcLiquidityOut(address dataStore, address token0, address token1, uint256 amount0, uint256 amount1) external view returns (uint256);
    function calcTokenPairOut(address dataStore, address token0, address token1, uint256 liquidity) external view returns (uint256, uint256);
    function getDefaultInterestRateStrategy(address dataStore) external view returns (address);
    function getDefaultPoolConfiguration(address dataStore) external view returns (uint256);
    function getMarginLevelThreshold(address dataStore) external view returns (uint256);
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
    ///0x6080604052348015600e575f5ffd5b5061641e8061001c5f395ff3fe608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c8063739118a41161006e578063739118a41461017457806378f212d1146101945780638f6c7a3c146101a7578063d28b0a15146101ba578063e335adb7146101e8578063f68a7131146101fb575f5ffd5b806328a0ccf4146100b5578063317b50ec146100e557806350ed592d1461010d57806357b91ca61461012e5780635a6f5710146101415780635c39f46714610154575b5f5ffd5b6100c86100c3366004615a7f565b61020e565b6040516001600160a01b0390911681526020015b60405180910390f35b6100f86100f3366004615a9a565b61021e565b604080519283526020830191909152016100dc565b61012061011b366004615a7f565b610239565b6040519081526020016100dc565b61012061013c366004615a7f565b610243565b61012061014f366004615a7f565b61024d565b610167610162366004615a7f565b610257565b6040516100dc9190615bec565b610187610182366004615d01565b610277565b6040516100dc9190615deb565b6100c86101a2366004615a7f565b61029a565b6101676101b5366004615eb1565b6102a4565b6101cd6101c8366004615ef1565b6102b1565b604080519384526020840192909252908201526060016100dc565b6100c86101f6366004615a7f565b6102d2565b610120610209366004615f55565b6102dc565b5f610218826102f6565b92915050565b5f5f61022c868686866103a7565b9150915094509492505050565b5f610218826103e2565b5f610218826104a6565b5f610218826104f7565b60605f610263836104f7565b9050610270835f83610516565b9392505050565b60605f610284848461060d565b905061029284845f84610683565b949350505050565b5f61021882610753565b6060610292848484610516565b5f5f5f6102c1888888888861078f565b925092509250955095509592505050565b5f6102188261087f565b5f6102ea86868686866108d0565b90505b95945050505050565b5f816001600160a01b03166321f8a721604051602001610334906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161036891815260200190565b602060405180830381865afa158015610383573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102189190615fac565b5f5f5f6103b48686610903565b90505f6103c188836109aa565b90505f5f6103cf8388611c22565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161046791815260200190565b602060405180830381865afa158015610482573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102189190615fc7565b5f816001600160a01b031663bd02d0f56040516020016104339060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663f3903b9f60405160200161043390615fde565b60605f610524858585611e33565b90505f815167ffffffffffffffff81111561054157610541616001565b60405190808252806020026020018201604052801561057a57816020015b610567615591565b81526020019060019003908161055f5790505b5090505f5b8251811015610603575f83828151811061059b5761059b616015565b602002602001015190506105ce60405180604001604052806007815260200166706f6f6c4b657960c81b81525082611ed4565b5f6105d98983611f09565b9050808484815181106105ee576105ee616015565b6020908102919091010152505060010161057f565b5095945050505050565b5f826001600160a01b031663f3903b9f61062684612367565b6040518263ffffffff1660e01b815260040161064491815260200190565b602060405180830381865afa15801561065f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102709190615fc7565b60605f610692868686866123eb565b90505f815167ffffffffffffffff8111156106af576106af616001565b6040519080825280602002602001820160405280156106e857816020015b6106d5615604565b8152602001906001900390816106cd5790505b5090505f5b8251811015610748575f83828151811061070957610709616015565b602002602001015190505f61071e8a83612473565b90508084848151811061073357610733616015565b602090810291909101015250506001016106ed565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161033490602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f61079d8888610903565b90505f6107aa8a836109aa565b90505f808060ff89166107e0576107c28a855f612a81565b9295509193506107d991508590508b5f8087612b70565b9050610810565b5f1960ff8a1601610810576107f68a855f612c3d565b92955091935061080d91508590505f8c8682612b70565b90505b5f61081a85612d06565b90505f8282116108335761082e828461603d565b61083d565b61083d838361603d565b90505f61084a8284612d9f565b90505f8484116108625761085d82616050565b610864565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a721604051602001610334906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f6108dc8686610903565b90505f6108e988836109aa565b90506108f78186865f612dda565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610924578183610927565b82825b6040519194509250610954906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b6109b2615657565b826109bb615657565b816001600160a01b03166391d4403c6040516020016109d990615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610a2d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a51919061606a565b610a5e5791506102189050565b816001600160a01b03166321f8a72185604051602001610a9e906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610ace929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b0291815260200190565b602060405180830381865afa158015610b1d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b419190615fac565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610bbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610bf391815260200190565b602060405180830381865afa158015610c0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c329190615fc7565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610c88906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610cb8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610cec91815260200190565b602060405180830381865afa158015610d07573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d2b9190615fc7565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610d8c9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610dbc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610df091815260200190565b602060405180830381865afa158015610e0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e2f9190615fc7565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610e9a9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610eca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610efe91815260200190565b602060405180830381865afa158015610f19573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3d9190615fc7565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610f9e9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610fce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100291815260200190565b602060405180830381865afa15801561101d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110419190615fc7565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016110be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110f291815260200190565b602060405180830381865afa15801561110d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111319190615fc7565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016111a6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016111da91815260200190565b602060405180830381865afa1580156111f5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112199190615fac565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016112c391815260200190565b602060405180830381865afa1580156112de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113029190615fc7565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161135990602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611389929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016113bd91815260200190565b602060405180830381865afa1580156113d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113fc9190615fc7565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161145e9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161148e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016114c291815260200190565b602060405180830381865afa1580156114dd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115019190615fc7565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161156d9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b6040516020818303038152906040528051906020012060405160200161159d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016115d191815260200190565b602060405180830381865afa1580156115ec573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116109190615fc7565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016116729060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016116d691815260200190565b602060405180830381865afa1580156116f1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117159190615fc7565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161176e90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161179e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016117d291815260200190565b602060405180830381865afa1580156117ed573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118119190615fc7565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161185f90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161188f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016118c391815260200190565b602060405180830381865afa1580156118de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119029190615fac565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611970906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016119a0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016119d491815260200190565b602060405180830381865afa1580156119ef573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a139190615fac565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611a76906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611aa6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611ada91815260200190565b602060405180830381865afa158015611af5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b199190615fc7565b60608201526040516001600160a01b0383169063bd02d0f5908690611b72906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611ba2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611bd691815260200190565b602060405180830381865afa158015611bf1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c159190615fc7565b6080820152949350505050565b5f5f5f5f611c2e61568b565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c929190615fc7565b6020820152611ca2875f80612f11565b506080840152506040820152611cba8760015f612f11565b5060a084015250606082015260408101516020820151611cdb918891612fbf565b61010082015260608101516020820151611cf6918891612fbf565b81610120018181525050611d366040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b815250826020015161307e565b611d6e604051806040016040528060128152602001710766172732e707269636552657365727665360741b815250826040015161307e565b611da660405180604001604052806012815260200171766172732e7072696365526573657276653160701b815250826060015161307e565b611dd96040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b81525082610100015161307e565b611e0c6040518060400160405280600c81526020016b766172732e616d6f756e743160a01b81525082610120015161307e565b610100810151610120820151608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611e5390615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611ead573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261029291908101906160ba565b611f05604051806040016040528060068152602001652573202d257360d01b81525083611f00846130a7565b6130d3565b5050565b611f11615591565b611f196156d4565b611f2384846109aa565b8152611f2e8461311f565b602082018190528151611f42915f90612f11565b608085015260a08401526040830152606082015280516020820151611f6a9190600190612f11565b61012085015261014084015260e08301526101008201528051611f8c90613162565b6101608381019190915260c0830191909152604080516103208101825283515151516001600160a01b039081166101a08301908152855151515184516395d89b4160e01b81529451939586959086019485946101c088019316916395d89b41916004808301925f9291908290030181865afa15801561200d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120349190810190616162565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015612081573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120a591906161f6565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612185573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526121ac9190810190616162565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061222091906161f6565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061230090612d06565b815260200161230f8686613344565b6001600160a01b03168152602001612336835f015160600151660800000000000016151590565b151581526020016123478686613434565b81528251515160c001516020909101526101809091018190529392505050565b5f6040516020016123a1906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61240586612367565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa15801561244c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102ed91908101906160ba565b61247b615604565b61248361573c565b61248d8484613536565b80825251805151602090910151516124a59190610903565b604082018190526124b79085906109aa565b6020820181905281516124cb918691613548565b50505050606082015260208101516124e290612d06565b610300820152805180515160209081015160e08401528083015151510151905161251891905f5b60200201516040015190613597565b60c0820152602081015160600151612530905f6135d8565b60a082015260e081015160c08201516125499190613606565b610100820181905260a08201516125609190613627565b61012082015260e081015160c0820151610100830151612581929190613643565b610140820152602081015181516103008301516125a292879290915f613660565b61016082015261014081015161018082015260e081015160c08201516101208301516125cf929190613643565b6101a08201526020810151606001516125e99060016135d8565b6101c08201528051805160209081015181015161020084015280830151518101510151905161261a91906001612509565b6101e0820181905261020082015161263191613606565b61022082018190526101c08201516126499190613627565b61024082018190526103008201516126619190613597565b6102608201526102008101516101e0820151610220830151612684929190613643565b610280820152602081015181516103008301516126a692879290916001613660565b6102a08201526102808101516102c08201526102008101516101e08201516102608301516126d5929190613643565b6102e082015280516126e6906137f5565b60808201528051516020015160e001516002146127de576127108161030001518260800151613606565b610320820181905261024082015161272791613597565b610340820181905260808201516103008301511161038083018190526102008301516101e084015161275893613837565b61036082018190526103a082015260e081015160a08201516127bf9186916127809190613627565b6127928460c001518560a00151613627565b6127a6856102000151866101c00151613627565b6127ba866101e00151876101c00151613627565b613859565b6103c082018190526103008201516127d79190613919565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612852573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128799190810190616162565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff16600281106128ce576128ce616015565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff166002811061291d5761291d616015565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa15801561297d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526129a49190810190616162565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612a8d615819565b612a98875f5f612f11565b506040840152508152612aad8760015f612f11565b5060608401525060208201528515612ad45787815f01818151612ad0919061603d565b9052505b606087015160381c61ffff166101208201819052612b03908990612afb9061271090613968565b612710612fbf565b610140820181905281516020830151612b2692612b219083906139c2565b612fbf565b608082018190526020820151612b3b91613968565b60c0820181905260408201516060830151610120840151612b5d908c90613a16565b9450945094509450505b93509350935093565b5f5f5f5f86118015612b80575083155b15612b8f575083905084612bc4565b5f87118015612b9c575084155b15612bab575085905082612bc4565b604051636331fab160e01b815260040160405180910390fd5b5f612bd389606001515f6135d8565b90505f612be58a6060015160016135d8565b90505f612c0385676765c793fa10079d601b1b612b2186600a6162f4565b90505f612c2185676765c793fa10079d601b1b612b2186600a6162f4565b9050612c2d8282612d9f565b9c9b505050505050505050505050565b5f5f5f5f612c49615819565b612c54875f5f612f11565b506040840152508152612c698760015f612f11565b5060608401525060208201528515612c91578781602001818151612c8d919061603d565b9052505b80516020820151612ca79190612b21818c6139c2565b608082018190528151612cb991613968565b60a0820152606087015160381c61ffff16610120820181905260a0820151612ce891612afb9061271090613968565b6040820151606083015161012084015160a0850151612b5d91613a16565b5f5f612d13835f5f612f11565b50505090505f612d258460015f612f11565b5050509050805f03612d3a57505f9392505050565b5f612d4985606001515f6135d8565b90505f612d5b866060015160016135d8565b90505f612d7985676765c793fa10079d601b1b612b2186600a6162f4565b90505f612d9785676765c793fa10079d601b1b612b2186600a6162f4565b90506108f782825b5f8115676765c793fa10079d601b1b60028404190484111715612dc0575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612de361568b565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e23573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e479190615fc7565b6020820152612e57865f80612f11565b50505060c0820152612e6b8660015f612f11565b50505060e08201528215612ea657848160c001818151612e8b919061603d565b90525060e081018051859190612ea290839061603d565b9052505b80602001515f03612ed657612ecf6103e8612ec9612ec48888613a3a565b613aa0565b90613968565b8152612f07565b612f04612eec8683602001518460c00151612fbf565b612eff8684602001518560e00151612fbf565b613b80565b81525b5195945050505050565b5f5f5f5f5f875f01518760ff1660028110612f2e57612f2e616015565b602002015190505f612f408989613b95565b9050805f03612f5c575f5f5f5f95509550955095505050612b67565b5f612f6b838b60800151613c67565b90505f612f78838a613597565b90505f8915612f9057612f8b8284613968565b612f92565b5f5b9050612f9e83856162ff565b84612fa9858261603d565b919a509850965094505050505093509350935093565b5f838302815f1985870982811083820303915050805f03612ff357838281612fe957612fe9616312565b0492505050610270565b8084116130135760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b611f05604051806040016040528060068152602001652573202d257360d01b8152508383613c97565b6060610218826040516020016130bf91815260200190565b604051602081830303815290604052613cde565b61311a8383836040516024016130eb93929190616326565b60408051601f198184030181529190526020810180516001600160e01b0316632ced7cef60e01b179052613ede565b505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6131976040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6131a1845f613eea565b6020830152815260608401516131b7905f6135d8565b6060820181905281516131dc91676765c793fa10079d601b1b90612b2190600a6162f4565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613257573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061327b9190615fc7565b608082015261328b846001613eea565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa15801561330a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061332e9190615fc7565b60a0820181905260809091015194909350915050565b5f5f6133508484613f30565b9050806001600160a01b03166321f8a72184604051602001613391906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016133c1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133f591815260200190565b602060405180830381865afa158015613410573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102929190615fac565b5f5f6134408484613f30565b9050806001600160a01b031663bd02d0f5846040516020016134939060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016134c3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134f791815260200190565b602060405180830381865afa158015613512573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102929190615fc7565b61353e61586e565b6102708383613feb565b5f5f5f5f5f61355687612d06565b90506135648787835f615209565b909350915081613575575f1961357f565b61357f8383612d9f565b945061358a886104a6565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176135b7575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff8516016135f8575060ff60601b19905060605b90198416901c905092915050565b5f81831161361d57613618838361603d565b610270565b610270828461603d565b5f61027083676765c793fa10079d601b1b612b2185600a6162f4565b5f8284116136595761365482616050565b610292565b5092915050565b5f61369a6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6136a68686865f615209565b602083015280825215806136d75750845160ff8416600281106136cb576136cb616015565b6020020151602001515f145b156136e5575f9150506102ed565b6136ee87615375565b60408201819052602082015161370391613597565b608082018190528151101561371b575f9150506102ed565b6080810151815161372c919061603d565b8160600181815250506137438660600151846135d8565b60a08201819052606082015161376f9161375e90600a6162f4565b676765c793fa10079d601b1b612fbf565b60c08201525f1960ff8416016137945760c081015161378e9085612d9f565b60c08201525b845160ff8416600281106137aa576137aa616015565b6020020151602001518160c0015111156137e757845160ff8416600281106137d4576137d4616015565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901613815575051602001516060015190565b81516020015160e00151613830575051602001516080015190565b505f919050565b5f841515838511146138515761384c82616050565b6102ed565b509392505050565b5f5f613864876104a6565b90505f6138718287613597565b90505f61387e8386613597565b90505f61388b898461635e565b90505f613898838961635e565b90505f6138a4836153bb565b90505f6138b0836153bb565b90505f841380156138c057505f83125b806138d457505f841280156138d457505f83135b156138e8575f9750505050505050506102ed565b805f036138fe575f9750505050505050506102ed565b6139088282612d9f565b9d9c50505050505050505050505050565b5f815f0361392857505f610218565b5f82841161393f5761393a848461603d565b613949565b613949838561603d565b90505f6139568285612d9f565b90508385116102925761384c81616050565b5f82613974838261603d565b91508111156102185760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826139ce83826162ff565b91508110156102185760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016139b9565b5f81156113881983900484111517613a2c575f5ffd5b506127109102611388010490565b5f811580613a5d57508282613a4f818361637d565b9250613a5b9083616394565b145b6102185760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016139b9565b5f815f03613aaf57505f919050565b5f6001613abb846153d0565b901c6001901b90506001818481613ad457613ad4616312565b048201901c90506001818481613aec57613aec616312565b048201901c90506001818481613b0457613b04616312565b048201901c90506001818481613b1c57613b1c616312565b048201901c90506001818481613b3457613b34616312565b048201901c90506001818481613b4c57613b4c616312565b048201901c90506001818481613b6457613b64616312565b048201901c905061027081828581613b7e57613b7e616312565b045b5f818310613b8e5781610270565b5090919050565b5f5f835f01518360ff1660028110613baf57613baf616015565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015613c08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c2c9190615fc7565b9050805f03613c3f575f92505050610218565b606082015160c0830151613c53828461603d565b613c5d919061603d565b9695505050505050565b5f8260a001515f03613c7a57505f610218565b5f613c858484615463565b60a08501519091506102929082613597565b61311a838383604051602401613caf939291906163b3565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052613ede565b60408051808201909152601081526f181899199a1a9b1b9c1cb0b131b232b360811b60208201528151606091905f90613d1890600261637d565b613d239060026162ff565b67ffffffffffffffff811115613d3b57613d3b616001565b6040519080825280601f01601f191660200182016040528015613d65576020820181803683370190505b509050600360fc1b815f81518110613d7f57613d7f616015565b60200101906001600160f81b03191690815f1a905350600f60fb1b81600181518110613dad57613dad616015565b60200101906001600160f81b03191690815f1a9053505f5b845181101561385157826004868381518110613de357613de3616015565b016020015182516001600160f81b031990911690911c60f81c908110613e0b57613e0b616015565b01602001516001600160f81b03191682613e2683600261637d565b613e319060026162ff565b81518110613e4157613e41616015565b60200101906001600160f81b03191690815f1a90535082858281518110613e6a57613e6a616015565b602091010151815160f89190911c600f16908110613e8a57613e8a616015565b01602001516001600160f81b03191682613ea583600261637d565b613eb09060036162ff565b81518110613ec057613ec0616015565b60200101906001600160f81b03191690815f1a905350600101613dc5565b613ee7816154a6565b50565b5f5f5f613f17855f01518560ff1660028110613f0857613f08616015565b60200201518660800151613c67565b90505f613f248686613b95565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001613f5390615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015613fa7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fcb919061606a565b61027057604051637357d91f60e01b8152600481018490526024016139b9565b613ff361586e565b82613ffc61586e565b816001600160a01b03166391d4403c60405160200161403c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614090573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140b4919061606a565b6140c15791506102189050565b816001600160a01b031663bd02d0f5856040516020016140fb906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161412b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161415f91815260200190565b602060405180830381865afa15801561417a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061419e9190615fc7565b816020018181525050816001600160a01b03166321f8a721856040516020016141e6906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614216929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161424a91815260200190565b602060405180830381865afa158015614265573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142899190615fac565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016142e5906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614315929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161434991815260200190565b602060405180830381865afa158015614364573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143889190615fac565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614403929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161443791815260200190565b602060405180830381865afa158015614452573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144769190615fc7565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016144ca9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b604051602081830303815290604052805190602001206040516020016144fa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161452e91815260200190565b602060405180830381865afa158015614549573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061456d9190615fc7565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016145c7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016145f7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161462b91815260200190565b602060405180830381865afa158015614646573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061466a9190615fc7565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016146c3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016146f3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161472791815260200190565b602060405180830381865afa158015614742573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147669190615fc7565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016147ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161482091815260200190565b602060405180830381865afa15801561483b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061485f9190615fc7565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016148b9906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016148e9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161491d91815260200190565b602060405180830381865afa158015614938573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061495c9190615fc7565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016149cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a0391815260200190565b602060405180830381865afa158015614a1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a429190615fc7565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614ab6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614aea91815260200190565b602060405180830381865afa158015614b05573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b299190615fac565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bd091815260200190565b602060405180830381865afa158015614beb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c0f9190615fc7565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614c649060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c94929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cc891815260200190565b602060405180830381865afa158015614ce3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d079190615fc7565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614d6290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d92929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614dc691815260200190565b602060405180830381865afa158015614de1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e059190615fc7565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614e5f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e8f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ec391815260200190565b602060405180830381865afa158015614ede573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f029190615fc7565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001614f649060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614f94929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fc891815260200190565b602060405180830381865afa158015614fe3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150079190615fc7565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161506290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615092929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150c691815260200190565b602060405180830381865afa1580156150e1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151059190615fc7565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615154906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615184929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151b891815260200190565b602060405180830381865afa1580156151d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151f79190615fc7565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b038681169116146152ac575f5f6152348a8a5f6154c5565b915091505f6152505f8c606001516135d890919063ffffffff16565b90505f61526e84676765c793fa10079d601b1b612b2185600a6162f4565b90505f61528c84676765c793fa10079d601b1b612b2186600a6162f4565b905061529882886162ff565b96506152a481876162ff565b955050505050505b865160200151516001600160a01b03868116911614615368575f5f6152d38a8a60016154c5565b915091505f6152f060018c606001516135d890919063ffffffff16565b90505f61530e84676765c793fa10079d601b1b612b2185600a6162f4565b90505f61532c84676765c793fa10079d601b1b612b2186600a6162f4565b90505f615339838d613597565b90505f615346838e613597565b9050615352828a6162ff565b985061535e81896162ff565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f8212156153cc57815f03610218565b5090565b5f80608083901c156153e457608092831c92015b604083901c156153f657604092831c92015b602083901c1561540857602092831c92015b601083901c1561541a57601092831c92015b600883901c1561542c57600892831c92015b600483901c1561543e57600492831c92015b600283901c1561545057600292831c92015b600183901c156102185760010192915050565b5f42820361547657506020820151610218565b5f61548584604001518461555d565b905061549e84602001518261359790919063ffffffff16565b915050610218565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b5f5f5f845f01518460ff16600281106154e0576154e0616015565b60200201516040015190505f615516875f01518660ff166002811061550757615507616015565b60200201518860800151615463565b9050811561552d576155288282613597565b61552f565b5f5b865190935060ff86166002811061554857615548616015565b60200201516020015193505050935093915050565b5f80615569834261603d565b615573908561637d565b6301e133809004905061029281676765c793fa10079d601b1b6162ff565b6040518061016001604052806155a5615894565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81525090565b60405180610120016040528061561861591b565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a0016040528061566a615996565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101a001604052806156e8615657565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615737615591565b905290565b60405180610400016040528061575061586e565b815260200161575d615657565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061018001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806158816159fd565b81525f6020820181905260409091015290565b60405180604001604052806002905b6159056040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816158a35790505090565b60405180604001604052806002905b6159806040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161592a5790505090565b60405180604001604052806002905b6159e76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816159a55790505090565b60405180604001604052806002905b615a556040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081615a0c5790505090565b6001600160a01b0381168114613ee7575f5ffd5b5f60208284031215615a8f575f5ffd5b813561027081615a6b565b5f5f5f5f60808587031215615aad575f5ffd5b8435615ab881615a6b565b93506020850135615ac881615a6b565b92506040850135615ad881615a6b565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b6002811015615be1578383038752815180516001600160a01b0316845260208101516101806020860152615b57610180860182615ae8565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615b1f565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615cf557603f19878603018452815180516101608752615c3a610160880182615b16565b90506020820151615c5660208901826001600160a01b03169052565b506040820151615c7160408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151615cb460e08901826001600160a01b03169052565b50610100820151615cca61010089018215159052565b5061012082810151908801526101409182015191909601526020938401939190910190600101615c12565b50929695505050505050565b5f5f60408385031215615d12575f5ffd5b8235615d1d81615a6b565b91506020830135615d2d81615a6b565b809150509250929050565b5f8260408101835f5b6002811015615be1578383038752815180516001600160a01b0316845260208101516101406020860152615d79610140860182615ae8565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050615d41565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615cf557603f19878603018452815180516101208752615e39610120880182615d38565b9050602082015160208801526040820151615e5f60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101615e11565b5f5f5f60608486031215615ec3575f5ffd5b8335615ece81615a6b565b95602085013595506040909401359392505050565b60ff81168114613ee7575f5ffd5b5f5f5f5f5f60a08688031215615f05575f5ffd5b8535615f1081615a6b565b94506020860135615f2081615a6b565b93506040860135615f3081615a6b565b9250606086013591506080860135615f4781615ee3565b809150509295509295909350565b5f5f5f5f5f60a08688031215615f69575f5ffd5b8535615f7481615a6b565b94506020860135615f8481615a6b565b93506040860135615f9481615a6b565b94979396509394606081013594506080013592915050565b5f60208284031215615fbc575f5ffd5b815161027081615a6b565b5f60208284031215615fd7575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561021857610218616029565b5f600160ff1b820161606457616064616029565b505f0390565b5f6020828403121561607a575f5ffd5b81518015158114610270575f5ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156160b2576160b2616001565b604052919050565b5f602082840312156160ca575f5ffd5b815167ffffffffffffffff8111156160e0575f5ffd5b8201601f810184136160f0575f5ffd5b805167ffffffffffffffff81111561610a5761610a616001565b8060051b61611a60208201616089565b91825260208184018101929081019087841115616135575f5ffd5b6020850194505b8385101561615757845182526020948501949091019061613c565b979650505050505050565b5f60208284031215616172575f5ffd5b815167ffffffffffffffff811115616188575f5ffd5b8201601f81018413616198575f5ffd5b805167ffffffffffffffff8111156161b2576161b2616001565b6161c5601f8201601f1916602001616089565b8181528560208385010111156161d9575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215616206575f5ffd5b815161027081615ee3565b6001815b600184111561624c5780850481111561623057616230616029565b600184161561623e57908102905b60019390931c928002616215565b935093915050565b5f8261626257506001610218565b8161626e57505f610218565b8160018114616284576002811461628e576162aa565b6001915050610218565b60ff84111561629f5761629f616029565b50506001821b610218565b5060208310610133831016604e8410600b84101617156162cd575081810a610218565b6162d95f198484616211565b805f19048211156162ec576162ec616029565b029392505050565b5f6102708383616254565b8082018082111561021857610218616029565b634e487b7160e01b5f52601260045260245ffd5b606081525f6163386060830186615ae8565b828103602084015261634a8186615ae8565b90508281036040840152613c5d8185615ae8565b8181035f83128015838313168383128216171561365957613659616029565b808202811582820484141761021857610218616029565b5f826163ae57634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f6163c56060830186615ae8565b82810360208401526163d78186615ae8565b91505082604083015294935050505056fea2646970667358221220bac0d7931d71246413cc40f02d9cf0b2b6dfea7621fe1c075c81b5dd998b267d64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pad\x1E\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01tW\x80cx\xF2\x12\xD1\x14a\x01\x94W\x80c\x8Flz<\x14a\x01\xA7W\x80c\xD2\x8B\n\x15\x14a\x01\xBAW\x80c\xE35\xAD\xB7\x14a\x01\xE8W\x80c\xF6\x8Aq1\x14a\x01\xFBW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xB5W\x80c1{P\xEC\x14a\0\xE5W\x80cP\xEDY-\x14a\x01\rW\x80cW\xB9\x1C\xA6\x14a\x01.W\x80cZoW\x10\x14a\x01AW\x80c\\9\xF4g\x14a\x01TW[__\xFD[a\0\xC8a\0\xC36`\x04aZ\x7FV[a\x02\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF8a\0\xF36`\x04aZ\x9AV[a\x02\x1EV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xDCV[a\x01 a\x01\x1B6`\x04aZ\x7FV[a\x029V[`@Q\x90\x81R` \x01a\0\xDCV[a\x01 a\x01<6`\x04aZ\x7FV[a\x02CV[a\x01 a\x01O6`\x04aZ\x7FV[a\x02MV[a\x01ga\x01b6`\x04aZ\x7FV[a\x02WV[`@Qa\0\xDC\x91\x90a[\xECV[a\x01\x87a\x01\x826`\x04a]\x01V[a\x02wV[`@Qa\0\xDC\x91\x90a]\xEBV[a\0\xC8a\x01\xA26`\x04aZ\x7FV[a\x02\x9AV[a\x01ga\x01\xB56`\x04a^\xB1V[a\x02\xA4V[a\x01\xCDa\x01\xC86`\x04a^\xF1V[a\x02\xB1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xDCV[a\0\xC8a\x01\xF66`\x04aZ\x7FV[a\x02\xD2V[a\x01 a\x02\t6`\x04a_UV[a\x02\xDCV[_a\x02\x18\x82a\x02\xF6V[\x92\x91PPV[__a\x02,\x86\x86\x86\x86a\x03\xA7V[\x91P\x91P\x94P\x94\x92PPPV[_a\x02\x18\x82a\x03\xE2V[_a\x02\x18\x82a\x04\xA6V[_a\x02\x18\x82a\x04\xF7V[``_a\x02c\x83a\x04\xF7V[\x90Pa\x02p\x83_\x83a\x05\x16V[\x93\x92PPPV[``_a\x02\x84\x84\x84a\x06\rV[\x90Pa\x02\x92\x84\x84_\x84a\x06\x83V[\x94\x93PPPPV[_a\x02\x18\x82a\x07SV[``a\x02\x92\x84\x84\x84a\x05\x16V[___a\x02\xC1\x88\x88\x88\x88\x88a\x07\x8FV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x02\x18\x82a\x08\x7FV[_a\x02\xEA\x86\x86\x86\x86\x86a\x08\xD0V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a_\xACV[___a\x03\xB4\x86\x86a\t\x03V[\x90P_a\x03\xC1\x88\x83a\t\xAAV[\x90P__a\x03\xCF\x83\x88a\x1C\"V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04g\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a_\xC7V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x043\x90a_\xDEV[``_a\x05$\x85\x85\x85a\x1E3V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05AWa\x05Aa`\x01V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05zW\x81` \x01[a\x05gaU\x91V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05_W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\x03W_\x83\x82\x81Q\x81\x10a\x05\x9BWa\x05\x9Ba`\x15V[` \x02` \x01\x01Q\x90Pa\x05\xCE`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RP\x82a\x1E\xD4V[_a\x05\xD9\x89\x83a\x1F\tV[\x90P\x80\x84\x84\x81Q\x81\x10a\x05\xEEWa\x05\xEEa`\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x7FV[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x06&\x84a#gV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02p\x91\x90a_\xC7V[``_a\x06\x92\x86\x86\x86\x86a#\xEBV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xAFWa\x06\xAFa`\x01V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xE8W\x81` \x01[a\x06\xD5aV\x04V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xCDW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x07HW_\x83\x82\x81Q\x81\x10a\x07\tWa\x07\ta`\x15V[` \x02` \x01\x01Q\x90P_a\x07\x1E\x8A\x83a$sV[\x90P\x80\x84\x84\x81Q\x81\x10a\x073Wa\x073a`\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\xEDV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x07\x9D\x88\x88a\t\x03V[\x90P_a\x07\xAA\x8A\x83a\t\xAAV[\x90P_\x80\x80`\xFF\x89\x16a\x07\xE0Wa\x07\xC2\x8A\x85_a*\x81V[\x92\x95P\x91\x93Pa\x07\xD9\x91P\x85\x90P\x8B_\x80\x87a+pV[\x90Pa\x08\x10V[_\x19`\xFF\x8A\x16\x01a\x08\x10Wa\x07\xF6\x8A\x85_a,=V[\x92\x95P\x91\x93Pa\x08\r\x91P\x85\x90P_\x8C\x86\x82a+pV[\x90P[_a\x08\x1A\x85a-\x06V[\x90P_\x82\x82\x11a\x083Wa\x08.\x82\x84a`=V[a\x08=V[a\x08=\x83\x83a`=V[\x90P_a\x08J\x82\x84a-\x9FV[\x90P_\x84\x84\x11a\x08bWa\x08]\x82a`PV[a\x08dV[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08\xDC\x86\x86a\t\x03V[\x90P_a\x08\xE9\x88\x83a\t\xAAV[\x90Pa\x08\xF7\x81\x86\x86_a-\xDAV[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\t$W\x81\x83a\t'V[\x82\x82[`@Q\x91\x94P\x92Pa\tT\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\xB2aVWV[\x82a\t\xBBaVWV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t\xD9\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n-W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nQ\x91\x90a`jV[a\n^W\x91Pa\x02\x18\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\n\x9E\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BA\x91\x90a_\xACV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C2\x91\x90a_\xC7V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\x88\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r+\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\x8C\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E/\x91\x90a_\xC7V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\x9A\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F=\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0F\x9E\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10A\x91\x90a_\xC7V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x111\x91\x90a_\xC7V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x19\x91\x90a_\xACV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x02\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13Y\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xBD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFC\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14^\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x8E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xC2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x01\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15m\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x9D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xD1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x10\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16r\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x15\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17n\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x11\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18_\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x02\x91\x90a_\xACV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x19p\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x13\x91\x90a_\xACV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1Av\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x19\x91\x90a_\xC7V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1Br\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x15\x91\x90a_\xC7V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1C.aV\x8BV[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x92\x91\x90a_\xC7V[` \x82\x01Ra\x1C\xA2\x87_\x80a/\x11V[P`\x80\x84\x01RP`@\x82\x01Ra\x1C\xBA\x87`\x01_a/\x11V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C\xDB\x91\x88\x91a/\xBFV[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1C\xF6\x91\x88\x91a/\xBFV[\x81a\x01 \x01\x81\x81RPPa\x1D6`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa0~V[a\x1Dn`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa0~V[a\x1D\xA6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa0~V[a\x1D\xD9`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa0~V[a\x1E\x0C`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa0~V[a\x01\0\x81\x01Qa\x01 \x82\x01Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1ES\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xADW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x92\x91\x90\x81\x01\x90a`\xBAV[a\x1F\x05`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83a\x1F\0\x84a0\xA7V[a0\xD3V[PPV[a\x1F\x11aU\x91V[a\x1F\x19aV\xD4V[a\x1F#\x84\x84a\t\xAAV[\x81Ra\x1F.\x84a1\x1FV[` \x82\x01\x81\x90R\x81Qa\x1FB\x91_\x90a/\x11V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x1Fj\x91\x90`\x01\x90a/\x11V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa\x1F\x8C\x90a1bV[a\x01`\x83\x81\x01\x91\x90\x91R`\xC0\x83\x01\x91\x90\x91R`@\x80Qa\x03 \x81\x01\x82R\x83QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xA0\x83\x01\x90\x81R\x85QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x95\x86\x95\x90\x86\x01\x94\x85\x94a\x01\xC0\x88\x01\x93\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a \rW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra 4\x91\x90\x81\x01\x90aabV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a \x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xA5\x91\x90aa\xF6V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\x85W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAC\x91\x90\x81\x01\x90aabV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\" \x91\x90aa\xF6V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a#\0\x90a-\x06V[\x81R` \x01a#\x0F\x86\x86a3DV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a#6\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a#G\x86\x86a44V[\x81R\x82QQQ`\xC0\x01Q` \x90\x91\x01Ra\x01\x80\x90\x91\x01\x81\x90R\x93\x92PPPV[_`@Q` \x01a#\xA1\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a$\x05\x86a#gV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$LW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xED\x91\x90\x81\x01\x90a`\xBAV[a${aV\x04V[a$\x83aW<V[a$\x8D\x84\x84a56V[\x80\x82RQ\x80QQ` \x90\x91\x01QQa$\xA5\x91\x90a\t\x03V[`@\x82\x01\x81\x90Ra$\xB7\x90\x85\x90a\t\xAAV[` \x82\x01\x81\x90R\x81Qa$\xCB\x91\x86\x91a5HV[PPPP``\x82\x01R` \x81\x01Qa$\xE2\x90a-\x06V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa%\x18\x91\x90_[` \x02\x01Q`@\x01Q\x90a5\x97V[`\xC0\x82\x01R` \x81\x01Q``\x01Qa%0\x90_a5\xD8V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa%I\x91\x90a6\x06V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa%`\x91\x90a6'V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa%\x81\x92\x91\x90a6CV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%\xA2\x92\x87\x92\x90\x91_a6`V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa%\xCF\x92\x91\x90a6CV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa%\xE9\x90`\x01a5\xD8V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa&\x1A\x91\x90`\x01a%\tV[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa&1\x91a6\x06V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa&I\x91\x90a6'V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&a\x91\x90a5\x97V[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa&\x84\x92\x91\x90a6CV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa&\xA6\x92\x87\x92\x90\x91`\x01a6`V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa&\xD5\x92\x91\x90a6CV[a\x02\xE0\x82\x01R\x80Qa&\xE6\x90a7\xF5V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a'\xDEWa'\x10\x81a\x03\0\x01Q\x82`\x80\x01Qa6\x06V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa''\x91a5\x97V[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa'X\x93a87V[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa'\xBF\x91\x86\x91a'\x80\x91\x90a6'V[a'\x92\x84`\xC0\x01Q\x85`\xA0\x01Qa6'V[a'\xA6\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa6'V[a'\xBA\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa6'V[a8YV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa'\xD7\x91\x90a9\x19V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a(RW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(y\x91\x90\x81\x01\x90aabV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a(\xCEWa(\xCEa`\x15V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a)\x1DWa)\x1Da`\x15V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)}W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)\xA4\x91\x90\x81\x01\x90aabV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a*\x8DaX\x19V[a*\x98\x87__a/\x11V[P`@\x84\x01RP\x81Ra*\xAD\x87`\x01_a/\x11V[P``\x84\x01RP` \x82\x01R\x85\x15a*\xD4W\x87\x81_\x01\x81\x81Qa*\xD0\x91\x90a`=V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra+\x03\x90\x89\x90a*\xFB\x90a'\x10\x90a9hV[a'\x10a/\xBFV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa+&\x92a+!\x90\x83\x90a9\xC2V[a/\xBFV[`\x80\x82\x01\x81\x90R` \x82\x01Qa+;\x91a9hV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa+]\x90\x8C\x90a:\x16V[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a+\x80WP\x83\x15[\x15a+\x8FWP\x83\x90P\x84a+\xC4V[_\x87\x11\x80\x15a+\x9CWP\x84\x15[\x15a+\xABWP\x85\x90P\x82a+\xC4V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a+\xD3\x89``\x01Q_a5\xD8V[\x90P_a+\xE5\x8A``\x01Q`\x01a5\xD8V[\x90P_a,\x03\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_a,!\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90Pa,-\x82\x82a-\x9FV[\x9C\x9BPPPPPPPPPPPPV[____a,IaX\x19V[a,T\x87__a/\x11V[P`@\x84\x01RP\x81Ra,i\x87`\x01_a/\x11V[P``\x84\x01RP` \x82\x01R\x85\x15a,\x91W\x87\x81` \x01\x81\x81Qa,\x8D\x91\x90a`=V[\x90RP[\x80Q` \x82\x01Qa,\xA7\x91\x90a+!\x81\x8Ca9\xC2V[`\x80\x82\x01\x81\x90R\x81Qa,\xB9\x91a9hV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa,\xE8\x91a*\xFB\x90a'\x10\x90a9hV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa+]\x91a:\x16V[__a-\x13\x83__a/\x11V[PPP\x90P_a-%\x84`\x01_a/\x11V[PPP\x90P\x80_\x03a-:WP_\x93\x92PPPV[_a-I\x85``\x01Q_a5\xD8V[\x90P_a-[\x86``\x01Q`\x01a5\xD8V[\x90P_a-y\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_a-\x97\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90Pa\x08\xF7\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a-\xC0W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a-\xE3aV\x8BV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.G\x91\x90a_\xC7V[` \x82\x01Ra.W\x86_\x80a/\x11V[PPP`\xC0\x82\x01Ra.k\x86`\x01_a/\x11V[PPP`\xE0\x82\x01R\x82\x15a.\xA6W\x84\x81`\xC0\x01\x81\x81Qa.\x8B\x91\x90a`=V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a.\xA2\x90\x83\x90a`=V[\x90RP[\x80` \x01Q_\x03a.\xD6Wa.\xCFa\x03\xE8a.\xC9a.\xC4\x88\x88a::V[a:\xA0V[\x90a9hV[\x81Ra/\x07V[a/\x04a.\xEC\x86\x83` \x01Q\x84`\xC0\x01Qa/\xBFV[a.\xFF\x86\x84` \x01Q\x85`\xE0\x01Qa/\xBFV[a;\x80V[\x81R[Q\x95\x94PPPPPV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10a/.Wa/.a`\x15V[` \x02\x01Q\x90P_a/@\x89\x89a;\x95V[\x90P\x80_\x03a/\\W____\x95P\x95P\x95P\x95PPPa+gV[_a/k\x83\x8B`\x80\x01Qa<gV[\x90P_a/x\x83\x8Aa5\x97V[\x90P_\x89\x15a/\x90Wa/\x8B\x82\x84a9hV[a/\x92V[_[\x90Pa/\x9E\x83\x85ab\xFFV[\x84a/\xA9\x85\x82a`=V[\x91\x9AP\x98P\x96P\x94PPPPP\x93P\x93P\x93P\x93V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a/\xF3W\x83\x82\x81a/\xE9Wa/\xE9ac\x12V[\x04\x92PPPa\x02pV[\x80\x84\x11a0\x13W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a\x1F\x05`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83a<\x97V[``a\x02\x18\x82`@Q` \x01a0\xBF\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra<\xDEV[a1\x1A\x83\x83\x83`@Q`$\x01a0\xEB\x93\x92\x91\x90ac&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c,\xED|\xEF`\xE0\x1B\x17\x90Ra>\xDEV[PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a1\x97`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a1\xA1\x84_a>\xEAV[` \x83\x01R\x81R``\x84\x01Qa1\xB7\x90_a5\xD8V[``\x82\x01\x81\x90R\x81Qa1\xDC\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a+!\x90`\nab\xF4V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2WW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2{\x91\x90a_\xC7V[`\x80\x82\x01Ra2\x8B\x84`\x01a>\xEAV[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3.\x91\x90a_\xC7V[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[__a3P\x84\x84a?0V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a3\x91\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a_\xACV[__a4@\x84\x84a?0V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a4\x93\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xF7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a_\xC7V[a5>aXnV[a\x02p\x83\x83a?\xEBV[_____a5V\x87a-\x06V[\x90Pa5d\x87\x87\x83_aR\tV[\x90\x93P\x91P\x81a5uW_\x19a5\x7FV[a5\x7F\x83\x83a-\x9FV[\x94Pa5\x8A\x88a\x04\xA6V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a5\xB7W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a5\xF8WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a6\x1DWa6\x18\x83\x83a`=V[a\x02pV[a\x02p\x82\x84a`=V[_a\x02p\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[_\x82\x84\x11a6YWa6T\x82a`PV[a\x02\x92V[P\x92\x91PPV[_a6\x9A`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a6\xA6\x86\x86\x86_aR\tV[` \x83\x01R\x80\x82R\x15\x80a6\xD7WP\x84Q`\xFF\x84\x16`\x02\x81\x10a6\xCBWa6\xCBa`\x15V[` \x02\x01Q` \x01Q_\x14[\x15a6\xE5W_\x91PPa\x02\xEDV[a6\xEE\x87aSuV[`@\x82\x01\x81\x90R` \x82\x01Qa7\x03\x91a5\x97V[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a7\x1BW_\x91PPa\x02\xEDV[`\x80\x81\x01Q\x81Qa7,\x91\x90a`=V[\x81``\x01\x81\x81RPPa7C\x86``\x01Q\x84a5\xD8V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa7o\x91a7^\x90`\nab\xF4V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xBFV[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a7\x94W`\xC0\x81\x01Qa7\x8E\x90\x85a-\x9FV[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a7\xAAWa7\xAAa`\x15V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a7\xE7W\x84Q`\xFF\x84\x16`\x02\x81\x10a7\xD4Wa7\xD4a`\x15V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a8\x15WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa80WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a8QWa8L\x82a`PV[a\x02\xEDV[P\x93\x92PPPV[__a8d\x87a\x04\xA6V[\x90P_a8q\x82\x87a5\x97V[\x90P_a8~\x83\x86a5\x97V[\x90P_a8\x8B\x89\x84ac^V[\x90P_a8\x98\x83\x89ac^V[\x90P_a8\xA4\x83aS\xBBV[\x90P_a8\xB0\x83aS\xBBV[\x90P_\x84\x13\x80\x15a8\xC0WP_\x83\x12[\x80a8\xD4WP_\x84\x12\x80\x15a8\xD4WP_\x83\x13[\x15a8\xE8W_\x97PPPPPPPPa\x02\xEDV[\x80_\x03a8\xFEW_\x97PPPPPPPPa\x02\xEDV[a9\x08\x82\x82a-\x9FV[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03a9(WP_a\x02\x18V[_\x82\x84\x11a9?Wa9:\x84\x84a`=V[a9IV[a9I\x83\x85a`=V[\x90P_a9V\x82\x85a-\x9FV[\x90P\x83\x85\x11a\x02\x92Wa8L\x81a`PV[_\x82a9t\x83\x82a`=V[\x91P\x81\x11\x15a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a9\xCE\x83\x82ab\xFFV[\x91P\x81\x10\x15a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a9\xB9V[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a:,W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a:]WP\x82\x82a:O\x81\x83ac}V[\x92Pa:[\x90\x83ac\x94V[\x14[a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a9\xB9V[_\x81_\x03a:\xAFWP_\x91\x90PV[_`\x01a:\xBB\x84aS\xD0V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a:\xD4Wa:\xD4ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a:\xECWa:\xECac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;\x04Wa;\x04ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;\x1CWa;\x1Cac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;4Wa;4ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;LWa;Lac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;dWa;dac\x12V[\x04\x82\x01\x90\x1C\x90Pa\x02p\x81\x82\x85\x81a;~Wa;~ac\x12V[\x04[_\x81\x83\x10a;\x8EW\x81a\x02pV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a;\xAFWa;\xAFa`\x15V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<,\x91\x90a_\xC7V[\x90P\x80_\x03a<?W_\x92PPPa\x02\x18V[``\x82\x01Q`\xC0\x83\x01Qa<S\x82\x84a`=V[a<]\x91\x90a`=V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a<zWP_a\x02\x18V[_a<\x85\x84\x84aTcV[`\xA0\x85\x01Q\x90\x91Pa\x02\x92\x90\x82a5\x97V[a1\x1A\x83\x83\x83`@Q`$\x01a<\xAF\x93\x92\x91\x90ac\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra>\xDEV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90_\x90a=\x18\x90`\x02ac}V[a=#\x90`\x02ab\xFFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=;Wa=;a`\x01V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a=eW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a=\x7FWa=\x7Fa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a=\xADWa=\xADa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x84Q\x81\x10\x15a8QW\x82`\x04\x86\x83\x81Q\x81\x10a=\xE3Wa=\xE3a`\x15V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a>\x0BWa>\x0Ba`\x15V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a>&\x83`\x02ac}V[a>1\x90`\x02ab\xFFV[\x81Q\x81\x10a>AWa>Aa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a>jWa>ja`\x15V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a>\x8AWa>\x8Aa`\x15V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a>\xA5\x83`\x02ac}V[a>\xB0\x90`\x03ab\xFFV[\x81Q\x81\x10a>\xC0Wa>\xC0a`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a=\xC5V[a>\xE7\x81aT\xA6V[PV[___a?\x17\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a?\x08Wa?\x08a`\x15V[` \x02\x01Q\x86`\x80\x01Qa<gV[\x90P_a?$\x86\x86a;\x95V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a?S\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xCB\x91\x90a`jV[a\x02pW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a9\xB9V[a?\xF3aXnV[\x82a?\xFCaXnV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a@<\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xB4\x91\x90a`jV[a@\xC1W\x91Pa\x02\x18\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\xFB\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA_\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\x9E\x91\x90a_\xC7V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aA\xE6\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aBJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\x89\x91\x90a_\xACV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aB\xE5\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aCI\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCdW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x88\x91\x90a_\xACV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDv\x91\x90a_\xC7V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aD\xCA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE.\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aEIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEm\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xC7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFj\x91\x90a_\xC7V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xC3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGf\x91\x90a_\xC7V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH_\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\xB9\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\\\x91\x90a_\xC7V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJB\x91\x90a_\xC7V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK)\x91\x90a_\xACV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\x0F\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aLd\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x07\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aMb\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xC6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\x05\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN_\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\x02\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOd\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\x07\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPb\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xC6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x05\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQT\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\x84\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xB8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xF7\x91\x90a_\xC7V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aR\xACW__aR4\x8A\x8A_aT\xC5V[\x91P\x91P_aRP_\x8C``\x01Qa5\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aRn\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[\x90P_aR\x8C\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90PaR\x98\x82\x88ab\xFFV[\x96PaR\xA4\x81\x87ab\xFFV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aShW__aR\xD3\x8A\x8A`\x01aT\xC5V[\x91P\x91P_aR\xF0`\x01\x8C``\x01Qa5\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aS\x0E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[\x90P_aS,\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_aS9\x83\x8Da5\x97V[\x90P_aSF\x83\x8Ea5\x97V[\x90PaSR\x82\x8Aab\xFFV[\x98PaS^\x81\x89ab\xFFV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aS\xCCW\x81_\x03a\x02\x18V[P\x90V[_\x80`\x80\x83\x90\x1C\x15aS\xE4W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aS\xF6W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aT\x08W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aT\x1AW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aT,W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aT>W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aTPW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x02\x18W`\x01\x01\x92\x91PPV[_B\x82\x03aTvWP` \x82\x01Qa\x02\x18V[_aT\x85\x84`@\x01Q\x84aU]V[\x90PaT\x9E\x84` \x01Q\x82a5\x97\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x02\x18V[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aT\xE0WaT\xE0a`\x15V[` \x02\x01Q`@\x01Q\x90P_aU\x16\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aU\x07WaU\x07a`\x15V[` \x02\x01Q\x88`\x80\x01QaTcV[\x90P\x81\x15aU-WaU(\x82\x82a5\x97V[aU/V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aUHWaUHa`\x15V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aUi\x83Ba`=V[aUs\x90\x85ac}V[c\x01\xE13\x80\x90\x04\x90Pa\x02\x92\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bab\xFFV[`@Q\x80a\x01`\x01`@R\x80aU\xA5aX\x94V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aV\x18aY\x1BV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aVjaY\x96V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xA0\x01`@R\x80aV\xE8aVWV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aW7aU\x91V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aWPaXnV[\x81R` \x01aW]aVWV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aX\x81aY\xFDV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\x05`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aX\xA3W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\x80`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aY*W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\xE7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aY\xA5W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aZU`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aZ\x0CW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a>\xE7W__\xFD[_` \x82\x84\x03\x12\x15aZ\x8FW__\xFD[\x815a\x02p\x81aZkV[____`\x80\x85\x87\x03\x12\x15aZ\xADW__\xFD[\x845aZ\xB8\x81aZkV[\x93P` \x85\x015aZ\xC8\x81aZkV[\x92P`@\x85\x015aZ\xD8\x81aZkV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15a[\xE1W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Ra[Wa\x01\x80\x86\x01\x82aZ\xE8V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pa[\x1FV[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\\\xF5W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01`\x87Ra\\:a\x01`\x88\x01\x82a[\x16V[\x90P` \x82\x01Qa\\V` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qa\\q`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qa\\\xB4`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qa\\\xCAa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\\\x12V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a]\x12W__\xFD[\x825a]\x1D\x81aZkV[\x91P` \x83\x015a]-\x81aZkV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15a[\xE1W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ra]ya\x01@\x86\x01\x82aZ\xE8V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pa]AV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\\\xF5W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ra^9a\x01 \x88\x01\x82a]8V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qa^_`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a^\x11V[___``\x84\x86\x03\x12\x15a^\xC3W__\xFD[\x835a^\xCE\x81aZkV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14a>\xE7W__\xFD[_____`\xA0\x86\x88\x03\x12\x15a_\x05W__\xFD[\x855a_\x10\x81aZkV[\x94P` \x86\x015a_ \x81aZkV[\x93P`@\x86\x015a_0\x81aZkV[\x92P``\x86\x015\x91P`\x80\x86\x015a_G\x81a^\xE3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15a_iW__\xFD[\x855a_t\x81aZkV[\x94P` \x86\x015a_\x84\x81aZkV[\x93P`@\x86\x015a_\x94\x81aZkV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\xBCW__\xFD[\x81Qa\x02p\x81aZkV[_` \x82\x84\x03\x12\x15a_\xD7W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x18Wa\x02\x18a`)V[_`\x01`\xFF\x1B\x82\x01a`dWa`da`)V[P_\x03\x90V[_` \x82\x84\x03\x12\x15a`zW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02pW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a`\xB2Wa`\xB2a`\x01V[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15a`\xCAW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`\xE0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a`\xF0W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\nWaa\na`\x01V[\x80`\x05\x1Baa\x1A` \x82\x01a`\x89V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aa5W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aaWW\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aa<V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aarW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\x88W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aa\x98W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\xB2Waa\xB2a`\x01V[aa\xC5`\x1F\x82\x01`\x1F\x19\x16` \x01a`\x89V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aa\xD9W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ab\x06W__\xFD[\x81Qa\x02p\x81a^\xE3V[`\x01\x81[`\x01\x84\x11\x15abLW\x80\x85\x04\x81\x11\x15ab0Wab0a`)V[`\x01\x84\x16\x15ab>W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ab\x15V[\x93P\x93\x91PPV[_\x82abbWP`\x01a\x02\x18V[\x81abnWP_a\x02\x18V[\x81`\x01\x81\x14ab\x84W`\x02\x81\x14ab\x8EWab\xAAV[`\x01\x91PPa\x02\x18V[`\xFF\x84\x11\x15ab\x9FWab\x9Fa`)V[PP`\x01\x82\x1Ba\x02\x18V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ab\xCDWP\x81\x81\na\x02\x18V[ab\xD9_\x19\x84\x84ab\x11V[\x80_\x19\x04\x82\x11\x15ab\xECWab\xECa`)V[\x02\x93\x92PPPV[_a\x02p\x83\x83abTV[\x80\x82\x01\x80\x82\x11\x15a\x02\x18Wa\x02\x18a`)V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[``\x81R_ac8``\x83\x01\x86aZ\xE8V[\x82\x81\x03` \x84\x01RacJ\x81\x86aZ\xE8V[\x90P\x82\x81\x03`@\x84\x01Ra<]\x81\x85aZ\xE8V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a6YWa6Ya`)V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x18Wa\x02\x18a`)V[_\x82ac\xAEWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_ac\xC5``\x83\x01\x86aZ\xE8V[\x82\x81\x03` \x84\x01Rac\xD7\x81\x86aZ\xE8V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xBA\xC0\xD7\x93\x1Dq$d\x13\xCC@\xF0-\x9C\xF0\xB2\xB6\xDF\xEAv!\xFE\x1C\x07\\\x81\xB5\xDD\x99\x8B&}dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c8063739118a41161006e578063739118a41461017457806378f212d1146101945780638f6c7a3c146101a7578063d28b0a15146101ba578063e335adb7146101e8578063f68a7131146101fb575f5ffd5b806328a0ccf4146100b5578063317b50ec146100e557806350ed592d1461010d57806357b91ca61461012e5780635a6f5710146101415780635c39f46714610154575b5f5ffd5b6100c86100c3366004615a7f565b61020e565b6040516001600160a01b0390911681526020015b60405180910390f35b6100f86100f3366004615a9a565b61021e565b604080519283526020830191909152016100dc565b61012061011b366004615a7f565b610239565b6040519081526020016100dc565b61012061013c366004615a7f565b610243565b61012061014f366004615a7f565b61024d565b610167610162366004615a7f565b610257565b6040516100dc9190615bec565b610187610182366004615d01565b610277565b6040516100dc9190615deb565b6100c86101a2366004615a7f565b61029a565b6101676101b5366004615eb1565b6102a4565b6101cd6101c8366004615ef1565b6102b1565b604080519384526020840192909252908201526060016100dc565b6100c86101f6366004615a7f565b6102d2565b610120610209366004615f55565b6102dc565b5f610218826102f6565b92915050565b5f5f61022c868686866103a7565b9150915094509492505050565b5f610218826103e2565b5f610218826104a6565b5f610218826104f7565b60605f610263836104f7565b9050610270835f83610516565b9392505050565b60605f610284848461060d565b905061029284845f84610683565b949350505050565b5f61021882610753565b6060610292848484610516565b5f5f5f6102c1888888888861078f565b925092509250955095509592505050565b5f6102188261087f565b5f6102ea86868686866108d0565b90505b95945050505050565b5f816001600160a01b03166321f8a721604051602001610334906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161036891815260200190565b602060405180830381865afa158015610383573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102189190615fac565b5f5f5f6103b48686610903565b90505f6103c188836109aa565b90505f5f6103cf8388611c22565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161046791815260200190565b602060405180830381865afa158015610482573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102189190615fc7565b5f816001600160a01b031663bd02d0f56040516020016104339060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663f3903b9f60405160200161043390615fde565b60605f610524858585611e33565b90505f815167ffffffffffffffff81111561054157610541616001565b60405190808252806020026020018201604052801561057a57816020015b610567615591565b81526020019060019003908161055f5790505b5090505f5b8251811015610603575f83828151811061059b5761059b616015565b602002602001015190506105ce60405180604001604052806007815260200166706f6f6c4b657960c81b81525082611ed4565b5f6105d98983611f09565b9050808484815181106105ee576105ee616015565b6020908102919091010152505060010161057f565b5095945050505050565b5f826001600160a01b031663f3903b9f61062684612367565b6040518263ffffffff1660e01b815260040161064491815260200190565b602060405180830381865afa15801561065f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102709190615fc7565b60605f610692868686866123eb565b90505f815167ffffffffffffffff8111156106af576106af616001565b6040519080825280602002602001820160405280156106e857816020015b6106d5615604565b8152602001906001900390816106cd5790505b5090505f5b8251811015610748575f83828151811061070957610709616015565b602002602001015190505f61071e8a83612473565b90508084848151811061073357610733616015565b602090810291909101015250506001016106ed565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161033490602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f61079d8888610903565b90505f6107aa8a836109aa565b90505f808060ff89166107e0576107c28a855f612a81565b9295509193506107d991508590508b5f8087612b70565b9050610810565b5f1960ff8a1601610810576107f68a855f612c3d565b92955091935061080d91508590505f8c8682612b70565b90505b5f61081a85612d06565b90505f8282116108335761082e828461603d565b61083d565b61083d838361603d565b90505f61084a8284612d9f565b90505f8484116108625761085d82616050565b610864565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a721604051602001610334906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f6108dc8686610903565b90505f6108e988836109aa565b90506108f78186865f612dda565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610924578183610927565b82825b6040519194509250610954906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b6109b2615657565b826109bb615657565b816001600160a01b03166391d4403c6040516020016109d990615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610a2d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a51919061606a565b610a5e5791506102189050565b816001600160a01b03166321f8a72185604051602001610a9e906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610ace929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b0291815260200190565b602060405180830381865afa158015610b1d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b419190615fac565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610bbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610bf391815260200190565b602060405180830381865afa158015610c0e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c329190615fc7565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610c88906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610cb8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610cec91815260200190565b602060405180830381865afa158015610d07573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d2b9190615fc7565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610d8c9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610dbc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610df091815260200190565b602060405180830381865afa158015610e0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e2f9190615fc7565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610e9a9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610eca929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610efe91815260200190565b602060405180830381865afa158015610f19573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3d9190615fc7565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610f9e9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610fce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100291815260200190565b602060405180830381865afa15801561101d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110419190615fc7565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016110be929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110f291815260200190565b602060405180830381865afa15801561110d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111319190615fc7565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a721908690608001604051602081830303815290604052805190602001206040516020016111a6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016111da91815260200190565b602060405180830381865afa1580156111f5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112199190615fac565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016112c391815260200190565b602060405180830381865afa1580156112de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113029190615fc7565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161135990602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001611389929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016113bd91815260200190565b602060405180830381865afa1580156113d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113fc9190615fc7565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161145e9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161148e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016114c291815260200190565b602060405180830381865afa1580156114dd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115019190615fc7565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161156d9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b6040516020818303038152906040528051906020012060405160200161159d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016115d191815260200190565b602060405180830381865afa1580156115ec573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116109190615fc7565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016116729060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116a2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016116d691815260200190565b602060405180830381865afa1580156116f1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117159190615fc7565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161176e90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161179e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016117d291815260200190565b602060405180830381865afa1580156117ed573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118119190615fc7565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161185f90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161188f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016118c391815260200190565b602060405180830381865afa1580156118de573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119029190615fac565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001611970906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016119a0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016119d491815260200190565b602060405180830381865afa1580156119ef573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a139190615fac565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611a76906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611aa6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611ada91815260200190565b602060405180830381865afa158015611af5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b199190615fc7565b60608201526040516001600160a01b0383169063bd02d0f5908690611b72906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611ba2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611bd691815260200190565b602060405180830381865afa158015611bf1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c159190615fc7565b6080820152949350505050565b5f5f5f5f611c2e61568b565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611c6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c929190615fc7565b6020820152611ca2875f80612f11565b506080840152506040820152611cba8760015f612f11565b5060a084015250606082015260408101516020820151611cdb918891612fbf565b61010082015260608101516020820151611cf6918891612fbf565b81610120018181525050611d366040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b815250826020015161307e565b611d6e604051806040016040528060128152602001710766172732e707269636552657365727665360741b815250826040015161307e565b611da660405180604001604052806012815260200171766172732e7072696365526573657276653160701b815250826060015161307e565b611dd96040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b81525082610100015161307e565b611e0c6040518060400160405280600c81526020016b766172732e616d6f756e743160a01b81525082610120015161307e565b610100810151610120820151608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611e5390615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611ead573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261029291908101906160ba565b611f05604051806040016040528060068152602001652573202d257360d01b81525083611f00846130a7565b6130d3565b5050565b611f11615591565b611f196156d4565b611f2384846109aa565b8152611f2e8461311f565b602082018190528151611f42915f90612f11565b608085015260a08401526040830152606082015280516020820151611f6a9190600190612f11565b61012085015261014084015260e08301526101008201528051611f8c90613162565b6101608381019190915260c0830191909152604080516103208101825283515151516001600160a01b039081166101a08301908152855151515184516395d89b4160e01b81529451939586959086019485946101c088019316916395d89b41916004808301925f9291908290030181865afa15801561200d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120349190810190616162565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015612081573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120a591906161f6565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612185573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526121ac9190810190616162565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121fc573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061222091906161f6565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061230090612d06565b815260200161230f8686613344565b6001600160a01b03168152602001612336835f015160600151660800000000000016151590565b151581526020016123478686613434565b81528251515160c001516020909101526101809091018190529392505050565b5f6040516020016123a1906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61240586612367565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa15801561244c573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102ed91908101906160ba565b61247b615604565b61248361573c565b61248d8484613536565b80825251805151602090910151516124a59190610903565b604082018190526124b79085906109aa565b6020820181905281516124cb918691613548565b50505050606082015260208101516124e290612d06565b610300820152805180515160209081015160e08401528083015151510151905161251891905f5b60200201516040015190613597565b60c0820152602081015160600151612530905f6135d8565b60a082015260e081015160c08201516125499190613606565b610100820181905260a08201516125609190613627565b61012082015260e081015160c0820151610100830151612581929190613643565b610140820152602081015181516103008301516125a292879290915f613660565b61016082015261014081015161018082015260e081015160c08201516101208301516125cf929190613643565b6101a08201526020810151606001516125e99060016135d8565b6101c08201528051805160209081015181015161020084015280830151518101510151905161261a91906001612509565b6101e0820181905261020082015161263191613606565b61022082018190526101c08201516126499190613627565b61024082018190526103008201516126619190613597565b6102608201526102008101516101e0820151610220830151612684929190613643565b610280820152602081015181516103008301516126a692879290916001613660565b6102a08201526102808101516102c08201526102008101516101e08201516102608301516126d5929190613643565b6102e082015280516126e6906137f5565b60808201528051516020015160e001516002146127de576127108161030001518260800151613606565b610320820181905261024082015161272791613597565b610340820181905260808201516103008301511161038083018190526102008301516101e084015161275893613837565b61036082018190526103a082015260e081015160a08201516127bf9186916127809190613627565b6127928460c001518560a00151613627565b6127a6856102000151866101c00151613627565b6127ba866101e00151876101c00151613627565b613859565b6103c082018190526103008201516127d79190613919565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612852573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128799190810190616162565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff16600281106128ce576128ce616015565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff166002811061291d5761291d616015565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa15801561297d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526129a49190810190616162565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612a8d615819565b612a98875f5f612f11565b506040840152508152612aad8760015f612f11565b5060608401525060208201528515612ad45787815f01818151612ad0919061603d565b9052505b606087015160381c61ffff166101208201819052612b03908990612afb9061271090613968565b612710612fbf565b610140820181905281516020830151612b2692612b219083906139c2565b612fbf565b608082018190526020820151612b3b91613968565b60c0820181905260408201516060830151610120840151612b5d908c90613a16565b9450945094509450505b93509350935093565b5f5f5f5f86118015612b80575083155b15612b8f575083905084612bc4565b5f87118015612b9c575084155b15612bab575085905082612bc4565b604051636331fab160e01b815260040160405180910390fd5b5f612bd389606001515f6135d8565b90505f612be58a6060015160016135d8565b90505f612c0385676765c793fa10079d601b1b612b2186600a6162f4565b90505f612c2185676765c793fa10079d601b1b612b2186600a6162f4565b9050612c2d8282612d9f565b9c9b505050505050505050505050565b5f5f5f5f612c49615819565b612c54875f5f612f11565b506040840152508152612c698760015f612f11565b5060608401525060208201528515612c91578781602001818151612c8d919061603d565b9052505b80516020820151612ca79190612b21818c6139c2565b608082018190528151612cb991613968565b60a0820152606087015160381c61ffff16610120820181905260a0820151612ce891612afb9061271090613968565b6040820151606083015161012084015160a0850151612b5d91613a16565b5f5f612d13835f5f612f11565b50505090505f612d258460015f612f11565b5050509050805f03612d3a57505f9392505050565b5f612d4985606001515f6135d8565b90505f612d5b866060015160016135d8565b90505f612d7985676765c793fa10079d601b1b612b2186600a6162f4565b90505f612d9785676765c793fa10079d601b1b612b2186600a6162f4565b90506108f782825b5f8115676765c793fa10079d601b1b60028404190484111715612dc0575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612de361568b565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612e23573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e479190615fc7565b6020820152612e57865f80612f11565b50505060c0820152612e6b8660015f612f11565b50505060e08201528215612ea657848160c001818151612e8b919061603d565b90525060e081018051859190612ea290839061603d565b9052505b80602001515f03612ed657612ecf6103e8612ec9612ec48888613a3a565b613aa0565b90613968565b8152612f07565b612f04612eec8683602001518460c00151612fbf565b612eff8684602001518560e00151612fbf565b613b80565b81525b5195945050505050565b5f5f5f5f5f875f01518760ff1660028110612f2e57612f2e616015565b602002015190505f612f408989613b95565b9050805f03612f5c575f5f5f5f95509550955095505050612b67565b5f612f6b838b60800151613c67565b90505f612f78838a613597565b90505f8915612f9057612f8b8284613968565b612f92565b5f5b9050612f9e83856162ff565b84612fa9858261603d565b919a509850965094505050505093509350935093565b5f838302815f1985870982811083820303915050805f03612ff357838281612fe957612fe9616312565b0492505050610270565b8084116130135760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b611f05604051806040016040528060068152602001652573202d257360d01b8152508383613c97565b6060610218826040516020016130bf91815260200190565b604051602081830303815290604052613cde565b61311a8383836040516024016130eb93929190616326565b60408051601f198184030181529190526020810180516001600160e01b0316632ced7cef60e01b179052613ede565b505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6131976040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6131a1845f613eea565b6020830152815260608401516131b7905f6135d8565b6060820181905281516131dc91676765c793fa10079d601b1b90612b2190600a6162f4565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613257573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061327b9190615fc7565b608082015261328b846001613eea565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa15801561330a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061332e9190615fc7565b60a0820181905260809091015194909350915050565b5f5f6133508484613f30565b9050806001600160a01b03166321f8a72184604051602001613391906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016133c1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133f591815260200190565b602060405180830381865afa158015613410573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102929190615fac565b5f5f6134408484613f30565b9050806001600160a01b031663bd02d0f5846040516020016134939060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016134c3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016134f791815260200190565b602060405180830381865afa158015613512573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102929190615fc7565b61353e61586e565b6102708383613feb565b5f5f5f5f5f61355687612d06565b90506135648787835f615209565b909350915081613575575f1961357f565b61357f8383612d9f565b945061358a886104a6565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176135b7575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff8516016135f8575060ff60601b19905060605b90198416901c905092915050565b5f81831161361d57613618838361603d565b610270565b610270828461603d565b5f61027083676765c793fa10079d601b1b612b2185600a6162f4565b5f8284116136595761365482616050565b610292565b5092915050565b5f61369a6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6136a68686865f615209565b602083015280825215806136d75750845160ff8416600281106136cb576136cb616015565b6020020151602001515f145b156136e5575f9150506102ed565b6136ee87615375565b60408201819052602082015161370391613597565b608082018190528151101561371b575f9150506102ed565b6080810151815161372c919061603d565b8160600181815250506137438660600151846135d8565b60a08201819052606082015161376f9161375e90600a6162f4565b676765c793fa10079d601b1b612fbf565b60c08201525f1960ff8416016137945760c081015161378e9085612d9f565b60c08201525b845160ff8416600281106137aa576137aa616015565b6020020151602001518160c0015111156137e757845160ff8416600281106137d4576137d4616015565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901613815575051602001516060015190565b81516020015160e00151613830575051602001516080015190565b505f919050565b5f841515838511146138515761384c82616050565b6102ed565b509392505050565b5f5f613864876104a6565b90505f6138718287613597565b90505f61387e8386613597565b90505f61388b898461635e565b90505f613898838961635e565b90505f6138a4836153bb565b90505f6138b0836153bb565b90505f841380156138c057505f83125b806138d457505f841280156138d457505f83135b156138e8575f9750505050505050506102ed565b805f036138fe575f9750505050505050506102ed565b6139088282612d9f565b9d9c50505050505050505050505050565b5f815f0361392857505f610218565b5f82841161393f5761393a848461603d565b613949565b613949838561603d565b90505f6139568285612d9f565b90508385116102925761384c81616050565b5f82613974838261603d565b91508111156102185760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826139ce83826162ff565b91508110156102185760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b60448201526064016139b9565b5f81156113881983900484111517613a2c575f5ffd5b506127109102611388010490565b5f811580613a5d57508282613a4f818361637d565b9250613a5b9083616394565b145b6102185760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b60448201526064016139b9565b5f815f03613aaf57505f919050565b5f6001613abb846153d0565b901c6001901b90506001818481613ad457613ad4616312565b048201901c90506001818481613aec57613aec616312565b048201901c90506001818481613b0457613b04616312565b048201901c90506001818481613b1c57613b1c616312565b048201901c90506001818481613b3457613b34616312565b048201901c90506001818481613b4c57613b4c616312565b048201901c90506001818481613b6457613b64616312565b048201901c905061027081828581613b7e57613b7e616312565b045b5f818310613b8e5781610270565b5090919050565b5f5f835f01518360ff1660028110613baf57613baf616015565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015613c08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c2c9190615fc7565b9050805f03613c3f575f92505050610218565b606082015160c0830151613c53828461603d565b613c5d919061603d565b9695505050505050565b5f8260a001515f03613c7a57505f610218565b5f613c858484615463565b60a08501519091506102929082613597565b61311a838383604051602401613caf939291906163b3565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052613ede565b60408051808201909152601081526f181899199a1a9b1b9c1cb0b131b232b360811b60208201528151606091905f90613d1890600261637d565b613d239060026162ff565b67ffffffffffffffff811115613d3b57613d3b616001565b6040519080825280601f01601f191660200182016040528015613d65576020820181803683370190505b509050600360fc1b815f81518110613d7f57613d7f616015565b60200101906001600160f81b03191690815f1a905350600f60fb1b81600181518110613dad57613dad616015565b60200101906001600160f81b03191690815f1a9053505f5b845181101561385157826004868381518110613de357613de3616015565b016020015182516001600160f81b031990911690911c60f81c908110613e0b57613e0b616015565b01602001516001600160f81b03191682613e2683600261637d565b613e319060026162ff565b81518110613e4157613e41616015565b60200101906001600160f81b03191690815f1a90535082858281518110613e6a57613e6a616015565b602091010151815160f89190911c600f16908110613e8a57613e8a616015565b01602001516001600160f81b03191682613ea583600261637d565b613eb09060036162ff565b81518110613ec057613ec0616015565b60200101906001600160f81b03191690815f1a905350600101613dc5565b613ee7816154a6565b50565b5f5f5f613f17855f01518560ff1660028110613f0857613f08616015565b60200201518660800151613c67565b90505f613f248686613b95565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001613f5390615fde565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015613fa7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fcb919061606a565b61027057604051637357d91f60e01b8152600481018490526024016139b9565b613ff361586e565b82613ffc61586e565b816001600160a01b03166391d4403c60405160200161403c906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614090573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140b4919061606a565b6140c15791506102189050565b816001600160a01b031663bd02d0f5856040516020016140fb906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161412b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161415f91815260200190565b602060405180830381865afa15801561417a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061419e9190615fc7565b816020018181525050816001600160a01b03166321f8a721856040516020016141e6906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614216929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161424a91815260200190565b602060405180830381865afa158015614265573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142899190615fac565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016142e5906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614315929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161434991815260200190565b602060405180830381865afa158015614364573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143889190615fac565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614403929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161443791815260200190565b602060405180830381865afa158015614452573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906144769190615fc7565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016144ca9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b604051602081830303815290604052805190602001206040516020016144fa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161452e91815260200190565b602060405180830381865afa158015614549573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061456d9190615fc7565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016145c7906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016145f7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161462b91815260200190565b602060405180830381865afa158015614646573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061466a9190615fc7565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016146c3906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016146f3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161472791815260200190565b602060405180830381865afa158015614742573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147669190615fc7565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016147ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161482091815260200190565b602060405180830381865afa15801561483b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061485f9190615fc7565b81515f60200201516080018181525050816001600160a01b031663bd02d0f5856040516020016148b9906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b604051602081830303815290604052805190602001206040516020016148e9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161491d91815260200190565b602060405180830381865afa158015614938573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061495c9190615fc7565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016149cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a0391815260200190565b602060405180830381865afa158015614a1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a429190615fc7565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614ab6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614aea91815260200190565b602060405180830381865afa158015614b05573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b299190615fac565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bd091815260200190565b602060405180830381865afa158015614beb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c0f9190615fc7565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001614c649060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c94929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cc891815260200190565b602060405180830381865afa158015614ce3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d079190615fc7565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001614d6290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d92929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614dc691815260200190565b602060405180830381865afa158015614de1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e059190615fc7565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001614e5f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001614e8f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ec391815260200190565b602060405180830381865afa158015614ede573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f029190615fc7565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001614f649060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001614f94929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fc891815260200190565b602060405180830381865afa158015614fe3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150079190615fc7565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161506290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615092929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150c691815260200190565b602060405180830381865afa1580156150e1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151059190615fc7565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615154906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615184929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151b891815260200190565b602060405180830381865afa1580156151d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151f79190615fc7565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b038681169116146152ac575f5f6152348a8a5f6154c5565b915091505f6152505f8c606001516135d890919063ffffffff16565b90505f61526e84676765c793fa10079d601b1b612b2185600a6162f4565b90505f61528c84676765c793fa10079d601b1b612b2186600a6162f4565b905061529882886162ff565b96506152a481876162ff565b955050505050505b865160200151516001600160a01b03868116911614615368575f5f6152d38a8a60016154c5565b915091505f6152f060018c606001516135d890919063ffffffff16565b90505f61530e84676765c793fa10079d601b1b612b2185600a6162f4565b90505f61532c84676765c793fa10079d601b1b612b2186600a6162f4565b90505f615339838d613597565b90505f615346838e613597565b9050615352828a6162ff565b985061535e81896162ff565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f5604051602001610433906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f8212156153cc57815f03610218565b5090565b5f80608083901c156153e457608092831c92015b604083901c156153f657604092831c92015b602083901c1561540857602092831c92015b601083901c1561541a57601092831c92015b600883901c1561542c57600892831c92015b600483901c1561543e57600492831c92015b600283901c1561545057600292831c92015b600183901c156102185760010192915050565b5f42820361547657506020820151610218565b5f61548584604001518461555d565b905061549e84602001518261359790919063ffffffff16565b915050610218565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b5f5f5f845f01518460ff16600281106154e0576154e0616015565b60200201516040015190505f615516875f01518660ff166002811061550757615507616015565b60200201518860800151615463565b9050811561552d576155288282613597565b61552f565b5f5b865190935060ff86166002811061554857615548616015565b60200201516020015193505050935093915050565b5f80615569834261603d565b615573908561637d565b6301e133809004905061029281676765c793fa10079d601b1b6162ff565b6040518061016001604052806155a5615894565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81525090565b60405180610120016040528061561861591b565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a0016040528061566a615996565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101a001604052806156e8615657565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f8152602001615737615591565b905290565b60405180610400016040528061575061586e565b815260200161575d615657565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061018001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806158816159fd565b81525f6020820181905260409091015290565b60405180604001604052806002905b6159056040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816158a35790505090565b60405180604001604052806002905b6159806040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161592a5790505090565b60405180604001604052806002905b6159e76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816159a55790505090565b60405180604001604052806002905b615a556040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b815260200190600190039081615a0c5790505090565b6001600160a01b0381168114613ee7575f5ffd5b5f60208284031215615a8f575f5ffd5b813561027081615a6b565b5f5f5f5f60808587031215615aad575f5ffd5b8435615ab881615a6b565b93506020850135615ac881615a6b565b92506040850135615ad881615a6b565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b6002811015615be1578383038752815180516001600160a01b0316845260208101516101806020860152615b57610180860182615ae8565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615b1f565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615cf557603f19878603018452815180516101608752615c3a610160880182615b16565b90506020820151615c5660208901826001600160a01b03169052565b506040820151615c7160408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151615cb460e08901826001600160a01b03169052565b50610100820151615cca61010089018215159052565b5061012082810151908801526101409182015191909601526020938401939190910190600101615c12565b50929695505050505050565b5f5f60408385031215615d12575f5ffd5b8235615d1d81615a6b565b91506020830135615d2d81615a6b565b809150509250929050565b5f8260408101835f5b6002811015615be1578383038752815180516001600160a01b0316845260208101516101406020860152615d79610140860182615ae8565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050615d41565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015615cf557603f19878603018452815180516101208752615e39610120880182615d38565b9050602082015160208801526040820151615e5f60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101615e11565b5f5f5f60608486031215615ec3575f5ffd5b8335615ece81615a6b565b95602085013595506040909401359392505050565b60ff81168114613ee7575f5ffd5b5f5f5f5f5f60a08688031215615f05575f5ffd5b8535615f1081615a6b565b94506020860135615f2081615a6b565b93506040860135615f3081615a6b565b9250606086013591506080860135615f4781615ee3565b809150509295509295909350565b5f5f5f5f5f60a08688031215615f69575f5ffd5b8535615f7481615a6b565b94506020860135615f8481615a6b565b93506040860135615f9481615a6b565b94979396509394606081013594506080013592915050565b5f60208284031215615fbc575f5ffd5b815161027081615a6b565b5f60208284031215615fd7575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561021857610218616029565b5f600160ff1b820161606457616064616029565b505f0390565b5f6020828403121561607a575f5ffd5b81518015158114610270575f5ffd5b604051601f8201601f1916810167ffffffffffffffff811182821017156160b2576160b2616001565b604052919050565b5f602082840312156160ca575f5ffd5b815167ffffffffffffffff8111156160e0575f5ffd5b8201601f810184136160f0575f5ffd5b805167ffffffffffffffff81111561610a5761610a616001565b8060051b61611a60208201616089565b91825260208184018101929081019087841115616135575f5ffd5b6020850194505b8385101561615757845182526020948501949091019061613c565b979650505050505050565b5f60208284031215616172575f5ffd5b815167ffffffffffffffff811115616188575f5ffd5b8201601f81018413616198575f5ffd5b805167ffffffffffffffff8111156161b2576161b2616001565b6161c5601f8201601f1916602001616089565b8181528560208385010111156161d9575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215616206575f5ffd5b815161027081615ee3565b6001815b600184111561624c5780850481111561623057616230616029565b600184161561623e57908102905b60019390931c928002616215565b935093915050565b5f8261626257506001610218565b8161626e57505f610218565b8160018114616284576002811461628e576162aa565b6001915050610218565b60ff84111561629f5761629f616029565b50506001821b610218565b5060208310610133831016604e8410600b84101617156162cd575081810a610218565b6162d95f198484616211565b805f19048211156162ec576162ec616029565b029392505050565b5f6102708383616254565b8082018082111561021857610218616029565b634e487b7160e01b5f52601260045260245ffd5b606081525f6163386060830186615ae8565b828103602084015261634a8186615ae8565b90508281036040840152613c5d8185615ae8565b8181035f83128015838313168383128216171561365957613659616029565b808202811582820484141761021857610218616029565b5f826163ae57634e487b7160e01b5f52601260045260245ffd5b500490565b606081525f6163c56060830186615ae8565b82810360208401526163d78186615ae8565b91505082604083015294935050505056fea2646970667358221220bac0d7931d71246413cc40f02d9cf0b2b6dfea7621fe1c075c81b5dd998b267d64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01tW\x80cx\xF2\x12\xD1\x14a\x01\x94W\x80c\x8Flz<\x14a\x01\xA7W\x80c\xD2\x8B\n\x15\x14a\x01\xBAW\x80c\xE35\xAD\xB7\x14a\x01\xE8W\x80c\xF6\x8Aq1\x14a\x01\xFBW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xB5W\x80c1{P\xEC\x14a\0\xE5W\x80cP\xEDY-\x14a\x01\rW\x80cW\xB9\x1C\xA6\x14a\x01.W\x80cZoW\x10\x14a\x01AW\x80c\\9\xF4g\x14a\x01TW[__\xFD[a\0\xC8a\0\xC36`\x04aZ\x7FV[a\x02\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF8a\0\xF36`\x04aZ\x9AV[a\x02\x1EV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xDCV[a\x01 a\x01\x1B6`\x04aZ\x7FV[a\x029V[`@Q\x90\x81R` \x01a\0\xDCV[a\x01 a\x01<6`\x04aZ\x7FV[a\x02CV[a\x01 a\x01O6`\x04aZ\x7FV[a\x02MV[a\x01ga\x01b6`\x04aZ\x7FV[a\x02WV[`@Qa\0\xDC\x91\x90a[\xECV[a\x01\x87a\x01\x826`\x04a]\x01V[a\x02wV[`@Qa\0\xDC\x91\x90a]\xEBV[a\0\xC8a\x01\xA26`\x04aZ\x7FV[a\x02\x9AV[a\x01ga\x01\xB56`\x04a^\xB1V[a\x02\xA4V[a\x01\xCDa\x01\xC86`\x04a^\xF1V[a\x02\xB1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xDCV[a\0\xC8a\x01\xF66`\x04aZ\x7FV[a\x02\xD2V[a\x01 a\x02\t6`\x04a_UV[a\x02\xDCV[_a\x02\x18\x82a\x02\xF6V[\x92\x91PPV[__a\x02,\x86\x86\x86\x86a\x03\xA7V[\x91P\x91P\x94P\x94\x92PPPV[_a\x02\x18\x82a\x03\xE2V[_a\x02\x18\x82a\x04\xA6V[_a\x02\x18\x82a\x04\xF7V[``_a\x02c\x83a\x04\xF7V[\x90Pa\x02p\x83_\x83a\x05\x16V[\x93\x92PPPV[``_a\x02\x84\x84\x84a\x06\rV[\x90Pa\x02\x92\x84\x84_\x84a\x06\x83V[\x94\x93PPPPV[_a\x02\x18\x82a\x07SV[``a\x02\x92\x84\x84\x84a\x05\x16V[___a\x02\xC1\x88\x88\x88\x88\x88a\x07\x8FV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x02\x18\x82a\x08\x7FV[_a\x02\xEA\x86\x86\x86\x86\x86a\x08\xD0V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a_\xACV[___a\x03\xB4\x86\x86a\t\x03V[\x90P_a\x03\xC1\x88\x83a\t\xAAV[\x90P__a\x03\xCF\x83\x88a\x1C\"V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04g\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x18\x91\x90a_\xC7V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x043\x90a_\xDEV[``_a\x05$\x85\x85\x85a\x1E3V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05AWa\x05Aa`\x01V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05zW\x81` \x01[a\x05gaU\x91V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05_W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\x03W_\x83\x82\x81Q\x81\x10a\x05\x9BWa\x05\x9Ba`\x15V[` \x02` \x01\x01Q\x90Pa\x05\xCE`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RP\x82a\x1E\xD4V[_a\x05\xD9\x89\x83a\x1F\tV[\x90P\x80\x84\x84\x81Q\x81\x10a\x05\xEEWa\x05\xEEa`\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x7FV[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x06&\x84a#gV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02p\x91\x90a_\xC7V[``_a\x06\x92\x86\x86\x86\x86a#\xEBV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xAFWa\x06\xAFa`\x01V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xE8W\x81` \x01[a\x06\xD5aV\x04V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xCDW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x07HW_\x83\x82\x81Q\x81\x10a\x07\tWa\x07\ta`\x15V[` \x02` \x01\x01Q\x90P_a\x07\x1E\x8A\x83a$sV[\x90P\x80\x84\x84\x81Q\x81\x10a\x073Wa\x073a`\x15V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06\xEDV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x07\x9D\x88\x88a\t\x03V[\x90P_a\x07\xAA\x8A\x83a\t\xAAV[\x90P_\x80\x80`\xFF\x89\x16a\x07\xE0Wa\x07\xC2\x8A\x85_a*\x81V[\x92\x95P\x91\x93Pa\x07\xD9\x91P\x85\x90P\x8B_\x80\x87a+pV[\x90Pa\x08\x10V[_\x19`\xFF\x8A\x16\x01a\x08\x10Wa\x07\xF6\x8A\x85_a,=V[\x92\x95P\x91\x93Pa\x08\r\x91P\x85\x90P_\x8C\x86\x82a+pV[\x90P[_a\x08\x1A\x85a-\x06V[\x90P_\x82\x82\x11a\x083Wa\x08.\x82\x84a`=V[a\x08=V[a\x08=\x83\x83a`=V[\x90P_a\x08J\x82\x84a-\x9FV[\x90P_\x84\x84\x11a\x08bWa\x08]\x82a`PV[a\x08dV[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x034\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08\xDC\x86\x86a\t\x03V[\x90P_a\x08\xE9\x88\x83a\t\xAAV[\x90Pa\x08\xF7\x81\x86\x86_a-\xDAV[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\t$W\x81\x83a\t'V[\x82\x82[`@Q\x91\x94P\x92Pa\tT\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\xB2aVWV[\x82a\t\xBBaVWV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t\xD9\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n-W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nQ\x91\x90a`jV[a\n^W\x91Pa\x02\x18\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\n\x9E\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BA\x91\x90a_\xACV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C2\x91\x90a_\xC7V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\x88\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xEC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x07W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r+\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\x8C\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E/\x91\x90a_\xC7V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\x9A\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xCA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xFE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x19W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F=\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0F\x9E\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10A\x91\x90a_\xC7V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xBE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xF2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x111\x91\x90a_\xC7V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x19\x91\x90a_\xACV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x02\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13Y\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xBD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFC\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14^\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x8E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xC2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x01\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15m\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x9D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xD1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x10\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16r\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x15\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17n\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x11\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18_\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x02\x91\x90a_\xACV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x19p\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x13\x91\x90a_\xACV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x1Av\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xA6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x19\x91\x90a_\xC7V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1Br\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xA2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xD6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x15\x91\x90a_\xC7V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1C.aV\x8BV[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x92\x91\x90a_\xC7V[` \x82\x01Ra\x1C\xA2\x87_\x80a/\x11V[P`\x80\x84\x01RP`@\x82\x01Ra\x1C\xBA\x87`\x01_a/\x11V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C\xDB\x91\x88\x91a/\xBFV[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1C\xF6\x91\x88\x91a/\xBFV[\x81a\x01 \x01\x81\x81RPPa\x1D6`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa0~V[a\x1Dn`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa0~V[a\x1D\xA6`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa0~V[a\x1D\xD9`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa0~V[a\x1E\x0C`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa0~V[a\x01\0\x81\x01Qa\x01 \x82\x01Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1ES\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xADW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x92\x91\x90\x81\x01\x90a`\xBAV[a\x1F\x05`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83a\x1F\0\x84a0\xA7V[a0\xD3V[PPV[a\x1F\x11aU\x91V[a\x1F\x19aV\xD4V[a\x1F#\x84\x84a\t\xAAV[\x81Ra\x1F.\x84a1\x1FV[` \x82\x01\x81\x90R\x81Qa\x1FB\x91_\x90a/\x11V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x1Fj\x91\x90`\x01\x90a/\x11V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa\x1F\x8C\x90a1bV[a\x01`\x83\x81\x01\x91\x90\x91R`\xC0\x83\x01\x91\x90\x91R`@\x80Qa\x03 \x81\x01\x82R\x83QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xA0\x83\x01\x90\x81R\x85QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x95\x86\x95\x90\x86\x01\x94\x85\x94a\x01\xC0\x88\x01\x93\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a \rW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra 4\x91\x90\x81\x01\x90aabV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a \x81W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xA5\x91\x90aa\xF6V[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\x85W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAC\x91\x90\x81\x01\x90aabV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xFCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\" \x91\x90aa\xF6V[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a#\0\x90a-\x06V[\x81R` \x01a#\x0F\x86\x86a3DV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a#6\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a#G\x86\x86a44V[\x81R\x82QQQ`\xC0\x01Q` \x90\x91\x01Ra\x01\x80\x90\x91\x01\x81\x90R\x93\x92PPPV[_`@Q` \x01a#\xA1\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a$\x05\x86a#gV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$LW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xED\x91\x90\x81\x01\x90a`\xBAV[a${aV\x04V[a$\x83aW<V[a$\x8D\x84\x84a56V[\x80\x82RQ\x80QQ` \x90\x91\x01QQa$\xA5\x91\x90a\t\x03V[`@\x82\x01\x81\x90Ra$\xB7\x90\x85\x90a\t\xAAV[` \x82\x01\x81\x90R\x81Qa$\xCB\x91\x86\x91a5HV[PPPP``\x82\x01R` \x81\x01Qa$\xE2\x90a-\x06V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa%\x18\x91\x90_[` \x02\x01Q`@\x01Q\x90a5\x97V[`\xC0\x82\x01R` \x81\x01Q``\x01Qa%0\x90_a5\xD8V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa%I\x91\x90a6\x06V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa%`\x91\x90a6'V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa%\x81\x92\x91\x90a6CV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%\xA2\x92\x87\x92\x90\x91_a6`V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa%\xCF\x92\x91\x90a6CV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa%\xE9\x90`\x01a5\xD8V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa&\x1A\x91\x90`\x01a%\tV[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa&1\x91a6\x06V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa&I\x91\x90a6'V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&a\x91\x90a5\x97V[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa&\x84\x92\x91\x90a6CV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa&\xA6\x92\x87\x92\x90\x91`\x01a6`V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa&\xD5\x92\x91\x90a6CV[a\x02\xE0\x82\x01R\x80Qa&\xE6\x90a7\xF5V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a'\xDEWa'\x10\x81a\x03\0\x01Q\x82`\x80\x01Qa6\x06V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa''\x91a5\x97V[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa'X\x93a87V[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa'\xBF\x91\x86\x91a'\x80\x91\x90a6'V[a'\x92\x84`\xC0\x01Q\x85`\xA0\x01Qa6'V[a'\xA6\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa6'V[a'\xBA\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa6'V[a8YV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa'\xD7\x91\x90a9\x19V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a(RW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(y\x91\x90\x81\x01\x90aabV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a(\xCEWa(\xCEa`\x15V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a)\x1DWa)\x1Da`\x15V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)}W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)\xA4\x91\x90\x81\x01\x90aabV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a*\x8DaX\x19V[a*\x98\x87__a/\x11V[P`@\x84\x01RP\x81Ra*\xAD\x87`\x01_a/\x11V[P``\x84\x01RP` \x82\x01R\x85\x15a*\xD4W\x87\x81_\x01\x81\x81Qa*\xD0\x91\x90a`=V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra+\x03\x90\x89\x90a*\xFB\x90a'\x10\x90a9hV[a'\x10a/\xBFV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa+&\x92a+!\x90\x83\x90a9\xC2V[a/\xBFV[`\x80\x82\x01\x81\x90R` \x82\x01Qa+;\x91a9hV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa+]\x90\x8C\x90a:\x16V[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a+\x80WP\x83\x15[\x15a+\x8FWP\x83\x90P\x84a+\xC4V[_\x87\x11\x80\x15a+\x9CWP\x84\x15[\x15a+\xABWP\x85\x90P\x82a+\xC4V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a+\xD3\x89``\x01Q_a5\xD8V[\x90P_a+\xE5\x8A``\x01Q`\x01a5\xD8V[\x90P_a,\x03\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_a,!\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90Pa,-\x82\x82a-\x9FV[\x9C\x9BPPPPPPPPPPPPV[____a,IaX\x19V[a,T\x87__a/\x11V[P`@\x84\x01RP\x81Ra,i\x87`\x01_a/\x11V[P``\x84\x01RP` \x82\x01R\x85\x15a,\x91W\x87\x81` \x01\x81\x81Qa,\x8D\x91\x90a`=V[\x90RP[\x80Q` \x82\x01Qa,\xA7\x91\x90a+!\x81\x8Ca9\xC2V[`\x80\x82\x01\x81\x90R\x81Qa,\xB9\x91a9hV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa,\xE8\x91a*\xFB\x90a'\x10\x90a9hV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa+]\x91a:\x16V[__a-\x13\x83__a/\x11V[PPP\x90P_a-%\x84`\x01_a/\x11V[PPP\x90P\x80_\x03a-:WP_\x93\x92PPPV[_a-I\x85``\x01Q_a5\xD8V[\x90P_a-[\x86``\x01Q`\x01a5\xD8V[\x90P_a-y\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_a-\x97\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90Pa\x08\xF7\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a-\xC0W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a-\xE3aV\x8BV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.G\x91\x90a_\xC7V[` \x82\x01Ra.W\x86_\x80a/\x11V[PPP`\xC0\x82\x01Ra.k\x86`\x01_a/\x11V[PPP`\xE0\x82\x01R\x82\x15a.\xA6W\x84\x81`\xC0\x01\x81\x81Qa.\x8B\x91\x90a`=V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a.\xA2\x90\x83\x90a`=V[\x90RP[\x80` \x01Q_\x03a.\xD6Wa.\xCFa\x03\xE8a.\xC9a.\xC4\x88\x88a::V[a:\xA0V[\x90a9hV[\x81Ra/\x07V[a/\x04a.\xEC\x86\x83` \x01Q\x84`\xC0\x01Qa/\xBFV[a.\xFF\x86\x84` \x01Q\x85`\xE0\x01Qa/\xBFV[a;\x80V[\x81R[Q\x95\x94PPPPPV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10a/.Wa/.a`\x15V[` \x02\x01Q\x90P_a/@\x89\x89a;\x95V[\x90P\x80_\x03a/\\W____\x95P\x95P\x95P\x95PPPa+gV[_a/k\x83\x8B`\x80\x01Qa<gV[\x90P_a/x\x83\x8Aa5\x97V[\x90P_\x89\x15a/\x90Wa/\x8B\x82\x84a9hV[a/\x92V[_[\x90Pa/\x9E\x83\x85ab\xFFV[\x84a/\xA9\x85\x82a`=V[\x91\x9AP\x98P\x96P\x94PPPPP\x93P\x93P\x93P\x93V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a/\xF3W\x83\x82\x81a/\xE9Wa/\xE9ac\x12V[\x04\x92PPPa\x02pV[\x80\x84\x11a0\x13W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a\x1F\x05`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83a<\x97V[``a\x02\x18\x82`@Q` \x01a0\xBF\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra<\xDEV[a1\x1A\x83\x83\x83`@Q`$\x01a0\xEB\x93\x92\x91\x90ac&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c,\xED|\xEF`\xE0\x1B\x17\x90Ra>\xDEV[PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a1\x97`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a1\xA1\x84_a>\xEAV[` \x83\x01R\x81R``\x84\x01Qa1\xB7\x90_a5\xD8V[``\x82\x01\x81\x90R\x81Qa1\xDC\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a+!\x90`\nab\xF4V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2WW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2{\x91\x90a_\xC7V[`\x80\x82\x01Ra2\x8B\x84`\x01a>\xEAV[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3.\x91\x90a_\xC7V[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[__a3P\x84\x84a?0V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a3\x91\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a_\xACV[__a4@\x84\x84a?0V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a4\x93\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4\xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\xF7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x92\x91\x90a_\xC7V[a5>aXnV[a\x02p\x83\x83a?\xEBV[_____a5V\x87a-\x06V[\x90Pa5d\x87\x87\x83_aR\tV[\x90\x93P\x91P\x81a5uW_\x19a5\x7FV[a5\x7F\x83\x83a-\x9FV[\x94Pa5\x8A\x88a\x04\xA6V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a5\xB7W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a5\xF8WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a6\x1DWa6\x18\x83\x83a`=V[a\x02pV[a\x02p\x82\x84a`=V[_a\x02p\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[_\x82\x84\x11a6YWa6T\x82a`PV[a\x02\x92V[P\x92\x91PPV[_a6\x9A`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a6\xA6\x86\x86\x86_aR\tV[` \x83\x01R\x80\x82R\x15\x80a6\xD7WP\x84Q`\xFF\x84\x16`\x02\x81\x10a6\xCBWa6\xCBa`\x15V[` \x02\x01Q` \x01Q_\x14[\x15a6\xE5W_\x91PPa\x02\xEDV[a6\xEE\x87aSuV[`@\x82\x01\x81\x90R` \x82\x01Qa7\x03\x91a5\x97V[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a7\x1BW_\x91PPa\x02\xEDV[`\x80\x81\x01Q\x81Qa7,\x91\x90a`=V[\x81``\x01\x81\x81RPPa7C\x86``\x01Q\x84a5\xD8V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa7o\x91a7^\x90`\nab\xF4V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xBFV[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a7\x94W`\xC0\x81\x01Qa7\x8E\x90\x85a-\x9FV[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a7\xAAWa7\xAAa`\x15V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a7\xE7W\x84Q`\xFF\x84\x16`\x02\x81\x10a7\xD4Wa7\xD4a`\x15V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a8\x15WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa80WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a8QWa8L\x82a`PV[a\x02\xEDV[P\x93\x92PPPV[__a8d\x87a\x04\xA6V[\x90P_a8q\x82\x87a5\x97V[\x90P_a8~\x83\x86a5\x97V[\x90P_a8\x8B\x89\x84ac^V[\x90P_a8\x98\x83\x89ac^V[\x90P_a8\xA4\x83aS\xBBV[\x90P_a8\xB0\x83aS\xBBV[\x90P_\x84\x13\x80\x15a8\xC0WP_\x83\x12[\x80a8\xD4WP_\x84\x12\x80\x15a8\xD4WP_\x83\x13[\x15a8\xE8W_\x97PPPPPPPPa\x02\xEDV[\x80_\x03a8\xFEW_\x97PPPPPPPPa\x02\xEDV[a9\x08\x82\x82a-\x9FV[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03a9(WP_a\x02\x18V[_\x82\x84\x11a9?Wa9:\x84\x84a`=V[a9IV[a9I\x83\x85a`=V[\x90P_a9V\x82\x85a-\x9FV[\x90P\x83\x85\x11a\x02\x92Wa8L\x81a`PV[_\x82a9t\x83\x82a`=V[\x91P\x81\x11\x15a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a9\xCE\x83\x82ab\xFFV[\x91P\x81\x10\x15a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a9\xB9V[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a:,W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a:]WP\x82\x82a:O\x81\x83ac}V[\x92Pa:[\x90\x83ac\x94V[\x14[a\x02\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a9\xB9V[_\x81_\x03a:\xAFWP_\x91\x90PV[_`\x01a:\xBB\x84aS\xD0V[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a:\xD4Wa:\xD4ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a:\xECWa:\xECac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;\x04Wa;\x04ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;\x1CWa;\x1Cac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;4Wa;4ac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;LWa;Lac\x12V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a;dWa;dac\x12V[\x04\x82\x01\x90\x1C\x90Pa\x02p\x81\x82\x85\x81a;~Wa;~ac\x12V[\x04[_\x81\x83\x10a;\x8EW\x81a\x02pV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a;\xAFWa;\xAFa`\x15V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<,\x91\x90a_\xC7V[\x90P\x80_\x03a<?W_\x92PPPa\x02\x18V[``\x82\x01Q`\xC0\x83\x01Qa<S\x82\x84a`=V[a<]\x91\x90a`=V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a<zWP_a\x02\x18V[_a<\x85\x84\x84aTcV[`\xA0\x85\x01Q\x90\x91Pa\x02\x92\x90\x82a5\x97V[a1\x1A\x83\x83\x83`@Q`$\x01a<\xAF\x93\x92\x91\x90ac\xB3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90Ra>\xDEV[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90_\x90a=\x18\x90`\x02ac}V[a=#\x90`\x02ab\xFFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=;Wa=;a`\x01V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a=eW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a=\x7FWa=\x7Fa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a=\xADWa=\xADa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x84Q\x81\x10\x15a8QW\x82`\x04\x86\x83\x81Q\x81\x10a=\xE3Wa=\xE3a`\x15V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a>\x0BWa>\x0Ba`\x15V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a>&\x83`\x02ac}V[a>1\x90`\x02ab\xFFV[\x81Q\x81\x10a>AWa>Aa`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a>jWa>ja`\x15V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a>\x8AWa>\x8Aa`\x15V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a>\xA5\x83`\x02ac}V[a>\xB0\x90`\x03ab\xFFV[\x81Q\x81\x10a>\xC0Wa>\xC0a`\x15V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a=\xC5V[a>\xE7\x81aT\xA6V[PV[___a?\x17\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a?\x08Wa?\x08a`\x15V[` \x02\x01Q\x86`\x80\x01Qa<gV[\x90P_a?$\x86\x86a;\x95V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a?S\x90a_\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xCB\x91\x90a`jV[a\x02pW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a9\xB9V[a?\xF3aXnV[\x82a?\xFCaXnV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a@<\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x90W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xB4\x91\x90a`jV[a@\xC1W\x91Pa\x02\x18\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\xFB\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA+\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA_\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aAzW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\x9E\x91\x90a_\xC7V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aA\xE6\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aBJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\x89\x91\x90a_\xACV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aB\xE5\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aCI\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCdW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x88\x91\x90a_\xACV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDv\x91\x90a_\xC7V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aD\xCA\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xFA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE.\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aEIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aEm\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xC7\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFj\x91\x90a_\xC7V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xC3\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xF3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGf\x91\x90a_\xC7V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aH_\x91\x90a_\xC7V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\xB9\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\\\x91\x90a_\xC7V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJB\x91\x90a_\xC7V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\xEA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK)\x91\x90a_\xACV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL\x0F\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aLd\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x07\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aMb\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xC6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\x05\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aN_\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\x02\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOd\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\x94\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xC8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\x07\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPb\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xC6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x05\x91\x90a_\xC7V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQT\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\x84\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xB8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xF7\x91\x90a_\xC7V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aR\xACW__aR4\x8A\x8A_aT\xC5V[\x91P\x91P_aRP_\x8C``\x01Qa5\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aRn\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[\x90P_aR\x8C\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90PaR\x98\x82\x88ab\xFFV[\x96PaR\xA4\x81\x87ab\xFFV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aShW__aR\xD3\x8A\x8A`\x01aT\xC5V[\x91P\x91P_aR\xF0`\x01\x8C``\x01Qa5\xD8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aS\x0E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x85`\nab\xF4V[\x90P_aS,\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba+!\x86`\nab\xF4V[\x90P_aS9\x83\x8Da5\x97V[\x90P_aSF\x83\x8Ea5\x97V[\x90PaSR\x82\x8Aab\xFFV[\x98PaS^\x81\x89ab\xFFV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x043\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aS\xCCW\x81_\x03a\x02\x18V[P\x90V[_\x80`\x80\x83\x90\x1C\x15aS\xE4W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aS\xF6W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aT\x08W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aT\x1AW`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aT,W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aT>W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aTPW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x02\x18W`\x01\x01\x92\x91PPV[_B\x82\x03aTvWP` \x82\x01Qa\x02\x18V[_aT\x85\x84`@\x01Q\x84aU]V[\x90PaT\x9E\x84` \x01Q\x82a5\x97\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x02\x18V[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aT\xE0WaT\xE0a`\x15V[` \x02\x01Q`@\x01Q\x90P_aU\x16\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aU\x07WaU\x07a`\x15V[` \x02\x01Q\x88`\x80\x01QaTcV[\x90P\x81\x15aU-WaU(\x82\x82a5\x97V[aU/V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aUHWaUHa`\x15V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aUi\x83Ba`=V[aUs\x90\x85ac}V[c\x01\xE13\x80\x90\x04\x90Pa\x02\x92\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bab\xFFV[`@Q\x80a\x01`\x01`@R\x80aU\xA5aX\x94V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aV\x18aY\x1BV[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aVjaY\x96V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xA0\x01`@R\x80aV\xE8aVWV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aW7aU\x91V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aWPaXnV[\x81R` \x01aW]aVWV[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aX\x81aY\xFDV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\x05`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aX\xA3W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\x80`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aY*W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aY\xE7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aY\xA5W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aZU`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aZ\x0CW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a>\xE7W__\xFD[_` \x82\x84\x03\x12\x15aZ\x8FW__\xFD[\x815a\x02p\x81aZkV[____`\x80\x85\x87\x03\x12\x15aZ\xADW__\xFD[\x845aZ\xB8\x81aZkV[\x93P` \x85\x015aZ\xC8\x81aZkV[\x92P`@\x85\x015aZ\xD8\x81aZkV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15a[\xE1W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Ra[Wa\x01\x80\x86\x01\x82aZ\xE8V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pa[\x1FV[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\\\xF5W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01`\x87Ra\\:a\x01`\x88\x01\x82a[\x16V[\x90P` \x82\x01Qa\\V` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qa\\q`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qa\\\xB4`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qa\\\xCAa\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\\\x12V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a]\x12W__\xFD[\x825a]\x1D\x81aZkV[\x91P` \x83\x015a]-\x81aZkV[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15a[\xE1W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ra]ya\x01@\x86\x01\x82aZ\xE8V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pa]AV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\\\xF5W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ra^9a\x01 \x88\x01\x82a]8V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qa^_`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a^\x11V[___``\x84\x86\x03\x12\x15a^\xC3W__\xFD[\x835a^\xCE\x81aZkV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14a>\xE7W__\xFD[_____`\xA0\x86\x88\x03\x12\x15a_\x05W__\xFD[\x855a_\x10\x81aZkV[\x94P` \x86\x015a_ \x81aZkV[\x93P`@\x86\x015a_0\x81aZkV[\x92P``\x86\x015\x91P`\x80\x86\x015a_G\x81a^\xE3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15a_iW__\xFD[\x855a_t\x81aZkV[\x94P` \x86\x015a_\x84\x81aZkV[\x93P`@\x86\x015a_\x94\x81aZkV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15a_\xBCW__\xFD[\x81Qa\x02p\x81aZkV[_` \x82\x84\x03\x12\x15a_\xD7W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x18Wa\x02\x18a`)V[_`\x01`\xFF\x1B\x82\x01a`dWa`da`)V[P_\x03\x90V[_` \x82\x84\x03\x12\x15a`zW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02pW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a`\xB2Wa`\xB2a`\x01V[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15a`\xCAW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`\xE0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a`\xF0W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\nWaa\na`\x01V[\x80`\x05\x1Baa\x1A` \x82\x01a`\x89V[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aa5W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aaWW\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aa<V[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aarW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\x88W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aa\x98W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\xB2Waa\xB2a`\x01V[aa\xC5`\x1F\x82\x01`\x1F\x19\x16` \x01a`\x89V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aa\xD9W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ab\x06W__\xFD[\x81Qa\x02p\x81a^\xE3V[`\x01\x81[`\x01\x84\x11\x15abLW\x80\x85\x04\x81\x11\x15ab0Wab0a`)V[`\x01\x84\x16\x15ab>W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ab\x15V[\x93P\x93\x91PPV[_\x82abbWP`\x01a\x02\x18V[\x81abnWP_a\x02\x18V[\x81`\x01\x81\x14ab\x84W`\x02\x81\x14ab\x8EWab\xAAV[`\x01\x91PPa\x02\x18V[`\xFF\x84\x11\x15ab\x9FWab\x9Fa`)V[PP`\x01\x82\x1Ba\x02\x18V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ab\xCDWP\x81\x81\na\x02\x18V[ab\xD9_\x19\x84\x84ab\x11V[\x80_\x19\x04\x82\x11\x15ab\xECWab\xECa`)V[\x02\x93\x92PPPV[_a\x02p\x83\x83abTV[\x80\x82\x01\x80\x82\x11\x15a\x02\x18Wa\x02\x18a`)V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[``\x81R_ac8``\x83\x01\x86aZ\xE8V[\x82\x81\x03` \x84\x01RacJ\x81\x86aZ\xE8V[\x90P\x82\x81\x03`@\x84\x01Ra<]\x81\x85aZ\xE8V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a6YWa6Ya`)V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x18Wa\x02\x18a`)V[_\x82ac\xAEWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[``\x81R_ac\xC5``\x83\x01\x86aZ\xE8V[\x82\x81\x03` \x84\x01Rac\xD7\x81\x86aZ\xE8V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xBA\xC0\xD7\x93\x1Dq$d\x13\xCC@\xF0-\x9C\xF0\xB2\xB6\xDF\xEAv!\xFE\x1C\x07\\\x81\xB5\xDD\x99\x8B&}dsolcC\0\x08\x1C\x003",
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
        getMarginLevelThreshold(getMarginLevelThresholdCall),
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
            [87u8, 185u8, 28u8, 166u8],
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
        const COUNT: usize = 12usize;
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
