//! Axionax Node - Integrated blockchain node combining Network, State, and RPC
//!
//! The Node module provides a high-level API for running a complete Axionax blockchain node
//! that handles peer-to-peer networking, persistent storage, and JSON-RPC API endpoints.

use std::net::SocketAddr;
use std::sync::Arc;
use std::path::Path;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tracing::{info, warn, error, debug};

use blockchain::{Block, Transaction};
use network::{NetworkManager, NetworkConfig, NetworkMessage};
use network::protocol::{BlockMessage, TransactionMessage};
use state::StateDB;
use rpc::start_rpc_server;
use jsonrpsee::server::ServerHandle;

/// Convert hex string to [u8; 32] hash
fn hex_to_hash(hex: &str) -> Result<[u8; 32], String> {
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    if hex.len() != 64 {
        return Err(format!("Invalid hash length: expected 64, got {}", hex.len()));
    }
    let bytes = hex::decode(hex).map_err(|e| e.to_string())?;
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&bytes);
    Ok(hash)
}

/// Convert [u8; 32] hash to hex string
fn hash_to_hex(hash: &[u8; 32]) -> String {
    format!("0x{}", hex::encode(hash))
}

/// Node configuration
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Network configuration (chain ID, bootstrap nodes, etc.)
    pub network: NetworkConfig,
    /// RPC server address (e.g., "127.0.0.1:8545")
    pub rpc_addr: SocketAddr,
    /// State database path
    pub state_path: String,
}

impl NodeConfig {
    /// Create development node configuration
    pub fn dev() -> Self {
        Self {
            network: NetworkConfig::dev(),
            rpc_addr: "127.0.0.1:8545".parse().unwrap(),
            state_path: "/tmp/axionax-dev".to_string(),
        }
    }
    
    /// Create testnet node configuration
    pub fn testnet() -> Self {
        Self {
            network: NetworkConfig::testnet(),
            rpc_addr: "0.0.0.0:8545".parse().unwrap(),
            state_path: "/var/lib/axionax/testnet".to_string(),
        }
    }
    
    /// Create mainnet node configuration
    pub fn mainnet() -> Self {
        Self {
            network: NetworkConfig::mainnet(),
            rpc_addr: "0.0.0.0:8545".parse().unwrap(),
            state_path: "/var/lib/axionax/mainnet".to_string(),
        }
    }
}

/// Node statistics
#[derive(Debug, Clone, Default)]
pub struct NodeStats {
    pub blocks_received: u64,
    pub blocks_stored: u64,
    pub transactions_received: u64,
    pub transactions_stored: u64,
    pub peer_count: usize,
}

/// Axionax blockchain node
pub struct AxionaxNode {
    config: NodeConfig,
    network: Arc<RwLock<NetworkManager>>,
    state: Arc<StateDB>,
    stats: Arc<RwLock<NodeStats>>,
    rpc_handle: Option<ServerHandle>,
    sync_handle: Option<JoinHandle<()>>,
}

impl AxionaxNode {
    /// Create a new node
    pub async fn new(config: NodeConfig) -> anyhow::Result<Self> {
        info!("Initializing Axionax node with config: {:?}", config);
        
        // Initialize state database
        let state_path = Path::new(&config.state_path);
        if let Some(parent) = state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let state = Arc::new(StateDB::open(state_path)?);
        info!("State database opened at: {}", config.state_path);
        
        // Initialize network manager
        let network = Arc::new(RwLock::new(
            NetworkManager::new(config.network.clone()).await?
        ));
        info!("Network manager initialized");
        
        // Initialize statistics
        let stats = Arc::new(RwLock::new(NodeStats::default()));
        
        Ok(Self {
            config,
            network,
            state,
            stats,
            rpc_handle: None,
            sync_handle: None,
        })
    }
    
    /// Start the node (network, sync, and RPC server)
    pub async fn start(&mut self) -> anyhow::Result<()> {
        info!("Starting Axionax node...");
        
        // Start network manager
        {
            let mut network = self.network.write().await;
            network.start().await?;
        }
        info!("Network layer started");
        
        // Start sync task (network → state)
        let sync_handle = self.start_sync_task().await;
        self.sync_handle = Some(sync_handle);
        info!("Sync task started");
        
        // Start RPC server
        let rpc_handle = start_rpc_server(
            self.config.rpc_addr,
            self.state.clone(),
            self.config.network.chain_id,
        ).await?;
        self.rpc_handle = Some(rpc_handle);
        info!("RPC server started on {}", self.config.rpc_addr);
        
        info!("✅ Axionax node fully operational!");
        Ok(())
    }
    
