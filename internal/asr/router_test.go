package asr

import (
	"math/big"
	"testing"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/axionaxprotocol/axionax-core/pkg/types"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewRouter(t *testing.T) {
	cfg := &config.ASRConfig{
		TopK:                 64,
		MaxQuota:             0.15,
		ExplorationRate:      0.05,
		NewcomerBoost:        0.1,
		PerformanceWindow:    30,
		AntiCollusionEnabled: true,
	}

	router := NewRouter(cfg)

	assert.NotNil(t, router)
	assert.Equal(t, cfg, router.config)
	assert.NotNil(t, router.workers)
	assert.NotNil(t, router.rng)
}

func TestRegisterWorker(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	worker := &types.Worker{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Status:  types.WorkerStatusActive,
		Specs: types.WorkerSpecs{
			CPUCores: 16,
			RAM:      64,
		},
	}

	router.RegisterWorker(worker)

	assert.Equal(t, 1, len(router.workers))
	assert.Equal(t, worker, router.workers[worker.Address])
}

func TestRemoveWorker(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	address := common.HexToAddress("0x1234567890123456789012345678901234567890")
	worker := &types.Worker{
		Address: address,
		Status:  types.WorkerStatusActive,
	}

	router.RegisterWorker(worker)
	assert.Equal(t, 1, len(router.workers))

	router.RemoveWorker(address)
	assert.Equal(t, 0, len(router.workers))
}

