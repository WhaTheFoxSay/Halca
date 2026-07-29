# HALCA ARCADE PLATFORM
> High-Performance Multi-Game Terminal Arcade Platform built in Rust

HALCA Arcade is a lightweight, high-performance, asynchronous terminal multiplayer arcade platform. Designed with a sleek **Cyber ASCII Art** aesthetic, it features real-time TCP socket networking, dynamic ASCII motion animation, and a modular multi-game architecture.

---

## [ ONE-LINE QUICK INSTALLATION ]

Install Halca Arcade platform instantly using the automated installer script:

```bash
# One-Line Curl Installer
curl -fsSL https://raw.githubusercontent.com/WhaTheFoxSay/Halca/main/install.sh | bash
```

Or run locally after cloning:
```bash
./install.sh
```

### 🎮 How to Launch
After installation completes successfully, simply open any terminal and type:
```bash
halca
```

---

## [ SAFE UNINSTALLER ]

Halca Arcade respects your system. Pre-existing system dependencies (such as C-compilers or Rust toolchains that were installed before Halca) **will NOT be removed**.

You can uninstall Halca at any time directly from inside the Arcade Hub menu:
1. Launch `halca`
2. Select **`[3] UNINSTALL HALCA ARCADE PLATFORM`**
3. Confirm with `[Y]`.

Or run the uninstaller script directly:
```bash
./uninstall.sh
```

---

## [ AVAILABLE GAMES ]

### Game 1: Type Racer (`games/type_racer/`)
* **Mode**: 4-Player PvP Cyber Typing Royale & Singleplayer Solo Speed Test.
* **Features**:
  * **Interactive Main Menu**: Clean ASCII motion track with real-time animated header.
  * **Dynamic Room Configuration**: Adjust multiplayer room capacity from **2 to 4 Players**.
  * **Player Nickname Customization**: Direct TUI nickname editor.
  * **Real-time Live Progress Track**: Track progress bars, WPM (Words Per Minute), and Accuracy % across all players simultaneously.
  * **Color-Coded Input**: Color feedback (Green for correct, Red for mistakes, Underline for cursor).
  * **Victory Podium**: End-of-match 1st, 2nd, and 3rd rank podium and leaderboard summary.

---

## [ ARCHITECTURE & PROJECT STRUCTURE ]

The platform is designed with a modular architecture where each game resides in its own isolated subfolder under `games/`:

```text
Halca/
├── Cargo.toml                  # Workspace & Dependencies Configuration
├── install.sh                  # One-Line Curl Automated Installer
├── uninstall.sh                # Safe Uninstaller Script
├── auto_deploy.sh              # Automated CI/CD Push & Deploy Script
├── deploy.sh                   # Remote Server Deployment Script
├── README.md                   # Repository Documentation
├── src/                        # Core Engine & Central Arcade Hub
│   ├── lib.rs                  # Module Exposer
│   └── bin/
│       ├── client.rs           # Central Arcade Client Launcher (halca CLI)
│       └── server.rs           # Multi-Game Async TCP Server
└── games/                      # Isolated Game Directory
    └── type_racer/             # Game 1: 4-Player Cyber Type Racer
        ├── mod.rs
        ├── models/             # Typing Models & Protocol
        │   ├── typing.rs
        │   └── protocol.rs
        └── ui/                 # Pure ASCII UI Components
            ├── motion_animator.rs
            ├── main_menu.rs
            ├── edit_name.rs
            ├── room_config.rs
            ├── lobby.rs
            ├── typing_race.rs
            └── podium.rs
```

---

## [ TECHNICAL STACK ]

* **Language**: Rust (Edition 2021)
* **Terminal UI (TUI)**: `ratatui` (v0.26) & `crossterm` (v0.27)
* **Async Networking**: `tokio` (v1.35) TCP Multi-Client Stream
* **Serialization**: `serde` & `serde_json`

---

## [ CONTROLS & NAVIGATION ]

* **Arcade Hub / Main Menu**: `[UP/DOWN]` Navigate Menu, `[ENTER]` Confirm, `[Q]` Exit.
* **Room Capacity Selector**: `[LEFT/RIGHT]` Adjust Capacity (2 to 4 Players).
* **Typing Arena**: Type the displayed target text prompt. Use `[BACKSPACE]` for errors.

---

## [ LICENSE ]
Proprietary / Private Development. All Rights Reserved.
