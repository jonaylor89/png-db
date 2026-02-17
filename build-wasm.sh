#!/bin/bash
set -e

echo "Building WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

wasm-pack build --target web --features wasm --no-default-features

# Copy only the necessary files, avoiding overwriting package.json if we want to manage it manually
# Or we can just let it overwrite but we need to make sure the name is correct.
# Given we want Changesets to manage the version, it's better to let Changesets manage web/package.json

cp pkg/png_db_bg.wasm web/
cp pkg/png_db.js web/
cp pkg/png_db.d.ts web/
cp pkg/png_db_bg.wasm.d.ts web/ 2>/dev/null || true

echo "WASM artifacts copied to web/ directory!"
