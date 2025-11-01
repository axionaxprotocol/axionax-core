# Testnet Launch Preparation Summary
**สร้างเมื่อ**: 2025-01-01  
**สถานะ**: ✅ เอกสารและเครื่องมือพร้อมใช้งาน

## 📚 เอกสารที่สร้าง

### 1. VPS_VALIDATOR_SETUP.md
**ไฟล์**: `docs/VPS_VALIDATOR_SETUP.md`  
**วัตถุประสงค์**: คู่มือการติดตั้ง Validator Node บน VPS แบบครบวงจร

**เนื้อหาสำคัญ**:
- ความต้องการ Hardware/Software (8 vCPU, 32GB RAM, 1TB SSD)
- การเตรียม VPS (Ubuntu 22.04, Firewall, User management)
- ติดตั้ง Dependencies (Rust, Python, Node.js)
- Build Axionax Node จาก source
- สร้างและจัดการ Validator Keys
- ตั้งค่า Configuration files
- Setup Systemd service
- Monitoring & Maintenance (Logs, Metrics, Health checks)
- Emergency procedures และ Troubleshooting
- Pre-launch checklist

**จำนวนหน้า**: ~400 บรรทัด  
**เหมาะสำหรับ**: Validators ที่จะรัน node บน VPS

---

### 2. GENESIS_CEREMONY.md
**ไฟล์**: `docs/GENESIS_CEREMONY.md`  
**วัตถุประสงค์**: ขั้นตอนการจัด Genesis Ceremony และ Launch เครือข่าย

**เนื้อหาสำคัญ**:
- Overview ของ Genesis Ceremony
- Roles & Responsibilities (Coordinator, Validators, Community)
- Timeline 4 phases:
  * Phase 1: Preparation (2-3 weeks)
  * Phase 2: Genesis Creation (1 week)
  * Phase 3: Launch Day
  * Phase 4: Post-Launch (24 hours)
- Genesis Configuration:
  * Network parameters (Chain ID 86137, Block time 5s)
  * Token allocations (1B AXX total supply)
  * Pre-deployed contracts
- Validator Registration process
- Launch Sequence (T-15min to T+1hr)
- Post-Launch Validation
- Emergency Procedures
- Success Criteria

**จำนวนหน้า**: ~650 บรรทัด  
**เหมาะสำหรับ**: Genesis Coordinator และ Core Team

---

### 3. TESTNET_LAUNCH.md
**ไฟล์**: `docs/TESTNET_LAUNCH.md`  
**วัตถุประสงค์**: Master Guide สำหรับการ Launch Testnet ครบวงจร

**เนื้อหาสำคัญ**:
- Overview (Objectives, Network specifications)
- Infrastructure Requirements:
  * 5-10 Validator nodes (geographic distribution)
  * Public RPC endpoints (Load balanced)
  * Block Explorer (Blockscout)
  * Faucet Service
  * Monitoring Stack (Prometheus, Grafana)
- Pre-Launch Checklist (3 weeks, 2 weeks, 1 week, Launch day)
- Deployment Architecture (Network topology diagram)
- Step-by-Step Launch Process (6 phases):
  1. Validator Deployment (D-7)
  2. Genesis Creation (D-3)
  3. Validator Initialization (D-1)
  4. Launch Sequence (D-Day)
  5. Public Services (T+30min)
  6. Public Announcement (T+1hr)
- Monitoring & Observability:
  * Key metrics (Blockchain, Node, Validator)
  * Grafana dashboards
  * Alert configuration
- Security Considerations:
  * Key management
  * Server hardening
  * Network security
  * Application security
- Post-Launch Operations (24 hours, Week 1, Ongoing)
- Troubleshooting (Common issues and solutions)

**จำนวนหน้า**: ~800 บรรทัด  
**เหมาะสำหรับ**: Project Manager, DevOps Team, Validators

---

## 🛠️ Scripts และ Tools

### 1. setup_validator.sh
**ไฟล์**: `scripts/setup_validator.sh`  
**ประเภท**: Bash Script (Automated setup)

