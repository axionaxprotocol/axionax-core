//! Network protocol definitions and message types

use serde::{Deserialize, Serialize};

/// Network message types for Axionax protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// New block propagation
    Block(BlockMessage),
    /// Transaction propagation
    Transaction(TransactionMessage),
    /// Consensus messages (challenges, proofs)
    Consensus(ConsensusMessage),
    /// Peer discovery and status
    Status(StatusMessage),
    /// Request for specific data
    Request(RequestMessage),
    /// Response to data request
    Response(ResponseMessage),
}

/// Block propagation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMessage {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub proposer: String,
    pub transactions: Vec<String>,
    pub state_root: String,
}

/// Transaction propagation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: u64,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

/// Consensus-related messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    /// PoPC challenge announcement
    Challenge {
        job_id: String,
        worker_id: String,
        sample_indices: Vec<u64>,
        seed: Vec<u8>,
        deadline: u64,
    },
    /// Worker's proof submission
    Proof {
        job_id: String,
        worker_id: String,
        outputs: Vec<Vec<u8>>,
        proof_data: Vec<u8>,
    },
    /// Validator's vote
    Vote {
        job_id: String,
        worker_id: String,
        validator_id: String,
        vote: bool, // true = PASS, false = FAIL
        signature: Vec<u8>,
    },
}

/// Peer status message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    pub chain_id: u64,
    pub genesis_hash: String,
    pub best_block: u64,
    pub best_hash: String,
    pub peer_count: usize,
    pub protocol_version: String,
}

/// Data request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestMessage {
    /// Request blocks by number range
    Blocks { from: u64, to: u64 },
    /// Request specific block by hash
    BlockByHash { hash: String },
    /// Request transaction by hash
    Transaction { hash: String },
    /// Request peer list
    Peers,
    /// Request chain status
    Status,
}

/// Response to data request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseMessage {
    /// Block data response
    Blocks { blocks: Vec<BlockMessage> },
    /// Transaction data response
    Transaction { tx: TransactionMessage },
    /// Peer list response
    Peers { peers: Vec<PeerInfo> },
    /// Status response
    Status { status: StatusMessage },
    /// Error response
    Error { message: String },
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub protocols: Vec<String>,
    pub agent_version: String,
}

/// Message type identifier for topic subscription
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageType {
    Blocks,
    Transactions,
    Consensus,
    Status,
}

impl MessageType {
    /// Get Gossipsub topic name
    pub fn topic_name(&self) -> String {
        match self {
            MessageType::Blocks => "/axionax/blocks/1.0.0".to_string(),
            MessageType::Transactions => "/axionax/txs/1.0.0".to_string(),
            MessageType::Consensus => "/axionax/consensus/1.0.0".to_string(),
            MessageType::Status => "/axionax/status/1.0.0".to_string(),
        }
    }
}

impl NetworkMessage {
    /// Serialize message to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize message from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }

    /// Get message type for routing
    pub fn message_type(&self) -> MessageType {
        match self {
            NetworkMessage::Block(_) => MessageType::Blocks,
            NetworkMessage::Transaction(_) => MessageType::Transactions,
            NetworkMessage::Consensus(_) => MessageType::Consensus,
            NetworkMessage::Status(_) => MessageType::Status,
            NetworkMessage::Request(_) => MessageType::Status,
            NetworkMessage::Response(_) => MessageType::Status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = NetworkMessage::Status(StatusMessage {
            chain_id: 86137,
            genesis_hash: "0x123".to_string(),
            best_block: 100,
            best_hash: "0xabc".to_string(),
            peer_count: 5,
            protocol_version: "1.0.0".to_string(),
        });

        let bytes = msg.to_bytes().unwrap();
        let decoded = NetworkMessage::from_bytes(&bytes).unwrap();

        match decoded {
            NetworkMessage::Status(status) => {
                assert_eq!(status.chain_id, 86137);
                assert_eq!(status.best_block, 100);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_topic_names() {
        assert_eq!(MessageType::Blocks.topic_name(), "/axionax/blocks/1.0.0");
        assert_eq!(MessageType::Transactions.topic_name(), "/axionax/txs/1.0.0");
        assert_eq!(MessageType::Consensus.topic_name(), "/axionax/consensus/1.0.0");
    }
}
