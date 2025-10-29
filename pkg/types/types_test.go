package types

import (
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestJobStatus_Constants(t *testing.T) {
	tests := []struct {
		name   string
		status JobStatus
	}{
		{"Pending", JobStatusPending},
		{"Assigned", JobStatusAssigned},
		{"Executing", JobStatusExecuting},
		{"Committed", JobStatusCommitted},
		{"Validating", JobStatusValidating},
		{"Completed", JobStatusCompleted},
		{"Failed", JobStatusFailed},
		{"Slashed", JobStatusSlashed},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.NotEmpty(t, string(tt.status))
		})
	}
}

func TestWorkerStatus_Constants(t *testing.T) {
	tests := []struct {
		name   string
		status WorkerStatus
	}{
		{"Active", WorkerStatusActive},
		{"Inactive", WorkerStatusInactive},
		{"Suspended", WorkerStatusSuspended},
		{"Slashed", WorkerStatusSlashed},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.NotEmpty(t, string(tt.status))
		})
	}
}

func TestValidatorStatus_Constants(t *testing.T) {
	tests := []struct {
		name   string
		status ValidatorStatus
	}{
		{"Active", ValidatorStatusActive},
		{"Inactive", ValidatorStatusInactive},
		{"Jailed", ValidatorStatusJailed},
		{"Slashed", ValidatorStatusSlashed},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.NotEmpty(t, string(tt.status))
		})
	}
}

func TestJob_Creation(t *testing.T) {
	job := Job{
		ID:     "job-123",
		Client: common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Specs: JobSpecs{
			GPU:       "NVIDIA RTX 4090",
			VRAM:      24,
			Framework: "PyTorch",
			Region:    "us-west",
			Tags:      []string{"ml", "inference"},
		},
		SLA: SLA{
			MaxLatency:     30 * time.Second,
			MaxRetries:     3,
			Timeout:        300 * time.Second,
			RequiredUptime: 0.99,
		},
		Price:       big.NewInt(1000000000000000000), // 1 AXX
		Status:      JobStatusPending,
		SubmittedAt: time.Now(),
	}

	assert.Equal(t, "job-123", job.ID)
	assert.NotEqual(t, common.Address{}, job.Client)
	assert.Equal(t, "NVIDIA RTX 4090", job.Specs.GPU)
	assert.Equal(t, 24, job.Specs.VRAM)
	assert.Equal(t, JobStatusPending, job.Status)
	assert.NotNil(t, job.Price)
}

func TestJobSpecs_GPURequirements(t *testing.T) {
	tests := []struct {
		name  string
		specs JobSpecs
		valid bool
	}{
		{
			name: "Valid GPU specs",
			specs: JobSpecs{
				GPU:  "NVIDIA RTX 4090",
				VRAM: 24,
			},
			valid: true,
		},
		{
			name: "Valid with region",
			specs: JobSpecs{
				GPU:    "NVIDIA RTX 4090",
				VRAM:   24,
				Region: "us-west",
			},
			valid: true,
		},
		{
			name: "No GPU required",
			specs: JobSpecs{
				Framework: "TensorFlow",
			},
			valid: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.NotNil(t, tt.specs)
		})
	}
}

func TestSLA_Validation(t *testing.T) {
	tests := []struct {
		name  string
		sla   SLA
		valid bool
	}{
		{
			name: "Valid SLA",
			sla: SLA{
				MaxLatency:     30 * time.Second,
				MaxRetries:     3,
				Timeout:        300 * time.Second,
				RequiredUptime: 0.99,
			},
			valid: true,
		},
		{
			name: "High uptime requirement",
			sla: SLA{
				MaxLatency:     10 * time.Second,
				MaxRetries:     5,
				Timeout:        600 * time.Second,
				RequiredUptime: 0.999,
			},
			valid: true,
		},
		{
			name: "Low uptime requirement",
			sla: SLA{
				MaxLatency:     60 * time.Second,
				MaxRetries:     1,
				Timeout:        120 * time.Second,
				RequiredUptime: 0.95,
			},
			valid: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.GreaterOrEqual(t, tt.sla.RequiredUptime, 0.0)
			assert.LessOrEqual(t, tt.sla.RequiredUptime, 1.0)
			assert.Greater(t, tt.sla.MaxLatency, time.Duration(0))
			assert.Greater(t, tt.sla.Timeout, time.Duration(0))
		})
	}
}

