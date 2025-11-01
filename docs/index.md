---
layout: default
title: Axionax Protocol Documentation
---

# Axionax Protocol v1.5
**Decentralized Compute Network with Proof-of-Probabilistic-Checking**

Welcome to the official documentation for Axionax Protocol - a next-generation decentralized compute infrastructure powered by novel consensus mechanisms.

## 🚀 Quick Start

### Prerequisites Installation

We provide automated dependency installers for all major platforms:

#### 🐧 Linux (Ubuntu/Debian/CentOS/RHEL/Arch/Alpine)
```bash
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh | bash
```

#### 🪟 Windows (PowerShell as Administrator)
```powershell
irm https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_windows.ps1 | iex
```

#### 🍎 macOS (10.15+)
```bash
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_macos.sh | bash
```

**These scripts install:** Rust, Node.js, Python, Docker, PostgreSQL, Nginx, Redis, and all development tools.

### Build & Run

```bash
# Clone repository
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core

# Build all components
cargo build --release --workspace

# Run tests
python3 tests/integration_simple.py
```

📖 **[Full Getting Started Guide →](../GETTING_STARTED.md)**

- [Getting Started](../GETTING_STARTED.md)
- [Quick Start Guide](../QUICKSTART.md)
- [Build Instructions](./BUILD.md)

## 📚 Core Documentation

### Architecture & Design
- [Architecture Overview](../ARCHITECTURE.md)
- [New Architecture](../NEW_ARCHITECTURE.md)
- [Project Structure](../PROJECT_STRUCTURE.md)

### Core Modules (v1.5)
- **PoPC** - Proof-of-Probabilistic-Checking Consensus
- **ASR** - Auto-Selection Router
- **PPC** - Posted Price Controller
- **DA** - Data Availability Subsystem

### Development
- [API Reference](./API_REFERENCE.md)
- [Testing Guide](../TESTING_GUIDE.md)
- [Contributing Guidelines](../CONTRIBUTING.md)

## 🔐 Security & Governance
- [Security Implementation](../SECURITY.md)
- [Governance Model](../GOVERNANCE.md)
- [Tokenomics](../TOKENOMICS.md)

## 🌐 Testnet
- [Testnet Integration](./TESTNET_INTEGRATION.md)
- [Testnet in a Box](../Axionax_v1.5_Testnet_in_a_Box/)

## 📈 Project Status
- [Current Status](../STATUS.md)
- [Roadmap](../ROADMAP.md)
- [Project Completion](../PROJECT_COMPLETION.md)

## 🔗 Resources
- [GitHub Repository](https://github.com/axionaxprotocol/axionax-core)
- [Open Issues](https://github.com/axionaxprotocol/axionax-core/issues)
- [v1.5 Testnet Milestone](https://github.com/axionaxprotocol/axionax-core/milestone/1)

## 📜 License
Axionax Protocol is open source software. See [LICENSE](../LICENSE) and [LICENSE NOTICE](../LICENSE_NOTICE.md) for details.

---
*Documentation for Axionax Protocol v1.5 Testnet*