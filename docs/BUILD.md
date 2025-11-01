# Axionax Core - Build and Development Guide

> **Version**: v1.6+ (Rust + Python + TypeScript)  
> **Last Updated**: November 2025

## Prerequisites

### Required Software

- **Rust 1.75+**: Install via rustup (https://rustup.rs/)
- **Python 3.10+**: For DeAI modules
- **Node.js 18+**: For TypeScript SDK
- **Make**: (optional) For build automation
- **Docker**: For containerized deployment
- **Git**: For version control

### Verify Installation

```bash
# Check Rust
rustc --version
# Should output: rustc 1.75.x or higher

# Check Cargo
cargo --version

# Check Python
python3 --version
# Should output: Python 3.10.x or higher

# Check Node.js
node --version
# Should output: v18.x.x or higher

# Check Docker
docker --version
# Should output: Docker version 24.x.x ...
```

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core
```

### 2. Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install nightly (recommended for performance)
rustup install nightly
rustup default nightly
```

### 3. Build the Rust Core

**Using Cargo:**
```bash
cargo build --release
```

**Using Make:**
```bash
make build-rust
```

The binary will be at `target/release/axionax-core`

### 4. Build Python DeAI Modules

```bash
cd deai
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

### 5. Build TypeScript SDK

```bash
cd sdk
npm install
npm run build
```

**Build with version info:**
```bash
cargo build --release --features "version-info"
# Or set environment variables
export AXIONAX_VERSION="v1.6.0"
export AXIONAX_COMMIT=$(git rev-parse --short HEAD)
cargo build --release
```

### 6. Verify Build

```bash
./target/release/axionax-core --version
# Should output: axionax-core v1.6.0

# Test Python modules
cd deai
python3 -c "import asr; print('ASR module OK')"
python3 -c "import fraud_detection; print('Fraud detection OK')"

# Test TypeScript SDK
cd sdk
npm test
```

Expected output:
```
Axionax Core v1.6.0
Git Commit: abc1234
Built:      2025-11-01
Rust:       1.75.0
Python:     3.10.x
Node.js:    18.x.x
```

## Building for Multiple Platforms

**Rust Cross-Compilation:**

```bash
# Install cross-compilation targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Build for different platforms
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu
```

Or use Make:
```bash
make build-all
```

## Docker Build

### Build Docker Image

```bash
docker build -t axionax-core:v1.6.0 .
```

### Run in Docker

```bash
docker run -p 8545:8545 -p 30303:30303 \
  -v $(pwd)/data:/home/axionax/.axionax \
  axionax-core:v1.6.0
```

### Using Docker Compose

```bash
docker compose up -d
```

## Development

### Running Tests

**Rust Tests:**
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_consensus

# Run benchmarks
cargo bench
```

**Python Tests:**
```bash
cd deai
source venv/bin/activate
pytest tests/
# or
python -m unittest discover tests/
```

**TypeScript Tests:**
```bash
cd sdk
npm test
npm run test:coverage
```

**Integration Tests:**
```bash
# Run all integration tests
make test-integration

# Or manually
cargo test --features integration
```

### Code Quality

**Rust:**
```bash
# Format code
cargo fmt

# Run linter (clippy)
cargo clippy -- -D warnings

# or
make lint-rust
```

**Python:**
```bash
cd deai
source venv/bin/activate

# Format code
black .
# or
autopep8 --in-place --recursive .

# Lint
pylint deai/
flake8 .

# Type checking
mypy deai/
```

**TypeScript:**
```bash
cd sdk

# Format
npm run format

# Lint
npm run lint

# Type check
npm run type-check
```

### Development Mode

**Run Rust node in dev mode:**
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- start --network testnet --dev

# Or use Make
make dev
```

**Run with watch mode:**
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

**Python development:**
```bash
cd deai
source venv/bin/activate

# Run in dev mode
python3 asr.py --dev

# Or use pytest watch
ptw -- tests/
```

### Hot Reload

**Rust (with cargo-watch):**
```bash
cargo watch -x 'run -- start'
```

**TypeScript SDK:**
```bash
cd sdk
npm run dev  # with watch mode
```

## Project Structure Reference

```
axionax-core/
├── cmd/axionax/              # 🦀 Main Rust application
│   └── src/main.rs
├── core/                     # 🦀 Core Rust modules
│   ├── blockchain/
│   ├── consensus/
│   └── crypto/
├── deai/                     # 🐍 Python DeAI layer
│   ├── asr.py
│   └── fraud_detection.py
├── sdk/                      # 📘 TypeScript SDK
│   └── src/
├── Cargo.toml                # Rust workspace
├── Makefile                  # Build automation
└── README.md
```

## Common Issues

### Rust Build Errors

**Solution:**
```bash
# Clean build cache
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --release
```

### Python Module Not Found

**Solution:**
```bash
cd deai
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### TypeScript Errors

**Solution:**
```bash
cd sdk
rm -rf node_modules package-lock.json
npm install
npm run build
# Clean build artifacts
make clean

# Rebuild
make build
```

### Docker Build Fails

**Solution:**
```bash
# Clean Docker build cache
docker builder prune

# Rebuild image
docker build --no-cache -t axionax-core:v1.5.0 .
```

## IDE Setup

### Visual Studio Code

Install recommended extensions:
- Go (golang.go)
- Docker (ms-azuretools.vscode-docker)

Create `.vscode/settings.json`:
```json
{
  "go.useLanguageServer": true,
  "go.lintTool": "golangci-lint",
  "go.formatTool": "gofmt",
  "editor.formatOnSave": true
}
```

### GoLand

1. Open project directory
2. GoLand will auto-detect Go modules
3. Enable format on save in Settings

## Performance Profiling

### CPU Profiling

```bash
go test -cpuprofile=cpu.prof ./...
go tool pprof cpu.prof
```

### Memory Profiling

```bash
go test -memprofile=mem.prof ./...
go tool pprof mem.prof
```

### Benchmarking

```bash
go test -bench=. -benchmem ./...
```

## Release Process

### 1. Update Version

Edit version in `cmd/axionax/main.go` or use build flags.

### 2. Build Release Binaries

```bash
make build-all
```

### 3. Create Release Package

```bash
# Create tarball
tar -czf axionax-core-v1.5.0-linux-amd64.tar.gz \
  -C build axionax-core-linux-amd64 \
  -C .. README.md LICENSE
```

### 4. Generate Checksums

```bash
sha256sum build/axionax-core-* > checksums.txt
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development workflow and guidelines.

## Support

- **Documentation**: https://docs.axionax.org
- **Discord**: https://discord.gg/axionax
- **GitHub Issues**: https://github.com/axionaxprotocol/axionax-core/issues
