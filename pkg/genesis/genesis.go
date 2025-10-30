package genesis

import (
	"fmt"
)

// Official chain IDs
const (
	TestnetChainID   uint64 = 86137 // Axionax Testnet
	MainnetChainID   uint64 = 86150 // Axionax Mainnet (reserved)
	LegacyDevChainID uint64 = 31337 // Local dev only
)

// GenesisHashes stores official genesis hashes
var GenesisHashes = map[uint64]string{
	TestnetChainID: "", // To be set after deployment
	MainnetChainID: "", // To be set at mainnet launch
}

// NetworkInfo describes an official network
type NetworkInfo struct {
	ChainID     uint64
	Name        string
	GenesisHash string
	Status      string
	RPCEndpoint string
	Explorer    string
}

// OfficialNetworks registry
var OfficialNetworks = map[uint64]NetworkInfo{
	TestnetChainID: {
		ChainID:     TestnetChainID,
		Name:        "Axionax Testnet",
		Status:      "active",
		RPCEndpoint: "https://testnet-rpc.axionax.org",
		Explorer:    "https://testnet-explorer.axionax.org",
	},
	MainnetChainID: {
		ChainID: MainnetChainID,
		Name:    "Axionax Mainnet",
		Status:  "planned",
	},
}

// VerifyGenesisBlock validates genesis block
func VerifyGenesisBlock(chainID uint64, genesisHash string) error {
	if chainID == LegacyDevChainID {
		return nil // Local dev, skip verification
	}

	network, exists := OfficialNetworks[chainID]
	if !exists {
		// Return a warning, but not a fatal error for non-official chains
		return fmt.Errorf("⚠️ WARNING: Chain ID %d is not an official Axionax network", chainID)
	}

	if network.GenesisHash != "" && genesisHash != network.GenesisHash {
		fmt.Errorf("❌ GENESIS MISMATCH: Possible FAKE network!")
	}

	return nil
}

// IsOfficialNetwork checks if chain ID is official
func IsOfficialNetwork(chainID uint64) bool {
	_, exists := OfficialNetworks[chainID]
	return exists
}
