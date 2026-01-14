#!/bin/bash
set -e

echo "Building WASM target..."
cargo build --release --target wasm32-unknown-unknown

echo "Setting up dist directory..."
mkdir -p dist

echo "Copying WASM binary..."
cp target/wasm32-unknown-unknown/release/ivy.wasm dist/

echo "Copying assets..."
cp -r assets dist/ 2>/dev/null || true

echo "Copying index.html..."
cp index.html dist/

echo "WASM build complete! Output in ./dist/"
