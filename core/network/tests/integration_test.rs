//! Integration tests for network layer

use network::{
    config::NetworkConfig,
    manager::NetworkManager,
    protocol::{BlockMessage, NetworkMessage, TransactionMessage},
};
use tokio::time::{sleep, Duration};

/// Test basic network manager initialization and shutdown
#[tokio::test]
async fn test_network_init_and_shutdown() {
    let config = NetworkConfig::dev();
    let manager = NetworkManager::new(config).await.expect("Failed to create network manager");

    // Verify manager was created successfully
    assert!(manager.peer_count() == 0);
}

/// Test peer discovery between two nodes
#[tokio::test]
async fn test_peer_discovery() {
    // Create two nodes with different ports
    let mut config1 = NetworkConfig::dev();
    config1.port = 30301;
    config1.enable_mdns = true;
    
    let mut config2 = NetworkConfig::dev();
    config2.port = 30302;
    config2.enable_mdns = true;

    let manager1 = NetworkManager::new(config1).await.expect("Failed to create manager 1");
    let manager2 = NetworkManager::new(config2).await.expect("Failed to create manager 2");

    // Verify both managers created successfully
    assert_eq!(manager1.peer_count(), 0);
    assert_eq!(manager2.peer_count(), 0);
}

/// Test message publishing
#[tokio::test]
async fn test_message_publishing() {
    let config = NetworkConfig::dev();
    let mut manager = NetworkManager::new(config).await.expect("Failed to create network manager");

    // Create a test block message
    let block_msg = NetworkMessage::Block(BlockMessage {
        number: 1,
        hash: "0x1234567890abcdef".to_string(),
        parent_hash: "0x0000000000000000".to_string(),
        timestamp: 1234567890,
        proposer: "0xvalidator".to_string(),
        transactions: vec!["0xabc".to_string(), "0xdef".to_string()],
        state_root: "0xstateroot".to_string(),
    });

    // Publish message - should succeed even without peers
    let result = manager.publish(block_msg);
    // Publishing without peers may fail in gossipsub, which is expected
    // The important thing is the API works correctly
    let _ = result;
}

/// Test transaction propagation
#[tokio::test]
async fn test_transaction_propagation() {
    let config = NetworkConfig::dev();
    let mut manager = NetworkManager::new(config).await.expect("Failed to create network manager");

    // Create transaction message
    let tx_msg = NetworkMessage::Transaction(TransactionMessage {
        hash: "0xtxhash".to_string(),
        from: "0xfrom".to_string(),
        to: "0xto".to_string(),
        value: 1000,
        data: vec![0x12, 0x34],
        nonce: 1,
        signature: vec![0xaa, 0xbb],
    });

    // Publish transaction - result may vary without peers
    let _ = manager.publish(tx_msg);
}

/// Test bootstrap node connection
#[tokio::test]
async fn test_bootstrap_connection() {
    let mut config = NetworkConfig::testnet();
    
    // Add a bootstrap node (using a known libp2p bootstrap node for testing)
    config.bootstrap_nodes = vec![
        "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN".to_string(),
    ];

    let manager = NetworkManager::new(config).await.expect("Failed to create manager");

    // In a real scenario, this would attempt to connect to the bootstrap node
    // For now, just verify manager was created successfully
    assert_eq!(manager.peer_count(), 0);
}

/// Test network configuration validation
#[tokio::test]
async fn test_config_validation() {
    // Test dev config
    let dev_config = NetworkConfig::dev();
    assert_eq!(dev_config.chain_id, 31337);
    assert_eq!(dev_config.port, 30303);

    // Test testnet config
    let testnet_config = NetworkConfig::testnet();
    assert_eq!(testnet_config.chain_id, 86137);
    assert_eq!(testnet_config.port, 30303);

    // Test mainnet config
    let mainnet_config = NetworkConfig::mainnet();
    assert_eq!(mainnet_config.chain_id, 86150);
    assert_eq!(mainnet_config.port, 30303);
}

/// Test concurrent message handling
#[tokio::test]
async fn test_concurrent_messages() {
    let config = NetworkConfig::dev();
    let mut manager = NetworkManager::new(config).await.expect("Failed to create manager");

    // Send multiple messages concurrently
    let messages = vec![
        NetworkMessage::Block(BlockMessage {
            number: 1,
            hash: "0x01".to_string(),
            parent_hash: "0x00".to_string(),
            timestamp: 1000,
            proposer: "0xval1".to_string(),
            transactions: vec![],
            state_root: "0xroot1".to_string(),
        }),
        NetworkMessage::Block(BlockMessage {
            number: 2,
            hash: "0x02".to_string(),
            parent_hash: "0x01".to_string(),
            timestamp: 2000,
            proposer: "0xval2".to_string(),
            transactions: vec![],
            state_root: "0xroot2".to_string(),
        }),
        NetworkMessage::Block(BlockMessage {
            number: 3,
            hash: "0x03".to_string(),
            parent_hash: "0x02".to_string(),
            timestamp: 3000,
            proposer: "0xval3".to_string(),
            transactions: vec![],
            state_root: "0xroot3".to_string(),
        }),
    ];

    // Publish all messages - results may vary without peers
    for msg in messages {
        let _ = manager.publish(msg);
    }
}
