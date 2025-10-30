# Stage 1: Builder
FROM rust:1.74 as builder

# Ensure the latest stable version of Rust and Cargo
RUN rustup update stable && rustup default stable && cargo --version

# Create a new empty shell project
RUN USER=root cargo new --bin axionax-node
WORKDIR /axionax-node

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./core/blockchain/Cargo.toml ./core/blockchain/Cargo.toml
COPY ./core/consensus/Cargo.toml ./core/consensus/Cargo.toml
COPY ./core/crypto/Cargo.toml ./core/crypto/Cargo.toml
COPY ./core/network/Cargo.toml ./core/network/Cargo.toml
COPY ./core/node/Cargo.toml ./core/node/Cargo.toml
COPY ./core/rpc/Cargo.toml ./core/rpc/Cargo.toml
COPY ./core/state/Cargo.toml ./core/state/Cargo.toml
COPY ./bridge/rust-python/Cargo.toml ./bridge/rust-python/Cargo.toml
COPY ./cmd/axionax/Cargo.toml ./cmd/axionax/Cargo.toml

# Copy sources
COPY ./core ./core
COPY ./bridge ./bridge
COPY ./cmd ./cmd

# Build the specific example
# Install build dependencies required by some crates (zstd, bindgen/libclang, pkg-config)
RUN apt-get update \
 && apt-get install -y --no-install-recommends clang libclang-dev pkg-config build-essential libzstd-dev llvm-dev ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Ensure bindgen can find libclang (common location on Debian-based images)
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu

RUN cargo build --release --example full_node -p node

# Stage 2: Final image
FROM debian:bookworm-slim

# Install runtime dependencies (SSL, libstdc++ newer versions, libclang)
RUN apt-get update && \
	apt-get install -y --no-install-recommends libssl-dev ca-certificates libstdc++6 clang libclang-dev && \
	rm -rf /var/lib/apt/lists/*

# Set LIBCLANG_PATH to a common lib location
ENV LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu

# Copy the built binary from the builder stage
COPY --from=builder /axionax-node/target/release/examples/full_node /usr/local/bin/axionax-node

# Expose the RPC port
EXPOSE 8545

# Set the entrypoint
CMD ["/usr/local/bin/axionax-node"]
