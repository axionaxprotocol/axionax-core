package config

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDefaultConfig(t *testing.T) {
	cfg := DefaultConfig()

	require.NotNil(t, cfg)

	// Test Node config
	assert.Equal(t, "axionax-node", cfg.Node.Name)
	assert.Equal(t, ".axionax", cfg.Node.DataDir)
	assert.Equal(t, "info", cfg.Node.LogLevel)
	assert.Equal(t, "full", cfg.Node.Mode)
	assert.Equal(t, uint64(31337), cfg.Node.ChainID)
	assert.Equal(t, "full", cfg.Node.SyncMode)

	// Test Network config
	assert.Equal(t, "0.0.0.0", cfg.Network.ListenAddr)
	assert.Equal(t, 30303, cfg.Network.P2PPort)
	assert.Equal(t, 50, cfg.Network.MaxPeers)
	assert.NotNil(t, cfg.Network.BootstrapNodes)
	assert.NotNil(t, cfg.Network.Seeds)

	// Test PoPC config
	assert.Equal(t, 1000, cfg.PoPC.SampleSize)
	assert.Equal(t, 0.025, cfg.PoPC.RedundancyRate)
	assert.Equal(t, 0.999, cfg.PoPC.MinConfidence)
	assert.True(t, cfg.PoPC.StratifiedSampling)
	assert.True(t, cfg.PoPC.AdaptiveEscalation)
	assert.Equal(t, 3600*time.Second, cfg.PoPC.FraudWindowTime)

	// Test ASR config
	assert.Equal(t, 64, cfg.ASR.TopK)
	assert.Equal(t, 0.125, cfg.ASR.MaxQuota)
	assert.Equal(t, 0.05, cfg.ASR.ExplorationRate)
	assert.Equal(t, 0.1, cfg.ASR.NewcomerBoost)
	assert.Equal(t, 30, cfg.ASR.PerformanceWindow)
	assert.True(t, cfg.ASR.AntiCollusionEnabled)

	// Test PPC config
	assert.Equal(t, 0.7, cfg.PPC.TargetUtilization)
	assert.Equal(t, 60.0, cfg.PPC.TargetQueueTime)
	assert.Equal(t, 0.1, cfg.PPC.Alpha)
	assert.Equal(t, 0.05, cfg.PPC.Beta)
	assert.Equal(t, 0.001, cfg.PPC.MinPrice)
	assert.Equal(t, 10.0, cfg.PPC.MaxPrice)
	assert.Equal(t, 300*time.Second, cfg.PPC.AdjustmentInterval)

	// Test DA config
	assert.Equal(t, "data/da", cfg.DA.StorageDir)
	assert.Equal(t, 1.5, cfg.DA.ErasureCodingRate)
	assert.Equal(t, 256, cfg.DA.ChunkSize)
	assert.Equal(t, 300*time.Second, cfg.DA.AvailabilityWindow)
	assert.Equal(t, 3, cfg.DA.ReplicationFactor)
	assert.True(t, cfg.DA.LiveAuditEnabled)

	// Test VRF config
	assert.Equal(t, 2, cfg.VRF.DelayBlocks)
	assert.True(t, cfg.VRF.UseDelayedVRF)

	// Test Consensus config
	assert.Equal(t, 5*time.Second, cfg.Consensus.BlockTime)
	assert.Equal(t, 100, cfg.Consensus.EpochLength)
	assert.Equal(t, "10000", cfg.Consensus.MinValidatorStake)
	assert.Equal(t, 100, cfg.Consensus.MaxValidators)
	assert.Equal(t, 0.1, cfg.Consensus.SlashingRate)
	assert.Equal(t, 500, cfg.Consensus.FalsePassPenalty)

	// Test API config
	assert.True(t, cfg.API.Enabled)
	assert.Equal(t, "127.0.0.1", cfg.API.ListenAddr)
	assert.Equal(t, 8545, cfg.API.RPCPort)
	assert.Equal(t, 8546, cfg.API.WSPort)
	assert.Equal(t, []string{"*"}, cfg.API.CORSOrigins)

	// Test Telemetry config
	assert.True(t, cfg.Telemetry.Enabled)
	assert.Equal(t, 9090, cfg.Telemetry.PrometheusPort)
	assert.Equal(t, "0.0.0.0:9090", cfg.Telemetry.MetricsAddr)
	assert.False(t, cfg.Telemetry.DeAIEnabled)
}

func TestLoadConfig_NoFile(t *testing.T) {
	// Test loading with non-existent file path
	cfg, err := LoadConfig("/non/existent/path/config.yaml")

	// Should return error for non-existent file with explicit path
	assert.Error(t, err)
	assert.Nil(t, cfg)
}

