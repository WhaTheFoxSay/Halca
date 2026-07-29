# HALCA ARCADE PLATFORM
> High-Performance Multi-Game Terminal Arcade Platform built in Rust

HALCA Arcade is a lightweight, high-performance, asynchronous terminal multiplayer arcade platform. Designed with a sleek **Cyber ASCII Art** aesthetic, it features real-time TCP socket networking, dynamic ASCII motion animation, and a modular multi-game architecture.

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

## [ QUICK START & INSTALLATION ]

### Option 1: One-Command Automated Setup (Recommended)
Clone the repository and run the automated installer script:
```bash
git clone git@github.com:WhaTheFoxSay/Halca.git
cd Halca
./setup.sh
```

`setup.sh` automatically detects your operating system (macOS / Linux), verifies required C-linker dependencies, installs `rustup` toolchain if missing, compiles the release binary, and launches the arcade launcher!

### Option 2: Manual Build from Source
If you already have Rust and a C-compiler installed:
```bash
# Clone the repository
git clone git@github.com:WhaTheFoxSay/Halca.git
cd Halca

# Build & Run the Central Arcade Client
cargo run --bin client
```

---

## [ ARCHITECTURE & PROJECT STRUCTURE ]

The platform is designed with a modular architecture where each game resides in its own isolated subfolder under `games/`:

```text
Halca/
├── Cargo.toml                  # Workspace & Dependencies Configuration
├── setup.sh                    # Automated One-Command Installer
├── deploy.sh                   # Remote Server Deployment Script
├── README.md                   # Repository Documentation
├── src/                        # Core Engine & Central Arcade Hub
│   ├── lib.rs                  # Module Exposer
│   └── bin/
│       ├── client.rs           # Central Arcade Client Launcher
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
