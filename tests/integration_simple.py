"""
Simplified Integration Tests for Rust-Python Bridge
Tests basic functionality without complex dependencies
"""

import sys
import os

# Add lib path for Rust bindings
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'deai', 'lib'))

try:
    import axionax_python as axx
    RUST_AVAILABLE = True
except ImportError as e:
    print(f"⚠️  Warning: Could not import axionax_python: {e}")
    RUST_AVAILABLE = False


def test_vrf_basic():
    """Test basic VRF operations"""
    if not RUST_AVAILABLE:
        print("⏭️  Skipping VRF test (Rust bindings not available)")
        return
    
    print("\n🔐 Testing VRF...")
    vrf = axx.PyVRF()
    
    input_data = b"test_input"
    proof, hash_output = vrf.prove(list(input_data))
    
    assert len(proof) > 0, "Proof should be generated"
    assert len(hash_output) == 32, "Hash should be 32 bytes"
    
    print(f"   ✓ VRF proof generated: {len(proof)} bytes")
    print(f"   ✓ VRF hash: {bytes(hash_output[:8]).hex()}...")


def test_validator_basic():
    """Test basic validator operations"""
    if not RUST_AVAILABLE:
        print("⏭️  Skipping validator test")
        return
    
    print("\n👥 Testing Validators...")
    v1 = axx.PyValidator("0x1234", 1000000)
    
    assert v1.address == "0x1234"
    assert v1.stake == 1000000
    assert v1.is_active == True
    
    print(f"   ✓ Validator created: {v1.address}")
    print(f"   ✓ Stake: {v1.stake}")


def test_consensus_basic():
    """Test basic consensus operations"""
    if not RUST_AVAILABLE:
        print("⏭️  Skipping consensus test")
        return
    
    print("\n⚙️  Testing Consensus Engine...")
    engine = axx.PyConsensusEngine()
    
    # Register validators
    v1 = axx.PyValidator("validator1", 500000)
    v2 = axx.PyValidator("validator2", 1000000)
    
    engine.register_validator(v1)
    engine.register_validator(v2)
    
    print("   ✓ Registered 2 validators")
    
    # Generate challenge
    challenge = engine.generate_challenge("job_123", 1000)
    assert challenge.job_id == "job_123"
    assert challenge.sample_size > 0
    
    print(f"   ✓ Challenge generated: job={challenge.job_id}, samples={challenge.sample_size}")
    
    # Fraud probability
    prob = axx.PyConsensusEngine.fraud_probability(0.1, 100)
    assert 0.0 <= prob <= 1.0
    
    print(f"   ✓ Fraud probability (10% fraud, 100 samples): {prob:.2%}")


def test_blockchain_basic():
    """Test basic blockchain operations"""
    if not RUST_AVAILABLE:
        print("⏭️  Skipping blockchain test")
        return
    
    print("\n⛓️  Testing Blockchain...")
    blockchain = axx.PyBlockchain()
    
    # Should have genesis block
    height = blockchain.latest_block_number()
    assert height == 0, "Genesis should be block 0"
    
    genesis = blockchain.get_block(0)
    assert genesis is not None
    assert genesis.number == 0
    
    print(f"   ✓ Genesis block created: #{genesis.number}")
    print(f"   ✓ Proposer: {genesis.proposer}")
    print(f"   ✓ Gas limit: {genesis.gas_used}")


def test_performance_basic():
    """Basic performance tests"""
    if not RUST_AVAILABLE:
        print("⏭️  Skipping performance test")
        return
    
    print("\n📊 Testing Performance...")
    import time
    
    # VRF performance
    vrf = axx.PyVRF()
    data = list(b"x" * 1024)
    iterations = 100
    
    start = time.time()
    for _ in range(iterations):
        vrf.prove(data)
    duration = time.time() - start
    
    ops_per_sec = iterations / duration
    print(f"   ✓ VRF: {ops_per_sec:.0f} ops/sec")
    
    # Validator registration performance
    engine = axx.PyConsensusEngine()
    validators = [axx.PyValidator(f"v{i}", 100000) for i in range(100)]
    
    start = time.time()
    for v in validators:
        engine.register_validator(v)
    duration = time.time() - start
    
    print(f"   ✓ Validator registration: {len(validators)} validators in {duration*1000:.2f}ms")


def main():
    print("=" * 60)
    print("🧪 Axionax Integration Tests (Simplified)")
    print("=" * 60)
    
    tests = [
        test_vrf_basic,
        test_validator_basic,
        test_consensus_basic,
        test_blockchain_basic,
        test_performance_basic,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except Exception as e:
            print(f"\n❌ Test failed: {test.__name__}")
            print(f"   Error: {e}")
            import traceback
            traceback.print_exc()
            failed += 1
    
    print("\n" + "=" * 60)
    print(f"Results: {passed} passed, {failed} failed")
    print("=" * 60)
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
