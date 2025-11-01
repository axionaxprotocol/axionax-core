# Axionax Core v1.6 - Project Structure

Updated: November 1, 2025

## Directory Overview

```
axionax-core/
├── docs/                             # 📚 All project documentation
│   ├── API_REFERENCE.md              # Complete API documentation
│   ├── ARCHITECTURE.md               # Architecture overview
│   ├── BUILD.md                      # Build and development guide
│   ├── CONTRIBUTING.md               # Contribution guidelines
│   ├── GETTING_STARTED.md            # Getting started guide
│   ├── GOVERNANCE.md                 # DAO governance
│   ├── INTEGRATION_*.md              # Integration guides
│   ├── NEW_ARCHITECTURE.md           # v1.6 multi-language architecture
│   ├── PROJECT_COMPLETION.md         # v1.6 completion summary
│   ├── QUICKSTART.md                 # Quick start guide
│   ├── ROADMAP.md                    # Development roadmap
│   ├── SECURITY*.md                  # Security documentation
│   ├── STATUS.md                     # Current project status
│   ├── TESTING_GUIDE.md              # Testing strategies
│   ├── TESTNET_INTEGRATION.md        # Testnet connection guide
│   ├── TOKENOMICS.md                 # Token economics
│   └── ...                           # Other documentation files
│
├── scripts/                          # 🔧 Utility scripts
│   ├── run_tests.sh                  # Main test runner (Linux/macOS)
│   ├── test.ps1                      # Test runner (Windows)
│   ├── quick-test.ps1                # Quick test (Windows)
│   ├── test-quick.ps1                # Alternative quick test (Windows)
│   └── README.md                     # Scripts documentation
│
├── environments/                     # 🌍 Environment configurations
│   ├── mainnet/                      # Mainnet config (reserved)
│   ├── testnet/                      # Testnet configurations
│   │   ├── Axionax_v1.5_Testnet_in_a_Box/  # v1.5 testnet
│   │   └── Axionax_v1.6_Testnet_in_a_Box/  # v1.6 testnet
│   ├── config.example.yaml           # Example configuration
│   ├── docker-compose.yaml           # Docker setup
│   └── README.md                     # Environment documentation
│
├── core/                             # 🦀 Rust core implementation (80%)
│   ├── blockchain/                   # Blockchain logic
│   ├── consensus/                    # PoPC consensus
│   ├── crypto/                       # Cryptography (VRF)
│   ├── network/                      # P2P networking
│   ├── node/                         # Node implementation
│   ├── rpc/                          # RPC server
│   └── state/                        # State management
│
├── bridge/                           # 🔗 Rust-Python bridge
│   └── rust-python/                  # PyO3 bindings
│
├── deai/                             # 🐍 Python DeAI layer (10%)
│   ├── asr.py                        # Auto-Selection Router
│   ├── fraud_detection.py            # Fraud detection ML
│   ├── requirements.txt              # Python dependencies
│   └── README.md                     # DeAI documentation
│
├── sdk/                              # 📦 TypeScript SDK (10%)
│   ├── src/                          # SDK source code
│   ├── package.json                  # NPM configuration
│   └── tsconfig.json                 # TypeScript config
│
├── cmd/                              # 💻 CLI applications
│   └── axionax/                      # Main CLI entry point
│
├── pkg/                              # 📦 Go public packages
│   ├── types/                        # Core data structures
│   └── config/                       # Configuration management
│
├── internal/                         # 🔒 Go private packages
│   ├── popc/                         # PoPC implementation
│   ├── asr/                          # ASR implementation
│   └── ppc/                          # PPC implementation
│
├── tests/                            # 🧪 Test suites
│   ├── integration_test.py           # Integration tests
│   └── integration_simple.py         # Simple integration tests
│
├── tools/                            # 🛠️ Development tools
│   ├── benchmark.py                  # Performance benchmarks
│   └── migrate_go_to_rust.py         # Migration tools
│
├── target/                           # 🎯 Rust build outputs
│   ├── debug/                        # Debug builds
│   └── release/                      # Release builds
│
├── Cargo.toml                        # Rust workspace config
├── Makefile                          # Build automation (multi-language)
├── Dockerfile                        # Docker image definition
├── README.md                         # Main README
├── LICENSE                           # License file
└── .gitignore                        # Git ignore rules
```

## Key Files

### Entry Points

- **`README.md`**: Main project overview and documentation links
- **`cmd/axionax/src/main.rs`**: Main CLI application (Rust)
- **`core/`**: Rust core implementation (primary)
- **`deai/`**: Python DeAI layer with ML components

### Core Logic (Rust v1.6)

- **`core/consensus/`**: PoPC consensus implementation
  - Challenge generation, proof verification, confidence calculation
  
- **`core/blockchain/`**: Blockchain data structures and logic
  - Block, Transaction, State management

