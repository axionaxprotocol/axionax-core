require("dotenv").config();
const fs = require("fs");
const path = require("path");
const { ethers } = require("hardhat");

(async ()=>{
  const total = process.env.AXX_TOTAL_SUPPLY || "1000000000000000000000000000000";
  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  const AXX = await (await ethers.getContractFactory("AXXToken")).deploy(total);
  await AXX.deployed();
  console.log("AXXToken:", AXX.address);

  const Vault = await (await ethers.getContractFactory("AxionaxVestingVault")).deploy(AXX.address);
  await Vault.deployed();
  console.log("VestingVault:", Vault.address);

  await (await AXX.approve(Vault.address, total)).wait();

  const out = { deployedAt: new Date().toISOString(), chain: "testnet", AXX_TOKEN_ADDRESS: AXX.address, VESTING_VAULT_ADDRESS: Vault.address };
  const shared = path.join(__dirname, "..", "..", "shared");
  if (!fs.existsSync(shared)) fs.mkdirSync(shared, { recursive: true });
  fs.writeFileSync(path.join(shared, "addresses.json"), JSON.stringify(out, null, 2));
  console.log("Wrote shared/addresses.json");
})().catch((e)=>{ console.error(e); process.exit(1); });
