# Getting Started with Axionax

This guide will help you set up your development environment and get started with Axionax Protocol.

## 📋 Table of Contents

- [System Requirements](#system-requirements)
- [Quick Installation](#quick-installation)
- [Manual Installation](#manual-installation)
- [Building from Source](#building-from-source)
- [Running Tests](#running-tests)
- [Deployment](#deployment)
- [Next Steps](#next-steps)

## 💻 System Requirements

### Minimum Requirements
- **CPU**: 4 cores
- **RAM**: 8 GB
- **Storage**: 50 GB SSD
- **OS**: Linux (Ubuntu 20.04+), Windows 10/11, or macOS 10.15+

### Recommended for Production
- **CPU**: 8+ cores
- **RAM**: 16+ GB
- **Storage**: 200+ GB NVMe SSD
- **Network**: 100+ Mbps bandwidth
- **OS**: Ubuntu 22.04 LTS or Debian 11+

## ⚡ Quick Installation

We provide automated installers for all major platforms. Choose your operating system:

### 🐧 Linux

Supports Ubuntu/Debian, CentOS/RHEL/Fedora, Arch/Manjaro, and Alpine Linux:

```bash
# One-command installation
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh | bash

# Or download and inspect first
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh
chmod +x install_dependencies_linux.sh
./install_dependencies_linux.sh
```

**What gets installed:**
- Rust toolchain (via rustup)
- Node.js 20 LTS
- Python 3.10+
- Docker & Docker Compose
- PostgreSQL 14+
- Nginx
- Certbot (Let's Encrypt)
- Build tools (gcc, make, pkg-config, openssl-dev)
- Monitoring tools (htop, netstat, jq)

### 🪟 Windows

Requires PowerShell 5.1+ and Administrator privileges:

```powershell
# Run PowerShell as Administrator, then:
irm https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_windows.ps1 | iex

# Or download and inspect first
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_windows.ps1" -OutFile "install_dependencies_windows.ps1"
.\install_dependencies_windows.ps1
```

**What gets installed:**
- Chocolatey package manager
- Git for Windows
- Visual Studio Code
- Node.js 20 LTS
- Python 3.11
- Rust (via rustup)
- Docker Desktop
- PostgreSQL 14
- Nginx
- Visual Studio Build Tools (for Rust compilation)
- WSL2 (for Docker backend)

**Note:** System restart required after installation.

### 🍎 macOS

Supports macOS 10.15 (Catalina) and later, including Apple Silicon (M1/M2/M3):

```bash
# One-command installation
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_macos.sh | bash

# Or download and inspect first
curl -O https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_macos.sh
chmod +x install_dependencies_macos.sh
./install_dependencies_macos.sh
```

**What gets installed:**
- Homebrew (if not already installed)
- Xcode Command Line Tools
- Rust toolchain
- Node.js 20 LTS
- Python 3.11
- Docker Desktop
- PostgreSQL 15
- Nginx
- Redis
- Development tools

## 🔧 Manual Installation

If you prefer to install dependencies manually:

### 1. Install Rust

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows (PowerShell)
# Download from: https://rustup.rs/
# Or use: winget install --id Rustlang.Rustup
```

Configure Rust:
```bash
rustup default stable
rustup component add clippy rustfmt
```

### 2. Install Node.js

```bash
# Linux (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# macOS
brew install node@20

# Windows
# Download from: https://nodejs.org/
# Or use: choco install nodejs-lts
```

### 3. Install Python

```bash
# Linux (Ubuntu/Debian)
sudo apt install python3 python3-pip python3-venv

# macOS
brew install python@3.11

# Windows
# Download from: https://www.python.org/
# Or use: choco install python
```

### 4. Install Docker

Follow the official Docker installation guide for your platform:
- Linux: https://docs.docker.com/engine/install/
- macOS: https://docs.docker.com/desktop/install/mac-install/
- Windows: https://docs.docker.com/desktop/install/windows-install/

### 5. Install PostgreSQL

```bash
# Linux (Ubuntu/Debian)
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql@15
brew services start postgresql@15

# Windows
# Download from: https://www.postgresql.org/download/windows/
# Or use: choco install postgresql14
```

### 6. Install Additional Tools

```bash
# Linux (Ubuntu/Debian)
sudo apt install build-essential pkg-config libssl-dev nginx

# macOS
brew install openssl pkg-config nginx

# Windows (PowerShell as Administrator)
choco install nginx
```

## 🏗️ Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core
```

### 2. Build Rust Core

```bash
# Build in release mode
cargo build --release

# The binary will be in target/release/axionax
```

### 3. Build Python Bindings

```bash
# Create Python virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install Python dependencies
cd bridge/rust-python
pip install -r requirements.txt

# Build and install Python bindings
maturin develop --release
```

### 4. Install TypeScript SDK

```bash
cd sdk
npm install
npm run build
```

## 🧪 Running Tests

### Run All Tests

```bash
# Unified test script (tests everything)
./scripts/run_tests.sh
```

### Run Individual Test Suites

```bash
# Rust core tests
cargo test --all

# Python integration tests
python tests/integration_test.py

# Benchmarks
python tools/benchmark.py
```

### Expected Output

```
Running Rust Core Tests...
✓ VRF generation and verification
✓ Consensus engine operations
✓ Validator registration

Running Python Integration Tests...
✓ PyO3 bridge functionality
✓ ASR router operations
✓ Fraud detection

Performance Benchmarks:
- VRF operations: 22,817 ops/sec
- Block validation: 3,500 blocks/sec
- TX verification: 45,000 tx/sec
```

## 🚀 Deployment

### Deploy RPC Node

```bash
# On a VPS server
sudo ./scripts/setup_rpc_node.sh

# Configure your domain
# RPC endpoint: https://testnet-rpc.your-domain.com
# WebSocket: wss://testnet-ws.your-domain.com
```

See [RPC Node Deployment Guide](./docs/RPC_NODE_DEPLOYMENT.md) for details.

### Deploy Block Explorer

```bash
# Deploy Blockscout explorer
sudo ./scripts/setup_explorer.sh

# Access at: https://testnet-explorer.your-domain.com
```

### Deploy Faucet

```bash
# Deploy testnet faucet
sudo ./scripts/setup_faucet.sh

# Access at: https://testnet-faucet.your-domain.com
```

## 📚 Next Steps

### For Developers

1. **Read the Architecture**: [NEW_ARCHITECTURE.md](./NEW_ARCHITECTURE.md)
2. **Try the Examples**: Check out the [Python API docs](./docs/PYTHON_API.md)
3. **Build a dApp**: Use the [TypeScript SDK](./sdk/README.md)
4. **Run a Validator**: Follow the [Validator Setup Guide](./docs/VPS_VALIDATOR_SETUP.md)

### For Validators

1. **Prepare Your VPS**: Use our dependency installers
2. **Generate Keys**: Follow the [Genesis Ceremony Guide](./docs/GENESIS_CEREMONY.md)
3. **Join Testnet**: See [Testnet Launch Guide](./docs/TESTNET_LAUNCH.md)
4. **Monitor Your Node**: Set up monitoring and alerts

### For Contributors

1. **Read Contributing Guide**: [CONTRIBUTING.md](./CONTRIBUTING.md)
2. **Check Open Issues**: https://github.com/axionaxprotocol/axionax-core/issues
3. **Join Discord**: https://discord.gg/axionax
4. **Submit PRs**: We welcome contributions!

## 🆘 Troubleshooting

### Rust Build Fails

```bash
# Update Rust
rustup update stable

# Clean build cache
cargo clean
cargo build --release
```

### Python Bindings Not Found

```bash
# Ensure maturin is installed
pip install maturin

# Rebuild bindings
cd bridge/rust-python
maturin develop --release
```

### Docker Permission Denied (Linux)

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, or run:
newgrp docker
```

### Windows Build Errors

Make sure Visual Studio Build Tools are installed:
```powershell
# Our installer includes this, but if needed:
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools"
```

## 📞 Support

- **Documentation**: https://docs.axionax.org
- **Discord**: https://discord.gg/axionax
- **Telegram**: https://t.me/axionax
- **Email**: support@axionax.org
- **Security Issues**: security@axionax.org

## 📄 License

This project is licensed under AGPLv3. See [LICENSE](./LICENSE) for details.

---

**Ready to build on Axionax?** 🚀

Visit [axionax.org](https://axionax.org) for more information.