**ฟังก์ชัน**:
- Update system packages
- Create dedicated user (axionax)
- Configure firewall (UFW)
- Install Rust (nightly toolchain)
- Install Node.js 18 LTS
- Clone repository from GitHub
- Build Axionax node (release mode)
- Setup Python environment
- Initialize directory structure
- Configure environment variables

**การใช้งาน**:
```bash
# Download and run
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/setup_validator.sh
sudo bash setup_validator.sh
```

**ระยะเวลา**: ~15-20 นาที (ขึ้นกับความเร็ว VPS)

---

### 2. setup_systemd.sh
**ไฟล์**: `scripts/setup_systemd.sh`  
**ประเภท**: Bash Script (Service configuration)

**ฟังก์ชัน**:
- สร้าง systemd service file
- Configure auto-restart policy
- Setup logging to files
- Enable security restrictions
- Enable service (start on boot)

**การใช้งาน**:
```bash
sudo bash ~/axionax-core/scripts/setup_systemd.sh
sudo systemctl start axionax-validator
```

**Service Management**:
```bash
sudo systemctl start|stop|restart|status axionax-validator
journalctl -u axionax-validator -f
```

---

### 3. create_genesis.py
**ไฟล์**: `tools/create_genesis.py`  
**ประเภท**: Python Script (Genesis generator)

**ฟังก์ชัน**:
- Load validators from JSON file
- Load token allocations
- Set genesis timestamp
- Configure network parameters
- Validate genesis configuration
- Calculate genesis hash
- Save genesis.json

**การใช้งาน**:
```bash
cd ~/axionax-core/tools
python3 create_genesis.py validators.json allocations.json
```

**Output**:
- `genesis.json` - Final genesis file
- Genesis hash (SHA-256)

**Features**:
- Support vesting schedules
- Pre-deploy smart contracts
- Validate total supply
- Multiple validators support

---

### 4. validators.example.json
**ไฟล์**: `tools/validators.example.json`  
**ประเภท**: JSON Template

**เนื้อหา**:
- Example validator entries (5 validators)
- Required fields:
  * name, address, operator
  * stake, commission
  * enode, location, hardware
- Ready to customize

**การใช้งาน**:
```bash
cp validators.example.json validators.json
nano validators.json  # Edit with real data
```

---

## 🎯 การใช้งานทั้งหมด

### สำหรับ Validators

1. **Setup VPS**:
   ```bash
   wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/setup_validator.sh
   sudo bash setup_validator.sh
   ```

2. **Generate Keys**:
   ```bash
   su - axionax
   axionax-core keys generate --output ~/.axionax/keystore/validator.json
   ```

3. **Submit Info to Coordinator**:
   - ส่งข้อมูล validator (address, enode, contact)

4. **Wait for Genesis**:
   ```bash
   wget https://testnet.axionax.org/genesis.json -O ~/.axionax/config/genesis.json
   ```

5. **Initialize & Start**:
   ```bash
   axionax-core init --config ~/.axionax/config/config.yaml --genesis ~/.axionax/config/genesis.json
   sudo bash ~/axionax-core/scripts/setup_systemd.sh
   sudo systemctl start axionax-validator
   ```

### สำหรับ Genesis Coordinator

1. **Collect Validator Info**:
   - รวบรวม validator submissions
   - สร้าง `validators.json`

2. **Generate Genesis**:
   ```bash
   cd ~/axionax-core/tools
   python3 create_genesis.py validators.json allocations.json
   ```

3. **Distribute Genesis**:
   ```bash
   # Upload to public server
   scp genesis.json user@testnet.axionax.org:/var/www/html/
   
   # Upload to GitHub
   gh release create v1.6.0-genesis genesis.json
   
   # Pin to IPFS
   ipfs add genesis.json
   ```

4. **Announce**:
   - Publish genesis hash
   - Share download links
   - Set launch time

5. **Coordinate Launch**:
   - Follow timeline in GENESIS_CEREMONY.md
   - Monitor all validators
   - Execute launch sequence

---

## 📊 Network Specifications

