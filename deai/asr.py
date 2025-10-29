# deai/asr.py
# Auto-Selection Router (ASR) migrated from Go to Python

import math
import random
from typing import List, Dict, Optional

# --- Placeholder Types (to be replaced with proper SDK types) ---

class WorkerSpecs:
    def __init__(self, gpus: List[Dict], region: str):
        self.gpus = gpus
        self.region = region

class JobSpecs:
    def __init__(self, gpu: str, vram: int, region: str):
        self.gpu = gpu
        self.vram = vram
        self.region = region

class WorkerPerformance:
    def __init__(self, total_jobs: int, popc_pass_rate: float, da_reliability: float, uptime: float):
        self.total_jobs = total_jobs
        self.popc_pass_rate = popc_pass_rate
        self.da_reliability = da_reliability
        self.uptime = uptime

class Worker:
    def __init__(self, address: str, specs: WorkerSpecs, status: str, quota_used: float, is_newcomer: bool, performance: WorkerPerformance):
        self.address = address
        self.specs = specs
        self.status = status
        self.quota_used = quota_used
        self.is_newcomer = is_newcomer
        self.performance = performance

class Job:
    def __init__(self, job_id: str, specs: JobSpecs):
        self.job_id = job_id
        self.specs = specs

class WorkerScore:
    def __init__(self, worker: Worker, suitability: float, performance: float, fairness: float, total_score: float):
        self.worker = worker
        self.suitability = suitability
        self.performance = performance
        self.fairness = fairness
        self.total_score = total_score

# --- ASR Router Implementation ---

class ASRRouter:
    def __init__(self, config: Dict):
        """
        Initializes the Auto-Selection Router.
        'config' is a dict with keys like:
        'top_k', 'max_quota', 'exploration_rate', 'newcomer_boost', 'anti_collusion_enabled'
        """
        self.config = config
        self.workers: Dict[str, Worker] = {}

    def register_worker(self, worker: Worker):
        self.workers[worker.address] = worker

    def remove_worker(self, address: str):
        if address in self.workers:
            del self.workers[address]

    def select_worker(self, job: Job, vrf_seed: bytes) -> Optional[Worker]:
        """Selects the best worker for a given job using the ASR algorithm."""
        # 1. Filter eligible workers
        eligible_workers = self._filter_eligible_workers(job)
        if not eligible_workers:
            print("No eligible workers found.")
            return None

        # 2. Score all eligible workers
        scored_workers = [self._calculate_worker_score(worker, job) for worker in eligible_workers]

        # 3. Sort by total score (descending)
        scored_workers.sort(key=lambda s: s.total_score, reverse=True)

        # 4. Select top K candidates
        top_k = self.config.get('top_k', 5)
        candidates = scored_workers[:top_k]

        # 5. ε-greedy exploration: sometimes select a newcomer
        if random.random() < self.config.get('exploration_rate', 0.05):
            newcomers = [c for c in candidates if c.worker.is_newcomer]
            if newcomers:
                print("Exploring newcomer...")
                return random.choice(newcomers).worker

        # 6. VRF-weighted selection from top K
        selected_score = self._vrf_weighted_selection(candidates, vrf_seed)
        if not selected_score:
            return None

        # Update quota (this is a simplified model)
        selected_worker = selected_score.worker
        selected_worker.quota_used += 1.0 / len(self.workers)
        
        return selected_worker

    def _filter_eligible_workers(self, job: Job) -> List[Worker]:
        eligible = []
        for worker in self.workers.values():
            if self._is_eligible(worker, job):
                eligible.append(worker)
        return eligible

    def _is_eligible(self, worker: Worker, job: Job) -> bool:
        # Check status
        if worker.status != "active":
            return False
        # Check quota
        if worker.quota_used >= self.config.get('max_quota', 1.0):
            return False
        # Check hardware
        if not self._meets_hardware_requirements(worker.specs, job.specs):
            return False
        # Check region
        if job.specs.region and worker.specs.region != job.specs.region:
            return False
        return True

    def _meets_hardware_requirements(self, worker_specs: WorkerSpecs, job_specs: JobSpecs) -> bool:
        if job_specs.gpu:
            has_gpu = any(
                gpu.get('model') == job_specs.gpu and gpu.get('vram', 0) >= job_specs.vram
                for gpu in worker_specs.gpus
            )
            if not has_gpu:
                return False
        return True

    def _calculate_worker_score(self, worker: Worker, job: Job) -> WorkerScore:
        suitability = self._calculate_suitability(worker, job)
        performance = self._calculate_performance(worker)
        fairness = self._calculate_fairness(worker)
        
        total_score = suitability * performance * fairness
        
        return WorkerScore(worker, suitability, performance, fairness, total_score)

    def _calculate_suitability(self, worker: Worker, job: Job) -> float:
        suitability = 1.0
        # Exact GPU match bonus
        if job.specs.gpu and any(gpu.get('model') == job.specs.gpu for gpu in worker.specs.gpus):
            suitability *= 1.2
        # Region match bonus
        if job.specs.region and worker.specs.region == job.specs.region:
            suitability *= 1.1
        return min(suitability, 2.0)

    def _calculate_performance(self, worker: Worker) -> float:
        if worker.performance.total_jobs == 0:
            return 0.5  # Neutral score for new workers
        
        # Weighted average of performance metrics
        perf = worker.performance
        performance = (
            perf.popc_pass_rate * 0.4 +
            perf.da_reliability * 0.3 +
            perf.uptime * 0.3
        )
        return performance

    def _calculate_fairness(self, worker: Worker) -> float:
        fairness = 1.0
        # Quota penalty
        quota_ratio = worker.quota_used / self.config.get('max_quota', 1.0)
        if quota_ratio > 0.8:
            fairness *= (1.0 - quota_ratio)
        # Newcomer boost
        if worker.is_newcomer:
            fairness *= (1.0 + self.config.get('newcomer_boost', 0.2))
        return fairness

    def _vrf_weighted_selection(self, candidates: List[WorkerScore], vrf_seed: bytes) -> Optional[WorkerScore]:
        if not candidates:
            return None

        total_weight = sum(c.total_score for c in candidates)
        if total_weight == 0:
            return random.choice(candidates)

        # Generate a deterministic random value from the VRF seed
        seed_value = int.from_bytes(vrf_seed[:8], 'big') / (2**64 - 1)
        threshold = seed_value * total_weight

        cumulative = 0.0
        for candidate in candidates:
            cumulative += candidate.total_score
            if cumulative >= threshold:
                return candidate

        return candidates[0] # Fallback

    def reset_epoch_quotas(self):
        for worker in self.workers.values():
            worker.quota_used = 0.0
