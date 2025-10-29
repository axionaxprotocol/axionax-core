// Package asr implements the Auto-Selection Router for worker assignment
package asr

import (
	"math"
	"math/rand"
	"sort"
	"time"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/axionaxprotocol/axionax-core/pkg/types"
	"github.com/ethereum/go-ethereum/common"
)

// Router handles automatic worker selection based on ASR algorithm
type Router struct {
	config  *config.ASRConfig
	workers map[common.Address]*types.Worker
	rng     *rand.Rand
}

// NewRouter creates a new ASR router
func NewRouter(cfg *config.ASRConfig) *Router {
	return &Router{
		config:  cfg,
		workers: make(map[common.Address]*types.Worker),
		rng:     rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

// RegisterWorker registers a new worker in the router
func (r *Router) RegisterWorker(worker *types.Worker) {
	r.workers[worker.Address] = worker
}

// RemoveWorker removes a worker from the router
func (r *Router) RemoveWorker(address common.Address) {
	delete(r.workers, address)
}

// WorkerScore represents a worker's score for selection
type WorkerScore struct {
	Worker      *types.Worker
	Suitability float64
	Performance float64
	Fairness    float64
	TotalScore  float64
}

// SelectWorker selects the best worker for a given job using ASR algorithm
func (r *Router) SelectWorker(job *types.Job, vrfSeed common.Hash) (*types.Worker, error) {
	// Filter eligible workers
	eligible := r.filterEligibleWorkers(job)
	if len(eligible) == 0 {
		return nil, ErrNoEligibleWorkers
	}

	// Score all eligible workers
	scored := make([]WorkerScore, 0, len(eligible))
	for _, worker := range eligible {
		score := r.calculateWorkerScore(worker, job)
		scored = append(scored, score)
	}

	// Sort by total score (descending)
	sort.Slice(scored, func(i, j int) bool {
		return scored[i].TotalScore > scored[j].TotalScore
	})

	// Select top K candidates
	topK := r.config.TopK
	if len(scored) < topK {
		topK = len(scored)
	}
	candidates := scored[:topK]

	// ε-greedy exploration: sometimes select a newcomer
	if r.shouldExplore() {
		newcomers := r.filterNewcomers(candidates)
		if len(newcomers) > 0 {
			return newcomers[r.rng.Intn(len(newcomers))].Worker, nil
		}
	}

	// VRF-weighted selection from top K
	selected := r.vrfWeightedSelection(candidates, vrfSeed)

	// Update quota
	selected.Worker.QuotaUsed += 1.0 / float64(len(r.workers))

	return selected.Worker, nil
}

// filterEligibleWorkers filters workers that meet job requirements
func (r *Router) filterEligibleWorkers(job *types.Job) []*types.Worker {
	eligible := make([]*types.Worker, 0)

	for _, worker := range r.workers {
		if !r.isEligible(worker, job) {
			continue
		}
		eligible = append(eligible, worker)
	}

	return eligible
}

// isEligible checks if a worker is eligible for a job
func (r *Router) isEligible(worker *types.Worker, job *types.Job) bool {
	// Check status
	if worker.Status != types.WorkerStatusActive {
		return false
	}

	// Check quota
	if worker.QuotaUsed >= r.config.MaxQuota {
		return false
	}

	// Check hardware requirements
	if !r.meetsHardwareRequirements(worker.Specs, job.Specs) {
		return false
	}

	// Check region if specified
	if job.Specs.Region != "" && worker.Specs.Region != job.Specs.Region {
		return false
	}

	return true
}

// meetsHardwareRequirements checks if worker specs meet job requirements
func (r *Router) meetsHardwareRequirements(workerSpecs types.WorkerSpecs, jobSpecs types.JobSpecs) bool {
	// Check GPU requirements
	if jobSpecs.GPU != "" {
		hasGPU := false
		for _, gpu := range workerSpecs.GPUs {
			if gpu.Model == jobSpecs.GPU && gpu.VRAM >= jobSpecs.VRAM {
				hasGPU = true
				break
			}
		}
		if !hasGPU {
			return false
		}
	}

	return true
}

// calculateWorkerScore computes a composite score for worker selection
func (r *Router) calculateWorkerScore(worker *types.Worker, job *types.Job) WorkerScore {
	score := WorkerScore{
		Worker: worker,
	}

	// Suitability: how well the worker matches job requirements
	score.Suitability = r.calculateSuitability(worker, job)

	// Performance: historical reliability (EWMA over performance window)
	score.Performance = r.calculatePerformance(worker)

	// Fairness: anti-collusion and newcomer boost
	score.Fairness = r.calculateFairness(worker)

	// Total score: weighted combination
	score.TotalScore = score.Suitability * score.Performance * score.Fairness

	return score
}

// calculateSuitability calculates how well worker matches job requirements
func (r *Router) calculateSuitability(worker *types.Worker, job *types.Job) float64 {
	suitability := 1.0

	// Exact match bonus for GPU
	if job.Specs.GPU != "" {
		for _, gpu := range worker.Specs.GPUs {
			if gpu.Model == job.Specs.GPU {
				suitability *= 1.2
				break
			}
		}
	}

	// Region match bonus
	if job.Specs.Region != "" && worker.Specs.Region == job.Specs.Region {
		suitability *= 1.1
	}

	return math.Min(suitability, 2.0) // Cap at 2x
}

// calculatePerformance calculates performance score using EWMA
func (r *Router) calculatePerformance(worker *types.Worker) float64 {
	if worker.Performance.TotalJobs == 0 {
		return 0.5 // Neutral score for new workers
	}

	// Combine multiple performance metrics
	poPCScore := worker.Performance.PoPCPassRate
	daScore := worker.Performance.DAReliability
	uptimeScore := worker.Performance.Uptime

	// Weighted average
	performance := (poPCScore*0.4 + daScore*0.3 + uptimeScore*0.3)

	return performance
}

// calculateFairness applies fairness adjustments and anti-collusion
func (r *Router) calculateFairness(worker *types.Worker) float64 {
	fairness := 1.0

	// Quota penalty: reduce score if approaching quota limit
	quotaUsageRatio := worker.QuotaUsed / r.config.MaxQuota
	if quotaUsageRatio > 0.8 {
		fairness *= (1.0 - quotaUsageRatio)
	}

	// Newcomer boost
	if worker.IsNewcomer {
		fairness *= (1.0 + r.config.NewcomerBoost)
	}

	// Anti-collusion: check for org/ASN concentration
	if r.config.AntiCollusionEnabled {
		// This would require checking all selected workers in the epoch
		// Simplified version here
		fairness *= 1.0
	}

	return fairness
}

// shouldExplore determines if we should explore (select newcomer) based on ε-greedy
func (r *Router) shouldExplore() bool {
	return r.rng.Float64() < r.config.ExplorationRate
}

// filterNewcomers filters newcomer workers from candidates
func (r *Router) filterNewcomers(candidates []WorkerScore) []WorkerScore {
	newcomers := make([]WorkerScore, 0)
	for _, c := range candidates {
		if c.Worker.IsNewcomer {
			newcomers = append(newcomers, c)
		}
	}
	return newcomers
}

// vrfWeightedSelection performs VRF-weighted selection from candidates
func (r *Router) vrfWeightedSelection(candidates []WorkerScore, vrfSeed common.Hash) WorkerScore {
	if len(candidates) == 0 {
		return WorkerScore{}
	}

	// Use VRF seed to generate deterministic weighted selection
	totalWeight := 0.0
	for _, c := range candidates {
		totalWeight += c.TotalScore
	}

	// Generate deterministic random value from VRF seed
	seedValue := float64(vrfSeed.Big().Uint64()) / float64(^uint64(0))
	threshold := seedValue * totalWeight

	// Select based on weighted threshold
	cumulative := 0.0
	for _, c := range candidates {
		cumulative += c.TotalScore
		if cumulative >= threshold {
			return c
		}
	}

	// Fallback to first candidate
	return candidates[0]
}

// ResetEpochQuotas resets all worker quotas at the start of a new epoch
func (r *Router) ResetEpochQuotas() {
	for _, worker := range r.workers {
		worker.QuotaUsed = 0.0
	}
}

var (
	ErrNoEligibleWorkers = &ASRError{message: "no eligible workers found"}
)

// ASRError represents an ASR-specific error
type ASRError struct {
	message string
}

func (e *ASRError) Error() string {
	return e.message
}
