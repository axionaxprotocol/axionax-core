# Stage 1: Rust Builder
# This stage compiles the Rust core application into a static binary.
FROM rust:1.73-slim-bookworm as rust-builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev

# Create a dummy project to cache dependencies
RUN USER=root cargo new --bin axionax-core
WORKDIR /build/axionax-core

# Copy dependency definitions
COPY Cargo.toml Cargo.lock ./
COPY core/Cargo.toml ./core/
COPY bridge/Cargo.toml ./bridge/
COPY deai/Cargo.toml ./deai/
COPY sdk/Cargo.toml ./sdk/

# Build dependencies to cache them
RUN cargo build --release --workspace

# Copy the actual source code
COPY core/ ./core/
COPY bridge/ ./bridge/
COPY deai/ ./deai/
COPY sdk/ ./sdk/

# Build the final binary
RUN cargo build --release --workspace

# Stage 2: Python Builder
# This stage prepares the Python environment and dependencies.
FROM python:3.11-slim-bookworm as python-builder

WORKDIR /app

# Install Python dependencies
COPY deai/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy Python source code
COPY deai/ ./deai/

# Stage 3: Final Production Image
# This stage combines the Rust binary and Python app into a small final image.
FROM debian:bookworm-slim

# Install runtime dependencies (ca-certificates for HTTPS, libssl for crypto)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN addgroup --system --gid 1000 axionax && \
    adduser --system --uid 1000 --gid 1000 axionax

# Set working directory
WORKDIR /home/axionax

# Copy the compiled Rust binary from the rust-builder stage
COPY --from=rust-builder /build/axionax-core/target/release/axionax-core /usr/local/bin/

# Copy the Python application and dependencies from the python-builder stage
COPY --from=python-builder /app /app

# Copy default config file
COPY config.example.yaml /home/axionax/config.yaml

# Set ownership to the non-root user
RUN chown -R axionax:axionax /home/axionax /app /usr/local/bin/axionax-core

# Switch to the non-root user
USER axionax

# Create data directory
RUN mkdir -p /home/axionax/.axionax

# Expose necessary ports
EXPOSE 8545 8546 30303 9090

# Health check (This will be updated later to use a proper health check endpoint)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8545/ || exit 1

# Default command to start the node
ENTRYPOINT ["axionax-core"]
CMD ["start"]
