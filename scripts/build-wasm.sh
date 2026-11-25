#!/usr/bin/env sh
# Build the browser demos (crates/playground) into static/pkg/.
# Needs: rustup target add wasm32-unknown-unknown
#        cargo install wasm-bindgen-cli --version 0.2.130 --locked
# Optional: wasm-opt (from binaryen) for a smaller bundle.
set -eu
cd "$(dirname "$0")/.."

cargo build -p playground --target wasm32-unknown-unknown --release

wasm-bindgen \
  --target web \
  --no-typescript \
  --out-dir static/pkg \
  target/wasm32-unknown-unknown/release/playground.wasm

if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int \
    static/pkg/playground_bg.wasm -o static/pkg/playground_bg.wasm
fi

ls -l static/pkg/playground_bg.wasm
