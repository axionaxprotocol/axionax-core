# Axionax Crypto Module - Usage Guide

Complete guide for using cryptographic functions in Axionax Core.

## 📚 Table of Contents

1. [Hash Functions](#hash-functions)
2. [Key Derivation (KDF)](#key-derivation-kdf)
3. [Digital Signatures](#digital-signatures)
4. [VRF (Verifiable Random Function)](#vrf-verifiable-random-function)
5. [Performance Comparison](#performance-comparison)
6. [Security Best Practices](#security-best-practices)

---

## Hash Functions

### SHA3-256 (Standard)

**Use for**: VRF, Consensus sampling, Standard compatibility

```rust
use crypto::hash;

let data = b"hello world";
let hash = hash::sha3_256(data);
assert_eq!(hash.len(), 32);
```

### Keccak256 (Ethereum Compatible)

**Use for**: Smart contract hashing, EVM compatibility

```rust
use crypto::hash;

let contract_code = b"contract code...";
let hash = hash::keccak256(contract_code);
```

### Blake2s-256 (Fast, 32-byte output)

**Use for**: Block headers, Transaction IDs, Merkle trees

```rust
use crypto::hash;

// Hash block header
let block_header = b"block data...";
let block_hash = hash::blake2s_256(block_header);

// Transaction ID
let tx_data = b"transaction data...";
let tx_id = hash::blake2s_256(tx_data);

// Merkle tree node
let left = hash::blake2s_256(b"left child");
let right = hash::blake2s_256(b"right child");
let parent = hash::blake2s_256(&[&left[..], &right[..]].concat());
```

**Performance**: 2-3x faster than SHA3-256

### Blake2b-512 (Fast, 64-byte output)

**Use for**: Extended security, VRF with more entropy

```rust
use crypto::hash;

let data = b"data requiring extended hash";
let hash = hash::blake2b_512(data);
assert_eq!(hash.len(), 64);
```

---

## Key Derivation (KDF)

### Password-Based Key Derivation

**Use for**: Wallet key generation, Encryption key derivation

```rust
use crypto::kdf;

// Derive encryption key from user password
let password = b"user_password_123";
let wallet_address = b"0x1234567890abcdef1234567890abcdef12345678"; // Use as salt
let encryption_key = kdf::derive_key(password, wallet_address)?;

// Use encryption_key for AES-256 or similar
```

**Important**: 
- Salt must be **unique per user/wallet**
- Salt should be >= 16 bytes (recommend 32 bytes)
- Same password + salt = same key (deterministic)

### Password Hashing for Storage

**Use for**: User authentication, Secure password storage

```rust
use crypto::kdf;

// Registration: Hash password for storage
let user_password = b"MySecurePassword123!";
let password_hash = kdf::hash_password(user_password)?;

// Store password_hash in database
save_to_database(username, &password_hash);

// Login: Verify password
let stored_hash = load_from_database(username);
let is_valid = kdf::verify_password(user_password, &stored_hash)?;

if is_valid {
    println!("Login successful!");
} else {
    println!("Invalid password");
}
```

**Security Features**:
- Automatic salt generation (cryptographically random)
- Memory-hard (resistant to ASIC/GPU attacks)
- Timing-attack resistant
- PHC string format (portable)

### Complete Wallet Example

```rust
use crypto::kdf;

struct Wallet {
    address: String,
    password_hash: String,
}

impl Wallet {
    /// Create new wallet with password
    fn new(password: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate wallet address (simplified)
        let address = format!("0x{:x}", rand::random::<u128>());
        
        // Hash password for storage
        let password_hash = kdf::hash_password(password)?;
        
        Ok(Self { address, password_hash })
    }
    
    /// Derive encryption key from password
    fn derive_key(&self, password: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // Use wallet address as salt
        let salt = self.address.as_bytes();
        Ok(kdf::derive_key(password, salt)?)
    }
    
    /// Verify password
    fn verify_password(&self, password: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(kdf::verify_password(password, &self.password_hash)?)
    }
}

// Usage
let wallet = Wallet::new(b"MySecurePassword123!")?;

// Verify password
assert!(wallet.verify_password(b"MySecurePassword123!")?);
assert!(!wallet.verify_password(b"WrongPassword")?);

// Derive key for encryption
let encryption_key = wallet.derive_key(b"MySecurePassword123!")?;
```

---

## Digital Signatures

### Ed25519 Signatures

**Use for**: Transaction signing, Message authentication

```rust
use crypto::signature;

// Generate keypair
let signing_key = signature::generate_keypair();
let verifying_key = signing_key.verifying_key();

// Sign message
let message = b"Transfer 100 AXX to 0x123...";
let signature = signature::sign(&signing_key, message);

// Verify signature
let is_valid = signature::verify(&verifying_key, message, &signature);
assert!(is_valid);
```

### Transaction Signing Example

```rust
use crypto::signature;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    nonce: u64,
    signature: Vec<u8>,
}

impl Transaction {
    fn sign(&mut self, signing_key: &SigningKey) {
        // Serialize transaction data (without signature)
        let data = format!("{}{}{}{}", self.from, self.to, self.amount, self.nonce);
        
        // Sign
        self.signature = signature::sign(signing_key, data.as_bytes());
    }
    
    fn verify(&self, verifying_key: &VerifyingKey) -> bool {
        let data = format!("{}{}{}{}", self.from, self.to, self.amount, self.nonce);
        signature::verify(verifying_key, data.as_bytes(), &self.signature)
    }
}
```

---

## VRF (Verifiable Random Function)

### Consensus Randomness

**Use for**: Validator selection, Challenge generation

```rust
use crypto::VRF;

// Create VRF instance
let vrf = VRF::new();

// Generate proof and random output
let input = b"block_number_12345";
let (proof, random_output) = vrf.prove(input);

// Get public key
let verifying_key = vrf.verifying_key();

// Anyone can verify the proof
let is_valid = VRF::verify(&verifying_key, input, &proof, &random_output);
assert!(is_valid);
```

### Validator Selection Example

```rust
use crypto::VRF;

fn select_validator(
    validators: &[String],
    block_number: u64,
    vrf: &VRF
) -> String {
    // Generate randomness from block number
    let input = format!("block_{}", block_number);
    let (_proof, random_output) = vrf.prove(input.as_bytes());
    
    // Use random output to select validator
    let index = u64::from_be_bytes(random_output[0..8].try_into().unwrap());
    let selected_index = (index as usize) % validators.len();
    
    validators[selected_index].clone()
}
```

---

## Performance Comparison

### Hash Functions (1KB data, 10,000 iterations)

| Algorithm | Speed (ops/sec) | Use Case |
|-----------|-----------------|----------|
| **SHA3-256** | 500-1,000 | VRF, Standard compliance |
| **Keccak256** | 500-1,000 | EVM compatibility |
| **Blake2s-256** | 1,500-3,000 | ⚡ General hashing (2-3x faster) |
| **Blake2b-512** | 1,200-2,500 | ⚡ Extended security |
| **Argon2id** | 10-100 | 🔐 Password/key derivation |

### Benchmarking

```rust
use std::time::Instant;
use crypto::hash;

fn benchmark_hash() {
    let data = b"x".repeat(1024); // 1KB
    let iterations = 10000;
    
    // Blake2s
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = hash::blake2s_256(&data);
    }
    let blake2s_time = start.elapsed();
    
    // SHA3
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = hash::sha3_256(&data);
    }
    let sha3_time = start.elapsed();
    
    println!("Blake2s: {:.0} ops/sec", 
             iterations as f64 / blake2s_time.as_secs_f64());
    println!("SHA3:    {:.0} ops/sec", 
             iterations as f64 / sha3_time.as_secs_f64());
    println!("Speedup: {:.2}x", 
             sha3_time.as_secs_f64() / blake2s_time.as_secs_f64());
}
```

---

## Security Best Practices

### 1. Hash Function Selection

✅ **DO**:
- Use **Blake2s-256** for general-purpose hashing (blocks, transactions, merkle trees)
- Use **SHA3-256** for VRF and consensus (standard compatibility)
- Use **Keccak256** only for Ethereum compatibility

❌ **DON'T**:
- Don't use fast hashes (Blake2) for password hashing
- Don't use slow hashes (Argon2) for general data hashing

### 2. Key Derivation

✅ **DO**:
- Always use **unique salts** per user/wallet
- Use salt length >= 16 bytes (recommend 32 bytes)
- Store wallet address or user ID as part of salt
- Use Argon2id for all password-based key derivation

❌ **DON'T**:
- Never use same salt for multiple users
- Never use predictable salts (timestamps, sequential numbers)
- Never derive keys using fast hashes (SHA3, Blake2)

### 3. Password Storage

✅ **DO**:
- Use `kdf::hash_password()` with automatic salt generation
- Store only the PHC string format hash
- Verify with `kdf::verify_password()`
- Use strong password requirements (length, complexity)

❌ **DON'T**:
- Never store passwords in plaintext
- Never use reversible encryption for passwords
- Never implement custom password hashing

### 4. Signatures

✅ **DO**:
- Sign the complete message/transaction
- Include nonce or timestamp to prevent replay attacks
- Verify signatures before processing transactions
- Use Ed25519 (32-byte keys, 64-byte signatures)

❌ **DON'T**:
- Don't sign partial data
- Don't reuse nonces
- Don't accept signatures without verification

---

## Algorithm Selection Guide

### For Block Hashing
```rust
// ✅ Recommended: Blake2s-256 (fast)
let block_hash = hash::blake2s_256(&block_data);
```

### For Transaction IDs
```rust
// ✅ Recommended: Blake2s-256 (fast + 32 bytes)
let tx_id = hash::blake2s_256(&tx_data);
```

### For Merkle Trees
```rust
// ✅ Recommended: Blake2s-256 (fast, many hashes)
let parent = hash::blake2s_256(&[&left[..], &right[..]].concat());
```

### For VRF / Consensus
```rust
// ✅ Recommended: SHA3-256 (standard, verified)
// Already used in VRF implementation
```

### For Wallet Key Derivation
```rust
// ✅ Recommended: Argon2id (secure, memory-hard)
let key = kdf::derive_key(password, wallet_address.as_bytes())?;
```

### For Password Storage
```rust
// ✅ Recommended: Argon2id with auto-salt
let hash = kdf::hash_password(password)?;
```

### For Smart Contract Hashing
```rust
// ✅ Recommended: Keccak256 (EVM compatible)
let contract_hash = hash::keccak256(&contract_code);
```

---

## Testing

Run all crypto tests:
```bash
cargo test -p crypto
```

Run with performance output:
```bash
cargo test -p crypto -- --nocapture
```

Run specific test:
```bash
cargo test -p crypto test_blake2_performance -- --nocapture
```

---

## Dependencies

```toml
[dependencies]
ed25519-dalek = "2.2"   # Ed25519 signatures
sha3 = "0.10"           # SHA3 and Keccak
blake2 = "0.10"         # Blake2s/Blake2b
argon2 = "0.5"          # Argon2id KDF
rand = "0.8"            # Random number generation
```

All from **RustCrypto** organization - well-maintained, audited, production-ready.

---

## License

Apache 2.0 License - See LICENSE file for details.
