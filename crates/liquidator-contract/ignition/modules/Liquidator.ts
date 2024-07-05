import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";
import { getContractAddress, setContractAddress} from "../../utils/helper";

export const liquidatorModule = buildModule("Liquidator", (m) => {
    const factory = getContractAddress("uniswapV3Factory");
    const dataStore = getContractAddress("DataStore");
    const exchangeRouter = getContractAddress("ExchangeRouter");
    const reader = getContractAddress("Reader");
    const liquidator = m.contract("Liquidator", [factory, dataStore, exchangeRouter, exchangeRouter] );
    setContractAddress("Liquidator", liquidator.target);

    return { liquidator };
});

export default liquidatorModule;