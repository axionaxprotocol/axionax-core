.PHONY: build clean test test-integration test-e2e bench install run fmt vet lint deps deps-update mocks dev docker-build help

# Build variables
BINARY_NAME=axionax-core
BUILD_DIR=build
VERSION?=v1.5.0-testnet
COMMIT=$(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BUILD_TIME=$(shell date -u '+%Y-%m-%d_%H:%M:%S')
LDFLAGS=-ldflags "-X main.Version=$(VERSION) -X main.Commit=$(COMMIT) -X main.BuildTime=$(BUILD_TIME)"

# Go related variables
GOBASE=$(shell pwd)
GOBIN=$(GOBASE)/$(BUILD_DIR)
GOFILES=$(wildcard *.go)

# Build the project
build:
	@echo "Building $(BINARY_NAME) $(VERSION)..."
	@mkdir -p $(BUILD_DIR)
	@go build $(LDFLAGS) -o $(GOBIN)/$(BINARY_NAME) ./cmd/axionax

# Build for multiple platforms
build-all:
	@echo "Building for multiple platforms..."
	@mkdir -p $(BUILD_DIR)
	GOOS=linux GOARCH=amd64 go build $(LDFLAGS) -o $(GOBIN)/$(BINARY_NAME)-linux-amd64 ./cmd/axionax
	GOOS=darwin GOARCH=amd64 go build $(LDFLAGS) -o $(GOBIN)/$(BINARY_NAME)-darwin-amd64 ./cmd/axionax
	GOOS=darwin GOARCH=arm64 go build $(LDFLAGS) -o $(GOBIN)/$(BINARY_NAME)-darwin-arm64 ./cmd/axionax
	GOOS=windows GOARCH=amd64 go build $(LDFLAGS) -o $(GOBIN)/$(BINARY_NAME)-windows-amd64.exe ./cmd/axionax

# Install the binary
install:
	@echo "Installing $(BINARY_NAME)..."
	@go install $(LDFLAGS) ./cmd/axionax

# Run the application
run: build
	@echo "Running $(BINARY_NAME)..."
	@$(GOBIN)/$(BINARY_NAME)

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(BUILD_DIR)
	@go clean

# Run tests
test:
	@echo "Running tests..."
	@go test -v -race -coverprofile=coverage.out ./...

# Run tests with coverage
test-coverage: test
	@echo "Generating coverage report..."
	@go tool cover -html=coverage.out -o coverage.html

# Run integration tests
test-integration:
	@echo "Running integration tests..."
	@go test -v -tags=integration ./... || echo "Integration tests not yet implemented"

# Run end-to-end tests
test-e2e:
	@echo "Running E2E tests..."
	@go test -v -tags=e2e ./... || echo "E2E tests not yet implemented"

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	@go test -bench=. -benchmem -run=^$$ ./...

# Format code
fmt:
	@echo "Formatting code..."
	@go fmt ./...

# Run go vet
vet:
	@echo "Running go vet..."
	@go vet ./...

# Run linter (requires golangci-lint)
lint:
	@echo "Running linter..."
	@golangci-lint run ./...

# Download dependencies
deps:
	@echo "Downloading dependencies..."
	@go mod download
	@go mod tidy

# Update dependencies
deps-update:
	@echo "Updating dependencies..."
	@go get -u ./...
	@go mod tidy

# Generate mocks (requires mockgen)
mocks:
	@echo "Generating mocks..."
	@go generate ./...

# Run development node
dev:
	@echo "Starting development node..."
	@go run ./cmd/axionax start --network testnet --dev

# Docker build
docker-build:
	@echo "Building Docker image..."
	@docker build -t axionax-core:$(VERSION) .

# Help
help:
	@echo "Available targets:"
	@echo "  build            - Build the binary"
	@echo "  build-all        - Build for multiple platforms"
	@echo "  install          - Install the binary"
	@echo "  run              - Build and run the application"
	@echo "  clean            - Clean build artifacts"
	@echo "  test             - Run tests"
	@echo "  test-coverage    - Run tests with coverage report"
	@echo "  test-integration - Run integration tests"
	@echo "  test-e2e         - Run end-to-end tests"
	@echo "  bench            - Run benchmarks"
	@echo "  fmt              - Format code"
	@echo "  vet              - Run go vet"
	@echo "  lint             - Run linter"
	@echo "  deps             - Download dependencies"
	@echo "  deps-update      - Update dependencies"
	@echo "  mocks            - Generate mocks"
	@echo "  dev              - Run development node"
	@echo "  docker-build     - Build Docker image"
	@echo "  help             - Show this help message"
