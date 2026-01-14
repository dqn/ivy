#!/bin/bash
set -e

echo "Building WASM target..."
cargo build --release --target wasm32-unknown-unknown

echo "Running wasm-bindgen..."
mkdir -p dist
wasm-bindgen target/wasm32-unknown-unknown/release/ivy.wasm \
    --out-dir ./dist \
    --target web \
    --no-typescript

echo "Copying assets..."
cp -r assets dist/ 2>/dev/null || true

echo "WASM build complete! Output in ./dist/"
