# Axionax Network Layer

Peer-to-peer networking layer for Axionax blockchain using libp2p.

## Overview

The network layer provides decentralized communication capabilities including:

- **Block Propagation**: Efficient distribution of new blocks to all validators
- **Transaction Broadcasting**: Mempool transaction sharing across the network
- **Consensus Messages**: PoPc challenge/proof/vote message routing
- **Peer Discovery**: Automatic peer finding via mDNS and Kademlia DHT
- **Secure Communication**: Encrypted connections using Noise protocol

## Architecture

### Components

1. **NetworkManager** (`manager.rs`)
   - Main entry point for network operations
   - Manages libp2p Swarm lifecycle
   - Handles message publishing and event processing
   - Provides peer management APIs

2. **AxionaxBehaviour** (`behaviour.rs`)
   - Combines multiple libp2p protocols:
     - **Gossipsub**: Message propagation (pub/sub)
     - **mDNS**: Local network peer discovery
     - **Kademlia DHT**: Distributed peer routing
     - **Identify**: Peer information exchange
     - **Ping**: Connection keep-alive

3. **Protocol Types** (`protocol.rs`)
   - Network message definitions
   - Serialization/deserialization
   - Topic routing for gossipsub

4. **Configuration** (`config.rs`)
   - Network presets (dev/testnet/mainnet)
   - Bootstrap nodes
   - Protocol parameters

## Usage

### Basic Initialization

```rust
use network::{NetworkManager, NetworkConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create network manager with testnet configuration
    let config = NetworkConfig::testnet();
    let mut manager = NetworkManager::new(config).await?;
    
    // Start listening and connecting to peers
    manager.start().await?;
    
    // Run event loop
    manager.run().await?;
    
    Ok(())
}
```

### Publishing Messages

```rust
use network::protocol::{NetworkMessage, BlockMessage};

// Create a block message
let block_msg = NetworkMessage::Block(BlockMessage {
    number: 100,
    hash: "0xabcdef...".to_string(),
    parent_hash: "0x123456...".to_string(),
    timestamp: 1234567890,
    proposer: "0xvalidator...".to_string(),
    transactions: vec![],
    state_root: "0xroot...".to_string(),
});

// Publish to network
manager.publish(block_msg)?;
```

### Network Configuration

#### Development Mode
```rust
let config = NetworkConfig::dev();
// Chain ID: 31337
// Port: 30303
// mDNS: enabled
```

#### Testnet Mode
```rust
let config = NetworkConfig::testnet();
// Chain ID: 86137
// Port: 30303
// Bootstrap nodes: configured
```

#### Mainnet Mode
```rust
let config = NetworkConfig::mainnet();
// Chain ID: 86150
// Port: 30303
// Bootstrap nodes: production nodes
```

### Custom Configuration

```rust
use network::{NetworkConfig, ValidationMode};

let mut config = NetworkConfig::default();
config.chain_id = 12345;
config.port = 40404;
config.max_peers = 100;
config.enable_mdns = true;
config.enable_kad = true;
config.validation_mode = ValidationMode::Strict;
config.bootstrap_nodes = vec![
    "/ip4/1.2.3.4/tcp/30303/p2p/QmPeerId...".to_string(),
];

let manager = NetworkManager::new(config).await?;
```

## Message Types

### Block Messages
Propagated via `axionax/blocks` topic:
- Block number, hash, parent hash
- Timestamp and proposer
- Transaction list
- State root

### Transaction Messages
Propagated via `axionax/transactions` topic:
- Transaction hash, from, to
- Value, data, nonce
- Signature

### Consensus Messages
Propagated via `axionax/consensus` topic:
- **Challenge**: PoPc challenge messages
- **Proof**: PoPc proof submissions
- **Vote**: Validator votes

### Status Messages
Propagated via `axionax/status` topic:
- Peer chain height and state
- Sync status

## Protocols

### Gossipsub (Pub/Sub)
- Message-based topics for different data types
- Configurable validation modes (strict/permissive/none)
- 1MB max message size
- Efficient message deduplication

### mDNS (Local Discovery)
- Automatic peer discovery on local networks
- Zero-configuration networking
- Ideal for development and private networks

### Kademlia DHT
- Distributed hash table for peer routing
- Bootstrap node support
- Global peer discovery

### Noise Protocol
- Authenticated encryption (XX handshake)
- Forward secrecy
- Protection against MITM attacks

### Yamux Multiplexing
- Multiple logical streams over single connection
- Efficient connection utilization

## Testing

Run unit tests:
```bash
cargo test -p network --lib
```

Run integration tests:
```bash
cargo test -p network --test integration_test
```

Run all tests:
```bash
cargo test -p network
```

## Performance Considerations

- **Message Size**: Maximum 1MB per message
- **Peer Limits**: Default 50 min, 100 max peers
- **Idle Timeout**: 30 seconds default
- **Gossipsub Cache**: 100 messages default

## Security

- All connections encrypted with Noise protocol
- Message authentication via Ed25519 signatures
- Peer validation via Identify protocol
- Configurable message validation modes

## Future Enhancements

- [ ] Message compression (gzip/zstd)
- [ ] Advanced peer scoring and reputation
- [ ] Bandwidth monitoring and rate limiting
- [ ] Custom transport protocols (QUIC fully tested)
- [ ] NAT traversal (STUN/TURN)
- [ ] Network metrics and monitoring
- [ ] Message prioritization
- [ ] Sharding support for scalability

## Dependencies

- **libp2p 0.53**: Core P2P networking framework
- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **sha3**: Message ID hashing
- **tracing**: Logging

## License

Licensed under Apache 2.0 License. See LICENSE file for details.
