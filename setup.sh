#!/usr/bin/env bash
set -e

echo "============================================================"
echo "   🚀 WELCOME TO HALCA MULTI-GAME TERMINAL ARCADE SETUP    "
echo "============================================================"

# 1. Detect OS
OS_TYPE="$(uname -s)"
echo "[+] Operating System detected: $OS_TYPE"

# 2. Check C Compiler Linker
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
    echo "[!] C Compiler (cc/gcc) not found. Installing..."
    if [ "$OS_TYPE" = "Darwin" ]; then
        xcode-select --install || true
    elif [ -f /etc/debian_version ]; then
        sudo apt-get update -y && sudo apt-get install -y build-essential
    fi
fi

# 3. Check / Install Rust Toolchain
if ! command -v cargo &> /dev/null; then
    echo "[!] Rust toolchain not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

source "$HOME/.cargo/env" 2>/dev/null || true

# 4. Build Release Binary
echo "[+] Compiling HALCA Arcade Client binary..."
cargo build --release --bin client

echo "============================================================"
echo "   ✅ SETUP COMPLETE! LAUNCHING HALCA ARCADE CLIENT...     "
echo "============================================================"
./target/release/client