```yaml
Network Name: Axionax Testnet
Chain ID: 86137
Consensus: PoPC (Proof-of-Probabilistic-Checking)
Block Time: 5 seconds
Epoch Length: 100 blocks
Min Validator Stake: 10,000 AXX
Max Validators: 100
Initial Validators: 5-10 nodes
Total Supply: 1,000,000,000 AXX

Token Allocations:
- Foundation: 300M AXX (30%)
- Rewards Pool: 250M AXX (25%)
- Community: 200M AXX (20%)
- Team: 150M AXX (15%)
- Public: 100M AXX (10%)

Public Services:
- RPC: https://testnet-rpc.axionax.org
- Explorer: https://testnet-explorer.axionax.org
- Faucet: https://testnet-faucet.axionax.org
```

---

## 🔐 Security Best Practices

### Validator Key Management
- ✅ Generate keys on validator machine (not coordinator)
- ✅ Backup validator.json encrypted (offline storage)
- ✅ Never share private key or mnemonic
- ✅ Use hardware security module (HSM) for production

### Server Security
- ✅ Disable password SSH (key-only)
- ✅ Configure firewall (allow only P2P ports)
- ✅ Regular security updates
- ✅ fail2ban for brute-force protection
- ✅ Monitoring and alerting

### Network Security
- ✅ DDoS protection for public endpoints
- ✅ Rate limiting on RPC/API
- ✅ HTTPS only (SSL certificates)
- ✅ CORS configured properly

---

## 📈 Success Criteria

Launch ถือว่าสำเร็จเมื่อ:
- ✅ All validators online (5-10 nodes, 100% active)
- ✅ Blocks produced consistently (>720 in first hour)
- ✅ No chain forks or consensus issues
- ✅ Average block time: 5-6 seconds
- ✅ Validator participation: >95%
- ✅ Public RPC endpoints accessible
- ✅ Block explorer syncing correctly
- ✅ Faucet operational
- ✅ No security incidents
- ✅ Community able to connect and transact

---

## 🚀 Next Steps

### ก่อน Launch (3 สัปดาห์)
1. [ ] ประกาศวันที่ Launch
2. [ ] เปิดรับสมัคร Validators
3. [ ] Provision VPS infrastructure
4. [ ] Setup DNS และ SSL certificates
5. [ ] เตรียม monitoring stack

### สัปดาห์ที่ 2
6. [ ] Validators setup nodes
7. [ ] Collect validator information
8. [ ] Create genesis.json draft
9. [ ] Conduct dry run test

### สัปดาห์ที่ 3
10. [ ] Finalize genesis.json
11. [ ] Distribute to validators
12. [ ] Validators initialize nodes
13. [ ] Final readiness check

### Launch Day
14. [ ] Execute launch sequence
15. [ ] Monitor consensus
16. [ ] Enable public services
17. [ ] Public announcement

---

## 📞 Support

**สำหรับ Validators**:
- Email: validators@axionax.org
- Discord: #validator-support
- Telegram: Validators Group (private)

**เอกสารเพิ่มเติม**:
- VPS Setup: `docs/VPS_VALIDATOR_SETUP.md`
- Genesis Ceremony: `docs/GENESIS_CEREMONY.md`
- Launch Guide: `docs/TESTNET_LAUNCH.md`
- API Reference: `docs/API_REFERENCE.md`
- Architecture: `docs/ARCHITECTURE.md`

**GitHub Repository**:
- https://github.com/axionaxprotocol/axionax-core

---

## ✅ สรุป

เอกสารและเครื่องมือครบชุดสำหรับ **Testnet Public Launch** พร้อมใช้งานแล้ว:

✅ **3 เอกสารหลัก** (2,500+ บรรทัดรวม):
- VPS_VALIDATOR_SETUP.md - คู่มือติดตั้ง validator
- GENESIS_CEREMONY.md - ขั้นตอน genesis ceremony
- TESTNET_LAUNCH.md - แผนการ launch ครบวงจร

✅ **4 Scripts/Tools**:
- setup_validator.sh - Automated VPS setup
- setup_systemd.sh - Systemd service config
- create_genesis.py - Genesis generator
- validators.example.json - Validator template

✅ **พร้อม Launch เครือข่าย**:
- Chain ID 86137 (Axionax Testnet)
- 5-10 validators เริ่มต้น
- Block time 5 วินาที
- PoPC consensus
- Public RPC, Explorer, Faucet

**เริ่มต้นได้เลย! 🚀**
