import fs from 'fs';
import path from 'path';
import parse from 'csv-parse';

export async function deployContract(name, args, contractOptions = {}) {
    let contractFactory = await ethers.getContractFactory(name, contractOptions);
    let contract = await contractFactory.deploy(...args);
    await contract.waitForDeployment();
    return contract;
}

export const deployAddresses = {
	localnet : "deployments/deployed_addresses.json",
};

export const defaultRpcs = {
    localnet: "http://192.168.2.106:8545",
};

export function getContractAddress(name){
    //console.log("network", process.env.HARDHAT_NETWORK);
    if (!process.env.HARDHAT_NETWORK){
        process.env.HARDHAT_NETWORK = 'localhost';
    }
    //console.log("network", process.env.HARDHAT_NETWORK);
    const jsonFile = path.join(__dirname, '..', deployAddresses[`${process.env.HARDHAT_NETWORK}`]);
    const json = JSON.parse(fs.readFileSync(jsonFile, 'utf8'))
    return json[`${name}#${name}`];    
}

export function setContractAddress(name, address){
    if (!process.env.HARDHAT_NETWORK){
        process.env.HARDHAT_NETWORK = 'localhost';
    }
    const jsonFile = path.join(__dirname, '..', deployAddresses[`${process.env.HARDHAT_NETWORK}`]);
    let addresses = JSON.parse(fs.readFileSync(jsonFile, 'utf8'));
    addresses[`${name}#${name}`] = address;
    fs.writeFileSync(jsonFile, JSON.stringify(addresses, null , 2), 'utf8');   
}