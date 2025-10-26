//! libp2p network behaviour implementation

use libp2p::{
    gossipsub::{self, IdentTopic, MessageAuthenticity, ValidationMode},
    identify,
    kad::{self, store::MemoryStore},
    mdns,
    ping,
    swarm::NetworkBehaviour,
    PeerId,
};
use std::time::Duration;

use crate::config::NetworkConfig;

/// Axionax network behaviour combining multiple libp2p protocols
#[derive(NetworkBehaviour)]
pub struct AxionaxBehaviour {
    /// Gossipsub for message propagation
    pub gossipsub: gossipsub::Behaviour,
    /// mDNS for local peer discovery
    pub mdns: mdns::tokio::Behaviour,
    /// Kademlia DHT for peer routing
    pub kad: kad::Behaviour<MemoryStore>,
    /// Identify protocol for peer information exchange
    pub identify: identify::Behaviour,
    /// Ping for connection keep-alive
    pub ping: ping::Behaviour,
}

impl AxionaxBehaviour {
    /// Create new network behaviour
    pub fn new(peer_id: PeerId, config: &NetworkConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Configure Gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(match config.validation_mode {
                crate::config::ValidationMode::None => ValidationMode::None,
                crate::config::ValidationMode::Permissive => ValidationMode::Permissive,
                crate::config::ValidationMode::Strict => ValidationMode::Strict,
            })
            .message_id_fn(|msg| {
                // Use first 20 bytes of message data as ID
                let mut hasher = sha3::Sha3_256::default();
                use sha3::Digest;
                hasher.update(&msg.data);
                let hash = hasher.finalize();
                gossipsub::MessageId::from(&hash[..20])
            })
            .max_transmit_size(1024 * 1024) // 1MB max message size
            .build()?;

        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(
                libp2p::identity::Keypair::generate_ed25519()
            ),
            gossipsub_config,
        )?;

        // Configure mDNS for local discovery
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            peer_id,
        )?;

        // Configure Kademlia DHT
        let mut kad_config = kad::Config::default();
        kad_config.set_query_timeout(Duration::from_secs(60));
        let store = MemoryStore::new(peer_id);
        let mut kad = kad::Behaviour::with_config(peer_id, store, kad_config);

        // Set Kademlia to server mode for bootstrap nodes
        kad.set_mode(Some(kad::Mode::Server));

        // Configure Identify
        let identify = identify::Behaviour::new(
            identify::Config::new(
                format!("/axionax/{}", config.protocol_version),
                libp2p::identity::Keypair::generate_ed25519().public(),
            )
            .with_agent_version(format!("axionax-core/{}", config.protocol_version)),
        );

        // Configure Ping
        let ping = ping::Behaviour::new(
            ping::Config::new()
                .with_interval(Duration::from_secs(30))
                .with_timeout(Duration::from_secs(10)),
        );

        Ok(Self {
            gossipsub,
            mdns,
            kad,
            identify,
            ping,
        })
    }

    /// Subscribe to a gossipsub topic
    pub fn subscribe(&mut self, topic: &str) -> Result<bool, gossipsub::SubscriptionError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.subscribe(&topic)
    }

    /// Publish message to a topic
    pub fn publish(&mut self, topic: &str, data: Vec<u8>) -> Result<gossipsub::MessageId, gossipsub::PublishError> {
        let topic = IdentTopic::new(topic);
        self.gossipsub.publish(topic, data)
    }

    /// Add peer to Kademlia routing table
    pub fn add_address(&mut self, peer_id: &PeerId, addr: libp2p::Multiaddr) {
        self.kad.add_address(peer_id, addr);
    }

    /// Get list of connected peers
    pub fn connected_peers(&self) -> Vec<&PeerId> {
        self.gossipsub.all_peers().map(|(peer_id, _)| peer_id).collect()
    }

    /// Get number of connected peers
    pub fn peer_count(&self) -> usize {
        self.connected_peers().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    #[tokio::test]
    async fn test_behaviour_creation() {
        let keypair = Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        let config = NetworkConfig::dev();

        let behaviour = AxionaxBehaviour::new(peer_id, &config);
        assert!(behaviour.is_ok());
    }
}