func TestFilterEligibleWorkers(t *testing.T) {
	cfg := &config.ASRConfig{
		MaxQuota: 0.15,
	}
	router := NewRouter(cfg)

	// Register various workers
	activeWorker := &types.Worker{
		Address:   common.HexToAddress("0x1111111111111111111111111111111111111111"),
		Status:    types.WorkerStatusActive,
		QuotaUsed: 0.05,
		Specs: types.WorkerSpecs{
			GPUs: []types.GPUSpec{
				{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
			},
			Region: "us-west",
		},
	}

	inactiveWorker := &types.Worker{
		Address:   common.HexToAddress("0x2222222222222222222222222222222222222222"),
		Status:    types.WorkerStatusInactive,
		QuotaUsed: 0.05,
		Specs: types.WorkerSpecs{
			GPUs: []types.GPUSpec{
				{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
			},
		},
	}

	quotaExceededWorker := &types.Worker{
		Address:   common.HexToAddress("0x3333333333333333333333333333333333333333"),
		Status:    types.WorkerStatusActive,
		QuotaUsed: 0.20, // Exceeds MaxQuota
		Specs: types.WorkerSpecs{
			GPUs: []types.GPUSpec{
				{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
			},
		},
	}

	wrongGPUWorker := &types.Worker{
		Address:   common.HexToAddress("0x4444444444444444444444444444444444444444"),
		Status:    types.WorkerStatusActive,
		QuotaUsed: 0.05,
		Specs: types.WorkerSpecs{
			GPUs: []types.GPUSpec{
				{Model: "NVIDIA GTX 1080", VRAM: 8, Count: 1},
			},
		},
	}

	router.RegisterWorker(activeWorker)
	router.RegisterWorker(inactiveWorker)
	router.RegisterWorker(quotaExceededWorker)
	router.RegisterWorker(wrongGPUWorker)

	job := &types.Job{
		ID: "test-job",
		Specs: types.JobSpecs{
			GPU:  "NVIDIA RTX 4090",
			VRAM: 24,
		},
	}

	eligible := router.filterEligibleWorkers(job)

	// Only activeWorker should be eligible
	assert.Equal(t, 1, len(eligible))
	assert.Equal(t, activeWorker.Address, eligible[0].Address)
}

func TestIsEligible(t *testing.T) {
	cfg := &config.ASRConfig{
		MaxQuota: 0.15,
	}
	router := NewRouter(cfg)

	tests := []struct {
		name     string
		worker   *types.Worker
		job      *types.Job
		eligible bool
	}{
		{
			name: "Eligible worker",
			worker: &types.Worker{
				Status:    types.WorkerStatusActive,
				QuotaUsed: 0.05,
				Specs: types.WorkerSpecs{
					GPUs: []types.GPUSpec{
						{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
					},
					Region: "us-west",
				},
			},
			job: &types.Job{
				Specs: types.JobSpecs{
					GPU:    "NVIDIA RTX 4090",
					VRAM:   24,
					Region: "us-west",
				},
			},
			eligible: true,
		},
		{
			name: "Inactive worker",
			worker: &types.Worker{
				Status:    types.WorkerStatusInactive,
				QuotaUsed: 0.05,
				Specs: types.WorkerSpecs{
					GPUs: []types.GPUSpec{
						{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
					},
				},
			},
			job: &types.Job{
				Specs: types.JobSpecs{
					GPU:  "NVIDIA RTX 4090",
					VRAM: 24,
				},
			},
			eligible: false,
		},
		{
			name: "Quota exceeded",
			worker: &types.Worker{
				Status:    types.WorkerStatusActive,
				QuotaUsed: 0.20,
				Specs: types.WorkerSpecs{
					GPUs: []types.GPUSpec{
						{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
					},
				},
			},
			job: &types.Job{
				Specs: types.JobSpecs{
					GPU:  "NVIDIA RTX 4090",
					VRAM: 24,
				},
			},
			eligible: false,
		},
		{
			name: "Wrong GPU model",
			worker: &types.Worker{
				Status:    types.WorkerStatusActive,
				QuotaUsed: 0.05,
				Specs: types.WorkerSpecs{
					GPUs: []types.GPUSpec{
						{Model: "NVIDIA GTX 1080", VRAM: 8, Count: 1},
					},
				},
			},
			job: &types.Job{
				Specs: types.JobSpecs{
					GPU:  "NVIDIA RTX 4090",
					VRAM: 24,
				},
			},
			eligible: false,
		},
		{
			name: "Wrong region",
			worker: &types.Worker{
				Status:    types.WorkerStatusActive,
				QuotaUsed: 0.05,
				Specs: types.WorkerSpecs{
					GPUs: []types.GPUSpec{
						{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
					},
					Region: "eu-central",
				},
			},
			job: &types.Job{
				Specs: types.JobSpecs{
					GPU:    "NVIDIA RTX 4090",
					VRAM:   24,
					Region: "us-west",
				},
			},
			eligible: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			eligible := router.isEligible(tt.worker, tt.job)
			assert.Equal(t, tt.eligible, eligible)
		})
	}
}

func TestCalculateWorkerScore(t *testing.T) {
	cfg := &config.ASRConfig{
		MaxQuota:             0.15,
		NewcomerBoost:        0.1,
		AntiCollusionEnabled: false,
	}
	router := NewRouter(cfg)

	worker := &types.Worker{
		Address:    common.HexToAddress("0x1234567890123456789012345678901234567890"),
		QuotaUsed:  0.05,
		IsNewcomer: false,
		Specs: types.WorkerSpecs{
			GPUs: []types.GPUSpec{
				{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
			},
			Region: "us-west",
		},
		Performance: types.PerformanceStats{
			TotalJobs:      100,
			SuccessfulJobs: 95,
			PoPCPassRate:   0.95,
			DAReliability:  0.98,
			Uptime:         0.99,
		},
	}

	job := &types.Job{
		Specs: types.JobSpecs{
			GPU:    "NVIDIA RTX 4090",
			VRAM:   24,
			Region: "us-west",
		},
	}

	score := router.calculateWorkerScore(worker, job)

	assert.NotNil(t, score)
	assert.Equal(t, worker, score.Worker)
	assert.Greater(t, score.Suitability, 0.0)
	assert.Greater(t, score.Performance, 0.0)
	assert.Greater(t, score.Fairness, 0.0)
	assert.Greater(t, score.TotalScore, 0.0)
}

func TestCalculateSuitability(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	tests := []struct {
		name          string
		workerSpecs   types.WorkerSpecs
		jobSpecs      types.JobSpecs
		minSuitability float64
	}{
		{
			name: "Exact GPU match with region match",
			workerSpecs: types.WorkerSpecs{
				GPUs: []types.GPUSpec{
					{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
				},
				Region: "us-west",
			},
			jobSpecs: types.JobSpecs{
				GPU:    "NVIDIA RTX 4090",
				VRAM:   24,
				Region: "us-west",
			},
			minSuitability: 1.3, // 1.0 * 1.2 (GPU match) * 1.1 (region match)
		},
		{
			name: "GPU match, no region specified",
			workerSpecs: types.WorkerSpecs{
				GPUs: []types.GPUSpec{
					{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
				},
			},
			jobSpecs: types.JobSpecs{
				GPU:  "NVIDIA RTX 4090",
				VRAM: 24,
			},
			minSuitability: 1.2, // 1.0 * 1.2 (GPU match)
		},
		{
			name: "No GPU required",
			workerSpecs: types.WorkerSpecs{
				Region: "us-west",
			},
			jobSpecs: types.JobSpecs{
				Region: "us-west",
			},
			minSuitability: 1.1, // 1.0 * 1.1 (region match)
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			worker := &types.Worker{
				Specs: tt.workerSpecs,
			}
			job := &types.Job{
				Specs: tt.jobSpecs,
			}

			suitability := router.calculateSuitability(worker, job)
			assert.GreaterOrEqual(t, suitability, tt.minSuitability)
		})
	}
}

func TestCalculatePerformance(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	tests := []struct {
		name        string
		performance types.PerformanceStats
		expected    float64
	}{
		{
			name: "New worker with no history",
			performance: types.PerformanceStats{
				TotalJobs: 0,
			},
			expected: 0.5, // Neutral score
		},
		{
			name: "Excellent performer",
			performance: types.PerformanceStats{
				TotalJobs:     100,
				PoPCPassRate:  0.99,
				DAReliability: 0.98,
				Uptime:        0.99,
			},
			expected: 0.98, // High performance
		},
		{
			name: "Average performer",
			performance: types.PerformanceStats{
				TotalJobs:     50,
				PoPCPassRate:  0.80,
				DAReliability: 0.75,
				Uptime:        0.85,
			},
			expected: 0.78, // Moderate performance
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			worker := &types.Worker{
				Performance: tt.performance,
			}

			perf := router.calculatePerformance(worker)
			assert.InDelta(t, tt.expected, perf, 0.05)
		})
	}
}

func TestCalculateFairness(t *testing.T) {
	cfg := &config.ASRConfig{
		MaxQuota:             0.15,
		NewcomerBoost:        0.1,
		AntiCollusionEnabled: false,
	}
	router := NewRouter(cfg)

	tests := []struct {
		name         string
		quotaUsed    float64
		isNewcomer   bool
		minFairness  float64
		maxFairness  float64
	}{
		{
			name:        "Normal worker, low quota usage",
			quotaUsed:   0.05,
			isNewcomer:  false,
			minFairness: 0.9,
			maxFairness: 1.1,
		},
		{
			name:        "Newcomer with low quota",
			quotaUsed:   0.05,
			isNewcomer:  true,
			minFairness: 1.0,
			maxFairness: 1.2, // Gets newcomer boost
		},
		{
			name:        "High quota usage (>80%)",
			quotaUsed:   0.13, // 86.7% of max quota
			isNewcomer:  false,
			minFairness: 0.0,
			maxFairness: 0.2, // Penalty for high quota
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			worker := &types.Worker{
				QuotaUsed:  tt.quotaUsed,
				IsNewcomer: tt.isNewcomer,
			}

			fairness := router.calculateFairness(worker)
			assert.GreaterOrEqual(t, fairness, tt.minFairness)
			assert.LessOrEqual(t, fairness, tt.maxFairness)
		})
	}
}

func TestSelectWorker(t *testing.T) {
	cfg := &config.ASRConfig{
		TopK:                 3,
		MaxQuota:             0.15,
		ExplorationRate:      0.0, // Disable exploration for deterministic test
		NewcomerBoost:        0.1,
		AntiCollusionEnabled: false,
	}
	router := NewRouter(cfg)

	// Register multiple workers with different scores
	for i := 1; i <= 5; i++ {
		worker := &types.Worker{
			Address:   common.BigToAddress(big.NewInt(int64(i))),
			Status:    types.WorkerStatusActive,
			QuotaUsed: 0.0,
			Specs: types.WorkerSpecs{
				GPUs: []types.GPUSpec{
					{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
				},
			},
			Performance: types.PerformanceStats{
				TotalJobs:     100,
				PoPCPassRate:  float64(90+i) / 100.0, // Varying performance
				DAReliability: 0.95,
				Uptime:        0.98,
			},
		}
		router.RegisterWorker(worker)
	}

	job := &types.Job{
		ID: "test-job",
		Specs: types.JobSpecs{
			GPU:  "NVIDIA RTX 4090",
			VRAM: 24,
		},
	}

	vrfSeed := common.HexToHash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")

	selected, err := router.SelectWorker(job, vrfSeed)

	require.NoError(t, err)
	assert.NotNil(t, selected)
	assert.Equal(t, types.WorkerStatusActive, selected.Status)
	assert.Greater(t, selected.QuotaUsed, 0.0) // Quota should be updated
}

func TestSelectWorker_NoEligibleWorkers(t *testing.T) {
	router := NewRouter(&config.ASRConfig{
		MaxQuota: 0.15,
	})

	// Register only inactive workers
	worker := &types.Worker{
		Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Status:  types.WorkerStatusInactive,
	}
	router.RegisterWorker(worker)

	job := &types.Job{
		ID: "test-job",
		Specs: types.JobSpecs{
			GPU:  "NVIDIA RTX 4090",
			VRAM: 24,
		},
	}

	vrfSeed := common.Hash{}

	selected, err := router.SelectWorker(job, vrfSeed)

	assert.Error(t, err)
	assert.Nil(t, selected)
	assert.Equal(t, ErrNoEligibleWorkers, err)
}

func TestResetEpochQuotas(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	// Register workers with quota usage
	for i := 1; i <= 3; i++ {
		worker := &types.Worker{
			Address:   common.BigToAddress(big.NewInt(int64(i))),
			Status:    types.WorkerStatusActive,
			QuotaUsed: float64(i) * 0.05,
		}
		router.RegisterWorker(worker)
	}

	// Verify quotas are set
	for _, worker := range router.workers {
		assert.Greater(t, worker.QuotaUsed, 0.0)
	}

	// Reset quotas
	router.ResetEpochQuotas()

	// Verify all quotas are reset to 0
	for _, worker := range router.workers {
		assert.Equal(t, 0.0, worker.QuotaUsed)
	}
}

func TestVRFWeightedSelection(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	candidates := []WorkerScore{
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x1111")},
			TotalScore: 1.0,
		},
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x2222")},
			TotalScore: 2.0,
		},
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x3333")},
			TotalScore: 3.0,
		},
	}

	vrfSeed := common.HexToHash("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")

	selected := router.vrfWeightedSelection(candidates, vrfSeed)

	assert.NotNil(t, selected.Worker)
	// Higher scores should have higher probability of selection
	// But we can't test exact selection due to VRF randomness
}

func TestFilterNewcomers(t *testing.T) {
	router := NewRouter(&config.ASRConfig{})

	candidates := []WorkerScore{
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x1111"), IsNewcomer: true},
			TotalScore: 1.0,
		},
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x2222"), IsNewcomer: false},
			TotalScore: 2.0,
		},
		{
			Worker:     &types.Worker{Address: common.HexToAddress("0x3333"), IsNewcomer: true},
			TotalScore: 3.0,
		},
	}

	newcomers := router.filterNewcomers(candidates)

	assert.Equal(t, 2, len(newcomers))
	for _, n := range newcomers {
		assert.True(t, n.Worker.IsNewcomer)
	}
}

func BenchmarkSelectWorker(b *testing.B) {
	cfg := &config.ASRConfig{
		TopK:                 64,
		MaxQuota:             0.15,
		ExplorationRate:      0.05,
		NewcomerBoost:        0.1,
		AntiCollusionEnabled: true,
	}
	router := NewRouter(cfg)

	// Register 100 workers
	for i := 1; i <= 100; i++ {
		worker := &types.Worker{
			Address:   common.BigToAddress(big.NewInt(int64(i))),
			Status:    types.WorkerStatusActive,
			QuotaUsed: 0.0,
			Specs: types.WorkerSpecs{
				GPUs: []types.GPUSpec{
					{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
				},
			},
			Performance: types.PerformanceStats{
				TotalJobs:     100,
				PoPCPassRate:  0.95,
				DAReliability: 0.95,
				Uptime:        0.98,
			},
		}
		router.RegisterWorker(worker)
	}

	job := &types.Job{
		ID: "bench-job",
		Specs: types.JobSpecs{
			GPU:  "NVIDIA RTX 4090",
			VRAM: 24,
		},
	}

	vrfSeed := common.HexToHash("0x1234567890abcdef")

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		router.SelectWorker(job, vrfSeed)
	}
}
