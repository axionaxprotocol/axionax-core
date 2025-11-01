#!/bin/bash
#
# Axionax Dependency Installer for Linux
# Supports: Ubuntu/Debian, CentOS/RHEL, Arch, Alpine
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Detect OS and package manager
detect_os() {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    VER=$VERSION_ID
  elif [ -f /etc/redhat-release ]; then
    OS="rhel"
  elif [ -f /etc/alpine-release ]; then
    OS="alpine"
  else
    OS=$(uname -s)
  fi
  
  echo "$OS"
}

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Axionax Dependency Installer${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

OS=$(detect_os)
echo -e "${GREEN}Detected OS: $OS${NC}"
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}This script must be run as root (use sudo)${NC}"
   exit 1
fi

# Common dependencies for all Linux distros
COMMON_DEPS="curl wget git build-essential pkg-config openssl"

case "$OS" in
  ubuntu|debian)
    echo -e "${BLUE}Installing dependencies for Ubuntu/Debian...${NC}"
    
    apt-get update -qq
    
    # Development tools
    apt-get install -y -qq \
      curl \
      wget \
      git \
      build-essential \
      pkg-config \
      libssl-dev \
      ca-certificates \
      gnupg \
      lsb-release
    
    # Rust (if not installed)
    if ! command -v rustc &> /dev/null; then
      echo -e "${BLUE}Installing Rust...${NC}"
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
    fi
    
    # Node.js LTS (via NodeSource)
    if ! command -v node &> /dev/null; then
      echo -e "${BLUE}Installing Node.js...${NC}"
      curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
      apt-get install -y -qq nodejs
    fi
    
    # Python 3 and pip
    apt-get install -y -qq \
      python3 \
      python3-pip \
      python3-venv
    
    # Docker
    if ! command -v docker &> /dev/null; then
      echo -e "${BLUE}Installing Docker...${NC}"
      apt-get install -y -qq \
        docker.io \
        docker-compose
      systemctl enable docker
      systemctl start docker
    fi
    
    # Nginx
    apt-get install -y -qq nginx
    
    # PostgreSQL
    apt-get install -y -qq \
      postgresql \
      postgresql-contrib
    
    # Certbot (Let's Encrypt)
    apt-get install -y -qq \
      certbot \
      python3-certbot-nginx
    
    # Firewall
    apt-get install -y -qq ufw
    
    # Monitoring tools
    apt-get install -y -qq \
      htop \
      netstat-nat \
      jq
    
    ;;
    
  centos|rhel|fedora)
    echo -e "${BLUE}Installing dependencies for CentOS/RHEL/Fedora...${NC}"
    
    # Enable EPEL repository
    if [ "$OS" = "centos" ] || [ "$OS" = "rhel" ]; then
      yum install -y epel-release
    fi
    
    yum update -y
    
    # Development tools
    yum groupinstall -y "Development Tools"
    yum install -y \
      curl \
      wget \
      git \
      openssl-devel \
      pkg-config
    
    # Rust
    if ! command -v rustc &> /dev/null; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
    fi
    
    # Node.js
    if ! command -v node &> /dev/null; then
      curl -fsSL https://rpm.nodesource.com/setup_20.x | bash -
      yum install -y nodejs
    fi
    
    # Python 3
    yum install -y \
      python3 \
      python3-pip
    
    # Docker
    if ! command -v docker &> /dev/null; then
      yum install -y \
        docker \
        docker-compose
      systemctl enable docker
      systemctl start docker
    fi
    
    # Nginx
    yum install -y nginx
    
    # PostgreSQL
    yum install -y \
      postgresql-server \
      postgresql-contrib
    
    # Certbot
    yum install -y certbot python3-certbot-nginx
    
    # Firewall
    yum install -y firewalld
    systemctl enable firewalld
    systemctl start firewalld
    
    # Monitoring
    yum install -y htop net-tools jq
    
    ;;
    
  arch|manjaro)
    echo -e "${BLUE}Installing dependencies for Arch Linux...${NC}"
    
    pacman -Syu --noconfirm
    
    # Development tools
    pacman -S --noconfirm \
      base-devel \
      curl \
      wget \
      git \
      openssl \
      pkg-config
    
    # Rust
    if ! command -v rustc &> /dev/null; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
    fi
    
    # Node.js
    pacman -S --noconfirm nodejs npm
    
    # Python
    pacman -S --noconfirm python python-pip
    
    # Docker
    if ! command -v docker &> /dev/null; then
      pacman -S --noconfirm docker docker-compose
      systemctl enable docker
      systemctl start docker
    fi
    
    # Nginx
    pacman -S --noconfirm nginx
    
    # PostgreSQL
    pacman -S --noconfirm postgresql
    
    # Certbot
    pacman -S --noconfirm certbot certbot-nginx
    
    # Firewall
    pacman -S --noconfirm ufw
    
    # Monitoring
    pacman -S --noconfirm htop net-tools jq
    
    ;;
    
  alpine)
    echo -e "${BLUE}Installing dependencies for Alpine Linux...${NC}"
    
    apk update
    
    # Development tools
    apk add \
      build-base \
      curl \
      wget \
      git \
      openssl-dev \
      pkgconfig
    
    # Rust
    if ! command -v rustc &> /dev/null; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
    fi
    
    # Node.js
    apk add nodejs npm
    
    # Python
    apk add python3 py3-pip
    
    # Docker
    if ! command -v docker &> /dev/null; then
      apk add docker docker-compose
      rc-update add docker boot
      service docker start
    fi
    
    # Nginx
    apk add nginx
    
    # PostgreSQL
    apk add postgresql postgresql-contrib
    
    # Certbot
    apk add certbot certbot-nginx
    
    # Firewall
    apk add iptables
    
    # Monitoring
    apk add htop jq
    
    ;;
    
  *)
    echo -e "${RED}Unsupported OS: $OS${NC}"
    echo "Please install dependencies manually:"
    echo "  - Rust (rustup)"
    echo "  - Node.js 18+"
    echo "  - Python 3.8+"
    echo "  - Docker & Docker Compose"
    echo "  - Nginx"
    echo "  - PostgreSQL"
    echo "  - Certbot"
    exit 1
    ;;
