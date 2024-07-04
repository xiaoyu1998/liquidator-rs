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

struct GetLiquidationHealthFactor {
    uint256 healthFactor;
    uint256 healthFactorLiquidationThreshold;
    bool isHealthFactorHigherThanLiquidationThreshold;
    uint256 userTotalCollateralUsd;
    uint256 userTotalDebtUsd;
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

    function getLiquidationHealthFactor(
        address dataStore, 
        address account
    ) external view returns (GetLiquidationHealthFactor memory);
}