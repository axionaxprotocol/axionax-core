//! Network error types

use thiserror::Error;

/// Network layer errors
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Network initialization failed: {0}")]
    InitializationError(String),

    #[error("Failed to send message: {0}")]
    SendError(String),

    #[error("Failed to receive message: {0}")]
    ReceiveError(String),

    #[error("Peer connection failed: {0}")]
    ConnectionError(String),

    #[error("Invalid peer ID: {0}")]
    InvalidPeerId(String),

    #[error("Message serialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("libp2p error: {0}")]
    Libp2pError(String),

    #[error("Network timeout")]
    Timeout,

    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    #[error("Topic subscription failed: {0}")]
    SubscriptionError(String),

    #[error("Invalid message format")]
    InvalidMessage,

    #[error("Network shutdown")]
    Shutdown,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type for network operations
pub type Result<T> = std::result::Result<T, NetworkError>;
