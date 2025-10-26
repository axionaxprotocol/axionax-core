//! State + RPC Integration Example
//!
//! Demonstrates storing blocks in StateDB and querying via JSON-RPC API
//!
//! Run with: cargo run --example state_rpc_integration

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use blockchain::{Block, Transaction};
use rpc::start_rpc_server;
use state::StateDB;
use tempfile::TempDir;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("\n=== Axionax State + RPC Integration Example ===\n");
    
    // 1. Create temporary StateDB
    let temp_dir = TempDir::new()?;
    let state = Arc::new(StateDB::open(temp_dir.path())?);
    println!("✓ Opened StateDB at: {}", temp_dir.path().display());
    
    // 2. Create and store sample blocks
    println!("\n--- Creating sample blocks ---");
    
    let genesis_block = Block {
        number: 0,
        hash: [0u8; 32],
        parent_hash: [0u8; 32],
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        proposer: "genesis".to_string(),
        transactions: vec![],
        state_root: [0u8; 32],
        gas_used: 0,
        gas_limit: 10_000_000,
    };
    
    state.store_block(&genesis_block)?;
    println!("✓ Stored genesis block #0");
    
    // Create block 1 with a transaction
    let tx_hash = [1u8; 32];
    let tx = Transaction {
        hash: tx_hash,
        from: "0xAlice".to_string(),
        to: "0xBob".to_string(),
        value: 1000,
        gas_price: 20,
        gas_limit: 21000,
        nonce: 0,
        data: vec![],
    };
    
    let block1_hash = [1u8; 32];
    let block1 = Block {
        number: 1,
        hash: block1_hash,
        parent_hash: genesis_block.hash,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        proposer: "validator1".to_string(),
        transactions: vec![tx.clone()],
        state_root: [1u8; 32],
        gas_used: 21000,
        gas_limit: 10_000_000,
    };
    
    state.store_block(&block1)?;
    state.store_transaction(&tx, &block1.hash)?;
    println!("✓ Stored block #1 with 1 transaction");
    
    // Create block 2
    let block2_hash = [2u8; 32];
    let block2 = Block {
        number: 2,
        hash: block2_hash,
        parent_hash: block1.hash,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        proposer: "validator2".to_string(),
        transactions: vec![],
        state_root: [2u8; 32],
        gas_used: 0,
        gas_limit: 10_000_000,
    };
    
    state.store_block(&block2)?;
    println!("✓ Stored block #2");
    
    // 3. Query state directly
    println!("\n--- Direct StateDB queries ---");
    let height = state.get_chain_height()?;
    println!("Chain height: {}", height);
    
    let latest = state.get_latest_block()?;
    println!("Latest block: #{} by {}", latest.number, latest.proposer);
    
    let retrieved_tx = state.get_transaction(&tx_hash)?;
    println!("Retrieved tx: {} -> {} (value: {})", retrieved_tx.from, retrieved_tx.to, retrieved_tx.value);
    
    // 4. Start RPC server
    println!("\n--- Starting RPC server ---");
    let addr: SocketAddr = "127.0.0.1:8545".parse()?;
    let _handle = start_rpc_server(addr, state.clone(), 86137).await?;
    println!("✓ RPC server listening on http://{}", addr);
    
    // 5. Test RPC endpoints with curl examples
    println!("\n--- RPC API Examples ---");
    println!("\nYou can test the API with these curl commands:\n");
    
    println!("1. Get current block number:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}}'");
    
    println!("\n2. Get latest block:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\":[\"latest\",false],\"id\":2}}'");
    
    println!("\n3. Get block by number:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\":[\"0x1\",false],\"id\":3}}'");
    
    println!("\n4. Get transaction by hash:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByHash\",\"params\":[\"0x0101010101010101010101010101010101010101010101010101010101010101\"],\"id\":4}}'");
    
    println!("\n5. Get chain ID:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_chainId\",\"params\":[],\"id\":5}}'");
    
    println!("\n--- Server running. Press Ctrl+C to stop ---\n");
    
    // Keep server running
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
