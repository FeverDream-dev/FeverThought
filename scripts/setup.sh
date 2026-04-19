#!/usr/bin/env bash
set -euo pipefail

echo "=== FeverThoth Development Setup ==="

check_tool() {
    if command -v "$1" &>/dev/null; then
        echo "✓ $1 found"
    else
        echo "✗ $1 not found — please install $2"
        return 1
    fi
}

check_tool node "Node.js 20+" || true
check_tool pnpm "pnpm 9+" || true
check_tool rustc "Rust via rustup" || true
check_tool cargo "Cargo (comes with Rust)" || true

echo ""
echo "Installing frontend dependencies..."
pnpm install

echo ""
echo "Checking Rust workspace..."
cargo check --workspace --exclude feverthoth-desktop 2>/dev/null && echo "✓ Rust workspace compiles" || echo "⚠ Some Rust crates have issues (Tauri needs system deps)"

echo ""
echo "=== Setup complete! Run 'pnpm dev' to start development. ==="
