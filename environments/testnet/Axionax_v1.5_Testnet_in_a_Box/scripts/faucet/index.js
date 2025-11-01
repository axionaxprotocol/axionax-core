import express from "express";
import cors from "cors";
import morgan from "morgan";
import rateLimit from "express-rate-limit";
import basicAuth from "basic-auth";
import { ethers } from "ethers";
import dotenv from "dotenv";
dotenv.config();

const app = express();
app.use(cors());
app.use(morgan("tiny"));

const needAuth = process.env.BASIC_AUTH || "";
app.use((req,res,next)=>{
  if(!needAuth) return next();
  const creds = basicAuth(req);
  const [u,p] = needAuth.split(":");
  if(!creds || creds.name!==u || creds.pass!==p){
    res.set("WWW-Authenticate",'Basic realm="faucet"');
    return res.status(401).send("Auth required");
  }
  next();
});

const limiter = rateLimit({ windowMs: 15*60*1000, max: 10 });
app.use(["/request","/request-erc20"], limiter);

const RPC = process.env.RPC_URL || "http://127.0.0.1:8545";
const CHAIN_ID = Number(process.env.CHAIN_ID || 8615);
const PORT = Number(process.env.PORT || 8081);
const PK = process.env.FAUCET_PRIVATE_KEY;
const ERC20 = process.env.ERC20_TOKEN_ADDRESS || "";

if(!PK) console.warn("WARNING: FAUCET_PRIVATE_KEY is missing!");

const provider = new ethers.providers.JsonRpcProvider(RPC);
const wallet = PK ? new ethers.Wallet(PK, provider) : ethers.Wallet.createRandom().connect(provider);
const erc20Abi = [
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
  "function transfer(address to,uint256 amount) returns (bool)"
];
const erc20 = ERC20 ? new ethers.Contract(ERC20, erc20Abi, wallet) : null;

app.get("/health", async (req,res)=>{
  try{
    const [bn, net] = await Promise.all([provider.getBlockNumber(), provider.getNetwork()]);
    res.json({ ok:true, blockNumber: bn, chainId: net.chainId, erc20: ERC20 || null });
  }catch(e){
    res.status(500).json({ ok:false, error: String(e) });
  }
});

app.get("/request", async (req,res)=>{
  try{
    const to = String(req.query.address||"");
    if(!ethers.utils.isAddress(to)) return res.json({ ok:false, error:"invalid address" });
    const amountEth = String(req.query.amountEth || process.env.FAUCET_AMOUNT_ETH || "1");
    const value = ethers.utils.parseEther(amountEth);
    const tx = await wallet.sendTransaction({ to, value });
    const r = await tx.wait();
    res.json({ ok:true, hash: tx.hash, blockNumber: r.blockNumber, amountEth });
  }catch(e){ res.json({ ok:false, error:String(e) }); }
});

app.get("/request-erc20", async (req,res)=>{
  try{
    if(!erc20) return res.json({ ok:false, error:"ERC20 not configured" });
    const to = String(req.query.address||"");
    if(!ethers.utils.isAddress(to)) return res.json({ ok:false, error:"invalid address" });
    const maxDefault = Number(process.env.MAX_ERC20_PER_REQUEST||1000);
    const amount = Number(req.query.amount || maxDefault);
    const [dec, sym] = await Promise.all([erc20.decimals(), erc20.symbol()]);
    const tx = await erc20.transfer(to, ethers.utils.parseUnits(String(amount), dec));
    const r = await tx.wait();
    res.json({ ok:true, hash: tx.hash, blockNumber: r.blockNumber, amount: String(amount), symbol: sym });
  }catch(e){ res.json({ ok:false, error:String(e) }); }
});

app.listen(PORT, "0.0.0.0", ()=>{
  console.log(`Faucet listening on 0.0.0.0:${PORT}, RPC=${RPC}, chainId=${CHAIN_ID}${ERC20?`, ERC20=${ERC20}`:""}`);
});
