pub use uniswap_v3_mint_callee::*;
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
pub mod uniswap_v3_mint_callee {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3MintCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3MintCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Owed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Owed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MintCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MintCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Owed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Owed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV3MINTCALLEE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\n\x14\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c{OS'\x14a\0;W\x80c\xD3Hy\x97\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x06UV[a\0cV[\0[a\0Na\0^6`\x04a\x06\xCEV[a\0\xFEV[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x81\x83\x01\x92\x83\x90Rc<\x8A}\x8D`\xE0\x1B\x90\x92R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91c<\x8A}\x8D\x91a\0\xB3\x91\x88\x91\x88\x91\x88\x91\x88\x91\x90`D\x01a\x07\xA1V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xF5\x91\x90a\x07\xEEV[PPPPPPPV[`\0a\x01\x0C\x82\x84\x01\x84a\x08\x12V[\x90Pa\x01-`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\t\x97`H\x919PV[a\x01R`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e9\xB2\xB722\xB9`\xD1\x1B\x81RPPV[a\x01|`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x18[[\xDD[\x9D\x0C\x13\xDD\xD9Y`\xAA\x1B\x81RPPV[a\x01\xA6`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x18[[\xDD[\x9D\x0CS\xDD\xD9Y`\xAA\x1B\x81RPPV[a\x02\x93`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02*\x91\x90a\x086V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x8F\x91\x90\x81\x01\x90a\x08iV[PPV[a\x02\xF3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x06W=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xA0\x96\x8B\xE0\x05f\x087\x01\xC9\xEFg\x1C\x16\x9D\x7F\xB0Z\xC8\x90}\xE4\xCA\x17\x18]\xE7L\xCB\xB6\x94\xD4\x91\x01`@Q\x80\x91\x03\x90\xA1\x84\x15a\x04\x11W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x086V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0F\x91\x90a\t\x1EV[P[\x83\x15a\x04\xF6W3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04y\x91\x90a\x086V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF4\x91\x90a\t\x1EV[P[PPPPPV[PV[\x92\x91PPV[\x84Q\x81\x10\x15a\x06!W\x82`\x04\x86\x83\x81Q\x81\x10a\x05$Wa\x05$a\t\x80V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x05LWa\x05La\t\x80V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x05g\x83`\x02a\tVV[a\x05r\x90`\x02a\tmV[\x81Q\x81\x10a\x05\x82Wa\x05\x82a\t\x80V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x05\xACWa\x05\xACa\t\x80V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x05\xCCWa\x05\xCCa\t\x80V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x05\xE7\x83`\x02a\tVV[a\x05\xF2\x90`\x03a\tmV[\x81Q\x81\x10a\x06\x02Wa\x06\x02a\t\x80V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x05\x06V[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xFDW`\0\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x06PW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06mW`\0\x80\xFD[\x855a\x06x\x81a\x06)V[\x94P` \x86\x015a\x06\x88\x81a\x06)V[\x93Pa\x06\x96`@\x87\x01a\x06>V[\x92Pa\x06\xA4``\x87\x01a\x06>V[\x91P`\x80\x86\x015`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x06\xC0W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06\xE4W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\tW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x07\x1AW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x071W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x07CW`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0[\x83\x81\x10\x15a\x07lW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07TV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x8D\x81` \x86\x01` \x86\x01a\x07QV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84`\x02\x0B` \x82\x01R\x83`\x02\x0B`@\x82\x01R`\x01`\x01`\x80\x1B\x03\x83\x16``\x82\x01R`\xA0`\x80\x82\x01R`\0a\x07\xE3`\xA0\x83\x01\x84a\x07uV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x01W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\x08$W`\0\x80\xFD[\x815a\x08/\x81a\x06)V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08HW`\0\x80\xFD[\x81Qa\x08/\x81a\x06)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08{W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x92W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08\xA3W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xBDWa\x08\xBDa\x08SV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xECWa\x08\xECa\x08SV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\t\x04W`\0\x80\xFD[a\t\x15\x82` \x83\x01` \x86\x01a\x07QV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\t0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08/W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\0Wa\x05\0a\t@V[\x80\x82\x01\x80\x82\x11\x15a\x05\0Wa\x05\0a\t@V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE-------------------------uniswapV3MintCallback--------------------------\xA2dipfsX\"\x12 \x9C)[{\xE4\x8B\x0C/\x9A\xCAj:\xF2\x8C\xC1I\xB2\xDA\x86\xF3\x95\x0B\xCD?\xABb\xBB\x88\xFEB\xA3\xF9dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static UNISWAPV3MINTCALLEE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c{OS'\x14a\0;W\x80c\xD3Hy\x97\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x06UV[a\0cV[\0[a\0Na\0^6`\x04a\x06\xCEV[a\0\xFEV[`@\x80Q3` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R\x81\x83\x01\x92\x83\x90Rc<\x8A}\x8D`\xE0\x1B\x90\x92R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91c<\x8A}\x8D\x91a\0\xB3\x91\x88\x91\x88\x91\x88\x91\x88\x91\x90`D\x01a\x07\xA1V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xF5\x91\x90a\x07\xEEV[PPPPPPPV[`\0a\x01\x0C\x82\x84\x01\x84a\x08\x12V[\x90Pa\x01-`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\t\x97`H\x919PV[a\x01R`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e9\xB2\xB722\xB9`\xD1\x1B\x81RPPV[a\x01|`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x18[[\xDD[\x9D\x0C\x13\xDD\xD9Y`\xAA\x1B\x81RPPV[a\x01\xA6`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x18[[\xDD[\x9D\x0CS\xDD\xD9Y`\xAA\x1B\x81RPPV[a\x02\x93`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02*\x91\x90a\x086V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x8F\x91\x90\x81\x01\x90a\x08iV[PPV[a\x02\xF3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x06W=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xA0\x96\x8B\xE0\x05f\x087\x01\xC9\xEFg\x1C\x16\x9D\x7F\xB0Z\xC8\x90}\xE4\xCA\x17\x18]\xE7L\xCB\xB6\x94\xD4\x91\x01`@Q\x80\x91\x03\x90\xA1\x84\x15a\x04\x11W3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x086V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0F\x91\x90a\t\x1EV[P[\x83\x15a\x04\xF6W3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04y\x91\x90a\x086V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF4\x91\x90a\t\x1EV[P[PPPPPV[PV[\x92\x91PPV[\x84Q\x81\x10\x15a\x06!W\x82`\x04\x86\x83\x81Q\x81\x10a\x05$Wa\x05$a\t\x80V[\x01` \x01Q\x82Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x90\x91\x1C`\xF8\x1C\x90\x81\x10a\x05LWa\x05La\t\x80V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x05g\x83`\x02a\tVV[a\x05r\x90`\x02a\tmV[\x81Q\x81\x10a\x05\x82Wa\x05\x82a\t\x80V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x82\x85\x82\x81Q\x81\x10a\x05\xACWa\x05\xACa\t\x80V[` \x91\x01\x01Q\x81Q`\xF8\x91\x90\x91\x1C`\x0F\x16\x90\x81\x10a\x05\xCCWa\x05\xCCa\t\x80V[\x01` \x01Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x82a\x05\xE7\x83`\x02a\tVV[a\x05\xF2\x90`\x03a\tmV[\x81Q\x81\x10a\x06\x02Wa\x06\x02a\t\x80V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x01\x01a\x05\x06V[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xFDW`\0\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x06PW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06mW`\0\x80\xFD[\x855a\x06x\x81a\x06)V[\x94P` \x86\x015a\x06\x88\x81a\x06)V[\x93Pa\x06\x96`@\x87\x01a\x06>V[\x92Pa\x06\xA4``\x87\x01a\x06>V[\x91P`\x80\x86\x015`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x06\xC0W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06\xE4W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\tW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x07\x1AW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x071W`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\x07CW`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[`\0[\x83\x81\x10\x15a\x07lW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07TV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x8D\x81` \x86\x01` \x86\x01a\x07QV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84`\x02\x0B` \x82\x01R\x83`\x02\x0B`@\x82\x01R`\x01`\x01`\x80\x1B\x03\x83\x16``\x82\x01R`\xA0`\x80\x82\x01R`\0a\x07\xE3`\xA0\x83\x01\x84a\x07uV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x01W`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\0` \x82\x84\x03\x12\x15a\x08$W`\0\x80\xFD[\x815a\x08/\x81a\x06)V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08HW`\0\x80\xFD[\x81Qa\x08/\x81a\x06)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08{W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x92W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x08\xA3W`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xBDWa\x08\xBDa\x08SV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xECWa\x08\xECa\x08SV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\t\x04W`\0\x80\xFD[a\t\x15\x82` \x83\x01` \x86\x01a\x07QV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\t0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08/W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\0Wa\x05\0a\t@V[\x80\x82\x01\x80\x82\x11\x15a\x05\0Wa\x05\0a\t@V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE-------------------------uniswapV3MintCallback--------------------------\xA2dipfsX\"\x12 \x9C)[{\xE4\x8B\x0C/\x9A\xCAj:\xF2\x8C\xC1I\xB2\xDA\x86\xF3\x95\x0B\xCD?\xABb\xBB\x88\xFEB\xA3\xF9dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static UNISWAPV3MINTCALLEE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV3MintCallee<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV3MintCallee<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV3MintCallee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV3MintCallee<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV3MintCallee<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UniswapV3MintCallee))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV3MintCallee<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV3MINTCALLEE_ABI.clone(),
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
                UNISWAPV3MINTCALLEE_ABI.clone(),
                UNISWAPV3MINTCALLEE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `mint` (0x7b4f5327) function
        pub fn mint(
            &self,
            pool: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 79, 83, 39],
                    (pool, recipient, tick_lower, tick_upper, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3MintCallback` (0xd3487997) function
        pub fn uniswap_v3_mint_callback(
            &self,
            amount_0_owed: ::ethers::core::types::U256,
            amount_1_owed: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 72, 121, 151], (amount_0_owed, amount_1_owed, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MintCallback` event
        pub fn mint_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintCallbackFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintCallbackFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV3MintCallee<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MintCallback", abi = "MintCallback(uint256,uint256)")]
    pub struct MintCallbackFilter {
        pub amount_0_owed: ::ethers::core::types::U256,
        pub amount_1_owed: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,address,int24,int24,uint128)` and selector `0x7b4f5327`
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
    #[ethcall(name = "mint", abi = "mint(address,address,int24,int24,uint128)")]
    pub struct MintCall {
        pub pool: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    ///Container type for all input parameters for the `uniswapV3MintCallback` function with signature `uniswapV3MintCallback(uint256,uint256,bytes)` and selector `0xd3487997`
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
        name = "uniswapV3MintCallback",
        abi = "uniswapV3MintCallback(uint256,uint256,bytes)"
    )]
    pub struct UniswapV3MintCallbackCall {
        pub amount_0_owed: ::ethers::core::types::U256,
        pub amount_1_owed: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum UniswapV3MintCalleeCalls {
        Mint(MintCall),
        UniswapV3MintCallback(UniswapV3MintCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for UniswapV3MintCalleeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <UniswapV3MintCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3MintCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UniswapV3MintCalleeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3MintCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for UniswapV3MintCalleeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3MintCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MintCall> for UniswapV3MintCalleeCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<UniswapV3MintCallbackCall> for UniswapV3MintCalleeCalls {
        fn from(value: UniswapV3MintCallbackCall) -> Self {
            Self::UniswapV3MintCallback(value)
        }
    }
}
