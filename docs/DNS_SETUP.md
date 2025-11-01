# DNS Setup Guide for Axionax Testnet

This guide explains how to configure DNS records for all Axionax testnet services.

## 📋 Overview

You need to set up several subdomains under `axionax.org` to host different testnet services. All subdomains will point to your VPS server(s).

## 🌐 Required Subdomains

### Production Testnet Services

| Subdomain | Purpose | Port | SSL |
|-----------|---------|------|-----|
| `testnet-rpc.axionax.org` | JSON-RPC HTTP endpoint | 8545 | ✅ Yes |
| `testnet-ws.axionax.org` | WebSocket endpoint | 8546 | ✅ Yes |
| `testnet-explorer.axionax.org` | Blockscout block explorer | 4000 | ✅ Yes |
| `testnet-faucet.axionax.org` | Token faucet service | 3000 | ✅ Yes |

### Optional Services

| Subdomain | Purpose | Port | SSL |
|-----------|---------|------|-----|
| `docs.axionax.org` | Documentation site (GitHub Pages) | - | ✅ Yes |
| `api.axionax.org` | REST API (if needed) | 8080 | ✅ Yes |
| `status.axionax.org` | Network status page | 80 | ✅ Yes |

## 🔧 DNS Configuration

### Option 1: All Services on One VPS

If you're running all services on a single VPS:

```
Type    Name                Value               TTL     Proxy
A       testnet-rpc         YOUR_VPS_IP         Auto    ❌ DNS Only
A       testnet-ws          YOUR_VPS_IP         Auto    ❌ DNS Only
A       testnet-explorer    YOUR_VPS_IP         Auto    ✅ Proxied (optional)
A       testnet-faucet      YOUR_VPS_IP         Auto    ✅ Proxied (optional)
```

**Example with IP `203.0.113.42`:**
```
A       testnet-rpc         203.0.113.42        Auto    ❌
A       testnet-ws          203.0.113.42        Auto    ❌
A       testnet-explorer    203.0.113.42        Auto    ✅
A       testnet-faucet      203.0.113.42        Auto    ✅
```

### Option 2: Multiple VPS Servers

If you're distributing services across multiple servers:

```
Type    Name                Value               TTL     Notes
A       testnet-rpc         RPC_SERVER_IP       Auto    Dedicated RPC node
A       testnet-ws          RPC_SERVER_IP       Auto    Same as RPC
A       testnet-explorer    EXPLORER_IP         Auto    Separate server
A       testnet-faucet      FAUCET_IP           Auto    Separate server
```

### Option 3: Using CNAME Records

If you want flexibility to change IPs later:

```
# First, create a base A record
A       testnet-base        YOUR_VPS_IP         Auto

# Then create CNAMEs pointing to it
CNAME   testnet-rpc         testnet-base.axionax.org    Auto
CNAME   testnet-ws          testnet-base.axionax.org    Auto
CNAME   testnet-explorer    testnet-base.axionax.org    Auto
CNAME   testnet-faucet      testnet-base.axionax.org    Auto
```

## 📝 Step-by-Step Setup (Cloudflare)

### 1. Login to Cloudflare Dashboard

Go to https://dash.cloudflare.com and select your domain `axionax.org`

### 2. Navigate to DNS Settings

Click on **DNS** in the left sidebar

### 3. Add A Records

For each subdomain:

1. Click **Add record**
2. Type: `A`
3. Name: `testnet-rpc` (for example)
4. IPv4 address: `YOUR_VPS_IP`
5. Proxy status: 
   - RPC/WS: **DNS only** (gray cloud) ← Important for WebSocket
   - Explorer/Faucet: **Proxied** (orange cloud) ← DDoS protection
6. TTL: **Auto**
7. Click **Save**

### 4. Repeat for All Subdomains

```
✓ testnet-rpc.axionax.org       → YOUR_VPS_IP (DNS only)
✓ testnet-ws.axionax.org        → YOUR_VPS_IP (DNS only)
✓ testnet-explorer.axionax.org  → YOUR_VPS_IP (Proxied)
✓ testnet-faucet.axionax.org    → YOUR_VPS_IP (Proxied)
```

### 5. Verify DNS Propagation

```bash
# Check if DNS records are live
dig testnet-rpc.axionax.org +short
dig testnet-ws.axionax.org +short
dig testnet-explorer.axionax.org +short
dig testnet-faucet.axionax.org +short

# Or use nslookup (Windows)
nslookup testnet-rpc.axionax.org
```

Expected output: Your VPS IP address

## 🔐 SSL Certificate Setup

Our deployment scripts automatically handle SSL certificates using Let's Encrypt.

### Automatic SSL (Recommended)

When you run the deployment scripts, Certbot will automatically:

1. Verify domain ownership
2. Issue SSL certificates
3. Configure Nginx
4. Set up auto-renewal

**Requirements:**
- DNS records must be pointing to your server
- Ports 80 and 443 must be open
- No other service using ports 80/443

### Manual SSL Setup (if needed)

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx

# Get certificate for RPC
sudo certbot --nginx -d testnet-rpc.axionax.org

# Get certificate for WebSocket
sudo certbot --nginx -d testnet-ws.axionax.org

# Get certificate for Explorer
sudo certbot --nginx -d testnet-explorer.axionax.org

# Get certificate for Faucet
sudo certbot --nginx -d testnet-faucet.axionax.org

