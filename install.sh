#!/usr/bin/env bash
set -e

# Terminal ANSI Color System
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
GREEN='\033[1;32m'
RED='\033[1;31m'
MAGENTA='\033[1;35m'
RESET='\033[0m'

HALCA_DIR="$HOME/.halca"
MANIFEST_LOG="$HALCA_DIR/install_manifest.log"
CARGO_BIN_DIR="$HOME/.cargo/bin"
LOCAL_BIN_DIR="$HOME/.local/bin"
USER_BIN_DIR="$HOME/bin"
RELEASE_BASE_URL="https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/releases"

mkdir -p "$HALCA_DIR"
mkdir -p "$CARGO_BIN_DIR" 2>/dev/null || true
mkdir -p "$LOCAL_BIN_DIR" 2>/dev/null || true
mkdir -p "$USER_BIN_DIR" 2>/dev/null || true

if [ ! -f "$MANIFEST_LOG" ]; then
    touch "$MANIFEST_LOG"
fi

log_installed() {
    local item_type="$1"
    local item_name="$2"
    if ! grep -q "^$item_type:$item_name" "$MANIFEST_LOG" 2>/dev/null; then
        echo "$item_type:$item_name" >> "$MANIFEST_LOG"
    fi
}

draw_progress_bar() {
    local text="$1"
    echo -e "${CYAN}[+] $text${RESET}"
    for i in {1..20}; do
        local filled=$(printf '▓%.0s' $(seq 1 $i))
        local empty=$(printf '░%.0s' $(seq 1 $((20 - i))))
        local pct=$((i * 5))
        printf "\r  ${YELLOW}[${filled}${empty}] ${pct}%%${RESET}"
        sleep 0.03
    done
    echo -e " ${GREEN}[COMPLETE]${RESET}\n"
}

clear 2>/dev/null || true
echo -e "${CYAN}[===] ============================================================ [===]${RESET}"
echo -e "${YELLOW}       HALCA MULTI-GAME TERMINAL ARCADE FAST SYSTEM SETUP        ${RESET}"
echo -e "${CYAN}[===] ============================================================ [===]${RESET}\n"

# 1. System Diagnostics
UNAME_OS_RAW="$(uname -s 2>/dev/null || echo "unknown")"
UNAME_ARCH_RAW="$(uname -m 2>/dev/null || echo "x86_64")"

UNAME_OS="$(echo "$UNAME_OS_RAW" | tr '[:upper:]' '[:lower:]')"

case "$UNAME_ARCH_RAW" in
    x86_64|amd64) TARGET_ARCH="x86_64" ;;
    arm64|aarch64) TARGET_ARCH="aarch64" ;;
    *) TARGET_ARCH="$UNAME_ARCH_RAW" ;;
esac

draw_progress_bar "INITIALIZING DIAGNOSTIC ENGINE: $UNAME_OS ($TARGET_ARCH)..."

CLIENT_BIN="$HALCA_DIR/bin/halca"
mkdir -p "$HALCA_DIR/bin"

BINARY_DOWNLOADED=false
BINARY_NAME="halca-${UNAME_OS}-${TARGET_ARCH}"

echo -e "${MAGENTA}[+] FETCHING PRE-COMPILED BINARY PACKAGE...${RESET}"
echo -e "    Target Asset: ${BINARY_NAME}"

if curl -sSfL "${RELEASE_BASE_URL}/${BINARY_NAME}" -o "$CLIENT_BIN" 2>/dev/null; then
    chmod +x "$CLIENT_BIN" 2>/dev/null || true
    BINARY_DOWNLOADED=true
    echo -e "    ${GREEN}[✓] FAST INSTALL: Pre-compiled Arcade Core retrieved in 2 seconds!${RESET}\n"
else
    echo -e "    ${YELLOW}[!] Pre-compiled binary not directly accessible. Switching to source compiler...${RESET}\n"
fi

# Fallback: Source Compilation if binary download wasn't cached
if [ "$BINARY_DOWNLOADED" = false ]; then
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}[+] Rust toolchain not found. Installing rustup...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
        log_installed "DEP" "rustup"
    fi
    source "$HOME/.cargo/env" 2>/dev/null || true

    INSTALL_SRC="$HALCA_DIR/source"
    echo -e "${CYAN}[+] Syncing Halca platform engine to $INSTALL_SRC...${RESET}"
    if [ -d "$INSTALL_SRC/.git" ]; then
        (cd "$INSTALL_SRC" && git pull origin main 2>/dev/null || true)
    else
        rm -rf "$INSTALL_SRC"
        mkdir -p "$INSTALL_SRC"
        git clone https://github.com/WhaTheFoxSay/Halca.git "$INSTALL_SRC"
    fi

    echo -e "${CYAN}[+] Compiling Halca Arcade Client binary...${RESET}"
    (cd "$INSTALL_SRC" && cargo build --release --bin client)
    CLIENT_BIN="$INSTALL_SRC/target/release/client"
fi

# Install Executable Binary into all standard BIN directories
TARGET_DIRS=("$CARGO_BIN_DIR" "$LOCAL_BIN_DIR" "$USER_BIN_DIR" "/usr/local/bin")

for TDIR in "${TARGET_DIRS[@]}"; do
    if [ -w "$TDIR" ]; then
        cp "$CLIENT_BIN" "$TDIR/halca" 2>/dev/null || true
        cp "$CLIENT_BIN" "$TDIR/HALCA" 2>/dev/null || true
        chmod +x "$TDIR/halca" 2>/dev/null || true
        chmod +x "$TDIR/HALCA" 2>/dev/null || true
    fi
done

log_installed "APP" "halca_binary"

# Configure PATH and Aliases in active Shell RC files
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
            echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$HOME/bin:$PATH"' >> "$RC_FILE"
            echo 'alias halca="$HOME/.cargo/bin/halca"' >> "$RC_FILE"
            echo 'alias HALCA="$HOME/.cargo/bin/HALCA"' >> "$RC_FILE"
        fi
    fi
done

export PATH="$CARGO_BIN_DIR:$LOCAL_BIN_DIR:$USER_BIN_DIR:/usr/local/bin:$PATH"

echo -e "${GREEN}============================================================${RESET}"
echo -e "${GREEN}   [ HALCA TERMINAL ARCADE INSTALLED SUCCESSFULLY ]         ${RESET}"
echo -e "${GREEN}============================================================${RESET}\n"
echo -e "   ${YELLOW}>>> Ketik 'halca' atau 'HALCA' lalu tekan ENTER! <<<${RESET}"
echo -e "   ${CYAN}>>> (Jika belum terbaca di terminal saat ini, jalankan: source ~/.zshrc) <<<${RESET}\n"
echo -e "${GREEN}============================================================${RESET}\n"

if [ -t 0 ]; then
    read -r -t 3 -p "Tekan ENTER untuk meluncurkan Halca Arcade..." || true
    echo ""
    "$CLIENT_BIN" || true
fi
