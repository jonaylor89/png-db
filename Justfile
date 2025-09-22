# Build CLI version
build:
    cargo build --release

# Build WASM version for web
build-wasm:
    wasm-pack build --target web --features wasm --no-default-features
    cp -r pkg/* web/

# Run CLI tests
test:
    cargo test

# Serve web demo
serve: build-wasm
    cd web && python3 -m http.server 8000

# Clean build artifacts
clean:
    cargo clean
    rm -rf pkg/
    rm -f web/*.wasm web/*.js web/*.d.ts web/package.json

# Create a test database
demo:
    ./target/release/png-db create -f demo.png -s '{"name":"string","age":"number"}'
    ./target/release/png-db insert -f demo.png -x 10 -y 20 -d '{"name":"Alice","age":30}'
    ./target/release/png-db insert -f demo.png -x 50 -y 60 -d '{"name":"Bob","age":25}'
    ./target/release/png-db query -f demo.png -w "WHERE age > 28"

# Show available commands
default:
    @just --list