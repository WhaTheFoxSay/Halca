#!/usr/bin/env bash
set -e

HALCA_DIR="$HOME/.halca"
MANIFEST_LOG="$HALCA_DIR/install_manifest.log"
BIN_LOCAL="$HOME/.local/bin/halca"
BIN_CARGO="$HOME/.cargo/bin/halca"
BIN_LOCAL_CAPS="$HOME/.local/bin/HALCA"
BIN_CARGO_CAPS="$HOME/.cargo/bin/HALCA"

echo "============================================================"
echo "   [ UNINSTALLING HALCA MULTI-GAME TERMINAL ARCADE ]       "
echo "============================================================"

# 1. Remove Halca Executable Binaries
for TARGET_BIN in "$BIN_LOCAL" "$BIN_CARGO" "$BIN_LOCAL_CAPS" "$BIN_CARGO_CAPS"; do
    if [ -f "$TARGET_BIN" ]; then
        echo "[+] Removing Halca executable binary: $TARGET_BIN"
        rm -f "$TARGET_BIN"
    fi
done

# 2. Check Manifest Log for Dependencies Installed by Halca
if [ -f "$MANIFEST_LOG" ]; then
    echo "[+] Inspecting background installation log manifest..."
    if grep -q "DEP:rustup" "$MANIFEST_LOG"; then
        echo "[!] Rustup toolchain was originally installed by Halca during setup."
        echo "    If you wish to remove rustup as well, execute: rustup self uninstall"
    else
        echo "[✓] Rustup toolchain was pre-existing on your device. Preserved safely."
    fi
fi

# 3. Clean up Shell RC PATH Entries
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
        sed -i '' '/Added by Halca Terminal Arcade Installer/d' "$RC_FILE" 2>/dev/null || sed -i '/Added by Halca Terminal Arcade Installer/d' "$RC_FILE" 2>/dev/null || true
    fi
done

# 4. Remove Halca Internal Directory
if [ -d "$HALCA_DIR" ]; then
    echo "[+] Removing Halca internal application directory: $HALCA_DIR"
    rm -rf "$HALCA_DIR"
fi

echo "============================================================"
echo "   [ HALCA ARCADE HAS BEEN UNINSTALLED SAFELY ]             "
echo "   Pre-existing system dependencies remain untouched.       "
echo "============================================================"
