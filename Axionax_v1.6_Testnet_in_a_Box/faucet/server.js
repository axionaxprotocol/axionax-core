const fs = require("fs");
const path = require("path");
const express = require("express");
const { ethers } = require("ethers");

const app = express();
const RPC_URL = process.env.RPC_URL || "http://127.0.0.1:8545";
const pk = process.env.FAUCET_PRIVATE_KEY;
if (!pk) throw new Error("FAUCET_PRIVATE_KEY not set");

const provider = new ethers.providers.JsonRpcProvider(RPC_URL);
const wallet = new ethers.Wallet(pk, provider);

const shared = path.join(__dirname, "..", "shared");
const addrFile = path.join(shared, "addresses.json");
let tokenAddr = null;
if (fs.existsSync(addrFile)) {
  tokenAddr = JSON.parse(fs.readFileSync(addrFile, "utf8")).AXX_TOKEN_ADDRESS;
  console.log("AXX token from addresses.json:", tokenAddr);
}

const AXX_ABI = [
  "function transfer(address to, uint256 value) external returns (bool)",
  "function balanceOf(address account) external view returns (uint256)",
  "function decimals() external view returns (uint8)"
];

const nativeDropWei = ethers.BigNumber.from(process.env.NATIVE_DROP_WEI || "1000000000000000000"); // 1 ETH
const axxDrop = ethers.BigNumber.from(process.env.AXX_DROP_AMOUNT || "10000000000000000000"); // 10 AXX

app.get("/health", (_,res)=>res.json({ok:true, address: wallet.address, token: tokenAddr}));

app.get("/request", async (req, res) => {
  try {
    const to = (req.query.address||"").trim();
    if (!to || !/^0x[a-fA-F0-9]{40}$/.test(to)) return res.status(400).json({error:"invalid address"});

    // send native
    await (await wallet.sendTransaction({ to, value: nativeDropWei })).wait();

    // send AXX
    if (!tokenAddr) throw new Error("Token not found; deployer might not be finished yet");
    const axx = new ethers.Contract(tokenAddr, AXX_ABI, wallet);
    await (await axx.transfer(to, axxDrop)).wait();

    res.json({ ok:true, sent_native: nativeDropWei.toString(), sent_axx: axxDrop.toString(), to });
  } catch (e) {
    console.error(e);
    res.status(500).json({ error: e.message || "failed" });
  }
});

const port = process.env.PORT || 8081;
app.listen(port, ()=>console.log("Faucet running on http://0.0.0.0:"+port));
