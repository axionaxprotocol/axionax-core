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

# Install build dependencies and libclang in the builder stage so bindgen can run
RUN apt-get update && \
	apt-get install -y --no-install-recommends \
		clang \
		libclang-dev \
		llvm-dev \
		pkg-config \
		libzstd-dev \
		build-essential && \
	# create symlink if needed
	set -eux; \
	for p in /usr/lib/llvm-* /lib/x86_64-linux-gnu /usr/lib; do \
		if ls "$p"/libclang.so* 1>/dev/null 2>&1; then \
			target=$(ls -1 "$p"/libclang.so* | head -n1); \
			ln -sf "$target" /usr/lib/libclang.so || true; \
			break; \
		fi; \
	done && \
	rm -rf /var/lib/apt/lists/*

# Build the specific example
RUN cargo build --release --example full_node -p node

# Stage 2: Final image
FROM debian:bullseye-slim

# Install OpenSSL, which is dynamically linked
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /axionax-node/target/release/examples/full_node /usr/local/bin/axionax-node

# Expose the RPC port
EXPOSE 8545

# Set the entrypoint
CMD ["/usr/local/bin/axionax-node"]
