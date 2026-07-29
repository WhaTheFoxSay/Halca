#!/usr/bin/env bash
set -e

if [ -f "./.env.server" ]; then
    source "./.env.server"
fi

SERVER_IP="${SERVER_IP:-127.0.0.1}"
SERVER_USER="${SERVER_USER:-inan}"
SERVER_PASS="${SERVER_PASS:-}"

if [ -z "$SERVER_PASS" ]; then
    echo "[!] ERROR: SERVER_PASS is not set. Please create .env.server file."
    exit 1
fi

echo "🚀 [HALCA DEPLOYMENT] Connecting to remote server $SERVER_IP..."

# 1. Sync source code to server
echo "📦 [HALCA DEPLOYMENT] Syncing source files to $SERVER_IP:~/halca-server..."
sshpass -p "$SERVER_PASS" rsync -avz --exclude 'target' --exclude '.git' --exclude 'memahami.md' --exclude '.env*' ./ $SERVER_USER@$SERVER_IP:~/halca-server/

# 2. Compile and run game server remotely
echo "⚡ [HALCA DEPLOYMENT] Building & Running game server on remote server..."
sshpass -p "$SERVER_PASS" ssh -o StrictHostKeyChecking=no $SERVER_USER@$SERVER_IP "bash -s" << 'EOF'
set -e
cd ~/halca-server

if ! command -v cargo &> /dev/null; then
    echo "⚙️ Installing Rust toolchain on remote server..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

source "$HOME/.cargo/env" 2>/dev/null || true

echo "🔨 Compiling Halca-RPG Game Server..."
cargo build --release --bin server

echo "🟢 Restarting Halca-RPG Game Server (Port 7777)..."
pkill -f "halca-rpg" || true
nohup ./target/release/server > server.log 2>&1 &

echo "✅ Server successfully deployed and running on TCP Port 7777!"
EOF
