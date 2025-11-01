#!/bin/bash
#
# Axionax Dependency Installer for macOS
# Supports: macOS 10.15+ (Catalina and later)
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Axionax Dependency Installer${NC}"
echo -e "${BLUE}   macOS Edition${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check macOS version
OS_VERSION=$(sw_vers -productVersion)
echo -e "${GREEN}macOS Version: $OS_VERSION${NC}"
echo ""

# Check if running on Apple Silicon or Intel
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    echo -e "${GREEN}Architecture: Apple Silicon (M1/M2/M3)${NC}"
elif [ "$ARCH" = "x86_64" ]; then
    echo -e "${GREEN}Architecture: Intel${NC}"
fi
echo ""

# Install Homebrew if not present
if ! command -v brew &> /dev/null; then
    echo -e "${BLUE}Installing Homebrew...${NC}"
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # Add Homebrew to PATH for Apple Silicon
    if [ "$ARCH" = "arm64" ]; then
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
        eval "$(/opt/homebrew/bin/brew shellenv)"
    fi
    
    echo -e "${GREEN}✓ Homebrew installed${NC}"
else
    echo -e "${GREEN}✓ Homebrew already installed${NC}"
    echo -e "${BLUE}Updating Homebrew...${NC}"
    brew update
fi

echo ""
echo -e "${BLUE}Installing development tools...${NC}"
echo ""

# Install Xcode Command Line Tools
if ! xcode-select -p &> /dev/null; then
    echo -e "${BLUE}Installing Xcode Command Line Tools...${NC}"
    xcode-select --install
    echo "Press any key after Xcode Command Line Tools installation completes..."
    read -n 1 -s
    echo -e "${GREEN}✓ Xcode Command Line Tools installed${NC}"
else
    echo -e "${GREEN}✓ Xcode Command Line Tools already installed${NC}"
fi

# Install core dependencies via Homebrew
echo -e "${BLUE}Installing core dependencies...${NC}"

brew install \
    curl \
    wget \
    git \
    openssl \
    pkg-config

# Install Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${BLUE}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed${NC}"
else
    echo -e "${GREEN}✓ Rust already installed${NC}"
    echo -e "${BLUE}Updating Rust...${NC}"
    rustup update
fi

# Configure Rust
rustup default stable
rustup component add clippy rustfmt

# Install Node.js (LTS version)
if ! command -v node &> /dev/null; then
    echo -e "${BLUE}Installing Node.js...${NC}"
    brew install node@20
    brew link node@20
    echo -e "${GREEN}✓ Node.js installed${NC}"
else
    echo -e "${GREEN}✓ Node.js already installed${NC}"
fi

# Install Python 3
if ! command -v python3 &> /dev/null; then
    echo -e "${BLUE}Installing Python 3...${NC}"
    brew install python@3.11
    echo -e "${GREEN}✓ Python installed${NC}"
else
    echo -e "${GREEN}✓ Python already installed${NC}"
fi

# Install Docker Desktop for Mac
if ! command -v docker &> /dev/null; then
    echo -e "${BLUE}Installing Docker Desktop...${NC}"
    
    if [ "$ARCH" = "arm64" ]; then
        brew install --cask docker
    else
        brew install --cask docker
    fi
    
    echo -e "${GREEN}✓ Docker Desktop installed${NC}"
    echo -e "${YELLOW}Please start Docker Desktop from Applications${NC}"
else
    echo -e "${GREEN}✓ Docker already installed${NC}"
fi

# Install PostgreSQL
if ! command -v psql &> /dev/null; then
    echo -e "${BLUE}Installing PostgreSQL...${NC}"
    brew install postgresql@15
    brew services start postgresql@15
    echo -e "${GREEN}✓ PostgreSQL installed and started${NC}"
else
    echo -e "${GREEN}✓ PostgreSQL already installed${NC}"
fi

# Install Nginx
if ! command -v nginx &> /dev/null; then
    echo -e "${BLUE}Installing Nginx...${NC}"
    brew install nginx
    echo -e "${GREEN}✓ Nginx installed${NC}"
else
    echo -e "${GREEN}✓ Nginx already installed${NC}"
fi

# Install Redis
if ! command -v redis-cli &> /dev/null; then
    echo -e "${BLUE}Installing Redis...${NC}"
    brew install redis
    echo -e "${GREEN}✓ Redis installed${NC}"
else
    echo -e "${GREEN}✓ Redis already installed${NC}"
fi

# Install useful development tools
echo ""
echo -e "${BLUE}Installing additional tools...${NC}"

brew install \
    jq \
    htop \
    wget \
    tree

# Install Node.js global packages
echo ""
echo -e "${BLUE}Installing Node.js global packages...${NC}"
npm install -g \
    yarn \
    typescript \
    ts-node \
    wscat

# Install Python packages
echo ""
echo -e "${BLUE}Installing Python packages...${NC}"
pip3 install --upgrade pip
pip3 install \
    virtualenv \
    pytest \
    requests

# Install VS Code (optional)
if ! command -v code &> /dev/null; then
    echo ""
    read -p "Install Visual Studio Code? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        brew install --cask visual-studio-code
        echo -e "${GREEN}✓ VS Code installed${NC}"
    fi
fi

# Configure shell (add to PATH if needed)
echo ""
echo -e "${BLUE}Configuring shell...${NC}"

SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bash_profile"
fi

if [ -n "$SHELL_RC" ]; then
    # Add Rust to PATH
    if ! grep -q "\.cargo/env" "$SHELL_RC"; then
        echo 'source $HOME/.cargo/env' >> "$SHELL_RC"
        echo -e "${GREEN}✓ Added Rust to $SHELL_RC${NC}"
    fi
    
    # Add Homebrew to PATH (for Apple Silicon)
    if [ "$ARCH" = "arm64" ] && ! grep -q "/opt/homebrew/bin/brew" "$SHELL_RC"; then
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> "$SHELL_RC"
        echo -e "${GREEN}✓ Added Homebrew to $SHELL_RC${NC}"
    fi
fi

# Verify installations
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   Verifying Installations${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

check_command() {
    if command -v $1 &> /dev/null; then
        VERSION=$($1 --version 2>&1 | head -n1 || echo "installed")
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
check_command psql
check_command nginx
check_command git
check_command jq

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Installation Complete! 🎉${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo ""
echo "1. Restart your terminal or run:"
echo "   ${BLUE}source ~/.zshrc${NC}  (or ~/.bash_profile)"
echo ""
echo "2. Start Docker Desktop from Applications"
echo ""
echo "3. Verify installations:"
echo "   ${BLUE}rustc --version${NC}"
echo "   ${BLUE}node --version${NC}"
echo "   ${BLUE}python3 --version${NC}"
echo ""
echo "4. Clone repository:"
echo "   ${BLUE}git clone https://github.com/axionaxprotocol/axionax-core.git${NC}"
echo ""
echo "5. Build project:"
echo "   ${BLUE}cd axionax-core${NC}"
echo "   ${BLUE}cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Development Tools:${NC}"
echo "  • VS Code: ${BLUE}code .${NC}"
echo "  • PostgreSQL: ${BLUE}brew services start postgresql@15${NC}"
echo "  • Nginx: ${BLUE}brew services start nginx${NC}"
echo "  • Redis: ${BLUE}brew services start redis${NC}"
echo ""
echo -e "${YELLOW}Useful Commands:${NC}"
echo "  • Update all: ${BLUE}brew update && brew upgrade${NC}"
echo "  • List services: ${BLUE}brew services list${NC}"
echo "  • Rust update: ${BLUE}rustup update${NC}"
echo ""
