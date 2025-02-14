///Module containing a contract's types and functions.
/**

```solidity
library Event {
    struct Liquidation { uint256 baseCollateral; uint256 baseDebtScaled; uint256 memeCollateral; uint256 memeDebtScaled; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod Event {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Liquidation { uint256 baseCollateral; uint256 baseDebtScaled; uint256 memeCollateral; uint256 memeDebtScaled; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Liquidation {
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<Liquidation> for UnderlyingRustTuple<'_> {
            fn from(value: Liquidation) -> Self {
                (
                    value.baseCollateral,
                    value.baseDebtScaled,
                    value.memeCollateral,
                    value.memeDebtScaled,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Liquidation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    baseCollateral: tuple.0,
                    baseDebtScaled: tuple.1,
                    memeCollateral: tuple.2,
                    memeDebtScaled: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Liquidation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Liquidation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseDebtScaled),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeDebtScaled),
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
        impl alloy_sol_types::SolType for Liquidation {
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
        impl alloy_sol_types::SolStruct for Liquidation {
            const NAME: &'static str = "Liquidation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Liquidation(uint256 baseCollateral,uint256 baseDebtScaled,uint256 memeCollateral,uint256 memeDebtScaled)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.baseCollateral,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.baseDebtScaled,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.memeCollateral,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.memeDebtScaled,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Liquidation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseCollateral,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.baseDebtScaled,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.memeCollateral,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.memeDebtScaled,
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
                    &rust.baseCollateral,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.baseDebtScaled,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.memeCollateral,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.memeDebtScaled,
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
    /**Creates a new wrapper around an on-chain [`Event`](self) contract instance.

See the [wrapper's documentation](`EventInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EventInstance<T, P, N> {
        EventInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Event`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Event`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EventInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EventInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EventInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EventInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Event`](self) contract instance.

See the [wrapper's documentation](`EventInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> EventInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EventInstance<T, P, N> {
            EventInstance {
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
    > EventInstance<T, P, N> {
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
    > EventInstance<T, P, N> {
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
library Event {
    struct Liquidation {
        uint256 baseCollateral;
        uint256 baseDebtScaled;
        uint256 memeCollateral;
        uint256 memeDebtScaled;
    }
}

interface IEventEmitter {
    function emitAdd(address adder, address baseToken, address memeToken, address to, uint256 amount0, uint256 amount1) external;
    function emitBorrow(address borrower, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 borrowAmount, uint256 borrowRate, Event.Liquidation memory liquidation) external;
    function emitClaimFees(address underlyingAsset, uint256 scaledUnclaimedFee, uint256 liquidityIndex, uint256 unclaimedFee) external;
    function emitClose(address account, uint256 positionId) external;
    function emitDeposit(address depositor, address baseToken, address memeToken, uint256 positionId, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
    function emitLiquidation(address liquidator, address account, uint256 positionId, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd, uint256 memePrice) external;
    function emitPoolCreated(address baseToken, address memeToken, address source, uint256 createdTimestamp, uint256 baseDecimals, uint256 memeDecimals) external;
    function emitPoolUpdated(address underlyingAsset, uint256 liquidityRate, uint256 borrowRate, uint256 liquidityIndex, uint256 borrowIndex) external;
    function emitPosition(address account, uint256 actionType, address baseToken, address memeToken, uint256 positionId, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
    function emitRemove(address remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 amount0, uint256 amount1) external;
    function emitRepay(address repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, Event.Liquidation memory liquidation) external;
    function emitSwap(address account, address tokenIn, address tokenOut, uint256 positionId, uint256 amountIn, uint256 amountOut, uint256 fee, Event.Liquidation memory liquidation) external;
    function emitWithdraw(address withdrawer, address baseToken, address memeToken, uint256 positionId, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "emitAdd",
    "inputs": [
      {
        "name": "adder",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitBorrow",
    "inputs": [
      {
        "name": "borrower",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "tokenIndex",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "borrowAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "borrowRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidation",
        "type": "tuple",
        "internalType": "struct Event.Liquidation",
        "components": [
          {
            "name": "baseCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseDebtScaled",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeDebtScaled",
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
    "name": "emitClaimFees",
    "inputs": [
      {
        "name": "underlyingAsset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "scaledUnclaimedFee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidityIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "unclaimedFee",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitClose",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitDeposit",
    "inputs": [
      {
        "name": "depositor",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "depositAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitLiquidation",
    "inputs": [
      {
        "name": "liquidator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "marginLevel",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "marginLevelLiquidationThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalCollateralUsd",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalDebtUsd",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memePrice",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitPoolCreated",
    "inputs": [
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "source",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "createdTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseDecimals",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeDecimals",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitPoolUpdated",
    "inputs": [
      {
        "name": "underlyingAsset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "liquidityRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "borrowRate",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidityIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "borrowIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitPosition",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "actionType",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitRemove",
    "inputs": [
      {
        "name": "remover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitRepay",
    "inputs": [
      {
        "name": "repayer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "tokenIndex",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "repayAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidation",
        "type": "tuple",
        "internalType": "struct Event.Liquidation",
        "components": [
          {
            "name": "baseCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseDebtScaled",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeDebtScaled",
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
    "name": "emitSwap",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenIn",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenOut",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountIn",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountOut",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "fee",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "liquidation",
        "type": "tuple",
        "internalType": "struct Event.Liquidation",
        "components": [
          {
            "name": "baseCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "baseDebtScaled",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeCollateral",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "memeDebtScaled",
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
    "name": "emitWithdraw",
    "inputs": [
      {
        "name": "withdrawer",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "withdrawAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IEventEmitter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**Function with signature `emitAdd(address,address,address,address,uint256,uint256)` and selector `0x9ed486eb`.
```solidity
function emitAdd(address adder, address baseToken, address memeToken, address to, uint256 amount0, uint256 amount1) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitAddCall {
        pub adder: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitAdd(address,address,address,address,uint256,uint256)`](emitAddCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitAddReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitAddCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitAddCall) -> Self {
                    (
                        value.adder,
                        value.baseToken,
                        value.memeToken,
                        value.to,
                        value.amount0,
                        value.amount1,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        adder: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        to: tuple.3,
                        amount0: tuple.4,
                        amount1: tuple.5,
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
            impl ::core::convert::From<emitAddReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitAddReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitAddReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitAddCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitAddReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitAdd(address,address,address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [158u8, 212u8, 134u8, 235u8];
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
                        &self.adder,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
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
    /**Function with signature `emitBorrow(address,address,address,uint256,uint8,uint256,uint256,(uint256,uint256,uint256,uint256))` and selector `0xea34a577`.
```solidity
function emitBorrow(address borrower, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 borrowAmount, uint256 borrowRate, Event.Liquidation memory liquidation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitBorrowCall {
        pub borrower: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub borrowAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`emitBorrow(address,address,address,uint256,uint8,uint256,uint256,(uint256,uint256,uint256,uint256))`](emitBorrowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitBorrowReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <Event::Liquidation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<emitBorrowCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitBorrowCall) -> Self {
                    (
                        value.borrower,
                        value.baseToken,
                        value.memeToken,
                        value.positionId,
                        value.tokenIndex,
                        value.borrowAmount,
                        value.borrowRate,
                        value.liquidation,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitBorrowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        borrower: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        positionId: tuple.3,
                        tokenIndex: tuple.4,
                        borrowAmount: tuple.5,
                        borrowRate: tuple.6,
                        liquidation: tuple.7,
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
            impl ::core::convert::From<emitBorrowReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitBorrowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitBorrowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitBorrowCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitBorrowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitBorrow(address,address,address,uint256,uint8,uint256,uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [234u8, 52u8, 165u8, 119u8];
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
                        &self.borrower,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowRate),
                    <Event::Liquidation as alloy_sol_types::SolType>::tokenize(
                        &self.liquidation,
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
    /**Function with signature `emitClaimFees(address,uint256,uint256,uint256)` and selector `0x9c845792`.
```solidity
function emitClaimFees(address underlyingAsset, uint256 scaledUnclaimedFee, uint256 liquidityIndex, uint256 unclaimedFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitClaimFeesCall {
        pub underlyingAsset: alloy::sol_types::private::Address,
        pub scaledUnclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub unclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitClaimFees(address,uint256,uint256,uint256)`](emitClaimFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitClaimFeesReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitClaimFeesCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitClaimFeesCall) -> Self {
                    (
                        value.underlyingAsset,
                        value.scaledUnclaimedFee,
                        value.liquidityIndex,
                        value.unclaimedFee,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitClaimFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        underlyingAsset: tuple.0,
                        scaledUnclaimedFee: tuple.1,
                        liquidityIndex: tuple.2,
                        unclaimedFee: tuple.3,
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
            impl ::core::convert::From<emitClaimFeesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitClaimFeesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitClaimFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitClaimFeesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitClaimFeesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitClaimFees(address,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [156u8, 132u8, 87u8, 146u8];
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
                        &self.underlyingAsset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.scaledUnclaimedFee),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.unclaimedFee),
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
    /**Function with signature `emitClose(address,uint256)` and selector `0x5a7a3776`.
```solidity
function emitClose(address account, uint256 positionId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitCloseCall {
        pub account: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitClose(address,uint256)`](emitCloseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitCloseReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<emitCloseCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitCloseCall) -> Self {
                    (value.account, value.positionId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitCloseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        positionId: tuple.1,
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
            impl ::core::convert::From<emitCloseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitCloseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitCloseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitCloseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitCloseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitClose(address,uint256)";
            const SELECTOR: [u8; 4] = [90u8, 122u8, 55u8, 118u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
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
    /**Function with signature `emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x119c6c83`.
```solidity
function emitDeposit(address depositor, address baseToken, address memeToken, uint256 positionId, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitDepositCall {
        pub depositor: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub depositAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256)`](emitDepositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitDepositReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
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
            impl ::core::convert::From<emitDepositCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitDepositCall) -> Self {
                    (
                        value.depositor,
                        value.baseToken,
                        value.memeToken,
                        value.positionId,
                        value.depositAmount,
                        value.baseCollateral,
                        value.baseDebtScaled,
                        value.memeCollateral,
                        value.memeDebtScaled,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitDepositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        depositor: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        positionId: tuple.3,
                        depositAmount: tuple.4,
                        baseCollateral: tuple.5,
                        baseDebtScaled: tuple.6,
                        memeCollateral: tuple.7,
                        memeDebtScaled: tuple.8,
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
            impl ::core::convert::From<emitDepositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitDepositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitDepositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitDepositCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [17u8, 156u8, 108u8, 131u8];
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
                        &self.depositor,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseDebtScaled),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeDebtScaled),
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
    /**Function with signature `emitLiquidation(address,address,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x42ff64a5`.
```solidity
function emitLiquidation(address liquidator, address account, uint256 positionId, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd, uint256 memePrice) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitLiquidationCall {
        pub liquidator: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub marginLevel: alloy::sol_types::private::primitives::aliases::U256,
        pub marginLevelLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateralUsd: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebtUsd: alloy::sol_types::private::primitives::aliases::U256,
        pub memePrice: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitLiquidation(address,address,uint256,uint256,uint256,uint256,uint256,uint256)`](emitLiquidationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitLiquidationReturn {}
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
            impl ::core::convert::From<emitLiquidationCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitLiquidationCall) -> Self {
                    (
                        value.liquidator,
                        value.account,
                        value.positionId,
                        value.marginLevel,
                        value.marginLevelLiquidationThreshold,
                        value.totalCollateralUsd,
                        value.totalDebtUsd,
                        value.memePrice,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitLiquidationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        liquidator: tuple.0,
                        account: tuple.1,
                        positionId: tuple.2,
                        marginLevel: tuple.3,
                        marginLevelLiquidationThreshold: tuple.4,
                        totalCollateralUsd: tuple.5,
                        totalDebtUsd: tuple.6,
                        memePrice: tuple.7,
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
            impl ::core::convert::From<emitLiquidationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: emitLiquidationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emitLiquidationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitLiquidationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitLiquidationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitLiquidation(address,address,uint256,uint256,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [66u8, 255u8, 100u8, 165u8];
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
                        &self.liquidator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.marginLevel),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.marginLevelLiquidationThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalCollateralUsd),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalDebtUsd),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memePrice),
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
    /**Function with signature `emitPoolCreated(address,address,address,uint256,uint256,uint256)` and selector `0x55ac84ba`.
```solidity
function emitPoolCreated(address baseToken, address memeToken, address source, uint256 createdTimestamp, uint256 baseDecimals, uint256 memeDecimals) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPoolCreatedCall {
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub source: alloy::sol_types::private::Address,
        pub createdTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDecimals: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDecimals: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitPoolCreated(address,address,address,uint256,uint256,uint256)`](emitPoolCreatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPoolCreatedReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitPoolCreatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitPoolCreatedCall) -> Self {
                    (
                        value.baseToken,
                        value.memeToken,
                        value.source,
                        value.createdTimestamp,
                        value.baseDecimals,
                        value.memeDecimals,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitPoolCreatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        baseToken: tuple.0,
                        memeToken: tuple.1,
                        source: tuple.2,
                        createdTimestamp: tuple.3,
                        baseDecimals: tuple.4,
                        memeDecimals: tuple.5,
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
            impl ::core::convert::From<emitPoolCreatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: emitPoolCreatedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emitPoolCreatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitPoolCreatedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitPoolCreatedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitPoolCreated(address,address,address,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [85u8, 172u8, 132u8, 186u8];
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
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.source,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseDecimals),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeDecimals),
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
    /**Function with signature `emitPoolUpdated(address,uint256,uint256,uint256,uint256)` and selector `0x7c24dab7`.
```solidity
function emitPoolUpdated(address underlyingAsset, uint256 liquidityRate, uint256 borrowRate, uint256 liquidityIndex, uint256 borrowIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPoolUpdatedCall {
        pub underlyingAsset: alloy::sol_types::private::Address,
        pub liquidityRate: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub borrowIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitPoolUpdated(address,uint256,uint256,uint256,uint256)`](emitPoolUpdatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPoolUpdatedReturn {}
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
            impl ::core::convert::From<emitPoolUpdatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitPoolUpdatedCall) -> Self {
                    (
                        value.underlyingAsset,
                        value.liquidityRate,
                        value.borrowRate,
                        value.liquidityIndex,
                        value.borrowIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitPoolUpdatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        underlyingAsset: tuple.0,
                        liquidityRate: tuple.1,
                        borrowRate: tuple.2,
                        liquidityIndex: tuple.3,
                        borrowIndex: tuple.4,
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
            impl ::core::convert::From<emitPoolUpdatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: emitPoolUpdatedReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for emitPoolUpdatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitPoolUpdatedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitPoolUpdatedReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitPoolUpdated(address,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [124u8, 36u8, 218u8, 183u8];
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
                        &self.underlyingAsset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowRate),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowIndex),
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
    /**Function with signature `emitPosition(address,uint256,address,address,uint256,uint256,uint256,uint256,uint256)` and selector `0x09cd7ba2`.
```solidity
function emitPosition(address account, uint256 actionType, address baseToken, address memeToken, uint256 positionId, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPositionCall {
        pub account: alloy::sol_types::private::Address,
        pub actionType: alloy::sol_types::private::primitives::aliases::U256,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitPosition(address,uint256,address,address,uint256,uint256,uint256,uint256,uint256)`](emitPositionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitPositionReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitPositionCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitPositionCall) -> Self {
                    (
                        value.account,
                        value.actionType,
                        value.baseToken,
                        value.memeToken,
                        value.positionId,
                        value.baseCollateral,
                        value.baseDebtScaled,
                        value.memeCollateral,
                        value.memeDebtScaled,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitPositionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        actionType: tuple.1,
                        baseToken: tuple.2,
                        memeToken: tuple.3,
                        positionId: tuple.4,
                        baseCollateral: tuple.5,
                        baseDebtScaled: tuple.6,
                        memeCollateral: tuple.7,
                        memeDebtScaled: tuple.8,
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
            impl ::core::convert::From<emitPositionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitPositionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitPositionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitPositionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitPositionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitPosition(address,uint256,address,address,uint256,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 205u8, 123u8, 162u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actionType),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseDebtScaled),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeDebtScaled),
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
    /**Function with signature `emitRemove(address,address,address,uint256,address,uint256,uint256)` and selector `0x292ae722`.
```solidity
function emitRemove(address remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 amount0, uint256 amount1) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRemoveCall {
        pub remover: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitRemove(address,address,address,uint256,address,uint256,uint256)`](emitRemoveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRemoveReturn {}
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
            impl ::core::convert::From<emitRemoveCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitRemoveCall) -> Self {
                    (
                        value.remover,
                        value.baseToken,
                        value.memeToken,
                        value.liquidity,
                        value.to,
                        value.amount0,
                        value.amount1,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitRemoveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        remover: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        liquidity: tuple.3,
                        to: tuple.4,
                        amount0: tuple.5,
                        amount1: tuple.6,
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
            impl ::core::convert::From<emitRemoveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitRemoveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitRemoveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitRemoveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitRemoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitRemove(address,address,address,uint256,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [41u8, 42u8, 231u8, 34u8];
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
                        &self.remover,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
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
    /**Function with signature `emitRepay(address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))` and selector `0x8262009e`.
```solidity
function emitRepay(address repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, Event.Liquidation memory liquidation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRepayCall {
        pub repayer: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub repayAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`emitRepay(address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))`](emitRepayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRepayReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
                <Event::Liquidation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<emitRepayCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitRepayCall) -> Self {
                    (
                        value.repayer,
                        value.baseToken,
                        value.memeToken,
                        value.positionId,
                        value.tokenIndex,
                        value.repayAmount,
                        value.liquidation,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitRepayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        repayer: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        positionId: tuple.3,
                        tokenIndex: tuple.4,
                        repayAmount: tuple.5,
                        liquidation: tuple.6,
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
            impl ::core::convert::From<emitRepayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitRepayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitRepayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitRepayCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitRepayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitRepay(address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [130u8, 98u8, 0u8, 158u8];
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
                        &self.repayer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.repayAmount),
                    <Event::Liquidation as alloy_sol_types::SolType>::tokenize(
                        &self.liquidation,
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
    /**Function with signature `emitSwap(address,address,address,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))` and selector `0x11ccb21d`.
```solidity
function emitSwap(address account, address tokenIn, address tokenOut, uint256 positionId, uint256 amountIn, uint256 amountOut, uint256 fee, Event.Liquidation memory liquidation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitSwapCall {
        pub account: alloy::sol_types::private::Address,
        pub tokenIn: alloy::sol_types::private::Address,
        pub tokenOut: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        pub amountOut: alloy::sol_types::private::primitives::aliases::U256,
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`emitSwap(address,address,address,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))`](emitSwapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitSwapReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                <Event::Liquidation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<emitSwapCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitSwapCall) -> Self {
                    (
                        value.account,
                        value.tokenIn,
                        value.tokenOut,
                        value.positionId,
                        value.amountIn,
                        value.amountOut,
                        value.fee,
                        value.liquidation,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitSwapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        tokenIn: tuple.1,
                        tokenOut: tuple.2,
                        positionId: tuple.3,
                        amountIn: tuple.4,
                        amountOut: tuple.5,
                        fee: tuple.6,
                        liquidation: tuple.7,
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
            impl ::core::convert::From<emitSwapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitSwapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitSwapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitSwapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                Event::Liquidation,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitSwapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitSwap(address,address,address,uint256,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [17u8, 204u8, 178u8, 29u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenOut,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOut),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <Event::Liquidation as alloy_sol_types::SolType>::tokenize(
                        &self.liquidation,
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
    /**Function with signature `emitWithdraw(address,address,address,uint256,uint256,address,uint256,uint256,uint256,uint256)` and selector `0x82fcd8ca`.
```solidity
function emitWithdraw(address withdrawer, address baseToken, address memeToken, uint256 positionId, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitWithdrawCall {
        pub withdrawer: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitWithdraw(address,address,address,uint256,uint256,address,uint256,uint256,uint256,uint256)`](emitWithdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitWithdrawReturn {}
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
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
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitWithdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitWithdrawCall) -> Self {
                    (
                        value.withdrawer,
                        value.baseToken,
                        value.memeToken,
                        value.positionId,
                        value.withdrawAmount,
                        value.to,
                        value.baseCollateral,
                        value.baseDebtScaled,
                        value.memeCollateral,
                        value.memeDebtScaled,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitWithdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawer: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        positionId: tuple.3,
                        withdrawAmount: tuple.4,
                        to: tuple.5,
                        baseCollateral: tuple.6,
                        baseDebtScaled: tuple.7,
                        memeCollateral: tuple.8,
                        memeDebtScaled: tuple.9,
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
            impl ::core::convert::From<emitWithdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emitWithdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitWithdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emitWithdrawCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitWithdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitWithdraw(address,address,address,uint256,uint256,address,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [130u8, 252u8, 216u8, 202u8];
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
                        &self.withdrawer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawAmount),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseDebtScaled),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeCollateral),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeDebtScaled),
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
    ///Container for all the [`IEventEmitter`](self) function calls.
    pub enum IEventEmitterCalls {
        emitAdd(emitAddCall),
        emitBorrow(emitBorrowCall),
        emitClaimFees(emitClaimFeesCall),
        emitClose(emitCloseCall),
        emitDeposit(emitDepositCall),
        emitLiquidation(emitLiquidationCall),
        emitPoolCreated(emitPoolCreatedCall),
        emitPoolUpdated(emitPoolUpdatedCall),
        emitPosition(emitPositionCall),
        emitRemove(emitRemoveCall),
        emitRepay(emitRepayCall),
        emitSwap(emitSwapCall),
        emitWithdraw(emitWithdrawCall),
    }
    #[automatically_derived]
    impl IEventEmitterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 205u8, 123u8, 162u8],
            [17u8, 156u8, 108u8, 131u8],
            [17u8, 204u8, 178u8, 29u8],
            [41u8, 42u8, 231u8, 34u8],
            [66u8, 255u8, 100u8, 165u8],
            [85u8, 172u8, 132u8, 186u8],
            [90u8, 122u8, 55u8, 118u8],
            [124u8, 36u8, 218u8, 183u8],
            [130u8, 98u8, 0u8, 158u8],
            [130u8, 252u8, 216u8, 202u8],
            [156u8, 132u8, 87u8, 146u8],
            [158u8, 212u8, 134u8, 235u8],
            [234u8, 52u8, 165u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IEventEmitterCalls {
        const NAME: &'static str = "IEventEmitterCalls";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::emitAdd(_) => <emitAddCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::emitBorrow(_) => {
                    <emitBorrowCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitClaimFees(_) => {
                    <emitClaimFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitClose(_) => {
                    <emitCloseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitDeposit(_) => {
                    <emitDepositCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitLiquidation(_) => {
                    <emitLiquidationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitPoolCreated(_) => {
                    <emitPoolCreatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitPoolUpdated(_) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitPosition(_) => {
                    <emitPositionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitRemove(_) => {
                    <emitRemoveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitRepay(_) => {
                    <emitRepayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emitSwap(_) => <emitSwapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::emitWithdraw(_) => {
                    <emitWithdrawCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IEventEmitterCalls>] = &[
                {
                    fn emitPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitPositionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitPosition)
                    }
                    emitPosition
                },
                {
                    fn emitDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitDeposit)
                    }
                    emitDeposit
                },
                {
                    fn emitSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitSwap)
                    }
                    emitSwap
                },
                {
                    fn emitRemove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitRemoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitRemove)
                    }
                    emitRemove
                },
                {
                    fn emitLiquidation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitLiquidationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitLiquidation)
                    }
                    emitLiquidation
                },
                {
                    fn emitPoolCreated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitPoolCreatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitPoolCreated)
                    }
                    emitPoolCreated
                },
                {
                    fn emitClose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitCloseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitClose)
                    }
                    emitClose
                },
                {
                    fn emitPoolUpdated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitPoolUpdated)
                    }
                    emitPoolUpdated
                },
                {
                    fn emitRepay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitRepayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitRepay)
                    }
                    emitRepay
                },
                {
                    fn emitWithdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitWithdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitWithdraw)
                    }
                    emitWithdraw
                },
                {
                    fn emitClaimFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitClaimFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitClaimFees)
                    }
                    emitClaimFees
                },
                {
                    fn emitAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitAdd)
                    }
                    emitAdd
                },
                {
                    fn emitBorrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IEventEmitterCalls> {
                        <emitBorrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IEventEmitterCalls::emitBorrow)
                    }
                    emitBorrow
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
                Self::emitAdd(inner) => {
                    <emitAddCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitBorrow(inner) => {
                    <emitBorrowCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitClaimFees(inner) => {
                    <emitClaimFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitClose(inner) => {
                    <emitCloseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitDeposit(inner) => {
                    <emitDepositCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitLiquidation(inner) => {
                    <emitLiquidationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitPoolCreated(inner) => {
                    <emitPoolCreatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitPoolUpdated(inner) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitPosition(inner) => {
                    <emitPositionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emitRemove(inner) => {
                    <emitRemoveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitRepay(inner) => {
                    <emitRepayCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitSwap(inner) => {
                    <emitSwapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::emitWithdraw(inner) => {
                    <emitWithdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::emitAdd(inner) => {
                    <emitAddCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::emitBorrow(inner) => {
                    <emitBorrowCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitClaimFees(inner) => {
                    <emitClaimFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitClose(inner) => {
                    <emitCloseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitDeposit(inner) => {
                    <emitDepositCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitLiquidation(inner) => {
                    <emitLiquidationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitPoolCreated(inner) => {
                    <emitPoolCreatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitPoolUpdated(inner) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitPosition(inner) => {
                    <emitPositionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitRemove(inner) => {
                    <emitRemoveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitRepay(inner) => {
                    <emitRepayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitSwap(inner) => {
                    <emitSwapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emitWithdraw(inner) => {
                    <emitWithdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IEventEmitter`](self) contract instance.

See the [wrapper's documentation](`IEventEmitterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEventEmitterInstance<T, P, N> {
        IEventEmitterInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<IEventEmitterInstance<T, P, N>>,
    > {
        IEventEmitterInstance::<T, P, N>::deploy(provider)
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
        IEventEmitterInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IEventEmitter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IEventEmitter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEventEmitterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IEventEmitterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEventEmitterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEventEmitterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IEventEmitter`](self) contract instance.

See the [wrapper's documentation](`IEventEmitterInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IEventEmitterInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> IEventEmitterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEventEmitterInstance<T, P, N> {
            IEventEmitterInstance {
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
    > IEventEmitterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`emitAdd`] function.
        pub fn emitAdd(
            &self,
            adder: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount0: alloy::sol_types::private::primitives::aliases::U256,
            amount1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitAddCall, N> {
            self.call_builder(
                &emitAddCall {
                    adder,
                    baseToken,
                    memeToken,
                    to,
                    amount0,
                    amount1,
                },
            )
        }
        ///Creates a new call builder for the [`emitBorrow`] function.
        pub fn emitBorrow(
            &self,
            borrower: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            tokenIndex: u8,
            borrowAmount: alloy::sol_types::private::primitives::aliases::U256,
            borrowRate: alloy::sol_types::private::primitives::aliases::U256,
            liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitBorrowCall, N> {
            self.call_builder(
                &emitBorrowCall {
                    borrower,
                    baseToken,
                    memeToken,
                    positionId,
                    tokenIndex,
                    borrowAmount,
                    borrowRate,
                    liquidation,
                },
            )
        }
        ///Creates a new call builder for the [`emitClaimFees`] function.
        pub fn emitClaimFees(
            &self,
            underlyingAsset: alloy::sol_types::private::Address,
            scaledUnclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
            liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
            unclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitClaimFeesCall, N> {
            self.call_builder(
                &emitClaimFeesCall {
                    underlyingAsset,
                    scaledUnclaimedFee,
                    liquidityIndex,
                    unclaimedFee,
                },
            )
        }
        ///Creates a new call builder for the [`emitClose`] function.
        pub fn emitClose(
            &self,
            account: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitCloseCall, N> {
            self.call_builder(
                &emitCloseCall {
                    account,
                    positionId,
                },
            )
        }
        ///Creates a new call builder for the [`emitDeposit`] function.
        pub fn emitDeposit(
            &self,
            depositor: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            depositAmount: alloy::sol_types::private::primitives::aliases::U256,
            baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
            baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
            memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
            memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitDepositCall, N> {
            self.call_builder(
                &emitDepositCall {
                    depositor,
                    baseToken,
                    memeToken,
                    positionId,
                    depositAmount,
                    baseCollateral,
                    baseDebtScaled,
                    memeCollateral,
                    memeDebtScaled,
                },
            )
        }
        ///Creates a new call builder for the [`emitLiquidation`] function.
        pub fn emitLiquidation(
            &self,
            liquidator: alloy::sol_types::private::Address,
            account: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            marginLevel: alloy::sol_types::private::primitives::aliases::U256,
            marginLevelLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
            totalCollateralUsd: alloy::sol_types::private::primitives::aliases::U256,
            totalDebtUsd: alloy::sol_types::private::primitives::aliases::U256,
            memePrice: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitLiquidationCall, N> {
            self.call_builder(
                &emitLiquidationCall {
                    liquidator,
                    account,
                    positionId,
                    marginLevel,
                    marginLevelLiquidationThreshold,
                    totalCollateralUsd,
                    totalDebtUsd,
                    memePrice,
                },
            )
        }
        ///Creates a new call builder for the [`emitPoolCreated`] function.
        pub fn emitPoolCreated(
            &self,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            source: alloy::sol_types::private::Address,
            createdTimestamp: alloy::sol_types::private::primitives::aliases::U256,
            baseDecimals: alloy::sol_types::private::primitives::aliases::U256,
            memeDecimals: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitPoolCreatedCall, N> {
            self.call_builder(
                &emitPoolCreatedCall {
                    baseToken,
                    memeToken,
                    source,
                    createdTimestamp,
                    baseDecimals,
                    memeDecimals,
                },
            )
        }
        ///Creates a new call builder for the [`emitPoolUpdated`] function.
        pub fn emitPoolUpdated(
            &self,
            underlyingAsset: alloy::sol_types::private::Address,
            liquidityRate: alloy::sol_types::private::primitives::aliases::U256,
            borrowRate: alloy::sol_types::private::primitives::aliases::U256,
            liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
            borrowIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitPoolUpdatedCall, N> {
            self.call_builder(
                &emitPoolUpdatedCall {
                    underlyingAsset,
                    liquidityRate,
                    borrowRate,
                    liquidityIndex,
                    borrowIndex,
                },
            )
        }
        ///Creates a new call builder for the [`emitPosition`] function.
        pub fn emitPosition(
            &self,
            account: alloy::sol_types::private::Address,
            actionType: alloy::sol_types::private::primitives::aliases::U256,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
            baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
            memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
            memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitPositionCall, N> {
            self.call_builder(
                &emitPositionCall {
                    account,
                    actionType,
                    baseToken,
                    memeToken,
                    positionId,
                    baseCollateral,
                    baseDebtScaled,
                    memeCollateral,
                    memeDebtScaled,
                },
            )
        }
        ///Creates a new call builder for the [`emitRemove`] function.
        pub fn emitRemove(
            &self,
            remover: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            amount0: alloy::sol_types::private::primitives::aliases::U256,
            amount1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitRemoveCall, N> {
            self.call_builder(
                &emitRemoveCall {
                    remover,
                    baseToken,
                    memeToken,
                    liquidity,
                    to,
                    amount0,
                    amount1,
                },
            )
        }
        ///Creates a new call builder for the [`emitRepay`] function.
        pub fn emitRepay(
            &self,
            repayer: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            tokenIndex: u8,
            repayAmount: alloy::sol_types::private::primitives::aliases::U256,
            liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitRepayCall, N> {
            self.call_builder(
                &emitRepayCall {
                    repayer,
                    baseToken,
                    memeToken,
                    positionId,
                    tokenIndex,
                    repayAmount,
                    liquidation,
                },
            )
        }
        ///Creates a new call builder for the [`emitSwap`] function.
        pub fn emitSwap(
            &self,
            account: alloy::sol_types::private::Address,
            tokenIn: alloy::sol_types::private::Address,
            tokenOut: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            amountIn: alloy::sol_types::private::primitives::aliases::U256,
            amountOut: alloy::sol_types::private::primitives::aliases::U256,
            fee: alloy::sol_types::private::primitives::aliases::U256,
            liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitSwapCall, N> {
            self.call_builder(
                &emitSwapCall {
                    account,
                    tokenIn,
                    tokenOut,
                    positionId,
                    amountIn,
                    amountOut,
                    fee,
                    liquidation,
                },
            )
        }
        ///Creates a new call builder for the [`emitWithdraw`] function.
        pub fn emitWithdraw(
            &self,
            withdrawer: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            positionId: alloy::sol_types::private::primitives::aliases::U256,
            withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
            baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
            memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
            memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitWithdrawCall, N> {
            self.call_builder(
                &emitWithdrawCall {
                    withdrawer,
                    baseToken,
                    memeToken,
                    positionId,
                    withdrawAmount,
                    to,
                    baseCollateral,
                    baseDebtScaled,
                    memeCollateral,
                    memeDebtScaled,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEventEmitterInstance<T, P, N> {
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
