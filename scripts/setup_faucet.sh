#!/bin/bash
#
# Axionax Faucet Setup Script
# Deploys testnet faucet application
#
# Usage: bash setup_faucet.sh [OPTIONS]
# Options:
#   --domain DOMAIN        Domain for faucet (e.g., testnet-faucet.axionax.org)
#   --ssl-email EMAIL      Email for Let's Encrypt SSL
#   --rpc-url URL         RPC endpoint URL (default: http://localhost:8545)
#   --chain-id ID         Chain ID (default: 86137)
#   --private-key KEY     Faucet wallet private key
#   --data-dir PATH       Data directory (default: /var/lib/axionax-faucet)

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Default configuration
DOMAIN=""
SSL_EMAIL=""
RPC_URL="http://localhost:8545"
CHAIN_ID=86137
PRIVATE_KEY=""
DATA_DIR="/var/lib/axionax-faucet"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --domain)
      DOMAIN="$2"
      shift 2
      ;;
    --ssl-email)
      SSL_EMAIL="$2"
      shift 2
      ;;
    --rpc-url)
      RPC_URL="$2"
      shift 2
      ;;
    --chain-id)
      CHAIN_ID="$2"
      shift 2
      ;;
    --private-key)
      PRIVATE_KEY="$2"
      shift 2
      ;;
    --data-dir)
      DATA_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

if [ -z "$DOMAIN" ]; then
  echo -e "${RED}Error: --domain is required${NC}"
  exit 1
fi

if [ -z "$SSL_EMAIL" ]; then
  echo -e "${RED}Error: --ssl-email is required${NC}"
  exit 1
fi

if [ -z "$PRIVATE_KEY" ]; then
  echo -e "${RED}Error: --private-key is required${NC}"
  exit 1
fi

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Axionax Faucet Setup${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}Configuration:${NC}"
echo "  Domain: $DOMAIN"
echo "  RPC URL: $RPC_URL"
echo "  Chain ID: $CHAIN_ID"
echo "  Data Directory: $DATA_DIR"
echo ""

# Check root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}Error: This script must be run as root${NC}"
   exit 1
fi

# Update system
echo -e "${BLUE}[1/8]${NC} Updating system..."
apt-get update -qq
apt-get upgrade -y -qq

# Install dependencies
echo -e "${BLUE}[2/8]${NC} Installing dependencies..."
apt-get install -y -qq \
  curl \
  wget \
  git \
  nginx \
  certbot \
  python3-certbot-nginx \
  build-essential \
  pkg-config \
  libssl-dev \
  ufw

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
  echo -e "${BLUE}[3/8]${NC} Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  echo -e "${BLUE}[3/8]${NC} Rust already installed"
fi

# Create faucet user
if ! id -u faucet &> /dev/null; then
  echo -e "${BLUE}[4/8]${NC} Creating faucet user..."
  useradd -r -m -s /bin/bash faucet
else
  echo -e "${BLUE}[4/8]${NC} User faucet already exists"
fi

# Create data directory
echo -e "${BLUE}[5/8]${NC} Setting up directories..."
mkdir -p "$DATA_DIR"
mkdir -p "$DATA_DIR/logs"
chown -R faucet:faucet "$DATA_DIR"

# Clone and build faucet
FAUCET_HOME="/home/faucet/axionax-core"
if [ ! -d "$FAUCET_HOME" ]; then
  echo -e "${BLUE}[6/8]${NC} Cloning Axionax repository..."
  sudo -u faucet git clone https://github.com/axionaxprotocol/axionax-core.git "$FAUCET_HOME"
else
  echo -e "${BLUE}[6/8]${NC} Updating repository..."
  cd "$FAUCET_HOME"
  sudo -u faucet git pull
fi

cd "$FAUCET_HOME/tools/faucet"

# Build faucet
echo -e "${BLUE}[7/8]${NC} Building faucet (this may take 5-10 minutes)..."
sudo -u faucet cargo build --release

# Copy binary
cp target/release/axionax-faucet /usr/local/bin/
chmod +x /usr/local/bin/axionax-faucet

