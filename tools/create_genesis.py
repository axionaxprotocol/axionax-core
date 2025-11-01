#!/usr/bin/env python3
"""
Genesis Configuration Generator
Creates genesis.json for Axionax Testnet launch
"""

import json
import sys
from datetime import datetime, timezone
from typing import List, Dict

class GenesisGenerator:
    def __init__(self, chain_id: int = 86137):
        self.chain_id = chain_id
        self.genesis = {
            "config": {
                "chainId": chain_id,
                "homesteadBlock": 0,
                "eip150Block": 0,
                "eip155Block": 0,
                "eip158Block": 0,
                "byzantiumBlock": 0,
                "constantinopleBlock": 0,
                "petersburgBlock": 0,
                "istanbulBlock": 0,
                "berlinBlock": 0,
                "londonBlock": 0,
                "axionax": {
                    "consensus": "popc",
                    "blockTime": 5,
                    "epochLength": 100,
                    "minValidatorStake": "10000000000000000000000",  # 10,000 AXX
                    "maxValidators": 100,
                    "slashingRate": 0.1,
                    "falsPassPenalty": 500  # 5% in basis points
                }
            },
            "nonce": "0x0",
            "timestamp": "0x0",  # Will be set
            "extraData": "0x",
            "gasLimit": "0x1c9c380",  # 30M gas
            "difficulty": "0x1",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "coinbase": "0x0000000000000000000000000000000000000000",
            "validators": [],
            "alloc": {}
        }
    
    def set_genesis_time(self, dt: datetime = None):
        """Set genesis timestamp"""
        if dt is None:
            dt = datetime.now(timezone.utc)
        timestamp = int(dt.timestamp())
        self.genesis["timestamp"] = hex(timestamp)
        print(f"Genesis time set to: {dt.isoformat()} (Unix: {timestamp})")
    
    def add_validator(self, address: str, name: str, stake: str, 
                     commission: float, enode: str = ""):
        """Add genesis validator"""
        if not address.startswith("0x"):
            address = f"0x{address}"
        
        validator = {
            "address": address,
            "name": name,
            "stake": stake,
            "commission": commission,
            "enode": enode,
            "active": True
        }
        self.genesis["validators"].append(validator)
        
        # Add to alloc (initial balance)
        self.genesis["alloc"][address] = {
            "balance": stake
        }
        print(f"Added validator: {name} ({address})")
    
    def add_allocation(self, address: str, balance: str, 
                      vesting: bool = False, vesting_schedule: str = ""):
        """Add token allocation"""
        if not address.startswith("0x"):
            address = f"0x{address}"
        
        alloc = {"balance": balance}
        if vesting:
            alloc["vesting"] = {
                "enabled": True,
                "schedule": vesting_schedule
            }
        
        self.genesis["alloc"][address] = alloc
        print(f"Added allocation: {address} = {balance} wei")
    
    def add_contract(self, address: str, bytecode: str, storage: Dict = None):
        """Add pre-deployed contract"""
        if not address.startswith("0x"):
            address = f"0x{address}"
        
        contract = {
            "balance": "0",
            "code": bytecode
        }
        if storage:
            contract["storage"] = storage
        
        self.genesis["alloc"][address] = contract
        print(f"Added contract at: {address}")
    
    def load_validators_from_file(self, filepath: str):
        """Load validators from JSON file"""
        with open(filepath, 'r') as f:
            validators = json.load(f)
        
        for v in validators:
            self.add_validator(
                address=v["address"],
                name=v["name"],
                stake=v.get("stake", "50000000000000000000000"),  # 50K AXX default
                commission=v.get("commission", 0.10),
                enode=v.get("enode", "")
            )
        print(f"Loaded {len(validators)} validators from {filepath}")
    
    def load_allocations_from_file(self, filepath: str):
        """Load token allocations from JSON file"""
        with open(filepath, 'r') as f:
            allocations = json.load(f)
        
        for alloc in allocations:
            self.add_allocation(
                address=alloc["address"],
                balance=alloc["balance"],
                vesting=alloc.get("vesting", False),
                vesting_schedule=alloc.get("vesting_schedule", "")
            )
        print(f"Loaded {len(allocations)} allocations from {filepath}")
    
    def validate(self) -> bool:
        """Validate genesis configuration"""
        errors = []
        
        # Check validators
        if len(self.genesis["validators"]) == 0:
            errors.append("No validators defined")
        
        if len(self.genesis["validators"]) < 3:
            errors.append("Warning: Less than 3 validators (not recommended)")
        
        # Check allocations
        total_supply = 0
        for addr, alloc in self.genesis["alloc"].items():
            if "balance" in alloc:
                total_supply += int(alloc["balance"], 16 if alloc["balance"].startswith("0x") else 10)
        
        print(f"\nValidation Results:")
        print(f"  Validators: {len(self.genesis['validators'])}")
        print(f"  Allocations: {len(self.genesis['alloc'])}")
        print(f"  Total Supply: {total_supply / 10**18:.2f} AXX")
        
        if errors:
            print("\nErrors:")
            for error in errors:
                print(f"  ❌ {error}")
            return False
        else:
            print("  ✅ Genesis configuration is valid")
            return True
    
    def save(self, filepath: str = "genesis.json"):
        """Save genesis to file"""
        with open(filepath, 'w') as f:
            json.dump(self.genesis, f, indent=2)
        print(f"\n✅ Genesis saved to: {filepath}")
        
        # Calculate and print hash
        import hashlib
        with open(filepath, 'rb') as f:
            genesis_hash = hashlib.sha256(f.read()).hexdigest()
        print(f"📝 Genesis Hash: 0x{genesis_hash}")
        return genesis_hash

