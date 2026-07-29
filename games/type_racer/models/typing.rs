use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerUpType {
    GlitchScreen,
    JammerText,
    FreezeInput,
}

impl PowerUpType {
    pub fn name(&self) -> &'static str {
        match self {
            PowerUpType::GlitchScreen => "[GLITCH SCREEN]",
            PowerUpType::JammerText => "[JAMMER TEXT]",
            PowerUpType::FreezeInput => "[FREEZE INPUT]",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgress {
    pub id: String,
    pub name: String,
    pub progress_pct: f32,
    pub wpm: u32,
    pub accuracy: u32,
    pub is_ready: bool,
    pub finished_place: Option<usize>,
    pub active_sabotage: Option<String>,
}

impl PlayerProgress {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            progress_pct: 0.0,
            wpm: 0,
            accuracy: 100,
            is_ready: false,
            finished_place: None,
            active_sabotage: None,
        }
    }
}

pub struct QuoteGenerator;

impl QuoteGenerator {
    pub fn get_random_quote() -> String {
        let quotes = [
            "The cyber realm rewards the swift and punishes the slow. Type fast, breach the mainframe, and claim the top rank.",
            "Rust async networking is powering this 4-player typing battle across the terminal grid in real time.",
            "System access granted. Overriding firewall security protocols. Execute payload before the timer expires.",
            "In the cyber grid, precision and typing speed determine total control of the system.",
            "Algorithm optimization complete. Synchronizing 4-player race sockets over TCP port 7777 without latency.",
        ];
        let idx = rand::random::<usize>() % quotes.len();
        quotes[idx].to_string()
    }
}
