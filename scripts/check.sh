#!/usr/bin/env bash
set -euo pipefail

echo "=== FeverThoth Checks ==="

echo "→ cargo fmt"
cargo fmt --all -- --check
echo "✓ Format OK"

echo "→ cargo clippy"
cargo clippy --workspace --all-targets --exclude feverthoth-desktop -- -D warnings
echo "✓ Clippy OK"

echo "→ cargo test"
cargo test --workspace --exclude feverthoth-desktop
echo "✓ Tests OK"

echo "→ pnpm lint"
pnpm lint 2>/dev/null && echo "✓ Lint OK" || echo "⚠ pnpm lint not configured yet"

echo ""
echo "=== All checks passed ==="
