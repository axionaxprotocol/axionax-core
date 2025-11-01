# Dependency Installation Scripts

Automated installers for setting up Axionax development environment on all major platforms.

## 🎯 Overview

These scripts provide one-command installation of all dependencies needed to build and run Axionax Protocol, including:

- **Rust toolchain** (via rustup)
- **Node.js 20 LTS** (for TypeScript SDK)
- **Python 3.10+** (for DeAI modules)
- **Docker & Docker Compose** (for containerized deployments)
- **PostgreSQL** (for Block Explorer)
- **Nginx** (for reverse proxy)
- **Redis** (for caching)
- **Build tools** (compilers, pkg-config, OpenSSL)
- **Development tools** (VS Code optional, monitoring tools)

## 🐧 Linux

Supports Ubuntu/Debian, CentOS/RHEL/Fedora, Arch/Manjaro, and Alpine Linux with automatic distribution detection.

### Installation

```bash
# One-command installation
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh | bash

# Or download and inspect first (recommended)
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh
chmod +x install_dependencies_linux.sh
./install_dependencies_linux.sh
```

### What Gets Installed

- **Rust**: Latest stable via rustup (with clippy and rustfmt)
- **Node.js**: Version 20 LTS via NodeSource repository
- **Python**: Python 3 with pip and venv
- **Docker**: Latest Docker Engine + Docker Compose
- **PostgreSQL**: Version 14+ (distribution default)
- **Nginx**: Latest stable
- **Certbot**: For Let's Encrypt SSL certificates
- **Build Tools**: 
  - Ubuntu/Debian: `build-essential`, `pkg-config`, `libssl-dev`
  - CentOS/RHEL: `gcc`, `gcc-c++`, `make`, `openssl-devel`
  - Arch: `base-devel`, `openssl`
  - Alpine: `build-base`, `openssl-dev`
- **Monitoring**: `htop`, `net-tools`, `jq`
- **Firewall**: `ufw` (Debian/Ubuntu) or `firewalld` (RHEL/CentOS)

### Post-Installation

The script automatically:
- Adds your user to the `docker` group
- Configures PATH for Rust
- Verifies all installations

You may need to log out and back in for docker group changes to take effect.

## 🪟 Windows

PowerShell script using Chocolatey package manager.

### Requirements

- **Windows 10/11** (64-bit)
- **PowerShell 5.1+** (included in Windows)
- **Administrator privileges**
- **Internet connection**

### Installation

```powershell
# Run PowerShell as Administrator, then:
irm https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_windows.ps1 | iex

# Or download and inspect first (recommended)
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_windows.ps1" -OutFile "install_dependencies_windows.ps1"
.\install_dependencies_windows.ps1
```

### What Gets Installed

- **Chocolatey**: Package manager for Windows
- **Git for Windows**: Version control
- **Visual Studio Code**: IDE (optional)
- **Node.js**: Version 20 LTS
- **Python**: Version 3.11
- **Rust**: Latest stable via rustup
- **Docker Desktop**: Docker with WSL2 backend
- **PostgreSQL**: Version 14
- **Nginx**: Latest stable
- **Visual Studio Build Tools**: Required for Rust compilation
- **OpenSSL**: Libraries for cryptography
- **Node.js Packages**: yarn, typescript, ts-node, wscat
- **Python Packages**: virtualenv, pytest, requests

### Special Features

- **WSL2 Setup**: Automatically enables WSL2 for Docker Desktop
- **Windows Defender Exclusions**: Adds Cargo and Rustup directories for faster builds
- **Installation Verification**: Checks all installed components

### Post-Installation

**Important**: System restart is required after installation for:
- WSL2 to be enabled
- Docker Desktop to work properly
- Environment variables to be updated

## 🍎 macOS

Homebrew-based installer for macOS 10.15 (Catalina) and later.

### Requirements

- **macOS 10.15+** (Catalina, Big Sur, Monterey, Ventura, Sonoma)
- **Apple Silicon (M1/M2/M3)** or **Intel** processor
- **Xcode Command Line Tools** (installed automatically)

### Installation

```bash
# One-command installation
curl -sSL https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_macos.sh | bash

# Or download and inspect first (recommended)
curl -O https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_macos.sh
chmod +x install_dependencies_macos.sh
./install_dependencies_macos.sh
```

### What Gets Installed