esac

# Verify installations
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Verifying Installations${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

check_command() {
  if command -v $1 &> /dev/null; then
    VERSION=$($1 --version 2>&1 | head -n1)
    echo -e "${GREEN}✓${NC} $1: $VERSION"
  else
    echo -e "${RED}✗${NC} $1: Not found"
  fi
}

check_command rustc
check_command cargo
check_command node
check_command npm
check_command python3
check_command pip3
check_command docker
check_command docker-compose
check_command nginx
check_command psql
check_command certbot
check_command git
check_command curl

# Add user to docker group (if not root)
if [ -n "$SUDO_USER" ]; then
  echo ""
  echo -e "${YELLOW}Adding $SUDO_USER to docker group...${NC}"
  usermod -aG docker "$SUDO_USER"
  echo -e "${GREEN}✓${NC} User added to docker group (logout/login required)"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Installation Complete! 🎉${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo ""
echo "1. Logout and login again (for docker group)"
echo "2. Verify Rust: ${BLUE}rustc --version${NC}"
echo "3. Clone repository: ${BLUE}git clone https://github.com/axionaxprotocol/axionax-core.git${NC}"
echo "4. Build project: ${BLUE}cd axionax-core && cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Deploy Services:${NC}"
echo "  • RPC Node: ${BLUE}bash scripts/setup_rpc_node.sh${NC}"
echo "  • Explorer: ${BLUE}bash scripts/setup_explorer.sh${NC}"
echo "  • Faucet: ${BLUE}bash scripts/setup_faucet.sh${NC}"
echo ""
