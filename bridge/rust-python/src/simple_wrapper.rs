// Simplified wrappers to avoid deep Rust API exposure to Python

use pyo3::prelude::*;
use consensus::ConsensusConfig;
use blockchain::BlockchainConfig;

/// Get default consensus config
pub fn default_consensus_config() -> ConsensusConfig {
    ConsensusConfig {
        sample_size: 100,
        min_confidence: 0.95,
        fraud_window_blocks: 1000,
        min_validator_stake: 100_000,
        false_pass_penalty_bps: 1000, // 10%
    }
}

/// Get default blockchain config
pub fn default_blockchain_config() -> BlockchainConfig {
    BlockchainConfig {
        block_time_secs: 12,
        max_block_size: 1_000_000,
        gas_limit: 30_000_000,
    }
}
