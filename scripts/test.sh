#!/usr/bin/env bash
set -euo pipefail

echo "🧪 Running tests..."

# Run all tests
cargo test

echo ""
echo "📊 Running tests with output (verbose)..."
cargo test -- --nocapture

# Optional: run benchmarks if criterion is added later
if cargo bench &> /dev/null; then
    echo ""
    echo "⚡ Running benchmarks..."
    cargo bench
fi

echo "✅ Tests complete"
