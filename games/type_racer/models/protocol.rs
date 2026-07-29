use serde::{Deserialize, Serialize};
use super::typing::PlayerProgress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientPacket {
    JoinLobby {
        player_name: String,
    },
    SetReadyStatus {
        is_ready: bool,
    },
    UpdateTypingProgress {
        typed_char_count: usize,
        error_count: usize,
        elapsed_millis: u128,
    },
    SendPowerUp {
        target_player_id: String,
        powerup_name: String,
    },
    SendLobbyChat {
        sender: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerPacket {
    LobbyStateBroadcast {
        players: Vec<PlayerProgress>,
        max_capacity: usize,
    },
    CountdownTimer {
        seconds_left: u32,
    },
    RaceStarted {
        target_quote: String,
    },
    RaceStateBroadcast {
        players: Vec<PlayerProgress>,
    },
    PowerUpReceived {
        attacker_name: String,
        powerup_name: String,
    },
    LobbyChatReceived {
        sender: String,
        message: String,
    },
    RaceFinished {
        podium: Vec<PlayerProgress>,
    },
}
