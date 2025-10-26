//! Network Layer for Axionax Core
//! 
//! Implements P2P networking using libp2p for:
//! - Block propagation
//! - Transaction propagation
//! - Consensus message distribution
//! - Peer discovery and management

pub mod protocol;
pub mod manager;
pub mod behaviour;
pub mod config;
pub mod error;

pub use manager::NetworkManager;
pub use protocol::{NetworkMessage, MessageType};
pub use config::NetworkConfig;
pub use error::{NetworkError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_module() {
        // Basic sanity test
        assert!(true);
    }
}
