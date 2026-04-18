#!/usr/bin/env bash
set -euo pipefail

echo "🔧 Building project..."

# Ensure cargo exists
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust (cargo) is not installed."
    exit 1
fi

# Build in release mode if flag passed
if [[ "${1:-}" == "--release" ]]; then
    cargo build --release
    echo "✅ Build complete (release)"
else
    cargo build
    echo "✅ Build complete (debug)"
fi
