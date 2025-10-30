#!/usr/bin/env python3
"""
Migration tool: Go implementation → Rust/Python implementation
Handles data migration, state transfer, and validation
"""

import json
import os
import sys
import hashlib
from typing import List, Dict, Any, Optional
from dataclasses import dataclass, asdict
from datetime import datetime

# Add lib path for Rust bindings
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'deai', 'lib'))
import axionax_python as axx


@dataclass
class MigrationConfig:
    """Migration configuration"""
    go_data_dir: str
    rust_data_dir: str
    backup_dir: str
    validate_after_migration: bool = True
    parallel_migration: bool = False
    chunk_size: int = 1000


@dataclass
class MigrationReport:
    """Migration report"""
    start_time: str
    end_time: str
    duration_seconds: float
    blocks_migrated: int
    transactions_migrated: int
    validators_migrated: int
    state_entries_migrated: int
    errors: List[str]
    warnings: List[str]
    validation_passed: bool


class GoDataReader:
    """Read data from Go implementation"""
    
    def __init__(self, data_dir: str):
        self.data_dir = data_dir
        self.blockchain_file = os.path.join(data_dir, "blockchain.json")
        self.validators_file = os.path.join(data_dir, "validators.json")
        self.state_file = os.path.join(data_dir, "state.json")
    
    def read_blockchain(self) -> List[Dict[str, Any]]:
        """Read blockchain data from Go format"""
        if not os.path.exists(self.blockchain_file):
            return []
        
        with open(self.blockchain_file, 'r') as f:
            data = json.load(f)
        
        return data.get('blocks', [])
    
    def read_validators(self) -> List[Dict[str, Any]]:
        """Read validator data from Go format"""
        if not os.path.exists(self.validators_file):
            return []
        
        with open(self.validators_file, 'r') as f:
            data = json.load(f)
        
        return data.get('validators', [])
    
    def read_state(self) -> Dict[str, Any]:
        """Read state data from Go format"""
        if not os.path.exists(self.state_file):
            return {}
        
        with open(self.state_file, 'r') as f:
            return json.load(f)


class RustDataWriter:
    """Write data to Rust implementation"""
    
    def __init__(self, data_dir: str):
        self.data_dir = data_dir
        os.makedirs(data_dir, exist_ok=True)
        
        # Initialize Rust components
        self.blockchain = axx.PyBlockchain()
        self.consensus = axx.PyConsensusEngine()
    
    def migrate_validators(self, go_validators: List[Dict[str, Any]]) -> int:
        """Migrate validators from Go to Rust"""
        count = 0
        
        for v in go_validators:
            try:
                # Convert Go validator format to Rust format
                validator = axx.PyValidator(
                    address=v['address'],
                    stake=int(v.get('stake', 0)),
                    reputation=float(v.get('reputation', 0.5))
                )
                
                self.consensus.register_validator(validator)
                count += 1
                
            except Exception as e:
                print(f"Warning: Failed to migrate validator {v.get('address')}: {e}")
        
        return count
    
    def migrate_blocks(self, go_blocks: List[Dict[str, Any]]) -> tuple[int, int]:
        """Migrate blocks from Go to Rust"""
        blocks_count = 0
        txs_count = 0
        
        # Sort blocks by number
        sorted_blocks = sorted(go_blocks, key=lambda b: b.get('number', 0))
        
        for block in sorted_blocks:
            try:
                # Skip genesis (already created)
                if block.get('number', 0) == 0:
                    continue
                
                # Convert transactions
                transactions = []
                for tx in block.get('transactions', []):
                    rust_tx = axx.PyTransaction(
                        from_addr=tx.get('from', ''),
                        to=tx.get('to', ''),
                        amount=int(tx.get('amount', 0)),
                        data=list(tx.get('data', b''))
                    )
                    transactions.append(rust_tx)
                    txs_count += 1
                
                # Add block
                self.blockchain.add_block(transactions)
                blocks_count += 1
                
            except Exception as e:
                print(f"Warning: Failed to migrate block {block.get('number')}: {e}")
        
        return blocks_count, txs_count
    
    def migrate_state(self, go_state: Dict[str, Any]) -> int:
        """
        Migrate state data directly into Rust's state database via PyO3 bindings.
        Falls back to JSON dump if direct binding is not available.
        """
        # This assumes a PyStateDB object is exposed via PyO3
        # with a `set(key, value)` method.
        count = 0
        try:
            # Example: self.blockchain.state_db() returns the state DB object
            state_db = self.blockchain.state_db() 
            print("   -> Using direct state migration via Rust bindings.")
            for key, value in go_state.items():
                # Assuming the state DB binding accepts string key and JSON-encoded string value
                state_db.set(key, json.dumps(value))
                count += 1
        except (AttributeError, NotImplementedError):
            print("   -> Warning: Direct state migration not available. Falling back to JSON dump.")
            # Fallback to JSON dump if direct binding is not implemented
            state_file = os.path.join(self.data_dir, "migrated_state.json")
            with open(state_file, 'w') as f:
                json.dump(go_state, f, indent=2)
            return len(go_state)
        
        return count


