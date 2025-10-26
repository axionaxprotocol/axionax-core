//! Axionax State Module
//!
//! Persistent storage layer using RocksDB for:
//! - Blocks and transactions
//! - Chain state and metadata
//! - Account balances and nonces
//! - Smart contract state

use rocksdb::{DB, Options};
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, error};

use blockchain::{Block, Transaction};

/// State database errors
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    #[error("Invalid block number: {0}")]
    InvalidBlockNumber(u64),
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
}

pub type Result<T> = std::result::Result<T, StateError>;

/// Column family names for RocksDB
mod cf {
    pub const BLOCKS: &str = "blocks";
    pub const BLOCK_HASH_TO_NUMBER: &str = "block_hash_to_number";
    pub const TRANSACTIONS: &str = "transactions";
    pub const TX_TO_BLOCK: &str = "tx_to_block";
    pub const CHAIN_STATE: &str = "chain_state";
    pub const ACCOUNTS: &str = "accounts";
}

/// State database wrapper for RocksDB
pub struct StateDB {
    db: Arc<DB>,
}

impl StateDB {
    /// Open or create a new state database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Opening state database at {:?}", path.as_ref());
        
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        
        // Define column families
        let cfs = vec![
            cf::BLOCKS,
            cf::BLOCK_HASH_TO_NUMBER,
            cf::TRANSACTIONS,
            cf::TX_TO_BLOCK,
            cf::CHAIN_STATE,
            cf::ACCOUNTS,
        ];
        
        let db = DB::open_cf(&opts, path, &cfs)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    /// Store a block in the database
    pub fn store_block(&self, block: &Block) -> Result<()> {
        debug!("Storing block #{} with hash {:?}", block.number, block.hash);
        
        // Serialize block
        let block_data = serde_json::to_vec(block)
            .map_err(|e| StateError::SerializationError(e.to_string()))?;
        
        // Get column family handles
        let cf_blocks = self.db.cf_handle(cf::BLOCKS)
            .ok_or_else(|| StateError::DatabaseError("Column family BLOCKS not found".to_string()))?;
        let cf_hash_map = self.db.cf_handle(cf::BLOCK_HASH_TO_NUMBER)
            .ok_or_else(|| StateError::DatabaseError("Column family BLOCK_HASH_TO_NUMBER not found".to_string()))?;
        
        // Store block by number (key: block_number, value: block_data)
        let number_key = format!("block_{}", block.number);
        self.db.put_cf(cf_blocks, number_key.as_bytes(), &block_data)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        // Store hash -> number mapping (hash as bytes)
        self.db.put_cf(cf_hash_map, &block.hash, block.number.to_be_bytes())
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        // Update chain height if this is the latest block
        self.update_chain_height(block.number)?;
        
        info!("Successfully stored block #{}", block.number);
        Ok(())
    }
    
    /// Get block by hash
    pub fn get_block_by_hash(&self, hash: &[u8; 32]) -> Result<Block> {
        debug!("Retrieving block with hash: {:?}", hash);
        
        // Get block number from hash
        let cf_hash_map = self.db.cf_handle(cf::BLOCK_HASH_TO_NUMBER)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let number_bytes = self.db.get_cf(cf_hash_map, hash)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StateError::BlockNotFound(format!("{:?}", hash)))?;
        
        let block_number = u64::from_be_bytes(number_bytes.try_into()
            .map_err(|_| StateError::DatabaseError("Invalid block number format".to_string()))?);
        
