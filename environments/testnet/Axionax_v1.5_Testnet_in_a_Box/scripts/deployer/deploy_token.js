import { readFileSync, mkdirSync, writeFileSync } from "fs";
import { ethers } from "ethers";
import solc from "solc";
import dotenv from "dotenv";
dotenv.config();

const RPC = process.env.RPC_URL || "http://127.0.0.1:8545";
const PK  = process.env.DEPLOYER_PRIVATE_KEY || process.env.FAUCET_PRIVATE_KEY
          || "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

function compile() {
  const source = readFileSync("contracts/AXX.sol", "utf8");
  const input = {
    language: "Solidity",
    sources: { "AXX.sol": { content: source } },
    settings: {
      optimizer: { enabled: true, runs: 200 },
      outputSelection: { "*": { "*": [ "abi", "evm.bytecode.object" ] } }
    }
  };
  const output = JSON.parse(solc.compile(JSON.stringify(input)));
  if(!output.contracts || !output.contracts["AXX.sol"] || !output.contracts["AXX.sol"].AXX){
    console.log("Solc output missing contracts. Debug keys:", Object.keys(output));
    console.log("Full solc output:", JSON.stringify(output, null, 2));
    process.exit(1);
  }
  return output.contracts["AXX.sol"].AXX;
}

async function main(){
  console.log("RPC:", RPC);
  const provider = new ethers.providers.JsonRpcProvider(RPC);
  const wallet = new ethers.Wallet(PK, provider);
  console.log("Deployer:", await wallet.getAddress());

  const { abi, evm } = compile();
  const bytecode = "0x" + evm.bytecode.object;
  const factory = new ethers.ContractFactory(abi, bytecode, wallet);

  console.log("Deploying AXX...");
  const initial = ethers.utils.parseUnits("1000000", 18); // 1,000,000 AXX
  const contract = await factory.deploy(initial, await wallet.getAddress());
  await contract.deployTransaction.wait();

  console.log("AXX deployed at:", contract.address);

  mkdirSync("ui_config_out", { recursive: true });
  const cfg = {
    name: "Axionax Local Testnet v1.5",
    rpc: "http://127.0.0.1:8545",
    chainId: 8615,
    symbol: "AXX",
    faucet: "http://127.0.0.1:8081",
    erc20: contract.address,
    explorer: "http://127.0.0.1:4000"
  };
  writeFileSync("ui_config_out/config.json", JSON.stringify(cfg, null, 2));
  writeFileSync("../.env.axx.tmp", `ERC20_TOKEN_ADDRESS=${contract.address}\r\n`, { encoding: "ascii" });

  console.log(">> wrote ui_config_out/config.json and .env.axx.tmp");
}

main().catch((e)=>{ console.error(e); process.exit(1); });
