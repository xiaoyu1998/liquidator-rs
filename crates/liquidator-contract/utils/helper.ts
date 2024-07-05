export const deployAddresses = {
	localhost : "deployments/localhost_deployed_addresses.json",
	localnet : "deployments/localnet_deployed_addresses.json",
	testnet : "deployments/testnet_deployed_addresses.json",
};

export function getContractAddress(name){
    if (!process.env.HARDHAT_NETWORK){
        process.env.HARDHAT_NETWORK = 'localhost';
    }
    const jsonFile = path.join(__dirname, '..', deployAddresses[`${process.env.HARDHAT_NETWORK}`]);
    const json = JSON.parse(fs.readFileSync(jsonFile, 'utf8'))
    return json[`${name}#${name}`];    
}

export function setContractAddress(name, address){
    if (!process.env.HARDHAT_NETWORK){
        process.env.HARDHAT_NETWORK = 'localhost';
    }
    const jsonFile = path.join(__dirname, '..', deployAddresses[`${process.env.HARDHAT_NETWORK}`]);
    const addresses = JSON.parse(fs.readFileSync(jsonFile, 'utf8'));
    addresses[`${name}#${name}`] = address;
    fs.writeFileSync(jsonFile, addresses, 'utf8');   
}