func TestWorker_Creation(t *testing.T) {
	// Create big.Int for stake: 10000 AXX = 10000 * 10^18 wei
	stake := new(big.Int)
	stake.SetString("10000000000000000000000", 10)

	worker := Worker{
		Address: common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"),
		Specs: WorkerSpecs{
			GPUs: []GPUSpec{
				{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 2},
			},
			CPUCores:  32,
			RAM:       128,
			Storage:   2000,
			Bandwidth: 1000,
			Region:    "us-west",
		},
		Performance: PerformanceStats{
			TotalJobs:      100,
			SuccessfulJobs: 95,
			FailedJobs:     5,
			PoPCPassRate:   0.95,
			DAReliability:  0.98,
			AvgLatency:     25.5,
			Uptime:         0.99,
			LastUpdated:    time.Now(),
		},
		Reputation:   0.95,
		Stake:        stake,
		Status:       WorkerStatusActive,
		RegisteredAt: time.Now().Add(-30 * 24 * time.Hour), // 30 days ago
		LastActiveAt: time.Now(),
		QuotaUsed:    0.05,
		IsNewcomer:   false,
	}

	assert.NotEqual(t, common.Address{}, worker.Address)
	assert.Equal(t, 2, worker.Specs.GPUs[0].Count)
	assert.Equal(t, 32, worker.Specs.CPUCores)
	assert.Equal(t, WorkerStatusActive, worker.Status)
	assert.Equal(t, 100, worker.Performance.TotalJobs)
	assert.GreaterOrEqual(t, worker.Reputation, 0.0)
	assert.LessOrEqual(t, worker.Reputation, 1.0)
}

func TestWorkerSpecs_MultipleGPUs(t *testing.T) {
	specs := WorkerSpecs{
		GPUs: []GPUSpec{
			{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 2},
			{Model: "NVIDIA A100", VRAM: 80, Count: 1},
		},
		CPUCores:  64,
		RAM:       256,
		Storage:   4000,
		Bandwidth: 10000,
		Region:    "us-east",
	}

	assert.Equal(t, 2, len(specs.GPUs))
	assert.Equal(t, 2, specs.GPUs[0].Count)
	assert.Equal(t, 1, specs.GPUs[1].Count)
	assert.Equal(t, "us-east", specs.Region)
}

func TestGPUSpec_Validation(t *testing.T) {
	tests := []struct {
		name  string
		gpu   GPUSpec
		valid bool
	}{
		{
			name: "Valid single GPU",
			gpu: GPUSpec{
				Model: "NVIDIA RTX 4090",
				VRAM:  24,
				Count: 1,
			},
			valid: true,
		},
		{
			name: "Valid multiple GPUs",
			gpu: GPUSpec{
				Model: "NVIDIA A100",
				VRAM:  80,
				Count: 8,
			},
			valid: true,
		},
		{
			name: "High VRAM",
			gpu: GPUSpec{
				Model: "NVIDIA H100",
				VRAM:  80,
				Count: 4,
			},
			valid: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.NotEmpty(t, tt.gpu.Model)
			assert.Greater(t, tt.gpu.VRAM, 0)
			assert.Greater(t, tt.gpu.Count, 0)
		})
	}
}

