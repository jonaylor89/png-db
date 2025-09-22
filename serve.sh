#!/bin/bash

# Build WASM first
./build-wasm.sh

# Start a simple HTTP server
echo "Starting development server at http://localhost:8000"
cd web && python3 -m http.server 8000