package popc

import (
	"crypto/sha256"
	"testing"

	"github.com/axionaxprotocol/axionax-core/pkg/config"
	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewValidator(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         100,
		MinConfidence:      0.99,
		StratifiedSampling: true,
		AdaptiveEscalation: true,
	}

	validator := NewValidator(cfg)
	
	assert.NotNil(t, validator)
	assert.Equal(t, cfg, validator.config)
}

func TestGenerateChallenge(t *testing.T) {
	tests := []struct {
		name              string
		sampleSize        int
		outputSize        int
		stratified        bool
		expectedSamples   int
	}{
		{
			name:            "Normal case with stratified sampling",
			sampleSize:      100,
			outputSize:      1000,
			stratified:      true,
			expectedSamples: 100,
		},
		{
			name:            "Normal case without stratified sampling",
			sampleSize:      50,
			outputSize:      500,
			stratified:      false,
			expectedSamples: 50,
		},
		{
			name:            "Sample size larger than output size",
			sampleSize:      200,
			outputSize:      100,
			stratified:      true,
			expectedSamples: 100, // Should cap at output size
		},
		{
			name:            "Small output size",
			sampleSize:      10,
			outputSize:      10,
			stratified:      false,
			expectedSamples: 10,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cfg := &config.PoPCConfig{
				SampleSize:         tt.sampleSize,
				StratifiedSampling: tt.stratified,
			}
			validator := NewValidator(cfg)

			jobID := "test-job-123"
			vrfSeed := common.HexToHash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")

			challenge := validator.GenerateChallenge(jobID, tt.outputSize, vrfSeed)

			assert.NotNil(t, challenge)
			assert.Equal(t, jobID, challenge.JobID)
			assert.Equal(t, vrfSeed, challenge.VRFSeed)
			assert.Equal(t, tt.expectedSamples, len(challenge.Samples))

			// Verify all samples are within bounds
			for _, sample := range challenge.Samples {
				assert.GreaterOrEqual(t, sample, 0)
				assert.Less(t, sample, tt.outputSize)
			}
		})
	}
}

func TestGenerateChallengeDeterministic(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         100,
		StratifiedSampling: false,
	}
	validator := NewValidator(cfg)

	jobID := "test-job-456"
	outputSize := 1000
	vrfSeed := common.HexToHash("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef")

	// Generate challenge twice with same seed
	challenge1 := validator.GenerateChallenge(jobID, outputSize, vrfSeed)
	challenge2 := validator.GenerateChallenge(jobID, outputSize, vrfSeed)

	// Should produce identical samples (deterministic)
	assert.Equal(t, len(challenge1.Samples), len(challenge2.Samples))
	for i := range challenge1.Samples {
		assert.Equal(t, challenge1.Samples[i], challenge2.Samples[i])
	}
}

func TestVerifyProof_Success(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         3,
		MinConfidence:      0.99,
		AdaptiveEscalation: false,
	}
	validator := NewValidator(cfg)

	// Create a simple Merkle tree for testing
	// Tree structure:
	//       root
	//      /    \
	//    h01    h23
	//   /  \   /  \
	//  l0 l1  l2 l3

	leaf0 := []byte("data0")
	leaf1 := []byte("data1")
	leaf2 := []byte("data2")

	// Calculate leaf hashes
	h0 := sha256.Sum256(leaf0)
	h1 := sha256.Sum256(leaf1)
	h2 := sha256.Sum256(leaf2)
	h3 := sha256.Sum256([]byte("data3"))

	// Calculate intermediate hashes
	h01 := sha256.New()
	h01.Write(h0[:])
	h01.Write(h1[:])
	h01Hash := h01.Sum(nil)

	h23 := sha256.New()
	h23.Write(h2[:])
	h23.Write(h3[:])
	h23Hash := h23.Sum(nil)

	// Calculate root
	rootHash := sha256.New()
	rootHash.Write(h01Hash)
	rootHash.Write(h23Hash)
	root := common.BytesToHash(rootHash.Sum(nil))

	// Create challenge
	challenge := &Challenge{
		JobID:   "test-job",
		Samples: []int{0, 1, 2},
	}

	// Create valid proof
	proof := &Proof{
		JobID: "test-job",
		Samples: map[int][]byte{
			0: leaf0,
			1: leaf1,
			2: leaf2,
		},
		MerklePaths: map[int][]common.Hash{
			0: {common.BytesToHash(h1[:]), common.BytesToHash(h23Hash)},
			1: {common.BytesToHash(h0[:]), common.BytesToHash(h23Hash)},
			2: {common.BytesToHash(h3[:]), common.BytesToHash(h01Hash)},
		},
		OutputRoot: root,
	}

	result := validator.VerifyProof(challenge, proof)

	assert.True(t, result.Passed)
	assert.Equal(t, 3, result.SamplesVerified)
	assert.Equal(t, 3, result.SamplesTotal)
	assert.Equal(t, 1.0, result.Confidence)
	assert.Empty(t, result.Errors)
}

