//! Full Axionax Node Example
//!
//! Demonstrates running a complete blockchain node with:
//! - P2P networking (libp2p)
//! - Persistent storage (RocksDB)
//! - JSON-RPC API server
//!
//! Run with: cargo run --example full_node -p node

use std::time::{SystemTime, UNIX_EPOCH};
use blockchain::{Block, Transaction};
use node::{AxionaxNode, NodeConfig};
use tempfile::TempDir;
use tokio::time::{sleep, Duration};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    println!("\n╔════════════════════════════════════════════╗");
    println!("║     Axionax Full Node Integration Demo    ║");
    println!("╚════════════════════════════════════════════╝\n");
    
    // Create temporary directory for this demo
    let temp_dir = TempDir::new()?;
    
    // Configure node
    let mut config = NodeConfig::dev();
    config.state_path = temp_dir.path().join("state").to_str().unwrap().to_string();
    config.rpc_addr = "127.0.0.1:8545".parse()?;
    
    println!("📋 Node Configuration:");
    println!("   Chain ID: {}", config.network.chain_id);
    println!("   RPC Address: {}", config.rpc_addr);
    println!("   State Path: {}", config.state_path);
    println!();
    
    // Create and start node
    println!("🚀 Starting Axionax node...\n");
    let mut node = AxionaxNode::new(config).await?;
    node.start().await?;
    
    // Give RPC server time to start
    sleep(Duration::from_secs(1)).await;
    
    println!("\n✅ Node is fully operational!\n");
    println!("═══════════════════════════════════════════════\n");
    
    // Create and store genesis block
    println!("📦 Creating genesis block...");
    let genesis = Block {
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
    
    node.state().store_block(&genesis)?;
    println!("   ✓ Genesis block stored");
    
    // Publish genesis block to network
    node.publish_block(&genesis).await?;
    println!("   ✓ Genesis block published to network\n");
    
    // Create and store block 1 with a transaction
    println!("📦 Creating block #1 with transaction...");
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
        parent_hash: genesis.hash,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        proposer: "validator1".to_string(),
        transactions: vec![tx.clone()],
        state_root: [1u8; 32],
        gas_used: 21000,
        gas_limit: 10_000_000,
    };
    
    node.state().store_block(&block1)?;
    node.state().store_transaction(&tx, &block1.hash)?;
    println!("   ✓ Block #1 stored with 1 transaction");
    
    // Publish block 1
    node.publish_block(&block1).await?;
    println!("   ✓ Block #1 published to network");
    
    // Publish transaction
    node.publish_transaction(&tx).await?;
    println!("   ✓ Transaction published to network\n");
    
    // Create block 2
    println!("📦 Creating block #2...");
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
    
    node.state().store_block(&block2)?;
    println!("   ✓ Block #2 stored");
    
    node.publish_block(&block2).await?;
    println!("   ✓ Block #2 published to network\n");
    
    // Display node statistics
    println!("═══════════════════════════════════════════════\n");
    println!("📊 Node Statistics:");
    let stats = node.stats().await;
    println!("   Blocks received: {}", stats.blocks_received);
    println!("   Blocks stored: {}", stats.blocks_stored);
    println!("   Transactions received: {}", stats.transactions_received);
    println!("   Transactions stored: {}", stats.transactions_stored);
    println!("   Connected peers: {}", node.peer_count().await);
    println!();
    
    // Display chain state
    println!("⛓️  Blockchain State:");
    let height = node.state().get_chain_height()?;
    println!("   Chain height: {}", height);
    
    let latest = node.state().get_latest_block()?;
    println!("   Latest block: #{} by {}", latest.number, latest.proposer);
    println!("   Latest block hash: 0x{}", hex::encode(&latest.hash[..8]));
    println!();
    
    // Display RPC API examples
    println!("═══════════════════════════════════════════════\n");
    println!("🔌 RPC API is now available at: http://127.0.0.1:8545\n");
    println!("You can test it with these curl commands:\n");
    
    println!("1️⃣  Get current block number:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}}'");
    println!();
    
    println!("2️⃣  Get latest block:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\":[\"latest\",false],\"id\":2}}'");
    println!();
    
    println!("3️⃣  Get block #1:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\":[\"0x1\",false],\"id\":3}}'");
    println!();
    
    println!("4️⃣  Get transaction:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByHash\",\"params\":[\"0x0101010101010101010101010101010101010101010101010101010101010101\"],\"id\":4}}'");
    println!();
    
    println!("5️⃣  Get chain ID:");
    println!("   curl -X POST http://127.0.0.1:8545 \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"jsonrpc\":\"2.0\",\"method\":\"eth_chainId\",\"params\":[],\"id\":5}}'");
    println!();
    
    println!("═══════════════════════════════════════════════\n");
    println!("✨ Full Node Integration Demo Complete!\n");
    println!("Components active:");
    println!("   ✓ Network Layer (libp2p + gossipsub)");
    println!("   ✓ State Management (RocksDB)");
    println!("   ✓ RPC Server (JSON-RPC 2.0)");
    println!();
    println!("🎯 Key Features Demonstrated:");
    println!("   • Block creation and storage");
    println!("   • Transaction handling");
    println!("   • P2P message publishing");
    println!("   • Persistent blockchain state");
    println!("   • Ethereum-compatible RPC API");
    println!();
    println!("Press Ctrl+C to stop the node...\n");
    
    // Keep the server running
    loop {
        sleep(Duration::from_secs(60)).await;
        
        // Periodically display stats
        let stats = node.stats().await;
        info!(
            "Stats: {} blocks stored, {} peers connected",
            stats.blocks_stored,
            node.peer_count().await
        );
    }
}
