///Module containing a contract's types and functions.
/**

```solidity
library Pool {
    struct Asset { address token; uint256 borrowIndex; uint256 borrowRate; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 unclaimedFee; }
    struct Props { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod Pool {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Asset { address token; uint256 borrowIndex; uint256 borrowRate; uint256 totalCollateral; uint256 totalCollateralWithDebt; uint256 totalDebtScaled; uint256 unclaimedFee; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Asset {
        pub token: alloy::sol_types::private::Address,
        pub borrowIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateralWithDebt: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
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
                    value.borrowIndex,
                    value.borrowRate,
                    value.totalCollateral,
                    value.totalCollateralWithDebt,
                    value.totalDebtScaled,
                    value.unclaimedFee,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Asset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    borrowIndex: tuple.1,
                    borrowRate: tuple.2,
                    totalCollateral: tuple.3,
                    totalCollateralWithDebt: tuple.4,
                    totalDebtScaled: tuple.5,
                    unclaimedFee: tuple.6,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowRate),
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
                    "Asset(address token,uint256 borrowIndex,uint256 borrowRate,uint256 totalCollateral,uint256 totalCollateralWithDebt,uint256 totalDebtScaled,uint256 unclaimedFee)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowRate)
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.unclaimedFee)
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
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowRate,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
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
                    &rust.borrowRate,
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
    /**```solidity
struct Props { Asset[2] assets; address bank; address interestRateStrategy; uint256 configuration; uint256 lastUpdateTimestamp; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Props {
        pub assets: [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
        pub bank: alloy::sol_types::private::Address,
        pub interestRateStrategy: alloy::sol_types::private::Address,
        pub configuration: alloy::sol_types::private::primitives::aliases::U256,
        pub lastUpdateTimestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [<Asset as alloy::sol_types::SolType>::RustType; 2usize],
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
        impl ::core::convert::From<Props> for UnderlyingRustTuple<'_> {
            fn from(value: Props) -> Self {
                (
                    value.assets,
                    value.bank,
                    value.interestRateStrategy,
                    value.configuration,
                    value.lastUpdateTimestamp,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Props {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    assets: tuple.0,
                    bank: tuple.1,
                    interestRateStrategy: tuple.2,
                    configuration: tuple.3,
                    lastUpdateTimestamp: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Props {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Props {
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
        impl alloy_sol_types::SolType for Props {
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
        impl alloy_sol_types::SolStruct for Props {
            const NAME: &'static str = "Props";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Props(Asset[2] assets,address bank,address interestRateStrategy,uint256 configuration,uint256 lastUpdateTimestamp)",
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
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Props {
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
    /**Creates a new wrapper around an on-chain [`Pool`](self) contract instance.

See the [wrapper's documentation](`PoolInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, provider: P) -> PoolInstance<T, P, N> {
        PoolInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Pool`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Pool`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Pool`](self) contract instance.

See the [wrapper's documentation](`PoolInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> PoolInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolInstance<T, P, N> {
            PoolInstance {
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
    > PoolInstance<T, P, N> {
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
    > PoolInstance<T, P, N> {
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
library Pool {
    struct Asset {
        address token;
        uint256 borrowIndex;
        uint256 borrowRate;
        uint256 totalCollateral;
        uint256 totalCollateralWithDebt;
        uint256 totalDebtScaled;
        uint256 unclaimedFee;
    }
    struct Props {
        Asset[2] assets;
        address bank;
        address interestRateStrategy;
        uint256 configuration;
        uint256 lastUpdateTimestamp;
    }
}

interface PoolFactory {
    struct CreatePoolParams {
        address token;
        address source;
    }

    error EmptyBase();
    error EmptyConfiguration();
    error EmptyInterestRateStrategy();
    error EmptyPool(bytes32 key);
    error InvalidDecimals(uint256 decimals, uint256 MaxValidDecimals);
    error PoolAlreadyExists(bytes32 key, address poolBank);

    constructor(address _roleStore, address _dataStore, address _eventEmitter);

    function createPool(CreatePoolParams memory params) external returns (Pool.Props memory);
    function dataStore() external view returns (address);
    function eventEmitter() external view returns (address);
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
    "name": "createPool",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct CreatePoolParams",
        "components": [
          {
            "name": "token",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "source",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Pool.Props",
        "components": [
          {
            "name": "assets",
            "type": "tuple[2]",
            "internalType": "struct Pool.Asset[2]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "borrowIndex",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "borrowRate",
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
                "name": "unclaimedFee",
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
          }
        ]
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
    "name": "EmptyBase",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyConfiguration",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyInterestRateStrategy",
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
    "name": "InvalidDecimals",
    "inputs": [
      {
        "name": "decimals",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "MaxValidDecimals",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "PoolAlreadyExists",
    "inputs": [
      {
        "name": "key",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "poolBank",
        "type": "address",
        "internalType": "address"
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
pub mod PoolFactory {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b50604051614c0a380380614c0a83398101604081905261002e91610062565b6001600160a01b0392831660805290821660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c051614af16101195f395f818160de01526105ff01525f818160970152818161011501528181610160015281816101b60152818161022f015281816103d90152818161054d0152818161057e01526105b801525f8181605301526103b70152614af15ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80634a4a7b041461004e578063660d0d67146100925780638251a687146100b95780639ff78c30146100d9575b5f5ffd5b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100cc6100c7366004613129565b610100565b6040516100899190613142565b6100757f000000000000000000000000000000000000000000000000000000000000000081565b610108613016565b61011061304a565b6101397f0000000000000000000000000000000000000000000000000000000000000000610655565b8082525f0361015b57604051630615264d60e21b815260040160405180910390fd5b6101847f000000000000000000000000000000000000000000000000000000000000000061071f565b6001600160a01b0316602082018190526101b157604051637d42556760e11b815260040160405180910390fd5b6101da7f00000000000000000000000000000000000000000000000000000000000000006107e3565b6001600160a01b031660408201819052610207576040516308d1270b60e21b815260040160405180910390fd5b60408101516102229061021d602086018661320f565b610821565b60608201819052610254907f0000000000000000000000000000000000000000000000000000000000000000906108c8565b60808201819052602001516001600160a01b0316156102ab576060810151608082015160200151604051630b8422c360e01b815260048101929092526001600160a01b031660248201526044015b60405180910390fd5b80604001516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030f919061322a565b60ff1660c0820152610324602084018461320f565b6001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801561035f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610383919061322a565b60ff1660e082015260c0810151815161039c915f611aac565b80825260e08201516103b091906001611aac565b81526040517f0000000000000000000000000000000000000000000000000000000000000000907f000000000000000000000000000000000000000000000000000000000000000090610402906130b5565b6001600160a01b03928316815291166020820152604001604051809103905ff080158015610432573d5f5f3e3d5ffd5b506001600160a01b0390811660a080840191909152604080516101c0810182528185015190931660e08085019182526b033b2e3c9fd0803ce80000006101008601525f61012086018190526101408601819052610160860181905261018086018190526101a08601529284019081528151928301909152829160c0830190806104be60208a018a61320f565b6001600160a01b031681526020016b033b2e3c9fd0803ce800000081526020015f81526020015f81526020015f81526020015f81526020015f81525081525081526020018260a001516001600160a01b0316815260200182602001516001600160a01b03168152602001825f015181526020016105384290565b905261010082018190526060820151610573917f00000000000000000000000000000000000000000000000000000000000000009190611b09565b60608101516105b3907f0000000000000000000000000000000000000000000000000000000000000000906105ae604087016020880161320f565b612d07565b6105e87f000000000000000000000000000000000000000000000000000000000000000082606001516105e34290565b612e04565b610100810151518051516020918201515161064a927f0000000000000000000000000000000000000000000000000000000000000000929190610631906040890190890161320f565b856101000151608001518660c001518760e00151612ed0565b610100015192915050565b5f816001600160a01b031663bd02d0f56040516020016106a6906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016106da91815260200190565b602060405180830381865afa1580156106f5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610719919061324a565b92915050565b5f816001600160a01b03166321f8a721604051602001610770906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016107a491815260200190565b602060405180830381865afa1580156107bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107199190613261565b5f816001600160a01b03166321f8a721604051602001610770906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b5f816001600160a01b0316836001600160a01b031610610842578183610845565b82825b6040519194509250610872906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b6108d0613016565b826108d9613016565b816001600160a01b03166391d4403c6040516020016108f79061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561094b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061096f919061329f565b61097c5791506107199050565b816001600160a01b03166321f8a721856040516020016109bc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016109ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a2091815260200190565b602060405180830381865afa158015610a3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5f9190613261565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610add929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b1191815260200190565b602060405180830381865afa158015610b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b50919061324a565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610ba6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610bd6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c0a91815260200190565b602060405180830381865afa158015610c25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c49919061324a565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610ca4906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610cd4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d0891815260200190565b602060405180830381865afa158015610d23573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d47919061324a565b815151606001526040516001600160a01b0383169063bd02d0f5908690610d70906020016132be565b60405160208183030381529060405280519060200120604051602001610da0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610dd491815260200190565b602060405180830381865afa158015610def573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e13919061324a565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610e6f906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001610e9f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610ed391815260200190565b602060405180830381865afa158015610eee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f12919061324a565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610f8f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610fc391815260200190565b602060405180830381865afa158015610fde573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611002919061324a565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611077929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110ab91815260200190565b602060405180830381865afa1580156110c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ea9190613261565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161119491815260200190565b602060405180830381865afa1580156111af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111d3919061324a565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161122a90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161125a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161128e91815260200190565b602060405180830381865afa1580156112a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112cd919061324a565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161132990602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611359929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161138d91815260200190565b602060405180830381865afa1580156113a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113cc919061324a565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016113fc906132ff565b6040516020818303038152906040528051906020012060405160200161142c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161146091815260200190565b602060405180830381865afa15801561147b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061149f919061324a565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016114fc90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161152c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161156091815260200190565b602060405180830381865afa15801561157b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159f919061324a565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016115f890602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611628929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161165c91815260200190565b602060405180830381865afa158015611677573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061169b919061324a565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016116e990602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611719929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161174d91815260200190565b602060405180830381865afa158015611768573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061178c9190613261565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016117fa906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161182a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161185e91815260200190565b602060405180830381865afa158015611879573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061189d9190613261565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611900906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611930929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161196491815260200190565b602060405180830381865afa15801561197f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a3919061324a565b60608201526040516001600160a01b0383169063bd02d0f59086906119fc906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a6091815260200190565b602060405180830381865afa158015611a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a9f919061324a565b6080820152949350505050565b5f60ff831115611ad95760405163c3ca0e3760e01b81526004810184905260ff60248201526044016102a2565b60ff60581b1960585f1960ff851601611af8575060ff60601b19905060605b9085169084901b1790509392505050565b5f839050806001600160a01b031663c80f4c62604051602001611b2b9061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611b7b575f5ffd5b505af1158015611b8d573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611bd1906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611c01929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611c64573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c889190613261565b50806001600160a01b031663e2a4853a84604051602001611cd0906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d00929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611d5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d81919061324a565b50806001600160a01b031663e2a4853a84604051602001611dc8906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611df8929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611e54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e78919061324a565b50806001600160a01b031663e2a4853a84604051602001611ec4906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ef4929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f51573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f75919061324a565b50806001600160a01b031663e2a4853a84604051602001611f95906132be565b60405160208183030381529060405280519060200120604051602001611fc5929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612022573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612046919061324a565b50806001600160a01b031663e2a4853a84604051602001612093906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016120c3929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612120573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612144919061324a565b50806001600160a01b031663e2a4853a8460405160200161218d906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016121bd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561221a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061223e919061324a565b50806001600160a01b031663ca446dd98460405160200161227f906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016122af929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612315573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123399190613261565b50806001600160a01b031663e2a4853a8460405160200161238190602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016123b1929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612410573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612434919061324a565b50806001600160a01b031663e2a4853a8460405160200161247b90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016124ab929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561250a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061252e919061324a565b50806001600160a01b031663e2a4853a8460405160200161257a90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016125aa929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561260a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061262e919061324a565b50806001600160a01b031663e2a4853a8460405160200161264e906132ff565b6040516020818303038152906040528051906020012060405160200161267e929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156126de573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612702919061324a565b50806001600160a01b031663e2a4853a8460405160200161274f90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161277f929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156127df573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612803919061324a565b50806001600160a01b031663e2a4853a8460405160200161284c90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161287c929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128dc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612900919061324a565b50806001600160a01b031663ca446dd98460405160200161293e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161296e929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016129b89291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af11580156129d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129f89190613261565b50806001600160a01b031663ca446dd984604051602001612a4a906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612a7a929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612ac5926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612ae1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b059190613261565b50806001600160a01b031663e2a4853a84604051602001612b4c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b7c929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612bbd929190918252602082015260400190565b6020604051808303815f875af1158015612bd9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bfd919061324a565b50806001600160a01b031663e2a4853a84604051602001612c4f906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612c7f929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612cc0929190918252602082015260400190565b6020604051808303815f875af1158015612cdc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d00919061324a565b5050505050565b5f612d128484612f54565b9050806001600160a01b031663ca446dd984604051602001612d53906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d83929190918252602082015260400190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201526001600160a01b03851660248201526044016020604051808303815f875af1158015612de0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d009190613261565b5f612e0f8484612f54565b9050806001600160a01b031663e2a4853a84604051602001612e629060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612e92929190918252602082015260400190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101859052604401612cc0565b604051632ad6425d60e11b81526001600160a01b03878116600483015286811660248301528581166044830152606482018590526084820184905260a482018390528816906355ac84ba9060c4015f604051808303815f87803b158015612f35575f5ffd5b505af1158015612f47573d5f5f3e3d5ffd5b5050505050505050505050565b5f5f839050806001600160a01b03166391d4403c604051602001612f779061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015612fcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fef919061329f565b61300f57604051637357d91f60e01b8152600481018490526024016102a2565b9392505050565b6040518060a001604052806130296130c2565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061012001604052805f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001613088613016565b81526020015f6001600160a01b031681526020015f81526020015f81526020016130b0613016565b905290565b61177b8061334183390190565b60405180604001604052806002905b6131136040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816130d15790505090565b5f604082840312801561313a575f5ffd5b509092915050565b8151610240820190825f5b60028110156131b857825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e08201915060208301925060018101905061314d565b50505060208301516001600160a01b039081166101c08401526040840151166101e083015260608301516102008301526080909201516102209091015290565b6001600160a01b038116811461320c575f5ffd5b50565b5f6020828403121561321f575f5ffd5b813561300f816131f8565b5f6020828403121561323a575f5ffd5b815160ff8116811461300f575f5ffd5b5f6020828403121561325a575f5ffd5b5051919050565b5f60208284031215613271575f5ffd5b815161300f816131f8565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f602082840312156132af575f5ffd5b8151801515811461300f575f5ffd5b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b60608201526080019056fe60c060405234801561000f575f5ffd5b5060405161177b38038061177b83398101604081905261002e91610126565b6001600160a01b039182166080908152911660a090815260408051808201825260028152614d4d60f01b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527f13b913c1f317dd71b6dfe53abe9380c652f9498c8de25b6b39d8263499ae7bb6818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc66060820152469481019490945230848401528151808503909301835260c09093019052805191012060045561015e565b6001600160a01b0381168114610123575f5ffd5b50565b5f5f60408385031215610137575f5ffd5b82516101428161010f565b60208401519092506101538161010f565b809150509250929050565b60805160a0516115ee61018d5f395f818161014a01526103ac01525f81816103360152610a2901526115ee5ff3fe60806040526004361061013f575f3560e01c8063523fba7f116100b35780639dc29fac1161006d5780639dc29fac14610443578063a9059cbb14610462578063d505accf14610481578063dd62ed3e146104a0578063e42c08f2146104d6578063eb40133f146104f5575f5ffd5b8063523fba7f14610370578063660d0d671461039b57806370a08231146103ce5780637ecebe00146103f95780638c1b5fde1461042457806395d89b41146101a6575f5ffd5b806330adf81f1161010457806330adf81f14610279578063313ce567146102ac578063352f9aed146102d25780633644e515146102f157806340c10f19146103065780634a4a7b0414610325575f5ffd5b806306fdde03146101a6578063078d3b79146101e9578063095ea7b31461020857806318160ddd1461023757806323b872dd1461025a575f5ffd5b366101a2575f61016e7f0000000000000000000000000000000000000000000000000000000000000000610514565b9050336001600160a01b038216146101a05760405163738d28df60e11b81523360048201526024015b60405180910390fd5b005b5f5ffd5b3480156101b1575f5ffd5b506101d3604051806040016040528060028152602001614d4d60f01b81525081565b6040516101e0919061126c565b60405180910390f35b3480156101f4575f5ffd5b506101a0610203366004611299565b6105c4565b348015610213575f5ffd5b506102276102223660046112d7565b610624565b60405190151581526020016101e0565b348015610242575f5ffd5b5061024c60015481565b6040519081526020016101e0565b348015610265575f5ffd5b50610227610274366004611299565b610639565b348015610284575f5ffd5b5061024c7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c981565b3480156102b7575f5ffd5b506102c0601281565b60405160ff90911681526020016101e0565b3480156102dd575f5ffd5b5061024c6102ec366004611301565b610687565b3480156102fc575f5ffd5b5061024c60045481565b348015610311575f5ffd5b506101a06103203660046112d7565b6106a3565b348015610330575f5ffd5b506103587f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101e0565b34801561037b575f5ffd5b5061024c61038a366004611301565b5f6020819052908152604090205481565b3480156103a6575f5ffd5b506103587f000000000000000000000000000000000000000000000000000000000000000081565b3480156103d9575f5ffd5b5061024c6103e8366004611301565b60026020525f908152604090205481565b348015610404575f5ffd5b5061024c610413366004611301565b60056020525f908152604090205481565b34801561042f575f5ffd5b5061024c61043e366004611301565b6106c3565b34801561044e575f5ffd5b506101a061045d3660046112d7565b6106df565b34801561046d575f5ffd5b5061022761047c3660046112d7565b6106fb565b34801561048c575f5ffd5b506101a061049b36600461131c565b610707565b3480156104ab575f5ffd5b5061024c6104ba36600461138d565b600360209081525f928352604080842090915290825290205481565b3480156104e1575f5ffd5b5061024c6104f0366004611301565b610908565b348015610500575f5ffd5b5061024c61050f366004611301565b610970565b5f816001600160a01b03166321f8a72160405160200161054b9060208082526003908201526215d39560ea1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161057f91815260200190565b602060405180830381865afa15801561059a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105be91906113c4565b92915050565b6106146040516020016105d6906113df565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610a0d565b61061f838383610abb565b505050565b5f610630338484610b03565b50600192915050565b6001600160a01b0383165f90815260036020908152604080832033808552925282205461067291869161066d908690611417565b610b03565b61067d848484610b64565b5060019392505050565b5f61069a6040516020016105d6906113df565b6105be82610c07565b6106b56040516020016105d6906113df565b6106bf8282610caf565b5050565b5f6106d66040516020016105d6906113df565b6105be82610d3d565b6106f16040516020016105d6906113df565b6106bf8282610ddd565b5f610630338484610b64565b428410156107455760405162461bcd60e51b815260206004820152600b60248201526a15518e881156141254915160aa1b6044820152606401610197565b6004546001600160a01b0388165f90815260056020526040812080549192917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9918b918b918b9190876107978361142a565b909155506040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810187905260e0016040516020818303038152906040528051906020012060405160200161081092919061190160f01b81526002810192909252602282015260420190565b60408051601f1981840301815282825280516020918201205f80855291840180845281905260ff88169284019290925260608301869052608083018590529092509060019060a0016020604051602081039080840390855afa158015610878573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116158015906108ae5750886001600160a01b0316816001600160a01b0316145b6108f25760405162461bcd60e51b815260206004820152601560248201527455463a20494e56414c49445f5349474e415455524560581b6044820152606401610197565b6108fd898989610b03565b505050505050505050565b6040516370a0823160e01b81523060048201525f906001600160a01b038316906370a0823190602401602060405180830381865afa15801561094c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105be9190611442565b5f6109836040516020016105d6906113df565b6040516370a0823160e01b81523060048201525f906001600160a01b038416906370a0823190602401602060405180830381865afa1580156109c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109eb9190611442565b6001600160a01b0384165f908152602081905260409020819055915050919050565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610a76573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9a9190611459565b6106bf57338160405163a35b150b60e01b8152600401610197929190611478565b306001600160a01b03831603610aef57604051637387c8a960e11b81526001600160a01b0383166004820152602401610197565b610afa838383610e64565b61061f83610f10565b6001600160a01b038381165f8181526003602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92591015b60405180910390a3505050565b6001600160a01b0383165f90815260026020526040902054610b869082610f91565b6001600160a01b038085165f908152600260205260408082209390935590841681522054610bb49082610fe6565b6001600160a01b038084165f8181526002602052604090819020939093559151908516907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90610b579085815260200190565b6001600160a01b0381165f818152602081905260408082205490516370a0823160e01b8152306004820152919290918391906370a0823190602401602060405180830381865afa158015610c5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c819190611442565b6001600160a01b0385165f9081526020819052604090208190559050610ca78282611417565b949350505050565b600154610cbc9082610fe6565b6001556001600160a01b0382165f90815260026020526040902054610ce19082610fe6565b6001600160a01b0383165f818152600260205260408082209390935591519091907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90610d319085815260200190565b60405180910390a35050565b6001600160a01b0381165f818152602081905260408082205490516370a0823160e01b8152306004820152919290918391906370a0823190602401602060405180830381865afa158015610d93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610db79190611442565b6001600160a01b0385165f9081526020819052604090208190559050610ca78183611417565b6001600160a01b0382165f90815260026020526040902054610dff9082610f91565b6001600160a01b0383165f90815260026020526040902055600154610e249082610f91565b6001556040518181525f906001600160a01b038416907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602001610d31565b805f03610e7057505050565b610e798261103a565b5f5f610e86858585611064565b915091508115610e97575050505050565b5f610ea1826111b3565b5090507fc9f14d9a0a9b46470c7c0b6c508f8283abaab7f795f153953c58cd4250824dae8183604051610ed592919061149b565b60405180910390a160405163012f3b8f60e71b81526001600160a01b0380881660048301528616602482015260448101859052606401610197565b6040516370a0823160e01b81523060048201526001600160a01b038216906370a0823190602401602060405180830381865afa158015610f52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f769190611442565b6001600160a01b039091165f90815260208190526040902055565b5f82610f9d8382611417565b91508111156105be5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610197565b5f82610ff283826114c8565b91508110156105be5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610197565b6001600160a01b0381166110615760405163d551823d60e01b815260040160405180910390fd5b50565b604080516001600160a01b038481166024830152604480830185905283518084039091018152606490920183526020820180516001600160e01b031663a9059cbb60e01b17905291515f926060929184918291908916906110c69085906114db565b5f604051808303815f865af19150503d805f81146110ff576040519150601f19603f3d011682016040523d82523d5f602084013e611104565b606091505b509150915081156111a35780515f03611162575f886001600160a01b03163b11611162575f6040518060400160405280601481526020017310d85b1b081d1bc81b9bdb8b58dbdb9d1c9858dd60621b815250945094505050506111ab565b5f81511180156111835750808060200190518101906111819190611459565b155b15611195575f945092506111ab915050565b6001945092506111ab915050565b5f9450925050505b935093915050565b60605f6044835110156111d857505060408051602081019091525f8082529092909150565b5f6111e4846020015190565b90506307b9e43360e51b6001600160e01b031982160161122357600484019350838060200190518101906112189190611505565b946001945092505050565b5f60405180602001604052805f815250909250925050915091565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61127e602083018461123e565b9392505050565b6001600160a01b0381168114611061575f5ffd5b5f5f5f606084860312156112ab575f5ffd5b83356112b681611285565b925060208401356112c681611285565b929592945050506040919091013590565b5f5f604083850312156112e8575f5ffd5b82356112f381611285565b946020939093013593505050565b5f60208284031215611311575f5ffd5b813561127e81611285565b5f5f5f5f5f5f5f60e0888a031215611332575f5ffd5b873561133d81611285565b9650602088013561134d81611285565b95506040880135945060608801359350608088013560ff81168114611370575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f6040838503121561139e575f5ffd5b82356113a981611285565b915060208301356113b981611285565b809150509250929050565b5f602082840312156113d4575f5ffd5b815161127e81611285565b6020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b634e487b7160e01b5f52601160045260245ffd5b818103818111156105be576105be611403565b5f6001820161143b5761143b611403565b5060010190565b5f60208284031215611452575f5ffd5b5051919050565b5f60208284031215611469575f5ffd5b8151801515811461127e575f5ffd5b6001600160a01b03831681526040602082018190525f90610ca79083018461123e565b604081525f6114ad604083018561123e565b82810360208401526114bf818561123e565b95945050505050565b808201808211156105be576105be611403565b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215611515575f5ffd5b815167ffffffffffffffff81111561152b575f5ffd5b8201601f8101841361153b575f5ffd5b805167ffffffffffffffff811115611555576115556114f1565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611584576115846114f1565b60405281815282820160200186101561159b575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea2646970667358221220e46b81fa1d09356b59039071c790004715a23f6d9ea86b9484f39f439606f55164736f6c634300081c0033a2646970667358221220bfa503d53194fa66dfb8e2266731ce7f3065445f52c4e0ea2fd6553bcbaa483564736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaL\n8\x03\x80aL\n\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QaJ\xF1a\x01\x19_9_\x81\x81`\xDE\x01Ra\x05\xFF\x01R_\x81\x81`\x97\x01R\x81\x81a\x01\x15\x01R\x81\x81a\x01`\x01R\x81\x81a\x01\xB6\x01R\x81\x81a\x02/\x01R\x81\x81a\x03\xD9\x01R\x81\x81a\x05M\x01R\x81\x81a\x05~\x01Ra\x05\xB8\x01R_\x81\x81`S\x01Ra\x03\xB7\x01RaJ\xF1_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80cJJ{\x04\x14a\0NW\x80cf\r\rg\x14a\0\x92W\x80c\x82Q\xA6\x87\x14a\0\xB9W\x80c\x9F\xF7\x8C0\x14a\0\xD9W[__\xFD[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCCa\0\xC76`\x04a1)V[a\x01\0V[`@Qa\0\x89\x91\x90a1BV[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x08a0\x16V[a\x01\x10a0JV[a\x019\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06UV[\x80\x82R_\x03a\x01[W`@Qc\x06\x15&M`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x1FV[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90Ra\x01\xB1W`@Qc}BUg`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xE3V[`\x01`\x01`\xA0\x1B\x03\x16`@\x82\x01\x81\x90Ra\x02\x07W`@Qc\x08\xD1'\x0B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Qa\x02\"\x90a\x02\x1D` \x86\x01\x86a2\x0FV[a\x08!V[``\x82\x01\x81\x90Ra\x02T\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x08\xC8V[`\x80\x82\x01\x81\x90R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\xABW``\x81\x01Q`\x80\x82\x01Q` \x01Q`@Qc\x0B\x84\"\xC3`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0F\x91\x90a2*V[`\xFF\x16`\xC0\x82\x01Ra\x03$` \x84\x01\x84a2\x0FV[`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a2*V[`\xFF\x16`\xE0\x82\x01R`\xC0\x81\x01Q\x81Qa\x03\x9C\x91_a\x1A\xACV[\x80\x82R`\xE0\x82\x01Qa\x03\xB0\x91\x90`\x01a\x1A\xACV[\x81R`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x04\x02\x90a0\xB5V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x042W=__>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x80\x84\x01\x91\x90\x91R`@\x80Qa\x01\xC0\x81\x01\x82R\x81\x85\x01Q\x90\x93\x16`\xE0\x80\x85\x01\x91\x82Rk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x01\0\x86\x01R_a\x01 \x86\x01\x81\x90Ra\x01@\x86\x01\x81\x90Ra\x01`\x86\x01\x81\x90Ra\x01\x80\x86\x01\x81\x90Ra\x01\xA0\x86\x01R\x92\x84\x01\x90\x81R\x81Q\x92\x83\x01\x90\x91R\x82\x91`\xC0\x83\x01\x90\x80a\x04\xBE` \x8A\x01\x8Aa2\x0FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x81RP\x81R` \x01\x82`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82_\x01Q\x81R` \x01a\x058B\x90V[\x90Ra\x01\0\x82\x01\x81\x90R``\x82\x01Qa\x05s\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a\x1B\tV[``\x81\x01Qa\x05\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x05\xAE`@\x87\x01` \x88\x01a2\x0FV[a-\x07V[a\x05\xE8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82``\x01Qa\x05\xE3B\x90V[a.\x04V[a\x01\0\x81\x01QQ\x80QQ` \x91\x82\x01QQa\x06J\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90a\x061\x90`@\x89\x01\x90\x89\x01a2\x0FV[\x85a\x01\0\x01Q`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Qa.\xD0V[a\x01\0\x01Q\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x06\xA6\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x19\x91\x90a2JV[\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x07p\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x19\x91\x90a2aV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x07p\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08BW\x81\x83a\x08EV[\x82\x82[`@Q\x91\x94P\x92Pa\x08r\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x08\xD0a0\x16V[\x82a\x08\xD9a0\x16V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x08\xF7\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\to\x91\x90a2\x9FV[a\t|W\x91Pa\x07\x19\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xBC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a2aV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BP\x91\x90a2JV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xA6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\xD6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CI\x91\x90a2JV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xA4\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rG\x91\x90a2JV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\rp\x90` \x01a2\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x13\x91\x90a2JV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0Eo\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x12\x91\x90a2JV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x02\x91\x90a2JV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10w\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xAB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xEA\x91\x90a2aV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x94\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD3\x91\x90a2JV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12*\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12Z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x8E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCD\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13)\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13Y\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCC\x91\x90a2JV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xFC\x90a2\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xFC\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9F\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xF8\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x9B\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xE9\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17M\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8C\x91\x90a2aV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xFA\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x9D\x91\x90a2aV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\0\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x190\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19d\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA3\x91\x90a2JV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x19\xFC\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x9F\x91\x90a2JV[`\x80\x82\x01R\x94\x93PPPPV[_`\xFF\x83\x11\x15a\x1A\xD9W`@Qc\xC3\xCA\x0E7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xFF`$\x82\x01R`D\x01a\x02\xA2V[`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a\x1A\xF8WP`\xFF``\x1B\x19\x90P``[\x90\x85\x16\x90\x84\x90\x1B\x17\x90P\x93\x92PPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1B+\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B{W__\xFD[PZ\xF1\x15\x80\x15a\x1B\x8DW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1B\xD1\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\x01\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CdW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x88\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xD0\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x81\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\xC8\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1ETW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ex\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\xC4\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FQW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fu\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x95\x90a2\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a F\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \x93\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a! W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!D\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\x8D\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\">\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\"\x7F\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xAF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#9\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x81\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$4\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a${\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%.\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%z\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&.\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&N\x90a2\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x02\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'O\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x03\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(L\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\0\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a)>\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xB8\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xF8\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a*J\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra*\xC5\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x05\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+L\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xFD\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,O\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\0\x91\x90a2JV[PPPPPV[_a-\x12\x84\x84a/TV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a-S\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\0\x91\x90a2aV[_a.\x0F\x84\x84a/TV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.b\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01a,\xC0V[`@Qc*\xD6B]`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R`d\x82\x01\x85\x90R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90cU\xAC\x84\xBA\x90`\xC4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/5W__\xFD[PZ\xF1\x15\x80\x15a/GW=__>=_\xFD[PPPPPPPPPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a/w\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xEF\x91\x90a2\x9FV[a0\x0FW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\xA2V[\x93\x92PPPV[`@Q\x80`\xA0\x01`@R\x80a0)a0\xC2V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01a0\x88a0\x16V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01a0\xB0a0\x16V[\x90R\x90V[a\x17{\x80a3A\x839\x01\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a1\x13`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\xD1W\x90PP\x90V[_`@\x82\x84\x03\x12\x80\x15a1:W__\xFD[P\x90\x92\x91PPV[\x81Qa\x02@\x82\x01\x90\x82_[`\x02\x81\x10\x15a1\xB8W\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pa1MV[PPP` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x84\x01R`@\x84\x01Q\x16a\x01\xE0\x83\x01R``\x83\x01Qa\x02\0\x83\x01R`\x80\x90\x92\x01Qa\x02 \x90\x91\x01R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2\x0CW__\xFD[PV[_` \x82\x84\x03\x12\x15a2\x1FW__\xFD[\x815a0\x0F\x81a1\xF8V[_` \x82\x84\x03\x12\x15a2:W__\xFD[\x81Q`\xFF\x81\x16\x81\x14a0\x0FW__\xFD[_` \x82\x84\x03\x12\x15a2ZW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a2qW__\xFD[\x81Qa0\x0F\x81a1\xF8V[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a2\xAFW__\xFD[\x81Q\x80\x15\x15\x81\x14a0\x0FW__\xFD[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V\xFE`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x17{8\x03\x80a\x17{\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01&V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80\x90\x81R\x91\x16`\xA0\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x02\x81RaMM`\xF0\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\x13\xB9\x13\xC1\xF3\x17\xDDq\xB6\xDF\xE5:\xBE\x93\x80\xC6R\xF9I\x8C\x8D\xE2[k9\xD8&4\x99\xAE{\xB6\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF\x94\x81\x01\x94\x90\x94R0\x84\x84\x01R\x81Q\x80\x85\x03\x90\x93\x01\x83R`\xC0\x90\x93\x01\x90R\x80Q\x91\x01 `\x04Ua\x01^V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01#W__\xFD[PV[__`@\x83\x85\x03\x12\x15a\x017W__\xFD[\x82Qa\x01B\x81a\x01\x0FV[` \x84\x01Q\x90\x92Pa\x01S\x81a\x01\x0FV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x15\xEEa\x01\x8D_9_\x81\x81a\x01J\x01Ra\x03\xAC\x01R_\x81\x81a\x036\x01Ra\n)\x01Ra\x15\xEE_\xF3\xFE`\x80`@R`\x046\x10a\x01?W_5`\xE0\x1C\x80cR?\xBA\x7F\x11a\0\xB3W\x80c\x9D\xC2\x9F\xAC\x11a\0mW\x80c\x9D\xC2\x9F\xAC\x14a\x04CW\x80c\xA9\x05\x9C\xBB\x14a\x04bW\x80c\xD5\x05\xAC\xCF\x14a\x04\x81W\x80c\xDDb\xED>\x14a\x04\xA0W\x80c\xE4,\x08\xF2\x14a\x04\xD6W\x80c\xEB@\x13?\x14a\x04\xF5W__\xFD[\x80cR?\xBA\x7F\x14a\x03pW\x80cf\r\rg\x14a\x03\x9BW\x80cp\xA0\x821\x14a\x03\xCEW\x80c~\xCE\xBE\0\x14a\x03\xF9W\x80c\x8C\x1B_\xDE\x14a\x04$W\x80c\x95\xD8\x9BA\x14a\x01\xA6W__\xFD[\x80c0\xAD\xF8\x1F\x11a\x01\x04W\x80c0\xAD\xF8\x1F\x14a\x02yW\x80c1<\xE5g\x14a\x02\xACW\x80c5/\x9A\xED\x14a\x02\xD2W\x80c6D\xE5\x15\x14a\x02\xF1W\x80c@\xC1\x0F\x19\x14a\x03\x06W\x80cJJ{\x04\x14a\x03%W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\xA6W\x80c\x07\x8D;y\x14a\x01\xE9W\x80c\t^\xA7\xB3\x14a\x02\x08W\x80c\x18\x16\r\xDD\x14a\x027W\x80c#\xB8r\xDD\x14a\x02ZW__\xFD[6a\x01\xA2W_a\x01n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x14V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x01\xA0W`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[__\xFD[4\x80\x15a\x01\xB1W__\xFD[Pa\x01\xD3`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMM`\xF0\x1B\x81RP\x81V[`@Qa\x01\xE0\x91\x90a\x12lV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF4W__\xFD[Pa\x01\xA0a\x02\x036`\x04a\x12\x99V[a\x05\xC4V[4\x80\x15a\x02\x13W__\xFD[Pa\x02'a\x02\"6`\x04a\x12\xD7V[a\x06$V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE0V[4\x80\x15a\x02BW__\xFD[Pa\x02L`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[4\x80\x15a\x02eW__\xFD[Pa\x02'a\x02t6`\x04a\x12\x99V[a\x069V[4\x80\x15a\x02\x84W__\xFD[Pa\x02L\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[4\x80\x15a\x02\xB7W__\xFD[Pa\x02\xC0`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xE0V[4\x80\x15a\x02\xDDW__\xFD[Pa\x02La\x02\xEC6`\x04a\x13\x01V[a\x06\x87V[4\x80\x15a\x02\xFCW__\xFD[Pa\x02L`\x04T\x81V[4\x80\x15a\x03\x11W__\xFD[Pa\x01\xA0a\x03 6`\x04a\x12\xD7V[a\x06\xA3V[4\x80\x15a\x030W__\xFD[Pa\x03X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[4\x80\x15a\x03{W__\xFD[Pa\x02La\x03\x8A6`\x04a\x13\x01V[_` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xA6W__\xFD[Pa\x03X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xD9W__\xFD[Pa\x02La\x03\xE86`\x04a\x13\x01V[`\x02` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x04W__\xFD[Pa\x02La\x04\x136`\x04a\x13\x01V[`\x05` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04/W__\xFD[Pa\x02La\x04>6`\x04a\x13\x01V[a\x06\xC3V[4\x80\x15a\x04NW__\xFD[Pa\x01\xA0a\x04]6`\x04a\x12\xD7V[a\x06\xDFV[4\x80\x15a\x04mW__\xFD[Pa\x02'a\x04|6`\x04a\x12\xD7V[a\x06\xFBV[4\x80\x15a\x04\x8CW__\xFD[Pa\x01\xA0a\x04\x9B6`\x04a\x13\x1CV[a\x07\x07V[4\x80\x15a\x04\xABW__\xFD[Pa\x02La\x04\xBA6`\x04a\x13\x8DV[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04\xE1W__\xFD[Pa\x02La\x04\xF06`\x04a\x13\x01V[a\t\x08V[4\x80\x15a\x05\0W__\xFD[Pa\x02La\x05\x0F6`\x04a\x13\x01V[a\tpV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05K\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x7F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x13\xC4V[\x92\x91PPV[a\x06\x14`@Q` \x01a\x05\xD6\x90a\x13\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\n\rV[a\x06\x1F\x83\x83\x83a\n\xBBV[PPPV[_a\x0603\x84\x84a\x0B\x03V[P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x80\x85R\x92R\x82 Ta\x06r\x91\x86\x91a\x06m\x90\x86\x90a\x14\x17V[a\x0B\x03V[a\x06}\x84\x84\x84a\x0BdV[P`\x01\x93\x92PPPV[_a\x06\x9A`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x05\xBE\x82a\x0C\x07V[a\x06\xB5`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x06\xBF\x82\x82a\x0C\xAFV[PPV[_a\x06\xD6`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x05\xBE\x82a\r=V[a\x06\xF1`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x06\xBF\x82\x82a\r\xDDV[_a\x0603\x84\x84a\x0BdV[B\x84\x10\x15a\x07EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x15Q\x8E\x88\x11V\x14\x12T\x91Q`\xAA\x1B`D\x82\x01R`d\x01a\x01\x97V[`\x04T`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x05` R`@\x81 \x80T\x91\x92\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x91\x8B\x91\x8B\x91\x8B\x91\x90\x87a\x07\x97\x83a\x14*V[\x90\x91UP`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x87\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\x10\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 _\x80\x85R\x91\x84\x01\x80\x84R\x81\x90R`\xFF\x88\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x86\x90R`\x80\x83\x01\x85\x90R\x90\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08xW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08\xAEWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x08\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtUF: INVALID_SIGNATURE`X\x1B`D\x82\x01R`d\x01a\x01\x97V[a\x08\xFD\x89\x89\x89a\x0B\x03V[PPPPPPPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x14BV[_a\t\x83`@Q` \x01a\x05\xD6\x90a\x13\xDFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEB\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x91\x90PV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nvW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9A\x91\x90a\x14YV[a\x06\xBFW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\x97\x92\x91\x90a\x14xV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\n\xEFW`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x01\x97V[a\n\xFA\x83\x83\x83a\x0EdV[a\x06\x1F\x83a\x0F\x10V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x02` R`@\x90 Ta\x0B\x86\x90\x82a\x0F\x91V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x02` R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x0B\xB4\x90\x82a\x0F\xE6V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x93\x90\x93U\x91Q\x90\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x0BW\x90\x85\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x81\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x0C\xA7\x82\x82a\x14\x17V[\x94\x93PPPPV[`\x01Ta\x0C\xBC\x90\x82a\x0F\xE6V[`\x01U`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x02` R`@\x90 Ta\x0C\xE1\x90\x82a\x0F\xE6V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x81\x81R`\x02` R`@\x80\x82 \x93\x90\x93U\x91Q\x90\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\r1\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB7\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x0C\xA7\x81\x83a\x14\x17V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x02` R`@\x90 Ta\r\xFF\x90\x82a\x0F\x91V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x02` R`@\x90 U`\x01Ta\x0E$\x90\x82a\x0F\x91V[`\x01U`@Q\x81\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\r1V[\x80_\x03a\x0EpWPPPV[a\x0Ey\x82a\x10:V[__a\x0E\x86\x85\x85\x85a\x10dV[\x91P\x91P\x81\x15a\x0E\x97WPPPPPV[_a\x0E\xA1\x82a\x11\xB3V[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x0E\xD5\x92\x91\x90a\x14\x9BV[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\x01\x97V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fv\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R` \x81\x90R`@\x90 UV[_\x82a\x0F\x9D\x83\x82a\x14\x17V[\x91P\x81\x11\x15a\x05\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x01\x97V[_\x82a\x0F\xF2\x83\x82a\x14\xC8V[\x91P\x81\x10\x15a\x05\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x01\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10aW`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q_\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x10\xC6\x90\x85\x90a\x14\xDBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x10\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x11\x04V[``\x91P[P\x91P\x91P\x81\x15a\x11\xA3W\x80Q_\x03a\x11bW_\x88`\x01`\x01`\xA0\x1B\x03\x16;\x11a\x11bW_`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\x11\xABV[_\x81Q\x11\x80\x15a\x11\x83WP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\x81\x91\x90a\x14YV[\x15[\x15a\x11\x95W_\x94P\x92Pa\x11\xAB\x91PPV[`\x01\x94P\x92Pa\x11\xAB\x91PPV[_\x94P\x92PPP[\x93P\x93\x91PPV[``_`D\x83Q\x10\x15a\x11\xD8WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x90\x92\x90\x91PV[_a\x11\xE4\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x12#W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x12\x18\x91\x90a\x15\x05V[\x94`\x01\x94P\x92PPPV[_`@Q\x80` \x01`@R\x80_\x81RP\x90\x92P\x92PP\x91P\x91V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x12~` \x83\x01\x84a\x12>V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10aW__\xFD[___``\x84\x86\x03\x12\x15a\x12\xABW__\xFD[\x835a\x12\xB6\x81a\x12\x85V[\x92P` \x84\x015a\x12\xC6\x81a\x12\x85V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a\x12\xE8W__\xFD[\x825a\x12\xF3\x81a\x12\x85V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x13\x11W__\xFD[\x815a\x12~\x81a\x12\x85V[_______`\xE0\x88\x8A\x03\x12\x15a\x132W__\xFD[\x875a\x13=\x81a\x12\x85V[\x96P` \x88\x015a\x13M\x81a\x12\x85V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x13pW__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x13\x9EW__\xFD[\x825a\x13\xA9\x81a\x12\x85V[\x91P` \x83\x015a\x13\xB9\x81a\x12\x85V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x13\xD4W__\xFD[\x81Qa\x12~\x81a\x12\x85V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xBEWa\x05\xBEa\x14\x03V[_`\x01\x82\x01a\x14;Wa\x14;a\x14\x03V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x14RW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14iW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12~W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0C\xA7\x90\x83\x01\x84a\x12>V[`@\x81R_a\x14\xAD`@\x83\x01\x85a\x12>V[\x82\x81\x03` \x84\x01Ra\x14\xBF\x81\x85a\x12>V[\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xBEWa\x05\xBEa\x14\x03V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x15\x15W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15+W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x15;W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15UWa\x15Ua\x14\xF1V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\x84Wa\x15\x84a\x14\xF1V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x15\x9BW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xE4k\x81\xFA\x1D\t5kY\x03\x90q\xC7\x90\0G\x15\xA2?m\x9E\xA8k\x94\x84\xF3\x9FC\x96\x06\xF5QdsolcC\0\x08\x1C\x003\xA2dipfsX\"\x12 \xBF\xA5\x03\xD51\x94\xFAf\xDF\xB8\xE2&g1\xCE\x7F0eD_R\xC4\xE0\xEA/\xD6U;\xCB\xAAH5dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80634a4a7b041461004e578063660d0d67146100925780638251a687146100b95780639ff78c30146100d9575b5f5ffd5b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100cc6100c7366004613129565b610100565b6040516100899190613142565b6100757f000000000000000000000000000000000000000000000000000000000000000081565b610108613016565b61011061304a565b6101397f0000000000000000000000000000000000000000000000000000000000000000610655565b8082525f0361015b57604051630615264d60e21b815260040160405180910390fd5b6101847f000000000000000000000000000000000000000000000000000000000000000061071f565b6001600160a01b0316602082018190526101b157604051637d42556760e11b815260040160405180910390fd5b6101da7f00000000000000000000000000000000000000000000000000000000000000006107e3565b6001600160a01b031660408201819052610207576040516308d1270b60e21b815260040160405180910390fd5b60408101516102229061021d602086018661320f565b610821565b60608201819052610254907f0000000000000000000000000000000000000000000000000000000000000000906108c8565b60808201819052602001516001600160a01b0316156102ab576060810151608082015160200151604051630b8422c360e01b815260048101929092526001600160a01b031660248201526044015b60405180910390fd5b80604001516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061030f919061322a565b60ff1660c0820152610324602084018461320f565b6001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801561035f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610383919061322a565b60ff1660e082015260c0810151815161039c915f611aac565b80825260e08201516103b091906001611aac565b81526040517f0000000000000000000000000000000000000000000000000000000000000000907f000000000000000000000000000000000000000000000000000000000000000090610402906130b5565b6001600160a01b03928316815291166020820152604001604051809103905ff080158015610432573d5f5f3e3d5ffd5b506001600160a01b0390811660a080840191909152604080516101c0810182528185015190931660e08085019182526b033b2e3c9fd0803ce80000006101008601525f61012086018190526101408601819052610160860181905261018086018190526101a08601529284019081528151928301909152829160c0830190806104be60208a018a61320f565b6001600160a01b031681526020016b033b2e3c9fd0803ce800000081526020015f81526020015f81526020015f81526020015f81526020015f81525081525081526020018260a001516001600160a01b0316815260200182602001516001600160a01b03168152602001825f015181526020016105384290565b905261010082018190526060820151610573917f00000000000000000000000000000000000000000000000000000000000000009190611b09565b60608101516105b3907f0000000000000000000000000000000000000000000000000000000000000000906105ae604087016020880161320f565b612d07565b6105e87f000000000000000000000000000000000000000000000000000000000000000082606001516105e34290565b612e04565b610100810151518051516020918201515161064a927f0000000000000000000000000000000000000000000000000000000000000000929190610631906040890190890161320f565b856101000151608001518660c001518760e00151612ed0565b610100015192915050565b5f816001600160a01b031663bd02d0f56040516020016106a6906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016106da91815260200190565b602060405180830381865afa1580156106f5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610719919061324a565b92915050565b5f816001600160a01b03166321f8a721604051602001610770906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016107a491815260200190565b602060405180830381865afa1580156107bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107199190613261565b5f816001600160a01b03166321f8a721604051602001610770906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b5f816001600160a01b0316836001600160a01b031610610842578183610845565b82825b6040519194509250610872906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b6108d0613016565b826108d9613016565b816001600160a01b03166391d4403c6040516020016108f79061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561094b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061096f919061329f565b61097c5791506107199050565b816001600160a01b03166321f8a721856040516020016109bc906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b604051602081830303815290604052805190602001206040516020016109ec929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a2091815260200190565b602060405180830381865afa158015610a3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5f9190613261565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610add929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b1191815260200190565b602060405180830381865afa158015610b2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b50919061324a565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001610ba6906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001610bd6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c0a91815260200190565b602060405180830381865afa158015610c25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c49919061324a565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610ca4906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610cd4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d0891815260200190565b602060405180830381865afa158015610d23573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d47919061324a565b815151606001526040516001600160a01b0383169063bd02d0f5908690610d70906020016132be565b60405160208183030381529060405280519060200120604051602001610da0929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610dd491815260200190565b602060405180830381865afa158015610def573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e13919061324a565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610e6f906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001610e9f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610ed391815260200190565b602060405180830381865afa158015610eee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f12919061324a565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610f8f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610fc391815260200190565b602060405180830381865afa158015610fde573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611002919061324a565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001611077929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110ab91815260200190565b602060405180830381865afa1580156110c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ea9190613261565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161119491815260200190565b602060405180830381865afa1580156111af573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111d3919061324a565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161122a90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b6040516020818303038152906040528051906020012060405160200161125a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161128e91815260200190565b602060405180830381865afa1580156112a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112cd919061324a565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161132990602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611359929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161138d91815260200190565b602060405180830381865afa1580156113a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113cc919061324a565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016113fc906132ff565b6040516020818303038152906040528051906020012060405160200161142c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161146091815260200190565b602060405180830381865afa15801561147b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061149f919061324a565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016114fc90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161152c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161156091815260200190565b602060405180830381865afa15801561157b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159f919061324a565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016115f890602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001611628929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161165c91815260200190565b602060405180830381865afa158015611677573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061169b919061324a565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016116e990602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001611719929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161174d91815260200190565b602060405180830381865afa158015611768573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061178c9190613261565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016117fa906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b6040516020818303038152906040528051906020012060405160200161182a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161185e91815260200190565b602060405180830381865afa158015611879573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061189d9190613261565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001611900906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001611930929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161196491815260200190565b602060405180830381865afa15801561197f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a3919061324a565b60608201526040516001600160a01b0383169063bd02d0f59086906119fc906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001611a2c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401611a6091815260200190565b602060405180830381865afa158015611a7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a9f919061324a565b6080820152949350505050565b5f60ff831115611ad95760405163c3ca0e3760e01b81526004810184905260ff60248201526044016102a2565b60ff60581b1960585f1960ff851601611af8575060ff60601b19905060605b9085169084901b1790509392505050565b5f839050806001600160a01b031663c80f4c62604051602001611b2b9061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611b7b575f5ffd5b505af1158015611b8d573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611bd1906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611c01929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611c64573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c889190613261565b50806001600160a01b031663e2a4853a84604051602001611cd0906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611d00929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611d5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d81919061324a565b50806001600160a01b031663e2a4853a84604051602001611dc8906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001611df8929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611e54573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e78919061324a565b50806001600160a01b031663e2a4853a84604051602001611ec4906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ef4929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611f51573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f75919061324a565b50806001600160a01b031663e2a4853a84604051602001611f95906132be565b60405160208183030381529060405280519060200120604051602001611fc5929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612022573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612046919061324a565b50806001600160a01b031663e2a4853a84604051602001612093906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b604051602081830303815290604052805190602001206040516020016120c3929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612120573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612144919061324a565b50806001600160a01b031663e2a4853a8460405160200161218d906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b604051602081830303815290604052805190602001206040516020016121bd929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561221a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061223e919061324a565b50806001600160a01b031663ca446dd98460405160200161227f906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b604051602081830303815290604052805190602001206040516020016122af929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015612315573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123399190613261565b50806001600160a01b031663e2a4853a8460405160200161238190602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b604051602081830303815290604052805190602001206040516020016123b1929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612410573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612434919061324a565b50806001600160a01b031663e2a4853a8460405160200161247b90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016124ab929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561250a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061252e919061324a565b50806001600160a01b031663e2a4853a8460405160200161257a90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016125aa929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561260a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061262e919061324a565b50806001600160a01b031663e2a4853a8460405160200161264e906132ff565b6040516020818303038152906040528051906020012060405160200161267e929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156126de573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612702919061324a565b50806001600160a01b031663e2a4853a8460405160200161274f90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b6040516020818303038152906040528051906020012060405160200161277f929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156127df573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612803919061324a565b50806001600160a01b031663e2a4853a8460405160200161284c90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b6040516020818303038152906040528051906020012060405160200161287c929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128dc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612900919061324a565b50806001600160a01b031663ca446dd98460405160200161293e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b6040516020818303038152906040528051906020012060405160200161296e929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b81526004016129b89291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af11580156129d4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129f89190613261565b50806001600160a01b031663ca446dd984604051602001612a4a906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612a7a929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612ac5926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612ae1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b059190613261565b50806001600160a01b031663e2a4853a84604051602001612b4c906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b7c929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612bbd929190918252602082015260400190565b6020604051808303815f875af1158015612bd9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bfd919061324a565b50806001600160a01b031663e2a4853a84604051602001612c4f906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612c7f929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401612cc0929190918252602082015260400190565b6020604051808303815f875af1158015612cdc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d00919061324a565b5050505050565b5f612d128484612f54565b9050806001600160a01b031663ca446dd984604051602001612d53906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612d83929190918252602082015260400190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201526001600160a01b03851660248201526044016020604051808303815f875af1158015612de0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d009190613261565b5f612e0f8484612f54565b9050806001600160a01b031663e2a4853a84604051602001612e629060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612e92929190918252602082015260400190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101859052604401612cc0565b604051632ad6425d60e11b81526001600160a01b03878116600483015286811660248301528581166044830152606482018590526084820184905260a482018390528816906355ac84ba9060c4015f604051808303815f87803b158015612f35575f5ffd5b505af1158015612f47573d5f5f3e3d5ffd5b5050505050505050505050565b5f5f839050806001600160a01b03166391d4403c604051602001612f779061327c565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa158015612fcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fef919061329f565b61300f57604051637357d91f60e01b8152600481018490526024016102a2565b9392505050565b6040518060a001604052806130296130c2565b81525f60208201819052604082018190526060820181905260809091015290565b6040518061012001604052805f81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001613088613016565b81526020015f6001600160a01b031681526020015f81526020015f81526020016130b0613016565b905290565b61177b8061334183390190565b60405180604001604052806002905b6131136040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816130d15790505090565b5f604082840312801561313a575f5ffd5b509092915050565b8151610240820190825f5b60028110156131b857825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e08201915060208301925060018101905061314d565b50505060208301516001600160a01b039081166101c08401526040840151166101e083015260608301516102008301526080909201516102209091015290565b6001600160a01b038116811461320c575f5ffd5b50565b5f6020828403121561321f575f5ffd5b813561300f816131f8565b5f6020828403121561323a575f5ffd5b815160ff8116811461300f575f5ffd5b5f6020828403121561325a575f5ffd5b5051919050565b5f60208284031215613271575f5ffd5b815161300f816131f8565b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f602082840312156132af575f5ffd5b8151801515811461300f575f5ffd5b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b60608201526080019056fe60c060405234801561000f575f5ffd5b5060405161177b38038061177b83398101604081905261002e91610126565b6001600160a01b039182166080908152911660a090815260408051808201825260028152614d4d60f01b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527f13b913c1f317dd71b6dfe53abe9380c652f9498c8de25b6b39d8263499ae7bb6818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc66060820152469481019490945230848401528151808503909301835260c09093019052805191012060045561015e565b6001600160a01b0381168114610123575f5ffd5b50565b5f5f60408385031215610137575f5ffd5b82516101428161010f565b60208401519092506101538161010f565b809150509250929050565b60805160a0516115ee61018d5f395f818161014a01526103ac01525f81816103360152610a2901526115ee5ff3fe60806040526004361061013f575f3560e01c8063523fba7f116100b35780639dc29fac1161006d5780639dc29fac14610443578063a9059cbb14610462578063d505accf14610481578063dd62ed3e146104a0578063e42c08f2146104d6578063eb40133f146104f5575f5ffd5b8063523fba7f14610370578063660d0d671461039b57806370a08231146103ce5780637ecebe00146103f95780638c1b5fde1461042457806395d89b41146101a6575f5ffd5b806330adf81f1161010457806330adf81f14610279578063313ce567146102ac578063352f9aed146102d25780633644e515146102f157806340c10f19146103065780634a4a7b0414610325575f5ffd5b806306fdde03146101a6578063078d3b79146101e9578063095ea7b31461020857806318160ddd1461023757806323b872dd1461025a575f5ffd5b366101a2575f61016e7f0000000000000000000000000000000000000000000000000000000000000000610514565b9050336001600160a01b038216146101a05760405163738d28df60e11b81523360048201526024015b60405180910390fd5b005b5f5ffd5b3480156101b1575f5ffd5b506101d3604051806040016040528060028152602001614d4d60f01b81525081565b6040516101e0919061126c565b60405180910390f35b3480156101f4575f5ffd5b506101a0610203366004611299565b6105c4565b348015610213575f5ffd5b506102276102223660046112d7565b610624565b60405190151581526020016101e0565b348015610242575f5ffd5b5061024c60015481565b6040519081526020016101e0565b348015610265575f5ffd5b50610227610274366004611299565b610639565b348015610284575f5ffd5b5061024c7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c981565b3480156102b7575f5ffd5b506102c0601281565b60405160ff90911681526020016101e0565b3480156102dd575f5ffd5b5061024c6102ec366004611301565b610687565b3480156102fc575f5ffd5b5061024c60045481565b348015610311575f5ffd5b506101a06103203660046112d7565b6106a3565b348015610330575f5ffd5b506103587f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101e0565b34801561037b575f5ffd5b5061024c61038a366004611301565b5f6020819052908152604090205481565b3480156103a6575f5ffd5b506103587f000000000000000000000000000000000000000000000000000000000000000081565b3480156103d9575f5ffd5b5061024c6103e8366004611301565b60026020525f908152604090205481565b348015610404575f5ffd5b5061024c610413366004611301565b60056020525f908152604090205481565b34801561042f575f5ffd5b5061024c61043e366004611301565b6106c3565b34801561044e575f5ffd5b506101a061045d3660046112d7565b6106df565b34801561046d575f5ffd5b5061022761047c3660046112d7565b6106fb565b34801561048c575f5ffd5b506101a061049b36600461131c565b610707565b3480156104ab575f5ffd5b5061024c6104ba36600461138d565b600360209081525f928352604080842090915290825290205481565b3480156104e1575f5ffd5b5061024c6104f0366004611301565b610908565b348015610500575f5ffd5b5061024c61050f366004611301565b610970565b5f816001600160a01b03166321f8a72160405160200161054b9060208082526003908201526215d39560ea1b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161057f91815260200190565b602060405180830381865afa15801561059a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105be91906113c4565b92915050565b6106146040516020016105d6906113df565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b815250610a0d565b61061f838383610abb565b505050565b5f610630338484610b03565b50600192915050565b6001600160a01b0383165f90815260036020908152604080832033808552925282205461067291869161066d908690611417565b610b03565b61067d848484610b64565b5060019392505050565b5f61069a6040516020016105d6906113df565b6105be82610c07565b6106b56040516020016105d6906113df565b6106bf8282610caf565b5050565b5f6106d66040516020016105d6906113df565b6105be82610d3d565b6106f16040516020016105d6906113df565b6106bf8282610ddd565b5f610630338484610b64565b428410156107455760405162461bcd60e51b815260206004820152600b60248201526a15518e881156141254915160aa1b6044820152606401610197565b6004546001600160a01b0388165f90815260056020526040812080549192917f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9918b918b918b9190876107978361142a565b909155506040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810187905260e0016040516020818303038152906040528051906020012060405160200161081092919061190160f01b81526002810192909252602282015260420190565b60408051601f1981840301815282825280516020918201205f80855291840180845281905260ff88169284019290925260608301869052608083018590529092509060019060a0016020604051602081039080840390855afa158015610878573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116158015906108ae5750886001600160a01b0316816001600160a01b0316145b6108f25760405162461bcd60e51b815260206004820152601560248201527455463a20494e56414c49445f5349474e415455524560581b6044820152606401610197565b6108fd898989610b03565b505050505050505050565b6040516370a0823160e01b81523060048201525f906001600160a01b038316906370a0823190602401602060405180830381865afa15801561094c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105be9190611442565b5f6109836040516020016105d6906113df565b6040516370a0823160e01b81523060048201525f906001600160a01b038416906370a0823190602401602060405180830381865afa1580156109c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109eb9190611442565b6001600160a01b0384165f908152602081905260409020819055915050919050565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610a76573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9a9190611459565b6106bf57338160405163a35b150b60e01b8152600401610197929190611478565b306001600160a01b03831603610aef57604051637387c8a960e11b81526001600160a01b0383166004820152602401610197565b610afa838383610e64565b61061f83610f10565b6001600160a01b038381165f8181526003602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92591015b60405180910390a3505050565b6001600160a01b0383165f90815260026020526040902054610b869082610f91565b6001600160a01b038085165f908152600260205260408082209390935590841681522054610bb49082610fe6565b6001600160a01b038084165f8181526002602052604090819020939093559151908516907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90610b579085815260200190565b6001600160a01b0381165f818152602081905260408082205490516370a0823160e01b8152306004820152919290918391906370a0823190602401602060405180830381865afa158015610c5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c819190611442565b6001600160a01b0385165f9081526020819052604090208190559050610ca78282611417565b949350505050565b600154610cbc9082610fe6565b6001556001600160a01b0382165f90815260026020526040902054610ce19082610fe6565b6001600160a01b0383165f818152600260205260408082209390935591519091907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90610d319085815260200190565b60405180910390a35050565b6001600160a01b0381165f818152602081905260408082205490516370a0823160e01b8152306004820152919290918391906370a0823190602401602060405180830381865afa158015610d93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610db79190611442565b6001600160a01b0385165f9081526020819052604090208190559050610ca78183611417565b6001600160a01b0382165f90815260026020526040902054610dff9082610f91565b6001600160a01b0383165f90815260026020526040902055600154610e249082610f91565b6001556040518181525f906001600160a01b038416907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90602001610d31565b805f03610e7057505050565b610e798261103a565b5f5f610e86858585611064565b915091508115610e97575050505050565b5f610ea1826111b3565b5090507fc9f14d9a0a9b46470c7c0b6c508f8283abaab7f795f153953c58cd4250824dae8183604051610ed592919061149b565b60405180910390a160405163012f3b8f60e71b81526001600160a01b0380881660048301528616602482015260448101859052606401610197565b6040516370a0823160e01b81523060048201526001600160a01b038216906370a0823190602401602060405180830381865afa158015610f52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f769190611442565b6001600160a01b039091165f90815260208190526040902055565b5f82610f9d8382611417565b91508111156105be5760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610197565b5f82610ff283826114c8565b91508110156105be5760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b6044820152606401610197565b6001600160a01b0381166110615760405163d551823d60e01b815260040160405180910390fd5b50565b604080516001600160a01b038481166024830152604480830185905283518084039091018152606490920183526020820180516001600160e01b031663a9059cbb60e01b17905291515f926060929184918291908916906110c69085906114db565b5f604051808303815f865af19150503d805f81146110ff576040519150601f19603f3d011682016040523d82523d5f602084013e611104565b606091505b509150915081156111a35780515f03611162575f886001600160a01b03163b11611162575f6040518060400160405280601481526020017310d85b1b081d1bc81b9bdb8b58dbdb9d1c9858dd60621b815250945094505050506111ab565b5f81511180156111835750808060200190518101906111819190611459565b155b15611195575f945092506111ab915050565b6001945092506111ab915050565b5f9450925050505b935093915050565b60605f6044835110156111d857505060408051602081019091525f8082529092909150565b5f6111e4846020015190565b90506307b9e43360e51b6001600160e01b031982160161122357600484019350838060200190518101906112189190611505565b946001945092505050565b5f60405180602001604052805f815250909250925050915091565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61127e602083018461123e565b9392505050565b6001600160a01b0381168114611061575f5ffd5b5f5f5f606084860312156112ab575f5ffd5b83356112b681611285565b925060208401356112c681611285565b929592945050506040919091013590565b5f5f604083850312156112e8575f5ffd5b82356112f381611285565b946020939093013593505050565b5f60208284031215611311575f5ffd5b813561127e81611285565b5f5f5f5f5f5f5f60e0888a031215611332575f5ffd5b873561133d81611285565b9650602088013561134d81611285565b95506040880135945060608801359350608088013560ff81168114611370575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f6040838503121561139e575f5ffd5b82356113a981611285565b915060208301356113b981611285565b809150509250929050565b5f602082840312156113d4575f5ffd5b815161127e81611285565b6020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b634e487b7160e01b5f52601160045260245ffd5b818103818111156105be576105be611403565b5f6001820161143b5761143b611403565b5060010190565b5f60208284031215611452575f5ffd5b5051919050565b5f60208284031215611469575f5ffd5b8151801515811461127e575f5ffd5b6001600160a01b03831681526040602082018190525f90610ca79083018461123e565b604081525f6114ad604083018561123e565b82810360208401526114bf818561123e565b95945050505050565b808201808211156105be576105be611403565b5f82518060208501845e5f920191825250919050565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215611515575f5ffd5b815167ffffffffffffffff81111561152b575f5ffd5b8201601f8101841361153b575f5ffd5b805167ffffffffffffffff811115611555576115556114f1565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611584576115846114f1565b60405281815282820160200186101561159b575f5ffd5b8160208401602083015e5f9181016020019190915294935050505056fea2646970667358221220e46b81fa1d09356b59039071c790004715a23f6d9ea86b9484f39f439606f55164736f6c634300081c0033a2646970667358221220bfa503d53194fa66dfb8e2266731ce7f3065445f52c4e0ea2fd6553bcbaa483564736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80cJJ{\x04\x14a\0NW\x80cf\r\rg\x14a\0\x92W\x80c\x82Q\xA6\x87\x14a\0\xB9W\x80c\x9F\xF7\x8C0\x14a\0\xD9W[__\xFD[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCCa\0\xC76`\x04a1)V[a\x01\0V[`@Qa\0\x89\x91\x90a1BV[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x08a0\x16V[a\x01\x10a0JV[a\x019\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06UV[\x80\x82R_\x03a\x01[W`@Qc\x06\x15&M`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x1FV[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01\x81\x90Ra\x01\xB1W`@Qc}BUg`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01\xDA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\xE3V[`\x01`\x01`\xA0\x1B\x03\x16`@\x82\x01\x81\x90Ra\x02\x07W`@Qc\x08\xD1'\x0B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Qa\x02\"\x90a\x02\x1D` \x86\x01\x86a2\x0FV[a\x08!V[``\x82\x01\x81\x90Ra\x02T\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x08\xC8V[`\x80\x82\x01\x81\x90R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02\xABW``\x81\x01Q`\x80\x82\x01Q` \x01Q`@Qc\x0B\x84\"\xC3`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0F\x91\x90a2*V[`\xFF\x16`\xC0\x82\x01Ra\x03$` \x84\x01\x84a2\x0FV[`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x83\x91\x90a2*V[`\xFF\x16`\xE0\x82\x01R`\xC0\x81\x01Q\x81Qa\x03\x9C\x91_a\x1A\xACV[\x80\x82R`\xE0\x82\x01Qa\x03\xB0\x91\x90`\x01a\x1A\xACV[\x81R`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x04\x02\x90a0\xB5V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x042W=__>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x80\x84\x01\x91\x90\x91R`@\x80Qa\x01\xC0\x81\x01\x82R\x81\x85\x01Q\x90\x93\x16`\xE0\x80\x85\x01\x91\x82Rk\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x01\0\x86\x01R_a\x01 \x86\x01\x81\x90Ra\x01@\x86\x01\x81\x90Ra\x01`\x86\x01\x81\x90Ra\x01\x80\x86\x01\x81\x90Ra\x01\xA0\x86\x01R\x92\x84\x01\x90\x81R\x81Q\x92\x83\x01\x90\x91R\x82\x91`\xC0\x83\x01\x90\x80a\x04\xBE` \x8A\x01\x8Aa2\x0FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x81RP\x81R` \x01\x82`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82_\x01Q\x81R` \x01a\x058B\x90V[\x90Ra\x01\0\x82\x01\x81\x90R``\x82\x01Qa\x05s\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a\x1B\tV[``\x81\x01Qa\x05\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x05\xAE`@\x87\x01` \x88\x01a2\x0FV[a-\x07V[a\x05\xE8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82``\x01Qa\x05\xE3B\x90V[a.\x04V[a\x01\0\x81\x01QQ\x80QQ` \x91\x82\x01QQa\x06J\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90a\x061\x90`@\x89\x01\x90\x89\x01a2\x0FV[\x85a\x01\0\x01Q`\x80\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Qa.\xD0V[a\x01\0\x01Q\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x06\xA6\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xDA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x19\x91\x90a2JV[\x92\x91PPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x07p\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xA4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x19\x91\x90a2aV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x07p\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08BW\x81\x83a\x08EV[\x82\x82[`@Q\x91\x94P\x92Pa\x08r\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[a\x08\xD0a0\x16V[\x82a\x08\xD9a0\x16V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x08\xF7\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\to\x91\x90a2\x9FV[a\t|W\x91Pa\x07\x19\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\t\xBC\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n \x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a2aV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BP\x91\x90a2JV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xA6\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\xD6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CI\x91\x90a2JV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xA4\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rG\x91\x90a2JV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\rp\x90` \x01a2\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xA0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x13\x91\x90a2JV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0Eo\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xD3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x12\x91\x90a2JV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F\x8F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\xC3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x02\x91\x90a2JV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10w\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xAB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xEA\x91\x90a2aV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x94\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xAFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD3\x91\x90a2JV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12*\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12Z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x8E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCD\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13)\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13Y\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCC\x91\x90a2JV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xFC\x90a2\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xFC\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9F\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x15\xF8\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16(\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\\\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16wW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x9B\x91\x90a2JV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xE9\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17M\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8C\x91\x90a2aV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x17\xFA\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x9D\x91\x90a2aV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x19\0\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x190\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19d\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA3\x91\x90a2JV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x19\xFC\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A,\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A`\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x9F\x91\x90a2JV[`\x80\x82\x01R\x94\x93PPPPV[_`\xFF\x83\x11\x15a\x1A\xD9W`@Qc\xC3\xCA\x0E7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xFF`$\x82\x01R`D\x01a\x02\xA2V[`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a\x1A\xF8WP`\xFF``\x1B\x19\x90P``[\x90\x85\x16\x90\x84\x90\x1B\x17\x90P\x93\x92PPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1B+\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B{W__\xFD[PZ\xF1\x15\x80\x15a\x1B\x8DW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1B\xD1\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\x01\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CdW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x88\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xD0\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x81\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1D\xC8\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1D\xF8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1ETW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ex\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1E\xC4\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FQW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fu\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1F\x95\x90a2\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a F\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a \x93\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \xC3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a! W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!D\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!\x8D\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\">\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\"\x7F\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"\xAF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#9\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#\x81\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$4\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a${\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%.\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a%z\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&.\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&N\x90a2\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&~\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xDEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x02\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'O\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x03\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(L\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\0\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a)>\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\xB8\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xD4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xF8\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a*J\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra*\xC5\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x05\x91\x90a2aV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a+L\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+|\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xFD\x91\x90a2JV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a,O\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xC0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\0\x91\x90a2JV[PPPPPV[_a-\x12\x84\x84a/TV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a-S\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\0\x91\x90a2aV[_a.\x0F\x84\x84a/TV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.b\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.\x92\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x01a,\xC0V[`@Qc*\xD6B]`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R`d\x82\x01\x85\x90R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x88\x16\x90cU\xAC\x84\xBA\x90`\xC4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/5W__\xFD[PZ\xF1\x15\x80\x15a/GW=__>=_\xFD[PPPPPPPPPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a/w\x90a2|V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xEF\x91\x90a2\x9FV[a0\x0FW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x02\xA2V[\x93\x92PPPV[`@Q\x80`\xA0\x01`@R\x80a0)a0\xC2V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01a0\x88a0\x16V[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01a0\xB0a0\x16V[\x90R\x90V[a\x17{\x80a3A\x839\x01\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[a1\x13`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a0\xD1W\x90PP\x90V[_`@\x82\x84\x03\x12\x80\x15a1:W__\xFD[P\x90\x92\x91PPV[\x81Qa\x02@\x82\x01\x90\x82_[`\x02\x81\x10\x15a1\xB8W\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pa1MV[PPP` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x84\x01R`@\x84\x01Q\x16a\x01\xE0\x83\x01R``\x83\x01Qa\x02\0\x83\x01R`\x80\x90\x92\x01Qa\x02 \x90\x91\x01R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2\x0CW__\xFD[PV[_` \x82\x84\x03\x12\x15a2\x1FW__\xFD[\x815a0\x0F\x81a1\xF8V[_` \x82\x84\x03\x12\x15a2:W__\xFD[\x81Q`\xFF\x81\x16\x81\x14a0\x0FW__\xFD[_` \x82\x84\x03\x12\x15a2ZW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a2qW__\xFD[\x81Qa0\x0F\x81a1\xF8V[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a2\xAFW__\xFD[\x81Q\x80\x15\x15\x81\x14a0\x0FW__\xFD[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V\xFE`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x17{8\x03\x80a\x17{\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01&V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80\x90\x81R\x91\x16`\xA0\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x02\x81RaMM`\xF0\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x01\x81R`1`\xF8\x1B\x90\x82\x01R\x81Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x83\x01R\x7F\x13\xB9\x13\xC1\xF3\x17\xDDq\xB6\xDF\xE5:\xBE\x93\x80\xC6R\xF9I\x8C\x8D\xE2[k9\xD8&4\x99\xAE{\xB6\x81\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF\x94\x81\x01\x94\x90\x94R0\x84\x84\x01R\x81Q\x80\x85\x03\x90\x93\x01\x83R`\xC0\x90\x93\x01\x90R\x80Q\x91\x01 `\x04Ua\x01^V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01#W__\xFD[PV[__`@\x83\x85\x03\x12\x15a\x017W__\xFD[\x82Qa\x01B\x81a\x01\x0FV[` \x84\x01Q\x90\x92Pa\x01S\x81a\x01\x0FV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x15\xEEa\x01\x8D_9_\x81\x81a\x01J\x01Ra\x03\xAC\x01R_\x81\x81a\x036\x01Ra\n)\x01Ra\x15\xEE_\xF3\xFE`\x80`@R`\x046\x10a\x01?W_5`\xE0\x1C\x80cR?\xBA\x7F\x11a\0\xB3W\x80c\x9D\xC2\x9F\xAC\x11a\0mW\x80c\x9D\xC2\x9F\xAC\x14a\x04CW\x80c\xA9\x05\x9C\xBB\x14a\x04bW\x80c\xD5\x05\xAC\xCF\x14a\x04\x81W\x80c\xDDb\xED>\x14a\x04\xA0W\x80c\xE4,\x08\xF2\x14a\x04\xD6W\x80c\xEB@\x13?\x14a\x04\xF5W__\xFD[\x80cR?\xBA\x7F\x14a\x03pW\x80cf\r\rg\x14a\x03\x9BW\x80cp\xA0\x821\x14a\x03\xCEW\x80c~\xCE\xBE\0\x14a\x03\xF9W\x80c\x8C\x1B_\xDE\x14a\x04$W\x80c\x95\xD8\x9BA\x14a\x01\xA6W__\xFD[\x80c0\xAD\xF8\x1F\x11a\x01\x04W\x80c0\xAD\xF8\x1F\x14a\x02yW\x80c1<\xE5g\x14a\x02\xACW\x80c5/\x9A\xED\x14a\x02\xD2W\x80c6D\xE5\x15\x14a\x02\xF1W\x80c@\xC1\x0F\x19\x14a\x03\x06W\x80cJJ{\x04\x14a\x03%W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\xA6W\x80c\x07\x8D;y\x14a\x01\xE9W\x80c\t^\xA7\xB3\x14a\x02\x08W\x80c\x18\x16\r\xDD\x14a\x027W\x80c#\xB8r\xDD\x14a\x02ZW__\xFD[6a\x01\xA2W_a\x01n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\x14V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x01\xA0W`@Qcs\x8D(\xDF`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\0[__\xFD[4\x80\x15a\x01\xB1W__\xFD[Pa\x01\xD3`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMM`\xF0\x1B\x81RP\x81V[`@Qa\x01\xE0\x91\x90a\x12lV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xF4W__\xFD[Pa\x01\xA0a\x02\x036`\x04a\x12\x99V[a\x05\xC4V[4\x80\x15a\x02\x13W__\xFD[Pa\x02'a\x02\"6`\x04a\x12\xD7V[a\x06$V[`@Q\x90\x15\x15\x81R` \x01a\x01\xE0V[4\x80\x15a\x02BW__\xFD[Pa\x02L`\x01T\x81V[`@Q\x90\x81R` \x01a\x01\xE0V[4\x80\x15a\x02eW__\xFD[Pa\x02'a\x02t6`\x04a\x12\x99V[a\x069V[4\x80\x15a\x02\x84W__\xFD[Pa\x02L\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[4\x80\x15a\x02\xB7W__\xFD[Pa\x02\xC0`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xE0V[4\x80\x15a\x02\xDDW__\xFD[Pa\x02La\x02\xEC6`\x04a\x13\x01V[a\x06\x87V[4\x80\x15a\x02\xFCW__\xFD[Pa\x02L`\x04T\x81V[4\x80\x15a\x03\x11W__\xFD[Pa\x01\xA0a\x03 6`\x04a\x12\xD7V[a\x06\xA3V[4\x80\x15a\x030W__\xFD[Pa\x03X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE0V[4\x80\x15a\x03{W__\xFD[Pa\x02La\x03\x8A6`\x04a\x13\x01V[_` \x81\x90R\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xA6W__\xFD[Pa\x03X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xD9W__\xFD[Pa\x02La\x03\xE86`\x04a\x13\x01V[`\x02` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04\x04W__\xFD[Pa\x02La\x04\x136`\x04a\x13\x01V[`\x05` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x04/W__\xFD[Pa\x02La\x04>6`\x04a\x13\x01V[a\x06\xC3V[4\x80\x15a\x04NW__\xFD[Pa\x01\xA0a\x04]6`\x04a\x12\xD7V[a\x06\xDFV[4\x80\x15a\x04mW__\xFD[Pa\x02'a\x04|6`\x04a\x12\xD7V[a\x06\xFBV[4\x80\x15a\x04\x8CW__\xFD[Pa\x01\xA0a\x04\x9B6`\x04a\x13\x1CV[a\x07\x07V[4\x80\x15a\x04\xABW__\xFD[Pa\x02La\x04\xBA6`\x04a\x13\x8DV[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04\xE1W__\xFD[Pa\x02La\x04\xF06`\x04a\x13\x01V[a\t\x08V[4\x80\x15a\x05\0W__\xFD[Pa\x02La\x05\x0F6`\x04a\x13\x01V[a\tpV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x05K\x90` \x80\x82R`\x03\x90\x82\x01Rb\x15\xD3\x95`\xEA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x7F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x13\xC4V[\x92\x91PPV[a\x06\x14`@Q` \x01a\x05\xD6\x90a\x13\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\n\rV[a\x06\x1F\x83\x83\x83a\n\xBBV[PPPV[_a\x0603\x84\x84a\x0B\x03V[P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x80\x85R\x92R\x82 Ta\x06r\x91\x86\x91a\x06m\x90\x86\x90a\x14\x17V[a\x0B\x03V[a\x06}\x84\x84\x84a\x0BdV[P`\x01\x93\x92PPPV[_a\x06\x9A`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x05\xBE\x82a\x0C\x07V[a\x06\xB5`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x06\xBF\x82\x82a\x0C\xAFV[PPV[_a\x06\xD6`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x05\xBE\x82a\r=V[a\x06\xF1`@Q` \x01a\x05\xD6\x90a\x13\xDFV[a\x06\xBF\x82\x82a\r\xDDV[_a\x0603\x84\x84a\x0BdV[B\x84\x10\x15a\x07EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x15Q\x8E\x88\x11V\x14\x12T\x91Q`\xAA\x1B`D\x82\x01R`d\x01a\x01\x97V[`\x04T`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x05` R`@\x81 \x80T\x91\x92\x91\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x91\x8B\x91\x8B\x91\x8B\x91\x90\x87a\x07\x97\x83a\x14*V[\x90\x91UP`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x87\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\x10\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 _\x80\x85R\x91\x84\x01\x80\x84R\x81\x90R`\xFF\x88\x16\x92\x84\x01\x92\x90\x92R``\x83\x01\x86\x90R`\x80\x83\x01\x85\x90R\x90\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08xW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x08\xAEWP\x88`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x08\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtUF: INVALID_SIGNATURE`X\x1B`D\x82\x01R`d\x01a\x01\x97V[a\x08\xFD\x89\x89\x89a\x0B\x03V[PPPPPPPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x14BV[_a\t\x83`@Q` \x01a\x05\xD6\x90a\x13\xDFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEB\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x91PP\x91\x90PV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nvW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9A\x91\x90a\x14YV[a\x06\xBFW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x01\x97\x92\x91\x90a\x14xV[0`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\n\xEFW`@Qcs\x87\xC8\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x01\x97V[a\n\xFA\x83\x83\x83a\x0EdV[a\x06\x1F\x83a\x0F\x10V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x02` R`@\x90 Ta\x0B\x86\x90\x82a\x0F\x91V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x02` R`@\x80\x82 \x93\x90\x93U\x90\x84\x16\x81R Ta\x0B\xB4\x90\x82a\x0F\xE6V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x81\x81R`\x02` R`@\x90\x81\x90 \x93\x90\x93U\x91Q\x90\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x0BW\x90\x85\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x81\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x0C\xA7\x82\x82a\x14\x17V[\x94\x93PPPPV[`\x01Ta\x0C\xBC\x90\x82a\x0F\xE6V[`\x01U`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x02` R`@\x90 Ta\x0C\xE1\x90\x82a\x0F\xE6V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x81\x81R`\x02` R`@\x80\x82 \x93\x90\x93U\x91Q\x90\x91\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\r1\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 T\x90Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92\x90\x91\x83\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB7\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R` \x81\x90R`@\x90 \x81\x90U\x90Pa\x0C\xA7\x81\x83a\x14\x17V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x02` R`@\x90 Ta\r\xFF\x90\x82a\x0F\x91V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x02` R`@\x90 U`\x01Ta\x0E$\x90\x82a\x0F\x91V[`\x01U`@Q\x81\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01a\r1V[\x80_\x03a\x0EpWPPPV[a\x0Ey\x82a\x10:V[__a\x0E\x86\x85\x85\x85a\x10dV[\x91P\x91P\x81\x15a\x0E\x97WPPPPPV[_a\x0E\xA1\x82a\x11\xB3V[P\x90P\x7F\xC9\xF1M\x9A\n\x9BFG\x0C|\x0BlP\x8F\x82\x83\xAB\xAA\xB7\xF7\x95\xF1S\x95<X\xCDBP\x82M\xAE\x81\x83`@Qa\x0E\xD5\x92\x91\x90a\x14\x9BV[`@Q\x80\x91\x03\x90\xA1`@Qc\x01/;\x8F`\xE7\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`D\x81\x01\x85\x90R`d\x01a\x01\x97V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fv\x91\x90a\x14BV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16_\x90\x81R` \x81\x90R`@\x90 UV[_\x82a\x0F\x9D\x83\x82a\x14\x17V[\x91P\x81\x11\x15a\x05\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x01\x97V[_\x82a\x0F\xF2\x83\x82a\x14\xC8V[\x91P\x81\x10\x15a\x05\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a\x01\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10aW`@Qc\xD5Q\x82=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q_\x92``\x92\x91\x84\x91\x82\x91\x90\x89\x16\x90a\x10\xC6\x90\x85\x90a\x14\xDBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x10\xFFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x11\x04V[``\x91P[P\x91P\x91P\x81\x15a\x11\xA3W\x80Q_\x03a\x11bW_\x88`\x01`\x01`\xA0\x1B\x03\x16;\x11a\x11bW_`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x10\xD8[\x1B\x08\x1D\x1B\xC8\x1B\x9B\xDB\x8BX\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B\x81RP\x94P\x94PPPPa\x11\xABV[_\x81Q\x11\x80\x15a\x11\x83WP\x80\x80` \x01\x90Q\x81\x01\x90a\x11\x81\x91\x90a\x14YV[\x15[\x15a\x11\x95W_\x94P\x92Pa\x11\xAB\x91PPV[`\x01\x94P\x92Pa\x11\xAB\x91PPV[_\x94P\x92PPP[\x93P\x93\x91PPV[``_`D\x83Q\x10\x15a\x11\xD8WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x90\x92\x90\x91PV[_a\x11\xE4\x84` \x01Q\x90V[\x90Pc\x07\xB9\xE43`\xE5\x1B`\x01`\x01`\xE0\x1B\x03\x19\x82\x16\x01a\x12#W`\x04\x84\x01\x93P\x83\x80` \x01\x90Q\x81\x01\x90a\x12\x18\x91\x90a\x15\x05V[\x94`\x01\x94P\x92PPPV[_`@Q\x80` \x01`@R\x80_\x81RP\x90\x92P\x92PP\x91P\x91V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x12~` \x83\x01\x84a\x12>V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10aW__\xFD[___``\x84\x86\x03\x12\x15a\x12\xABW__\xFD[\x835a\x12\xB6\x81a\x12\x85V[\x92P` \x84\x015a\x12\xC6\x81a\x12\x85V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a\x12\xE8W__\xFD[\x825a\x12\xF3\x81a\x12\x85V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x13\x11W__\xFD[\x815a\x12~\x81a\x12\x85V[_______`\xE0\x88\x8A\x03\x12\x15a\x132W__\xFD[\x875a\x13=\x81a\x12\x85V[\x96P` \x88\x015a\x13M\x81a\x12\x85V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x13pW__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x13\x9EW__\xFD[\x825a\x13\xA9\x81a\x12\x85V[\x91P` \x83\x015a\x13\xB9\x81a\x12\x85V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x13\xD4W__\xFD[\x81Qa\x12~\x81a\x12\x85V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\xBEWa\x05\xBEa\x14\x03V[_`\x01\x82\x01a\x14;Wa\x14;a\x14\x03V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x14RW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14iW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12~W__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0C\xA7\x90\x83\x01\x84a\x12>V[`@\x81R_a\x14\xAD`@\x83\x01\x85a\x12>V[\x82\x81\x03` \x84\x01Ra\x14\xBF\x81\x85a\x12>V[\x95\x94PPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x05\xBEWa\x05\xBEa\x14\x03V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x15\x15W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15+W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x15;W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15UWa\x15Ua\x14\xF1V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\x84Wa\x15\x84a\x14\xF1V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x15\x9BW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xE4k\x81\xFA\x1D\t5kY\x03\x90q\xC7\x90\0G\x15\xA2?m\x9E\xA8k\x94\x84\xF3\x9FC\x96\x06\xF5QdsolcC\0\x08\x1C\x003\xA2dipfsX\"\x12 \xBF\xA5\x03\xD51\x94\xFAf\xDF\xB8\xE2&g1\xCE\x7F0eD_R\xC4\xE0\xEA/\xD6U;\xCB\xAAH5dsolcC\0\x08\x1C\x003",
    );
    /**```solidity
struct CreatePoolParams { address token; address source; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreatePoolParams {
        pub token: alloy::sol_types::private::Address,
        pub source: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<CreatePoolParams> for UnderlyingRustTuple<'_> {
            fn from(value: CreatePoolParams) -> Self {
                (value.token, value.source)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CreatePoolParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    source: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CreatePoolParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CreatePoolParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.source,
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
        impl alloy_sol_types::SolType for CreatePoolParams {
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
        impl alloy_sol_types::SolStruct for CreatePoolParams {
            const NAME: &'static str = "CreatePoolParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreatePoolParams(address token,address source)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.source,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CreatePoolParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.source,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.source,
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
    /**Custom error with signature `EmptyBase()` and selector `0x23449c2c`.
```solidity
error EmptyBase();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyBase {}
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
        impl ::core::convert::From<EmptyBase> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyBase) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyBase {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyBase {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyBase()";
            const SELECTOR: [u8; 4] = [35u8, 68u8, 156u8, 44u8];
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
    /**Custom error with signature `EmptyConfiguration()` and selector `0x18549934`.
```solidity
error EmptyConfiguration();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyConfiguration {}
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
        impl ::core::convert::From<EmptyConfiguration> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyConfiguration) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyConfiguration {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyConfiguration {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyConfiguration()";
            const SELECTOR: [u8; 4] = [24u8, 84u8, 153u8, 52u8];
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
    /**Custom error with signature `EmptyInterestRateStrategy()` and selector `0xfa84aace`.
```solidity
error EmptyInterestRateStrategy();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyInterestRateStrategy {}
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
        impl ::core::convert::From<EmptyInterestRateStrategy>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyInterestRateStrategy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyInterestRateStrategy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyInterestRateStrategy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyInterestRateStrategy()";
            const SELECTOR: [u8; 4] = [250u8, 132u8, 170u8, 206u8];
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
    /**Custom error with signature `InvalidDecimals(uint256,uint256)` and selector `0xc3ca0e37`.
```solidity
error InvalidDecimals(uint256 decimals, uint256 MaxValidDecimals);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDecimals {
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
        pub MaxValidDecimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidDecimals> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDecimals) -> Self {
                (value.decimals, value.MaxValidDecimals)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidDecimals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    decimals: tuple.0,
                    MaxValidDecimals: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDecimals {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDecimals(uint256,uint256)";
            const SELECTOR: [u8; 4] = [195u8, 202u8, 14u8, 55u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.MaxValidDecimals),
                )
            }
        }
    };
    /**Custom error with signature `PoolAlreadyExists(bytes32,address)` and selector `0x0b8422c3`.
```solidity
error PoolAlreadyExists(bytes32 key, address poolBank);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PoolAlreadyExists {
        pub key: alloy::sol_types::private::FixedBytes<32>,
        pub poolBank: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<PoolAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: PoolAlreadyExists) -> Self {
                (value.key, value.poolBank)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    key: tuple.0,
                    poolBank: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PoolAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PoolAlreadyExists(bytes32,address)";
            const SELECTOR: [u8; 4] = [11u8, 132u8, 34u8, 195u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.poolBank,
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
    /**Function with signature `createPool((address,address))` and selector `0x8251a687`.
```solidity
function createPool(CreatePoolParams memory params) external returns (Pool.Props memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createPoolCall {
        pub params: <CreatePoolParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`createPool((address,address))`](createPoolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createPoolReturn {
        pub _0: <Pool::Props as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (CreatePoolParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <CreatePoolParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createPoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: createPoolCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createPoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Pool::Props,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Pool::Props as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createPoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createPoolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createPoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createPoolCall {
            type Parameters<'a> = (CreatePoolParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createPoolReturn;
            type ReturnTuple<'a> = (Pool::Props,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createPool((address,address))";
            const SELECTOR: [u8; 4] = [130u8, 81u8, 166u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<CreatePoolParams as alloy_sol_types::SolType>::tokenize(&self.params),)
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
    ///Container for all the [`PoolFactory`](self) function calls.
    pub enum PoolFactoryCalls {
        createPool(createPoolCall),
        dataStore(dataStoreCall),
        eventEmitter(eventEmitterCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl PoolFactoryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [74u8, 74u8, 123u8, 4u8],
            [102u8, 13u8, 13u8, 103u8],
            [130u8, 81u8, 166u8, 135u8],
            [159u8, 247u8, 140u8, 48u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolFactoryCalls {
        const NAME: &'static str = "PoolFactoryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::createPool(_) => {
                    <createPoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::dataStore(_) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventEmitter(_) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolFactoryCalls>] = &[
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn createPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryCalls> {
                        <createPoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryCalls::createPool)
                    }
                    createPool
                },
                {
                    fn eventEmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryCalls> {
                        <eventEmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryCalls::eventEmitter)
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
                Self::createPool(inner) => {
                    <createPoolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::createPool(inner) => {
                    <createPoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`PoolFactory`](self) custom errors.
    pub enum PoolFactoryErrors {
        EmptyBase(EmptyBase),
        EmptyConfiguration(EmptyConfiguration),
        EmptyInterestRateStrategy(EmptyInterestRateStrategy),
        EmptyPool(EmptyPool),
        InvalidDecimals(InvalidDecimals),
        PoolAlreadyExists(PoolAlreadyExists),
    }
    #[automatically_derived]
    impl PoolFactoryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 132u8, 34u8, 195u8],
            [24u8, 84u8, 153u8, 52u8],
            [35u8, 68u8, 156u8, 44u8],
            [115u8, 87u8, 217u8, 31u8],
            [195u8, 202u8, 14u8, 55u8],
            [250u8, 132u8, 170u8, 206u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolFactoryErrors {
        const NAME: &'static str = "PoolFactoryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyBase(_) => <EmptyBase as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyConfiguration(_) => {
                    <EmptyConfiguration as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyInterestRateStrategy(_) => {
                    <EmptyInterestRateStrategy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidDecimals(_) => {
                    <InvalidDecimals as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PoolAlreadyExists(_) => {
                    <PoolAlreadyExists as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolFactoryErrors>] = &[
                {
                    fn PoolAlreadyExists(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <PoolAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::PoolAlreadyExists)
                    }
                    PoolAlreadyExists
                },
                {
                    fn EmptyConfiguration(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <EmptyConfiguration as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::EmptyConfiguration)
                    }
                    EmptyConfiguration
                },
                {
                    fn EmptyBase(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <EmptyBase as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::EmptyBase)
                    }
                    EmptyBase
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::EmptyPool)
                    }
                    EmptyPool
                },
                {
                    fn InvalidDecimals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <InvalidDecimals as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::InvalidDecimals)
                    }
                    InvalidDecimals
                },
                {
                    fn EmptyInterestRateStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolFactoryErrors> {
                        <EmptyInterestRateStrategy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolFactoryErrors::EmptyInterestRateStrategy)
                    }
                    EmptyInterestRateStrategy
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
                Self::EmptyBase(inner) => {
                    <EmptyBase as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyConfiguration(inner) => {
                    <EmptyConfiguration as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyInterestRateStrategy(inner) => {
                    <EmptyInterestRateStrategy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidDecimals(inner) => {
                    <InvalidDecimals as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PoolAlreadyExists(inner) => {
                    <PoolAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EmptyBase(inner) => {
                    <EmptyBase as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyConfiguration(inner) => {
                    <EmptyConfiguration as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyInterestRateStrategy(inner) => {
                    <EmptyInterestRateStrategy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidDecimals(inner) => {
                    <InvalidDecimals as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PoolAlreadyExists(inner) => {
                    <PoolAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolFactory`](self) contract instance.

See the [wrapper's documentation](`PoolFactoryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolFactoryInstance<T, P, N> {
        PoolFactoryInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<PoolFactoryInstance<T, P, N>>,
    > {
        PoolFactoryInstance::<
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
        PoolFactoryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**A [`PoolFactory`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolFactory`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolFactoryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolFactoryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolFactoryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolFactoryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolFactory`](self) contract instance.

See the [wrapper's documentation](`PoolFactoryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<PoolFactoryInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> PoolFactoryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolFactoryInstance<T, P, N> {
            PoolFactoryInstance {
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
    > PoolFactoryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`createPool`] function.
        pub fn createPool(
            &self,
            params: <CreatePoolParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, createPoolCall, N> {
            self.call_builder(&createPoolCall { params })
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
    > PoolFactoryInstance<T, P, N> {
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
