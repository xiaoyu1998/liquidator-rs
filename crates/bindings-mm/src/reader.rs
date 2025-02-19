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
    ///0x6080604052348015600e575f5ffd5b506172008061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610111575f3560e01c8063739118a41161009e578063c2bdeda11161006e578063c2bdeda114610284578063d28b0a15146102b2578063e335adb7146102c5578063eed07428146102d8578063f68a7131146102eb575f5ffd5b8063739118a41461022b57806378f212d11461024b5780638f6c7a3c1461025e578063a6a100a014610271575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c457806357b91ca6146101e55780635a6f5710146101f85780635c39f4671461020b575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b610128610123366004616648565b6102fe565b6040516101359190616691565b60405180910390f35b61015161014c366004616648565b61031e565b6040516001600160a01b039091168152602001610135565b61017c61017736600461677c565b61032e565b60408051928352602083019190915201610135565b61012861019f3660046167ca565b610349565b6101b76101b2366004616864565b61035e565b604051610135919061690f565b6101d76101d2366004616648565b610417565b604051908152602001610135565b6101d76101f3366004616648565b610421565b6101d7610206366004616648565b61042b565b61021e610219366004616648565b610435565b6040516101359190616ac9565b61023e610239366004616be8565b61044e565b6040516101359190616cd2565b610151610259366004616648565b610469565b61021e61026c3660046167ca565b610473565b61023e61027f366004616864565b610480565b610297610292366004616da6565b61048c565b60408051938452602084019290925290820152606001610135565b6102976102c0366004616da6565b6104ae565b6101516102d3366004616648565b6104be565b6101286102e6366004616864565b6104c8565b6101d76102f9366004616e0a565b6104d4565b60605f61030a836104ee565b9050610317835f83610580565b9392505050565b5f610328826105fc565b92915050565b5f5f61033c868686866106ad565b9150915094509492505050565b6060610356848484610580565b949350505050565b60605f825167ffffffffffffffff81111561037b5761037b6167fc565b6040519080825280602002602001820160405280156103b457816020015b6103a1616095565b8152602001906001900390816103995790505b5090505f5b835181101561040f575f6103e6868684815181106103d9576103d9616e61565b60200260200101516106e9565b9050808383815181106103fb576103fb616e61565b6020908102919091010152506001016103b9565b509392505050565b5f61032882611961565b5f610328826119b2565b5f610328826104ee565b60605f610441836104ee565b9050610317835f83611a03565b60605f61045b8484611af7565b905061035684845f84611b6d565b5f61032882611b92565b6060610356848484611a03565b60606103178383611bce565b5f5f5f61049c8888888888611c84565b9250925092505b955095509592505050565b5f5f5f61049c8888888888611d8e565b5f61032882611e07565b60606103178383611e58565b5f6104e28686868686611f33565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161050d90616e75565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161054191815260200190565b602060405180830381865afa15801561055c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190616e98565b60606105b46040518060400160405280601381526020017219d95d141bdbdb1cd25b999bcc081cdd185c9d606a1b81525050565b6105e46040518060400160405280601181526020017019d95d141bdbdb1cd25b999bcc08195b99607a1b81525050565b5f6105f0858585611f66565b90506104e58582612007565b5f816001600160a01b03166321f8a72160405160200161063a906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161066e91815260200190565b602060405180830381865afa158015610689573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190616eaf565b5f5f5f6106ba86866120bd565b90505f6106c788836106e9565b90505f5f6106d68a8489612164565b50919c909b509950505050505050505050565b6106f1616095565b826106fa616095565b816001600160a01b03166391d4403c60405160200161071890616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561076c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107909190616eca565b61079d5791506103289050565b816001600160a01b03166321f8a721856040516020016107dd906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161080d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161084191815260200190565b602060405180830381865afa15801561085c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108809190616eaf565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016108fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161093291815260200190565b602060405180830381865afa15801561094d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109719190616e98565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016109c7906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016109f7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a2b91815260200190565b602060405180830381865afa158015610a46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a6a9190616e98565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610acb9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610afb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b2f91815260200190565b602060405180830381865afa158015610b4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b6e9190616e98565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610bd99060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610c09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c3d91815260200190565b602060405180830381865afa158015610c58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c7c9190616e98565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610cdd9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d0d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4191815260200190565b602060405180830381865afa158015610d5c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d809190616e98565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610dfd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e3191815260200190565b602060405180830381865afa158015610e4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e709190616e98565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001610ee5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f1991815260200190565b602060405180830381865afa158015610f34573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f589190616eaf565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100291815260200190565b602060405180830381865afa15801561101d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110419190616e98565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161109890602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016110c8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110fc91815260200190565b602060405180830381865afa158015611117573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061113b9190616e98565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161119d9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016111cd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161120191815260200190565b602060405180830381865afa15801561121c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112409190616e98565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016112ac9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016112dc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131091815260200190565b602060405180830381865afa15801561132b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061134f9190616e98565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016113b19060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161141591815260200190565b602060405180830381865afa158015611430573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114549190616e98565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016114ad90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016114dd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161151191815260200190565b602060405180830381865afa15801561152c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115509190616e98565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161159e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161160291815260200190565b602060405180830381865afa15801561161d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116419190616eaf565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016116af906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171391815260200190565b602060405180830381865afa15801561172e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117529190616eaf565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016117b5906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016117e5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161181991815260200190565b602060405180830381865afa158015611834573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118589190616e98565b60608201526040516001600160a01b0383169063bd02d0f59086906118b1906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161191591815260200190565b602060405180830381865afa158015611930573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119549190616e98565b6080820152949350505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f611a11858585611f66565b90505f815167ffffffffffffffff811115611a2e57611a2e6167fc565b604051908082528060200260200182016040528015611a6757816020015b611a546160c9565b815260200190600190039081611a4c5790505b5090505f5b8251811015611aed575f838281518110611a8857611a88616e61565b60200260200101519050611ab860405180604001604052806007815260200166706f6f6c4b657960c81b81525050565b5f611ac3898361235d565b905080848481518110611ad857611ad8616e61565b60209081029190910101525050600101611a6c565b5095945050505050565b5f826001600160a01b031663f3903b9f611b10846127fd565b6040518263ffffffff1660e01b8152600401611b2e91815260200190565b602060405180830381865afa158015611b49573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103179190616e98565b60605f611b7c86868686612881565b9050611b888682611bce565b9695505050505050565b5f816001600160a01b03166321f8a72160405160200161063a90602080825260089082015267545245415355525960c01b604082015260600190565b60605f825167ffffffffffffffff811115611beb57611beb6167fc565b604051908082528060200260200182016040528015611c2457816020015b611c11616148565b815260200190600190039081611c095790505b5090505f5b835181101561040f575f848281518110611c4557611c45616e61565b602002602001015190505f611c5a8783612909565b905080848481518110611c6f57611c6f616e61565b60209081029190910101525050600101611c29565b5f5f5f5f611c9288886120bd565b90505f611c9f8a836106e9565b90505f808060ff8916611cd557611cb78d8b86612f18565b929550919350611cce91508590505f858d82613032565b9050611d05565b5f1960ff8a1601611d0557611ceb8d8b86613114565b929550919350611d029150859050845f808e613032565b90505b805f03611d1f575f5f5f97509750975050505050506104a3565b5f611d2985613216565b90505f828211611d4257611d3d8284616efd565b611d4c565b611d4c8383616efd565b90505f611d5982846132c5565b90505f848411611d7157611d6c82616f10565b611d73565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f611d9c88886120bd565b90505f611da98a836106e9565b90505f808060ff8916611dd957611dc28d8b865f613300565b929550919350611cce91508590508b5f8087613032565b5f1960ff8a1601611d0557611df08d8b865f61342a565b929550919350611d0291508590505f8c8682613032565b5f816001600160a01b03166321f8a72160405160200161063a906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60408051808201909152600d81526c33b2ba2837b7b639a4b733379960991b60209091015260605f825167ffffffffffffffff811115611e9a57611e9a6167fc565b604051908082528060200260200182016040528015611ed357816020015b611ec061619b565b815260200190600190039081611eb85790505b5090505f5b835181101561040f575f848281518110611ef457611ef4616e61565b602002602001015190505f611f098783613539565b905080848481518110611f1e57611f1e616e61565b60209081029190910101525050600101611ed8565b5f5f611f3f86866120bd565b90505f611f4c88836106e9565b9050611f5a8186865f6137f9565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001611f8690616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611fe0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103569190810190616f2a565b60605f825167ffffffffffffffff811115612024576120246167fc565b60405190808252806020026020018201604052801561205d57816020015b61204a61619b565b8152602001906001900390816120425790505b5090505f5b835181101561040f575f84828151811061207e5761207e616e61565b602002602001015190505f6120938783613539565b9050808484815181106120a8576120a8616e61565b60209081029190910101525050600101612062565b5f816001600160a01b0316836001600160a01b0316106120de5781836120e1565b82825b604051919450925061210e906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b5f5f5f5f6121706161c0565b61217988613953565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e79190616e98565b816020018181525050612201875f5f8461014001516139a4565b5060808401525060408201526101408101516122239088906001905f906139a4565b5060a084015250606082015260208101515f0361224c575f5f5f5f945094509450945050612354565b61225f8682604001518360200151613a89565b6101008201526060810151602082015161227a918891613a89565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a090930151919650945090925090505b93509350935093565b6123656160c9565b61236d61620f565b61237784846106e9565b815261238284613b48565b602082015261239084613953565b6101808201819052815160208301516123ab925f91906139a4565b608085015260a084015260408301526060820152805160208201516101808301516123d992916001916139a4565b61012085015261014084015260e083015261010082015280516123fb90613b8b565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa15801561247a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124a19190810190616fb1565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa1580156124ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125129190617045565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156125f2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526126199190810190616fb1565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015612669573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061268d9190617045565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061276d90613216565b815260200161277c8686613d9a565b6001600160a01b031681526020016127a3835f015160600151660800000000000016151590565b151581526020016127b48686613e8a565b81528251515160c00151602082015282516040909101906127d89087905f80613f8c565b81526020016127ea86845f015161407e565b90526101a0909101819052905092915050565b5f604051602001612837906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61289b866127fd565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa1580156128e2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104e59190810190616f2a565b612911616148565b61291961627d565b6129238484614089565b8082525180515161293c9160015b6020020151516120bd565b6040820181905261294e9085906106e9565b60208201819052815161296291869161409b565b505050506060820152602081015161297990613216565b610300820152805180515160209081015160e0840152808301515151015190516129af91905f5b602002015160400151906140eb565b60c08201526020810151606001516129c7905f61412c565b60a082015260e081015160c08201516129e0919061415a565b610100820181905260a08201516129f7919061417b565b61012082015260e081015160c0820151610100830151612a18929190614197565b61014082015260208101518151610300830151612a3992879290915f6141b4565b61016082015261014081015161018082015260e081015160c0820151610120830151612a66929190614197565b6101a0820152602081015160600151612a8090600161412c565b6101c082015280518051602090810151810151610200840152808301515181015101519051612ab1919060016129a0565b6101e08201819052610200820151612ac89161415a565b61022082018190526101c0820151612ae0919061417b565b6102408201819052610300820151612af891906140eb565b6102608201526102008101516101e0820151610220830151612b1b929190614197565b61028082015260208101518151610300830151612b3d928792909160016141b4565b6102a08201526102808101516102c08201526102008101516101e0820151610260830151612b6c929190614197565b6102e08201528051612b7d90614349565b60808201528051516020015160e00151600214612c7557612ba7816103000151826080015161415a565b6103208201819052610240820151612bbe916140eb565b610340820181905260808201516103008301511161038083018190526102008301516101e0840151612bef9361438b565b61036082018190526103a082015260e081015160a0820151612c56918691612c17919061417b565b612c298460c001518560a0015161417b565b612c3d856102000151866101c0015161417b565b612c51866101e00151876101c0015161417b565b6143a5565b6103c08201819052610300820151612c6e9190614465565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612ce9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d109190810190616fb1565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612d6557612d65616e61565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612db457612db4616e61565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612e14573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612e3b9190810190616fb1565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612f2461635a565b612f2d88613953565b6101c08201819052612f449087905f9081906139a4565b5060408401525081526101c0810151612f639087906001905f906139a4565b50606084015250602082015280511580612f7f57506020810151155b15612f96575f5f5f5f945094509450945050612354565b606086015160381c61ffff166101408201819052612fc5908890612fbd90612710906144b4565b612710613a89565b610180820181905281516020830151612fe892612fe39083906144b4565b613a89565b608082018190526020820151612ffe91906144b4565b60e0820181905260408201516060830151610140840151613020908b90614509565b94509450945094505093509350935093565b5f5f5f5f86118015613042575083155b15613051575083905084613086565b5f8711801561305e575084155b1561306d575085905082613086565b604051636331fab160e01b815260040160405180910390fd5b5f61309589606001515f61412c565b90505f6130a78a60600151600161412c565b90505f6130c585676765c793fa10079d601b1b612fe386600a617143565b90505f6130e385676765c793fa10079d601b1b612fe386600a617143565b9050805f036130fa575f96505050505050506104e5565b61310482826132c5565b9c9b505050505050505050505050565b5f5f5f5f61312061635a565b61312988613953565b6101c082018190526131409087905f9081906139a4565b5060408401525081526101c081015161315f9087906001905f906139a4565b5060608401525060208201528051158061317b57506020810151155b15613192575f5f5f5f945094509450945050612354565b805160208201516131a89190612fe3818b6144b4565b610100820181905281516131bc91906144b4565b610120820152606086015160381c61ffff1661014082018190526101208201516131ef9161271090612fe39082906144b4565b6101a082018190526040820151606083015161014084015161012085015161302091614509565b5f5f613224835f5f5f6139a4565b50505090505f6132378460015f5f6139a4565b5050509050805f0361324c57505f9392505050565b5f61325b85606001515f61412c565b90505f61326d8660600151600161412c565b90505f61328b85676765c793fa10079d601b1b612fe386600a617143565b90505f6132a985676765c793fa10079d601b1b612fe386600a617143565b9050805f036132bf57505f979650505050505050565b611f5a82825b5f8115676765c793fa10079d601b1b600284041904841117156132e6575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f61330c61635a565b61331589613953565b6101c0820181905261332c9088905f9081906139a4565b5060408401525081526101c081015161334b9088906001905f906139a4565b5060608401525060208201528051158061336757506020810151155b1561337e575f5f5f5f94509450945094505061341f565b85156133995787815f018181516133959190616efd565b9052505b606087015160381c61ffff1661014082018190526133c0908990612fbd90612710906144b4565b6101608201819052815160208301516133de92612fe390839061452d565b6080820181905260208201516133f3916144b4565b60c0820181905260408201516060830151610140840151613415908c90614509565b9450945094509450505b945094509450949050565b5f5f5f5f61343661635a565b61343f89613953565b6101c082018190526134569088905f9081906139a4565b5060408401525081526101c08101516134759088906001905f906139a4565b5060608401525060208201528051158061349157506020810151155b156134a8575f5f5f5f94509450945094505061341f565b85156134c45787816020018181516134c09190616efd565b9052505b805160208201516134da9190612fe3818c61452d565b6080820181905281516134ec916144b4565b60a0820152606087015160381c61ffff16610140820181905260a082015161351b91612fbd90612710906144b4565b6040820151606083015161014084015160a085015161341591614509565b61354161619b565b6135496163c1565b61355384846106e9565b808252602001516001600160a01b031661358857604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156135f9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136209190810190616fb1565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561366d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136919190617045565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015613711573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526137389190810190616fb1565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613788573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137ac9190617045565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016137e786845f015161407e565b90526020909101819052905092915050565b5f6138026161c0565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613842573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138669190616e98565b6020820152613877865f80806139a4565b50505060c082015261388c8660015f806139a4565b50505060e082015282156138c757848160c0018181516138ac9190616efd565b90525060e0810180518591906138c3908390616efd565b9052505b60c081015115806138da575060e0810151155b156138e8575f915050610356565b80602001515f03613918576139116103e861390b6139068888614581565b6145e7565b906144b4565b8152613949565b61394661392e8683602001518460c00151613a89565b6139418684602001518560e00151613a89565b6146c7565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff16600281106139c1576139c1616e61565b602002015190505f6139d38a8a6146df565b9050805f036139ef575f5f5f5f9550955095509550505061341f565b5f6139fe838c608001516147cc565b90505f613a0b828a6140eb565b90505f8915613a3057818410613a2a57613a2584836144b4565b613a32565b5f613a32565b5f5b90505f613a3f858d6140eb565b90505f8c15613a6457848210613a5e57613a5982866144b4565b613a66565b5f613a66565b5f5b9050613a72858761714e565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613abd57838281613ab357613ab3617161565b0492505050610317565b808411613add5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f613bc06040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613bca845f614915565b602083015281526060840151613be0905f61412c565b606082018190528151613c0591676765c793fa10079d601b1b90612fe390600a617143565b604082015260208101515f03613c20575f6080820152613cc0565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cba9190616e98565b60808201525b613ccb846001614915565b602083018190529082525f03613ce6575f60a0820152613d86565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613d5c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d809190616e98565b60a08201525b80608001518160a001519250925050915091565b5f5f613da6848461495b565b9050806001600160a01b03166321f8a72184604051602001613de7906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e17929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e4b91815260200190565b602060405180830381865afa158015613e66573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190616eaf565b5f5f613e96848461495b565b9050806001600160a01b031663bd02d0f584604051602001613ee99060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613f19929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f4d91815260200190565b602060405180830381865afa158015613f68573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190616e98565b5f5f613f998560016146df565b90508215613fae57613fab8482616efd565b90505b5f613fb887614a16565b90505f613fc583836140eb565b875160200151606001519091505f908210613ff35787516020015160600151613fee9083616efd565b613ff5565b5f5b90506140216040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525050565b61404e6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525050565b611f5a604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525050565b5f6103178383614a5a565b6140916163e1565b6103178383614a73565b5f5f5f5f5f6140aa888861407e565b90506140b88787835f615c91565b9093509150816140c9575f196140d3565b6140d383836132c5565b94506140de886119b2565b9350939792965093509350565b5f81156b019d971e4fe8401e74000000198390048411151761410b575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff85160161414c575060ff60601b19905060605b90198416901c905092915050565b5f8183116141715761416c8383616efd565b610317565b6103178284616efd565b5f61031783676765c793fa10079d601b1b612fe385600a617143565b5f8284116141ad576141a882616f10565b610356565b5092915050565b5f6141ee6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6141fa8686865f615c91565b6020830152808252158061422b5750845160ff84166002811061421f5761421f616e61565b6020020151602001515f145b15614239575f9150506104e5565b61424287615dfd565b604082018190526020820151614257916140eb565b608082018190528151101561426f575f9150506104e5565b608081015181516142809190616efd565b81606001818152505061429786606001518461412c565b60a0820181905260608201516142c3916142b290600a617143565b676765c793fa10079d601b1b613a89565b60c08201525f1960ff8416016142e85760c08101516142e290856132c5565b60c08201525b845160ff8416600281106142fe576142fe616e61565b6020020151602001518160c00151111561433b57845160ff84166002811061432857614328616e61565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901614369575051602001516060015190565b81516020015160e00151614384575051602001516080015190565b505f919050565b5f8415158385111461040f576143a082616f10565b6104e5565b5f5f6143b0876119b2565b90505f6143bd82876140eb565b90505f6143ca83866140eb565b90505f6143d78984617175565b90505f6143e48389617175565b90505f6143f083615e43565b90505f6143fc83615e43565b90505f8413801561440c57505f83125b8061442057505f8412801561442057505f83135b15614434575f9750505050505050506104e5565b805f0361444a575f9750505050505050506104e5565b61445482826132c5565b9d9c50505050505050505050505050565b5f815f0361447457505f610328565b5f82841161448b576144868484616efd565b614495565b6144958385616efd565b90505f6144a282856132c5565b9050838511610356576143a081616f10565b5f826144c08382616efd565b91508111156103285760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161357f565b5f8115611388198390048411151761451f575f5ffd5b506127109102611388010490565b5f82614539838261714e565b91508110156103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161357f565b5f8115806145a4575082826145968183617194565b92506145a290836171ab565b145b6103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161357f565b5f815f036145f657505f919050565b5f600161460284615e58565b901c6001901b9050600181848161461b5761461b617161565b048201901c9050600181848161463357614633617161565b048201901c9050600181848161464b5761464b617161565b048201901c9050600181848161466357614663617161565b048201901c9050600181848161467b5761467b617161565b048201901c9050600181848161469357614693617161565b048201901c905060018184816146ab576146ab617161565b048201901c9050610317818285816146c5576146c5617161565b045b5f8183106146d55781610317565b5090919050565b50565b5f5f835f01518360ff16600281106146f9576146f9616e61565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614752573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147769190616e98565b9050805f03614789575f92505050610328565b606082015160c083015161479d908261714e565b82106147c15760c08301516147b28284616efd565b6147bc9190616efd565b611b88565b5f9695505050505050565b5f8260a001515f036147df57505f610328565b5f6147ea8484615eeb565b60a085015190915061035690826140eb565b845181101561040f5782600486838151811061481a5761481a616e61565b016020015182516001600160f81b031990911690911c60f81c90811061484257614842616e61565b01602001516001600160f81b0319168261485d836002617194565b61486890600261714e565b8151811061487857614878616e61565b60200101906001600160f81b03191690815f1a905350828582815181106148a1576148a1616e61565b602091010151815160f89190911c600f169081106148c1576148c1616e61565b01602001516001600160f81b031916826148dc836002617194565b6148e790600361714e565b815181106148f7576148f7616e61565b60200101906001600160f81b03191690815f1a9053506001016147fc565b5f5f5f614942855f01518560ff166002811061493357614933616e61565b602002015186608001516147cc565b90505f61494f86866146df565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c60405160200161497e90616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156149d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149f69190616eca565b61031757604051637357d91f60e01b81526004810184905260240161357f565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5611b1084615f2e565b614a7b6163e1565b82614a846163e1565b816001600160a01b03166391d4403c604051602001614ac4906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614b18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b3c9190616eca565b614b495791506103289050565b816001600160a01b031663bd02d0f585604051602001614b83906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bb3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614be791815260200190565b602060405180830381865afa158015614c02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c269190616e98565b816020018181525050816001600160a01b03166321f8a72185604051602001614c6e906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c9e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cd291815260200190565b602060405180830381865afa158015614ced573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d119190616eaf565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614d6d906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d9d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614dd191815260200190565b602060405180830381865afa158015614dec573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e109190616eaf565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614e8b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ebf91815260200190565b602060405180830381865afa158015614eda573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614efe9190616e98565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614f529060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f82929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fb691815260200190565b602060405180830381865afa158015614fd1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ff59190616e98565b81515f60200201516040018181525050816001600160a01b031663bd02d0f58560405160200161504f906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161507f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150b391815260200190565b602060405180830381865afa1580156150ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150f29190616e98565b81515f60200201516060018181525050816001600160a01b031663bd02d0f58560405160200161514b906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161517b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151af91815260200190565b602060405180830381865afa1580156151ca573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151ee9190616e98565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615274929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152a891815260200190565b602060405180830381865afa1580156152c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152e79190616e98565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615341906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615371929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153a591815260200190565b602060405180830381865afa1580156153c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153e49190616e98565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615457929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161548b91815260200190565b602060405180830381865afa1580156154a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154ca9190616e98565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a7219086906080016040516020818303038152906040528051906020012060405160200161553e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161557291815260200190565b602060405180830381865afa15801561558d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155b19190616eaf565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161565891815260200190565b602060405180830381865afa158015615673573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156979190616e98565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156ec9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161571c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161575091815260200190565b602060405180830381865afa15801561576b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061578f9190616e98565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016157ea90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161581a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161584e91815260200190565b602060405180830381865afa158015615869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061588d9190616e98565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016158e790602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001615917929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161594b91815260200190565b602060405180830381865afa158015615966573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061598a9190616e98565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016159ec9060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615a1c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a5091815260200190565b602060405180830381865afa158015615a6b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a8f9190616e98565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615aea90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b4e91815260200190565b602060405180830381865afa158015615b69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b8d9190616e98565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615bdc906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615c0c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c4091815260200190565b602060405180830381865afa158015615c5b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c7f9190616e98565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615d34575f5f615cbc8a8a5f615fc9565b915091505f615cd85f8c6060015161412c90919063ffffffff16565b90505f615cf684676765c793fa10079d601b1b612fe385600a617143565b90505f615d1484676765c793fa10079d601b1b612fe386600a617143565b9050615d20828861714e565b9650615d2c818761714e565b955050505050505b865160200151516001600160a01b03868116911614615df0575f5f615d5b8a8a6001615fc9565b915091505f615d7860018c6060015161412c90919063ffffffff16565b90505f615d9684676765c793fa10079d601b1b612fe385600a617143565b90505f615db484676765c793fa10079d601b1b612fe386600a617143565b90505f615dc1838d6140eb565b90505f615dce838e6140eb565b9050615dda828a61714e565b9850615de6818961714e565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615e5457815f03610328565b5090565b5f80608083901c15615e6c57608092831c92015b604083901c15615e7e57604092831c92015b602083901c15615e9057602092831c92015b601083901c15615ea257601092831c92015b600883901c15615eb457600892831c92015b600483901c15615ec657600492831c92015b600283901c15615ed857600292831c92015b600183901c156103285760010192915050565b5f428203615efe57506020820151610328565b5f615f0d846040015184616061565b9050615f268460200151826140eb90919063ffffffff16565b915050610328565b80518051515f918291615f42916001612931565b905080604051602001615f7b90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b60405160208183030381529060405280519060200120604051602001615fab929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff1660028110615fe457615fe4616e61565b60200201516040015190505f61601a875f01518660ff166002811061600b5761600b616e61565b60200201518860800151615eeb565b905081156160315761602c82826140eb565b616033565b5f5b865190935060ff86166002811061604c5761604c616e61565b60200201516020015193505050935093915050565b5f8061606d8342616efd565b6160779085617194565b6301e133809004905061035681676765c793fa10079d601b1b61714e565b6040518060a001604052806160a8616407565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101a001604052806160dd61646e565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b60405180610120016040528061615c6164f5565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806161ae616570565b81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c00160405280616223616095565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016162786160c9565b905290565b6040518061040001604052806162916163e1565b815260200161629e616095565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806163d4616095565b815260200161627861619b565b60405180606001604052806163f46165c6565b81525f6020820181905260409091015290565b60405180604001604052806002905b6164586040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816164165790505090565b60405180604001604052806002905b6164df6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161647d5790505090565b60405180604001604052806002905b61655a6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165045790505090565b60405180604001604052806002905b6165b060405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b81526020019060019003908161657f5790505090565b60405180604001604052806002905b61661e6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165d55790505090565b6001600160a01b03811681146146dc575f5ffd5b5f60208284031215616658575f5ffd5b813561031781616634565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057868503603f1901845281518051606080885260a08801919088015f5b600281101561674357898403605f19018252825180516001600160a01b0316855260208082015160809187018290529061671790870182616663565b6040838101519088015260609283015192909601919091525060209283019291909101906001016166db565b505050602082810151888201526040928301519290970191909152949384019391909101906001016166b7565b50929695505050505050565b5f5f5f5f6080858703121561678f575f5ffd5b843561679a81616634565b935060208501356167aa81616634565b925060408501356167ba81616634565b9396929550929360600135925050565b5f5f5f606084860312156167dc575f5ffd5b83356167e781616634565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616839576168396167fc565b604052919050565b5f67ffffffffffffffff82111561685a5761685a6167fc565b5060051b60200190565b5f5f60408385031215616875575f5ffd5b823561688081616634565b9150602083013567ffffffffffffffff81111561689b575f5ffd5b8301601f810185136168ab575f5ffd5b80356168be6168b982616841565b616810565b8082825260208201915060208360051b8501019250878311156168df575f5ffd5b6020840193505b828410156169015783358252602093840193909101906168e6565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b818110156169f35783518051845f5b60028110156169a257825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e082019150602083019250600181019050616937565b5050506020818101516001600160a01b039081166101c08701526040830151166101e08601526060820151610200860152608090910151610220850152939093019261024090920191600101616928565b509095945050505050565b5f8260408101835f5b60028110156169f3578383038752815180516001600160a01b0316845260208101516101806020860152616a3f610180860182616663565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616a07565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057603f19878603018452815180516101a08752616b176101a08801826169fe565b90506020820151616b3360208901826001600160a01b03169052565b506040820151616b4e60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616b9160e08901826001600160a01b03169052565b50610100820151616ba761010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616aef565b5f5f60408385031215616bf9575f5ffd5b8235616c0481616634565b91506020830135616c1481616634565b809150509250929050565b5f8260408101835f5b60028110156169f3578383038752815180516001600160a01b0316845260208101516101406020860152616c60610140860182616663565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616c28565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057603f19878603018452815180516101208752616d20610120880182616c1f565b9050602082015160208801526040820151616d4660408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616cf8565b60ff811681146146dc575f5ffd5b5f5f5f5f5f60a08688031215616dba575f5ffd5b8535616dc581616634565b94506020860135616dd581616634565b93506040860135616de581616634565b9250606086013591506080860135616dfc81616d98565b809150509295509295909350565b5f5f5f5f5f60a08688031215616e1e575f5ffd5b8535616e2981616634565b94506020860135616e3981616634565b93506040860135616e4981616634565b94979396509394606081013594506080013592915050565b634e487b7160e01b5f52603260045260245ffd5b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f60208284031215616ea8575f5ffd5b5051919050565b5f60208284031215616ebf575f5ffd5b815161031781616634565b5f60208284031215616eda575f5ffd5b81518015158114610317575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561032857610328616ee9565b5f600160ff1b8201616f2457616f24616ee9565b505f0390565b5f60208284031215616f3a575f5ffd5b815167ffffffffffffffff811115616f50575f5ffd5b8201601f81018413616f60575f5ffd5b8051616f6e6168b982616841565b8082825260208201915060208360051b850101925086831115616f8f575f5ffd5b6020840193505b82841015611b88578351825260209384019390910190616f96565b5f60208284031215616fc1575f5ffd5b815167ffffffffffffffff811115616fd7575f5ffd5b8201601f81018413616fe7575f5ffd5b805167ffffffffffffffff811115617001576170016167fc565b617014601f8201601f1916602001616810565b818152856020838501011115617028575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215617055575f5ffd5b815161031781616d98565b6001815b600184111561709b5780850481111561707f5761707f616ee9565b600184161561708d57908102905b60019390931c928002617064565b935093915050565b5f826170b157506001610328565b816170bd57505f610328565b81600181146170d357600281146170dd576170f9565b6001915050610328565b60ff8411156170ee576170ee616ee9565b50506001821b610328565b5060208310610133831016604e8410600b841016171561711c575081810a610328565b6171285f198484617060565b805f190482111561713b5761713b616ee9565b029392505050565b5f61031783836170a3565b8082018082111561032857610328616ee9565b634e487b7160e01b5f52601260045260245ffd5b8181035f8312801583831316838312821617156141ad576141ad616ee9565b808202811582820484141761032857610328616ee9565b5f826171c557634e487b7160e01b5f52601260045260245ffd5b50049056fea26469706673582212201646eefa6c3541c4b9aebc7b77c5fa8fbe4b5ae0fc216d2e7e29865ef7feecea64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Par\0\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02\x84W\x80c\xD2\x8B\n\x15\x14a\x02\xB2W\x80c\xE35\xAD\xB7\x14a\x02\xC5W\x80c\xEE\xD0t(\x14a\x02\xD8W\x80c\xF6\x8Aq1\x14a\x02\xEBW__\xFD[\x80cs\x91\x18\xA4\x14a\x02+W\x80cx\xF2\x12\xD1\x14a\x02KW\x80c\x8Flz<\x14a\x02^W\x80c\xA6\xA1\0\xA0\x14a\x02qW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cW\xB9\x1C\xA6\x14a\x01\xE5W\x80cZoW\x10\x14a\x01\xF8W\x80c\\9\xF4g\x14a\x02\x0BW__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04afHV[a\x02\xFEV[`@Qa\x015\x91\x90af\x91V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04afHV[a\x03\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04ag|V[a\x03.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ag\xCAV[a\x03IV[a\x01\xB7a\x01\xB26`\x04ahdV[a\x03^V[`@Qa\x015\x91\x90ai\x0FV[a\x01\xD7a\x01\xD26`\x04afHV[a\x04\x17V[`@Q\x90\x81R` \x01a\x015V[a\x01\xD7a\x01\xF36`\x04afHV[a\x04!V[a\x01\xD7a\x02\x066`\x04afHV[a\x04+V[a\x02\x1Ea\x02\x196`\x04afHV[a\x045V[`@Qa\x015\x91\x90aj\xC9V[a\x02>a\x0296`\x04ak\xE8V[a\x04NV[`@Qa\x015\x91\x90al\xD2V[a\x01Qa\x02Y6`\x04afHV[a\x04iV[a\x02\x1Ea\x02l6`\x04ag\xCAV[a\x04sV[a\x02>a\x02\x7F6`\x04ahdV[a\x04\x80V[a\x02\x97a\x02\x926`\x04am\xA6V[a\x04\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x97a\x02\xC06`\x04am\xA6V[a\x04\xAEV[a\x01Qa\x02\xD36`\x04afHV[a\x04\xBEV[a\x01(a\x02\xE66`\x04ahdV[a\x04\xC8V[a\x01\xD7a\x02\xF96`\x04an\nV[a\x04\xD4V[``_a\x03\n\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x05\x80V[\x93\x92PPPV[_a\x03(\x82a\x05\xFCV[\x92\x91PPV[__a\x03<\x86\x86\x86\x86a\x06\xADV[\x91P\x91P\x94P\x94\x92PPPV[``a\x03V\x84\x84\x84a\x05\x80V[\x94\x93PPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xB4W\x81` \x01[a\x03\xA1a`\x95V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_a\x03\xE6\x86\x86\x84\x81Q\x81\x10a\x03\xD9Wa\x03\xD9anaV[` \x02` \x01\x01Qa\x06\xE9V[\x90P\x80\x83\x83\x81Q\x81\x10a\x03\xFBWa\x03\xFBanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x03\xB9V[P\x93\x92PPPV[_a\x03(\x82a\x19aV[_a\x03(\x82a\x19\xB2V[_a\x03(\x82a\x04\xEEV[``_a\x04A\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x1A\x03V[``_a\x04[\x84\x84a\x1A\xF7V[\x90Pa\x03V\x84\x84_\x84a\x1BmV[_a\x03(\x82a\x1B\x92V[``a\x03V\x84\x84\x84a\x1A\x03V[``a\x03\x17\x83\x83a\x1B\xCEV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1C\x84V[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1D\x8EV[_a\x03(\x82a\x1E\x07V[``a\x03\x17\x83\x83a\x1EXV[_a\x04\xE2\x86\x86\x86\x86\x86a\x1F3V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x05\r\x90anuV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90an\x98V[``a\x05\xB4`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x1C\xDD\x18\\\x9D`j\x1B\x81RPPV[a\x05\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x19[\x99`z\x1B\x81RPPV[_a\x05\xF0\x85\x85\x85a\x1FfV[\x90Pa\x04\xE5\x85\x82a \x07V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90an\xAFV[___a\x06\xBA\x86\x86a \xBDV[\x90P_a\x06\xC7\x88\x83a\x06\xE9V[\x90P__a\x06\xD6\x8A\x84\x89a!dV[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[a\x06\xF1a`\x95V[\x82a\x06\xFAa`\x95V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x07\x18\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x90\x91\x90an\xCAV[a\x07\x9DW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x07\xDD\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x80\x91\x90an\xAFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tq\x91\x90an\x98V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\t\xC7\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nj\x91\x90an\x98V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\n\xCB\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bn\x91\x90an\x98V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xD9\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CXW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C|\x91\x90an\x98V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xDD\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x80\x91\x90an\x98V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ELW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ep\x91\x90an\x98V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FX\x91\x90an\xAFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10A\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10\x98\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xFC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11;\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\x9D\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x01\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12@\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xAC\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x10\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13O\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xB1\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x140W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14T\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xAD\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15P\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x15\x9E\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16A\x91\x90an\xAFV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xAF\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17R\x91\x90an\xAFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\xB5\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x184W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18X\x91\x90an\x98V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x18\xB1\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x190W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19T\x91\x90an\x98V[`\x80\x82\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x1A\x11\x85\x85\x85a\x1FfV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A.Wa\x1A.ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AgW\x81` \x01[a\x1ATa`\xC9V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1ALW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x1A\xEDW_\x83\x82\x81Q\x81\x10a\x1A\x88Wa\x1A\x88anaV[` \x02` \x01\x01Q\x90Pa\x1A\xB8`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RPPV[_a\x1A\xC3\x89\x83a#]V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8anaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1AlV[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x1B\x10\x84a'\xFDV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B.\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x17\x91\x90an\x98V[``_a\x1B|\x86\x86\x86\x86a(\x81V[\x90Pa\x1B\x88\x86\x82a\x1B\xCEV[\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xEBWa\x1B\xEBag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C$W\x81` \x01[a\x1C\x11aaHV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1C\tW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1CEWa\x1CEanaV[` \x02` \x01\x01Q\x90P_a\x1CZ\x87\x83a)\tV[\x90P\x80\x84\x84\x81Q\x81\x10a\x1CoWa\x1CoanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1C)V[____a\x1C\x92\x88\x88a \xBDV[\x90P_a\x1C\x9F\x8A\x83a\x06\xE9V[\x90P_\x80\x80`\xFF\x89\x16a\x1C\xD5Wa\x1C\xB7\x8D\x8B\x86a/\x18V[\x92\x95P\x91\x93Pa\x1C\xCE\x91P\x85\x90P_\x85\x8D\x82a02V[\x90Pa\x1D\x05V[_\x19`\xFF\x8A\x16\x01a\x1D\x05Wa\x1C\xEB\x8D\x8B\x86a1\x14V[\x92\x95P\x91\x93Pa\x1D\x02\x91P\x85\x90P\x84_\x80\x8Ea02V[\x90P[\x80_\x03a\x1D\x1FW___\x97P\x97P\x97PPPPPPa\x04\xA3V[_a\x1D)\x85a2\x16V[\x90P_\x82\x82\x11a\x1DBWa\x1D=\x82\x84an\xFDV[a\x1DLV[a\x1DL\x83\x83an\xFDV[\x90P_a\x1DY\x82\x84a2\xC5V[\x90P_\x84\x84\x11a\x1DqWa\x1Dl\x82ao\x10V[a\x1DsV[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x1D\x9C\x88\x88a \xBDV[\x90P_a\x1D\xA9\x8A\x83a\x06\xE9V[\x90P_\x80\x80`\xFF\x89\x16a\x1D\xD9Wa\x1D\xC2\x8D\x8B\x86_a3\0V[\x92\x95P\x91\x93Pa\x1C\xCE\x91P\x85\x90P\x8B_\x80\x87a02V[_\x19`\xFF\x8A\x16\x01a\x1D\x05Wa\x1D\xF0\x8D\x8B\x86_a4*V[\x92\x95P\x91\x93Pa\x1D\x02\x91P\x85\x90P_\x8C\x86\x82a02V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81Rl3\xB2\xBA(7\xB7\xB69\xA4\xB737\x99`\x99\x1B` \x90\x91\x01R``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x9AWa\x1E\x9Aag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xD3W\x81` \x01[a\x1E\xC0aa\x9BV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xB8W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1E\xF4Wa\x1E\xF4anaV[` \x02` \x01\x01Q\x90P_a\x1F\t\x87\x83a59V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1F\x1EWa\x1F\x1EanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1E\xD8V[__a\x1F?\x86\x86a \xBDV[\x90P_a\x1FL\x88\x83a\x06\xE9V[\x90Pa\x1FZ\x81\x86\x86_a7\xF9V[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1F\x86\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03V\x91\x90\x81\x01\x90ao*V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a $Wa $ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a ]W\x81` \x01[a Jaa\x9BV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a BW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a ~Wa ~anaV[` \x02` \x01\x01Q\x90P_a \x93\x87\x83a59V[\x90P\x80\x84\x84\x81Q\x81\x10a \xA8Wa \xA8anaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a bV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a \xDEW\x81\x83a \xE1V[\x82\x82[`@Q\x91\x94P\x92Pa!\x0E\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[____a!paa\xC0V[a!y\x88a9SV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE7\x91\x90an\x98V[\x81` \x01\x81\x81RPPa\"\x01\x87__\x84a\x01@\x01Qa9\xA4V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"#\x90\x88\x90`\x01\x90_\x90a9\xA4V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"LW____\x94P\x94P\x94P\x94PPa#TV[a\"_\x86\x82`@\x01Q\x83` \x01Qa:\x89V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"z\x91\x88\x91a:\x89V[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x96P\x94P\x90\x92P\x90P[\x93P\x93P\x93P\x93V[a#ea`\xC9V[a#mab\x0FV[a#w\x84\x84a\x06\xE9V[\x81Ra#\x82\x84a;HV[` \x82\x01Ra#\x90\x84a9SV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa#\xAB\x92_\x91\x90a9\xA4V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa#\xD9\x92\x91`\x01\x91a9\xA4V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa#\xFB\x90a;\x8BV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$zW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xA1\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x12\x91\x90apEV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a%\xF2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\x19\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x8D\x91\x90apEV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a'm\x90a2\x16V[\x81R` \x01a'|\x86\x86a=\x9AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a'\xA3\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a'\xB4\x86\x86a>\x8AV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a'\xD8\x90\x87\x90_\x80a?\x8CV[\x81R` \x01a'\xEA\x86\x84_\x01Qa@~V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a(7\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a(\x9B\x86a'\xFDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xE2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90ao*V[a)\x11aaHV[a)\x19ab}V[a)#\x84\x84a@\x89V[\x80\x82RQ\x80QQa)<\x91`\x01[` \x02\x01QQa \xBDV[`@\x82\x01\x81\x90Ra)N\x90\x85\x90a\x06\xE9V[` \x82\x01\x81\x90R\x81Qa)b\x91\x86\x91a@\x9BV[PPPP``\x82\x01R` \x81\x01Qa)y\x90a2\x16V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa)\xAF\x91\x90_[` \x02\x01Q`@\x01Q\x90a@\xEBV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa)\xC7\x90_aA,V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa)\xE0\x91\x90aAZV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa)\xF7\x91\x90aA{V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa*\x18\x92\x91\x90aA\x97V[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa*9\x92\x87\x92\x90\x91_aA\xB4V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa*f\x92\x91\x90aA\x97V[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa*\x80\x90`\x01aA,V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa*\xB1\x91\x90`\x01a)\xA0V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa*\xC8\x91aAZV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa*\xE0\x91\x90aA{V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa*\xF8\x91\x90a@\xEBV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa+\x1B\x92\x91\x90aA\x97V[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa+=\x92\x87\x92\x90\x91`\x01aA\xB4V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa+l\x92\x91\x90aA\x97V[a\x02\xE0\x82\x01R\x80Qa+}\x90aCIV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a,uWa+\xA7\x81a\x03\0\x01Q\x82`\x80\x01QaAZV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa+\xBE\x91a@\xEBV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa+\xEF\x93aC\x8BV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa,V\x91\x86\x91a,\x17\x91\x90aA{V[a,)\x84`\xC0\x01Q\x85`\xA0\x01QaA{V[a,=\x85a\x02\0\x01Q\x86a\x01\xC0\x01QaA{V[a,Q\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01QaA{V[aC\xA5V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa,n\x91\x90aDeV[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x10\x91\x90\x81\x01\x90ao\xB1V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a-eWa-eanaV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a-\xB4Wa-\xB4anaV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x14W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.;\x91\x90\x81\x01\x90ao\xB1V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a/$acZV[a/-\x88a9SV[a\x01\xC0\x82\x01\x81\x90Ra/D\x90\x87\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa/c\x90\x87\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a/\x7FWP` \x81\x01Q\x15[\x15a/\x96W____\x94P\x94P\x94P\x94PPa#TV[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra/\xC5\x90\x88\x90a/\xBD\x90a'\x10\x90aD\xB4V[a'\x10a:\x89V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa/\xE8\x92a/\xE3\x90\x83\x90aD\xB4V[a:\x89V[`\x80\x82\x01\x81\x90R` \x82\x01Qa/\xFE\x91\x90aD\xB4V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0 \x90\x8B\x90aE\tV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a0BWP\x83\x15[\x15a0QWP\x83\x90P\x84a0\x86V[_\x87\x11\x80\x15a0^WP\x84\x15[\x15a0mWP\x85\x90P\x82a0\x86V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a0\x95\x89``\x01Q_aA,V[\x90P_a0\xA7\x8A``\x01Q`\x01aA,V[\x90P_a0\xC5\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a0\xE3\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P\x80_\x03a0\xFAW_\x96PPPPPPPa\x04\xE5V[a1\x04\x82\x82a2\xC5V[\x9C\x9BPPPPPPPPPPPPV[____a1 acZV[a1)\x88a9SV[a\x01\xC0\x82\x01\x81\x90Ra1@\x90\x87\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa1_\x90\x87\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a1{WP` \x81\x01Q\x15[\x15a1\x92W____\x94P\x94P\x94P\x94PPa#TV[\x80Q` \x82\x01Qa1\xA8\x91\x90a/\xE3\x81\x8BaD\xB4V[a\x01\0\x82\x01\x81\x90R\x81Qa1\xBC\x91\x90aD\xB4V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa1\xEF\x91a'\x10\x90a/\xE3\x90\x82\x90aD\xB4V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0 \x91aE\tV[__a2$\x83___a9\xA4V[PPP\x90P_a27\x84`\x01__a9\xA4V[PPP\x90P\x80_\x03a2LWP_\x93\x92PPPV[_a2[\x85``\x01Q_aA,V[\x90P_a2m\x86``\x01Q`\x01aA,V[\x90P_a2\x8B\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a2\xA9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P\x80_\x03a2\xBFWP_\x97\x96PPPPPPPV[a\x1FZ\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a2\xE6W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a3\x0CacZV[a3\x15\x89a9SV[a\x01\xC0\x82\x01\x81\x90Ra3,\x90\x88\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa3K\x90\x88\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a3gWP` \x81\x01Q\x15[\x15a3~W____\x94P\x94P\x94P\x94PPa4\x1FV[\x85\x15a3\x99W\x87\x81_\x01\x81\x81Qa3\x95\x91\x90an\xFDV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra3\xC0\x90\x89\x90a/\xBD\x90a'\x10\x90aD\xB4V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa3\xDE\x92a/\xE3\x90\x83\x90aE-V[`\x80\x82\x01\x81\x90R` \x82\x01Qa3\xF3\x91aD\xB4V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa4\x15\x90\x8C\x90aE\tV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a46acZV[a4?\x89a9SV[a\x01\xC0\x82\x01\x81\x90Ra4V\x90\x88\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4u\x90\x88\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a4\x91WP` \x81\x01Q\x15[\x15a4\xA8W____\x94P\x94P\x94P\x94PPa4\x1FV[\x85\x15a4\xC4W\x87\x81` \x01\x81\x81Qa4\xC0\x91\x90an\xFDV[\x90RP[\x80Q` \x82\x01Qa4\xDA\x91\x90a/\xE3\x81\x8CaE-V[`\x80\x82\x01\x81\x90R\x81Qa4\xEC\x91aD\xB4V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa5\x1B\x91a/\xBD\x90a'\x10\x90aD\xB4V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa4\x15\x91aE\tV[a5Aaa\x9BV[a5Iac\xC1V[a5S\x84\x84a\x06\xE9V[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a5\x88W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a5\xF9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6 \x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a6mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x91\x91\x90apEV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\x11W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra78\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xAC\x91\x90apEV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a7\xE7\x86\x84_\x01Qa@~V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a8\x02aa\xC0V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8f\x91\x90an\x98V[` \x82\x01Ra8w\x86_\x80\x80a9\xA4V[PPP`\xC0\x82\x01Ra8\x8C\x86`\x01_\x80a9\xA4V[PPP`\xE0\x82\x01R\x82\x15a8\xC7W\x84\x81`\xC0\x01\x81\x81Qa8\xAC\x91\x90an\xFDV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a8\xC3\x90\x83\x90an\xFDV[\x90RP[`\xC0\x81\x01Q\x15\x80a8\xDAWP`\xE0\x81\x01Q\x15[\x15a8\xE8W_\x91PPa\x03VV[\x80` \x01Q_\x03a9\x18Wa9\x11a\x03\xE8a9\x0Ba9\x06\x88\x88aE\x81V[aE\xE7V[\x90aD\xB4V[\x81Ra9IV[a9Fa9.\x86\x83` \x01Q\x84`\xC0\x01Qa:\x89V[a9A\x86\x84` \x01Q\x85`\xE0\x01Qa:\x89V[aF\xC7V[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a9\xC1Wa9\xC1anaV[` \x02\x01Q\x90P_a9\xD3\x8A\x8AaF\xDFV[\x90P\x80_\x03a9\xEFW____\x95P\x95P\x95P\x95PPPa4\x1FV[_a9\xFE\x83\x8C`\x80\x01QaG\xCCV[\x90P_a:\x0B\x82\x8Aa@\xEBV[\x90P_\x89\x15a:0W\x81\x84\x10a:*Wa:%\x84\x83aD\xB4V[a:2V[_a:2V[_[\x90P_a:?\x85\x8Da@\xEBV[\x90P_\x8C\x15a:dW\x84\x82\x10a:^Wa:Y\x82\x86aD\xB4V[a:fV[_a:fV[_[\x90Pa:r\x85\x87aqNV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a:\xBDW\x83\x82\x81a:\xB3Wa:\xB3aqaV[\x04\x92PPPa\x03\x17V[\x80\x84\x11a:\xDDW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a;\xC0`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a;\xCA\x84_aI\x15V[` \x83\x01R\x81R``\x84\x01Qa;\xE0\x90_aA,V[``\x82\x01\x81\x90R\x81Qa<\x05\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a/\xE3\x90`\naqCV[`@\x82\x01R` \x81\x01Q_\x03a< W_`\x80\x82\x01Ra<\xC0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xBA\x91\x90an\x98V[`\x80\x82\x01R[a<\xCB\x84`\x01aI\x15V[` \x83\x01\x81\x90R\x90\x82R_\x03a<\xE6W_`\xA0\x82\x01Ra=\x86V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\x80\x91\x90an\x98V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a=\xA6\x84\x84aI[V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a=\xE7\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\x17\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>K\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>fW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90an\xAFV[__a>\x96\x84\x84aI[V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a>\xE9\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?M\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90an\x98V[__a?\x99\x85`\x01aF\xDFV[\x90P\x82\x15a?\xAEWa?\xAB\x84\x82an\xFDV[\x90P[_a?\xB8\x87aJ\x16V[\x90P_a?\xC5\x83\x83a@\xEBV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10a?\xF3W\x87Q` \x01Q``\x01Qa?\xEE\x90\x83an\xFDV[a?\xF5V[_[\x90Pa@!`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RPPV[a@N`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RPPV[a\x1FZ`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RPPV[_a\x03\x17\x83\x83aJZV[a@\x91ac\xE1V[a\x03\x17\x83\x83aJsV[_____a@\xAA\x88\x88a@~V[\x90Pa@\xB8\x87\x87\x83_a\\\x91V[\x90\x93P\x91P\x81a@\xC9W_\x19a@\xD3V[a@\xD3\x83\x83a2\xC5V[\x94Pa@\xDE\x88a\x19\xB2V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aA\x0BW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aALWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11aAqWaAl\x83\x83an\xFDV[a\x03\x17V[a\x03\x17\x82\x84an\xFDV[_a\x03\x17\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[_\x82\x84\x11aA\xADWaA\xA8\x82ao\x10V[a\x03VV[P\x92\x91PPV[_aA\xEE`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\xFA\x86\x86\x86_a\\\x91V[` \x83\x01R\x80\x82R\x15\x80aB+WP\x84Q`\xFF\x84\x16`\x02\x81\x10aB\x1FWaB\x1FanaV[` \x02\x01Q` \x01Q_\x14[\x15aB9W_\x91PPa\x04\xE5V[aBB\x87a]\xFDV[`@\x82\x01\x81\x90R` \x82\x01QaBW\x91a@\xEBV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15aBoW_\x91PPa\x04\xE5V[`\x80\x81\x01Q\x81QaB\x80\x91\x90an\xFDV[\x81``\x01\x81\x81RPPaB\x97\x86``\x01Q\x84aA,V[`\xA0\x82\x01\x81\x90R``\x82\x01QaB\xC3\x91aB\xB2\x90`\naqCV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba:\x89V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01aB\xE8W`\xC0\x81\x01QaB\xE2\x90\x85a2\xC5V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10aB\xFEWaB\xFEanaV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15aC;W\x84Q`\xFF\x84\x16`\x02\x81\x10aC(WaC(anaV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01aCiWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01QaC\x84WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x04\x0FWaC\xA0\x82ao\x10V[a\x04\xE5V[__aC\xB0\x87a\x19\xB2V[\x90P_aC\xBD\x82\x87a@\xEBV[\x90P_aC\xCA\x83\x86a@\xEBV[\x90P_aC\xD7\x89\x84aquV[\x90P_aC\xE4\x83\x89aquV[\x90P_aC\xF0\x83a^CV[\x90P_aC\xFC\x83a^CV[\x90P_\x84\x13\x80\x15aD\x0CWP_\x83\x12[\x80aD WP_\x84\x12\x80\x15aD WP_\x83\x13[\x15aD4W_\x97PPPPPPPPa\x04\xE5V[\x80_\x03aDJW_\x97PPPPPPPPa\x04\xE5V[aDT\x82\x82a2\xC5V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aDtWP_a\x03(V[_\x82\x84\x11aD\x8BWaD\x86\x84\x84an\xFDV[aD\x95V[aD\x95\x83\x85an\xFDV[\x90P_aD\xA2\x82\x85a2\xC5V[\x90P\x83\x85\x11a\x03VWaC\xA0\x81ao\x10V[_\x82aD\xC0\x83\x82an\xFDV[\x91P\x81\x11\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aE\x1FW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aE9\x83\x82aqNV[\x91P\x81\x10\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81\x15\x80aE\xA4WP\x82\x82aE\x96\x81\x83aq\x94V[\x92PaE\xA2\x90\x83aq\xABV[\x14[a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81_\x03aE\xF6WP_\x91\x90PV[_`\x01aF\x02\x84a^XV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aF\x1BWaF\x1BaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF3WaF3aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aFKWaFKaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aFcWaFcaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF{WaF{aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF\x93WaF\x93aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF\xABWaF\xABaqaV[\x04\x82\x01\x90\x1C\x90Pa\x03\x17\x81\x82\x85\x81aF\xC5WaF\xC5aqaV[\x04[_\x81\x83\x10aF\xD5W\x81a\x03\x17V[P\x90\x91\x90PV[PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aF\xF9WaF\xF9anaV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGv\x91\x90an\x98V[\x90P\x80_\x03aG\x89W_\x92PPPa\x03(V[``\x82\x01Q`\xC0\x83\x01QaG\x9D\x90\x82aqNV[\x82\x10aG\xC1W`\xC0\x83\x01QaG\xB2\x82\x84an\xFDV[aG\xBC\x91\x90an\xFDV[a\x1B\x88V[_\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aG\xDFWP_a\x03(V[_aG\xEA\x84\x84a^\xEBV[`\xA0\x85\x01Q\x90\x91Pa\x03V\x90\x82a@\xEBV[\x84Q\x81\x10\x15a\x04\x0FW\x82`\x04\x86\x83\x81Q\x81\x10aH\x1AWaH\x1AanaV[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10aHBWaHBanaV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82aH]\x83`\x02aq\x94V[aHh\x90`\x02aqNV[\x81Q\x81\x10aHxWaHxanaV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10aH\xA1WaH\xA1anaV[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10aH\xC1WaH\xC1anaV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82aH\xDC\x83`\x02aq\x94V[aH\xE7\x90`\x03aqNV[\x81Q\x81\x10aH\xF7WaH\xF7anaV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01aG\xFCV[___aIB\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aI3WaI3anaV[` \x02\x01Q\x86`\x80\x01QaG\xCCV[\x90P_aIO\x86\x86aF\xDFV[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aI~\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xF6\x91\x90an\xCAV[a\x03\x17W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a5\x7FV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x1B\x10\x84a_.V[aJ{ac\xE1V[\x82aJ\x84ac\xE1V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aJ\xC4\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK<\x91\x90an\xCAV[aKIW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK\x83\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL&\x91\x90an\x98V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aLn\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x11\x91\x90an\xAFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aMm\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x9D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xD1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\x10\x91\x90an\xAFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\x8B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xBF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xFE\x91\x90an\x98V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOR\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\x82\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xB6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xF5\x91\x90an\x98V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPO\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xF2\x91\x90an\x98V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQK\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ{\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xAF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xEE\x91\x90an\x98V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aRt\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xA8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\xE7\x91\x90an\x98V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aSA\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aSq\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xA5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xE4\x91\x90an\x98V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTW\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xCA\x91\x90an\x98V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU>\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUr\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xB1\x91\x90an\xAFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVX\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVsW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x97\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xEC\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aWkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\x8F\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xEA\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aXiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x8D\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\xE7\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY\x17\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYK\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\x8A\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\xEC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\x8F\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\xEA\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[N\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\x8D\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\xDC\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\x0C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\\x7F\x91\x90an\x98V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]4W__a\\\xBC\x8A\x8A_a_\xC9V[\x91P\x91P_a\\\xD8_\x8C``\x01QaA,\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\xF6\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[\x90P_a]\x14\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90Pa] \x82\x88aqNV[\x96Pa],\x81\x87aqNV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]\xF0W__a][\x8A\x8A`\x01a_\xC9V[\x91P\x91P_a]x`\x01\x8C``\x01QaA,\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a]\x96\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[\x90P_a]\xB4\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a]\xC1\x83\x8Da@\xEBV[\x90P_a]\xCE\x83\x8Ea@\xEBV[\x90Pa]\xDA\x82\x8AaqNV[\x98Pa]\xE6\x81\x89aqNV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a^TW\x81_\x03a\x03(V[P\x90V[_\x80`\x80\x83\x90\x1C\x15a^lW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a^~W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a^\x90W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a^\xA2W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a^\xB4W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a^\xC6W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a^\xD8W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03(W`\x01\x01\x92\x91PPV[_B\x82\x03a^\xFEWP` \x82\x01Qa\x03(V[_a_\r\x84`@\x01Q\x84a`aV[\x90Pa_&\x84` \x01Q\x82a@\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03(V[\x80Q\x80QQ_\x91\x82\x91a_B\x91`\x01a)1V[\x90P\x80`@Q` \x01a_{\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10a_\xE4Wa_\xE4anaV[` \x02\x01Q`@\x01Q\x90P_a`\x1A\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10a`\x0BWa`\x0BanaV[` \x02\x01Q\x88`\x80\x01Qa^\xEBV[\x90P\x81\x15a`1Wa`,\x82\x82a@\xEBV[a`3V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10a`LWa`LanaV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80a`m\x83Ban\xFDV[a`w\x90\x85aq\x94V[c\x01\xE13\x80\x90\x04\x90Pa\x03V\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaqNV[`@Q\x80`\xA0\x01`@R\x80a`\xA8ad\x07V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xA0\x01`@R\x80a`\xDDadnV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aa\\ad\xF5V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aa\xAEaepV[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ab#a`\x95V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01abxa`\xC9V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80ab\x91ac\xE1V[\x81R` \x01ab\x9Ea`\x95V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80ac\xD4a`\x95V[\x81R` \x01abxaa\x9BV[`@Q\x80``\x01`@R\x80ac\xF4ae\xC6V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[adX`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ad\x16W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ad\xDF`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ad}W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aeZ`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\x04W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ae\xB0`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\x7FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\x1E`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xD5W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aF\xDCW__\xFD[_` \x82\x84\x03\x12\x15afXW__\xFD[\x815a\x03\x17\x81af4V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15agCW\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ag\x17\x90\x87\x01\x82afcV[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01af\xDBV[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01af\xB7V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15ag\x8FW__\xFD[\x845ag\x9A\x81af4V[\x93P` \x85\x015ag\xAA\x81af4V[\x92P`@\x85\x015ag\xBA\x81af4V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ag\xDCW__\xFD[\x835ag\xE7\x81af4V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15ah9Wah9ag\xFCV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ahZWahZag\xFCV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ahuW__\xFD[\x825ah\x80\x81af4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\x9BW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13ah\xABW__\xFD[\x805ah\xBEah\xB9\x82ahAV[ah\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ah\xDFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15ai\x01W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ah\xE6V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15ai\xF3W\x83Q\x80Q\x84_[`\x02\x81\x10\x15ai\xA2W\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pai7V[PPP` \x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x87\x01R`@\x83\x01Q\x16a\x01\xE0\x86\x01R``\x82\x01Qa\x02\0\x86\x01R`\x80\x90\x91\x01Qa\x02 \x85\x01R\x93\x90\x93\x01\x92a\x02@\x90\x92\x01\x91`\x01\x01ai(V[P\x90\x95\x94PPPPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ai\xF3W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Raj?a\x01\x80\x86\x01\x82afcV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Paj\x07V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Rak\x17a\x01\xA0\x88\x01\x82ai\xFEV[\x90P` \x82\x01Qak3` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QakN`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qak\x91`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qak\xA7a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aj\xEFV[__`@\x83\x85\x03\x12\x15ak\xF9W__\xFD[\x825al\x04\x81af4V[\x91P` \x83\x015al\x14\x81af4V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ai\xF3W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ral`a\x01@\x86\x01\x82afcV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pal(V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ram a\x01 \x88\x01\x82al\x1FV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QamF`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01al\xF8V[`\xFF\x81\x16\x81\x14aF\xDCW__\xFD[_____`\xA0\x86\x88\x03\x12\x15am\xBAW__\xFD[\x855am\xC5\x81af4V[\x94P` \x86\x015am\xD5\x81af4V[\x93P`@\x86\x015am\xE5\x81af4V[\x92P``\x86\x015\x91P`\x80\x86\x015am\xFC\x81am\x98V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15an\x1EW__\xFD[\x855an)\x81af4V[\x94P` \x86\x015an9\x81af4V[\x93P`@\x86\x015anI\x81af4V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15an\xA8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15an\xBFW__\xFD[\x81Qa\x03\x17\x81af4V[_` \x82\x84\x03\x12\x15an\xDAW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x17W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03(Wa\x03(an\xE9V[_`\x01`\xFF\x1B\x82\x01ao$Wao$an\xE9V[P_\x03\x90V[_` \x82\x84\x03\x12\x15ao:W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aoPW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ao`W__\xFD[\x80Qaonah\xB9\x82ahAV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15ao\x8FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1B\x88W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ao\x96V[_` \x82\x84\x03\x12\x15ao\xC1W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ao\xD7W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ao\xE7W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\x01Wap\x01ag\xFCV[ap\x14`\x1F\x82\x01`\x1F\x19\x16` \x01ah\x10V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ap(W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15apUW__\xFD[\x81Qa\x03\x17\x81am\x98V[`\x01\x81[`\x01\x84\x11\x15ap\x9BW\x80\x85\x04\x81\x11\x15ap\x7FWap\x7Fan\xE9V[`\x01\x84\x16\x15ap\x8DW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02apdV[\x93P\x93\x91PPV[_\x82ap\xB1WP`\x01a\x03(V[\x81ap\xBDWP_a\x03(V[\x81`\x01\x81\x14ap\xD3W`\x02\x81\x14ap\xDDWap\xF9V[`\x01\x91PPa\x03(V[`\xFF\x84\x11\x15ap\xEEWap\xEEan\xE9V[PP`\x01\x82\x1Ba\x03(V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aq\x1CWP\x81\x81\na\x03(V[aq(_\x19\x84\x84ap`V[\x80_\x19\x04\x82\x11\x15aq;Waq;an\xE9V[\x02\x93\x92PPPV[_a\x03\x17\x83\x83ap\xA3V[\x80\x82\x01\x80\x82\x11\x15a\x03(Wa\x03(an\xE9V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aA\xADWaA\xADan\xE9V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03(Wa\x03(an\xE9V[_\x82aq\xC5WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x16F\xEE\xFAl5A\xC4\xB9\xAE\xBC{w\xC5\xFA\x8F\xBEKZ\xE0\xFC!m.~)\x86^\xF7\xFE\xEC\xEAdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610111575f3560e01c8063739118a41161009e578063c2bdeda11161006e578063c2bdeda114610284578063d28b0a15146102b2578063e335adb7146102c5578063eed07428146102d8578063f68a7131146102eb575f5ffd5b8063739118a41461022b57806378f212d11461024b5780638f6c7a3c1461025e578063a6a100a014610271575f5ffd5b806350376aaa116100e457806350376aaa146101a457806350ed592d146101c457806357b91ca6146101e55780635a6f5710146101f85780635c39f4671461020b575f5ffd5b80631a3081751461011557806328a0ccf41461013e578063317b50ec14610169578063382fc72e14610191575b5f5ffd5b610128610123366004616648565b6102fe565b6040516101359190616691565b60405180910390f35b61015161014c366004616648565b61031e565b6040516001600160a01b039091168152602001610135565b61017c61017736600461677c565b61032e565b60408051928352602083019190915201610135565b61012861019f3660046167ca565b610349565b6101b76101b2366004616864565b61035e565b604051610135919061690f565b6101d76101d2366004616648565b610417565b604051908152602001610135565b6101d76101f3366004616648565b610421565b6101d7610206366004616648565b61042b565b61021e610219366004616648565b610435565b6040516101359190616ac9565b61023e610239366004616be8565b61044e565b6040516101359190616cd2565b610151610259366004616648565b610469565b61021e61026c3660046167ca565b610473565b61023e61027f366004616864565b610480565b610297610292366004616da6565b61048c565b60408051938452602084019290925290820152606001610135565b6102976102c0366004616da6565b6104ae565b6101516102d3366004616648565b6104be565b6101286102e6366004616864565b6104c8565b6101d76102f9366004616e0a565b6104d4565b60605f61030a836104ee565b9050610317835f83610580565b9392505050565b5f610328826105fc565b92915050565b5f5f61033c868686866106ad565b9150915094509492505050565b6060610356848484610580565b949350505050565b60605f825167ffffffffffffffff81111561037b5761037b6167fc565b6040519080825280602002602001820160405280156103b457816020015b6103a1616095565b8152602001906001900390816103995790505b5090505f5b835181101561040f575f6103e6868684815181106103d9576103d9616e61565b60200260200101516106e9565b9050808383815181106103fb576103fb616e61565b6020908102919091010152506001016103b9565b509392505050565b5f61032882611961565b5f610328826119b2565b5f610328826104ee565b60605f610441836104ee565b9050610317835f83611a03565b60605f61045b8484611af7565b905061035684845f84611b6d565b5f61032882611b92565b6060610356848484611a03565b60606103178383611bce565b5f5f5f61049c8888888888611c84565b9250925092505b955095509592505050565b5f5f5f61049c8888888888611d8e565b5f61032882611e07565b60606103178383611e58565b5f6104e28686868686611f33565b90505b95945050505050565b5f816001600160a01b031663f3903b9f60405160200161050d90616e75565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161054191815260200190565b602060405180830381865afa15801561055c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190616e98565b60606105b46040518060400160405280601381526020017219d95d141bdbdb1cd25b999bcc081cdd185c9d606a1b81525050565b6105e46040518060400160405280601181526020017019d95d141bdbdb1cd25b999bcc08195b99607a1b81525050565b5f6105f0858585611f66565b90506104e58582612007565b5f816001600160a01b03166321f8a72160405160200161063a906020808252600a9082015269544f4b454e5f4241534560b01b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161066e91815260200190565b602060405180830381865afa158015610689573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103289190616eaf565b5f5f5f6106ba86866120bd565b90505f6106c788836106e9565b90505f5f6106d68a8489612164565b50919c909b509950505050505050505050565b6106f1616095565b826106fa616095565b816001600160a01b03166391d4403c60405160200161071890616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa15801561076c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107909190616eca565b61079d5791506103289050565b816001600160a01b03166321f8a721856040516020016107dd906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161080d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161084191815260200190565b602060405180830381865afa15801561085c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108809190616eaf565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f5908690608001604051602081830303815290604052805190602001206040516020016108fe929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161093291815260200190565b602060405180830381865afa15801561094d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109719190616e98565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016109c7906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b604051602081830303815290604052805190602001206040516020016109f7929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610a2b91815260200190565b602060405180830381865afa158015610a46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a6a9190616e98565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001610acb9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f30000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610afb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610b2f91815260200190565b602060405180830381865afa158015610b4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b6e9190616e98565b81515f60200201516060018181525050816001600160a01b031663bd02d0f585604051602001610bd99060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60405160208183030381529060405280519060200120604051602001610c09929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610c3d91815260200190565b602060405180830381865afa158015610c58573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c7c9190616e98565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001610cdd9060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f300000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001610d0d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610d4191815260200190565b602060405180830381865afa158015610d5c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d809190616e98565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001610dfd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610e3191815260200190565b602060405180830381865afa158015610e4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e709190616e98565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001610ee5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401610f1991815260200190565b602060405180830381865afa158015610f34573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f589190616eaf565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161100291815260200190565b602060405180830381865afa15801561101d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110419190616e98565b8151600160200201516020018181525050816001600160a01b031663bd02d0f58560405160200161109890602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b604051602081830303815290604052805190602001206040516020016110c8929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016110fc91815260200190565b602060405180830381865afa158015611117573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061113b9190616e98565b8151600160200201516040018181525050816001600160a01b031663bd02d0f58560405160200161119d9060208082526017908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f31000000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016111cd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161120191815260200190565b602060405180830381865afa15801561121c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112409190616e98565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016112ac9060208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604051602081830303815290604052805190602001206040516020016112dc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161131091815260200190565b602060405180830381865afa15801561132b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061134f9190616e98565b8151600160200201516080018181525050816001600160a01b031663bd02d0f5856040516020016113b19060208082526018908201527f504f4f4c5f544f54414c5f5343414c45445f444542545f310000000000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016113e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161141591815260200190565b602060405180830381865afa158015611430573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114549190616e98565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016114ad90602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b604051602081830303815290604052805190602001206040516020016114dd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161151191815260200190565b602060405180830381865afa15801561152c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115509190616e98565b81516001602002015160c0018181525050816001600160a01b03166321f8a7218560405160200161159e90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115ce929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161160291815260200190565b602060405180830381865afa15801561161d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116419190616eaf565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a721856040516020016116af906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016116df929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161171391815260200190565b602060405180830381865afa15801561172e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117529190616eaf565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f5856040516020016117b5906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016117e5929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161181991815260200190565b602060405180830381865afa158015611834573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118589190616e98565b60608201526040516001600160a01b0383169063bd02d0f59086906118b1906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016118e1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161191591815260200190565b602060405180830381865afa158015611930573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119549190616e98565b6080820152949350505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601a908201527f44454641554c545f504f4f4c5f434f4e46494755524154494f4e000000000000604082015260600190565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b60605f611a11858585611f66565b90505f815167ffffffffffffffff811115611a2e57611a2e6167fc565b604051908082528060200260200182016040528015611a6757816020015b611a546160c9565b815260200190600190039081611a4c5790505b5090505f5b8251811015611aed575f838281518110611a8857611a88616e61565b60200260200101519050611ab860405180604001604052806007815260200166706f6f6c4b657960c81b81525050565b5f611ac3898361235d565b905080848481518110611ad857611ad8616e61565b60209081029190910101525050600101611a6c565b5095945050505050565b5f826001600160a01b031663f3903b9f611b10846127fd565b6040518263ffffffff1660e01b8152600401611b2e91815260200190565b602060405180830381865afa158015611b49573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103179190616e98565b60605f611b7c86868686612881565b9050611b888682611bce565b9695505050505050565b5f816001600160a01b03166321f8a72160405160200161063a90602080825260089082015267545245415355525960c01b604082015260600190565b60605f825167ffffffffffffffff811115611beb57611beb6167fc565b604051908082528060200260200182016040528015611c2457816020015b611c11616148565b815260200190600190039081611c095790505b5090505f5b835181101561040f575f848281518110611c4557611c45616e61565b602002602001015190505f611c5a8783612909565b905080848481518110611c6f57611c6f616e61565b60209081029190910101525050600101611c29565b5f5f5f5f611c9288886120bd565b90505f611c9f8a836106e9565b90505f808060ff8916611cd557611cb78d8b86612f18565b929550919350611cce91508590505f858d82613032565b9050611d05565b5f1960ff8a1601611d0557611ceb8d8b86613114565b929550919350611d029150859050845f808e613032565b90505b805f03611d1f575f5f5f97509750975050505050506104a3565b5f611d2985613216565b90505f828211611d4257611d3d8284616efd565b611d4c565b611d4c8383616efd565b90505f611d5982846132c5565b90505f848411611d7157611d6c82616f10565b611d73565b815b969b5094995094975050505050505050955095509592505050565b5f5f5f5f611d9c88886120bd565b90505f611da98a836106e9565b90505f808060ff8916611dd957611dc28d8b865f613300565b929550919350611cce91508590508b5f8087613032565b5f1960ff8a1601611d0557611df08d8b865f61342a565b929550919350611d0291508590505f8c8682613032565b5f816001600160a01b03166321f8a72160405160200161063a906020808252601e908201527f44454641554c545f494e5445524553545f524154455f53545241544547590000604082015260600190565b60408051808201909152600d81526c33b2ba2837b7b639a4b733379960991b60209091015260605f825167ffffffffffffffff811115611e9a57611e9a6167fc565b604051908082528060200260200182016040528015611ed357816020015b611ec061619b565b815260200190600190039081611eb85790505b5090505f5b835181101561040f575f848281518110611ef457611ef4616e61565b602002602001015190505f611f098783613539565b905080848481518110611f1e57611f1e616e61565b60209081029190910101525050600101611ed8565b5f5f611f3f86866120bd565b90505f611f4c88836106e9565b9050611f5a8186865f6137f9565b98975050505050505050565b6060836001600160a01b031663f069052a604051602001611f8690616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604481018590526064015f60405180830381865afa158015611fe0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103569190810190616f2a565b60605f825167ffffffffffffffff811115612024576120246167fc565b60405190808252806020026020018201604052801561205d57816020015b61204a61619b565b8152602001906001900390816120425790505b5090505f5b835181101561040f575f84828151811061207e5761207e616e61565b602002602001015190505f6120938783613539565b9050808484815181106120a8576120a8616e61565b60209081029190910101525050600101612062565b5f816001600160a01b0316836001600160a01b0316106120de5781836120e1565b82825b604051919450925061210e906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03808616918301919091528316606082015260800160405160208183030381529060405280519060200120905092915050565b5f5f5f5f6121706161c0565b61217988613953565b8161014001818152505086602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156121c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121e79190616e98565b816020018181525050612201875f5f8461014001516139a4565b5060808401525060408201526101408101516122239088906001905f906139a4565b5060a084015250606082015260208101515f0361224c575f5f5f5f945094509450945050612354565b61225f8682604001518360200151613a89565b6101008201526060810151602082015161227a918891613a89565b6101208201908152604080518082018252601081526f766172732e746f74616c537570706c7960801b602091820152815180830183526012808252710766172732e707269636552657365727665360741b918301919091528251808401845290815271766172732e7072696365526573657276653160701b9082015281518083018352600c8082526b0766172732e616d6f756e74360a41b91830191909152825180840190935282526b766172732e616d6f756e743160a01b9101526101008201519051608083015160a090930151919650945090925090505b93509350935093565b6123656160c9565b61236d61620f565b61237784846106e9565b815261238284613b48565b602082015261239084613953565b6101808201819052815160208301516123ab925f91906139a4565b608085015260a084015260408301526060820152805160208201516101808301516123d992916001916139a4565b61012085015261014084015260e083015261010082015280516123fb90613b8b565b61016083015260c0820152604080516103608101825282515151516001600160a01b039081166101e08301908152845151515184516395d89b4160e01b81529451939485946101a08601948594936102008801939116916395d89b41916004808201925f929091908290030181865afa15801561247a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124a19190810190616fb1565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa1580156124ee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125129190617045565b60ff168152865151516020908101518183015260c08089015160408085019190915289515151606090810151818601528a515151608090810151818701528b51515160a09081015181880152838d015194870194909452908b015160e08601528a01516101008501529089015161012090930192909252918352805161018081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa1580156125f2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526126199190810190616fb1565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015612669573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061268d9190617045565b60ff1681528651516020908101518101518183015261016088015160408084019190915288515182015160609081015181850152895151830151608090810151818601528a515184015160a0908101518187015260e0808d015160c080890191909152610100808f015192890192909252610120808f0151928901929092526101408e015191909701529590965295875288518201516001600160a01b0390811692880192909252885181015190911690860152865184015193850193909352855182015191840191909152601b90830152835191019061276d90613216565b815260200161277c8686613d9a565b6001600160a01b031681526020016127a3835f015160600151660800000000000016151590565b151581526020016127b48686613e8a565b81528251515160c00151602082015282516040909101906127d89087905f80613f8c565b81526020016127ea86845f015161407e565b90526101a0909101819052905092915050565b5f604051602001612837906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b6060846001600160a01b031663f069052a61289b866127fd565b6040516001600160e01b031960e084901b168152600481019190915260248101869052604481018590526064015f60405180830381865afa1580156128e2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104e59190810190616f2a565b612911616148565b61291961627d565b6129238484614089565b8082525180515161293c9160015b6020020151516120bd565b6040820181905261294e9085906106e9565b60208201819052815161296291869161409b565b505050506060820152602081015161297990613216565b610300820152805180515160209081015160e0840152808301515151015190516129af91905f5b602002015160400151906140eb565b60c08201526020810151606001516129c7905f61412c565b60a082015260e081015160c08201516129e0919061415a565b610100820181905260a08201516129f7919061417b565b61012082015260e081015160c0820151610100830151612a18929190614197565b61014082015260208101518151610300830151612a3992879290915f6141b4565b61016082015261014081015161018082015260e081015160c0820151610120830151612a66929190614197565b6101a0820152602081015160600151612a8090600161412c565b6101c082015280518051602090810151810151610200840152808301515181015101519051612ab1919060016129a0565b6101e08201819052610200820151612ac89161415a565b61022082018190526101c0820151612ae0919061417b565b6102408201819052610300820151612af891906140eb565b6102608201526102008101516101e0820151610220830151612b1b929190614197565b61028082015260208101518151610300830151612b3d928792909160016141b4565b6102a08201526102808101516102c08201526102008101516101e0820151610260830151612b6c929190614197565b6102e08201528051612b7d90614349565b60808201528051516020015160e00151600214612c7557612ba7816103000151826080015161415a565b6103208201819052610240820151612bbe916140eb565b610340820181905260808201516103008301511161038083018190526102008301516101e0840151612bef9361438b565b61036082018190526103a082015260e081015160a0820151612c56918691612c17919061417b565b612c298460c001518560a0015161417b565b612c3d856102000151866101c0015161417b565b612c51866101e00151876101c0015161417b565b6143a5565b6103c08201819052610300820151612c6e9190614465565b6103e08201525b604080516102a08101825282515151516001600160a01b039081166101608301908152845151515184516395d89b4160e01b81529451939485946101208601948594936101808801939116916395d89b41916004808201925f929091908290030181865afa158015612ce9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d109190810190616fb1565b81526020018660a0015181526020018660e0015181526020018660c0015181526020018661014001518152602001866101600151815260200186602001515f01515f60ff1660028110612d6557612d65616e61565b60200201516040015181526020018661018001518152602001866101a001518152508152602001604051806101400160405280865f01515f0151600160ff1660028110612db457612db4616e61565b60209081029190910151516001600160a01b03168252875151910190600160200201515f01516001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612e14573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612e3b9190810190616fb1565b81526101c087015160208083019190915261020088015160408301526101e0880151606083015261028088015160808301526102a088015160a08301528701515160c09091019060016020020151604001518152602001866102c001518152602001866102e001518152508152508152602001825f0151602001518152602001825f0151604001516001600160a01b0316815260200182606001518152602001826080015181526020018261030001518152602001826103a001518152602001826103c001518152602001826103e0015181525091505092915050565b5f5f5f5f612f2461635a565b612f2d88613953565b6101c08201819052612f449087905f9081906139a4565b5060408401525081526101c0810151612f639087906001905f906139a4565b50606084015250602082015280511580612f7f57506020810151155b15612f96575f5f5f5f945094509450945050612354565b606086015160381c61ffff166101408201819052612fc5908890612fbd90612710906144b4565b612710613a89565b610180820181905281516020830151612fe892612fe39083906144b4565b613a89565b608082018190526020820151612ffe91906144b4565b60e0820181905260408201516060830151610140840151613020908b90614509565b94509450945094505093509350935093565b5f5f5f5f86118015613042575083155b15613051575083905084613086565b5f8711801561305e575084155b1561306d575085905082613086565b604051636331fab160e01b815260040160405180910390fd5b5f61309589606001515f61412c565b90505f6130a78a60600151600161412c565b90505f6130c585676765c793fa10079d601b1b612fe386600a617143565b90505f6130e385676765c793fa10079d601b1b612fe386600a617143565b9050805f036130fa575f96505050505050506104e5565b61310482826132c5565b9c9b505050505050505050505050565b5f5f5f5f61312061635a565b61312988613953565b6101c082018190526131409087905f9081906139a4565b5060408401525081526101c081015161315f9087906001905f906139a4565b5060608401525060208201528051158061317b57506020810151155b15613192575f5f5f5f945094509450945050612354565b805160208201516131a89190612fe3818b6144b4565b610100820181905281516131bc91906144b4565b610120820152606086015160381c61ffff1661014082018190526101208201516131ef9161271090612fe39082906144b4565b6101a082018190526040820151606083015161014084015161012085015161302091614509565b5f5f613224835f5f5f6139a4565b50505090505f6132378460015f5f6139a4565b5050509050805f0361324c57505f9392505050565b5f61325b85606001515f61412c565b90505f61326d8660600151600161412c565b90505f61328b85676765c793fa10079d601b1b612fe386600a617143565b90505f6132a985676765c793fa10079d601b1b612fe386600a617143565b9050805f036132bf57505f979650505050505050565b611f5a82825b5f8115676765c793fa10079d601b1b600284041904841117156132e6575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f5f5f5f61330c61635a565b61331589613953565b6101c0820181905261332c9088905f9081906139a4565b5060408401525081526101c081015161334b9088906001905f906139a4565b5060608401525060208201528051158061336757506020810151155b1561337e575f5f5f5f94509450945094505061341f565b85156133995787815f018181516133959190616efd565b9052505b606087015160381c61ffff1661014082018190526133c0908990612fbd90612710906144b4565b6101608201819052815160208301516133de92612fe390839061452d565b6080820181905260208201516133f3916144b4565b60c0820181905260408201516060830151610140840151613415908c90614509565b9450945094509450505b945094509450949050565b5f5f5f5f61343661635a565b61343f89613953565b6101c082018190526134569088905f9081906139a4565b5060408401525081526101c08101516134759088906001905f906139a4565b5060608401525060208201528051158061349157506020810151155b156134a8575f5f5f5f94509450945094505061341f565b85156134c45787816020018181516134c09190616efd565b9052505b805160208201516134da9190612fe3818c61452d565b6080820181905281516134ec916144b4565b60a0820152606087015160381c61ffff16610140820181905260a082015161351b91612fbd90612710906144b4565b6040820151606083015161014084015160a085015161341591614509565b61354161619b565b6135496163c1565b61355384846106e9565b808252602001516001600160a01b031661358857604051637357d91f60e01b8152600481018490526024015b60405180910390fd5b604080516101208101825282515151516001600160a01b0390811660a08301908152845151515184516395d89b4160e01b8152945193948594606086019485949360c08801939116916395d89b41916004808201925f929091908290030181865afa1580156135f9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136209190810190616fb1565b815286515151516040805163313ce56760e01b815290516020938401936001600160a01b039093169263313ce56792600480820193918290030181865afa15801561366d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136919190617045565b60ff168152865151516020908101519181019190915290825260408051608081018252875151830151516001600160a01b0390811682528851518401515183516395d89b4160e01b815293519585019592948501939116916395d89b41916004808201925f929091908290030181865afa158015613711573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526137389190810190616fb1565b8152865151602090910190600160200201515f01516001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613788573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137ac9190617045565b60ff16815286515160209091019060016020020151602001518152508152508152602001601b60ff1681526020016137e786845f015161407e565b90526020909101819052905092915050565b5f6138026161c0565b85602001516001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613842573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138669190616e98565b6020820152613877865f80806139a4565b50505060c082015261388c8660015f806139a4565b50505060e082015282156138c757848160c0018181516138ac9190616efd565b90525060e0810180518591906138c3908390616efd565b9052505b60c081015115806138da575060e0810151155b156138e8575f915050610356565b80602001515f03613918576139116103e861390b6139068888614581565b6145e7565b906144b4565b8152613949565b61394661392e8683602001518460c00151613a89565b6139418684602001518560e00151613a89565b6146c7565b81525b5195945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601e908201527f5452414441424c455f444542545f4d554c5449504945525f464143544f520000604082015260600190565b5f5f5f5f5f885f01518860ff16600281106139c1576139c1616e61565b602002015190505f6139d38a8a6146df565b9050805f036139ef575f5f5f5f9550955095509550505061341f565b5f6139fe838c608001516147cc565b90505f613a0b828a6140eb565b90505f8915613a3057818410613a2a57613a2584836144b4565b613a32565b5f613a32565b5f5b90505f613a3f858d6140eb565b90505f8c15613a6457848210613a5e57613a5982866144b4565b613a66565b5f613a66565b5f5b9050613a72858761714e565b9f959e50919c50909a509298505050505050505050565b5f838302815f1985870982811083820303915050805f03613abd57838281613ab357613ab3617161565b0492505050610317565b808411613add5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b5f5f613bc06040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b613bca845f614915565b602083015281526060840151613be0905f61412c565b606082018190528151613c0591676765c793fa10079d601b1b90612fe390600a617143565b604082015260208101515f03613c20575f6080820152613cc0565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613cba9190616e98565b60808201525b613ccb846001614915565b602083018190529082525f03613ce6575f60a0820152613d86565b60408481015181516060810183528351815260208085015190820190815284840151828501908152935163fdd63ecf60e01b815291516004830152516024820152915160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015613d5c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d809190616e98565b60a08201525b80608001518160a001519250925050915091565b5f5f613da6848461495b565b9050806001600160a01b03166321f8a72184604051602001613de7906020808252600b908201526a504f4f4c5f534f5552434560a81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613e17929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613e4b91815260200190565b602060405180830381865afa158015613e66573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190616eaf565b5f5f613e96848461495b565b9050806001600160a01b031663bd02d0f584604051602001613ee99060208082526017908201527f504f4f4c5f435245415445445f54494d455f5354414d50000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001613f19929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613f4d91815260200190565b602060405180830381865afa158015613f68573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103569190616e98565b5f5f613f998560016146df565b90508215613fae57613fab8482616efd565b90505b5f613fb887614a16565b90505f613fc583836140eb565b875160200151606001519091505f908210613ff35787516020015160600151613fee9083616efd565b613ff5565b5f5b90506140216040518060400160405280600b81526020016a706f6f6c42616c616e636560a81b81525050565b61404e6040518060400160405280600e81526020016d6d61784465706f7369745261746560901b81525050565b611f5a604051806040016040528060118152602001701c1bdbdb10985b185b98d950591a9d5cdd607a1b81525050565b5f6103178383614a5a565b6140916163e1565b6103178383614a73565b5f5f5f5f5f6140aa888861407e565b90506140b88787835f615c91565b9093509150816140c9575f196140d3565b6140d383836132c5565b94506140de886119b2565b9350939792965093509350565b5f81156b019d971e4fe8401e74000000198390048411151761410b575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f60ff60581b1960585f1960ff85160161414c575060ff60601b19905060605b90198416901c905092915050565b5f8183116141715761416c8383616efd565b610317565b6103178284616efd565b5f61031783676765c793fa10079d601b1b612fe385600a617143565b5f8284116141ad576141a882616f10565b610356565b5092915050565b5f6141ee6040518060e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b6141fa8686865f615c91565b6020830152808252158061422b5750845160ff84166002811061421f5761421f616e61565b6020020151602001515f145b15614239575f9150506104e5565b61424287615dfd565b604082018190526020820151614257916140eb565b608082018190528151101561426f575f9150506104e5565b608081015181516142809190616efd565b81606001818152505061429786606001518461412c565b60a0820181905260608201516142c3916142b290600a617143565b676765c793fa10079d601b1b613a89565b60c08201525f1960ff8416016142e85760c08101516142e290856132c5565b60c08201525b845160ff8416600281106142fe576142fe616e61565b6020020151602001518160c00151111561433b57845160ff84166002811061432857614328616e61565b6020020151602001518160c00181815250505b60c001519695505050505050565b80516020015160e001515f905f1901614369575051602001516060015190565b81516020015160e00151614384575051602001516080015190565b505f919050565b5f8415158385111461040f576143a082616f10565b6104e5565b5f5f6143b0876119b2565b90505f6143bd82876140eb565b90505f6143ca83866140eb565b90505f6143d78984617175565b90505f6143e48389617175565b90505f6143f083615e43565b90505f6143fc83615e43565b90505f8413801561440c57505f83125b8061442057505f8412801561442057505f83135b15614434575f9750505050505050506104e5565b805f0361444a575f9750505050505050506104e5565b61445482826132c5565b9d9c50505050505050505050505050565b5f815f0361447457505f610328565b5f82841161448b576144868484616efd565b614495565b6144958385616efd565b90505f6144a282856132c5565b9050838511610356576143a081616f10565b5f826144c08382616efd565b91508111156103285760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b604482015260640161357f565b5f8115611388198390048411151761451f575f5ffd5b506127109102611388010490565b5f82614539838261714e565b91508110156103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6164642d6f766572666c6f7760601b604482015260640161357f565b5f8115806145a4575082826145968183617194565b92506145a290836171ab565b145b6103285760405162461bcd60e51b815260206004820152601460248201527364732d6d6174682d6d756c2d6f766572666c6f7760601b604482015260640161357f565b5f815f036145f657505f919050565b5f600161460284615e58565b901c6001901b9050600181848161461b5761461b617161565b048201901c9050600181848161463357614633617161565b048201901c9050600181848161464b5761464b617161565b048201901c9050600181848161466357614663617161565b048201901c9050600181848161467b5761467b617161565b048201901c9050600181848161469357614693617161565b048201901c905060018184816146ab576146ab617161565b048201901c9050610317818285816146c5576146c5617161565b045b5f8183106146d55781610317565b5090919050565b50565b5f5f835f01518360ff16600281106146f9576146f9616e61565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614752573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906147769190616e98565b9050805f03614789575f92505050610328565b606082015160c083015161479d908261714e565b82106147c15760c08301516147b28284616efd565b6147bc9190616efd565b611b88565b5f9695505050505050565b5f8260a001515f036147df57505f610328565b5f6147ea8484615eeb565b60a085015190915061035690826140eb565b845181101561040f5782600486838151811061481a5761481a616e61565b016020015182516001600160f81b031990911690911c60f81c90811061484257614842616e61565b01602001516001600160f81b0319168261485d836002617194565b61486890600261714e565b8151811061487857614878616e61565b60200101906001600160f81b03191690815f1a905350828582815181106148a1576148a1616e61565b602091010151815160f89190911c600f169081106148c1576148c1616e61565b01602001516001600160f81b031916826148dc836002617194565b6148e790600361714e565b815181106148f7576148f7616e61565b60200101906001600160f81b03191690815f1a9053506001016147fc565b5f5f5f614942855f01518560ff166002811061493357614933616e61565b602002015186608001516147cc565b90505f61494f86866146df565b96919550909350505050565b5f5f839050806001600160a01b03166391d4403c60405160200161497e90616e75565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101869052604401602060405180830381865afa1580156149d2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906149f69190616eca565b61031757604051637357d91f60e01b81526004810184905260240161357f565b5f816001600160a01b031663bd02d0f560405160200161050d9060208082526010908201526f4d41585f4445504f5349545f5241544560801b604082015260600190565b5f826001600160a01b031663bd02d0f5611b1084615f2e565b614a7b6163e1565b82614a846163e1565b816001600160a01b03166391d4403c604051602001614ac4906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa158015614b18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614b3c9190616eca565b614b495791506103289050565b816001600160a01b031663bd02d0f585604051602001614b83906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001614bb3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614be791815260200190565b602060405180830381865afa158015614c02573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614c269190616e98565b816020018181525050816001600160a01b03166321f8a72185604051602001614c6e906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614c9e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614cd291815260200190565b602060405180830381865afa158015614ced573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614d119190616eaf565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001614d6d906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001614d9d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614dd191815260200190565b602060405180830381865afa158015614dec573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614e109190616eaf565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001614e8b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614ebf91815260200190565b602060405180830381865afa158015614eda573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614efe9190616e98565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001614f529060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001614f82929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401614fb691815260200190565b602060405180830381865afa158015614fd1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ff59190616e98565b81515f60200201516040018181525050816001600160a01b031663bd02d0f58560405160200161504f906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161507f929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016150b391815260200190565b602060405180830381865afa1580156150ce573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906150f29190616e98565b81515f60200201516060018181525050816001600160a01b031663bd02d0f58560405160200161514b906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161517b929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016151af91815260200190565b602060405180830381865afa1580156151ca573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906151ee9190616e98565b81515160a00152604080516020818101526017918101919091527f504f535f454e5452595f53484f52545f50524943455f3000000000000000000060608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615274929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016152a891815260200190565b602060405180830381865afa1580156152c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152e79190616e98565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615341906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615371929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016153a591815260200190565b602060405180830381865afa1580156153c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153e49190616e98565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615457929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161548b91815260200190565b602060405180830381865afa1580156154a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154ca9190616e98565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a7219086906080016040516020818303038152906040528051906020012060405160200161553e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161557291815260200190565b602060405180830381865afa15801561558d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155b19190616eaf565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161565891815260200190565b602060405180830381865afa158015615673573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156979190616e98565b8151600160200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156ec9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b6040516020818303038152906040528051906020012060405160200161571c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161575091815260200190565b602060405180830381865afa15801561576b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061578f9190616e98565b8151600160200201516040018181525050816001600160a01b031663bd02d0f5856040516020016157ea90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b6040516020818303038152906040528051906020012060405160200161581a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161584e91815260200190565b602060405180830381865afa158015615869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061588d9190616e98565b8151600160200201516060018181525050816001600160a01b031663bd02d0f5856040516020016158e790602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b60405160208183030381529060405280519060200120604051602001615917929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161594b91815260200190565b602060405180830381865afa158015615966573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061598a9190616e98565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f5856040516020016159ec9060208082526017908201527f504f535f454e5452595f53484f52545f50524943455f31000000000000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001615a1c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a5091815260200190565b602060405180830381865afa158015615a6b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a8f9190616e98565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001615aea90602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b1a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b4e91815260200190565b602060405180830381865afa158015615b69573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b8d9190616e98565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f585604051602001615bdc906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001615c0c929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c4091815260200190565b602060405180830381865afa158015615c5b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c7f9190616e98565b81516020015160e00152949350505050565b825151515f908190819081906001600160a01b03868116911614615d34575f5f615cbc8a8a5f615fc9565b915091505f615cd85f8c6060015161412c90919063ffffffff16565b90505f615cf684676765c793fa10079d601b1b612fe385600a617143565b90505f615d1484676765c793fa10079d601b1b612fe386600a617143565b9050615d20828861714e565b9650615d2c818761714e565b955050505050505b865160200151516001600160a01b03868116911614615df0575f5f615d5b8a8a6001615fc9565b915091505f615d7860018c6060015161412c90919063ffffffff16565b90505f615d9684676765c793fa10079d601b1b612fe385600a617143565b90505f615db484676765c793fa10079d601b1b612fe386600a617143565b90505f615dc1838d6140eb565b90505f615dce838e6140eb565b9050615dda828a61714e565b9850615de6818961714e565b9750505050505050505b9097909650945050505050565b5f816001600160a01b031663bd02d0f560405160200161050d906020808252601290820152712222a12a2fa9a0a322aa2cafa320a1aa27a960711b604082015260600190565b5f5f821215615e5457815f03610328565b5090565b5f80608083901c15615e6c57608092831c92015b604083901c15615e7e57604092831c92015b602083901c15615e9057602092831c92015b601083901c15615ea257601092831c92015b600883901c15615eb457600892831c92015b600483901c15615ec657600492831c92015b600283901c15615ed857600292831c92015b600183901c156103285760010192915050565b5f428203615efe57506020820151610328565b5f615f0d846040015184616061565b9050615f268460200151826140eb90919063ffffffff16565b915050610328565b80518051515f918291615f42916001612931565b905080604051602001615f7b90602080825260129082015271545741505f415645524147455f505249434560701b604082015260600190565b60405160208183030381529060405280519060200120604051602001615fab929190918252602082015260400190565b60405160208183030381529060405280519060200120915050919050565b5f5f5f845f01518460ff1660028110615fe457615fe4616e61565b60200201516040015190505f61601a875f01518660ff166002811061600b5761600b616e61565b60200201518860800151615eeb565b905081156160315761602c82826140eb565b616033565b5f5b865190935060ff86166002811061604c5761604c616e61565b60200201516020015193505050935093915050565b5f8061606d8342616efd565b6160779085617194565b6301e133809004905061035681676765c793fa10079d601b1b61714e565b6040518060a001604052806160a8616407565b81525f60208201819052604082018190526060820181905260809091015290565b604051806101a001604052806160dd61646e565b81526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f6001600160a01b031681526020015f151581526020015f81526020015f81526020015f81526020015f81525090565b60405180610120016040528061615c6164f5565b81526020015f81526020015f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180606001604052806161ae616570565b81526020015f81526020015f81525090565b6040518061016001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b604051806101c00160405280616223616095565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020016162786160c9565b905290565b6040518061040001604052806162916163e1565b815260200161629e616095565b81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f151581526020015f81526020015f81526020015f81525090565b604051806101e001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b60405180604001604052806163d4616095565b815260200161627861619b565b60405180606001604052806163f46165c6565b81525f6020820181905260409091015290565b60405180604001604052806002905b6164586040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816164165790505090565b60405180604001604052806002905b6164df6040518061018001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b81526020019060019003908161647d5790505090565b60405180604001604052806002905b61655a6040518061014001604052805f6001600160a01b03168152602001606081526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165045790505090565b60405180604001604052806002905b6165b060405180608001604052805f6001600160a01b03168152602001606081526020015f81526020015f81525090565b81526020019060019003908161657f5790505090565b60405180604001604052806002905b61661e6040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165d55790505090565b6001600160a01b03811681146146dc575f5ffd5b5f60208284031215616658575f5ffd5b813561031781616634565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057868503603f1901845281518051606080885260a08801919088015f5b600281101561674357898403605f19018252825180516001600160a01b0316855260208082015160809187018290529061671790870182616663565b6040838101519088015260609283015192909601919091525060209283019291909101906001016166db565b505050602082810151888201526040928301519290970191909152949384019391909101906001016166b7565b50929695505050505050565b5f5f5f5f6080858703121561678f575f5ffd5b843561679a81616634565b935060208501356167aa81616634565b925060408501356167ba81616634565b9396929550929360600135925050565b5f5f5f606084860312156167dc575f5ffd5b83356167e781616634565b95602085013595506040909401359392505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715616839576168396167fc565b604052919050565b5f67ffffffffffffffff82111561685a5761685a6167fc565b5060051b60200190565b5f5f60408385031215616875575f5ffd5b823561688081616634565b9150602083013567ffffffffffffffff81111561689b575f5ffd5b8301601f810185136168ab575f5ffd5b80356168be6168b982616841565b616810565b8082825260208201915060208360051b8501019250878311156168df575f5ffd5b6020840193505b828410156169015783358252602093840193909101906168e6565b809450505050509250929050565b602080825282518282018190525f918401906040840190835b818110156169f35783518051845f5b60028110156169a257825160018060a01b0381511683526020810151602084015260408101516040840152606081015160608401526080810151608084015260a081015160a084015260c081015160c08401525060e082019150602083019250600181019050616937565b5050506020818101516001600160a01b039081166101c08701526040830151166101e08601526060820151610200860152608090910151610220850152939093019261024090920191600101616928565b509095945050505050565b5f8260408101835f5b60028110156169f3578383038752815180516001600160a01b0316845260208101516101806020860152616a3f610180860182616663565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601526101408201516101408601526101608201516101608601528094505050602082019150602087019650600181019050616a07565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057603f19878603018452815180516101a08752616b176101a08801826169fe565b90506020820151616b3360208901826001600160a01b03169052565b506040820151616b4e60408901826001600160a01b03169052565b50606082015160608801526080820151608088015260a082015160a088015260c082015160c088015260e0820151616b9160e08901826001600160a01b03169052565b50610100820151616ba761010089018215159052565b506101208281015190880152610140808301519088015261016080830151908801526101809182015191909601526020938401939190910190600101616aef565b5f5f60408385031215616bf9575f5ffd5b8235616c0481616634565b91506020830135616c1481616634565b809150509250929050565b5f8260408101835f5b60028110156169f3578383038752815180516001600160a01b0316845260208101516101406020860152616c60610140860182616663565b905060408201516040860152606082015160608601526080820151608086015260a082015160a086015260c082015160c086015260e082015160e08601526101008201516101008601526101208201516101208601528094505050602082019150602087019650600181019050616c28565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561677057603f19878603018452815180516101208752616d20610120880182616c1f565b9050602082015160208801526040820151616d4660408901826001600160a01b03169052565b50606082810151908801526080808301519088015260a0808301519088015260c0808301519088015260e080830151908801526101009182015191909601526020938401939190910190600101616cf8565b60ff811681146146dc575f5ffd5b5f5f5f5f5f60a08688031215616dba575f5ffd5b8535616dc581616634565b94506020860135616dd581616634565b93506040860135616de581616634565b9250606086013591506080860135616dfc81616d98565b809150509295509295909350565b5f5f5f5f5f60a08688031215616e1e575f5ffd5b8535616e2981616634565b94506020860135616e3981616634565b93506040860135616e4981616634565b94979396509394606081013594506080013592915050565b634e487b7160e01b5f52603260045260245ffd5b6020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b5f60208284031215616ea8575f5ffd5b5051919050565b5f60208284031215616ebf575f5ffd5b815161031781616634565b5f60208284031215616eda575f5ffd5b81518015158114610317575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561032857610328616ee9565b5f600160ff1b8201616f2457616f24616ee9565b505f0390565b5f60208284031215616f3a575f5ffd5b815167ffffffffffffffff811115616f50575f5ffd5b8201601f81018413616f60575f5ffd5b8051616f6e6168b982616841565b8082825260208201915060208360051b850101925086831115616f8f575f5ffd5b6020840193505b82841015611b88578351825260209384019390910190616f96565b5f60208284031215616fc1575f5ffd5b815167ffffffffffffffff811115616fd7575f5ffd5b8201601f81018413616fe7575f5ffd5b805167ffffffffffffffff811115617001576170016167fc565b617014601f8201601f1916602001616810565b818152856020838501011115617028575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215617055575f5ffd5b815161031781616d98565b6001815b600184111561709b5780850481111561707f5761707f616ee9565b600184161561708d57908102905b60019390931c928002617064565b935093915050565b5f826170b157506001610328565b816170bd57505f610328565b81600181146170d357600281146170dd576170f9565b6001915050610328565b60ff8411156170ee576170ee616ee9565b50506001821b610328565b5060208310610133831016604e8410600b841016171561711c575081810a610328565b6171285f198484617060565b805f190482111561713b5761713b616ee9565b029392505050565b5f61031783836170a3565b8082018082111561032857610328616ee9565b634e487b7160e01b5f52601260045260245ffd5b8181035f8312801583831316838312821617156141ad576141ad616ee9565b808202811582820484141761032857610328616ee9565b5f826171c557634e487b7160e01b5f52601260045260245ffd5b50049056fea26469706673582212201646eefa6c3541c4b9aebc7b77c5fa8fbe4b5ae0fc216d2e7e29865ef7feecea64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x11W_5`\xE0\x1C\x80cs\x91\x18\xA4\x11a\0\x9EW\x80c\xC2\xBD\xED\xA1\x11a\0nW\x80c\xC2\xBD\xED\xA1\x14a\x02\x84W\x80c\xD2\x8B\n\x15\x14a\x02\xB2W\x80c\xE35\xAD\xB7\x14a\x02\xC5W\x80c\xEE\xD0t(\x14a\x02\xD8W\x80c\xF6\x8Aq1\x14a\x02\xEBW__\xFD[\x80cs\x91\x18\xA4\x14a\x02+W\x80cx\xF2\x12\xD1\x14a\x02KW\x80c\x8Flz<\x14a\x02^W\x80c\xA6\xA1\0\xA0\x14a\x02qW__\xFD[\x80cP7j\xAA\x11a\0\xE4W\x80cP7j\xAA\x14a\x01\xA4W\x80cP\xEDY-\x14a\x01\xC4W\x80cW\xB9\x1C\xA6\x14a\x01\xE5W\x80cZoW\x10\x14a\x01\xF8W\x80c\\9\xF4g\x14a\x02\x0BW__\xFD[\x80c\x1A0\x81u\x14a\x01\x15W\x80c(\xA0\xCC\xF4\x14a\x01>W\x80c1{P\xEC\x14a\x01iW\x80c8/\xC7.\x14a\x01\x91W[__\xFD[a\x01(a\x01#6`\x04afHV[a\x02\xFEV[`@Qa\x015\x91\x90af\x91V[`@Q\x80\x91\x03\x90\xF3[a\x01Qa\x01L6`\x04afHV[a\x03\x1EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x015V[a\x01|a\x01w6`\x04ag|V[a\x03.V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x015V[a\x01(a\x01\x9F6`\x04ag\xCAV[a\x03IV[a\x01\xB7a\x01\xB26`\x04ahdV[a\x03^V[`@Qa\x015\x91\x90ai\x0FV[a\x01\xD7a\x01\xD26`\x04afHV[a\x04\x17V[`@Q\x90\x81R` \x01a\x015V[a\x01\xD7a\x01\xF36`\x04afHV[a\x04!V[a\x01\xD7a\x02\x066`\x04afHV[a\x04+V[a\x02\x1Ea\x02\x196`\x04afHV[a\x045V[`@Qa\x015\x91\x90aj\xC9V[a\x02>a\x0296`\x04ak\xE8V[a\x04NV[`@Qa\x015\x91\x90al\xD2V[a\x01Qa\x02Y6`\x04afHV[a\x04iV[a\x02\x1Ea\x02l6`\x04ag\xCAV[a\x04sV[a\x02>a\x02\x7F6`\x04ahdV[a\x04\x80V[a\x02\x97a\x02\x926`\x04am\xA6V[a\x04\x8CV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x015V[a\x02\x97a\x02\xC06`\x04am\xA6V[a\x04\xAEV[a\x01Qa\x02\xD36`\x04afHV[a\x04\xBEV[a\x01(a\x02\xE66`\x04ahdV[a\x04\xC8V[a\x01\xD7a\x02\xF96`\x04an\nV[a\x04\xD4V[``_a\x03\n\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x05\x80V[\x93\x92PPPV[_a\x03(\x82a\x05\xFCV[\x92\x91PPV[__a\x03<\x86\x86\x86\x86a\x06\xADV[\x91P\x91P\x94P\x94\x92PPPV[``a\x03V\x84\x84\x84a\x05\x80V[\x94\x93PPPPV[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xB4W\x81` \x01[a\x03\xA1a`\x95V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\x99W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_a\x03\xE6\x86\x86\x84\x81Q\x81\x10a\x03\xD9Wa\x03\xD9anaV[` \x02` \x01\x01Qa\x06\xE9V[\x90P\x80\x83\x83\x81Q\x81\x10a\x03\xFBWa\x03\xFBanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x03\xB9V[P\x93\x92PPPV[_a\x03(\x82a\x19aV[_a\x03(\x82a\x19\xB2V[_a\x03(\x82a\x04\xEEV[``_a\x04A\x83a\x04\xEEV[\x90Pa\x03\x17\x83_\x83a\x1A\x03V[``_a\x04[\x84\x84a\x1A\xF7V[\x90Pa\x03V\x84\x84_\x84a\x1BmV[_a\x03(\x82a\x1B\x92V[``a\x03V\x84\x84\x84a\x1A\x03V[``a\x03\x17\x83\x83a\x1B\xCEV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1C\x84V[\x92P\x92P\x92P[\x95P\x95P\x95\x92PPPV[___a\x04\x9C\x88\x88\x88\x88\x88a\x1D\x8EV[_a\x03(\x82a\x1E\x07V[``a\x03\x17\x83\x83a\x1EXV[_a\x04\xE2\x86\x86\x86\x86\x86a\x1F3V[\x90P[\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9F`@Q` \x01a\x05\r\x90anuV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90an\x98V[``a\x05\xB4`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x1C\xDD\x18\\\x9D`j\x1B\x81RPPV[a\x05\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x19\xD9]\x14\x1B\xDB\xDB\x1C\xD2[\x99\x9B\xCC\x08\x19[\x99`z\x1B\x81RPPV[_a\x05\xF0\x85\x85\x85a\x1FfV[\x90Pa\x04\xE5\x85\x82a \x07V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\n\x90\x82\x01RiTOKEN_BASE`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03(\x91\x90an\xAFV[___a\x06\xBA\x86\x86a \xBDV[\x90P_a\x06\xC7\x88\x83a\x06\xE9V[\x90P__a\x06\xD6\x8A\x84\x89a!dV[P\x91\x9C\x90\x9BP\x99PPPPPPPPPPV[a\x06\xF1a`\x95V[\x82a\x06\xFAa`\x95V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a\x07\x18\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x90\x91\x90an\xCAV[a\x07\x9DW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x07\xDD\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08A\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x80\x91\x90an\xAFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x08\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tMW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tq\x91\x90an\x98V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\t\xC7\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\t\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n+\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nj\x91\x90an\x98V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\n\xCB\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_0\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\n\xFB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B/\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bn\x91\x90an\x98V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0B\xD9\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0C\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C=\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CXW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C|\x91\x90an\x98V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x0C\xDD\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\r\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x80\x91\x90an\x98V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r\xFD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ELW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ep\x91\x90an\x98V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FX\x91\x90an\xAFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10A\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x10\x98\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10\xC8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xFC\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11;\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x11\x9D\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xCD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\x01\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12@\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x12\xAC\x90` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x10\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13+W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13O\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x13\xB1\x90` \x80\x82R`\x18\x90\x82\x01R\x7FPOOL_TOTAL_SCALED_DEBT_1\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x140W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14T\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x14\xAD\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x11\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15P\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x15\x9E\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xCE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x02\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16A\x91\x90an\xAFV[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a\x16\xAF\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xDF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\x13\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17.W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17R\x91\x90an\xAFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a\x17\xB5\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xE5\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x19\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x184W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18X\x91\x90an\x98V[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a\x18\xB1\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xE1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x15\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x190W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19T\x91\x90an\x98V[`\x80\x82\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1A\x90\x82\x01R\x7FDEFAULT_POOL_CONFIGURATION\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[``_a\x1A\x11\x85\x85\x85a\x1FfV[\x90P_\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A.Wa\x1A.ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AgW\x81` \x01[a\x1ATa`\xC9V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1ALW\x90P[P\x90P_[\x82Q\x81\x10\x15a\x1A\xEDW_\x83\x82\x81Q\x81\x10a\x1A\x88Wa\x1A\x88anaV[` \x02` \x01\x01Q\x90Pa\x1A\xB8`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fpoolKey`\xC8\x1B\x81RPPV[_a\x1A\xC3\x89\x83a#]V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8anaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1AlV[P\x95\x94PPPPPV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x90;\x9Fa\x1B\x10\x84a'\xFDV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B.\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x17\x91\x90an\x98V[``_a\x1B|\x86\x86\x86\x86a(\x81V[\x90Pa\x1B\x88\x86\x82a\x1B\xCEV[\x96\x95PPPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\x08\x90\x82\x01RgTREASURY`\xC0\x1B`@\x82\x01R``\x01\x90V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xEBWa\x1B\xEBag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C$W\x81` \x01[a\x1C\x11aaHV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1C\tW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1CEWa\x1CEanaV[` \x02` \x01\x01Q\x90P_a\x1CZ\x87\x83a)\tV[\x90P\x80\x84\x84\x81Q\x81\x10a\x1CoWa\x1CoanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1C)V[____a\x1C\x92\x88\x88a \xBDV[\x90P_a\x1C\x9F\x8A\x83a\x06\xE9V[\x90P_\x80\x80`\xFF\x89\x16a\x1C\xD5Wa\x1C\xB7\x8D\x8B\x86a/\x18V[\x92\x95P\x91\x93Pa\x1C\xCE\x91P\x85\x90P_\x85\x8D\x82a02V[\x90Pa\x1D\x05V[_\x19`\xFF\x8A\x16\x01a\x1D\x05Wa\x1C\xEB\x8D\x8B\x86a1\x14V[\x92\x95P\x91\x93Pa\x1D\x02\x91P\x85\x90P\x84_\x80\x8Ea02V[\x90P[\x80_\x03a\x1D\x1FW___\x97P\x97P\x97PPPPPPa\x04\xA3V[_a\x1D)\x85a2\x16V[\x90P_\x82\x82\x11a\x1DBWa\x1D=\x82\x84an\xFDV[a\x1DLV[a\x1DL\x83\x83an\xFDV[\x90P_a\x1DY\x82\x84a2\xC5V[\x90P_\x84\x84\x11a\x1DqWa\x1Dl\x82ao\x10V[a\x1DsV[\x81[\x96\x9BP\x94\x99P\x94\x97PPPPPPPP\x95P\x95P\x95\x92PPPV[____a\x1D\x9C\x88\x88a \xBDV[\x90P_a\x1D\xA9\x8A\x83a\x06\xE9V[\x90P_\x80\x80`\xFF\x89\x16a\x1D\xD9Wa\x1D\xC2\x8D\x8B\x86_a3\0V[\x92\x95P\x91\x93Pa\x1C\xCE\x91P\x85\x90P\x8B_\x80\x87a02V[_\x19`\xFF\x8A\x16\x01a\x1D\x05Wa\x1D\xF0\x8D\x8B\x86_a4*V[\x92\x95P\x91\x93Pa\x1D\x02\x91P\x85\x90P_\x8C\x86\x82a02V[_\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!`@Q` \x01a\x06:\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FDEFAULT_INTEREST_RATE_STRATEGY\0\0`@\x82\x01R``\x01\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81Rl3\xB2\xBA(7\xB7\xB69\xA4\xB737\x99`\x99\x1B` \x90\x91\x01R``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x9AWa\x1E\x9Aag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\xD3W\x81` \x01[a\x1E\xC0aa\x9BV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\xB8W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a\x1E\xF4Wa\x1E\xF4anaV[` \x02` \x01\x01Q\x90P_a\x1F\t\x87\x83a59V[\x90P\x80\x84\x84\x81Q\x81\x10a\x1F\x1EWa\x1F\x1EanaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x1E\xD8V[__a\x1F?\x86\x86a \xBDV[\x90P_a\x1FL\x88\x83a\x06\xE9V[\x90Pa\x1FZ\x81\x86\x86_a7\xF9V[\x98\x97PPPPPPPPV[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*`@Q` \x01a\x1F\x86\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03V\x91\x90\x81\x01\x90ao*V[``_\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a $Wa $ag\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a ]W\x81` \x01[a Jaa\x9BV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a BW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x04\x0FW_\x84\x82\x81Q\x81\x10a ~Wa ~anaV[` \x02` \x01\x01Q\x90P_a \x93\x87\x83a59V[\x90P\x80\x84\x84\x81Q\x81\x10a \xA8Wa \xA8anaV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a bV[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a \xDEW\x81\x83a \xE1V[\x82\x82[`@Q\x91\x94P\x92Pa!\x0E\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[____a!paa\xC0V[a!y\x88a9SV[\x81a\x01@\x01\x81\x81RPP\x86` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xE7\x91\x90an\x98V[\x81` \x01\x81\x81RPPa\"\x01\x87__\x84a\x01@\x01Qa9\xA4V[P`\x80\x84\x01RP`@\x82\x01Ra\x01@\x81\x01Qa\"#\x90\x88\x90`\x01\x90_\x90a9\xA4V[P`\xA0\x84\x01RP``\x82\x01R` \x81\x01Q_\x03a\"LW____\x94P\x94P\x94P\x94PPa#TV[a\"_\x86\x82`@\x01Q\x83` \x01Qa:\x89V[a\x01\0\x82\x01R``\x81\x01Q` \x82\x01Qa\"z\x91\x88\x91a:\x89V[a\x01 \x82\x01\x90\x81R`@\x80Q\x80\x82\x01\x82R`\x10\x81Rovars.totalSupply`\x80\x1B` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\x12\x80\x82Rq\x07f\x17'2\xE7\x07&\x966U&W6W'fS`t\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R\x90\x81Rqvars.priceReserve1`p\x1B\x90\x82\x01R\x81Q\x80\x83\x01\x83R`\x0C\x80\x82Rk\x07f\x17'2\xE6\x16\xD6\xF7V\xE7C`\xA4\x1B\x91\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R\x82Rkvars.amount1`\xA0\x1B\x91\x01Ra\x01\0\x82\x01Q\x90Q`\x80\x83\x01Q`\xA0\x90\x93\x01Q\x91\x96P\x94P\x90\x92P\x90P[\x93P\x93P\x93P\x93V[a#ea`\xC9V[a#mab\x0FV[a#w\x84\x84a\x06\xE9V[\x81Ra#\x82\x84a;HV[` \x82\x01Ra#\x90\x84a9SV[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa#\xAB\x92_\x91\x90a9\xA4V[`\x80\x85\x01R`\xA0\x84\x01R`@\x83\x01R``\x82\x01R\x80Q` \x82\x01Qa\x01\x80\x83\x01Qa#\xD9\x92\x91`\x01\x91a9\xA4V[a\x01 \x85\x01Ra\x01@\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01R\x80Qa#\xFB\x90a;\x8BV[a\x01`\x83\x01R`\xC0\x82\x01R`@\x80Qa\x03`\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xE0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01\xA0\x86\x01\x94\x85\x94\x93a\x02\0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$zW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xA1\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x12\x91\x90apEV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x81\x83\x01R`\xC0\x80\x89\x01Q`@\x80\x85\x01\x91\x90\x91R\x89QQQ``\x90\x81\x01Q\x81\x86\x01R\x8AQQQ`\x80\x90\x81\x01Q\x81\x87\x01R\x8BQQQ`\xA0\x90\x81\x01Q\x81\x88\x01R\x83\x8D\x01Q\x94\x87\x01\x94\x90\x94R\x90\x8B\x01Q`\xE0\x86\x01R\x8A\x01Qa\x01\0\x85\x01R\x90\x89\x01Qa\x01 \x90\x93\x01\x92\x90\x92R\x91\x83R\x80Qa\x01\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a%\xF2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\x19\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x8D\x91\x90apEV[`\xFF\x16\x81R\x86QQ` \x90\x81\x01Q\x81\x01Q\x81\x83\x01Ra\x01`\x88\x01Q`@\x80\x84\x01\x91\x90\x91R\x88QQ\x82\x01Q``\x90\x81\x01Q\x81\x85\x01R\x89QQ\x83\x01Q`\x80\x90\x81\x01Q\x81\x86\x01R\x8AQQ\x84\x01Q`\xA0\x90\x81\x01Q\x81\x87\x01R`\xE0\x80\x8D\x01Q`\xC0\x80\x89\x01\x91\x90\x91Ra\x01\0\x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01 \x80\x8F\x01Q\x92\x89\x01\x92\x90\x92Ra\x01@\x8E\x01Q\x91\x90\x97\x01R\x95\x90\x96R\x95\x87R\x88Q\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x88\x01\x92\x90\x92R\x88Q\x81\x01Q\x90\x91\x16\x90\x86\x01R\x86Q\x84\x01Q\x93\x85\x01\x93\x90\x93R\x85Q\x82\x01Q\x91\x84\x01\x91\x90\x91R`\x1B\x90\x83\x01R\x83Q\x91\x01\x90a'm\x90a2\x16V[\x81R` \x01a'|\x86\x86a=\x9AV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a'\xA3\x83_\x01Q``\x01Qf\x08\0\0\0\0\0\0\x16\x15\x15\x90V[\x15\x15\x81R` \x01a'\xB4\x86\x86a>\x8AV[\x81R\x82QQQ`\xC0\x01Q` \x82\x01R\x82Q`@\x90\x91\x01\x90a'\xD8\x90\x87\x90_\x80a?\x8CV[\x81R` \x01a'\xEA\x86\x84_\x01Qa@~V[\x90Ra\x01\xA0\x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_`@Q` \x01a(7\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``\x84`\x01`\x01`\xA0\x1B\x03\x16c\xF0i\x05*a(\x9B\x86a'\xFDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xE2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xE5\x91\x90\x81\x01\x90ao*V[a)\x11aaHV[a)\x19ab}V[a)#\x84\x84a@\x89V[\x80\x82RQ\x80QQa)<\x91`\x01[` \x02\x01QQa \xBDV[`@\x82\x01\x81\x90Ra)N\x90\x85\x90a\x06\xE9V[` \x82\x01\x81\x90R\x81Qa)b\x91\x86\x91a@\x9BV[PPPP``\x82\x01R` \x81\x01Qa)y\x90a2\x16V[a\x03\0\x82\x01R\x80Q\x80QQ` \x90\x81\x01Q`\xE0\x84\x01R\x80\x83\x01QQQ\x01Q\x90Qa)\xAF\x91\x90_[` \x02\x01Q`@\x01Q\x90a@\xEBV[`\xC0\x82\x01R` \x81\x01Q``\x01Qa)\xC7\x90_aA,V[`\xA0\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa)\xE0\x91\x90aAZV[a\x01\0\x82\x01\x81\x90R`\xA0\x82\x01Qa)\xF7\x91\x90aA{V[a\x01 \x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01\0\x83\x01Qa*\x18\x92\x91\x90aA\x97V[a\x01@\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa*9\x92\x87\x92\x90\x91_aA\xB4V[a\x01`\x82\x01Ra\x01@\x81\x01Qa\x01\x80\x82\x01R`\xE0\x81\x01Q`\xC0\x82\x01Qa\x01 \x83\x01Qa*f\x92\x91\x90aA\x97V[a\x01\xA0\x82\x01R` \x81\x01Q``\x01Qa*\x80\x90`\x01aA,V[a\x01\xC0\x82\x01R\x80Q\x80Q` \x90\x81\x01Q\x81\x01Qa\x02\0\x84\x01R\x80\x83\x01QQ\x81\x01Q\x01Q\x90Qa*\xB1\x91\x90`\x01a)\xA0V[a\x01\xE0\x82\x01\x81\x90Ra\x02\0\x82\x01Qa*\xC8\x91aAZV[a\x02 \x82\x01\x81\x90Ra\x01\xC0\x82\x01Qa*\xE0\x91\x90aA{V[a\x02@\x82\x01\x81\x90Ra\x03\0\x82\x01Qa*\xF8\x91\x90a@\xEBV[a\x02`\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02 \x83\x01Qa+\x1B\x92\x91\x90aA\x97V[a\x02\x80\x82\x01R` \x81\x01Q\x81Qa\x03\0\x83\x01Qa+=\x92\x87\x92\x90\x91`\x01aA\xB4V[a\x02\xA0\x82\x01Ra\x02\x80\x81\x01Qa\x02\xC0\x82\x01Ra\x02\0\x81\x01Qa\x01\xE0\x82\x01Qa\x02`\x83\x01Qa+l\x92\x91\x90aA\x97V[a\x02\xE0\x82\x01R\x80Qa+}\x90aCIV[`\x80\x82\x01R\x80QQ` \x01Q`\xE0\x01Q`\x02\x14a,uWa+\xA7\x81a\x03\0\x01Q\x82`\x80\x01QaAZV[a\x03 \x82\x01\x81\x90Ra\x02@\x82\x01Qa+\xBE\x91a@\xEBV[a\x03@\x82\x01\x81\x90R`\x80\x82\x01Qa\x03\0\x83\x01Q\x11a\x03\x80\x83\x01\x81\x90Ra\x02\0\x83\x01Qa\x01\xE0\x84\x01Qa+\xEF\x93aC\x8BV[a\x03`\x82\x01\x81\x90Ra\x03\xA0\x82\x01R`\xE0\x81\x01Q`\xA0\x82\x01Qa,V\x91\x86\x91a,\x17\x91\x90aA{V[a,)\x84`\xC0\x01Q\x85`\xA0\x01QaA{V[a,=\x85a\x02\0\x01Q\x86a\x01\xC0\x01QaA{V[a,Q\x86a\x01\xE0\x01Q\x87a\x01\xC0\x01QaA{V[aC\xA5V[a\x03\xC0\x82\x01\x81\x90Ra\x03\0\x82\x01Qa,n\x91\x90aDeV[a\x03\xE0\x82\x01R[`@\x80Qa\x02\xA0\x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01`\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94a\x01 \x86\x01\x94\x85\x94\x93a\x01\x80\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\x10\x91\x90\x81\x01\x90ao\xB1V[\x81R` \x01\x86`\xA0\x01Q\x81R` \x01\x86`\xE0\x01Q\x81R` \x01\x86`\xC0\x01Q\x81R` \x01\x86a\x01@\x01Q\x81R` \x01\x86a\x01`\x01Q\x81R` \x01\x86` \x01Q_\x01Q_`\xFF\x16`\x02\x81\x10a-eWa-eanaV[` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x01\x80\x01Q\x81R` \x01\x86a\x01\xA0\x01Q\x81RP\x81R` \x01`@Q\x80a\x01@\x01`@R\x80\x86_\x01Q_\x01Q`\x01`\xFF\x16`\x02\x81\x10a-\xB4Wa-\xB4anaV[` \x90\x81\x02\x91\x90\x91\x01QQ`\x01`\x01`\xA0\x1B\x03\x16\x82R\x87QQ\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\x14W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra.;\x91\x90\x81\x01\x90ao\xB1V[\x81Ra\x01\xC0\x87\x01Q` \x80\x83\x01\x91\x90\x91Ra\x02\0\x88\x01Q`@\x83\x01Ra\x01\xE0\x88\x01Q``\x83\x01Ra\x02\x80\x88\x01Q`\x80\x83\x01Ra\x02\xA0\x88\x01Q`\xA0\x83\x01R\x87\x01QQ`\xC0\x90\x91\x01\x90`\x01` \x02\x01Q`@\x01Q\x81R` \x01\x86a\x02\xC0\x01Q\x81R` \x01\x86a\x02\xE0\x01Q\x81RP\x81RP\x81R` \x01\x82_\x01Q` \x01Q\x81R` \x01\x82_\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82``\x01Q\x81R` \x01\x82`\x80\x01Q\x81R` \x01\x82a\x03\0\x01Q\x81R` \x01\x82a\x03\xA0\x01Q\x81R` \x01\x82a\x03\xC0\x01Q\x81R` \x01\x82a\x03\xE0\x01Q\x81RP\x91PP\x92\x91PPV[____a/$acZV[a/-\x88a9SV[a\x01\xC0\x82\x01\x81\x90Ra/D\x90\x87\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa/c\x90\x87\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a/\x7FWP` \x81\x01Q\x15[\x15a/\x96W____\x94P\x94P\x94P\x94PPa#TV[``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra/\xC5\x90\x88\x90a/\xBD\x90a'\x10\x90aD\xB4V[a'\x10a:\x89V[a\x01\x80\x82\x01\x81\x90R\x81Q` \x83\x01Qa/\xE8\x92a/\xE3\x90\x83\x90aD\xB4V[a:\x89V[`\x80\x82\x01\x81\x90R` \x82\x01Qa/\xFE\x91\x90aD\xB4V[`\xE0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa0 \x90\x8B\x90aE\tV[\x94P\x94P\x94P\x94PP\x93P\x93P\x93P\x93V[____\x86\x11\x80\x15a0BWP\x83\x15[\x15a0QWP\x83\x90P\x84a0\x86V[_\x87\x11\x80\x15a0^WP\x84\x15[\x15a0mWP\x85\x90P\x82a0\x86V[`@Qcc1\xFA\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a0\x95\x89``\x01Q_aA,V[\x90P_a0\xA7\x8A``\x01Q`\x01aA,V[\x90P_a0\xC5\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a0\xE3\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P\x80_\x03a0\xFAW_\x96PPPPPPPa\x04\xE5V[a1\x04\x82\x82a2\xC5V[\x9C\x9BPPPPPPPPPPPPV[____a1 acZV[a1)\x88a9SV[a\x01\xC0\x82\x01\x81\x90Ra1@\x90\x87\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa1_\x90\x87\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a1{WP` \x81\x01Q\x15[\x15a1\x92W____\x94P\x94P\x94P\x94PPa#TV[\x80Q` \x82\x01Qa1\xA8\x91\x90a/\xE3\x81\x8BaD\xB4V[a\x01\0\x82\x01\x81\x90R\x81Qa1\xBC\x91\x90aD\xB4V[a\x01 \x82\x01R``\x86\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra\x01 \x82\x01Qa1\xEF\x91a'\x10\x90a/\xE3\x90\x82\x90aD\xB4V[a\x01\xA0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa\x01 \x85\x01Qa0 \x91aE\tV[__a2$\x83___a9\xA4V[PPP\x90P_a27\x84`\x01__a9\xA4V[PPP\x90P\x80_\x03a2LWP_\x93\x92PPPV[_a2[\x85``\x01Q_aA,V[\x90P_a2m\x86``\x01Q`\x01aA,V[\x90P_a2\x8B\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a2\xA9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P\x80_\x03a2\xBFWP_\x97\x96PPPPPPPV[a\x1FZ\x82\x82[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a2\xE6W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[____a3\x0CacZV[a3\x15\x89a9SV[a\x01\xC0\x82\x01\x81\x90Ra3,\x90\x88\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa3K\x90\x88\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a3gWP` \x81\x01Q\x15[\x15a3~W____\x94P\x94P\x94P\x94PPa4\x1FV[\x85\x15a3\x99W\x87\x81_\x01\x81\x81Qa3\x95\x91\x90an\xFDV[\x90RP[``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90Ra3\xC0\x90\x89\x90a/\xBD\x90a'\x10\x90aD\xB4V[a\x01`\x82\x01\x81\x90R\x81Q` \x83\x01Qa3\xDE\x92a/\xE3\x90\x83\x90aE-V[`\x80\x82\x01\x81\x90R` \x82\x01Qa3\xF3\x91aD\xB4V[`\xC0\x82\x01\x81\x90R`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Qa4\x15\x90\x8C\x90aE\tV[\x94P\x94P\x94P\x94PP[\x94P\x94P\x94P\x94\x90PV[____a46acZV[a4?\x89a9SV[a\x01\xC0\x82\x01\x81\x90Ra4V\x90\x88\x90_\x90\x81\x90a9\xA4V[P`@\x84\x01RP\x81Ra\x01\xC0\x81\x01Qa4u\x90\x88\x90`\x01\x90_\x90a9\xA4V[P``\x84\x01RP` \x82\x01R\x80Q\x15\x80a4\x91WP` \x81\x01Q\x15[\x15a4\xA8W____\x94P\x94P\x94P\x94PPa4\x1FV[\x85\x15a4\xC4W\x87\x81` \x01\x81\x81Qa4\xC0\x91\x90an\xFDV[\x90RP[\x80Q` \x82\x01Qa4\xDA\x91\x90a/\xE3\x81\x8CaE-V[`\x80\x82\x01\x81\x90R\x81Qa4\xEC\x91aD\xB4V[`\xA0\x82\x01R``\x87\x01Q`8\x1Ca\xFF\xFF\x16a\x01@\x82\x01\x81\x90R`\xA0\x82\x01Qa5\x1B\x91a/\xBD\x90a'\x10\x90aD\xB4V[`@\x82\x01Q``\x83\x01Qa\x01@\x84\x01Q`\xA0\x85\x01Qa4\x15\x91aE\tV[a5Aaa\x9BV[a5Iac\xC1V[a5S\x84\x84a\x06\xE9V[\x80\x82R` \x01Q`\x01`\x01`\xA0\x1B\x03\x16a5\x88W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Qa\x01 \x81\x01\x82R\x82QQQQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x83\x01\x90\x81R\x84QQQQ\x84Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x94Q\x93\x94\x85\x94``\x86\x01\x94\x85\x94\x93`\xC0\x88\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a5\xF9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6 \x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQQQ`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q` \x93\x84\x01\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c1<\xE5g\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a6mW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x91\x91\x90apEV[`\xFF\x16\x81R\x86QQQ` \x90\x81\x01Q\x91\x81\x01\x91\x90\x91R\x90\x82R`@\x80Q`\x80\x81\x01\x82R\x87QQ\x83\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R\x88QQ\x84\x01QQ\x83Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x93Q\x95\x85\x01\x95\x92\x94\x85\x01\x93\x91\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x82\x01\x92_\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a7\x11W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra78\x91\x90\x81\x01\x90ao\xB1V[\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xAC\x91\x90apEV[`\xFF\x16\x81R\x86QQ` \x90\x91\x01\x90`\x01` \x02\x01Q` \x01Q\x81RP\x81RP\x81R` \x01`\x1B`\xFF\x16\x81R` \x01a7\xE7\x86\x84_\x01Qa@~V[\x90R` \x90\x91\x01\x81\x90R\x90P\x92\x91PPV[_a8\x02aa\xC0V[\x85` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8f\x91\x90an\x98V[` \x82\x01Ra8w\x86_\x80\x80a9\xA4V[PPP`\xC0\x82\x01Ra8\x8C\x86`\x01_\x80a9\xA4V[PPP`\xE0\x82\x01R\x82\x15a8\xC7W\x84\x81`\xC0\x01\x81\x81Qa8\xAC\x91\x90an\xFDV[\x90RP`\xE0\x81\x01\x80Q\x85\x91\x90a8\xC3\x90\x83\x90an\xFDV[\x90RP[`\xC0\x81\x01Q\x15\x80a8\xDAWP`\xE0\x81\x01Q\x15[\x15a8\xE8W_\x91PPa\x03VV[\x80` \x01Q_\x03a9\x18Wa9\x11a\x03\xE8a9\x0Ba9\x06\x88\x88aE\x81V[aE\xE7V[\x90aD\xB4V[\x81Ra9IV[a9Fa9.\x86\x83` \x01Q\x84`\xC0\x01Qa:\x89V[a9A\x86\x84` \x01Q\x85`\xE0\x01Qa:\x89V[aF\xC7V[\x81R[Q\x95\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x1E\x90\x82\x01R\x7FTRADABLE_DEBT_MULTIPIER_FACTOR\0\0`@\x82\x01R``\x01\x90V[_____\x88_\x01Q\x88`\xFF\x16`\x02\x81\x10a9\xC1Wa9\xC1anaV[` \x02\x01Q\x90P_a9\xD3\x8A\x8AaF\xDFV[\x90P\x80_\x03a9\xEFW____\x95P\x95P\x95P\x95PPPa4\x1FV[_a9\xFE\x83\x8C`\x80\x01QaG\xCCV[\x90P_a:\x0B\x82\x8Aa@\xEBV[\x90P_\x89\x15a:0W\x81\x84\x10a:*Wa:%\x84\x83aD\xB4V[a:2V[_a:2V[_[\x90P_a:?\x85\x8Da@\xEBV[\x90P_\x8C\x15a:dW\x84\x82\x10a:^Wa:Y\x82\x86aD\xB4V[a:fV[_a:fV[_[\x90Pa:r\x85\x87aqNV[\x9F\x95\x9EP\x91\x9CP\x90\x9AP\x92\x98PPPPPPPPPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a:\xBDW\x83\x82\x81a:\xB3Wa:\xB3aqaV[\x04\x92PPPa\x03\x17V[\x80\x84\x11a:\xDDW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[__a;\xC0`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[a;\xCA\x84_aI\x15V[` \x83\x01R\x81R``\x84\x01Qa;\xE0\x90_aA,V[``\x82\x01\x81\x90R\x81Qa<\x05\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90a/\xE3\x90`\naqCV[`@\x82\x01R` \x81\x01Q_\x03a< W_`\x80\x82\x01Ra<\xC0V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xBA\x91\x90an\x98V[`\x80\x82\x01R[a<\xCB\x84`\x01aI\x15V[` \x83\x01\x81\x90R\x90\x82R_\x03a<\xE6W_`\xA0\x82\x01Ra=\x86V[`@\x84\x81\x01Q\x81Q``\x81\x01\x83R\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01\x90\x81R\x84\x84\x01Q\x82\x85\x01\x90\x81R\x93Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x91Q`\x04\x83\x01RQ`$\x82\x01R\x91Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\\W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\x80\x91\x90an\x98V[`\xA0\x82\x01R[\x80`\x80\x01Q\x81`\xA0\x01Q\x92P\x92PP\x91P\x91V[__a=\xA6\x84\x84aI[V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x84`@Q` \x01a=\xE7\x90` \x80\x82R`\x0B\x90\x82\x01RjPOOL_SOURCE`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\x17\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>K\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>fW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90an\xAFV[__a>\x96\x84\x84aI[V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x84`@Q` \x01a>\xE9\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOOL_CREATED_TIME_STAMP\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?M\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03V\x91\x90an\x98V[__a?\x99\x85`\x01aF\xDFV[\x90P\x82\x15a?\xAEWa?\xAB\x84\x82an\xFDV[\x90P[_a?\xB8\x87aJ\x16V[\x90P_a?\xC5\x83\x83a@\xEBV[\x87Q` \x01Q``\x01Q\x90\x91P_\x90\x82\x10a?\xF3W\x87Q` \x01Q``\x01Qa?\xEE\x90\x83an\xFDV[a?\xF5V[_[\x90Pa@!`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jpoolBalance`\xA8\x1B\x81RPPV[a@N`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mmaxDepositRate`\x90\x1B\x81RPPV[a\x1FZ`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x1C\x1B\xDB\xDB\x10\x98[\x18[\x98\xD9PY\x1A\x9D\\\xDD`z\x1B\x81RPPV[_a\x03\x17\x83\x83aJZV[a@\x91ac\xE1V[a\x03\x17\x83\x83aJsV[_____a@\xAA\x88\x88a@~V[\x90Pa@\xB8\x87\x87\x83_a\\\x91V[\x90\x93P\x91P\x81a@\xC9W_\x19a@\xD3V[a@\xD3\x83\x83a2\xC5V[\x94Pa@\xDE\x88a\x19\xB2V[\x93P\x93\x97\x92\x96P\x93P\x93PV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aA\x0BW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aALWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x81\x83\x11aAqWaAl\x83\x83an\xFDV[a\x03\x17V[a\x03\x17\x82\x84an\xFDV[_a\x03\x17\x83gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[_\x82\x84\x11aA\xADWaA\xA8\x82ao\x10V[a\x03VV[P\x92\x91PPV[_aA\xEE`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aA\xFA\x86\x86\x86_a\\\x91V[` \x83\x01R\x80\x82R\x15\x80aB+WP\x84Q`\xFF\x84\x16`\x02\x81\x10aB\x1FWaB\x1FanaV[` \x02\x01Q` \x01Q_\x14[\x15aB9W_\x91PPa\x04\xE5V[aBB\x87a]\xFDV[`@\x82\x01\x81\x90R` \x82\x01QaBW\x91a@\xEBV[`\x80\x82\x01\x81\x90R\x81Q\x10\x15aBoW_\x91PPa\x04\xE5V[`\x80\x81\x01Q\x81QaB\x80\x91\x90an\xFDV[\x81``\x01\x81\x81RPPaB\x97\x86``\x01Q\x84aA,V[`\xA0\x82\x01\x81\x90R``\x82\x01QaB\xC3\x91aB\xB2\x90`\naqCV[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba:\x89V[`\xC0\x82\x01R_\x19`\xFF\x84\x16\x01aB\xE8W`\xC0\x81\x01QaB\xE2\x90\x85a2\xC5V[`\xC0\x82\x01R[\x84Q`\xFF\x84\x16`\x02\x81\x10aB\xFEWaB\xFEanaV[` \x02\x01Q` \x01Q\x81`\xC0\x01Q\x11\x15aC;W\x84Q`\xFF\x84\x16`\x02\x81\x10aC(WaC(anaV[` \x02\x01Q` \x01Q\x81`\xC0\x01\x81\x81RPP[`\xC0\x01Q\x96\x95PPPPPPV[\x80Q` \x01Q`\xE0\x01Q_\x90_\x19\x01aCiWPQ` \x01Q``\x01Q\x90V[\x81Q` \x01Q`\xE0\x01QaC\x84WPQ` \x01Q`\x80\x01Q\x90V[P_\x91\x90PV[_\x84\x15\x15\x83\x85\x11\x14a\x04\x0FWaC\xA0\x82ao\x10V[a\x04\xE5V[__aC\xB0\x87a\x19\xB2V[\x90P_aC\xBD\x82\x87a@\xEBV[\x90P_aC\xCA\x83\x86a@\xEBV[\x90P_aC\xD7\x89\x84aquV[\x90P_aC\xE4\x83\x89aquV[\x90P_aC\xF0\x83a^CV[\x90P_aC\xFC\x83a^CV[\x90P_\x84\x13\x80\x15aD\x0CWP_\x83\x12[\x80aD WP_\x84\x12\x80\x15aD WP_\x83\x13[\x15aD4W_\x97PPPPPPPPa\x04\xE5V[\x80_\x03aDJW_\x97PPPPPPPPa\x04\xE5V[aDT\x82\x82a2\xC5V[\x9D\x9CPPPPPPPPPPPPPV[_\x81_\x03aDtWP_a\x03(V[_\x82\x84\x11aD\x8BWaD\x86\x84\x84an\xFDV[aD\x95V[aD\x95\x83\x85an\xFDV[\x90P_aD\xA2\x82\x85a2\xC5V[\x90P\x83\x85\x11a\x03VWaC\xA0\x81ao\x10V[_\x82aD\xC0\x83\x82an\xFDV[\x91P\x81\x11\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17aE\x1FW__\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[_\x82aE9\x83\x82aqNV[\x91P\x81\x10\x15a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-add-overflow``\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81\x15\x80aE\xA4WP\x82\x82aE\x96\x81\x83aq\x94V[\x92PaE\xA2\x90\x83aq\xABV[\x14[a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rsds-math-mul-overflow``\x1B`D\x82\x01R`d\x01a5\x7FV[_\x81_\x03aE\xF6WP_\x91\x90PV[_`\x01aF\x02\x84a^XV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81aF\x1BWaF\x1BaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF3WaF3aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aFKWaFKaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aFcWaFcaqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF{WaF{aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF\x93WaF\x93aqaV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81aF\xABWaF\xABaqaV[\x04\x82\x01\x90\x1C\x90Pa\x03\x17\x81\x82\x85\x81aF\xC5WaF\xC5aqaV[\x04[_\x81\x83\x10aF\xD5W\x81a\x03\x17V[P\x90\x91\x90PV[PV[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aF\xF9WaF\xF9anaV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGv\x91\x90an\x98V[\x90P\x80_\x03aG\x89W_\x92PPPa\x03(V[``\x82\x01Q`\xC0\x83\x01QaG\x9D\x90\x82aqNV[\x82\x10aG\xC1W`\xC0\x83\x01QaG\xB2\x82\x84an\xFDV[aG\xBC\x91\x90an\xFDV[a\x1B\x88V[_\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aG\xDFWP_a\x03(V[_aG\xEA\x84\x84a^\xEBV[`\xA0\x85\x01Q\x90\x91Pa\x03V\x90\x82a@\xEBV[\x84Q\x81\x10\x15a\x04\x0FW\x82`\x04\x86\x83\x81Q\x81\x10aH\x1AWaH\x1AanaV[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10aHBWaHBanaV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82aH]\x83`\x02aq\x94V[aHh\x90`\x02aqNV[\x81Q\x81\x10aHxWaHxanaV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP\x82\x85\x82\x81Q\x81\x10aH\xA1WaH\xA1anaV[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10aH\xC1WaH\xC1anaV[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82aH\xDC\x83`\x02aq\x94V[aH\xE7\x90`\x03aqNV[\x81Q\x81\x10aH\xF7WaH\xF7anaV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01aG\xFCV[___aIB\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aI3WaI3anaV[` \x02\x01Q\x86`\x80\x01QaG\xCCV[\x90P_aIO\x86\x86aF\xDFV[\x96\x91\x95P\x90\x93PPPPV[__\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aI~\x90anuV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xF6\x91\x90an\xCAV[a\x03\x17W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a5\x7FV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x10\x90\x82\x01RoMAX_DEPOSIT_RATE`\x80\x1B`@\x82\x01R``\x01\x90V[_\x82`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5a\x1B\x10\x84a_.V[aJ{ac\xE1V[\x82aJ\x84ac\xE1V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aJ\xC4\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK<\x91\x90an\xCAV[aKIW\x91Pa\x03(\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aK\x83\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aK\xB3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\xE7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\x02W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aL&\x91\x90an\x98V[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aLn\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aL\x9E\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aL\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aL\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\x11\x91\x90an\xAFV[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aMm\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aM\x9D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aM\xD1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\xECW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\x10\x91\x90an\xAFV[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aN\x8B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aN\xBF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN\xDAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aN\xFE\x91\x90an\x98V[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aOR\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aO\x82\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aO\xB6\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aO\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aO\xF5\x91\x90an\x98V[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aPO\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aP\x7F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aP\xB3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aP\xCEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aP\xF2\x91\x90an\x98V[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aQK\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aQ{\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aQ\xAF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQ\xEE\x91\x90an\x98V[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91R\x7FPOS_ENTRY_SHORT_PRICE_0\0\0\0\0\0\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aRt\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aR\xA8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\xE7\x91\x90an\x98V[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aSA\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aSq\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xA5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xE4\x91\x90an\x98V[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTW\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT\x8B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xCA\x91\x90an\x98V[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aU>\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aUr\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xB1\x91\x90an\xAFV[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVX\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aVsW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\x97\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xEC\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aWkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\x8F\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xEA\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aXN\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aXiW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x8D\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\xE7\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY\x17\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYK\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYfW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\x8A\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aY\xEC\x90` \x80\x82R`\x17\x90\x82\x01R\x7FPOS_ENTRY_SHORT_PRICE_1\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZP\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZkW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\x8F\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\xEA\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\x1A\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[N\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\x8D\x91\x90an\x98V[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a[\xDC\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\x0C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\@\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\[W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\\x7F\x91\x90an\x98V[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]4W__a\\\xBC\x8A\x8A_a_\xC9V[\x91P\x91P_a\\\xD8_\x8C``\x01QaA,\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a\\\xF6\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[\x90P_a]\x14\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90Pa] \x82\x88aqNV[\x96Pa],\x81\x87aqNV[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a]\xF0W__a][\x8A\x8A`\x01a_\xC9V[\x91P\x91P_a]x`\x01\x8C``\x01QaA,\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_a]\x96\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x85`\naqCV[\x90P_a]\xB4\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba/\xE3\x86`\naqCV[\x90P_a]\xC1\x83\x8Da@\xEBV[\x90P_a]\xCE\x83\x8Ea@\xEBV[\x90Pa]\xDA\x82\x8AaqNV[\x98Pa]\xE6\x81\x89aqNV[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x05\r\x90` \x80\x82R`\x12\x90\x82\x01Rq\"\"\xA1*/\xA9\xA0\xA3\"\xAA,\xAF\xA3 \xA1\xAA'\xA9`q\x1B`@\x82\x01R``\x01\x90V[__\x82\x12\x15a^TW\x81_\x03a\x03(V[P\x90V[_\x80`\x80\x83\x90\x1C\x15a^lW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a^~W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a^\x90W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a^\xA2W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a^\xB4W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a^\xC6W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a^\xD8W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x03(W`\x01\x01\x92\x91PPV[_B\x82\x03a^\xFEWP` \x82\x01Qa\x03(V[_a_\r\x84`@\x01Q\x84a`aV[\x90Pa_&\x84` \x01Q\x82a@\xEB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa\x03(V[\x80Q\x80QQ_\x91\x82\x91a_B\x91`\x01a)1V[\x90P\x80`@Q` \x01a_{\x90` \x80\x82R`\x12\x90\x82\x01RqTWAP_AVERAGE_PRICE`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10a_\xE4Wa_\xE4anaV[` \x02\x01Q`@\x01Q\x90P_a`\x1A\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10a`\x0BWa`\x0BanaV[` \x02\x01Q\x88`\x80\x01Qa^\xEBV[\x90P\x81\x15a`1Wa`,\x82\x82a@\xEBV[a`3V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10a`LWa`LanaV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[_\x80a`m\x83Ban\xFDV[a`w\x90\x85aq\x94V[c\x01\xE13\x80\x90\x04\x90Pa\x03V\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaqNV[`@Q\x80`\xA0\x01`@R\x80a`\xA8ad\x07V[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80a\x01\xA0\x01`@R\x80a`\xDDadnV[\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01 \x01`@R\x80aa\\ad\xF5V[\x81R` \x01_\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80aa\xAEaepV[\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01`\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xC0\x01`@R\x80ab#a`\x95V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01abxa`\xC9V[\x90R\x90V[`@Q\x80a\x04\0\x01`@R\x80ab\x91ac\xE1V[\x81R` \x01ab\x9Ea`\x95V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80a\x01\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80ac\xD4a`\x95V[\x81R` \x01abxaa\x9BV[`@Q\x80``\x01`@R\x80ac\xF4ae\xC6V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[adX`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ad\x16W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ad\xDF`@Q\x80a\x01\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ad}W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[aeZ`@Q\x80a\x01@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\x04W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[ae\xB0`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\x7FW\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\x1E`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xD5W\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aF\xDCW__\xFD[_` \x82\x84\x03\x12\x15afXW__\xFD[\x815a\x03\x17\x81af4V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q``\x80\x88R`\xA0\x88\x01\x91\x90\x88\x01_[`\x02\x81\x10\x15agCW\x89\x84\x03`_\x19\x01\x82R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x80\x82\x01Q`\x80\x91\x87\x01\x82\x90R\x90ag\x17\x90\x87\x01\x82afcV[`@\x83\x81\x01Q\x90\x88\x01R``\x92\x83\x01Q\x92\x90\x96\x01\x91\x90\x91RP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01af\xDBV[PPP` \x82\x81\x01Q\x88\x82\x01R`@\x92\x83\x01Q\x92\x90\x97\x01\x91\x90\x91R\x94\x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01af\xB7V[P\x92\x96\x95PPPPPPV[____`\x80\x85\x87\x03\x12\x15ag\x8FW__\xFD[\x845ag\x9A\x81af4V[\x93P` \x85\x015ag\xAA\x81af4V[\x92P`@\x85\x015ag\xBA\x81af4V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[___``\x84\x86\x03\x12\x15ag\xDCW__\xFD[\x835ag\xE7\x81af4V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15ah9Wah9ag\xFCV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ahZWahZag\xFCV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15ahuW__\xFD[\x825ah\x80\x81af4V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\x9BW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13ah\xABW__\xFD[\x805ah\xBEah\xB9\x82ahAV[ah\x10V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15ah\xDFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15ai\x01W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ah\xE6V[\x80\x94PPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15ai\xF3W\x83Q\x80Q\x84_[`\x02\x81\x10\x15ai\xA2W\x82Q`\x01\x80`\xA0\x1B\x03\x81Q\x16\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R`\xA0\x81\x01Q`\xA0\x84\x01R`\xC0\x81\x01Q`\xC0\x84\x01RP`\xE0\x82\x01\x91P` \x83\x01\x92P`\x01\x81\x01\x90Pai7V[PPP` \x81\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\xC0\x87\x01R`@\x83\x01Q\x16a\x01\xE0\x86\x01R``\x82\x01Qa\x02\0\x86\x01R`\x80\x90\x91\x01Qa\x02 \x85\x01R\x93\x90\x93\x01\x92a\x02@\x90\x92\x01\x91`\x01\x01ai(V[P\x90\x95\x94PPPPPV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ai\xF3W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01\x80` \x86\x01Raj?a\x01\x80\x86\x01\x82afcV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01Ra\x01@\x82\x01Qa\x01@\x86\x01Ra\x01`\x82\x01Qa\x01`\x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Paj\x07V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01\xA0\x87Rak\x17a\x01\xA0\x88\x01\x82ai\xFEV[\x90P` \x82\x01Qak3` \x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`@\x82\x01QakN`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x01Q``\x88\x01R`\x80\x82\x01Q`\x80\x88\x01R`\xA0\x82\x01Q`\xA0\x88\x01R`\xC0\x82\x01Q`\xC0\x88\x01R`\xE0\x82\x01Qak\x91`\xE0\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Pa\x01\0\x82\x01Qak\xA7a\x01\0\x89\x01\x82\x15\x15\x90RV[Pa\x01 \x82\x81\x01Q\x90\x88\x01Ra\x01@\x80\x83\x01Q\x90\x88\x01Ra\x01`\x80\x83\x01Q\x90\x88\x01Ra\x01\x80\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aj\xEFV[__`@\x83\x85\x03\x12\x15ak\xF9W__\xFD[\x825al\x04\x81af4V[\x91P` \x83\x015al\x14\x81af4V[\x80\x91PP\x92P\x92\x90PV[_\x82`@\x81\x01\x83_[`\x02\x81\x10\x15ai\xF3W\x83\x83\x03\x87R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x81\x01Qa\x01@` \x86\x01Ral`a\x01@\x86\x01\x82afcV[\x90P`@\x82\x01Q`@\x86\x01R``\x82\x01Q``\x86\x01R`\x80\x82\x01Q`\x80\x86\x01R`\xA0\x82\x01Q`\xA0\x86\x01R`\xC0\x82\x01Q`\xC0\x86\x01R`\xE0\x82\x01Q`\xE0\x86\x01Ra\x01\0\x82\x01Qa\x01\0\x86\x01Ra\x01 \x82\x01Qa\x01 \x86\x01R\x80\x94PPP` \x82\x01\x91P` \x87\x01\x96P`\x01\x81\x01\x90Pal(V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15agpW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Qa\x01 \x87Ram a\x01 \x88\x01\x82al\x1FV[\x90P` \x82\x01Q` \x88\x01R`@\x82\x01QamF`@\x89\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P``\x82\x81\x01Q\x90\x88\x01R`\x80\x80\x83\x01Q\x90\x88\x01R`\xA0\x80\x83\x01Q\x90\x88\x01R`\xC0\x80\x83\x01Q\x90\x88\x01R`\xE0\x80\x83\x01Q\x90\x88\x01Ra\x01\0\x91\x82\x01Q\x91\x90\x96\x01R` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01al\xF8V[`\xFF\x81\x16\x81\x14aF\xDCW__\xFD[_____`\xA0\x86\x88\x03\x12\x15am\xBAW__\xFD[\x855am\xC5\x81af4V[\x94P` \x86\x015am\xD5\x81af4V[\x93P`@\x86\x015am\xE5\x81af4V[\x92P``\x86\x015\x91P`\x80\x86\x015am\xFC\x81am\x98V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[_____`\xA0\x86\x88\x03\x12\x15an\x1EW__\xFD[\x855an)\x81af4V[\x94P` \x86\x015an9\x81af4V[\x93P`@\x86\x015anI\x81af4V[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15an\xA8W__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15an\xBFW__\xFD[\x81Qa\x03\x17\x81af4V[_` \x82\x84\x03\x12\x15an\xDAW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x17W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03(Wa\x03(an\xE9V[_`\x01`\xFF\x1B\x82\x01ao$Wao$an\xE9V[P_\x03\x90V[_` \x82\x84\x03\x12\x15ao:W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aoPW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ao`W__\xFD[\x80Qaonah\xB9\x82ahAV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15ao\x8FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1B\x88W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90ao\x96V[_` \x82\x84\x03\x12\x15ao\xC1W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ao\xD7W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13ao\xE7W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ap\x01Wap\x01ag\xFCV[ap\x14`\x1F\x82\x01`\x1F\x19\x16` \x01ah\x10V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ap(W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15apUW__\xFD[\x81Qa\x03\x17\x81am\x98V[`\x01\x81[`\x01\x84\x11\x15ap\x9BW\x80\x85\x04\x81\x11\x15ap\x7FWap\x7Fan\xE9V[`\x01\x84\x16\x15ap\x8DW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02apdV[\x93P\x93\x91PPV[_\x82ap\xB1WP`\x01a\x03(V[\x81ap\xBDWP_a\x03(V[\x81`\x01\x81\x14ap\xD3W`\x02\x81\x14ap\xDDWap\xF9V[`\x01\x91PPa\x03(V[`\xFF\x84\x11\x15ap\xEEWap\xEEan\xE9V[PP`\x01\x82\x1Ba\x03(V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aq\x1CWP\x81\x81\na\x03(V[aq(_\x19\x84\x84ap`V[\x80_\x19\x04\x82\x11\x15aq;Waq;an\xE9V[\x02\x93\x92PPPV[_a\x03\x17\x83\x83ap\xA3V[\x80\x82\x01\x80\x82\x11\x15a\x03(Wa\x03(an\xE9V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15aA\xADWaA\xADan\xE9V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03(Wa\x03(an\xE9V[_\x82aq\xC5WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x16F\xEE\xFAl5A\xC4\xB9\xAE\xBC{w\xC5\xFA\x8F\xBEKZ\xE0\xFC!m.~)\x86^\xF7\xFE\xEC\xEAdsolcC\0\x08\x1C\x003",
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
