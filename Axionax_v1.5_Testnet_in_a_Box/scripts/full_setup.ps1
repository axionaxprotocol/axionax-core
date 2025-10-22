# === full_setup.ps1 ===
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$Here = Split-Path -Parent $MyInvocation.MyCommand.Path
if(-not $Here){ $Here = Get-Location }

Push-Location $Here
try{
  # 0) เตรียมโฟลเดอร์
  @('deployer','deployer\contracts','faucet','ui','blockscout-data','scripts') | %{
    if(-not (Test-Path $_)){ New-Item -ItemType Directory $_ | Out-Null }
  }

  # 1) docker-compose.yml (Ganache + Faucet + UI + Blockscout + Postgres + Deployer)
$compose = @'
services:
  hardhat:
    image: trufflesuite/ganache:v7.9.2
    command:
      - --server.host=0.0.0.0
      - --server.port=8545
      - --chain.chainId=8615
      - --miner.blockTime=2
      - --database.dbPath=/data
      - --wallet.mnemonic
      - test test test test test test test test test test test junk
      - --wallet.totalAccounts=10
      - --wallet.defaultBalance=10000
    ports: ["8545:8545"]
    volumes:
      - ./chain-data:/data
    restart: unless-stopped

  postgres:
    image: postgres:15
    environment:
      POSTGRES_USER: blockscout
      POSTGRES_PASSWORD: blockscout
      POSTGRES_DB: blockscout
    volumes:
      - ./blockscout-data:/var/lib/postgresql/data
    restart: unless-stopped

  blockscout:
    image: ghcr.io/blockscout/blockscout:latest
    depends_on: [postgres]
    environment:
      MIX_ENV: prod
      DATABASE_URL: postgresql://blockscout:blockscout@postgres:5432/blockscout
      ETHEREUM_JSONRPC_HTTP_URL: http://hardhat:8545
      ETHEREUM_JSONRPC_TRACE_URL: http://hardhat:8545
      CHAIN_ID: "8615"
      NETWORK: "Axionax Local Testnet v1.5"
      COIN: "AXX"
      PORT: "4000"
      DISABLE_EXCHANGE_RATES: "true"
    ports: ["4000:4000"]
    restart: unless-stopped

  faucet:
    image: node:18-bullseye
    working_dir: /app
    env_file: [.env]
    environment:
      RPC_URL: http://hardhat:8545
    command: >
      bash -lc "npm i; node index.js"
    ports: ["8081:8081"]
    volumes:
      - ./faucet:/app
    depends_on: [ hardhat ]
    restart: unless-stopped

  ui:
    image: nginx:alpine
    ports: ["8080:80"]
    volumes:
      - ./ui:/usr/share/nginx/html:ro
    depends_on: [ faucet ]
    restart: unless-stopped

  deployer:
    image: node:18-bullseye
    working_dir: /app
    environment:
      RPC_URL: http://hardhat:8545
    command: >
      bash -lc "npm i; node deploy_token.js"
    volumes:
      - ./deployer:/app
    depends_on: [ hardhat ]
    restart: "no"
'@
Set-Content -Encoding UTF8 .\docker-compose.yml $compose

  # 2) .env (ค่าเริ่มต้น)
$pk   = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
$envTxt = @"
RPC_URL=http://hardhat:8545
CHAIN_ID=8615
PORT=8081
FAUCET_PRIVATE_KEY=$pk
# จะถูกเติม ERC20_TOKEN_ADDRESS หลัง deploy
# BASIC_AUTH=admin:password   # เปิดถ้าต้องการล็อก Faucet
# FAUCET_AMOUNT_ETH=1
# MAX_ERC20_PER_REQUEST=1000
"@
Set-Content -Encoding ASCII .\.env $envTxt

  # 3) Faucet (index.js + package.json)
$faucetPkg = @'
{
  "name": "axx-faucet",
  "private": true,
  "type": "module",
  "scripts": { "start": "node index.js" },
  "dependencies": {
    "basic-auth": "^2.0.1",
    "cors": "^2.8.5",
    "dotenv": "^16.4.5",
    "ethers": "^5.7.2",
    "express": "^4.19.2",
    "express-rate-limit": "^7.3.0",
    "morgan": "^1.10.0"
  }
}
'@
Set-Content -Encoding UTF8 .\faucet\package.json $faucetPkg

$faucetIndex = @'
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
'@
Set-Content -Encoding UTF8 .\faucet\index.js $faucetIndex

  # 4) Deployer (AXX.sol + deploy script)
$axx = @'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract AXX {
    string public name = "Axionax Token";
    string public symbol = "AXX";
    uint8  public decimals = 18;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;
    address public owner;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    modifier onlyOwner(){ require(msg.sender==owner,"not owner"); _; }

    constructor(uint256 initialSupply, address to){
        owner = msg.sender;
        _mint(to, initialSupply);
    }

    function _mint(address to, uint256 amount) internal {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "balance");
        unchecked {
            balanceOf[from] -= amount;
            balanceOf[to]   += amount;
        }
        emit Transfer(from, to, amount);
    }
}
'@
Set-Content -Encoding ASCII .\deployer\contracts\AXX.sol $axx

