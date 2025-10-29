// core/src/popc/mod.rs
// Proof-of-Probabilistic-Checking (PoPC) migrated from Go to Rust

use anyhow::{Result, anyhow};
use primitive_types::H256;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// --- Configuration ---

pub struct PoPCConfig {
    pub sample_size: usize,
    pub min_confidence: f64,
    pub stratified_sampling: bool,
    pub adaptive_escalation: bool,
}

impl Default for PoPCConfig {
    fn default() -> Self {
        Self {
            sample_size: 50,
            min_confidence: 0.99,
            stratified_sampling: true,
            adaptive_escalation: false,
        }
    }
}

// --- Core Structures ---

#[derive(Debug, Clone)]
pub struct Challenge {
    pub job_id: String,
    pub samples: Vec<usize>,
    pub vrf_seed: H256,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub job_id: String,
    pub samples: HashMap<usize, Vec<u8>>,
    pub merkle_paths: HashMap<usize, Vec<H256>>,
    pub output_root: H256,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub job_id: String,
    pub passed: bool,
    pub samples_verified: usize,
    pub samples_total: usize,
    pub confidence: f64,
    pub errors: Vec<String>,
}

// --- Validator Logic ---

pub struct Validator {
    config: PoPCConfig,
}

impl Validator {
    pub fn new(config: PoPCConfig) -> Self {
        Self { config }
    }

    /// Generates a PoPC challenge using a VRF seed.
    pub fn generate_challenge(&self, job_id: String, output_size: usize, vrf_seed: H256) -> Challenge {
        let mut seed_bytes = [0u8; 32];
        seed_bytes.copy_from_slice(vrf_seed.as_bytes());
        let mut rng = StdRng::from_seed(seed_bytes);

        let sample_size = self.config.sample_size.min(output_size);

        let samples = if self.config.stratified_sampling {
            self.generate_stratified_samples(&mut rng, output_size, sample_size)
        } else {
            self.generate_random_samples(&mut rng, output_size, sample_size)
        };

        Challenge { job_id, samples, vrf_seed }
    }

    fn generate_random_samples(&self, rng: &mut StdRng, output_size: usize, sample_size: usize) -> Vec<usize> {
        (0..sample_size).map(|_| rng.gen_range(0..output_size)).collect()
    }

    fn generate_stratified_samples(&self, rng: &mut StdRng, output_size: usize, sample_size: usize) -> Vec<usize> {
        if output_size == 0 || sample_size == 0 {
            return vec![];
        }

        let mut samples = Vec::with_capacity(sample_size);
        let strata_count = (sample_size as f64).sqrt() as usize;
        if strata_count == 0 {
            return self.generate_random_samples(rng, output_size, sample_size);
        }

        let strata_size = output_size / strata_count;
        let samples_per_strata = sample_size / strata_count;

        for i in 0..strata_count {
            let start = i * strata_size;
            let end = if i == strata_count - 1 { output_size } else { start + strata_size };
            for _ in 0..samples_per_strata {
                if start < end {
                    samples.push(rng.gen_range(start..end));
                }
            }
        }

        // Fill remaining samples
        while samples.len() < sample_size {
            samples.push(rng.gen_range(0..output_size));
        }

        samples
    }

    /// Verifies a worker's proof against a challenge.
    pub fn verify_proof(&self, challenge: &Challenge, proof: &Proof) -> ValidationResult {
        let mut verified = 0;
        let mut errors = Vec::new();

        for &idx in &challenge.samples {
            match self.verify_sample(idx, proof) {
                Ok(_) => verified += 1,
                Err(e) => errors.push(e.to_string()),
            }
        }

        let confidence = self.calculate_confidence(verified, challenge.samples.len());
        let passed = confidence >= self.config.min_confidence;

        ValidationResult {
            job_id: challenge.job_id.clone(),
            passed,
            samples_verified: verified,
            samples_total: challenge.samples.len(),
            confidence,
            errors,
        }
    }

