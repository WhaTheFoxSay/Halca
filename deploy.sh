#!/usr/bin/env bash
set -e

SERVER_IP="10.85.12.2"
SERVER_USER="inan"
SERVER_PASS="kadal123"
REMOTE_DIR="~/halca-server"

echo "🚀 [HALCA DEPLOYMENT] Connecting to remote server $SERVER_IP..."

# 1. Ensure remote directory exists
sshpass -p "$SERVER_PASS" ssh -o StrictHostKeyChecking=no "$SERVER_USER@$SERVER_IP" "mkdir -p $REMOTE_DIR"

# 2. Sync project files to remote server
echo "📦 [HALCA DEPLOYMENT] Syncing source files to $SERVER_IP:$REMOTE_DIR..."
sshpass -p "$SERVER_PASS" rsync -avz --exclude 'target' --exclude '.git' ./ "$SERVER_USER@$SERVER_IP:$REMOTE_DIR/"

# 3. Check/Install Rust on remote server & Run Game Server on Port 7777
echo "⚡ [HALCA DEPLOYMENT] Building & Running game server on remote server..."
sshpass -p "$SERVER_PASS" ssh -o StrictHostKeyChecking=no "$SERVER_USER@$SERVER_IP" "bash -s" << 'EOF'
set -e
cd ~/halca-server

if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null; then
    echo "⚙️ Installing build-essential (C linker cc/gcc) on remote server..."
    echo "kadal123" | sudo -S apt-get update -y
    echo "kadal123" | sudo -S apt-get install -y build-essential
fi

if ! command -v cargo &> /dev/null; then
    echo "⚙️ Installing Rust toolchain on remote server..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

source "$HOME/.cargo/env" || true
echo "🔨 Compiling Halca-RPG Game Server..."
cargo build --release --bin server

echo "🟢 Restarting Halca-RPG Game Server (Port 7777)..."
pkill -f "halca-rpg" || true
nohup ./target/release/server > server.log 2>&1 &

echo "✅ Server successfully deployed and running on TCP Port 7777!"
EOF
