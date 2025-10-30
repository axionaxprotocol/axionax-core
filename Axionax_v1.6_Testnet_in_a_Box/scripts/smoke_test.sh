#!/usr/bin/env bash
set -euo pipefail

# Lightweight smoke tests for "Testnet in a Box"
# - checks JS syntax of key scripts
# - validates docker-compose file
# - optionally (commented) can bring up containers for a fuller integration test

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "[smoke] Root: $ROOT_DIR"

echo "[smoke] 1) Node syntax check (deploy & faucet scripts)"
node --check "$ROOT_DIR/deployer/deploy.js" "$ROOT_DIR/deployer/deploy_token.js" "$ROOT_DIR/faucet/index.js" || {
  echo "[smoke] node syntax check failed" >&2
  exit 2
}

echo "[smoke] 2) docker-compose file validation"
docker-compose -f "$ROOT_DIR/docker-compose.yml" config >/dev/null
echo "[smoke] docker-compose config OK"

echo "[smoke] All quick checks passed."

# For a fuller integration smoke (will start many services, DB, blockscout):
# Uncomment to run, be aware this can take several minutes and requires Docker
# docker-compose -f "$ROOT_DIR/docker-compose.yml" up -d --build
# sleep 8
# curl -fsS http://127.0.0.1:8545 || echo "RPC not reachable yet"

exit 0