class MigrationValidator:
    """Validate migration results"""
    
    def __init__(self, go_reader: GoDataReader, rust_writer: RustDataWriter):
        self.go_reader = go_reader
        self.rust_writer = rust_writer
    
    def validate_blockchain(self) -> tuple[bool, List[str]]:
        """Validate blockchain migration"""
        errors = []
        
        go_blocks = self.go_reader.read_blockchain()
        rust_height = self.rust_writer.blockchain.height()
        
        # Check block count (including genesis)
        expected_height = len(go_blocks)
        if rust_height != expected_height:
            errors.append(f"Block count mismatch: expected {expected_height}, got {rust_height}")
        
        # Validate each block
        for i, go_block in enumerate(go_blocks):
            rust_block = self.rust_writer.blockchain.get_block(i)
            
            if rust_block is None:
                errors.append(f"Block {i} not found in Rust blockchain")
                continue
            
            # Validate block number
            if rust_block.number != go_block.get('number', 0):
                errors.append(f"Block {i} number mismatch")
            
            # Validate transaction count
            expected_txs = len(go_block.get('transactions', []))
            if rust_block.transactions_count != expected_txs:
                errors.append(f"Block {i} transaction count mismatch")
        
        return len(errors) == 0, errors
    
    def validate_validators(self) -> tuple[bool, List[str]]:
        """Validate validator migration"""
        errors = []
        
        go_validators = self.go_reader.read_validators()
        rust_validators = self.rust_writer.consensus.get_validators()
        
        # Check count
        if len(rust_validators) != len(go_validators):
            errors.append(f"Validator count mismatch: expected {len(go_validators)}, got {len(rust_validators)}")
        
        # Validate each validator
        go_addrs = {v['address'] for v in go_validators}
        rust_addrs = {v.address for v in rust_validators}
        
        missing = go_addrs - rust_addrs
        if missing:
            errors.append(f"Missing validators: {missing}")
        
        return len(errors) == 0, errors
    
    def validate_state(self) -> tuple[bool, List[str]]:
        """Validate state migration"""
        errors = []
        go_state = self.go_reader.read_state()

        # Try to validate directly from Rust's state DB first
        try:
            state_db = self.rust_writer.blockchain.state_db()
            print("   -> Validating state directly from Rust DB.")
            for key, go_value in go_state.items():
                rust_value_json = state_db.get(key)
                if rust_value_json is None:
                    errors.append(f"State key missing in Rust DB: {key}")
                    continue
                
                rust_value = json.loads(rust_value_json)
                if rust_value != go_value:
                    errors.append(f"State value mismatch for key: {key}")

        except (AttributeError, NotImplementedError):
            # Fallback to validating the JSON file if direct access is not available
            print("   -> Direct state validation not available. Falling back to JSON file.")
            state_file = os.path.join(self.rust_writer.data_dir, "migrated_state.json")
            if not os.path.exists(state_file):
                errors.append("Migrated state file (migrated_state.json) not found for validation.")
                return False, errors
            
            with open(state_file, 'r') as f:
                rust_state = json.load(f)
            
            if set(go_state.keys()) != set(rust_state.keys()):
                missing = set(go_state.keys()) - set(rust_state.keys())
                extra = set(rust_state.keys()) - set(rust_state.keys())
                errors.append(f"State key mismatch. Missing: {missing}, Extra: {extra}")

        return len(errors) == 0, errors


