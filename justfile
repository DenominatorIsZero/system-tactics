# SystemTactics - Game Development Commands

# Default recipe shows available commands
default:
    @just --list

# Setup and Maintenance Commands

# Install WASM target and required development tools
setup:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-server-runner
    cargo install wasm-bindgen-cli

# Clean all build artifacts
clean:
    cargo clean

# Copy assets to embedded location for WASM builds
copy-assets:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Copying assets for WASM embedding..."
    rm -rf game/src/assets
    cp -r assets game/src/assets
    echo "Assets copied to game/src/assets/"

# Build Commands

# Build all workspace components (debug)
build:
    cargo build --workspace

# Build all workspace components (release)
build-release:
    cargo build --workspace --release

# Build WASM binary (debug) - for testing with wasm-server-runner
build-wasm: copy-assets
    cargo build --target wasm32-unknown-unknown --bin game

# Build WASM binary (release) - for testing with wasm-server-runner
build-wasm-release: copy-assets
    cargo build --target wasm32-unknown-unknown --bin game --release

# Build web package with wasm-bindgen (for website deployment)
build-web: copy-assets
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building release WASM for SystemTactics web deployment..."
    cargo build --target wasm32-unknown-unknown --bin game --release
    echo "Creating web directory..."
    mkdir -p game/web
    echo "Generating JavaScript wrapper with wasm-bindgen..."
    wasm-bindgen --no-typescript --target web \
        --out-dir ./game/web/ \
        --out-name "game" \
        ./target/wasm32-unknown-unknown/release/game.wasm
    echo "Web package ready in game/web/"
    echo "Files generated:"
    ls -lh game/web/

# Run Commands

# Run the level editor
run-level-editor:
    cargo run --bin level-editor

# Run the game natively (not WASM)
run-game:
    cargo run --bin game

# Run WASM game with development server (debug build)
wasm: build-wasm
    wasm-server-runner target/wasm32-unknown-unknown/debug/game.wasm

# Run WASM game with development server (release build)
wasm-release: build-wasm-release
    wasm-server-runner target/wasm32-unknown-unknown/release/game.wasm

# Serve built web package locally for testing
serve-web:
    @echo "Starting local server for SystemTactics web package..."
    @echo "Open http://localhost:8000 in your browser"
    @cd game/web && python -m http.server 8000

# Deploy to rust-website repository (copies WASM files)
deploy-web: build-web
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Deploying SystemTactics to rust-website..."
    WEBSITE_DIR="/Users/eren/Projects/rust-website/static/wasm/system-tactics"
    mkdir -p "$WEBSITE_DIR"
    cp game/web/* "$WEBSITE_DIR/"
    echo "SystemTactics deployed to $WEBSITE_DIR"
    echo "Files deployed:"
    ls -lh "$WEBSITE_DIR"

# Code Quality Commands

# Format all code
fmt:
    cargo fmt --all

# Run clippy linting
lint:
    cargo clippy --workspace -- -D warnings

# Run tests
test:
    cargo test --workspace

# Run all checks (format, lint, test)
check: fmt lint test
