import { readFileSync, writeFileSync, mkdirSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { ethers } from "ethers";
import dotenv from "dotenv";
import solc from "solc";

dotenv.config();
const __filename = fileURLToPath(import.meta.url);
const __dirname  = path.dirname(__filename);

const RPC = process.env.RPC_URL || "http://127.0.0.1:8545";
const PK  = process.env.DEPLOYER_PRIVATE_KEY || process.env.FAUCET_PRIVATE_KEY
         || "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

function compile() {
  // --- อ่าน source และล้าง BOM ---
  const srcPath = path.join(__dirname, "contracts", "AXX.sol");
  let source = readFileSync(srcPath, "utf8");
  if (source.charCodeAt(0) === 0xFEFF) source = source.slice(1); // strip BOM

  // --- SOLC Standard JSON ---
  const fileKey = "AXX.sol";
  const input = {
    language: "Solidity",
    sources: { [fileKey]: { content: source } },
    settings: {
      optimizer: { enabled: true, runs: 200 },
      outputSelection: { "*": { "*": [ "abi", "evm.bytecode", "evm.bytecode.object" ] } }
    }
  };

  const outRaw = solc.compile(JSON.stringify(input));
  let out;
  try { out = JSON.parse(outRaw); }
  catch (e) {
    console.error("Failed to parse solc output:", outRaw);
    throw e;
  }

  // --- ถ้ามี error ให้แสดงแล้วหยุดทันที ---
  if (out.errors && out.errors.length) {
    const errs = out.errors.filter(e => e.severity === "error");
    if (errs.length) {
      console.error("Solc errors:");
      errs.forEach(e => console.error(e.formattedMessage || e.message || JSON.stringify(e)));
      process.exit(1);
    } else {
      // แสดง warning ไว้เผื่อดีบั๊ก
      out.errors.forEach(e => console.warn(e.formattedMessage || e.message || JSON.stringify(e)));
    }
  }

  if (!out.contracts || !out.contracts[fileKey]) {
    console.error("Solc output missing contracts. Debug keys:", Object.keys(out));
    console.error("Full solc output:", JSON.stringify(out, null, 2));
    process.exit(1);
  }

  const artifact = out.contracts[fileKey]["AXX"];
  if (!artifact || !artifact.abi || !artifact.evm || !artifact.evm.bytecode || !artifact.evm.bytecode.object) {
    console.error("Compiled artifact incomplete:", JSON.stringify(artifact, null, 2));
    process.exit(1);
  }

  return { abi: artifact.abi, bytecode: "0x" + artifact.evm.bytecode.object };
}

async function main() {
  console.log("RPC:", RPC);
  const provider = new ethers.providers.JsonRpcProvider(RPC);
  const wallet   = new ethers.Wallet(PK, provider);
  console.log("Deployer:", wallet.address);

  const { abi, bytecode } = compile();
  const factory = new ethers.ContractFactory(abi, bytecode, wallet);
  const initialSupply = ethers.utils.parseUnits("100000000", 18);

  console.log("Deploying AXX...");
  const token = await factory.deploy(initialSupply, wallet.address);
  await token.deployed();
  console.log("AXX deployed at:", token.address);

  const outDir = path.resolve(__dirname, "../ui_config_out");
  mkdirSync(outDir, { recursive: true });
  const cfg = {
    rpcUrl: RPC,
    faucetUrl: process.env.FAUCET_URL || "http://127.0.0.1:8081",
    chainIdHex: "0x21A7",
    erc20: { symbol: "AXX", address: token.address, decimals: 18 }
  };
  writeFileSync(path.join(outDir, "config.json"), JSON.stringify(cfg, null, 2));

  writeFileSync(path.resolve(__dirname, "../..", ".env.axx.tmp"), `ERC20_TOKEN_ADDRESS=${token.address}\n`);
  console.log(">> wrote ui_config_out/config.json and .env.axx.tmp");
}

main().catch(e => { console.error(e); process.exit(1); });