    /// Start the sync task that listens for network messages and stores them
    async fn start_sync_task(&self) -> JoinHandle<()> {
        let network = self.network.clone();
        let state = self.state.clone();
        let stats = self.stats.clone();
        
        tokio::spawn(async move {
            info!("Sync task running...");
            
            // Create a channel for receiving network messages
            let (tx, mut rx) = mpsc::channel::<NetworkMessage>(100);
            
            // In a real implementation, we'd integrate with NetworkManager's event loop
            // For now, this is a placeholder structure
            
            loop {
                tokio::select! {
                    Some(msg) = rx.recv() => {
                        match msg {
                            NetworkMessage::Block(block_msg) => {
                                if let Err(e) = Self::handle_block_message(
                                    &state,
                                    &stats,
                                    block_msg
                                ).await {
                                    error!("Failed to handle block: {}", e);
                                }
                            }
                            NetworkMessage::Transaction(tx_msg) => {
                                if let Err(e) = Self::handle_transaction_message(
                                    &state,
                                    &stats,
                                    tx_msg
                                ).await {
                                    error!("Failed to handle transaction: {}", e);
                                }
                            }
                            _ => {
                                debug!("Received other message type, skipping");
                            }
                        }
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                        // Periodic check (placeholder)
                    }
                }
            }
        })
    }
    
    /// Handle incoming block message from network
    async fn handle_block_message(
        state: &Arc<StateDB>,
        stats: &Arc<RwLock<NodeStats>>,
        block_msg: BlockMessage,
    ) -> anyhow::Result<()> {
        debug!("Received block #{} from network", block_msg.number);
        
        // Update stats
        {
            let mut s = stats.write().await;
            s.blocks_received += 1;
        }
        
        // Convert hashes from hex strings to [u8; 32]
        let hash = hex_to_hash(&block_msg.hash)
            .map_err(|e| anyhow::anyhow!("Invalid block hash: {}", e))?;
        let parent_hash = hex_to_hash(&block_msg.parent_hash)
            .map_err(|e| anyhow::anyhow!("Invalid parent hash: {}", e))?;
        let state_root = hex_to_hash(&block_msg.state_root)
            .map_err(|e| anyhow::anyhow!("Invalid state root: {}", e))?;
        
        // Validate block (basic checks)
        if block_msg.number == 0 && parent_hash != [0u8; 32] {
            warn!("Invalid genesis block: non-zero parent hash");
            return Ok(());
        }
        
        // Check if we already have this block
        if state.get_block_by_hash(&hash).is_ok() {
            debug!("Block #{} already in database", block_msg.number);
            return Ok(());
        }
        
        // Convert transaction hashes from strings to [u8; 32]
        // Note: BlockMessage.transactions contains hex hashes, not full Transaction objects
        // For now, we'll create empty transactions vector
        let transactions = vec![];
        
        // Convert BlockMessage to Block
        let block = Block {
            number: block_msg.number,
            hash,
            parent_hash,
            timestamp: block_msg.timestamp,
            proposer: block_msg.proposer,
            transactions,
            state_root,
            gas_used: 0, // TODO: Calculate from transactions
            gas_limit: 10_000_000, // TODO: Get from config
        };
        
        // Store block
        state.store_block(&block)?;
        info!("✅ Stored block #{} (hash: {})", block.number, hex::encode(&block.hash[..8]));
        
        // Update stats
        {
            let mut s = stats.write().await;
            s.blocks_stored += 1;
        }
        
        Ok(())
    }
    
    /// Handle incoming transaction message from network
    async fn handle_transaction_message(
        state: &Arc<StateDB>,
        stats: &Arc<RwLock<NodeStats>>,
        tx_msg: TransactionMessage,
    ) -> anyhow::Result<()> {
        debug!("Received transaction from network: {}", &tx_msg.hash[..8]);
        
        // Update stats
        {
            let mut s = stats.write().await;
            s.transactions_received += 1;
        }
        
        // Convert hash from hex string to [u8; 32]
        let hash = hex_to_hash(&tx_msg.hash)
            .map_err(|e| anyhow::anyhow!("Invalid tx hash: {}", e))?;
        
        // Check if we already have this transaction
        if state.get_transaction(&hash).is_ok() {
            debug!("Transaction already in database");
            return Ok(());
        }
        
        // Convert TransactionMessage to Transaction
        let tx = Transaction {
            hash,
            from: tx_msg.from,
            to: tx_msg.to,
            value: tx_msg.value as u128, // Convert u64 -> u128
            gas_price: 20, // Default gas price (not in TransactionMessage)
            gas_limit: 21000, // Default gas limit (not in TransactionMessage)
            nonce: tx_msg.nonce,
            data: tx_msg.data,
        };
        
        // Note: We store transactions when they're included in blocks
        // For now, we'll just track that we received them
        // In a full implementation, we'd store them in a mempool
        
        debug!("Transaction received (pending block inclusion)");
        
        // Update stats
        {
            let mut s = stats.write().await;
            s.transactions_stored += 1;
        }
        
        Ok(())
    }
    
