#!/usr/bin/env bash
set -euo pipefail

echo "🧪 Starting development mode..."

# Auto-rebuild on changes if cargo-watch is installed
if command -v cargo-watch &> /dev/null; then
    echo "👀 Using cargo-watch for live reload..."
    cargo watch -x run
else
    echo "⚠️ cargo-watch not found. Falling back to manual run."
    echo "💡 Install with: cargo install cargo-watch"
    cargo run
fi
