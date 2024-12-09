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

interface EventEmitter {
    error Unauthorized(address msgSender, string role);

    event Add(address indexed adder, address baseToken, address memeToken, address to, uint256 baseAmount, uint256 memeAmount);
    event Borrow(address indexed borrower, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 borrowAmount, uint256 borrowRate, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
    event ClaimFees(address indexed pool, uint256 scaledUnclaimedFee, uint256 liquidityIndex, uint256 unclaimedFee);
    event Close(address indexed poolUsd, address indexed account, uint256 amountUsdStartClose, uint256 amountUsdAfterRepayAndSellCollateral, uint256 amountUsdAfterBuyCollateralAndRepay);
    event Deposit(address indexed depositor, address baseToken, address memeToken, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
    event Liquidation(address indexed liquidator, address indexed account, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd);
    event PoolUpdated(address indexed pool, uint256 liquidityRate, uint256 borrowRate, uint256 liquidityIndex, uint256 borrowIndex);
    event Remove(address indexed remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 baseAmount, uint256 memeAmount);
    event Repay(address indexed repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
    event Swap(address indexed account, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
    event Withdraw(address indexed withdrawer, address baseToken, address memeToken, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);

    constructor(address _roleStore);

    function emitAdd(address supplier, address baseToken, address memeToken, address to, uint256 baseAmount, uint256 memeAmount) external;
    function emitBorrow(address borrower, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 borrowAmount, uint256 borrowRate, Event.Liquidation memory liquidation) external;
    function emitClaimFees(address underlyingAsset, uint256 scaledUnclaimedFee, uint256 liquidityIndex, uint256 unclaimedFee) external;
    function emitClose(address underlyingAssetUsd, address account, uint256 amountUsdStartClose, uint256 amountUsdAfterRepayAndSellCollateral, uint256 amountUsdAfterBuyCollateralAndRepay) external;
    function emitDeposit(address depositor, address baseToken, address memeToken, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
    function emitLiquidation(address liquidator, address account, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd) external;
    function emitPoolUpdated(address underlyingAsset, uint256 liquidityRate, uint256 borrowRate, uint256 liquidityIndex, uint256 borrowIndex) external;
    function emitRemove(address remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 baseAmount, uint256 memeAmount) external;
    function emitRepay(address eventEmitter, address repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, Event.Liquidation memory liquidation) external;
    function emitSwap(address account, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee, Event.Liquidation memory liquidation) external;
    function emitWithdraw(address withdrawer, address baseToken, address memeToken, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
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
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "emitAdd",
    "inputs": [
      {
        "name": "supplier",
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
        "name": "baseAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeAmount",
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
        "name": "underlyingAssetUsd",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountUsdStartClose",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountUsdAfterRepayAndSellCollateral",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amountUsdAfterBuyCollateralAndRepay",
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
        "name": "baseAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "memeAmount",
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
        "name": "eventEmitter",
        "type": "address",
        "internalType": "address"
      },
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
    "type": "event",
    "name": "Add",
    "inputs": [
      {
        "name": "adder",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "baseAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Borrow",
    "inputs": [
      {
        "name": "borrower",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "tokenIndex",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "borrowAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "borrowRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ClaimFees",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "scaledUnclaimedFee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "liquidityIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "unclaimedFee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Close",
    "inputs": [
      {
        "name": "poolUsd",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amountUsdStartClose",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amountUsdAfterRepayAndSellCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amountUsdAfterBuyCollateralAndRepay",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Deposit",
    "inputs": [
      {
        "name": "depositor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "depositAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Liquidation",
    "inputs": [
      {
        "name": "liquidator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "marginLevel",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "marginLevelLiquidationThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "totalCollateralUsd",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "totalDebtUsd",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PoolUpdated",
    "inputs": [
      {
        "name": "pool",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "liquidityRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "borrowRate",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "liquidityIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "borrowIndex",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Remove",
    "inputs": [
      {
        "name": "remover",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "baseAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Repay",
    "inputs": [
      {
        "name": "repayer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "positionId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "tokenIndex",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "repayAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Swap",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenIn",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "tokenOut",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amountIn",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amountOut",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "fee",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Withdraw",
    "inputs": [
      {
        "name": "withdrawer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "baseToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "memeToken",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "withdrawAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "baseCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "baseDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeCollateral",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "memeDebtScaled",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
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
pub mod EventEmitter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052348015600e575f5ffd5b50604051610e82380380610e82833981016040819052602b91603b565b6001600160a01b03166080526066565b5f60208284031215604a575f5ffd5b81516001600160a01b0381168114605f575f5ffd5b9392505050565b608051610dfe6100845f395f818160f501526107db0152610dfe5ff3fe608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c806389cad4d91161006e57806389cad4d91461015957806394e0dd1d1461016c57806396de247f1461017f5780639c845792146101925780639ed486eb146101a5578063ea34a577146101b8575f5ffd5b806315f762d5146100b5578063292ae722146100ca578063324ac765146100dd5780634a4a7b04146100f05780636a6d992f146101335780637c24dab714610146575b5f5ffd5b6100c86100c3366004610895565b6101cb565b005b6100c86100d8366004610912565b61029b565b6100c86100eb36600461097f565b61031d565b6101177f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100c8610141366004610a45565b610387565b6100c8610154366004610aca565b61043b565b6100c8610167366004610b08565b6104a7565b6100c861017a366004610b74565b610522565b6100c861018d366004610bc5565b610594565b6100c86101a0366004610c33565b610635565b6100c86101b3366004610c69565b610699565b6100c86101c6366004610ccb565b610711565b61021b6040516020016101dd90610d3a565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b8152506107bf565b604080516001600160a01b038a81168252898116602083015291810188905286821660608201526080810186905260a0810185905260c0810184905260e08101839052908a16907fb50b96cf5652f7259ba33891436a3874ecce39e522a18fb973cb021949f573df906101000160405180910390a2505050505050505050565b6102ad6040516020016101dd90610d3a565b604080516001600160a01b038881168252878116602083015291810186905284821660608201526080810184905260a08101839052908816907fe10a339dd5329df14a8ec13eb4115b75ab032fd40e2ff2594a4a5bf80e497a419060c0015b60405180910390a250505050505050565b61032f6040516020016101dd90610d3a565b60408051848152602081018490529081018290526001600160a01b0380861691908716907fb149470201600d81e1698a9d001d20f8d2a036d2c0f425436b65a154e95eb53a9060600160405180910390a35050505050565b6103996040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038e811682528d8116978201979097529384018b905260ff8a16918401919091526080830188905260a083019590955260c082019290925260e08101919091526101008101929092528816907f4fc3cf57f1c587a9f0808812dd72668a2b82f54411b0737d06c4043ff044cf9a90610120015b60405180910390a25050505050505050565b61044d6040516020016101dd90610d3a565b6040805185815260208101859052908101839052606081018290526001600160a01b038616907fc320a8529b15b851aaa68519919ac344e1caceaf4f47d15df6e58181dfec63199060800160405180910390a25050505050565b6104b96040516020016101dd90610d3a565b604080516001600160a01b0389811682528881166020830152918101879052606081018690526080810185905260a0810184905260c08101839052908916907f259af91af89c9a6b13d53607d57f43b151235f69d54d2339133e57cfb62bf4c59060e001610429565b6105346040516020016101dd90610d3a565b6040805185815260208101859052908101839052606081018290526001600160a01b0380871691908816907fc70fd1eb40d7ae44a5425c09dece5a023b47277356e005b6caef1600cfc7334e9060800160405180910390a3505050505050565b6105a66040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038e811682528d8116978201979097529384018b90529083018990526080830188905260a083019590955260c082019290925260e08101919091526101008101929092528816907f533855cb8a87f0995f6c3390665eb37da822f5550b13a973a77906d018a2a89c906101200161030c565b6106476040516020016101dd90610d3a565b60408051848152602081018490529081018290526001600160a01b038516907fd2e6085315c6e1c1c7406a47c7d006a8c1f931396d868c16046dea71365ff0319060600160405180910390a250505050565b6106ab6040516020016101dd90610d3a565b604080516001600160a01b03878116825286811660208301528581168284015260608201859052608082018490529151918816917fcbdb4dd8f84983be7c158013b8d74dacf114333078949f99dfb5b66e6621964a9181900360a00190a2505050505050565b6107236040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038f811682528e8116978201979097529384018c905260ff8b16918401919091526080830189905260a0830188905260c083019590955260e08201929092526101008101919091526101208101929092528916907f2ef1e8737d096826c9abef1201bb205ea380555780a54bd904cf67ca7dba8c5f9061014001610429565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610828573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061084c9190610d5e565b61087657338160405163a35b150b60e01b815260040161086d929190610d84565b60405180910390fd5b5050565b80356001600160a01b0381168114610890575f5ffd5b919050565b5f5f5f5f5f5f5f5f5f6101208a8c0312156108ae575f5ffd5b6108b78a61087a565b98506108c560208b0161087a565b97506108d360408b0161087a565b965060608a013595506108e860808b0161087a565b989b979a50959894979660a0860135965060c08601359560e0810135955061010001359350915050565b5f5f5f5f5f5f5f60e0888a031215610928575f5ffd5b6109318861087a565b965061093f6020890161087a565b955061094d6040890161087a565b9450606088013593506109626080890161087a565b9699959850939692959460a0840135945060c09093013592915050565b5f5f5f5f5f60a08688031215610993575f5ffd5b61099c8661087a565b94506109aa6020870161087a565b94979496505050506040830135926060810135926080909101359150565b803560ff81168114610890575f5ffd5b5f608082840312156109e8575f5ffd5b6040516080810181811067ffffffffffffffff82111715610a1757634e487b7160e01b5f52604160045260245ffd5b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f5f5f5f5f5f5f5f610160898b031215610a5d575f5ffd5b610a668961087a565b9750610a7460208a0161087a565b9650610a8260408a0161087a565b9550610a9060608a0161087a565b945060808901359350610aa560a08a016109c8565b925060c08901359150610abb8a60e08b016109d8565b90509295985092959890939650565b5f5f5f5f5f60a08688031215610ade575f5ffd5b610ae78661087a565b97602087013597506040870135966060810135965060800135945092505050565b5f5f5f5f5f5f5f5f610100898b031215610b20575f5ffd5b610b298961087a565b9750610b3760208a0161087a565b9650610b4560408a0161087a565b979a96995096976060810135975060808101359660a0820135965060c0820135955060e0909101359350915050565b5f5f5f5f5f5f60c08789031215610b89575f5ffd5b610b928761087a565b9550610ba06020880161087a565b95989597505050506040840135936060810135936080820135935060a0909101359150565b5f5f5f5f5f5f5f610140888a031215610bdc575f5ffd5b610be58861087a565b9650610bf36020890161087a565b9550610c016040890161087a565b9450606088013593506080880135925060a08801359150610c258960c08a016109d8565b905092959891949750929550565b5f5f5f5f60808587031215610c46575f5ffd5b610c4f8561087a565b966020860135965060408601359560600135945092505050565b5f5f5f5f5f5f60c08789031215610c7e575f5ffd5b610c878761087a565b9550610c956020880161087a565b9450610ca36040880161087a565b9350610cb16060880161087a565b9598949750929560808101359460a0909101359350915050565b5f5f5f5f5f5f5f5f610160898b031215610ce3575f5ffd5b610cec8961087a565b9750610cfa60208a0161087a565b9650610d0860408a0161087a565b955060608901359450610d1d60808a016109c8565b935060a0890135925060c08901359150610abb8a60e08b016109d8565b6020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f60208284031215610d6e575f5ffd5b81518015158114610d7d575f5ffd5b9392505050565b60018060a01b0383168152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f830116840101915050939250505056fea26469706673582212201014eb0b513259081078d1b3ef92cf88dbd384ec3b5197559d0efeb72b9747b264736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15`\x0EW__\xFD[P`@Qa\x0E\x828\x03\x80a\x0E\x82\x839\x81\x01`@\x81\x90R`+\x91`;V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`fV[_` \x82\x84\x03\x12\x15`JW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`_W__\xFD[\x93\x92PPPV[`\x80Qa\r\xFEa\0\x84_9_\x81\x81`\xF5\x01Ra\x07\xDB\x01Ra\r\xFE_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x89\xCA\xD4\xD9\x11a\0nW\x80c\x89\xCA\xD4\xD9\x14a\x01YW\x80c\x94\xE0\xDD\x1D\x14a\x01lW\x80c\x96\xDE$\x7F\x14a\x01\x7FW\x80c\x9C\x84W\x92\x14a\x01\x92W\x80c\x9E\xD4\x86\xEB\x14a\x01\xA5W\x80c\xEA4\xA5w\x14a\x01\xB8W__\xFD[\x80c\x15\xF7b\xD5\x14a\0\xB5W\x80c)*\xE7\"\x14a\0\xCAW\x80c2J\xC7e\x14a\0\xDDW\x80cJJ{\x04\x14a\0\xF0W\x80cjm\x99/\x14a\x013W\x80c|$\xDA\xB7\x14a\x01FW[__\xFD[a\0\xC8a\0\xC36`\x04a\x08\x95V[a\x01\xCBV[\0[a\0\xC8a\0\xD86`\x04a\t\x12V[a\x02\x9BV[a\0\xC8a\0\xEB6`\x04a\t\x7FV[a\x03\x1DV[a\x01\x17\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xC8a\x01A6`\x04a\nEV[a\x03\x87V[a\0\xC8a\x01T6`\x04a\n\xCAV[a\x04;V[a\0\xC8a\x01g6`\x04a\x0B\x08V[a\x04\xA7V[a\0\xC8a\x01z6`\x04a\x0BtV[a\x05\"V[a\0\xC8a\x01\x8D6`\x04a\x0B\xC5V[a\x05\x94V[a\0\xC8a\x01\xA06`\x04a\x0C3V[a\x065V[a\0\xC8a\x01\xB36`\x04a\x0CiV[a\x06\x99V[a\0\xC8a\x01\xC66`\x04a\x0C\xCBV[a\x07\x11V[a\x02\x1B`@Q` \x01a\x01\xDD\x90a\r:V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x07\xBFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x82R\x89\x81\x16` \x83\x01R\x91\x81\x01\x88\x90R\x86\x82\x16``\x82\x01R`\x80\x81\x01\x86\x90R`\xA0\x81\x01\x85\x90R`\xC0\x81\x01\x84\x90R`\xE0\x81\x01\x83\x90R\x90\x8A\x16\x90\x7F\xB5\x0B\x96\xCFVR\xF7%\x9B\xA38\x91Cj8t\xEC\xCE9\xE5\"\xA1\x8F\xB9s\xCB\x02\x19I\xF5s\xDF\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[a\x02\xAD`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R\x87\x81\x16` \x83\x01R\x91\x81\x01\x86\x90R\x84\x82\x16``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x83\x90R\x90\x88\x16\x90\x7F\xE1\n3\x9D\xD52\x9D\xF1J\x8E\xC1>\xB4\x11[u\xAB\x03/\xD4\x0E/\xF2YJJ[\xF8\x0EIzA\x90`\xC0\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x03/`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7F\xB1IG\x02\x01`\r\x81\xE1i\x8A\x9D\0\x1D \xF8\xD2\xA06\xD2\xC0\xF4%Cke\xA1T\xE9^\xB5:\x90``\x01`@Q\x80\x91\x03\x90\xA3PPPPPV[a\x03\x99`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16\x82R\x8D\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8B\x90R`\xFF\x8A\x16\x91\x84\x01\x91\x90\x91R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x95\x90\x95R`\xC0\x82\x01\x92\x90\x92R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x81\x01\x92\x90\x92R\x88\x16\x90\x7FO\xC3\xCFW\xF1\xC5\x87\xA9\xF0\x80\x88\x12\xDDrf\x8A+\x82\xF5D\x11\xB0s}\x06\xC4\x04?\xF0D\xCF\x9A\x90a\x01 \x01[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[a\x04M`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\xC3 \xA8R\x9B\x15\xB8Q\xAA\xA6\x85\x19\x91\x9A\xC3D\xE1\xCA\xCE\xAFOG\xD1]\xF6\xE5\x81\x81\xDF\xECc\x19\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x04\xB9`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R\x88\x81\x16` \x83\x01R\x91\x81\x01\x87\x90R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x83\x90R\x90\x89\x16\x90\x7F%\x9A\xF9\x1A\xF8\x9C\x9Ak\x13\xD56\x07\xD5\x7FC\xB1Q#_i\xD5M#9\x13>W\xCF\xB6+\xF4\xC5\x90`\xE0\x01a\x04)V[a\x054`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\xC7\x0F\xD1\xEB@\xD7\xAED\xA5B\\\t\xDE\xCEZ\x02;G'sV\xE0\x05\xB6\xCA\xEF\x16\0\xCF\xC73N\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x05\xA6`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16\x82R\x8D\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8B\x90R\x90\x83\x01\x89\x90R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x95\x90\x95R`\xC0\x82\x01\x92\x90\x92R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x81\x01\x92\x90\x92R\x88\x16\x90\x7FS8U\xCB\x8A\x87\xF0\x99_l3\x90f^\xB3}\xA8\"\xF5U\x0B\x13\xA9s\xA7y\x06\xD0\x18\xA2\xA8\x9C\x90a\x01 \x01a\x03\x0CV[a\x06G`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD2\xE6\x08S\x15\xC6\xE1\xC1\xC7@jG\xC7\xD0\x06\xA8\xC1\xF919m\x86\x8C\x16\x04m\xEAq6_\xF01\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPPV[a\x06\xAB`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R\x85\x81\x16\x82\x84\x01R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R\x91Q\x91\x88\x16\x91\x7F\xCB\xDBM\xD8\xF8I\x83\xBE|\x15\x80\x13\xB8\xD7M\xAC\xF1\x1430x\x94\x9F\x99\xDF\xB5\xB6nf!\x96J\x91\x81\x90\x03`\xA0\x01\x90\xA2PPPPPPV[a\x07#`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16\x82R\x8E\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8C\x90R`\xFF\x8B\x16\x91\x84\x01\x91\x90\x91R`\x80\x83\x01\x89\x90R`\xA0\x83\x01\x88\x90R`\xC0\x83\x01\x95\x90\x95R`\xE0\x82\x01\x92\x90\x92Ra\x01\0\x81\x01\x91\x90\x91Ra\x01 \x81\x01\x92\x90\x92R\x89\x16\x90\x7F.\xF1\xE8s}\th&\xC9\xAB\xEF\x12\x01\xBB ^\xA3\x80UW\x80\xA5K\xD9\x04\xCFg\xCA}\xBA\x8C_\x90a\x01@\x01a\x04)V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08L\x91\x90a\r^V[a\x08vW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x08m\x92\x91\x90a\r\x84V[`@Q\x80\x91\x03\x90\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x90W__\xFD[\x91\x90PV[_________a\x01 \x8A\x8C\x03\x12\x15a\x08\xAEW__\xFD[a\x08\xB7\x8Aa\x08zV[\x98Pa\x08\xC5` \x8B\x01a\x08zV[\x97Pa\x08\xD3`@\x8B\x01a\x08zV[\x96P``\x8A\x015\x95Pa\x08\xE8`\x80\x8B\x01a\x08zV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x81\x015\x95Pa\x01\0\x015\x93P\x91PPV[_______`\xE0\x88\x8A\x03\x12\x15a\t(W__\xFD[a\t1\x88a\x08zV[\x96Pa\t?` \x89\x01a\x08zV[\x95Pa\tM`@\x89\x01a\x08zV[\x94P``\x88\x015\x93Pa\tb`\x80\x89\x01a\x08zV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a\t\x93W__\xFD[a\t\x9C\x86a\x08zV[\x94Pa\t\xAA` \x87\x01a\x08zV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14a\x08\x90W__\xFD[_`\x80\x82\x84\x03\x12\x15a\t\xE8W__\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\n\x17WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[________a\x01`\x89\x8B\x03\x12\x15a\n]W__\xFD[a\nf\x89a\x08zV[\x97Pa\nt` \x8A\x01a\x08zV[\x96Pa\n\x82`@\x8A\x01a\x08zV[\x95Pa\n\x90``\x8A\x01a\x08zV[\x94P`\x80\x89\x015\x93Pa\n\xA5`\xA0\x8A\x01a\t\xC8V[\x92P`\xC0\x89\x015\x91Pa\n\xBB\x8A`\xE0\x8B\x01a\t\xD8V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_____`\xA0\x86\x88\x03\x12\x15a\n\xDEW__\xFD[a\n\xE7\x86a\x08zV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x015\x94P\x92PPPV[________a\x01\0\x89\x8B\x03\x12\x15a\x0B W__\xFD[a\x0B)\x89a\x08zV[\x97Pa\x0B7` \x8A\x01a\x08zV[\x96Pa\x0BE`@\x8A\x01a\x08zV[\x97\x9A\x96\x99P\x96\x97``\x81\x015\x97P`\x80\x81\x015\x96`\xA0\x82\x015\x96P`\xC0\x82\x015\x95P`\xE0\x90\x91\x015\x93P\x91PPV[______`\xC0\x87\x89\x03\x12\x15a\x0B\x89W__\xFD[a\x0B\x92\x87a\x08zV[\x95Pa\x0B\xA0` \x88\x01a\x08zV[\x95\x98\x95\x97PPPP`@\x84\x015\x93``\x81\x015\x93`\x80\x82\x015\x93P`\xA0\x90\x91\x015\x91PV[_______a\x01@\x88\x8A\x03\x12\x15a\x0B\xDCW__\xFD[a\x0B\xE5\x88a\x08zV[\x96Pa\x0B\xF3` \x89\x01a\x08zV[\x95Pa\x0C\x01`@\x89\x01a\x08zV[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91Pa\x0C%\x89`\xC0\x8A\x01a\t\xD8V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[____`\x80\x85\x87\x03\x12\x15a\x0CFW__\xFD[a\x0CO\x85a\x08zV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[______`\xC0\x87\x89\x03\x12\x15a\x0C~W__\xFD[a\x0C\x87\x87a\x08zV[\x95Pa\x0C\x95` \x88\x01a\x08zV[\x94Pa\x0C\xA3`@\x88\x01a\x08zV[\x93Pa\x0C\xB1``\x88\x01a\x08zV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[________a\x01`\x89\x8B\x03\x12\x15a\x0C\xE3W__\xFD[a\x0C\xEC\x89a\x08zV[\x97Pa\x0C\xFA` \x8A\x01a\x08zV[\x96Pa\r\x08`@\x8A\x01a\x08zV[\x95P``\x89\x015\x94Pa\r\x1D`\x80\x8A\x01a\t\xC8V[\x93P`\xA0\x89\x015\x92P`\xC0\x89\x015\x91Pa\n\xBB\x8A`\xE0\x8B\x01a\t\xD8V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\rnW__\xFD[\x81Q\x80\x15\x15\x81\x14a\r}W__\xFD[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x10\x14\xEB\x0BQ2Y\x08\x10x\xD1\xB3\xEF\x92\xCF\x88\xDB\xD3\x84\xEC;Q\x97U\x9D\x0E\xFE\xB7+\x97G\xB2dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c806389cad4d91161006e57806389cad4d91461015957806394e0dd1d1461016c57806396de247f1461017f5780639c845792146101925780639ed486eb146101a5578063ea34a577146101b8575f5ffd5b806315f762d5146100b5578063292ae722146100ca578063324ac765146100dd5780634a4a7b04146100f05780636a6d992f146101335780637c24dab714610146575b5f5ffd5b6100c86100c3366004610895565b6101cb565b005b6100c86100d8366004610912565b61029b565b6100c86100eb36600461097f565b61031d565b6101177f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100c8610141366004610a45565b610387565b6100c8610154366004610aca565b61043b565b6100c8610167366004610b08565b6104a7565b6100c861017a366004610b74565b610522565b6100c861018d366004610bc5565b610594565b6100c86101a0366004610c33565b610635565b6100c86101b3366004610c69565b610699565b6100c86101c6366004610ccb565b610711565b61021b6040516020016101dd90610d3a565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b8152506107bf565b604080516001600160a01b038a81168252898116602083015291810188905286821660608201526080810186905260a0810185905260c0810184905260e08101839052908a16907fb50b96cf5652f7259ba33891436a3874ecce39e522a18fb973cb021949f573df906101000160405180910390a2505050505050505050565b6102ad6040516020016101dd90610d3a565b604080516001600160a01b038881168252878116602083015291810186905284821660608201526080810184905260a08101839052908816907fe10a339dd5329df14a8ec13eb4115b75ab032fd40e2ff2594a4a5bf80e497a419060c0015b60405180910390a250505050505050565b61032f6040516020016101dd90610d3a565b60408051848152602081018490529081018290526001600160a01b0380861691908716907fb149470201600d81e1698a9d001d20f8d2a036d2c0f425436b65a154e95eb53a9060600160405180910390a35050505050565b6103996040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038e811682528d8116978201979097529384018b905260ff8a16918401919091526080830188905260a083019590955260c082019290925260e08101919091526101008101929092528816907f4fc3cf57f1c587a9f0808812dd72668a2b82f54411b0737d06c4043ff044cf9a90610120015b60405180910390a25050505050505050565b61044d6040516020016101dd90610d3a565b6040805185815260208101859052908101839052606081018290526001600160a01b038616907fc320a8529b15b851aaa68519919ac344e1caceaf4f47d15df6e58181dfec63199060800160405180910390a25050505050565b6104b96040516020016101dd90610d3a565b604080516001600160a01b0389811682528881166020830152918101879052606081018690526080810185905260a0810184905260c08101839052908916907f259af91af89c9a6b13d53607d57f43b151235f69d54d2339133e57cfb62bf4c59060e001610429565b6105346040516020016101dd90610d3a565b6040805185815260208101859052908101839052606081018290526001600160a01b0380871691908816907fc70fd1eb40d7ae44a5425c09dece5a023b47277356e005b6caef1600cfc7334e9060800160405180910390a3505050505050565b6105a66040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038e811682528d8116978201979097529384018b90529083018990526080830188905260a083019590955260c082019290925260e08101919091526101008101929092528816907f533855cb8a87f0995f6c3390665eb37da822f5550b13a973a77906d018a2a89c906101200161030c565b6106476040516020016101dd90610d3a565b60408051848152602081018490529081018290526001600160a01b038516907fd2e6085315c6e1c1c7406a47c7d006a8c1f931396d868c16046dea71365ff0319060600160405180910390a250505050565b6106ab6040516020016101dd90610d3a565b604080516001600160a01b03878116825286811660208301528581168284015260608201859052608082018490529151918816917fcbdb4dd8f84983be7c158013b8d74dacf114333078949f99dfb5b66e6621964a9181900360a00190a2505050505050565b6107236040516020016101dd90610d3a565b805160208083015160408085015160608087015183516001600160a01b038f811682528e8116978201979097529384018c905260ff8b16918401919091526080830189905260a0830188905260c083019590955260e08201929092526101008101919091526101208101929092528916907f2ef1e8737d096826c9abef1201bb205ea380555780a54bd904cf67ca7dba8c5f9061014001610429565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610828573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061084c9190610d5e565b61087657338160405163a35b150b60e01b815260040161086d929190610d84565b60405180910390fd5b5050565b80356001600160a01b0381168114610890575f5ffd5b919050565b5f5f5f5f5f5f5f5f5f6101208a8c0312156108ae575f5ffd5b6108b78a61087a565b98506108c560208b0161087a565b97506108d360408b0161087a565b965060608a013595506108e860808b0161087a565b989b979a50959894979660a0860135965060c08601359560e0810135955061010001359350915050565b5f5f5f5f5f5f5f60e0888a031215610928575f5ffd5b6109318861087a565b965061093f6020890161087a565b955061094d6040890161087a565b9450606088013593506109626080890161087a565b9699959850939692959460a0840135945060c09093013592915050565b5f5f5f5f5f60a08688031215610993575f5ffd5b61099c8661087a565b94506109aa6020870161087a565b94979496505050506040830135926060810135926080909101359150565b803560ff81168114610890575f5ffd5b5f608082840312156109e8575f5ffd5b6040516080810181811067ffffffffffffffff82111715610a1757634e487b7160e01b5f52604160045260245ffd5b6040908152833582526020808501359083015283810135908201526060928301359281019290925250919050565b5f5f5f5f5f5f5f5f610160898b031215610a5d575f5ffd5b610a668961087a565b9750610a7460208a0161087a565b9650610a8260408a0161087a565b9550610a9060608a0161087a565b945060808901359350610aa560a08a016109c8565b925060c08901359150610abb8a60e08b016109d8565b90509295985092959890939650565b5f5f5f5f5f60a08688031215610ade575f5ffd5b610ae78661087a565b97602087013597506040870135966060810135965060800135945092505050565b5f5f5f5f5f5f5f5f610100898b031215610b20575f5ffd5b610b298961087a565b9750610b3760208a0161087a565b9650610b4560408a0161087a565b979a96995096976060810135975060808101359660a0820135965060c0820135955060e0909101359350915050565b5f5f5f5f5f5f60c08789031215610b89575f5ffd5b610b928761087a565b9550610ba06020880161087a565b95989597505050506040840135936060810135936080820135935060a0909101359150565b5f5f5f5f5f5f5f610140888a031215610bdc575f5ffd5b610be58861087a565b9650610bf36020890161087a565b9550610c016040890161087a565b9450606088013593506080880135925060a08801359150610c258960c08a016109d8565b905092959891949750929550565b5f5f5f5f60808587031215610c46575f5ffd5b610c4f8561087a565b966020860135965060408601359560600135945092505050565b5f5f5f5f5f5f60c08789031215610c7e575f5ffd5b610c878761087a565b9550610c956020880161087a565b9450610ca36040880161087a565b9350610cb16060880161087a565b9598949750929560808101359460a0909101359350915050565b5f5f5f5f5f5f5f5f610160898b031215610ce3575f5ffd5b610cec8961087a565b9750610cfa60208a0161087a565b9650610d0860408a0161087a565b955060608901359450610d1d60808a016109c8565b935060a0890135925060c08901359150610abb8a60e08b016109d8565b6020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b5f60208284031215610d6e575f5ffd5b81518015158114610d7d575f5ffd5b9392505050565b60018060a01b0383168152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f830116840101915050939250505056fea26469706673582212201014eb0b513259081078d1b3ef92cf88dbd384ec3b5197559d0efeb72b9747b264736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x89\xCA\xD4\xD9\x11a\0nW\x80c\x89\xCA\xD4\xD9\x14a\x01YW\x80c\x94\xE0\xDD\x1D\x14a\x01lW\x80c\x96\xDE$\x7F\x14a\x01\x7FW\x80c\x9C\x84W\x92\x14a\x01\x92W\x80c\x9E\xD4\x86\xEB\x14a\x01\xA5W\x80c\xEA4\xA5w\x14a\x01\xB8W__\xFD[\x80c\x15\xF7b\xD5\x14a\0\xB5W\x80c)*\xE7\"\x14a\0\xCAW\x80c2J\xC7e\x14a\0\xDDW\x80cJJ{\x04\x14a\0\xF0W\x80cjm\x99/\x14a\x013W\x80c|$\xDA\xB7\x14a\x01FW[__\xFD[a\0\xC8a\0\xC36`\x04a\x08\x95V[a\x01\xCBV[\0[a\0\xC8a\0\xD86`\x04a\t\x12V[a\x02\x9BV[a\0\xC8a\0\xEB6`\x04a\t\x7FV[a\x03\x1DV[a\x01\x17\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xC8a\x01A6`\x04a\nEV[a\x03\x87V[a\0\xC8a\x01T6`\x04a\n\xCAV[a\x04;V[a\0\xC8a\x01g6`\x04a\x0B\x08V[a\x04\xA7V[a\0\xC8a\x01z6`\x04a\x0BtV[a\x05\"V[a\0\xC8a\x01\x8D6`\x04a\x0B\xC5V[a\x05\x94V[a\0\xC8a\x01\xA06`\x04a\x0C3V[a\x065V[a\0\xC8a\x01\xB36`\x04a\x0CiV[a\x06\x99V[a\0\xC8a\x01\xC66`\x04a\x0C\xCBV[a\x07\x11V[a\x02\x1B`@Q` \x01a\x01\xDD\x90a\r:V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x07\xBFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x82R\x89\x81\x16` \x83\x01R\x91\x81\x01\x88\x90R\x86\x82\x16``\x82\x01R`\x80\x81\x01\x86\x90R`\xA0\x81\x01\x85\x90R`\xC0\x81\x01\x84\x90R`\xE0\x81\x01\x83\x90R\x90\x8A\x16\x90\x7F\xB5\x0B\x96\xCFVR\xF7%\x9B\xA38\x91Cj8t\xEC\xCE9\xE5\"\xA1\x8F\xB9s\xCB\x02\x19I\xF5s\xDF\x90a\x01\0\x01`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[a\x02\xAD`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R\x87\x81\x16` \x83\x01R\x91\x81\x01\x86\x90R\x84\x82\x16``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x83\x90R\x90\x88\x16\x90\x7F\xE1\n3\x9D\xD52\x9D\xF1J\x8E\xC1>\xB4\x11[u\xAB\x03/\xD4\x0E/\xF2YJJ[\xF8\x0EIzA\x90`\xC0\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[a\x03/`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x90\x87\x16\x90\x7F\xB1IG\x02\x01`\r\x81\xE1i\x8A\x9D\0\x1D \xF8\xD2\xA06\xD2\xC0\xF4%Cke\xA1T\xE9^\xB5:\x90``\x01`@Q\x80\x91\x03\x90\xA3PPPPPV[a\x03\x99`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16\x82R\x8D\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8B\x90R`\xFF\x8A\x16\x91\x84\x01\x91\x90\x91R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x95\x90\x95R`\xC0\x82\x01\x92\x90\x92R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x81\x01\x92\x90\x92R\x88\x16\x90\x7FO\xC3\xCFW\xF1\xC5\x87\xA9\xF0\x80\x88\x12\xDDrf\x8A+\x82\xF5D\x11\xB0s}\x06\xC4\x04?\xF0D\xCF\x9A\x90a\x01 \x01[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[a\x04M`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x7F\xC3 \xA8R\x9B\x15\xB8Q\xAA\xA6\x85\x19\x91\x9A\xC3D\xE1\xCA\xCE\xAFOG\xD1]\xF6\xE5\x81\x81\xDF\xECc\x19\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x04\xB9`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R\x88\x81\x16` \x83\x01R\x91\x81\x01\x87\x90R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x83\x90R\x90\x89\x16\x90\x7F%\x9A\xF9\x1A\xF8\x9C\x9Ak\x13\xD56\x07\xD5\x7FC\xB1Q#_i\xD5M#9\x13>W\xCF\xB6+\xF4\xC5\x90`\xE0\x01a\x04)V[a\x054`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x91\x90\x88\x16\x90\x7F\xC7\x0F\xD1\xEB@\xD7\xAED\xA5B\\\t\xDE\xCEZ\x02;G'sV\xE0\x05\xB6\xCA\xEF\x16\0\xCF\xC73N\x90`\x80\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x05\xA6`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16\x82R\x8D\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8B\x90R\x90\x83\x01\x89\x90R`\x80\x83\x01\x88\x90R`\xA0\x83\x01\x95\x90\x95R`\xC0\x82\x01\x92\x90\x92R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x81\x01\x92\x90\x92R\x88\x16\x90\x7FS8U\xCB\x8A\x87\xF0\x99_l3\x90f^\xB3}\xA8\"\xF5U\x0B\x13\xA9s\xA7y\x06\xD0\x18\xA2\xA8\x9C\x90a\x01 \x01a\x03\x0CV[a\x06G`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xD2\xE6\x08S\x15\xC6\xE1\xC1\xC7@jG\xC7\xD0\x06\xA8\xC1\xF919m\x86\x8C\x16\x04m\xEAq6_\xF01\x90``\x01`@Q\x80\x91\x03\x90\xA2PPPPV[a\x06\xAB`@Q` \x01a\x01\xDD\x90a\r:V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R\x85\x81\x16\x82\x84\x01R``\x82\x01\x85\x90R`\x80\x82\x01\x84\x90R\x91Q\x91\x88\x16\x91\x7F\xCB\xDBM\xD8\xF8I\x83\xBE|\x15\x80\x13\xB8\xD7M\xAC\xF1\x1430x\x94\x9F\x99\xDF\xB5\xB6nf!\x96J\x91\x81\x90\x03`\xA0\x01\x90\xA2PPPPPPV[a\x07#`@Q` \x01a\x01\xDD\x90a\r:V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x80\x87\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16\x82R\x8E\x81\x16\x97\x82\x01\x97\x90\x97R\x93\x84\x01\x8C\x90R`\xFF\x8B\x16\x91\x84\x01\x91\x90\x91R`\x80\x83\x01\x89\x90R`\xA0\x83\x01\x88\x90R`\xC0\x83\x01\x95\x90\x95R`\xE0\x82\x01\x92\x90\x92Ra\x01\0\x81\x01\x91\x90\x91Ra\x01 \x81\x01\x92\x90\x92R\x89\x16\x90\x7F.\xF1\xE8s}\th&\xC9\xAB\xEF\x12\x01\xBB ^\xA3\x80UW\x80\xA5K\xD9\x04\xCFg\xCA}\xBA\x8C_\x90a\x01@\x01a\x04)V[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08L\x91\x90a\r^V[a\x08vW3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x08m\x92\x91\x90a\r\x84V[`@Q\x80\x91\x03\x90\xFD[PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x90W__\xFD[\x91\x90PV[_________a\x01 \x8A\x8C\x03\x12\x15a\x08\xAEW__\xFD[a\x08\xB7\x8Aa\x08zV[\x98Pa\x08\xC5` \x8B\x01a\x08zV[\x97Pa\x08\xD3`@\x8B\x01a\x08zV[\x96P``\x8A\x015\x95Pa\x08\xE8`\x80\x8B\x01a\x08zV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x81\x015\x95Pa\x01\0\x015\x93P\x91PPV[_______`\xE0\x88\x8A\x03\x12\x15a\t(W__\xFD[a\t1\x88a\x08zV[\x96Pa\t?` \x89\x01a\x08zV[\x95Pa\tM`@\x89\x01a\x08zV[\x94P``\x88\x015\x93Pa\tb`\x80\x89\x01a\x08zV[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[_____`\xA0\x86\x88\x03\x12\x15a\t\x93W__\xFD[a\t\x9C\x86a\x08zV[\x94Pa\t\xAA` \x87\x01a\x08zV[\x94\x97\x94\x96PPPP`@\x83\x015\x92``\x81\x015\x92`\x80\x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14a\x08\x90W__\xFD[_`\x80\x82\x84\x03\x12\x15a\t\xE8W__\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\n\x17WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[________a\x01`\x89\x8B\x03\x12\x15a\n]W__\xFD[a\nf\x89a\x08zV[\x97Pa\nt` \x8A\x01a\x08zV[\x96Pa\n\x82`@\x8A\x01a\x08zV[\x95Pa\n\x90``\x8A\x01a\x08zV[\x94P`\x80\x89\x015\x93Pa\n\xA5`\xA0\x8A\x01a\t\xC8V[\x92P`\xC0\x89\x015\x91Pa\n\xBB\x8A`\xE0\x8B\x01a\t\xD8V[\x90P\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_____`\xA0\x86\x88\x03\x12\x15a\n\xDEW__\xFD[a\n\xE7\x86a\x08zV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x81\x015\x96P`\x80\x015\x94P\x92PPPV[________a\x01\0\x89\x8B\x03\x12\x15a\x0B W__\xFD[a\x0B)\x89a\x08zV[\x97Pa\x0B7` \x8A\x01a\x08zV[\x96Pa\x0BE`@\x8A\x01a\x08zV[\x97\x9A\x96\x99P\x96\x97``\x81\x015\x97P`\x80\x81\x015\x96`\xA0\x82\x015\x96P`\xC0\x82\x015\x95P`\xE0\x90\x91\x015\x93P\x91PPV[______`\xC0\x87\x89\x03\x12\x15a\x0B\x89W__\xFD[a\x0B\x92\x87a\x08zV[\x95Pa\x0B\xA0` \x88\x01a\x08zV[\x95\x98\x95\x97PPPP`@\x84\x015\x93``\x81\x015\x93`\x80\x82\x015\x93P`\xA0\x90\x91\x015\x91PV[_______a\x01@\x88\x8A\x03\x12\x15a\x0B\xDCW__\xFD[a\x0B\xE5\x88a\x08zV[\x96Pa\x0B\xF3` \x89\x01a\x08zV[\x95Pa\x0C\x01`@\x89\x01a\x08zV[\x94P``\x88\x015\x93P`\x80\x88\x015\x92P`\xA0\x88\x015\x91Pa\x0C%\x89`\xC0\x8A\x01a\t\xD8V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[____`\x80\x85\x87\x03\x12\x15a\x0CFW__\xFD[a\x0CO\x85a\x08zV[\x96` \x86\x015\x96P`@\x86\x015\x95``\x015\x94P\x92PPPV[______`\xC0\x87\x89\x03\x12\x15a\x0C~W__\xFD[a\x0C\x87\x87a\x08zV[\x95Pa\x0C\x95` \x88\x01a\x08zV[\x94Pa\x0C\xA3`@\x88\x01a\x08zV[\x93Pa\x0C\xB1``\x88\x01a\x08zV[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[________a\x01`\x89\x8B\x03\x12\x15a\x0C\xE3W__\xFD[a\x0C\xEC\x89a\x08zV[\x97Pa\x0C\xFA` \x8A\x01a\x08zV[\x96Pa\r\x08`@\x8A\x01a\x08zV[\x95P``\x89\x015\x94Pa\r\x1D`\x80\x8A\x01a\t\xC8V[\x93P`\xA0\x89\x015\x92P`\xC0\x89\x015\x91Pa\n\xBB\x8A`\xE0\x8B\x01a\t\xD8V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\rnW__\xFD[\x81Q\x80\x15\x15\x81\x14a\r}W__\xFD[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x10\x14\xEB\x0BQ2Y\x08\x10x\xD1\xB3\xEF\x92\xCF\x88\xDB\xD3\x84\xEC;Q\x97U\x9D\x0E\xFE\xB7+\x97G\xB2dsolcC\0\x08\x1C\x003",
    );
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
    /**Event with signature `Add(address,address,address,address,uint256,uint256)` and selector `0xcbdb4dd8f84983be7c158013b8d74dacf114333078949f99dfb5b66e6621964a`.
```solidity
event Add(address indexed adder, address baseToken, address memeToken, address to, uint256 baseAmount, uint256 memeAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Add {
        #[allow(missing_docs)]
        pub adder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Add {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Add(address,address,address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                203u8,
                219u8,
                77u8,
                216u8,
                248u8,
                73u8,
                131u8,
                190u8,
                124u8,
                21u8,
                128u8,
                19u8,
                184u8,
                215u8,
                77u8,
                172u8,
                241u8,
                20u8,
                51u8,
                48u8,
                120u8,
                148u8,
                159u8,
                153u8,
                223u8,
                181u8,
                182u8,
                110u8,
                102u8,
                33u8,
                150u8,
                74u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    adder: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    to: data.2,
                    baseAmount: data.3,
                    memeAmount: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
                    > as alloy_sol_types::SolType>::tokenize(&self.baseAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.adder.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.adder,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Add {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Add> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Add) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Borrow(address,address,address,uint256,uint8,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x2ef1e8737d096826c9abef1201bb205ea380555780a54bd904cf67ca7dba8c5f`.
```solidity
event Borrow(address indexed borrower, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 borrowAmount, uint256 borrowRate, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Borrow {
        #[allow(missing_docs)]
        pub borrower: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub tokenIndex: u8,
        #[allow(missing_docs)]
        pub borrowAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Borrow {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Borrow(address,address,address,uint256,uint8,uint256,uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                46u8,
                241u8,
                232u8,
                115u8,
                125u8,
                9u8,
                104u8,
                38u8,
                201u8,
                171u8,
                239u8,
                18u8,
                1u8,
                187u8,
                32u8,
                94u8,
                163u8,
                128u8,
                85u8,
                87u8,
                128u8,
                165u8,
                75u8,
                217u8,
                4u8,
                207u8,
                103u8,
                202u8,
                125u8,
                186u8,
                140u8,
                95u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    borrower: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    positionId: data.2,
                    tokenIndex: data.3,
                    borrowAmount: data.4,
                    borrowRate: data.5,
                    baseCollateral: data.6,
                    baseDebtScaled: data.7,
                    memeCollateral: data.8,
                    memeDebtScaled: data.9,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.borrower.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.borrower,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Borrow {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Borrow> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Borrow) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ClaimFees(address,uint256,uint256,uint256)` and selector `0xd2e6085315c6e1c1c7406a47c7d006a8c1f931396d868c16046dea71365ff031`.
```solidity
event ClaimFees(address indexed pool, uint256 scaledUnclaimedFee, uint256 liquidityIndex, uint256 unclaimedFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ClaimFees {
        #[allow(missing_docs)]
        pub pool: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub scaledUnclaimedFee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ClaimFees {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ClaimFees(address,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                230u8,
                8u8,
                83u8,
                21u8,
                198u8,
                225u8,
                193u8,
                199u8,
                64u8,
                106u8,
                71u8,
                199u8,
                208u8,
                6u8,
                168u8,
                193u8,
                249u8,
                49u8,
                57u8,
                109u8,
                134u8,
                140u8,
                22u8,
                4u8,
                109u8,
                234u8,
                113u8,
                54u8,
                95u8,
                240u8,
                49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pool: topics.1,
                    scaledUnclaimedFee: data.0,
                    liquidityIndex: data.1,
                    unclaimedFee: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.pool.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.pool,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ClaimFees {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ClaimFees> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ClaimFees) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Close(address,address,uint256,uint256,uint256)` and selector `0xb149470201600d81e1698a9d001d20f8d2a036d2c0f425436b65a154e95eb53a`.
```solidity
event Close(address indexed poolUsd, address indexed account, uint256 amountUsdStartClose, uint256 amountUsdAfterRepayAndSellCollateral, uint256 amountUsdAfterBuyCollateralAndRepay);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Close {
        #[allow(missing_docs)]
        pub poolUsd: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountUsdStartClose: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountUsdAfterRepayAndSellCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountUsdAfterBuyCollateralAndRepay: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Close {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Close(address,address,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                177u8,
                73u8,
                71u8,
                2u8,
                1u8,
                96u8,
                13u8,
                129u8,
                225u8,
                105u8,
                138u8,
                157u8,
                0u8,
                29u8,
                32u8,
                248u8,
                210u8,
                160u8,
                54u8,
                210u8,
                192u8,
                244u8,
                37u8,
                67u8,
                107u8,
                101u8,
                161u8,
                84u8,
                233u8,
                94u8,
                181u8,
                58u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    poolUsd: topics.1,
                    account: topics.2,
                    amountUsdStartClose: data.0,
                    amountUsdAfterRepayAndSellCollateral: data.1,
                    amountUsdAfterBuyCollateralAndRepay: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountUsdStartClose),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.amountUsdAfterRepayAndSellCollateral,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.amountUsdAfterBuyCollateralAndRepay,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.poolUsd.clone(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.poolUsd,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Close {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Close> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Close) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Deposit(address,address,address,uint256,uint256,uint256,uint256,uint256)` and selector `0x259af91af89c9a6b13d53607d57f43b151235f69d54d2339133e57cfb62bf4c5`.
```solidity
event Deposit(address indexed depositor, address baseToken, address memeToken, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Deposit {
        #[allow(missing_docs)]
        pub depositor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub depositAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Deposit {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Deposit(address,address,address,uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                37u8,
                154u8,
                249u8,
                26u8,
                248u8,
                156u8,
                154u8,
                107u8,
                19u8,
                213u8,
                54u8,
                7u8,
                213u8,
                127u8,
                67u8,
                177u8,
                81u8,
                35u8,
                95u8,
                105u8,
                213u8,
                77u8,
                35u8,
                57u8,
                19u8,
                62u8,
                87u8,
                207u8,
                182u8,
                43u8,
                244u8,
                197u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    depositor: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    depositAmount: data.2,
                    baseCollateral: data.3,
                    baseDebtScaled: data.4,
                    memeCollateral: data.5,
                    memeDebtScaled: data.6,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.depositor.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.depositor,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Deposit {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Deposit> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Deposit) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Liquidation(address,address,uint256,uint256,uint256,uint256)` and selector `0xc70fd1eb40d7ae44a5425c09dece5a023b47277356e005b6caef1600cfc7334e`.
```solidity
event Liquidation(address indexed liquidator, address indexed account, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Liquidation {
        #[allow(missing_docs)]
        pub liquidator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub marginLevel: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub marginLevelLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalCollateralUsd: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalDebtUsd: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Liquidation {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Liquidation(address,address,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8,
                15u8,
                209u8,
                235u8,
                64u8,
                215u8,
                174u8,
                68u8,
                165u8,
                66u8,
                92u8,
                9u8,
                222u8,
                206u8,
                90u8,
                2u8,
                59u8,
                71u8,
                39u8,
                115u8,
                86u8,
                224u8,
                5u8,
                182u8,
                202u8,
                239u8,
                22u8,
                0u8,
                207u8,
                199u8,
                51u8,
                78u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    liquidator: topics.1,
                    account: topics.2,
                    marginLevel: data.0,
                    marginLevelLiquidationThreshold: data.1,
                    totalCollateralUsd: data.2,
                    totalDebtUsd: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.liquidator.clone(),
                    self.account.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.liquidator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Liquidation {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Liquidation> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Liquidation) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PoolUpdated(address,uint256,uint256,uint256,uint256)` and selector `0xc320a8529b15b851aaa68519919ac344e1caceaf4f47d15df6e58181dfec6319`.
```solidity
event PoolUpdated(address indexed pool, uint256 liquidityRate, uint256 borrowRate, uint256 liquidityIndex, uint256 borrowIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PoolUpdated {
        #[allow(missing_docs)]
        pub pool: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidityRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub borrowRate: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub liquidityIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PoolUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PoolUpdated(address,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                195u8,
                32u8,
                168u8,
                82u8,
                155u8,
                21u8,
                184u8,
                81u8,
                170u8,
                166u8,
                133u8,
                25u8,
                145u8,
                154u8,
                195u8,
                68u8,
                225u8,
                202u8,
                206u8,
                175u8,
                79u8,
                71u8,
                209u8,
                93u8,
                246u8,
                229u8,
                129u8,
                129u8,
                223u8,
                236u8,
                99u8,
                25u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pool: topics.1,
                    liquidityRate: data.0,
                    borrowRate: data.1,
                    liquidityIndex: data.2,
                    borrowIndex: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.pool.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.pool,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PoolUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PoolUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PoolUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Remove(address,address,address,uint256,address,uint256,uint256)` and selector `0xe10a339dd5329df14a8ec13eb4115b75ab032fd40e2ff2594a4a5bf80e497a41`.
```solidity
event Remove(address indexed remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 baseAmount, uint256 memeAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Remove {
        #[allow(missing_docs)]
        pub remover: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Remove {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Remove(address,address,address,uint256,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                10u8,
                51u8,
                157u8,
                213u8,
                50u8,
                157u8,
                241u8,
                74u8,
                142u8,
                193u8,
                62u8,
                180u8,
                17u8,
                91u8,
                117u8,
                171u8,
                3u8,
                47u8,
                212u8,
                14u8,
                47u8,
                242u8,
                89u8,
                74u8,
                74u8,
                91u8,
                248u8,
                14u8,
                73u8,
                122u8,
                65u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    remover: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    liquidity: data.2,
                    to: data.3,
                    baseAmount: data.4,
                    memeAmount: data.5,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
                    > as alloy_sol_types::SolType>::tokenize(&self.baseAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.remover.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.remover,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Remove {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Remove> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Remove) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Repay(address,address,address,uint256,uint8,uint256,uint256,uint256,uint256,uint256)` and selector `0x4fc3cf57f1c587a9f0808812dd72668a2b82f54411b0737d06c4043ff044cf9a`.
```solidity
event Repay(address indexed repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Repay {
        #[allow(missing_docs)]
        pub repayer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub tokenIndex: u8,
        #[allow(missing_docs)]
        pub repayAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Repay {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Repay(address,address,address,uint256,uint8,uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                79u8,
                195u8,
                207u8,
                87u8,
                241u8,
                197u8,
                135u8,
                169u8,
                240u8,
                128u8,
                136u8,
                18u8,
                221u8,
                114u8,
                102u8,
                138u8,
                43u8,
                130u8,
                245u8,
                68u8,
                17u8,
                176u8,
                115u8,
                125u8,
                6u8,
                196u8,
                4u8,
                63u8,
                240u8,
                68u8,
                207u8,
                154u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    repayer: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    positionId: data.2,
                    tokenIndex: data.3,
                    repayAmount: data.4,
                    baseCollateral: data.5,
                    baseDebtScaled: data.6,
                    memeCollateral: data.7,
                    memeDebtScaled: data.8,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.repayer.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.repayer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Repay {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Repay> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Repay) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Swap(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0x533855cb8a87f0995f6c3390665eb37da822f5550b13a973a77906d018a2a89c`.
```solidity
event Swap(address indexed account, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Swap {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenIn: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenOut: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amountOut: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Swap {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Swap(address,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                83u8,
                56u8,
                85u8,
                203u8,
                138u8,
                135u8,
                240u8,
                153u8,
                95u8,
                108u8,
                51u8,
                144u8,
                102u8,
                94u8,
                179u8,
                125u8,
                168u8,
                34u8,
                245u8,
                85u8,
                11u8,
                19u8,
                169u8,
                115u8,
                167u8,
                121u8,
                6u8,
                208u8,
                24u8,
                162u8,
                168u8,
                156u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    tokenIn: data.0,
                    tokenOut: data.1,
                    amountIn: data.2,
                    amountOut: data.3,
                    fee: data.4,
                    baseCollateral: data.5,
                    baseDebtScaled: data.6,
                    memeCollateral: data.7,
                    memeDebtScaled: data.8,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.tokenOut,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountIn),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountOut),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Swap {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Swap> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Swap) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Withdraw(address,address,address,uint256,address,uint256,uint256,uint256,uint256)` and selector `0xb50b96cf5652f7259ba33891436a3874ecce39e522a18fb973cb021949f573df`.
```solidity
event Withdraw(address indexed withdrawer, address baseToken, address memeToken, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Withdraw {
        #[allow(missing_docs)]
        pub withdrawer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub memeToken: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Withdraw {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Withdraw(address,address,address,uint256,address,uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                181u8,
                11u8,
                150u8,
                207u8,
                86u8,
                82u8,
                247u8,
                37u8,
                155u8,
                163u8,
                56u8,
                145u8,
                67u8,
                106u8,
                56u8,
                116u8,
                236u8,
                206u8,
                57u8,
                229u8,
                34u8,
                161u8,
                143u8,
                185u8,
                115u8,
                203u8,
                2u8,
                25u8,
                73u8,
                245u8,
                115u8,
                223u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawer: topics.1,
                    baseToken: data.0,
                    memeToken: data.1,
                    withdrawAmount: data.2,
                    to: data.3,
                    baseCollateral: data.4,
                    baseDebtScaled: data.5,
                    memeCollateral: data.6,
                    memeDebtScaled: data.7,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.baseToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.memeToken,
                    ),
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.withdrawer.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.withdrawer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Withdraw {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Withdraw> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Withdraw) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _roleStore);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _roleStore: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._roleStore,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _roleStore: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                )
            }
        }
    };
    /**Function with signature `emitAdd(address,address,address,address,uint256,uint256)` and selector `0x9ed486eb`.
```solidity
function emitAdd(address supplier, address baseToken, address memeToken, address to, uint256 baseAmount, uint256 memeAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitAddCall {
        pub supplier: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub baseAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub memeAmount: alloy::sol_types::private::primitives::aliases::U256,
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
                        value.supplier,
                        value.baseToken,
                        value.memeToken,
                        value.to,
                        value.baseAmount,
                        value.memeAmount,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitAddCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        supplier: tuple.0,
                        baseToken: tuple.1,
                        memeToken: tuple.2,
                        to: tuple.3,
                        baseAmount: tuple.4,
                        memeAmount: tuple.5,
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
                        &self.supplier,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.baseAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeAmount),
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
    /**Function with signature `emitClose(address,address,uint256,uint256,uint256)` and selector `0x324ac765`.
```solidity
function emitClose(address underlyingAssetUsd, address account, uint256 amountUsdStartClose, uint256 amountUsdAfterRepayAndSellCollateral, uint256 amountUsdAfterBuyCollateralAndRepay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitCloseCall {
        pub underlyingAssetUsd: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
        pub amountUsdStartClose: alloy::sol_types::private::primitives::aliases::U256,
        pub amountUsdAfterRepayAndSellCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub amountUsdAfterBuyCollateralAndRepay: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitClose(address,address,uint256,uint256,uint256)`](emitCloseCall) function.
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
                alloy::sol_types::sol_data::Address,
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
                    (
                        value.underlyingAssetUsd,
                        value.account,
                        value.amountUsdStartClose,
                        value.amountUsdAfterRepayAndSellCollateral,
                        value.amountUsdAfterBuyCollateralAndRepay,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emitCloseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        underlyingAssetUsd: tuple.0,
                        account: tuple.1,
                        amountUsdStartClose: tuple.2,
                        amountUsdAfterRepayAndSellCollateral: tuple.3,
                        amountUsdAfterBuyCollateralAndRepay: tuple.4,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
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
            const SIGNATURE: &'static str = "emitClose(address,address,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [50u8, 74u8, 199u8, 101u8];
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
                        &self.underlyingAssetUsd,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountUsdStartClose),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.amountUsdAfterRepayAndSellCollateral,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.amountUsdAfterBuyCollateralAndRepay,
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
    /**Function with signature `emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256)` and selector `0x89cad4d9`.
```solidity
function emitDeposit(address depositor, address baseToken, address memeToken, uint256 depositAmount, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitDepositCall {
        pub depositor: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub depositAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256)`](emitDepositCall) function.
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
                        depositAmount: tuple.3,
                        baseCollateral: tuple.4,
                        baseDebtScaled: tuple.5,
                        memeCollateral: tuple.6,
                        memeDebtScaled: tuple.7,
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
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitDepositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitDeposit(address,address,address,uint256,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [137u8, 202u8, 212u8, 217u8];
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
    /**Function with signature `emitLiquidation(address,address,uint256,uint256,uint256,uint256)` and selector `0x94e0dd1d`.
```solidity
function emitLiquidation(address liquidator, address account, uint256 marginLevel, uint256 marginLevelLiquidationThreshold, uint256 totalCollateralUsd, uint256 totalDebtUsd) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitLiquidationCall {
        pub liquidator: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
        pub marginLevel: alloy::sol_types::private::primitives::aliases::U256,
        pub marginLevelLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
        pub totalCollateralUsd: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebtUsd: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitLiquidation(address,address,uint256,uint256,uint256,uint256)`](emitLiquidationCall) function.
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<emitLiquidationCall> for UnderlyingRustTuple<'_> {
                fn from(value: emitLiquidationCall) -> Self {
                    (
                        value.liquidator,
                        value.account,
                        value.marginLevel,
                        value.marginLevelLiquidationThreshold,
                        value.totalCollateralUsd,
                        value.totalDebtUsd,
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
                        marginLevel: tuple.2,
                        marginLevelLiquidationThreshold: tuple.3,
                        totalCollateralUsd: tuple.4,
                        totalDebtUsd: tuple.5,
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
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emitLiquidationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emitLiquidation(address,address,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [148u8, 224u8, 221u8, 29u8];
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
    /**Function with signature `emitRemove(address,address,address,uint256,address,uint256,uint256)` and selector `0x292ae722`.
```solidity
function emitRemove(address remover, address baseToken, address memeToken, uint256 liquidity, address to, uint256 baseAmount, uint256 memeAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRemoveCall {
        pub remover: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
        pub baseAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub memeAmount: alloy::sol_types::private::primitives::aliases::U256,
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
                        value.baseAmount,
                        value.memeAmount,
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
                        baseAmount: tuple.5,
                        memeAmount: tuple.6,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.baseAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.memeAmount),
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
    /**Function with signature `emitRepay(address,address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))` and selector `0x6a6d992f`.
```solidity
function emitRepay(address eventEmitter, address repayer, address baseToken, address memeToken, uint256 positionId, uint8 tokenIndex, uint256 repayAmount, Event.Liquidation memory liquidation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitRepayCall {
        pub eventEmitter: alloy::sol_types::private::Address,
        pub repayer: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub repayAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`emitRepay(address,address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))`](emitRepayCall) function.
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
                        value.eventEmitter,
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
                        eventEmitter: tuple.0,
                        repayer: tuple.1,
                        baseToken: tuple.2,
                        memeToken: tuple.3,
                        positionId: tuple.4,
                        tokenIndex: tuple.5,
                        repayAmount: tuple.6,
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
            const SIGNATURE: &'static str = "emitRepay(address,address,address,address,uint256,uint8,uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [106u8, 109u8, 153u8, 47u8];
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
                        &self.eventEmitter,
                    ),
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
    /**Function with signature `emitSwap(address,address,address,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))` and selector `0x96de247f`.
```solidity
function emitSwap(address account, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee, Event.Liquidation memory liquidation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitSwapCall {
        pub account: alloy::sol_types::private::Address,
        pub tokenIn: alloy::sol_types::private::Address,
        pub tokenOut: alloy::sol_types::private::Address,
        pub amountIn: alloy::sol_types::private::primitives::aliases::U256,
        pub amountOut: alloy::sol_types::private::primitives::aliases::U256,
        pub fee: alloy::sol_types::private::primitives::aliases::U256,
        pub liquidation: <Event::Liquidation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`emitSwap(address,address,address,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))`](emitSwapCall) function.
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
                        amountIn: tuple.3,
                        amountOut: tuple.4,
                        fee: tuple.5,
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
            const SIGNATURE: &'static str = "emitSwap(address,address,address,uint256,uint256,uint256,(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [150u8, 222u8, 36u8, 127u8];
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
    /**Function with signature `emitWithdraw(address,address,address,uint256,address,uint256,uint256,uint256,uint256)` and selector `0x15f762d5`.
```solidity
function emitWithdraw(address withdrawer, address baseToken, address memeToken, uint256 withdrawAmount, address to, uint256 baseCollateral, uint256 baseDebtScaled, uint256 memeCollateral, uint256 memeDebtScaled) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emitWithdrawCall {
        pub withdrawer: alloy::sol_types::private::Address,
        pub baseToken: alloy::sol_types::private::Address,
        pub memeToken: alloy::sol_types::private::Address,
        pub withdrawAmount: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
        pub baseCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub baseDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
        pub memeCollateral: alloy::sol_types::private::primitives::aliases::U256,
        pub memeDebtScaled: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`emitWithdraw(address,address,address,uint256,address,uint256,uint256,uint256,uint256)`](emitWithdrawCall) function.
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
                        withdrawAmount: tuple.3,
                        to: tuple.4,
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
            const SIGNATURE: &'static str = "emitWithdraw(address,address,address,uint256,address,uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [21u8, 247u8, 98u8, 213u8];
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
    ///Container for all the [`EventEmitter`](self) function calls.
    pub enum EventEmitterCalls {
        emitAdd(emitAddCall),
        emitBorrow(emitBorrowCall),
        emitClaimFees(emitClaimFeesCall),
        emitClose(emitCloseCall),
        emitDeposit(emitDepositCall),
        emitLiquidation(emitLiquidationCall),
        emitPoolUpdated(emitPoolUpdatedCall),
        emitRemove(emitRemoveCall),
        emitRepay(emitRepayCall),
        emitSwap(emitSwapCall),
        emitWithdraw(emitWithdrawCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl EventEmitterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [21u8, 247u8, 98u8, 213u8],
            [41u8, 42u8, 231u8, 34u8],
            [50u8, 74u8, 199u8, 101u8],
            [74u8, 74u8, 123u8, 4u8],
            [106u8, 109u8, 153u8, 47u8],
            [124u8, 36u8, 218u8, 183u8],
            [137u8, 202u8, 212u8, 217u8],
            [148u8, 224u8, 221u8, 29u8],
            [150u8, 222u8, 36u8, 127u8],
            [156u8, 132u8, 87u8, 146u8],
            [158u8, 212u8, 134u8, 235u8],
            [234u8, 52u8, 165u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EventEmitterCalls {
        const NAME: &'static str = "EventEmitterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 12usize;
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
                Self::emitPoolUpdated(_) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EventEmitterCalls>] = &[
                {
                    fn emitWithdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitWithdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitWithdraw)
                    }
                    emitWithdraw
                },
                {
                    fn emitRemove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitRemoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitRemove)
                    }
                    emitRemove
                },
                {
                    fn emitClose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitCloseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitClose)
                    }
                    emitClose
                },
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn emitRepay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitRepayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitRepay)
                    }
                    emitRepay
                },
                {
                    fn emitPoolUpdated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitPoolUpdated)
                    }
                    emitPoolUpdated
                },
                {
                    fn emitDeposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitDepositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitDeposit)
                    }
                    emitDeposit
                },
                {
                    fn emitLiquidation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitLiquidationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitLiquidation)
                    }
                    emitLiquidation
                },
                {
                    fn emitSwap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitSwapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitSwap)
                    }
                    emitSwap
                },
                {
                    fn emitClaimFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitClaimFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitClaimFees)
                    }
                    emitClaimFees
                },
                {
                    fn emitAdd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitAddCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitAdd)
                    }
                    emitAdd
                },
                {
                    fn emitBorrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterCalls> {
                        <emitBorrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterCalls::emitBorrow)
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
                Self::emitPoolUpdated(inner) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::emitPoolUpdated(inner) => {
                    <emitPoolUpdatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::roleStore(inner) => {
                    <roleStoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EventEmitter`](self) custom errors.
    pub enum EventEmitterErrors {
        Unauthorized(Unauthorized),
    }
    #[automatically_derived]
    impl EventEmitterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[163u8, 91u8, 21u8, 11u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EventEmitterErrors {
        const NAME: &'static str = "EventEmitterErrors";
        const MIN_DATA_LENGTH: usize = 96usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
            ) -> alloy_sol_types::Result<EventEmitterErrors>] = &[
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EventEmitterErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EventEmitterErrors::Unauthorized)
                    }
                    Unauthorized
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
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EventEmitter`](self) events.
    pub enum EventEmitterEvents {
        Add(Add),
        Borrow(Borrow),
        ClaimFees(ClaimFees),
        Close(Close),
        Deposit(Deposit),
        Liquidation(Liquidation),
        PoolUpdated(PoolUpdated),
        Remove(Remove),
        Repay(Repay),
        Swap(Swap),
        Withdraw(Withdraw),
    }
    #[automatically_derived]
    impl EventEmitterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                37u8,
                154u8,
                249u8,
                26u8,
                248u8,
                156u8,
                154u8,
                107u8,
                19u8,
                213u8,
                54u8,
                7u8,
                213u8,
                127u8,
                67u8,
                177u8,
                81u8,
                35u8,
                95u8,
                105u8,
                213u8,
                77u8,
                35u8,
                57u8,
                19u8,
                62u8,
                87u8,
                207u8,
                182u8,
                43u8,
                244u8,
                197u8,
            ],
            [
                46u8,
                241u8,
                232u8,
                115u8,
                125u8,
                9u8,
                104u8,
                38u8,
                201u8,
                171u8,
                239u8,
                18u8,
                1u8,
                187u8,
                32u8,
                94u8,
                163u8,
                128u8,
                85u8,
                87u8,
                128u8,
                165u8,
                75u8,
                217u8,
                4u8,
                207u8,
                103u8,
                202u8,
                125u8,
                186u8,
                140u8,
                95u8,
            ],
            [
                79u8,
                195u8,
                207u8,
                87u8,
                241u8,
                197u8,
                135u8,
                169u8,
                240u8,
                128u8,
                136u8,
                18u8,
                221u8,
                114u8,
                102u8,
                138u8,
                43u8,
                130u8,
                245u8,
                68u8,
                17u8,
                176u8,
                115u8,
                125u8,
                6u8,
                196u8,
                4u8,
                63u8,
                240u8,
                68u8,
                207u8,
                154u8,
            ],
            [
                83u8,
                56u8,
                85u8,
                203u8,
                138u8,
                135u8,
                240u8,
                153u8,
                95u8,
                108u8,
                51u8,
                144u8,
                102u8,
                94u8,
                179u8,
                125u8,
                168u8,
                34u8,
                245u8,
                85u8,
                11u8,
                19u8,
                169u8,
                115u8,
                167u8,
                121u8,
                6u8,
                208u8,
                24u8,
                162u8,
                168u8,
                156u8,
            ],
            [
                177u8,
                73u8,
                71u8,
                2u8,
                1u8,
                96u8,
                13u8,
                129u8,
                225u8,
                105u8,
                138u8,
                157u8,
                0u8,
                29u8,
                32u8,
                248u8,
                210u8,
                160u8,
                54u8,
                210u8,
                192u8,
                244u8,
                37u8,
                67u8,
                107u8,
                101u8,
                161u8,
                84u8,
                233u8,
                94u8,
                181u8,
                58u8,
            ],
            [
                181u8,
                11u8,
                150u8,
                207u8,
                86u8,
                82u8,
                247u8,
                37u8,
                155u8,
                163u8,
                56u8,
                145u8,
                67u8,
                106u8,
                56u8,
                116u8,
                236u8,
                206u8,
                57u8,
                229u8,
                34u8,
                161u8,
                143u8,
                185u8,
                115u8,
                203u8,
                2u8,
                25u8,
                73u8,
                245u8,
                115u8,
                223u8,
            ],
            [
                195u8,
                32u8,
                168u8,
                82u8,
                155u8,
                21u8,
                184u8,
                81u8,
                170u8,
                166u8,
                133u8,
                25u8,
                145u8,
                154u8,
                195u8,
                68u8,
                225u8,
                202u8,
                206u8,
                175u8,
                79u8,
                71u8,
                209u8,
                93u8,
                246u8,
                229u8,
                129u8,
                129u8,
                223u8,
                236u8,
                99u8,
                25u8,
            ],
            [
                199u8,
                15u8,
                209u8,
                235u8,
                64u8,
                215u8,
                174u8,
                68u8,
                165u8,
                66u8,
                92u8,
                9u8,
                222u8,
                206u8,
                90u8,
                2u8,
                59u8,
                71u8,
                39u8,
                115u8,
                86u8,
                224u8,
                5u8,
                182u8,
                202u8,
                239u8,
                22u8,
                0u8,
                207u8,
                199u8,
                51u8,
                78u8,
            ],
            [
                203u8,
                219u8,
                77u8,
                216u8,
                248u8,
                73u8,
                131u8,
                190u8,
                124u8,
                21u8,
                128u8,
                19u8,
                184u8,
                215u8,
                77u8,
                172u8,
                241u8,
                20u8,
                51u8,
                48u8,
                120u8,
                148u8,
                159u8,
                153u8,
                223u8,
                181u8,
                182u8,
                110u8,
                102u8,
                33u8,
                150u8,
                74u8,
            ],
            [
                210u8,
                230u8,
                8u8,
                83u8,
                21u8,
                198u8,
                225u8,
                193u8,
                199u8,
                64u8,
                106u8,
                71u8,
                199u8,
                208u8,
                6u8,
                168u8,
                193u8,
                249u8,
                49u8,
                57u8,
                109u8,
                134u8,
                140u8,
                22u8,
                4u8,
                109u8,
                234u8,
                113u8,
                54u8,
                95u8,
                240u8,
                49u8,
            ],
            [
                225u8,
                10u8,
                51u8,
                157u8,
                213u8,
                50u8,
                157u8,
                241u8,
                74u8,
                142u8,
                193u8,
                62u8,
                180u8,
                17u8,
                91u8,
                117u8,
                171u8,
                3u8,
                47u8,
                212u8,
                14u8,
                47u8,
                242u8,
                89u8,
                74u8,
                74u8,
                91u8,
                248u8,
                14u8,
                73u8,
                122u8,
                65u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EventEmitterEvents {
        const NAME: &'static str = "EventEmitterEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Add as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Add as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Add)
                }
                Some(<Borrow as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Borrow as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Borrow)
                }
                Some(<ClaimFees as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ClaimFees as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ClaimFees)
                }
                Some(<Close as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Close as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Close)
                }
                Some(<Deposit as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Deposit as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Deposit)
                }
                Some(<Liquidation as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Liquidation as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Liquidation)
                }
                Some(<PoolUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PoolUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PoolUpdated)
                }
                Some(<Remove as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Remove as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Remove)
                }
                Some(<Repay as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Repay as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Repay)
                }
                Some(<Swap as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Swap as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Swap)
                }
                Some(<Withdraw as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Withdraw as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Withdraw)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for EventEmitterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Add(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Borrow(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ClaimFees(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Close(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Deposit(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Liquidation(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PoolUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Remove(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Repay(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdraw(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Add(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Borrow(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ClaimFees(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Close(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Deposit(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Liquidation(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PoolUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Remove(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Repay(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdraw(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EventEmitter`](self) contract instance.

See the [wrapper's documentation](`EventEmitterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EventEmitterInstance<T, P, N> {
        EventEmitterInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EventEmitterInstance<T, P, N>>,
    > {
        EventEmitterInstance::<T, P, N>::deploy(provider, _roleStore)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EventEmitterInstance::<T, P, N>::deploy_builder(provider, _roleStore)
    }
    /**A [`EventEmitter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EventEmitter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EventEmitterInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EventEmitterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EventEmitterInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EventEmitterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EventEmitter`](self) contract instance.

See the [wrapper's documentation](`EventEmitterInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EventEmitterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _roleStore);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _roleStore },
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
    impl<T, P: ::core::clone::Clone, N> EventEmitterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EventEmitterInstance<T, P, N> {
            EventEmitterInstance {
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
    > EventEmitterInstance<T, P, N> {
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
            supplier: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            baseAmount: alloy::sol_types::private::primitives::aliases::U256,
            memeAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitAddCall, N> {
            self.call_builder(
                &emitAddCall {
                    supplier,
                    baseToken,
                    memeToken,
                    to,
                    baseAmount,
                    memeAmount,
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
            underlyingAssetUsd: alloy::sol_types::private::Address,
            account: alloy::sol_types::private::Address,
            amountUsdStartClose: alloy::sol_types::private::primitives::aliases::U256,
            amountUsdAfterRepayAndSellCollateral: alloy::sol_types::private::primitives::aliases::U256,
            amountUsdAfterBuyCollateralAndRepay: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitCloseCall, N> {
            self.call_builder(
                &emitCloseCall {
                    underlyingAssetUsd,
                    account,
                    amountUsdStartClose,
                    amountUsdAfterRepayAndSellCollateral,
                    amountUsdAfterBuyCollateralAndRepay,
                },
            )
        }
        ///Creates a new call builder for the [`emitDeposit`] function.
        pub fn emitDeposit(
            &self,
            depositor: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
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
            marginLevel: alloy::sol_types::private::primitives::aliases::U256,
            marginLevelLiquidationThreshold: alloy::sol_types::private::primitives::aliases::U256,
            totalCollateralUsd: alloy::sol_types::private::primitives::aliases::U256,
            totalDebtUsd: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitLiquidationCall, N> {
            self.call_builder(
                &emitLiquidationCall {
                    liquidator,
                    account,
                    marginLevel,
                    marginLevelLiquidationThreshold,
                    totalCollateralUsd,
                    totalDebtUsd,
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
        ///Creates a new call builder for the [`emitRemove`] function.
        pub fn emitRemove(
            &self,
            remover: alloy::sol_types::private::Address,
            baseToken: alloy::sol_types::private::Address,
            memeToken: alloy::sol_types::private::Address,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            baseAmount: alloy::sol_types::private::primitives::aliases::U256,
            memeAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, emitRemoveCall, N> {
            self.call_builder(
                &emitRemoveCall {
                    remover,
                    baseToken,
                    memeToken,
                    liquidity,
                    to,
                    baseAmount,
                    memeAmount,
                },
            )
        }
        ///Creates a new call builder for the [`emitRepay`] function.
        pub fn emitRepay(
            &self,
            eventEmitter: alloy::sol_types::private::Address,
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
                    eventEmitter,
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
                    withdrawAmount,
                    to,
                    baseCollateral,
                    baseDebtScaled,
                    memeCollateral,
                    memeDebtScaled,
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
    > EventEmitterInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Add`] event.
        pub fn Add_filter(&self) -> alloy_contract::Event<T, &P, Add, N> {
            self.event_filter::<Add>()
        }
        ///Creates a new event filter for the [`Borrow`] event.
        pub fn Borrow_filter(&self) -> alloy_contract::Event<T, &P, Borrow, N> {
            self.event_filter::<Borrow>()
        }
        ///Creates a new event filter for the [`ClaimFees`] event.
        pub fn ClaimFees_filter(&self) -> alloy_contract::Event<T, &P, ClaimFees, N> {
            self.event_filter::<ClaimFees>()
        }
        ///Creates a new event filter for the [`Close`] event.
        pub fn Close_filter(&self) -> alloy_contract::Event<T, &P, Close, N> {
            self.event_filter::<Close>()
        }
        ///Creates a new event filter for the [`Deposit`] event.
        pub fn Deposit_filter(&self) -> alloy_contract::Event<T, &P, Deposit, N> {
            self.event_filter::<Deposit>()
        }
        ///Creates a new event filter for the [`Liquidation`] event.
        pub fn Liquidation_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Liquidation, N> {
            self.event_filter::<Liquidation>()
        }
        ///Creates a new event filter for the [`PoolUpdated`] event.
        pub fn PoolUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PoolUpdated, N> {
            self.event_filter::<PoolUpdated>()
        }
        ///Creates a new event filter for the [`Remove`] event.
        pub fn Remove_filter(&self) -> alloy_contract::Event<T, &P, Remove, N> {
            self.event_filter::<Remove>()
        }
        ///Creates a new event filter for the [`Repay`] event.
        pub fn Repay_filter(&self) -> alloy_contract::Event<T, &P, Repay, N> {
            self.event_filter::<Repay>()
        }
        ///Creates a new event filter for the [`Swap`] event.
        pub fn Swap_filter(&self) -> alloy_contract::Event<T, &P, Swap, N> {
            self.event_filter::<Swap>()
        }
        ///Creates a new event filter for the [`Withdraw`] event.
        pub fn Withdraw_filter(&self) -> alloy_contract::Event<T, &P, Withdraw, N> {
            self.event_filter::<Withdraw>()
        }
    }
}
