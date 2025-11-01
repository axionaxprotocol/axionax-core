require("@nomicfoundation/hardhat-toolbox");
module.exports = {
  solidity: "0.8.24",
  networks: {
    hardhat: {
      chainId: 8615,
      mining: { auto: true, interval: 2000 },
      accounts: { count: 10 }
    }
  }
};
