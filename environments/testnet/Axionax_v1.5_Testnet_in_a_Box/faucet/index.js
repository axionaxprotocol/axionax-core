import express from "express";
import cors from "cors";
import morgan from "morgan";
import "dotenv/config";
import { ethers } from "ethers";
import rateLimit from 'express-rate-limit';
import basicAuth from 'basic-auth';

const app = express();
app.use(cors());
app.use(express.json());
app.use(morgan("dev"));

const RPC_URL = process.env.RPC_URL || "http://hardhat:8545";
const PORT = parseInt(process.env.PORT || "8081", 10);
const CHAIN_ID = parseInt(process.env.CHAIN_ID || "8615", 10);
const FAUCET_PRIVATE_KEY = process.env.FAUCET_PRIVATE_KEY;
const ERC20_TOKEN_ADDRESS = process.env.ERC20_TOKEN_ADDRESS || null;
const ERC20_DECIMALS = parseInt(process.env.ERC20_DECIMALS || "18", 10);
const FAUCET_AMOUNT_ETH = process.env.FAUCET_AMOUNT_ETH || "1";
const FAUCET_AMOUNT_ERC20 = process.env.FAUCET_AMOUNT_ERC20 || "1000";

const provider = new ethers.providers.JsonRpcProvider(RPC_URL);
const wallet = FAUCET_PRIVATE_KEY ? new ethers.Wallet(FAUCET_PRIVATE_KEY, provider) : null;

const erc20Abi = [
  "function name() view returns (string)",
  "function symbol() view returns (string)",
  "function decimals() view returns (uint8)",
  "function balanceOf(address) view returns (uint256)",
  "function transfer(address to, uint256 amount) returns (bool)"
];
const erc20 = ERC20_TOKEN_ADDRESS
  ? new ethers.Contract(ERC20_TOKEN_ADDRESS, erc20Abi, wallet ?? provider)
  : null;

const limiter = rateLimit({ windowMs: 15 * 60 * 1000, max: 10 }); // 10 ครั้ง/15 นาที
app.use(['/request','/request-erc20'], limiter);

const needAuth = process.env.BASIC_AUTH || '';
app.use((req, res, next) => {
  if (!needAuth) return next();
  const creds = basicAuth(req);
  const [u, p] = needAuth.split(':');
  if (!creds || creds.name !== u || creds.pass !== p) {
    res.set('WWW-Authenticate','Basic realm="faucet"');
    return res.status(401).send('Auth required');
  }
  next();
});

// Health
app.get("/health", async (_req, res) => {
  try {
    const [bn, net] = await Promise.all([
      provider.getBlockNumber(),
      provider.getNetwork()
    ]);
    res.json({ ok: true, blockNumber: bn, chainId: net.chainId, erc20: ERC20_TOKEN_ADDRESS || null });
  } catch (e) {
    res.status(500).json({ ok: false, error: e.message });
  }
});

// ขอ native (ETH/AXX) จาก faucet
app.get("/request", async (req, res) => {
  try {
    if (!wallet) throw new Error("Faucet wallet not configured");
    const address = req.query.address;
    if (!ethers.utils.isAddress(address)) return res.json({ ok: false, error: "invalid address" });
    const amount = ethers.utils.parseEther(String(FAUCET_AMOUNT_ETH));
    const tx = await wallet.sendTransaction({ to: address, value: amount });
    const rec = await tx.wait();
    res.json({ ok: true, hash: tx.hash, blockNumber: rec.blockNumber, amountEth: String(FAUCET_AMOUNT_ETH) });
  } catch (e) {
    res.status(500).json({ ok: false, error: e.message });
  }
});

// ขอ ERC-20 AXX
app.get("/request-erc20", async (req, res) => {
  try {
    if (!erc20 || !wallet) throw new Error("ERC20 faucet not configured");
    const address = req.query.address;
    const rawAmt = req.query.amount ?? FAUCET_AMOUNT_ERC20;
    if (!ethers.utils.isAddress(address)) return res.json({ ok: false, error: "invalid address" });
    const amount = ethers.utils.parseUnits(String(rawAmt), ERC20_DECIMALS);
    const tx = await erc20.connect(wallet).transfer(address, amount);
    const rec = await tx.wait();
    const sym = await erc20.symbol().catch(() => "AXX");
    res.json({ ok: true, hash: tx.hash, blockNumber: rec.blockNumber, amount: String(rawAmt), symbol: sym });
  } catch (e) {
    res.status(500).json({ ok: false, error: e.message });
  }
});

app.listen(PORT, "0.0.0.0", () => {
  console.log(`Faucet listening on 0.0.0.0:${PORT}, RPC=${RPC_URL}, chainId=${CHAIN_ID}, ERC20=${ERC20_TOKEN_ADDRESS ?? "none"}`);
});
