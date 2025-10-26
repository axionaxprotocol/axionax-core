//! Network manager for P2P communication

use futures::StreamExt;
use libp2p::{
    identity::Keypair,
    multiaddr::Protocol,
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm, SwarmBuilder,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::{
    behaviour::AxionaxBehaviour,
    config::NetworkConfig,
    error::{NetworkError, Result},
    protocol::{MessageType, NetworkMessage},
};

/// Network manager handles P2P communication
pub struct NetworkManager {
    swarm: Swarm<AxionaxBehaviour>,
    config: NetworkConfig,
    local_peer_id: PeerId,
    message_tx: mpsc::UnboundedSender<NetworkMessage>,
    message_rx: mpsc::UnboundedReceiver<NetworkMessage>,
}

/// Network events
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// New peer connected
    PeerConnected(PeerId),
    /// Peer disconnected
    PeerDisconnected(PeerId),
    /// New message received
    MessageReceived {
        peer: PeerId,
        message: NetworkMessage,
    },
    /// Message published successfully
    MessagePublished {
        message_id: String,
    },
}

impl NetworkManager {
    /// Create new network manager
    pub async fn new(config: NetworkConfig) -> Result<Self> {
        info!("Initializing network manager");

        // Generate keypair for peer identity
        let keypair = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(keypair.public());
        
        info!("Local peer ID: {}", local_peer_id);

        // Create network behaviour
        let behaviour = AxionaxBehaviour::new(local_peer_id, &config)
            .map_err(|e| NetworkError::InitializationError(e.to_string()))?;

        // Build swarm
        let swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )
            .map_err(|e| NetworkError::InitializationError(e.to_string()))?
            .with_quic()
            .with_behaviour(|_| behaviour)
            .map_err(|e| NetworkError::InitializationError(e.to_string()))?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(config.idle_timeout)
            })
            .build();

        // Create message channels
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Ok(Self {
            swarm,
            config,
            local_peer_id,
            message_tx,
            message_rx,
        })
    }

    /// Start the network manager
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting network manager");

        // Listen on configured address
        let listen_addr: Multiaddr = self.config.listen_multiaddr()
            .parse()
            .map_err(|e| NetworkError::InitializationError(format!("Invalid listen address: {}", e)))?;

        self.swarm.listen_on(listen_addr.clone())
            .map_err(|e| NetworkError::InitializationError(e.to_string()))?;

        info!("Listening on {}", listen_addr);

        // Subscribe to topics
        self.subscribe_to_topics()?;

        // Connect to bootstrap nodes
        self.connect_bootstrap_nodes().await?;

        Ok(())
    }

    /// Subscribe to network topics
    fn subscribe_to_topics(&mut self) -> Result<()> {
        let topics = vec![
            MessageType::Blocks,
            MessageType::Transactions,
            MessageType::Consensus,
            MessageType::Status,
        ];

        for topic in topics {
            let topic_name = topic.topic_name();
            self.swarm.behaviour_mut().subscribe(&topic_name)
                .map_err(|e| NetworkError::SubscriptionError(e.to_string()))?;
            debug!("Subscribed to topic: {}", topic_name);
        }

        Ok(())
    }

    /// Connect to bootstrap nodes
    async fn connect_bootstrap_nodes(&mut self) -> Result<()> {
        if self.config.bootstrap_nodes.is_empty() {
            debug!("No bootstrap nodes configured");
            return Ok(());
        }

        info!("Connecting to {} bootstrap nodes", self.config.bootstrap_nodes.len());

        for node in &self.config.bootstrap_nodes {
            match node.parse::<Multiaddr>() {
                Ok(addr) => {
                    if let Some(Protocol::P2p(peer_id)) = addr.iter().last() {
                        let peer_id = PeerId::from_multihash(peer_id.into())
                            .map_err(|_e| NetworkError::InvalidPeerId(format!("Invalid peer ID")))?;
                        
                        self.swarm.behaviour_mut().add_address(&peer_id, addr.clone());
                        
                        match self.swarm.dial(addr.clone()) {
                            Ok(_) => info!("Dialing bootstrap node: {}", addr),
                            Err(e) => warn!("Failed to dial bootstrap node {}: {}", addr, e),
                        }
                    }
                }
                Err(e) => warn!("Invalid bootstrap node address {}: {}", node, e),
            }
        }

        Ok(())
    }

    /// Publish message to network
    pub fn publish(&mut self, message: NetworkMessage) -> Result<()> {
        let topic = message.message_type().topic_name();
        let data = message.to_bytes()
            .map_err(|e| NetworkError::SerializationError(e))?;

        self.swarm.behaviour_mut().publish(&topic, data)
            .map_err(|e| NetworkError::SendError(e.to_string()))?;

        debug!("Published message to topic: {}", topic);
        Ok(())
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }

    /// Get number of connected peers
    pub fn peer_count(&self) -> usize {
        self.swarm.behaviour().peer_count()
    }

    /// Get list of connected peers
    pub fn connected_peers(&self) -> Vec<PeerId> {
        self.swarm.behaviour().connected_peers()
            .iter()
            .map(|&peer_id| *peer_id)
            .collect()
    }

    /// Run the network event loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Network manager running");

        loop {
            tokio::select! {
                // Handle swarm events
                event = self.swarm.select_next_some() => {
                    if let Err(e) = self.handle_swarm_event(event).await {
                        error!("Error handling swarm event: {}", e);
                    }
                }

                // Handle outgoing messages
                Some(message) = self.message_rx.recv() => {
                    if let Err(e) = self.publish(message) {
                        error!("Error publishing message: {}", e);
                    }
                }
            }
        }
    }

    /// Handle swarm events
    async fn handle_swarm_event(&mut self, event: SwarmEvent<crate::behaviour::AxionaxBehaviourEvent>) -> Result<()> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            SwarmEvent::Behaviour(event) => {
                debug!("Behaviour event: {:?}", event);
                // Handle gossipsub messages here
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("Connected to peer: {}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                info!("Disconnected from peer {}: {:?}", peer_id, cause);
            }
            SwarmEvent::IncomingConnection { send_back_addr, .. } => {
                debug!("Incoming connection from {}", send_back_addr);
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                if let Some(peer_id) = peer_id {
                    warn!("Outgoing connection error to {}: {}", peer_id, error);
                } else {
                    warn!("Outgoing connection error: {}", error);
                }
            }
            SwarmEvent::IncomingConnectionError { send_back_addr, error, .. } => {
                warn!("Incoming connection error from {}: {}", send_back_addr, error);
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_manager_creation() {
        let config = NetworkConfig::dev();
        let manager = NetworkManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_peer_id_generation() {
        let config = NetworkConfig::dev();
        let manager = NetworkManager::new(config).await.unwrap();
        let peer_id = manager.local_peer_id();
        assert!(!peer_id.to_string().is_empty());
    }
}
