// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol';
import '@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';
import './Interface.sol';
import './PoolAddress.sol';

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Liquidator is Ownable, IUniswapV3SwapCallback {

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4295128739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

    address factoryUniswapV3;
    address exchangeRouter;
    address reader;
    address dataStore;

    constructor(
        address _factory, 
        address _dataStore,
        address _exchangeRouter, 
        address _reader
    ) Ownable(msg.sender) { 
        factoryUniswapV3 = _factory;
        dataStore = _dataStore;
        exchangeRouter = _exchangeRouter;
        reader = _reader;
    }

    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) private view returns (IUniswapV3Pool) {
        return IUniswapV3Pool(PoolAddress.computeAddress(factoryUniswapV3, PoolAddress.getPoolKey(tokenA, tokenB, fee)));
    }

    function liquidate(
        LiquidationParams calldata params
    ) external onlyOwner returns (int256) {

        uint256 usdBalanceBeforeLiquidate = IERC20(params.usdToken).balanceOf(address(this));

        //buy debts
        for (uint256 i = 0; i < params.debts.length; i ++) {
            Asset memory debt = params.debts[i];
            IUniswapV3Pool pool = getPool(params.usdToken, debt.token, params.uniswapFee);
            bool zeroForOne = params.usdToken < debt.token;
            uint256 debtAmount = IReader(reader).getDebt(dataStore, debt.token, params.account);

            //should check debtAmount is in a range of debt.amount;
            //require(debtAmount <= debt.amount*(1 + 0.1%), "Liquidator: debtAmount is too larger than expectation");

            pool.swap(
                address(this),
                zeroForOne,
                -int256(debtAmount),
                zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
                abi.encode(params.usdToken, debt.token, params.uniswapFee)
            );
        }

        ExecutionLiquidationParams memory executionLiquidationParams = ExecutionLiquidationParams(
           params.account
        );

        IExchangeRouter(exchangeRouter).executeLiquidation(executionLiquidationParams);

        //sell collaterals
        for (uint256 i = 0; i < params.collaterals.length; i ++) {
            Asset memory collateral = params.collaterals[i];
            IUniswapV3Pool pool = getPool(params.usdToken, collateral.token, params.uniswapFee);
            //uint256 collateralAmount = IReader(reader).getCollateral(dataStore, debt.token, params.account);
            bool zeroForOne = collateral.token < params.usdToken;
            pool.swap(
                address(this),
                zeroForOne,
                int256(collateral.amount),
                zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
                abi.encode(params.usdToken, collateral.token, params.uniswapFee)
            );
        }

        uint256 usdBalanceAfterLiquidate = IERC20(params.usdToken).balanceOf(address(this));
        int256 usdGain = int256(usdBalanceAfterLiquidate) - int256(usdBalanceBeforeLiquidate) - int256(params.gasFeeUsd) ;


        require(usdGain > 0, "Liquidator: there is no profit of this liquidation action");
        return usdGain;

    }

    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external override {
        (address usdToken, address token, uint24 uniswapFee) = abi.decode(data, (address, address, uint24));
        verifyCallback(factoryUniswapV3, PoolAddress.getPoolKey(usdToken, token, uniswapFee));

        if (amount0Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token0()).transferFrom(
                address(this), 
                msg.sender, 
                uint256(amount0Delta)
            );
        } else if (amount1Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token1()).transferFrom(
                address(this), 
                msg.sender, 
                uint256(amount1Delta)
            );
        } else {
            // if both are not gt 0, both must be 0.
            assert(amount0Delta == 0 && amount1Delta == 0);
        }
    }

    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }

}
