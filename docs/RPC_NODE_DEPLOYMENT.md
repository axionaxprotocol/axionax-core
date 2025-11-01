# RPC Node Deployment Guide

Quick guide for deploying Axionax RPC nodes for public testnet access.

## 📋 Prerequisites

- **VPS/Server**: 4 vCPU, 16GB RAM, 500GB SSD minimum
- **OS**: Ubuntu 22.04 LTS or later
- **Network**: Public IP address, open ports 80, 443, 30303
- **Domain**: DNS records for RPC endpoint (e.g., testnet-rpc.axionax.org)

## 🚀 Quick Deployment

### 1. Initial Setup

```bash
# Download setup script
wget https://raw.githubusercontent.com/axionaxprotocol/axionax-core/main/scripts/setup_rpc_node.sh

# Make executable
chmod +x setup_rpc_node.sh

# Run as root with your domain
sudo ./setup_rpc_node.sh \
  --domain testnet-rpc.axionax.org \
  --ssl-email your-email@example.com \
  --chain-id 86137
```

### 2. Add Genesis File

Before starting the node, you need the genesis file:

```bash
# Copy genesis file to data directory
sudo cp genesis.json /var/lib/axionax/genesis.json
sudo chown axionax:axionax /var/lib/axionax/genesis.json
```

### 3. Configure Bootstrap Nodes

Edit the config file to add bootstrap nodes:

```bash
sudo nano /var/lib/axionax/config.toml
```

Add validator nodes under `[network]`:

```toml
bootstrap_nodes = [
    "/ip4/192.168.1.1/tcp/30303/p2p/QmValidatorNode1",
    "/ip4/192.168.1.2/tcp/30303/p2p/QmValidatorNode2",
]
```

### 4. Start the Node

```bash
# Start service
sudo systemctl start axionax-rpc

# Check status
sudo systemctl status axionax-rpc

# View logs
sudo journalctl -u axionax-rpc -f
```

## 🧪 Testing

### Test Local RPC

```bash
curl -X POST http://localhost:8545 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'

# Expected response:
# {"jsonrpc":"2.0","result":"0x150a9","id":1}
```

### Test Public HTTPS Endpoint

```bash
curl -X POST https://testnet-rpc.axionax.org \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

### Test WebSocket

```bash
# Install wscat if needed
npm install -g wscat

# Connect to WebSocket
wscat -c wss://testnet-ws.axionax.org

# Send subscription request
{"jsonrpc":"2.0","method":"eth_subscribe","params":["newHeads"],"id":1}
```

## 📊 Monitoring

### Check Node Health

```bash
# RPC health endpoint
curl http://localhost:8545/health

# System metrics
curl http://localhost:9090/metrics
```

### Log Monitoring

```bash
# Real-time logs
sudo journalctl -u axionax-rpc -f

# Last 100 lines
sudo journalctl -u axionax-rpc -n 100

# Errors only
sudo journalctl -u axionax-rpc -p err
```

### Resource Usage

```bash
# Check CPU/Memory
htop

# Check disk usage
df -h /var/lib/axionax

# Check network
netstat -tulpn | grep axionax
```

## 🔧 Configuration Options

### Custom Ports

```bash
sudo ./setup_rpc_node.sh \
  --rpc-port 9545 \
  --ws-port 9546 \
  --domain testnet-rpc.axionax.org
```

### Custom Data Directory

```bash
sudo ./setup_rpc_node.sh \
  --data-dir /mnt/ssd/axionax \
  --domain testnet-rpc.axionax.org
```

### Different Chain ID (Mainnet)

```bash
sudo ./setup_rpc_node.sh \
  --chain-id 86150 \
  --domain rpc.axionax.org
```

## 🛡️ Security

### Firewall Rules

Already configured by setup script:
- Port 22 (SSH) - Open
- Port 80 (HTTP) - Open (redirects to HTTPS)
- Port 443 (HTTPS) - Open
- Port 30303 (P2P) - Open
- Port 8545 (RPC) - Blocked (nginx proxy only)
- Port 8546 (WebSocket) - Blocked (nginx proxy only)

### Rate Limiting

Nginx is configured with rate limiting:
- **Default**: 1000 requests/minute per IP
- **Burst**: 100 requests allowed before dropping
- Returns 429 (Too Many Requests) when exceeded

To adjust:

```bash
sudo nano /etc/nginx/sites-available/axionax-rpc
# Modify: limit_req_zone $binary_remote_addr zone=rpc_limit:10m rate=1000r/m;
sudo nginx -t
sudo systemctl reload nginx
```

### SSL Certificates

Automatically configured with Let's Encrypt. Auto-renewal enabled.

To manually renew:

```bash
sudo certbot renew
```

## 🔄 Maintenance

### Update Node Software

```bash
# Stop node
sudo systemctl stop axionax-rpc

