#!/bin/bash
set -e

echo "Building WASM..."
wasm-pack build --target web --release

echo "Build complete. To run, use a local server, e.g.:"
echo "python3 -m http.server"
