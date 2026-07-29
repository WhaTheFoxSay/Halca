# [ HALCA MULTI-GAME TERMINAL ARCADE PLATFORM ]
> **High-Performance Multi-Game Engine & Real-Time Multiplayer Arcade for the Terminal**

[![Rust](https://img.shields.io/badge/language-Rust_2021-orange.svg)](https://www.rust-lang.org/)
[![Networking](https://img.shields.io/badge/networking-Tokio_Async-blue.svg)](https://tokio.rs/)
[![UI Framework](https://img.shields.io/badge/UI-Ratatui_TUI-green.svg)](https://ratatui.rs/)
[![Architecture](https://img.shields.io/badge/architecture-Multi--Game_Hub-purple.svg)](#-architecture--project-structure)

**HALCA Arcade** is an extensible terminal-based multiplayer arcade platform built in Rust. It combines ultra-fast asynchronous TCP networking, clean Cyber ASCII motion graphics, dynamic server connection management, and modular game engines—all accessible directly from your command line.

---

## [===] ⚡ WHY HALCA ARCADE? [===]

* ⚡ **Ultra-Fast 3-Second Setup**: Pre-compiled binary releases for macOS, Linux, and Windows download in under 3 seconds with zero compilation and zero Rust toolchain required.
* 🛡️ **No Administrator Rights Required**: Safe user-level installation on Windows, Linux, and macOS without triggering UAC or root prompts.
* 🌐 **Dynamic Server Connection**: Connect to any game server IP/Domain manually, test TCP connection live, or fallback to Offline Mode (Singleplayer Solo Speed Test).
* 🖥️ **Cyber ASCII Visuals**: High-contrast, dynamic ASCII motion graphics designed specifically for terminal enthusiasts.
* 🎮 **Multi-Game Extensibility**: Modular subfolder architecture (`games/`) allowing endless new games to be added under a single platform launcher.

---

## [===] 🚀 INSTANT ONE-LINE INSTALLATION [===]

### 🍎 Linux & macOS (Bash / Zsh):
Run this single command in your terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/install.sh | bash
```

### 🪟 Windows (PowerShell - No Admin Needed):
Run this single command in PowerShell:
```powershell
iwr -useb https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/install.ps1 | iex
```

### 🎮 How to Launch
After setup completes, simply type `halca` or `HALCA` anywhere in your command prompt / terminal:
```bash
halca
```

---

## [===] 🌐 DYNAMIC SERVER CONNECTION ENGINE [===]

When launching Halca Arcade, the client presents an interactive **Server Connection Setup**:

1. **[ INPUT SERVER ADDRESS ]**: Type any game server IP or Domain (e.g. `10.85.12.2:7777` or `server.halca.net:7777`).
2. **[ TCP CONNECTION TEST ]**: Real-time socket check verifies server availability.
   * **Connected**: Saves configuration to `~/.halca/server_config.json` and opens Arcade Hub.
   * **Connection Failed**: Opens an interactive recovery menu:
     * `[1] RETRY CONNECTION` (Re-test same server)
     * `[2] TRY ANOTHER ADDRESS` (Input new IP/Domain)
     * `[3] PLAY OFFLINE` (Launch Singleplayer Solo Mode)

---

## [===] 🕹️ FEATURED GAMES [===]

### Game 1: Cyber Type Racer (`games/type_racer/`)

A high-speed PvP typing arena designed for 1 to 4 players. Test your typing WPM and accuracy against live opponents over the network or practice solo offline.

* **Singleplayer Solo Speed Test**: Offline WPM & Accuracy training vs AI timer.
* **Custom Room Capacity**: Adjust room sizes from **2 to 4 Players** before entering the match lobby.
* **Live Progress Trackers**: Real-time progress bars, WPM meters, and color-coded character validation (Green = Correct, Red = Typo, Underline = Cursor).
* **Player Profile Customization**: Edit your player nickname directly in the TUI menu.
* **Victory Podium**: Top-3 medal standings (`[GOLD]`, `[SILVER]`, `[BRONZE]`) and full match leaderboard metrics.

---

## [===] ⌨️ CONTROLS & NAVIGATION [===]

| Screen / Context | Key Bindings | Action |
| :--- | :--- | :--- |
| **Server Connection** | `[Key Presses]` | Input Server IP / Domain |
| | `[ENTER]` | Join Server & Test Socket |
| | `[TAB]` | Launch Offline Mode |
| **Arcade Hub Menu** | `[UP / DOWN]` | Navigate Game List |
| | `[ENTER]` | Launch Selected Game |
| | `[C]` | Change Server Address |
| | `[Q] / [ESC]` | Exit Platform |
| **Room Config** | `[LEFT / RIGHT]` | Adjust Capacity (2 - 4 Players) |
| **Typing Race** | `[Key Presses]` | Type Target Quote Prompt |
| | `[BACKSPACE]` | Correct Typo Mistakes |
| | `[ESC]` | Return to Game Menu |
| **Victory Podium** | `[SPACE] / [ENTER]` | Return to Main Menu |

---

## [===] 📂 ARCHITECTURE & PROJECT STRUCTURE [===]

```text
Halca/
├── Cargo.toml                  # Cargo Workspace Configuration
├── install.sh                  # One-Line Curl Installer (Linux & macOS)
├── install.ps1                 # One-Line PowerShell Installer (Windows)
├── uninstall.sh                # Safe Uninstaller Script
├── auto_deploy.sh              # Automated CI/CD Push & Remote Rebuild Script
├── README.md                   # Repository Documentation
├── memahami.md                 # Source of Truth Agentic AI Mandate
├── src/                        # Platform Hub & Launcher
│   ├── lib.rs                  # Module Declarations
│   ├── config.rs               # Server Configuration & TCP Socket Tester
│   └── bin/
│       ├── client.rs           # Central Arcade Client Launcher ('halca' CLI)
│       └── server.rs           # Async TCP Server & Direct HTTP Binary Mirror
└── games/                      # Extensible Games Folder
    └── type_racer/             # Game 1: Cyber Typing Royale
        ├── mod.rs
        ├── models/             # Typing Models & Protocol
        │   ├── typing.rs
        │   └── protocol.rs
        └── ui/                 # ASCII UI Renderers
            ├── motion_animator.rs
            ├── main_menu.rs
            ├── edit_name.rs
            ├── room_config.rs
            ├── lobby.rs
            ├── typing_race.rs
            └── podium.rs
```

---

## [===] 🗑️ SAFE UNINSTALLATION [===]

Halca Arcade guarantees a non-destructive cleanup process. System packages that existed before Halca installation are preserved untouched.

To uninstall Halca Arcade:
1. Launch `halca` -> Select **`[3] UNINSTALL HALCA ARCADE PLATFORM`**, or
2. Run the uninstaller directly:
   ```bash
   ./uninstall.sh
   ```

---

## [===] 🛠️ TECH STACK [===]

* **Language**: Rust (Edition 2021)
* **Terminal UI (TUI)**: `ratatui` (v0.26) & `crossterm` (v0.27)
* **Async Network Engine**: `tokio` (v1.35)
* **Protocol Serialization**: `serde` & `serde_json`

---

## [===] 📄 LICENSE [===]

Developed & Maintained for HALCA Arcade Platform. All rights reserved.
