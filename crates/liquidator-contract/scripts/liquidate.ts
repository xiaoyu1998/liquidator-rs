import { contractAt, sendTxn } from "../utils/helper";
import { getErrorMsgFromTx, getErrorMsg } from "../utils/error";
import { LiquidationParamsStructOutput, AssetStructOutput } from "../typechain-types/contracts/Liquidator";

async function main() {
    const [owner] = await ethers.getSigners();

    const liquidationParams: LiquidationParamsStruct = {
        account: "0x5aa3B6d49e2AAC9AD7c687C79A899AA6Db2e3cbf",
        usdToken: "0x82ce67E0112D36D2BD267Ed95ABdF64D54D41Fa4",
        collaterals: [
            {
              token: "0x82ce67E0112D36D2BD267Ed95ABdF64D54D41Fa4",
              amount: BigInt("200000000000")
            },
            {
              token: "0x41c04f3310B569841D58AEdeDd9E38102c529b12",
              amount: BigInt("20000000000000000000000")
            }
        ],
        debts: [
            {
              token: "0x82ce67E0112D36D2BD267Ed95ABdF64D54D41Fa4",
              amount: BigInt("100000000000")
            },
            {
              token: "0x41c04f3310B569841D58AEdeDd9E38102c529b12",
              amount: BigInt("10000000000000000000000")
            }
        ],
        uniswapFee: BigInt("3000"),
        gasFeeUsd: BigInt("0")
    };

    const liquidatorAddress = "0x9fc131ccDeAa47F910d7C61569b63fE0C42979dc";
    const liquidator = await contractAt("Liquidator", liquidatorAddress);
    //const estimatedGas = await liquidator.liquidate.estimateGas(liquidationParams);
    // console.log("owner usdt", await usdt.balanceOf(owner)); 
    // console.log("owner uni", await uni.balanceOf(owner)); 
    //console.log(liquidator);
    try {
        await sendTxn(
            liquidator.liquidate(liquidationParams, {gasLimit:3000000}), `liquidator.liquidate(${liquidationParams})`
        );
    } catch(err) {
        console.log("Error:", await getErrorMsgFromTx(err.receipt.hash));
        //console.log("Error:", await getErrorMsg(err.data));
        //console.log(err.data);

        // for (const key in err) {
        //     if (err.hasOwnProperty(key)) {
        //         console.log(`${key}: ${err[key]}`);
        //     }
        // }
    }

}


main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error)
    process.exit(1)
  })