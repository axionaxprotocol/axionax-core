# Multi-stage build for Axionax Core
FROM golang:1.21-alpine AS builder

# Install build dependencies
RUN apk add --no-cache git make gcc musl-dev linux-headers

# Set working directory
WORKDIR /build

# Copy go mod files
COPY go.mod go.sum ./
RUN go mod download

# Copy source code
COPY . .

# Build the binary
RUN make build

# Final stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1000 axionax && \
    adduser -D -u 1000 -G axionax axionax

# Set working directory
WORKDIR /home/axionax

# Copy binary from builder
COPY --from=builder /build/build/axionax-core /usr/local/bin/

# Copy default config
COPY config.example.yaml /home/axionax/config.yaml

# Set ownership
RUN chown -R axionax:axionax /home/axionax

# Switch to non-root user
USER axionax

# Create data directory
RUN mkdir -p /home/axionax/.axionax

# Expose ports
EXPOSE 8545 8546 30303 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8545/ || exit 1

# Default command
ENTRYPOINT ["axionax-core"]
CMD ["start", "--network", "testnet"]