- **`core/crypto/`**: Cryptographic primitives
  - VRF, signatures, hashing

- **`core/network/`**: P2P networking (libp2p)
  - Node discovery, message propagation

- **`core/rpc/`**: JSON-RPC server
  - Standard Ethereum RPC + Axionax custom methods

### DeAI Layer (Python)

- **`deai/asr.py`**: Auto-Selection Router
  - ML-powered worker selection
  - VRF-weighted assignment
  - Fairness guarantees

- **`deai/fraud_detection.py`**: Fraud detection
  - Anomaly detection
  - Pattern recognition
  - Anti-collusion mechanisms

### TypeScript SDK

- **`sdk/src/`**: Client library for dApps
  - Job submission
  - Worker management
  - Event subscriptions

### Configuration

- **`environments/config.example.yaml`**: Example configuration
  - PoPC, ASR, PPC, DA, VRF, Consensus settings
  - Network, RPC, Storage configurations

### Documentation

- **`docs/API_REFERENCE.md`**: Complete API documentation
  - Standard Ethereum RPC + Axionax custom methods

- **`docs/NEW_ARCHITECTURE.md`**: v1.6 multi-language architecture
  - Rust, Python, TypeScript integration

- **`docs/INTEGRATION_MIGRATION_GUIDE.md`**: Migration guide
  - Go to Rust migration
  - PyO3 integration
  - Performance benchmarks

- **`environments/README.md`**: Environment setup
  - Testnet and mainnet configurations

## Build System

### Makefile Targets

```bash
make build           # Build Rust core
make build-all       # Build for all platforms
make clean           # Clean build artifacts
make test            # Run all tests (Rust + Python)
make test-coverage   # Generate coverage report
make fmt             # Format code (Rust + Python)
make lint            # Run linters
make deps            # Install dependencies
make dev             # Run development node
make docker-build    # Build Docker image
make benchmark       # Run performance benchmarks
```

### Scripts

- **`scripts/run_tests.sh`**: Unified test script (Linux/macOS)
  - Builds Rust core
  - Compiles Python bindings
  - Runs integration tests
  - Executes benchmarks

- **`scripts/test.ps1`**: Windows test script
  - PowerShell equivalent of run_tests.sh

- **`scripts/quick-test.ps1`**: Quick development tests

### Build Outputs

- **Rust**: `target/debug/` or `target/release/`
- **Python**: `*.so` (Linux), `*.dylib` (macOS), `*.pyd` (Windows)
- **TypeScript**: `sdk/dist/`

### Dependencies

- **Rust**: Defined in `Cargo.toml`
  - Key crates: tokio, serde, libp2p, sha3
  
- **Python**: Defined in `deai/requirements.txt`
  - Key packages: numpy, scikit-learn, pyo3
  
- **TypeScript**: Defined in `sdk/package.json`
  - Key packages: ethers, web3

> **Note**: Go implementation (v1.5) has been fully migrated to Rust (v1.6) for better performance and memory safety.

## Development Status (v1.6)

### ✅ Completed

- [x] Multi-language architecture (Rust + Python + TypeScript)
- [x] Rust core implementation (consensus, crypto, blockchain)
- [x] Python DeAI layer (ASR, fraud detection)
- [x] TypeScript SDK for dApp integration
- [x] PyO3 bridge for Rust-Python integration
- [x] Project structure reorganization
- [x] Documentation overhaul
- [x] Performance benchmarks (3x faster than Go)
- [x] Integration tests (20/20 passing)
- [x] Repository restructure and cleanup

### 🚧 In Progress (v1.7)

- [ ] Network layer (libp2p integration)
- [ ] RocksDB state storage
- [ ] JSON-RPC server implementation
- [ ] WebSocket subscriptions
- [ ] Full testnet deployment

### 📅 Planned (v1.8+)

- [ ] Mainnet preparation
- [ ] Security audit
- [ ] Production deployment scripts
- [ ] Prometheus metrics
- [ ] DAO governance contracts
- [ ] Comprehensive test coverage
- [ ] CI/CD pipeline
- [ ] Guardian nodes (space-based validators)

## Version History

### v1.6 (Current) - Multi-Language Architecture
- **Release**: Q4 2025
- **Focus**: Rust core, Python ML, TypeScript SDK
- **Performance**: 3x faster, 2.67x less memory
- **Status**: ✅ Core complete, ready for network layer

### v1.5 (Legacy) - Go Implementation
- **Release**: Q3 2025
- **Focus**: Pure Go implementation
- **Status**: Archived in `legacy/go-v1.5/`

### v1.7 (Next) - Network Layer
- **Target**: Q1 2026
- **Focus**: libp2p, RocksDB, JSON-RPC
- **Status**: 📅 Planned

## Integration Points

### Testnet v1.6