# Copy frontend files
mkdir -p /var/www/faucet
cp -r public/* /var/www/faucet/
chown -R www-data:www-data /var/www/faucet

# Create environment file
cat > "$DATA_DIR/.env" <<EOF
FAUCET_PRIVATE_KEY=$PRIVATE_KEY
RPC_URL=$RPC_URL
CHAIN_ID=$CHAIN_ID
RUST_LOG=info
EOF

chmod 600 "$DATA_DIR/.env"
chown faucet:faucet "$DATA_DIR/.env"

# Create systemd service
cat > /etc/systemd/system/axionax-faucet.service <<EOF
[Unit]
Description=Axionax Testnet Faucet
After=network.target

[Service]
Type=simple
User=faucet
Group=faucet
WorkingDirectory=$DATA_DIR
EnvironmentFile=$DATA_DIR/.env
ExecStart=/usr/local/bin/axionax-faucet
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=axionax-faucet

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=true

# Resource limits
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOF

# Configure nginx
echo -e "${BLUE}[8/8]${NC} Configuring nginx..."
cat > /etc/nginx/sites-available/axionax-faucet <<EOF
# Faucet API backend
upstream faucet_api {
    server 127.0.0.1:3000;
}

# Rate limiting
limit_req_zone \$binary_remote_addr zone=faucet_limit:10m rate=10r/m;
limit_req_status 429;

server {
    listen 80;
    server_name $DOMAIN;

    # Frontend (static files)
    root /var/www/faucet;
    index index.html;

    # API requests
    location /api/ {
        # Rate limiting
        limit_req zone=faucet_limit burst=5 nodelay;
        
        proxy_pass http://faucet_api/;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

    # Direct API endpoints (backward compatibility)
    location ~ ^/(health|info|request|stats)\$ {
        limit_req zone=faucet_limit burst=5 nodelay;
        
        proxy_pass http://faucet_api;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

    # Static files
    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # Health check
    location /healthz {
        access_log off;
        return 200 'OK';
        add_header Content-Type text/plain;
    }

    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
}
EOF

# Enable site
ln -sf /etc/nginx/sites-available/axionax-faucet /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

# Test nginx
nginx -t

# Configure firewall
echo -e "${GREEN}Configuring firewall...${NC}"
ufw --force enable
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow 80/tcp
ufw allow 443/tcp

# Start services
echo -e "${GREEN}Starting services...${NC}"
systemctl daemon-reload
systemctl enable axionax-faucet
systemctl start axionax-faucet
systemctl restart nginx

# Wait for service
sleep 5

# Check if faucet is running
if curl -sf http://localhost:3000/health > /dev/null 2>&1; then
  echo -e "${GREEN}✓ Faucet service is running${NC}"
else
  echo -e "${RED}✗ Warning: Faucet service may not be running properly${NC}"
  echo -e "${YELLOW}Check logs: journalctl -u axionax-faucet -f${NC}"
fi

# Setup SSL
echo -e "${GREEN}Setting up SSL certificate...${NC}"
certbot --nginx -d "$DOMAIN" --email "$SSL_EMAIL" --agree-tos --non-interactive --redirect

# Save configuration
cat > "$DATA_DIR/config.txt" <<EOF
Axionax Faucet Configuration
============================
Domain: https://$DOMAIN
Chain ID: $CHAIN_ID
RPC URL: $RPC_URL

Data Directory: $DATA_DIR
Environment File: $DATA_DIR/.env

Installed: $(date)

⚠️  IMPORTANT: Keep the private key in .env secure!
EOF

chmod 600 "$DATA_DIR/config.txt"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Setup Complete! 🎉${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Faucet URL:${NC}"
echo "  ${BLUE}https://$DOMAIN${NC}"
echo ""
echo -e "${YELLOW}Management Commands:${NC}"
echo ""
echo "Check status:"
echo "  ${BLUE}sudo systemctl status axionax-faucet${NC}"
echo ""
echo "View logs:"
echo "  ${BLUE}sudo journalctl -u axionax-faucet -f${NC}"
echo ""
echo "Restart faucet:"
echo "  ${BLUE}sudo systemctl restart axionax-faucet${NC}"
echo ""
echo "Stop faucet:"
echo "  ${BLUE}sudo systemctl stop axionax-faucet${NC}"
echo ""
echo -e "${YELLOW}Testing:${NC}"
echo ""
echo "Health check:"
echo "  ${BLUE}curl https://$DOMAIN/health${NC}"
echo ""
echo "Get faucet info:"
echo "  ${BLUE}curl https://$DOMAIN/info${NC}"
echo ""
echo "Request tokens (example):"
echo "  ${BLUE}curl -X POST https://$DOMAIN/request \\${NC}"
echo "  ${BLUE}  -H 'Content-Type: application/json' \\${NC}"
echo "  ${BLUE}  -d '{\"address\":\"0x1234...\"}'${NC}"
echo ""
echo -e "${YELLOW}⚠️  Important:${NC}"
echo "  - Private key is stored in $DATA_DIR/.env (keep secure!)"
echo "  - Fund the faucet wallet with testnet tokens"
echo "  - Monitor faucet balance regularly"
echo "  - Rate limited to 10 requests/minute per IP"
echo "  - Users can request once every 24 hours"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Visit https://$DOMAIN and test the faucet"
echo "  2. Fund the faucet wallet address"
echo "  3. Update website with faucet link"
echo "  4. Announce to community"
echo ""
