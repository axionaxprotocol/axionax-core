//! Axionax Cryptography
//!
//! Cryptographic primitives for Axionax blockchain:
//! - **VRF**: Verifiable Random Functions for consensus
//! - **Hash**: SHA3-256, Keccak256, Blake2s-256, Blake2b-512
//! - **Signatures**: Ed25519 digital signatures
//! - **KDF**: Argon2id key derivation and password hashing

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use sha3::{Digest, Sha3_256};

// Re-export commonly used KDF functions
pub use kdf::{derive_key, hash_password, verify_password};

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
    use blake2::{Blake2s256, Blake2b512};

    /// SHA3-256 hash function
    /// Use for: VRF, Consensus sampling, Standard compatibility
    pub fn sha3_256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }

    /// Keccak256 hash function (Ethereum compatibility)
    /// Use for: Smart contract hashing, EVM compatibility
    pub fn keccak256(data: &[u8]) -> [u8; 32] {
        use sha3::Keccak256;
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }

    /// Blake2s-256: Fast general-purpose hashing (32 bytes output)
    /// 
    /// **Performance**: 2-3x faster than SHA3-256
    /// 
    /// **Use for**:
    /// - Block header hashing
    /// - Transaction ID generation
    /// - Merkle tree nodes
    /// - General-purpose hashing where speed matters
    /// 
    /// # Examples
    /// ```
    /// use crypto::hash;
    /// 
    /// let data = b"hello world";
    /// let hash = hash::blake2s_256(data);
    /// assert_eq!(hash.len(), 32);
    /// ```
    pub fn blake2s_256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Blake2s256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Blake2b-512: Fast hashing with larger output (64 bytes)
    /// 
    /// **Performance**: 2-3x faster than SHA3-512
    /// 
    /// **Use for**:
    /// - VRF with extended output
    /// - Random sampling requiring more entropy
    /// - Applications needing extra security margin
    /// - HMAC and key derivation
    /// 
    /// # Examples
    /// ```
    /// use crypto::hash;
    /// 
    /// let data = b"hello world";
    /// let hash = hash::blake2b_512(data);
    /// assert_eq!(hash.len(), 64);
    /// ```
    pub fn blake2b_512(data: &[u8]) -> [u8; 64] {
        let mut hasher = Blake2b512::new();
        hasher.update(data);
        hasher.finalize().into()
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

/// Key Derivation Functions (KDF) module
pub mod kdf {
    use argon2::{
        Argon2, 
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier,
        password_hash::{rand_core::OsRng, SaltString, Error as Argon2Error}
    };

    /// Derive a 32-byte key from password using Argon2id
    /// 
    /// # Examples
    /// ```
    /// use crypto::kdf;
    /// 
    /// let password = b"my_password";
    /// let salt = b"unique_salt_for_this_user_16"; // >= 16 bytes
    /// let key = kdf::derive_key(password, salt).unwrap();
    /// assert_eq!(key.len(), 32);
    /// ```
    pub fn derive_key(password: &[u8], salt: &[u8]) -> Result<[u8; 32], Argon2Error> {
        let argon2 = Argon2::default();
        let mut output = [0u8; 32];
        
        argon2.hash_password_into(password, salt, &mut output)?;
        Ok(output)
    }

    /// Hash password for secure storage (includes automatic salt generation)
    /// 
    /// # Examples
    /// ```
    /// use crypto::kdf;
    /// 
    /// let password = b"user_password_123";
    /// let hash_str = kdf::hash_password(password).unwrap();
    /// 
    /// // Verify
    /// assert!(kdf::verify_password(password, &hash_str).unwrap());
    /// ```
    pub fn hash_password(password: &[u8]) -> Result<String, Argon2Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2.hash_password(password, &salt)
            .map(|hash| hash.to_string())
    }

    /// Verify password against stored Argon2 hash
    pub fn verify_password(password: &[u8], hash_str: &str) -> Result<bool, Argon2Error> {
        let parsed_hash = PasswordHash::new(hash_str)?;
        let argon2 = Argon2::default();
        
        Ok(argon2.verify_password(password, &parsed_hash).is_ok())
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
    fn test_hash_blake2s() {
        let data = b"hello world";
        let hash = hash::blake2s_256(data);
        assert_eq!(hash.len(), 32);
        
        // Same input = same output (determinism)
        let hash2 = hash::blake2s_256(data);
        assert_eq!(hash, hash2);
        
        // Different input = different output
        let hash3 = hash::blake2s_256(b"different data");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_hash_blake2b() {
        let data = b"hello world";
        let hash = hash::blake2b_512(data);
        assert_eq!(hash.len(), 64);
        
        // Determinism check
        let hash2 = hash::blake2b_512(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_blake2_vs_sha3_different_outputs() {
        let data = b"test data";
        
        let blake2s = hash::blake2s_256(data);
        let sha3 = hash::sha3_256(data);
        
        // Different algorithms should produce different hashes
        assert_ne!(blake2s, sha3);
    }

    #[test]
    fn test_blake2_performance() {
        use std::time::Instant;
        let data = b"x".repeat(1024); // 1KB data
        let iterations = 10000;
        
        // Blake2s benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = hash::blake2s_256(&data);
        }
        let blake2s_duration = start.elapsed();
        
        // SHA3 benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = hash::sha3_256(&data);
        }
        let sha3_duration = start.elapsed();
        
        println!("\nPerformance Comparison (1KB data, {} iterations):", iterations);
        println!("  Blake2s-256: {:.2}ms ({:.0} ops/sec)", 
                 blake2s_duration.as_secs_f64() * 1000.0,
                 iterations as f64 / blake2s_duration.as_secs_f64());
        println!("  SHA3-256:    {:.2}ms ({:.0} ops/sec)", 
                 sha3_duration.as_secs_f64() * 1000.0,
                 iterations as f64 / sha3_duration.as_secs_f64());
        println!("  Speedup:     {:.2}x faster", 
                 sha3_duration.as_secs_f64() / blake2s_duration.as_secs_f64());
        
        // Blake2s should be faster than SHA3
        assert!(blake2s_duration < sha3_duration, 
                "Blake2s should be faster than SHA3");
    }

    #[test]
    fn test_signature() {
        let signing_key = signature::generate_keypair();
        let message = b"sign this message";
        
        let sig = signature::sign(&signing_key, message);
        let verifying_key = signing_key.verifying_key();
        assert!(signature::verify(&verifying_key, message, &sig));
    }

    // KDF tests
    #[test]
    fn test_kdf_key_derivation() {
        let password = b"my_password";
        let salt = b"unique_salt_1234567890123456"; // 28 bytes
        
        let key1 = kdf::derive_key(password, salt).unwrap();
        let key2 = kdf::derive_key(password, salt).unwrap();
        
        // Same input = same key (deterministic)
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
        
        // Different salt = different key
        let salt2 = b"different_salt_12345678901234";
        let key3 = kdf::derive_key(password, salt2).unwrap();
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_kdf_password_hash_verify() {
        let password = b"my_secure_password_123";
        
        // Hash password
        let hash = kdf::hash_password(password).unwrap();
        
        // Hash should be in PHC string format
        assert!(hash.starts_with("$argon2"));
        
        // Verify correct password
        assert!(kdf::verify_password(password, &hash).unwrap());
        
        // Verify wrong password
        assert!(!kdf::verify_password(b"wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_kdf_unique_salts() {
        let password = b"same_password";
        
        // Hash same password twice
        let hash1 = kdf::hash_password(password).unwrap();
        let hash2 = kdf::hash_password(password).unwrap();
        
        // Different salts = different hashes
        assert_ne!(hash1, hash2);
        
        // But both should verify correctly
        assert!(kdf::verify_password(password, &hash1).unwrap());
        assert!(kdf::verify_password(password, &hash2).unwrap());
    }
}