$deployPkg = @'
{
  "name": "axx-deployer",
  "private": true,
  "type": "module",
  "scripts": { "start": "node deploy_token.js" },
  "dependencies": {
    "dotenv": "^16.4.5",
    "ethers": "^5.7.2",
    "solc": "^0.8.30"
  }
}
'@
Set-Content -Encoding UTF8 .\deployer\package.json $deployPkg

$deployJs = @'
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
'@
Set-Content -Encoding UTF8 .\deployer\deploy_token.js $deployJs

  # 5) UI config (ไม่เขียนทับ index.html เดิมของคุณ)
$uiCfg = @'
{
  "name": "Axionax Local Testnet v1.5",
  "rpc": "http://127.0.0.1:8545",
  "chainId": 8615,
  "symbol": "AXX",
  "faucet": "http://127.0.0.1:8081",
  "erc20": "",
  "explorer": "http://127.0.0.1:4000"
}
'@
Set-Content -Encoding ASCII .\ui\config.json $uiCfg

  # 6) ขึ้นบริการหลัก (โหนด + Faucet + UI + Explorer)
docker compose up -d hardhat faucet ui postgres blockscout | Out-Null

  # 7) Deploy ERC-20 แล้วอัปเดตระบบ
docker compose run --rm deployer | Write-Host
if(Test-Path .\.env.axx.tmp){
  Add-Content .\.env (Get-Content .\.env.axx.tmp -Raw)
  $cfgPath = ".\deployer\ui_config_out\config.json"
  if(Test-Path $cfgPath){ Copy-Item $cfgPath .\ui\config.json -Force }
}

  # 8) รีสตาร์ท Faucet/UI ให้อ่านค่าใหม่
docker compose up -d --force-recreate faucet ui | Out-Null

  # 9) ตรวจสุขภาพ
Write-Host "`n=== Health checks ===" -ForegroundColor Cyan
Write-Host ("RPC block: " + (Invoke-RestMethod -Uri http://127.0.0.1:8545 -Method Post -ContentType 'application/json' -Body '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}').result)
Write-Host ("Faucet:   " + (Invoke-RestMethod http://127.0.0.1:8081/health | ConvertTo-Json -Compress))
Write-Host ("Portal:   " + (Invoke-RestMethod http://127.0.0.1:8080/config.json | ConvertTo-Json -Compress))
Write-Host "Explorer: http://127.0.0.1:4000" -ForegroundColor Yellow
Write-Host "`nDone. Open http://127.0.0.1:8080 (Portal) → Add Network/Token → ขอเหรียญ" -ForegroundColor Green

}catch{
  Write-Error $_
}finally{
  Pop-Location | Out-Null
}
# === end full_setup.ps1 ===
