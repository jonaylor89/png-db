#!/bin/bash
set -e

echo "Building WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build WASM package
wasm-pack build --target web --features wasm --no-default-features

# Update package name to include scope
# Using node for reliable JSON manipulation across platforms
if command -v node &> /dev/null; then
    node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('pkg/package.json', 'utf8'));
pkg.name = '@jonaylor89/png-db';
fs.writeFileSync('pkg/package.json', JSON.stringify(pkg, null, 2) + '\n');
"
    echo "✓ Updated package name to @jonaylor89/png-db"
else
    # Fallback to sed for systems without node
    sed -i.bak 's/"name": "png-db"/"name": "@jonaylor89\/png-db"/' pkg/package.json
    rm -f pkg/package.json.bak
    echo "✓ Updated package name to @jonaylor89/png-db"
fi

echo "WASM package built successfully in ./pkg/"
