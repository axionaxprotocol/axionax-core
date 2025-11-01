# VPS Validator Setup Guide
## Axionax v1.6 Testnet Public Launch

> **เอกสารนี้**: คู่มือการติดตั้งและตั้งค่า Validator Node บน VPS สำหรับ Testnet Public Launch

## 📋 สารบัญ
- [ความต้องการขั้นต่ำ](#ความต้องการขั้นต่ำ)
- [การเตรียม VPS](#การเตรียม-vps)
- [ติดตั้ง Dependencies](#ติดตั้ง-dependencies)
- [Build Axionax Node](#build-axionax-node)
- [ตั้งค่า Validator](#ตั้งค่า-validator)
- [Genesis Setup](#genesis-setup)
- [เริ่มต้น Validator Node](#เริ่มต้น-validator-node)
- [Monitoring & Maintenance](#monitoring--maintenance)

---

## ความต้องการขั้นต่ำ

### Hardware Requirements
- **CPU**: 4 cores (8 threads แนะนำ)
- **RAM**: 16GB (32GB แนะนำสำหรับ production)
- **Storage**: 500GB SSD NVMe
- **Network**: 1Gbps, Public IP, Ports 30303 (P2P), 8545 (RPC), 9090 (Metrics)

### Software Requirements
- **OS**: Ubuntu 22.04 LTS (แนะนำ) / Ubuntu 20.04 LTS
- **Rust**: 1.75+ (nightly recommended)
- **Python**: 3.10+ (สำหรับ DeAI modules)
- **Node.js**: 18+ (สำหรับ TypeScript SDK)
- **Docker**: 24+ (optional, สำหรับ containerized deployment)
- **Git**: 2.30+

### Network Requirements
- **Inbound Ports**:
  - `30303/tcp` - P2P networking
  - `30303/udp` - P2P discovery
- **Outbound Ports**:
  - `80/tcp`, `443/tcp` - Package downloads, bootstrap nodes
- **Firewall**: UFW หรือ iptables

---

## การเตรียม VPS

### 1. เชื่อมต่อ VPS
```bash
# SSH เข้า VPS ด้วย SSH key (แนะนำ)
ssh -i ~/.ssh/id_rsa root@YOUR_VPS_IP

# หรือใช้ password (ไม่แนะนำสำหรับ production)
ssh root@YOUR_VPS_IP
```

### 2. Update System
```bash
# Update package lists
sudo apt update && sudo apt upgrade -y

# Install essential tools
sudo apt install -y curl wget git build-essential \
    pkg-config libssl-dev ufw htop net-tools \
    software-properties-common
```

### 3. Create Dedicated User
```bash
# สร้าง user สำหรับรัน validator (security best practice)
sudo useradd -m -s /bin/bash axionax
sudo usermod -aG sudo axionax

# Set password
sudo passwd axionax

# Switch to axionax user
su - axionax
```

### 4. Configure Firewall
```bash
# Enable UFW
sudo ufw enable

# Allow SSH (important!)
sudo ufw allow 22/tcp

# Allow Axionax P2P
sudo ufw allow 30303/tcp
sudo ufw allow 30303/udp

# Allow RPC (optional, ถ้าต้องการเปิด public RPC)
# sudo ufw allow 8545/tcp

# Allow Metrics (optional)
# sudo ufw allow 9090/tcp

# Check status
sudo ufw status
```

---

## ติดตั้ง Dependencies

### 1. Install Rust
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source $HOME/.cargo/env

# Install nightly toolchain (recommended for performance)
rustup install nightly
rustup default nightly

# Verify installation
rustc --version
cargo --version
```

### 2. Install Python 3.10+
```bash
# Ubuntu 22.04 มาพร้อม Python 3.10 อยู่แล้ว
python3 --version

# Install pip and venv
sudo apt install -y python3-pip python3-venv python3-dev

# Verify
pip3 --version
```

### 3. Install Node.js & npm
```bash
# Install Node.js 18 LTS
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Verify
node --version
npm --version
```

### 4. Install Additional Dependencies
```bash
# Install Python ML libraries (for DeAI)
pip3 install --user numpy scipy scikit-learn torch

# Install Rust build tools
cargo install cargo-watch cargo-edit

# Install protobuf compiler (for proto files)
sudo apt install -y protobuf-compiler
```

---

## Build Axionax Node

### 1. Clone Repository
```bash
# Clone to home directory
cd ~
git clone https://github.com/axionaxprotocol/axionax-core.git
cd axionax-core

# Checkout stable branch (or specific tag)
git checkout main  # หรือ git checkout v1.6.0
```

### 2. Build Rust Core
```bash
# Build release binary (optimized)
cargo build --release

# Binary จะอยู่ที่ target/release/axionax-core
ls -lh target/release/axionax-core

# Optional: Copy to system path
sudo cp target/release/axionax-core /usr/local/bin/
sudo chmod +x /usr/local/bin/axionax-core
```

### 3. Build Python DeAI Modules
```bash
# Setup Python virtual environment
cd ~/axionax-core/deai
python3 -m venv venv
source venv/bin/activate

# Install requirements
pip install -r requirements.txt

# Test DeAI modules
python3 -c "import asr; print('ASR module loaded successfully')"
python3 -c "import fraud_detection; print('Fraud detection loaded successfully')"

deactivate
```

### 4. Build TypeScript SDK (Optional)
```bash
cd ~/axionax-core/sdk
npm install
npm run build

# Verify
npm run test
```

---

## ตั้งค่า Validator

### 1. Create Node Directory Structure
```bash
# Create data directory
mkdir -p ~/.axionax/{keystore,config,data,logs}

# Set permissions
chmod 700 ~/.axionax
chmod 700 ~/.axionax/keystore
```

### 2. Generate Validator Keys
```bash
# สร้าง validator keypair
axionax-core keys generate --output ~/.axionax/keystore/validator.json

# Output จะได้:
# - Public Key (validator address)
# - Private Key (เก็บไว้ปลอดภัย!)
# - Mnemonic phrase (backup!)

# IMPORTANT: Backup mnemonic phrase ลงกระดาษหรือเก็บไว้ offline
```

**⚠️ CRITICAL**: 
- **เก็บ mnemonic phrase ไว้ปลอดภัย** - นี่คือกุญแจสำคัญในการกู้คืน validator
- **อย่า share private key** ให้ใครเด็ดขาด
- **Backup keypair** ไปเก็บไว้ที่อื่น (encrypted USB, offline storage)

### 3. Configure Node
```bash
# Copy example config
cp ~/axionax-core/environments/config.example.yaml ~/.axionax/config/config.yaml

# Edit configuration
nano ~/.axionax/config/config.yaml
```

แก้ไข `config.yaml`:
```yaml
node:
  name: "validator-01"  # เปลี่ยนเป็นชื่อ validator ของคุณ
  data_dir: "/home/axionax/.axionax/data"
  log_level: "info"
  mode: "validator"  # ⚠️ สำคัญ! ตั้งเป็น validator
  chain_id: 86137    # Axionax Testnet

network:
  listen_addr: "0.0.0.0"
  p2p_port: 30303
  max_peers: 50
  bootstrap_nodes:
    - "enode://GENESIS_NODE_1@IP:30303"  # จะได้หลัง genesis
    - "enode://GENESIS_NODE_2@IP:30303"

validator:
  keystore_path: "/home/axionax/.axionax/keystore/validator.json"
  stake_amount: "10000"  # Minimum 10,000 AXX
  commission_rate: 0.10  # 10% commission

consensus:
  block_time: 5s
  min_validator_stake: "10000"

api:
  enabled: true
  listen_addr: "127.0.0.1"  # Bind เฉพาะ localhost (ปลอดภัย)
  rpc_port: 8545
  ws_port: 8546

telemetry:
  enabled: true
  prometheus_port: 9090
  metrics_addr: "0.0.0.0:9090"
```

### 4. Set Environment Variables
```bash
# Add to ~/.bashrc
cat >> ~/.bashrc << 'EOF'
# Axionax Environment
export AXIONAX_HOME="$HOME/.axionax"
export AXIONAX_CONFIG="$AXIONAX_HOME/config/config.yaml"
export AXIONAX_KEYSTORE="$AXIONAX_HOME/keystore"
export RUST_LOG=info
export RUST_BACKTRACE=1

# Python DeAI
export PYTHONPATH="$HOME/axionax-core/deai:$PYTHONPATH"
EOF

# Reload
source ~/.bashrc
```

---

## Genesis Setup

### 1. รอ Genesis File จาก Coordinator
Genesis file จะถูกสร้างโดย **Genesis Coordinator** และแจกจ่ายให้กับ validators ทั้งหมด

```bash
# Download genesis.json (URL จะได้หลัง genesis ceremony)
wget https://testnet.axionax.org/genesis.json -O ~/.axionax/config/genesis.json

# Verify genesis hash
sha256sum ~/.axionax/config/genesis.json
# ต้องตรงกับ hash ที่ประกาศอย่างเป็นทางการ
```

### 2. ตรวจสอบ Genesis File
```bash
# Check genesis content
cat ~/.axionax/config/genesis.json | jq '.'

# Verify your validator is included
cat ~/.axionax/config/genesis.json | jq '.validators[] | select(.address=="YOUR_VALIDATOR_ADDRESS")'
```

### 3. Initialize Node with Genesis
```bash
# Initialize blockchain state from genesis
axionax-core init --config ~/.axionax/config/config.yaml \
                  --genesis ~/.axionax/config/genesis.json

# Verify initialization
ls -lh ~/.axionax/data/
```

---

## เริ่มต้น Validator Node

### 1. Create Systemd Service
```bash
# Create service file
sudo nano /etc/systemd/system/axionax-validator.service
```

เพิ่มเนื้อหา:
```ini
[Unit]
Description=Axionax Validator Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=axionax
Group=axionax
WorkingDirectory=/home/axionax/axionax-core

# Environment
Environment="AXIONAX_HOME=/home/axionax/.axionax"
Environment="RUST_LOG=info"
Environment="RUST_BACKTRACE=1"

# Start command
ExecStart=/usr/local/bin/axionax-core start \
    --config /home/axionax/.axionax/config/config.yaml \
    --validator

# Restart policy
Restart=always
RestartSec=10
LimitNOFILE=65535

# Logging
StandardOutput=append:/home/axionax/.axionax/logs/validator.log
StandardError=append:/home/axionax/.axionax/logs/validator.error.log

[Install]
WantedBy=multi-user.target
```

### 2. Enable and Start Service
```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable service (start on boot)
sudo systemctl enable axionax-validator

# Start validator
sudo systemctl start axionax-validator

# Check status
sudo systemctl status axionax-validator

# Follow logs
journalctl -u axionax-validator -f
```

### 3. Verify Node Operation
```bash
# Check if node is running
ps aux | grep axionax-core

# Check listening ports
sudo netstat -tulnp | grep axionax

# Test RPC endpoint
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'

# Check peer count
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'

# Check validator status
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getValidatorInfo","params":["YOUR_VALIDATOR_ADDRESS"],"id":1}'
```

---

## Monitoring & Maintenance

### 1. Log Monitoring
```bash
# Real-time logs
journalctl -u axionax-validator -f

# Last 100 lines
journalctl -u axionax-validator -n 100

# Logs by date
journalctl -u axionax-validator --since "2025-01-01" --until "2025-01-02"

# Check for errors
journalctl -u axionax-validator | grep -i error

# Application logs
tail -f ~/.axionax/logs/validator.log
tail -f ~/.axionax/logs/validator.error.log
```

### 2. Metrics & Prometheus
```bash
# Check Prometheus metrics endpoint
curl http://localhost:9090/metrics

# Key metrics to monitor:
# - axionax_block_height
# - axionax_peer_count
# - axionax_validator_stake
# - axionax_consensus_participation
# - axionax_missed_blocks
```

### 3. Health Checks
Create monitoring script:
```bash
nano ~/monitor_validator.sh
```

```bash
#!/bin/bash
# Validator Health Check Script

VALIDATOR_ADDRESS="YOUR_VALIDATOR_ADDRESS"
RPC_URL="http://127.0.0.1:8545"

echo "=== Axionax Validator Health Check ==="
echo "Time: $(date)"

# 1. Check if process is running
if pgrep -x "axionax-core" > /dev/null; then
    echo "✓ Validator process is running"
else
    echo "✗ Validator process is NOT running"
    exit 1
fi

# 2. Check RPC connectivity
if curl -s -X POST $RPC_URL \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' > /dev/null; then
    echo "✓ RPC endpoint is responsive"
else
    echo "✗ RPC endpoint is NOT responsive"
fi

# 3. Check peer count
PEER_COUNT=$(curl -s -X POST $RPC_URL \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}' | jq -r '.result' | xargs printf "%d\n")
echo "✓ Connected peers: $PEER_COUNT"

# 4. Check sync status
SYNCING=$(curl -s -X POST $RPC_URL \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}' | jq -r '.result')
if [ "$SYNCING" = "false" ]; then
    echo "✓ Node is fully synced"
else
    echo "⚠ Node is syncing..."
fi

# 5. Check validator status
VALIDATOR_INFO=$(curl -s -X POST $RPC_URL \
    -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"method\":\"axn_getValidatorInfo\",\"params\":[\"$VALIDATOR_ADDRESS\"],\"id\":1}")
echo "✓ Validator info: $VALIDATOR_INFO"

echo "=== Health Check Complete ==="
```

```bash
chmod +x ~/monitor_validator.sh

# Run manually
~/monitor_validator.sh

# Or setup cron (every 5 minutes)
crontab -e
# Add: */5 * * * * /home/axionax/monitor_validator.sh >> /home/axionax/.axionax/logs/health.log 2>&1
```

### 4. Backup Strategy
```bash
# Backup keystore (CRITICAL!)
tar -czf ~/axionax-keystore-backup-$(date +%Y%m%d).tar.gz ~/.axionax/keystore/
# Transfer to secure offline storage

# Backup config
cp ~/.axionax/config/config.yaml ~/axionax-config-backup-$(date +%Y%m%d).yaml

# Backup full node data (optional, large file)
# tar -czf ~/axionax-data-backup-$(date +%Y%m%d).tar.gz ~/.axionax/data/
```

### 5. Update Node
```bash
# Stop validator
sudo systemctl stop axionax-validator

# Pull latest code
cd ~/axionax-core
git pull origin main

# Rebuild
cargo build --release

# Update binary
sudo cp target/release/axionax-core /usr/local/bin/

# Restart validator
sudo systemctl start axionax-validator

# Verify
sudo systemctl status axionax-validator
```

### 6. Common Maintenance Commands
```bash
# Restart validator
sudo systemctl restart axionax-validator

# Stop validator
sudo systemctl stop axionax-validator

# View service logs
journalctl -u axionax-validator -f

# Check disk space
df -h

# Check memory usage
free -h

# Check CPU usage
top

# Network connections
sudo netstat -anp | grep axionax
```

---

## 🚨 Emergency Procedures

### Node Won't Start
```bash
# 1. Check logs for errors
journalctl -u axionax-validator -n 100 --no-pager

# 2. Verify config syntax
axionax-core config validate --config ~/.axionax/config/config.yaml

# 3. Check file permissions
ls -la ~/.axionax/keystore/
ls -la ~/.axionax/config/

# 4. Verify genesis file integrity
sha256sum ~/.axionax/config/genesis.json

# 5. Try manual start (debug mode)
RUST_LOG=debug axionax-core start --config ~/.axionax/config/config.yaml --validator
```

### Lost Connection to Network
```bash
# 1. Check firewall
sudo ufw status

# 2. Check P2P port
sudo netstat -tulnp | grep 30303

# 3. Test connectivity to bootstrap nodes
nc -zv BOOTSTRAP_NODE_IP 30303

# 4. Restart network stack
sudo systemctl restart networking
```

### Validator Being Slashed
```bash
# Check missed blocks
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getValidatorStats","params":["YOUR_VALIDATOR_ADDRESS"],"id":1}'

# Check for false PASS accusations
tail -f ~/.axionax/logs/validator.log | grep -i "slash\|penalty\|fraud"
```

---

## 📞 Support & Resources

- **Documentation**: https://docs.axionax.org
- **Discord**: https://discord.gg/axionax
- **Telegram**: https://t.me/axionax
- **GitHub**: https://github.com/axionaxprotocol/axionax-core
- **Email**: validators@axionax.org

---

## ✅ Pre-Launch Checklist

- [ ] VPS meets minimum hardware requirements
- [ ] All dependencies installed correctly
- [ ] Axionax node built from source (release mode)
- [ ] Validator keys generated and backed up securely
- [ ] Configuration file customized for your validator
- [ ] Genesis file downloaded and verified
- [ ] Node initialized with genesis
- [ ] Systemd service configured and enabled
- [ ] Firewall configured correctly
- [ ] Monitoring script setup
- [ ] Backup strategy in place
- [ ] Coordinator notified of validator readiness
- [ ] Validator address registered in genesis

---

**Next Steps**: After completing this setup, proceed to [GENESIS_CEREMONY.md](./GENESIS_CEREMONY.md) สำหรับขั้นตอนการเข้าร่วม Genesis Launch
