// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol';
import '@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3MintCallback.sol';
import '@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';
import './PoolAddress.sol';
import "./Printer.sol";
import "./PayableMulticall.sol";

struct swapParams {
    address tokenIn;
    address tokenOut;
    address to;
    uint24 uniswapFee;
    uint256 amount;
}

struct mintParams {
    address tokenIn;
    address tokenOut;
    address to;
    uint24 uniswapFee;
    int24 tickLower;
    int24 tickUpper;
    uint128 amount;
}

struct burnParams {
    address tokenIn;
    address tokenOut;
    uint24 uniswapFee;
    int24 tickLower;
    int24 tickUpper;
    uint128 amount;
}

contract Multiswap is Ownable, IUniswapV3SwapCallback, IUniswapV3MintCallback, PayableMulticall {

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4295128739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

    address factoryUniswapV3;

    constructor(
        address _factory
    ) Ownable(msg.sender) { 
        factoryUniswapV3 = _factory;
    }

    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) private view returns (IUniswapV3Pool) {
        return IUniswapV3Pool(PoolAddress.computeAddress(factoryUniswapV3, PoolAddress.getPoolKey(tokenA, tokenB, fee)));
    }

    function swap(
        swapParams calldata params
    ) external onlyOwner {
        IUniswapV3Pool pool = getPool(params.tokenIn, params.tokenOut, params.uniswapFee);
        bool zeroForOne = params.tokenIn < params.tokenOut;  
        pool.swap(
            params.to,
            zeroForOne,
            int256(params.amount),
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
            abi.encode(params.tokenIn, params.tokenOut, params.uniswapFee)
        );
    }

    function mint(
        mintParams calldata params
    ) external {
        IUniswapV3Pool pool = getPool(params.tokenIn, params.tokenOut, params.uniswapFee);
        pool.mint(
            params.to, 
            params.tickLower, 
            params.tickUpper, 
            params.amount, 
            abi.encode(params.tokenIn, params.tokenOut, params.uniswapFee)
        );
    }  

    function burn(
        burnParams calldata params
    ) external {
        IUniswapV3Pool pool = getPool(params.tokenIn, params.tokenOut, params.uniswapFee);
        pool.burn(
            params.tickLower, 
            params.tickUpper, 
            params.amount
        );
    } 

    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external override {
        (address usdToken, address token, uint24 uniswapFee) = abi.decode(data, (address, address, uint24));
        verifyCallback(factoryUniswapV3, PoolAddress.getPoolKey(usdToken, token, uniswapFee));

        if (amount0Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token0()).transfer(msg.sender, uint256(amount0Delta));
        } else if (amount1Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token1()).transfer(msg.sender, uint256(amount1Delta));
        } else {
            // if both are not gt 0, both must be 0.
            assert(amount0Delta == 0 && amount1Delta == 0);
        }
    }

    function uniswapV3MintCallback(
        uint256 amount0Owed,
        uint256 amount1Owed,
        bytes calldata data
    ) external override {
        (address usdToken, address token, uint24 uniswapFee) = abi.decode(data, (address, address, uint24));
        verifyCallback(factoryUniswapV3, PoolAddress.getPoolKey(usdToken, token, uniswapFee));

        //emit MintCallback(amount0Owed, amount1Owed);
        if (amount0Owed > 0)
            IERC20(IUniswapV3Pool(msg.sender).token0()).transfer(msg.sender, amount0Owed);
        if (amount1Owed > 0)
            IERC20(IUniswapV3Pool(msg.sender).token1()).transfer(msg.sender, amount1Owed);
    }

    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }


}