def main():
    """Main function"""
    if len(sys.argv) < 2:
        print("Usage: python3 create_genesis.py <validators.json> [allocations.json]")
        print("\nExample validators.json:")
        print("""[
  {
    "name": "Validator-01",
    "address": "0x1234567890abcdef1234567890abcdef12345678",
    "stake": "50000000000000000000000",
    "commission": 0.10,
    "enode": "enode://..."
  }
]""")
        sys.exit(1)
    
    validators_file = sys.argv[1]
    allocations_file = sys.argv[2] if len(sys.argv) > 2 else None
    
    # Create generator
    print("=" * 60)
    print("Axionax Genesis Generator")
    print("=" * 60)
    
    gen = GenesisGenerator(chain_id=86137)
    
    # Set genesis time
    # For production, set specific time
    # genesis_time = datetime(2025, 1, 15, 0, 0, 0, tzinfo=timezone.utc)
    # gen.set_genesis_time(genesis_time)
    gen.set_genesis_time()  # Use current time
    
    # Load validators
    print(f"\nLoading validators from: {validators_file}")
    gen.load_validators_from_file(validators_file)
    
    # Load allocations (optional)
    if allocations_file:
        print(f"\nLoading allocations from: {allocations_file}")
        gen.load_allocations_from_file(allocations_file)
    else:
        # Add default allocations
        print("\nAdding default allocations...")
        # Foundation
        gen.add_allocation(
            "0xF0UNDA7I0N0000000000000000000000000000001",
            "300000000000000000000000000",  # 300M AXX
            vesting=True,
            vesting_schedule="4 years linear"
        )
        # Rewards Pool
        gen.add_allocation(
            "0xREWARD5P00L0000000000000000000000000000002",
            "250000000000000000000000000",  # 250M AXX
        )
    
    # Validate
    print("\n" + "=" * 60)
    if not gen.validate():
        print("❌ Validation failed!")
        sys.exit(1)
    
    # Save
    print("=" * 60)
    genesis_hash = gen.save("genesis.json")
    
    print("\n" + "=" * 60)
    print("Next Steps:")
    print("=" * 60)
    print("1. Review genesis.json")
    print("2. Distribute to all validators")
    print(f"3. Announce genesis hash: 0x{genesis_hash}")
    print("4. Validators verify and initialize nodes")
    print("5. Coordinate launch time")
    print("\nFor support: validators@axionax.org")

if __name__ == "__main__":
    main()
