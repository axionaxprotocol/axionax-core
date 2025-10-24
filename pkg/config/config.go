// Package config handles configuration management for Axionax nodes
package config

import (
	"time"

	"github.com/spf13/viper"
)

// Config holds all configuration for an Axionax node
type Config struct {
	Node      NodeConfig      `mapstructure:"node"`
	Network   NetworkConfig   `mapstructure:"network"`
	PoPC      PoPCConfig      `mapstructure:"popc"`
	ASR       ASRConfig       `mapstructure:"asr"`
	PPC       PPCConfig       `mapstructure:"ppc"`
	DA        DAConfig        `mapstructure:"da"`
	VRF       VRFConfig       `mapstructure:"vrf"`
	Consensus ConsensusConfig `mapstructure:"consensus"`
	API       APIConfig       `mapstructure:"api"`
	Telemetry TelemetryConfig `mapstructure:"telemetry"`
}

// NodeConfig defines node-specific settings
type NodeConfig struct {
	Name     string `mapstructure:"name"`
	DataDir  string `mapstructure:"data_dir"`
	LogLevel string `mapstructure:"log_level"`
	Mode     string `mapstructure:"mode"` // validator, worker, full, light
	ChainID  uint64 `mapstructure:"chain_id"`
	SyncMode string `mapstructure:"sync_mode"`
}

// NetworkConfig defines network settings
type NetworkConfig struct {
	ListenAddr     string   `mapstructure:"listen_addr"`
	P2PPort        int      `mapstructure:"p2p_port"`
	MaxPeers       int      `mapstructure:"max_peers"`
	BootstrapNodes []string `mapstructure:"bootstrap_nodes"`
	Seeds          []string `mapstructure:"seeds"`
}

// PoPCConfig defines Proof-of-Probabilistic-Checking parameters
type PoPCConfig struct {
	SampleSize         int           `mapstructure:"sample_size"`     // s, default 600-1500
	RedundancyRate     float64       `mapstructure:"redundancy_rate"` // β, default 2-3%
	MinConfidence      float64       `mapstructure:"min_confidence"`  // Required detection probability
	StratifiedSampling bool          `mapstructure:"stratified_sampling"`
	AdaptiveEscalation bool          `mapstructure:"adaptive_escalation"`
	FraudWindowTime    time.Duration `mapstructure:"fraud_window_time"` // ~3600s
}

// ASRConfig defines Auto-Selection Router parameters
type ASRConfig struct {
	TopK                 int     `mapstructure:"top_k"`              // K, default 64
	MaxQuota             float64 `mapstructure:"max_quota"`          // q_max, default 10-15%
	ExplorationRate      float64 `mapstructure:"exploration_rate"`   // ε, default 5%
	NewcomerBoost        float64 `mapstructure:"newcomer_boost"`     // Bonus for newcomers
	PerformanceWindow    int     `mapstructure:"performance_window"` // Days to consider, 7-30
	AntiCollusionEnabled bool    `mapstructure:"anti_collusion_enabled"`
}

// PPCConfig defines Posted Price Controller parameters
type PPCConfig struct {
	TargetUtilization  float64       `mapstructure:"target_utilization"` // util*, default 0.7
	TargetQueueTime    float64       `mapstructure:"target_queue_time"`  // q*, default 60s
	Alpha              float64       `mapstructure:"alpha"`              // Price adjustment rate
	Beta               float64       `mapstructure:"beta"`               // Queue weight
	MinPrice           float64       `mapstructure:"min_price"`          // p_min
	MaxPrice           float64       `mapstructure:"max_price"`          // p_max
	AdjustmentInterval time.Duration `mapstructure:"adjustment_interval"`
}

// DAConfig defines Data Availability parameters
type DAConfig struct {
	StorageDir         string        `mapstructure:"storage_dir"`
	ErasureCodingRate  float64       `mapstructure:"erasure_coding_rate"` // e.g., 1.5x
	ChunkSize          int           `mapstructure:"chunk_size"`          // in KB
	AvailabilityWindow time.Duration `mapstructure:"availability_window"` // Δt_DA
	ReplicationFactor  int           `mapstructure:"replication_factor"`
	LiveAuditEnabled   bool          `mapstructure:"live_audit_enabled"`
}

// VRFConfig defines Verifiable Random Function parameters
type VRFConfig struct {
	DelayBlocks   int  `mapstructure:"delay_blocks"` // k, ≥2 blocks
	UseDelayedVRF bool `mapstructure:"use_delayed_vrf"`
}

