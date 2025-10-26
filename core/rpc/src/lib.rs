//! Axionax RPC Server
//!
//! JSON-RPC 2.0 API server for blockchain queries and transaction submission

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    server::{Server, ServerHandle},
    types::ErrorObjectOwned,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info};

use blockchain::{Block, Transaction};
use state::StateDB;

/// RPC server errors
#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
    
    #[error("State error: {0}")]
    StateError(#[from] state::StateError),
}

impl From<RpcError> for ErrorObjectOwned {
    fn from(error: RpcError) -> Self {
        match error {
            RpcError::BlockNotFound(msg) => ErrorObjectOwned::owned(-32001, msg, None::<()>),
            RpcError::TransactionNotFound(msg) => ErrorObjectOwned::owned(-32002, msg, None::<()>),
            RpcError::InvalidParams(msg) => ErrorObjectOwned::owned(-32602, msg, None::<()>),
            RpcError::InternalError(msg) => ErrorObjectOwned::owned(-32603, msg, None::<()>),
            RpcError::StateError(e) => ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>),
        }
    }
}

/// Block response format (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    pub number: String,        // hex-encoded
    pub hash: String,           // hex-encoded
    pub parent_hash: String,    // hex-encoded
    pub timestamp: String,      // hex-encoded
    pub proposer: String,
    pub transactions: Vec<String>, // tx hashes (hex-encoded)
    pub state_root: String,     // hex-encoded
    pub gas_used: String,       // hex-encoded
    pub gas_limit: String,      // hex-encoded
}

impl From<Block> for BlockResponse {
    fn from(block: Block) -> Self {
        BlockResponse {
            number: format!("0x{:x}", block.number),
            hash: format!("0x{}", hex::encode(block.hash)),
            parent_hash: format!("0x{}", hex::encode(block.parent_hash)),
            timestamp: format!("0x{:x}", block.timestamp),
            proposer: block.proposer,
            transactions: block.transactions.iter()
                .map(|tx| format!("0x{}", hex::encode(tx.hash)))
                .collect(),
            state_root: format!("0x{}", hex::encode(block.state_root)),
            gas_used: format!("0x{:x}", block.gas_used),
            gas_limit: format!("0x{:x}", block.gas_limit),
        }
    }
}

/// Transaction response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,           // hex-encoded
    pub from: String,
    pub to: String,
    pub value: String,          // hex-encoded
    pub gas_price: String,      // hex-encoded
    pub gas_limit: String,      // hex-encoded
    pub nonce: String,          // hex-encoded
    pub data: String,           // hex-encoded
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        TransactionResponse {
            hash: format!("0x{}", hex::encode(tx.hash)),
            from: tx.from,
            to: tx.to,
            value: format!("0x{:x}", tx.value),
            gas_price: format!("0x{:x}", tx.gas_price),
            gas_limit: format!("0x{:x}", tx.gas_limit),
            nonce: format!("0x{:x}", tx.nonce),
            data: format!("0x{}", hex::encode(tx.data)),
        }
    }
}

/// Ethereum-compatible JSON-RPC API
#[rpc(server)]
pub trait AxionaxRpc {
    /// Get current block number (chain height)
    #[method(name = "eth_blockNumber")]
    async fn block_number(&self) -> RpcResult<String>;
    
    /// Get block by number
    #[method(name = "eth_getBlockByNumber")]
    async fn get_block_by_number(
        &self,
        block_number: String,
        full_transactions: bool,
    ) -> RpcResult<Option<BlockResponse>>;
    
    /// Get block by hash
    #[method(name = "eth_getBlockByHash")]
    async fn get_block_by_hash(
        &self,
        block_hash: String,
        full_transactions: bool,
    ) -> RpcResult<Option<BlockResponse>>;
    
    /// Get transaction by hash
    #[method(name = "eth_getTransactionByHash")]
    async fn get_transaction_by_hash(
        &self,
        tx_hash: String,
    ) -> RpcResult<Option<TransactionResponse>>;
    
    /// Get chain ID
    #[method(name = "eth_chainId")]
    async fn chain_id(&self) -> RpcResult<String>;
    
    /// Net version (chain ID as string)
    #[method(name = "net_version")]
    async fn net_version(&self) -> RpcResult<String>;
}

/// RPC server implementation
pub struct AxionaxRpcServerImpl {
    state: Arc<StateDB>,
    chain_id: u64,
}

impl AxionaxRpcServerImpl {
    /// Create new RPC server
    pub fn new(state: Arc<StateDB>, chain_id: u64) -> Self {
        Self { state, chain_id }
    }
}

#[async_trait]
impl AxionaxRpcServer for AxionaxRpcServerImpl {
    async fn block_number(&self) -> RpcResult<String> {
        let height = self.state.get_chain_height()
            .map_err(RpcError::from)?;
        
        Ok(format!("0x{:x}", height))
    }
    
    async fn get_block_by_number(
        &self,
        block_number: String,
        _full_transactions: bool,
    ) -> RpcResult<Option<BlockResponse>> {
        // Parse block number (hex or "latest")
        let number = if block_number == "latest" {
            self.state.get_chain_height()
                .map_err(RpcError::from)?
        } else {
            parse_hex_u64(&block_number)
                .map_err(|e| RpcError::InvalidParams(e))?
        };
        
        match self.state.get_block_by_number(number) {
            Ok(block) => Ok(Some(block.into())),
            Err(state::StateError::BlockNotFound(_)) => Ok(None),
            Err(e) => Err(RpcError::from(e).into()),
        }
    }
    
