# Testnet Public Launch Guide
## Axionax v1.6 - Complete Deployment Plan

> **Document Purpose**: Master guide สำหรับการเตรียมและ launch Axionax Testnet สู่สาธารณะ

## 📑 Table of Contents
- [Overview](#overview)
- [Infrastructure Requirements](#infrastructure-requirements)
- [Pre-Launch Checklist](#pre-launch-checklist)
- [Deployment Architecture](#deployment-architecture)
- [Step-by-Step Launch Process](#step-by-step-launch-process)
- [Monitoring & Observability](#monitoring--observability)
- [Security Considerations](#security-considerations)
- [Post-Launch Operations](#post-launch-operations)
- [Troubleshooting](#troubleshooting)

---

## Overview

### Launch Objectives
1. **Decentralized Network**: Deploy 5-10 validator nodes across different geographic locations
2. **Public Accessibility**: Provide RPC endpoints, Block Explorer, and Faucet for developers
3. **Stability**: Ensure >99.9% uptime and consistent block production
4. **Security**: Implement best practices for validator key management and network security
5. **Community Engagement**: Enable smooth onboarding for early users and developers

### Network Specifications
```yaml
Network Name: Axionax Testnet
Chain ID: 86137
Consensus: PoPC (Proof-of-Probabilistic-Checking)
Block Time: 5 seconds
Initial Validators: 5-10 nodes
TPS Target: 150+ transactions/second
Token Symbol: AXX (Testnet)
```

---

## Infrastructure Requirements

### Validator Nodes (Minimum 5 nodes)

**Hardware per Node**:
- **CPU**: 8 vCPU (Intel Xeon or AMD EPYC)
- **RAM**: 32 GB DDR4
- **Storage**: 1 TB NVMe SSD
- **Network**: 1 Gbps bandwidth, low latency (<50ms inter-node)
- **OS**: Ubuntu 22.04 LTS

**Geographic Distribution** (Recommended):
- 1x Singapore (Asia)
- 1x Frankfurt (Europe)
- 1x N. Virginia (US-East)
- 1x Oregon (US-West)
- 1x Sydney (Oceania)

**Estimated Monthly Cost**: $100-150 per validator (cloud VPS)

### Infrastructure Services

#### 1. Public RPC Endpoints
- **Load Balancer**: Nginx or HAProxy
- **Backend**: 2-3 Full Nodes (non-validator)
- **Rate Limiting**: 1000 req/min per IP
- **DDoS Protection**: Cloudflare or AWS Shield

#### 2. Block Explorer
- **Blockscout**: Full-featured explorer
- **Backend**: PostgreSQL database
- **Indexer**: Real-time blockchain indexer
- **Frontend**: React-based UI

#### 3. Faucet Service
- **API**: Python Flask/FastAPI
- **Rate Limiting**: 1 request per address per day
- **Distribution**: 100 AXX per request
- **Captcha**: hCaptcha or reCAPTCHA

#### 4. Monitoring Stack
- **Metrics**: Prometheus + Grafana
- **Logs**: ELK Stack (Elasticsearch, Logstash, Kibana)
- **Alerts**: AlertManager + PagerDuty/Discord webhooks
- **Uptime**: UptimeRobot or custom health checks

---

## Pre-Launch Checklist

### 3 Weeks Before Launch

**Infrastructure Setup**:
- [ ] Provision 5-10 VPS instances
- [ ] Setup DNS records (testnet.axionax.org, rpc.axionax.org, explorer.axionax.org)
- [ ] Obtain SSL certificates (Let's Encrypt)
- [ ] Configure firewalls and security groups
- [ ] Setup monitoring and alerting

**Code & Documentation**:
- [ ] Finalize v1.6 codebase
- [ ] Audit smart contracts
- [ ] Complete API documentation
- [ ] Prepare deployment scripts
- [ ] Write operational runbooks

**Community**:
- [ ] Announce testnet launch date
- [ ] Open validator applications
- [ ] Create Discord/Telegram channels
- [ ] Prepare marketing materials

### 2 Weeks Before Launch

**Validator Onboarding**:
- [ ] Distribute VPS setup guides
- [ ] Collect validator public keys
- [ ] Verify validator node connectivity
- [ ] Conduct validator training session
- [ ] Setup emergency communication channels

**Genesis Preparation**:
- [ ] Define token allocations
- [ ] Create genesis.json draft
- [ ] Review with validators
- [ ] Finalize genesis parameters

**Testing**:
- [ ] Internal testnet trial run
- [ ] Load testing (stress test)
- [ ] Security penetration testing
- [ ] Disaster recovery drills

### 1 Week Before Launch

**Final Preparations**:
- [ ] Finalize genesis.json
- [ ] Distribute genesis file to validators
- [ ] Validators initialize nodes
- [ ] Conduct dry run launch
- [ ] Verify all services ready

**Public Announcement**:
- [ ] Publish genesis hash
- [ ] Release countdown website
- [ ] Social media campaign
- [ ] Press release (if applicable)

### Launch Day (D-Day)

**T-2 hours**:
- [ ] Final validator roll call
- [ ] Verify all systems green
- [ ] Monitoring dashboards ready
- [ ] Support team on standby

**T-30 minutes**:
- [ ] Validators in position
- [ ] Final configuration checks
- [ ] Launch coordinator ready

**T-0 (Launch Time)**:
- [ ] Execute launch sequence
- [ ] Monitor block production
- [ ] Verify consensus
- [ ] Enable public services

---

## Deployment Architecture

```
                                    Internet
                                       |
                              [Cloudflare CDN]
                                       |
                    +------------------+------------------+
                    |                  |                  |
              [Load Balancer]    [Load Balancer]   [Load Balancer]
                   RPC              Explorer           Faucet
                    |                  |                  |
        +-----------+-----------+      |                  |
        |           |           |      |                  |
    [RPC Node 1] [RPC Node 2] [RPC Node 3]              |
        |           |           |      |                  |
        +-----+-----+-----+-----+      |                  |
              |                        |                  |
        [P2P Network]            [Blockscout]       [Faucet API]
              |                    + [PostgreSQL]     + [Redis]
              |
    +---------+---------+---------+---------+
    |         |         |         |         |
[Validator1][Validator2][Validator3][Validator4][Validator5]
    |         |         |         |         |
 [Singapore] [Frankfurt] [N.Virginia] [Oregon] [Sydney]
```

### Network Topology

**Validator Network** (Private P2P):
- Full mesh connectivity between validators
- VPN or private network (optional for security)
- Direct peering for low latency

**Public Network**:
- RPC nodes expose JSON-RPC interface
- Load balanced for high availability
- Rate limited and DDoS protected

---

## Step-by-Step Launch Process

### Phase 1: Validator Deployment (D-7)

**1.1 Setup VPS for Each Validator**
```bash
# On each validator VPS
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/setup_validator.sh
sudo bash setup_validator.sh
```

**1.2 Generate Validator Keys**
```bash
su - axionax
axionax-core keys generate --output ~/.axionax/keystore/validator.json
```

**1.3 Submit Validator Information**
Each validator submits:
- Name & Operator
- Public Key & Address
- Enode URL
- Contact Information

**1.4 Configure Nodes**
```bash
nano ~/.axionax/config/config.yaml
# Set mode: "validator"
# Set validator name
# Configure network settings
```

### Phase 2: Genesis Creation (D-3)

**2.1 Collect Validator Data**
Coordinator compiles `validators.json`:
```json
[
  {
    "name": "Validator-01",
    "address": "0x...",
    "stake": "50000000000000000000000",
    "commission": 0.10,
    "enode": "enode://..."
  }
]
```

**2.2 Generate Genesis**
```bash
cd ~/axionax-core/tools
python3 create_genesis.py validators.json allocations.json
```

**2.3 Review & Finalize**
- Validators review genesis.json
- Verify parameters correct
- Sign-off from all validators

**2.4 Distribute Genesis**
```bash
# Upload to public location
scp genesis.json user@testnet.axionax.org:/var/www/html/
# Upload to GitHub release
gh release create v1.6.0-genesis genesis.json
# Pin to IPFS
ipfs add genesis.json
```

**2.5 Announce Genesis Hash**
```
🎉 Genesis Ready!
Hash: 0xabcd1234...
Download: https://testnet.axionax.org/genesis.json
IPFS: QmXxXxXx...
Launch: 2025-01-15 00:00:00 UTC
```

### Phase 3: Validator Initialization (D-1)

**3.1 Download Genesis**
```bash
# All validators
wget https://testnet.axionax.org/genesis.json -O ~/.axionax/config/genesis.json
```

**3.2 Verify Genesis Hash**
```bash
sha256sum ~/.axionax/config/genesis.json
# Must match announced hash
```

**3.3 Initialize Node**
```bash
axionax-core init --config ~/.axionax/config/config.yaml \
                  --genesis ~/.axionax/config/genesis.json
```

**3.4 Setup Systemd Service**
```bash
sudo bash ~/axionax-core/scripts/setup_systemd.sh
sudo systemctl enable axionax-validator
```

**3.5 Dry Run Test**
```bash
# Test start (don't leave running)
sudo systemctl start axionax-validator
# Check logs
journalctl -u axionax-validator -f
# Stop after verification
sudo systemctl stop axionax-validator
```

### Phase 4: Launch Sequence (D-Day)

**4.1 Pre-Launch (T-15 min)**
- Final validator roll call
- Verify network connectivity
- Monitoring dashboards active

**4.2 Coordinator Start (T-5 min)**
```bash
# Coordinator starts first
sudo systemctl start axionax-validator
# Get enode for bootstrap
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"admin_nodeInfo","params":[],"id":1}' | jq -r '.result.enode'
```

**4.3 Share Bootstrap Enode (T-3 min)**
Coordinator shares enode with all validators

**4.4 Validators Join (T-0)**
```bash
# All validators execute simultaneously
sudo systemctl start axionax-validator
```

**4.5 Monitor Consensus (T+5 min)**
```bash
# Check block production
watch -n 1 'curl -s -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}" | jq -r ".result" | xargs printf "%d\n"'
```

**4.6 Verify All Validators Active (T+10 min)**
```bash
# Check validator participation
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getValidatorSet","params":[],"id":1}' | jq '.'
```

### Phase 5: Public Services (T+30 min)

**5.1 Start RPC Nodes**
```bash
# On RPC node servers
docker-compose -f rpc-node.yml up -d
```

**5.2 Start Block Explorer**
```bash
# On explorer server
cd ~/blockscout
docker-compose up -d
```

**5.3 Start Faucet**
```bash
# On faucet server
cd ~/faucet
docker-compose up -d
```

**5.4 Enable Public Access**
- Update DNS records (if not already done)
- Open firewall rules for public traffic
- Verify SSL certificates active

**5.5 Smoke Tests**
```bash
# Test RPC
curl https://testnet-rpc.axionax.org \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Test Explorer
curl https://testnet-explorer.axionax.org/api/v2/stats

# Test Faucet
curl 'https://testnet-faucet.axionax.org/health'
```

### Phase 6: Public Announcement (T+1 hour)

**6.1 Verify Stability**
- 720+ blocks produced (1 hour @ 5s blocks)
- All validators participating
- No consensus issues

**6.2 Publish Launch Announcement**
```markdown
🎊 Axionax Testnet is LIVE! 🎊

Network Details:
- Chain ID: 86137
- RPC: https://testnet-rpc.axionax.org
- Explorer: https://testnet-explorer.axionax.org
- Faucet: https://testnet-faucet.axionax.org

Current Stats:
- Block Height: 720+
- Validators: 5/5 (100% active)
- Avg Block Time: 5.2s
- Network Health: Excellent

Get Started:
1. Add network to MetaMask
2. Get test tokens from faucet
3. Deploy your dApp!

Documentation: https://docs.axionax.org
Discord: https://discord.gg/axionax
```

---

## Monitoring & Observability

### Key Metrics to Monitor

**Blockchain Metrics**:
- Block height (should increase steadily)
- Block time (target: 5s ± 0.5s)
- Validator participation rate (target: >99%)
- Transaction throughput (TPS)
- Pending transaction count
- Gas price trends

**Node Metrics**:
- Peer count (should be stable)
- Sync status (should be false = synced)
- Memory usage (< 80%)
- CPU usage (< 70% average)
- Disk I/O
- Network bandwidth

**Validator Metrics**:
- Blocks proposed
- Blocks missed (should be 0)
- Stake amount
- Commission earned
- Slashing events (should be 0)

### Grafana Dashboards

Create dashboards for:
1. **Network Overview**: Block production, validators, TPS
2. **Validator Performance**: Individual validator stats
3. **Node Health**: System resources, connectivity
4. **Service Availability**: RPC, Explorer, Faucet uptime

### Alerts

Configure alerts for:
- 🚨 **Critical**: Network stall (no new blocks for 1 minute)
- 🚨 **Critical**: Validator offline
- ⚠️ **Warning**: High block time variance (>8s)
- ⚠️ **Warning**: Low peer count (<3 peers)
- ⚠️ **Warning**: High memory usage (>90%)
- ℹ️ **Info**: New validator joined
- ℹ️ **Info**: Slashing event

---

## Security Considerations

### Validator Security

**Key Management**:
- ✅ Store private keys encrypted
- ✅ Backup keys to offline storage
- ✅ Never commit keys to version control
- ✅ Use hardware security modules (HSM) for production

**Server Hardening**:
- ✅ Disable password SSH (use key-only)
- ✅ Enable automatic security updates
- ✅ Configure firewall (allow only necessary ports)
- ✅ Use fail2ban for brute-force protection
- ✅ Regular security audits

**Network Security**:
- ✅ DDoS protection for public endpoints
- ✅ Rate limiting on RPC/API
- ✅ Private validator network (optional VPN)
- ✅ Monitoring for suspicious activity

### Application Security

**RPC Endpoints**:
- ✅ HTTPS only (no HTTP)
- ✅ CORS configured properly
- ✅ Rate limiting (per IP/per method)
- ✅ Request validation
- ✅ Disable sensitive methods (`admin_*`, `personal_*` for public)

**Faucet**:
- ✅ Captcha verification
- ✅ Rate limiting (per address/per IP)
- ✅ Maximum distribution limits
- ✅ Monitoring for abuse

**Smart Contracts**:
- ✅ Audited by professional auditors
- ✅ Time-locked upgrades
- ✅ Emergency pause functionality
- ✅ Bug bounty program

---

## Post-Launch Operations

### First 24 Hours

**Hourly Tasks**:
- Monitor block production
- Check validator participation
- Verify no errors in logs
- Test public endpoints
- Monitor social media for issues

**Metrics to Track**:
- Uptime: Should be 100%
- Block production: Consistent 5s intervals
- Transaction success rate: >99%
- Network latency: <200ms average

### First Week

**Daily Tasks**:
- Review metrics dashboards
- Check for anomalies
- Backup validator keys
- Update documentation as needed
- Engage with community feedback

**Improvements**:
- Optimize node performance
- Add more RPC nodes if needed
- Enhance monitoring
- Fix any discovered bugs

### Ongoing Operations

**Weekly**:
- Security audits
- Performance reviews
- Capacity planning
- Community updates

**Monthly**:
- Software updates
- Infrastructure optimization
- Cost analysis
- Feature planning

---

## Troubleshooting

### Common Issues

**Network Not Starting**:
```bash
# Check genesis hash matches
sha256sum ~/.axionax/config/genesis.json

# Verify config
axionax-core config validate --config ~/.axionax/config/config.yaml

# Check firewall
sudo ufw status

# Manual start with debug
RUST_LOG=debug axionax-core start --config ~/.axionax/config/config.yaml
```

**Validator Not Producing Blocks**:
```bash
# Check validator status
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"axn_getValidatorInfo","params":["YOUR_ADDRESS"],"id":1}'

# Verify stake
# Check connectivity to other validators
# Review logs for errors
```

**High Missed Block Rate**:
- Check system resources (CPU, memory, disk)
- Verify network connectivity and latency
- Ensure system time synchronized (NTP)
- Check for competing processes

**RPC Endpoint Not Responding**:
```bash
# Check nginx
sudo systemctl status nginx

# Check node
systemctl status axionax-validator

# Check logs
journalctl -u axionax-validator -n 100

# Test locally
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
```

---

## Additional Resources

- **VPS Setup**: [VPS_VALIDATOR_SETUP.md](./VPS_VALIDATOR_SETUP.md)
- **Genesis Ceremony**: [GENESIS_CEREMONY.md](./GENESIS_CEREMONY.md)
- **API Reference**: [API_REFERENCE.md](./API_REFERENCE.md)
- **Architecture**: [ARCHITECTURE.md](./ARCHITECTURE.md)

**Support**:
- Email: validators@axionax.org
- Discord: https://discord.gg/axionax
- Documentation: https://docs.axionax.org

---

**Good luck with your testnet launch! 🚀**
