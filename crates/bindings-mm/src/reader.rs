///Module containing a contract's types and functions.
/**

```solidity
library ReaderPoolUtils {
    struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLiquidity; uint256 avaiableLoan; }
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
struct Asset { address token; string symbol; uint256 decimals; uint256 borrowIndex; uint256 borrowApy; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 poolBalance; uint256 priceLiquidity; uint256 avaiableLiquidity; uint256 avaiableLoan; }
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
        pub avaiableLoan: alloy::sol_types::private::primitives::aliases::U256,
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
                    value.avaiableLoan,
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
                    avaiableLoan: tuple.11,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.avaiableLoan),
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
                    "Asset(address token,string symbol,uint256 decimals,uint256 borrowIndex,uint256 borrowApy,uint256 totalCollateral,uint256 totalCollateralWithDebt,uint256 totalDebtScaled,uint256 poolBalance,uint256 priceLiquidity,uint256 avaiableLiquidity,uint256 avaiableLoan)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.avaiableLoan)
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
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avaiableLoan,
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
                    &rust.avaiableLiquidity,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avaiableLoan,
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
        uint256 avaiableLoan;
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
                "name": "avaiableLoan",
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
                "name": "avaiableLoan",
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
    ///0x6080604052348015600e575f5ffd5b50615d138061001c5f395ff3fe608060405234801561000f575f5ffd5b50600436106100a6575f3560e01c8063739118a41161006e578063739118a41461015657806378f212d1146101765780638f6c7a3c14610189578063d28b0a151461019c578063e335adb7146101ca578063f68a7131146101dd575f5ffd5b806328a0ccf4146100aa578063317b50ec146100da57806350ed592d146101025780635a6f5710146101235780635c39f46714610136575b5f5ffd5b6100bd6100b83660046153e9565b6101f0565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ed6100e8366004615404565b610200565b604080519283526020830191909152016100d1565b6101156101103660046153e9565b61021b565b6040519081526020016100d1565b6101156101313660046153e9565b610225565b6101496101443660046153e9565b61022f565b6040516100d19190615556565b610169610164366004615663565b61024f565b6040516100d1919061574d565b6100bd6101843660046153e9565b610272565b610149610197366004615813565b61027c565b6101af6101aa366004615853565b610289565b604080519384526020840192909252908201526060016100d1565b6100bd6101d83660046153e9565b6102aa565b6101156101eb3660046158b7565b6102b4565b5f6101fa826102ce565b92915050565b5f5f61020e8686868661037f565b9150915094509492505050565b5f6101fa826103ba565b5f6101fa8261047e565b60605f61023b8361047e565b9050610248835f8361049d565b9392505050565b60605f61025c848461056b565b905061026a84845f846105e1565b949350505050565b5f6101fa826106b1565b606061026a84848461049d565b5f5f5f61029988888888886106ed565b925092509250955095509592505050565b5f6101fa826107dd565b5f6102c2868686868661082e565b90505b95945050505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161034091815260200190565b602060405180830381865afa15801561035b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa919061590e565b5f5f5f61038c8686610861565b90505f6103998883610908565b90505f5f6103a78388611b80565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043f91815260200190565b602060405180830381865afa15801561045a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa9190615929565b5f816001600160a01b031663f3903b9f60405160200161040b90615940565b60605f6104ab858585611d33565b90505f815167ffffffffffffffff8111156104c8576104c8615963565b60405190808252806020026020018201604052801561050157816020015b6104ee614f13565b8152602001906001900390816104e65790505b5090505f5b8251811015610561575f83828151811061052257610522615977565b602002602001015190505f6105378983611dd4565b90508084848151811061054c5761054c615977565b60209081029190910101525050600101610506565b5095945050505050565b5f826001600160a01b031663f3903b9f610584846121fe565b6040518263ffffffff1660e01b81526004016105a291815260200190565b602060405180830381865afa1580156105bd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102489190615929565b60605f6105f086868686612282565b90505f815167ffffffffffffffff81111561060d5761060d615963565b60405190808252806020026020018201604052801561064657816020015b610633614f80565b81526020019060019003908161062b5790505b5090505f5b82518110156106a6575f83828151811061066757610667615977565b602002602001015190505f61067c8a8361230a565b90508084848151811061069157610691615977565b6020908102919091010152505060010161064b565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161030c90602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6106fb8888610861565b90505f6107088a83610908565b90505f808060ff891661073e576107208a855f612918565b92955091935061073791508590508b5f8087612a07565b905061076e565b5f1960ff8a160161076e576107548a855f612ad4565b92955091935061076b91508590505f8c8682612a07565b90505b5f61077885612b9d565b90505f8282116107915761078c828461599f565b61079b565b61079b838361599f565b90505f6107a88284612c36565b90505f8484116107c0576107bb826159b2565b6107c2565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f61083a8686610861565b90505f6108478883610908565b90506108558186865f612c71565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610882578183610885565b82825b60405191945092506108b2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610910614fd3565b82610919614fd3565b816001600160a01b03166391d4403c60405160200161093790615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561098b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109af91906159cc565b6109bc5791506101fa9050565b816001600160a01b03166321f8a721856040516020016109fc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a6091815260200190565b602060405180830381865afa158015610a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9f919061590e565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610b1d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b5191815260200190565b602060405180830381865afa158015610b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b909190615929565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610be6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4a91815260200190565b602060405180830381865afa158015610c65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c899190615929565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610cea9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4e91815260200190565b602060405180830381865afa158015610d69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8d9190615929565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610df89060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610e28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e5c91815260200190565b602060405180830381865afa158015610e77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9b9190615929565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610efc9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610f2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f6091815260200190565b602060405180830381865afa158015610f7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9f9190615929565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161101c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161105091815260200190565b602060405180830381865afa15801561106b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061108f9190615929565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611104929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113891815260200190565b602060405180830381865afa158015611153573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611177919061590e565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161122191815260200190565b602060405180830381865afa15801561123c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112609190615929565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016112b790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016112e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131b91815260200190565b602060405180830381865afa158015611336573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061135a9190615929565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016113bc9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161142091815260200190565b602060405180830381865afa15801561143b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145f9190615929565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016114cb9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016114fb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161152f91815260200190565b602060405180830381865afa15801561154a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061156e9190615929565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016115d09060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611600929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163491815260200190565b602060405180830381865afa15801561164f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116739190615929565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016116cc90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161173091815260200190565b602060405180830381865afa15801561174b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061176f9190615929565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016117bd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ed929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161182191815260200190565b602060405180830381865afa15801561183c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611860919061590e565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016118ce906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161193291815260200190565b602060405180830381865afa15801561194d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611971919061590e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016119d4906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a04929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a3891815260200190565b602060405180830381865afa158015611a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a779190615929565b60608201526040516001600160a01b0383169063bd02d0f5908690611ad0906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611b00929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b3491815260200190565b602060405180830381865afa158015611b4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b739190615929565b6080820152949350505050565b5f5f5f5f611b8c615007565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf09190615929565b6020820152611c00875f80612da8565b506080840152506040820152611c188760015f612da8565b5060a084015250606082015260408101516020820151611c39918891612e53565b61010082015260608101516020820151611c54918891612e53565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611d5390615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611dad573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261026a9190810190615a1c565b611ddc614f13565b611de4615050565b611dee8484610908565b8152611df984612f12565b602082018190528151611e0d915f90612da8565b60a085015260808401526040830152606082015280516020820151611e359190600190612da8565b60a085015261010084015260c083015260e0820152604080516103008101825282515151516001600160a01b039081166101808301908152845151515184516395d89b4160e01b81529451939485946101408601948594936101a08801939116916395d89b41916004808201925f929091908290030181865afa158015611ebe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611ee59190810190615ac4565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015611f32573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f569190615b58565b60ff16815286515151602090810151818301528751515160409081015181840152885151516060908101518185015289515151608090810151818601528a51515160a09081015181870152838c015160c0870152918b015160e08601528a015161010085015289015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612032573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120599190810190615ac4565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120cd9190615b58565b60ff168152865151602090810151810151818301528751518101516040908101518184015288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260c0808d01518188015260e0808e015190880152610100808e015190880152610120808e01519701969096529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906121a490612b9d565b81526020016121b38686612f5b565b6001600160a01b031681526020016121da835f015160600151660800000000000016151590565b151581526020016121eb868661304b565b9052610140909101819052905092915050565b5f604051602001612238906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61229c866121fe565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa1580156122e3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102c59190810190615a1c565b612312614f80565b61231a6150ac565b612324848461314d565b808252518051516020909101515161233c9190610861565b6040820181905261234e908590610908565b60208201819052815161236291869161315f565b505050506060820152602081015161237990612b9d565b610300820152805180515160209081015160e0840152808301515151015190516123af91905f5b602002015160400151906131ae565b60c08201526020810151606001516123c7905f6131ef565b60a082015260e081015160c08201516123e0919061321d565b610100820181905260a08201516123f7919061323e565b61012082015260e081015160c082015161010083015161241892919061325a565b6101408201526020810151815161030083015161243992879290915f613277565b61016082015261014081015161018082015260e081015160c082015161012083015161246692919061325a565b6101a08201526020810151606001516124809060016131ef565b6101c0820152805180516020908101518101516102008401528083015151810151015190516124b1919060016123a0565b6101e082018190526102008201516124c89161321d565b61022082018190526101c08201516124e0919061323e565b61024082018190526103008201516124f891906131ae565b6102608201526102008101516101e082015161022083015161251b92919061325a565b6102808201526020810151815161030083015161253d92879290916001613277565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161256c92919061325a565b6102e0820152805161257d9061340c565b60808201528051516020015160e00151600214612675576125a7816103000151826080015161321d565b61032082018190526102408201516125be916131ae565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516125ef9361344e565b61036082018190526103a082015260e081015160a0820151612656918691612617919061323e565b6126298460c001518560a0015161323e565b61263d856102000151866101c0015161323e565b612651866101e00151876101c0015161323e565b613475565b6103c0820181905261030082015161266e9190613535565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa1580156126e9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127109190810190615ac4565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061276557612765615977565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106127b4576127b4615977565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612814573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261283b9190810190615ac4565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612924615189565b61292f875f5f612da8565b5060408401525081526129448760015f612da8565b506060840152506020820152851561296b5787815f01818151612967919061599f565b9052505b606087015160381c61ffff16610120820181905261299a908990612992906127109061354e565b612710612e53565b6101408201819052815160208301516129bd926129b89083906135a8565b612e53565b6080820181905260208201516129d29161354e565b60c08201819052604082015160608301516101208401516129f4908c906135fc565b9450945094509450505b93509350935093565b5f5f5f5f86118015612a17575083155b15612a26575083905084612a5b565b5f87118015612a33575084155b15612a42575085905082612a5b565b604051636331fab160e01b815260040160405180910390fd5b5f612a6a89606001515f6131ef565b90505f612a7c8a6060015160016131ef565b90505f612a9a85676765c793fa10079d601b1b6129b886600a615c56565b90505f612ab885676765c793fa10079d601b1b6129b886600a615c56565b9050612ac48282612c36565b9c9b505050505050505050505050565b5f5f5f5f612ae0615189565b612aeb875f5f612da8565b506040840152508152612b008760015f612da8565b5060608401525060208201528515612b28578781602001818151612b24919061599f565b9052505b80516020820151612b3e91906129b8818c6135a8565b608082018190528151612b509161354e565b60a0820152606087015160381c61ffff16610120820181905260a0820151612b7f91612992906127109061354e565b6040820151606083015161012084015160a08501516129f4916135fc565b5f5f612baa835f5f612da8565b50505090505f612bbc8460015f612da8565b5050509050805f03612bd157505f9392505050565b5f612be085606001515f6131ef565b90505f612bf2866060015160016131ef565b90505f612c1085676765c793fa10079d601b1b6129b886600a615c56565b90505f612c2e85676765c793fa10079d601b1b6129b886600a615c56565b905061085582825b5f8115676765c793fa10079d601b1b60028404190484111715612c57575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612c7a615007565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612cba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cde9190615929565b6020820152612cee865f80612da8565b50505060c0820152612d028660015f612da8565b50505060e08201528215612d3d57848160c001818151612d22919061599f565b90525060e081018051859190612d3990839061599f565b9052505b80602001515f03612d6d57612d666103e8612d60612d5b8888613620565b613686565b9061354e565b8152612d9e565b612d9b612d838683602001518460c00151612e53565b612d968684602001518560e00151612e53565b613766565b81525b5195945050505050565b5f5f5f5f5f875f01518760ff1660028110612dc557612dc5615977565b602002015190505f612dd7898961377b565b9050805f03612df3575f5f5f5f955095509550955050506129fe565b5f612e02838b6080015161384d565b90505f8815612e255781612e16848b6131ae565b612e20919061599f565b612e27565b5f5b9050612e338284615c61565b83612e3e848261599f565b91995097509550935050505093509350935093565b5f838302815f1985870982811083820303915050805f03612e8757838281612e7d57612e7d615c74565b0492505050610248565b808411612ea75760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601590820152741093d49493d5d7d490551157d512149154d213d311605a1b604082015260600190565b5f5f612f67848461387d565b9050806001600160a01b03166321f8a72184604051602001612fa8906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612fd8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161300c91815260200190565b602060405180830381865afa158015613027573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a919061590e565b5f5f613057848461387d565b9050806001600160a01b031663bd02d0f5846040516020016130aa9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016130da929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161310e91815260200190565b602060405180830381865afa158015613129573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a9190615929565b6131556151d8565b6102488383613938565b5f5f5f5f5f61316d87612b9d565b905061317b8787835f614b56565b90935091508161318c575f19613196565b6131968383612c36565b94506131a188614cc2565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176131ce575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff85160161320f575060ff60601b19905060605b90198416901c905092915050565b5f8183116132345761322f838361599f565b610248565b610248828461599f565b5f61024883676765c793fa10079d601b1b6129b885600a615c56565b5f8284116132705761326b826159b2565b61026a565b5092915050565b5f6132b16040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6132bd8686865f614b56565b602083015280825215806132ee5750845160ff8416600281106132e2576132e2615977565b6020020151602001515f145b156132fc575f9150506102c5565b61330587614d13565b60408201819052602082015161331a916131ae565b6080820181905281511015613332575f9150506102c5565b60808101518151613343919061599f565b81606001818152505061335a8660600151846131ef565b60a0820181905260608201516133869161337590600a615c56565b676765c793fa10079d601b1b612e53565b60c08201525f1960ff8416016133ab5760c08101516133a59085612c36565b60c08201525b845160ff8416600281106133c1576133c1615977565b6020020151602001518160c0015111156133fe57845160ff8416600281106133eb576133eb615977565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f190161342c575051602001516060015190565b81516020015160e00151613447575051602001516080015190565b505f919050565b5f84801561345b57508284115b61346d57613468826159b2565b6102c5565b509392505050565b5f5f61348087614cc2565b90505f61348d82876131ae565b90505f61349a83866131ae565b90505f6134a78984615c88565b90505f6134b48389615c88565b90505f6134c083614d59565b90505f6134cc83614d59565b90505f841380156134dc57505f83125b806134f057505f841280156134f057505f83135b15613504575f9750505050505050506102c5565b805f0361351a575f9750505050505050506102c5565b6135248282612c36565b9d9c50505050505050505050505050565b5f82156135465761322f8284612c36565b505f92915050565b5f8261355a838261599f565b91508111156101fa5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826135b48382615c61565b91508110156101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161359f565b5f81156113881983900484111517613612575f5ffd5b506127109102611388010490565b5f811580613643575082826136358183615ca7565b92506136419083615cbe565b145b6101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161359f565b5f815f0361369557505f919050565b5f60016136a184614d6e565b901c6001901b905060018184816136ba576136ba615c74565b048201901c905060018184816136d2576136d2615c74565b048201901c905060018184816136ea576136ea615c74565b048201901c9050600181848161370257613702615c74565b048201901c9050600181848161371a5761371a615c74565b048201901c9050600181848161373257613732615c74565b048201901c9050600181848161374a5761374a615c74565b048201901c90506102488182858161376457613764615c74565b045b5f8183106137745781610248565b5090919050565b5f5f835f01518360ff166002811061379557613795615977565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156137ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138129190615929565b9050805f03613825575f925050506101fa565b606082015160c0830151613839828461599f565b613843919061599f565b9695505050505050565b5f8260a001515f0361386057505f6101fa565b5f61386b8484614e01565b60a085015190915061026a90826131ae565b5f5f839050806001600160a01b03166391d4403c6040516020016138a090615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156138f4573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061391891906159cc565b61024857604051637357d91f60e01b81526004810184905260240161359f565b6139406151d8565b826139496151d8565b816001600160a01b03166391d4403c604051602001613989906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156139dd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a0191906159cc565b613a0e5791506101fa9050565b816001600160a01b031663bd02d0f585604051602001613a48906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001613a78929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aac91815260200190565b602060405180830381865afa158015613ac7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613aeb9190615929565b816020018181525050816001600160a01b03166321f8a72185604051602001613b33906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b63929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b9791815260200190565b602060405180830381865afa158015613bb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bd6919061590e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613c32906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c9691815260200190565b602060405180830381865afa158015613cb1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd5919061590e565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613d50929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d8491815260200190565b602060405180830381865afa158015613d9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613dc39190615929565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613e179060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e47929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e7b91815260200190565b602060405180830381865afa158015613e96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613eba9190615929565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613f14906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f44929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f7891815260200190565b602060405180830381865afa158015613f93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fb79190615929565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001614010906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614040929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161407491815260200190565b602060405180830381865afa15801561408f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140b39190615929565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614139929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161416d91815260200190565b602060405180830381865afa158015614188573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906141ac9190615929565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614206906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614236929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161426a91815260200190565b602060405180830381865afa158015614285573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142a99190615929565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161431c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161435091815260200190565b602060405180830381865afa15801561436b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061438f9190615929565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614403929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161443791815260200190565b602060405180830381865afa158015614452573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614476919061590e565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161451d91815260200190565b602060405180830381865afa158015614538573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061455c9190615929565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016145b19060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016145e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161461591815260200190565b602060405180830381865afa158015614630573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146549190615929565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016146af90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016146df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161471391815260200190565b602060405180830381865afa15801561472e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147529190615929565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016147ac90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016147dc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161481091815260200190565b602060405180830381865afa15801561482b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061484f9190615929565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016148b19060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016148e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161491591815260200190565b602060405180830381865afa158015614930573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149549190615929565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016149af90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016149df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a1391815260200190565b602060405180830381865afa158015614a2e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a529190615929565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001614aa1906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ad1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b0591815260200190565b602060405180830381865afa158015614b20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b449190615929565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614614bf9575f5f614b818a8a5f614e47565b915091505f614b9d5f8c606001516131ef90919063ffffffff16565b90505f614bbb84676765c793fa10079d601b1b6129b885600a615c56565b90505f614bd984676765c793fa10079d601b1b6129b886600a615c56565b9050614be58288615c61565b9650614bf18187615c61565b955050505050505b865160200151516001600160a01b03868116911614614cb5575f5f614c208a8a6001614e47565b915091505f614c3d60018c606001516131ef90919063ffffffff16565b90505f614c5b84676765c793fa10079d601b1b6129b885600a615c56565b90505f614c7984676765c793fa10079d601b1b6129b886600a615c56565b90505f614c86838d6131ae565b90505f614c93838e6131ae565b9050614c9f828a615c61565b9850614cab8189615c61565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161040b9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215614d6a57815f036101fa565b5090565b5f80608083901c15614d8257608092831c92015b604083901c15614d9457604092831c92015b602083901c15614da657602092831c92015b601083901c15614db857601092831c92015b600883901c15614dca57600892831c92015b600483901c15614ddc57600492831c92015b600283901c15614dee57600292831c92015b600183901c156101fa5760010192915050565b5f428203614e14575060208201516101fa565b5f614e23846040015184614edf565b9050614e3c8460200151826131ae90919063ffffffff16565b9150506101fa565b50565b5f5f5f845f01518460ff1660028110614e6257614e62615977565b60200201516040015190505f614e98875f01518660ff1660028110614e8957614e89615977565b60200201518860800151614e01565b90508115614eaf57614eaa82826131ae565b614eb1565b5f5b865190935060ff861660028110614eca57614eca615977565b60200201516020015193505050935093915050565b5f80614eeb834261599f565b614ef59085615ca7565b6301e133809004905061026a81676765c793fa10079d601b1b615c61565b604051806101400160405280614f276151fe565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81525090565b604051806101200160405280614f94615285565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614fe6615300565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101600160405280615064614fd3565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016150a7614f13565b905290565b6040518061040001604052806150c06151d8565b81526020016150cd614fd3565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806151eb615367565b81525f6020820181905260409091015290565b60405180604001604052806002905b61526f6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161520d5790505090565b60405180604001604052806002905b6152ea6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816152945790505090565b60405180604001604052806002905b6153516040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161530f5790505090565b60405180604001604052806002905b6153bf6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816153765790505090565b6001600160a01b0381168114614e44575f5ffd5b5f602082840312156153f9575f5ffd5b8135610248816153d5565b5f5f5f5f60808587031215615417575f5ffd5b8435615422816153d5565b93506020850135615432816153d5565b92506040850135615442816153d5565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b600281101561554b578383038752815180516001600160a01b03168452602081015161018060208601526154c1610180860182615452565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615489565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561565757603f198786030184528151805161014087526155a4610140880182615480565b905060208201516155c060208901826001600160a01b03169052565b5060408201516155db60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e082015161561e60e08901826001600160a01b03169052565b5061010082015161563461010089018215159052565b50610120918201519690910195909552602093840193919091019060010161557c565b50929695505050505050565b5f5f60408385031215615674575f5ffd5b823561567f816153d5565b9150602083013561568f816153d5565b809150509250929050565b5f8260408101835f5b600281101561554b578383038752815180516001600160a01b03168452602081015161014060208601526156db610140860182615452565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e086015261010082015161010086015261012082015161012086015280945050506020820191506020870196506001810190506156a3565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561565757603f1987860301845281518051610120875261579b61012088018261569a565b90506020820151602088015260408201516157c160408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101615773565b5f5f5f60608486031215615825575f5ffd5b8335615830816153d5565b95602085013595506040909401359392505050565b60ff81168114614e44575f5ffd5b5f5f5f5f5f60a08688031215615867575f5ffd5b8535615872816153d5565b94506020860135615882816153d5565b93506040860135615892816153d5565b92506060860135915060808601356158a981615845565b809150509295509295909350565b5f5f5f5f5f60a086880312156158cb575f5ffd5b85356158d6816153d5565b945060208601356158e6816153d5565b935060408601356158f6816153d5565b94979396509394606081013594506080013592915050565b5f6020828403121561591e575f5ffd5b8151610248816153d5565b5f60208284031215615939575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156101fa576101fa61598b565b5f600160ff1b82016159c6576159c661598b565b505f0390565b5f602082840312156159dc575f5ffd5b81518015158114610248575f5ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715615a1457615a14615963565b604052919050565b5f60208284031215615a2c575f5ffd5b815167ffffffffffffffff811115615a42575f5ffd5b8201601f81018413615a52575f5ffd5b805167ffffffffffffffff811115615a6c57615a6c615963565b8060051b615a7c602082016159eb565b91825260208184018101929081019087841115615a97575f5ffd5b6020850194505b83851015615ab9578451825260209485019490910190615a9e565b979650505050505050565b5f60208284031215615ad4575f5ffd5b815167ffffffffffffffff811115615aea575f5ffd5b8201601f81018413615afa575f5ffd5b805167ffffffffffffffff811115615b1457615b14615963565b615b27601f8201601f19166020016159eb565b818152856020838501011115615b3b575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215615b68575f5ffd5b815161024881615845565b6001815b6001841115615bae57808504811115615b9257615b9261598b565b6001841615615ba057908102905b60019390931c928002615b77565b935093915050565b5f82615bc4575060016101fa565b81615bd057505f6101fa565b8160018114615be65760028114615bf057615c0c565b60019150506101fa565b60ff841115615c0157615c0161598b565b50506001821b6101fa565b5060208310610133831016604e8410600b8410161715615c2f575081810a6101fa565b615c3b5f198484615b73565b805f1904821115615c4e57615c4e61598b565b029392505050565b5f6102488383615bb6565b808201808211156101fa576101fa61598b565b634e487b7160e01b5f52601260045260245ffd5b8181035f8312801583831316838312821617156132705761327061598b565b80820281158282048414176101fa576101fa61598b565b5f82615cd857634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220c2fcebff608d55e370fec1c2d8f282d38c7b1775c7714ef025a3010fa0c302bf64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa]\x13\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01VW\x80cx\xF2\x12\xD1\x14a\x01vW\x80c\x8Flz<\x14a\x01\x89W\x80c\xD2\x8B\n\x15\x14a\x01\x9CW\x80c\xE35\xAD\xB7\x14a\x01\xCAW\x80c\xF6\x8Aq1\x14a\x01\xDDW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xAAW\x80c1{P\xEC\x14a\0\xDAW\x80cP\xEDY-\x14a\x01\x02W\x80cZoW\x10\x14a\x01#W\x80c\\9\xF4g\x14a\x016W[__\xFD[a\0\xBDa\0\xB86`\x04aS\xE9V[a\x01\xF0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEDa\0\xE86`\x04aT\x04V[a\x02\0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD1V[a\x01\x15a\x01\x106`\x04aS\xE9V[a\x02\x1BV[`@Q\x90\x81R` \x01a\0\xD1V[a\x01\x15a\x0116`\x04aS\xE9V[a\x02%V[a\x01Ia\x01D6`\x04aS\xE9V[a\x02/V[`@Qa\0\xD1\x91\x90aUVV[a\x01ia\x01d6`\x04aVcV[a\x02OV[`@Qa\0\xD1\x91\x90aWMV[a\0\xBDa\x01\x846`\x04aS\xE9V[a\x02rV[a\x01Ia\x01\x976`\x04aX\x13V[a\x02|V[a\x01\xAFa\x01\xAA6`\x04aXSV[a\x02\x89V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD1V[a\0\xBDa\x01\xD86`\x04aS\xE9V[a\x02\xAAV[a\x01\x15a\x01\xEB6`\x04aX\xB7V[a\x02\xB4V[_a\x01\xFA\x82a\x02\xCEV[\x92\x91PPV[__a\x02\x0E\x86\x86\x86\x86a\x03\x7FV[\x91P\x91P\x94P\x94\x92PPPV[_a\x01\xFA\x82a\x03\xBAV[_a\x01\xFA\x82a\x04~V[``_a\x02;\x83a\x04~V[\x90Pa\x02H\x83_\x83a\x04\x9DV[\x93\x92PPPV[``_a\x02\\\x84\x84a\x05kV[\x90Pa\x02j\x84\x84_\x84a\x05\xE1V[\x94\x93PPPPV[_a\x01\xFA\x82a\x06\xB1V[``a\x02j\x84\x84\x84a\x04\x9DV[___a\x02\x99\x88\x88\x88\x88\x88a\x06\xEDV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x01\xFA\x82a\x07\xDDV[_a\x02\xC2\x86\x86\x86\x86\x86a\x08.V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aY\x0EV[___a\x03\x8C\x86\x86a\x08aV[\x90P_a\x03\x99\x88\x83a\t\x08V[\x90P__a\x03\xA7\x83\x88a\x1B\x80V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aY)V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04\x0B\x90aY@V[``_a\x04\xAB\x85\x85\x85a\x1D3V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC8Wa\x04\xC8aYcV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x01W\x81` \x01[a\x04\xEEaO\x13V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\xE6W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x05aW_\x83\x82\x81Q\x81\x10a\x05\"Wa\x05\"aYwV[` \x02` \x01\x01Q\x90P_a\x057\x89\x83a\x1D\xD4V[\x90P\x80\x84\x84\x81Q\x81\x10a\x05LWa\x05LaYwV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x06V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x05\x84\x84a!\xFEV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02H\x91\x90aY)V[``_a\x05\xF0\x86\x86\x86\x86a\"\x82V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\raYcV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06FW\x81` \x01[a\x063aO\x80V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06+W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xA6W_\x83\x82\x81Q\x81\x10a\x06gWa\x06gaYwV[` \x02` \x01\x01Q\x90P_a\x06|\x8A\x83a#\nV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\x91Wa\x06\x91aYwV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06KV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x06\xFB\x88\x88a\x08aV[\x90P_a\x07\x08\x8A\x83a\t\x08V[\x90P_\x80\x80`\xFF\x89\x16a\x07>Wa\x07 \x8A\x85_a)\x18V[\x92\x95P\x91\x93Pa\x077\x91P\x85\x90P\x8B_\x80\x87a*\x07V[\x90Pa\x07nV[_\x19`\xFF\x8A\x16\x01a\x07nWa\x07T\x8A\x85_a*\xD4V[\x92\x95P\x91\x93Pa\x07k\x91P\x85\x90P_\x8C\x86\x82a*\x07V[\x90P[_a\x07x\x85a+\x9DV[\x90P_\x82\x82\x11a\x07\x91Wa\x07\x8C\x82\x84aY\x9FV[a\x07\x9BV[a\x07\x9B\x83\x83aY\x9FV[\x90P_a\x07\xA8\x82\x84a,6V[\x90P_\x84\x84\x11a\x07\xC0Wa\x07\xBB\x82aY\xB2V[a\x07\xC2V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08:\x86\x86a\x08aV[\x90P_a\x08G\x88\x83a\t\x08V[\x90Pa\x08U\x81\x86\x86_a,qV[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x82W\x81\x83a\x08\x85V[\x82\x82[`@Q\x91\x94P\x92Pa\x08\xB2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\x10aO\xD3V[\x82a\t\x19aO\xD3V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t7\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aY\xCCV[a\t\xBCW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xFC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90aY\x0EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BQ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x90\x91\x90aY)V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xE6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90aY)V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xEA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90aY)V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\xF8\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90aY)V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\xFC\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9F\x91\x90aY)V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10P\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8F\x91\x90aY)V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x118\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11w\x91\x90aY\x0EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90aY)V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB7\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x136W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Z\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xBC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14 \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aY)V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xCB\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xD0\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x164\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16s\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\xCC\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17o\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xBD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18`\x91\x90aY\x0EV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18\xCE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x192\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19q\x91\x90aY\x0EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xD4\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ASW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Aw\x91\x90aY)V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1A\xD0\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bs\x91\x90aY)V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1B\x8CaP\x07V[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF0\x91\x90aY)V[` \x82\x01Ra\x1C\0\x87_\x80a-\xA8V[P`\x80\x84\x01RP`@\x82\x01Ra\x1C\x18\x87`\x01_a-\xA8V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C9\x91\x88\x91a.SV[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1CT\x91\x88\x91a.SV[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1DS\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xADW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02j\x91\x90\x81\x01\x90aZ\x1CV[a\x1D\xDCaO\x13V[a\x1D\xE4aPPV[a\x1D\xEE\x84\x84a\t\x08V[\x81Ra\x1D\xF9\x84a/\x12V[` \x82\x01\x81\x90R\x81Qa\x1E\r\x91_\x90a-\xA8V[`\xA0\x85\x01R`\x80\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x1E5\x91\x90`\x01\x90a-\xA8V[`\xA0\x85\x01Ra\x01\0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01R`@\x80Qa\x03\0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\x80\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01@\x86\x01\x94\x85\x94\x93a\x01\xA0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xBEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE5\x91\x90\x81\x01\x90aZ\xC4V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FV\x91\x90a[XV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R\x87QQQ`@\x90\x81\x01Q\x81\x84\x01R\x88QQQ``\x90\x81\x01Q\x81\x85\x01R\x89QQQ`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\xA0\x90\x81\x01Q\x81\x87\x01R\x83\x8C\x01Q`\xC0\x87\x01R\x91\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a 2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra Y\x91\x90\x81\x01\x90aZ\xC4V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xCD\x91\x90a[XV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01R\x87QQ\x81\x01Q`@\x90\x81\x01Q\x81\x84\x01R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xC0\x80\x8D\x01Q\x81\x88\x01R`\xE0\x80\x8E\x01Q\x90\x88\x01Ra\x01\0\x80\x8E\x01Q\x90\x88\x01Ra\x01 \x80\x8E\x01Q\x97\x01\x96\x90\x96R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a!\xA4\x90a+\x9DV[\x81R` \x01a!\xB3\x86\x86a/[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a!\xDA\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a!\xEB\x86\x86a0KV[\x90Ra\x01@\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a\"8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a\"\x9C\x86a!\xFEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xE3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xC5\x91\x90\x81\x01\x90aZ\x1CV[a#\x12aO\x80V[a#\x1AaP\xACV[a#$\x84\x84a1MV[\x80\x82RQ\x80QQ` \x90\x91\x01QQa#<\x91\x90a\x08aV[`@\x82\x01\x81\x90Ra#N\x90\x85\x90a\t\x08V[` \x82\x01\x81\x90R\x81Qa#b\x91\x86\x91a1_V[PPPP``\x82\x01R` \x81\x01Qa#y\x90a+\x9DV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa#\xAF\x91\x90_[` \x02\x01Q`@\x01Q\x90a1\xAEV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa#\xC7\x90_a1\xEFV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa#\xE0\x91\x90a2\x1DV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa#\xF7\x91\x90a2>V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa$\x18\x92\x91\x90a2ZV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa$9\x92\x87\x92\x90\x91_a2wV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa$f\x92\x91\x90a2ZV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa$\x80\x90`\x01a1\xEFV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa$\xB1\x91\x90`\x01a#\xA0V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa$\xC8\x91a2\x1DV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa$\xE0\x91\x90a2>V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa$\xF8\x91\x90a1\xAEV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa%\x1B\x92\x91\x90a2ZV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%=\x92\x87\x92\x90\x91`\x01a2wV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa%l\x92\x91\x90a2ZV[a\x02\xE0\x82\x01R\x80Qa%}\x90a4\x0CV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a&uWa%\xA7\x81a\x03\0\x01Q\x82`\x80\x01Qa2\x1DV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa%\xBE\x91a1\xAEV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa%\xEF\x93a4NV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa&V\x91\x86\x91a&\x17\x91\x90a2>V[a&)\x84`\xC0\x01Q\x85`\xA0\x01Qa2>V[a&=\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa2>V[a&Q\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa2>V[a4uV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&n\x91\x90a55V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\x10\x91\x90\x81\x01\x90aZ\xC4V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a'eWa'eaYwV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a'\xB4Wa'\xB4aYwV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x14W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(;\x91\x90\x81\x01\x90aZ\xC4V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a)$aQ\x89V[a)/\x87__a-\xA8V[P`@\x84\x01RP\x81Ra)D\x87`\x01_a-\xA8V[P``\x84\x01RP` \x82\x01R\x85\x15a)kW\x87\x81_\x01\x81\x81Qa)g\x91\x90aY\x9FV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra)\x9A\x90\x89\x90a)\x92\x90a'\x10\x90a5NV[a'\x10a.SV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa)\xBD\x92a)\xB8\x90\x83\x90a5\xA8V[a.SV[`\x80\x82\x01\x81\x90R` \x82\x01Qa)\xD2\x91a5NV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa)\xF4\x90\x8C\x90a5\xFCV[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a*\x17WP\x83\x15[\x15a*&WP\x83\x90P\x84a*[V[_\x87\x11\x80\x15a*3WP\x84\x15[\x15a*BWP\x85\x90P\x82a*[V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*j\x89``\x01Q_a1\xEFV[\x90P_a*|\x8A``\x01Q`\x01a1\xEFV[\x90P_a*\x9A\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_a*\xB8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90Pa*\xC4\x82\x82a,6V[\x9C\x9BPPPPPPPPPPPPV[____a*\xE0aQ\x89V[a*\xEB\x87__a-\xA8V[P`@\x84\x01RP\x81Ra+\0\x87`\x01_a-\xA8V[P``\x84\x01RP` \x82\x01R\x85\x15a+(W\x87\x81` \x01\x81\x81Qa+$\x91\x90aY\x9FV[\x90RP[\x80Q` \x82\x01Qa+>\x91\x90a)\xB8\x81\x8Ca5\xA8V[`\x80\x82\x01\x81\x90R\x81Qa+P\x91a5NV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa+\x7F\x91a)\x92\x90a'\x10\x90a5NV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa)\xF4\x91a5\xFCV[__a+\xAA\x83__a-\xA8V[PPP\x90P_a+\xBC\x84`\x01_a-\xA8V[PPP\x90P\x80_\x03a+\xD1WP_\x93\x92PPPV[_a+\xE0\x85``\x01Q_a1\xEFV[\x90P_a+\xF2\x86``\x01Q`\x01a1\xEFV[\x90P_a,\x10\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_a,.\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90Pa\x08U\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a,WW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a,zaP\x07V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xDE\x91\x90aY)V[` \x82\x01Ra,\xEE\x86_\x80a-\xA8V[PPP`\xC0\x82\x01Ra-\x02\x86`\x01_a-\xA8V[PPP`\xE0\x82\x01R\x82\x15a-=W\x84\x81`\xC0\x01\x81\x81Qa-\"\x91\x90aY\x9FV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a-9\x90\x83\x90aY\x9FV[\x90RP[\x80` \x01Q_\x03a-mWa-fa\x03\xE8a-`a-[\x88\x88a6 V[a6\x86V[\x90a5NV[\x81Ra-\x9EV[a-\x9Ba-\x83\x86\x83` \x01Q\x84`\xC0\x01Qa.SV[a-\x96\x86\x84` \x01Q\x85`\xE0\x01Qa.SV[a7fV[\x81R[Q\x95\x94PPPPPV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10a-\xC5Wa-\xC5aYwV[` \x02\x01Q\x90P_a-\xD7\x89\x89a7{V[\x90P\x80_\x03a-\xF3W____\x95P\x95P\x95P\x95PPPa)\xFEV[_a.\x02\x83\x8B`\x80\x01Qa8MV[\x90P_\x88\x15a.%W\x81a.\x16\x84\x8Ba1\xAEV[a. \x91\x90aY\x9FV[a.'V[_[\x90Pa.3\x82\x84a\\aV[\x83a.>\x84\x82aY\x9FV[\x91\x99P\x97P\x95P\x93PPPP\x93P\x93P\x93P\x93V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a.\x87W\x83\x82\x81a.}Wa.}a\\tV[\x04\x92PPPa\x02HV[\x80\x84\x11a.\xA7W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10\x93\xD4\x94\x93\xD5\xD7\xD4\x90U\x11W\xD5\x12\x14\x91T\xD2\x13\xD3\x11`Z\x1B`@\x82\x01R``\x01\x90V[__a/g\x84\x84a8}V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a/\xA8\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xD8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aY\x0EV[__a0W\x84\x84a8}V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a0\xAA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\xDA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\x0E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aY)V[a1UaQ\xD8V[a\x02H\x83\x83a98V[_____a1m\x87a+\x9DV[\x90Pa1{\x87\x87\x83_aKVV[\x90\x93P\x91P\x81a1\x8CW_\x19a1\x96V[a1\x96\x83\x83a,6V[\x94Pa1\xA1\x88aL\xC2V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a1\xCEW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a2\x0FWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a24Wa2/\x83\x83aY\x9FV[a\x02HV[a\x02H\x82\x84aY\x9FV[_a\x02H\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[_\x82\x84\x11a2pWa2k\x82aY\xB2V[a\x02jV[P\x92\x91PPV[_a2\xB1`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a2\xBD\x86\x86\x86_aKVV[` \x83\x01R\x80\x82R\x15\x80a2\xEEWP\x84Q`\xFF\x84\x16`\x02\x81\x10a2\xE2Wa2\xE2aYwV[` \x02\x01Q` \x01Q_\x14[\x15a2\xFCW_\x91PPa\x02\xC5V[a3\x05\x87aM\x13V[`@\x82\x01\x81\x90R` \x82\x01Qa3\x1A\x91a1\xAEV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a32W_\x91PPa\x02\xC5V[`\x80\x81\x01Q\x81Qa3C\x91\x90aY\x9FV[\x81``\x01\x81\x81RPPa3Z\x86``\x01Q\x84a1\xEFV[`\xA0\x82\x01\x81\x90R``\x82\x01Qa3\x86\x91a3u\x90`\na\\VV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba.SV[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a3\xABW`\xC0\x81\x01Qa3\xA5\x90\x85a,6V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a3\xC1Wa3\xC1aYwV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a3\xFEW\x84Q`\xFF\x84\x16`\x02\x81\x10a3\xEBWa3\xEBaYwV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a4,WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa4GWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x80\x15a4[WP\x82\x84\x11[a4mWa4h\x82aY\xB2V[a\x02\xC5V[P\x93\x92PPPV[__a4\x80\x87aL\xC2V[\x90P_a4\x8D\x82\x87a1\xAEV[\x90P_a4\x9A\x83\x86a1\xAEV[\x90P_a4\xA7\x89\x84a\\\x88V[\x90P_a4\xB4\x83\x89a\\\x88V[\x90P_a4\xC0\x83aMYV[\x90P_a4\xCC\x83aMYV[\x90P_\x84\x13\x80\x15a4\xDCWP_\x83\x12[\x80a4\xF0WP_\x84\x12\x80\x15a4\xF0WP_\x83\x13[\x15a5\x04W_\x97PPPPPPPPa\x02\xC5V[\x80_\x03a5\x1AW_\x97PPPPPPPPa\x02\xC5V[a5$\x82\x82a,6V[\x9D\x9CPPPPPPPPPPPPPV[_\x82\x15a5FWa2/\x82\x84a,6V[P_\x92\x91PPV[_\x82a5Z\x83\x82aY\x9FV[\x91P\x81\x11\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a5\xB4\x83\x82a\\aV[\x91P\x81\x10\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a5\x9FV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a6\x12W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a6CWP\x82\x82a65\x81\x83a\\\xA7V[\x92Pa6A\x90\x83a\\\xBEV[\x14[a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a5\x9FV[_\x81_\x03a6\x95WP_\x91\x90PV[_`\x01a6\xA1\x84aMnV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a6\xBAWa6\xBAa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xD2Wa6\xD2a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xEAWa6\xEAa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7\x02Wa7\x02a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7\x1AWa7\x1Aa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a72Wa72a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7JWa7Ja\\tV[\x04\x82\x01\x90\x1C\x90Pa\x02H\x81\x82\x85\x81a7dWa7da\\tV[\x04[_\x81\x83\x10a7tW\x81a\x02HV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a7\x95Wa7\x95aYwV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x12\x91\x90aY)V[\x90P\x80_\x03a8%W_\x92PPPa\x01\xFAV[``\x82\x01Q`\xC0\x83\x01Qa89\x82\x84aY\x9FV[a8C\x91\x90aY\x9FV[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a8`WP_a\x01\xFAV[_a8k\x84\x84aN\x01V[`\xA0\x85\x01Q\x90\x91Pa\x02j\x90\x82a1\xAEV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a8\xA0\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x18\x91\x90aY\xCCV[a\x02HW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a5\x9FV[a9@aQ\xD8V[\x82a9IaQ\xD8V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a9\x89\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\x01\x91\x90aY\xCCV[a:\x0EW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a:H\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xAC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xEB\x91\x90aY)V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a;3\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;c\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x97\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xD6\x91\x90aY\x0EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a<2\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD5\x91\x90aY\x0EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xC3\x91\x90aY)V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\x17\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>G\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>{\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xBA\x91\x90aY)V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\x14\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?x\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xB7\x91\x90aY)V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\x10\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xB3\x91\x90aY)V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aAm\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xAC\x91\x90aY)V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aB\x06\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aBj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xA9\x91\x90aY)V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aCP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x8F\x91\x90aY)V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDv\x91\x90aY\x0EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\\\x91\x90aY)V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xB1\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFT\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xAF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGR\x91\x90aY)V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\xAC\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\x10\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHO\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\xB1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aIT\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI\xAF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJR\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ\xA1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKD\x91\x90aY)V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aK\xF9W__aK\x81\x8A\x8A_aNGV[\x91P\x91P_aK\x9D_\x8C``\x01Qa1\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aK\xBB\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[\x90P_aK\xD9\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90PaK\xE5\x82\x88a\\aV[\x96PaK\xF1\x81\x87a\\aV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aL\xB5W__aL \x8A\x8A`\x01aNGV[\x91P\x91P_aL=`\x01\x8C``\x01Qa1\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aL[\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[\x90P_aLy\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_aL\x86\x83\x8Da1\xAEV[\x90P_aL\x93\x83\x8Ea1\xAEV[\x90PaL\x9F\x82\x8Aa\\aV[\x98PaL\xAB\x81\x89a\\aV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aMjW\x81_\x03a\x01\xFAV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aM\x82W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aM\x94W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aM\xA6W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aM\xB8W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aM\xCAW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aM\xDCW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aM\xEEW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x01\xFAW`\x01\x01\x92\x91PPV[_B\x82\x03aN\x14WP` \x82\x01Qa\x01\xFAV[_aN#\x84`@\x01Q\x84aN\xDFV[\x90PaN<\x84` \x01Q\x82a1\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x01\xFAV[PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aNbWaNbaYwV[` \x02\x01Q`@\x01Q\x90P_aN\x98\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aN\x89WaN\x89aYwV[` \x02\x01Q\x88`\x80\x01QaN\x01V[\x90P\x81\x15aN\xAFWaN\xAA\x82\x82a1\xAEV[aN\xB1V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aN\xCAWaN\xCAaYwV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aN\xEB\x83BaY\x9FV[aN\xF5\x90\x85a\\\xA7V[c\x01\xE13\x80\x90\x04\x90Pa\x02j\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\\aV[`@Q\x80a\x01@\x01`@R\x80aO'aQ\xFEV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aO\x94aR\x85V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aO\xE6aS\0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80aPdaO\xD3V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aP\xA7aO\x13V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aP\xC0aQ\xD8V[\x81R` \x01aP\xCDaO\xD3V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aQ\xEBaSgV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aRo`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\rW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aR\xEA`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x94W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aSQ`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aS\x0FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aS\xBF`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aSvW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aNDW__\xFD[_` \x82\x84\x03\x12\x15aS\xF9W__\xFD[\x815a\x02H\x81aS\xD5V[____`\x80\x85\x87\x03\x12\x15aT\x17W__\xFD[\x845aT\"\x81aS\xD5V[\x93P` \x85\x015aT2\x81aS\xD5V[\x92P`@\x85\x015aTB\x81aS\xD5V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aUKW\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01RaT\xC1a\x01\x80\x86\x01\x82aTRV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaT\x89V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aVWW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01@\x87RaU\xA4a\x01@\x88\x01\x82aT\x80V[\x90P` \x82\x01QaU\xC0` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QaU\xDB`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01QaV\x1E`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01QaV4a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aU|V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aVtW__\xFD[\x825aV\x7F\x81aS\xD5V[\x91P` \x83\x015aV\x8F\x81aS\xD5V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aUKW\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01RaV\xDBa\x01@\x86\x01\x82aTRV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaV\xA3V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aVWW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RaW\x9Ba\x01 \x88\x01\x82aV\x9AV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QaW\xC1`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aWsV[___``\x84\x86\x03\x12\x15aX%W__\xFD[\x835aX0\x81aS\xD5V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14aNDW__\xFD[_____`\xA0\x86\x88\x03\x12\x15aXgW__\xFD[\x855aXr\x81aS\xD5V[\x94P` \x86\x015aX\x82\x81aS\xD5V[\x93P`@\x86\x015aX\x92\x81aS\xD5V[\x92P``\x86\x015\x91P`\x80\x86\x015aX\xA9\x81aXEV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15aX\xCBW__\xFD[\x855aX\xD6\x81aS\xD5V[\x94P` \x86\x015aX\xE6\x81aS\xD5V[\x93P`@\x86\x015aX\xF6\x81aS\xD5V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15aY\x1EW__\xFD[\x81Qa\x02H\x81aS\xD5V[_` \x82\x84\x03\x12\x15aY9W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01\xFAWa\x01\xFAaY\x8BV[_`\x01`\xFF\x1B\x82\x01aY\xC6WaY\xC6aY\x8BV[P_\x03\x90V[_` \x82\x84\x03\x12\x15aY\xDCW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02HW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aZ\x14WaZ\x14aYcV[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15aZ,W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZRW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZlWaZlaYcV[\x80`\x05\x1BaZ|` \x82\x01aY\xEBV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aZ\x97W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aZ\xB9W\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aZ\x9EV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aZ\xD4W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xEAW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZ\xFAW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x14Wa[\x14aYcV[a['`\x1F\x82\x01`\x1F\x19\x16` \x01aY\xEBV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a[;W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a[hW__\xFD[\x81Qa\x02H\x81aXEV[`\x01\x81[`\x01\x84\x11\x15a[\xAEW\x80\x85\x04\x81\x11\x15a[\x92Wa[\x92aY\x8BV[`\x01\x84\x16\x15a[\xA0W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a[wV[\x93P\x93\x91PPV[_\x82a[\xC4WP`\x01a\x01\xFAV[\x81a[\xD0WP_a\x01\xFAV[\x81`\x01\x81\x14a[\xE6W`\x02\x81\x14a[\xF0Wa\\\x0CV[`\x01\x91PPa\x01\xFAV[`\xFF\x84\x11\x15a\\\x01Wa\\\x01aY\x8BV[PP`\x01\x82\x1Ba\x01\xFAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\\/WP\x81\x81\na\x01\xFAV[a\\;_\x19\x84\x84a[sV[\x80_\x19\x04\x82\x11\x15a\\NWa\\NaY\x8BV[\x02\x93\x92PPPV[_a\x02H\x83\x83a[\xB6V[\x80\x82\x01\x80\x82\x11\x15a\x01\xFAWa\x01\xFAaY\x8BV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a2pWa2paY\x8BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\xFAWa\x01\xFAaY\x8BV[_\x82a\\\xD8WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC2\xFC\xEB\xFF`\x8DU\xE3p\xFE\xC1\xC2\xD8\xF2\x82\xD3\x8C{\x17u\xC7qN\xF0%\xA3\x01\x0F\xA0\xC3\x02\xBFdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100a6575f3560e01c8063739118a41161006e578063739118a41461015657806378f212d1146101765780638f6c7a3c14610189578063d28b0a151461019c578063e335adb7146101ca578063f68a7131146101dd575f5ffd5b806328a0ccf4146100aa578063317b50ec146100da57806350ed592d146101025780635a6f5710146101235780635c39f46714610136575b5f5ffd5b6100bd6100b83660046153e9565b6101f0565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ed6100e8366004615404565b610200565b604080519283526020830191909152016100d1565b6101156101103660046153e9565b61021b565b6040519081526020016100d1565b6101156101313660046153e9565b610225565b6101496101443660046153e9565b61022f565b6040516100d19190615556565b610169610164366004615663565b61024f565b6040516100d1919061574d565b6100bd6101843660046153e9565b610272565b610149610197366004615813565b61027c565b6101af6101aa366004615853565b610289565b604080519384526020840192909252908201526060016100d1565b6100bd6101d83660046153e9565b6102aa565b6101156101eb3660046158b7565b6102b4565b5f6101fa826102ce565b92915050565b5f5f61020e8686868661037f565b9150915094509492505050565b5f6101fa826103ba565b5f6101fa8261047e565b60605f61023b8361047e565b9050610248835f8361049d565b9392505050565b60605f61025c848461056b565b905061026a84845f846105e1565b949350505050565b5f6101fa826106b1565b606061026a84848461049d565b5f5f5f61029988888888886106ed565b925092509250955095509592505050565b5f6101fa826107dd565b5f6102c2868686868661082e565b90505b95945050505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161034091815260200190565b602060405180830381865afa15801561035b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa919061590e565b5f5f5f61038c8686610861565b90505f6103998883610908565b90505f5f6103a78388611b80565b50919c909b509950505050505050505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161043f91815260200190565b602060405180830381865afa15801561045a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101fa9190615929565b5f816001600160a01b031663f3903b9f60405160200161040b90615940565b60605f6104ab858585611d33565b90505f815167ffffffffffffffff8111156104c8576104c8615963565b60405190808252806020026020018201604052801561050157816020015b6104ee614f13565b8152602001906001900390816104e65790505b5090505f5b8251811015610561575f83828151811061052257610522615977565b602002602001015190505f6105378983611dd4565b90508084848151811061054c5761054c615977565b60209081029190910101525050600101610506565b5095945050505050565b5f826001600160a01b031663f3903b9f610584846121fe565b6040518263ffffffff1660e01b81526004016105a291815260200190565b602060405180830381865afa1580156105bd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102489190615929565b60605f6105f086868686612282565b90505f815167ffffffffffffffff81111561060d5761060d615963565b60405190808252806020026020018201604052801561064657816020015b610633614f80565b81526020019060019003908161062b5790505b5090505f5b82518110156106a6575f83828151811061066757610667615977565b602002602001015190505f61067c8a8361230a565b90508084848151811061069157610691615977565b6020908102919091010152505060010161064b565b509695505050505050565b5f816001600160a01b03166321f8a72160405160200161030c90602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f6106fb8888610861565b90505f6107088a83610908565b90505f808060ff891661073e576107208a855f612918565b92955091935061073791508590508b5f8087612a07565b905061076e565b5f1960ff8a160161076e576107548a855f612ad4565b92955091935061076b91508590505f8c8682612a07565b90505b5f61077885612b9d565b90505f8282116107915761078c828461599f565b61079b565b61079b838361599f565b90505f6107a88284612c36565b90505f8484116107c0576107bb826159b2565b6107c2565b815b969b5094995094975050505050505050955095509592505050565b5f816001600160a01b03166321f8a72160405160200161030c906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b5f5f61083a8686610861565b90505f6108478883610908565b90506108558186865f612c71565b98975050505050505050565b5f816001600160a01b0316836001600160a01b031610610882578183610885565b82825b60405191945092506108b2906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b610910614fd3565b82610919614fd3565b816001600160a01b03166391d4403c60405160200161093790615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561098b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109af91906159cc565b6109bc5791506101fa9050565b816001600160a01b03166321f8a721856040516020016109fc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a6091815260200190565b602060405180830381865afa158015610a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9f919061590e565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610b1d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b5191815260200190565b602060405180830381865afa158015610b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b909190615929565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610be6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4a91815260200190565b602060405180830381865afa158015610c65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c899190615929565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610cea9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4e91815260200190565b602060405180830381865afa158015610d69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d8d9190615929565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610df89060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610e28929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e5c91815260200190565b602060405180830381865afa158015610e77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e9b9190615929565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610efc9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610f2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f6091815260200190565b602060405180830381865afa158015610f7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f9f9190615929565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161101c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161105091815260200190565b602060405180830381865afa15801561106b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061108f9190615929565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611104929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161113891815260200190565b602060405180830381865afa158015611153573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611177919061590e565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161122191815260200190565b602060405180830381865afa15801561123c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112609190615929565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016112b790602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016112e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131b91815260200190565b602060405180830381865afa158015611336573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061135a9190615929565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016113bc9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161142091815260200190565b602060405180830381865afa15801561143b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145f9190615929565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016114cb9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016114fb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161152f91815260200190565b602060405180830381865afa15801561154a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061156e9190615929565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016115d09060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611600929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161163491815260200190565b602060405180830381865afa15801561164f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116739190615929565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016116cc90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016116fc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161173091815260200190565b602060405180830381865afa15801561174b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061176f9190615929565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016117bd90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ed929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161182191815260200190565b602060405180830381865afa15801561183c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611860919061590e565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016118ce906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161193291815260200190565b602060405180830381865afa15801561194d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611971919061590e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016119d4906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611a04929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a3891815260200190565b602060405180830381865afa158015611a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a779190615929565b60608201526040516001600160a01b0383169063bd02d0f5908690611ad0906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611b00929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611b3491815260200190565b602060405180830381865afa158015611b4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b739190615929565b6080820152949350505050565b5f5f5f5f611b8c615007565b86602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611bcc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf09190615929565b6020820152611c00875f80612da8565b506080840152506040820152611c188760015f612da8565b5060a084015250606082015260408101516020820151611c39918891612e53565b61010082015260608101516020820151611c54918891612e53565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a0909301519199909850919650945092505050565b6060836001600160a01b031663f069052a604051602001611d5390615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611dad573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261026a9190810190615a1c565b611ddc614f13565b611de4615050565b611dee8484610908565b8152611df984612f12565b602082018190528151611e0d915f90612da8565b60a085015260808401526040830152606082015280516020820151611e359190600190612da8565b60a085015261010084015260c083015260e0820152604080516103008101825282515151516001600160a01b039081166101808301908152845151515184516395d89b4160e01b81529451939485946101408601948594936101a08801939116916395d89b41916004808201925f929091908290030181865afa158015611ebe573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611ee59190810190615ac4565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015611f32573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f569190615b58565b60ff16815286515151602090810151818301528751515160409081015181840152885151516060908101518185015289515151608090810151818601528a51515160a09081015181870152838c015160c0870152918b015160e08601528a015161010085015289015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612032573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526120599190810190615ac4565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120cd9190615b58565b60ff168152865151602090810151810151818301528751518101516040908101518184015288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260c0808d01518188015260e0808e015190880152610100808e015190880152610120808e01519701969096529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b9083015283519101906121a490612b9d565b81526020016121b38686612f5b565b6001600160a01b031681526020016121da835f015160600151660800000000000016151590565b151581526020016121eb868661304b565b9052610140909101819052905092915050565b5f604051602001612238906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61229c866121fe565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa1580156122e3573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102c59190810190615a1c565b612312614f80565b61231a6150ac565b612324848461314d565b808252518051516020909101515161233c9190610861565b6040820181905261234e908590610908565b60208201819052815161236291869161315f565b505050506060820152602081015161237990612b9d565b610300820152805180515160209081015160e0840152808301515151015190516123af91905f5b602002015160400151906131ae565b60c08201526020810151606001516123c7905f6131ef565b60a082015260e081015160c08201516123e0919061321d565b610100820181905260a08201516123f7919061323e565b61012082015260e081015160c082015161010083015161241892919061325a565b6101408201526020810151815161030083015161243992879290915f613277565b61016082015261014081015161018082015260e081015160c082015161012083015161246692919061325a565b6101a08201526020810151606001516124809060016131ef565b6101c0820152805180516020908101518101516102008401528083015151810151015190516124b1919060016123a0565b6101e082018190526102008201516124c89161321d565b61022082018190526101c08201516124e0919061323e565b61024082018190526103008201516124f891906131ae565b6102608201526102008101516101e082015161022083015161251b92919061325a565b6102808201526020810151815161030083015161253d92879290916001613277565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161256c92919061325a565b6102e0820152805161257d9061340c565b60808201528051516020015160e00151600214612675576125a7816103000151826080015161321d565b61032082018190526102408201516125be916131ae565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516125ef9361344e565b61036082018190526103a082015260e081015160a0820151612656918691612617919061323e565b6126298460c001518560a0015161323e565b61263d856102000151866101c0015161323e565b612651866101e00151876101c0015161323e565b613475565b6103c0820181905261030082015161266e9190613535565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa1580156126e9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526127109190810190615ac4565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061276557612765615977565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106127b4576127b4615977565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612814573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261283b9190810190615ac4565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612924615189565b61292f875f5f612da8565b5060408401525081526129448760015f612da8565b506060840152506020820152851561296b5787815f01818151612967919061599f565b9052505b606087015160381c61ffff16610120820181905261299a908990612992906127109061354e565b612710612e53565b6101408201819052815160208301516129bd926129b89083906135a8565b612e53565b6080820181905260208201516129d29161354e565b60c08201819052604082015160608301516101208401516129f4908c906135fc565b9450945094509450505b93509350935093565b5f5f5f5f86118015612a17575083155b15612a26575083905084612a5b565b5f87118015612a33575084155b15612a42575085905082612a5b565b604051636331fab160e01b815260040160405180910390fd5b5f612a6a89606001515f6131ef565b90505f612a7c8a6060015160016131ef565b90505f612a9a85676765c793fa10079d601b1b6129b886600a615c56565b90505f612ab885676765c793fa10079d601b1b6129b886600a615c56565b9050612ac48282612c36565b9c9b505050505050505050505050565b5f5f5f5f612ae0615189565b612aeb875f5f612da8565b506040840152508152612b008760015f612da8565b5060608401525060208201528515612b28578781602001818151612b24919061599f565b9052505b80516020820151612b3e91906129b8818c6135a8565b608082018190528151612b509161354e565b60a0820152606087015160381c61ffff16610120820181905260a0820151612b7f91612992906127109061354e565b6040820151606083015161012084015160a08501516129f4916135fc565b5f5f612baa835f5f612da8565b50505090505f612bbc8460015f612da8565b5050509050805f03612bd157505f9392505050565b5f612be085606001515f6131ef565b90505f612bf2866060015160016131ef565b90505f612c1085676765c793fa10079d601b1b6129b886600a615c56565b90505f612c2e85676765c793fa10079d601b1b6129b886600a615c56565b905061085582825b5f8115676765c793fa10079d601b1b60028404190484111715612c57575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f612c7a615007565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612cba573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cde9190615929565b6020820152612cee865f80612da8565b50505060c0820152612d028660015f612da8565b50505060e08201528215612d3d57848160c001818151612d22919061599f565b90525060e081018051859190612d3990839061599f565b9052505b80602001515f03612d6d57612d666103e8612d60612d5b8888613620565b613686565b9061354e565b8152612d9e565b612d9b612d838683602001518460c00151612e53565b612d968684602001518560e00151612e53565b613766565b81525b5195945050505050565b5f5f5f5f5f875f01518760ff1660028110612dc557612dc5615977565b602002015190505f612dd7898961377b565b9050805f03612df3575f5f5f5f955095509550955050506129fe565b5f612e02838b6080015161384d565b90505f8815612e255781612e16848b6131ae565b612e20919061599f565b612e27565b5f5b9050612e338284615c61565b83612e3e848261599f565b91995097509550935050505093509350935093565b5f838302815f1985870982811083820303915050805f03612e8757838281612e7d57612e7d615c74565b0492505050610248565b808411612ea75760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601590820152741093d49493d5d7d490551157d512149154d213d311605a1b604082015260600190565b5f5f612f67848461387d565b9050806001600160a01b03166321f8a72184604051602001612fa8906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612fd8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161300c91815260200190565b602060405180830381865afa158015613027573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a919061590e565b5f5f613057848461387d565b9050806001600160a01b031663bd02d0f5846040516020016130aa9060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016130da929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161310e91815260200190565b602060405180830381865afa158015613129573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061026a9190615929565b6131556151d8565b6102488383613938565b5f5f5f5f5f61316d87612b9d565b905061317b8787835f614b56565b90935091508161318c575f19613196565b6131968383612c36565b94506131a188614cc2565b9350939792965093509350565b5f81156b019d971e4fe8401e7400000019839004841115176131ce575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff85160161320f575060ff60601b19905060605b90198416901c905092915050565b5f8183116132345761322f838361599f565b610248565b610248828461599f565b5f61024883676765c793fa10079d601b1b6129b885600a615c56565b5f8284116132705761326b826159b2565b61026a565b5092915050565b5f6132b16040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6132bd8686865f614b56565b602083015280825215806132ee5750845160ff8416600281106132e2576132e2615977565b6020020151602001515f145b156132fc575f9150506102c5565b61330587614d13565b60408201819052602082015161331a916131ae565b6080820181905281511015613332575f9150506102c5565b60808101518151613343919061599f565b81606001818152505061335a8660600151846131ef565b60a0820181905260608201516133869161337590600a615c56565b676765c793fa10079d601b1b612e53565b60c08201525f1960ff8416016133ab5760c08101516133a59085612c36565b60c08201525b845160ff8416600281106133c1576133c1615977565b6020020151602001518160c0015111156133fe57845160ff8416600281106133eb576133eb615977565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f190161342c575051602001516060015190565b81516020015160e00151613447575051602001516080015190565b505f919050565b5f84801561345b57508284115b61346d57613468826159b2565b6102c5565b509392505050565b5f5f61348087614cc2565b90505f61348d82876131ae565b90505f61349a83866131ae565b90505f6134a78984615c88565b90505f6134b48389615c88565b90505f6134c083614d59565b90505f6134cc83614d59565b90505f841380156134dc57505f83125b806134f057505f841280156134f057505f83135b15613504575f9750505050505050506102c5565b805f0361351a575f9750505050505050506102c5565b6135248282612c36565b9d9c50505050505050505050505050565b5f82156135465761322f8284612c36565b505f92915050565b5f8261355a838261599f565b91508111156101fa5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b60448201526064015b60405180910390fd5b5f826135b48382615c61565b91508110156101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161359f565b5f81156113881983900484111517613612575f5ffd5b506127109102611388010490565b5f811580613643575082826136358183615ca7565b92506136419083615cbe565b145b6101fa5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161359f565b5f815f0361369557505f919050565b5f60016136a184614d6e565b901c6001901b905060018184816136ba576136ba615c74565b048201901c905060018184816136d2576136d2615c74565b048201901c905060018184816136ea576136ea615c74565b048201901c9050600181848161370257613702615c74565b048201901c9050600181848161371a5761371a615c74565b048201901c9050600181848161373257613732615c74565b048201901c9050600181848161374a5761374a615c74565b048201901c90506102488182858161376457613764615c74565b045b5f8183106137745781610248565b5090919050565b5f5f835f01518360ff166002811061379557613795615977565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156137ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138129190615929565b9050805f03613825575f925050506101fa565b606082015160c0830151613839828461599f565b613843919061599f565b9695505050505050565b5f8260a001515f0361386057505f6101fa565b5f61386b8484614e01565b60a085015190915061026a90826131ae565b5f5f839050806001600160a01b03166391d4403c6040516020016138a090615940565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156138f4573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061391891906159cc565b61024857604051637357d91f60e01b81526004810184905260240161359f565b6139406151d8565b826139496151d8565b816001600160a01b03166391d4403c604051602001613989906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156139dd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a0191906159cc565b613a0e5791506101fa9050565b816001600160a01b031663bd02d0f585604051602001613a48906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001613a78929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613aac91815260200190565b602060405180830381865afa158015613ac7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613aeb9190615929565b816020018181525050816001600160a01b03166321f8a72185604051602001613b33906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613b63929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b9791815260200190565b602060405180830381865afa158015613bb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613bd6919061590e565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001613c32906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613c62929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c9691815260200190565b602060405180830381865afa158015613cb1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cd5919061590e565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613d50929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613d8491815260200190565b602060405180830381865afa158015613d9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613dc39190615929565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613e179060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e47929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e7b91815260200190565b602060405180830381865afa158015613e96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613eba9190615929565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613f14906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001613f44929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f7891815260200190565b602060405180830381865afa158015613f93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613fb79190615929565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001614010906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614040929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161407491815260200190565b602060405180830381865afa15801561408f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906140b39190615929565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614139929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161416d91815260200190565b602060405180830381865afa158015614188573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906141ac9190615929565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001614206906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614236929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161426a91815260200190565b602060405180830381865afa158015614285573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142a99190615929565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161431c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161435091815260200190565b602060405180830381865afa15801561436b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061438f9190615929565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001614403929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161443791815260200190565b602060405180830381865afa158015614452573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614476919061590e565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161451d91815260200190565b602060405180830381865afa158015614538573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061455c9190615929565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016145b19060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016145e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161461591815260200190565b602060405180830381865afa158015614630573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906146549190615929565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016146af90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016146df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161471391815260200190565b602060405180830381865afa15801561472e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147529190615929565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016147ac90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016147dc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161481091815260200190565b602060405180830381865afa15801561482b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061484f9190615929565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016148b19060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016148e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161491591815260200190565b602060405180830381865afa158015614930573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149549190615929565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016149af90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016149df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614a1391815260200190565b602060405180830381865afa158015614a2e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a529190615929565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001614aa1906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001614ad1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b0591815260200190565b602060405180830381865afa158015614b20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b449190615929565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614614bf9575f5f614b818a8a5f614e47565b915091505f614b9d5f8c606001516131ef90919063ffffffff16565b90505f614bbb84676765c793fa10079d601b1b6129b885600a615c56565b90505f614bd984676765c793fa10079d601b1b6129b886600a615c56565b9050614be58288615c61565b9650614bf18187615c61565b955050505050505b865160200151516001600160a01b03868116911614614cb5575f5f614c208a8a6001614e47565b915091505f614c3d60018c606001516131ef90919063ffffffff16565b90505f614c5b84676765c793fa10079d601b1b6129b885600a615c56565b90505f614c7984676765c793fa10079d601b1b6129b886600a615c56565b90505f614c86838d6131ae565b90505f614c93838e6131ae565b9050614c9f828a615c61565b9850614cab8189615c61565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161040b9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161040b906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215614d6a57815f036101fa565b5090565b5f80608083901c15614d8257608092831c92015b604083901c15614d9457604092831c92015b602083901c15614da657602092831c92015b601083901c15614db857601092831c92015b600883901c15614dca57600892831c92015b600483901c15614ddc57600492831c92015b600283901c15614dee57600292831c92015b600183901c156101fa5760010192915050565b5f428203614e14575060208201516101fa565b5f614e23846040015184614edf565b9050614e3c8460200151826131ae90919063ffffffff16565b9150506101fa565b50565b5f5f5f845f01518460ff1660028110614e6257614e62615977565b60200201516040015190505f614e98875f01518660ff1660028110614e8957614e89615977565b60200201518860800151614e01565b90508115614eaf57614eaa82826131ae565b614eb1565b5f5b865190935060ff861660028110614eca57614eca615977565b60200201516020015193505050935093915050565b5f80614eeb834261599f565b614ef59085615ca7565b6301e133809004905061026a81676765c793fa10079d601b1b615c61565b604051806101400160405280614f276151fe565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81525090565b604051806101200160405280614f94615285565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518060a00160405280614fe6615300565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061014001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101600160405280615064614fd3565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016150a7614f13565b905290565b6040518061040001604052806150c06151d8565b81526020016150cd614fd3565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806151eb615367565b81525f6020820181905260409091015290565b60405180604001604052806002905b61526f6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161520d5790505090565b60405180604001604052806002905b6152ea6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816152945790505090565b60405180604001604052806002905b6153516040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161530f5790505090565b60405180604001604052806002905b6153bf6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816153765790505090565b6001600160a01b0381168114614e44575f5ffd5b5f602082840312156153f9575f5ffd5b8135610248816153d5565b5f5f5f5f60808587031215615417575f5ffd5b8435615422816153d5565b93506020850135615432816153d5565b92506040850135615442816153d5565b9396929550929360600135925050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f8260408101835f5b600281101561554b578383038752815180516001600160a01b03168452602081015161018060208601526154c1610180860182615452565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050615489565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561565757603f198786030184528151805161014087526155a4610140880182615480565b905060208201516155c060208901826001600160a01b03169052565b5060408201516155db60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e082015161561e60e08901826001600160a01b03169052565b5061010082015161563461010089018215159052565b50610120918201519690910195909552602093840193919091019060010161557c565b50929695505050505050565b5f5f60408385031215615674575f5ffd5b823561567f816153d5565b9150602083013561568f816153d5565b809150509250929050565b5f8260408101835f5b600281101561554b578383038752815180516001600160a01b03168452602081015161014060208601526156db610140860182615452565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e086015261010082015161010086015261012082015161012086015280945050506020820191506020870196506001810190506156a3565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561565757603f1987860301845281518051610120875261579b61012088018261569a565b90506020820151602088015260408201516157c160408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101615773565b5f5f5f60608486031215615825575f5ffd5b8335615830816153d5565b95602085013595506040909401359392505050565b60ff81168114614e44575f5ffd5b5f5f5f5f5f60a08688031215615867575f5ffd5b8535615872816153d5565b94506020860135615882816153d5565b93506040860135615892816153d5565b92506060860135915060808601356158a981615845565b809150509295509295909350565b5f5f5f5f5f60a086880312156158cb575f5ffd5b85356158d6816153d5565b945060208601356158e6816153d5565b935060408601356158f6816153d5565b94979396509394606081013594506080013592915050565b5f6020828403121561591e575f5ffd5b8151610248816153d5565b5f60208284031215615939575f5ffd5b5051919050565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156101fa576101fa61598b565b5f600160ff1b82016159c6576159c661598b565b505f0390565b5f602082840312156159dc575f5ffd5b81518015158114610248575f5ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715615a1457615a14615963565b604052919050565b5f60208284031215615a2c575f5ffd5b815167ffffffffffffffff811115615a42575f5ffd5b8201601f81018413615a52575f5ffd5b805167ffffffffffffffff811115615a6c57615a6c615963565b8060051b615a7c602082016159eb565b91825260208184018101929081019087841115615a97575f5ffd5b6020850194505b83851015615ab9578451825260209485019490910190615a9e565b979650505050505050565b5f60208284031215615ad4575f5ffd5b815167ffffffffffffffff811115615aea575f5ffd5b8201601f81018413615afa575f5ffd5b805167ffffffffffffffff811115615b1457615b14615963565b615b27601f8201601f19166020016159eb565b818152856020838501011115615b3b575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215615b68575f5ffd5b815161024881615845565b6001815b6001841115615bae57808504811115615b9257615b9261598b565b6001841615615ba057908102905b60019390931c928002615b77565b935093915050565b5f82615bc4575060016101fa565b81615bd057505f6101fa565b8160018114615be65760028114615bf057615c0c565b60019150506101fa565b60ff841115615c0157615c0161598b565b50506001821b6101fa565b5060208310610133831016604e8410600b8410161715615c2f575081810a6101fa565b615c3b5f198484615b73565b805f1904821115615c4e57615c4e61598b565b029392505050565b5f6102488383615bb6565b808201808211156101fa576101fa61598b565b634e487b7160e01b5f52601260045260245ffd5b8181035f8312801583831316838312821617156132705761327061598b565b80820281158282048414176101fa576101fa61598b565b5f82615cd857634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220c2fcebff608d55e370fec1c2d8f282d38c7b1775c7714ef025a3010fa0c302bf64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0nW\x80cs\x91\x18\xA4\x14a\x01VW\x80cx\xF2\x12\xD1\x14a\x01vW\x80c\x8Flz<\x14a\x01\x89W\x80c\xD2\x8B\n\x15\x14a\x01\x9CW\x80c\xE35\xAD\xB7\x14a\x01\xCAW\x80c\xF6\x8Aq1\x14a\x01\xDDW__\xFD[\x80c(\xA0\xCC\xF4\x14a\0\xAAW\x80c1{P\xEC\x14a\0\xDAW\x80cP\xEDY-\x14a\x01\x02W\x80cZoW\x10\x14a\x01#W\x80c\\9\xF4g\x14a\x016W[__\xFD[a\0\xBDa\0\xB86`\x04aS\xE9V[a\x01\xF0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEDa\0\xE86`\x04aT\x04V[a\x02\0V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD1V[a\x01\x15a\x01\x106`\x04aS\xE9V[a\x02\x1BV[`@Q\x90\x81R` \x01a\0\xD1V[a\x01\x15a\x0116`\x04aS\xE9V[a\x02%V[a\x01Ia\x01D6`\x04aS\xE9V[a\x02/V[`@Qa\0\xD1\x91\x90aUVV[a\x01ia\x01d6`\x04aVcV[a\x02OV[`@Qa\0\xD1\x91\x90aWMV[a\0\xBDa\x01\x846`\x04aS\xE9V[a\x02rV[a\x01Ia\x01\x976`\x04aX\x13V[a\x02|V[a\x01\xAFa\x01\xAA6`\x04aXSV[a\x02\x89V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xD1V[a\0\xBDa\x01\xD86`\x04aS\xE9V[a\x02\xAAV[a\x01\x15a\x01\xEB6`\x04aX\xB7V[a\x02\xB4V[_a\x01\xFA\x82a\x02\xCEV[\x92\x91PPV[__a\x02\x0E\x86\x86\x86\x86a\x03\x7FV[\x91P\x91P\x94P\x94\x92PPPV[_a\x01\xFA\x82a\x03\xBAV[_a\x01\xFA\x82a\x04~V[``_a\x02;\x83a\x04~V[\x90Pa\x02H\x83_\x83a\x04\x9DV[\x93\x92PPPV[``_a\x02\\\x84\x84a\x05kV[\x90Pa\x02j\x84\x84_\x84a\x05\xE1V[\x94\x93PPPPV[_a\x01\xFA\x82a\x06\xB1V[``a\x02j\x84\x84\x84a\x04\x9DV[___a\x02\x99\x88\x88\x88\x88\x88a\x06\xEDV[\x92P\x92P\x92P\x95P\x95P\x95\x92PPPV[_a\x01\xFA\x82a\x07\xDDV[_a\x02\xC2\x86\x86\x86\x86\x86a\x08.V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aY\x0EV[___a\x03\x8C\x86\x86a\x08aV[\x90P_a\x03\x99\x88\x83a\t\x08V[\x90P__a\x03\xA7\x83\x88a\x1B\x80V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04?\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90aY)V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x04\x0B\x90aY@V[``_a\x04\xAB\x85\x85\x85a\x1D3V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC8Wa\x04\xC8aYcV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x01W\x81` \x01[a\x04\xEEaO\x13V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x04\xE6W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x05aW_\x83\x82\x81Q\x81\x10a\x05\"Wa\x05\"aYwV[` \x02` \x01\x01Q\x90P_a\x057\x89\x83a\x1D\xD4V[\x90P\x80\x84\x84\x81Q\x81\x10a\x05LWa\x05LaYwV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05\x06V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x05\x84\x84a!\xFEV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02H\x91\x90aY)V[``_a\x05\xF0\x86\x86\x86\x86a\"\x82V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\rWa\x06\raYcV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06FW\x81` \x01[a\x063aO\x80V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06+W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x06\xA6W_\x83\x82\x81Q\x81\x10a\x06gWa\x06gaYwV[` \x02` \x01\x01Q\x90P_a\x06|\x8A\x83a#\nV[\x90P\x80\x84\x84\x81Q\x81\x10a\x06\x91Wa\x06\x91aYwV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x06KV[P\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x06\xFB\x88\x88a\x08aV[\x90P_a\x07\x08\x8A\x83a\t\x08V[\x90P_\x80\x80`\xFF\x89\x16a\x07>Wa\x07 \x8A\x85_a)\x18V[\x92\x95P\x91\x93Pa\x077\x91P\x85\x90P\x8B_\x80\x87a*\x07V[\x90Pa\x07nV[_\x19`\xFF\x8A\x16\x01a\x07nWa\x07T\x8A\x85_a*\xD4V[\x92\x95P\x91\x93Pa\x07k\x91P\x85\x90P_\x8C\x86\x82a*\x07V[\x90P[_a\x07x\x85a+\x9DV[\x90P_\x82\x82\x11a\x07\x91Wa\x07\x8C\x82\x84aY\x9FV[a\x07\x9BV[a\x07\x9B\x83\x83aY\x9FV[\x90P_a\x07\xA8\x82\x84a,6V[\x90P_\x84\x84\x11a\x07\xC0Wa\x07\xBB\x82aY\xB2V[a\x07\xC2V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x03\x0C\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[__a\x08:\x86\x86a\x08aV[\x90P_a\x08G\x88\x83a\t\x08V[\x90Pa\x08U\x81\x86\x86_a,qV[\x98\x97PPPPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x82W\x81\x83a\x08\x85V[\x82\x82[`@Q\x91\x94P\x92Pa\x08\xB2\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\t\x10aO\xD3V[\x82a\t\x19aO\xD3V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\t7\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aY\xCCV[a\t\xBCW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xFC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90aY\x0EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x1D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BQ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x90\x91\x90aY)V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xE6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CJ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90aY)V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xEA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90aY)V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\r\xF8\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x9B\x91\x90aY)V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0E\xFC\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x9F\x91\x90aY)V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10P\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10kW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8F\x91\x90aY)V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x118\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11w\x91\x90aY\x0EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12`\x91\x90aY)V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB7\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x136W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Z\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xBC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14 \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14_\x91\x90aY)V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xCB\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xD0\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x164\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16s\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x16\xCC\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17o\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xBD\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xED\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18!\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18<W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18`\x91\x90aY\x0EV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x18\xCE\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x192\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19q\x91\x90aY\x0EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\xD4\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ASW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Aw\x91\x90aY)V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x1A\xD0\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bs\x91\x90aY)V[`\x80\x82\x01R\x94\x93PPPPV[____a\x1B\x8CaP\x07V[\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF0\x91\x90aY)V[` \x82\x01Ra\x1C\0\x87_\x80a-\xA8V[P`\x80\x84\x01RP`@\x82\x01Ra\x1C\x18\x87`\x01_a-\xA8V[P`\xA0\x84\x01RP``\x82\x01R`@\x81\x01Q` \x82\x01Qa\x1C9\x91\x88\x91a.SV[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\x1CT\x91\x88\x91a.SV[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x99\x90\x98P\x91\x96P\x94P\x92PPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1DS\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xADW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02j\x91\x90\x81\x01\x90aZ\x1CV[a\x1D\xDCaO\x13V[a\x1D\xE4aPPV[a\x1D\xEE\x84\x84a\t\x08V[\x81Ra\x1D\xF9\x84a/\x12V[` \x82\x01\x81\x90R\x81Qa\x1E\r\x91_\x90a-\xA8V[`\xA0\x85\x01R`\x80\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x1E5\x91\x90`\x01\x90a-\xA8V[`\xA0\x85\x01Ra\x01\0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01R`@\x80Qa\x03\0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\x80\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01@\x86\x01\x94\x85\x94\x93a\x01\xA0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xBEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE5\x91\x90\x81\x01\x90aZ\xC4V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FV\x91\x90a[XV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R\x87QQQ`@\x90\x81\x01Q\x81\x84\x01R\x88QQQ``\x90\x81\x01Q\x81\x85\x01R\x89QQQ`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\xA0\x90\x81\x01Q\x81\x87\x01R\x83\x8C\x01Q`\xC0\x87\x01R\x91\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a 2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra Y\x91\x90\x81\x01\x90aZ\xC4V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xCD\x91\x90a[XV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01R\x87QQ\x81\x01Q`@\x90\x81\x01Q\x81\x84\x01R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xC0\x80\x8D\x01Q\x81\x88\x01R`\xE0\x80\x8E\x01Q\x90\x88\x01Ra\x01\0\x80\x8E\x01Q\x90\x88\x01Ra\x01 \x80\x8E\x01Q\x97\x01\x96\x90\x96R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a!\xA4\x90a+\x9DV[\x81R` \x01a!\xB3\x86\x86a/[V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a!\xDA\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a!\xEB\x86\x86a0KV[\x90Ra\x01@\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a\"8\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a\"\x9C\x86a!\xFEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xE3W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xC5\x91\x90\x81\x01\x90aZ\x1CV[a#\x12aO\x80V[a#\x1AaP\xACV[a#$\x84\x84a1MV[\x80\x82RQ\x80QQ` \x90\x91\x01QQa#<\x91\x90a\x08aV[`@\x82\x01\x81\x90Ra#N\x90\x85\x90a\t\x08V[` \x82\x01\x81\x90R\x81Qa#b\x91\x86\x91a1_V[PPPP``\x82\x01R` \x81\x01Qa#y\x90a+\x9DV[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa#\xAF\x91\x90_[` \x02\x01Q`@\x01Q\x90a1\xAEV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa#\xC7\x90_a1\xEFV[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa#\xE0\x91\x90a2\x1DV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa#\xF7\x91\x90a2>V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa$\x18\x92\x91\x90a2ZV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa$9\x92\x87\x92\x90\x91_a2wV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa$f\x92\x91\x90a2ZV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa$\x80\x90`\x01a1\xEFV[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa$\xB1\x91\x90`\x01a#\xA0V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa$\xC8\x91a2\x1DV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa$\xE0\x91\x90a2>V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa$\xF8\x91\x90a1\xAEV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa%\x1B\x92\x91\x90a2ZV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%=\x92\x87\x92\x90\x91`\x01a2wV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa%l\x92\x91\x90a2ZV[a\x02\xE0\x82\x01R\x80Qa%}\x90a4\x0CV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a&uWa%\xA7\x81a\x03\0\x01Q\x82`\x80\x01Qa2\x1DV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa%\xBE\x91a1\xAEV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa%\xEF\x93a4NV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa&V\x91\x86\x91a&\x17\x91\x90a2>V[a&)\x84`\xC0\x01Q\x85`\xA0\x01Qa2>V[a&=\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa2>V[a&Q\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa2>V[a4uV[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa&n\x91\x90a55V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'\x10\x91\x90\x81\x01\x90aZ\xC4V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a'eWa'eaYwV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a'\xB4Wa'\xB4aYwV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x14W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(;\x91\x90\x81\x01\x90aZ\xC4V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a)$aQ\x89V[a)/\x87__a-\xA8V[P`@\x84\x01RP\x81Ra)D\x87`\x01_a-\xA8V[P``\x84\x01RP` \x82\x01R\x85\x15a)kW\x87\x81_\x01\x81\x81Qa)g\x91\x90aY\x9FV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90Ra)\x9A\x90\x89\x90a)\x92\x90a'\x10\x90a5NV[a'\x10a.SV[a\x01@\x82\x01\x81\x90R\x81Q` \x83\x01Qa)\xBD\x92a)\xB8\x90\x83\x90a5\xA8V[a.SV[`\x80\x82\x01\x81\x90R` \x82\x01Qa)\xD2\x91a5NV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Qa)\xF4\x90\x8C\x90a5\xFCV[\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a*\x17WP\x83\x15[\x15a*&WP\x83\x90P\x84a*[V[_\x87\x11\x80\x15a*3WP\x84\x15[\x15a*BWP\x85\x90P\x82a*[V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*j\x89``\x01Q_a1\xEFV[\x90P_a*|\x8A``\x01Q`\x01a1\xEFV[\x90P_a*\x9A\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_a*\xB8\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90Pa*\xC4\x82\x82a,6V[\x9C\x9BPPPPPPPPPPPPV[____a*\xE0aQ\x89V[a*\xEB\x87__a-\xA8V[P`@\x84\x01RP\x81Ra+\0\x87`\x01_a-\xA8V[P``\x84\x01RP` \x82\x01R\x85\x15a+(W\x87\x81` \x01\x81\x81Qa+$\x91\x90aY\x9FV[\x90RP[\x80Q` \x82\x01Qa+>\x91\x90a)\xB8\x81\x8Ca5\xA8V[`\x80\x82\x01\x81\x90R\x81Qa+P\x91a5NV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01 \x82\x01\x81\x90R`\xA0\x82\x01Qa+\x7F\x91a)\x92\x90a'\x10\x90a5NV[`@\x82\x01Q``\x83\x01Qa\x01 \x84\x01Q`\xA0\x85\x01Qa)\xF4\x91a5\xFCV[__a+\xAA\x83__a-\xA8V[PPP\x90P_a+\xBC\x84`\x01_a-\xA8V[PPP\x90P\x80_\x03a+\xD1WP_\x93\x92PPPV[_a+\xE0\x85``\x01Q_a1\xEFV[\x90P_a+\xF2\x86``\x01Q`\x01a1\xEFV[\x90P_a,\x10\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_a,.\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90Pa\x08U\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a,WW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_a,zaP\x07V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xDE\x91\x90aY)V[` \x82\x01Ra,\xEE\x86_\x80a-\xA8V[PPP`\xC0\x82\x01Ra-\x02\x86`\x01_a-\xA8V[PPP`\xE0\x82\x01R\x82\x15a-=W\x84\x81`\xC0\x01\x81\x81Qa-\"\x91\x90aY\x9FV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a-9\x90\x83\x90aY\x9FV[\x90RP[\x80` \x01Q_\x03a-mWa-fa\x03\xE8a-`a-[\x88\x88a6 V[a6\x86V[\x90a5NV[\x81Ra-\x9EV[a-\x9Ba-\x83\x86\x83` \x01Q\x84`\xC0\x01Qa.SV[a-\x96\x86\x84` \x01Q\x85`\xE0\x01Qa.SV[a7fV[\x81R[Q\x95\x94PPPPPV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10a-\xC5Wa-\xC5aYwV[` \x02\x01Q\x90P_a-\xD7\x89\x89a7{V[\x90P\x80_\x03a-\xF3W____\x95P\x95P\x95P\x95PPPa)\xFEV[_a.\x02\x83\x8B`\x80\x01Qa8MV[\x90P_\x88\x15a.%W\x81a.\x16\x84\x8Ba1\xAEV[a. \x91\x90aY\x9FV[a.'V[_[\x90Pa.3\x82\x84a\\aV[\x83a.>\x84\x82aY\x9FV[\x91\x99P\x97P\x95P\x93PPPP\x93P\x93P\x93P\x93V[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a.\x87W\x83\x82\x81a.}Wa.}a\\tV[\x04\x92PPPa\x02HV[\x80\x84\x11a.\xA7W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10\x93\xD4\x94\x93\xD5\xD7\xD4\x90U\x11W\xD5\x12\x14\x91T\xD2\x13\xD3\x11`Z\x1B`@\x82\x01R``\x01\x90V[__a/g\x84\x84a8}V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a/\xA8\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/\xD8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0\x0C\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aY\x0EV[__a0W\x84\x84a8}V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a0\xAA\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a0\xDA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a1\x0E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02j\x91\x90aY)V[a1UaQ\xD8V[a\x02H\x83\x83a98V[_____a1m\x87a+\x9DV[\x90Pa1{\x87\x87\x83_aKVV[\x90\x93P\x91P\x81a1\x8CW_\x19a1\x96V[a1\x96\x83\x83a,6V[\x94Pa1\xA1\x88aL\xC2V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a1\xCEW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a2\x0FWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a24Wa2/\x83\x83aY\x9FV[a\x02HV[a\x02H\x82\x84aY\x9FV[_a\x02H\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[_\x82\x84\x11a2pWa2k\x82aY\xB2V[a\x02jV[P\x92\x91PPV[_a2\xB1`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a2\xBD\x86\x86\x86_aKVV[` \x83\x01R\x80\x82R\x15\x80a2\xEEWP\x84Q`\xFF\x84\x16`\x02\x81\x10a2\xE2Wa2\xE2aYwV[` \x02\x01Q` \x01Q_\x14[\x15a2\xFCW_\x91PPa\x02\xC5V[a3\x05\x87aM\x13V[`@\x82\x01\x81\x90R` \x82\x01Qa3\x1A\x91a1\xAEV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a32W_\x91PPa\x02\xC5V[`\x80\x81\x01Q\x81Qa3C\x91\x90aY\x9FV[\x81``\x01\x81\x81RPPa3Z\x86``\x01Q\x84a1\xEFV[`\xA0\x82\x01\x81\x90R``\x82\x01Qa3\x86\x91a3u\x90`\na\\VV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba.SV[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a3\xABW`\xC0\x81\x01Qa3\xA5\x90\x85a,6V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a3\xC1Wa3\xC1aYwV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a3\xFEW\x84Q`\xFF\x84\x16`\x02\x81\x10a3\xEBWa3\xEBaYwV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a4,WPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa4GWPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x80\x15a4[WP\x82\x84\x11[a4mWa4h\x82aY\xB2V[a\x02\xC5V[P\x93\x92PPPV[__a4\x80\x87aL\xC2V[\x90P_a4\x8D\x82\x87a1\xAEV[\x90P_a4\x9A\x83\x86a1\xAEV[\x90P_a4\xA7\x89\x84a\\\x88V[\x90P_a4\xB4\x83\x89a\\\x88V[\x90P_a4\xC0\x83aMYV[\x90P_a4\xCC\x83aMYV[\x90P_\x84\x13\x80\x15a4\xDCWP_\x83\x12[\x80a4\xF0WP_\x84\x12\x80\x15a4\xF0WP_\x83\x13[\x15a5\x04W_\x97PPPPPPPPa\x02\xC5V[\x80_\x03a5\x1AW_\x97PPPPPPPPa\x02\xC5V[a5$\x82\x82a,6V[\x9D\x9CPPPPPPPPPPPPPV[_\x82\x15a5FWa2/\x82\x84a,6V[P_\x92\x91PPV[_\x82a5Z\x83\x82aY\x9FV[\x91P\x81\x11\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x82a5\xB4\x83\x82a\\aV[\x91P\x81\x10\x15a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a5\x9FV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a6\x12W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x81\x15\x80a6CWP\x82\x82a65\x81\x83a\\\xA7V[\x92Pa6A\x90\x83a\\\xBEV[\x14[a\x01\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a5\x9FV[_\x81_\x03a6\x95WP_\x91\x90PV[_`\x01a6\xA1\x84aMnV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a6\xBAWa6\xBAa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xD2Wa6\xD2a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a6\xEAWa6\xEAa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7\x02Wa7\x02a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7\x1AWa7\x1Aa\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a72Wa72a\\tV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a7JWa7Ja\\tV[\x04\x82\x01\x90\x1C\x90Pa\x02H\x81\x82\x85\x81a7dWa7da\\tV[\x04[_\x81\x83\x10a7tW\x81a\x02HV[P\x90\x91\x90PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10a7\x95Wa7\x95aYwV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x12\x91\x90aY)V[\x90P\x80_\x03a8%W_\x92PPPa\x01\xFAV[``\x82\x01Q`\xC0\x83\x01Qa89\x82\x84aY\x9FV[a8C\x91\x90aY\x9FV[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03a8`WP_a\x01\xFAV[_a8k\x84\x84aN\x01V[`\xA0\x85\x01Q\x90\x91Pa\x02j\x90\x82a1\xAEV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a8\xA0\x90aY@V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xF4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x18\x91\x90aY\xCCV[a\x02HW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a5\x9FV[a9@aQ\xD8V[\x82a9IaQ\xD8V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a9\x89\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\x01\x91\x90aY\xCCV[a:\x0EW\x91Pa\x01\xFA\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a:H\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:x\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xAC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xEB\x91\x90aY)V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a;3\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;c\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x97\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xD6\x91\x90aY\x0EV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a<2\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<b\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xD5\x91\x90aY\x0EV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=P\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xC3\x91\x90aY)V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\x17\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>G\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>{\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xBA\x91\x90aY)V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a?\x14\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?x\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\xB7\x91\x90aY)V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a@\x10\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xB3\x91\x90aY)V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aAm\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xAC\x91\x90aY)V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aB\x06\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aBj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB\xA9\x91\x90aY)V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aCP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aCkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x8F\x91\x90aY)V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDv\x91\x90aY\x0EV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x1D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\\\x91\x90aY)V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aE\xB1\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aE\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aF\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aFT\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aF\xAF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aF\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aG\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGR\x91\x90aY)V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aG\xAC\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aG\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aH\x10\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHO\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aH\xB1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aH\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aIT\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aI\xAF\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aI\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJ\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJR\x91\x90aY)V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ\xA1\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKD\x91\x90aY)V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aK\xF9W__aK\x81\x8A\x8A_aNGV[\x91P\x91P_aK\x9D_\x8C``\x01Qa1\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aK\xBB\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[\x90P_aK\xD9\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90PaK\xE5\x82\x88a\\aV[\x96PaK\xF1\x81\x87a\\aV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aL\xB5W__aL \x8A\x8A`\x01aNGV[\x91P\x91P_aL=`\x01\x8C``\x01Qa1\xEF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aL[\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x85`\na\\VV[\x90P_aLy\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba)\xB8\x86`\na\\VV[\x90P_aL\x86\x83\x8Da1\xAEV[\x90P_aL\x93\x83\x8Ea1\xAEV[\x90PaL\x9F\x82\x8Aa\\aV[\x98PaL\xAB\x81\x89a\\aV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x04\x0B\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15aMjW\x81_\x03a\x01\xFAV[P\x90V[_\x80`\x80\x83\x90\x1C\x15aM\x82W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aM\x94W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aM\xA6W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aM\xB8W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aM\xCAW`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aM\xDCW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aM\xEEW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x01\xFAW`\x01\x01\x92\x91PPV[_B\x82\x03aN\x14WP` \x82\x01Qa\x01\xFAV[_aN#\x84`@\x01Q\x84aN\xDFV[\x90PaN<\x84` \x01Q\x82a1\xAE\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x01\xFAV[PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aNbWaNbaYwV[` \x02\x01Q`@\x01Q\x90P_aN\x98\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10aN\x89WaN\x89aYwV[` \x02\x01Q\x88`\x80\x01QaN\x01V[\x90P\x81\x15aN\xAFWaN\xAA\x82\x82a1\xAEV[aN\xB1V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aN\xCAWaN\xCAaYwV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80aN\xEB\x83BaY\x9FV[aN\xF5\x90\x85a\\\xA7V[c\x01\xE13\x80\x90\x04\x90Pa\x02j\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\\aV[`@Q\x80a\x01@\x01`@R\x80aO'aQ\xFEV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aO\x94aR\x85V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80aO\xE6aS\0V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01@\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80aPdaO\xD3V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01aP\xA7aO\x13V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80aP\xC0aQ\xD8V[\x81R` \x01aP\xCDaO\xD3V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aQ\xEBaSgV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aRo`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\rW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aR\xEA`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aR\x94W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aSQ`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aS\x0FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aS\xBF`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81aSvW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aNDW__\xFD[_` \x82\x84\x03\x12\x15aS\xF9W__\xFD[\x815a\x02H\x81aS\xD5V[____`\x80\x85\x87\x03\x12\x15aT\x17W__\xFD[\x845aT\"\x81aS\xD5V[\x93P` \x85\x015aT2\x81aS\xD5V[\x92P`@\x85\x015aTB\x81aS\xD5V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aUKW\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01RaT\xC1a\x01\x80\x86\x01\x82aTRV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaT\x89V[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aVWW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01@\x87RaU\xA4a\x01@\x88\x01\x82aT\x80V[\x90P` \x82\x01QaU\xC0` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QaU\xDB`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01QaV\x1E`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01QaV4a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x91\x82\x01Q\x96\x90\x91\x01\x95\x90\x95R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aU|V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aVtW__\xFD[\x825aV\x7F\x81aS\xD5V[\x91P` \x83\x015aV\x8F\x81aS\xD5V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15aUKW\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01RaV\xDBa\x01@\x86\x01\x82aTRV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90PaV\xA3V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aVWW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87RaW\x9Ba\x01 \x88\x01\x82aV\x9AV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QaW\xC1`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aWsV[___``\x84\x86\x03\x12\x15aX%W__\xFD[\x835aX0\x81aS\xD5V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\xFF\x81\x16\x81\x14aNDW__\xFD[_____`\xA0\x86\x88\x03\x12\x15aXgW__\xFD[\x855aXr\x81aS\xD5V[\x94P` \x86\x015aX\x82\x81aS\xD5V[\x93P`@\x86\x015aX\x92\x81aS\xD5V[\x92P``\x86\x015\x91P`\x80\x86\x015aX\xA9\x81aXEV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15aX\xCBW__\xFD[\x855aX\xD6\x81aS\xD5V[\x94P` \x86\x015aX\xE6\x81aS\xD5V[\x93P`@\x86\x015aX\xF6\x81aS\xD5V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[_` \x82\x84\x03\x12\x15aY\x1EW__\xFD[\x81Qa\x02H\x81aS\xD5V[_` \x82\x84\x03\x12\x15aY9W__\xFD[PQ\x91\x90PV[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x01\xFAWa\x01\xFAaY\x8BV[_`\x01`\xFF\x1B\x82\x01aY\xC6WaY\xC6aY\x8BV[P_\x03\x90V[_` \x82\x84\x03\x12\x15aY\xDCW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x02HW__\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aZ\x14WaZ\x14aYcV[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15aZ,W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZRW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZlWaZlaYcV[\x80`\x05\x1BaZ|` \x82\x01aY\xEBV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x87\x84\x11\x15aZ\x97W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15aZ\xB9W\x84Q\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aZ\x9EV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15aZ\xD4W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xEAW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aZ\xFAW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x14Wa[\x14aYcV[a['`\x1F\x82\x01`\x1F\x19\x16` \x01aY\xEBV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a[;W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a[hW__\xFD[\x81Qa\x02H\x81aXEV[`\x01\x81[`\x01\x84\x11\x15a[\xAEW\x80\x85\x04\x81\x11\x15a[\x92Wa[\x92aY\x8BV[`\x01\x84\x16\x15a[\xA0W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a[wV[\x93P\x93\x91PPV[_\x82a[\xC4WP`\x01a\x01\xFAV[\x81a[\xD0WP_a\x01\xFAV[\x81`\x01\x81\x14a[\xE6W`\x02\x81\x14a[\xF0Wa\\\x0CV[`\x01\x91PPa\x01\xFAV[`\xFF\x84\x11\x15a\\\x01Wa\\\x01aY\x8BV[PP`\x01\x82\x1Ba\x01\xFAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\\/WP\x81\x81\na\x01\xFAV[a\\;_\x19\x84\x84a[sV[\x80_\x19\x04\x82\x11\x15a\\NWa\\NaY\x8BV[\x02\x93\x92PPPV[_a\x02H\x83\x83a[\xB6V[\x80\x82\x01\x80\x82\x11\x15a\x01\xFAWa\x01\xFAaY\x8BV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a2pWa2paY\x8BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x01\xFAWa\x01\xFAaY\x8BV[_\x82a\\\xD8WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC2\xFC\xEB\xFF`\x8DU\xE3p\xFE\xC1\xC2\xD8\xF2\x82\xD3\x8C{\x17u\xC7qN\xF0%\xA3\x01\x0F\xA0\xC3\x02\xBFdsolcC\0\x08\x1C\x003",
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
