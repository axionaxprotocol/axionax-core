import { ethers } from "ethers";
const RPC = process.env.RPC_URL || "http://hardhat:8545";
async function main(){
  const provider = new ethers.providers.JsonRpcProvider(RPC);
  const bn = await provider.getBlockNumber();
  console.log("RPC OK, block:", bn);
  // (placeholder) put your real deployment here later
}
main().catch(e=>{ console.error(e); process.exit(1); });
