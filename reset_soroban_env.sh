#!/usr/bin/env bash
set -euo pipefail

# === CONFIG ===
PROTO_VERSION="${1:-22}"    # Pass 22 or 23 as first argument. Default: 22
RUST_22="1.88.0"
RUST_23="1.78.0"
SDK_22="22.0.8"
SDK_23="23.0.0-rc.2.2"
CLI_22="soroban-cli"
CLI_23="stellar-cli"
CLI_22_VER="22.8.2"
CLI_23_VER="23.0.0"

echo "Resetting environment for Protocol $PROTO_VERSION..."

# === FUNCTIONS ===
uninstall_cli() {
    echo "Checking for existing CLI binaries..."
    if command -v soroban >/dev/null 2>&1; then
        echo "Uninstalling soroban-cli..."
        cargo uninstall soroban-cli || true
    fi
    if command -v stellar >/dev/null 2>&1; then
        echo "Uninstalling stellar-cli..."
        cargo uninstall stellar-cli || true
    fi
}

set_versions() {
    if [ "$PROTO_VERSION" = "22" ]; then
        CLI_NAME=$CLI_22
        CLI_VER=$CLI_22_VER
        SDK_VER=$SDK_22
        RUST_VER=$RUST_22
    elif [ "$PROTO_VERSION" = "23" ]; then
        CLI_NAME=$CLI_23
        CLI_VER=$CLI_23_VER
        SDK_VER=$SDK_23
        RUST_VER=$RUST_23
    else
        echo "Unsupported Protocol version: $PROTO_VERSION"
        exit 1
    fi
}

install_cli() {
    echo "Installing $CLI_NAME $CLI_VER..."
    cargo install "$CLI_NAME" --version "$CLI_VER" --force
}

pin_rust() {
    echo "Setting Rust toolchain to $RUST_VER..."
    rustup override set "$RUST_VER"
}

update_cargo_toml() {
    echo "Pinning soroban-sdk to $SDK_VER in Cargo.toml..."
    sed -i.bak -E "s/soroban-sdk\s*=\s*\"[0-9a-zA-Z.\-]+\"/soroban-sdk = \"$SDK_VER\"/" Cargo.toml
    sed -i.bak -E "s/soroban-sdk\s*=\s*\{ version = \"[0-9a-zA-Z.\-]+\"/soroban-sdk = { version = \"$SDK_VER\"/" Cargo.toml
    rm -f Cargo.toml.bak
}

rebuild_project() {
    echo "Cleaning previous build artifacts and Cargo lockfile..."
    rm -rf Cargo.lock target
    cargo generate-lockfile || true

    echo "Fetching dependencies..."
    cargo update -p soroban-sdk --precise "$SDK_VER"

    echo "Rebuilding contract..."
    cargo build --target wasm32-unknown-unknown --release
}

cleanup_cargo() {
    echo "Cleaning Cargo caches..."
    cargo cache --autoclean || true
    rm -rf ~/.cargo/registry ~/.cargo/git
    echo "Cleaned Cargo registry and git directories."
}

# === EXECUTION ===
uninstall_cli
set_versions
install_cli
pin_rust
update_cargo_toml
rebuild_project
cleanup_cargo

echo "Environment reset complete:"
echo "   CLI:  $CLI_NAME $CLI_VER"
echo "   SDK:  soroban-sdk $SDK_VER"
echo "   Rust: $RUST_VER"
echo "Ready for deploy: $CLI_NAME contract deploy ..."
