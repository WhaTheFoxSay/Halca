#!/usr/bin/env bash
set -e

HALCA_DIR="$HOME/.halca"
MANIFEST_LOG="$HALCA_DIR/install_manifest.log"
LOCAL_BIN_DIR="$HOME/.local/bin"
CARGO_BIN_DIR="$HOME/.cargo/bin"

mkdir -p "$HALCA_DIR"
mkdir -p "$LOCAL_BIN_DIR" 2>/dev/null || true
mkdir -p "$CARGO_BIN_DIR" 2>/dev/null || true

if [ ! -f "$MANIFEST_LOG" ]; then
    touch "$MANIFEST_LOG"
fi

echo "============================================================"
echo "   [ HALCA MULTI-GAME TERMINAL ARCADE UNIVERSAL SETUP ]     "
echo "============================================================"

log_installed() {
    local item_type="$1"
    local item_name="$2"
    if ! grep -q "^$item_type:$item_name" "$MANIFEST_LOG" 2>/dev/null; then
        echo "$item_type:$item_name" >> "$MANIFEST_LOG"
    fi
}

# 1. Detect Operating System & Kernel
UNAME_OS="$(uname -s 2>/dev/null || echo "Unknown")"
echo "[+] Target OS Environment Detected: $UNAME_OS"

# 2. Check Package Manager & C Compiler
if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
    echo "[!] C Compiler not found. Detecting system package manager..."
    case "$UNAME_OS" in
        Darwin)
            xcode-select --install 2>/dev/null || true
            log_installed "DEP" "xcode_tools"
            ;;
        Linux)
            if command -v apt-get &> /dev/null; then
                sudo apt-get update -y && sudo apt-get install -y build-essential
                log_installed "DEP" "build_essential"
            elif command -v dnf &> /dev/null; then
                sudo dnf groupinstall -y "Development Tools" || sudo dnf install -y gcc gcc-c++ make
                log_installed "DEP" "dnf_dev_tools"
            elif command -v yum &> /dev/null; then
                sudo yum groupinstall -y "Development Tools"
                log_installed "DEP" "yum_dev_tools"
            elif command -v pacman &> /dev/null; then
                sudo pacman -Sy --noconfirm base-devel
                log_installed "DEP" "pacman_base_devel"
            elif command -v apk &> /dev/null; then
                sudo apk add build-base
                log_installed "DEP" "apk_build_base"
            elif command -v zypper &> /dev/null; then
                sudo zypper install -t pattern devel_basis
                log_installed "DEP" "zypper_devel_basis"
            fi
            ;;
        FreeBSD|OpenBSD|NetBSD)
            if command -v pkg &> /dev/null; then
                sudo pkg install -y gcc make
                log_installed "DEP" "freebsd_gcc"
            elif command -v pkg_add &> /dev/null; then
                sudo pkg_add gcc make
                log_installed "DEP" "openbsd_gcc"
            fi
            ;;
        Haiku)
            pkgman install -y devel:gcc
            log_installed "DEP" "haiku_gcc"
            ;;
        *)
            echo "[!] Unrecognized package manager. Please ensure a C compiler (gcc/clang) is installed."
            ;;
    esac
else
    echo "[✓] C Compiler already present. SKIP (Preserved safely)."
fi

# 3. Check Rust Toolchain
if ! command -v cargo &> /dev/null; then
    echo "[+] Rust toolchain not found. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
    log_installed "DEP" "rustup"
else
    echo "[✓] Rust toolchain already present. SKIP (Preserved safely)."
fi

source "$HOME/.cargo/env" 2>/dev/null || true

# 4. Clone or Sync Halca Platform Source
INSTALL_SRC="$HALCA_DIR/source"
echo "[+] Syncing Halca Arcade platform engine to $INSTALL_SRC..."

if [ -d "$INSTALL_SRC/.git" ]; then
    (cd "$INSTALL_SRC" && git pull origin main 2>/dev/null || true)
else
    rm -rf "$INSTALL_SRC"
    mkdir -p "$INSTALL_SRC"
    if [ -d "./games" ]; then
        cp -R ./ "$INSTALL_SRC/"
    else
        git clone https://github.com/WhaTheFoxSay/Halca.git "$INSTALL_SRC"
    fi
fi

# 5. Build Halca Arcade Client Release Binary
echo "[+] Compiling Halca Arcade Client binary..."
(cd "$INSTALL_SRC" && cargo build --release --bin client)

CLIENT_BIN="$INSTALL_SRC/target/release/client"

# 6. Install Executable Binary & Uppercase Alias ('halca' & 'HALCA')
for TARGET_DIR in "$CARGO_BIN_DIR" "$LOCAL_BIN_DIR"; do
    if [ -d "$TARGET_DIR" ]; then
        cp "$CLIENT_BIN" "$TARGET_DIR/halca"
        cp "$CLIENT_BIN" "$TARGET_DIR/HALCA" 2>/dev/null || true
        chmod +x "$TARGET_DIR/halca" 2>/dev/null || true
        chmod +x "$TARGET_DIR/HALCA" 2>/dev/null || true
    fi
done

log_installed "APP" "halca_binary"

# 7. Add PATH to all active Shell Configuration Files
SHELL_FILES=(
    "$HOME/.zshrc"
    "$HOME/.bashrc"
    "$HOME/.bash_profile"
    "$HOME/.profile"
    "$HOME/.config/fish/config.fish"
    "$HOME/config/settings/boot/UserSetup"
)

for RC_FILE in "${SHELL_FILES[@]}"; do
    if [ -f "$RC_FILE" ]; then
        if ! grep -q 'PATH.*\.cargo/bin' "$RC_FILE" 2>/dev/null && ! grep -q 'PATH.*\.local/bin' "$RC_FILE" 2>/dev/null; then
            echo '' >> "$RC_FILE"
            echo '# Added by Halca Terminal Arcade Installer' >> "$RC_FILE"
            echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"' >> "$RC_FILE"
        fi
    fi
done

export PATH="$CARGO_BIN_DIR:$LOCAL_BIN_DIR:$PATH"

echo "============================================================"
echo "   [ HALCA TERMINAL ARCADE INSTALLED SUCCESSFULLY ]         "
echo "============================================================"
echo ""
echo "   >>> Ketik 'halca' atau 'HALCA' lalu tekan ENTER! <<<"
echo ""
echo "============================================================"
