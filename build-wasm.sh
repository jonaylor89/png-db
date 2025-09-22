#!/bin/bash
set -e

echo "Building WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

wasm-pack build --target web --features wasm --no-default-features

echo "WASM package built successfully!"
echo "You can now serve the web directory with any HTTP server."
echo ""
echo "For example:"
echo "  cd web && python3 -m http.server 8000"
echo "  or"
echo "  cd web && npx serve ."