    fn verify_sample(&self, index: usize, proof: &Proof) -> Result<()> {
        let sample_data = proof.samples.get(&index)
            .ok_or_else(|| anyhow!("Missing sample at index {}", index))?;

        let merkle_path = proof.merkle_paths.get(&index)
            .ok_or_else(|| anyhow!("Missing Merkle path for index {}", index))?;

        if !Self::verify_merkle_proof(sample_data, merkle_path, index, proof.output_root) {
            return Err(anyhow!("Invalid Merkle proof for index {}", index));
        }

        Ok(())
    }

    /// Verifies a Merkle proof for a single sample.
    pub fn verify_merkle_proof(data: &[u8], path: &[H256], index: usize, root: H256) -> bool {
        let mut current_hash = H256::from_slice(Sha256::digest(data).as_slice());

        for (i, sibling) in path.iter().enumerate() {
            let mut hasher = Sha256::new();
            if (index >> i) & 1 == 0 {
                // Current is left child
                hasher.update(current_hash.as_bytes());
                hasher.update(sibling.as_bytes());
            } else {
                // Current is right child
                hasher.update(sibling.as_bytes());
                hasher.update(current_hash.as_bytes());
            }
            current_hash = H256::from_slice(hasher.finalize().as_slice());
        }

        current_hash == root
    }

    fn calculate_confidence(&self, verified: usize, total: usize) -> f64 {
        if total == 0 { return 0.0; }
        (verified as f64) / (total as f64)
    }
}

/// Estimates the probability of detecting fraud.
/// P_detect = 1 - (1 - f)^s
pub fn estimate_fraud_detection_probability(fraud_rate: f64, sample_size: usize) -> f64 {
    1.0 - (1.0 - fraud_rate).powf(sample_size as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    // A simple Merkle tree for testing
    fn build_test_tree(data: &Vec<Vec<u8>>) -> (H256, HashMap<usize, Vec<H256>>) {
        let leaves: Vec<H256> = data.iter().map(|d| H256::from_slice(Sha256::digest(d).as_slice())).collect();
        let mut paths = HashMap::new();
        let root = build_tree_level(&leaves, 0, &mut paths);
        (root, paths)
    }

    fn build_tree_level(nodes: &[H256], level_idx: usize, paths: &mut HashMap<usize, Vec<H256>>) -> H256 {
        if nodes.len() == 1 {
            return nodes[0];
        }

        // Add siblings to paths
        for (i, node) in nodes.iter().enumerate() {
            let sibling_idx = if i % 2 == 0 { i + 1 } else { i - 1 };
             if sibling_idx < nodes.len() {
                 // This is a naive way to find original index, works for power of 2
                 let original_idx = i * (1 << level_idx);
                 if let Some(path) = paths.get_mut(&original_idx) {
                    path.push(nodes[sibling_idx]);
                 } else {
                    paths.insert(original_idx, vec![nodes[sibling_idx]]);
                 }
             }
        }

        let parents: Vec<H256> = nodes.chunks(2).map(|pair| {
            let mut hasher = Sha256::new();
            hasher.update(pair[0].as_bytes());
            if pair.len() > 1 {
                hasher.update(pair[1].as_bytes());
            } else {
                hasher.update(pair[0].as_bytes()); // Duplicate if odd
            }
            H256::from_slice(hasher.finalize().as_slice())
        }).collect();

        build_tree_level(&parents, level_idx + 1, paths)
    }

    #[test]
    fn test_generate_challenge() {
        let config = PoPCConfig::default();
        let validator = Validator::new(config);
        let challenge = validator.generate_challenge("job-1".to_string(), 1000, H256::random());
        assert_eq!(challenge.samples.len(), 50);
    }

    // Note: A full Merkle proof verification test is complex.
    // The provided test structure is a starting point.
    // A robust implementation would require a proper Merkle tree library.
}
