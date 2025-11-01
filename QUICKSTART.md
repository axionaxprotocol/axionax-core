# Axionax Core v1.6 - Quick Start Guide

Welcome to the Axionax v1.6 multi-language architecture! This guide will help you get started with building the project, running a node, and interacting with the testnet.

## 🎯 What You'll Learn

- Build the Rust core and Python bridge
- Run an Axionax node
- Become a validator
- Register as a compute worker
- Submit jobs to the network

## ⚡ Quick Setup (10 minutes)

### Step 1: Prerequisites

Ensure you have:
- **Rust 1.75+** & Cargo - [Install](https://rustup.rs/)
- **Python 3.10+** - [Download](https://www.python.org/downloads/)
- **A C++ compiler** (like GCC, Clang, or MSVC for RocksDB)
- **16GB RAM** recommended

### Step 2: Clone and Build

This process builds the Rust core, the Python bridge, and prepares the necessary executables.

```bash
# 1. Clone the repository
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core

# 2. Build the Rust core workspace
echo "🦀 Building Rust core..."
cargo build --release --workspace

# 3. Build the Python-Rust bridge
echo "🐍 Building Python bridge..."
cd bridge/rust-python
./build.sh
cd ../.. # Return to root directory
```

After these steps, you will have the main executable at `target/release/axionax-core`.

### Step 3: Configure Your Node

Configuration is now handled via command-line flags or environment variables. (config.yaml is deprecated).

```bash
# Initialize default configuration (optional, shows default paths)
target/release/axionax-core config init
```

### Step 4: Start Your Node

This command starts the core node and connects it to the public testnet.

```bash
target/release/axionax-core start --network testnet
```

🎉 **Success!** Your Axionax node is now running and syncing with the testnet.

---

## 👤 User Paths

Choose your path. You will need testnet AXX tokens for staking.

*Note: The testnet faucet is currently under development. For now, please request tokens in the #testnet-faucet channel on our Discord.*

### Path A: 🏛️ Run a Validator

Validators secure the network by performing PoPC validation.

**Requirements:**
- Minimum 100,000 AXX stake
- Reliable uptime and internet connection

**Steps:**

1. **Generate validator keys:**
```bash
target/release/axionax-core keys generate --type validator
# Your address: 0x...
```

2. **Get testnet AXX from Discord**

3. **Stake AXX:**
```bash
target/release/axionax-core stake deposit 100000 --address 0xYourValidatorAddress
```

4. **Start validating:**
```bash
target/release/axionax-core validator start
```

5. **Check status:**
```bash
target/release/axionax-core validator status
```

### Path B: 🔧 Run a Worker

Workers provide compute power and earn rewards.

**Requirements:**
- Capable hardware (GPU optional but recommended)
- Stable internet connection

**Steps:**

1. **Create hardware specification (`worker-specs.json`):**
```json
{
  "gpus": [{ "model": "NVIDIA RTX 4090", "vram": 24, "count": 1 }],
  "cpu_cores": 16,
  "ram": 64,
  "storage": 1000,
  "bandwidth": 1000,
  "region": "us-west"
}
```

2. **Generate worker keys:**
```bash
target/release/axionax-core keys generate --type worker
```

3. **Get testnet AXX from Discord**

4. **Register as a worker:**
```bash
target/release/axionax-core worker register --specs worker-specs.json
```

5. **Start the worker:**
```bash
target/release/axionax-core worker start
```

### Path C: 💼 Submit Jobs (Client)

Submit compute jobs using the TypeScript SDK or direct JSON-RPC calls.

**Using TypeScript SDK:**
```typescript
import { AxionaxClient } from '@axionax/sdk';
const client = new AxionaxClient('http://localhost:8545'); // Or public RPC
const jobId = await client.submitJob({ /* ... job spec ... */ });
```

**Using cURL:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "axn_submitJob",
    "params": [{ "specs": { "gpu": "NVIDIA RTX 4090" } }],
    "id": 1
  }'
```

---

## 🔍 Monitoring & Debugging

### View Logs

Logs are now printed directly to standard output (stdout). You can redirect them to a file.

```bash
# Run node and save logs to a file
target/release/axionax-core start --network testnet > node.log 2>&1 &

# View logs in real-time
tail -f node.log
```

### Querying the Node

Use the JSON-RPC interface for queries.

```bash
# Get node version
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_version","params":[],"id":1}'

# Get latest block number
curl -X POST http://localhost:8545 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_blockNumber","params":[],"id":1}'
```

---

## 🛠️ Common Commands (`target/release/axionax-core`)

### Keys Management
- `keys generate --type [validator|worker]`
- `keys list`

### Staking
- `stake balance --address 0x...`
- `stake deposit <amount> --address 0x...`
- `stake withdraw <amount>`

### Status Checks
- `version`
- `validator status`
- `worker status`

---

## 🐛 Troubleshooting

### Build Failures (Rust)

**Problem**: `error: failed to run custom build command for rocksdb-sys`

**Solution**: Ensure you have a C++ compiler installed (GCC, Clang, or MSVC).
- **Ubuntu/Debian**: `sudo apt-get install build-essential`
- **Fedora/CentOS**: `sudo yum groupinstall "Development Tools"`
- **macOS**: `xcode-select --install`
- **Windows**: Install Visual Studio with "Desktop development with C++".

### Python Bridge Issues

**Problem**: `ImportError: cannot import name 'axionax_python'`

**Solution**: The `build.sh` script should handle this. If issues persist, manually set the `PYTHONPATH`:
```bash
export PYTHONPATH=$(pwd)/bridge/rust-python/lib:$PYTHONPATH
```

### Connection Refused

**Problem**: Cannot connect to RPC at `localhost:8545`.

**Solution**:
1. Make sure your node is running.
2. Check if you specified a different port with the `--rpc-port` flag.
3. Check your firewall settings.

---

## 📚 Next Steps

- **Deep Dive:** Read the [New Architecture](./NEW_ARCHITECTURE.md) document.
- **Contributing:** See the [Contributing Guide](./CONTRIBUTING.md).
- **Join Community:** Find us on [Discord](https://discord.gg/axionax).

---

**Made with 💜 by the Axionax community**

Happy building! 🚀