- **Homebrew**: Package manager (if not already installed)
- **Xcode Command Line Tools**: Apple development tools
- **Rust**: Latest stable via rustup
- **Node.js**: Version 20 LTS
- **Python**: Version 3.11
- **Docker Desktop**: Docker with native Mac support
- **PostgreSQL**: Version 15
- **Nginx**: Latest stable
- **Redis**: Latest stable
- **Development Tools**: OpenSSL, pkg-config, curl, wget, git
- **Utilities**: jq, htop, tree
- **Visual Studio Code**: Optional installation

### Architecture Support

The script automatically detects:
- **Apple Silicon (M1/M2/M3)**: Uses ARM64 binaries from `/opt/homebrew`
- **Intel**: Uses x86_64 binaries from `/usr/local`

### Post-Installation

- **Docker Desktop**: Start from Applications folder
- **PostgreSQL**: `brew services start postgresql@15`
- **Nginx**: `brew services start nginx`
- **Redis**: `brew services start redis`
- **Shell Configuration**: Automatically adds to `.zshrc` or `.bash_profile`

## 🔍 Verification

All scripts include verification steps that check installed versions:

```bash
# Linux/macOS
rustc --version
cargo --version
node --version
npm --version
python3 --version
pip3 --version
docker --version
psql --version
nginx -v
git --version
```

```powershell
# Windows
rustc --version
cargo --version
node --version
npm --version
python --version
pip --version
docker --version
psql --version
nginx -v
git --version
```

## ⚡ Quick Start After Installation

### 1. Clone Repository

```bash
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core
```

### 2. Build Rust Core

```bash
cargo build --release
```

### 3. Build Python Bindings

```bash
cd bridge/rust-python
pip install maturin
maturin develop --release
cd ../..
```

### 4. Install TypeScript SDK

```bash
cd sdk
npm install
npm run build
cd ..
```

### 5. Run Tests

```bash
# Run all tests
./scripts/run_tests.sh

# Or run individually
cargo test --all
python tests/integration_test.py
```

## 🛠️ Troubleshooting

### Linux: Permission Denied for Docker

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, or run:
newgrp docker
```

### Windows: Execution Policy Error

```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force
```

### Windows: WSL2 Installation Failed

```powershell
# Enable WSL manually
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart

# Restart computer, then set WSL2 as default
wsl --set-default-version 2
```

### macOS: Xcode Command Line Tools Prompt

If the GUI prompt appears, wait for installation to complete before continuing the script.

### Rust Not in PATH

```bash
# Linux/macOS
source $HOME/.cargo/env

# Windows (PowerShell)
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

## 📋 Manual Installation

If you prefer not to use the automated scripts, see [GETTING_STARTED.md](../GETTING_STARTED.md#manual-installation) for manual installation instructions.

## 🔐 Security Considerations

### Reviewing Scripts

We recommend reviewing scripts before running them:

```bash
# Download and inspect
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/install_dependencies_linux.sh
cat install_dependencies_linux.sh

# Run after review
chmod +x install_dependencies_linux.sh
./install_dependencies_linux.sh
```

### What Scripts Do

All scripts:
- Only install software from official sources (rustup.rs, nodejs.org, docker.com, etc.)
- Use package managers (apt, yum, homebrew, chocolatey)
- Don't modify system files beyond package installation
- Don't download or execute arbitrary code
- Are open source and auditable on GitHub

### Root/Administrator Access

Scripts require elevated privileges to:
- Install system packages
- Configure system services (Docker, PostgreSQL, Nginx)
- Modify system PATH
- Enable system features (WSL2 on Windows)

## 📞 Support

If you encounter issues:

1. **Check Prerequisites**: Ensure you meet OS version requirements
2. **Review Error Messages**: Most errors include helpful hints
3. **Consult Documentation**: [GETTING_STARTED.md](../GETTING_STARTED.md)
4. **Ask for Help**:
   - Discord: https://discord.gg/axionax
   - GitHub Issues: https://github.com/axionaxprotocol/axionax-core/issues
   - Email: support@axionax.org

## 📄 License

These scripts are part of Axionax Protocol and licensed under AGPLv3. See [LICENSE](../LICENSE) for details.

---

**Ready to build Axionax?** 🚀

After installation, continue with [GETTING_STARTED.md](../GETTING_STARTED.md) for build instructions.
