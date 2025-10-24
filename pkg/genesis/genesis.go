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
ID:     TestnetChainID,
ame:        "Axionax Testnet",
     "active",
dpoint: "https://testnet-rpc.axionax.io",
   "https://testnet-explorer.axionax.io",
},
MainnetChainID: {
ID: MainnetChainID,
ame:    "Axionax Mainnet",
 "planned",
},
}

// VerifyGenesisBlock validates genesis block
func VerifyGenesisBlock(chainID uint64, genesisHash string) error {
if chainID == LegacyDevChainID {
 nil // Local dev
}

network, exists := OfficialNetworks[chainID]
if !exists {
 fmt.Errorf("⚠️ WARNING: Chain ID %d is not official Axionax", chainID)
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
