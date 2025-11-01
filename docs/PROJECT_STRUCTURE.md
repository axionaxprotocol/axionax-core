# Axionax Core v1.5 - Project Structure

Updated: October 22, 2025

## Directory Overview

```
axionax-core/
├── cmd/                              # Command-line applications
│   └── axionax/                      # Main CLI entry point
│       └── main.go                   # CLI implementation with Cobra
│
├── pkg/                              # Public packages (importable)
│   ├── types/                        # Core data structures
│   │   └── types.go                  # Job, Worker, Validator, Block types
│   └── config/                       # Configuration management
│       └── config.go                 # Config loading and defaults
│
├── internal/                         # Private packages (internal use only)
│   ├── popc/                         # Proof-of-Probabilistic-Checking
│   │   └── validator.go              # PoPC validation logic
│   ├── asr/                          # Auto-Selection Router
│   │   └── router.go                 # Worker selection algorithm
│   ├── ppc/                          # Posted Price Controller
│   │   └── controller.go             # Dynamic pricing mechanism
│   ├── da/                           # Data Availability (to be implemented)
│   ├── vrf/                          # Verifiable Random Function (TBI)
│   ├── consensus/                    # Consensus engine (TBI)
│   └── execution/                    # Execution engine (TBI)
│
├── docs/                             # Documentation
│   ├── API_REFERENCE.md              # Complete API documentation
│   ├── BUILD.md                      # Build and development guide
│   ├── TESTNET_INTEGRATION.md        # Testnet connection guide
│   ├── POPC.md                       # PoPC documentation (TBD)
│   ├── ASR.md                        # ASR documentation (TBD)
│   ├── PPC.md                        # PPC documentation (TBD)
│   ├── DA.md                         # DA documentation (TBD)
│   └── VRF.md                        # VRF documentation (TBD)
│
├── Axionax_v1.5_Testnet_in_a_Box/   # Local testnet environment
│   ├── docker-compose.yml            # Docker Compose configuration
│   ├── README.md                     # Testnet setup guide
│   ├── chain/                        # Hardhat/Anvil configuration
│   ├── deployer/                     # Contract deployment scripts
│   ├── faucet/                       # Test token faucet
│   ├── ui/                           # Web UI for testnet
│   ├── reverse-proxy/                # Nginx reverse proxy
│   └── shared/                       # Shared data (addresses, etc.)
│
├── build/                            # Build outputs (ignored by git)
│   └── axionax-core                  # Compiled binary
│
├── data/                             # Runtime data (ignored by git)
│   └── .axionax/                     # Node data directory
│
├── go.mod                            # Go module definition
├── go.sum                            # Go dependencies checksums
├── Makefile                          # Build automation
├── Dockerfile                        # Docker image definition
├── docker-compose.yaml               # Docker Compose for node
├── config.example.yaml               # Example configuration
├── .gitignore                        # Git ignore rules
│
├── README.md                         # Main README
├── QUICKSTART.md                     # Quick start guide
├── ARCHITECTURE.md                   # Architecture overview
├── ROADMAP.md                        # Development roadmap
├── TOKENOMICS.md                     # Token economics
├── GOVERNANCE.md                     # DAO governance
├── SECURITY.md                       # Security policy
├── CONTRIBUTING.md                   # Contribution guidelines
├── LICENSE                           # MIT License
│
└── .github/                          # GitHub configuration (TBD)
    └── workflows/                    # CI/CD workflows
```

## Key Files

### Entry Points

- **`cmd/axionax/main.go`**: Main CLI application with all commands
  - Commands: start, version, keys, stake, validator, worker, config

### Core Logic

- **`pkg/types/types.go`**: Core data structures
  - Job, Worker, Validator, Block, Transaction types
  
- **`pkg/config/config.go`**: Configuration management
  - Default config, loading from file/env, validation

- **`internal/popc/validator.go`**: PoPC implementation
  - Challenge generation, proof verification, confidence calculation

- **`internal/asr/router.go`**: Worker selection
  - Scoring algorithm, VRF-weighted selection, quota management

- **`internal/ppc/controller.go`**: Dynamic pricing
  - Price adjustment based on utilization and queue metrics

### Configuration

- **`config.example.yaml`**: Example configuration with all parameters
  - PoPC, ASR, PPC, DA, VRF, Consensus settings

### Documentation

- **`docs/API_REFERENCE.md`**: Complete API documentation
  - Standard Ethereum RPC + Axionax custom methods

- **`docs/BUILD.md`**: Build instructions
  - Prerequisites, building, Docker, development workflow

- **`docs/TESTNET_INTEGRATION.md`**: Testnet setup
  - Connecting to Anvil, using faucet, contract integration

## Build System

### Makefile Targets

```bash
make build           # Build binary
make build-all       # Build for all platforms
make clean           # Clean build artifacts
make test            # Run tests
make test-coverage   # Generate coverage report
make fmt             # Format code
make vet             # Run go vet
make lint            # Run linter
make deps            # Download dependencies
make dev             # Run development node
make docker-build    # Build Docker image
```

### Go Modules

- Main module: `github.com/axionaxprotocol/axionax-core`
- Key dependencies:
  - `github.com/ethereum/go-ethereum` - Ethereum client library
  - `github.com/spf13/cobra` - CLI framework
  - `github.com/spf13/viper` - Configuration management

## Development Status

### ✅ Implemented

- [x] Project structure and build system
- [x] CLI framework with all commands
- [x] Configuration management
- [x] Core type definitions
- [x] PoPC validator (basic implementation)
- [x] ASR router (basic implementation)
- [x] PPC controller (basic implementation)
- [x] Documentation structure
- [x] Docker support
- [x] Testnet integration guide

### 🚧 In Progress

- [ ] Full PoPC implementation with fraud proofs
- [ ] Complete ASR with anti-collusion
- [ ] RPC server implementation
- [ ] Execution engine
- [ ] Consensus mechanism
- [ ] Data availability layer
- [ ] VRF integration

### 📅 Planned

- [ ] WebSocket subscriptions
- [ ] Prometheus metrics
- [ ] DeAI sentinel integration
- [ ] DAO governance contracts
- [ ] Production deployment scripts
- [ ] Comprehensive test coverage
- [ ] CI/CD pipeline

## Integration Points

### Testnet (Anvil)

- **RPC**: http://localhost:8545
- **Chain ID**: 31337
- **Smart Contracts**: See `Axionax_v1.5_Testnet_in_a_Box/shared/addresses.json`

### External Services

- **Blockscout Explorer**: http://localhost:4001
- **Faucet API**: http://localhost:8081
- **Faucet Web UI**: http://localhost:8080

## Data Flow

```
Client Submit Job
    ↓
RPC Server (port 8545)
    ↓
ASR Router → Select Worker
    ↓
Worker Executes Job
    ↓
DA Pre-commit
    ↓
Commit to Chain
    ↓
Delayed VRF (k blocks)
    ↓
PoPC Challenge
    ↓
Worker Submits Proof
    ↓
Validators Verify
    ↓
Seal Block
    ↓
Settlement & Rewards
```

## Security Considerations

- **Private Keys**: Stored in `~/.axionax/keystore/`
- **Configuration**: Never commit real credentials
- **Testnet Only**: This is testnet software, not production-ready

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for:
- Code style guidelines
- PR process
- Testing requirements
- Documentation standards

## License

MIT License - See [LICENSE](../LICENSE) file

---

Last updated: October 22, 2025
