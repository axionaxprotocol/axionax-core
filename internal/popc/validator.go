// Package popc implements Proof-of-Probabilistic-Checking validation
package popc

import (
	"crypto/sha256"
	"encoding/binary"
	"fmt"
	"math"
	"math/rand"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/axionaxprotocol/axionax-core/pkg/types"
	"github.com/ethereum/go-ethereum/common"
)

// Validator handles PoPC validation logic
type Validator struct {
	config *config.PoPCConfig
}

// NewValidator creates a new PoPC validator
func NewValidator(cfg *config.PoPCConfig) *Validator {
	return &Validator{
		config: cfg,
	}
}

// Challenge represents a PoPC challenge set
type Challenge struct {
	JobID      string        `json:"job_id"`
	Samples    []int         `json:"samples"`    // Sample indices
	VRFSeed    common.Hash   `json:"vrf_seed"`
	BlockDelay int           `json:"block_delay"`
}

// Proof represents worker's proof for challenged samples
type Proof struct {
	JobID       string           `json:"job_id"`
	Samples     map[int][]byte   `json:"samples"`      // index -> data
	MerklePaths map[int][]common.Hash `json:"merkle_paths"` // index -> path
	OutputRoot  common.Hash      `json:"output_root"`
}

// ValidationResult represents the result of PoPC validation
type ValidationResult struct {
	JobID       string  `json:"job_id"`
	Passed      bool    `json:"passed"`
	SamplesVerified int `json:"samples_verified"`
	SamplesTotal    int `json:"samples_total"`
	Confidence  float64 `json:"confidence"`
	Errors      []string `json:"errors,omitempty"`
}

// GenerateChallenge creates a PoPC challenge using VRF
func (v *Validator) GenerateChallenge(jobID string, outputSize int, vrfSeed common.Hash) *Challenge {
	// Use VRF seed to generate deterministic random samples
	rng := rand.New(rand.NewSource(int64(binary.BigEndian.Uint64(vrfSeed[:8]))))
	
	sampleSize := v.config.SampleSize
	if sampleSize > outputSize {
		sampleSize = outputSize
	}

	// Generate unique sample indices using stratified sampling
	samples := make([]int, 0, sampleSize)
	if v.config.StratifiedSampling {
		samples = v.generateStratifiedSamples(rng, outputSize, sampleSize)
	} else {
		samples = v.generateRandomSamples(rng, outputSize, sampleSize)
	}

	return &Challenge{
		JobID:      jobID,
		Samples:    samples,
		VRFSeed:    vrfSeed,
		BlockDelay: 0,
	}
}

// generateStratifiedSamples generates samples using stratified sampling
func (v *Validator) generateStratifiedSamples(rng *rand.Rand, outputSize, sampleSize int) []int {
	samples := make([]int, 0, sampleSize)
	
	// Divide output into strata
	strataCount := int(math.Sqrt(float64(sampleSize)))
	strataSize := outputSize / strataCount
	samplesPerStrata := sampleSize / strataCount

	for i := 0; i < strataCount; i++ {
		strataStart := i * strataSize
		strataEnd := strataStart + strataSize
		if i == strataCount-1 {
			strataEnd = outputSize
		}

		// Sample within this stratum
		for j := 0; j < samplesPerStrata; j++ {
			sample := strataStart + rng.Intn(strataEnd-strataStart)
			samples = append(samples, sample)
		}
	}

	// Fill remaining samples randomly
	for len(samples) < sampleSize {
		sample := rng.Intn(outputSize)
		samples = append(samples, sample)
	}

	return samples
}

// generateRandomSamples generates random samples
func (v *Validator) generateRandomSamples(rng *rand.Rand, outputSize, sampleSize int) []int {
	samples := make([]int, sampleSize)
	for i := 0; i < sampleSize; i++ {
		samples[i] = rng.Intn(outputSize)
	}
	return samples
}

// VerifyProof verifies a worker's proof against a challenge
func (v *Validator) VerifyProof(challenge *Challenge, proof *Proof) *ValidationResult {
	result := &ValidationResult{
		JobID:        challenge.JobID,
		SamplesTotal: len(challenge.Samples),
		Errors:       make([]string, 0),
	}

	verified := 0
	for _, idx := range challenge.Samples {
		// Check if sample exists in proof
		sampleData, ok := proof.Samples[idx]
		if !ok {
			result.Errors = append(result.Errors, fmt.Sprintf("Missing sample at index %d", idx))
			continue
		}

		// Check if merkle path exists
		merklePath, ok := proof.MerklePaths[idx]
		if !ok {
			result.Errors = append(result.Errors, fmt.Sprintf("Missing Merkle path for index %d", idx))
			continue
		}

		// Verify Merkle proof
		if v.verifyMerkleProof(sampleData, merklePath, idx, proof.OutputRoot) {
			verified++
		} else {
			result.Errors = append(result.Errors, fmt.Sprintf("Invalid Merkle proof for index %d", idx))
		}
	}

	result.SamplesVerified = verified
	result.Confidence = v.calculateConfidence(verified, result.SamplesTotal)
	result.Passed = result.Confidence >= v.config.MinConfidence

	// Adaptive escalation: if confidence is borderline, require more samples
	if v.config.AdaptiveEscalation && !result.Passed && result.Confidence > 0.95 {
		result.Errors = append(result.Errors, "Confidence borderline - adaptive escalation recommended")
	}

	return result
}

// verifyMerkleProof verifies a Merkle proof for a single sample
func (v *Validator) verifyMerkleProof(data []byte, path []common.Hash, index int, root common.Hash) bool {
	// Compute leaf hash
	currentHash := sha256.Sum256(data)
	current := common.BytesToHash(currentHash[:])

	// Traverse the Merkle path
	for i, sibling := range path {
		h := sha256.New()
		
		// Determine if current should be left or right child
		if (index>>i)&1 == 0 {
			// Current is left child
			h.Write(current.Bytes())
			h.Write(sibling.Bytes())
		} else {
			// Current is right child
			h.Write(sibling.Bytes())
			h.Write(current.Bytes())
		}
		
		hash := h.Sum(nil)
		current = common.BytesToHash(hash)
	}

	return current == root
}

// calculateConfidence calculates detection probability
// P_detect = 1 - (1 - f)^s where f is fraud rate and s is sample size
func (v *Validator) calculateConfidence(verified, total int) float64 {
	if total == 0 {
		return 0.0
	}

	passRate := float64(verified) / float64(total)
	
	// Assuming worst case fraud rate estimation
	// If pass rate is high, confidence is high
	// This is a simplified model; real implementation would be more sophisticated
	
	return passRate
}

// EstimateFraudDetectionProbability estimates the probability of detecting fraud
// given a fraud rate f and sample size s
func EstimateFraudDetectionProbability(fraudRate float64, sampleSize int) float64 {
	// P_detect = 1 - (1 - f)^s
	return 1.0 - math.Pow(1.0-fraudRate, float64(sampleSize))
}
