# Axionax Mainnet v1.5 Architecture → Current Stack Compatibility

This document maps the requested mainnet architecture (v1.5 baseline) to what exists in this repository (local testnet-in-a-box), highlights gaps, and proposes a phased migration.

## Summary
- What we HAVE now (this repo): EVM execution via Anvil, Blockscout (backend/frontend), Faucet, Edge reverse-proxy with HTTPS+CORS, UI portal, and a Proto JSON-RPC mock layer to demo whitepaper calls.
- What is MISSING for mainnet: Custom consensus (PoPC-PoS), P2P/libp2p stack, DA nodes/attestations, precompiles (BLS/VRF/BLAKE3/PoPC helper), on-chain marketplace modules, governance/DAO, remote-signer with HSM, telemetry stack, and security hardening.

## Layer-by-layer

- L0 Infra
  - Target: HSM/TPM, Secure Boot, FDE, mTLS, WAF.  Status: Not automated here. Provided systemd unit template and Nginx mainnet config to integrate later.

- L1 Protocol Core
  - Crypto: Ed25519 acct, BLS12-381, ECVRF-P256, BLAKE3-256 → Not implemented (current EVM uses secp256k1; no precompiles). Gaps: precompiles and signature scheme changes.
  - P2P libp2p/GossipSub → Not present. Current uses no p2p (Anvil). Needs new node (axionaxd).
  - Consensus (PoPC-PoS) → Not present; Anvil single-node. Requires axionaxd.
  - Execution (EVM) → Present (Anvil) but no custom precompiles.
  - Data Availability (DA) → Not present; no blob/RS/attest modules.
  - Settlement/fees → EIP-1559 present in EVM baseline; Base_t PID and slashing economics not implemented.

- L2 System Services
  - Explorer/Indexer/Faucet → Present. Ready for mainnet with ChainID 8615 when node exists.
  - Remote Signer/Key Ceremony/Genesis Publisher → Not implemented. Added placeholders (systemd, signer service in compose). Requires real binaries/PKI.
  - Telemetry (Prometheus/Grafana/Loki/OTLP/SIEM) → Not implemented. Can be added in ops manifests later.

- L3 Axionax Cloud (Marketplace)
  - Job lifecycle/SLA/escrow/pricing → Not present as smart contracts or node modules. Needs on-chain contracts and off-chain services.

- L4 Governance
  - DAO/Upgrade process → Not present. Requires governance contracts and node support for parameter updates.

## Configs added for Mainnet Ops
- ops/configs/genesis.config.yaml — mainnet baseline parameters
- ops/configs/params.toml — DAO-adjustable parameters
- ops/configs/peers.txt — bootstrap peers placeholder
- ops/configs/wallet_add_network.json — MetaMask add-network
- ops/manifests/docker-compose.mainnet.yml — placeholder stack (axionaxd, signer, explorer, edge)
- ops/manifests/nginx.mainnet.conf — mainnet edge config with TLS and rate limits
- ops/manifests/systemd/axionaxd.service — systemd skeleton
- docs/MAINNET_READINESS.md and ops/playbooks/* — readiness and incident playbooks

## RPC/Interfaces
- Required (whitepaper): eth_*, popc_*, da_*, market_*, chain_*, engine_*.
- Current: eth_* via Anvil; proto mock exposes subset of popc_*/da_*/price_* for demos via /proto-rpc.

## Compatibility verdict
- Execution and L2 services are compatible to bootstrap a “shell mainnet” once a real node (axionaxd) exists and exposes 8615 JSON-RPC.
- Core protocol features (PoPC, DA, P2P, precompiles, slashing) are not implemented here; the mainnet manifests act as scaffolding/placeholders only.

## Phased migration plan
1. Node foundation (axionaxd)
   - Implement/bring axionaxd with: libp2p, PoPC-PoS, DA subsystem stubs, EVM with precompiles, engine API.
   - Expose RPC compatible with Blockscout and proto routes. Target ChainID=8615.
2. Security & keys
   - Integrate remote-signer (HSM/TPM), key ceremony automation, mTLS.
3. Telemetry & ops
   - Add Prometheus, Grafana, Loki, OTLP exporters, SIEM hooks; alert rules per P0 metrics.
4. Marketplace & governance
   - Deploy on-chain contracts (JobManager, Bidder, CommitProve, Slasher, OracleSet) and DAO contracts.
   - Wire /market_* RPCs and operational playbooks for slashing/rebate.
5. Public hardening
   - Rate-limits, WAF, quotas; finalize CORS to production domains; implement OIDC/JWT for admin endpoints.

## Quick checks (what you can verify today)
- UI and Blockscout work over the edge proxy (HTTPS).
- /proto-rpc returns mock responses to demo whitepaper calls.
- Wallet add-network for mainnet is prepared under ops/configs (for future RPC). New ChainID won’t work until axionaxd is deployed.