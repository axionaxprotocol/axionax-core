//! Axionax Blockchain Core
//!
//! Block production, chain management, and transaction processing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Block represents a block in the chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub hash: [u8; 32],
    pub parent_hash: [u8; 32],
    pub timestamp: u64,
    pub proposer: String,
    pub transactions: Vec<Transaction>,
    pub state_root: [u8; 32],
    pub gas_used: u64,
    pub gas_limit: u64,
}

/// Transaction represents a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: [u8; 32],
    pub from: String,
    pub to: String,
    pub value: u128,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub nonce: u64,
    pub data: Vec<u8>,
}

/// Blockchain manages the chain state
pub struct Blockchain {
    blocks: Arc<RwLock<HashMap<u64, Block>>>,
    latest_block: Arc<RwLock<u64>>,
    config: BlockchainConfig,
}

/// Blockchain configuration
#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    pub block_time_secs: u64,
    pub max_block_size: usize,
    pub gas_limit: u64,
}

impl Blockchain {
    /// Creates a new blockchain
    pub fn new(config: BlockchainConfig) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(HashMap::new())),
            latest_block: Arc::new(RwLock::new(0)),
            config,
        }
    }

    /// Adds a new block to the chain
    pub async fn add_block(&self, block: Block) -> Result<(), String> {
        let mut blocks = self.blocks.write().await;
        let mut latest = self.latest_block.write().await;

        if block.number != *latest + 1 {
            return Err("Invalid block number".to_string());
        }

        blocks.insert(block.number, block);
        *latest += 1;
        Ok(())
    }

    /// Gets a block by number
    pub async fn get_block(&self, number: u64) -> Option<Block> {
        let blocks = self.blocks.read().await;
        blocks.get(&number).cloned()
    }

    /// Gets the latest block number
    pub async fn get_latest_block_number(&self) -> u64 {
        *self.latest_block.read().await
    }

    /// Initialize blockchain with genesis block
    pub async fn init_with_genesis(&self) {
        let mut blocks = self.blocks.write().await;
        if blocks.is_empty() {
            let genesis = Self::create_genesis();
            blocks.insert(0, genesis);
        }
    }

    /// Creates genesis block
    pub fn create_genesis() -> Block {
        Block {
            number: 0,
            hash: [0u8; 32],
            parent_hash: [0u8; 32],
            timestamp: 0,
            proposer: "genesis".to_string(),
            transactions: vec![],
            state_root: [0u8; 32],
            gas_used: 0,
            gas_limit: 30_000_000,
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            block_time_secs: 5,
            max_block_size: 1_000_000,
            gas_limit: 30_000_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_block() {
        let blockchain = Blockchain::new(BlockchainConfig::default());
        
        let genesis = Blockchain::create_genesis();
        let result = blockchain.add_block(genesis).await;
        
        // Genesis block number is 0, so after adding it, latest should be 0
        // But we expect block number 0 to be the first block
        assert!(result.is_err()); // Should fail because genesis.number = 0, but latest = 0
        
        // Let's add a proper block 1
        let block1 = Block {
            number: 1,
            hash: [1u8; 32],
            parent_hash: [0u8; 32],
            timestamp: 100,
            proposer: "validator1".to_string(),
            transactions: vec![],
            state_root: [1u8; 32],
            gas_used: 0,
            gas_limit: 30_000_000,
        };
        
        let result = blockchain.add_block(block1).await;
        assert!(result.is_ok());
        assert_eq!(blockchain.get_latest_block_number().await, 1);
    }

    #[test]
    fn test_create_genesis() {
        let genesis = Blockchain::create_genesis();
        assert_eq!(genesis.number, 0);
        assert_eq!(genesis.transactions.len(), 0);
    }
}
