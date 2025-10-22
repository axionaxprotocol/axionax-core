require("dotenv").config();
require("@nomiclabs/hardhat-ethers");

const RPC_URL = process.env.RPC_URL || "http://127.0.0.1:8545";

module.exports = {
  solidity: { version: "0.8.20", settings: { optimizer: { enabled: true, runs: 200 } } },
  networks: {
    testnet: { url: RPC_URL }
  }
};
