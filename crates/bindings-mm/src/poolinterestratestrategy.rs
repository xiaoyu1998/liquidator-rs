///Module containing a contract's types and functions.
/**

```solidity
library InterestUtils {
    struct CalculateInterestRatesParams { uint256 totalPoolBalance; uint256 totalDebt; uint256 totalPoolBalanceBase; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod InterestUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct CalculateInterestRatesParams { uint256 totalPoolBalance; uint256 totalDebt; uint256 totalPoolBalanceBase; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CalculateInterestRatesParams {
        pub totalPoolBalance: alloy::sol_types::private::primitives::aliases::U256,
        pub totalDebt: alloy::sol_types::private::primitives::aliases::U256,
        pub totalPoolBalanceBase: alloy::sol_types::private::primitives::aliases::U256,
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<CalculateInterestRatesParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: CalculateInterestRatesParams) -> Self {
                (value.totalPoolBalance, value.totalDebt, value.totalPoolBalanceBase)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CalculateInterestRatesParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    totalPoolBalance: tuple.0,
                    totalDebt: tuple.1,
                    totalPoolBalanceBase: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CalculateInterestRatesParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for CalculateInterestRatesParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalPoolBalance),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalDebt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalPoolBalanceBase),
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
        impl alloy_sol_types::SolType for CalculateInterestRatesParams {
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
        impl alloy_sol_types::SolStruct for CalculateInterestRatesParams {
            const NAME: &'static str = "CalculateInterestRatesParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CalculateInterestRatesParams(uint256 totalPoolBalance,uint256 totalDebt,uint256 totalPoolBalanceBase)",
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
                            &self.totalPoolBalance,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalDebt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalPoolBalanceBase,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CalculateInterestRatesParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalPoolBalance,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalDebt,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalPoolBalanceBase,
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
                    &rust.totalPoolBalance,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalDebt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalPoolBalanceBase,
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
    /**Creates a new wrapper around an on-chain [`InterestUtils`](self) contract instance.

See the [wrapper's documentation](`InterestUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> InterestUtilsInstance<T, P, N> {
        InterestUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`InterestUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`InterestUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct InterestUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for InterestUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("InterestUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > InterestUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`InterestUtils`](self) contract instance.

See the [wrapper's documentation](`InterestUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> InterestUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> InterestUtilsInstance<T, P, N> {
            InterestUtilsInstance {
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
    > InterestUtilsInstance<T, P, N> {
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
    > InterestUtilsInstance<T, P, N> {
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
library InterestUtils {
    struct CalculateInterestRatesParams {
        uint256 totalPoolBalance;
        uint256 totalDebt;
        uint256 totalPoolBalanceBase;
    }
}

interface PoolInterestRateStrategy {
    struct InterestRateStrategyParams {
        uint256 optimalUsageRatio;
        uint256 rate0;
        uint256[] rateSlope1;
        uint256[] rateSlope2;
    }

    error InvalidOptimalUsageRate(uint256 optimalUsageRatio);

    constructor(InterestRateStrategyParams params);

    function BALANCE10000() external view returns (uint256);
    function BALANCE1000000() external view returns (uint256);
    function BALANCE50000() external view returns (uint256);
    function BALANCE500000() external view returns (uint256);
    function MAX_EXCESS_USAGE_RATIO() external view returns (uint256);
    function OPTIMAL_USAGE_RATIO() external view returns (uint256);
    function SLOPESIZE() external view returns (uint8);
    function calculateInterestRates(InterestUtils.CalculateInterestRatesParams memory params) external view returns (uint256);
    function getOptimalUsageRatio() external view returns (uint256);
    function getRateSlope1() external view returns (uint256[] memory);
    function getRateSlope2() external view returns (uint256[] memory);
    function getRatebase() external view returns (uint256);
    function getSlopeIndex(uint256 balance) external view returns (uint8);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct PoolInterestRateStrategy.InterestRateStrategyParams",
        "components": [
          {
            "name": "optimalUsageRatio",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "rate0",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "rateSlope1",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "rateSlope2",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "BALANCE10000",
    "inputs": [],
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
    "name": "BALANCE1000000",
    "inputs": [],
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
    "name": "BALANCE50000",
    "inputs": [],
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
    "name": "BALANCE500000",
    "inputs": [],
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
    "name": "MAX_EXCESS_USAGE_RATIO",
    "inputs": [],
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
    "name": "OPTIMAL_USAGE_RATIO",
    "inputs": [],
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
    "name": "SLOPESIZE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateInterestRates",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct InterestUtils.CalculateInterestRatesParams",
        "components": [
          {
            "name": "totalPoolBalance",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalDebt",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "totalPoolBalanceBase",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
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
    "name": "getOptimalUsageRatio",
    "inputs": [],
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
    "name": "getRateSlope1",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRateSlope2",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRatebase",
    "inputs": [],
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
    "name": "getSlopeIndex",
    "inputs": [
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "InvalidOptimalUsageRate",
    "inputs": [
      {
        "name": "optimalUsageRatio",
        "type": "uint256",
        "internalType": "uint256"
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
pub mod PoolInterestRateStrategy {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b50604051610aff380380610aff83398101604081905261002e9161020c565b80516b033b2e3c9fd0803ce80000001015610066578051604051635c86f9f960e11b8152600481019190915260240160405180910390fd5b80516080528051610083906b033b2e3c9fd0803ce80000006102b4565b60a05260208082015160c052604082015180516100a3925f9201906100c4565b50606081015180516100bd916001916020909101906100c4565b50506102d9565b828054828255905f5260205f209081019282156100fd579160200282015b828111156100fd5782518255916020019190600101906100e2565b5061010992915061010d565b5090565b5b80821115610109575f815560010161010e565b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b038111828210171561015757610157610121565b60405290565b604051601f8201601f191681016001600160401b038111828210171561018557610185610121565b604052919050565b5f82601f83011261019c575f5ffd5b81516001600160401b038111156101b5576101b5610121565b8060051b6101c56020820161015d565b918252602081850181019290810190868411156101e0575f5ffd5b6020860192505b838310156102025782518252602092830192909101906101e7565b9695505050505050565b5f6020828403121561021c575f5ffd5b81516001600160401b03811115610231575f5ffd5b820160808185031215610242575f5ffd5b61024a610135565b815181526020808301519082015260408201516001600160401b03811115610270575f5ffd5b61027c8682850161018d565b60408301525060608201516001600160401b0381111561029a575f5ffd5b6102a68682850161018d565b606083015250949350505050565b818103818111156102d357634e487b7160e01b5f52601160045260245ffd5b92915050565b60805160a05160c0516107d361032c5f395f8181610159015261040e01525f81816101b0015261050101525f818160f201528181610187015281816104d20152818161052201526105a101526107d35ff3fe608060405234801561000f575f5ffd5b50600436106100cb575f3560e01c8063a3a9bc3311610088578063b1b0648511610063578063b1b06485146101d2578063d41a41ba146101da578063ed719cf8146101e2578063fdd63ecf146101ea575f5ffd5b8063a3a9bc331461017d578063a8602e8614610185578063a9c622f8146101ab575f5ffd5b80632dd9037b146100cf57806354c365c6146100ed57806375005bb0146101225780637aa786151461012a5780638567992a1461013257806387f0409d14610157575b5f5ffd5b6100d76101fd565b6040516100e49190610674565b60405180910390f35b6101147f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020016100e4565b6100d7610252565b6101146102a6565b6101456101403660046106b6565b6102c0565b60405160ff90911681526020016100e4565b7f0000000000000000000000000000000000000000000000000000000000000000610114565b610114610362565b7f0000000000000000000000000000000000000000000000000000000000000000610114565b6101147f000000000000000000000000000000000000000000000000000000000000000081565b610145600681565b610114610379565b610114610391565b6101146101f83660046106cd565b6103a9565b60605f80548060200260200160405190810160405280929190818152602001828054801561024857602002820191905f5260205f20905b815481526020019060010190808311610234575b5050505050905090565b6060600180548060200260200160405190810160405280929190818152602001828054801561024857602002820191905f5260205f2090815481526020019060010190808311610234575050505050905090565b6102bd676765c793fa10079d601b1b612710610746565b81565b5f6102d8676765c793fa10079d601b1b612710610746565b8210156102e657505f919050565b6102fd676765c793fa10079d601b1b61c350610746565b82101561030c57506001919050565b610324676765c793fa10079d601b1b6207a120610746565b82101561033357506002919050565b61034b676765c793fa10079d601b1b620f4240610746565b82101561035a57506003919050565b506004919050565b6102bd676765c793fa10079d601b1b61c350610746565b6102bd676765c793fa10079d601b1b620f4240610746565b6102bd676765c793fa10079d601b1b6207a120610746565b5f81602001515f036103bc57505f919050565b6104026040518061012001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b602080840151908201527f0000000000000000000000000000000000000000000000000000000000000000604080830191909152830151610442906102c0565b60ff1660e082018190525f8054909190811061046057610460610763565b905f5260205f2001548160a001818152505060018160e001518154811061048957610489610763565b5f918252602091829020015460c0830152810151156104d057825180825260208201516104b591610777565b6080820181905260208201516104ca916105f8565b60608201525b7f000000000000000000000000000000000000000000000000000000000000000081606001511115610594576105567f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008360600151610550919061078a565b906105f8565b610100820181905260c082015161056c91610633565b8160a0015161057b9190610777565b8160400181815161058c9190610777565b9052506105ee565b6105d981606001516105d37f00000000000000000000000000000000000000000000000000000000000000008460a001516105f890919063ffffffff16565b90610633565b816040018181516105ea9190610777565b9052505b6040015192915050565b5f8115676765c793fa10079d601b1b60028404190484111715610619575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f81156b019d971e4fe8401e740000001983900484111517610653575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b602080825282518282018190525f918401906040840190835b818110156106ab57835183526020938401939092019160010161068d565b509095945050505050565b5f602082840312156106c6575f5ffd5b5035919050565b5f60608284031280156106de575f5ffd5b506040516060810167ffffffffffffffff8111828210171561070e57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b634e487b7160e01b5f52601160045260245ffd5b808202811582820484141761075d5761075d610732565b92915050565b634e487b7160e01b5f52603260045260245ffd5b8082018082111561075d5761075d610732565b8181038181111561075d5761075d61073256fea264697066735822122004ab95bc508e1ce5c2f2e1b9631627b9de275900c5dec38b955ee71adaa581b664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\n\xFF8\x03\x80a\n\xFF\x839\x81\x01`@\x81\x90Ra\0.\x91a\x02\x0CV[\x80Qk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10\x15a\0fW\x80Q`@Qc\\\x86\xF9\xF9`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\x80R\x80Qa\0\x83\x90k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x02\xB4V[`\xA0R` \x80\x82\x01Q`\xC0R`@\x82\x01Q\x80Qa\0\xA3\x92_\x92\x01\x90a\0\xC4V[P``\x81\x01Q\x80Qa\0\xBD\x91`\x01\x91` \x90\x91\x01\x90a\0\xC4V[PPa\x02\xD9V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\0\xFDW\x91` \x02\x82\x01[\x82\x81\x11\x15a\0\xFDW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\0\xE2V[Pa\x01\t\x92\x91Pa\x01\rV[P\x90V[[\x80\x82\x11\x15a\x01\tW_\x81U`\x01\x01a\x01\x0EV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01WWa\x01Wa\x01!V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x85Wa\x01\x85a\x01!V[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x01\x9CW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xB5Wa\x01\xB5a\x01!V[\x80`\x05\x1Ba\x01\xC5` \x82\x01a\x01]V[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15a\x01\xE0W__\xFD[` \x86\x01\x92P[\x83\x83\x10\x15a\x02\x02W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90a\x01\xE7V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x02\x1CW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x021W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x02BW__\xFD[a\x02Ja\x015V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02pW__\xFD[a\x02|\x86\x82\x85\x01a\x01\x8DV[`@\x83\x01RP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x9AW__\xFD[a\x02\xA6\x86\x82\x85\x01a\x01\x8DV[``\x83\x01RP\x94\x93PPPPV[\x81\x81\x03\x81\x81\x11\x15a\x02\xD3WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Qa\x07\xD3a\x03,_9_\x81\x81a\x01Y\x01Ra\x04\x0E\x01R_\x81\x81a\x01\xB0\x01Ra\x05\x01\x01R_\x81\x81`\xF2\x01R\x81\x81a\x01\x87\x01R\x81\x81a\x04\xD2\x01R\x81\x81a\x05\"\x01Ra\x05\xA1\x01Ra\x07\xD3_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCBW_5`\xE0\x1C\x80c\xA3\xA9\xBC3\x11a\0\x88W\x80c\xB1\xB0d\x85\x11a\0cW\x80c\xB1\xB0d\x85\x14a\x01\xD2W\x80c\xD4\x1AA\xBA\x14a\x01\xDAW\x80c\xEDq\x9C\xF8\x14a\x01\xE2W\x80c\xFD\xD6>\xCF\x14a\x01\xEAW__\xFD[\x80c\xA3\xA9\xBC3\x14a\x01}W\x80c\xA8`.\x86\x14a\x01\x85W\x80c\xA9\xC6\"\xF8\x14a\x01\xABW__\xFD[\x80c-\xD9\x03{\x14a\0\xCFW\x80cT\xC3e\xC6\x14a\0\xEDW\x80cu\0[\xB0\x14a\x01\"W\x80cz\xA7\x86\x15\x14a\x01*W\x80c\x85g\x99*\x14a\x012W\x80c\x87\xF0@\x9D\x14a\x01WW[__\xFD[a\0\xD7a\x01\xFDV[`@Qa\0\xE4\x91\x90a\x06tV[`@Q\x80\x91\x03\x90\xF3[a\x01\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\0\xE4V[a\0\xD7a\x02RV[a\x01\x14a\x02\xA6V[a\x01Ea\x01@6`\x04a\x06\xB6V[a\x02\xC0V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xE4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\x14V[a\x01\x14a\x03bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\x14V[a\x01\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01E`\x06\x81V[a\x01\x14a\x03yV[a\x01\x14a\x03\x91V[a\x01\x14a\x01\xF86`\x04a\x06\xCDV[a\x03\xA9V[``_\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02HW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x024W[PPPPP\x90P\x90V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02HW` \x02\x82\x01\x91\x90_R` _ \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x024WPPPPP\x90P\x90V[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba'\x10a\x07FV[\x81V[_a\x02\xD8gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba'\x10a\x07FV[\x82\x10\x15a\x02\xE6WP_\x91\x90PV[a\x02\xFDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\xC3Pa\x07FV[\x82\x10\x15a\x03\x0CWP`\x01\x91\x90PV[a\x03$gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x07\xA1 a\x07FV[\x82\x10\x15a\x033WP`\x02\x91\x90PV[a\x03Kgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x0FB@a\x07FV[\x82\x10\x15a\x03ZWP`\x03\x91\x90PV[P`\x04\x91\x90PV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\xC3Pa\x07FV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x0FB@a\x07FV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x07\xA1 a\x07FV[_\x81` \x01Q_\x03a\x03\xBCWP_\x91\x90PV[a\x04\x02`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[` \x80\x84\x01Q\x90\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x80\x83\x01\x91\x90\x91R\x83\x01Qa\x04B\x90a\x02\xC0V[`\xFF\x16`\xE0\x82\x01\x81\x90R_\x80T\x90\x91\x90\x81\x10a\x04`Wa\x04`a\x07cV[\x90_R` _ \x01T\x81`\xA0\x01\x81\x81RPP`\x01\x81`\xE0\x01Q\x81T\x81\x10a\x04\x89Wa\x04\x89a\x07cV[_\x91\x82R` \x91\x82\x90 \x01T`\xC0\x83\x01R\x81\x01Q\x15a\x04\xD0W\x82Q\x80\x82R` \x82\x01Qa\x04\xB5\x91a\x07wV[`\x80\x82\x01\x81\x90R` \x82\x01Qa\x04\xCA\x91a\x05\xF8V[``\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81``\x01Q\x11\x15a\x05\x94Wa\x05V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83``\x01Qa\x05P\x91\x90a\x07\x8AV[\x90a\x05\xF8V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Qa\x05l\x91a\x063V[\x81`\xA0\x01Qa\x05{\x91\x90a\x07wV[\x81`@\x01\x81\x81Qa\x05\x8C\x91\x90a\x07wV[\x90RPa\x05\xEEV[a\x05\xD9\x81``\x01Qa\x05\xD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xA0\x01Qa\x05\xF8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x063V[\x81`@\x01\x81\x81Qa\x05\xEA\x91\x90a\x07wV[\x90RP[`@\x01Q\x92\x91PPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x06\x19W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x06SW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x06\xABW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x8DV[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x06\xC6W__\xFD[P5\x91\x90PV[_``\x82\x84\x03\x12\x80\x15a\x06\xDEW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x0EWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07]Wa\x07]a\x072V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07]Wa\x07]a\x072V[\x81\x81\x03\x81\x81\x11\x15a\x07]Wa\x07]a\x072V\xFE\xA2dipfsX\"\x12 \x04\xAB\x95\xBCP\x8E\x1C\xE5\xC2\xF2\xE1\xB9c\x16'\xB9\xDE'Y\0\xC5\xDE\xC3\x8B\x95^\xE7\x1A\xDA\xA5\x81\xB6dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100cb575f3560e01c8063a3a9bc3311610088578063b1b0648511610063578063b1b06485146101d2578063d41a41ba146101da578063ed719cf8146101e2578063fdd63ecf146101ea575f5ffd5b8063a3a9bc331461017d578063a8602e8614610185578063a9c622f8146101ab575f5ffd5b80632dd9037b146100cf57806354c365c6146100ed57806375005bb0146101225780637aa786151461012a5780638567992a1461013257806387f0409d14610157575b5f5ffd5b6100d76101fd565b6040516100e49190610674565b60405180910390f35b6101147f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020016100e4565b6100d7610252565b6101146102a6565b6101456101403660046106b6565b6102c0565b60405160ff90911681526020016100e4565b7f0000000000000000000000000000000000000000000000000000000000000000610114565b610114610362565b7f0000000000000000000000000000000000000000000000000000000000000000610114565b6101147f000000000000000000000000000000000000000000000000000000000000000081565b610145600681565b610114610379565b610114610391565b6101146101f83660046106cd565b6103a9565b60605f80548060200260200160405190810160405280929190818152602001828054801561024857602002820191905f5260205f20905b815481526020019060010190808311610234575b5050505050905090565b6060600180548060200260200160405190810160405280929190818152602001828054801561024857602002820191905f5260205f2090815481526020019060010190808311610234575050505050905090565b6102bd676765c793fa10079d601b1b612710610746565b81565b5f6102d8676765c793fa10079d601b1b612710610746565b8210156102e657505f919050565b6102fd676765c793fa10079d601b1b61c350610746565b82101561030c57506001919050565b610324676765c793fa10079d601b1b6207a120610746565b82101561033357506002919050565b61034b676765c793fa10079d601b1b620f4240610746565b82101561035a57506003919050565b506004919050565b6102bd676765c793fa10079d601b1b61c350610746565b6102bd676765c793fa10079d601b1b620f4240610746565b6102bd676765c793fa10079d601b1b6207a120610746565b5f81602001515f036103bc57505f919050565b6104026040518061012001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b602080840151908201527f0000000000000000000000000000000000000000000000000000000000000000604080830191909152830151610442906102c0565b60ff1660e082018190525f8054909190811061046057610460610763565b905f5260205f2001548160a001818152505060018160e001518154811061048957610489610763565b5f918252602091829020015460c0830152810151156104d057825180825260208201516104b591610777565b6080820181905260208201516104ca916105f8565b60608201525b7f000000000000000000000000000000000000000000000000000000000000000081606001511115610594576105567f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008360600151610550919061078a565b906105f8565b610100820181905260c082015161056c91610633565b8160a0015161057b9190610777565b8160400181815161058c9190610777565b9052506105ee565b6105d981606001516105d37f00000000000000000000000000000000000000000000000000000000000000008460a001516105f890919063ffffffff16565b90610633565b816040018181516105ea9190610777565b9052505b6040015192915050565b5f8115676765c793fa10079d601b1b60028404190484111715610619575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f81156b019d971e4fe8401e740000001983900484111517610653575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b602080825282518282018190525f918401906040840190835b818110156106ab57835183526020938401939092019160010161068d565b509095945050505050565b5f602082840312156106c6575f5ffd5b5035919050565b5f60608284031280156106de575f5ffd5b506040516060810167ffffffffffffffff8111828210171561070e57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b634e487b7160e01b5f52601160045260245ffd5b808202811582820484141761075d5761075d610732565b92915050565b634e487b7160e01b5f52603260045260245ffd5b8082018082111561075d5761075d610732565b8181038181111561075d5761075d61073256fea264697066735822122004ab95bc508e1ce5c2f2e1b9631627b9de275900c5dec38b955ee71adaa581b664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCBW_5`\xE0\x1C\x80c\xA3\xA9\xBC3\x11a\0\x88W\x80c\xB1\xB0d\x85\x11a\0cW\x80c\xB1\xB0d\x85\x14a\x01\xD2W\x80c\xD4\x1AA\xBA\x14a\x01\xDAW\x80c\xEDq\x9C\xF8\x14a\x01\xE2W\x80c\xFD\xD6>\xCF\x14a\x01\xEAW__\xFD[\x80c\xA3\xA9\xBC3\x14a\x01}W\x80c\xA8`.\x86\x14a\x01\x85W\x80c\xA9\xC6\"\xF8\x14a\x01\xABW__\xFD[\x80c-\xD9\x03{\x14a\0\xCFW\x80cT\xC3e\xC6\x14a\0\xEDW\x80cu\0[\xB0\x14a\x01\"W\x80cz\xA7\x86\x15\x14a\x01*W\x80c\x85g\x99*\x14a\x012W\x80c\x87\xF0@\x9D\x14a\x01WW[__\xFD[a\0\xD7a\x01\xFDV[`@Qa\0\xE4\x91\x90a\x06tV[`@Q\x80\x91\x03\x90\xF3[a\x01\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\0\xE4V[a\0\xD7a\x02RV[a\x01\x14a\x02\xA6V[a\x01Ea\x01@6`\x04a\x06\xB6V[a\x02\xC0V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\0\xE4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\x14V[a\x01\x14a\x03bV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\x14V[a\x01\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01E`\x06\x81V[a\x01\x14a\x03yV[a\x01\x14a\x03\x91V[a\x01\x14a\x01\xF86`\x04a\x06\xCDV[a\x03\xA9V[``_\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02HW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x024W[PPPPP\x90P\x90V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02HW` \x02\x82\x01\x91\x90_R` _ \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x024WPPPPP\x90P\x90V[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba'\x10a\x07FV[\x81V[_a\x02\xD8gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba'\x10a\x07FV[\x82\x10\x15a\x02\xE6WP_\x91\x90PV[a\x02\xFDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\xC3Pa\x07FV[\x82\x10\x15a\x03\x0CWP`\x01\x91\x90PV[a\x03$gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x07\xA1 a\x07FV[\x82\x10\x15a\x033WP`\x02\x91\x90PV[a\x03Kgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x0FB@a\x07FV[\x82\x10\x15a\x03ZWP`\x03\x91\x90PV[P`\x04\x91\x90PV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Ba\xC3Pa\x07FV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x0FB@a\x07FV[a\x02\xBDgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bb\x07\xA1 a\x07FV[_\x81` \x01Q_\x03a\x03\xBCWP_\x91\x90PV[a\x04\x02`@Q\x80a\x01 \x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[` \x80\x84\x01Q\x90\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x80\x83\x01\x91\x90\x91R\x83\x01Qa\x04B\x90a\x02\xC0V[`\xFF\x16`\xE0\x82\x01\x81\x90R_\x80T\x90\x91\x90\x81\x10a\x04`Wa\x04`a\x07cV[\x90_R` _ \x01T\x81`\xA0\x01\x81\x81RPP`\x01\x81`\xE0\x01Q\x81T\x81\x10a\x04\x89Wa\x04\x89a\x07cV[_\x91\x82R` \x91\x82\x90 \x01T`\xC0\x83\x01R\x81\x01Q\x15a\x04\xD0W\x82Q\x80\x82R` \x82\x01Qa\x04\xB5\x91a\x07wV[`\x80\x82\x01\x81\x90R` \x82\x01Qa\x04\xCA\x91a\x05\xF8V[``\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81``\x01Q\x11\x15a\x05\x94Wa\x05V\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83``\x01Qa\x05P\x91\x90a\x07\x8AV[\x90a\x05\xF8V[a\x01\0\x82\x01\x81\x90R`\xC0\x82\x01Qa\x05l\x91a\x063V[\x81`\xA0\x01Qa\x05{\x91\x90a\x07wV[\x81`@\x01\x81\x81Qa\x05\x8C\x91\x90a\x07wV[\x90RPa\x05\xEEV[a\x05\xD9\x81``\x01Qa\x05\xD3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xA0\x01Qa\x05\xF8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x063V[\x81`@\x01\x81\x81Qa\x05\xEA\x91\x90a\x07wV[\x90RP[`@\x01Q\x92\x91PPV[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x06\x19W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x06SW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x06\xABW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\x8DV[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x06\xC6W__\xFD[P5\x91\x90PV[_``\x82\x84\x03\x12\x80\x15a\x06\xDEW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x0EWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07]Wa\x07]a\x072V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07]Wa\x07]a\x072V[\x81\x81\x03\x81\x81\x11\x15a\x07]Wa\x07]a\x072V\xFE\xA2dipfsX\"\x12 \x04\xAB\x95\xBCP\x8E\x1C\xE5\xC2\xF2\xE1\xB9c\x16'\xB9\xDE'Y\0\xC5\xDE\xC3\x8B\x95^\xE7\x1A\xDA\xA5\x81\xB6dsolcC\0\x08\x1C\x003",
    );
    /**```solidity
struct InterestRateStrategyParams { uint256 optimalUsageRatio; uint256 rate0; uint256[] rateSlope1; uint256[] rateSlope2; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InterestRateStrategyParams {
        pub optimalUsageRatio: alloy::sol_types::private::primitives::aliases::U256,
        pub rate0: alloy::sol_types::private::primitives::aliases::U256,
        pub rateSlope1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub rateSlope2: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InterestRateStrategyParams>
        for UnderlyingRustTuple<'_> {
            fn from(value: InterestRateStrategyParams) -> Self {
                (
                    value.optimalUsageRatio,
                    value.rate0,
                    value.rateSlope1,
                    value.rateSlope2,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InterestRateStrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    optimalUsageRatio: tuple.0,
                    rate0: tuple.1,
                    rateSlope1: tuple.2,
                    rateSlope2: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for InterestRateStrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for InterestRateStrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.optimalUsageRatio),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.rate0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.rateSlope1),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.rateSlope2),
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
        impl alloy_sol_types::SolType for InterestRateStrategyParams {
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
        impl alloy_sol_types::SolStruct for InterestRateStrategyParams {
            const NAME: &'static str = "InterestRateStrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "InterestRateStrategyParams(uint256 optimalUsageRatio,uint256 rate0,uint256[] rateSlope1,uint256[] rateSlope2)",
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
                            &self.optimalUsageRatio,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.rate0)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.rateSlope1)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.rateSlope2)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for InterestRateStrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.optimalUsageRatio,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.rate0)
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rateSlope1,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rateSlope2,
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
                    &rust.optimalUsageRatio,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rate0,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rateSlope1,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rateSlope2,
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
    /**Custom error with signature `InvalidOptimalUsageRate(uint256)` and selector `0xb90df3f2`.
```solidity
error InvalidOptimalUsageRate(uint256 optimalUsageRatio);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOptimalUsageRate {
        pub optimalUsageRatio: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidOptimalUsageRate> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOptimalUsageRate) -> Self {
                (value.optimalUsageRatio,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOptimalUsageRate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { optimalUsageRatio: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOptimalUsageRate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOptimalUsageRate(uint256)";
            const SELECTOR: [u8; 4] = [185u8, 13u8, 243u8, 242u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.optimalUsageRatio),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(InterestRateStrategyParams params);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub params: <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (InterestRateStrategyParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
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
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (InterestRateStrategyParams,);
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
                    <InterestRateStrategyParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
        }
    };
    /**Function with signature `BALANCE10000()` and selector `0x7aa78615`.
```solidity
function BALANCE10000() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE10000Call {}
    ///Container type for the return parameters of the [`BALANCE10000()`](BALANCE10000Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE10000Return {
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
            impl ::core::convert::From<BALANCE10000Call> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE10000Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE10000Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<BALANCE10000Return> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE10000Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE10000Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BALANCE10000Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BALANCE10000Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BALANCE10000()";
            const SELECTOR: [u8; 4] = [122u8, 167u8, 134u8, 21u8];
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
    /**Function with signature `BALANCE1000000()` and selector `0xd41a41ba`.
```solidity
function BALANCE1000000() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE1000000Call {}
    ///Container type for the return parameters of the [`BALANCE1000000()`](BALANCE1000000Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE1000000Return {
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
            impl ::core::convert::From<BALANCE1000000Call> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE1000000Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE1000000Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<BALANCE1000000Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE1000000Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for BALANCE1000000Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BALANCE1000000Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BALANCE1000000Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BALANCE1000000()";
            const SELECTOR: [u8; 4] = [212u8, 26u8, 65u8, 186u8];
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
    /**Function with signature `BALANCE50000()` and selector `0xa3a9bc33`.
```solidity
function BALANCE50000() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE50000Call {}
    ///Container type for the return parameters of the [`BALANCE50000()`](BALANCE50000Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE50000Return {
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
            impl ::core::convert::From<BALANCE50000Call> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE50000Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE50000Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<BALANCE50000Return> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE50000Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE50000Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BALANCE50000Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BALANCE50000Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BALANCE50000()";
            const SELECTOR: [u8; 4] = [163u8, 169u8, 188u8, 51u8];
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
    /**Function with signature `BALANCE500000()` and selector `0xed719cf8`.
```solidity
function BALANCE500000() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE500000Call {}
    ///Container type for the return parameters of the [`BALANCE500000()`](BALANCE500000Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BALANCE500000Return {
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
            impl ::core::convert::From<BALANCE500000Call> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE500000Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE500000Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<BALANCE500000Return> for UnderlyingRustTuple<'_> {
                fn from(value: BALANCE500000Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BALANCE500000Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BALANCE500000Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = BALANCE500000Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BALANCE500000()";
            const SELECTOR: [u8; 4] = [237u8, 113u8, 156u8, 248u8];
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
    /**Function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`.
```solidity
function MAX_EXCESS_USAGE_RATIO() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_EXCESS_USAGE_RATIOCall {}
    ///Container type for the return parameters of the [`MAX_EXCESS_USAGE_RATIO()`](MAX_EXCESS_USAGE_RATIOCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_EXCESS_USAGE_RATIOReturn {
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
            impl ::core::convert::From<MAX_EXCESS_USAGE_RATIOCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_EXCESS_USAGE_RATIOCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_EXCESS_USAGE_RATIOCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<MAX_EXCESS_USAGE_RATIOReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_EXCESS_USAGE_RATIOReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_EXCESS_USAGE_RATIOReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_EXCESS_USAGE_RATIOCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_EXCESS_USAGE_RATIOReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_EXCESS_USAGE_RATIO()";
            const SELECTOR: [u8; 4] = [169u8, 198u8, 34u8, 248u8];
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
    /**Function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`.
```solidity
function OPTIMAL_USAGE_RATIO() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMAL_USAGE_RATIOCall {}
    ///Container type for the return parameters of the [`OPTIMAL_USAGE_RATIO()`](OPTIMAL_USAGE_RATIOCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPTIMAL_USAGE_RATIOReturn {
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
            impl ::core::convert::From<OPTIMAL_USAGE_RATIOCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMAL_USAGE_RATIOCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMAL_USAGE_RATIOCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<OPTIMAL_USAGE_RATIOReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPTIMAL_USAGE_RATIOReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPTIMAL_USAGE_RATIOReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPTIMAL_USAGE_RATIOCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPTIMAL_USAGE_RATIOReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPTIMAL_USAGE_RATIO()";
            const SELECTOR: [u8; 4] = [84u8, 195u8, 101u8, 198u8];
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
    /**Function with signature `SLOPESIZE()` and selector `0xb1b06485`.
```solidity
function SLOPESIZE() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLOPESIZECall {}
    ///Container type for the return parameters of the [`SLOPESIZE()`](SLOPESIZECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLOPESIZEReturn {
        pub _0: u8,
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
            impl ::core::convert::From<SLOPESIZECall> for UnderlyingRustTuple<'_> {
                fn from(value: SLOPESIZECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLOPESIZECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLOPESIZEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SLOPESIZEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLOPESIZEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLOPESIZECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLOPESIZEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLOPESIZE()";
            const SELECTOR: [u8; 4] = [177u8, 176u8, 100u8, 133u8];
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
    /**Function with signature `calculateInterestRates((uint256,uint256,uint256))` and selector `0xfdd63ecf`.
```solidity
function calculateInterestRates(InterestUtils.CalculateInterestRatesParams memory params) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateInterestRatesCall {
        pub params: <InterestUtils::CalculateInterestRatesParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateInterestRates((uint256,uint256,uint256))`](calculateInterestRatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateInterestRatesReturn {
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
            type UnderlyingSolTuple<'a> = (InterestUtils::CalculateInterestRatesParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <InterestUtils::CalculateInterestRatesParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateInterestRatesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateInterestRatesCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateInterestRatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<calculateInterestRatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateInterestRatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateInterestRatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateInterestRatesCall {
            type Parameters<'a> = (InterestUtils::CalculateInterestRatesParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateInterestRatesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateInterestRates((uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [253u8, 214u8, 62u8, 207u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <InterestUtils::CalculateInterestRatesParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    /**Function with signature `getOptimalUsageRatio()` and selector `0xa8602e86`.
```solidity
function getOptimalUsageRatio() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOptimalUsageRatioCall {}
    ///Container type for the return parameters of the [`getOptimalUsageRatio()`](getOptimalUsageRatioCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOptimalUsageRatioReturn {
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
            impl ::core::convert::From<getOptimalUsageRatioCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOptimalUsageRatioCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOptimalUsageRatioCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getOptimalUsageRatioReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOptimalUsageRatioReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOptimalUsageRatioReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOptimalUsageRatioCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOptimalUsageRatioReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOptimalUsageRatio()";
            const SELECTOR: [u8; 4] = [168u8, 96u8, 46u8, 134u8];
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
    /**Function with signature `getRateSlope1()` and selector `0x2dd9037b`.
```solidity
function getRateSlope1() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRateSlope1Call {}
    ///Container type for the return parameters of the [`getRateSlope1()`](getRateSlope1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRateSlope1Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRateSlope1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getRateSlope1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRateSlope1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRateSlope1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getRateSlope1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRateSlope1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRateSlope1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRateSlope1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRateSlope1()";
            const SELECTOR: [u8; 4] = [45u8, 217u8, 3u8, 123u8];
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
    /**Function with signature `getRateSlope2()` and selector `0x75005bb0`.
```solidity
function getRateSlope2() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRateSlope2Call {}
    ///Container type for the return parameters of the [`getRateSlope2()`](getRateSlope2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRateSlope2Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRateSlope2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getRateSlope2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRateSlope2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getRateSlope2Return> for UnderlyingRustTuple<'_> {
                fn from(value: getRateSlope2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRateSlope2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRateSlope2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRateSlope2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRateSlope2()";
            const SELECTOR: [u8; 4] = [117u8, 0u8, 91u8, 176u8];
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
    /**Function with signature `getRatebase()` and selector `0x87f0409d`.
```solidity
function getRatebase() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRatebaseCall {}
    ///Container type for the return parameters of the [`getRatebase()`](getRatebaseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRatebaseReturn {
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
            impl ::core::convert::From<getRatebaseCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRatebaseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRatebaseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getRatebaseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRatebaseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRatebaseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRatebaseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRatebaseReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRatebase()";
            const SELECTOR: [u8; 4] = [135u8, 240u8, 64u8, 157u8];
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
    /**Function with signature `getSlopeIndex(uint256)` and selector `0x8567992a`.
```solidity
function getSlopeIndex(uint256 balance) external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlopeIndexCall {
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getSlopeIndex(uint256)`](getSlopeIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlopeIndexReturn {
        pub _0: u8,
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
            impl ::core::convert::From<getSlopeIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlopeIndexCall) -> Self {
                    (value.balance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlopeIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { balance: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getSlopeIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlopeIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlopeIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlopeIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlopeIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlopeIndex(uint256)";
            const SELECTOR: [u8; 4] = [133u8, 103u8, 153u8, 42u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.balance),
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
    ///Container for all the [`PoolInterestRateStrategy`](self) function calls.
    pub enum PoolInterestRateStrategyCalls {
        BALANCE10000(BALANCE10000Call),
        BALANCE1000000(BALANCE1000000Call),
        BALANCE50000(BALANCE50000Call),
        BALANCE500000(BALANCE500000Call),
        MAX_EXCESS_USAGE_RATIO(MAX_EXCESS_USAGE_RATIOCall),
        OPTIMAL_USAGE_RATIO(OPTIMAL_USAGE_RATIOCall),
        SLOPESIZE(SLOPESIZECall),
        calculateInterestRates(calculateInterestRatesCall),
        getOptimalUsageRatio(getOptimalUsageRatioCall),
        getRateSlope1(getRateSlope1Call),
        getRateSlope2(getRateSlope2Call),
        getRatebase(getRatebaseCall),
        getSlopeIndex(getSlopeIndexCall),
    }
    #[automatically_derived]
    impl PoolInterestRateStrategyCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [45u8, 217u8, 3u8, 123u8],
            [84u8, 195u8, 101u8, 198u8],
            [117u8, 0u8, 91u8, 176u8],
            [122u8, 167u8, 134u8, 21u8],
            [133u8, 103u8, 153u8, 42u8],
            [135u8, 240u8, 64u8, 157u8],
            [163u8, 169u8, 188u8, 51u8],
            [168u8, 96u8, 46u8, 134u8],
            [169u8, 198u8, 34u8, 248u8],
            [177u8, 176u8, 100u8, 133u8],
            [212u8, 26u8, 65u8, 186u8],
            [237u8, 113u8, 156u8, 248u8],
            [253u8, 214u8, 62u8, 207u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolInterestRateStrategyCalls {
        const NAME: &'static str = "PoolInterestRateStrategyCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BALANCE10000(_) => {
                    <BALANCE10000Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::BALANCE1000000(_) => {
                    <BALANCE1000000Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::BALANCE50000(_) => {
                    <BALANCE50000Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::BALANCE500000(_) => {
                    <BALANCE500000Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_EXCESS_USAGE_RATIO(_) => {
                    <MAX_EXCESS_USAGE_RATIOCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPTIMAL_USAGE_RATIO(_) => {
                    <OPTIMAL_USAGE_RATIOCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::SLOPESIZE(_) => {
                    <SLOPESIZECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateInterestRates(_) => {
                    <calculateInterestRatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOptimalUsageRatio(_) => {
                    <getOptimalUsageRatioCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRateSlope1(_) => {
                    <getRateSlope1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRateSlope2(_) => {
                    <getRateSlope2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRatebase(_) => {
                    <getRatebaseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlopeIndex(_) => {
                    <getSlopeIndexCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls>] = &[
                {
                    fn getRateSlope1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <getRateSlope1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::getRateSlope1)
                    }
                    getRateSlope1
                },
                {
                    fn OPTIMAL_USAGE_RATIO(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <OPTIMAL_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::OPTIMAL_USAGE_RATIO)
                    }
                    OPTIMAL_USAGE_RATIO
                },
                {
                    fn getRateSlope2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <getRateSlope2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::getRateSlope2)
                    }
                    getRateSlope2
                },
                {
                    fn BALANCE10000(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <BALANCE10000Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::BALANCE10000)
                    }
                    BALANCE10000
                },
                {
                    fn getSlopeIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <getSlopeIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::getSlopeIndex)
                    }
                    getSlopeIndex
                },
                {
                    fn getRatebase(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <getRatebaseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::getRatebase)
                    }
                    getRatebase
                },
                {
                    fn BALANCE50000(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <BALANCE50000Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::BALANCE50000)
                    }
                    BALANCE50000
                },
                {
                    fn getOptimalUsageRatio(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <getOptimalUsageRatioCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::getOptimalUsageRatio)
                    }
                    getOptimalUsageRatio
                },
                {
                    fn MAX_EXCESS_USAGE_RATIO(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <MAX_EXCESS_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::MAX_EXCESS_USAGE_RATIO)
                    }
                    MAX_EXCESS_USAGE_RATIO
                },
                {
                    fn SLOPESIZE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <SLOPESIZECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::SLOPESIZE)
                    }
                    SLOPESIZE
                },
                {
                    fn BALANCE1000000(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <BALANCE1000000Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::BALANCE1000000)
                    }
                    BALANCE1000000
                },
                {
                    fn BALANCE500000(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <BALANCE500000Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::BALANCE500000)
                    }
                    BALANCE500000
                },
                {
                    fn calculateInterestRates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyCalls> {
                        <calculateInterestRatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyCalls::calculateInterestRates)
                    }
                    calculateInterestRates
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
                Self::BALANCE10000(inner) => {
                    <BALANCE10000Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BALANCE1000000(inner) => {
                    <BALANCE1000000Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BALANCE50000(inner) => {
                    <BALANCE50000Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BALANCE500000(inner) => {
                    <BALANCE500000Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_EXCESS_USAGE_RATIO(inner) => {
                    <MAX_EXCESS_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPTIMAL_USAGE_RATIO(inner) => {
                    <OPTIMAL_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SLOPESIZE(inner) => {
                    <SLOPESIZECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::calculateInterestRates(inner) => {
                    <calculateInterestRatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOptimalUsageRatio(inner) => {
                    <getOptimalUsageRatioCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRateSlope1(inner) => {
                    <getRateSlope1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRateSlope2(inner) => {
                    <getRateSlope2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRatebase(inner) => {
                    <getRatebaseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlopeIndex(inner) => {
                    <getSlopeIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BALANCE10000(inner) => {
                    <BALANCE10000Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BALANCE1000000(inner) => {
                    <BALANCE1000000Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BALANCE50000(inner) => {
                    <BALANCE50000Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BALANCE500000(inner) => {
                    <BALANCE500000Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_EXCESS_USAGE_RATIO(inner) => {
                    <MAX_EXCESS_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPTIMAL_USAGE_RATIO(inner) => {
                    <OPTIMAL_USAGE_RATIOCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SLOPESIZE(inner) => {
                    <SLOPESIZECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateInterestRates(inner) => {
                    <calculateInterestRatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOptimalUsageRatio(inner) => {
                    <getOptimalUsageRatioCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRateSlope1(inner) => {
                    <getRateSlope1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRateSlope2(inner) => {
                    <getRateSlope2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRatebase(inner) => {
                    <getRatebaseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlopeIndex(inner) => {
                    <getSlopeIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolInterestRateStrategy`](self) custom errors.
    pub enum PoolInterestRateStrategyErrors {
        InvalidOptimalUsageRate(InvalidOptimalUsageRate),
    }
    #[automatically_derived]
    impl PoolInterestRateStrategyErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[185u8, 13u8, 243u8, 242u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolInterestRateStrategyErrors {
        const NAME: &'static str = "PoolInterestRateStrategyErrors";
        const MIN_DATA_LENGTH: usize = 32usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidOptimalUsageRate(_) => {
                    <InvalidOptimalUsageRate as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolInterestRateStrategyErrors>] = &[
                {
                    fn InvalidOptimalUsageRate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolInterestRateStrategyErrors> {
                        <InvalidOptimalUsageRate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolInterestRateStrategyErrors::InvalidOptimalUsageRate)
                    }
                    InvalidOptimalUsageRate
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
                Self::InvalidOptimalUsageRate(inner) => {
                    <InvalidOptimalUsageRate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidOptimalUsageRate(inner) => {
                    <InvalidOptimalUsageRate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolInterestRateStrategy`](self) contract instance.

See the [wrapper's documentation](`PoolInterestRateStrategyInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolInterestRateStrategyInstance<T, P, N> {
        PoolInterestRateStrategyInstance::<T, P, N>::new(address, provider)
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
        params: <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PoolInterestRateStrategyInstance<T, P, N>>,
    > {
        PoolInterestRateStrategyInstance::<T, P, N>::deploy(provider, params)
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
        params: <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        PoolInterestRateStrategyInstance::<T, P, N>::deploy_builder(provider, params)
    }
    /**A [`PoolInterestRateStrategy`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolInterestRateStrategy`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolInterestRateStrategyInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolInterestRateStrategyInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolInterestRateStrategyInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolInterestRateStrategyInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolInterestRateStrategy`](self) contract instance.

See the [wrapper's documentation](`PoolInterestRateStrategyInstance`) for more details.*/
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
            params: <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::Result<PoolInterestRateStrategyInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, params);
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
            params: <InterestRateStrategyParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { params },
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
    impl<T, P: ::core::clone::Clone, N> PoolInterestRateStrategyInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolInterestRateStrategyInstance<T, P, N> {
            PoolInterestRateStrategyInstance {
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
    > PoolInterestRateStrategyInstance<T, P, N> {
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
        ///Creates a new call builder for the [`BALANCE10000`] function.
        pub fn BALANCE10000(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BALANCE10000Call, N> {
            self.call_builder(&BALANCE10000Call {})
        }
        ///Creates a new call builder for the [`BALANCE1000000`] function.
        pub fn BALANCE1000000(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BALANCE1000000Call, N> {
            self.call_builder(&BALANCE1000000Call {})
        }
        ///Creates a new call builder for the [`BALANCE50000`] function.
        pub fn BALANCE50000(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BALANCE50000Call, N> {
            self.call_builder(&BALANCE50000Call {})
        }
        ///Creates a new call builder for the [`BALANCE500000`] function.
        pub fn BALANCE500000(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, BALANCE500000Call, N> {
            self.call_builder(&BALANCE500000Call {})
        }
        ///Creates a new call builder for the [`MAX_EXCESS_USAGE_RATIO`] function.
        pub fn MAX_EXCESS_USAGE_RATIO(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_EXCESS_USAGE_RATIOCall, N> {
            self.call_builder(&MAX_EXCESS_USAGE_RATIOCall {})
        }
        ///Creates a new call builder for the [`OPTIMAL_USAGE_RATIO`] function.
        pub fn OPTIMAL_USAGE_RATIO(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OPTIMAL_USAGE_RATIOCall, N> {
            self.call_builder(&OPTIMAL_USAGE_RATIOCall {})
        }
        ///Creates a new call builder for the [`SLOPESIZE`] function.
        pub fn SLOPESIZE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SLOPESIZECall, N> {
            self.call_builder(&SLOPESIZECall {})
        }
        ///Creates a new call builder for the [`calculateInterestRates`] function.
        pub fn calculateInterestRates(
            &self,
            params: <InterestUtils::CalculateInterestRatesParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateInterestRatesCall, N> {
            self.call_builder(
                &calculateInterestRatesCall {
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`getOptimalUsageRatio`] function.
        pub fn getOptimalUsageRatio(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOptimalUsageRatioCall, N> {
            self.call_builder(&getOptimalUsageRatioCall {})
        }
        ///Creates a new call builder for the [`getRateSlope1`] function.
        pub fn getRateSlope1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRateSlope1Call, N> {
            self.call_builder(&getRateSlope1Call {})
        }
        ///Creates a new call builder for the [`getRateSlope2`] function.
        pub fn getRateSlope2(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRateSlope2Call, N> {
            self.call_builder(&getRateSlope2Call {})
        }
        ///Creates a new call builder for the [`getRatebase`] function.
        pub fn getRatebase(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRatebaseCall, N> {
            self.call_builder(&getRatebaseCall {})
        }
        ///Creates a new call builder for the [`getSlopeIndex`] function.
        pub fn getSlopeIndex(
            &self,
            balance: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlopeIndexCall, N> {
            self.call_builder(&getSlopeIndexCall { balance })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolInterestRateStrategyInstance<T, P, N> {
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