class MigrationManager:
    """Main migration manager"""
    
    def __init__(self, config: MigrationConfig):
        self.config = config
        self.go_reader = GoDataReader(config.go_data_dir)
        self.rust_writer = RustDataWriter(config.rust_data_dir)
        self.validator = MigrationValidator(self.go_reader, self.rust_writer)
        
        # Create backup directory
        os.makedirs(config.backup_dir, exist_ok=True)
    
    def backup_go_data(self) -> bool:
        """Backup Go data before migration"""
        try:
            import shutil
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            backup_path = os.path.join(self.config.backup_dir, f"go_data_{timestamp}")
            
            shutil.copytree(self.config.go_data_dir, backup_path)
            print(f"✅ Backup created: {backup_path}")
            return True
            
        except Exception as e:
            print(f"❌ Backup failed: {e}")
            return False
    
    def migrate(self) -> MigrationReport:
        """Execute migration"""
        start_time = datetime.now()
        errors = []
        warnings = []
        
        print("🚀 Starting migration: Go → Rust/Python")
        print(f"   Go data: {self.config.go_data_dir}")
        print(f"   Rust data: {self.config.rust_data_dir}")
        
        # Step 1: Backup
        print("\n📦 Step 1: Backing up Go data...")
        if not self.backup_go_data():
            errors.append("Backup failed")
            # Continue anyway if user confirms
        
        # Step 2: Migrate validators
        print("\n👥 Step 2: Migrating validators...")
        go_validators = self.go_reader.read_validators()
        validators_migrated = self.rust_writer.migrate_validators(go_validators)
        print(f"   Migrated {validators_migrated}/{len(go_validators)} validators")
        
        # Step 3: Migrate blockchain
        print("\n⛓️  Step 3: Migrating blockchain...")
        go_blocks = self.go_reader.read_blockchain()
        blocks_migrated, txs_migrated = self.rust_writer.migrate_blocks(go_blocks)
        print(f"   Migrated {blocks_migrated} blocks, {txs_migrated} transactions")
        
        # Step 4: Migrate state
        print("\n💾 Step 4: Migrating state...")
        go_state = self.go_reader.read_state()
        state_entries = self.rust_writer.migrate_state(go_state)
        print(f"   Migrated {state_entries} state entries")
        
        # Step 5: Validation
        validation_passed = True
        if self.config.validate_after_migration:
            print("\n✅ Step 5: Validating migration...")
            
            blockchain_valid, blockchain_errors = self.validator.validate_blockchain()
            if not blockchain_valid:
                errors.extend(blockchain_errors)
                validation_passed = False
            else:
                print("   ✓ Blockchain validation passed")
            
            validators_valid, validator_errors = self.validator.validate_validators()
            if not validators_valid:
                errors.extend(validator_errors)
                validation_passed = False
            else:
                print("   ✓ Validators validation passed")
            
            state_valid, state_errors = self.validator.validate_state()
            if not state_valid:
                errors.extend(state_errors)
                validation_passed = False
            else:
                print("   ✓ State validation passed")
        
        end_time = datetime.now()
        duration = (end_time - start_time).total_seconds()
        
        # Create report
        report = MigrationReport(
            start_time=start_time.isoformat(),
            end_time=end_time.isoformat(),
            duration_seconds=duration,
            blocks_migrated=blocks_migrated,
            transactions_migrated=txs_migrated,
            validators_migrated=validators_migrated,
            state_entries_migrated=state_entries,
            errors=errors,
            warnings=warnings,
            validation_passed=validation_passed
        )
        
        # Save report
        report_file = os.path.join(self.config.rust_data_dir, "migration_report.json")
        with open(report_file, 'w') as f:
            json.dump(asdict(report), f, indent=2)
        
        print(f"\n{'✅' if validation_passed else '❌'} Migration completed in {duration:.2f}s")
        print(f"   Report saved: {report_file}")
        
        return report


def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Migrate from Go to Rust/Python")
    parser.add_argument("--go-data", required=True, help="Go data directory")
    parser.add_argument("--rust-data", required=True, help="Rust data directory")
    parser.add_argument("--backup", default="./backups", help="Backup directory")
    parser.add_argument("--no-validate", action="store_true", help="Skip validation")
    
    args = parser.parse_args()
    
    config = MigrationConfig(
        go_data_dir=args.go_data,
        rust_data_dir=args.rust_data,
        backup_dir=args.backup,
        validate_after_migration=not args.no_validate
    )
    
    manager = MigrationManager(config)
    report = manager.migrate()
    
    # Exit with error code if validation failed
    sys.exit(0 if report.validation_passed else 1)


if __name__ == "__main__":
    main()
