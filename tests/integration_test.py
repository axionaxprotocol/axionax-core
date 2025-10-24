"""
Integration tests for Rust-Python bridge
Tests the full stack: Rust core → Python bindings → DeAI layer
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'lib'))

import pytest
import axionax_python as axx
from deai.asr import AutoSelectionRouter, WorkerProfile
from deai.fraud_detection import FraudDetector, ProofFeatures

class TestRustPythonBridge:
    """Test PyO3 bindings work correctly"""
    
    def test_crypto_hashing(self):
        """Test hash functions exposed from Rust"""
        data = b"Hello Axionax"
        
        # Test SHA3-256
        hash1 = axx.PyCrypto.sha3_256(list(data))
        assert len(hash1) == 32
        
        # Test Keccak256
        hash2 = axx.PyCrypto.keccak256(list(data))
        assert len(hash2) == 32
        
        # Same input should give same output
        hash1_again = axx.PyCrypto.sha3_256(list(data))
        assert hash1 == hash1_again
    
    def test_vrf_operations(self):
        """Test VRF prove and verify"""
        seed = [1, 2, 3, 4] * 8  # 32 bytes
        vrf = axx.PyVRF(seed)
        
        input_data = b"test_input"
        proof, hash_output = vrf.prove(list(input_data))
        
        # Proof and hash should be generated
        assert len(proof) > 0
        assert len(hash_output) > 0
        
        # Verification should succeed
        is_valid = vrf.verify(list(input_data), proof, hash_output)
        assert is_valid
        
        # Wrong input should fail
        is_invalid = vrf.verify(list(b"wrong_input"), proof, hash_output)
        assert not is_invalid
    
    def test_validator_operations(self):
        """Test validator management"""
        validator = axx.PyValidator(
            address="0x1234567890123456789012345678901234567890",
            stake=1000000,
            reputation=0.95
        )
        
        assert validator.address == "0x1234567890123456789012345678901234567890"
        assert validator.stake == 1000000
        assert validator.reputation == 0.95
        assert validator.fraud_count == 0
        assert validator.total_checks == 0
    
    def test_consensus_engine(self):
        """Test consensus engine operations"""
        engine = axx.PyConsensusEngine()
        
        # Register validators
        v1 = axx.PyValidator("validator1", 500000, 0.9)
        v2 = axx.PyValidator("validator2", 1000000, 0.95)
        
        engine.register_validator(v1)
        engine.register_validator(v2)
        
        # Check validators registered
        validators = engine.get_validators()
        assert len(validators) == 2
        
        # Calculate fraud probability
        prob = engine.fraud_probability("validator1")
        assert 0.0 <= prob <= 1.0
        
        # Generate challenge
        block_hash = [0] * 32
        challenge = engine.generate_challenge(block_hash, "validator1")
        assert len(challenge) > 0
    
    def test_blockchain_operations(self):
        """Test blockchain operations"""
        blockchain = axx.PyBlockchain()
        
        # Should have genesis block
        assert blockchain.height() == 1
        
        genesis = blockchain.get_latest_block()
        assert genesis is not None
        assert genesis.number == 0
        
        # Add a block with transactions
        tx1 = axx.PyTransaction(
            from_addr="0xabc",
            to="0xdef",
            amount=100,
            data=[]
        )
        tx2 = axx.PyTransaction(
            from_addr="0xdef",
            to="0x123",
            amount=50,
            data=[]
        )
        
        blockchain.add_block([tx1, tx2])
        
        # Check chain grew
        assert blockchain.height() == 2
        
        latest = blockchain.get_latest_block()
        assert latest.number == 1
        assert latest.transactions_count == 2
        
        # Get block by number
        block_1 = blockchain.get_block(1)
        assert block_1 is not None
        assert block_1.number == 1


class TestFullStackIntegration:
    """Test complete integration: Rust → Python → DeAI"""
    
    def test_consensus_with_asr(self):
        """Test consensus engine integrated with ASR"""
        # Create Rust consensus engine
        engine = axx.PyConsensusEngine()
        
        # Register validators
        validators = [
            axx.PyValidator(f"validator_{i}", 500000 + i * 100000, 0.8 + i * 0.05)
            for i in range(5)
        ]
        
        for v in validators:
            engine.register_validator(v)
        
        # Convert to WorkerProfile for ASR
        workers = []
        rust_validators = engine.get_validators()
        for v in rust_validators:
            worker = WorkerProfile(
                address=v.address,
                reputation=v.reputation,
                stake=v.stake,
                compute_power=100.0,
                bandwidth=1000.0,
                success_rate=v.reputation,
                avg_response_time=50.0,
                specialization="general"
            )
            workers.append(worker)
        
        # Use ASR to select workers
        asr = AutoSelectionRouter()
        job_requirements = {
            "compute": 50.0,
            "bandwidth": 500.0,
            "reputation_min": 0.8,
            "task_type": "general"
        }
        
        selected = asr.select_workers(workers, job_requirements, k=3)
        
        assert len(selected) <= 3
        for worker in selected:
            assert worker.reputation >= 0.8
    
    def test_fraud_detection_with_consensus(self):
        """Test fraud detection with consensus data"""
        # Create blockchain and consensus
        blockchain = axx.PyBlockchain()
        engine = axx.PyConsensusEngine()
        
        # Register validators
        v1 = axx.PyValidator("honest_validator", 1000000, 0.95)
        v2 = axx.PyValidator("suspicious_validator", 500000, 0.6)
        
        engine.register_validator(v1)
        engine.register_validator(v2)
        
        # Generate challenges for fraud detection
        block_hash = list(b"block_hash_123" * 3)  # 42 bytes
        
        challenge1 = engine.generate_challenge(block_hash, "honest_validator")
        challenge2 = engine.generate_challenge(block_hash, "suspicious_validator")
        
        # Create fraud detector
        detector = FraudDetector()
        
        # Create proof features (simulated)
        proof1 = ProofFeatures(
            response_time=45.0,
            proof_size=256,
            validation_score=0.98,
            validator_reputation=0.95,
            stake_amount=1000000,
            historical_accuracy=0.97,
            challenge_difficulty=0.7
        )
        
        proof2 = ProofFeatures(
            response_time=150.0,  # Suspiciously slow
            proof_size=512,       # Suspiciously large
            validation_score=0.65,
            validator_reputation=0.6,
            stake_amount=500000,
            historical_accuracy=0.62,
            challenge_difficulty=0.7
        )
        
        # Train detector with normal proofs
        normal_proofs = [
            ProofFeatures(40 + i, 240 + i*10, 0.95 + i*0.01, 0.9, 1000000, 0.94, 0.7)
            for i in range(10)
        ]
        detector.train(normal_proofs)
        
        # Detect fraud
        risk1 = detector.detect_fraud(proof1)
        risk2 = detector.detect_fraud(proof2)
        
        # Honest validator should have low risk
        assert risk1["risk_score"] < 0.5
        
        # Suspicious validator should have higher risk
        assert risk2["risk_score"] > risk1["risk_score"]
    
    def test_end_to_end_transaction_flow(self):
        """Test complete transaction flow with all components"""
        # 1. Create blockchain
        blockchain = axx.PyBlockchain()
        
        # 2. Create consensus engine
        engine = axx.PyConsensusEngine()
        
        # 3. Register validators
        validators = [
            axx.PyValidator(f"0xvalidator{i}", 1000000, 0.9)
            for i in range(3)
        ]
        for v in validators:
            engine.register_validator(v)
        
        # 4. Create transactions
        transactions = [
            axx.PyTransaction(f"0xsender{i}", f"0xreceiver{i}", 100 * i, [])
            for i in range(5)
        ]
        
        # 5. Generate VRF challenge for block
        block_hash = list(b"new_block_hash" + b"0" * 18)  # 32 bytes
        challenge = engine.generate_challenge(block_hash, "0xvalidator0")
        
        assert len(challenge) > 0
        
        # 6. Add block to chain
        blockchain.add_block(transactions)
        
        # 7. Verify chain state
        assert blockchain.height() == 2
        latest = blockchain.get_latest_block()
        assert latest.transactions_count == 5
        
        # 8. Calculate fraud probability for validator
        fraud_prob = engine.fraud_probability("0xvalidator0")
        assert 0.0 <= fraud_prob <= 1.0


class TestPerformance:
    """Performance benchmarks for integration"""
    
    def test_hash_performance(self):
        """Benchmark hashing speed"""
        import time
        
        data = list(b"x" * 1024)  # 1KB
        iterations = 1000
        
        start = time.time()
        for _ in range(iterations):
            axx.PyCrypto.sha3_256(data)
        duration = time.time() - start
        
        ops_per_sec = iterations / duration
        print(f"\nSHA3-256: {ops_per_sec:.0f} ops/sec")
        
        # Should be reasonably fast
        assert ops_per_sec > 500
    
    def test_blockchain_performance(self):
        """Benchmark block addition speed"""
        import time
        
        blockchain = axx.PyBlockchain()
        
        # Create 100 transactions
        transactions = [
            axx.PyTransaction(f"0x{i}", f"0x{i+1}", i, [])
            for i in range(100)
        ]
        
        start = time.time()
        blockchain.add_block(transactions)
        duration = time.time() - start
        
        print(f"\nBlock with 100 txs: {duration*1000:.2f}ms")
        
        # Should complete in reasonable time
        assert duration < 1.0  # Less than 1 second


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
