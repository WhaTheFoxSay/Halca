#!/usr/bin/env bash
set -e

HALCA_DIR="$HOME/.halca"
MANIFEST_LOG="$HALCA_DIR/install_manifest.log"
BIN_LOCAL="$HOME/.local/bin/halca"
BIN_CARGO="$HOME/.cargo/bin/halca"

echo "============================================================"
echo "   🗑️ UNINSTALLING HALCA MULTI-GAME TERMINAL ARCADE        "
echo "============================================================"

# 1. Remove Halca Binaries
if [ -f "$BIN_LOCAL" ]; then
    echo "[+] Removing Halca executable binary: $BIN_LOCAL"
    rm -f "$BIN_LOCAL"
fi

if [ -f "$BIN_CARGO" ]; then
    echo "[+] Removing Halca executable binary: $BIN_CARGO"
    rm -f "$BIN_CARGO"
fi

# 2. Check Manifest Log for dependencies installed by Halca
if [ -f "$MANIFEST_LOG" ]; then
    echo "[+] Reading installation log manifest..."
    if grep -q "DEP:rustup" "$MANIFEST_LOG"; then
        echo "[!] Rustup was installed by Halca during setup."
        echo "    If you wish to remove rustup as well, run: rustup self uninstall"
    else
        echo "[✓] Rustup was pre-existing on your system. Preserved safely."
    fi
fi

# 3. Clean up Shell RC PATH Entries
for SHELL_RC in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile"; do
    if [ -f "$SHELL_RC" ]; then
        sed -i '' '/Added by Halca Terminal Arcade Installer/d' "$SHELL_RC" 2>/dev/null || sed -i '/Added by Halca Terminal Arcade Installer/d' "$SHELL_RC" 2>/dev/null || true
    fi
done

# 4. Remove Halca Application Directory
if [ -d "$HALCA_DIR" ]; then
    echo "[+] Removing Halca internal directory: $HALCA_DIR"
    rm -rf "$HALCA_DIR"
fi

echo "============================================================"
echo " ✅ HALCA ARCADE HAS BEEN UNINSTALLED SAFELY!               "
echo "    Pre-existing system dependencies remain untouched.       "
echo "============================================================"
