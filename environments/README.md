# Axionax Environments

This directory contains environment-specific configurations and deployment files for Axionax Core.

## Directory Structure

```
environments/
├── mainnet/              # Mainnet configuration (reserved, not launched)
├── testnet/              # Testnet configurations
│   ├── Axionax_v1.5_Testnet_in_a_Box/
│   └── Axionax_v1.6_Testnet_in_a_Box/
├── config.example.yaml   # Example configuration template
└── docker-compose.yaml   # Docker compose setup
```

## Environments

### Testnet

- **Chain ID**: 86137
- **Status**: Active for testing
- **Purpose**: Development, testing, and integration
- **Versions**:
  - v1.5: Legacy testnet setup
  - v1.6: Current testnet setup with Rust/Python architecture

### Mainnet

- **Chain ID**: 86150
- **Status**: Reserved, NOT launched
- **Purpose**: Production network (future)

⚠️ **WARNING**: Any network claiming to be "Axionax Mainnet" is a SCAM. Verify at https://axionax.org/networks

## Configuration

### Using config.example.yaml

Copy and modify the example configuration:

```bash
cp config.example.yaml config.yaml
# Edit config.yaml with your settings
```

### Using Docker Compose

Start a local testnet:

```bash
docker-compose up -d
```

## Documentation

- [Testnet v1.5 Setup](./testnet/Axionax_v1.5_Testnet_in_a_Box/README.md)
- [Testnet v1.6 Setup](./testnet/Axionax_v1.6_Testnet_in_a_Box/README.md)
- [Main Documentation](../docs/)

## Security

For security concerns, please see [SECURITY.md](../docs/SECURITY.md) and report vulnerabilities to security@axionax.org