    async fn get_block_by_hash(
        &self,
        block_hash: String,
        _full_transactions: bool,
    ) -> RpcResult<Option<BlockResponse>> {
        let hash = parse_hex_hash(&block_hash)
            .map_err(|e| RpcError::InvalidParams(e))?;
        
        match self.state.get_block_by_hash(&hash) {
            Ok(block) => Ok(Some(block.into())),
            Err(state::StateError::BlockNotFound(_)) => Ok(None),
            Err(e) => Err(RpcError::from(e).into()),
        }
    }
    
    async fn get_transaction_by_hash(
        &self,
        tx_hash: String,
    ) -> RpcResult<Option<TransactionResponse>> {
        let hash = parse_hex_hash(&tx_hash)
            .map_err(|e| RpcError::InvalidParams(e))?;
        
        match self.state.get_transaction(&hash) {
            Ok(tx) => Ok(Some(tx.into())),
            Err(state::StateError::TransactionNotFound(_)) => Ok(None),
            Err(e) => Err(RpcError::from(e).into()),
        }
    }
    
    async fn chain_id(&self) -> RpcResult<String> {
        Ok(format!("0x{:x}", self.chain_id))
    }
    
    async fn net_version(&self) -> RpcResult<String> {
        Ok(self.chain_id.to_string())
    }
}

/// Start RPC server
pub async fn start_rpc_server(
    addr: SocketAddr,
    state: Arc<StateDB>,
    chain_id: u64,
) -> anyhow::Result<ServerHandle> {
    info!("Starting RPC server on {}", addr);
    
    let server = Server::builder()
        .build(addr)
        .await?;
    
    let rpc_impl = AxionaxRpcServerImpl::new(state, chain_id);
    let handle = server.start(rpc_impl.into_rpc());
    
    info!("RPC server started successfully");
    Ok(handle)
}

/// Parse hex string to u64
fn parse_hex_u64(hex: &str) -> Result<u64, String> {
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    u64::from_str_radix(hex, 16)
        .map_err(|e| format!("Invalid hex number: {}", e))
}

/// Parse hex string to 32-byte hash
fn parse_hex_hash(hex: &str) -> Result<[u8; 32], String> {
    let hex = hex.strip_prefix("0x").unwrap_or(hex);
    
    if hex.len() != 64 {
        return Err(format!("Invalid hash length: expected 64 hex chars, got {}", hex.len()));
    }
    
    let bytes = hex::decode(hex)
        .map_err(|e| format!("Invalid hex string: {}", e))?;
    
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&bytes);
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_state() -> Arc<StateDB> {
        let temp_dir = TempDir::new().unwrap();
        Arc::new(StateDB::open(temp_dir.path()).unwrap())
    }

    #[tokio::test]
    async fn test_rpc_block_number() {
        let state = create_test_state();
        let rpc = AxionaxRpcServerImpl::new(state, 86137);
        
        let result = rpc.block_number().await.unwrap();
        assert_eq!(result, "0x0"); // Genesis state
    }

    #[tokio::test]
    async fn test_rpc_chain_id() {
        let state = create_test_state();
        let rpc = AxionaxRpcServerImpl::new(state, 86137);
        
        let result = rpc.chain_id().await.unwrap();
        assert_eq!(result, "0x15079"); // 86137 in hex
    }

    #[tokio::test]
    async fn test_rpc_net_version() {
        let state = create_test_state();
        let rpc = AxionaxRpcServerImpl::new(state, 86137);
        
        let result = rpc.net_version().await.unwrap();
        assert_eq!(result, "86137");
    }

    #[test]
    fn test_parse_hex_u64() {
        assert_eq!(parse_hex_u64("0x10").unwrap(), 16);
        assert_eq!(parse_hex_u64("10").unwrap(), 16);
        assert_eq!(parse_hex_u64("0xff").unwrap(), 255);
        assert!(parse_hex_u64("invalid").is_err());
    }

    #[test]
    fn test_parse_hex_hash() {
        let hash_str = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = parse_hex_hash(hash_str).unwrap();
        assert_eq!(result.len(), 32);
        assert_eq!(result[0], 0x12);
        assert_eq!(result[31], 0xef);
        
        // Invalid length
        assert!(parse_hex_hash("0x1234").is_err());
        
        // Invalid hex
        assert!(parse_hex_hash("0xZZZZ567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").is_err());
    }

    #[tokio::test]
    async fn test_rpc_get_block_not_found() {
        let state = create_test_state();
        let rpc = AxionaxRpcServerImpl::new(state, 86137);
        
        let result = rpc.get_block_by_number("0x999".to_string(), false).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_rpc_get_transaction_not_found() {
        let state = create_test_state();
        let rpc = AxionaxRpcServerImpl::new(state, 86137);
        
        let hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = rpc.get_transaction_by_hash(hash.to_string()).await.unwrap();
        assert!(result.is_none());
    }
}

