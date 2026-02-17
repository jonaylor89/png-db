# Build CLI version
build:
    cargo build --release

# Build WASM version for web
build-wasm:
    ./build-wasm.sh

# Run CLI tests
test:
    cargo test

# Clean build artifacts
clean:
    cargo clean
    rm -rf pkg/
    rm -f web/png_db_bg.wasm web/png_db.js web/png_db.d.ts web/png_db_bg.wasm.d.ts web/package.json


# Create a test database
demo:
    ./target/release/png-db create -f demo.png -s '{"name":"string","age":"number"}'
    ./target/release/png-db insert -f demo.png -x 10 -y 20 -d '{"name":"Alice","age":30}'
    ./target/release/png-db insert -f demo.png -x 50 -y 60 -d '{"name":"Bob","age":25}'
    ./target/release/png-db query -f demo.png -w "WHERE age > 28"

# Show available commands
default:
    @just --list
