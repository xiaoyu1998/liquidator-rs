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
    function getPools2(address dataStore, bytes32[] memory poolKeys) external view returns (Pool.Props[] memory);
    function getPoolsCount(address dataStore) external view returns (uint256);
    function getPoolsInfo(address dataStore) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, uint256 start, uint256 end) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPoolsInfo(address dataStore, bytes32[] memory poolKeys) external view returns (ReaderPoolUtils.GetPoolInfo[] memory);
    function getPositions(address dataStore, bytes32[] memory positionKeys) external view returns (ReaderPositionUtils.GetPosition[] memory);
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
        "internalType": "struct Pool.Props[]",
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
    ///0x6080604052348015600e575f5ffd5b506175128061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610111575f3560e01c80635c39f4671161009e578063c2bdeda11161006e578063c2bdeda114610284578063d28b0a15146102b2578063e335adb7146102c5578063eed07428146102d8578063f68a7131146102eb575f5ffd5b80635c39f4671461022b578063739118a41461024b57806378f212d11461025e5780638f6c7a3c14610271575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c4578063525f560c146101e557806357b91ca6146102055780635a6f571014610218575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b6101286101233660046168ed565b6102fe565b6040516101359190616936565b60405180910390f35b61015161014c3660046168ed565b61031e565b6040516001600160a01b039091168152602001610135565b61017c610177366004616a21565b61032e565b60408051928352602083019190915201610135565b61012861019f366004616a6f565b610349565b6101b76101b2366004616b09565b61035e565b6040516101359190616bb4565b6101d76101d23660046168ed565b610417565b604051908152602001610135565b6101f86101f3366004616b09565b610421565b6040516101359190616d56565b6101d76102133660046168ed565b61042d565b6101d76102263660046168ed565b610437565b61023e6102393660046168ed565b610441565b6040516101359190616ee7565b6101f8610259366004617006565b61045a565b61015161026c3660046168ed565b610475565b61023e61027f366004616a6f565b61047f565b61029761029236600461704b565b61048c565b60408051938452602084019290925290820152606001610135565b6102976102c036600461704b565b6104ae565b6101516102d33660046168ed565b6104be565b6101286102e6366004616b09565b6104c8565b6101d76102f93660046170af565b6104d4565b60605f61030a836104ee565b9050610317835f83610580565b9392505050565b5f61032882610602565b92915050565b5f5f61033c868686866106b3565b9150915094509492505050565b6060610356848484610580565b949350505050565b60605f825167ffffffffffffffff81111561037b5761037b616aa1565b6040519080825280602002602001820160405280156103b457816020015b6103a161633a565b8152602001906001900390816103995790505b5090505f5b835181101561040f575f6103e6868684815181106103d9576103d9617106565b60200260200101516106ef565b9050808383815181106103fb576103fb617106565b6020908102919091010152506001016103b9565b509392505050565b5f61032882611967565b606061031783836119b8565b5f61032882611a6e565b5f610328826104ee565b60605f61044d836104ee565b9050610317835f83611abf565b60605f6104678484611bb6565b905061035684845f84611c2c565b5f61032882611c51565b6060610356848484611abf565b5f5f5f61049c8888888888611c8d565b9250925092505b955095509592505050565b5f5f5f61049c8888888888611d97565b5f61032882611e10565b60606103178383611e61565b5f6104e28686868686611f47565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161050d9061711a565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161054191815260200190565b602060405180830381865afa15801561055c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610328919061713d565b60606105b76040518060400160405280601381526020017219d95d141bdbdb1cd25b999bcc081cdd185c9d606a1b81525084611f7a565b6105ea6040518060400160405280601181526020017019d95d141bdbdb1cd25b999bcc08195b99607a1b81525083611f7a565b5f6105f6858585611fa7565b90506104e58582612048565b5f816001600160a01b03166321f8a721604051602001610640906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161067491815260200190565b602060405180830381865afa15801561068f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190617154565b5f5f5f6106c086866120fe565b90505f6106cd88836106ef565b90505f5f6106dc8a84896121a5565b50919c909b509950505050505050505050565b6106f761633a565b8261070061633a565b816001600160a01b03166391d4403c60405160200161071e9061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610772573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610796919061716f565b6107a35791506103289050565b816001600160a01b03166321f8a721856040516020016107e3906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610813929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161084791815260200190565b602060405180830381865afa158015610862573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108869190617154565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610904929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161093891815260200190565b602060405180830381865afa158015610953573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610977919061713d565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016109cd906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016109fd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a3191815260200190565b602060405180830381865afa158015610a4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a70919061713d565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610ad19060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610b01929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b3591815260200190565b602060405180830381865afa158015610b50573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b74919061713d565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610bdf9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610c0f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4391815260200190565b602060405180830381865afa158015610c5e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c82919061713d565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610ce39060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d13929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4791815260200190565b602060405180830381865afa158015610d62573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d86919061713d565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610e03929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e3791815260200190565b602060405180830381865afa158015610e52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e76919061713d565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001610eeb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f1f91815260200190565b602060405180830381865afa158015610f3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5e9190617154565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100891815260200190565b602060405180830381865afa158015611023573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611047919061713d565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161109e90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016110ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161110291815260200190565b602060405180830381865afa15801561111d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611141919061713d565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016111a39060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016111d3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161120791815260200190565b602060405180830381865afa158015611222573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611246919061713d565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016112b29060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016112e2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131691815260200190565b602060405180830381865afa158015611331573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611355919061713d565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016113b79060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161141b91815260200190565b602060405180830381865afa158015611436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145a919061713d565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016114b390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016114e3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161151791815260200190565b602060405180830381865afa158015611532573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611556919061713d565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016115a490602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115d4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161160891815260200190565b602060405180830381865afa158015611623573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116479190617154565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016116b5906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116e5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171991815260200190565b602060405180830381865afa158015611734573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117589190617154565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016117bb906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016117eb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161181f91815260200190565b602060405180830381865afa15801561183a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185e919061713d565b60608201526040516001600160a01b0383169063bd02d0f59086906118b7906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161191b91815260200190565b602060405180830381865afa158015611936573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061195a919061713d565b6080820152949350505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b60605f825167ffffffffffffffff8111156119d5576119d5616aa1565b604051908082528060200260200182016040528015611a0e57816020015b6119fb61636e565b8152602001906001900390816119f35790505b5090505f5b835181101561040f575f848281518110611a2f57611a2f617106565b602002602001015190505f611a4487836123fa565b905080848481518110611a5957611a59617106565b60209081029190910101525050600101611a13565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f611acd858585611fa7565b90505f815167ffffffffffffffff811115611aea57611aea616aa1565b604051908082528060200260200182016040528015611b2357816020015b611b106163c1565b815260200190600190039081611b085790505b5090505f5b8251811015611bac575f838281518110611b4457611b44617106565b60200260200101519050611b7760405180604001604052806007815260200166706f6f6c4b657960c81b81525082612a09565b5f611b828983612a3a565b905080848481518110611b9757611b97617106565b60209081029190910101525050600101611b28565b5095945050505050565b5f826001600160a01b031663f3903b9f611bcf84612eda565b6040518263ffffffff1660e01b8152600401611bed91815260200190565b602060405180830381865afa158015611c08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610317919061713d565b60605f611c3b86868686612f5e565b9050611c4786826119b8565b9695505050505050565b5f816001600160a01b03166321f8a72160405160200161064090602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f611c9b88886120fe565b90505f611ca88a836106ef565b90505f808060ff8916611cde57611cc08d8b86612fe6565b929550919350611cd791508590505f858d82613100565b9050611d0e565b5f1960ff8a1601611d0e57611cf48d8b866131e2565b929550919350611d0b9150859050845f808e613100565b90505b805f03611d28575f5f5f97509750975050505050506104a3565b5f611d32856132e4565b90505f828211611d4b57611d4682846171a2565b611d55565b611d5583836171a2565b90505f611d628284613393565b90505f848411611d7a57611d75826171b5565b611d7c565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f611da588886120fe565b90505f611db28a836106ef565b90505f808060ff8916611de257611dcb8d8b865f6133ce565b929550919350611cd791508590508b5f8087613100565b5f1960ff8a1601611d0e57611df98d8b865f6134f8565b929550919350611d0b91508590505f8c8682613100565b5f816001600160a01b03166321f8a721604051602001610640906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b6060611e936040518060400160405280600d81526020016c33b2ba2837b7b639a4b733379960991b8152508351611f7a565b5f825167ffffffffffffffff811115611eae57611eae616aa1565b604051908082528060200260200182016040528015611ee757816020015b611ed4616440565b815260200190600190039081611ecc5790505b5090505f5b835181101561040f575f848281518110611f0857611f08617106565b602002602001015190505f611f1d8783613607565b905080848481518110611f3257611f32617106565b60209081029190910101525050600101611eec565b5f5f611f5386866120fe565b90505f611f6088836106ef565b9050611f6e8186865f6138c7565b98975050505050505050565b611fa3604051806040016040528060068152602001652573202d257360d01b8152508383613a21565b5050565b6060836001600160a01b031663f069052a604051602001611fc79061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015612021573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261035691908101906171cf565b60605f825167ffffffffffffffff81111561206557612065616aa1565b60405190808252806020026020018201604052801561209e57816020015b61208b616440565b8152602001906001900390816120835790505b5090505f5b835181101561040f575f8482815181106120bf576120bf617106565b602002602001015190505f6120d48783613607565b9050808484815181106120e9576120e9617106565b602090810291909101015250506001016120a3565b5f816001600160a01b0316836001600160a01b03161061211f578183612122565b82825b604051919450925061214f906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b5f5f5f5f6121b1616465565b6121ba88613a6d565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612204573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612228919061713d565b816020018181525050612242875f5f846101400151613abe565b5060808401525060408201526101408101516122649088906001905f90613abe565b5060a084015250606082015260208101515f0361228d575f5f5f5f9450945094509450506123f1565b6122a08682604001518360200151613ba3565b610100820152606081015160208201516122bb918891613ba3565b816101200181815250506122fb6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151611f7a565b612333604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151611f7a565b61236b60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151611f7a565b61239e6040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151611f7a565b6123d16040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151611f7a565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b61240261636e565b61240a6164b4565b6124148484613c62565b8082525180515161242d9160015b6020020151516120fe565b6040820181905261243f9085906106ef565b602082018190528151612453918691613c74565b505050506060820152602081015161246a906132e4565b610300820152805180515160209081015160e0840152808301515151015190516124a091905f5b60200201516040015190613cc4565b60c08201526020810151606001516124b8905f613d05565b60a082015260e081015160c08201516124d19190613d33565b610100820181905260a08201516124e89190613d54565b61012082015260e081015160c0820151610100830151612509929190613d70565b6101408201526020810151815161030083015161252a92879290915f613d8d565b61016082015261014081015161018082015260e081015160c0820151610120830151612557929190613d70565b6101a0820152602081015160600151612571906001613d05565b6101c0820152805180516020908101518101516102008401528083015151810151015190516125a291906001612491565b6101e082018190526102008201516125b991613d33565b61022082018190526101c08201516125d19190613d54565b61024082018190526103008201516125e99190613cc4565b6102608201526102008101516101e082015161022083015161260c929190613d70565b6102808201526020810151815161030083015161262e92879290916001613d8d565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161265d929190613d70565b6102e0820152805161266e90613f22565b60808201528051516020015160e00151600214612766576126988161030001518260800151613d33565b61032082018190526102408201516126af91613cc4565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516126e093613f64565b61036082018190526103a082015260e081015160a08201516127479186916127089190613d54565b61271a8460c001518560a00151613d54565b61272e856102000151866101c00151613d54565b612742866101e00151876101c00151613d54565b613f7e565b6103c0820181905261030082015161275f919061403e565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa1580156127da573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128019190810190617256565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061285657612856617106565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106128a5576128a5617106565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612905573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261292c9190810190617256565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b611fa3604051806040016040528060068152602001652573202d257360d01b81525083612a358461408d565b6140b9565b612a426163c1565b612a4a616591565b612a5484846106ef565b8152612a5f84614100565b6020820152612a6d84613a6d565b610180820181905281516020830151612a88925f9190613abe565b608085015260a08401526040830152606082015280516020820151610180830151612ab69291600191613abe565b61012085015261014084015260e08301526101008201528051612ad890614143565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa158015612b57573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612b7e9190810190617256565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015612bcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bef91906172ea565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612ccf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612cf69190810190617256565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6a91906172ea565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b908301528351910190612e4a906132e4565b8152602001612e598686614352565b6001600160a01b03168152602001612e80835f015160600151660800000000000016151590565b15158152602001612e918686614442565b81528251515160c0015160208201528251604090910190612eb59087905f80614544565b8152602001612ec786845f015161463f565b90526101a0909101819052905092915050565b5f604051602001612f14906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a612f7886612eda565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612fbf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104e591908101906171cf565b5f5f5f5f612ff26165ff565b612ffb88613a6d565b6101c082018190526130129087905f908190613abe565b5060408401525081526101c08101516130319087906001905f90613abe565b5060608401525060208201528051158061304d57506020810151155b15613064575f5f5f5f9450945094509450506123f1565b606086015160381c61ffff16610140820181905261309390889061308b906127109061464a565b612710613ba3565b6101808201819052815160208301516130b6926130b190839061464a565b613ba3565b6080820181905260208201516130cc919061464a565b60e08201819052604082015160608301516101408401516130ee908b9061469f565b94509450945094505093509350935093565b5f5f5f5f86118015613110575083155b1561311f575083905084613154565b5f8711801561312c575084155b1561313b575085905082613154565b604051636331fab160e01b815260040160405180910390fd5b5f61316389606001515f613d05565b90505f6131758a606001516001613d05565b90505f61319385676765c793fa10079d601b1b6130b186600a6173e8565b90505f6131b185676765c793fa10079d601b1b6130b186600a6173e8565b9050805f036131c8575f96505050505050506104e5565b6131d28282613393565b9c9b505050505050505050505050565b5f5f5f5f6131ee6165ff565b6131f788613a6d565b6101c0820181905261320e9087905f908190613abe565b5060408401525081526101c081015161322d9087906001905f90613abe565b5060608401525060208201528051158061324957506020810151155b15613260575f5f5f5f9450945094509450506123f1565b8051602082015161327691906130b1818b61464a565b6101008201819052815161328a919061464a565b610120820152606086015160381c61ffff1661014082018190526101208201516132bd91612710906130b190829061464a565b6101a08201819052604082015160608301516101408401516101208501516130ee9161469f565b5f5f6132f2835f5f5f613abe565b50505090505f6133058460015f5f613abe565b5050509050805f0361331a57505f9392505050565b5f61332985606001515f613d05565b90505f61333b86606001516001613d05565b90505f61335985676765c793fa10079d601b1b6130b186600a6173e8565b90505f61337785676765c793fa10079d601b1b6130b186600a6173e8565b9050805f0361338d57505f979650505050505050565b611f6e82825b5f8115676765c793fa10079d601b1b600284041904841117156133b4575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f6133da6165ff565b6133e389613a6d565b6101c082018190526133fa9088905f908190613abe565b5060408401525081526101c08101516134199088906001905f90613abe565b5060608401525060208201528051158061343557506020810151155b1561344c575f5f5f5f9450945094509450506134ed565b85156134675787815f0181815161346391906171a2565b9052505b606087015160381c61ffff16610140820181905261348e90899061308b906127109061464a565b6101608201819052815160208301516134ac926130b19083906146c3565b6080820181905260208201516134c19161464a565b60c08201819052604082015160608301516101408401516134e3908c9061469f565b9450945094509450505b945094509450949050565b5f5f5f5f6135046165ff565b61350d89613a6d565b6101c082018190526135249088905f908190613abe565b5060408401525081526101c08101516135439088906001905f90613abe565b5060608401525060208201528051158061355f57506020810151155b15613576575f5f5f5f9450945094509450506134ed565b851561359257878160200181815161358e91906171a2565b9052505b805160208201516135a891906130b1818c6146c3565b6080820181905281516135ba9161464a565b60a0820152606087015160381c61ffff16610140820181905260a08201516135e99161308b906127109061464a565b6040820151606083015161014084015160a08501516134e39161469f565b61360f616440565b613617616666565b61362184846106ef565b808252602001516001600160a01b031661365657604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156136c7573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136ee9190810190617256565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561373b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061375f91906172ea565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156137df573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526138069190810190617256565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613856573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061387a91906172ea565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016138b586845f015161463f565b90526020909101819052905092915050565b5f6138d0616465565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613910573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613934919061713d565b6020820152613945865f8080613abe565b50505060c082015261395a8660015f80613abe565b50505060e0820152821561399557848160c00181815161397a91906171a2565b90525060e0810180518591906139919083906171a2565b9052505b60c081015115806139a8575060e0810151155b156139b6575f915050610356565b80602001515f036139e6576139df6103e86139d96139d48888614717565b61477d565b9061464a565b8152613a17565b613a146139fc8683602001518460c00151613ba3565b613a0f8684602001518560e00151613ba3565b61485d565b81525b5195945050505050565b613a68838383604051602401613a39939291906173f3565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614872565b505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff1660028110613adb57613adb617106565b602002015190505f613aed8a8a61487e565b9050805f03613b09575f5f5f5f955095509550955050506134ed565b5f613b18838c6080015161496b565b90505f613b25828a613cc4565b90505f8915613b4a57818410613b4457613b3f848361464a565b613b4c565b5f613b4c565b5f5b90505f613b59858d613cc4565b90505f8c15613b7e57848210613b7857613b73828661464a565b613b80565b5f613b80565b5f5b9050613b8c8587617428565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613bd757838281613bcd57613bcd61743b565b0492505050610317565b808411613bf75760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b613c6a616686565b610317838361499b565b5f5f5f5f5f613c83888861463f565b9050613c918787835f615bb9565b909350915081613ca2575f19613cac565b613cac8383613393565b9450613cb788611a6e565b9350939792965093509350565b5f81156b019d971e4fe8401e740000001983900484111517613ce4575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff851601613d25575060ff60601b19905060605b90198416901c905092915050565b5f818311613d4a57613d4583836171a2565b610317565b61031782846171a2565b5f61031783676765c793fa10079d601b1b6130b185600a6173e8565b5f828411613d8657613d81826171b5565b610356565b5092915050565b5f613dc76040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613dd38686865f615bb9565b60208301528082521580613e045750845160ff841660028110613df857613df8617106565b6020020151602001515f145b15613e12575f9150506104e5565b613e1b87615d25565b604082018190526020820151613e3091613cc4565b6080820181905281511015613e48575f9150506104e5565b60808101518151613e5991906171a2565b816060018181525050613e70866060015184613d05565b60a082018190526060820151613e9c91613e8b90600a6173e8565b676765c793fa10079d601b1b613ba3565b60c08201525f1960ff841601613ec15760c0810151613ebb9085613393565b60c08201525b845160ff841660028110613ed757613ed7617106565b6020020151602001518160c001511115613f1457845160ff841660028110613f0157613f01617106565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901613f42575051602001516060015190565b81516020015160e00151613f5d575051602001516080015190565b505f919050565b5f8415158385111461040f57613f79826171b5565b6104e5565b5f5f613f8987611a6e565b90505f613f968287613cc4565b90505f613fa38386613cc4565b90505f613fb0898461744f565b90505f613fbd838961744f565b90505f613fc983615d6b565b90505f613fd583615d6b565b90505f84138015613fe557505f83125b80613ff957505f84128015613ff957505f83135b1561400d575f9750505050505050506104e5565b805f03614023575f9750505050505050506104e5565b61402d8282613393565b9d9c50505050505050505050505050565b5f815f0361404d57505f610328565b5f8284116140645761405f84846171a2565b61406e565b61406e83856171a2565b90505f61407b8285613393565b905083851161035657613f79816171b5565b6060610328826040516020016140a591815260200190565b604051602081830303815290604052615d80565b613a688383836040516024016140d19392919061746e565b60408051601f198184030181529190526020810180516001600160e01b0316632ced7cef60e01b179052614872565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6141786040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614182845f615f80565b602083015281526060840151614198905f613d05565b6060820181905281516141bd91676765c793fa10079d601b1b906130b190600a6173e8565b604082015260208101515f036141d8575f6080820152614278565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561424e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614272919061713d565b60808201525b614283846001615f80565b602083018190529082525f0361429e575f60a082015261433e565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614314573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614338919061713d565b60a08201525b80608001518160a001519250925050915091565b5f5f61435e8484615fc6565b9050806001600160a01b03166321f8a7218460405160200161439f906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016143cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161440391815260200190565b602060405180830381865afa15801561441e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190617154565b5f5f61444e8484615fc6565b9050806001600160a01b031663bd02d0f5846040516020016144a19060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016144d1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161450591815260200190565b602060405180830381865afa158015614520573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610356919061713d565b5f5f61455185600161487e565b905082156145665761456384826171a2565b90505b5f61457087616081565b90505f61457d8383613cc4565b875160200151606001519091505f9082106145ab57875160200151606001516145a690836171a2565b6145ad565b5f5b90506145dc6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525085611f7a565b61460c6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525084611f7a565b611f6e604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525083611f7a565b5f61031783836160c5565b5f8261465683826171a2565b91508111156103285760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161364d565b5f811561138819839004841115176146b5575f5ffd5b506127109102611388010490565b5f826146cf8382617428565b91508110156103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161364d565b5f81158061473a5750828261472c81836174a6565b925061473890836174bd565b145b6103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161364d565b5f815f0361478c57505f919050565b5f6001614798846160de565b901c6001901b905060018184816147b1576147b161743b565b048201901c905060018184816147c9576147c961743b565b048201901c905060018184816147e1576147e161743b565b048201901c905060018184816147f9576147f961743b565b048201901c905060018184816148115761481161743b565b048201901c905060018184816148295761482961743b565b048201901c905060018184816148415761484161743b565b048201901c90506103178182858161485b5761485b61743b565b045b5f81831061486b5781610317565b5090919050565b61487b81616171565b50565b5f5f835f01518360ff166002811061489857614898617106565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156148f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614915919061713d565b9050805f03614928575f92505050610328565b606082015160c083015161493c9082617428565b82106149605760c083015161495182846171a2565b61495b91906171a2565b611c47565b5f9695505050505050565b5f8260a001515f0361497e57505f610328565b5f6149898484616190565b60a08501519091506103569082613cc4565b6149a3616686565b826149ac616686565b816001600160a01b03166391d4403c6040516020016149ec906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614a40573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a64919061716f565b614a715791506103289050565b816001600160a01b031663bd02d0f585604051602001614aab906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614adb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b0f91815260200190565b602060405180830381865afa158015614b2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b4e919061713d565b816020018181525050816001600160a01b03166321f8a72185604051602001614b96906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bc6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bfa91815260200190565b602060405180830381865afa158015614c15573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c399190617154565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614c95906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614cc5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cf991815260200190565b602060405180830381865afa158015614d14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d389190617154565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614db3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614de791815260200190565b602060405180830381865afa158015614e02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e26919061713d565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614e7a9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614eaa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ede91815260200190565b602060405180830381865afa158015614ef9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f1d919061713d565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614f77906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614fa7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fdb91815260200190565b602060405180830381865afa158015614ff6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061501a919061713d565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001615073906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016150a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150d791815260200190565b602060405180830381865afa1580156150f2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615116919061713d565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161519c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151d091815260200190565b602060405180830381865afa1580156151eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061520f919061713d565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615269906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615299929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152cd91815260200190565b602060405180830381865afa1580156152e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061530c919061713d565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161537f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153b391815260200190565b602060405180830381865afa1580156153ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153f2919061713d565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615466929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161549a91815260200190565b602060405180830381865afa1580156154b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154d99190617154565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161558091815260200190565b602060405180830381865afa15801561559b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155bf919061713d565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156149060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615644929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161567891815260200190565b602060405180830381865afa158015615693573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156b7919061713d565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161571290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615742929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161577691815260200190565b602060405180830381865afa158015615791573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157b5919061713d565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161580f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161583f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161587391815260200190565b602060405180830381865afa15801561588e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906158b2919061713d565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016159149060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615944929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161597891815260200190565b602060405180830381865afa158015615993573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159b7919061713d565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615a1290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615a42929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a7691815260200190565b602060405180830381865afa158015615a91573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615ab5919061713d565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615b04906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b6891815260200190565b602060405180830381865afa158015615b83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615ba7919061713d565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615c5c575f5f615be48a8a5f6161d3565b915091505f615c005f8c60600151613d0590919063ffffffff16565b90505f615c1e84676765c793fa10079d601b1b6130b185600a6173e8565b90505f615c3c84676765c793fa10079d601b1b6130b186600a6173e8565b9050615c488288617428565b9650615c548187617428565b955050505050505b865160200151516001600160a01b03868116911614615d18575f5f615c838a8a60016161d3565b915091505f615ca060018c60600151613d0590919063ffffffff16565b90505f615cbe84676765c793fa10079d601b1b6130b185600a6173e8565b90505f615cdc84676765c793fa10079d601b1b6130b186600a6173e8565b90505f615ce9838d613cc4565b90505f615cf6838e613cc4565b9050615d02828a617428565b9850615d0e8189617428565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615d7c57815f03610328565b5090565b60408051808201909152601081526f181899199a1a9b1b9c1cb0b131b232b360811b60208201528151606091905f90615dba9060026174a6565b615dc5906002617428565b67ffffffffffffffff811115615ddd57615ddd616aa1565b6040519080825280601f01601f191660200182016040528015615e07576020820181803683370190505b509050600360fc1b815f81518110615e2157615e21617106565b60200101906001600160f81b03191690815f1a905350600f60fb1b81600181518110615e4f57615e4f617106565b60200101906001600160f81b03191690815f1a9053505f5b845181101561040f57826004868381518110615e8557615e85617106565b016020015182516001600160f81b031990911690911c60f81c908110615ead57615ead617106565b01602001516001600160f81b03191682615ec88360026174a6565b615ed3906002617428565b81518110615ee357615ee3617106565b60200101906001600160f81b03191690815f1a90535082858281518110615f0c57615f0c617106565b602091010151815160f89190911c600f16908110615f2c57615f2c617106565b01602001516001600160f81b03191682615f478360026174a6565b615f52906003617428565b81518110615f6257615f62617106565b60200101906001600160f81b03191690815f1a905350600101615e67565b5f5f5f615fad855f01518560ff1660028110615f9e57615f9e617106565b6020020151866080015161496b565b90505f615fba868661487e565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001615fe99061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa15801561603d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616061919061716f565b61031757604051637357d91f60e01b81526004810184905260240161364d565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5611bcf8461626b565b5f80608083901c156160f257608092831c92015b604083901c1561610457604092831c92015b602083901c1561611657602092831c92015b601083901c1561612857601092831c92015b600883901c1561613a57600892831c92015b600483901c1561614c57600492831c92015b600283901c1561615e57600292831c92015b600183901c156103285760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b5f4282036161a357506020820151610328565b5f6161b2846040015184616306565b90506161cb846020015182613cc490919063ffffffff16565b915050610328565b5f5f5f845f01518460ff16600281106161ee576161ee617106565b60200201516040015190505f616224875f01518660ff166002811061621557616215617106565b60200201518860800151616190565b9050811561623b576162368282613cc4565b61623d565b5f5b865190935060ff86166002811061625657616256617106565b60200201516020015193505050935093915050565b80518051515f91829161627f916001612422565b9050806040516020016162b890602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b604051602081830303815290604052805190602001206040516020016162e8929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f8061631283426171a2565b61631c90856174a6565b6301e133809004905061035681676765c793fa10079d601b1b617428565b6040518060a0016040528061634d6166ac565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101200160405280616382616713565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101a001604052806163d561678e565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280616453616815565b81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518061040001604052806164c8616686565b81526020016164d561633a565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101c001604052806165a561633a565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016165fa6163c1565b905290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806040016040528061667961633a565b81526020016165fa616440565b604051806060016040528061669961686b565b81525f6020820181905260409091015290565b60405180604001604052806002905b6166fd6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166bb5790505090565b60405180604001604052806002905b6167786040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167225790505090565b60405180604001604052806002905b6167ff6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161679d5790505090565b60405180604001604052806002905b61685560405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816168245790505090565b60405180604001604052806002905b6168c36040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161687a5790505090565b6001600160a01b038116811461487b575f5ffd5b5f602082840312156168fd575f5ffd5b8135610317816168d9565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557868503603f1901845281518051606080885260a08801919088015f5b60028110156169e857898403605f19018252825180516001600160a01b031685526020808201516080918701829052906169bc90870182616908565b604083810151908801526060928301519290960191909152506020928301929190910190600101616980565b5050506020828101518882015260409283015192909701919091529493840193919091019060010161695c565b50929695505050505050565b5f5f5f5f60808587031215616a34575f5ffd5b8435616a3f816168d9565b93506020850135616a4f816168d9565b92506040850135616a5f816168d9565b9396929550929360600135925050565b5f5f5f60608486031215616a81575f5ffd5b8335616a8c816168d9565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616ade57616ade616aa1565b604052919050565b5f67ffffffffffffffff821115616aff57616aff616aa1565b5060051b60200190565b5f5f60408385031215616b1a575f5ffd5b8235616b25816168d9565b9150602083013567ffffffffffffffff811115616b40575f5ffd5b8301601f81018513616b50575f5ffd5b8035616b63616b5e82616ae6565b616ab5565b8082825260208201915060208360051b850101925087831115616b84575f5ffd5b6020840193505b82841015616ba6578335825260209384019390910190616b8b565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015616c985783518051845f5b6002811015616c4757825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e082019150602083019250600181019050616bdc565b5050506020818101516001600160a01b039081166101c08701526040830151166101e08601526060820151610200860152608090910151610220850152939093019261024090920191600101616bcd565b509095945050505050565b5f8260408101835f5b6002811015616c98578383038752815180516001600160a01b0316845260208101516101406020860152616ce4610140860182616908565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616cac565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557603f19878603018452815180516101208752616da4610120880182616ca3565b9050602082015160208801526040820151616dca60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616d7c565b5f8260408101835f5b6002811015616c98578383038752815180516001600160a01b0316845260208101516101806020860152616e5d610180860182616908565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616e25565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557603f19878603018452815180516101a08752616f356101a0880182616e1c565b90506020820151616f5160208901826001600160a01b03169052565b506040820151616f6c60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616faf60e08901826001600160a01b03169052565b50610100820151616fc561010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616f0d565b5f5f60408385031215617017575f5ffd5b8235617022816168d9565b91506020830135617032816168d9565b809150509250929050565b60ff8116811461487b575f5ffd5b5f5f5f5f5f60a0868803121561705f575f5ffd5b853561706a816168d9565b9450602086013561707a816168d9565b9350604086013561708a816168d9565b92506060860135915060808601356170a18161703d565b809150509295509295909350565b5f5f5f5f5f60a086880312156170c3575f5ffd5b85356170ce816168d9565b945060208601356170de816168d9565b935060408601356170ee816168d9565b94979396509394606081013594506080013592915050565b634e487b7160e01b5f52603260045260245ffd5b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f6020828403121561714d575f5ffd5b5051919050565b5f60208284031215617164575f5ffd5b8151610317816168d9565b5f6020828403121561717f575f5ffd5b81518015158114610317575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156103285761032861718e565b5f600160ff1b82016171c9576171c961718e565b505f0390565b5f602082840312156171df575f5ffd5b815167ffffffffffffffff8111156171f5575f5ffd5b8201601f81018413617205575f5ffd5b8051617213616b5e82616ae6565b8082825260208201915060208360051b850101925086831115617234575f5ffd5b6020840193505b82841015611c4757835182526020938401939091019061723b565b5f60208284031215617266575f5ffd5b815167ffffffffffffffff81111561727c575f5ffd5b8201601f8101841361728c575f5ffd5b805167ffffffffffffffff8111156172a6576172a6616aa1565b6172b9601f8201601f1916602001616ab5565b8181528560208385010111156172cd575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f602082840312156172fa575f5ffd5b81516103178161703d565b6001815b6001841115617340578085048111156173245761732461718e565b600184161561733257908102905b60019390931c928002617309565b935093915050565b5f8261735657506001610328565b8161736257505f610328565b816001811461737857600281146173825761739e565b6001915050610328565b60ff8411156173935761739361718e565b50506001821b610328565b5060208310610133831016604e8410600b84101617156173c1575081810a610328565b6173cd5f198484617305565b805f19048211156173e0576173e061718e565b029392505050565b5f6103178383617348565b606081525f6174056060830186616908565b82810360208401526174178186616908565b915050826040830152949350505050565b808201808211156103285761032861718e565b634e487b7160e01b5f52601260045260245ffd5b8181035f831280158383131683831282161715613d8657613d8661718e565b606081525f6174806060830186616908565b82810360208401526174928186616908565b90508281036040840152611c478185616908565b80820281158282048414176103285761032861718e565b5f826174d757634e487b7160e01b5f52601260045260245ffd5b50049056fea264697066735822122083042749d65c56f0e77a6750b765d1de562e40c1c34bd08ffadb1f5491b5d6e464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pau\x12\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80c\\9\xF4g\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02\x84W\x80c\xD2\x8B\n\x15\x14a\x02\xB2W\x80c\xE35\xAD\xB7\x14a\x02\xC5W\x80c\xEE\xD0t(\x14a\x02\xD8W\x80c\xF6\x8Aq1\x14a\x02\xEBW__\xFD[\x80c\\9\xF4g\x14a\x02+W\x80cs\x91\x18\xA4\x14a\x02KW\x80cx\xF2\x12\xD1\x14a\x02^W\x80c\x8Flz<\x14a\x02qW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cR_V\x0C\x14a\x01\xE5W\x80cW\xB9\x1C\xA6\x14a\x02\x05W\x80cZoW\x10\x14a\x02\x18W__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04ah\xEDV[a\x02\xFEV[`@Qa\x015\x91\x90ai6V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04ah\xEDV[a\x03\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04aj!V[a\x03.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ajoV[a\x03IV[a\x01\xB7a\x01\xB26`\x04ak\tV[a\x03^V[`@Qa\x015\x91\x90ak\xB4V[a\x01\xD7a\x01\xD26`\x04ah\xEDV[a\x04\x17V[`@Q\x90\x81R` \x01a\x015V[a\x01\xF8a\x01\xF36`\x04ak\tV[a\x04!V[`@Qa\x015\x91\x90amVV[a\x01\xD7a\x02\x136`\x04ah\xEDV[a\x04-V[a\x01\xD7a\x02&6`\x04ah\xEDV[a\x047V[a\x02>a\x0296`\x04ah\xEDV[a\x04AV[`@Qa\x015\x91\x90an\xE7V[a\x01\xF8a\x02Y6`\x04ap\x06V[a\x04ZV[a\x01Qa\x02l6`\x04ah\xEDV[a\x04uV[a\x02>a\x02\x7F6`\x04ajoV[a\x04\x7FV[a\x02\x97a\x02\x926`\x04apKV[a\x04\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x97a\x02\xC06`\x04apKV[a\x04\xAEV[a\x01Qa\x02\xD36`\x04ah\xEDV[a\x04\xBEV[a\x01(a\x02\xE66`\x04ak\tV[a\x04\xC8V[a\x01\xD7a\x02\xF96`\x04ap\xAFV[a\x04\xD4V[``_a\x03\n\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x05\x80V[\x93\x92PPPV[_a\x03(\x82a\x06\x02V[\x92\x91PPV[__a\x03<\x86\x86\x86\x86a\x06\xB3V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03V\x84\x84\x84a\x05\x80V[\x94\x93PPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{aj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xB4W\x81` \x01[a\x03\xA1ac:V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_a\x03\xE6\x86\x86\x84\x81Q\x81\x10a\x03\xD9Wa\x03\xD9aq\x06V[` \x02` \x01\x01Qa\x06\xEFV[\x90P\x80\x83\x83\x81Q\x81\x10a\x03\xFBWa\x03\xFBaq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x03\xB9V[P\x93\x92PPPV[_a\x03(\x82a\x19gV[``a\x03\x17\x83\x83a\x19\xB8V[_a\x03(\x82a\x1AnV[_a\x03(\x82a\x04\xEEV[``_a\x04M\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x1A\xBFV[``_a\x04g\x84\x84a\x1B\xB6V[\x90Pa\x03V\x84\x84_\x84a\x1C,V[_a\x03(\x82a\x1CQV[``a\x03V\x84\x84\x84a\x1A\xBFV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1C\x8DV[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1D\x97V[_a\x03(\x82a\x1E\x10V[``a\x03\x17\x83\x83a\x1EaV[_a\x04\xE2\x86\x86\x86\x86\x86a\x1FGV[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x05\r\x90aq\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90aq=V[``a\x05\xB7`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x1C\xDD\x18\\\x9D`j\x1B\x81RP\x84a\x1FzV[a\x05\xEA`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x19[\x99`z\x1B\x81RP\x83a\x1FzV[_a\x05\xF6\x85\x85\x85a\x1F\xA7V[\x90Pa\x04\xE5\x85\x82a HV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90aqTV[___a\x06\xC0\x86\x86a \xFEV[\x90P_a\x06\xCD\x88\x83a\x06\xEFV[\x90P__a\x06\xDC\x8A\x84\x89a!\xA5V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[a\x06\xF7ac:V[\x82a\x07\0ac:V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x07\x1E\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90aqoV[a\x07\xA3W\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x07\xE3\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08G\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x86\x91\x90aqTV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tw\x91\x90aq=V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\t\xCD\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\np\x91\x90aq=V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\n\xD1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x01\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BPW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bt\x91\x90aq=V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xDF\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x0F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x82\x91\x90aq=V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xE3\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rG\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rbW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x86\x91\x90aq=V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ERW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ev\x91\x90aq=V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90aqTV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10G\x91\x90aq=V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10\x9E\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11A\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\xA3\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12F\x91\x90aq=V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB2\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x131W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13U\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xB7\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x146W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14Z\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xB3\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xE3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x17\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x152W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15V\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x15\xA4\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16G\x91\x90aqTV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xB5\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x174W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17X\x91\x90aqTV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\xBB\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18^\x91\x90aq=V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x18\xB7\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x196W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19Z\x91\x90aq=V[`\x80\x82\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xD5Wa\x19\xD5aj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x0EW\x81` \x01[a\x19\xFBacnV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\xF3W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1A/Wa\x1A/aq\x06V[` \x02` \x01\x01Q\x90P_a\x1AD\x87\x83a#\xFAV[\x90P\x80\x84\x84\x81Q\x81\x10a\x1AYWa\x1AYaq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1A\x13V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x1A\xCD\x85\x85\x85a\x1F\xA7V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xEAWa\x1A\xEAaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B#W\x81` \x01[a\x1B\x10ac\xC1V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\x08W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x1B\xACW_\x83\x82\x81Q\x81\x10a\x1BDWa\x1BDaq\x06V[` \x02` \x01\x01Q\x90Pa\x1Bw`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RP\x82a*\tV[_a\x1B\x82\x89\x83a*:V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1B\x97Wa\x1B\x97aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1B(V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x1B\xCF\x84a.\xDAV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x17\x91\x90aq=V[``_a\x1C;\x86\x86\x86\x86a/^V[\x90Pa\x1CG\x86\x82a\x19\xB8V[\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x1C\x9B\x88\x88a \xFEV[\x90P_a\x1C\xA8\x8A\x83a\x06\xEFV[\x90P_\x80\x80`\xFF\x89\x16a\x1C\xDEWa\x1C\xC0\x8D\x8B\x86a/\xE6V[\x92\x95P\x91\x93Pa\x1C\xD7\x91P\x85\x90P_\x85\x8D\x82a1\0V[\x90Pa\x1D\x0EV[_\x19`\xFF\x8A\x16\x01a\x1D\x0EWa\x1C\xF4\x8D\x8B\x86a1\xE2V[\x92\x95P\x91\x93Pa\x1D\x0B\x91P\x85\x90P\x84_\x80\x8Ea1\0V[\x90P[\x80_\x03a\x1D(W___\x97P\x97P\x97PPPPPPa\x04\xA3V[_a\x1D2\x85a2\xE4V[\x90P_\x82\x82\x11a\x1DKWa\x1DF\x82\x84aq\xA2V[a\x1DUV[a\x1DU\x83\x83aq\xA2V[\x90P_a\x1Db\x82\x84a3\x93V[\x90P_\x84\x84\x11a\x1DzWa\x1Du\x82aq\xB5V[a\x1D|V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x1D\xA5\x88\x88a \xFEV[\x90P_a\x1D\xB2\x8A\x83a\x06\xEFV[\x90P_\x80\x80`\xFF\x89\x16a\x1D\xE2Wa\x1D\xCB\x8D\x8B\x86_a3\xCEV[\x92\x95P\x91\x93Pa\x1C\xD7\x91P\x85\x90P\x8B_\x80\x87a1\0V[_\x19`\xFF\x8A\x16\x01a\x1D\x0EWa\x1D\xF9\x8D\x8B\x86_a4\xF8V[\x92\x95P\x91\x93Pa\x1D\x0B\x91P\x85\x90P_\x8C\x86\x82a1\0V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``a\x1E\x93`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l3\xB2\xBA(7\xB7\xB69\xA4\xB737\x99`\x99\x1B\x81RP\x83Qa\x1FzV[_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAEWa\x1E\xAEaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xE7W\x81` \x01[a\x1E\xD4ad@V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1F\x08Wa\x1F\x08aq\x06V[` \x02` \x01\x01Q\x90P_a\x1F\x1D\x87\x83a6\x07V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1F2Wa\x1F2aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1E\xECV[__a\x1FS\x86\x86a \xFEV[\x90P_a\x1F`\x88\x83a\x06\xEFV[\x90Pa\x1Fn\x81\x86\x86_a8\xC7V[\x98\x97PPPPPPPPV[a\x1F\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83a:!V[PPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1F\xC7\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a !W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03V\x91\x90\x81\x01\x90aq\xCFV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a eWa eaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x9EW\x81` \x01[a \x8Bad@V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x83W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a \xBFWa \xBFaq\x06V[` \x02` \x01\x01Q\x90P_a \xD4\x87\x83a6\x07V[\x90P\x80\x84\x84\x81Q\x81\x10a \xE9Wa \xE9aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a \xA3V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a!\x1FW\x81\x83a!\"V[\x82\x82[`@Q\x91\x94P\x92Pa!O\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[____a!\xB1adeV[a!\xBA\x88a:mV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"(\x91\x90aq=V[\x81` \x01\x81\x81RPPa\"B\x87__\x84a\x01@\x01Qa:\xBEV[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"d\x90\x88\x90`\x01\x90_\x90a:\xBEV[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"\x8DW____\x94P\x94P\x94P\x94PPa#\xF1V[a\"\xA0\x86\x82`@\x01Q\x83` \x01Qa;\xA3V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"\xBB\x91\x88\x91a;\xA3V[\x81a\x01 \x01\x81\x81RPPa\"\xFB`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa\x1FzV[a#3`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa\x1FzV[a#k`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa\x1FzV[a#\x9E`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa\x1FzV[a#\xD1`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa\x1FzV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[a$\x02acnV[a$\nad\xB4V[a$\x14\x84\x84a<bV[\x80\x82RQ\x80QQa$-\x91`\x01[` \x02\x01QQa \xFEV[`@\x82\x01\x81\x90Ra$?\x90\x85\x90a\x06\xEFV[` \x82\x01\x81\x90R\x81Qa$S\x91\x86\x91a<tV[PPPP``\x82\x01R` \x81\x01Qa$j\x90a2\xE4V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa$\xA0\x91\x90_[` \x02\x01Q`@\x01Q\x90a<\xC4V[`\xC0\x82\x01R` \x81\x01Q``\x01Qa$\xB8\x90_a=\x05V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa$\xD1\x91\x90a=3V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa$\xE8\x91\x90a=TV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa%\t\x92\x91\x90a=pV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%*\x92\x87\x92\x90\x91_a=\x8DV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa%W\x92\x91\x90a=pV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa%q\x90`\x01a=\x05V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa%\xA2\x91\x90`\x01a$\x91V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa%\xB9\x91a=3V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa%\xD1\x91\x90a=TV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa%\xE9\x91\x90a<\xC4V[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa&\x0C\x92\x91\x90a=pV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa&.\x92\x87\x92\x90\x91`\x01a=\x8DV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa&]\x92\x91\x90a=pV[a\x02\xE0\x82\x01R\x80Qa&n\x90a?\"V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a'fWa&\x98\x81a\x03\0\x01Q\x82`\x80\x01Qa=3V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa&\xAF\x91a<\xC4V[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa&\xE0\x93a?dV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa'G\x91\x86\x91a'\x08\x91\x90a=TV[a'\x1A\x84`\xC0\x01Q\x85`\xA0\x01Qa=TV[a'.\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa=TV[a'B\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa=TV[a?~V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa'_\x91\x90a@>V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\xDAW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x01\x91\x90\x81\x01\x90arVV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a(VWa(Vaq\x06V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a(\xA5Wa(\xA5aq\x06V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x05W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra),\x91\x90\x81\x01\x90arVV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[a\x1F\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83a*5\x84a@\x8DV[a@\xB9V[a*Bac\xC1V[a*Jae\x91V[a*T\x84\x84a\x06\xEFV[\x81Ra*_\x84aA\0V[` \x82\x01Ra*m\x84a:mV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa*\x88\x92_\x91\x90a:\xBEV[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa*\xB6\x92\x91`\x01\x91a:\xBEV[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa*\xD8\x90aACV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+WW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+~\x91\x90\x81\x01\x90arVV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xEF\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xCFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xF6\x91\x90\x81\x01\x90arVV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-j\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a.J\x90a2\xE4V[\x81R` \x01a.Y\x86\x86aCRV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a.\x80\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a.\x91\x86\x86aDBV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a.\xB5\x90\x87\x90_\x80aEDV[\x81R` \x01a.\xC7\x86\x84_\x01QaF?V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a/\x14\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a/x\x86a.\xDAV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xBFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90aq\xCFV[____a/\xF2ae\xFFV[a/\xFB\x88a:mV[a\x01\xC0\x82\x01\x81\x90Ra0\x12\x90\x87\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa01\x90\x87\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a0MWP` \x81\x01Q\x15[\x15a0dW____\x94P\x94P\x94P\x94PPa#\xF1V[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra0\x93\x90\x88\x90a0\x8B\x90a'\x10\x90aFJV[a'\x10a;\xA3V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\xB6\x92a0\xB1\x90\x83\x90aFJV[a;\xA3V[`\x80\x82\x01\x81\x90R` \x82\x01Qa0\xCC\x91\x90aFJV[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0\xEE\x90\x8B\x90aF\x9FV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a1\x10WP\x83\x15[\x15a1\x1FWP\x83\x90P\x84a1TV[_\x87\x11\x80\x15a1,WP\x84\x15[\x15a1;WP\x85\x90P\x82a1TV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a1c\x89``\x01Q_a=\x05V[\x90P_a1u\x8A``\x01Q`\x01a=\x05V[\x90P_a1\x93\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a1\xB1\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P\x80_\x03a1\xC8W_\x96PPPPPPPa\x04\xE5V[a1\xD2\x82\x82a3\x93V[\x9C\x9BPPPPPPPPPPPPV[____a1\xEEae\xFFV[a1\xF7\x88a:mV[a\x01\xC0\x82\x01\x81\x90Ra2\x0E\x90\x87\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa2-\x90\x87\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a2IWP` \x81\x01Q\x15[\x15a2`W____\x94P\x94P\x94P\x94PPa#\xF1V[\x80Q` \x82\x01Qa2v\x91\x90a0\xB1\x81\x8BaFJV[a\x01\0\x82\x01\x81\x90R\x81Qa2\x8A\x91\x90aFJV[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa2\xBD\x91a'\x10\x90a0\xB1\x90\x82\x90aFJV[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0\xEE\x91aF\x9FV[__a2\xF2\x83___a:\xBEV[PPP\x90P_a3\x05\x84`\x01__a:\xBEV[PPP\x90P\x80_\x03a3\x1AWP_\x93\x92PPPV[_a3)\x85``\x01Q_a=\x05V[\x90P_a3;\x86``\x01Q`\x01a=\x05V[\x90P_a3Y\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a3w\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P\x80_\x03a3\x8DWP_\x97\x96PPPPPPPV[a\x1Fn\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a3\xB4W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a3\xDAae\xFFV[a3\xE3\x89a:mV[a\x01\xC0\x82\x01\x81\x90Ra3\xFA\x90\x88\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4\x19\x90\x88\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a45WP` \x81\x01Q\x15[\x15a4LW____\x94P\x94P\x94P\x94PPa4\xEDV[\x85\x15a4gW\x87\x81_\x01\x81\x81Qa4c\x91\x90aq\xA2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra4\x8E\x90\x89\x90a0\x8B\x90a'\x10\x90aFJV[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa4\xAC\x92a0\xB1\x90\x83\x90aF\xC3V[`\x80\x82\x01\x81\x90R` \x82\x01Qa4\xC1\x91aFJV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa4\xE3\x90\x8C\x90aF\x9FV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a5\x04ae\xFFV[a5\r\x89a:mV[a\x01\xC0\x82\x01\x81\x90Ra5$\x90\x88\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa5C\x90\x88\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a5_WP` \x81\x01Q\x15[\x15a5vW____\x94P\x94P\x94P\x94PPa4\xEDV[\x85\x15a5\x92W\x87\x81` \x01\x81\x81Qa5\x8E\x91\x90aq\xA2V[\x90RP[\x80Q` \x82\x01Qa5\xA8\x91\x90a0\xB1\x81\x8CaF\xC3V[`\x80\x82\x01\x81\x90R\x81Qa5\xBA\x91aFJV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa5\xE9\x91a0\x8B\x90a'\x10\x90aFJV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa4\xE3\x91aF\x9FV[a6\x0Fad@V[a6\x17affV[a6!\x84\x84a\x06\xEFV[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a6VW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a6\xC7W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xEE\x91\x90\x81\x01\x90arVV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7_\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\xDFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra8\x06\x91\x90\x81\x01\x90arVV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8z\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a8\xB5\x86\x84_\x01QaF?V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a8\xD0adeV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a94\x91\x90aq=V[` \x82\x01Ra9E\x86_\x80\x80a:\xBEV[PPP`\xC0\x82\x01Ra9Z\x86`\x01_\x80a:\xBEV[PPP`\xE0\x82\x01R\x82\x15a9\x95W\x84\x81`\xC0\x01\x81\x81Qa9z\x91\x90aq\xA2V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a9\x91\x90\x83\x90aq\xA2V[\x90RP[`\xC0\x81\x01Q\x15\x80a9\xA8WP`\xE0\x81\x01Q\x15[\x15a9\xB6W_\x91PPa\x03VV[\x80` \x01Q_\x03a9\xE6Wa9\xDFa\x03\xE8a9\xD9a9\xD4\x88\x88aG\x17V[aG}V[\x90aFJV[\x81Ra:\x17V[a:\x14a9\xFC\x86\x83` \x01Q\x84`\xC0\x01Qa;\xA3V[a:\x0F\x86\x84` \x01Q\x85`\xE0\x01Qa;\xA3V[aH]V[\x81R[Q\x95\x94PPPPPV[a:h\x83\x83\x83`@Q`$\x01a:9\x93\x92\x91\x90as\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaHrV[PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a:\xDBWa:\xDBaq\x06V[` \x02\x01Q\x90P_a:\xED\x8A\x8AaH~V[\x90P\x80_\x03a;\tW____\x95P\x95P\x95P\x95PPPa4\xEDV[_a;\x18\x83\x8C`\x80\x01QaIkV[\x90P_a;%\x82\x8Aa<\xC4V[\x90P_\x89\x15a;JW\x81\x84\x10a;DWa;?\x84\x83aFJV[a;LV[_a;LV[_[\x90P_a;Y\x85\x8Da<\xC4V[\x90P_\x8C\x15a;~W\x84\x82\x10a;xWa;s\x82\x86aFJV[a;\x80V[_a;\x80V[_[\x90Pa;\x8C\x85\x87at(V[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a;\xD7W\x83\x82\x81a;\xCDWa;\xCDat;V[\x04\x92PPPa\x03\x17V[\x80\x84\x11a;\xF7W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a<jaf\x86V[a\x03\x17\x83\x83aI\x9BV[_____a<\x83\x88\x88aF?V[\x90Pa<\x91\x87\x87\x83_a[\xB9V[\x90\x93P\x91P\x81a<\xA2W_\x19a<\xACV[a<\xAC\x83\x83a3\x93V[\x94Pa<\xB7\x88a\x1AnV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a<\xE4W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=%WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a=JWa=E\x83\x83aq\xA2V[a\x03\x17V[a\x03\x17\x82\x84aq\xA2V[_a\x03\x17\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[_\x82\x84\x11a=\x86Wa=\x81\x82aq\xB5V[a\x03VV[P\x92\x91PPV[_a=\xC7`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a=\xD3\x86\x86\x86_a[\xB9V[` \x83\x01R\x80\x82R\x15\x80a>\x04WP\x84Q`\xFF\x84\x16`\x02\x81\x10a=\xF8Wa=\xF8aq\x06V[` \x02\x01Q` \x01Q_\x14[\x15a>\x12W_\x91PPa\x04\xE5V[a>\x1B\x87a]%V[`@\x82\x01\x81\x90R` \x82\x01Qa>0\x91a<\xC4V[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a>HW_\x91PPa\x04\xE5V[`\x80\x81\x01Q\x81Qa>Y\x91\x90aq\xA2V[\x81``\x01\x81\x81RPPa>p\x86``\x01Q\x84a=\x05V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa>\x9C\x91a>\x8B\x90`\nas\xE8V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xA3V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a>\xC1W`\xC0\x81\x01Qa>\xBB\x90\x85a3\x93V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a>\xD7Wa>\xD7aq\x06V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a?\x14W\x84Q`\xFF\x84\x16`\x02\x81\x10a?\x01Wa?\x01aq\x06V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a?BWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa?]WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x04\x0FWa?y\x82aq\xB5V[a\x04\xE5V[__a?\x89\x87a\x1AnV[\x90P_a?\x96\x82\x87a<\xC4V[\x90P_a?\xA3\x83\x86a<\xC4V[\x90P_a?\xB0\x89\x84atOV[\x90P_a?\xBD\x83\x89atOV[\x90P_a?\xC9\x83a]kV[\x90P_a?\xD5\x83a]kV[\x90P_\x84\x13\x80\x15a?\xE5WP_\x83\x12[\x80a?\xF9WP_\x84\x12\x80\x15a?\xF9WP_\x83\x13[\x15a@\rW_\x97PPPPPPPPa\x04\xE5V[\x80_\x03a@#W_\x97PPPPPPPPa\x04\xE5V[a@-\x82\x82a3\x93V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03a@MWP_a\x03(V[_\x82\x84\x11a@dWa@_\x84\x84aq\xA2V[a@nV[a@n\x83\x85aq\xA2V[\x90P_a@{\x82\x85a3\x93V[\x90P\x83\x85\x11a\x03VWa?y\x81aq\xB5V[``a\x03(\x82`@Q` \x01a@\xA5\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra]\x80V[a:h\x83\x83\x83`@Q`$\x01a@\xD1\x93\x92\x91\x90atnV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c,\xED|\xEF`\xE0\x1B\x17\x90RaHrV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__aAx`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\x82\x84_a_\x80V[` \x83\x01R\x81R``\x84\x01QaA\x98\x90_a=\x05V[``\x82\x01\x81\x90R\x81QaA\xBD\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a0\xB1\x90`\nas\xE8V[`@\x82\x01R` \x81\x01Q_\x03aA\xD8W_`\x80\x82\x01RaBxV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBNW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aBr\x91\x90aq=V[`\x80\x82\x01R[aB\x83\x84`\x01a_\x80V[` \x83\x01\x81\x90R\x90\x82R_\x03aB\x9EW_`\xA0\x82\x01RaC>V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC8\x91\x90aq=V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__aC^\x84\x84a_\xC6V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01aC\x9F\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90aqTV[__aDN\x84\x84a_\xC6V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01aD\xA1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90aq=V[__aEQ\x85`\x01aH~V[\x90P\x82\x15aEfWaEc\x84\x82aq\xA2V[\x90P[_aEp\x87a`\x81V[\x90P_aE}\x83\x83a<\xC4V[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10aE\xABW\x87Q` \x01Q``\x01QaE\xA6\x90\x83aq\xA2V[aE\xADV[_[\x90PaE\xDC`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a\x1FzV[aF\x0C`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a\x1FzV[a\x1Fn`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a\x1FzV[_a\x03\x17\x83\x83a`\xC5V[_\x82aFV\x83\x82aq\xA2V[\x91P\x81\x11\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a6MV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aF\xB5W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aF\xCF\x83\x82at(V[\x91P\x81\x10\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a6MV[_\x81\x15\x80aG:WP\x82\x82aG,\x81\x83at\xA6V[\x92PaG8\x90\x83at\xBDV[\x14[a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a6MV[_\x81_\x03aG\x8CWP_\x91\x90PV[_`\x01aG\x98\x84a`\xDEV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aG\xB1WaG\xB1at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xC9WaG\xC9at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xE1WaG\xE1at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xF9WaG\xF9at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\x11WaH\x11at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH)WaH)at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aHAWaHAat;V[\x04\x82\x01\x90\x1C\x90Pa\x03\x17\x81\x82\x85\x81aH[WaH[at;V[\x04[_\x81\x83\x10aHkW\x81a\x03\x17V[P\x90\x91\x90PV[aH{\x81aaqV[PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aH\x98WaH\x98aq\x06V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x15\x91\x90aq=V[\x90P\x80_\x03aI(W_\x92PPPa\x03(V[``\x82\x01Q`\xC0\x83\x01QaI<\x90\x82at(V[\x82\x10aI`W`\xC0\x83\x01QaIQ\x82\x84aq\xA2V[aI[\x91\x90aq\xA2V[a\x1CGV[_\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aI~WP_a\x03(V[_aI\x89\x84\x84aa\x90V[`\xA0\x85\x01Q\x90\x91Pa\x03V\x90\x82a<\xC4V[aI\xA3af\x86V[\x82aI\xACaf\x86V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aI\xEC\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJd\x91\x90aqoV[aJqW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ\xAB\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xDB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x0F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKN\x91\x90aq=V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aK\x96\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xC6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xFA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL9\x91\x90aqTV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aL\x95\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xF9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM8\x91\x90aqTV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN&\x91\x90aq=V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aNz\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\x1D\x91\x90aq=V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOw\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\x1A\x91\x90aq=V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPs\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x16\x91\x90aq=V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x0F\x91\x90aq=V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aRi\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR\x99\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xCD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x0C\x91\x90aq=V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xF2\x91\x90aq=V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xD9\x91\x90aqTV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aU\x80\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xBF\x91\x90aq=V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\x14\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aVD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aV\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\xB7\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aWB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xB5\x91\x90aq=V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\x0F\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX?\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXs\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\xB2\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\x14\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aYD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\xB7\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZ\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\xB5\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\x04\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\xA7\x91\x90aq=V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\\\\W__a[\xE4\x8A\x8A_aa\xD3V[\x91P\x91P_a\\\0_\x8C``\x01Qa=\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\x1E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[\x90P_a\\<\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90Pa\\H\x82\x88at(V[\x96Pa\\T\x81\x87at(V[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]\x18W__a\\\x83\x8A\x8A`\x01aa\xD3V[\x91P\x91P_a\\\xA0`\x01\x8C``\x01Qa=\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\xBE\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[\x90P_a\\\xDC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a\\\xE9\x83\x8Da<\xC4V[\x90P_a\\\xF6\x83\x8Ea<\xC4V[\x90Pa]\x02\x82\x8Aat(V[\x98Pa]\x0E\x81\x89at(V[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a]|W\x81_\x03a\x03(V[P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90_\x90a]\xBA\x90`\x02at\xA6V[a]\xC5\x90`\x02at(V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDDWa]\xDDaj\xA1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a^\x07W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a^!Wa^!aq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a^OWa^Oaq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x84Q\x81\x10\x15a\x04\x0FW\x82`\x04\x86\x83\x81Q\x81\x10a^\x85Wa^\x85aq\x06V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a^\xADWa^\xADaq\x06V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a^\xC8\x83`\x02at\xA6V[a^\xD3\x90`\x02at(V[\x81Q\x81\x10a^\xE3Wa^\xE3aq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a_\x0CWa_\x0Caq\x06V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a_,Wa_,aq\x06V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a_G\x83`\x02at\xA6V[a_R\x90`\x03at(V[\x81Q\x81\x10a_bWa_baq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a^gV[___a_\xAD\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a_\x9EWa_\x9Eaq\x06V[` \x02\x01Q\x86`\x80\x01QaIkV[\x90P_a_\xBA\x86\x86aH~V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a_\xE9\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`a\x91\x90aqoV[a\x03\x17W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a6MV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x1B\xCF\x84abkV[_\x80`\x80\x83\x90\x1C\x15a`\xF2W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aa\x04W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aa\x16W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aa(W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aa:W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aaLW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aa^W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03(W`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[_B\x82\x03aa\xA3WP` \x82\x01Qa\x03(V[_aa\xB2\x84`@\x01Q\x84ac\x06V[\x90Paa\xCB\x84` \x01Q\x82a<\xC4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03(V[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aa\xEEWaa\xEEaq\x06V[` \x02\x01Q`@\x01Q\x90P_ab$\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10ab\x15Wab\x15aq\x06V[` \x02\x01Q\x88`\x80\x01Qaa\x90V[\x90P\x81\x15ab;Wab6\x82\x82a<\xC4V[ab=V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10abVWabVaq\x06V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[\x80Q\x80QQ_\x91\x82\x91ab\x7F\x91`\x01a$\"V[\x90P\x80`@Q` \x01ab\xB8\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[_\x80ac\x12\x83Baq\xA2V[ac\x1C\x90\x85at\xA6V[c\x01\xE13\x80\x90\x04\x90Pa\x03V\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bat(V[`@Q\x80`\xA0\x01`@R\x80acMaf\xACV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01 \x01`@R\x80ac\x82ag\x13V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xA0\x01`@R\x80ac\xD5ag\x8EV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80adSah\x15V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x04\0\x01`@R\x80ad\xC8af\x86V[\x81R` \x01ad\xD5ac:V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ae\xA5ac:V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01ae\xFAac\xC1V[\x90R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80afyac:V[\x81R` \x01ae\xFAad@V[`@Q\x80``\x01`@R\x80af\x99ahkV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xFD`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81af\xBBW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[agx`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\"W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag\xFF`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\x9DW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ahU`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ah$W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ah\xC3`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ahzW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aH{W__\xFD[_` \x82\x84\x03\x12\x15ah\xFDW__\xFD[\x815a\x03\x17\x81ah\xD9V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15ai\xE8W\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ai\xBC\x90\x87\x01\x82ai\x08V[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ai\x80V[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ai\\V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15aj4W__\xFD[\x845aj?\x81ah\xD9V[\x93P` \x85\x015ajO\x81ah\xD9V[\x92P`@\x85\x015aj_\x81ah\xD9V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15aj\x81W__\xFD[\x835aj\x8C\x81ah\xD9V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aj\xDEWaj\xDEaj\xA1V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aj\xFFWaj\xFFaj\xA1V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ak\x1AW__\xFD[\x825ak%\x81ah\xD9V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ak@W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13akPW__\xFD[\x805akcak^\x82aj\xE6V[aj\xB5V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ak\x84W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15ak\xA6W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ak\x8BV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15al\x98W\x83Q\x80Q\x84_[`\x02\x81\x10\x15alGW\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pak\xDCV[PPP` \x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x87\x01R`@\x83\x01Q\x16a\x01\xE0\x86\x01R``\x82\x01Qa\x02\0\x86\x01R`\x80\x90\x91\x01Qa\x02 \x85\x01R\x93\x90\x93\x01\x92a\x02@\x90\x92\x01\x91`\x01\x01ak\xCDV[P\x90\x95\x94PPPPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15al\x98W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ral\xE4a\x01@\x86\x01\x82ai\x08V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pal\xACV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ram\xA4a\x01 \x88\x01\x82al\xA3V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qam\xCA`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01am|V[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15al\x98W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Ran]a\x01\x80\x86\x01\x82ai\x08V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pan%V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Rao5a\x01\xA0\x88\x01\x82an\x1CV[\x90P` \x82\x01QaoQ` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qaol`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qao\xAF`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qao\xC5a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ao\rV[__`@\x83\x85\x03\x12\x15ap\x17W__\xFD[\x825ap\"\x81ah\xD9V[\x91P` \x83\x015ap2\x81ah\xD9V[\x80\x91PP\x92P\x92\x90PV[`\xFF\x81\x16\x81\x14aH{W__\xFD[_____`\xA0\x86\x88\x03\x12\x15ap_W__\xFD[\x855apj\x81ah\xD9V[\x94P` \x86\x015apz\x81ah\xD9V[\x93P`@\x86\x015ap\x8A\x81ah\xD9V[\x92P``\x86\x015\x91P`\x80\x86\x015ap\xA1\x81ap=V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15ap\xC3W__\xFD[\x855ap\xCE\x81ah\xD9V[\x94P` \x86\x015ap\xDE\x81ah\xD9V[\x93P`@\x86\x015ap\xEE\x81ah\xD9V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aqMW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aqdW__\xFD[\x81Qa\x03\x17\x81ah\xD9V[_` \x82\x84\x03\x12\x15aq\x7FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x17W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03(Wa\x03(aq\x8EV[_`\x01`\xFF\x1B\x82\x01aq\xC9Waq\xC9aq\x8EV[P_\x03\x90V[_` \x82\x84\x03\x12\x15aq\xDFW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\xF5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ar\x05W__\xFD[\x80Qar\x13ak^\x82aj\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15ar4W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1CGW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ar;V[_` \x82\x84\x03\x12\x15arfW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar|W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ar\x8CW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\xA6War\xA6aj\xA1V[ar\xB9`\x1F\x82\x01`\x1F\x19\x16` \x01aj\xB5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ar\xCDW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ar\xFAW__\xFD[\x81Qa\x03\x17\x81ap=V[`\x01\x81[`\x01\x84\x11\x15as@W\x80\x85\x04\x81\x11\x15as$Was$aq\x8EV[`\x01\x84\x16\x15as2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02as\tV[\x93P\x93\x91PPV[_\x82asVWP`\x01a\x03(V[\x81asbWP_a\x03(V[\x81`\x01\x81\x14asxW`\x02\x81\x14as\x82Was\x9EV[`\x01\x91PPa\x03(V[`\xFF\x84\x11\x15as\x93Was\x93aq\x8EV[PP`\x01\x82\x1Ba\x03(V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15as\xC1WP\x81\x81\na\x03(V[as\xCD_\x19\x84\x84as\x05V[\x80_\x19\x04\x82\x11\x15as\xE0Was\xE0aq\x8EV[\x02\x93\x92PPPV[_a\x03\x17\x83\x83asHV[``\x81R_at\x05``\x83\x01\x86ai\x08V[\x82\x81\x03` \x84\x01Rat\x17\x81\x86ai\x08V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03(Wa\x03(aq\x8EV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a=\x86Wa=\x86aq\x8EV[``\x81R_at\x80``\x83\x01\x86ai\x08V[\x82\x81\x03` \x84\x01Rat\x92\x81\x86ai\x08V[\x90P\x82\x81\x03`@\x84\x01Ra\x1CG\x81\x85ai\x08V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03(Wa\x03(aq\x8EV[_\x82at\xD7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x83\x04'I\xD6\\V\xF0\xE7zgP\xB7e\xD1\xDEV.@\xC1\xC3K\xD0\x8F\xFA\xDB\x1FT\x91\xB5\xD6\xE4dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610111575f3560e01c80635c39f4671161009e578063c2bdeda11161006e578063c2bdeda114610284578063d28b0a15146102b2578063e335adb7146102c5578063eed07428146102d8578063f68a7131146102eb575f5ffd5b80635c39f4671461022b578063739118a41461024b57806378f212d11461025e5780638f6c7a3c14610271575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c4578063525f560c146101e557806357b91ca6146102055780635a6f571014610218575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b6101286101233660046168ed565b6102fe565b6040516101359190616936565b60405180910390f35b61015161014c3660046168ed565b61031e565b6040516001600160a01b039091168152602001610135565b61017c610177366004616a21565b61032e565b60408051928352602083019190915201610135565b61012861019f366004616a6f565b610349565b6101b76101b2366004616b09565b61035e565b6040516101359190616bb4565b6101d76101d23660046168ed565b610417565b604051908152602001610135565b6101f86101f3366004616b09565b610421565b6040516101359190616d56565b6101d76102133660046168ed565b61042d565b6101d76102263660046168ed565b610437565b61023e6102393660046168ed565b610441565b6040516101359190616ee7565b6101f8610259366004617006565b61045a565b61015161026c3660046168ed565b610475565b61023e61027f366004616a6f565b61047f565b61029761029236600461704b565b61048c565b60408051938452602084019290925290820152606001610135565b6102976102c036600461704b565b6104ae565b6101516102d33660046168ed565b6104be565b6101286102e6366004616b09565b6104c8565b6101d76102f93660046170af565b6104d4565b60605f61030a836104ee565b9050610317835f83610580565b9392505050565b5f61032882610602565b92915050565b5f5f61033c868686866106b3565b9150915094509492505050565b6060610356848484610580565b949350505050565b60605f825167ffffffffffffffff81111561037b5761037b616aa1565b6040519080825280602002602001820160405280156103b457816020015b6103a161633a565b8152602001906001900390816103995790505b5090505f5b835181101561040f575f6103e6868684815181106103d9576103d9617106565b60200260200101516106ef565b9050808383815181106103fb576103fb617106565b6020908102919091010152506001016103b9565b509392505050565b5f61032882611967565b606061031783836119b8565b5f61032882611a6e565b5f610328826104ee565b60605f61044d836104ee565b9050610317835f83611abf565b60605f6104678484611bb6565b905061035684845f84611c2c565b5f61032882611c51565b6060610356848484611abf565b5f5f5f61049c8888888888611c8d565b9250925092505b955095509592505050565b5f5f5f61049c8888888888611d97565b5f61032882611e10565b60606103178383611e61565b5f6104e28686868686611f47565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161050d9061711a565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161054191815260200190565b602060405180830381865afa15801561055c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610328919061713d565b60606105b76040518060400160405280601381526020017219d95d141bdbdb1cd25b999bcc081cdd185c9d606a1b81525084611f7a565b6105ea6040518060400160405280601181526020017019d95d141bdbdb1cd25b999bcc08195b99607a1b81525083611f7a565b5f6105f6858585611fa7565b90506104e58582612048565b5f816001600160a01b03166321f8a721604051602001610640906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161067491815260200190565b602060405180830381865afa15801561068f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190617154565b5f5f5f6106c086866120fe565b90505f6106cd88836106ef565b90505f5f6106dc8a84896121a5565b50919c909b509950505050505050505050565b6106f761633a565b8261070061633a565b816001600160a01b03166391d4403c60405160200161071e9061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015610772573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610796919061716f565b6107a35791506103289050565b816001600160a01b03166321f8a721856040516020016107e3906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001610813929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161084791815260200190565b602060405180830381865afa158015610862573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108869190617154565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610904929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161093891815260200190565b602060405180830381865afa158015610953573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610977919061713d565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016109cd906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016109fd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a3191815260200190565b602060405180830381865afa158015610a4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a70919061713d565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610ad19060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610b01929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b3591815260200190565b602060405180830381865afa158015610b50573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b74919061713d565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610bdf9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610c0f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c4391815260200190565b602060405180830381865afa158015610c5e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c82919061713d565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610ce39060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d13929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4791815260200190565b602060405180830381865afa158015610d62573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d86919061713d565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610e03929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e3791815260200190565b602060405180830381865afa158015610e52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e76919061713d565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001610eeb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f1f91815260200190565b602060405180830381865afa158015610f3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5e9190617154565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100891815260200190565b602060405180830381865afa158015611023573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611047919061713d565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161109e90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016110ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161110291815260200190565b602060405180830381865afa15801561111d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611141919061713d565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016111a39060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016111d3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161120791815260200190565b602060405180830381865afa158015611222573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611246919061713d565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016112b29060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016112e2929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131691815260200190565b602060405180830381865afa158015611331573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611355919061713d565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016113b79060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161141b91815260200190565b602060405180830381865afa158015611436573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061145a919061713d565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016114b390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016114e3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161151791815260200190565b602060405180830381865afa158015611532573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611556919061713d565b81516001602002015160c0018181525050816001600160a01b03166321f8a721856040516020016115a490602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115d4929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161160891815260200190565b602060405180830381865afa158015611623573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116479190617154565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016116b5906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116e5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171991815260200190565b602060405180830381865afa158015611734573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117589190617154565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016117bb906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016117eb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161181f91815260200190565b602060405180830381865afa15801561183a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185e919061713d565b60608201526040516001600160a01b0383169063bd02d0f59086906118b7906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118e7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161191b91815260200190565b602060405180830381865afa158015611936573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061195a919061713d565b6080820152949350505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b60605f825167ffffffffffffffff8111156119d5576119d5616aa1565b604051908082528060200260200182016040528015611a0e57816020015b6119fb61636e565b8152602001906001900390816119f35790505b5090505f5b835181101561040f575f848281518110611a2f57611a2f617106565b602002602001015190505f611a4487836123fa565b905080848481518110611a5957611a59617106565b60209081029190910101525050600101611a13565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f611acd858585611fa7565b90505f815167ffffffffffffffff811115611aea57611aea616aa1565b604051908082528060200260200182016040528015611b2357816020015b611b106163c1565b815260200190600190039081611b085790505b5090505f5b8251811015611bac575f838281518110611b4457611b44617106565b60200260200101519050611b7760405180604001604052806007815260200166706f6f6c4b657960c81b81525082612a09565b5f611b828983612a3a565b905080848481518110611b9757611b97617106565b60209081029190910101525050600101611b28565b5095945050505050565b5f826001600160a01b031663f3903b9f611bcf84612eda565b6040518263ffffffff1660e01b8152600401611bed91815260200190565b602060405180830381865afa158015611c08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610317919061713d565b60605f611c3b86868686612f5e565b9050611c4786826119b8565b9695505050505050565b5f816001600160a01b03166321f8a72160405160200161064090602080825260089082015267545245415355525960c01b604082015260600190565b5f5f5f5f611c9b88886120fe565b90505f611ca88a836106ef565b90505f808060ff8916611cde57611cc08d8b86612fe6565b929550919350611cd791508590505f858d82613100565b9050611d0e565b5f1960ff8a1601611d0e57611cf48d8b866131e2565b929550919350611d0b9150859050845f808e613100565b90505b805f03611d28575f5f5f97509750975050505050506104a3565b5f611d32856132e4565b90505f828211611d4b57611d4682846171a2565b611d55565b611d5583836171a2565b90505f611d628284613393565b90505f848411611d7a57611d75826171b5565b611d7c565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f611da588886120fe565b90505f611db28a836106ef565b90505f808060ff8916611de257611dcb8d8b865f6133ce565b929550919350611cd791508590508b5f8087613100565b5f1960ff8a1601611d0e57611df98d8b865f6134f8565b929550919350611d0b91508590505f8c8682613100565b5f816001600160a01b03166321f8a721604051602001610640906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b6060611e936040518060400160405280600d81526020016c33b2ba2837b7b639a4b733379960991b8152508351611f7a565b5f825167ffffffffffffffff811115611eae57611eae616aa1565b604051908082528060200260200182016040528015611ee757816020015b611ed4616440565b815260200190600190039081611ecc5790505b5090505f5b835181101561040f575f848281518110611f0857611f08617106565b602002602001015190505f611f1d8783613607565b905080848481518110611f3257611f32617106565b60209081029190910101525050600101611eec565b5f5f611f5386866120fe565b90505f611f6088836106ef565b9050611f6e8186865f6138c7565b98975050505050505050565b611fa3604051806040016040528060068152602001652573202d257360d01b8152508383613a21565b5050565b6060836001600160a01b031663f069052a604051602001611fc79061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015612021573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261035691908101906171cf565b60605f825167ffffffffffffffff81111561206557612065616aa1565b60405190808252806020026020018201604052801561209e57816020015b61208b616440565b8152602001906001900390816120835790505b5090505f5b835181101561040f575f8482815181106120bf576120bf617106565b602002602001015190505f6120d48783613607565b9050808484815181106120e9576120e9617106565b602090810291909101015250506001016120a3565b5f816001600160a01b0316836001600160a01b03161061211f578183612122565b82825b604051919450925061214f906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b5f5f5f5f6121b1616465565b6121ba88613a6d565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612204573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612228919061713d565b816020018181525050612242875f5f846101400151613abe565b5060808401525060408201526101408101516122649088906001905f90613abe565b5060a084015250606082015260208101515f0361228d575f5f5f5f9450945094509450506123f1565b6122a08682604001518360200151613ba3565b610100820152606081015160208201516122bb918891613ba3565b816101200181815250506122fb6040518060400160405280601081526020016f766172732e746f74616c537570706c7960801b8152508260200151611f7a565b612333604051806040016040528060128152602001710766172732e707269636552657365727665360741b8152508260400151611f7a565b61236b60405180604001604052806012815260200171766172732e7072696365526573657276653160701b8152508260600151611f7a565b61239e6040518060400160405280600c81526020016b0766172732e616d6f756e74360a41b815250826101000151611f7a565b6123d16040518060400160405280600c81526020016b766172732e616d6f756e743160a01b815250826101200151611f7a565b80610100015181610120015182608001518360a001519450945094509450505b93509350935093565b61240261636e565b61240a6164b4565b6124148484613c62565b8082525180515161242d9160015b6020020151516120fe565b6040820181905261243f9085906106ef565b602082018190528151612453918691613c74565b505050506060820152602081015161246a906132e4565b610300820152805180515160209081015160e0840152808301515151015190516124a091905f5b60200201516040015190613cc4565b60c08201526020810151606001516124b8905f613d05565b60a082015260e081015160c08201516124d19190613d33565b610100820181905260a08201516124e89190613d54565b61012082015260e081015160c0820151610100830151612509929190613d70565b6101408201526020810151815161030083015161252a92879290915f613d8d565b61016082015261014081015161018082015260e081015160c0820151610120830151612557929190613d70565b6101a0820152602081015160600151612571906001613d05565b6101c0820152805180516020908101518101516102008401528083015151810151015190516125a291906001612491565b6101e082018190526102008201516125b991613d33565b61022082018190526101c08201516125d19190613d54565b61024082018190526103008201516125e99190613cc4565b6102608201526102008101516101e082015161022083015161260c929190613d70565b6102808201526020810151815161030083015161262e92879290916001613d8d565b6102a08201526102808101516102c08201526102008101516101e082015161026083015161265d929190613d70565b6102e0820152805161266e90613f22565b60808201528051516020015160e00151600214612766576126988161030001518260800151613d33565b61032082018190526102408201516126af91613cc4565b610340820181905260808201516103008301511161038083018190526102008301516101e08401516126e093613f64565b61036082018190526103a082015260e081015160a08201516127479186916127089190613d54565b61271a8460c001518560a00151613d54565b61272e856102000151866101c00151613d54565b612742866101e00151876101c00151613d54565b613f7e565b6103c0820181905261030082015161275f919061403e565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa1580156127da573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526128019190810190617256565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff166002811061285657612856617106565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff16600281106128a5576128a5617106565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612905573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261292c9190810190617256565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b611fa3604051806040016040528060068152602001652573202d257360d01b81525083612a358461408d565b6140b9565b612a426163c1565b612a4a616591565b612a5484846106ef565b8152612a5f84614100565b6020820152612a6d84613a6d565b610180820181905281516020830151612a88925f9190613abe565b608085015260a08401526040830152606082015280516020820151610180830151612ab69291600191613abe565b61012085015261014084015260e08301526101008201528051612ad890614143565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa158015612b57573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612b7e9190810190617256565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa158015612bcb573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bef91906172ea565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015612ccf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612cf69190810190617256565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d6a91906172ea565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b908301528351910190612e4a906132e4565b8152602001612e598686614352565b6001600160a01b03168152602001612e80835f015160600151660800000000000016151590565b15158152602001612e918686614442565b81528251515160c0015160208201528251604090910190612eb59087905f80614544565b8152602001612ec786845f015161463f565b90526101a0909101819052905092915050565b5f604051602001612f14906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a612f7886612eda565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa158015612fbf573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104e591908101906171cf565b5f5f5f5f612ff26165ff565b612ffb88613a6d565b6101c082018190526130129087905f908190613abe565b5060408401525081526101c08101516130319087906001905f90613abe565b5060608401525060208201528051158061304d57506020810151155b15613064575f5f5f5f9450945094509450506123f1565b606086015160381c61ffff16610140820181905261309390889061308b906127109061464a565b612710613ba3565b6101808201819052815160208301516130b6926130b190839061464a565b613ba3565b6080820181905260208201516130cc919061464a565b60e08201819052604082015160608301516101408401516130ee908b9061469f565b94509450945094505093509350935093565b5f5f5f5f86118015613110575083155b1561311f575083905084613154565b5f8711801561312c575084155b1561313b575085905082613154565b604051636331fab160e01b815260040160405180910390fd5b5f61316389606001515f613d05565b90505f6131758a606001516001613d05565b90505f61319385676765c793fa10079d601b1b6130b186600a6173e8565b90505f6131b185676765c793fa10079d601b1b6130b186600a6173e8565b9050805f036131c8575f96505050505050506104e5565b6131d28282613393565b9c9b505050505050505050505050565b5f5f5f5f6131ee6165ff565b6131f788613a6d565b6101c0820181905261320e9087905f908190613abe565b5060408401525081526101c081015161322d9087906001905f90613abe565b5060608401525060208201528051158061324957506020810151155b15613260575f5f5f5f9450945094509450506123f1565b8051602082015161327691906130b1818b61464a565b6101008201819052815161328a919061464a565b610120820152606086015160381c61ffff1661014082018190526101208201516132bd91612710906130b190829061464a565b6101a08201819052604082015160608301516101408401516101208501516130ee9161469f565b5f5f6132f2835f5f5f613abe565b50505090505f6133058460015f5f613abe565b5050509050805f0361331a57505f9392505050565b5f61332985606001515f613d05565b90505f61333b86606001516001613d05565b90505f61335985676765c793fa10079d601b1b6130b186600a6173e8565b90505f61337785676765c793fa10079d601b1b6130b186600a6173e8565b9050805f0361338d57505f979650505050505050565b611f6e82825b5f8115676765c793fa10079d601b1b600284041904841117156133b4575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f6133da6165ff565b6133e389613a6d565b6101c082018190526133fa9088905f908190613abe565b5060408401525081526101c08101516134199088906001905f90613abe565b5060608401525060208201528051158061343557506020810151155b1561344c575f5f5f5f9450945094509450506134ed565b85156134675787815f0181815161346391906171a2565b9052505b606087015160381c61ffff16610140820181905261348e90899061308b906127109061464a565b6101608201819052815160208301516134ac926130b19083906146c3565b6080820181905260208201516134c19161464a565b60c08201819052604082015160608301516101408401516134e3908c9061469f565b9450945094509450505b945094509450949050565b5f5f5f5f6135046165ff565b61350d89613a6d565b6101c082018190526135249088905f908190613abe565b5060408401525081526101c08101516135439088906001905f90613abe565b5060608401525060208201528051158061355f57506020810151155b15613576575f5f5f5f9450945094509450506134ed565b851561359257878160200181815161358e91906171a2565b9052505b805160208201516135a891906130b1818c6146c3565b6080820181905281516135ba9161464a565b60a0820152606087015160381c61ffff16610140820181905260a08201516135e99161308b906127109061464a565b6040820151606083015161014084015160a08501516134e39161469f565b61360f616440565b613617616666565b61362184846106ef565b808252602001516001600160a01b031661365657604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156136c7573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136ee9190810190617256565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561373b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061375f91906172ea565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156137df573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526138069190810190617256565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613856573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061387a91906172ea565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016138b586845f015161463f565b90526020909101819052905092915050565b5f6138d0616465565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613910573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613934919061713d565b6020820152613945865f8080613abe565b50505060c082015261395a8660015f80613abe565b50505060e0820152821561399557848160c00181815161397a91906171a2565b90525060e0810180518591906139919083906171a2565b9052505b60c081015115806139a8575060e0810151155b156139b6575f915050610356565b80602001515f036139e6576139df6103e86139d96139d48888614717565b61477d565b9061464a565b8152613a17565b613a146139fc8683602001518460c00151613ba3565b613a0f8684602001518560e00151613ba3565b61485d565b81525b5195945050505050565b613a68838383604051602401613a39939291906173f3565b60408051601f198184030181529190526020810180516001600160e01b0316635821efa160e01b179052614872565b505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff1660028110613adb57613adb617106565b602002015190505f613aed8a8a61487e565b9050805f03613b09575f5f5f5f955095509550955050506134ed565b5f613b18838c6080015161496b565b90505f613b25828a613cc4565b90505f8915613b4a57818410613b4457613b3f848361464a565b613b4c565b5f613b4c565b5f5b90505f613b59858d613cc4565b90505f8c15613b7e57848210613b7857613b73828661464a565b613b80565b5f613b80565b5f5b9050613b8c8587617428565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613bd757838281613bcd57613bcd61743b565b0492505050610317565b808411613bf75760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b613c6a616686565b610317838361499b565b5f5f5f5f5f613c83888861463f565b9050613c918787835f615bb9565b909350915081613ca2575f19613cac565b613cac8383613393565b9450613cb788611a6e565b9350939792965093509350565b5f81156b019d971e4fe8401e740000001983900484111517613ce4575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff851601613d25575060ff60601b19905060605b90198416901c905092915050565b5f818311613d4a57613d4583836171a2565b610317565b61031782846171a2565b5f61031783676765c793fa10079d601b1b6130b185600a6173e8565b5f828411613d8657613d81826171b5565b610356565b5092915050565b5f613dc76040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613dd38686865f615bb9565b60208301528082521580613e045750845160ff841660028110613df857613df8617106565b6020020151602001515f145b15613e12575f9150506104e5565b613e1b87615d25565b604082018190526020820151613e3091613cc4565b6080820181905281511015613e48575f9150506104e5565b60808101518151613e5991906171a2565b816060018181525050613e70866060015184613d05565b60a082018190526060820151613e9c91613e8b90600a6173e8565b676765c793fa10079d601b1b613ba3565b60c08201525f1960ff841601613ec15760c0810151613ebb9085613393565b60c08201525b845160ff841660028110613ed757613ed7617106565b6020020151602001518160c001511115613f1457845160ff841660028110613f0157613f01617106565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901613f42575051602001516060015190565b81516020015160e00151613f5d575051602001516080015190565b505f919050565b5f8415158385111461040f57613f79826171b5565b6104e5565b5f5f613f8987611a6e565b90505f613f968287613cc4565b90505f613fa38386613cc4565b90505f613fb0898461744f565b90505f613fbd838961744f565b90505f613fc983615d6b565b90505f613fd583615d6b565b90505f84138015613fe557505f83125b80613ff957505f84128015613ff957505f83135b1561400d575f9750505050505050506104e5565b805f03614023575f9750505050505050506104e5565b61402d8282613393565b9d9c50505050505050505050505050565b5f815f0361404d57505f610328565b5f8284116140645761405f84846171a2565b61406e565b61406e83856171a2565b90505f61407b8285613393565b905083851161035657613f79816171b5565b6060610328826040516020016140a591815260200190565b604051602081830303815290604052615d80565b613a688383836040516024016140d19392919061746e565b60408051601f198184030181529190526020810180516001600160e01b0316632ced7cef60e01b179052614872565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f6141786040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614182845f615f80565b602083015281526060840151614198905f613d05565b6060820181905281516141bd91676765c793fa10079d601b1b906130b190600a6173e8565b604082015260208101515f036141d8575f6080820152614278565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa15801561424e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614272919061713d565b60808201525b614283846001615f80565b602083018190529082525f0361429e575f60a082015261433e565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614314573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614338919061713d565b60a08201525b80608001518160a001519250925050915091565b5f5f61435e8484615fc6565b9050806001600160a01b03166321f8a7218460405160200161439f906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016143cf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161440391815260200190565b602060405180830381865afa15801561441e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190617154565b5f5f61444e8484615fc6565b9050806001600160a01b031663bd02d0f5846040516020016144a19060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016144d1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161450591815260200190565b602060405180830381865afa158015614520573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610356919061713d565b5f5f61455185600161487e565b905082156145665761456384826171a2565b90505b5f61457087616081565b90505f61457d8383613cc4565b875160200151606001519091505f9082106145ab57875160200151606001516145a690836171a2565b6145ad565b5f5b90506145dc6040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525085611f7a565b61460c6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525084611f7a565b611f6e604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525083611f7a565b5f61031783836160c5565b5f8261465683826171a2565b91508111156103285760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161364d565b5f811561138819839004841115176146b5575f5ffd5b506127109102611388010490565b5f826146cf8382617428565b91508110156103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161364d565b5f81158061473a5750828261472c81836174a6565b925061473890836174bd565b145b6103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161364d565b5f815f0361478c57505f919050565b5f6001614798846160de565b901c6001901b905060018184816147b1576147b161743b565b048201901c905060018184816147c9576147c961743b565b048201901c905060018184816147e1576147e161743b565b048201901c905060018184816147f9576147f961743b565b048201901c905060018184816148115761481161743b565b048201901c905060018184816148295761482961743b565b048201901c905060018184816148415761484161743b565b048201901c90506103178182858161485b5761485b61743b565b045b5f81831061486b5781610317565b5090919050565b61487b81616171565b50565b5f5f835f01518360ff166002811061489857614898617106565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa1580156148f1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614915919061713d565b9050805f03614928575f92505050610328565b606082015160c083015161493c9082617428565b82106149605760c083015161495182846171a2565b61495b91906171a2565b611c47565b5f9695505050505050565b5f8260a001515f0361497e57505f610328565b5f6149898484616190565b60a08501519091506103569082613cc4565b6149a3616686565b826149ac616686565b816001600160a01b03166391d4403c6040516020016149ec906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614a40573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a64919061716f565b614a715791506103289050565b816001600160a01b031663bd02d0f585604051602001614aab906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614adb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614b0f91815260200190565b602060405180830381865afa158015614b2a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b4e919061713d565b816020018181525050816001600160a01b03166321f8a72185604051602001614b96906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bc6929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614bfa91815260200190565b602060405180830381865afa158015614c15573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c399190617154565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614c95906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614cc5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cf991815260200190565b602060405180830381865afa158015614d14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d389190617154565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614db3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614de791815260200190565b602060405180830381865afa158015614e02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e26919061713d565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614e7a9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614eaa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ede91815260200190565b602060405180830381865afa158015614ef9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614f1d919061713d565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001614f77906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001614fa7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fdb91815260200190565b602060405180830381865afa158015614ff6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061501a919061713d565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001615073906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b604051602081830303815290604052805190602001206040516020016150a3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150d791815260200190565b602060405180830381865afa1580156150f2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615116919061713d565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161519c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151d091815260200190565b602060405180830381865afa1580156151eb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061520f919061713d565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615269906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615299929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152cd91815260200190565b602060405180830381865afa1580156152e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061530c919061713d565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161537f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153b391815260200190565b602060405180830381865afa1580156153ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153f2919061713d565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615466929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161549a91815260200190565b602060405180830381865afa1580156154b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154d99190617154565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161558091815260200190565b602060405180830381865afa15801561559b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155bf919061713d565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156149060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615644929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161567891815260200190565b602060405180830381865afa158015615693573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156b7919061713d565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161571290602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615742929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161577691815260200190565b602060405180830381865afa158015615791573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157b5919061713d565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161580f90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b6040516020818303038152906040528051906020012060405160200161583f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161587391815260200190565b602060405180830381865afa15801561588e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906158b2919061713d565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016159149060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615944929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161597891815260200190565b602060405180830381865afa158015615993573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159b7919061713d565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615a1290602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615a42929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a7691815260200190565b602060405180830381865afa158015615a91573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615ab5919061713d565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615b04906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b34929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b6891815260200190565b602060405180830381865afa158015615b83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615ba7919061713d565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615c5c575f5f615be48a8a5f6161d3565b915091505f615c005f8c60600151613d0590919063ffffffff16565b90505f615c1e84676765c793fa10079d601b1b6130b185600a6173e8565b90505f615c3c84676765c793fa10079d601b1b6130b186600a6173e8565b9050615c488288617428565b9650615c548187617428565b955050505050505b865160200151516001600160a01b03868116911614615d18575f5f615c838a8a60016161d3565b915091505f615ca060018c60600151613d0590919063ffffffff16565b90505f615cbe84676765c793fa10079d601b1b6130b185600a6173e8565b90505f615cdc84676765c793fa10079d601b1b6130b186600a6173e8565b90505f615ce9838d613cc4565b90505f615cf6838e613cc4565b9050615d02828a617428565b9850615d0e8189617428565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615d7c57815f03610328565b5090565b60408051808201909152601081526f181899199a1a9b1b9c1cb0b131b232b360811b60208201528151606091905f90615dba9060026174a6565b615dc5906002617428565b67ffffffffffffffff811115615ddd57615ddd616aa1565b6040519080825280601f01601f191660200182016040528015615e07576020820181803683370190505b509050600360fc1b815f81518110615e2157615e21617106565b60200101906001600160f81b03191690815f1a905350600f60fb1b81600181518110615e4f57615e4f617106565b60200101906001600160f81b03191690815f1a9053505f5b845181101561040f57826004868381518110615e8557615e85617106565b016020015182516001600160f81b031990911690911c60f81c908110615ead57615ead617106565b01602001516001600160f81b03191682615ec88360026174a6565b615ed3906002617428565b81518110615ee357615ee3617106565b60200101906001600160f81b03191690815f1a90535082858281518110615f0c57615f0c617106565b602091010151815160f89190911c600f16908110615f2c57615f2c617106565b01602001516001600160f81b03191682615f478360026174a6565b615f52906003617428565b81518110615f6257615f62617106565b60200101906001600160f81b03191690815f1a905350600101615e67565b5f5f5f615fad855f01518560ff1660028110615f9e57615f9e617106565b6020020151866080015161496b565b90505f615fba868661487e565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c604051602001615fe99061711a565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa15801561603d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190616061919061716f565b61031757604051637357d91f60e01b81526004810184905260240161364d565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5611bcf8461626b565b5f80608083901c156160f257608092831c92015b604083901c1561610457604092831c92015b602083901c1561611657602092831c92015b601083901c1561612857601092831c92015b600883901c1561613a57600892831c92015b600483901c1561614c57600492831c92015b600283901c1561615e57600292831c92015b600183901c156103285760010192915050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b5f4282036161a357506020820151610328565b5f6161b2846040015184616306565b90506161cb846020015182613cc490919063ffffffff16565b915050610328565b5f5f5f845f01518460ff16600281106161ee576161ee617106565b60200201516040015190505f616224875f01518660ff166002811061621557616215617106565b60200201518860800151616190565b9050811561623b576162368282613cc4565b61623d565b5f5b865190935060ff86166002811061625657616256617106565b60200201516020015193505050935093915050565b80518051515f91829161627f916001612422565b9050806040516020016162b890602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b604051602081830303815290604052805190602001206040516020016162e8929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f8061631283426171a2565b61631c90856174a6565b6301e133809004905061035681676765c793fa10079d601b1b617428565b6040518060a0016040528061634d6166ac565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101200160405280616382616713565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101a001604052806163d561678e565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b6040518060600160405280616453616815565b81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6040518061040001604052806164c8616686565b81526020016164d561633a565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101c001604052806165a561633a565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016165fa6163c1565b905290565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806040016040528061667961633a565b81526020016165fa616440565b604051806060016040528061669961686b565b81525f6020820181905260409091015290565b60405180604001604052806002905b6166fd6040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166bb5790505090565b60405180604001604052806002905b6167786040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816167225790505090565b60405180604001604052806002905b6167ff6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161679d5790505090565b60405180604001604052806002905b61685560405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b8152602001906001900390816168245790505090565b60405180604001604052806002905b6168c36040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161687a5790505090565b6001600160a01b038116811461487b575f5ffd5b5f602082840312156168fd575f5ffd5b8135610317816168d9565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557868503603f1901845281518051606080885260a08801919088015f5b60028110156169e857898403605f19018252825180516001600160a01b031685526020808201516080918701829052906169bc90870182616908565b604083810151908801526060928301519290960191909152506020928301929190910190600101616980565b5050506020828101518882015260409283015192909701919091529493840193919091019060010161695c565b50929695505050505050565b5f5f5f5f60808587031215616a34575f5ffd5b8435616a3f816168d9565b93506020850135616a4f816168d9565b92506040850135616a5f816168d9565b9396929550929360600135925050565b5f5f5f60608486031215616a81575f5ffd5b8335616a8c816168d9565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616ade57616ade616aa1565b604052919050565b5f67ffffffffffffffff821115616aff57616aff616aa1565b5060051b60200190565b5f5f60408385031215616b1a575f5ffd5b8235616b25816168d9565b9150602083013567ffffffffffffffff811115616b40575f5ffd5b8301601f81018513616b50575f5ffd5b8035616b63616b5e82616ae6565b616ab5565b8082825260208201915060208360051b850101925087831115616b84575f5ffd5b6020840193505b82841015616ba6578335825260209384019390910190616b8b565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b81811015616c985783518051845f5b6002811015616c4757825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e082019150602083019250600181019050616bdc565b5050506020818101516001600160a01b039081166101c08701526040830151166101e08601526060820151610200860152608090910151610220850152939093019261024090920191600101616bcd565b509095945050505050565b5f8260408101835f5b6002811015616c98578383038752815180516001600160a01b0316845260208101516101406020860152616ce4610140860182616908565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616cac565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557603f19878603018452815180516101208752616da4610120880182616ca3565b9050602082015160208801526040820151616dca60408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616d7c565b5f8260408101835f5b6002811015616c98578383038752815180516001600160a01b0316845260208101516101806020860152616e5d610180860182616908565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616e25565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015616a1557603f19878603018452815180516101a08752616f356101a0880182616e1c565b90506020820151616f5160208901826001600160a01b03169052565b506040820151616f6c60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616faf60e08901826001600160a01b03169052565b50610100820151616fc561010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616f0d565b5f5f60408385031215617017575f5ffd5b8235617022816168d9565b91506020830135617032816168d9565b809150509250929050565b60ff8116811461487b575f5ffd5b5f5f5f5f5f60a0868803121561705f575f5ffd5b853561706a816168d9565b9450602086013561707a816168d9565b9350604086013561708a816168d9565b92506060860135915060808601356170a18161703d565b809150509295509295909350565b5f5f5f5f5f60a086880312156170c3575f5ffd5b85356170ce816168d9565b945060208601356170de816168d9565b935060408601356170ee816168d9565b94979396509394606081013594506080013592915050565b634e487b7160e01b5f52603260045260245ffd5b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f6020828403121561714d575f5ffd5b5051919050565b5f60208284031215617164575f5ffd5b8151610317816168d9565b5f6020828403121561717f575f5ffd5b81518015158114610317575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b818103818111156103285761032861718e565b5f600160ff1b82016171c9576171c961718e565b505f0390565b5f602082840312156171df575f5ffd5b815167ffffffffffffffff8111156171f5575f5ffd5b8201601f81018413617205575f5ffd5b8051617213616b5e82616ae6565b8082825260208201915060208360051b850101925086831115617234575f5ffd5b6020840193505b82841015611c4757835182526020938401939091019061723b565b5f60208284031215617266575f5ffd5b815167ffffffffffffffff81111561727c575f5ffd5b8201601f8101841361728c575f5ffd5b805167ffffffffffffffff8111156172a6576172a6616aa1565b6172b9601f8201601f1916602001616ab5565b8181528560208385010111156172cd575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f602082840312156172fa575f5ffd5b81516103178161703d565b6001815b6001841115617340578085048111156173245761732461718e565b600184161561733257908102905b60019390931c928002617309565b935093915050565b5f8261735657506001610328565b8161736257505f610328565b816001811461737857600281146173825761739e565b6001915050610328565b60ff8411156173935761739361718e565b50506001821b610328565b5060208310610133831016604e8410600b84101617156173c1575081810a610328565b6173cd5f198484617305565b805f19048211156173e0576173e061718e565b029392505050565b5f6103178383617348565b606081525f6174056060830186616908565b82810360208401526174178186616908565b915050826040830152949350505050565b808201808211156103285761032861718e565b634e487b7160e01b5f52601260045260245ffd5b8181035f831280158383131683831282161715613d8657613d8661718e565b606081525f6174806060830186616908565b82810360208401526174928186616908565b90508281036040840152611c478185616908565b80820281158282048414176103285761032861718e565b5f826174d757634e487b7160e01b5f52601260045260245ffd5b50049056fea264697066735822122083042749d65c56f0e77a6750b765d1de562e40c1c34bd08ffadb1f5491b5d6e464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80c\\9\xF4g\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02\x84W\x80c\xD2\x8B\n\x15\x14a\x02\xB2W\x80c\xE35\xAD\xB7\x14a\x02\xC5W\x80c\xEE\xD0t(\x14a\x02\xD8W\x80c\xF6\x8Aq1\x14a\x02\xEBW__\xFD[\x80c\\9\xF4g\x14a\x02+W\x80cs\x91\x18\xA4\x14a\x02KW\x80cx\xF2\x12\xD1\x14a\x02^W\x80c\x8Flz<\x14a\x02qW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cR_V\x0C\x14a\x01\xE5W\x80cW\xB9\x1C\xA6\x14a\x02\x05W\x80cZoW\x10\x14a\x02\x18W__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04ah\xEDV[a\x02\xFEV[`@Qa\x015\x91\x90ai6V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04ah\xEDV[a\x03\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04aj!V[a\x03.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ajoV[a\x03IV[a\x01\xB7a\x01\xB26`\x04ak\tV[a\x03^V[`@Qa\x015\x91\x90ak\xB4V[a\x01\xD7a\x01\xD26`\x04ah\xEDV[a\x04\x17V[`@Q\x90\x81R` \x01a\x015V[a\x01\xF8a\x01\xF36`\x04ak\tV[a\x04!V[`@Qa\x015\x91\x90amVV[a\x01\xD7a\x02\x136`\x04ah\xEDV[a\x04-V[a\x01\xD7a\x02&6`\x04ah\xEDV[a\x047V[a\x02>a\x0296`\x04ah\xEDV[a\x04AV[`@Qa\x015\x91\x90an\xE7V[a\x01\xF8a\x02Y6`\x04ap\x06V[a\x04ZV[a\x01Qa\x02l6`\x04ah\xEDV[a\x04uV[a\x02>a\x02\x7F6`\x04ajoV[a\x04\x7FV[a\x02\x97a\x02\x926`\x04apKV[a\x04\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x97a\x02\xC06`\x04apKV[a\x04\xAEV[a\x01Qa\x02\xD36`\x04ah\xEDV[a\x04\xBEV[a\x01(a\x02\xE66`\x04ak\tV[a\x04\xC8V[a\x01\xD7a\x02\xF96`\x04ap\xAFV[a\x04\xD4V[``_a\x03\n\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x05\x80V[\x93\x92PPPV[_a\x03(\x82a\x06\x02V[\x92\x91PPV[__a\x03<\x86\x86\x86\x86a\x06\xB3V[\x91P\x91P\x94P\x94\x92PPPV[``a\x03V\x84\x84\x84a\x05\x80V[\x94\x93PPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{aj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xB4W\x81` \x01[a\x03\xA1ac:V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_a\x03\xE6\x86\x86\x84\x81Q\x81\x10a\x03\xD9Wa\x03\xD9aq\x06V[` \x02` \x01\x01Qa\x06\xEFV[\x90P\x80\x83\x83\x81Q\x81\x10a\x03\xFBWa\x03\xFBaq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x03\xB9V[P\x93\x92PPPV[_a\x03(\x82a\x19gV[``a\x03\x17\x83\x83a\x19\xB8V[_a\x03(\x82a\x1AnV[_a\x03(\x82a\x04\xEEV[``_a\x04M\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x1A\xBFV[``_a\x04g\x84\x84a\x1B\xB6V[\x90Pa\x03V\x84\x84_\x84a\x1C,V[_a\x03(\x82a\x1CQV[``a\x03V\x84\x84\x84a\x1A\xBFV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1C\x8DV[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1D\x97V[_a\x03(\x82a\x1E\x10V[``a\x03\x17\x83\x83a\x1EaV[_a\x04\xE2\x86\x86\x86\x86\x86a\x1FGV[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x05\r\x90aq\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90aq=V[``a\x05\xB7`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x1C\xDD\x18\\\x9D`j\x1B\x81RP\x84a\x1FzV[a\x05\xEA`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x19[\x99`z\x1B\x81RP\x83a\x1FzV[_a\x05\xF6\x85\x85\x85a\x1F\xA7V[\x90Pa\x04\xE5\x85\x82a HV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06t\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90aqTV[___a\x06\xC0\x86\x86a \xFEV[\x90P_a\x06\xCD\x88\x83a\x06\xEFV[\x90P__a\x06\xDC\x8A\x84\x89a!\xA5V[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[a\x06\xF7ac:V[\x82a\x07\0ac:V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x07\x1E\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x96\x91\x90aqoV[a\x07\xA3W\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x07\xE3\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08G\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x86\x91\x90aqTV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\x04\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tw\x91\x90aq=V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\t\xCD\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\np\x91\x90aq=V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\n\xD1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0B\x01\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BPW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bt\x91\x90aq=V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xDF\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\x0F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x82\x91\x90aq=V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xE3\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\x13\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rG\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rbW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x86\x91\x90aq=V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\x03\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ERW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ev\x91\x90aq=V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90aqTV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10G\x91\x90aq=V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10\x9E\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11A\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\xA3\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x07\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\"W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12F\x91\x90aq=V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xB2\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xE2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x16\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x131W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13U\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xB7\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x146W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14Z\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xB3\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xE3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x17\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x152W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15V\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x15\xA4\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xD4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x08\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16#W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16G\x91\x90aqTV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xB5\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x174W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17X\x91\x90aqTV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\xBB\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18^\x91\x90aq=V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x18\xB7\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x1B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x196W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19Z\x91\x90aq=V[`\x80\x82\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x19\xD5Wa\x19\xD5aj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x0EW\x81` \x01[a\x19\xFBacnV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\xF3W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1A/Wa\x1A/aq\x06V[` \x02` \x01\x01Q\x90P_a\x1AD\x87\x83a#\xFAV[\x90P\x80\x84\x84\x81Q\x81\x10a\x1AYWa\x1AYaq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1A\x13V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x1A\xCD\x85\x85\x85a\x1F\xA7V[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xEAWa\x1A\xEAaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B#W\x81` \x01[a\x1B\x10ac\xC1V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\x08W\x90P[P\x90P_[\x82Q\x81\x10\x15a\x1B\xACW_\x83\x82\x81Q\x81\x10a\x1BDWa\x1BDaq\x06V[` \x02` \x01\x01Q\x90Pa\x1Bw`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RP\x82a*\tV[_a\x1B\x82\x89\x83a*:V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1B\x97Wa\x1B\x97aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1B(V[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x1B\xCF\x84a.\xDAV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x17\x91\x90aq=V[``_a\x1C;\x86\x86\x86\x86a/^V[\x90Pa\x1CG\x86\x82a\x19\xB8V[\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[____a\x1C\x9B\x88\x88a \xFEV[\x90P_a\x1C\xA8\x8A\x83a\x06\xEFV[\x90P_\x80\x80`\xFF\x89\x16a\x1C\xDEWa\x1C\xC0\x8D\x8B\x86a/\xE6V[\x92\x95P\x91\x93Pa\x1C\xD7\x91P\x85\x90P_\x85\x8D\x82a1\0V[\x90Pa\x1D\x0EV[_\x19`\xFF\x8A\x16\x01a\x1D\x0EWa\x1C\xF4\x8D\x8B\x86a1\xE2V[\x92\x95P\x91\x93Pa\x1D\x0B\x91P\x85\x90P\x84_\x80\x8Ea1\0V[\x90P[\x80_\x03a\x1D(W___\x97P\x97P\x97PPPPPPa\x04\xA3V[_a\x1D2\x85a2\xE4V[\x90P_\x82\x82\x11a\x1DKWa\x1DF\x82\x84aq\xA2V[a\x1DUV[a\x1DU\x83\x83aq\xA2V[\x90P_a\x1Db\x82\x84a3\x93V[\x90P_\x84\x84\x11a\x1DzWa\x1Du\x82aq\xB5V[a\x1D|V[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x1D\xA5\x88\x88a \xFEV[\x90P_a\x1D\xB2\x8A\x83a\x06\xEFV[\x90P_\x80\x80`\xFF\x89\x16a\x1D\xE2Wa\x1D\xCB\x8D\x8B\x86_a3\xCEV[\x92\x95P\x91\x93Pa\x1C\xD7\x91P\x85\x90P\x8B_\x80\x87a1\0V[_\x19`\xFF\x8A\x16\x01a\x1D\x0EWa\x1D\xF9\x8D\x8B\x86_a4\xF8V[\x92\x95P\x91\x93Pa\x1D\x0B\x91P\x85\x90P_\x8C\x86\x82a1\0V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06@\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[``a\x1E\x93`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l3\xB2\xBA(7\xB7\xB69\xA4\xB737\x99`\x99\x1B\x81RP\x83Qa\x1FzV[_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xAEWa\x1E\xAEaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xE7W\x81` \x01[a\x1E\xD4ad@V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1F\x08Wa\x1F\x08aq\x06V[` \x02` \x01\x01Q\x90P_a\x1F\x1D\x87\x83a6\x07V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1F2Wa\x1F2aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1E\xECV[__a\x1FS\x86\x86a \xFEV[\x90P_a\x1F`\x88\x83a\x06\xEFV[\x90Pa\x1Fn\x81\x86\x86_a8\xC7V[\x98\x97PPPPPPPPV[a\x1F\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83\x83a:!V[PPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1F\xC7\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a !W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03V\x91\x90\x81\x01\x90aq\xCFV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a eWa eaj\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x9EW\x81` \x01[a \x8Bad@V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \x83W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a \xBFWa \xBFaq\x06V[` \x02` \x01\x01Q\x90P_a \xD4\x87\x83a6\x07V[\x90P\x80\x84\x84\x81Q\x81\x10a \xE9Wa \xE9aq\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a \xA3V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a!\x1FW\x81\x83a!\"V[\x82\x82[`@Q\x91\x94P\x92Pa!O\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[____a!\xB1adeV[a!\xBA\x88a:mV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"(\x91\x90aq=V[\x81` \x01\x81\x81RPPa\"B\x87__\x84a\x01@\x01Qa:\xBEV[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"d\x90\x88\x90`\x01\x90_\x90a:\xBEV[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"\x8DW____\x94P\x94P\x94P\x94PPa#\xF1V[a\"\xA0\x86\x82`@\x01Q\x83` \x01Qa;\xA3V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"\xBB\x91\x88\x91a;\xA3V[\x81a\x01 \x01\x81\x81RPPa\"\xFB`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01ovars.totalSupply`\x80\x1B\x81RP\x82` \x01Qa\x1FzV[a#3`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x81RP\x82`@\x01Qa\x1FzV[a#k`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qvars.priceReserve1`p\x1B\x81RP\x82``\x01Qa\x1FzV[a#\x9E`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x81RP\x82a\x01\0\x01Qa\x1FzV[a#\xD1`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kvars.amount1`\xA0\x1B\x81RP\x82a\x01 \x01Qa\x1FzV[\x80a\x01\0\x01Q\x81a\x01 \x01Q\x82`\x80\x01Q\x83`\xA0\x01Q\x94P\x94P\x94P\x94PP[\x93P\x93P\x93P\x93V[a$\x02acnV[a$\nad\xB4V[a$\x14\x84\x84a<bV[\x80\x82RQ\x80QQa$-\x91`\x01[` \x02\x01QQa \xFEV[`@\x82\x01\x81\x90Ra$?\x90\x85\x90a\x06\xEFV[` \x82\x01\x81\x90R\x81Qa$S\x91\x86\x91a<tV[PPPP``\x82\x01R` \x81\x01Qa$j\x90a2\xE4V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa$\xA0\x91\x90_[` \x02\x01Q`@\x01Q\x90a<\xC4V[`\xC0\x82\x01R` \x81\x01Q``\x01Qa$\xB8\x90_a=\x05V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa$\xD1\x91\x90a=3V[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa$\xE8\x91\x90a=TV[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa%\t\x92\x91\x90a=pV[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa%*\x92\x87\x92\x90\x91_a=\x8DV[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa%W\x92\x91\x90a=pV[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa%q\x90`\x01a=\x05V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa%\xA2\x91\x90`\x01a$\x91V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa%\xB9\x91a=3V[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa%\xD1\x91\x90a=TV[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa%\xE9\x91\x90a<\xC4V[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa&\x0C\x92\x91\x90a=pV[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa&.\x92\x87\x92\x90\x91`\x01a=\x8DV[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa&]\x92\x91\x90a=pV[a\x02\xE0\x82\x01R\x80Qa&n\x90a?\"V[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a'fWa&\x98\x81a\x03\0\x01Q\x82`\x80\x01Qa=3V[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa&\xAF\x91a<\xC4V[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa&\xE0\x93a?dV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa'G\x91\x86\x91a'\x08\x91\x90a=TV[a'\x1A\x84`\xC0\x01Q\x85`\xA0\x01Qa=TV[a'.\x85a\x02\0\x01Q\x86a\x01\xC0\x01Qa=TV[a'B\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01Qa=TV[a?~V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa'_\x91\x90a@>V[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\xDAW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x01\x91\x90\x81\x01\x90arVV[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a(VWa(Vaq\x06V[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a(\xA5Wa(\xA5aq\x06V[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x05W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra),\x91\x90\x81\x01\x90arVV[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[a\x1F\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e%s -%s`\xD0\x1B\x81RP\x83a*5\x84a@\x8DV[a@\xB9V[a*Bac\xC1V[a*Jae\x91V[a*T\x84\x84a\x06\xEFV[\x81Ra*_\x84aA\0V[` \x82\x01Ra*m\x84a:mV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa*\x88\x92_\x91\x90a:\xBEV[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa*\xB6\x92\x91`\x01\x91a:\xBEV[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa*\xD8\x90aACV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+WW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+~\x91\x90\x81\x01\x90arVV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+\xCBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xEF\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xCFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xF6\x91\x90\x81\x01\x90arVV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-j\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a.J\x90a2\xE4V[\x81R` \x01a.Y\x86\x86aCRV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a.\x80\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a.\x91\x86\x86aDBV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a.\xB5\x90\x87\x90_\x80aEDV[\x81R` \x01a.\xC7\x86\x84_\x01QaF?V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a/\x14\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a/x\x86a.\xDAV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xBFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90aq\xCFV[____a/\xF2ae\xFFV[a/\xFB\x88a:mV[a\x01\xC0\x82\x01\x81\x90Ra0\x12\x90\x87\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa01\x90\x87\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a0MWP` \x81\x01Q\x15[\x15a0dW____\x94P\x94P\x94P\x94PPa#\xF1V[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra0\x93\x90\x88\x90a0\x8B\x90a'\x10\x90aFJV[a'\x10a;\xA3V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa0\xB6\x92a0\xB1\x90\x83\x90aFJV[a;\xA3V[`\x80\x82\x01\x81\x90R` \x82\x01Qa0\xCC\x91\x90aFJV[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0\xEE\x90\x8B\x90aF\x9FV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a1\x10WP\x83\x15[\x15a1\x1FWP\x83\x90P\x84a1TV[_\x87\x11\x80\x15a1,WP\x84\x15[\x15a1;WP\x85\x90P\x82a1TV[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a1c\x89``\x01Q_a=\x05V[\x90P_a1u\x8A``\x01Q`\x01a=\x05V[\x90P_a1\x93\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a1\xB1\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P\x80_\x03a1\xC8W_\x96PPPPPPPa\x04\xE5V[a1\xD2\x82\x82a3\x93V[\x9C\x9BPPPPPPPPPPPPV[____a1\xEEae\xFFV[a1\xF7\x88a:mV[a\x01\xC0\x82\x01\x81\x90Ra2\x0E\x90\x87\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa2-\x90\x87\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a2IWP` \x81\x01Q\x15[\x15a2`W____\x94P\x94P\x94P\x94PPa#\xF1V[\x80Q` \x82\x01Qa2v\x91\x90a0\xB1\x81\x8BaFJV[a\x01\0\x82\x01\x81\x90R\x81Qa2\x8A\x91\x90aFJV[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa2\xBD\x91a'\x10\x90a0\xB1\x90\x82\x90aFJV[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0\xEE\x91aF\x9FV[__a2\xF2\x83___a:\xBEV[PPP\x90P_a3\x05\x84`\x01__a:\xBEV[PPP\x90P\x80_\x03a3\x1AWP_\x93\x92PPPV[_a3)\x85``\x01Q_a=\x05V[\x90P_a3;\x86``\x01Q`\x01a=\x05V[\x90P_a3Y\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a3w\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P\x80_\x03a3\x8DWP_\x97\x96PPPPPPPV[a\x1Fn\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a3\xB4W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a3\xDAae\xFFV[a3\xE3\x89a:mV[a\x01\xC0\x82\x01\x81\x90Ra3\xFA\x90\x88\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4\x19\x90\x88\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a45WP` \x81\x01Q\x15[\x15a4LW____\x94P\x94P\x94P\x94PPa4\xEDV[\x85\x15a4gW\x87\x81_\x01\x81\x81Qa4c\x91\x90aq\xA2V[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra4\x8E\x90\x89\x90a0\x8B\x90a'\x10\x90aFJV[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa4\xAC\x92a0\xB1\x90\x83\x90aF\xC3V[`\x80\x82\x01\x81\x90R` \x82\x01Qa4\xC1\x91aFJV[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa4\xE3\x90\x8C\x90aF\x9FV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a5\x04ae\xFFV[a5\r\x89a:mV[a\x01\xC0\x82\x01\x81\x90Ra5$\x90\x88\x90_\x90\x81\x90a:\xBEV[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa5C\x90\x88\x90`\x01\x90_\x90a:\xBEV[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a5_WP` \x81\x01Q\x15[\x15a5vW____\x94P\x94P\x94P\x94PPa4\xEDV[\x85\x15a5\x92W\x87\x81` \x01\x81\x81Qa5\x8E\x91\x90aq\xA2V[\x90RP[\x80Q` \x82\x01Qa5\xA8\x91\x90a0\xB1\x81\x8CaF\xC3V[`\x80\x82\x01\x81\x90R\x81Qa5\xBA\x91aFJV[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa5\xE9\x91a0\x8B\x90a'\x10\x90aFJV[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa4\xE3\x91aF\x9FV[a6\x0Fad@V[a6\x17affV[a6!\x84\x84a\x06\xEFV[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a6VW`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a6\xC7W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xEE\x91\x90\x81\x01\x90arVV[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7_\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\xDFW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra8\x06\x91\x90\x81\x01\x90arVV[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8z\x91\x90ar\xEAV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a8\xB5\x86\x84_\x01QaF?V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a8\xD0adeV[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a94\x91\x90aq=V[` \x82\x01Ra9E\x86_\x80\x80a:\xBEV[PPP`\xC0\x82\x01Ra9Z\x86`\x01_\x80a:\xBEV[PPP`\xE0\x82\x01R\x82\x15a9\x95W\x84\x81`\xC0\x01\x81\x81Qa9z\x91\x90aq\xA2V[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a9\x91\x90\x83\x90aq\xA2V[\x90RP[`\xC0\x81\x01Q\x15\x80a9\xA8WP`\xE0\x81\x01Q\x15[\x15a9\xB6W_\x91PPa\x03VV[\x80` \x01Q_\x03a9\xE6Wa9\xDFa\x03\xE8a9\xD9a9\xD4\x88\x88aG\x17V[aG}V[\x90aFJV[\x81Ra:\x17V[a:\x14a9\xFC\x86\x83` \x01Q\x84`\xC0\x01Qa;\xA3V[a:\x0F\x86\x84` \x01Q\x85`\xE0\x01Qa;\xA3V[aH]V[\x81R[Q\x95\x94PPPPPV[a:h\x83\x83\x83`@Q`$\x01a:9\x93\x92\x91\x90as\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cX!\xEF\xA1`\xE0\x1B\x17\x90RaHrV[PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a:\xDBWa:\xDBaq\x06V[` \x02\x01Q\x90P_a:\xED\x8A\x8AaH~V[\x90P\x80_\x03a;\tW____\x95P\x95P\x95P\x95PPPa4\xEDV[_a;\x18\x83\x8C`\x80\x01QaIkV[\x90P_a;%\x82\x8Aa<\xC4V[\x90P_\x89\x15a;JW\x81\x84\x10a;DWa;?\x84\x83aFJV[a;LV[_a;LV[_[\x90P_a;Y\x85\x8Da<\xC4V[\x90P_\x8C\x15a;~W\x84\x82\x10a;xWa;s\x82\x86aFJV[a;\x80V[_a;\x80V[_[\x90Pa;\x8C\x85\x87at(V[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a;\xD7W\x83\x82\x81a;\xCDWa;\xCDat;V[\x04\x92PPPa\x03\x17V[\x80\x84\x11a;\xF7W`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[a<jaf\x86V[a\x03\x17\x83\x83aI\x9BV[_____a<\x83\x88\x88aF?V[\x90Pa<\x91\x87\x87\x83_a[\xB9V[\x90\x93P\x91P\x81a<\xA2W_\x19a<\xACV[a<\xAC\x83\x83a3\x93V[\x94Pa<\xB7\x88a\x1AnV[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a<\xE4W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01a=%WP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11a=JWa=E\x83\x83aq\xA2V[a\x03\x17V[a\x03\x17\x82\x84aq\xA2V[_a\x03\x17\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[_\x82\x84\x11a=\x86Wa=\x81\x82aq\xB5V[a\x03VV[P\x92\x91PPV[_a=\xC7`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a=\xD3\x86\x86\x86_a[\xB9V[` \x83\x01R\x80\x82R\x15\x80a>\x04WP\x84Q`\xFF\x84\x16`\x02\x81\x10a=\xF8Wa=\xF8aq\x06V[` \x02\x01Q` \x01Q_\x14[\x15a>\x12W_\x91PPa\x04\xE5V[a>\x1B\x87a]%V[`@\x82\x01\x81\x90R` \x82\x01Qa>0\x91a<\xC4V[`\x80\x82\x01\x81\x90R\x81Q\x10\x15a>HW_\x91PPa\x04\xE5V[`\x80\x81\x01Q\x81Qa>Y\x91\x90aq\xA2V[\x81``\x01\x81\x81RPPa>p\x86``\x01Q\x84a=\x05V[`\xA0\x82\x01\x81\x90R``\x82\x01Qa>\x9C\x91a>\x8B\x90`\nas\xE8V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba;\xA3V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01a>\xC1W`\xC0\x81\x01Qa>\xBB\x90\x85a3\x93V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10a>\xD7Wa>\xD7aq\x06V[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15a?\x14W\x84Q`\xFF\x84\x16`\x02\x81\x10a?\x01Wa?\x01aq\x06V[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01a?BWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01Qa?]WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x04\x0FWa?y\x82aq\xB5V[a\x04\xE5V[__a?\x89\x87a\x1AnV[\x90P_a?\x96\x82\x87a<\xC4V[\x90P_a?\xA3\x83\x86a<\xC4V[\x90P_a?\xB0\x89\x84atOV[\x90P_a?\xBD\x83\x89atOV[\x90P_a?\xC9\x83a]kV[\x90P_a?\xD5\x83a]kV[\x90P_\x84\x13\x80\x15a?\xE5WP_\x83\x12[\x80a?\xF9WP_\x84\x12\x80\x15a?\xF9WP_\x83\x13[\x15a@\rW_\x97PPPPPPPPa\x04\xE5V[\x80_\x03a@#W_\x97PPPPPPPPa\x04\xE5V[a@-\x82\x82a3\x93V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03a@MWP_a\x03(V[_\x82\x84\x11a@dWa@_\x84\x84aq\xA2V[a@nV[a@n\x83\x85aq\xA2V[\x90P_a@{\x82\x85a3\x93V[\x90P\x83\x85\x11a\x03VWa?y\x81aq\xB5V[``a\x03(\x82`@Q` \x01a@\xA5\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra]\x80V[a:h\x83\x83\x83`@Q`$\x01a@\xD1\x93\x92\x91\x90atnV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c,\xED|\xEF`\xE0\x1B\x17\x90RaHrV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__aAx`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\x82\x84_a_\x80V[` \x83\x01R\x81R``\x84\x01QaA\x98\x90_a=\x05V[``\x82\x01\x81\x90R\x81QaA\xBD\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a0\xB1\x90`\nas\xE8V[`@\x82\x01R` \x81\x01Q_\x03aA\xD8W_`\x80\x82\x01RaBxV[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aBNW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aBr\x91\x90aq=V[`\x80\x82\x01R[aB\x83\x84`\x01a_\x80V[` \x83\x01\x81\x90R\x90\x82R_\x03aB\x9EW_`\xA0\x82\x01RaC>V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC8\x91\x90aq=V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__aC^\x84\x84a_\xC6V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01aC\x9F\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aC\xCF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\x03\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90aqTV[__aDN\x84\x84a_\xC6V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01aD\xA1\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aD\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aE\x05\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90aq=V[__aEQ\x85`\x01aH~V[\x90P\x82\x15aEfWaEc\x84\x82aq\xA2V[\x90P[_aEp\x87a`\x81V[\x90P_aE}\x83\x83a<\xC4V[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10aE\xABW\x87Q` \x01Q``\x01QaE\xA6\x90\x83aq\xA2V[aE\xADV[_[\x90PaE\xDC`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RP\x85a\x1FzV[aF\x0C`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RP\x84a\x1FzV[a\x1Fn`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RP\x83a\x1FzV[_a\x03\x17\x83\x83a`\xC5V[_\x82aFV\x83\x82aq\xA2V[\x91P\x81\x11\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a6MV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aF\xB5W__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aF\xCF\x83\x82at(V[\x91P\x81\x10\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a6MV[_\x81\x15\x80aG:WP\x82\x82aG,\x81\x83at\xA6V[\x92PaG8\x90\x83at\xBDV[\x14[a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a6MV[_\x81_\x03aG\x8CWP_\x91\x90PV[_`\x01aG\x98\x84a`\xDEV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aG\xB1WaG\xB1at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xC9WaG\xC9at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xE1WaG\xE1at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aG\xF9WaG\xF9at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH\x11WaH\x11at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aH)WaH)at;V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aHAWaHAat;V[\x04\x82\x01\x90\x1C\x90Pa\x03\x17\x81\x82\x85\x81aH[WaH[at;V[\x04[_\x81\x83\x10aHkW\x81a\x03\x17V[P\x90\x91\x90PV[aH{\x81aaqV[PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aH\x98WaH\x98aq\x06V[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\x15\x91\x90aq=V[\x90P\x80_\x03aI(W_\x92PPPa\x03(V[``\x82\x01Q`\xC0\x83\x01QaI<\x90\x82at(V[\x82\x10aI`W`\xC0\x83\x01QaIQ\x82\x84aq\xA2V[aI[\x91\x90aq\xA2V[a\x1CGV[_\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aI~WP_a\x03(V[_aI\x89\x84\x84aa\x90V[`\xA0\x85\x01Q\x90\x91Pa\x03V\x90\x82a<\xC4V[aI\xA3af\x86V[\x82aI\xACaf\x86V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aI\xEC\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJd\x91\x90aqoV[aJqW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aJ\xAB\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aJ\xDB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x0F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK*W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKN\x91\x90aq=V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aK\x96\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xC6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xFA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\x15W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL9\x91\x90aqTV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aL\x95\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\xC5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xF9\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM8\x91\x90aqTV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN&\x91\x90aq=V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aNz\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\x1D\x91\x90aq=V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOw\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xDB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xF6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\x1A\x91\x90aq=V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPs\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\xA3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xD7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\x16\x91\x90aq=V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ\x9C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xD0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xEBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\x0F\x91\x90aq=V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aRi\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aR\x99\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xCD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\x0C\x91\x90aq=V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xF2\x91\x90aq=V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTf\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT\x9A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xD9\x91\x90aqTV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aU\x80\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xBF\x91\x90aq=V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\x14\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aVD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aV\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\xB7\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aWB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xB5\x91\x90aq=V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\x0F\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX?\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXs\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aX\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\xB2\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\x14\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aYD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYx\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aY\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\xB7\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\x12\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZv\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZ\x91W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\xB5\x91\x90aq=V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\x04\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[h\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\xA7\x91\x90aq=V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\\\\W__a[\xE4\x8A\x8A_aa\xD3V[\x91P\x91P_a\\\0_\x8C``\x01Qa=\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\x1E\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[\x90P_a\\<\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90Pa\\H\x82\x88at(V[\x96Pa\\T\x81\x87at(V[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]\x18W__a\\\x83\x8A\x8A`\x01aa\xD3V[\x91P\x91P_a\\\xA0`\x01\x8C``\x01Qa=\x05\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\xBE\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x85`\nas\xE8V[\x90P_a\\\xDC\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba0\xB1\x86`\nas\xE8V[\x90P_a\\\xE9\x83\x8Da<\xC4V[\x90P_a\\\xF6\x83\x8Ea<\xC4V[\x90Pa]\x02\x82\x8Aat(V[\x98Pa]\x0E\x81\x89at(V[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a]|W\x81_\x03a\x03(V[P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x10\x81Ro\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B` \x82\x01R\x81Q``\x91\x90_\x90a]\xBA\x90`\x02at\xA6V[a]\xC5\x90`\x02at(V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDDWa]\xDDaj\xA1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a^\x07W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a^!Wa^!aq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a^OWa^Oaq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_[\x84Q\x81\x10\x15a\x04\x0FW\x82`\x04\x86\x83\x81Q\x81\x10a^\x85Wa^\x85aq\x06V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a^\xADWa^\xADaq\x06V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a^\xC8\x83`\x02at\xA6V[a^\xD3\x90`\x02at(V[\x81Q\x81\x10a^\xE3Wa^\xE3aq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a_\x0CWa_\x0Caq\x06V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a_,Wa_,aq\x06V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a_G\x83`\x02at\xA6V[a_R\x90`\x03at(V[\x81Q\x81\x10a_bWa_baq\x06V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a^gV[___a_\xAD\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10a_\x9EWa_\x9Eaq\x06V[` \x02\x01Q\x86`\x80\x01QaIkV[\x90P_a_\xBA\x86\x86aH~V[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a_\xE9\x90aq\x1AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`a\x91\x90aqoV[a\x03\x17W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a6MV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x1B\xCF\x84abkV[_\x80`\x80\x83\x90\x1C\x15a`\xF2W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15aa\x04W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15aa\x16W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15aa(W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15aa:W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15aaLW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15aa^W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03(W`\x01\x01\x92\x91PPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[_B\x82\x03aa\xA3WP` \x82\x01Qa\x03(V[_aa\xB2\x84`@\x01Q\x84ac\x06V[\x90Paa\xCB\x84` \x01Q\x82a<\xC4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03(V[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10aa\xEEWaa\xEEaq\x06V[` \x02\x01Q`@\x01Q\x90P_ab$\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10ab\x15Wab\x15aq\x06V[` \x02\x01Q\x88`\x80\x01Qaa\x90V[\x90P\x81\x15ab;Wab6\x82\x82a<\xC4V[ab=V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10abVWabVaq\x06V[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[\x80Q\x80QQ_\x91\x82\x91ab\x7F\x91`\x01a$\"V[\x90P\x80`@Q` \x01ab\xB8\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xE8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[_\x80ac\x12\x83Baq\xA2V[ac\x1C\x90\x85at\xA6V[c\x01\xE13\x80\x90\x04\x90Pa\x03V\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bat(V[`@Q\x80`\xA0\x01`@R\x80acMaf\xACV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01 \x01`@R\x80ac\x82ag\x13V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xA0\x01`@R\x80ac\xD5ag\x8EV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80adSah\x15V[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x04\0\x01`@R\x80ad\xC8af\x86V[\x81R` \x01ad\xD5ac:V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ae\xA5ac:V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01ae\xFAac\xC1V[\x90R\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80afyac:V[\x81R` \x01ae\xFAad@V[`@Q\x80``\x01`@R\x80af\x99ahkV[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xFD`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81af\xBBW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[agx`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\"W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ag\xFF`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ag\x9DW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ahU`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ah$W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ah\xC3`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ahzW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aH{W__\xFD[_` \x82\x84\x03\x12\x15ah\xFDW__\xFD[\x815a\x03\x17\x81ah\xD9V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15ai\xE8W\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ai\xBC\x90\x87\x01\x82ai\x08V[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01ai\x80V[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ai\\V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15aj4W__\xFD[\x845aj?\x81ah\xD9V[\x93P` \x85\x015ajO\x81ah\xD9V[\x92P`@\x85\x015aj_\x81ah\xD9V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15aj\x81W__\xFD[\x835aj\x8C\x81ah\xD9V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aj\xDEWaj\xDEaj\xA1V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aj\xFFWaj\xFFaj\xA1V[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ak\x1AW__\xFD[\x825ak%\x81ah\xD9V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ak@W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13akPW__\xFD[\x805akcak^\x82aj\xE6V[aj\xB5V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ak\x84W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15ak\xA6W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ak\x8BV[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15al\x98W\x83Q\x80Q\x84_[`\x02\x81\x10\x15alGW\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pak\xDCV[PPP` \x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x87\x01R`@\x83\x01Q\x16a\x01\xE0\x86\x01R``\x82\x01Qa\x02\0\x86\x01R`\x80\x90\x91\x01Qa\x02 \x85\x01R\x93\x90\x93\x01\x92a\x02@\x90\x92\x01\x91`\x01\x01ak\xCDV[P\x90\x95\x94PPPPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15al\x98W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ral\xE4a\x01@\x86\x01\x82ai\x08V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pal\xACV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ram\xA4a\x01 \x88\x01\x82al\xA3V[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01Qam\xCA`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01am|V[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15al\x98W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Ran]a\x01\x80\x86\x01\x82ai\x08V[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pan%V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aj\x15W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Rao5a\x01\xA0\x88\x01\x82an\x1CV[\x90P` \x82\x01QaoQ` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01Qaol`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qao\xAF`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qao\xC5a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01ao\rV[__`@\x83\x85\x03\x12\x15ap\x17W__\xFD[\x825ap\"\x81ah\xD9V[\x91P` \x83\x015ap2\x81ah\xD9V[\x80\x91PP\x92P\x92\x90PV[`\xFF\x81\x16\x81\x14aH{W__\xFD[_____`\xA0\x86\x88\x03\x12\x15ap_W__\xFD[\x855apj\x81ah\xD9V[\x94P` \x86\x015apz\x81ah\xD9V[\x93P`@\x86\x015ap\x8A\x81ah\xD9V[\x92P``\x86\x015\x91P`\x80\x86\x015ap\xA1\x81ap=V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15ap\xC3W__\xFD[\x855ap\xCE\x81ah\xD9V[\x94P` \x86\x015ap\xDE\x81ah\xD9V[\x93P`@\x86\x015ap\xEE\x81ah\xD9V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15aqMW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15aqdW__\xFD[\x81Qa\x03\x17\x81ah\xD9V[_` \x82\x84\x03\x12\x15aq\x7FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x17W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03(Wa\x03(aq\x8EV[_`\x01`\xFF\x1B\x82\x01aq\xC9Waq\xC9aq\x8EV[P_\x03\x90V[_` \x82\x84\x03\x12\x15aq\xDFW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aq\xF5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ar\x05W__\xFD[\x80Qar\x13ak^\x82aj\xE6V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15ar4W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1CGW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ar;V[_` \x82\x84\x03\x12\x15arfW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar|W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ar\x8CW__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ar\xA6War\xA6aj\xA1V[ar\xB9`\x1F\x82\x01`\x1F\x19\x16` \x01aj\xB5V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ar\xCDW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15ar\xFAW__\xFD[\x81Qa\x03\x17\x81ap=V[`\x01\x81[`\x01\x84\x11\x15as@W\x80\x85\x04\x81\x11\x15as$Was$aq\x8EV[`\x01\x84\x16\x15as2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02as\tV[\x93P\x93\x91PPV[_\x82asVWP`\x01a\x03(V[\x81asbWP_a\x03(V[\x81`\x01\x81\x14asxW`\x02\x81\x14as\x82Was\x9EV[`\x01\x91PPa\x03(V[`\xFF\x84\x11\x15as\x93Was\x93aq\x8EV[PP`\x01\x82\x1Ba\x03(V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15as\xC1WP\x81\x81\na\x03(V[as\xCD_\x19\x84\x84as\x05V[\x80_\x19\x04\x82\x11\x15as\xE0Was\xE0aq\x8EV[\x02\x93\x92PPPV[_a\x03\x17\x83\x83asHV[``\x81R_at\x05``\x83\x01\x86ai\x08V[\x82\x81\x03` \x84\x01Rat\x17\x81\x86ai\x08V[\x91PP\x82`@\x83\x01R\x94\x93PPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03(Wa\x03(aq\x8EV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a=\x86Wa=\x86aq\x8EV[``\x81R_at\x80``\x83\x01\x86ai\x08V[\x82\x81\x03` \x84\x01Rat\x92\x81\x86ai\x08V[\x90P\x82\x81\x03`@\x84\x01Ra\x1CG\x81\x85ai\x08V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03(Wa\x03(aq\x8EV[_\x82at\xD7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x83\x04'I\xD6\\V\xF0\xE7zgP\xB7e\xD1\xDEV.@\xC1\xC3K\xD0\x8F\xFA\xDB\x1FT\x91\xB5\xD6\xE4dsolcC\0\x08\x1C\x003",
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
    /**Function with signature `getPools2(address,bytes32[])` and selector `0x50376aaa`.
```solidity
function getPools2(address dataStore, bytes32[] memory poolKeys) external view returns (Pool.Props[] memory);
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
            <Pool::Props as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<Pool::Props>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <Pool::Props as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<Pool::Props>,);
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
    /**Function with signature `getPositions(address,bytes32[])` and selector `0x525f560c`.
```solidity
function getPositions(address dataStore, bytes32[] memory positionKeys) external view returns (ReaderPositionUtils.GetPosition[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions_0Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub positionKeys: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`getPositions(address,bytes32[])`](getPositions_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions_0Return {
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
            impl ::core::convert::From<getPositions_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPositions_0Call) -> Self {
                    (value.dataStore, value.positionKeys)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositions_0Call {
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
            impl ::core::convert::From<getPositions_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPositions_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPositions_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositions_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositions_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<ReaderPositionUtils::GetPosition>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPositions(address,bytes32[])";
            const SELECTOR: [u8; 4] = [82u8, 95u8, 86u8, 12u8];
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
    /**Function with signature `getPositions(address,address)` and selector `0x739118a4`.
```solidity
function getPositions(address dataStore, address account) external view returns (ReaderPositionUtils.GetPosition[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions_1Call {
        pub dataStore: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getPositions(address,address)`](getPositions_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPositions_1Return {
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
            impl ::core::convert::From<getPositions_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getPositions_1Call) -> Self {
                    (value.dataStore, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPositions_1Call {
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
            impl ::core::convert::From<getPositions_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPositions_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPositions_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPositions_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPositions_1Return;
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
        getPools2(getPools2Call),
        getPoolsCount(getPoolsCountCall),
        getPoolsInfo_0(getPoolsInfo_0Call),
        getPoolsInfo_1(getPoolsInfo_1Call),
        getPoolsInfo_2(getPoolsInfo_2Call),
        getPositions_0(getPositions_0Call),
        getPositions_1(getPositions_1Call),
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
            [82u8, 95u8, 86u8, 12u8],
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
                Self::getPositions_0(_) => {
                    <getPositions_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPositions_1(_) => {
                    <getPositions_1Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn getPositions_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPositions_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPositions_0)
                    }
                    getPositions_0
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
                    fn getPositions_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ReaderCalls> {
                        <getPositions_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ReaderCalls::getPositions_1)
                    }
                    getPositions_1
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
                Self::getPositions_0(inner) => {
                    <getPositions_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPositions_1(inner) => {
                    <getPositions_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getPositions_0(inner) => {
                    <getPositions_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPositions_1(inner) => {
                    <getPositions_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`getPositions_0`] function.
        pub fn getPositions_0(
            &self,
            dataStore: alloy::sol_types::private::Address,
            positionKeys: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositions_0Call, N> {
            self.call_builder(
                &getPositions_0Call {
                    dataStore,
                    positionKeys,
                },
            )
        }
        ///Creates a new call builder for the [`getPositions_1`] function.
        pub fn getPositions_1(
            &self,
            dataStore: alloy::sol_types::private::Address,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPositions_1Call, N> {
            self.call_builder(
                &getPositions_1Call {
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
