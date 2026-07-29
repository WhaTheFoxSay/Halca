#!/usr/bin/env bash
set -e

# Load sensitive environment variables if present
if [ -f "$HOME/halca/.env.server" ]; then
    source "$HOME/halca/.env.server"
elif [ -f "./.env.server" ]; then
    source "./.env.server"
fi

COMMIT_MSG="${1:-update: automated sync and deployment}"
SERVER_IP="${SERVER_IP:-127.0.0.1}"
SERVER_USER="${SERVER_USER:-inan}"
SERVER_PASS="${SERVER_PASS:-}"

if [ -z "$SERVER_PASS" ]; then
    echo "[!] ERROR: SERVER_PASS is not set in environment or .env.server!"
    exit 1
fi

echo "============================================================"
echo " 🚀 HALCA AUTOMATED CI/CD: PUSH TO GITHUB & PULL ON SERVER "
echo "============================================================"

# 1. Ensure git changes are committed locally
echo "[+] Checking local git status..."
export GIT_SSH_COMMAND="ssh -i $HOME/.ssh/id_ed25519_halca_deploy -o StrictHostKeyChecking=no"

git add .
if git diff-index --quiet HEAD --; then
    echo "[!] No local changes to commit. Proceeding to sync remote server..."
else
    echo "[+] Committing local changes: $COMMIT_MSG"
    git commit -m "$COMMIT_MSG"
fi

# 2. Push to GitHub Private Repository
echo "[+] Pushing changes to GitHub origin/main..."
git push origin main

# 3. Trigger remote pull & rebuild on production server
echo "[+] Triggering remote git pull & rebuild on server $SERVER_IP..."
sshpass -p "$SERVER_PASS" ssh -o StrictHostKeyChecking=no "$SERVER_USER@$SERVER_IP" "bash -s" << 'EOF'
set -e
cd ~/halca-server

export GIT_SSH_COMMAND="ssh -i ~/.ssh/id_ed25519_halca_server -o StrictHostKeyChecking=no"
echo "[+] Fetching latest code from GitHub Private Repo..."
git fetch origin main
git reset --hard origin/main

source "$HOME/.cargo/env" 2>/dev/null || true
echo "[+] Rebuilding Halca-RPG Game Server..."
cargo build --release --bin server

echo "[+] Restarting Game Server (Port 7777)..."
pkill -f "halca-rpg" || true
nohup ./target/release/server > server.log 2>&1 &

echo "============================================================"
echo " ✅ SUCCESS: AUTOMATED PUSH TO GITHUB & DEPLOYED TO SERVER! "
echo "============================================================"
EOF
