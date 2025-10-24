//! Axionax Cryptography
//!
//! VRF, signatures, and cryptographic primitives

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use sha3::{Digest, Sha3_256};

/// VRF (Verifiable Random Function) implementation
pub struct VRF {
    signing_key: SigningKey,
}

impl VRF {
    /// Creates a new VRF instance
    pub fn new() -> Self {
        let signing_key = SigningKey::from_bytes(&rand::random());
        Self { signing_key }
    }

    /// Creates VRF from existing signing key
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    /// Generates VRF proof and output
    pub fn prove(&self, input: &[u8]) -> (Vec<u8>, [u8; 32]) {
        // Simplified VRF: hash input with secret key
        let mut hasher = Sha3_256::new();
        hasher.update(self.signing_key.to_bytes());
        hasher.update(input);
        let hash = hasher.finalize();

        let signature = self.signing_key.sign(input);
        let proof = signature.to_bytes().to_vec();
        
        let mut output = [0u8; 32];
        output.copy_from_slice(&hash);
        
        (proof, output)
    }

    /// Verifies VRF proof
    pub fn verify(verifying_key: &VerifyingKey, input: &[u8], proof: &[u8], _output: &[u8; 32]) -> bool {
        if proof.len() != 64 {
            return false;
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(proof);
        
        let signature = Signature::from_bytes(&sig_bytes);
        verifying_key.verify(input, &signature).is_ok()
    }

    /// Gets verifying key (public key)
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}

/// Hash functions
pub mod hash {
    use super::*;

    pub fn sha3_256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }

    pub fn keccak256(data: &[u8]) -> [u8; 32] {
        use sha3::Keccak256;
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }
}

/// Digital signature utilities
pub mod signature {
    use super::*;

    pub fn sign(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
        signing_key.sign(message).to_bytes().to_vec()
    }

    pub fn verify(verifying_key: &VerifyingKey, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);

        let sig = Signature::from_bytes(&sig_bytes);
        verifying_key.verify(message, &sig).is_ok()
    }

    pub fn generate_keypair() -> SigningKey {
        SigningKey::from_bytes(&rand::random())
    }
}

impl Default for VRF {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrf_prove_verify() {
        let vrf = VRF::new();
        let input = b"test input";
        
        let (proof, output) = vrf.prove(input);
        let verifying_key = vrf.verifying_key();
        
        assert!(VRF::verify(&verifying_key, input, &proof, &output));
    }

    #[test]
    fn test_hash_sha3() {
        let data = b"hello world";
        let hash = hash::sha3_256(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_signature() {
        let signing_key = signature::generate_keypair();
        let message = b"sign this message";
        
        let sig = signature::sign(&signing_key, message);
        let verifying_key = signing_key.verifying_key();
        assert!(signature::verify(&verifying_key, message, &sig));
    }
}
