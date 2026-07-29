#!/usr/bin/env bash
set -e

echo "============================================================"
echo "   [ HALCA ARCADE PRE-COMPILED RELEASE BINARY BUILDER ]     "
echo "============================================================"

mkdir -p releases

echo "[+] Compiling release binary for current platform..."
cargo build --release --bin client

UNAME_OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
UNAME_ARCH="$(uname -m)"

case "$UNAME_ARCH" in
    x86_64|amd64) TARGET_ARCH="x86_64" ;;
    arm64|aarch64) TARGET_ARCH="aarch64" ;;
    *) TARGET_ARCH="$UNAME_ARCH" ;;
esac

BINARY_NAME="halca-${UNAME_OS}-${TARGET_ARCH}"
if [ "$UNAME_OS" = "windows" ] || [[ "$UNAME_OS" == mingw* ]] || [[ "$UNAME_OS" == cygwin* ]]; then
    BINARY_NAME="halca-windows-${TARGET_ARCH}.exe"
fi

cp target/release/client "releases/$BINARY_NAME"
echo "[✓] Pre-compiled binary created: releases/$BINARY_NAME"

ls -lh releases/
