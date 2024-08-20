pub use i_exchange_router::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_exchange_router {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("executeBorrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBorrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BorrowUtils.BorrowParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeClose"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeClose"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct CloseUtils.CloseParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeClosePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeClosePosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct CloseUtils.ClosePositionParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DepositUtils.DepositParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeLiquidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Address],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LiquidationUtils.LiquidationParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RedeemUtils.RedeemParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRepay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeRepay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepayUtils.RepayParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeRepaySubstitute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeRepaySubstitute",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct RepaySubstituteUtils.RepaySubstituteParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSupply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SupplyUtils.SupplyParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapUtils.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSwapExactOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeSwapExactOut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapUtils.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct WithdrawUtils.WithdrawParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IEXCHANGEROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IExchangeRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IExchangeRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IExchangeRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IExchangeRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IExchangeRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IExchangeRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IExchangeRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IEXCHANGEROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `executeBorrow` (0x020ccf1c) function
        pub fn execute_borrow(
            &self,
            params: BorrowParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 12, 207, 28], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeClose` (0x1eccf46f) function
        pub fn execute_close(
            &self,
            params: CloseParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 204, 244, 111], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeClosePosition` (0x827eb78a) function
        pub fn execute_close_position(
            &self,
            params: ClosePositionParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 126, 183, 138], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeDeposit` (0xef68ac96) function
        pub fn execute_deposit(
            &self,
            params: DepositParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 104, 172, 150], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeLiquidation` (0xb85c51d5) function
        pub fn execute_liquidation(
            &self,
            params: LiquidationParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 92, 81, 213], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRedeem` (0x7b27af0b) function
        pub fn execute_redeem(
            &self,
            params: RedeemParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 39, 175, 11], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRepay` (0x9152cd1e) function
        pub fn execute_repay(
            &self,
            params: RepayParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 82, 205, 30], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeRepaySubstitute` (0x2bf5e965) function
        pub fn execute_repay_substitute(
            &self,
            params: RepaySubstituteParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 245, 233, 101], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSupply` (0x345bde74) function
        pub fn execute_supply(
            &self,
            params: SupplyParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 91, 222, 116], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwap` (0xf0e4646b) function
        pub fn execute_swap(
            &self,
            params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 228, 100, 107], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwapExactOut` (0x8760de8b) function
        pub fn execute_swap_exact_out(
            &self,
            params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 96, 222, 139], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithdraw` (0x92058b8f) function
        pub fn execute_withdraw(
            &self,
            params: WithdrawParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 5, 139, 143], (params,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IExchangeRouter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `executeBorrow` function with signature `executeBorrow((address,uint256))` and selector `0x020ccf1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeBorrow", abi = "executeBorrow((address,uint256))")]
    pub struct ExecuteBorrowCall {
        pub params: BorrowParams,
    }
    ///Container type for all input parameters for the `executeClose` function with signature `executeClose((address))` and selector `0x1eccf46f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeClose", abi = "executeClose((address))")]
    pub struct ExecuteCloseCall {
        pub params: CloseParams,
    }
    ///Container type for all input parameters for the `executeClosePosition` function with signature `executeClosePosition((address,address,uint256))` and selector `0x827eb78a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeClosePosition",
        abi = "executeClosePosition((address,address,uint256))"
    )]
    pub struct ExecuteClosePositionCall {
        pub params: ClosePositionParams,
    }
    ///Container type for all input parameters for the `executeDeposit` function with signature `executeDeposit((address))` and selector `0xef68ac96`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeDeposit", abi = "executeDeposit((address))")]
    pub struct ExecuteDepositCall {
        pub params: DepositParams,
    }
    ///Container type for all input parameters for the `executeLiquidation` function with signature `executeLiquidation((address))` and selector `0xb85c51d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeLiquidation", abi = "executeLiquidation((address))")]
    pub struct ExecuteLiquidationCall {
        pub params: LiquidationParams,
    }
    ///Container type for all input parameters for the `executeRedeem` function with signature `executeRedeem((address,uint256,address))` and selector `0x7b27af0b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeRedeem", abi = "executeRedeem((address,uint256,address))")]
    pub struct ExecuteRedeemCall {
        pub params: RedeemParams,
    }
    ///Container type for all input parameters for the `executeRepay` function with signature `executeRepay((address,uint256))` and selector `0x9152cd1e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeRepay", abi = "executeRepay((address,uint256))")]
    pub struct ExecuteRepayCall {
        pub params: RepayParams,
    }
    ///Container type for all input parameters for the `executeRepaySubstitute` function with signature `executeRepaySubstitute((address,uint256,address))` and selector `0x2bf5e965`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeRepaySubstitute",
        abi = "executeRepaySubstitute((address,uint256,address))"
    )]
    pub struct ExecuteRepaySubstituteCall {
        pub params: RepaySubstituteParams,
    }
    ///Container type for all input parameters for the `executeSupply` function with signature `executeSupply((address,address))` and selector `0x345bde74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeSupply", abi = "executeSupply((address,address))")]
    pub struct ExecuteSupplyCall {
        pub params: SupplyParams,
    }
    ///Container type for all input parameters for the `executeSwap` function with signature `executeSwap((address,address,uint256,uint256))` and selector `0xf0e4646b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeSwap",
        abi = "executeSwap((address,address,uint256,uint256))"
    )]
    pub struct ExecuteSwapCall {
        pub params: SwapParams,
    }
    ///Container type for all input parameters for the `executeSwapExactOut` function with signature `executeSwapExactOut((address,address,uint256,uint256))` and selector `0x8760de8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeSwapExactOut",
        abi = "executeSwapExactOut((address,address,uint256,uint256))"
    )]
    pub struct ExecuteSwapExactOutCall {
        pub params: SwapParams,
    }
    ///Container type for all input parameters for the `executeWithdraw` function with signature `executeWithdraw((address,uint256,address))` and selector `0x92058b8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeWithdraw",
        abi = "executeWithdraw((address,uint256,address))"
    )]
    pub struct ExecuteWithdrawCall {
        pub params: WithdrawParams,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum IExchangeRouterCalls {
        ExecuteBorrow(ExecuteBorrowCall),
        ExecuteClose(ExecuteCloseCall),
        ExecuteClosePosition(ExecuteClosePositionCall),
        ExecuteDeposit(ExecuteDepositCall),
        ExecuteLiquidation(ExecuteLiquidationCall),
        ExecuteRedeem(ExecuteRedeemCall),
        ExecuteRepay(ExecuteRepayCall),
        ExecuteRepaySubstitute(ExecuteRepaySubstituteCall),
        ExecuteSupply(ExecuteSupplyCall),
        ExecuteSwap(ExecuteSwapCall),
        ExecuteSwapExactOut(ExecuteSwapExactOutCall),
        ExecuteWithdraw(ExecuteWithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IExchangeRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteBorrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBorrow(decoded));
            }
            if let Ok(decoded) = <ExecuteCloseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteClose(decoded));
            }
            if let Ok(decoded) = <ExecuteClosePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteClosePosition(decoded));
            }
            if let Ok(decoded) = <ExecuteDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteDeposit(decoded));
            }
            if let Ok(decoded) = <ExecuteLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteLiquidation(decoded));
            }
            if let Ok(decoded) = <ExecuteRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRedeem(decoded));
            }
            if let Ok(decoded) = <ExecuteRepayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepay(decoded));
            }
            if let Ok(decoded) = <ExecuteRepaySubstituteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteRepaySubstitute(decoded));
            }
            if let Ok(decoded) = <ExecuteSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSupply(decoded));
            }
            if let Ok(decoded) = <ExecuteSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSwap(decoded));
            }
            if let Ok(decoded) = <ExecuteSwapExactOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSwapExactOut(decoded));
            }
            if let Ok(decoded) = <ExecuteWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IExchangeRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteBorrow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteClose(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteClosePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRepay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteRepaySubstitute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSwapExactOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IExchangeRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecuteBorrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteClose(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteClosePosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRepay(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteRepaySubstitute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwapExactOut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteWithdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteBorrowCall> for IExchangeRouterCalls {
        fn from(value: ExecuteBorrowCall) -> Self {
            Self::ExecuteBorrow(value)
        }
    }
    impl ::core::convert::From<ExecuteCloseCall> for IExchangeRouterCalls {
        fn from(value: ExecuteCloseCall) -> Self {
            Self::ExecuteClose(value)
        }
    }
    impl ::core::convert::From<ExecuteClosePositionCall> for IExchangeRouterCalls {
        fn from(value: ExecuteClosePositionCall) -> Self {
            Self::ExecuteClosePosition(value)
        }
    }
    impl ::core::convert::From<ExecuteDepositCall> for IExchangeRouterCalls {
        fn from(value: ExecuteDepositCall) -> Self {
            Self::ExecuteDeposit(value)
        }
    }
    impl ::core::convert::From<ExecuteLiquidationCall> for IExchangeRouterCalls {
        fn from(value: ExecuteLiquidationCall) -> Self {
            Self::ExecuteLiquidation(value)
        }
    }
    impl ::core::convert::From<ExecuteRedeemCall> for IExchangeRouterCalls {
        fn from(value: ExecuteRedeemCall) -> Self {
            Self::ExecuteRedeem(value)
        }
    }
    impl ::core::convert::From<ExecuteRepayCall> for IExchangeRouterCalls {
        fn from(value: ExecuteRepayCall) -> Self {
            Self::ExecuteRepay(value)
        }
    }
    impl ::core::convert::From<ExecuteRepaySubstituteCall> for IExchangeRouterCalls {
        fn from(value: ExecuteRepaySubstituteCall) -> Self {
            Self::ExecuteRepaySubstitute(value)
        }
    }
    impl ::core::convert::From<ExecuteSupplyCall> for IExchangeRouterCalls {
        fn from(value: ExecuteSupplyCall) -> Self {
            Self::ExecuteSupply(value)
        }
    }
    impl ::core::convert::From<ExecuteSwapCall> for IExchangeRouterCalls {
        fn from(value: ExecuteSwapCall) -> Self {
            Self::ExecuteSwap(value)
        }
    }
    impl ::core::convert::From<ExecuteSwapExactOutCall> for IExchangeRouterCalls {
        fn from(value: ExecuteSwapExactOutCall) -> Self {
            Self::ExecuteSwapExactOut(value)
        }
    }
    impl ::core::convert::From<ExecuteWithdrawCall> for IExchangeRouterCalls {
        fn from(value: ExecuteWithdrawCall) -> Self {
            Self::ExecuteWithdraw(value)
        }
    }
}
