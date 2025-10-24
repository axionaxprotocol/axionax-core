"""
Axionax DeAI - Fraud Detection System

ML-based fraud detection for PoPC verification
"""

import numpy as np
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from sklearn.ensemble import IsolationForest


@dataclass
class ProofData:
    """PoPC Proof data structure"""
    job_id: str
    worker_address: str
    samples: Dict[int, bytes]
    merkle_paths: Dict[int, List[bytes]]
    output_root: bytes
    timestamp: int


@dataclass
class FraudAnalysis:
    """Fraud detection result"""
    is_suspicious: bool
    confidence: float
    anomaly_score: float
    reasons: List[str]


class FraudDetector:
    """
    ML-based fraud detection system
    
    Uses statistical analysis and anomaly detection to identify
    potentially fraudulent PoPC proofs
    """
    
    def __init__(self, contamination: float = 0.01):
        """
        Initialize fraud detector
        
        Args:
            contamination: Expected fraction of outliers (0.0 to 0.5)
        """
        self.contamination = contamination
        self.model = IsolationForest(
            contamination=contamination,
            random_state=42
        )
        self.is_trained = False
        
    def extract_features(self, proof: ProofData) -> np.ndarray:
        """
        Extract features from proof data for ML analysis
        
        Args:
            proof: Proof data
            
        Returns:
            Feature vector
        """
        features = []
        
        # Sample coverage
        expected_samples = 1000  # From PoPC config
        actual_samples = len(proof.samples)
        coverage = actual_samples / expected_samples
        features.append(coverage)
        
        # Merkle path consistency
        path_lengths = [len(paths) for paths in proof.merkle_paths.values()]
        if path_lengths:
            features.append(np.mean(path_lengths))
            features.append(np.std(path_lengths))
        else:
            features.append(0.0)
            features.append(0.0)
            
        # Sample data size distribution
        sample_sizes = [len(data) for data in proof.samples.values()]
        if sample_sizes:
            features.append(np.mean(sample_sizes))
            features.append(np.std(sample_sizes))
        else:
            features.append(0.0)
            features.append(0.0)
            
        # Timing analysis
        features.append(proof.timestamp)
        
        return np.array(features).reshape(1, -1)
    
    def train(self, historical_proofs: List[ProofData]):
        """
        Train the fraud detection model on historical data
        
        Args:
            historical_proofs: List of verified historical proofs
        """
        if len(historical_proofs) < 10:
            raise ValueError("Need at least 10 proofs for training")
            
        features = np.vstack([
            self.extract_features(proof) for proof in historical_proofs
        ])
        
        self.model.fit(features)
        self.is_trained = True
        
    def analyze(self, proof: ProofData) -> FraudAnalysis:
        """
        Analyze proof for potential fraud
        
        Args:
            proof: Proof data to analyze
            
        Returns:
            FraudAnalysis result
        """
        reasons = []
        
        # Basic sanity checks
        if len(proof.samples) == 0:
            reasons.append("No samples provided")
            
        if len(proof.merkle_paths) != len(proof.samples):
            reasons.append("Merkle path count mismatch")
            
        # ML-based anomaly detection
        anomaly_score = 0.0
        if self.is_trained:
            features = self.extract_features(proof)
            anomaly_score = -self.model.score_samples(features)[0]
            prediction = self.model.predict(features)[0]
            
            if prediction == -1:
                reasons.append(f"Anomalous pattern detected (score: {anomaly_score:.3f})")
        
        # Statistical checks
        sample_sizes = [len(data) for data in proof.samples.values()]
        if sample_sizes:
            mean_size = np.mean(sample_sizes)
            std_size = np.std(sample_sizes)
            
            # Unusual size distribution
            if std_size > mean_size * 0.5:
                reasons.append("High variance in sample sizes")
                
        # Path length consistency
        path_lengths = [len(paths) for paths in proof.merkle_paths.values()]
        if path_lengths and len(set(path_lengths)) > 3:
            reasons.append("Inconsistent Merkle path lengths")
        
        # Final determination
        is_suspicious = len(reasons) > 0
        confidence = min(1.0, len(reasons) * 0.3 + abs(anomaly_score) * 0.4)
        
        return FraudAnalysis(
            is_suspicious=is_suspicious,
            confidence=confidence,
            anomaly_score=anomaly_score,
            reasons=reasons
        )
    
    def batch_analyze(
        self,
        proofs: List[ProofData]
    ) -> List[FraudAnalysis]:
        """
        Analyze multiple proofs in batch
        
        Args:
            proofs: List of proofs
            
        Returns:
            List of fraud analyses
        """
        return [self.analyze(proof) for proof in proofs]
    
    def get_risk_score(self, worker_address: str, history: List[ProofData]) -> float:
        """
        Calculate worker risk score based on historical proofs
        
        Args:
            worker_address: Worker to analyze
            history: Historical proofs from this worker
            
        Returns:
            Risk score (0.0 to 1.0)
        """
        if not history:
            return 0.5  # Neutral score for new workers
            
        analyses = [self.analyze(proof) for proof in history]
        
        suspicious_count = sum(1 for a in analyses if a.is_suspicious)
        risk_ratio = suspicious_count / len(analyses)
        
        # Weight recent proofs more heavily
        recent_analyses = analyses[-10:]
        recent_suspicious = sum(1 for a in recent_analyses if a.is_suspicious)
        recent_risk = recent_suspicious / len(recent_analyses) if recent_analyses else 0.5
        
        # Combined score
        return risk_ratio * 0.4 + recent_risk * 0.6


def main():
    """Example usage"""
    import time
    
    # Create sample proof data
    proof = ProofData(
        job_id="job-123",
        worker_address="0x1234",
        samples={i: b"sample_data" for i in range(1000)},
        merkle_paths={i: [b"path1", b"path2"] for i in range(1000)},
        output_root=b"root_hash_32_bytes",
        timestamp=int(time.time())
    )
    
    # Initialize detector
    detector = FraudDetector()
    
    # Analyze proof
    result = detector.analyze(proof)
    
    print(f"Fraud Analysis:")
    print(f"  Suspicious: {result.is_suspicious}")
    print(f"  Confidence: {result.confidence:.2f}")
    print(f"  Anomaly Score: {result.anomaly_score:.3f}")
    print(f"  Reasons: {result.reasons if result.reasons else 'None'}")


if __name__ == "__main__":
    main()