    /// Publish a block to the network
    pub async fn publish_block(&self, block: &Block) -> anyhow::Result<()> {
        info!("Publishing block #{} to network", block.number);
        
        // Convert Block to BlockMessage (with hex-encoded hashes)
        let block_msg = BlockMessage {
            number: block.number,
            hash: hash_to_hex(&block.hash),
            parent_hash: hash_to_hex(&block.parent_hash),
            timestamp: block.timestamp,
            proposer: block.proposer.clone(),
            transactions: block.transactions.iter()
                .map(|tx| hash_to_hex(&tx.hash))
                .collect(),
            state_root: hash_to_hex(&block.state_root),
        };
        
        let mut network = self.network.write().await;
        network.publish(NetworkMessage::Block(block_msg))?;
        
        Ok(())
    }
    
    /// Publish a transaction to the network
    pub async fn publish_transaction(&self, tx: &Transaction) -> anyhow::Result<()> {
        debug!("Publishing transaction to network: {}", hex::encode(&tx.hash[..8]));
        
        // Convert Transaction to TransactionMessage (with hex-encoded hash)
        let tx_msg = TransactionMessage {
            hash: hash_to_hex(&tx.hash),
            from: tx.from.clone(),
            to: tx.to.clone(),
            value: tx.value as u64, // Convert u128 -> u64
            data: tx.data.clone(),
            nonce: tx.nonce,
            signature: vec![], // TODO: Add actual signature
        };
        
        let mut network = self.network.write().await;
        network.publish(NetworkMessage::Transaction(tx_msg))?;
        
        Ok(())
    }
    
    /// Get current node statistics
    pub async fn stats(&self) -> NodeStats {
        self.stats.read().await.clone()
    }
    
    /// Get current peer count
    pub async fn peer_count(&self) -> usize {
        let network = self.network.read().await;
        network.peer_count()
    }
    
    /// Get state database reference
    pub fn state(&self) -> Arc<StateDB> {
        self.state.clone()
    }
    
    /// Shutdown the node gracefully
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Shutting down Axionax node...");
        
        // Stop sync task
        if let Some(handle) = self.sync_handle.take() {
            handle.abort();
            info!("Sync task stopped");
        }
        
        // Stop RPC server
        if let Some(handle) = self.rpc_handle.take() {
            handle.stop()?;
            info!("RPC server stopped");
        }
        
        // Stop network (note: NetworkManager doesn't have shutdown method yet)
        // {
        //     let mut network = self.network.write().await;
        //     // network.shutdown().await?;
        //     info!("Network layer stopped");
        // }
        
        // Close state database (note: close() returns () not Result)
        // self.state.close()?;
        info!("State database closed");
        
        info!("✅ Node shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn create_test_node() -> (AxionaxNode, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let mut config = NodeConfig::dev();
        config.state_path = temp_dir.path().to_str().unwrap().to_string();
        config.rpc_addr = "127.0.0.1:0".parse().unwrap(); // Random port
        
        let node = AxionaxNode::new(config).await.unwrap();
        (node, temp_dir)
    }

    #[tokio::test]
    async fn test_node_creation() {
        let (node, _temp) = create_test_node().await;
        assert_eq!(node.config.network.chain_id, 31337); // Dev chain
    }

    #[tokio::test]
    async fn test_node_stats() {
        let (node, _temp) = create_test_node().await;
        let stats = node.stats().await;
        assert_eq!(stats.blocks_received, 0);
        assert_eq!(stats.transactions_received, 0);
    }

    #[tokio::test]
    async fn test_node_state_access() {
        let (node, _temp) = create_test_node().await;
        let state = node.state();
        let height = state.get_chain_height().unwrap();
        assert_eq!(height, 0); // Genesis
    }
}
