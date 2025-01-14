///Module containing a contract's types and functions.
/**

```solidity
library BorrowUtils {
    struct BorrowParams { uint256 positionId; uint8 tokenIndex; uint256 borrowAmount; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod BorrowUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct BorrowParams { uint256 positionId; uint8 tokenIndex; uint256 borrowAmount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BorrowParams {
        pub positionId: alloy::sol_types::private::primitives::aliases::U256,
        pub tokenIndex: u8,
        pub borrowAmount: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            u8,
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
        impl ::core::convert::From<BorrowParams> for UnderlyingRustTuple<'_> {
            fn from(value: BorrowParams) -> Self {
                (value.positionId, value.tokenIndex, value.borrowAmount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BorrowParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    positionId: tuple.0,
                    tokenIndex: tuple.1,
                    borrowAmount: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BorrowParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BorrowParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.positionId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.borrowAmount),
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
        impl alloy_sol_types::SolType for BorrowParams {
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
        impl alloy_sol_types::SolStruct for BorrowParams {
            const NAME: &'static str = "BorrowParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BorrowParams(uint256 positionId,uint8 tokenIndex,uint256 borrowAmount)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.positionId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.borrowAmount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BorrowParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.positionId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.borrowAmount,
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
                    &rust.positionId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.borrowAmount,
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
    /**Creates a new wrapper around an on-chain [`BorrowUtils`](self) contract instance.

See the [wrapper's documentation](`BorrowUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BorrowUtilsInstance<T, P, N> {
        BorrowUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`BorrowUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BorrowUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BorrowUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BorrowUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BorrowUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BorrowUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BorrowUtils`](self) contract instance.

See the [wrapper's documentation](`BorrowUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BorrowUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BorrowUtilsInstance<T, P, N> {
            BorrowUtilsInstance {
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
    > BorrowUtilsInstance<T, P, N> {
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
    > BorrowUtilsInstance<T, P, N> {
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
library BorrowUtils {
    struct BorrowParams {
        uint256 positionId;
        uint8 tokenIndex;
        uint256 borrowAmount;
    }
}

interface BorrowHandler {
    error AccountNotMatch(address accountInPosition, address account);
    error EmptyBorrowAmounts();
    error EmptyCollateral();
    error EmptyPool(bytes32 key);
    error EmptyPosition();
    error InsufficientReverveForBorrow(uint256 amountToBorrow, uint256 availableReserve);
    error MarginBelowThreshold(uint256 marginLevel, uint256 marginLevelThreshold);
    error MathOverflowedMulDiv();
    error TokenIndexNotSupport();
    error Unauthorized(address msgSender, string role);
    error liquidityDidNotReachShortThreshord(uint256 threshold, uint256 basePriceReserve);

    constructor(address _roleStore, address _dataStore, address _eventEmitter);

    function dataStore() external view returns (address);
    function eventEmitter() external view returns (address);
    function executeBorrow(address account, BorrowUtils.BorrowParams memory BorrowParams) external;
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
    "name": "executeBorrow",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "BorrowParams",
        "type": "tuple",
        "internalType": "struct BorrowUtils.BorrowParams",
        "components": [
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
          }
        ]
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
    "type": "error",
    "name": "AccountNotMatch",
    "inputs": [
      {
        "name": "accountInPosition",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "EmptyBorrowAmounts",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyCollateral",
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
    "name": "EmptyPosition",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientReverveForBorrow",
    "inputs": [
      {
        "name": "amountToBorrow",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "availableReserve",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MarginBelowThreshold",
    "inputs": [
      {
        "name": "marginLevel",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "marginLevelThreshold",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "TokenIndexNotSupport",
    "inputs": []
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
  },
  {
    "type": "error",
    "name": "liquidityDidNotReachShortThreshord",
    "inputs": [
      {
        "name": "threshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "basePriceReserve",
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
pub mod BorrowHandler {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b50604051616b49380380616b4983398101604081905261002e91610062565b6001600160a01b0391821660805291811660a0521660c0526100ac565b6001600160a01b038116811461005f575f5ffd5b50565b5f5f5f60608486031215610074575f5ffd5b835161007f8161004b565b60208501519093506100908161004b565b60408501519092506100a18161004b565b809150509250925092565b60805160a05160c051616a4c6100fd5f395f818160d201526101a801525f8181605301526103f401525f81816096015281816101790152818161021d0152818161032301526106160152616a4c5ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80634a4a7b041461004e578063660d0d67146100915780639a24a668146100b85780639ff78c30146100cd575b5f5ffd5b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100cb6100c63660046166d1565b6100f4565b005b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100fc61021a565b61016b60405160200161012d906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b8152506103d8565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200160208101906101f1919061670f565b60ff1681526020018360400135815250905061020d8382610486565b50610216610614565b5050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016102599061672f565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161028d91815260200190565b602060405180830381865afa1580156102a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102cc9190616766565b905080156103215760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161035f9061672f565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af11580156103b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102169190616766565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610441573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610465919061677d565b61021657338160405163a35b150b60e01b81526004016103189291906167ca565b6104b46040518060400160405280600d81526020016c65786563757465426f72726f7760981b8152506106cd565b6104bc61655c565b6104ce83835f015184604001516106f1565b6060830152604082018190528251905180515160209190910151516104f4929190610712565b6020830152808252825160408301516060850151608086015161051d9488949390929091610753565b805160408201516060840151608085015161053c93929190805f610840565b610552825f015182606001518360400151610ac4565b6105638260200151825f0151611d6e565b815160208201518251610577929190611d92565b6020828101516040838101515180515193015151908501516060860151608087015186515161060f968a95909490939092909160ff8316600281106105be576105be6167ed565b6020908102919091015160409081015181516080810183528c830180515151850151825280515151840151828601528051518501518501518285015251519093015190910151606083015290612f67565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016106529061672f565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af11580156106a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106ca9190616766565b50565b6106ca60405180604001604052806002815260200161257360f01b81525082613020565b6106f961658e565b5f610705858585613065565b915091505b935093915050565b61071a6165b4565b5f5f6107268585613092565b90505f610733878361313b565b905061073f818361433d565b6107488161436b565b969095509350505050565b5f1960ff831601610768576107688585614417565b82604001516001600160a01b0316866001600160a01b0316146107b75760408084015190516312e38abf60e11b81526001600160a01b0391821660048201529087166024820152604401610318565b805f036107d7576040516379646aaf60e01b815260040160405180910390fd5b6107e08261445d565b5f6107ea86614466565b90505f6107f886858461451c565b935050505080831115610828576040516323caca8160e11b81526004810184905260248101829052604401610318565b6108368787878787886145cb565b5050505050505050565b5f61084a84614846565b90505f84126108b8578551819060ff87166002811061086b5761086b6167ed565b60200201516020018181516108809190616815565b9052508651819060ff87166002811061089b5761089b6167ed565b60200201516060018181516108b09190616815565b905250610919565b8551819060ff8716600281106108d0576108d06167ed565b60200201516020018181516108e59190616828565b9052508651819060ff871660028110610900576109006167ed565b60200201516060018181516109159190616828565b9052505b811561099c5785515f9060ff871660028110610937576109376167ed565b602002015160400151905080885f01518760ff166002811061095b5761095b6167ed565b602002015160a0018181516109709190616828565b90525086515f9060ff88166002811061098b5761098b6167ed565b60200201516040015250610abc9050565b825f036109a95750610abc565b5f6109b384614846565b90505f6109e9895f01518860ff16600281106109d1576109d16167ed565b6020020151602001518361485b90919063ffffffff16565b90505f8512610a57578751819060ff891660028110610a0a57610a0a6167ed565b6020020151604001818151610a1f9190616815565b9052508851819060ff891660028110610a3a57610a3a6167ed565b602002015160a001818151610a4f9190616815565b905250610ab8565b8751819060ff891660028110610a6f57610a6f6167ed565b6020020151604001818151610a849190616828565b9052508851819060ff891660028110610a9f57610a9f6167ed565b602002015160a001818151610ab49190616828565b9052505b5050505b505050505050565b5f839050806001600160a01b031663c80f4c62604051602001610b08906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015610b58575f5ffd5b505af1158015610b6a573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62610b8a8460400151614896565b856040518363ffffffff1660e01b8152600401610bb1929190918252602082015260400190565b5f604051808303815f87803b158015610bc8575f5ffd5b505af1158015610bda573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001610c18906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c48929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401610c89929190918252602082015260400190565b6020604051808303815f875af1158015610ca5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cc99190616766565b50806001600160a01b031663ca446dd984604051602001610d09906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610d39929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352610d84926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015610da0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dc4919061683b565b50806001600160a01b031663ca446dd984604051602001610e04906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610e34929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015610e97573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebb919061683b565b50806001600160a01b031663e2a4853a84604051602001610f009060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001610f30929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015610f8d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fb19190616766565b50806001600160a01b031663e2a4853a84604051602001610ff69060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611026929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611082573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110a69190616766565b50806001600160a01b031663e2a4853a846040516020016110f1906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611121929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561117e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111a29190616766565b50806001600160a01b031663e2a4853a846040516020016111ec906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161121c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611279573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061129d9190616766565b50806001600160a01b031663e2a4853a846040516020016112e9906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611319929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611376573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061139a9190616766565b50806001600160a01b031663e2a4853a846040516020016113e5906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611415929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611472573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114969190616766565b50806001600160a01b031663e2a4853a846040516020016114d5906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611505929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611563573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115879190616766565b50806001600160a01b031663ca446dd9846040516020016115c7906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115f7929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561165d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611681919061683b565b50806001600160a01b031663e2a4853a846040516020016116c69060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016116f6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611755573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117799190616766565b50806001600160a01b031663e2a4853a846040516020016117be9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ee929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561184d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118719190616766565b50806001600160a01b031663e2a4853a846040516020016118bc90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016118ec929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561194c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119709190616766565b50806001600160a01b031663e2a4853a846040516020016119ba90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016119ea929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e9190616766565b50806001600160a01b031663e2a4853a84604051602001611aba90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611aea929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611b4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b6e9190616766565b50806001600160a01b031663e2a4853a84604051602001611bb990602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001611be9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6d9190616766565b50806001600160a01b031663e2a4853a84604051602001611cac906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cdc929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401611d27929190918252602082015260400190565b6020604051808303815f875af1158015611d43573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d679190616766565b5050505050565b611d778161491a565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611dd2906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611e22575f5ffd5b505af1158015611e34573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611e78906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ea8929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f2f919061683b565b50806001600160a01b031663e2a4853a84604051602001611f77906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fa7929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612004573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120289190616766565b50806001600160a01b031663e2a4853a8460405160200161206f906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b6040516020818303038152906040528051906020012060405160200161209f929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156120fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211f9190616766565b50806001600160a01b031663e2a4853a8460405160200161216b906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161219b929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156121f8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061221c9190616766565b50806001600160a01b031663e2a4853a8460405160200161223c90616856565b6040516020818303038152906040528051906020012060405160200161226c929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156122c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122ed9190616766565b50806001600160a01b031663e2a4853a8460405160200161233a906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161236a929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156123c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123eb9190616766565b50806001600160a01b031663e2a4853a84604051602001612434906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001612464929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156124c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124e59190616766565b50806001600160a01b031663ca446dd984604051602001612526906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612556929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156125bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125e0919061683b565b50806001600160a01b031663e2a4853a8460405160200161262890602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001612658929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156126b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126db9190616766565b50806001600160a01b031663e2a4853a8460405160200161272290602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001612752929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156127b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127d59190616766565b50806001600160a01b031663e2a4853a8460405160200161282190602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612851929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128d59190616766565b50806001600160a01b031663e2a4853a846040516020016128f590616897565b60405160208183030381529060405280519060200120604051602001612925929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612985573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129a99190616766565b50806001600160a01b031663e2a4853a846040516020016129f690602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a26929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612a86573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aaa9190616766565b50806001600160a01b031663e2a4853a84604051602001612af390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b23929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612b83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ba79190616766565b50806001600160a01b031663ca446dd984604051602001612be590602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c15929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401612c5f9291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612c7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9f919061683b565b50806001600160a01b031663ca446dd984604051602001612cf1906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612d21929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612d6c926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dac919061683b565b50806001600160a01b031663e2a4853a84604051602001612df3906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e23929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612e64929190918252602082015260400190565b6020604051808303815f875af1158015612e80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea49190616766565b50806001600160a01b031663e2a4853a84604051602001612ef6906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612f26929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401611d27929190918252602082015260400190565b6040805163ea34a57760e01b81526001600160a01b038a81166004830152898116602483015288811660448301526064820188905260ff8716608483015260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a169063ea34a57790610164015f604051808303815f87803b158015612fff575f5ffd5b505af1158015613011573d5f5f3e3d5ffd5b50505050505050505050505050565b61021682826040516024016130369291906168d8565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614afc565b61306d61658e565b5f5f6130798685614b05565b90505f6130868683614b6b565b90506107488782614b84565b5f816001600160a01b0316836001600160a01b0316106130b35781836130b6565b82825b60405191945092506130e3906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b6040516020818303038152906040528051906020012090505b92915050565b6131436165b4565b8261314c6165b4565b816001600160a01b03166391d4403c604051602001613188906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156131dc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613200919061677d565b61320d5791506131359050565b816001600160a01b03166321f8a7218560405160200161324d906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161327d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016132b191815260200190565b602060405180830381865afa1580156132cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132f0919061683b565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161336e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133a291815260200190565b602060405180830381865afa1580156133bd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e19190616766565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613437906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613467929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161349b91815260200190565b602060405180830381865afa1580156134b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134da9190616766565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613535906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613565929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161359991815260200190565b602060405180830381865afa1580156135b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135d89190616766565b815151606001526040516001600160a01b0383169063bd02d0f590869061360190602001616856565b60405160208183030381529060405280519060200120604051602001613631929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161366591815260200190565b602060405180830381865afa158015613680573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136a49190616766565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001613700906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001613730929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161376491815260200190565b602060405180830381865afa15801561377f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a39190616766565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613820929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161385491815260200190565b602060405180830381865afa15801561386f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138939190616766565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613908929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161393c91815260200190565b602060405180830381865afa158015613957573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061397b919061683b565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a2591815260200190565b602060405180830381865afa158015613a40573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a649190616766565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001613abb90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001613aeb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b1f91815260200190565b602060405180830381865afa158015613b3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b5e9190616766565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001613bba90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613bea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c1e91815260200190565b602060405180830381865afa158015613c39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c5d9190616766565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001613c8d90616897565b60405160208183030381529060405280519060200120604051602001613cbd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613cf191815260200190565b602060405180830381865afa158015613d0c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d309190616766565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001613d8d90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613dbd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613df191815260200190565b602060405180830381865afa158015613e0c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e309190616766565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001613e8990602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613eb9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613eed91815260200190565b602060405180830381865afa158015613f08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f2c9190616766565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001613f7a90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613faa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613fde91815260200190565b602060405180830381865afa158015613ff9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061401d919061683b565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a7218560405160200161408b906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016140bb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140ef91815260200190565b602060405180830381865afa15801561410a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061412e919061683b565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001614191906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016141c1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016141f591815260200190565b602060405180830381865afa158015614210573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142349190616766565b60608201526040516001600160a01b0383169063bd02d0f590869061428d906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016142bd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016142f191815260200190565b602060405180830381865afa15801561430c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143309190616766565b6080820152949350505050565b60208201516001600160a01b031661021657604051637357d91f60e01b815260048101829052602401610318565b6080810151429081900361437d575050565b81515160a00151156143d35781515f906143a690825b6020020151604001518460800151614bfe565b83519091506143ca9082905f5b602002015160200151614c3a90919063ffffffff16565b83515160200152505b81516020015160a001511561440f5781515f906143f1906001614393565b835190915061440390829060016143b3565b83516020908101510152505b608090910152565b6060810151660800000000000016610216575f5f6144358484614c7b565b60405163fe0081af60e01b815260048101839052602481018290529193509150604401610318565b6106ca81614cdb565b5f816001600160a01b031663bd02d0f56040516020016144a9906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144dd91815260200190565b602060405180830381865afa1580156144f8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131359190616766565b5f5f5f5f5f875f01518760ff1660028110614539576145396167ed565b602002015190505f61454b8989614d0f565b9050805f03614567575f5f5f5f955095509550955050506145c2565b5f614576838b60800151614de1565b90505f614583838a614c3a565b90505f891561459b576145968284614e11565b61459d565b5f5b90506145a98385616815565b846145b48582616828565b919a50985096509450505050505b93509350935093565b61462f604051806101c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61463886614e66565b80825261464990879087905f614f0f565b6040830152602082018190525f0361467457604051636c53056d60e01b815260040160405180910390fd5b6060860151614683908561507b565b606082015261469183614846565b6080820181905260608201516146bf9190676765c793fa10079d601b1b906146ba90600a6169e0565b6150a9565b60a082015260ff84166001146146d9578060a001516146e9565b805160a08201516146e991614c3a565b60c08201526146f782614846565b60e0820181905260608201516147209190676765c793fa10079d601b1b906146ba90600a6169e0565b61010082015260ff841660011461473c5780610100015161474d565b805161010082015161474d91614c3a565b6101208201525f8313614773578060c00151816020015161476e9190616828565b614787565b8060c0015181602001516147879190616815565b6101408201525f82136147ae5780610120015181604001516147a99190616828565b6147c3565b80610120015181604001516147c39190616815565b61016082018190525f036147d75750610abc565b6101608101516101408201516147ec9161485b565b6101808201526147fb87615168565b6101a08201819052610180820151101561483d576101808101516101a08201516040516382d6353f60e01b815260048101929092526024820152604401610318565b50505050505050565b5f5f82121561485757815f03613135565b5090565b5f8115676765c793fa10079d601b1b6002840419048411171561487c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f6040516020016148d0906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61494f6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614959845f6151b9565b60208301528152606084015161496f905f61507b565b60608201819052815161499491676765c793fa10079d601b1b906146ba90600a6169e0565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614a0f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a339190616766565b6080820152614a438460016151b9565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa158015614ac2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ae69190616766565b60a0820181905260809091015194909350915050565b6106ca816151ff565b5f604051602001614b32906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b038516908201526060810183905260800161311c565b614b7361658e565b614b7d838361521e565b9392505050565b60408101516001600160a01b0316614baf57604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102165760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610318565b5f80614c0a8342616828565b614c1490856169eb565b6301e1338090049050614c3281676765c793fa10079d601b1b616815565b949350505050565b5f81156b019d971e4fe8401e740000001983900484111517614c5a575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f5f614c8c84606001515f61507b565b90505f614c9886616430565b90505f614cbb82614caa85600a6169e0565b676765c793fa10079d601b1b6150a9565b90505f614cc9875f5f61451c565b50939a91995090975050505050505050565b60ff811615801590614cf1575060ff8116600114155b156106ca57604051632813581b60e21b815260040160405180910390fd5b5f5f835f01518360ff1660028110614d2957614d296167ed565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614d82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614da69190616766565b9050805f03614db9575f92505050613135565b606082015160c0830151614dcd8284616828565b614dd79190616828565b9695505050505050565b5f8260a001515f03614df457505f613135565b5f614dff8484616481565b60a0850151909150614c329082614c3a565b5f82614e1d8382616828565b91508111156131355760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610318565b5f5f614e73835f5f61451c565b50505090505f614e858460015f61451c565b5050509050805f03614e9a57505f9392505050565b5f614ea985606001515f61507b565b90505f614ebb8660600151600161507b565b90505f614ed985676765c793fa10079d601b1b6146ba86600a6169e0565b90505f614ef785676765c793fa10079d601b1b6146ba86600a6169e0565b9050614f03828261485b565b98975050505050505050565b825151515f908190819081906001600160a01b03868116911614614fb2575f5f614f3a8a8a5f6164c4565b915091505f614f565f8c6060015161507b90919063ffffffff16565b90505f614f7484676765c793fa10079d601b1b6146ba85600a6169e0565b90505f614f9284676765c793fa10079d601b1b6146ba86600a6169e0565b9050614f9e8288616815565b9650614faa8187616815565b955050505050505b865160200151516001600160a01b0386811691161461506e575f5f614fd98a8a60016164c4565b915091505f614ff660018c6060015161507b90919063ffffffff16565b90505f61501484676765c793fa10079d601b1b6146ba85600a6169e0565b90505f61503284676765c793fa10079d601b1b6146ba86600a6169e0565b90505f61503f838d614c3a565b90505f61504c838e614c3a565b9050615058828a616815565b98506150648189616815565b9750505050505050505b9097909650945050505050565b5f60ff60581b1960585f1960ff85160161509b575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f036150dd578382816150d3576150d3616a02565b0492505050614b7d565b8084116150fd5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f56040516020016144a99060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f5f5f6151e6855f01518560ff16600281106151d7576151d76167ed565b60200201518660800151614de1565b90505f6151f38686614d0f565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b61522661658e565b8261522f61658e565b816001600160a01b03166391d4403c60405160200161526f906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156152c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152e7919061677d565b6152f45791506131359050565b816001600160a01b031663bd02d0f58560405160200161532e906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161535e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161539291815260200190565b602060405180830381865afa1580156153ad573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153d19190616766565b816020018181525050816001600160a01b03166321f8a72185604051602001615419906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615449929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161547d91815260200190565b602060405180830381865afa158015615498573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154bc919061683b565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615518906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615548929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161557c91815260200190565b602060405180830381865afa158015615597573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155bb919061683b565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615636929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161566a91815260200190565b602060405180830381865afa158015615685573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156a99190616766565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156fd9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161572d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161576191815260200190565b602060405180830381865afa15801561577c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157a09190616766565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016157fa906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161582a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161585e91815260200190565b602060405180830381865afa158015615879573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061589d9190616766565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016158f6906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615926929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161595a91815260200190565b602060405180830381865afa158015615975573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159999190616766565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615a19929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a4d91815260200190565b602060405180830381865afa158015615a68573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a8c9190616766565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615ae6906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b4a91815260200190565b602060405180830381865afa158015615b65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b899190616766565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615bfc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c3091815260200190565b602060405180830381865afa158015615c4b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c6f9190616766565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615ce3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d1791815260200190565b602060405180830381865afa158015615d32573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d56919061683b565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615dfd91815260200190565b602060405180830381865afa158015615e18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e3c9190616766565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001615e919060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615ec1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615ef591815260200190565b602060405180830381865afa158015615f10573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615f349190616766565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001615f8f90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615fbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615ff391815260200190565b602060405180830381865afa15801561600e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906160329190616766565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161608c90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016160bc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016160f091815260200190565b602060405180830381865afa15801561610b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061612f9190616766565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161618b90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016161bb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016161ef91815260200190565b602060405180830381865afa15801561620a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061622e9190616766565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161628990602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016162b9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016162ed91815260200190565b602060405180830381865afa158015616308573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061632c9190616766565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f58560405160200161637b906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016163ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016163df91815260200190565b602060405180830381865afa1580156163fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061641e9190616766565b81516020015160e00152949350505050565b5f816001600160a01b031663bd02d0f56040516020016144a99060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f42820361649457506020820151613135565b5f6164a3846040015184614bfe565b90506164bc846020015182614c3a90919063ffffffff16565b915050613135565b5f5f5f845f01518460ff16600281106164df576164df6167ed565b60200201516040015190505f616515875f01518660ff1660028110616506576165066167ed565b60200201518860800151616481565b9050811561652c576165278282614c3a565b61652e565b5f5b865190935060ff861660028110616547576165476167ed565b60200201516020015193505050935093915050565b604051806080016040528061656f6165b4565b81526020015f815260200161658261658e565b81526020015f81525090565b60405180606001604052806165a16165e8565b81525f6020820181905260409091015290565b6040518060a001604052806165c7616656565b81525f60208201819052604082018190526060820181905260809091015290565b60405180604001604052806002905b6166406040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165f75790505090565b60405180604001604052806002905b6166a76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166655790505090565b6001600160a01b03811681146106ca575f5ffd5b5f5f82840360808112156166e3575f5ffd5b83356166ee816166bd565b92506060601f1982011215616701575f5ffd5b506020830190509250929050565b5f6020828403121561671f575f5ffd5b813560ff81168114614b7d575f5ffd5b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215616776575f5ffd5b5051919050565b5f6020828403121561678d575f5ffd5b81518015158114614b7d575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90614c329083018461679c565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082018082111561313557613135616801565b8181038181111561313557613135616801565b5f6020828403121561684b575f5ffd5b8151614b7d816166bd565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6168ea604083018561679c565b82810360208401526168fc818561679c565b95945050505050565b6001815b600184111561070a5780850481111561692457616924616801565b600184161561693257908102905b60019390931c928002616909565b5f8261694e57506001613135565b8161695a57505f613135565b8160018114616970576002811461697a57616996565b6001915050613135565b60ff84111561698b5761698b616801565b50506001821b613135565b5060208310610133831016604e8410600b84101617156169b9575081810a613135565b6169c55f198484616905565b805f19048211156169d8576169d8616801565b029392505050565b5f614b7d8383616940565b808202811582820484141761313557613135616801565b634e487b7160e01b5f52601260045260245ffdfea264697066735822122007fa207f8137c448e8f32c39d1eed36392ae1445e31d3d519f4d37adaee0ca4364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QakI8\x03\x80akI\x839\x81\x01`@\x81\x90Ra\0.\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x91\x81\x16`\xA0R\x16`\xC0Ra\0\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0_W__\xFD[PV[___``\x84\x86\x03\x12\x15a\0tW__\xFD[\x83Qa\0\x7F\x81a\0KV[` \x85\x01Q\x90\x93Pa\0\x90\x81a\0KV[`@\x85\x01Q\x90\x92Pa\0\xA1\x81a\0KV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QajLa\0\xFD_9_\x81\x81`\xD2\x01Ra\x01\xA8\x01R_\x81\x81`S\x01Ra\x03\xF4\x01R_\x81\x81`\x96\x01R\x81\x81a\x01y\x01R\x81\x81a\x02\x1D\x01R\x81\x81a\x03#\x01Ra\x06\x16\x01RajL_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80cJJ{\x04\x14a\0NW\x80cf\r\rg\x14a\0\x91W\x80c\x9A$\xA6h\x14a\0\xB8W\x80c\x9F\xF7\x8C0\x14a\0\xCDW[__\xFD[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCBa\0\xC66`\x04af\xD1V[a\0\xF4V[\0[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFCa\x02\x1AV[a\x01k`@Q` \x01a\x01-\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x03\xD8V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x01` \x81\x01\x90a\x01\xF1\x91\x90ag\x0FV[`\xFF\x16\x81R` \x01\x83`@\x015\x81RP\x90Pa\x02\r\x83\x82a\x04\x86V[Pa\x02\x16a\x06\x14V[PPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x02Y\x90ag/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90agfV[\x90P\x80\x15a\x03!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03_\x90ag/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x16\x91\x90agfV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04e\x91\x90ag}V[a\x02\x16W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x03\x18\x92\x91\x90ag\xCAV[a\x04\xB4`@Q\x80`@\x01`@R\x80`\r\x81R` \x01lexecuteBorrow`\x98\x1B\x81RPa\x06\xCDV[a\x04\xBCae\\V[a\x04\xCE\x83\x83_\x01Q\x84`@\x01Qa\x06\xF1V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x04\xF4\x92\x91\x90a\x07\x12V[` \x83\x01R\x80\x82R\x82Q`@\x83\x01Q``\x85\x01Q`\x80\x86\x01Qa\x05\x1D\x94\x88\x94\x93\x90\x92\x90\x91a\x07SV[\x80Q`@\x82\x01Q``\x84\x01Q`\x80\x85\x01Qa\x05<\x93\x92\x91\x90\x80_a\x08@V[a\x05R\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\n\xC4V[a\x05c\x82` \x01Q\x82_\x01Qa\x1DnV[\x81Q` \x82\x01Q\x82Qa\x05w\x92\x91\x90a\x1D\x92V[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80QQ\x93\x01QQ\x90\x85\x01Q``\x86\x01Q`\x80\x87\x01Q\x86QQa\x06\x0F\x96\x8A\x95\x90\x94\x90\x93\x90\x92\x90\x91`\xFF\x83\x16`\x02\x81\x10a\x05\xBEWa\x05\xBEag\xEDV[` \x90\x81\x02\x91\x90\x91\x01Q`@\x90\x81\x01Q\x81Q`\x80\x81\x01\x83R\x8C\x83\x01\x80QQQ\x85\x01Q\x82R\x80QQQ\x84\x01Q\x82\x86\x01R\x80QQ\x85\x01Q\x85\x01Q\x82\x85\x01RQQ\x90\x93\x01Q\x90\x91\x01Q``\x83\x01R\x90a/gV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x06R\x90ag/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCA\x91\x90agfV[PV[a\x06\xCA`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82a0 V[a\x06\xF9ae\x8EV[_a\x07\x05\x85\x85\x85a0eV[\x91P\x91P[\x93P\x93\x91PPV[a\x07\x1Aae\xB4V[__a\x07&\x85\x85a0\x92V[\x90P_a\x073\x87\x83a1;V[\x90Pa\x07?\x81\x83aC=V[a\x07H\x81aCkV[\x96\x90\x95P\x93PPPPV[_\x19`\xFF\x83\x16\x01a\x07hWa\x07h\x85\x85aD\x17V[\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xB7W`@\x80\x84\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x87\x16`$\x82\x01R`D\x01a\x03\x18V[\x80_\x03a\x07\xD7W`@Qcydj\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xE0\x82aD]V[_a\x07\xEA\x86aDfV[\x90P_a\x07\xF8\x86\x85\x84aE\x1CV[\x93PPPP\x80\x83\x11\x15a\x08(W`@Qc#\xCA\xCA\x81`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x03\x18V[a\x086\x87\x87\x87\x87\x87\x88aE\xCBV[PPPPPPPPV[_a\x08J\x84aHFV[\x90P_\x84\x12a\x08\xB8W\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08kWa\x08kag\xEDV[` \x02\x01Q` \x01\x81\x81Qa\x08\x80\x91\x90ah\x15V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08\x9BWa\x08\x9Bag\xEDV[` \x02\x01Q``\x01\x81\x81Qa\x08\xB0\x91\x90ah\x15V[\x90RPa\t\x19V[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08\xD0Wa\x08\xD0ag\xEDV[` \x02\x01Q` \x01\x81\x81Qa\x08\xE5\x91\x90ah(V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\t\0Wa\t\0ag\xEDV[` \x02\x01Q``\x01\x81\x81Qa\t\x15\x91\x90ah(V[\x90RP[\x81\x15a\t\x9CW\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\t7Wa\t7ag\xEDV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\t[Wa\t[ag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\tp\x91\x90ah(V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\t\x8BWa\t\x8Bag\xEDV[` \x02\x01Q`@\x01RPa\n\xBC\x90PV[\x82_\x03a\t\xA9WPa\n\xBCV[_a\t\xB3\x84aHFV[\x90P_a\t\xE9\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\t\xD1Wa\t\xD1ag\xEDV[` \x02\x01Q` \x01Q\x83aH[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\nWW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n\nWa\n\nag\xEDV[` \x02\x01Q`@\x01\x81\x81Qa\n\x1F\x91\x90ah\x15V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n:Wa\n:ag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\nO\x91\x90ah\x15V[\x90RPa\n\xB8V[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\noWa\noag\xEDV[` \x02\x01Q`@\x01\x81\x81Qa\n\x84\x91\x90ah(V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n\x9FWa\n\x9Fag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\n\xB4\x91\x90ah(V[\x90RP[PPP[PPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x0B\x08\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0BXW__\xFD[PZ\xF1\x15\x80\x15a\x0BjW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x0B\x8A\x84`@\x01QaH\x96V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xC8W__\xFD[PZ\xF1\x15\x80\x15a\x0B\xDAW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0C\x18\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0CH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xA5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC9\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\r\t\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\r\x84\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC4\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x0E\x04\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBB\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0F\0\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB1\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0F\xF6\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA6\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x10\xF1\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA2\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x11\xEC\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x9D\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x12\xE9\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13vW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9A\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x13\xE5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x96\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\xD5\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x87\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x15\xC7\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x81\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xC6\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17y\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xBE\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xEE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18q\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xBC\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19p\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xBA\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bn\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xB9\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cm\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xAC\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dg\x91\x90agfV[PPPPPV[a\x1Dw\x81aI\x1AV[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1D\xD2\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\"W__\xFD[PZ\xF1\x15\x80\x15a\x1E4W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1Ex\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F/\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1Fw\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a (\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a o\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1F\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!k\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\x9B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x1C\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"<\x90ahVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"l\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xED\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#:\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#j\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xEB\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$4\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xE5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a%&\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%V\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xE0\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&(\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&X\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xDB\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'\"\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'R\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xD5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(!\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(Q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xD5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(\xF5\x90ah\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xA9\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)\xF6\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xAA\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*\xF3\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA7\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a+\xE5\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,_\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9F\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a,\xF1\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra-l\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xAC\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-\xF3\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA4\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.\xF6\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Qc\xEA4\xA5w`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\xFF\x87\x16`\x84\x83\x01R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\xEA4\xA5w\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xFFW__\xFD[PZ\xF1\x15\x80\x15a0\x11W=__>=_\xFD[PPPPPPPPPPPPPV[a\x02\x16\x82\x82`@Q`$\x01a06\x92\x91\x90ah\xD8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaJ\xFCV[a0mae\x8EV[__a0y\x86\x85aK\x05V[\x90P_a0\x86\x86\x83aKkV[\x90Pa\x07H\x87\x82aK\x84V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a0\xB3W\x81\x83a0\xB6V[\x82\x82[`@Q\x91\x94P\x92Pa0\xE3\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a1Cae\xB4V[\x82a1Lae\xB4V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a1\x88\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\0\x91\x90ag}V[a2\rW\x91Pa15\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a2M\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2}\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xF0\x91\x90ah;V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE1\x91\x90agfV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a47\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4g\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x9B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xDA\x91\x90agfV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a55\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5e\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xD8\x91\x90agfV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a6\x01\x90` \x01ahVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a61\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6e\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xA4\x91\x90agfV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a7\0\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a70\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7d\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA3\x91\x90agfV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8 \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8T\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x93\x91\x90agfV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\x08\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9<\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9WW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9{\x91\x90ah;V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:%\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:d\x91\x90agfV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a:\xBB\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;^\x91\x90agfV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a;\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<]\x91\x90agfV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a<\x8D\x90ah\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=0\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a=\x8D\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>0\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\x89\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?,\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a?z\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x1D\x91\x90ah;V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a@\x8B\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xBB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA.\x91\x90ah;V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aA\x91\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB4\x91\x90agfV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aB\x8D\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC0\x91\x90agfV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02\x16W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x18V[`\x80\x81\x01QB\x90\x81\x90\x03aC}WPPV[\x81QQ`\xA0\x01Q\x15aC\xD3W\x81Q_\x90aC\xA6\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaK\xFEV[\x83Q\x90\x91PaC\xCA\x90\x82\x90_[` \x02\x01Q` \x01QaL:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aD\x0FW\x81Q_\x90aC\xF1\x90`\x01aC\x93V[\x83Q\x90\x91PaD\x03\x90\x82\x90`\x01aC\xB3V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02\x16W__aD5\x84\x84aL{V[`@Qc\xFE\0\x81\xAF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R\x91\x93P\x91P`D\x01a\x03\x18V[a\x06\xCA\x81aL\xDBV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xDD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a15\x91\x90agfV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10aE9WaE9ag\xEDV[` \x02\x01Q\x90P_aEK\x89\x89aM\x0FV[\x90P\x80_\x03aEgW____\x95P\x95P\x95P\x95PPPaE\xC2V[_aEv\x83\x8B`\x80\x01QaM\xE1V[\x90P_aE\x83\x83\x8AaL:V[\x90P_\x89\x15aE\x9BWaE\x96\x82\x84aN\x11V[aE\x9DV[_[\x90PaE\xA9\x83\x85ah\x15V[\x84aE\xB4\x85\x82ah(V[\x91\x9AP\x98P\x96P\x94PPPPP[\x93P\x93P\x93P\x93V[aF/`@Q\x80a\x01\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aF8\x86aNfV[\x80\x82RaFI\x90\x87\x90\x87\x90_aO\x0FV[`@\x83\x01R` \x82\x01\x81\x90R_\x03aFtW`@QclS\x05m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x86\x01QaF\x83\x90\x85aP{V[``\x82\x01RaF\x91\x83aHFV[`\x80\x82\x01\x81\x90R``\x82\x01QaF\xBF\x91\x90gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[aP\xA9V[`\xA0\x82\x01R`\xFF\x84\x16`\x01\x14aF\xD9W\x80`\xA0\x01QaF\xE9V[\x80Q`\xA0\x82\x01QaF\xE9\x91aL:V[`\xC0\x82\x01RaF\xF7\x82aHFV[`\xE0\x82\x01\x81\x90R``\x82\x01QaG \x91\x90gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[a\x01\0\x82\x01R`\xFF\x84\x16`\x01\x14aG<W\x80a\x01\0\x01QaGMV[\x80Qa\x01\0\x82\x01QaGM\x91aL:V[a\x01 \x82\x01R_\x83\x13aGsW\x80`\xC0\x01Q\x81` \x01QaGn\x91\x90ah(V[aG\x87V[\x80`\xC0\x01Q\x81` \x01QaG\x87\x91\x90ah\x15V[a\x01@\x82\x01R_\x82\x13aG\xAEW\x80a\x01 \x01Q\x81`@\x01QaG\xA9\x91\x90ah(V[aG\xC3V[\x80a\x01 \x01Q\x81`@\x01QaG\xC3\x91\x90ah\x15V[a\x01`\x82\x01\x81\x90R_\x03aG\xD7WPa\n\xBCV[a\x01`\x81\x01Qa\x01@\x82\x01QaG\xEC\x91aH[V[a\x01\x80\x82\x01RaG\xFB\x87aQhV[a\x01\xA0\x82\x01\x81\x90Ra\x01\x80\x82\x01Q\x10\x15aH=Wa\x01\x80\x81\x01Qa\x01\xA0\x82\x01Q`@Qc\x82\xD65?`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x03\x18V[PPPPPPPV[__\x82\x12\x15aHWW\x81_\x03a15V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aH|W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_`@Q` \x01aH\xD0\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__aIO`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aIY\x84_aQ\xB9V[` \x83\x01R\x81R``\x84\x01QaIo\x90_aP{V[``\x82\x01\x81\x90R\x81QaI\x94\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ3\x91\x90agfV[`\x80\x82\x01RaJC\x84`\x01aQ\xB9V[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xE6\x91\x90agfV[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[a\x06\xCA\x81aQ\xFFV[_`@Q` \x01aK2\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01a1\x1CV[aKsae\x8EV[aK}\x83\x83aR\x1EV[\x93\x92PPPV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aK\xAFW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02\x16W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x03\x18V[_\x80aL\n\x83Bah(V[aL\x14\x90\x85ai\xEBV[c\x01\xE13\x80\x90\x04\x90PaL2\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bah\x15V[\x94\x93PPPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aLZW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[___aL\x8C\x84``\x01Q_aP{V[\x90P_aL\x98\x86ad0V[\x90P_aL\xBB\x82aL\xAA\x85`\nai\xE0V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xA9V[\x90P_aL\xC9\x87__aE\x1CV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[`\xFF\x81\x16\x15\x80\x15\x90aL\xF1WP`\xFF\x81\x16`\x01\x14\x15[\x15a\x06\xCAW`@Qc(\x13X\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aM)WaM)ag\xEDV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xA6\x91\x90agfV[\x90P\x80_\x03aM\xB9W_\x92PPPa15V[``\x82\x01Q`\xC0\x83\x01QaM\xCD\x82\x84ah(V[aM\xD7\x91\x90ah(V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aM\xF4WP_a15V[_aM\xFF\x84\x84ad\x81V[`\xA0\x85\x01Q\x90\x91PaL2\x90\x82aL:V[_\x82aN\x1D\x83\x82ah(V[\x91P\x81\x11\x15a15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x03\x18V[__aNs\x83__aE\x1CV[PPP\x90P_aN\x85\x84`\x01_aE\x1CV[PPP\x90P\x80_\x03aN\x9AWP_\x93\x92PPPV[_aN\xA9\x85``\x01Q_aP{V[\x90P_aN\xBB\x86``\x01Q`\x01aP{V[\x90P_aN\xD9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90P_aN\xF7\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90PaO\x03\x82\x82aH[V[\x98\x97PPPPPPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aO\xB2W__aO:\x8A\x8A_ad\xC4V[\x91P\x91P_aOV_\x8C``\x01QaP{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aOt\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x85`\nai\xE0V[\x90P_aO\x92\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90PaO\x9E\x82\x88ah\x15V[\x96PaO\xAA\x81\x87ah\x15V[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aPnW__aO\xD9\x8A\x8A`\x01ad\xC4V[\x91P\x91P_aO\xF6`\x01\x8C``\x01QaP{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aP\x14\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x85`\nai\xE0V[\x90P_aP2\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90P_aP?\x83\x8DaL:V[\x90P_aPL\x83\x8EaL:V[\x90PaPX\x82\x8Aah\x15V[\x98PaPd\x81\x89ah\x15V[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aP\x9BWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aP\xDDW\x83\x82\x81aP\xD3WaP\xD3aj\x02V[\x04\x92PPPaK}V[\x80\x84\x11aP\xFDW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aQ\xE6\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aQ\xD7WaQ\xD7ag\xEDV[` \x02\x01Q\x86`\x80\x01QaM\xE1V[\x90P_aQ\xF3\x86\x86aM\x0FV[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[aR&ae\x8EV[\x82aR/ae\x8EV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aRo\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\xE7\x91\x90ag}V[aR\xF4W\x91Pa15\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aS.\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS^\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\x92\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xD1\x91\x90agfV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aT\x19\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTI\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT}\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xBC\x91\x90ah;V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aU\x18\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aUH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aU|\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xBB\x91\x90ah;V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aV\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\xA9\x91\x90agfV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xFD\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWa\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xA0\x91\x90agfV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xFA\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aXyW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x9D\x91\x90agfV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\xF6\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYZ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYuW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\x99\x91\x90agfV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZM\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\x8C\x91\x90agfV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\xE6\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\x89\x91\x90agfV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\o\x91\x90agfV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\xE3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\x17\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]V\x91\x90ah;V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^<\x91\x90agfV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a^\x91\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a^\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a^\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_4\x91\x90agfV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a_\x8F\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a_\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`2\x91\x90agfV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a`\x8C\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a`\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a`\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aa\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aa/\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aa\x8B\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa\xBB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aa\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ab\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ab.\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ab\x89\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ab\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ac\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ac,\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ac{\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ac\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ac\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ac\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ad\x1E\x91\x90agfV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_B\x82\x03ad\x94WP` \x82\x01Qa15V[_ad\xA3\x84`@\x01Q\x84aK\xFEV[\x90Pad\xBC\x84` \x01Q\x82aL:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa15V[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10ad\xDFWad\xDFag\xEDV[` \x02\x01Q`@\x01Q\x90P_ae\x15\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10ae\x06Wae\x06ag\xEDV[` \x02\x01Q\x88`\x80\x01Qad\x81V[\x90P\x81\x15ae,Wae'\x82\x82aL:V[ae.V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aeGWaeGag\xEDV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[`@Q\x80`\x80\x01`@R\x80aeoae\xB4V[\x81R` \x01_\x81R` \x01ae\x82ae\x8EV[\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ae\xA1ae\xE8V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80ae\xC7afVV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af@`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xF7W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xA7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81afeW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xCAW__\xFD[__\x82\x84\x03`\x80\x81\x12\x15af\xE3W__\xFD[\x835af\xEE\x81af\xBDV[\x92P```\x1F\x19\x82\x01\x12\x15ag\x01W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15ag\x1FW__\xFD[\x815`\xFF\x81\x16\x81\x14aK}W__\xFD[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15agvW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ag\x8DW__\xFD[\x81Q\x80\x15\x15\x81\x14aK}W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aL2\x90\x83\x01\x84ag\x9CV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a15Wa15ah\x01V[\x81\x81\x03\x81\x81\x11\x15a15Wa15ah\x01V[_` \x82\x84\x03\x12\x15ahKW__\xFD[\x81QaK}\x81af\xBDV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_ah\xEA`@\x83\x01\x85ag\x9CV[\x82\x81\x03` \x84\x01Rah\xFC\x81\x85ag\x9CV[\x95\x94PPPPPV[`\x01\x81[`\x01\x84\x11\x15a\x07\nW\x80\x85\x04\x81\x11\x15ai$Wai$ah\x01V[`\x01\x84\x16\x15ai2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ai\tV[_\x82aiNWP`\x01a15V[\x81aiZWP_a15V[\x81`\x01\x81\x14aipW`\x02\x81\x14aizWai\x96V[`\x01\x91PPa15V[`\xFF\x84\x11\x15ai\x8BWai\x8Bah\x01V[PP`\x01\x82\x1Ba15V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ai\xB9WP\x81\x81\na15V[ai\xC5_\x19\x84\x84ai\x05V[\x80_\x19\x04\x82\x11\x15ai\xD8Wai\xD8ah\x01V[\x02\x93\x92PPPV[_aK}\x83\x83ai@V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a15Wa15ah\x01V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x07\xFA \x7F\x817\xC4H\xE8\xF3,9\xD1\xEE\xD3c\x92\xAE\x14E\xE3\x1D=Q\x9FM7\xAD\xAE\xE0\xCACdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061004a575f3560e01c80634a4a7b041461004e578063660d0d67146100915780639a24a668146100b85780639ff78c30146100cd575b5f5ffd5b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200160405180910390f35b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100cb6100c63660046166d1565b6100f4565b005b6100757f000000000000000000000000000000000000000000000000000000000000000081565b6100fc61021a565b61016b60405160200161012d906020808252600a908201526921a7a72a2927a62622a960b11b604082015260600190565b604051602081830303815290604052805190602001206040518060400160405280600a81526020016921a7a72a2927a62622a960b11b8152506103d8565b5f6040518060a001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602001835f013581526020018360200160208101906101f1919061670f565b60ff1681526020018360400135815250905061020d8382610486565b50610216610614565b5050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bd02d0f56040516020016102599061672f565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161028d91815260200190565b602060405180830381865afa1580156102a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102cc9190616766565b905080156103215760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c0060448201526064015b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a60405160200161035f9061672f565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152600160248201526044016020604051808303815f875af11580156103b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102169190616766565b60405163ac4ab3fb60e01b8152336004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ac4ab3fb90604401602060405180830381865afa158015610441573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610465919061677d565b61021657338160405163a35b150b60e01b81526004016103189291906167ca565b6104b46040518060400160405280600d81526020016c65786563757465426f72726f7760981b8152506106cd565b6104bc61655c565b6104ce83835f015184604001516106f1565b6060830152604082018190528251905180515160209190910151516104f4929190610712565b6020830152808252825160408301516060850151608086015161051d9488949390929091610753565b805160408201516060840151608085015161053c93929190805f610840565b610552825f015182606001518360400151610ac4565b6105638260200151825f0151611d6e565b815160208201518251610577929190611d92565b6020828101516040838101515180515193015151908501516060860151608087015186515161060f968a95909490939092909160ff8316600281106105be576105be6167ed565b6020908102919091015160409081015181516080810183528c830180515151850151825280515151840151828601528051518501518501518285015251519093015190910151606083015290612f67565b505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e2a4853a6040516020016106529061672f565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b16825260048201525f60248201526044016020604051808303815f875af11580156106a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106ca9190616766565b50565b6106ca60405180604001604052806002815260200161257360f01b81525082613020565b6106f961658e565b5f610705858585613065565b915091505b935093915050565b61071a6165b4565b5f5f6107268585613092565b90505f610733878361313b565b905061073f818361433d565b6107488161436b565b969095509350505050565b5f1960ff831601610768576107688585614417565b82604001516001600160a01b0316866001600160a01b0316146107b75760408084015190516312e38abf60e11b81526001600160a01b0391821660048201529087166024820152604401610318565b805f036107d7576040516379646aaf60e01b815260040160405180910390fd5b6107e08261445d565b5f6107ea86614466565b90505f6107f886858461451c565b935050505080831115610828576040516323caca8160e11b81526004810184905260248101829052604401610318565b6108368787878787886145cb565b5050505050505050565b5f61084a84614846565b90505f84126108b8578551819060ff87166002811061086b5761086b6167ed565b60200201516020018181516108809190616815565b9052508651819060ff87166002811061089b5761089b6167ed565b60200201516060018181516108b09190616815565b905250610919565b8551819060ff8716600281106108d0576108d06167ed565b60200201516020018181516108e59190616828565b9052508651819060ff871660028110610900576109006167ed565b60200201516060018181516109159190616828565b9052505b811561099c5785515f9060ff871660028110610937576109376167ed565b602002015160400151905080885f01518760ff166002811061095b5761095b6167ed565b602002015160a0018181516109709190616828565b90525086515f9060ff88166002811061098b5761098b6167ed565b60200201516040015250610abc9050565b825f036109a95750610abc565b5f6109b384614846565b90505f6109e9895f01518860ff16600281106109d1576109d16167ed565b6020020151602001518361485b90919063ffffffff16565b90505f8512610a57578751819060ff891660028110610a0a57610a0a6167ed565b6020020151604001818151610a1f9190616815565b9052508851819060ff891660028110610a3a57610a3a6167ed565b602002015160a001818151610a4f9190616815565b905250610ab8565b8751819060ff891660028110610a6f57610a6f6167ed565b6020020151604001818151610a849190616828565b9052508851819060ff891660028110610a9f57610a9f6167ed565b602002015160a001818151610ab49190616828565b9052505b5050505b505050505050565b5f839050806001600160a01b031663c80f4c62604051602001610b08906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015610b58575f5ffd5b505af1158015610b6a573d5f5f3e3d5ffd5b50505050806001600160a01b031663c80f4c62610b8a8460400151614896565b856040518363ffffffff1660e01b8152600401610bb1929190918252602082015260400190565b5f604051808303815f87803b158015610bc8575f5ffd5b505af1158015610bda573d5f5f3e3d5ffd5b50505050806001600160a01b031663e2a4853a84604051602001610c18906020808252600690820152651413d4d7d25160d21b604082015260600190565b60405160208183030381529060405280519060200120604051602001610c48929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401610c89929190918252602082015260400190565b6020604051808303815f875af1158015610ca5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cc99190616766565b50806001600160a01b031663ca446dd984604051602001610d09906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610d39929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352610d84926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015610da0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dc4919061683b565b50806001600160a01b031663ca446dd984604051602001610e04906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001610e34929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015610e97573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebb919061683b565b50806001600160a01b031663e2a4853a84604051602001610f009060208082526010908201526f0504f535f434f4c4c41544552414c5f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001610f30929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015610f8d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fb19190616766565b50806001600160a01b031663e2a4853a84604051602001610ff69060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b60405160208183030381529060405280519060200120604051602001611026929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611082573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110a69190616766565b50806001600160a01b031663e2a4853a846040516020016110f1906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611121929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561117e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111a29190616766565b50806001600160a01b031663e2a4853a846040516020016111ec906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161121c929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611279573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061129d9190616766565b50806001600160a01b031663e2a4853a846040516020016112e9906020808252601790820152760504f535f454e5452595f53484f52545f50524943455f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611319929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611376573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061139a9190616766565b50806001600160a01b031663e2a4853a846040516020016113e5906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001611415929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611472573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114969190616766565b50806001600160a01b031663e2a4853a846040516020016114d5906020808252600a90820152690504f535f545950455f360b41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611505929190918252602082015260400190565b60408051601f1981840301815290829052805160209091012085515160e0908101519084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611563573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115879190616766565b50806001600160a01b031663ca446dd9846040516020016115c7906020808252600b908201526a504f535f544f4b454e5f3160a81b604082015260600190565b604051602081830303815290604052805190602001206040516020016115f7929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af115801561165d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611681919061683b565b50806001600160a01b031663e2a4853a846040516020016116c69060208082526010908201526f504f535f434f4c4c41544552414c5f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016116f6929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015611755573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117799190616766565b50806001600160a01b031663e2a4853a846040516020016117be9060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b604051602081830303815290604052805190602001206040516020016117ee929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af115801561184d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118719190616766565b50806001600160a01b031663e2a4853a846040516020016118bc90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016118ec929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af115801561194c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119709190616766565b50806001600160a01b031663e2a4853a846040516020016119ba90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016119ea929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611a4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a6e9190616766565b50806001600160a01b031663e2a4853a84604051602001611aba90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001611aea929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611b4a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b6e9190616766565b50806001600160a01b031663e2a4853a84604051602001611bb990602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001611be9929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015611c49573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6d9190616766565b50806001600160a01b031663e2a4853a84604051602001611cac906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b60405160208183030381529060405280519060200120604051602001611cdc929190918252602082015260400190565b60408051601f19818403018152919052805160209091012084516001602002015160e001516040518363ffffffff1660e01b8152600401611d27929190918252602082015260400190565b6020604051808303815f875af1158015611d43573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d679190616766565b5050505050565b611d778161491a565b91518051602090910151604090810193909352919091015250565b5f839050806001600160a01b031663c80f4c62604051602001611dd2906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b1682526004820152602481018690526044015f604051808303815f87803b158015611e22575f5ffd5b505af1158015611e34573d5f5f3e3d5ffd5b50505050806001600160a01b031663ca446dd984604051602001611e78906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b60405160208183030381529060405280519060200120604051602001611ea8929190918252602082015260400190565b60408051808303601f19018152908290528051602090910120855151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af1158015611f0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f2f919061683b565b50806001600160a01b031663e2a4853a84604051602001611f77906020808252601390820152720504f4f4c5f424f52524f575f494e4445585f3606c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001611fa7929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af1158015612004573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120289190616766565b50806001600160a01b031663e2a4853a8460405160200161206f906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b6040516020818303038152906040528051906020012060405160200161209f929190918252602082015260400190565b60408051808303601f190181528282528051602090910120865151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156120fb573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061211f9190616766565b50806001600160a01b031663e2a4853a8460405160200161216b906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b6040516020818303038152906040528051906020012060405160200161219b929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156121f8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061221c9190616766565b50806001600160a01b031663e2a4853a8460405160200161223c90616856565b6040516020818303038152906040528051906020012060405160200161226c929190918252602082015260400190565b60408051808303601f190181529082905280516020909101208551516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156122c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122ed9190616766565b50806001600160a01b031663e2a4853a8460405160200161233a906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b6040516020818303038152906040528051906020012060405160200161236a929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156123c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123eb9190616766565b50806001600160a01b031663e2a4853a84604051602001612434906020808252601490820152730504f4f4c5f554e434c41494d45445f4645455f360641b604082015260600190565b60405160208183030381529060405280519060200120604051602001612464929190918252602082015260400190565b60408051808303601f1901815290829052805160209091012085515160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156124c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124e59190616766565b50806001600160a01b031663ca446dd984604051602001612526906020808252600c908201526b504f4f4c5f544f4b454e5f3160a01b604082015260600190565b60405160208183030381529060405280519060200120604051602001612556929190918252602082015260400190565b60408051808303601f19018152908290528051602091820120865190910151516001600160e01b031960e085901b16835260048301919091526001600160a01b031660248201526044016020604051808303815f875af11580156125bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125e0919061683b565b50806001600160a01b031663e2a4853a8460405160200161262890602080825260139082015272504f4f4c5f424f52524f575f494e4445585f3160681b604082015260600190565b60405160208183030381529060405280519060200120604051602001612658929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651820151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156126b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126db9190616766565b50806001600160a01b031663e2a4853a8460405160200161272290602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001612752929190918252602082015260400190565b60408051808303601f190181528282528051602091820120875190910151909101516001600160e01b031960e085901b168352600483019190915260248201526044016020604051808303815f875af11580156127b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127d59190616766565b50806001600160a01b031663e2a4853a8460405160200161282190602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001612851929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516060015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af11580156128b1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128d59190616766565b50806001600160a01b031663e2a4853a846040516020016128f590616897565b60405160208183030381529060405280519060200120604051602001612925929190918252602082015260400190565b60408051808303601f190181529082905280516020918201208651909101516080015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612985573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129a99190616766565b50806001600160a01b031663e2a4853a846040516020016129f690602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001612a26929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160a0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612a86573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612aaa9190616766565b50806001600160a01b031663e2a4853a84604051602001612af390602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001612b23929190918252602082015260400190565b60408051808303601f1901815290829052805160209182012086519091015160c0015160e084901b6001600160e01b0319168352600483019190915260248201526044016020604051808303815f875af1158015612b83573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ba79190616766565b50806001600160a01b031663ca446dd984604051602001612be590602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001612c15929190918252602082015260400190565b6040516020818303038152906040528051906020012084602001516040518363ffffffff1660e01b8152600401612c5f9291909182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612c7b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c9f919061683b565b50806001600160a01b031663ca446dd984604051602001612cf1906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612d21929190918252602082015260400190565b60408051601f198184030181528282528051602090910120908601516001600160e01b031960e085901b168352612d6c926004019182526001600160a01b0316602082015260400190565b6020604051808303815f875af1158015612d88573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dac919061683b565b50806001600160a01b031663e2a4853a84604051602001612df3906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b60405160208183030381529060405280519060200120604051602001612e23929190918252602082015260400190565b6040516020818303038152906040528051906020012084606001516040518363ffffffff1660e01b8152600401612e64929190918252602082015260400190565b6020604051808303815f875af1158015612e80573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ea49190616766565b50806001600160a01b031663e2a4853a84604051602001612ef6906020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b60405160208183030381529060405280519060200120604051602001612f26929190918252602082015260400190565b6040516020818303038152906040528051906020012084608001516040518363ffffffff1660e01b8152600401611d27929190918252602082015260400190565b6040805163ea34a57760e01b81526001600160a01b038a81166004830152898116602483015288811660448301526064820188905260ff8716608483015260a4820186905260c48201859052835160e48301526020840151610104830152918301516101248201526060830151610144820152908a169063ea34a57790610164015f604051808303815f87803b158015612fff575f5ffd5b505af1158015613011573d5f5f3e3d5ffd5b50505050505050505050505050565b61021682826040516024016130369291906168d8565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052614afc565b61306d61658e565b5f5f6130798685614b05565b90505f6130868683614b6b565b90506107488782614b84565b5f816001600160a01b0316836001600160a01b0316106130b35781836130b6565b82825b60405191945092506130e3906020016020808252600490820152631413d3d360e21b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b0380861691830191909152831660608201526080015b6040516020818303038152906040528051906020012090505b92915050565b6131436165b4565b8261314c6165b4565b816001600160a01b03166391d4403c604051602001613188906020808252600990820152681413d3d317d31254d560ba1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156131dc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613200919061677d565b61320d5791506131359050565b816001600160a01b03166321f8a7218560405160200161324d906020808252600c908201526b0504f4f4c5f544f4b454e5f360a41b604082015260600190565b6040516020818303038152906040528051906020012060405160200161327d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016132b191815260200190565b602060405180830381865afa1580156132cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132f0919061683b565b8151516001600160a01b03918216905260408051602081810152601391810191909152720504f4f4c5f424f52524f575f494e4445585f3606c1b60608201529083169063bd02d0f59086906080016040516020818303038152906040528051906020012060405160200161336e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016133a291815260200190565b602060405180830381865afa1580156133bd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e19190616766565b81515f60200201516020018181525050816001600160a01b031663bd02d0f585604051602001613437906020808252601290820152710504f4f4c5f424f52524f575f524154455f360741b604082015260600190565b60405160208183030381529060405280519060200120604051602001613467929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161349b91815260200190565b602060405180830381865afa1580156134b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134da9190616766565b81515f60200201516040018181525050816001600160a01b031663bd02d0f585604051602001613535906020808252601790820152760504f4f4c5f544f54414c5f434f4c4c41544552414c5f3604c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001613565929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161359991815260200190565b602060405180830381865afa1580156135b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135d89190616766565b815151606001526040516001600160a01b0383169063bd02d0f590869061360190602001616856565b60405160208183030381529060405280519060200120604051602001613631929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161366591815260200190565b602060405180830381865afa158015613680573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136a49190616766565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001613700906020808252601890820152770504f4f4c5f544f54414c5f5343414c45445f444542545f360441b604082015260600190565b60405160208183030381529060405280519060200120604051602001613730929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161376491815260200190565b602060405180830381865afa15801561377f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137a39190616766565b81515160a0015260408051602081810152601491810191909152730504f4f4c5f554e434c41494d45445f4645455f360641b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001613820929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161385491815260200190565b602060405180830381865afa15801561386f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138939190616766565b81515160c0015260408051602081810152600c918101919091526b504f4f4c5f544f4b454e5f3160a01b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001613908929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161393c91815260200190565b602060405180830381865afa158015613957573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061397b919061683b565b81516020908101516001600160a01b0392831690526040805180830183905260138183015272504f4f4c5f424f52524f575f494e4445585f3160681b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613a2591815260200190565b602060405180830381865afa158015613a40573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613a649190616766565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001613abb90602080825260129082015271504f4f4c5f424f52524f575f524154455f3160701b604082015260600190565b60405160208183030381529060405280519060200120604051602001613aeb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613b1f91815260200190565b602060405180830381865afa158015613b3a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b5e9190616766565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001613bba90602080825260179082015276504f4f4c5f544f54414c5f434f4c4c41544552414c5f3160481b604082015260600190565b60405160208183030381529060405280519060200120604051602001613bea929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613c1e91815260200190565b602060405180830381865afa158015613c39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c5d9190616766565b8151600160200201516060018181525050816001600160a01b031663bd02d0f585604051602001613c8d90616897565b60405160208183030381529060405280519060200120604051602001613cbd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613cf191815260200190565b602060405180830381865afa158015613d0c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d309190616766565b8151600160200201516080018181525050816001600160a01b031663bd02d0f585604051602001613d8d90602080825260189082015277504f4f4c5f544f54414c5f5343414c45445f444542545f3160401b604082015260600190565b60405160208183030381529060405280519060200120604051602001613dbd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613df191815260200190565b602060405180830381865afa158015613e0c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613e309190616766565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f585604051602001613e8990602080825260149082015273504f4f4c5f554e434c41494d45445f4645455f3160601b604082015260600190565b60405160208183030381529060405280519060200120604051602001613eb9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613eed91815260200190565b602060405180830381865afa158015613f08573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f2c9190616766565b81516001602002015160c0018181525050816001600160a01b03166321f8a72185604051602001613f7a90602080825260099082015268504f4f4c5f42414e4b60b81b604082015260600190565b60405160208183030381529060405280519060200120604051602001613faa929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401613fde91815260200190565b602060405180830381865afa158015613ff9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061401d919061683b565b81602001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a7218560405160200161408b906020808252601b908201527f504f4f4c5f494e5445524553545f524154455f53545241544547590000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016140bb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016140ef91815260200190565b602060405180830381865afa15801561410a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061412e919061683b565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b031663bd02d0f585604051602001614191906020808252601290820152712827a7a62fa1a7a72324a3aaa920aa24a7a760711b604082015260600190565b604051602081830303815290604052805190602001206040516020016141c1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016141f591815260200190565b602060405180830381865afa158015614210573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906142349190616766565b60608201526040516001600160a01b0383169063bd02d0f590869061428d906020016020808252601b908201527f504f4f4c5f4c4153545f5550444154455f54494d455f5354414d500000000000604082015260600190565b604051602081830303815290604052805190602001206040516020016142bd929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016142f191815260200190565b602060405180830381865afa15801561430c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906143309190616766565b6080820152949350505050565b60208201516001600160a01b031661021657604051637357d91f60e01b815260048101829052602401610318565b6080810151429081900361437d575050565b81515160a00151156143d35781515f906143a690825b6020020151604001518460800151614bfe565b83519091506143ca9082905f5b602002015160200151614c3a90919063ffffffff16565b83515160200152505b81516020015160a001511561440f5781515f906143f1906001614393565b835190915061440390829060016143b3565b83516020908101510152505b608090910152565b6060810151660800000000000016610216575f5f6144358484614c7b565b60405163fe0081af60e01b815260048101839052602481018290529193509150604401610318565b6106ca81614cdb565b5f816001600160a01b031663bd02d0f56040516020016144a9906020808252600f908201526e4d41585f424f52524f575f5241544560881b604082015260600190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016144dd91815260200190565b602060405180830381865afa1580156144f8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131359190616766565b5f5f5f5f5f875f01518760ff1660028110614539576145396167ed565b602002015190505f61454b8989614d0f565b9050805f03614567575f5f5f5f955095509550955050506145c2565b5f614576838b60800151614de1565b90505f614583838a614c3a565b90505f891561459b576145968284614e11565b61459d565b5f5b90506145a98385616815565b846145b48582616828565b919a50985096509450505050505b93509350935093565b61462f604051806101c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b61463886614e66565b80825261464990879087905f614f0f565b6040830152602082018190525f0361467457604051636c53056d60e01b815260040160405180910390fd5b6060860151614683908561507b565b606082015261469183614846565b6080820181905260608201516146bf9190676765c793fa10079d601b1b906146ba90600a6169e0565b6150a9565b60a082015260ff84166001146146d9578060a001516146e9565b805160a08201516146e991614c3a565b60c08201526146f782614846565b60e0820181905260608201516147209190676765c793fa10079d601b1b906146ba90600a6169e0565b61010082015260ff841660011461473c5780610100015161474d565b805161010082015161474d91614c3a565b6101208201525f8313614773578060c00151816020015161476e9190616828565b614787565b8060c0015181602001516147879190616815565b6101408201525f82136147ae5780610120015181604001516147a99190616828565b6147c3565b80610120015181604001516147c39190616815565b61016082018190525f036147d75750610abc565b6101608101516101408201516147ec9161485b565b6101808201526147fb87615168565b6101a08201819052610180820151101561483d576101808101516101a08201516040516382d6353f60e01b815260048101929092526024820152604401610318565b50505050505050565b5f5f82121561485757815f03613135565b5090565b5f8115676765c793fa10079d601b1b6002840419048411171561487c575f5ffd5b50676765c793fa10079d601b1b9190910260028204010490565b5f6040516020016148d0906020808252601590820152741050d0d3d5539517d413d4d2551253d397d31254d5605a1b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b03841690820152606001604051602081830303815290604052805190602001209050919050565b5f5f61494f6040518060c001604052805f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b614959845f6151b9565b60208301528152606084015161496f905f61507b565b60608201819052815161499491676765c793fa10079d601b1b906146ba90600a6169e0565b6040828101918252858101518151606081018352845181526020808601519082019081529351818401908152925163fdd63ecf60e01b81529051600482015292516024840152905160448301526001600160a01b03169063fdd63ecf90606401602060405180830381865afa158015614a0f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614a339190616766565b6080820152614a438460016151b9565b6020838101918252918352604086810151815160608101835285518152925193830193845281850151838301908152915163fdd63ecf60e01b815292516004840152925160248301525160448201526001600160a01b039091169063fdd63ecf90606401602060405180830381865afa158015614ac2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ae69190616766565b60a0820181905260809091015194909350915050565b6106ca816151ff565b5f604051602001614b32906020808252600890820152672827a9a4aa24a7a760c11b604082015260600190565b60408051601f198184030181528282528051602091820120908301526001600160a01b038516908201526060810183905260800161311c565b614b7361658e565b614b7d838361521e565b9392505050565b60408101516001600160a01b0316614baf57604051634dfbbff360e01b815260040160405180910390fd5b80604001516001600160a01b0316826001600160a01b0316146102165760408082015190516312e38abf60e11b81526001600160a01b0391821660048201529083166024820152604401610318565b5f80614c0a8342616828565b614c1490856169eb565b6301e1338090049050614c3281676765c793fa10079d601b1b616815565b949350505050565b5f81156b019d971e4fe8401e740000001983900484111517614c5a575f5ffd5b50676765c793fa10079d601b1b91026b019d971e4fe8401e74000000010490565b5f5f5f614c8c84606001515f61507b565b90505f614c9886616430565b90505f614cbb82614caa85600a6169e0565b676765c793fa10079d601b1b6150a9565b90505f614cc9875f5f61451c565b50939a91995090975050505050505050565b60ff811615801590614cf1575060ff8116600114155b156106ca57604051632813581b60e21b815260040160405180910390fd5b5f5f835f01518360ff1660028110614d2957614d296167ed565b60209081029190910151908501518151604051637216047960e11b81526001600160a01b0391821660048201529293505f9291169063e42c08f290602401602060405180830381865afa158015614d82573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614da69190616766565b9050805f03614db9575f92505050613135565b606082015160c0830151614dcd8284616828565b614dd79190616828565b9695505050505050565b5f8260a001515f03614df457505f613135565b5f614dff8484616481565b60a0850151909150614c329082614c3a565b5f82614e1d8382616828565b91508111156131355760405162461bcd60e51b815260206004820152601560248201527464732d6d6174682d7375622d756e646572666c6f7760581b6044820152606401610318565b5f5f614e73835f5f61451c565b50505090505f614e858460015f61451c565b5050509050805f03614e9a57505f9392505050565b5f614ea985606001515f61507b565b90505f614ebb8660600151600161507b565b90505f614ed985676765c793fa10079d601b1b6146ba86600a6169e0565b90505f614ef785676765c793fa10079d601b1b6146ba86600a6169e0565b9050614f03828261485b565b98975050505050505050565b825151515f908190819081906001600160a01b03868116911614614fb2575f5f614f3a8a8a5f6164c4565b915091505f614f565f8c6060015161507b90919063ffffffff16565b90505f614f7484676765c793fa10079d601b1b6146ba85600a6169e0565b90505f614f9284676765c793fa10079d601b1b6146ba86600a6169e0565b9050614f9e8288616815565b9650614faa8187616815565b955050505050505b865160200151516001600160a01b0386811691161461506e575f5f614fd98a8a60016164c4565b915091505f614ff660018c6060015161507b90919063ffffffff16565b90505f61501484676765c793fa10079d601b1b6146ba85600a6169e0565b90505f61503284676765c793fa10079d601b1b6146ba86600a6169e0565b90505f61503f838d614c3a565b90505f61504c838e614c3a565b9050615058828a616815565b98506150648189616815565b9750505050505050505b9097909650945050505050565b5f60ff60581b1960585f1960ff85160161509b575060ff60601b19905060605b90198416901c905092915050565b5f838302815f1985870982811083820303915050805f036150dd578382816150d3576150d3616a02565b0492505050614b7d565b8084116150fd5760405163227bc15360e01b815260040160405180910390fd5b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f816001600160a01b031663bd02d0f56040516020016144a99060208082526017908201527f4d415247494e5f4c4556454c4c5f5448524553484f4c44000000000000000000604082015260600190565b5f5f5f6151e6855f01518560ff16600281106151d7576151d76167ed565b60200201518660800151614de1565b90505f6151f38686614d0f565b96919550909350505050565b5f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b61522661658e565b8261522f61658e565b816001600160a01b03166391d4403c60405160200161526f906020808252600d908201526c1413d4d2551253d397d31254d5609a1b604082015260600190565b60408051601f198184030181529082905280516020909101206001600160e01b031960e084901b168252600482015260248101879052604401602060405180830381865afa1580156152c3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906152e7919061677d565b6152f45791506131359050565b816001600160a01b031663bd02d0f58560405160200161532e906020808252600690820152651413d4d7d25160d21b604082015260600190565b6040516020818303038152906040528051906020012060405160200161535e929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161539291815260200190565b602060405180830381865afa1580156153ad573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906153d19190616766565b816020018181525050816001600160a01b03166321f8a72185604051602001615419906020808252600b908201526a1413d4d7d050d0d3d5539560aa1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615449929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161547d91815260200190565b602060405180830381865afa158015615498573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906154bc919061683b565b81604001906001600160a01b031690816001600160a01b031681525050816001600160a01b03166321f8a72185604051602001615518906020808252600b908201526a0504f535f544f4b454e5f360ac1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615548929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161557c91815260200190565b602060405180830381865afa158015615597573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906155bb919061683b565b8151516001600160a01b039182169052604080516020818101526010918101919091526f0504f535f434f4c4c41544552414c5f360841b60608201529083169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615636929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161566a91815260200190565b602060405180830381865afa158015615685573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906156a99190616766565b81515f60200201516020018181525050816001600160a01b031663bd02d0f5856040516020016156fd9060208082526010908201526f0504f535f444542545343414c45445f360841b604082015260600190565b6040516020818303038152906040528051906020012060405160200161572d929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161576191815260200190565b602060405180830381865afa15801561577c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906157a09190616766565b81515f60200201516040018181525050816001600160a01b031663bd02d0f5856040516020016157fa906020808252601690820152750504f535f454e5452595f4c4f4e475f50524943455f360541b604082015260600190565b6040516020818303038152906040528051906020012060405160200161582a929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161585e91815260200190565b602060405180830381865afa158015615879573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061589d9190616766565b81515f60200201516060018181525050816001600160a01b031663bd02d0f5856040516020016158f6906020808252601590820152740504f535f4143435f4c4f4e475f414d4f554e545f3605c1b604082015260600190565b60405160208183030381529060405280519060200120604051602001615926929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b815260040161595a91815260200190565b602060405180830381865afa158015615975573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906159999190616766565b81515160a0015260408051602081810152601791810191909152760504f535f454e5452595f53484f52545f50524943455f3604c1b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615a19929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615a4d91815260200190565b602060405180830381865afa158015615a68573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615a8c9190616766565b81515f60200201516080018181525050816001600160a01b031663bd02d0f585604051602001615ae6906020808252601690820152750504f535f4143435f53484f52545f414d4f554e545f360541b604082015260600190565b60405160208183030381529060405280519060200120604051602001615b16929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615b4a91815260200190565b602060405180830381865afa158015615b65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615b899190616766565b81515160c0015260408051602081810152600a91810191909152690504f535f545950455f360b41b60608201526001600160a01b0383169063bd02d0f590869060800160405160208183030381529060405280519060200120604051602001615bfc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615c3091815260200190565b602060405180830381865afa158015615c4b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615c6f9190616766565b81515160e0015260408051602081810152600b918101919091526a504f535f544f4b454e5f3160a81b60608201526001600160a01b038316906321f8a72190869060800160405160208183030381529060405280519060200120604051602001615ce3929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615d1791815260200190565b602060405180830381865afa158015615d32573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615d56919061683b565b81516020908101516001600160a01b039283169052604080518083018390526010818301526f504f535f434f4c4c41544552414c5f3160801b60608083019190915282518083039091018152608082019092528151919092012060a0820187905260c08201529083169063bd02d0f59060e001604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615dfd91815260200190565b602060405180830381865afa158015615e18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615e3c9190616766565b8151600160200201516020018181525050816001600160a01b031663bd02d0f585604051602001615e919060208082526010908201526f504f535f444542545343414c45445f3160801b604082015260600190565b60405160208183030381529060405280519060200120604051602001615ec1929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615ef591815260200190565b602060405180830381865afa158015615f10573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190615f349190616766565b8151600160200201516040018181525050816001600160a01b031663bd02d0f585604051602001615f8f90602080825260169082015275504f535f454e5452595f4c4f4e475f50524943455f3160501b604082015260600190565b60405160208183030381529060405280519060200120604051602001615fbf929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b8152600401615ff391815260200190565b602060405180830381865afa15801561600e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906160329190616766565b8151600160200201516060018181525050816001600160a01b031663bd02d0f58560405160200161608c90602080825260159082015274504f535f4143435f4c4f4e475f414d4f554e545f3160581b604082015260600190565b604051602081830303815290604052805190602001206040516020016160bc929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016160f091815260200190565b602060405180830381865afa15801561610b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061612f9190616766565b81516001602002015160a0018181525050816001600160a01b031663bd02d0f58560405160200161618b90602080825260179082015276504f535f454e5452595f53484f52545f50524943455f3160481b604082015260600190565b604051602081830303815290604052805190602001206040516020016161bb929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016161ef91815260200190565b602060405180830381865afa15801561620a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061622e9190616766565b8151600160200201516080018181525050816001600160a01b031663bd02d0f58560405160200161628990602080825260169082015275504f535f4143435f53484f52545f414d4f554e545f3160501b604082015260600190565b604051602081830303815290604052805190602001206040516020016162b9929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016162ed91815260200190565b602060405180830381865afa158015616308573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061632c9190616766565b81516001602002015160c0018181525050816001600160a01b031663bd02d0f58560405160200161637b906020808252600a9082015269504f535f545950455f3160b01b604082015260600190565b604051602081830303815290604052805190602001206040516020016163ab929190918252602082015260400190565b604051602081830303815290604052805190602001206040518263ffffffff1660e01b81526004016163df91815260200190565b602060405180830381865afa1580156163fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061641e9190616766565b81516020015160e00152949350505050565b5f816001600160a01b031663bd02d0f56040516020016144a99060208082526019908201527f53484f52545f4c49515549444954595f5448524553484f4c4400000000000000604082015260600190565b5f42820361649457506020820151613135565b5f6164a3846040015184614bfe565b90506164bc846020015182614c3a90919063ffffffff16565b915050613135565b5f5f5f845f01518460ff16600281106164df576164df6167ed565b60200201516040015190505f616515875f01518660ff1660028110616506576165066167ed565b60200201518860800151616481565b9050811561652c576165278282614c3a565b61652e565b5f5b865190935060ff861660028110616547576165476167ed565b60200201516020015193505050935093915050565b604051806080016040528061656f6165b4565b81526020015f815260200161658261658e565b81526020015f81525090565b60405180606001604052806165a16165e8565b81525f6020820181905260409091015290565b6040518060a001604052806165c7616656565b81525f60208201819052604082018190526060820181905260809091015290565b60405180604001604052806002905b6166406040518061010001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816165f75790505090565b60405180604001604052806002905b6166a76040518060e001604052805f6001600160a01b031681526020015f81526020015f81526020015f81526020015f81526020015f81526020015f81525090565b8152602001906001900390816166655790505090565b6001600160a01b03811681146106ca575f5ffd5b5f5f82840360808112156166e3575f5ffd5b83356166ee816166bd565b92506060601f1982011215616701575f5ffd5b506020830190509250929050565b5f6020828403121561671f575f5ffd5b813560ff81168114614b7d575f5ffd5b60208082526017908201527f5245454e5452414e43595f47554152445f535441545553000000000000000000604082015260600190565b5f60208284031215616776575f5ffd5b5051919050565b5f6020828403121561678d575f5ffd5b81518015158114614b7d575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90614c329083018461679c565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8082018082111561313557613135616801565b8181038181111561313557613135616801565b5f6020828403121561684b575f5ffd5b8151614b7d816166bd565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152600360fc1b606082015260800190565b60208082526021908201527f504f4f4c5f544f54414c5f434f4c4c41544552414c5f574954485f444542545f6040820152603160f81b606082015260800190565b604081525f6168ea604083018561679c565b82810360208401526168fc818561679c565b95945050505050565b6001815b600184111561070a5780850481111561692457616924616801565b600184161561693257908102905b60019390931c928002616909565b5f8261694e57506001613135565b8161695a57505f613135565b8160018114616970576002811461697a57616996565b6001915050613135565b60ff84111561698b5761698b616801565b50506001821b613135565b5060208310610133831016604e8410600b84101617156169b9575081810a613135565b6169c55f198484616905565b805f19048211156169d8576169d8616801565b029392505050565b5f614b7d8383616940565b808202811582820484141761313557613135616801565b634e487b7160e01b5f52601260045260245ffdfea264697066735822122007fa207f8137c448e8f32c39d1eed36392ae1445e31d3d519f4d37adaee0ca4364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80cJJ{\x04\x14a\0NW\x80cf\r\rg\x14a\0\x91W\x80c\x9A$\xA6h\x14a\0\xB8W\x80c\x9F\xF7\x8C0\x14a\0\xCDW[__\xFD[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xCBa\0\xC66`\x04af\xD1V[a\0\xF4V[\0[a\0u\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFCa\x02\x1AV[a\x01k`@Q` \x01a\x01-\x90` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x03\xD8V[_`@Q\x80`\xA0\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83_\x015\x81R` \x01\x83` \x01` \x81\x01\x90a\x01\xF1\x91\x90ag\x0FV[`\xFF\x16\x81R` \x01\x83`@\x015\x81RP\x90Pa\x02\r\x83\x82a\x04\x86V[Pa\x02\x16a\x06\x14V[PPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01a\x02Y\x90ag/V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x8D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCC\x91\x90agfV[\x90P\x80\x15a\x03!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x03_\x90ag/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`\x01`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x16\x91\x90agfV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04e\x91\x90ag}V[a\x02\x16W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x03\x18\x92\x91\x90ag\xCAV[a\x04\xB4`@Q\x80`@\x01`@R\x80`\r\x81R` \x01lexecuteBorrow`\x98\x1B\x81RPa\x06\xCDV[a\x04\xBCae\\V[a\x04\xCE\x83\x83_\x01Q\x84`@\x01Qa\x06\xF1V[``\x83\x01R`@\x82\x01\x81\x90R\x82Q\x90Q\x80QQ` \x91\x90\x91\x01QQa\x04\xF4\x92\x91\x90a\x07\x12V[` \x83\x01R\x80\x82R\x82Q`@\x83\x01Q``\x85\x01Q`\x80\x86\x01Qa\x05\x1D\x94\x88\x94\x93\x90\x92\x90\x91a\x07SV[\x80Q`@\x82\x01Q``\x84\x01Q`\x80\x85\x01Qa\x05<\x93\x92\x91\x90\x80_a\x08@V[a\x05R\x82_\x01Q\x82``\x01Q\x83`@\x01Qa\n\xC4V[a\x05c\x82` \x01Q\x82_\x01Qa\x1DnV[\x81Q` \x82\x01Q\x82Qa\x05w\x92\x91\x90a\x1D\x92V[` \x82\x81\x01Q`@\x83\x81\x01QQ\x80QQ\x93\x01QQ\x90\x85\x01Q``\x86\x01Q`\x80\x87\x01Q\x86QQa\x06\x0F\x96\x8A\x95\x90\x94\x90\x93\x90\x92\x90\x91`\xFF\x83\x16`\x02\x81\x10a\x05\xBEWa\x05\xBEag\xEDV[` \x90\x81\x02\x91\x90\x91\x01Q`@\x90\x81\x01Q\x81Q`\x80\x81\x01\x83R\x8C\x83\x01\x80QQQ\x85\x01Q\x82R\x80QQQ\x84\x01Q\x82\x86\x01R\x80QQ\x85\x01Q\x85\x01Q\x82\x85\x01RQQ\x90\x93\x01Q\x90\x91\x01Q``\x83\x01R\x90a/gV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:`@Q` \x01a\x06R\x90ag/V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R_`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCA\x91\x90agfV[PV[a\x06\xCA`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a%s`\xF0\x1B\x81RP\x82a0 V[a\x06\xF9ae\x8EV[_a\x07\x05\x85\x85\x85a0eV[\x91P\x91P[\x93P\x93\x91PPV[a\x07\x1Aae\xB4V[__a\x07&\x85\x85a0\x92V[\x90P_a\x073\x87\x83a1;V[\x90Pa\x07?\x81\x83aC=V[a\x07H\x81aCkV[\x96\x90\x95P\x93PPPPV[_\x19`\xFF\x83\x16\x01a\x07hWa\x07h\x85\x85aD\x17V[\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xB7W`@\x80\x84\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x87\x16`$\x82\x01R`D\x01a\x03\x18V[\x80_\x03a\x07\xD7W`@Qcydj\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xE0\x82aD]V[_a\x07\xEA\x86aDfV[\x90P_a\x07\xF8\x86\x85\x84aE\x1CV[\x93PPPP\x80\x83\x11\x15a\x08(W`@Qc#\xCA\xCA\x81`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R`D\x01a\x03\x18V[a\x086\x87\x87\x87\x87\x87\x88aE\xCBV[PPPPPPPPV[_a\x08J\x84aHFV[\x90P_\x84\x12a\x08\xB8W\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08kWa\x08kag\xEDV[` \x02\x01Q` \x01\x81\x81Qa\x08\x80\x91\x90ah\x15V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08\x9BWa\x08\x9Bag\xEDV[` \x02\x01Q``\x01\x81\x81Qa\x08\xB0\x91\x90ah\x15V[\x90RPa\t\x19V[\x85Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\x08\xD0Wa\x08\xD0ag\xEDV[` \x02\x01Q` \x01\x81\x81Qa\x08\xE5\x91\x90ah(V[\x90RP\x86Q\x81\x90`\xFF\x87\x16`\x02\x81\x10a\t\0Wa\t\0ag\xEDV[` \x02\x01Q``\x01\x81\x81Qa\t\x15\x91\x90ah(V[\x90RP[\x81\x15a\t\x9CW\x85Q_\x90`\xFF\x87\x16`\x02\x81\x10a\t7Wa\t7ag\xEDV[` \x02\x01Q`@\x01Q\x90P\x80\x88_\x01Q\x87`\xFF\x16`\x02\x81\x10a\t[Wa\t[ag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\tp\x91\x90ah(V[\x90RP\x86Q_\x90`\xFF\x88\x16`\x02\x81\x10a\t\x8BWa\t\x8Bag\xEDV[` \x02\x01Q`@\x01RPa\n\xBC\x90PV[\x82_\x03a\t\xA9WPa\n\xBCV[_a\t\xB3\x84aHFV[\x90P_a\t\xE9\x89_\x01Q\x88`\xFF\x16`\x02\x81\x10a\t\xD1Wa\t\xD1ag\xEDV[` \x02\x01Q` \x01Q\x83aH[\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x85\x12a\nWW\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n\nWa\n\nag\xEDV[` \x02\x01Q`@\x01\x81\x81Qa\n\x1F\x91\x90ah\x15V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n:Wa\n:ag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\nO\x91\x90ah\x15V[\x90RPa\n\xB8V[\x87Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\noWa\noag\xEDV[` \x02\x01Q`@\x01\x81\x81Qa\n\x84\x91\x90ah(V[\x90RP\x88Q\x81\x90`\xFF\x89\x16`\x02\x81\x10a\n\x9FWa\n\x9Fag\xEDV[` \x02\x01Q`\xA0\x01\x81\x81Qa\n\xB4\x91\x90ah(V[\x90RP[PPP[PPPPPPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x0B\x08\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0BXW__\xFD[PZ\xF1\x15\x80\x15a\x0BjW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLba\x0B\x8A\x84`@\x01QaH\x96V[\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xB1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xC8W__\xFD[PZ\xF1\x15\x80\x15a\x0B\xDAW=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0C\x18\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0CH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xA5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC9\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\r\t\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\r9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra\r\x84\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC4\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x0E\x04\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0E4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBB\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0F\0\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x0F0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB1\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x0F\xF6\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x10&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA6\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x10\xF1\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA2\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x11\xEC\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x12\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x9D\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x12\xE9\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x13\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13vW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x9A\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x13\xE5\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x14\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x96\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x14\xD5\x90` \x80\x82R`\n\x90\x82\x01Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\x05\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xE0\x90\x81\x01Q\x90\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15cW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x87\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x15\xC7\x90` \x80\x82R`\x0B\x90\x82\x01RjPOS_TOKEN_1`\xA8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x15\xF7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x81\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x16\xC6\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_COLLATERAL_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x16\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17UW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17y\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x17\xBE\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x17\xEE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18MW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18q\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x18\xBC\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x18\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19p\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x19\xBA\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x19\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1AJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1An\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1A\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1A\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1BJW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bn\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1B\xB9\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1B\xE9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1CIW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cm\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1C\xAC\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1C\xDC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x84Q`\x01` \x02\x01Q`\xE0\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Dg\x91\x90agfV[PPPPPV[a\x1Dw\x81aI\x1AV[\x91Q\x80Q` \x90\x91\x01Q`@\x90\x81\x01\x93\x90\x93R\x91\x90\x91\x01RPV[_\x83\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x0FLb`@Q` \x01a\x1D\xD2\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\"W__\xFD[PZ\xF1\x15\x80\x15a\x1E4W=__>=_\xFD[PPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a\x1Ex\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1E\xA8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F/\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\x1Fw\x90` \x80\x82R`\x13\x90\x82\x01Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x1F\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x04W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a (\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a o\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a \x9F\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x86QQ\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \xFBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x1F\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a!k\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a!\x9B\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a!\xF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x1C\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a\"<\x90ahVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\"l\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xED\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a#:\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#j\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xEB\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a$4\x90` \x80\x82R`\x14\x90\x82\x01Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a$d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x85QQ`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xE5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a%&\x90` \x80\x82R`\x0C\x90\x82\x01RkPOOL_TOKEN_1`\xA0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a%V\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01QQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a%\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xE0\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a&(\x90` \x80\x82R`\x13\x90\x82\x01RrPOOL_BORROW_INDEX_1`h\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a&X\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x82\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a&\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xDB\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a'\"\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a'R\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x87Q\x90\x91\x01Q\x90\x91\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xD5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(!\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a(Q\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q``\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xB1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xD5\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a(\xF5\x90ah\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a)%\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\x80\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xA9\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a)\xF6\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a*&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xA0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xAA\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a*\xF3\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a+#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x91\x82\x01 \x86Q\x90\x91\x01Q`\xC0\x01Q`\xE0\x84\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R`\x04\x83\x01\x91\x90\x91R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xA7\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a+\xE5\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a,\x15\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,_\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9F\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9\x84`@Q` \x01a,\xF1\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a-!\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x90\x91\x01 \x90\x86\x01Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x83Ra-l\x92`\x04\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a-\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xAC\x91\x90ah;V[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a-\xF3\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a.#\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84``\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a.d\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a.\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xA4\x91\x90agfV[P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xE2\xA4\x85:\x84`@Q` \x01a.\xF6\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a/&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x80\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D'\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Qc\xEA4\xA5w`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\xFF\x87\x16`\x84\x83\x01R`\xA4\x82\x01\x86\x90R`\xC4\x82\x01\x85\x90R\x83Q`\xE4\x83\x01R` \x84\x01Qa\x01\x04\x83\x01R\x91\x83\x01Qa\x01$\x82\x01R``\x83\x01Qa\x01D\x82\x01R\x90\x8A\x16\x90c\xEA4\xA5w\x90a\x01d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\xFFW__\xFD[PZ\xF1\x15\x80\x15a0\x11W=__>=_\xFD[PPPPPPPPPPPPPV[a\x02\x16\x82\x82`@Q`$\x01a06\x92\x91\x90ah\xD8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90RaJ\xFCV[a0mae\x8EV[__a0y\x86\x85aK\x05V[\x90P_a0\x86\x86\x83aKkV[\x90Pa\x07H\x87\x82aK\x84V[_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a0\xB3W\x81\x83a0\xB6V[\x82\x82[`@Q\x91\x94P\x92Pa0\xE3\x90` \x01` \x80\x82R`\x04\x90\x82\x01Rc\x14\x13\xD3\xD3`\xE2\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x91\x83\x01\x91\x90\x91R\x83\x16``\x82\x01R`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a1Cae\xB4V[\x82a1Lae\xB4V[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01a1\x88\x90` \x80\x82R`\t\x90\x82\x01Rh\x14\x13\xD3\xD3\x17\xD3\x12T\xD5`\xBA\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xDCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\0\x91\x90ag}V[a2\rW\x91Pa15\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a2M\x90` \x80\x82R`\x0C\x90\x82\x01Rk\x05\x04\xF4\xF4\xC5\xF5D\xF4\xB4T\xE5\xF3`\xA4\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a2}\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\xF0\x91\x90ah;V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x13\x91\x81\x01\x91\x90\x91Rr\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF4\x94\xE4DU\x85\xF3`l\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a3n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a3\xA2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE1\x91\x90agfV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a47\x90` \x80\x82R`\x12\x90\x82\x01Rq\x05\x04\xF4\xF4\xC5\xF4$\xF5%$\xF5u\xF5$\x15DU\xF3`t\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a4g\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a4\x9B\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xDA\x91\x90agfV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a55\x90` \x80\x82R`\x17\x90\x82\x01Rv\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`L\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a5e\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\x99\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xD8\x91\x90agfV[\x81QQ``\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90a6\x01\x90` \x01ahVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a61\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6e\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x80W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xA4\x91\x90agfV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a7\0\x90` \x80\x82R`\x18\x90\x82\x01Rw\x05\x04\xF4\xF4\xC5\xF5D\xF5D\x14\xC5\xF544\x14\xC4TE\xF4DT%E\xF3`D\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a70\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7d\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA3\x91\x90agfV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x14\x91\x81\x01\x91\x90\x91Rs\x05\x04\xF4\xF4\xC5\xF5T\xE44\xC4\x14\x94\xD4TE\xF4dTU\xF3`d\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a8 \x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8T\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x93\x91\x90agfV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\x0C\x91\x81\x01\x91\x90\x91RkPOOL_TOKEN_1`\xA0\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a9\x08\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9<\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9WW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9{\x91\x90ah;V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x13\x81\x83\x01RrPOOL_BORROW_INDEX_1`h\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:%\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:d\x91\x90agfV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a:\xBB\x90` \x80\x82R`\x12\x90\x82\x01RqPOOL_BORROW_RATE_1`p\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a:\xEB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x1F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;^\x91\x90agfV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a;\xBA\x90` \x80\x82R`\x17\x90\x82\x01RvPOOL_TOTAL_COLLATERAL_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a;\xEA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x1E\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<]\x91\x90agfV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a<\x8D\x90ah\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a<\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=0\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a=\x8D\x90` \x80\x82R`\x18\x90\x82\x01RwPOOL_TOTAL_SCALED_DEBT_1`@\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a=\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>0\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a>\x89\x90` \x80\x82R`\x14\x90\x82\x01RsPOOL_UNCLAIMED_FEE_1``\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a>\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?,\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a?z\x90` \x80\x82R`\t\x90\x82\x01RhPOOL_BANK`\xB8\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a?\xAA\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a?\xDE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x1D\x91\x90ah;V[\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01a@\x8B\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_INTEREST_RATE_STRATEGY\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a@\xBB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA.\x91\x90ah;V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aA\x91\x90` \x80\x82R`\x12\x90\x82\x01Rq('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA$\xA7\xA7`q\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aA\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB4\x91\x90agfV[``\x82\x01R`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90aB\x8D\x90` \x01` \x80\x82R`\x1B\x90\x82\x01R\x7FPOOL_LAST_UPDATE_TIME_STAMP\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aB\xBD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aB\xF1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC0\x91\x90agfV[`\x80\x82\x01R\x94\x93PPPPV[` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x02\x16W`@QcsW\xD9\x1F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x18V[`\x80\x81\x01QB\x90\x81\x90\x03aC}WPPV[\x81QQ`\xA0\x01Q\x15aC\xD3W\x81Q_\x90aC\xA6\x90\x82[` \x02\x01Q`@\x01Q\x84`\x80\x01QaK\xFEV[\x83Q\x90\x91PaC\xCA\x90\x82\x90_[` \x02\x01Q` \x01QaL:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83QQ` \x01RP[\x81Q` \x01Q`\xA0\x01Q\x15aD\x0FW\x81Q_\x90aC\xF1\x90`\x01aC\x93V[\x83Q\x90\x91PaD\x03\x90\x82\x90`\x01aC\xB3V[\x83Q` \x90\x81\x01Q\x01RP[`\x80\x90\x91\x01RV[``\x81\x01Qf\x08\0\0\0\0\0\0\x16a\x02\x16W__aD5\x84\x84aL{V[`@Qc\xFE\0\x81\xAF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R\x91\x93P\x91P`D\x01a\x03\x18V[a\x06\xCA\x81aL\xDBV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x0F\x90\x82\x01RnMAX_BORROW_RATE`\x88\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aD\xDD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xF8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a15\x91\x90agfV[_____\x87_\x01Q\x87`\xFF\x16`\x02\x81\x10aE9WaE9ag\xEDV[` \x02\x01Q\x90P_aEK\x89\x89aM\x0FV[\x90P\x80_\x03aEgW____\x95P\x95P\x95P\x95PPPaE\xC2V[_aEv\x83\x8B`\x80\x01QaM\xE1V[\x90P_aE\x83\x83\x8AaL:V[\x90P_\x89\x15aE\x9BWaE\x96\x82\x84aN\x11V[aE\x9DV[_[\x90PaE\xA9\x83\x85ah\x15V[\x84aE\xB4\x85\x82ah(V[\x91\x9AP\x98P\x96P\x94PPPPP[\x93P\x93P\x93P\x93V[aF/`@Q\x80a\x01\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aF8\x86aNfV[\x80\x82RaFI\x90\x87\x90\x87\x90_aO\x0FV[`@\x83\x01R` \x82\x01\x81\x90R_\x03aFtW`@QclS\x05m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x86\x01QaF\x83\x90\x85aP{V[``\x82\x01RaF\x91\x83aHFV[`\x80\x82\x01\x81\x90R``\x82\x01QaF\xBF\x91\x90gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[aP\xA9V[`\xA0\x82\x01R`\xFF\x84\x16`\x01\x14aF\xD9W\x80`\xA0\x01QaF\xE9V[\x80Q`\xA0\x82\x01QaF\xE9\x91aL:V[`\xC0\x82\x01RaF\xF7\x82aHFV[`\xE0\x82\x01\x81\x90R``\x82\x01QaG \x91\x90gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[a\x01\0\x82\x01R`\xFF\x84\x16`\x01\x14aG<W\x80a\x01\0\x01QaGMV[\x80Qa\x01\0\x82\x01QaGM\x91aL:V[a\x01 \x82\x01R_\x83\x13aGsW\x80`\xC0\x01Q\x81` \x01QaGn\x91\x90ah(V[aG\x87V[\x80`\xC0\x01Q\x81` \x01QaG\x87\x91\x90ah\x15V[a\x01@\x82\x01R_\x82\x13aG\xAEW\x80a\x01 \x01Q\x81`@\x01QaG\xA9\x91\x90ah(V[aG\xC3V[\x80a\x01 \x01Q\x81`@\x01QaG\xC3\x91\x90ah\x15V[a\x01`\x82\x01\x81\x90R_\x03aG\xD7WPa\n\xBCV[a\x01`\x81\x01Qa\x01@\x82\x01QaG\xEC\x91aH[V[a\x01\x80\x82\x01RaG\xFB\x87aQhV[a\x01\xA0\x82\x01\x81\x90Ra\x01\x80\x82\x01Q\x10\x15aH=Wa\x01\x80\x81\x01Qa\x01\xA0\x82\x01Q`@Qc\x82\xD65?`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x03\x18V[PPPPPPPV[__\x82\x12\x15aHWW\x81_\x03a15V[P\x90V[_\x81\x15gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B`\x02\x84\x04\x19\x04\x84\x11\x17\x15aH|W__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[_`@Q` \x01aH\xD0\x90` \x80\x82R`\x15\x90\x82\x01Rt\x10P\xD0\xD3\xD5S\x95\x17\xD4\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`Z\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[__aIO`@Q\x80`\xC0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[aIY\x84_aQ\xB9V[` \x83\x01R\x81R``\x84\x01QaIo\x90_aP{V[``\x82\x01\x81\x90R\x81QaI\x94\x91gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x90aF\xBA\x90`\nai\xE0V[`@\x82\x81\x01\x91\x82R\x85\x81\x01Q\x81Q``\x81\x01\x83R\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01\x90\x81R\x93Q\x81\x84\x01\x90\x81R\x92Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x90Q`\x04\x82\x01R\x92Q`$\x84\x01R\x90Q`D\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x0FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ3\x91\x90agfV[`\x80\x82\x01RaJC\x84`\x01aQ\xB9V[` \x83\x81\x01\x91\x82R\x91\x83R`@\x86\x81\x01Q\x81Q``\x81\x01\x83R\x85Q\x81R\x92Q\x93\x83\x01\x93\x84R\x81\x85\x01Q\x83\x83\x01\x90\x81R\x91Qc\xFD\xD6>\xCF`\xE0\x1B\x81R\x92Q`\x04\x84\x01R\x92Q`$\x83\x01RQ`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xFD\xD6>\xCF\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\xC2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xE6\x91\x90agfV[`\xA0\x82\x01\x81\x90R`\x80\x90\x91\x01Q\x94\x90\x93P\x91PPV[a\x06\xCA\x81aQ\xFFV[_`@Q` \x01aK2\x90` \x80\x82R`\x08\x90\x82\x01Rg('\xA9\xA4\xAA$\xA7\xA7`\xC1\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x82\x01R``\x81\x01\x83\x90R`\x80\x01a1\x1CV[aKsae\x8EV[aK}\x83\x83aR\x1EV[\x93\x92PPPV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16aK\xAFW`@QcM\xFB\xBF\xF3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02\x16W`@\x80\x82\x01Q\x90Qc\x12\xE3\x8A\xBF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x83\x16`$\x82\x01R`D\x01a\x03\x18V[_\x80aL\n\x83Bah(V[aL\x14\x90\x85ai\xEBV[c\x01\xE13\x80\x90\x04\x90PaL2\x81gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1Bah\x15V[\x94\x93PPPPV[_\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17aLZW__\xFD[Pgge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1B\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[___aL\x8C\x84``\x01Q_aP{V[\x90P_aL\x98\x86ad0V[\x90P_aL\xBB\x82aL\xAA\x85`\nai\xE0V[gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaP\xA9V[\x90P_aL\xC9\x87__aE\x1CV[P\x93\x9A\x91\x99P\x90\x97PPPPPPPPV[`\xFF\x81\x16\x15\x80\x15\x90aL\xF1WP`\xFF\x81\x16`\x01\x14\x15[\x15a\x06\xCAW`@Qc(\x13X\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x83_\x01Q\x83`\xFF\x16`\x02\x81\x10aM)WaM)ag\xEDV[` \x90\x81\x02\x91\x90\x91\x01Q\x90\x85\x01Q\x81Q`@Qcr\x16\x04y`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xE4,\x08\xF2\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aM\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aM\xA6\x91\x90agfV[\x90P\x80_\x03aM\xB9W_\x92PPPa15V[``\x82\x01Q`\xC0\x83\x01QaM\xCD\x82\x84ah(V[aM\xD7\x91\x90ah(V[\x96\x95PPPPPPV[_\x82`\xA0\x01Q_\x03aM\xF4WP_a15V[_aM\xFF\x84\x84ad\x81V[`\xA0\x85\x01Q\x90\x91PaL2\x90\x82aL:V[_\x82aN\x1D\x83\x82ah(V[\x91P\x81\x11\x15a15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rtds-math-sub-underflow`X\x1B`D\x82\x01R`d\x01a\x03\x18V[__aNs\x83__aE\x1CV[PPP\x90P_aN\x85\x84`\x01_aE\x1CV[PPP\x90P\x80_\x03aN\x9AWP_\x93\x92PPPV[_aN\xA9\x85``\x01Q_aP{V[\x90P_aN\xBB\x86``\x01Q`\x01aP{V[\x90P_aN\xD9\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90P_aN\xF7\x85gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90PaO\x03\x82\x82aH[V[\x98\x97PPPPPPPPV[\x82QQQ_\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aO\xB2W__aO:\x8A\x8A_ad\xC4V[\x91P\x91P_aOV_\x8C``\x01QaP{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aOt\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x85`\nai\xE0V[\x90P_aO\x92\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90PaO\x9E\x82\x88ah\x15V[\x96PaO\xAA\x81\x87ah\x15V[\x95PPPPPP[\x86Q` \x01QQ`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14aPnW__aO\xD9\x8A\x8A`\x01ad\xC4V[\x91P\x91P_aO\xF6`\x01\x8C``\x01QaP{\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_aP\x14\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x85`\nai\xE0V[\x90P_aP2\x84gge\xC7\x93\xFA\x10\x07\x9D`\x1B\x1BaF\xBA\x86`\nai\xE0V[\x90P_aP?\x83\x8DaL:V[\x90P_aPL\x83\x8EaL:V[\x90PaPX\x82\x8Aah\x15V[\x98PaPd\x81\x89ah\x15V[\x97PPPPPPPP[\x90\x97\x90\x96P\x94PPPPPV[_`\xFF`X\x1B\x19`X_\x19`\xFF\x85\x16\x01aP\x9BWP`\xFF``\x1B\x19\x90P``[\x90\x19\x84\x16\x90\x1C\x90P\x92\x91PPV[_\x83\x83\x02\x81_\x19\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aP\xDDW\x83\x82\x81aP\xD3WaP\xD3aj\x02V[\x04\x92PPPaK}V[\x80\x84\x11aP\xFDW`@Qc\"{\xC1S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x17\x90\x82\x01R\x7FMARGIN_LEVELL_THRESHOLD\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[___aQ\xE6\x85_\x01Q\x85`\xFF\x16`\x02\x81\x10aQ\xD7WaQ\xD7ag\xEDV[` \x02\x01Q\x86`\x80\x01QaM\xE1V[\x90P_aQ\xF3\x86\x86aM\x0FV[\x96\x91\x95P\x90\x93PPPPV[_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[aR&ae\x8EV[\x82aR/ae\x8EV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\x91\xD4@<`@Q` \x01aRo\x90` \x80\x82R`\r\x90\x82\x01Rl\x14\x13\xD4\xD2U\x12S\xD3\x97\xD3\x12T\xD5`\x9A\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x82R`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aR\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aR\xE7\x91\x90ag}V[aR\xF4W\x91Pa15\x90PV[\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aS.\x90` \x80\x82R`\x06\x90\x82\x01Re\x14\x13\xD4\xD7\xD2Q`\xD2\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aS^\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\x92\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aS\xADW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aS\xD1\x91\x90agfV[\x81` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aT\x19\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x14\x13\xD4\xD7\xD0P\xD0\xD3\xD5S\x95`\xAA\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aTI\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aT}\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aT\xBC\x91\x90ah;V[\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!\x85`@Q` \x01aU\x18\x90` \x80\x82R`\x0B\x90\x82\x01Rj\x05\x04\xF55\xF5D\xF4\xB4T\xE5\xF3`\xAC\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aUH\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aU|\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aU\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\xBB\x91\x90ah;V[\x81QQ`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R`@\x80Q` \x81\x81\x01R`\x10\x91\x81\x01\x91\x90\x91Ro\x05\x04\xF55\xF44\xF4\xC4\xC4\x15DU$\x14\xC5\xF3`\x84\x1B``\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aV6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aVj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aV\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aV\xA9\x91\x90agfV[\x81Q_` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aV\xFD\x90` \x80\x82R`\x10\x90\x82\x01Ro\x05\x04\xF55\xF4DT%E44\x14\xC4TE\xF3`\x84\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aW-\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aWa\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aW|W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aW\xA0\x91\x90agfV[\x81Q_` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aW\xFA\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4T\xE5E%\x95\xF4\xC4\xF4\xE4u\xF5\x05$\x944U\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aX*\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aX^\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aXyW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aX\x9D\x91\x90agfV[\x81Q_` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aX\xF6\x90` \x80\x82R`\x15\x90\x82\x01Rt\x05\x04\xF55\xF4\x1445\xF4\xC4\xF4\xE4u\xF4\x14\xD4\xF5T\xE5E\xF3`\\\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aY&\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aYZ\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aYuW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aY\x99\x91\x90agfV[\x81QQ`\xA0\x01R`@\x80Q` \x81\x81\x01R`\x17\x91\x81\x01\x91\x90\x91Rv\x05\x04\xF55\xF4T\xE5E%\x95\xF54\x84\xF5%E\xF5\x05$\x944U\xF3`L\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aZ\x19\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aZM\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aZhW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aZ\x8C\x91\x90agfV[\x81Q_` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aZ\xE6\x90` \x80\x82R`\x16\x90\x82\x01Ru\x05\x04\xF55\xF4\x1445\xF54\x84\xF5%E\xF4\x14\xD4\xF5T\xE5E\xF3`T\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\x16\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a[J\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a[eW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a[\x89\x91\x90agfV[\x81QQ`\xC0\x01R`@\x80Q` \x81\x81\x01R`\n\x91\x81\x01\x91\x90\x91Ri\x05\x04\xF55\xF5E\x95\x04U\xF3`\xB4\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xBD\x02\xD0\xF5\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a[\xFC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\\0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\\KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\\o\x91\x90agfV[\x81QQ`\xE0\x01R`@\x80Q` \x81\x81\x01R`\x0B\x91\x81\x01\x91\x90\x91RjPOS_TOKEN_1`\xA8\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c!\xF8\xA7!\x90\x86\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\\\xE3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\x17\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a]2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a]V\x91\x90ah;V[\x81Q` \x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x90R`@\x80Q\x80\x83\x01\x83\x90R`\x10\x81\x83\x01RoPOS_COLLATERAL_1`\x80\x1B``\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\xA0\x82\x01\x87\x90R`\xC0\x82\x01R\x90\x83\x16\x90c\xBD\x02\xD0\xF5\x90`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a]\xFD\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a^\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a^<\x91\x90agfV[\x81Q`\x01` \x02\x01Q` \x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a^\x91\x90` \x80\x82R`\x10\x90\x82\x01RoPOS_DEBTSCALED_1`\x80\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a^\xC1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a^\xF5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a_\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a_4\x91\x90agfV[\x81Q`\x01` \x02\x01Q`@\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a_\x8F\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ENTRY_LONG_PRICE_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a_\xBF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a_\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a`\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a`2\x91\x90agfV[\x81Q`\x01` \x02\x01Q``\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01a`\x8C\x90` \x80\x82R`\x15\x90\x82\x01RtPOS_ACC_LONG_AMOUNT_1`X\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a`\xBC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a`\xF0\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aa\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aa/\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xA0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01aa\x8B\x90` \x80\x82R`\x17\x90\x82\x01RvPOS_ENTRY_SHORT_PRICE_1`H\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01aa\xBB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aa\xEF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ab\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ab.\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\x80\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ab\x89\x90` \x80\x82R`\x16\x90\x82\x01RuPOS_ACC_SHORT_AMOUNT_1`P\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ab\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ab\xED\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ac\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ac,\x91\x90agfV[\x81Q`\x01` \x02\x01Q`\xC0\x01\x81\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5\x85`@Q` \x01ac{\x90` \x80\x82R`\n\x90\x82\x01RiPOS_TYPE_1`\xB0\x1B`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01ac\xAB\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01ac\xDF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15ac\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90ad\x1E\x91\x90agfV[\x81Q` \x01Q`\xE0\x01R\x94\x93PPPPV[_\x81`\x01`\x01`\xA0\x1B\x03\x16c\xBD\x02\xD0\xF5`@Q` \x01aD\xA9\x90` \x80\x82R`\x19\x90\x82\x01R\x7FSHORT_LIQUIDITY_THRESHOLD\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_B\x82\x03ad\x94WP` \x82\x01Qa15V[_ad\xA3\x84`@\x01Q\x84aK\xFEV[\x90Pad\xBC\x84` \x01Q\x82aL:\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PPa15V[___\x84_\x01Q\x84`\xFF\x16`\x02\x81\x10ad\xDFWad\xDFag\xEDV[` \x02\x01Q`@\x01Q\x90P_ae\x15\x87_\x01Q\x86`\xFF\x16`\x02\x81\x10ae\x06Wae\x06ag\xEDV[` \x02\x01Q\x88`\x80\x01Qad\x81V[\x90P\x81\x15ae,Wae'\x82\x82aL:V[ae.V[_[\x86Q\x90\x93P`\xFF\x86\x16`\x02\x81\x10aeGWaeGag\xEDV[` \x02\x01Q` \x01Q\x93PPP\x93P\x93\x91PPV[`@Q\x80`\x80\x01`@R\x80aeoae\xB4V[\x81R` \x01_\x81R` \x01ae\x82ae\x8EV[\x81R` \x01_\x81RP\x90V[`@Q\x80``\x01`@R\x80ae\xA1ae\xE8V[\x81R_` \x82\x01\x81\x90R`@\x90\x91\x01R\x90V[`@Q\x80`\xA0\x01`@R\x80ae\xC7afVV[\x81R_` \x82\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af@`@Q\x80a\x01\0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81ae\xF7W\x90PP\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90[af\xA7`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81afeW\x90PP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xCAW__\xFD[__\x82\x84\x03`\x80\x81\x12\x15af\xE3W__\xFD[\x835af\xEE\x81af\xBDV[\x92P```\x1F\x19\x82\x01\x12\x15ag\x01W__\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15ag\x1FW__\xFD[\x815`\xFF\x81\x16\x81\x14aK}W__\xFD[` \x80\x82R`\x17\x90\x82\x01R\x7FREENTRANCY_GUARD_STATUS\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15agvW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15ag\x8DW__\xFD[\x81Q\x80\x15\x15\x81\x14aK}W__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90aL2\x90\x83\x01\x84ag\x9CV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a15Wa15ah\x01V[\x81\x81\x03\x81\x81\x11\x15a15Wa15ah\x01V[_` \x82\x84\x03\x12\x15ahKW__\xFD[\x81QaK}\x81af\xBDV[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`\x03`\xFC\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`!\x90\x82\x01R\x7FPOOL_TOTAL_COLLATERAL_WITH_DEBT_`@\x82\x01R`1`\xF8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_ah\xEA`@\x83\x01\x85ag\x9CV[\x82\x81\x03` \x84\x01Rah\xFC\x81\x85ag\x9CV[\x95\x94PPPPPV[`\x01\x81[`\x01\x84\x11\x15a\x07\nW\x80\x85\x04\x81\x11\x15ai$Wai$ah\x01V[`\x01\x84\x16\x15ai2W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02ai\tV[_\x82aiNWP`\x01a15V[\x81aiZWP_a15V[\x81`\x01\x81\x14aipW`\x02\x81\x14aizWai\x96V[`\x01\x91PPa15V[`\xFF\x84\x11\x15ai\x8BWai\x8Bah\x01V[PP`\x01\x82\x1Ba15V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15ai\xB9WP\x81\x81\na15V[ai\xC5_\x19\x84\x84ai\x05V[\x80_\x19\x04\x82\x11\x15ai\xD8Wai\xD8ah\x01V[\x02\x93\x92PPPV[_aK}\x83\x83ai@V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a15Wa15ah\x01V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x07\xFA \x7F\x817\xC4H\xE8\xF3,9\xD1\xEE\xD3c\x92\xAE\x14E\xE3\x1D=Q\x9FM7\xAD\xAE\xE0\xCACdsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `AccountNotMatch(address,address)` and selector `0x25c7157e`.
```solidity
error AccountNotMatch(address accountInPosition, address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccountNotMatch {
        pub accountInPosition: alloy::sol_types::private::Address,
        pub account: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<AccountNotMatch> for UnderlyingRustTuple<'_> {
            fn from(value: AccountNotMatch) -> Self {
                (value.accountInPosition, value.account)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AccountNotMatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    accountInPosition: tuple.0,
                    account: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccountNotMatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccountNotMatch(address,address)";
            const SELECTOR: [u8; 4] = [37u8, 199u8, 21u8, 126u8];
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
                        &self.accountInPosition,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `EmptyBorrowAmounts()` and selector `0x79646aaf`.
```solidity
error EmptyBorrowAmounts();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyBorrowAmounts {}
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
        impl ::core::convert::From<EmptyBorrowAmounts> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyBorrowAmounts) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyBorrowAmounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyBorrowAmounts {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyBorrowAmounts()";
            const SELECTOR: [u8; 4] = [121u8, 100u8, 106u8, 175u8];
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
    /**Custom error with signature `EmptyCollateral()` and selector `0x6c53056d`.
```solidity
error EmptyCollateral();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCollateral {}
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
        impl ::core::convert::From<EmptyCollateral> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCollateral) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyCollateral {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCollateral {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCollateral()";
            const SELECTOR: [u8; 4] = [108u8, 83u8, 5u8, 109u8];
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
    /**Custom error with signature `EmptyPosition()` and selector `0x4dfbbff3`.
```solidity
error EmptyPosition();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyPosition {}
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
        impl ::core::convert::From<EmptyPosition> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyPosition) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyPosition {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyPosition {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyPosition()";
            const SELECTOR: [u8; 4] = [77u8, 251u8, 191u8, 243u8];
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
    /**Custom error with signature `InsufficientReverveForBorrow(uint256,uint256)` and selector `0x47959502`.
```solidity
error InsufficientReverveForBorrow(uint256 amountToBorrow, uint256 availableReserve);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientReverveForBorrow {
        pub amountToBorrow: alloy::sol_types::private::primitives::aliases::U256,
        pub availableReserve: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InsufficientReverveForBorrow>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientReverveForBorrow) -> Self {
                (value.amountToBorrow, value.availableReserve)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientReverveForBorrow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountToBorrow: tuple.0,
                    availableReserve: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientReverveForBorrow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientReverveForBorrow(uint256,uint256)";
            const SELECTOR: [u8; 4] = [71u8, 149u8, 149u8, 2u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountToBorrow),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.availableReserve),
                )
            }
        }
    };
    /**Custom error with signature `MarginBelowThreshold(uint256,uint256)` and selector `0x82d6353f`.
```solidity
error MarginBelowThreshold(uint256 marginLevel, uint256 marginLevelThreshold);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MarginBelowThreshold {
        pub marginLevel: alloy::sol_types::private::primitives::aliases::U256,
        pub marginLevelThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MarginBelowThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: MarginBelowThreshold) -> Self {
                (value.marginLevel, value.marginLevelThreshold)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MarginBelowThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    marginLevel: tuple.0,
                    marginLevelThreshold: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MarginBelowThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MarginBelowThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [130u8, 214u8, 53u8, 63u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.marginLevel),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.marginLevelThreshold),
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
    /**Custom error with signature `TokenIndexNotSupport()` and selector `0xa04d606c`.
```solidity
error TokenIndexNotSupport();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenIndexNotSupport {}
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
        impl ::core::convert::From<TokenIndexNotSupport> for UnderlyingRustTuple<'_> {
            fn from(value: TokenIndexNotSupport) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TokenIndexNotSupport {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TokenIndexNotSupport {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TokenIndexNotSupport()";
            const SELECTOR: [u8; 4] = [160u8, 77u8, 96u8, 108u8];
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
    /**Custom error with signature `liquidityDidNotReachShortThreshord(uint256,uint256)` and selector `0xfe0081af`.
```solidity
error liquidityDidNotReachShortThreshord(uint256 threshold, uint256 basePriceReserve);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct liquidityDidNotReachShortThreshord {
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        pub basePriceReserve: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<liquidityDidNotReachShortThreshord>
        for UnderlyingRustTuple<'_> {
            fn from(value: liquidityDidNotReachShortThreshord) -> Self {
                (value.threshold, value.basePriceReserve)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for liquidityDidNotReachShortThreshord {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    threshold: tuple.0,
                    basePriceReserve: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for liquidityDidNotReachShortThreshord {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "liquidityDidNotReachShortThreshord(uint256,uint256)";
            const SELECTOR: [u8; 4] = [254u8, 0u8, 129u8, 175u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.basePriceReserve),
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
    /**Function with signature `executeBorrow(address,(uint256,uint8,uint256))` and selector `0x9a24a668`.
```solidity
function executeBorrow(address account, BorrowUtils.BorrowParams memory BorrowParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBorrowCall {
        pub account: alloy::sol_types::private::Address,
        pub BorrowParams: <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`executeBorrow(address,(uint256,uint8,uint256))`](executeBorrowCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeBorrowReturn {}
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
                BorrowUtils::BorrowParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<executeBorrowCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeBorrowCall) -> Self {
                    (value.account, value.BorrowParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBorrowCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        account: tuple.0,
                        BorrowParams: tuple.1,
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
            impl ::core::convert::From<executeBorrowReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeBorrowReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeBorrowReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeBorrowCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                BorrowUtils::BorrowParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeBorrowReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeBorrow(address,(uint256,uint8,uint256))";
            const SELECTOR: [u8; 4] = [154u8, 36u8, 166u8, 104u8];
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
                    <BorrowUtils::BorrowParams as alloy_sol_types::SolType>::tokenize(
                        &self.BorrowParams,
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
    ///Container for all the [`BorrowHandler`](self) function calls.
    pub enum BorrowHandlerCalls {
        dataStore(dataStoreCall),
        eventEmitter(eventEmitterCall),
        executeBorrow(executeBorrowCall),
        roleStore(roleStoreCall),
    }
    #[automatically_derived]
    impl BorrowHandlerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [74u8, 74u8, 123u8, 4u8],
            [102u8, 13u8, 13u8, 103u8],
            [154u8, 36u8, 166u8, 104u8],
            [159u8, 247u8, 140u8, 48u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BorrowHandlerCalls {
        const NAME: &'static str = "BorrowHandlerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::dataStore(_) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eventEmitter(_) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeBorrow(_) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<BorrowHandlerCalls>] = &[
                {
                    fn roleStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerCalls> {
                        <roleStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerCalls::roleStore)
                    }
                    roleStore
                },
                {
                    fn dataStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerCalls> {
                        <dataStoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerCalls::dataStore)
                    }
                    dataStore
                },
                {
                    fn executeBorrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerCalls> {
                        <executeBorrowCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerCalls::executeBorrow)
                    }
                    executeBorrow
                },
                {
                    fn eventEmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerCalls> {
                        <eventEmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerCalls::eventEmitter)
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
                Self::dataStore(inner) => {
                    <dataStoreCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eventEmitter(inner) => {
                    <eventEmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::executeBorrow(inner) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::executeBorrow(inner) => {
                    <executeBorrowCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`BorrowHandler`](self) custom errors.
    pub enum BorrowHandlerErrors {
        AccountNotMatch(AccountNotMatch),
        EmptyBorrowAmounts(EmptyBorrowAmounts),
        EmptyCollateral(EmptyCollateral),
        EmptyPool(EmptyPool),
        EmptyPosition(EmptyPosition),
        InsufficientReverveForBorrow(InsufficientReverveForBorrow),
        MarginBelowThreshold(MarginBelowThreshold),
        MathOverflowedMulDiv(MathOverflowedMulDiv),
        TokenIndexNotSupport(TokenIndexNotSupport),
        Unauthorized(Unauthorized),
        liquidityDidNotReachShortThreshord(liquidityDidNotReachShortThreshord),
    }
    #[automatically_derived]
    impl BorrowHandlerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [34u8, 123u8, 193u8, 83u8],
            [37u8, 199u8, 21u8, 126u8],
            [71u8, 149u8, 149u8, 2u8],
            [77u8, 251u8, 191u8, 243u8],
            [108u8, 83u8, 5u8, 109u8],
            [115u8, 87u8, 217u8, 31u8],
            [121u8, 100u8, 106u8, 175u8],
            [130u8, 214u8, 53u8, 63u8],
            [160u8, 77u8, 96u8, 108u8],
            [163u8, 91u8, 21u8, 11u8],
            [254u8, 0u8, 129u8, 175u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BorrowHandlerErrors {
        const NAME: &'static str = "BorrowHandlerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccountNotMatch(_) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyBorrowAmounts(_) => {
                    <EmptyBorrowAmounts as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCollateral(_) => {
                    <EmptyCollateral as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyPool(_) => <EmptyPool as alloy_sol_types::SolError>::SELECTOR,
                Self::EmptyPosition(_) => {
                    <EmptyPosition as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientReverveForBorrow(_) => {
                    <InsufficientReverveForBorrow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MarginBelowThreshold(_) => {
                    <MarginBelowThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MathOverflowedMulDiv(_) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TokenIndexNotSupport(_) => {
                    <TokenIndexNotSupport as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Unauthorized(_) => {
                    <Unauthorized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::liquidityDidNotReachShortThreshord(_) => {
                    <liquidityDidNotReachShortThreshord as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<BorrowHandlerErrors>] = &[
                {
                    fn MathOverflowedMulDiv(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::MathOverflowedMulDiv)
                    }
                    MathOverflowedMulDiv
                },
                {
                    fn AccountNotMatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <AccountNotMatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::AccountNotMatch)
                    }
                    AccountNotMatch
                },
                {
                    fn InsufficientReverveForBorrow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <InsufficientReverveForBorrow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::InsufficientReverveForBorrow)
                    }
                    InsufficientReverveForBorrow
                },
                {
                    fn EmptyPosition(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <EmptyPosition as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::EmptyPosition)
                    }
                    EmptyPosition
                },
                {
                    fn EmptyCollateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <EmptyCollateral as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::EmptyCollateral)
                    }
                    EmptyCollateral
                },
                {
                    fn EmptyPool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <EmptyPool as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::EmptyPool)
                    }
                    EmptyPool
                },
                {
                    fn EmptyBorrowAmounts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <EmptyBorrowAmounts as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::EmptyBorrowAmounts)
                    }
                    EmptyBorrowAmounts
                },
                {
                    fn MarginBelowThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <MarginBelowThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::MarginBelowThreshold)
                    }
                    MarginBelowThreshold
                },
                {
                    fn TokenIndexNotSupport(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <TokenIndexNotSupport as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::TokenIndexNotSupport)
                    }
                    TokenIndexNotSupport
                },
                {
                    fn Unauthorized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <Unauthorized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::Unauthorized)
                    }
                    Unauthorized
                },
                {
                    fn liquidityDidNotReachShortThreshord(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BorrowHandlerErrors> {
                        <liquidityDidNotReachShortThreshord as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BorrowHandlerErrors::liquidityDidNotReachShortThreshord)
                    }
                    liquidityDidNotReachShortThreshord
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
                Self::AccountNotMatch(inner) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyBorrowAmounts(inner) => {
                    <EmptyBorrowAmounts as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyCollateral(inner) => {
                    <EmptyCollateral as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyPosition(inner) => {
                    <EmptyPosition as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientReverveForBorrow(inner) => {
                    <InsufficientReverveForBorrow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MarginBelowThreshold(inner) => {
                    <MarginBelowThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TokenIndexNotSupport(inner) => {
                    <TokenIndexNotSupport as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::liquidityDidNotReachShortThreshord(inner) => {
                    <liquidityDidNotReachShortThreshord as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccountNotMatch(inner) => {
                    <AccountNotMatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyBorrowAmounts(inner) => {
                    <EmptyBorrowAmounts as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyCollateral(inner) => {
                    <EmptyCollateral as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyPool(inner) => {
                    <EmptyPool as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyPosition(inner) => {
                    <EmptyPosition as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientReverveForBorrow(inner) => {
                    <InsufficientReverveForBorrow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MarginBelowThreshold(inner) => {
                    <MarginBelowThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MathOverflowedMulDiv(inner) => {
                    <MathOverflowedMulDiv as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TokenIndexNotSupport(inner) => {
                    <TokenIndexNotSupport as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Unauthorized(inner) => {
                    <Unauthorized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::liquidityDidNotReachShortThreshord(inner) => {
                    <liquidityDidNotReachShortThreshord as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BorrowHandler`](self) contract instance.

See the [wrapper's documentation](`BorrowHandlerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BorrowHandlerInstance<T, P, N> {
        BorrowHandlerInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<BorrowHandlerInstance<T, P, N>>,
    > {
        BorrowHandlerInstance::<
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
        BorrowHandlerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _roleStore, _dataStore, _eventEmitter)
    }
    /**A [`BorrowHandler`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BorrowHandler`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BorrowHandlerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BorrowHandlerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BorrowHandlerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BorrowHandlerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BorrowHandler`](self) contract instance.

See the [wrapper's documentation](`BorrowHandlerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BorrowHandlerInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> BorrowHandlerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BorrowHandlerInstance<T, P, N> {
            BorrowHandlerInstance {
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
    > BorrowHandlerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`executeBorrow`] function.
        pub fn executeBorrow(
            &self,
            account: alloy::sol_types::private::Address,
            BorrowParams: <BorrowUtils::BorrowParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeBorrowCall, N> {
            self.call_builder(
                &executeBorrowCall {
                    account,
                    BorrowParams,
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
    > BorrowHandlerInstance<T, P, N> {
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
