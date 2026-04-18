#!/usr/bin/env bash
set -euo pipefail

echo "🚀 Running Super Shell..."

# Default to debug build
if [[ "${1:-}" == "--release" ]]; then
    cargo run --release
else
    cargo run
fi