func TestPerformanceStats_Calculation(t *testing.T) {
	stats := PerformanceStats{
		TotalJobs:      100,
		SuccessfulJobs: 95,
		FailedJobs:     5,
		PoPCPassRate:   0.95,
		DAReliability:  0.98,
		AvgLatency:     30.0,
		Uptime:         0.99,
		LastUpdated:    time.Now(),
	}

	// Calculate success rate
	successRate := float64(stats.SuccessfulJobs) / float64(stats.TotalJobs)
	assert.Equal(t, 0.95, successRate)

	// Verify total jobs consistency
	assert.Equal(t, stats.TotalJobs, stats.SuccessfulJobs+stats.FailedJobs)

	// Verify rates are within valid range
	assert.GreaterOrEqual(t, stats.PoPCPassRate, 0.0)
	assert.LessOrEqual(t, stats.PoPCPassRate, 1.0)
	assert.GreaterOrEqual(t, stats.DAReliability, 0.0)
	assert.LessOrEqual(t, stats.DAReliability, 1.0)
	assert.GreaterOrEqual(t, stats.Uptime, 0.0)
	assert.LessOrEqual(t, stats.Uptime, 1.0)
}

func TestValidator_Creation(t *testing.T) {
	// Create big.Int for stake: 50000 AXX = 50000 * 10^18 wei
	stake := new(big.Int)
	stake.SetString("50000000000000000000000", 10)

	validator := Validator{
		Address:      common.HexToAddress("0x9999999999999999999999999999999999999999"),
		Stake:        stake,
		Commission:   0.05, // 5%
		Status:       ValidatorStatusActive,
		TotalVotes:   1000,
		CorrectVotes: 990,
		FalsePass:    2,
		RegisteredAt: time.Now().Add(-90 * 24 * time.Hour), // 90 days ago
	}

	assert.NotEqual(t, common.Address{}, validator.Address)
	assert.NotNil(t, validator.Stake)
	assert.GreaterOrEqual(t, validator.Commission, 0.0)
	assert.LessOrEqual(t, validator.Commission, 1.0)
	assert.Equal(t, ValidatorStatusActive, validator.Status)
	assert.Greater(t, validator.CorrectVotes, validator.FalsePass)

	// Calculate accuracy
	accuracy := float64(validator.CorrectVotes) / float64(validator.TotalVotes)
	assert.GreaterOrEqual(t, accuracy, 0.95) // Should have >95% accuracy
}

func TestBlock_Creation(t *testing.T) {
	block := Block{
		Number:     12345,
		Hash:       common.HexToHash("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"),
		ParentHash: common.HexToHash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"),
		Timestamp:  time.Now(),
		Proposer:   common.HexToAddress("0x1111111111111111111111111111111111111111"),
		Transactions: []Transaction{
			{
				Hash:     common.HexToHash("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
				From:     common.HexToAddress("0x2222222222222222222222222222222222222222"),
				To:       common.HexToAddress("0x3333333333333333333333333333333333333333"),
				Value:    big.NewInt(1000000000000000000),
				GasPrice: big.NewInt(20000000000),
				GasLimit: 21000,
				Nonce:    0,
			},
		},
		StateRoot:   common.HexToHash("0x1111111111111111111111111111111111111111111111111111111111111111"),
		ReceiptRoot: common.HexToHash("0x2222222222222222222222222222222222222222222222222222222222222222"),
		GasUsed:     21000,
		GasLimit:    30000000,
	}

	assert.Greater(t, block.Number, uint64(0))
	assert.NotEqual(t, common.Hash{}, block.Hash)
	assert.NotEqual(t, common.Hash{}, block.ParentHash)
	assert.NotEqual(t, common.Address{}, block.Proposer)
	assert.Equal(t, 1, len(block.Transactions))
	assert.LessOrEqual(t, block.GasUsed, block.GasLimit)
}

func TestTransaction_Creation(t *testing.T) {
	tx := Transaction{
		Hash:     common.HexToHash("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"),
		From:     common.HexToAddress("0x1111111111111111111111111111111111111111"),
		To:       common.HexToAddress("0x2222222222222222222222222222222222222222"),
		Value:    big.NewInt(5000000000000000000), // 5 AXX
		GasPrice: big.NewInt(50000000000),         // 50 gwei
		GasLimit: 100000,
		Nonce:    42,
		Data:     []byte("test data"),
	}

	assert.NotEqual(t, common.Hash{}, tx.Hash)
	assert.NotEqual(t, common.Address{}, tx.From)
	assert.NotEqual(t, common.Address{}, tx.To)
	assert.NotNil(t, tx.Value)
	assert.NotNil(t, tx.GasPrice)
	assert.Greater(t, tx.GasLimit, uint64(0))
}

func TestWorker_QuotaManagement(t *testing.T) {
	worker := Worker{
		Address:   common.HexToAddress("0x1234567890123456789012345678901234567890"),
		Status:    WorkerStatusActive,
		QuotaUsed: 0.0,
	}

	// Simulate job assignments
	jobQuota := 0.02 // 2% per job

	for i := 0; i < 5; i++ {
		worker.QuotaUsed += jobQuota
	}

	assert.Equal(t, 0.10, worker.QuotaUsed) // 5 jobs * 2% = 10%

	// Reset quota (new epoch)
	worker.QuotaUsed = 0.0
	assert.Equal(t, 0.0, worker.QuotaUsed)
}

func TestJob_StatusTransitions(t *testing.T) {
	job := Job{
		ID:          "transition-test",
		Status:      JobStatusPending,
		SubmittedAt: time.Now(),
	}

	// Valid status transitions
	validTransitions := []JobStatus{
		JobStatusPending,
		JobStatusAssigned,
		JobStatusExecuting,
		JobStatusCommitted,
		JobStatusValidating,
		JobStatusCompleted,
	}

	for i, status := range validTransitions {
		job.Status = status
		assert.Equal(t, validTransitions[i], job.Status)
	}
}

func TestWorker_ReputationRange(t *testing.T) {
	tests := []struct {
		name       string
		reputation float64
		valid      bool
	}{
		{"Perfect reputation", 1.0, true},
		{"High reputation", 0.95, true},
		{"Medium reputation", 0.75, true},
		{"Low reputation", 0.50, true},
		{"Zero reputation", 0.0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			worker := Worker{
				Reputation: tt.reputation,
			}
			assert.GreaterOrEqual(t, worker.Reputation, 0.0)
			assert.LessOrEqual(t, worker.Reputation, 1.0)
		})
	}
}