- **Location**: `environments/testnet/Axionax_v1.6_Testnet_in_a_Box/`
- **RPC**: http://localhost:8545
- **Chain ID**: 86137
- **Setup**: See `environments/testnet/Axionax_v1.6_Testnet_in_a_Box/README.md`

### Testnet v1.5 (Legacy)

- **Location**: `environments/testnet/Axionax_v1.5_Testnet_in_a_Box/`
- **RPC**: http://localhost:8545
- **Chain ID**: 31337 (Anvil local)
- **Smart Contracts**: See `shared/addresses.json`

### External Services

- **Blockscout Explorer**: http://localhost:4001
- **Faucet API**: http://localhost:8081
- **Faucet Web UI**: http://localhost:8080

### Performance Metrics

**Rust v1.6 vs Go v1.5:**

| Operation | Go v1.5 | Rust v1.6 | Improvement |
|-----------|---------|-----------|-------------|
| VRF operations | 8,500 ops/sec | 22,817 ops/sec | **2.68x** |
| Block validation | 1,200 blocks/sec | 3,500 blocks/sec | **2.92x** |
| TX verification | 15,000 tx/sec | 45,000 tx/sec | **3.0x** |
| Memory (idle) | 120 MB | 45 MB | **2.67x less** |

**Python Integration (PyO3):** < 10% overhead

## Data Flow (v1.6)

```
Client Submit Job
    ↓
JSON-RPC Server (Rust)
    ↓
Python ASR Router → Select Worker (ML)
    ↓
Worker Executes Job
    ↓
DA Pre-commit
    ↓
Commit to Chain
    ↓
Delayed VRF (k blocks) - Rust Crypto
    ↓
PoPC Challenge Generation - Rust Consensus
    ↓
Worker Submits Proof
    ↓
Validators Verify - Rust + Python
    ↓
Fraud Detection - Python DeAI
    ↓
Seal Block - Rust Blockchain
    ↓
Settlement & Rewards
```

## Technology Stack

### Rust Core (80%)
- **Consensus**: PoPC implementation
- **Blockchain**: Block, transaction, state management
- **Cryptography**: VRF, signatures, hashing (sha3)
- **Network**: libp2p for P2P communication
- **Storage**: RocksDB integration (planned)
- **RPC**: JSON-RPC server with WebSocket support

### Python DeAI (10%)
- **ASR**: Machine learning worker selection
- **Fraud Detection**: Anomaly detection
- **Anti-Collusion**: Pattern recognition
- **Integration**: PyO3 bindings to Rust

### TypeScript SDK (10%)
- **Client Library**: dApp integration
- **Job Management**: Submit and monitor jobs
- **Worker Interface**: Worker registration and status
- **Event Subscriptions**: Real-time updates

## Security Considerations

- **Private Keys**: Stored in `~/.axionax/keystore/`
- **Configuration**: Never commit real credentials
- **Testnet Only**: This is testnet software (Chain ID 86137)
- **Mainnet Reserved**: Chain ID 86150 (not launched)
- **Code Signing**: All releases should be signed
- **Audit**: Security audit planned before mainnet

⚠️ **WARNING**: Any network claiming to be "Axionax Mainnet" is a SCAM.  
Verify at: https://axionax.org/networks

## Quick Links

### Getting Started
- [Main README](../README.md) - Project overview
- [Getting Started](./GETTING_STARTED.md) - Setup guide
- [Quick Start](./QUICKSTART.md) - Quick start guide

### Architecture
- [Architecture Overview](./ARCHITECTURE.md) - System design
- [New Architecture v1.6](./NEW_ARCHITECTURE.md) - Multi-language design
- [Project Completion](./PROJECT_COMPLETION.md) - v1.6 summary

### Development
- [Build Guide](./BUILD.md) - Building the project
- [Testing Guide](./TESTING_GUIDE.md) - Testing strategies
- [Contributing](./CONTRIBUTING.md) - Contribution guidelines

### Integration
- [Integration Guide](./INTEGRATION_README.md) - Integration overview
- [Migration Guide](./INTEGRATION_MIGRATION_GUIDE.md) - Go to Rust migration
- [Testnet Integration](./TESTNET_INTEGRATION.md) - Testnet setup

### Protocol
- [Tokenomics](./TOKENOMICS.md) - Token economics
- [Governance](./GOVERNANCE.md) - DAO governance
- [Security](./SECURITY.md) - Security policy
- [Roadmap](./ROADMAP.md) - Development roadmap

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for:
- Code style guidelines (Rust, Python, TypeScript)
- PR process
- Testing requirements
- Documentation standards

## License

GNU Affero General Public License v3.0 (AGPLv3) - See [LICENSE](../LICENSE) file

---

**Last updated**: November 1, 2025  
**Version**: 1.6.0-dev  
**Status**: Core complete, network layer in progress
