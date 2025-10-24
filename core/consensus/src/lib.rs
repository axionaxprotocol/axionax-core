//! Axionax Consensus Engine (PoPC)
//! 
//! Implements Proof-of-Probabilistic-Checking consensus mechanism

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// PoPC Validator represents a network validator
#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub stake: u128,
    pub total_votes: u64,
    pub correct_votes: u64,
    pub false_pass: u64,
    pub is_active: bool,
}

/// Challenge represents a PoPC verification challenge
#[derive(Debug, Clone)]
pub struct Challenge {
    pub job_id: String,
    pub samples: Vec<usize>,
    pub vrf_seed: [u8; 32],
    pub sample_size: usize,
}

/// ConsensusEngine manages the PoPC consensus
pub struct ConsensusEngine {
    validators: Arc<RwLock<HashMap<String, Validator>>>,
    config: ConsensusConfig,
}

/// Configuration for consensus engine
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    pub sample_size: usize,
    pub min_confidence: f64,
    pub fraud_window_blocks: u64,
    pub min_validator_stake: u128,
    pub false_pass_penalty_bps: u16, // basis points
}

impl ConsensusEngine {
    /// Creates a new consensus engine
    pub fn new(config: ConsensusConfig) -> Self {
        Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Registers a new validator
    pub async fn register_validator(&self, validator: Validator) -> Result<(), String> {
        if validator.stake < self.config.min_validator_stake {
            return Err("Insufficient stake".to_string());
        }

        let mut validators = self.validators.write().await;
        validators.insert(validator.address.clone(), validator);
        Ok(())
    }

    /// Generates a PoPC challenge
    pub fn generate_challenge(&self, job_id: String, output_size: usize, vrf_seed: [u8; 32]) -> Challenge {
        let sample_size = self.config.sample_size.min(output_size);
        
        // Generate deterministic samples using VRF seed
        let samples = self.generate_samples(output_size, sample_size, &vrf_seed);

        Challenge {
            job_id,
            samples,
            vrf_seed,
            sample_size,
        }
    }

    /// Verifies a proof against a challenge
    pub fn verify_proof(&self, _challenge: &Challenge, _proof_data: &[u8]) -> bool {
        // TODO: Implement Merkle proof verification
        true
    }

    /// Calculates fraud detection probability
    pub fn fraud_detection_probability(fraud_rate: f64, sample_size: usize) -> f64 {
        1.0 - (1.0 - fraud_rate).powi(sample_size as i32)
    }

    fn generate_samples(&self, output_size: usize, sample_size: usize, seed: &[u8; 32]) -> Vec<usize> {
        use sha3::{Sha3_256, Digest};
        
        let mut samples = Vec::with_capacity(sample_size);
        let mut hasher = Sha3_256::new();
        
        for i in 0..sample_size {
            hasher.update(seed);
            hasher.update(&i.to_le_bytes());
            let hash = hasher.finalize_reset();
            
            let index = u64::from_le_bytes(hash[0..8].try_into().unwrap()) as usize % output_size;
            samples.push(index);
        }
        
        samples
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            sample_size: 1000,
            min_confidence: 0.999,
            fraud_window_blocks: 720,
            min_validator_stake: 10_000 * 10_u128.pow(18),
            false_pass_penalty_bps: 500,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_validator() {
        let engine = ConsensusEngine::new(ConsensusConfig::default());
        
        let validator = Validator {
            address: "0x1234".to_string(),
            stake: 10_000 * 10_u128.pow(18),
            total_votes: 0,
            correct_votes: 0,
            false_pass: 0,
            is_active: true,
        };

        let result = engine.register_validator(validator).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_fraud_detection_probability() {
        let prob = ConsensusEngine::fraud_detection_probability(0.1, 100);
        assert!(prob > 0.9999);
    }

    #[test]
    fn test_generate_challenge() {
        let engine = ConsensusEngine::new(ConsensusConfig::default());
        let challenge = engine.generate_challenge(
            "job-123".to_string(),
            10000,
            [1u8; 32],
        );

        assert_eq!(challenge.job_id, "job-123");
        assert_eq!(challenge.samples.len(), 1000);
    }
}
