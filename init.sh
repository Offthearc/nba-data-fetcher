#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

echo "==> Checking Rust toolchain..."
cargo --version
rustc --version

echo "==> Installing/updating dependencies..."
cargo fetch

echo "==> Building project..."
cargo build

echo "==> Running tests..."
cargo test

echo "==> Running clippy..."
cargo clippy -- -D warnings

echo "==> Checking formatting..."
cargo fmt --check

echo ""
echo "All checks passed."
