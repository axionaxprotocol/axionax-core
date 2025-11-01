# Domain & Subdomain Setup Summary

## 📋 Quick Reference

### Main Domain (Already Registered)
```
✅ axionax.org - Main website
```

### Subdomains Needed (No New Registration Required)

You **DO NOT** need to register new domains. Just add DNS records for these subdomains:

#### Testnet Services
```
testnet-rpc.axionax.org          → Your VPS IP
testnet-ws.axionax.org           → Your VPS IP
testnet-explorer.axionax.org     → Your VPS IP
testnet-faucet.axionax.org       → Your VPS IP
```

#### Optional Services
```
docs.axionax.org                 → GitHub Pages or VPS IP
api.axionax.org                  → API server IP
status.axionax.org               → Status page IP
```

## 🚀 Quick Setup (Cloudflare Example)

1. **Login to Cloudflare**
   - Go to https://dash.cloudflare.com
   - Select `axionax.org`

2. **Click DNS**

3. **Add Records** (for each subdomain):
   ```
   Type: A
   Name: testnet-rpc
   IPv4: YOUR_VPS_IP (e.g., 203.0.113.42)
   Proxy: DNS only (gray cloud) ← Important for RPC/WS
   TTL: Auto
   ```

4. **Repeat for all subdomains**

## ⚠️ Important Notes

### Cloudflare Proxy Settings

- **RPC & WebSocket**: Must be "DNS only" (gray cloud)
  - Reason: WebSocket needs direct connection
  - Subdomains: `testnet-rpc`, `testnet-ws`

- **Explorer & Faucet**: Can be "Proxied" (orange cloud)
  - Reason: Benefits from DDoS protection
  - Subdomains: `testnet-explorer`, `testnet-faucet`

### DNS Propagation

- **Wait Time**: 5-10 minutes typically
- **Check with**: `dig testnet-rpc.axionax.org +short`
- **Expected**: Your VPS IP address

### SSL Certificates

Our deployment scripts automatically handle SSL via Let's Encrypt:
- No manual certificate management needed
- Auto-renewal configured
- Just ensure ports 80 and 443 are open

## 📝 Complete Guide

For detailed instructions, see [DNS_SETUP.md](./DNS_SETUP.md)

## 🎯 Action Items

### Before Testnet Launch:

- [ ] Provision VPS server(s)
- [ ] Get VPS IP address(es)
- [ ] Add DNS A records for all subdomains
- [ ] Wait for DNS propagation (5-10 mins)
- [ ] Run deployment scripts:
  - [ ] `scripts/setup_rpc_node.sh`
  - [ ] `scripts/setup_explorer.sh`
  - [ ] `scripts/setup_faucet.sh`
- [ ] Verify SSL certificates installed
- [ ] Test all endpoints
- [ ] Update website with live URLs
- [ ] Announce testnet launch

## 💰 Cost Estimate

### Domain Costs
```
✅ Already have: axionax.org
🆓 Subdomains: FREE (unlimited)
```

### VPS Costs (Monthly)
```
Option 1: Single VPS (All services)
- DigitalOcean: $24/month (4GB RAM, 2 vCPU)
- Hetzner: €9.90/month (4GB RAM, 2 vCPU)

Option 2: Multiple VPS (Distributed)
- RPC Node: $12/month
- Explorer: $12/month
- Faucet: $6/month
Total: ~$30/month
```

### DNS/CDN
```
✅ Cloudflare Free Tier: $0/month
   - Unlimited DNS queries
   - Basic DDoS protection
   - Free SSL certificates
```

**Total Monthly Cost**: $24-30 USD for VPS only

## 🆘 Need Help?

**Questions about domains/DNS?**
- Email: support@axionax.org
- GitHub Issues: https://github.com/axionaxprotocol/axionax-core/issues
- Documentation: [DNS_SETUP.md](./DNS_SETUP.md)

## 📚 Related Documents

- [DNS_SETUP.md](./DNS_SETUP.md) - Complete DNS configuration guide
- [VPS_VALIDATOR_SETUP.md](../docs/VPS_VALIDATOR_SETUP.md) - VPS server setup
- [TESTNET_LAUNCH.md](../docs/TESTNET_LAUNCH.md) - Launch procedure
- [GETTING_STARTED.md](../GETTING_STARTED.md) - Development setup

---

**Summary**: You don't need to register any new domains. Just add subdomain DNS records pointing to your VPS, wait 5-10 minutes, and run the deployment scripts. Everything else is automated! 🚀
