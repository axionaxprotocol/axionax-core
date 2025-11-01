#!/bin/bash
#
# Axionax Block Explorer Setup Script
# Deploys Blockscout explorer for Axionax testnet
#
# Usage: bash setup_explorer.sh [OPTIONS]
# Options:
#   --domain DOMAIN        Domain for explorer (e.g., testnet-explorer.axionax.org)
#   --ssl-email EMAIL      Email for Let's Encrypt SSL
#   --rpc-url URL         RPC endpoint URL (default: http://localhost:8545)
#   --ws-url URL          WebSocket URL (default: ws://localhost:8546)
#   --chain-id ID         Chain ID (default: 86137)
#   --data-dir PATH       Data directory (default: /var/lib/axionax-explorer)

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
WS_URL="ws://localhost:8546"
CHAIN_ID=86137
DATA_DIR="/var/lib/axionax-explorer"
DB_PASSWORD=""

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
    --ws-url)
      WS_URL="$2"
      shift 2
      ;;
    --chain-id)
      CHAIN_ID="$2"
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

# Generate random database password
DB_PASSWORD=$(openssl rand -base64 32)

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Axionax Block Explorer Setup${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}Configuration:${NC}"
echo "  Domain: $DOMAIN"
echo "  RPC URL: $RPC_URL"
echo "  WebSocket URL: $WS_URL"
echo "  Chain ID: $CHAIN_ID"
echo "  Data Directory: $DATA_DIR"
echo ""

# Check root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}Error: This script must be run as root${NC}"
   exit 1
fi

# Update system
echo -e "${BLUE}[1/9]${NC} Updating system..."
apt-get update -qq
apt-get upgrade -y -qq

# Install dependencies
echo -e "${BLUE}[2/9]${NC} Installing dependencies..."
apt-get install -y -qq \
  curl \
  wget \
  git \
  nginx \
  certbot \
  python3-certbot-nginx \
  docker.io \
  docker-compose \
  ufw \
  postgresql \
  postgresql-contrib

# Start Docker
systemctl start docker
systemctl enable docker

# Create data directory
echo -e "${BLUE}[3/9]${NC} Creating data directories..."
mkdir -p "$DATA_DIR"
mkdir -p "$DATA_DIR/postgres-data"
mkdir -p "$DATA_DIR/redis-data"
mkdir -p "$DATA_DIR/logs"

# Setup PostgreSQL
echo -e "${BLUE}[4/9]${NC} Configuring PostgreSQL..."
sudo -u postgres psql -c "CREATE USER blockscout WITH PASSWORD '$DB_PASSWORD';" 2>/dev/null || true
sudo -u postgres psql -c "CREATE DATABASE blockscout OWNER blockscout;" 2>/dev/null || true
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE blockscout TO blockscout;" 2>/dev/null || true

# Create docker-compose.yml
echo -e "${BLUE}[5/9]${NC} Creating Docker Compose configuration..."
cat > "$DATA_DIR/docker-compose.yml" <<EOF
version: '3.8'

services:
  redis:
    image: redis:7-alpine
    container_name: axionax-redis
    restart: always
    volumes:
      - $DATA_DIR/redis-data:/data
    command: redis-server --appendonly yes
    networks:
      - explorer-network

  db:
    image: postgres:15-alpine
    container_name: axionax-postgres
    restart: always
    environment:
      POSTGRES_DB: blockscout
      POSTGRES_USER: blockscout
      POSTGRES_PASSWORD: $DB_PASSWORD
    volumes:
      - $DATA_DIR/postgres-data:/var/lib/postgresql/data
    networks:
      - explorer-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U blockscout"]
      interval: 10s
      timeout: 5s
      retries: 5

  blockscout:
    image: blockscout/blockscout:latest
    container_name: axionax-blockscout
    restart: always
    depends_on:
      - db
      - redis
    environment:
      # Network
      ETHEREUM_JSONRPC_VARIANT: geth
      ETHEREUM_JSONRPC_HTTP_URL: $RPC_URL
      ETHEREUM_JSONRPC_WS_URL: $WS_URL
      ETHEREUM_JSONRPC_TRACE_URL: $RPC_URL
      
      # Chain configuration
      CHAIN_ID: '$CHAIN_ID'
      COIN: 'AXX'
      SUBNETWORK: 'Axionax Testnet'
      NETWORK: 'Axionax'
      LOGO: '/images/axionax_logo.svg'
      LOGO_FOOTER: '/images/axionax_logo.svg'
      
      # Database
      DATABASE_URL: postgresql://blockscout:$DB_PASSWORD@db:5432/blockscout?ssl=false
      
      # Redis
      REDIS_URL: redis://redis:6379
      
      # Secret key (generate random)
      SECRET_KEY_BASE: $(openssl rand -base64 64 | tr -d '\n')
      
      # Block fetcher
      BLOCK_TRANSFORMER: clique
      FETCH_REWARDS_WAY: manual
      INDEXER_DISABLE_PENDING_TRANSACTIONS_FETCHER: 'false'
      INDEXER_DISABLE_INTERNAL_TRANSACTIONS_FETCHER: 'false'
      
      # API
      API_V2_ENABLED: 'true'
      API_RATE_LIMIT: 50
      API_RATE_LIMIT_TIME_INTERVAL: 1s
      
      # UI
      SHOW_TESTNET_LABEL: 'true'
      TESTNET_LABEL: 'TESTNET'
      SUPPORTED_CHAINS: '[{"title":"Axionax Testnet","url":"https://$DOMAIN"}]'
      
      # Social links
      FOOTER_GITHUB_LINK: 'https://github.com/axionaxprotocol'
      FOOTER_TWITTER_LINK: 'https://twitter.com/axionax'
      FOOTER_TELEGRAM_LINK: 'https://t.me/axionax'
      
      # Other
      PORT: '4000'
      HEALTHY_BLOCKS_PERIOD: 60
      INDEXER_MEMORY_LIMIT: 1
      POOL_SIZE: 30
      ECTO_USE_SSL: 'false'
      
    ports:
      - "127.0.0.1:4000:4000"
    networks:
      - explorer-network
    volumes:
      - $DATA_DIR/logs:/app/logs
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:4000/api/v1/health/liveness"]
      interval: 30s
      timeout: 10s
      retries: 3

