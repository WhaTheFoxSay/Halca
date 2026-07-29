#!/usr/bin/env bash
set -e

HALCA_DIR="$HOME/.halca"
MANIFEST_LOG="$HALCA_DIR/install_manifest.log"
LOCAL_BIN_DIR="$HOME/.local/bin"
CARGO_BIN_DIR="$HOME/.cargo/bin"

mkdir -p "$HALCA_DIR"
mkdir -p "$LOCAL_BIN_DIR"
mkdir -p "$CARGO_BIN_DIR"

if [ ! -f "$MANIFEST_LOG" ]; then
    touch "$MANIFEST_LOG"
fi

echo "============================================================"
echo "   🚀 HALCA MULTI-GAME TERMINAL ARCADE ONE-LINE INSTALLER   "
echo "============================================================"

# Helper function to log installed items
log_installed() {
    local item_type="$1"
    local item_name="$2"
    if ! grep -q "^$item_type:$item_name" "$MANIFEST_LOG" 2>/dev/null; then
        echo "$item_type:$item_name" >> "$MANIFEST_LOG"
    fi
}

# 1. Detect OS
OS_TYPE="$(uname -s)"
echo "[+] Operating System detected: $OS_TYPE"

# 2. Check C Compiler Linker
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
    echo "[+] C Compiler not found. Installing..."
    if [ "$OS_TYPE" = "Darwin" ]; then
        xcode-select --install || true
        log_installed "DEP" "xcode_tools"
    elif [ -f /etc/debian_version ]; then
        sudo apt-get update -y && sudo apt-get install -y build-essential
        log_installed "DEP" "build_essential"
    fi
else
    echo "[✓] C Compiler already installed. SKIP (Not marked for uninstall)."
fi

# 3. Check Rust Toolchain
if ! command -v cargo &> /dev/null; then
    echo "[+] Rust toolchain not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    log_installed "DEP" "rustup"
else
    echo "[✓] Rust toolchain already installed. SKIP (Not marked for uninstall)."
fi

source "$HOME/.cargo/env" 2>/dev/null || true

# 4. Clone or Copy Halca Arcade Source
INSTALL_SRC="$HALCA_DIR/source"
echo "[+] Syncing Halca Arcade platform engine to $INSTALL_SRC..."

if [ -d "$INSTALL_SRC/.git" ]; then
    (cd "$INSTALL_SRC" && git pull origin main)
else
    rm -rf "$INSTALL_SRC"
    mkdir -p "$INSTALL_SRC"
    if [ -d "./games" ]; then
        cp -R ./ "$INSTALL_SRC/"
    else
        git clone https://github.com/WhaTheFoxSay/Halca.git "$INSTALL_SRC"
    fi
fi

# 5. Build Halca Arcade Client Binary
echo "[+] Compiling Halca Arcade Client binary..."
(cd "$INSTALL_SRC" && cargo build --release --bin client)

# 6. Install Executable Binary into both ~/.local/bin and ~/.cargo/bin
CLIENT_BIN="$INSTALL_SRC/target/release/client"

cp "$CLIENT_BIN" "$LOCAL_BIN_DIR/halca"
chmod +x "$LOCAL_BIN_DIR/halca"

if [ -d "$CARGO_BIN_DIR" ]; then
    cp "$CLIENT_BIN" "$CARGO_BIN_DIR/halca"
    chmod +x "$CARGO_BIN_DIR/halca"
fi

log_installed "APP" "halca_binary"

# 7. Check & Update PATH Environment Variable
for SHELL_RC in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile"; do
    if [ -f "$SHELL_RC" ]; then
        if ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$SHELL_RC"; then
            echo '' >> "$SHELL_RC"
            echo '# Added by Halca Terminal Arcade Installer' >> "$SHELL_RC"
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
        fi
    fi
done

export PATH="$CARGO_BIN_DIR:$LOCAL_BIN_DIR:$PATH"

echo "============================================================"
echo "   ✅ HALCA MULTI-GAME TERMINAL ARCADE INSTALLED SUCCESSFULLY! "
echo "============================================================"
echo ""
echo "   👉 Ketik \"halca\" di terminal kamu untuk membuka menu Halca!"
echo "      (Jika belum terbaca di window terminal saat ini, jalankan:"
echo "       source ~/.zshrc  atau buka window terminal baru)"
echo ""
echo "============================================================"
