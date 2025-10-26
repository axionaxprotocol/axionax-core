# Axionax Core v1.5 - Quick Start Guide

Welcome to Axionax! This guide will help you get started with running a node, validator, or worker on the Axionax testnet.

## 🎯 What You'll Learn

- Start a local testnet environment
- Run an Axionax node
- Become a validator
- Register as a compute worker
- Submit and monitor jobs

## ⚡ Quick Setup (5 minutes)

### Step 1: Prerequisites

Ensure you have:
- **Docker Desktop** installed and running
- **Go 1.21+** (for building from source)
- **16GB RAM** available

### Step 2: Clone and Build

```bash
# Clone repository
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core

# Build Axionax Core
make build
```

### Step 3: Start Testnet

```bash
# Start local testnet (Anvil + Explorer + Faucet)
cd Axionax_v1.5_Testnet_in_a_Box
docker compose up -d

# Verify services
docker compose ps
```

You should see:
- ✅ hardhat (Anvil) - http://localhost:8545
- ✅ blockscout - http://localhost:4001
- ✅ faucet - http://localhost:8080

### Step 4: Configure Axionax Node

```bash
cd ..
./build/axionax-core config init
```

Edit `config.yaml` to connect to local testnet (already configured by default).

### Step 5: Start Your Node

```bash
./build/axionax-core start --network testnet
```

🎉 **Success!** Your Axionax node is now running!

---

## 👤 User Paths

Choose your path:

### Path A: 🏛️ Run a Validator

Validators secure the network by performing PoPC validation.

**Requirements:**
- Minimum 10,000 AXX stake
- Reliable uptime
- Fast verification capabilities

**Steps:**

1. **Generate keys:**
```bash
./build/axionax-core keys generate --type validator
# Your address: 0xYourValidatorAddress
```

2. **Get testnet AXX:**
```bash
# Using curl
curl -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" \
  "http://localhost:8081/request?address=0xYourValidatorAddress"

# Or visit http://localhost:8080 in browser
```

3. **Stake AXX:**
```bash
./build/axionax-core stake deposit 10000 --address 0xYourValidatorAddress
```

4. **Start validating:**
```bash
./build/axionax-core validator start
```

5. **Check status:**
```bash
./build/axionax-core validator status
```

**Expected output:**
```
📊 Validator Status:
  Status: Active
  Stake: 10,000 AXX
  Validations: 0
  Success Rate: N/A (new validator)
```

### Path B: 🔧 Run a Worker

Workers provide compute power and earn rewards.

**Requirements:**
- Capable hardware (GPU recommended)
- Stable internet connection
- Minimum stake for registration

**Steps:**

1. **Create hardware spec:**
```bash
cat > worker-specs.json <<EOF
{
  "gpus": [{
    "model": "NVIDIA RTX 4090",
    "vram": 24,
    "count": 1
  }],
  "cpu_cores": 16,
  "ram": 64,
  "storage": 1000,
  "bandwidth": 1000,
  "region": "us-west",
  "asn": "AS15169",
  "organization": "my-org"
}
EOF
```

2. **Generate worker keys:**
```bash
./build/axionax-core keys generate --type worker
```

3. **Get testnet AXX from faucet** (same as validator)

4. **Register as worker:**
```bash
./build/axionax-core worker register --specs worker-specs.json
```

5. **Start worker:**
```bash
./build/axionax-core worker start
```

6. **Monitor status:**
```bash
./build/axionax-core worker status
```

**Expected output:**
```
📊 Worker Status:
  Status: Active
  Jobs Completed: 0
  Success Rate: N/A (new worker)
  Current Quota: 0%
```

### Path C: 💼 Submit Jobs (Client)

Submit compute jobs to the network.

**Steps:**

1. **Create job specification:**
```json
{
  "specs": {
    "gpu": "NVIDIA RTX 4090",
    "vram": 24,
    "framework": "PyTorch",
    "region": "us-west"
  },
  "sla": {
    "max_latency": "30s",
    "max_retries": 3,
    "timeout": "300s",
    "required_uptime": 0.99
  }
}
```

2. **Submit via RPC:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "axn_submitJob",
    "params": [/* job spec here */],
    "id": 1
  }'
```

3. **Monitor job:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "axn_getJobStatus",
    "params": ["job_abc123"],
    "id": 1
  }'
```

---

## 🔍 Monitoring & Debugging

### View Logs

```bash
# Node logs
tail -f ~/.axionax/logs/node.log

# Docker logs
cd Axionax_v1.5_Testnet_in_a_Box
docker compose logs -f hardhat
```

### Check Metrics

Visit http://localhost:9090/metrics for Prometheus metrics.

### Explore Blockchain

Visit http://localhost:4001 for Blockscout explorer.

### Query Pricing

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getPricingInfo","params":[],"id":1}'
```

---

## 🛠️ Common Commands

### Configuration

```bash
# Show current config
./build/axionax-core config show

# Initialize default config
./build/axionax-core config init
```

### Keys Management

```bash
# Generate new key
./build/axionax-core keys generate --type validator

# List keys
./build/axionax-core keys list
```

### Staking

```bash
# Check balance
./build/axionax-core stake balance

# Deposit stake
./build/axionax-core stake deposit 10000 --address 0x...

# Withdraw stake
./build/axionax-core stake withdraw 5000
```

### Status Checks

```bash
# Node version
./build/axionax-core version

# Validator status
./build/axionax-core validator status

# Worker status
./build/axionax-core worker status
```

---

## 🐛 Troubleshooting

### Connection Refused

**Problem:** Cannot connect to RPC endpoint

**Solution:**
```bash
# Check if testnet is running
cd Axionax_v1.5_Testnet_in_a_Box
docker compose ps

# Restart if needed
docker compose restart
```

### Chain ID Mismatch

**Problem:** Wrong chain ID

**Solution:** Check config.yaml has `chain_id: 31337`

### Insufficient Funds

**Problem:** Not enough AXX for transactions

**Solution:** Request more from faucet:
```bash
curl -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" \
  "http://localhost:8081/request?address=0xYourAddress"
```

### Docker Issues

**Problem:** Docker containers not starting

**Solution:**
```bash
# Clean up
docker compose down -v

# Restart
docker compose up -d
```

---

## 📚 Next Steps

- **Deep Dive:** Read [Architecture Overview](../ARCHITECTURE.md)
- **API Reference:** See [API Documentation](./API_REFERENCE.md)
- **Build from Source:** Follow [Build Guide](./BUILD.md)
- **Production Deploy:** Check [Testnet Integration](./TESTNET_INTEGRATION.md)
- **Join Community:** Discord at https://discord.gg/axionax

---

## 🆘 Getting Help

- **Documentation:** https://docs.axionax.org
- **GitHub Issues:** https://github.com/axionaxprotocol/axionax-core/issues
- **Discord:** https://discord.gg/axionax
- **Telegram:** https://t.me/axionax

---

## ⚠️ Testnet Disclaimer

This is a **testnet** environment. Do not use real assets or run this in production without proper security audits.

---

**Made with 💜 by the Axionax community**

Happy building! 🚀
