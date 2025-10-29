"""
Axionax DeAI - Auto Selection Router (ASR)

ML-based worker selection system for optimal job assignment
"""

import numpy as np
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass


@dataclass
class WorkerSpecs:
    """Worker hardware specifications"""
    address: str
    gpu_model: str
    vram: int
    cpu_cores: int
    ram: int
    region: str


@dataclass
class JobSpecs:
    """Job requirements"""
    gpu_model: str
    vram: int
    region: Optional[str] = None


@dataclass
class WorkerScore:
    """Worker selection score"""
    worker: WorkerSpecs
    suitability: float
    performance: float
    fairness: float
    total_score: float


class AutoSelectionRouter:
    """
    Auto Selection Router for worker assignment

    Implements the ASR algorithm from Axionax whitepaper:
    - Suitability scoring based on hardware match
    - Performance history analysis
    - Fairness via quota management
    - VRF-weighted selection from top-K
    """

    def __init__(
        self,
        top_k: int = 64,
        max_quota: float = 0.15,
        exploration_rate: float = 0.05,
        newcomer_boost: float = 0.1
    ):
        self.top_k = top_k
        self.max_quota = max_quota
        self.exploration_rate = exploration_rate
        self.newcomer_boost = newcomer_boost

    def calculate_suitability(
        self,
        worker: WorkerSpecs,
        job: JobSpecs
    ) -> float:
        """
        Calculate hardware suitability score

        Args:
            worker: Worker specifications
            job: Job requirements

        Returns:
            Suitability score (0.0 to 2.0+)
        """
        score = 1.0
        
        # GPU match bonus
        if job.gpu_model and worker.gpu_model == job.gpu_model:
            score *= 1.2

        # VRAM adequacy
        if worker.vram >= job.vram:
            score *= 1.1

        # Region preference
        if job.region and worker.region == job.region:
            score *= 1.1

        return score

    def calculate_performance(
        self,
        worker_stats: Dict[str, float]
    ) -> float:
        """
        Calculate performance score from historical data

        Args:
            worker_stats: Dictionary with performance metrics

        Returns:
            Performance score (0.0 to 1.0)
        """
        popc_rate = worker_stats.get('popc_pass_rate', 0.5)
        da_reliability = worker_stats.get('da_reliability', 0.5)
        uptime = worker_stats.get('uptime', 0.5)
        
        # Weighted average
        weights = [0.4, 0.3, 0.3]
        scores = [popc_rate, da_reliability, uptime]
        
        return sum(w * s for w, s in zip(weights, scores))

    def calculate_fairness(
        self,
        quota_used: float,
        is_newcomer: bool = False
    ) -> float:
        """
        Calculate fairness score based on quota usage
        
        Args:
            quota_used: Current epoch quota usage (0.0 to 1.0)
            is_newcomer: Whether worker is new to the network

        Returns:
            Fairness score (0.0 to 1.2)
        """
        # Penalize high quota usage
        utilization_ratio = quota_used / self.max_quota

        if utilization_ratio < 0.5:
            score = 1.0
        elif utilization_ratio < 0.8:
            score = 1.0 - (utilization_ratio - 0.5) * 0.5
        else:
            score = 0.1

        # Newcomer boost
        if is_newcomer:
            score = min(1.2, score + self.newcomer_boost)

        return score

    def score_workers(
        self,
        workers: List[WorkerSpecs],
        job: JobSpecs,
        worker_stats: Dict[str, Dict[str, float]],
        worker_quotas: Dict[str, float]
    ) -> List[WorkerScore]:
        """
        Score all eligible workers for a job

        Args:
            workers: List of worker specifications
            job: Job requirements
            worker_stats: Performance statistics per worker
            worker_quotas: Current quota usage per worker

        Returns:
            List of WorkerScore objects
        """
        scores = []

        for worker in workers:
            # Skip if quota exceeded
            if worker_quotas.get(worker.address, 0) >= self.max_quota:
                continue

            suitability = self.calculate_suitability(worker, job)
            performance = self.calculate_performance(
                worker_stats.get(worker.address, {})
            )
            fairness = self.calculate_fairness(
                worker_quotas.get(worker.address, 0),
                is_newcomer=worker_stats.get(worker.address, {}).get('is_newcomer', False)
            )

            total = suitability * performance * fairness

            scores.append(WorkerScore(
                worker=worker,
                suitability=suitability,
                performance=performance,
                fairness=fairness,
                total_score=total
            ))

        return sorted(scores, key=lambda x: x.total_score, reverse=True)

    def select_worker(
        self,
        workers: List[WorkerSpecs],
        job: JobSpecs,
        worker_stats: Dict[str, Dict[str, float]],
        worker_quotas: Dict[str, float],
        vrf_seed: bytes
    ) -> Optional[WorkerSpecs]:
        """
        Select optimal worker for job using ASR algorithm

        Args:
            workers: Available workers
            job: Job specifications
            worker_stats: Performance history
            worker_quotas: Quota usage
            vrf_seed: VRF seed for randomness

        Returns:
            Selected worker or None
        """
        # Score all workers
        scored = self.score_workers(workers, job, worker_stats, worker_quotas)

        if not scored:
            return None

        # Exploration: sometimes pick random worker
        if np.random.random() < self.exploration_rate:
            return np.random.choice([s.worker for s in scored])

        # Select from top-K using VRF-weighted selection
        top_candidates = scored[:min(self.top_k, len(scored))]

        # Calculate probabilities based on scores
        scores_array = np.array([c.total_score for c in top_candidates])
        probabilities = scores_array / scores_array.sum()

        # VRF-based selection (using seed as randomness source)
        np.random.seed(int.from_bytes(vrf_seed[:4], 'big'))
        selected_idx = np.random.choice(len(top_candidates), p=probabilities)

        return top_candidates[selected_idx].worker


def main():
    """Example usage"""
    # Create sample workers
    workers = [
        WorkerSpecs(
            address="0x1111",
            gpu_model="NVIDIA RTX 4090",
            vram=24,
            cpu_cores=16,
            ram=64,
            region="us-west"
        ),
        WorkerSpecs(
            address="0x2222",
            gpu_model="NVIDIA A100",
            vram=80,
            cpu_cores=32,
            ram=128,
            region="us-east"
        ),
    ]

    # Job requirements
    job = JobSpecs(
        gpu_model="NVIDIA RTX 4090",
        vram=24,
        region="us-west"
    )

    # Mock stats and quotas
    worker_stats = {
        "0x1111": {"popc_pass_rate": 0.95, "da_reliability": 0.98, "uptime": 0.99},
        "0x2222": {"popc_pass_rate": 0.97, "da_reliability": 0.96, "uptime": 0.98},
    }

    worker_quotas = {
        "0x1111": 0.05,
        "0x2222": 0.08,
    }

    # Initialize ASR and select worker
    asr = AutoSelectionRouter()
    selected = asr.select_worker(workers, job, worker_stats, worker_quotas, b"vrf_seed_12345678")

    if selected:
        print(f"Selected worker: {selected.address}")
        print(f"GPU: {selected.gpu_model}, VRAM: {selected.vram}GB")
        print(f"Region: {selected.region}")


if __name__ == "__main__":
    main()