func TestLoadConfig_ValidYAML(t *testing.T) {
	// Create a temporary config file
	tmpDir := t.TempDir()
	configPath := filepath.Join(tmpDir, "config.yaml")

	configContent := `
node:
  name: test-node
  data_dir: /tmp/test-data
  log_level: debug
  mode: validator
  chain_id: 12345
  sync_mode: fast

network:
  listen_addr: 192.168.1.1
  p2p_port: 40000
  max_peers: 100

popc:
  sample_size: 2000
  redundancy_rate: 0.03
  min_confidence: 0.9999
  stratified_sampling: false
  adaptive_escalation: false
  fraud_window_time: 7200s

asr:
  top_k: 128
  max_quota: 0.20
  exploration_rate: 0.10
  newcomer_boost: 0.15
  performance_window: 60
  anti_collusion_enabled: false

ppc:
  target_utilization: 0.8
  target_queue_time: 120.0
  alpha: 0.15
  beta: 0.08
  min_price: 0.01
  max_price: 20.0
  adjustment_interval: 600s

api:
  enabled: false
  listen_addr: 0.0.0.0
  rpc_port: 9545
  ws_port: 9546

telemetry:
  enabled: false
  prometheus_port: 8090
  deai_enabled: true
`

	err := os.WriteFile(configPath, []byte(configContent), 0644)
	require.NoError(t, err)

	// Load config from file
	cfg, err := LoadConfig(configPath)

	require.NoError(t, err)
	require.NotNil(t, cfg)

	// Verify loaded values
	assert.Equal(t, "test-node", cfg.Node.Name)
	assert.Equal(t, "/tmp/test-data", cfg.Node.DataDir)
	assert.Equal(t, "debug", cfg.Node.LogLevel)
	assert.Equal(t, "validator", cfg.Node.Mode)
	assert.Equal(t, uint64(12345), cfg.Node.ChainID)
	assert.Equal(t, "fast", cfg.Node.SyncMode)

	assert.Equal(t, "192.168.1.1", cfg.Network.ListenAddr)
	assert.Equal(t, 40000, cfg.Network.P2PPort)
	assert.Equal(t, 100, cfg.Network.MaxPeers)

	assert.Equal(t, 2000, cfg.PoPC.SampleSize)
	assert.Equal(t, 0.03, cfg.PoPC.RedundancyRate)
	assert.Equal(t, 0.9999, cfg.PoPC.MinConfidence)
	assert.False(t, cfg.PoPC.StratifiedSampling)
	assert.False(t, cfg.PoPC.AdaptiveEscalation)
	assert.Equal(t, 7200*time.Second, cfg.PoPC.FraudWindowTime)

	assert.Equal(t, 128, cfg.ASR.TopK)
	assert.Equal(t, 0.20, cfg.ASR.MaxQuota)
	assert.Equal(t, 0.10, cfg.ASR.ExplorationRate)
	assert.Equal(t, 0.15, cfg.ASR.NewcomerBoost)
	assert.Equal(t, 60, cfg.ASR.PerformanceWindow)
	assert.False(t, cfg.ASR.AntiCollusionEnabled)

	assert.Equal(t, 0.8, cfg.PPC.TargetUtilization)
	assert.Equal(t, 120.0, cfg.PPC.TargetQueueTime)
	assert.Equal(t, 0.15, cfg.PPC.Alpha)
	assert.Equal(t, 0.08, cfg.PPC.Beta)
	assert.Equal(t, 0.01, cfg.PPC.MinPrice)
	assert.Equal(t, 20.0, cfg.PPC.MaxPrice)
	assert.Equal(t, 600*time.Second, cfg.PPC.AdjustmentInterval)

	assert.False(t, cfg.API.Enabled)
	assert.Equal(t, "0.0.0.0", cfg.API.ListenAddr)
	assert.Equal(t, 9545, cfg.API.RPCPort)
	assert.Equal(t, 9546, cfg.API.WSPort)

	assert.False(t, cfg.Telemetry.Enabled)
	assert.Equal(t, 8090, cfg.Telemetry.PrometheusPort)
	assert.True(t, cfg.Telemetry.DeAIEnabled)
}

func TestLoadConfig_PartialYAML(t *testing.T) {
	// Test loading with partial config - should merge with defaults
	tmpDir := t.TempDir()
	configPath := filepath.Join(tmpDir, "config.yaml")

	configContent := `
node:
  name: partial-node
  chain_id: 99999

asr:
  top_k: 32
`

	err := os.WriteFile(configPath, []byte(configContent), 0644)
	require.NoError(t, err)

	cfg, err := LoadConfig(configPath)

	require.NoError(t, err)
	require.NotNil(t, cfg)

	// Verify overridden values
	assert.Equal(t, "partial-node", cfg.Node.Name)
	assert.Equal(t, uint64(99999), cfg.Node.ChainID)
	assert.Equal(t, 32, cfg.ASR.TopK)

	// Verify default values are still present
	assert.Equal(t, ".axionax", cfg.Node.DataDir)
	assert.Equal(t, "info", cfg.Node.LogLevel)
	assert.Equal(t, 0.125, cfg.ASR.MaxQuota)
	assert.Equal(t, 1000, cfg.PoPC.SampleSize)
}

