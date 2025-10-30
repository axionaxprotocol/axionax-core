#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="$ROOT_DIR/.env"
EXAMPLE="$ROOT_DIR/.env.example"

echo "[check_env] looking for .env in $ROOT_DIR"
if [ -f "$ENV_FILE" ]; then
  echo "[check_env] .env found"
else
  echo "[check_env] .env not found — copying from .env.example to .env (please edit sensitive values)"
  if [ -f "$EXAMPLE" ]; then
    cp "$EXAMPLE" "$ENV_FILE"
    echo "[check_env] copied .env.example -> .env"
    echo "[check_env] Please update DEPLOYER_PRIVATE_KEY and FAUCET_PRIVATE_KEY in .env before running the testnet."
  else
    echo "[check_env] no .env.example found; please create a .env file with required variables" >&2
    exit 1
  fi
fi

echo "[check_env] verifying required variables"
missing=0
for v in RPC_URL DEPLOYER_PRIVATE_KEY FAUCET_PRIVATE_KEY CHAIN_ID; do
  if ! grep -q "^${v}=" "$ENV_FILE"; then
    echo "[check_env] Missing $v in .env"
    missing=$((missing+1))
  fi
done

if [ "$missing" -ne 0 ]; then
  echo "[check_env] found $missing missing variables — please update $ENV_FILE" >&2
  exit 2
fi

echo "[check_env] .env looks OK (at least contains required keys)."
exit 0