networks:
  explorer-network:
    driver: bridge
EOF

# Configure nginx
echo -e "${BLUE}[6/9]${NC} Configuring nginx..."
cat > /etc/nginx/sites-available/axionax-explorer <<EOF
upstream blockscout {
    server 127.0.0.1:4000;
}

# Rate limiting
limit_req_zone \$binary_remote_addr zone=explorer_limit:10m rate=100r/s;
limit_req_status 429;

server {
    listen 80;
    server_name $DOMAIN;

    # Rate limiting
    limit_req zone=explorer_limit burst=200 nodelay;

    # Client body size
    client_max_body_size 10M;

    location / {
        proxy_pass http://blockscout;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_cache_bypass \$http_upgrade;
        
        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }

    # WebSocket support for live updates
    location /socket/ {
        proxy_pass http://blockscout;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host \$host;
        proxy_read_timeout 86400;
    }

    # Health check
    location /health {
        access_log off;
        return 200 'OK';
        add_header Content-Type text/plain;
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
EOF

# Enable site
ln -sf /etc/nginx/sites-available/axionax-explorer /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

# Test nginx
nginx -t

# Configure firewall
echo -e "${BLUE}[7/9]${NC} Configuring firewall..."
ufw --force enable
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow 80/tcp
ufw allow 443/tcp

# Start services
echo -e "${BLUE}[8/9]${NC} Starting services..."
systemctl restart nginx

cd "$DATA_DIR"
docker-compose up -d

# Wait for services
echo -e "${YELLOW}Waiting for services to start (this may take 2-3 minutes)...${NC}"
sleep 10

# Check if Blockscout is running
for i in {1..30}; do
  if curl -sf http://localhost:4000/api/v1/health/liveness > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Blockscout is running${NC}"
    break
  fi
  echo -n "."
  sleep 10
done

# Setup SSL
echo -e "${BLUE}[9/9]${NC} Setting up SSL certificate..."
certbot --nginx -d "$DOMAIN" --email "$SSL_EMAIL" --agree-tos --non-interactive --redirect

# Save configuration
cat > "$DATA_DIR/config.txt" <<EOF
Axionax Block Explorer Configuration
=====================================
Domain: https://$DOMAIN
Chain ID: $CHAIN_ID
RPC URL: $RPC_URL
WebSocket URL: $WS_URL
Database Password: $DB_PASSWORD

Data Directory: $DATA_DIR
Docker Compose: $DATA_DIR/docker-compose.yml

Installed: $(date)
EOF

chmod 600 "$DATA_DIR/config.txt"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Setup Complete! 🎉${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Block Explorer URL:${NC}"
echo "  ${BLUE}https://$DOMAIN${NC}"
echo ""
echo -e "${YELLOW}Management Commands:${NC}"
echo ""
echo "Start services:"
echo "  ${BLUE}cd $DATA_DIR && docker-compose up -d${NC}"
echo ""
echo "Stop services:"
echo "  ${BLUE}cd $DATA_DIR && docker-compose down${NC}"
echo ""
echo "View logs:"
echo "  ${BLUE}cd $DATA_DIR && docker-compose logs -f blockscout${NC}"
echo ""
echo "Restart services:"
echo "  ${BLUE}cd $DATA_DIR && docker-compose restart${NC}"
echo ""
echo "Check status:"
echo "  ${BLUE}cd $DATA_DIR && docker-compose ps${NC}"
echo ""
echo -e "${YELLOW}Database Information:${NC}"
echo "  User: blockscout"
echo "  Database: blockscout"
echo "  Password: (saved in $DATA_DIR/config.txt)"
echo ""
echo -e "${YELLOW}⚠️  Important:${NC}"
echo "  - Initial indexing may take 30-60 minutes"
echo "  - Check logs if blocks aren't showing: docker-compose logs -f"
echo "  - Ensure RPC endpoint is accessible from this server"
echo "  - Monitor disk usage (PostgreSQL can grow large)"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Visit https://$DOMAIN and verify explorer is working"
echo "  2. Check that blocks are being indexed"
echo "  3. Update website with explorer link"
echo "  4. Setup monitoring/alerts"
echo ""
