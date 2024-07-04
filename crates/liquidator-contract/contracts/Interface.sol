// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

struct LiquidationParams {
    address account;
    address usdToken;
    Asset[] collaterals;
    Asset[] debts;
    uint24 uniswapFee;
    uint256 gasFeeUsd;
}

struct ExecutionLiquidationParams {
    address account;
}

struct Asset {
    address token;
    uint256 amount;
}

interface IExchangeRouter {
    function executeLiquidation(
        ExecutionLiquidationParams calldata params
    ) external payable;
    
}

interface IReader {
    function getDebt(
        address dataStore, 
        address pool,
        address account
    ) external view returns (uint256);  
}