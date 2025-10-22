Axionax v1.5 — Testnet-in-a-Box Readiness Report (2025-10-20)

Overview
This report captures the current local testnet state, endpoints, deployed contracts, and how to operate/restore the stack. It also includes a short checklist to publish public endpoints for external testers.

Stack and Versions (local)
- RPC node: Foundry Anvil (Chain ID 31337), block time 2s
- Explorer backend: Blockscout API (latest) + Postgres 15
- Explorer UI: Blockscout frontend (latest)
- Faucet: Node.js service (express + ethers)
- UI: Nginx serving static web (config.json driven)
- Tooling: Docker 28.4.0, Compose v2.39.4, Node.js 22.20.0, npm 10.9.3, Hardhat 2.22.5, ethers 5.7.2

Local Endpoints (current machine)
- RPC:       http://127.0.0.1:8545
- Explorer API: http://127.0.0.1:4000
- Explorer UI:  http://127.0.0.1:4001
- Faucet API:   http://127.0.0.1:8081
- Web UI:       http://127.0.0.1:8080
- UI proxy to explorer: http://127.0.0.1:8080/blockscout-api/

Deployed Contracts (Chain ID 31337)
- AXXToken:      0x5FbDB2315678afecb367f032d93F642f64180aa3
- VestingVault:  0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
- Record: shared/addresses.json

Genesis Funding
- Script: chain/scripts/fund_from_csv_and_record.js (robust CSV parsing)
- Source CSV: chain/genesis/genesis_allocation_v1_5.csv
- Output Report: shared/funding_report_<timestamp>.json

Operate (local)
1) Start services
   - docker compose up -d
2) Check status
   - docker compose ps
3) Quick health
   - RPC: curl http://127.0.0.1:8545
   - Faucet: curl -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" http://127.0.0.1:8081/health
   - Explorer API: curl "http://127.0.0.1:4000/api/v2/blocks?type=canonical&limit=1"
   - UI proxy: curl "http://127.0.0.1:8080/blockscout-api/api/v2/blocks?type=canonical&limit=1"

Snapshots and Restore
- Created artifacts (2025-10-20):
  - snapshots/config-and-genesis.zip   (shared, ui, chain/genesis)
  - snapshots/data-volumes.zip         (blockscout-data, ganache-data)
  - snapshots/blockscout.dump          (pg_dump custom format)

Restore hints
- Data folders: stop services, restore folders, start services
- Blockscout DB:
  1) docker compose stop blockscout blockscout-frontend
  2) docker compose exec -T postgres bash -lc "dropdb -U blockscout blockscout && createdb -U blockscout blockscout && pg_restore -U blockscout -d blockscout /tmp/blockscout.dump"
  3) docker compose start blockscout blockscout-frontend

Public Readiness Checklist (for external testers)
1) Domain & TLS
   - Obtain subdomains: rpc.<domain>, explorer.<domain>, faucet.<domain>, app.<domain>
   - Terminate TLS (Let’s Encrypt) via Nginx/Traefik/Caddy; ensure HTTPS URLs everywhere
2) Reverse proxy
   - Point public vhosts to the compose services (8545/4000/4001/8081/8080)
   - Preserve/forward headers for Blockscout, and enable gzip
3) CORS and security
   - Faucet: keep BASIC_AUTH; enable rate limit; optionally IP allowlist
   - UI: ensure it fetches the public explorer API (or proxy via same-origin path)
4) Documentation
   - Publish Chain ID (31337), symbol AXX, RPC URL, Explorer, Faucet, AXX token address
   - Provide Join guide (see docs/JOIN_TESTNET.md)
5) Monitoring
   - Add basic liveness probes and log retention

Notes and Known Issues
- Blockscout frontend may restart during first boot while assets are prepared; usually settles after backend migrations.
- Current chain ID is 31337 (commonly used by local Anvil/Hardhat). For a distinct public testnet, consider changing chain ID in docker-compose (anvil --chain-id <id>) and redeploying contracts, then update UI config and docs accordingly.
- Faucet requires FAUCET_PRIVATE_KEY with sufficient funds and ERC20 balance. Update .env, then restart faucet.

Quick Links
- Join instructions: docs/JOIN_TESTNET.md
- Config: ui/config.json, docker-compose.yml, .env (for faucet)
- Deployer scripts: chain/scripts/, deployer/