        // Get block by number
        self.get_block_by_number(block_number)
    }
    
    /// Get block by number
    pub fn get_block_by_number(&self, number: u64) -> Result<Block> {
        debug!("Retrieving block #{}", number);
        
        let cf_blocks = self.db.cf_handle(cf::BLOCKS)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let number_key = format!("block_{}", number);
        let block_data = self.db.get_cf(cf_blocks, number_key.as_bytes())
            .map_err(|e| StateError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StateError::BlockNotFound(number.to_string()))?;
        
        let block: Block = serde_json::from_slice(&block_data)
            .map_err(|e| StateError::SerializationError(e.to_string()))?;
        
        Ok(block)
    }
    
    /// Get the latest block
    pub fn get_latest_block(&self) -> Result<Block> {
        let height = self.get_chain_height()?;
        self.get_block_by_number(height)
    }
    
    /// Store a transaction
    pub fn store_transaction(&self, tx: &Transaction, block_hash: &[u8; 32]) -> Result<()> {
        debug!("Storing transaction {:?}", tx.hash);
        
        // Serialize transaction
        let tx_data = serde_json::to_vec(tx)
            .map_err(|e| StateError::SerializationError(e.to_string()))?;
        
        let cf_txs = self.db.cf_handle(cf::TRANSACTIONS)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        let cf_tx_map = self.db.cf_handle(cf::TX_TO_BLOCK)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        // Store transaction
        self.db.put_cf(cf_txs, &tx.hash, &tx_data)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        // Store tx -> block mapping
        self.db.put_cf(cf_tx_map, &tx.hash, block_hash)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get transaction by hash
    pub fn get_transaction(&self, tx_hash: &[u8; 32]) -> Result<Transaction> {
        debug!("Retrieving transaction {:?}", tx_hash);
        
        let cf_txs = self.db.cf_handle(cf::TRANSACTIONS)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let tx_data = self.db.get_cf(cf_txs, tx_hash)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StateError::TransactionNotFound(format!("{:?}", tx_hash)))?;
        
        let tx: Transaction = serde_json::from_slice(&tx_data)
            .map_err(|e| StateError::SerializationError(e.to_string()))?;
        
        Ok(tx)
    }
    
    /// Get block hash containing a transaction
    pub fn get_transaction_block(&self, tx_hash: &[u8; 32]) -> Result<[u8; 32]> {
        let cf_tx_map = self.db.cf_handle(cf::TX_TO_BLOCK)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let block_hash = self.db.get_cf(cf_tx_map, tx_hash)
            .map_err(|e| StateError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StateError::TransactionNotFound(format!("{:?}", tx_hash)))?;
        
        block_hash.try_into()
            .map_err(|_| StateError::DatabaseError("Invalid block hash format".to_string()))
    }
    
    /// Get current chain height
    pub fn get_chain_height(&self) -> Result<u64> {
        let cf_state = self.db.cf_handle(cf::CHAIN_STATE)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        match self.db.get_cf(cf_state, b"chain_height") {
            Ok(Some(bytes)) => {
                let height = u64::from_be_bytes(bytes.try_into()
                    .map_err(|_| StateError::DatabaseError("Invalid height format".to_string()))?);
                Ok(height)
            },
            Ok(None) => Ok(0), // Genesis state
            Err(e) => Err(StateError::DatabaseError(e.to_string())),
        }
    }
    
    /// Update chain height (internal)
    fn update_chain_height(&self, new_height: u64) -> Result<()> {
        let current_height = self.get_chain_height()?;
        
        if new_height > current_height {
            let cf_state = self.db.cf_handle(cf::CHAIN_STATE)
                .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
            
            self.db.put_cf(cf_state, b"chain_height", new_height.to_be_bytes())
                .map_err(|e| StateError::DatabaseError(e.to_string()))?;
            
            debug!("Updated chain height to {}", new_height);
        }
        
        Ok(())
    }
    
    /// Store state root hash
    pub fn store_state_root(&self, block_number: u64, state_root: &str) -> Result<()> {
        let cf_state = self.db.cf_handle(cf::CHAIN_STATE)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let key = format!("state_root_{}", block_number);
        self.db.put_cf(cf_state, key.as_bytes(), state_root.as_bytes())
            .map_err(|e| StateError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get state root hash
    pub fn get_state_root(&self, block_number: u64) -> Result<String> {
        let cf_state = self.db.cf_handle(cf::CHAIN_STATE)
            .ok_or_else(|| StateError::DatabaseError("Column family not found".to_string()))?;
        
        let key = format!("state_root_{}", block_number);
        let root_bytes = self.db.get_cf(cf_state, key.as_bytes())
            .map_err(|e| StateError::DatabaseError(e.to_string()))?
            .ok_or_else(|| StateError::KeyNotFound(key))?;
        
        String::from_utf8(root_bytes)
            .map_err(|e| StateError::DatabaseError(e.to_string()))
    }
    
    /// Get all blocks in range
    pub fn get_blocks_range(&self, start: u64, end: u64) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();
        
        for number in start..=end {
            match self.get_block_by_number(number) {
                Ok(block) => blocks.push(block),
                Err(StateError::BlockNotFound(_)) => break, // Stop at first missing block
                Err(e) => return Err(e),
            }
        }
        
        Ok(blocks)
    }
    
    /// Close the database
    pub fn close(self) {
        info!("Closing state database");
        drop(self.db);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_block(number: u64) -> Block {
        let hash_val = number as u8;
        let mut hash = [0u8; 32];
        hash[31] = hash_val;
        
        let mut parent_hash = [0u8; 32];
        if number > 0 {
            parent_hash[31] = (number - 1) as u8;
        }
        
        let mut state_root = [0u8; 32];
        state_root[31] = hash_val;
        
        Block {
            number,
            hash,
            parent_hash,
            timestamp: 1234567890 + number,
            proposer: "0xvalidator".to_string(),
            transactions: vec![],
            state_root,
            gas_used: 0,
            gas_limit: 10_000_000,
        }
    }

    fn create_test_tx(id: u8) -> Transaction {
        let mut hash = [0u8; 32];
        hash[31] = id;
        
        Transaction {
            hash,
            from: "0xfrom".to_string(),
            to: "0xto".to_string(),
            value: 1000,
            gas_price: 20,
            gas_limit: 21000,
            nonce: 1,
            data: vec![],
        }
    }

    #[test]
    fn test_state_db_open() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        // Check initial chain height
        assert_eq!(db.get_chain_height().unwrap(), 0);
    }

    #[test]
    fn test_store_and_get_block() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        let block = create_test_block(1);
        let block_hash = block.hash;
        
        // Store block
        db.store_block(&block).unwrap();
        
        // Get block by number
        let retrieved = db.get_block_by_number(1).unwrap();
        assert_eq!(retrieved.hash, block_hash);
        assert_eq!(retrieved.number, block.number);
        
        // Get block by hash
        let retrieved = db.get_block_by_hash(&block_hash).unwrap();
        assert_eq!(retrieved.number, block.number);
        
        // Check chain height updated
        assert_eq!(db.get_chain_height().unwrap(), 1);
    }

    #[test]
    fn test_store_multiple_blocks() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        // Store blocks 1-5
        for i in 1..=5 {
            let block = create_test_block(i);
            db.store_block(&block).unwrap();
        }
        
        // Verify chain height
        assert_eq!(db.get_chain_height().unwrap(), 5);
        
        // Get latest block
        let latest = db.get_latest_block().unwrap();
        assert_eq!(latest.number, 5);
        
        // Get block range
        let blocks = db.get_blocks_range(2, 4).unwrap();
        assert_eq!(blocks.len(), 3);
        assert_eq!(blocks[0].number, 2);
        assert_eq!(blocks[2].number, 4);
    }

    #[test]
    fn test_store_and_get_transaction() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        let tx = create_test_tx(1);
        let tx_hash = tx.hash;
        
        let mut block_hash = [0u8; 32];
        block_hash[31] = 10;
        
        // Store transaction
        db.store_transaction(&tx, &block_hash).unwrap();
        
        // Get transaction
        let retrieved = db.get_transaction(&tx_hash).unwrap();
        assert_eq!(retrieved.hash, tx_hash);
        assert_eq!(retrieved.from, tx.from);
        
        // Get block containing transaction
        let retrieved_block_hash = db.get_transaction_block(&tx_hash).unwrap();
        assert_eq!(retrieved_block_hash, block_hash);
    }

    #[test]
    fn test_state_root() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        let block_number = 10;
        let state_root = "0xabcdef1234567890";
        
        // Store state root
        db.store_state_root(block_number, state_root).unwrap();
        
        // Get state root
        let retrieved = db.get_state_root(block_number).unwrap();
        assert_eq!(retrieved, state_root);
    }

    #[test]
    fn test_block_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        let result = db.get_block_by_number(999);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StateError::BlockNotFound(_)));
    }

    #[test]
    fn test_transaction_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let db = StateDB::open(temp_dir.path()).unwrap();
        
        let mut nonexistent_hash = [0u8; 32];
        nonexistent_hash[0] = 0xff;
        
        let result = db.get_transaction(&nonexistent_hash);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StateError::TransactionNotFound(_)));
    }
}

