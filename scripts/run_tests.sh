#!/bin/bash
#
# Axionax v1.6 - Unified Test Runner
# This script builds the Rust core, the Python bridge, and runs integration tests.
#

set -e # Exit immediately if a command exits with a non-zero status.

echo "🚀 Starting Axionax v1.6 Unified Test Suite..."
echo "=============================================="

# --- Step 1: Build Rust Core ---
echo ""
echo "🦀 Step 1/4: Building Rust core workspace..."
if cargo build --release --workspace; then
    echo "✅ Rust core built successfully."
else
    echo "❌ Failed to build Rust core."
    exit 1
fi

# --- Step 2: Build Python-Rust Bridge ---
echo ""
echo "🐍 Step 2/4: Building Python-Rust bridge (PyO3)..."
BRIDGE_DIR="bridge/rust-python"
if [ -f "$BRIDGE_DIR/build.sh" ]; then
    (cd "$BRIDGE_DIR" && ./build.sh)
    echo "✅ Python bridge built successfully."
else
    echo "❌ Bridge build script not found at $BRIDGE_DIR/build.sh"
    exit 1
fi

# --- Step 3: Run Integration Tests ---
echo ""
echo "🔗 Step 3/4: Running Python integration tests..."
python3 tests/integration_simple.py

# --- Step 4: Run Benchmarks (Optional) ---
echo ""
echo "📊 Step 4/4: Running performance benchmarks..."
python3 tools/benchmark.py

echo ""
echo "=============================================="
echo "🎉 All tests and benchmarks completed successfully!"