# Test auto-renewal
sudo certbot renew --dry-run
```

## 🔥 Firewall Configuration

Open required ports on your VPS:

```bash
# Ubuntu/Debian (ufw)
sudo ufw allow 22/tcp      # SSH
sudo ufw allow 80/tcp      # HTTP (for Let's Encrypt)
sudo ufw allow 443/tcp     # HTTPS
sudo ufw allow 8545/tcp    # RPC (if not behind nginx)
sudo ufw allow 8546/tcp    # WebSocket (if not behind nginx)
sudo ufw allow 30303/tcp   # P2P
sudo ufw allow 30303/udp   # P2P
sudo ufw enable

# CentOS/RHEL (firewalld)
sudo firewall-cmd --permanent --add-service=ssh
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --permanent --add-port=8545/tcp
sudo firewall-cmd --permanent --add-port=8546/tcp
sudo firewall-cmd --permanent --add-port=30303/tcp
sudo firewall-cmd --permanent --add-port=30303/udp
sudo firewall-cmd --reload
```

## ✅ Verification Checklist

After DNS setup, verify each service:

### 1. DNS Resolution
```bash
# All should return your VPS IP
dig testnet-rpc.axionax.org +short
dig testnet-ws.axionax.org +short
dig testnet-explorer.axionax.org +short
dig testnet-faucet.axionax.org +short
```

### 2. SSL Certificates
```bash
# Check if HTTPS works
curl -I https://testnet-rpc.axionax.org
curl -I https://testnet-explorer.axionax.org
curl -I https://testnet-faucet.axionax.org
```

### 3. Service Availability
```bash
# RPC endpoint
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' \
  https://testnet-rpc.axionax.org

# WebSocket (use wscat)
wscat -c wss://testnet-ws.axionax.org

# Explorer (should return HTML)
curl https://testnet-explorer.axionax.org

# Faucet (should return HTML)
curl https://testnet-faucet.axionax.org
```

## 🚨 Common Issues

### Issue 1: DNS Not Propagating

**Symptoms:** `dig` returns no results

**Solutions:**
- Wait 5-10 minutes for DNS propagation
- Check Cloudflare dashboard for correct records
- Verify domain nameservers point to Cloudflare
- Flush local DNS cache:
  ```bash
  # Linux
  sudo systemd-resolve --flush-caches
  
  # macOS
  sudo dscacheutil -flushcache
  
  # Windows
  ipconfig /flushdns
  ```

### Issue 2: SSL Certificate Failed

**Symptoms:** Certbot returns error

**Solutions:**
- Ensure DNS records are live (wait 10 minutes after adding)
- Check ports 80/443 are open
- Verify no other service is using port 80
- Check nginx is not running on port 80 yet
- Temporarily disable Cloudflare proxy (use DNS only)

### Issue 3: WebSocket Connection Failed

**Symptoms:** WS connection times out

**Solutions:**
- Disable Cloudflare proxy for `testnet-ws.axionax.org` (must be DNS only)
- Ensure port 8546 is open
- Check nginx WebSocket configuration
- Verify Upgrade headers are set correctly

### Issue 4: Rate Limiting on Cloudflare

**Symptoms:** 429 Too Many Requests

**Solutions:**
- Use "DNS only" for RPC/WS subdomains
- Configure Cloudflare rate limiting rules
- Consider Cloudflare Enterprise for higher limits
- Use multiple RPC endpoints for load balancing

## 📊 Production Checklist

Before public launch:

- [ ] All DNS records created and verified
- [ ] SSL certificates issued for all domains
- [ ] Firewall rules configured
- [ ] Nginx reverse proxy configured
- [ ] Services tested via curl/wscat
- [ ] Rate limiting configured
- [ ] DDoS protection enabled (Cloudflare)
- [ ] Monitoring alerts set up
- [ ] Backup DNS provider configured (optional)
- [ ] CDN/proxy for static assets (optional)

## 🔄 DNS Providers

### Cloudflare (Recommended)

**Pros:**
- Free SSL certificates
- DDoS protection
- Global CDN
- Easy UI
- Fast propagation

**Cons:**
- WebSocket requires "DNS only" mode
- Rate limiting on free tier

**Setup:** https://dash.cloudflare.com

### Namecheap

**Pros:**
- Simple interface
- Good for domain registration

**Cons:**
- Slower propagation
- No built-in DDoS protection

**Setup:** https://ap.www.namecheap.com/domains/domaincontrolpanel/axionax.org/advancedns

### Route 53 (AWS)

**Pros:**
- High availability
- Programmable (API/CLI)
- Health checks
- Traffic routing

**Cons:**
- Pay-per-query
- Complex UI

**Setup:** https://console.aws.amazon.com/route53

## 📚 Next Steps

After DNS setup:

1. **Deploy RPC Node**: Run `scripts/setup_rpc_node.sh`
2. **Deploy Block Explorer**: Run `scripts/setup_explorer.sh`
3. **Deploy Faucet**: Run `scripts/setup_faucet.sh`
4. **Test All Services**: Use verification checklist above
5. **Update Website**: Point dashboard to live endpoints
6. **Announce Launch**: Social media, Discord, blog post

## 🆘 Need Help?

- **GitHub Issues**: https://github.com/axionaxprotocol/axionax-core/issues
- **Email**: support@axionax.org
- **Documentation**: See deployment scripts in `scripts/` directory

## 📄 License

This documentation is part of Axionax Protocol, licensed under AGPLv3.

---

**Ready to deploy?** Follow the [VPS Validator Setup Guide](../docs/VPS_VALIDATOR_SETUP.md) and [Testnet Launch Guide](../docs/TESTNET_LAUNCH.md).