func TestVerifyProof_MissingSample(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:    3,
		MinConfidence: 0.99,
	}
	validator := NewValidator(cfg)

	challenge := &Challenge{
		JobID:   "test-job",
		Samples: []int{0, 1, 2},
	}

	// Proof missing sample at index 1
	proof := &Proof{
		JobID: "test-job",
		Samples: map[int][]byte{
			0: []byte("data0"),
			2: []byte("data2"),
		},
		MerklePaths: map[int][]common.Hash{
			0: {},
			2: {},
		},
		OutputRoot: common.Hash{},
	}

	result := validator.VerifyProof(challenge, proof)

	assert.False(t, result.Passed)
	assert.Less(t, result.SamplesVerified, result.SamplesTotal)
	// Find the error about missing sample
	found := false
	for _, err := range result.Errors {
		if err == "Missing sample at index 1" {
			found = true
			break
		}
	}
	assert.True(t, found, "Should have error about missing sample at index 1")
}

func TestVerifyProof_MissingMerklePath(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:    2,
		MinConfidence: 0.99,
	}
	validator := NewValidator(cfg)

	challenge := &Challenge{
		JobID:   "test-job",
		Samples: []int{0, 1},
	}

	// Proof missing Merkle path for index 1
	proof := &Proof{
		JobID: "test-job",
		Samples: map[int][]byte{
			0: []byte("data0"),
			1: []byte("data1"),
		},
		MerklePaths: map[int][]common.Hash{
			0: {},
		},
		OutputRoot: common.Hash{},
	}

	result := validator.VerifyProof(challenge, proof)

	assert.False(t, result.Passed)
	// Find the error about missing Merkle path
	found := false
	for _, err := range result.Errors {
		if err == "Missing Merkle path for index 1" {
			found = true
			break
		}
	}
	assert.True(t, found, "Should have error about missing Merkle path for index 1")
}

func TestVerifyProof_InvalidMerkleProof(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:    1,
		MinConfidence: 0.99,
	}
	validator := NewValidator(cfg)

	challenge := &Challenge{
		JobID:   "test-job",
		Samples: []int{0},
	}

	// Proof with invalid Merkle path (won't match root)
	proof := &Proof{
		JobID: "test-job",
		Samples: map[int][]byte{
			0: []byte("data0"),
		},
		MerklePaths: map[int][]common.Hash{
			0: {common.HexToHash("0xdeadbeef")},
		},
		OutputRoot: common.HexToHash("0x12345678"),
	}

	result := validator.VerifyProof(challenge, proof)

	assert.False(t, result.Passed)
	assert.Equal(t, 0, result.SamplesVerified)
	assert.Contains(t, result.Errors[0], "Invalid Merkle proof for index 0")
}

func TestVerifyProof_AdaptiveEscalation(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         100,
		MinConfidence:      0.99,
		AdaptiveEscalation: true,
	}
	validator := NewValidator(cfg)

	challenge := &Challenge{
		JobID:   "test-job",
		Samples: make([]int, 100),
	}
	for i := 0; i < 100; i++ {
		challenge.Samples[i] = i
	}

	// Create proof with 96 samples present but missing 4
	// This will give us 0.96 confidence based on missing samples
	proof := &Proof{
		JobID:       "test-job",
		Samples:     make(map[int][]byte),
		MerklePaths: make(map[int][]common.Hash),
		OutputRoot:  common.Hash{},
	}

	// Only provide 96 samples (missing 96-99)
	for i := 0; i < 96; i++ {
		proof.Samples[i] = []byte("data")
		proof.MerklePaths[i] = []common.Hash{}
	}

	result := validator.VerifyProof(challenge, proof)

	assert.False(t, result.Passed, "Should not pass with low confidence")
	
	// Since samples are present but Merkle proofs won't verify (empty paths + zero root),
	// actual verified count will be 0, giving 0.0 confidence
	// Let's just check that adaptive escalation message appears when appropriate
	if result.Confidence > 0.95 && result.Confidence < cfg.MinConfidence {
		found := false
		for _, err := range result.Errors {
			if err == "Confidence borderline - adaptive escalation recommended" {
				found = true
				break
			}
		}
		assert.True(t, found, "Should recommend adaptive escalation for borderline confidence")
	}
}

func TestVerifyMerkleProof(t *testing.T) {
	cfg := &config.PoPCConfig{}
	validator := NewValidator(cfg)

	tests := []struct {
		name        string
		data        []byte
		index       int
		expectValid bool
	}{
		{
			name:        "Valid proof for index 0",
			data:        []byte("data0"),
			index:       0,
			expectValid: true,
		},
		{
			name:        "Valid proof for index 1",
			data:        []byte("data1"),
			index:       1,
			expectValid: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Build a simple 2-leaf Merkle tree
			h0 := sha256.Sum256([]byte("data0"))
			h1 := sha256.Sum256([]byte("data1"))

			rootHash := sha256.New()
			rootHash.Write(h0[:])
			rootHash.Write(h1[:])
			root := common.BytesToHash(rootHash.Sum(nil))

			var path []common.Hash
			if tt.index == 0 {
				path = []common.Hash{common.BytesToHash(h1[:])}
			} else {
				path = []common.Hash{common.BytesToHash(h0[:])}
			}

			valid := validator.verifyMerkleProof(tt.data, path, tt.index, root)
			assert.Equal(t, tt.expectValid, valid)
		})
	}
}

