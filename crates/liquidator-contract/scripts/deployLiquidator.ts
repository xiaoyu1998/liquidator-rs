import { deployContract, getContractAddress, setContractAddress} from "../utils/helper";

async function main() {
    const [owner] = await ethers.getSigners();
    const factory = getContractAddress("UniswapV3Factory");
    const dataStore = getContractAddress("DataStore");
    const exchangeRouter = getContractAddress("ExchangeRouter");
    const reader = getContractAddress("Reader");
    console.log("UniswapV3Factory", factory);
    console.log("DataStore", dataStore);
    console.log("ExchangeRouter", exchangeRouter);
    console.log("Reader", reader);

    //dex havs add role controller
    const liquidator = await deployContract("Liquidator", [factory, dataStore, exchangeRouter, exchangeRouter] );
    setContractAddress("Liquidator", liquidator.target);
    console.log("Liquidator", liquidator.target);

}


main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error)
    process.exit(1)
  })