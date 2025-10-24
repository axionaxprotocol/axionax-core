#!/usr/bin/env python3
"""
Performance benchmark suite for Axionax v1.6
Compares Rust implementation performance across different operations
"""

import time
import sys
import os
from typing import List, Dict, Any
from dataclasses import dataclass, asdict
import json

# Add lib path for Rust bindings
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'deai', 'lib'))

try:
    import axionax_python as axx
    RUST_AVAILABLE = True
except ImportError:
    print("⚠️  Rust bindings not available, skipping benchmarks")
    RUST_AVAILABLE = False
    sys.exit(0)


@dataclass
class BenchmarkResult:
    """Benchmark result data"""
    name: str
    operations: int
    duration_seconds: float
    ops_per_second: float
    avg_latency_ms: float
    memory_mb: float = 0.0


class Benchmarker:
    """Performance benchmarking suite"""
    
    def __init__(self):
        self.results: List[BenchmarkResult] = []
    
    def benchmark(self, name: str, fn, iterations: int = 1000) -> BenchmarkResult:
        """Run a benchmark"""
        print(f"\n📊 {name}")
        print(f"   Running {iterations:,} iterations...")
        
        # Warmup
        for _ in range(10):
            fn()
        
        # Actual benchmark
        start = time.time()
        for _ in range(iterations):
            fn()
        duration = time.time() - start
        
        ops_per_sec = iterations / duration
        avg_latency_ms = (duration / iterations) * 1000
        
        result = BenchmarkResult(
            name=name,
            operations=iterations,
            duration_seconds=duration,
            ops_per_second=ops_per_sec,
            avg_latency_ms=avg_latency_ms
        )
        
        self.results.append(result)
        
        print(f"   ✓ {ops_per_sec:,.0f} ops/sec")
        print(f"   ✓ {avg_latency_ms:.3f}ms avg latency")
        
        return result
    
    def save_results(self, filename: str):
        """Save results to JSON"""
        with open(filename, 'w') as f:
            json.dump([asdict(r) for r in self.results], f, indent=2)
        print(f"\n💾 Results saved to {filename}")


def main():
    """Run all benchmarks"""
    print("=" * 70)
    print("⚡ Axionax v1.6 Performance Benchmarks")
    print("=" * 70)
    
    benchmarker = Benchmarker()
    
    # VRF Benchmarks
    vrf = axx.PyVRF()
    data_small = list(b"x" * 64)
    data_medium = list(b"x" * 1024)
    data_large = list(b"x" * 1024 * 10)
    
    benchmarker.benchmark(
        "VRF Proof (64 bytes)",
        lambda: vrf.prove(data_small),
        iterations=10000
    )
    
    benchmarker.benchmark(
        "VRF Proof (1 KB)",
        lambda: vrf.prove(data_medium),
        iterations=5000
    )
    
    benchmarker.benchmark(
        "VRF Proof (10 KB)",
        lambda: vrf.prove(data_large),
        iterations=1000
    )
    
    # Validator Benchmarks
    benchmarker.benchmark(
        "Validator Creation",
        lambda: axx.PyValidator(f"0x{os.urandom(20).hex()}", 1000000),
        iterations=50000
    )
    
    # Consensus Benchmarks
    engine = axx.PyConsensusEngine()
    validators = [axx.PyValidator(f"v{i}", 100000) for i in range(100)]
    for v in validators:
        engine.register_validator(v)
    
    benchmarker.benchmark(
        "Validator Registration",
        lambda: engine.register_validator(axx.PyValidator(f"v{os.urandom(8).hex()}", 100000)),
        iterations=1000
    )
    
    benchmarker.benchmark(
        "Challenge Generation",
        lambda: engine.generate_challenge(f"job_{os.urandom(8).hex()}", 1000),
        iterations=5000
    )
    
    benchmarker.benchmark(
        "Fraud Probability Calculation",
        lambda: axx.PyConsensusEngine.fraud_probability(0.1, 100),
        iterations=100000
    )
    
    # Blockchain Benchmarks
    blockchain = axx.PyBlockchain()
    
    benchmarker.benchmark(
        "Get Block by Number",
        lambda: blockchain.get_block(0),
        iterations=10000
    )
    
    benchmarker.benchmark(
        "Get Latest Block Number",
        lambda: blockchain.latest_block_number(),
        iterations=50000
    )
    
    # Transaction Benchmarks
    benchmarker.benchmark(
        "Transaction Creation",
        lambda: axx.PyTransaction(
            f"0x{os.urandom(20).hex()}",
            f"0x{os.urandom(20).hex()}",
            100,
            []
        ),
        iterations=50000
    )
    
    # Complex Operations
    def complex_consensus_flow():
        """Simulate a full consensus flow"""
        job_id = f"job_{os.urandom(8).hex()}"
        challenge = engine.generate_challenge(job_id, 1000)
        prob = axx.PyConsensusEngine.fraud_probability(0.05, challenge.sample_size)
        return prob
    
    benchmarker.benchmark(
        "Full Consensus Flow",
        complex_consensus_flow,
        iterations=1000
    )
    
    # Print summary
    print("\n" + "=" * 70)
    print("📈 Benchmark Summary")
    print("=" * 70)
    
    for result in benchmarker.results:
        print(f"\n{result.name}:")
        print(f"  • {result.ops_per_second:>12,.0f} ops/sec")
        print(f"  • {result.avg_latency_ms:>12.3f} ms/op")
    
    # Save results
    benchmarker.save_results("benchmark_results.json")
    
    # Performance targets
    print("\n" + "=" * 70)
    print("🎯 Performance Targets")
    print("=" * 70)
    
    targets = {
        "VRF Proof (64 bytes)": 20000,
        "Challenge Generation": 5000,
        "Validator Registration": 1000,
        "Full Consensus Flow": 500,
    }
    
    passed = 0
    failed = 0
    
    for result in benchmarker.results:
        if result.name in targets:
            target = targets[result.name]
            if result.ops_per_second >= target:
                print(f"✅ {result.name}: {result.ops_per_second:,.0f} >= {target:,} ops/sec")
                passed += 1
            else:
                print(f"❌ {result.name}: {result.ops_per_second:,.0f} < {target:,} ops/sec")
                failed += 1
    
    print(f"\n{passed} passed, {failed} failed")
    
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
