//! Network configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen address for P2P networking
    pub listen_addr: String,

    /// Port for P2P connections
    pub port: u16,

    /// Maximum number of peers
    pub max_peers: usize,

    /// Minimum number of peers to maintain
    pub min_peers: usize,

    /// Bootstrap nodes for initial peer discovery
    pub bootstrap_nodes: Vec<String>,

    /// Enable mDNS for local peer discovery
    pub enable_mdns: bool,

    /// Enable Kademlia DHT
    pub enable_kad: bool,

    /// Gossipsub message cache size
    pub gossipsub_cache_size: usize,

    /// Message validation mode
    pub validation_mode: ValidationMode,

    /// Connection idle timeout
    pub idle_timeout: Duration,

    /// Network protocol version
    pub protocol_version: String,

    /// Chain ID for network isolation
    pub chain_id: u64,
}

/// Gossipsub validation mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationMode {
    /// No validation (development only)
    None,
    /// Basic signature validation
    Permissive,
    /// Strict validation
    Strict,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".to_string(),
            port: 30303,
            max_peers: 50,
            min_peers: 5,
            bootstrap_nodes: vec![],
            enable_mdns: true,
            enable_kad: true,
            gossipsub_cache_size: 1000,
            validation_mode: ValidationMode::Permissive,
            idle_timeout: Duration::from_secs(60),
            protocol_version: "1.0.0".to_string(),
            chain_id: 86137, // Axionax Testnet
        }
    }
}

impl NetworkConfig {
    /// Create config for testnet
    pub fn testnet() -> Self {
        Self {
            chain_id: 86137,
            bootstrap_nodes: vec![
                // Will be updated with actual bootstrap nodes
                "/dns4/testnet-node1.axionax.org/tcp/30303/p2p/12D3KooW...".to_string(),
            ],
            ..Default::default()
        }
    }

    /// Create config for mainnet
    pub fn mainnet() -> Self {
        Self {
            chain_id: 86150,
            validation_mode: ValidationMode::Strict,
            max_peers: 100,
            bootstrap_nodes: vec![
                // Will be updated with actual bootstrap nodes
                "/dns4/mainnet-node1.axionax.org/tcp/30303/p2p/12D3KooW...".to_string(),
            ],
            ..Default::default()
        }
    }

    /// Create config for local development
    pub fn dev() -> Self {
        Self {
            chain_id: 31337,
            enable_mdns: true,
            max_peers: 10,
            validation_mode: ValidationMode::None,
            bootstrap_nodes: vec![],
            ..Default::default()
        }
    }

    /// Get full listen multiaddr
    pub fn listen_multiaddr(&self) -> String {
        format!("/ip4/{}/tcp/{}", self.listen_addr, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NetworkConfig::default();
        assert_eq!(config.chain_id, 86137);
        assert_eq!(config.port, 30303);
        assert!(config.enable_mdns);
    }

    #[test]
    fn test_testnet_config() {
        let config = NetworkConfig::testnet();
        assert_eq!(config.chain_id, 86137);
        assert!(!config.bootstrap_nodes.is_empty());
    }

    #[test]
    fn test_mainnet_config() {
        let config = NetworkConfig::mainnet();
        assert_eq!(config.chain_id, 86150);
        assert_eq!(config.max_peers, 100);
        assert!(matches!(config.validation_mode, ValidationMode::Strict));
    }

    #[test]
    fn test_listen_multiaddr() {
        let config = NetworkConfig::default();
        assert_eq!(config.listen_multiaddr(), "/ip4/0.0.0.0/tcp/30303");
    }
}
