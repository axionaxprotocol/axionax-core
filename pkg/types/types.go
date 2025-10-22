// Package types defines core data structures for the Axionax protocol
package types

import (
	"math/big"
	"time"

	"github.com/ethereum/go-ethereum/common"
)

// Job represents a compute job submitted to the Axionax network
type Job struct {
	ID          string         `json:"id"`
	Client      common.Address `json:"client"`
	Worker      common.Address `json:"worker,omitempty"`
	Specs       JobSpecs       `json:"specs"`
	SLA         SLA            `json:"sla"`
	Price       *big.Int       `json:"price"`
	Status      JobStatus      `json:"status"`
	SubmittedAt time.Time      `json:"submitted_at"`
	CompletedAt *time.Time     `json:"completed_at,omitempty"`
	OutputRoot  common.Hash    `json:"output_root,omitempty"`
}

// JobSpecs defines the requirements for a compute job
type JobSpecs struct {
	GPU        string   `json:"gpu"`
	VRAM       int      `json:"vram"`        // in GB
	Framework  string   `json:"framework"`
	Region     string   `json:"region"`
	Tags       []string `json:"tags"`
}

// SLA defines service level agreement for a job
type SLA struct {
	MaxLatency   time.Duration `json:"max_latency"`
	MaxRetries   int           `json:"max_retries"`
	Timeout      time.Duration `json:"timeout"`
	RequiredUptime float64     `json:"required_uptime"` // 0.0 to 1.0
}

// JobStatus represents the current status of a job
type JobStatus string

const (
	JobStatusPending    JobStatus = "pending"
	JobStatusAssigned   JobStatus = "assigned"
	JobStatusExecuting  JobStatus = "executing"
	JobStatusCommitted  JobStatus = "committed"
	JobStatusValidating JobStatus = "validating"
	JobStatusCompleted  JobStatus = "completed"
	JobStatusFailed     JobStatus = "failed"
	JobStatusSlashed    JobStatus = "slashed"
)

// Worker represents a compute provider in the network
type Worker struct {
	Address       common.Address    `json:"address"`
	Specs         WorkerSpecs       `json:"specs"`
	Performance   PerformanceStats  `json:"performance"`
	Reputation    float64           `json:"reputation"` // 0.0 to 1.0
	Stake         *big.Int          `json:"stake"`
	Status        WorkerStatus      `json:"status"`
	RegisteredAt  time.Time         `json:"registered_at"`
	LastActiveAt  time.Time         `json:"last_active_at"`
	QuotaUsed     float64           `json:"quota_used"`    // Current epoch usage
	IsNewcomer    bool              `json:"is_newcomer"`
}

// WorkerSpecs defines the hardware and software capabilities of a worker
type WorkerSpecs struct {
	GPUs      []GPUSpec `json:"gpus"`
	CPUCores  int       `json:"cpu_cores"`
	RAM       int       `json:"ram"`        // in GB
	Storage   int       `json:"storage"`    // in GB
	Bandwidth int       `json:"bandwidth"`  // in Mbps
	Region    string    `json:"region"`
	ASN       string    `json:"asn"`
	Organization string `json:"organization"`
}

// GPUSpec defines GPU specifications
type GPUSpec struct {
	Model string `json:"model"`
	VRAM  int    `json:"vram"` // in GB
	Count int    `json:"count"`
}

// PerformanceStats tracks worker's historical performance
type PerformanceStats struct {
	TotalJobs         int       `json:"total_jobs"`
	SuccessfulJobs    int       `json:"successful_jobs"`
	FailedJobs        int       `json:"failed_jobs"`
	PoPCPassRate      float64   `json:"popc_pass_rate"`
	DAReliability     float64   `json:"da_reliability"`
	AvgLatency        float64   `json:"avg_latency"` // in seconds
	Uptime            float64   `json:"uptime"`      // 0.0 to 1.0
	LastUpdated       time.Time `json:"last_updated"`
}

// WorkerStatus represents the current status of a worker
type WorkerStatus string

const (
	WorkerStatusActive      WorkerStatus = "active"
	WorkerStatusInactive    WorkerStatus = "inactive"
	WorkerStatusSuspended   WorkerStatus = "suspended"
	WorkerStatusSlashed     WorkerStatus = "slashed"
)

// Validator represents a network validator
type Validator struct {
	Address      common.Address `json:"address"`
	Stake        *big.Int       `json:"stake"`
	Commission   float64        `json:"commission"` // 0.0 to 1.0
	Status       ValidatorStatus `json:"status"`
	TotalVotes   int            `json:"total_votes"`
	CorrectVotes int            `json:"correct_votes"`
	FalsePass    int            `json:"false_pass"`
	RegisteredAt time.Time      `json:"registered_at"`
}

// ValidatorStatus represents the current status of a validator
type ValidatorStatus string

const (
	ValidatorStatusActive    ValidatorStatus = "active"
	ValidatorStatusInactive  ValidatorStatus = "inactive"
	ValidatorStatusJailed    ValidatorStatus = "jailed"
	ValidatorStatusSlashed   ValidatorStatus = "slashed"
)

// Block represents a block in the Axionax chain
type Block struct {
	Number       uint64         `json:"number"`
	Hash         common.Hash    `json:"hash"`
	ParentHash   common.Hash    `json:"parent_hash"`
	Timestamp    time.Time      `json:"timestamp"`
	Proposer     common.Address `json:"proposer"`
	Transactions []Transaction  `json:"transactions"`
	StateRoot    common.Hash    `json:"state_root"`
	ReceiptRoot  common.Hash    `json:"receipt_root"`
	GasUsed      uint64         `json:"gas_used"`
	GasLimit     uint64         `json:"gas_limit"`
}

// Transaction represents a transaction in the Axionax chain
type Transaction struct {
	Hash     common.Hash    `json:"hash"`
	From     common.Address `json:"from"`
	To       common.Address `json:"to"`
	Value    *big.Int       `json:"value"`
	GasPrice *big.Int       `json:"gas_price"`
	GasLimit uint64         `json:"gas_limit"`
	Nonce    uint64         `json:"nonce"`
	Data     []byte         `json:"data"`
}