# Update repository
cd /home/axionax/axionax-core
sudo -u axionax git pull

# Rebuild
sudo -u axionax cargo build --release --bin axionax

# Copy new binary
sudo cp target/release/axionax /usr/local/bin/

# Restart
sudo systemctl start axionax-rpc
```

### Backup State Database

```bash
# Stop node
sudo systemctl stop axionax-rpc

# Backup
sudo tar -czf axionax-backup-$(date +%Y%m%d).tar.gz /var/lib/axionax/state

# Start node
sudo systemctl start axionax-rpc
```

### Restore from Backup

```bash
# Stop node
sudo systemctl stop axionax-rpc

# Remove current state
sudo rm -rf /var/lib/axionax/state

# Restore
sudo tar -xzf axionax-backup-20250120.tar.gz -C /

# Fix permissions
sudo chown -R axionax:axionax /var/lib/axionax

# Start node
sudo systemctl start axionax-rpc
```

## 📈 Performance Tuning

### Increase File Descriptors

```bash
# Edit limits
sudo nano /etc/security/limits.conf

# Add:
axionax soft nofile 65535
axionax hard nofile 65535

# Restart node
sudo systemctl restart axionax-rpc
```

### Optimize RocksDB

Edit `/var/lib/axionax/config.toml`:

```toml
[state]
cache_size = 2048  # Increase cache (in MB)

[performance]
db_compression = true
worker_threads = 8  # Match CPU cores
```

### Enable Pruning

For RPC nodes that don't need full history:

```toml
[state]
pruning_enabled = true
pruning_keep_blocks = 100000  # Keep last 100k blocks
```

## 🚨 Troubleshooting

### Node Won't Start

```bash
# Check logs
sudo journalctl -u axionax-rpc -n 50

# Check config syntax
/usr/local/bin/axionax --config /var/lib/axionax/config.toml --check

# Check permissions
sudo ls -la /var/lib/axionax
```

### RPC Not Responding

```bash
# Check if process is running
ps aux | grep axionax

# Check if port is listening
sudo netstat -tulpn | grep 8545

# Check nginx status
sudo systemctl status nginx

# Test direct connection (bypass nginx)
curl -X POST http://localhost:8545 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}'
```

### High CPU Usage

```bash
# Check current load
top -u axionax

# Reduce worker threads
sudo nano /var/lib/axionax/config.toml
# Set: worker_threads = 4

# Disable expensive APIs
# Set: trace_enabled = false
# Set: debug_enabled = false
```

### Disk Full

```bash
# Check usage
df -h

# Enable pruning (see Performance Tuning)

# Or clean old logs
sudo journalctl --vacuum-time=7d
sudo find /var/lib/axionax/logs -name "*.log.*" -mtime +7 -delete
```

## 📞 Support

- **Documentation**: https://docs.axionax.org
- **GitHub Issues**: https://github.com/axionaxprotocol/axionax-core/issues
- **Discord**: https://discord.gg/axionax
- **Telegram**: https://t.me/axionax

## ✅ Post-Deployment Checklist

- [ ] Node is syncing with network
- [ ] RPC endpoint responds to requests
- [ ] WebSocket connections work
- [ ] SSL certificates are valid
- [ ] Firewall rules are active
- [ ] Monitoring is configured
- [ ] Backups are scheduled
- [ ] DNS records are correct
- [ ] Rate limiting is working
- [ ] Logs are being rotated
- [ ] Update website dashboard with RPC URL
- [ ] Test with MetaMask
- [ ] Announce to community

## 🔗 Next Steps

After RPC node is running:

1. **Deploy Block Explorer** (see `docs/EXPLORER_SETUP.md`)
2. **Deploy Faucet** (see `docs/FAUCET_SETUP.md`)
3. **Setup Monitoring Dashboard** (Grafana + Prometheus)
4. **Configure Alerting** (notify on downtime/errors)
5. **Load Balancing** (if deploying multiple RPC nodes)

---

**Generated**: 2025-01-20  
**Version**: 1.0  
**Tested on**: Ubuntu 22.04 LTS
