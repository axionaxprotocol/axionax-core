#!/bin/bash
# Systemd Service Setup for Axionax Validator
# Run as root: sudo bash setup_systemd.sh

set -e

AXIONAX_USER="axionax"
SERVICE_FILE="/etc/systemd/system/axionax-validator.service"

echo "Setting up Axionax Validator systemd service..."

# Create service file
cat > $SERVICE_FILE << 'EOF'
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
Environment="PYTHONPATH=/home/axionax/axionax-core/deai"

# Start command
ExecStart=/usr/local/bin/axionax-core start \
    --config /home/axionax/.axionax/config/config.yaml \
    --validator

# Restart policy
Restart=always
RestartSec=10
LimitNOFILE=65535

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=read-only

# Logging
StandardOutput=append:/home/axionax/.axionax/logs/validator.log
StandardError=append:/home/axionax/.axionax/logs/validator.error.log
SyslogIdentifier=axionax-validator

[Install]
WantedBy=multi-user.target
EOF

echo "Service file created at $SERVICE_FILE"

# Reload systemd
systemctl daemon-reload
echo "Systemd reloaded"

# Enable service
systemctl enable axionax-validator
echo "Service enabled (will start on boot)"

echo ""
echo "Setup complete! To control the validator:"
echo "  Start:   sudo systemctl start axionax-validator"
echo "  Stop:    sudo systemctl stop axionax-validator"
echo "  Restart: sudo systemctl restart axionax-validator"
echo "  Status:  sudo systemctl status axionax-validator"
echo "  Logs:    journalctl -u axionax-validator -f"
echo ""