func TestValidator_VotingAccuracy(t *testing.T) {
	validator := Validator{
		TotalVotes:   1000,
		CorrectVotes: 980,
		FalsePass:    5,
	}

	accuracy := float64(validator.CorrectVotes) / float64(validator.TotalVotes)
	falsePassRate := float64(validator.FalsePass) / float64(validator.TotalVotes)

	assert.Equal(t, 0.98, accuracy)
	assert.Equal(t, 0.005, falsePassRate)

	// High accuracy validator should have low false pass rate
	assert.Less(t, falsePassRate, 0.01) // Less than 1%
}

func TestJob_CompletionTracking(t *testing.T) {
	submittedAt := time.Now()
	job := Job{
		ID:          "completion-test",
		SubmittedAt: submittedAt,
		Status:      JobStatusPending,
	}

	// Job gets completed
	completedAt := time.Now().Add(5 * time.Minute)
	job.CompletedAt = &completedAt
	job.Status = JobStatusCompleted

	require.NotNil(t, job.CompletedAt)
	duration := job.CompletedAt.Sub(job.SubmittedAt)

	assert.Greater(t, duration, time.Duration(0))
	assert.LessOrEqual(t, duration, 10*time.Minute)
}

func BenchmarkJob_Creation(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = Job{
			ID:     "bench-job",
			Client: common.HexToAddress("0x1234567890123456789012345678901234567890"),
			Specs: JobSpecs{
				GPU:  "NVIDIA RTX 4090",
				VRAM: 24,
			},
			Price:       big.NewInt(1000000000000000000),
			Status:      JobStatusPending,
			SubmittedAt: time.Now(),
		}
	}
}

func BenchmarkWorker_Creation(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = Worker{
			Address: common.HexToAddress("0x1234567890123456789012345678901234567890"),
			Specs: WorkerSpecs{
				GPUs: []GPUSpec{
					{Model: "NVIDIA RTX 4090", VRAM: 24, Count: 1},
				},
				CPUCores: 16,
				RAM:      64,
			},
			Status:       WorkerStatusActive,
			RegisteredAt: time.Now(),
		}
	}
}