func TestLoadConfig_InvalidYAML(t *testing.T) {
	tmpDir := t.TempDir()
	configPath := filepath.Join(tmpDir, "config.yaml")

	invalidContent := `
node:
  name: test
  invalid: [[[
`

	err := os.WriteFile(configPath, []byte(invalidContent), 0644)
	require.NoError(t, err)

	cfg, err := LoadConfig(configPath)

	// Should return error for invalid YAML
	assert.Error(t, err)
	assert.Nil(t, cfg)
}

func TestNodeConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	assert.NotEmpty(t, cfg.Node.Name)
	assert.NotEmpty(t, cfg.Node.DataDir)
	assert.NotEmpty(t, cfg.Node.LogLevel)
	assert.NotEmpty(t, cfg.Node.Mode)
	assert.Greater(t, cfg.Node.ChainID, uint64(0))
}

func TestPoPCConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	// Sample size should be in recommended range
	assert.GreaterOrEqual(t, cfg.PoPC.SampleSize, 600)
	assert.LessOrEqual(t, cfg.PoPC.SampleSize, 2000)

	// Redundancy rate should be reasonable
	assert.GreaterOrEqual(t, cfg.PoPC.RedundancyRate, 0.02)
	assert.LessOrEqual(t, cfg.PoPC.RedundancyRate, 0.05)

	// Min confidence should be very high
	assert.GreaterOrEqual(t, cfg.PoPC.MinConfidence, 0.99)

	// Fraud window should be positive
	assert.Greater(t, cfg.PoPC.FraudWindowTime, time.Duration(0))
}

func TestASRConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	// TopK should be reasonable
	assert.GreaterOrEqual(t, cfg.ASR.TopK, 32)
	assert.LessOrEqual(t, cfg.ASR.TopK, 128)

	// MaxQuota should be between 10-20%
	assert.GreaterOrEqual(t, cfg.ASR.MaxQuota, 0.10)
	assert.LessOrEqual(t, cfg.ASR.MaxQuota, 0.20)

	// Exploration rate should be small
	assert.GreaterOrEqual(t, cfg.ASR.ExplorationRate, 0.01)
	assert.LessOrEqual(t, cfg.ASR.ExplorationRate, 0.10)

	// Performance window should be positive
	assert.Greater(t, cfg.ASR.PerformanceWindow, 0)
}

func TestPPCConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	// Target utilization should be between 0 and 1
	assert.GreaterOrEqual(t, cfg.PPC.TargetUtilization, 0.0)
	assert.LessOrEqual(t, cfg.PPC.TargetUtilization, 1.0)

	// Queue time should be positive
	assert.Greater(t, cfg.PPC.TargetQueueTime, 0.0)

	// Price bounds should be valid
	assert.Greater(t, cfg.PPC.MinPrice, 0.0)
	assert.Greater(t, cfg.PPC.MaxPrice, cfg.PPC.MinPrice)

	// Adjustment interval should be positive
	assert.Greater(t, cfg.PPC.AdjustmentInterval, time.Duration(0))
}

func TestDAConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	assert.NotEmpty(t, cfg.DA.StorageDir)
	assert.Greater(t, cfg.DA.ErasureCodingRate, 1.0)
	assert.Greater(t, cfg.DA.ChunkSize, 0)
	assert.Greater(t, cfg.DA.ReplicationFactor, 0)
	assert.Greater(t, cfg.DA.AvailabilityWindow, time.Duration(0))
}

func TestVRFConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	// Delay blocks should be at least 2
	assert.GreaterOrEqual(t, cfg.VRF.DelayBlocks, 2)
}

func TestConsensusConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	assert.Greater(t, cfg.Consensus.BlockTime, time.Duration(0))
	assert.Greater(t, cfg.Consensus.EpochLength, 0)
	assert.NotEmpty(t, cfg.Consensus.MinValidatorStake)
	assert.Greater(t, cfg.Consensus.MaxValidators, 0)

	// Slashing rate should be between 0 and 1
	assert.GreaterOrEqual(t, cfg.Consensus.SlashingRate, 0.0)
	assert.LessOrEqual(t, cfg.Consensus.SlashingRate, 1.0)

	// False pass penalty should be at least 500 basis points (5%)
	assert.GreaterOrEqual(t, cfg.Consensus.FalsePassPenalty, 500)
}

func TestAPIConfig_Defaults(t *testing.T) {
	cfg := DefaultConfig()

	assert.NotEmpty(t, cfg.API.ListenAddr)
	assert.Greater(t, cfg.API.RPCPort, 0)
	assert.Greater(t, cfg.API.WSPort, 0)
	assert.NotNil(t, cfg.API.CORSOrigins)
}

func BenchmarkDefaultConfig(b *testing.B) {
	for i := 0; i < b.N; i++ {
		DefaultConfig()
	}
}

func BenchmarkLoadConfig(b *testing.B) {
	tmpDir := b.TempDir()
	configPath := filepath.Join(tmpDir, "config.yaml")

	configContent := `
node:
  name: bench-node
  chain_id: 12345
`
	err := os.WriteFile(configPath, []byte(configContent), 0644)
	require.NoError(b, err)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		LoadConfig(configPath)
	}
}
