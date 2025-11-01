require("dotenv").config();
const fs = require("fs");
const path = require("path");
const { ethers } = require("hardhat");
const { parse } = require("csv-parse/sync");

(async () => {
  const shared = path.join(__dirname, "..", "..", "shared");
  const addrFile = path.join(shared, "addresses.json");
  if (!fs.existsSync(addrFile)) throw new Error("addresses.json not found — deploy first");
  const { AXX_TOKEN_ADDRESS, VESTING_VAULT_ADDRESS } = JSON.parse(fs.readFileSync(addrFile, "utf8"));

  const csvPath = path.join(__dirname, "..", "genesis", "genesis_allocation_v1_5.csv");
  const csvRaw = fs.readFileSync(csvPath, "utf8");

  // Robust CSV parsing: handle quotes, commas, stray spaces, BOM, and blank lines
  const records = parse(csvRaw, {
    columns: true,
    skip_empty_lines: true,
    trim: true
  });

  // Normalize a row's keys to lookups like address, amount_axx, unlock_type, cliff_months, vest_months
  const normalizeRow = (row) => {
    const out = {};
    for (const [k, v] of Object.entries(row)) {
      const nk = String(k).replace(/\uFEFF/g, "").replace(/\s+/g, "").toLowerCase();
      out[nk] = typeof v === "string" ? v.trim() : v;
    }
    return out;
  };

  const AXX = await ethers.getContractAt("AXXToken", AXX_TOKEN_ADDRESS);
  const Vault = await ethers.getContractAt("AxionaxVestingVault", VESTING_VAULT_ADDRESS);
  const DEC = ethers.BigNumber.from("1000000000000000000");

  const sent = [];
  for (const raw of records) {
    const row = normalizeRow(raw);
    const addr = row.address;
    const amtStr = row.amount_axx ?? row.amountaxx ?? row["amount_axx "] ?? null;
    const unlock = (row.unlock_type || "").toLowerCase();
    const cliff = parseInt(row.cliff_months || "0", 10);
    const vest = parseInt(row.vest_months || "0", 10);

    if (!addr || !ethers.utils.isAddress(addr)) {
      console.warn("Skip row: invalid/missing address ->", raw.address || addr);
      continue;
    }
    if (!amtStr) {
      console.warn("Skip row: amount_AXX missing for", addr);
      continue;
    }

    // Clean amount like " 50,000,000,000 " => 50000000000 (AXX units)
    const cleaned = String(amtStr).replace(/[,\s\"]+/g, "");
    if (!/^\d+$/.test(cleaned)) {
      console.warn("Skip row: amount not numeric after clean ->", amtStr, "=>", cleaned);
      continue;
    }
    const amtAXX = ethers.BigNumber.from(cleaned);
    const wei = amtAXX.mul(DEC);

    if (unlock === "unlocked") {
      const tx = await AXX.transfer(addr, wei);
      const rec = await tx.wait();
      console.log("Transfer", amtAXX.toString(), "AXX ->", addr, "tx:", rec.transactionHash);
      sent.push({ type: "transfer", to: addr, amountAXX: amtAXX.toString(), tx: rec.transactionHash });
    } else {
      // Default vesting: use current timestamp as start if not provided/parsed
      const nowTs = Math.floor(Date.now() / 1000);
      const tx = await Vault.fund(addr, nowTs, cliff || 0, vest || 0, wei, false);
      const rec = await tx.wait();
      console.log("Vesting", amtAXX.toString(), "AXX for", addr, `cliff=${cliff}m vest=${vest}m`, "tx:", rec.transactionHash);
      sent.push({ type: "vesting", to: addr, amountAXX: amtAXX.toString(), cliff, vest, tx: rec.transactionHash });
    }
  }

  // Write a small report next to shared addresses
  const outPath = path.join(shared, `funding_report_${Date.now()}.json`);
  fs.writeFileSync(outPath, JSON.stringify({ when: new Date().toISOString(), count: sent.length, items: sent }, null, 2));
  console.log("Funding from CSV complete. Report:", outPath);
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
