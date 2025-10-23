# Axionax Core

> Layer-1 blockchain for decentralized compute with PoPC consensus, ASR auto-selection, and DeAI security

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.5.0--testnet-blue)](https://github.com/axionaxprotocol/axionax-core/releases)
[![Status](https://img.shields.io/badge/status-testnet-orange)](https://github.com/axionaxprotocol/axionax-core/projects)

## 🎯 Vision

Axionax is a Layer-1 blockchain that unifies **Execution**, **Validation (PoPC)**, **Data Availability**, and **Settlement** with emphasis on performance, transparency, and security for decentralized compute markets.

## ✨ Key Features

- **🎲 Proof-of-Probabilistic-Checking (PoPC)**: Scalable validation through statistical sampling (O(s) verification cost)
- **🤖 Auto-Selection Router (ASR)**: Protocol-driven worker assignment (no auctions) with fairness guarantees
- **💰 Posted Price Controller**: Dynamic pricing based on utilization and queue length
- **🛡️ Advanced Security**: Delayed VRF, stratified sampling, replica diversity, fraud-proof window
- **🤖 DeAI Sentinel**: Real-time anomaly detection and DAO reporting
- **🏛️ DAO Governance**: Community-driven parameter tuning

## 🏗️ Architecture

```mermaid
flowchart LR
    Clients[Users / DApps] --> RPC[RPC Nodes]
    RPC --> Exec[Execution Engine]
    Exec --> PoPC[PoPC Validation]
    PoPC --> DA[Data Availability]
    DA --> Settle[Settlement]
    Settle -.-> Exec
    
    ASR[ASR Router] --> Exec
    PPC[Price Controller] -.-> ASR
    DeAI[DeAI Sentinel] -.-> DAO[Axionax DAO]
    DAO -.-> ASR
    DAO -.-> PPC
```

📖 **[Full Architecture Documentation →](./ARCHITECTURE.md)**

## 🚀 Quick Start

> **🎯 New to Axionax?** Start with our [Quick Start Guide](./QUICKSTART.md) for a step-by-step tutorial!

### Prerequisites

- **Go 1.21+** (for building from source) - [Download](https://go.dev/dl/)
- **Docker Desktop** (for running testnet) - [Download](https://www.docker.com/products/docker-desktop)
- **16GB RAM** minimum
- **100GB SSD** storage (for full node)

### Quick Setup (5 minutes)

```bash
# 1. Clone the repository
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core

# 2. Build from source
make build

# 3. Start local testnet
cd Axionax_v1.5_Testnet_in_a_Box
docker compose up -d
cd ..

# 4. Initialize and start node
./build/axionax-core config init
./build/axionax-core start --network testnet
```

**🎉 That's it!** Your node is now running and connected to the local testnet.

### Testnet Endpoints

Once started, you'll have access to:

| Service | URL | Description |
|---------|-----|-------------|
| **RPC** | http://localhost:8545 | JSON-RPC endpoint |
| **WebSocket** | ws://localhost:8546 | WebSocket endpoint |
| **Explorer** | http://localhost:4001 | Blockscout blockchain explorer |
| **Faucet** | http://localhost:8080 | Test AXX token faucet |
| **Metrics** | http://localhost:9090 | Prometheus metrics |

**Chain ID:** 31337 (Anvil testnet)

### Running a Validator

```bash
# 1. Generate validator keys
./build/axionax-core keys generate --type validator

# 2. Get test AXX from faucet (visit http://localhost:8080)
# Or use curl:
curl -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" \
  "http://localhost:8081/request?address=<your-address>"

# 3. Stake AXX tokens (minimum 10,000 AXX)
./build/axionax-core stake deposit 10000 --address <your-address>

# 4. Start validating
./build/axionax-core validator start

# 5. Check status
./build/axionax-core validator status
```

**Validator Rewards:**
- PoPC validation fees
- Block rewards
- Commission on delegations

### Running a Worker (Compute Provider)

```bash
# 1. Create hardware specification
cat > worker-specs.json <<EOF
{
  "gpus": [{
    "model": "NVIDIA RTX 4090",
    "vram": 24,
    "count": 1
  }],
  "cpu_cores": 16,
  "ram": 64,
  "storage": 1000,
  "bandwidth": 1000,
  "region": "us-west"
}
EOF

# 2. Generate worker keys
./build/axionax-core keys generate --type worker

# 3. Register as worker
./build/axionax-core worker register --specs worker-specs.json

# 4. Start accepting jobs
./build/axionax-core worker start

# 5. Monitor performance
./build/axionax-core worker status
```

**Worker Earnings:**
- Job execution rewards (based on PPC pricing)
- Performance bonuses (high PoPC pass rate)
- Newcomer boosts (via ASR ε-greedy exploration)

## 📊 Tokenomics

- **Total Supply**: 1,000,000,000,000 AXX (1 Trillion, fixed cap)
- **Emission Rate**: ~2.25% APY from Ecosystem Reserve
- **Token Utilities**: Gas fees, Staking, Medium of Exchange, Governance

📖 **[Full Tokenomics Documentation →](./TOKENOMICS.md)**

## 🗺️ Roadmap

| Phase | Timeline | Status | Key Deliverables |
|-------|----------|--------|------------------|
| **v1.5 Testnet** | Q4'25 - Q1'26 | 🟡 In Progress | PoPC, ASR, PPC, DeAI Sentinel MVP |
| **v1.6 Production DA** | Q2'26 | 📅 Planned | Erasure coding, DA optimizer |
| **v1.7 Governance** | Q3'26 | 📅 Planned | DAO hardening, permissionless onboarding |
| **Mainnet Genesis** | Q3'26 - Q2'27 | 📅 Planned | Public launch, SDK/CLI tools |
| **Guardian Nodes** | Q3'28 - Q2'29 | 🔬 Research | Space-based validator nodes |

📖 **[Full Roadmap →](./ROADMAP.md)**

## 📚 Documentation

- **[Project Status](./STATUS.md)** - Current status and next steps
- **[Architecture Overview](./ARCHITECTURE.md)** - System design and component breakdown
- **[Testnet Integration Guide](./docs/TESTNET_INTEGRATION.md)** - Connect to local testnet
- **[API Reference](./docs/API_REFERENCE.md)** - Complete API documentation
- **[Whitepaper v1.5](./docs/whitepaper_v1_5_EN.md)** - Technical specifications
- **[Security Model](./SECURITY.md)** - Threat model and mitigations
- **[Governance Guide](./GOVERNANCE.md)** - DAO participation
- **[Tokenomics](./TOKENOMICS.md)** - Token economics and distribution
- **[Contributing](./CONTRIBUTING.md)** - How to contribute

### Protocol Documentation

- **[PoPC Validation](./docs/POPC.md)** - Proof-of-Probabilistic-Checking
- **[ASR Router](./docs/ASR.md)** - Auto-Selection Router algorithm
- **[PPC Controller](./docs/PPC.md)** - Posted Price Controller mechanism
- **[Data Availability](./docs/DA.md)** - DA layer implementation
- **[VRF Integration](./docs/VRF.md)** - Delayed VRF for security

### Developer Resources

- **[CLI Reference](./docs/CLI.md)** - Command-line interface guide
- **[Go SDK](./docs/GO_SDK.md)** - Go development kit
- **[Smart Contracts](./Axionax_v1.5_Testnet_in_a_Box/chain/contracts/)** - Solidity contracts
- **[Deployment Guide](./Axionax_v1.5_Testnet_in_a_Box/docs/PUBLIC_DEPLOYMENT.md)** - Production deployment

## 🔐 Security

Key security mechanisms:
- **Delayed VRF** (k-block) for challenge selection
- **Stratified + Adaptive Sampling** for coverage
- **Replica Diversity** to prevent collusion
- **Fraud-Proof Window** with validator accountability
- **DA Pre-commit** with live audits

🐛 **Found a vulnerability?** Please report to security@axionax.io (PGP key available)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🔗 Links

- **Website**: https://axionax.io
- **Documentation**: https://docs.axionax.io
- **Discord**: https://discord.gg/axionax
- **Twitter**: https://twitter.com/axionaxprotocol
- **Telegram**: https://t.me/axionax

## 🙏 Acknowledgments

Built with inspiration from:
- Ethereum's execution layer
- Celestia's data availability
- Filecoin's proof mechanisms
- Akash Network's compute marketplace

---

**⚠️ Testnet Disclaimer**: This software is currently in testnet phase. Do not use on mainnet with real assets until official production release.

Made with 💜 by the Axionax community