// ConsensusConfig defines consensus parameters
type ConsensusConfig struct {
	BlockTime         time.Duration `mapstructure:"block_time"`
	EpochLength       int           `mapstructure:"epoch_length"` // Blocks per epoch
	MinValidatorStake string        `mapstructure:"min_validator_stake"`
	MaxValidators     int           `mapstructure:"max_validators"`
	SlashingRate      float64       `mapstructure:"slashing_rate"`      // For false pass
	FalsePassPenalty  int           `mapstructure:"false_pass_penalty"` // basis points, ≥500
}

// APIConfig defines API server settings
type APIConfig struct {
	Enabled     bool     `mapstructure:"enabled"`
	ListenAddr  string   `mapstructure:"listen_addr"`
	RPCPort     int      `mapstructure:"rpc_port"`
	WSPort      int      `mapstructure:"ws_port"`
	CORSOrigins []string `mapstructure:"cors_origins"`
}

// TelemetryConfig defines telemetry and monitoring settings
type TelemetryConfig struct {
	Enabled        bool   `mapstructure:"enabled"`
	PrometheusPort int    `mapstructure:"prometheus_port"`
	MetricsAddr    string `mapstructure:"metrics_addr"`
	DeAIEnabled    bool   `mapstructure:"deai_enabled"`
}

// DefaultConfig returns a config with default values
func DefaultConfig() *Config {
	return &Config{
		Node: NodeConfig{
			Name:     "axionax-node",
			DataDir:  ".axionax",
			LogLevel: "info",
			Mode:     "full",
			ChainID:  31337,
			SyncMode: "full",
		},
		Network: NetworkConfig{
			ListenAddr:     "0.0.0.0",
			P2PPort:        30303,
			MaxPeers:       50,
			BootstrapNodes: []string{},
			Seeds:          []string{},
		},
		PoPC: PoPCConfig{
			SampleSize:         1000,
			RedundancyRate:     0.025, // 2.5%
			MinConfidence:      0.999,
			StratifiedSampling: true,
			AdaptiveEscalation: true,
			FraudWindowTime:    3600 * time.Second,
		},
		ASR: ASRConfig{
			TopK:                 64,
			MaxQuota:             0.125, // 12.5%
			ExplorationRate:      0.05,  // 5%
			NewcomerBoost:        0.1,
			PerformanceWindow:    30, // days
			AntiCollusionEnabled: true,
		},
		PPC: PPCConfig{
			TargetUtilization:  0.7,
			TargetQueueTime:    60.0,
			Alpha:              0.1,
			Beta:               0.05,
			MinPrice:           0.001,
			MaxPrice:           10.0,
			AdjustmentInterval: 300 * time.Second, // 5 minutes
		},
		DA: DAConfig{
			StorageDir:         "data/da",
			ErasureCodingRate:  1.5,
			ChunkSize:          256, // KB
			AvailabilityWindow: 300 * time.Second,
			ReplicationFactor:  3,
			LiveAuditEnabled:   true,
		},
		VRF: VRFConfig{
			DelayBlocks:   2,
			UseDelayedVRF: true,
		},
		Consensus: ConsensusConfig{
			BlockTime:         5 * time.Second,
			EpochLength:       100,
			MinValidatorStake: "10000",
			MaxValidators:     100,
			SlashingRate:      0.1, // 10%
			FalsePassPenalty:  500, // 5%
		},
		API: APIConfig{
			Enabled:     true,
			ListenAddr:  "127.0.0.1",
			RPCPort:     8545,
			WSPort:      8546,
			CORSOrigins: []string{"*"},
		},
		Telemetry: TelemetryConfig{
			Enabled:        true,
			PrometheusPort: 9090,
			MetricsAddr:    "0.0.0.0:9090",
			DeAIEnabled:    false,
		},
	}
}

// LoadConfig loads configuration from file and environment
func LoadConfig(configPath string) (*Config, error) {
	config := DefaultConfig()

	if configPath != "" {
		viper.SetConfigFile(configPath)
	} else {
		viper.SetConfigName("config")
		viper.SetConfigType("yaml")
		viper.AddConfigPath(".")
		viper.AddConfigPath("$HOME/.axionax")
		viper.AddConfigPath("/etc/axionax/")
	}

	viper.AutomaticEnv()
	viper.SetEnvPrefix("AXIONAX")

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, err
		}
		// Config file not found; using defaults
	}

	if err := viper.Unmarshal(config); err != nil {
		return nil, err
	}

	return config, nil
}