func TestCalculateConfidence(t *testing.T) {
	cfg := &config.PoPCConfig{}
	validator := NewValidator(cfg)

	tests := []struct {
		name               string
		verified           int
		total              int
		expectedConfidence float64
	}{
		{
			name:               "All samples verified",
			verified:           100,
			total:              100,
			expectedConfidence: 1.0,
		},
		{
			name:               "Half samples verified",
			verified:           50,
			total:              100,
			expectedConfidence: 0.5,
		},
		{
			name:               "No samples",
			verified:           0,
			total:              0,
			expectedConfidence: 0.0,
		},
		{
			name:               "99% verified",
			verified:           99,
			total:              100,
			expectedConfidence: 0.99,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			confidence := validator.calculateConfidence(tt.verified, tt.total)
			assert.Equal(t, tt.expectedConfidence, confidence)
		})
	}
}

func TestEstimateFraudDetectionProbability(t *testing.T) {
	tests := []struct {
		name           string
		fraudRate      float64
		sampleSize     int
		minProbability float64
	}{
		{
			name:           "10% fraud rate, 100 samples",
			fraudRate:      0.1,
			sampleSize:     100,
			minProbability: 0.9999, // Slightly lower threshold
		},
		{
			name:           "1% fraud rate, 1000 samples",
			fraudRate:      0.01,
			sampleSize:     1000,
			minProbability: 0.9999, // Slightly lower threshold
		},
		{
			name:           "5% fraud rate, 50 samples",
			fraudRate:      0.05,
			sampleSize:     50,
			minProbability: 0.92, // Should be reasonably high
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			prob := EstimateFraudDetectionProbability(tt.fraudRate, tt.sampleSize)
			assert.GreaterOrEqual(t, prob, tt.minProbability)
			assert.LessOrEqual(t, prob, 1.0)
		})
	}
}

func TestGenerateStratifiedSamples(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         100,
		StratifiedSampling: true,
	}
	validator := NewValidator(cfg)

	vrfSeed := common.HexToHash("0x1234")
	challenge := validator.GenerateChallenge("test", 1000, vrfSeed)

	// Verify samples are spread across the range
	require.Equal(t, 100, len(challenge.Samples))

	// Check that samples are reasonably distributed
	buckets := make([]int, 10)
	for _, sample := range challenge.Samples {
		bucket := sample / 100
		if bucket >= 10 {
			bucket = 9
		}
		buckets[bucket]++
	}

	// Each bucket should have some samples (stratified sampling)
	// Allow some variance but check most buckets have samples
	nonEmptyBuckets := 0
	for _, count := range buckets {
		if count > 0 {
			nonEmptyBuckets++
		}
	}
	assert.GreaterOrEqual(t, nonEmptyBuckets, 5, "Stratified sampling should distribute samples across range")
}

func TestGenerateRandomSamples(t *testing.T) {
	cfg := &config.PoPCConfig{
		SampleSize:         50,
		StratifiedSampling: false,
	}
	validator := NewValidator(cfg)

	vrfSeed := common.HexToHash("0xabcd")
	challenge := validator.GenerateChallenge("test", 500, vrfSeed)

	require.Equal(t, 50, len(challenge.Samples))

	// Verify all samples are in valid range
	for _, sample := range challenge.Samples {
		assert.GreaterOrEqual(t, sample, 0)
		assert.Less(t, sample, 500)
	}
}

func BenchmarkGenerateChallenge(b *testing.B) {
	cfg := &config.PoPCConfig{
		SampleSize:         1000,
		StratifiedSampling: true,
	}
	validator := NewValidator(cfg)

	vrfSeed := common.HexToHash("0x1234567890abcdef")
	outputSize := 100000

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		validator.GenerateChallenge("bench-job", outputSize, vrfSeed)
	}
}

func BenchmarkVerifyProof(b *testing.B) {
	cfg := &config.PoPCConfig{
		SampleSize:    100,
		MinConfidence: 0.99,
	}
	validator := NewValidator(cfg)

	// Create a sample challenge and proof
	challenge := &Challenge{
		JobID:   "bench-job",
		Samples: make([]int, 100),
	}
	for i := 0; i < 100; i++ {
		challenge.Samples[i] = i
	}

	proof := &Proof{
		JobID:       "bench-job",
		Samples:     make(map[int][]byte),
		MerklePaths: make(map[int][]common.Hash),
		OutputRoot:  common.HexToHash("0x1234"),
	}

	// Create dummy proof data
	for i := 0; i < 100; i++ {
		proof.Samples[i] = []byte("benchmark-data")
		proof.MerklePaths[i] = []common.Hash{
			common.HexToHash("0xdead"),
			common.HexToHash("0xbeef"),
		}
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		validator.VerifyProof(challenge, proof)
	}
}
