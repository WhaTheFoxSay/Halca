# 🎮 HALCA TERMINAL ARCADE PLATFORM
> **High-Performance Multi-Game Engine & Real-Time Multiplayer Arcade for the Terminal**

[![Rust](https://img.shields.io/badge/language-Rust_2021-orange.svg)](https://www.rust-lang.org/)
[![Networking](https://img.shields.io/badge/networking-Tokio_Async-blue.svg)](https://tokio.rs/)
[![UI Framework](https://img.shields.io/badge/UI-Ratatui_TUI-green.svg)](https://ratatui.rs/)
[![Architecture](https://img.shields.io/badge/architecture-Multi--Game_Hub-purple.svg)](#-architecture--project-structure)

**HALCA Arcade** is an extensible terminal-based multiplayer arcade platform built in Rust. It combines ultra-fast asynchronous TCP networking, clean Cyber ASCII motion graphics, and modular game engines—all accessible directly from your command line.

---

## ⚡ WHY HALCA ARCADE?

* ⚡ **Blazing Fast**: Engineered in pure Rust with `tokio` async networking for sub-millisecond multiplayer packet broadcasts over TCP Port 7777.
* 🖥️ **Cyber ASCII Visuals**: High-contrast, dynamic ASCII motion graphics designed specifically for terminal enthusiasts. No heavy GUI dependencies required.
* 📦 **Smart 1-Line Installer**: One `curl` script sets up the entire platform and binds the `halca` command to your shell.
* 🛡️ **Safe Dependency Tracking**: Smart installer tracks newly added tools without disturbing pre-existing system packages (`gcc`, `rustup`, etc.).
* 🎮 **Multi-Game Extensibility**: Modular subfolder architecture (`games/`) allowing endless new games to be added under a single platform launcher.

---

## 🚀 QUICK INSTALLATION

Install Halca Arcade in seconds with our automated single-line installer:

```bash
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/install.sh | bash
```

### 🎮 How to Launch
After setup completes, simply type `halca` anywhere in your terminal:
```bash
halca
```

---

## 🕹️ FEATURED GAMES

### 🏎️ Game 1: Cyber Type Racer (`games/type_racer/`)

A high-speed PvP typing arena designed for 1 to 4 players. Test your typing WPM and accuracy against live opponents over the network or practice solo offline.

* **Singleplayer Solo Speed Test**: Offline WPM & Accuracy training vs AI timer.
* **Custom Room Capacity**: Adjust room sizes from **2 to 4 Players** before entering the match lobby.
* **Live Progress Trackers**: Real-time progress bars, WPM meters, and color-coded character validation (Green = Correct, Red = Typo, Underline = Cursor).
* **Player Profile Customization**: Edit your player nickname directly in the TUI menu.
* **Victory Podium**: Top-3 medal standings (`[GOLD]`, `[SILVER]`, `[BRONZE]`) and full match leaderboard metrics.

---

## ⌨️ CONTROLS & NAVIGATION

| Screen / Context | Key Bindings | Action |
| :--- | :--- | :--- |
| **Arcade Hub Menu** | `[UP / DOWN]` | Navigate Game List |
| | `[ENTER]` | Launch Selected Game |
| | `[Q] / [ESC]` | Exit Platform |
| **Room Config** | `[LEFT / RIGHT]` | Adjust Capacity (2 - 4 Players) |
| **Typing Race** | `[Key Presses]` | Type Target Quote Prompt |
| | `[BACKSPACE]` | Correct Typo Mistakes |
| | `[ESC]` | Return to Game Menu |
| **Victory Podium** | `[SPACE] / [ENTER]` | Return to Main Menu |

---

## 📂 ARCHITECTURE & PROJECT STRUCTURE

HALCA Arcade uses an isolated subfolder hierarchy under `games/` to keep game engines decoupled and easily maintainable:

```text
Halca/
├── Cargo.toml                  # Cargo Workspace Configuration
├── install.sh                  # One-Line Curl Automated Installer
├── uninstall.sh                # Safe Uninstaller Script
├── auto_deploy.sh              # Automated CI/CD Push & Remote Rebuild Script
├── deploy.sh                   # Deployment Utility
├── README.md                   # Repository Documentation
├── src/                        # Platform Hub & Launcher
│   ├── lib.rs                  # Module Declarations
│   └── bin/
│       ├── client.rs           # Central Arcade Client Launcher ('halca' CLI)
│       └── server.rs           # Async TCP Server (Port 7777)
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

## 🗑️ SAFE UNINSTALLATION

Halca Arcade guarantees a non-destructive cleanup process. System packages that existed before Halca installation are preserved untouched.

To uninstall Halca Arcade:
1. Launch `halca` -> Select **`[3] UNINSTALL HALCA ARCADE PLATFORM`**, or
2. Run the uninstaller directly:
   ```bash
   ./uninstall.sh
   ```

---

## 🛠️ TECH STACK

* **Language**: Rust (Edition 2021)
* **Terminal UI (TUI)**: `ratatui` (v0.26) & `crossterm` (v0.27)
* **Async Network Engine**: `tokio` (v1.35)
* **Protocol Serialization**: `serde` & `serde_json`

---

## 📄 LICENSE

Developed & Maintained for HALCA Arcade Platform. All rights reserved.
