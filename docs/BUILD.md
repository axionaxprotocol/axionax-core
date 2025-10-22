# Axionax Core - Build and Development Guide

## Prerequisites

### Required Software

- **Go 1.21+**: Download from https://go.dev/dl/
- **Make**: (optional, for using Makefile)
- **Docker**: For containerized deployment
- **Git**: For version control

### Verify Installation

```bash
go version
# Should output: go version go1.21.x ...

docker --version
# Should output: Docker version 24.x.x ...
```

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core
```

### 2. Download Dependencies

```bash
go mod download
go mod tidy
```

### 3. Build the Binary

**Using Make:**
```bash
make build
```

**Using Go directly:**
```bash
go build -o build/axionax-core ./cmd/axionax
```

**Build with version info:**
```bash
go build -ldflags "-X main.Version=v1.5.0 -X main.Commit=$(git rev-parse --short HEAD)" \
  -o build/axionax-core ./cmd/axionax
```

### 4. Verify Build

```bash
./build/axionax-core version
```

Expected output:
```
Axionax Core
Version:    v1.5.0-testnet
Git Commit: abc1234
Built:      2025-10-22_10:30:00
Go Version: go1.21+
```

## Building for Multiple Platforms

```bash
# Linux (AMD64)
GOOS=linux GOARCH=amd64 go build -o build/axionax-core-linux-amd64 ./cmd/axionax

# macOS (Intel)
GOOS=darwin GOARCH=amd64 go build -o build/axionax-core-darwin-amd64 ./cmd/axionax

# macOS (Apple Silicon)
GOOS=darwin GOARCH=arm64 go build -o build/axionax-core-darwin-arm64 ./cmd/axionax

# Windows
GOOS=windows GOARCH=amd64 go build -o build/axionax-core-windows-amd64.exe ./cmd/axionax
```

Or use Make:
```bash
make build-all
```

## Docker Build

### Build Docker Image

```bash
docker build -t axionax-core:v1.5.0 .
```

### Run in Docker

```bash
docker run -p 8545:8545 -p 30303:30303 \
  -v $(pwd)/data:/home/axionax/.axionax \
  axionax-core:v1.5.0
```

### Using Docker Compose

```bash
docker compose up -d
```

## Development

### Running Tests

```bash
# Run all tests
go test ./...

# Run tests with coverage
go test -cover ./...

# Generate coverage report
make test-coverage
# Opens coverage.html in browser
```

### Code Quality

```bash
# Format code
go fmt ./...
# or
make fmt

# Run linter (requires golangci-lint)
golangci-lint run ./...
# or
make lint

# Run go vet
go vet ./...
# or
make vet
```

### Development Mode

```bash
# Run directly without building
go run ./cmd/axionax start --network testnet --dev

# Or use Make
make dev
```

### Hot Reload (with air)

Install air:
```bash
go install github.com/cosmtrek/air@latest
```

Run with hot reload:
```bash
air
```

## Project Structure

```
axionax-core/
├── cmd/
│   └── axionax/          # CLI entry point
│       └── main.go
├── pkg/                  # Public packages
│   ├── types/            # Core types
│   └── config/           # Configuration
├── internal/             # Private packages
│   ├── popc/             # PoPC validation
│   ├── asr/              # ASR router
│   ├── ppc/              # PPC controller
│   ├── da/               # Data availability
│   ├── vrf/              # VRF implementation
│   └── consensus/        # Consensus engine
├── docs/                 # Documentation
├── Axionax_v1.5_Testnet_in_a_Box/  # Testnet environment
├── go.mod                # Go dependencies
├── go.sum
├── Makefile              # Build automation
├── Dockerfile            # Docker image
├── docker-compose.yaml   # Docker Compose config
└── README.md
```

## Common Issues

### Go Not Found

**Solution:** Install Go from https://go.dev/dl/ and add to PATH

### Dependency Errors

**Solution:**
```bash
go clean -modcache
go mod download
go mod tidy
```

### Build Errors

**Solution:**
```bash
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

- **Documentation**: https://docs.axionax.io
- **Discord**: https://discord.gg/axionax
- **GitHub Issues**: https://github.com/axionaxprotocol/axionax-core/issues
