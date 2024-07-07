pub use dex_store_utils::*;
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
pub mod dex_store_utils {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("get"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataStore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAssetB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DexEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DexEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnderlyAssetEmpty"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEXSTOREUTILS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x03\xCFa\09`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`,WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c-a\x8A\x1A\x14a\0EW\x80c\xE28\xD8\xC2\x14a\0tW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x02\xCEV[a\0\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x81\x80\x15a\0\x80W`\0\x80\xFD[Pa\0\x94a\0\x8F6`\x04a\x03\x19V[a\x01\x18V[\0[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!a\0\xB1\x85\x85a\x02\x0BV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xCF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x10\x91\x90a\x03uV[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a\x015WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x15a\x01SW`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01zW`@Qc\x13\xE8I\xDD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9a\x01\x93\x85\x85a\x02\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x04\x91\x90a\x03uV[PPPPPV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\x020W\x83\x85a\x023V[\x84\x84[\x91P\x91P`@Q` \x01a\x02^\x90` \x80\x82R`\x03\x90\x82\x01Rb\x08\x88\xAB`\xEB\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x91\x83\x01\x91\x90\x91R\x82\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xCBW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\xE3W`\0\x80\xFD[\x835a\x02\xEE\x81a\x02\xB6V[\x92P` \x84\x015a\x02\xFE\x81a\x02\xB6V[\x91P`@\x84\x015a\x03\x0E\x81a\x02\xB6V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x03/W`\0\x80\xFD[\x845a\x03:\x81a\x02\xB6V[\x93P` \x85\x015a\x03J\x81a\x02\xB6V[\x92P`@\x85\x015a\x03Z\x81a\x02\xB6V[\x91P``\x85\x015a\x03j\x81a\x02\xB6V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x03\x87W`\0\x80\xFD[\x81Qa\x03\x92\x81a\x02\xB6V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 #\xE1\xA7\xB5\xCA\x8C\xCFV\xCC:\x1C\xE13:\t\xCCGS\"%W\xA2I\x19\xF2y\xB1\xAD&\xFA\xEF=dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static DEXSTOREUTILS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\0@W`\x005`\xE0\x1C\x80c-a\x8A\x1A\x14a\0EW\x80c\xE28\xD8\xC2\x14a\0tW[`\0\x80\xFD[a\0Xa\0S6`\x04a\x02\xCEV[a\0\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x81\x80\x15a\0\x80W`\0\x80\xFD[Pa\0\x94a\0\x8F6`\x04a\x03\x19V[a\x01\x18V[\0[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c!\xF8\xA7!a\0\xB1\x85\x85a\x02\x0BV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xCF\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x10\x91\x90a\x03uV[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80a\x015WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15[\x15a\x01SW`@Qc\x08h\x96S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01zW`@Qc\x13\xE8I\xDD`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16c\xCADm\xD9a\x01\x93\x85\x85a\x02\x0BV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x04\x91\x90a\x03uV[PPPPPV[`\0\x80`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x10a\x020W\x83\x85a\x023V[\x84\x84[\x91P\x91P`@Q` \x01a\x02^\x90` \x80\x82R`\x03\x90\x82\x01Rb\x08\x88\xAB`\xEB\x1B`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x91\x83\x01\x91\x90\x91R\x82\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x92PPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xCBW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02\xE3W`\0\x80\xFD[\x835a\x02\xEE\x81a\x02\xB6V[\x92P` \x84\x015a\x02\xFE\x81a\x02\xB6V[\x91P`@\x84\x015a\x03\x0E\x81a\x02\xB6V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x03/W`\0\x80\xFD[\x845a\x03:\x81a\x02\xB6V[\x93P` \x85\x015a\x03J\x81a\x02\xB6V[\x92P`@\x85\x015a\x03Z\x81a\x02\xB6V[\x91P``\x85\x015a\x03j\x81a\x02\xB6V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x03\x87W`\0\x80\xFD[\x81Qa\x03\x92\x81a\x02\xB6V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 #\xE1\xA7\xB5\xCA\x8C\xCFV\xCC:\x1C\xE13:\t\xCCGS\"%W\xA2I\x19\xF2y\xB1\xAD&\xFA\xEF=dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static DEXSTOREUTILS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DexStoreUtils<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DexStoreUtils<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DexStoreUtils<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DexStoreUtils<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DexStoreUtils<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DexStoreUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DexStoreUtils<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEXSTOREUTILS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DEXSTOREUTILS_ABI.clone(),
                DEXSTOREUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `get` (0x2d618a1a) function
        pub fn get(
            &self,
            data_store: ::ethers::core::types::Address,
            underlying_asset_a: ::ethers::core::types::Address,
            underlying_asset_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [45, 97, 138, 26],
                    (data_store, underlying_asset_a, underlying_asset_b),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DexStoreUtils<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DexEmpty` with signature `DexEmpty()` and selector `0x9f424ee8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DexEmpty", abi = "DexEmpty()")]
    pub struct DexEmpty;
    ///Custom Error type `UnderlyAssetEmpty` with signature `UnderlyAssetEmpty()` and selector `0x08689653`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnderlyAssetEmpty", abi = "UnderlyAssetEmpty()")]
    pub struct UnderlyAssetEmpty;
    ///Container type for all of the contract's custom errors
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
    pub enum DexStoreUtilsErrors {
        DexEmpty(DexEmpty),
        UnderlyAssetEmpty(UnderlyAssetEmpty),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DexStoreUtilsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DexEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DexEmpty(decoded));
            }
            if let Ok(decoded) = <UnderlyAssetEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnderlyAssetEmpty(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DexStoreUtilsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DexEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyAssetEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DexStoreUtilsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DexEmpty as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnderlyAssetEmpty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DexStoreUtilsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DexEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyAssetEmpty(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DexStoreUtilsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DexEmpty> for DexStoreUtilsErrors {
        fn from(value: DexEmpty) -> Self {
            Self::DexEmpty(value)
        }
    }
    impl ::core::convert::From<UnderlyAssetEmpty> for DexStoreUtilsErrors {
        fn from(value: UnderlyAssetEmpty) -> Self {
            Self::UnderlyAssetEmpty(value)
        }
    }
    ///Container type for all input parameters for the `get` function with signature `get(address,address,address)` and selector `0x2d618a1a`
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
    #[ethcall(name = "get", abi = "get(address,address,address)")]
    pub struct GetCall {
        pub data_store: ::ethers::core::types::Address,
        pub underlying_asset_a: ::ethers::core::types::Address,
        pub underlying_asset_b: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `get` function with signature `get(address,address,address)` and selector `0x2d618a1a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetReturn(pub ::ethers::core::types::Address);
}
