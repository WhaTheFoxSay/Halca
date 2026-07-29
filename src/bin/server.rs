use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};
use halca_rpg::games::type_racer::models::{
    protocol::{ClientPacket, ServerPacket},
    typing::{PlayerProgress, QuoteGenerator},
};

struct RoomState {
    pub players: HashMap<SocketAddr, PlayerProgress>,
    pub max_capacity: usize,
    pub is_racing: bool,
    pub current_quote: String,
    pub finished_podium: Vec<PlayerProgress>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            max_capacity: 4,
            is_racing: false,
            current_quote: QuoteGenerator::get_random_quote(),
            finished_podium: Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:7777";
    let listener = TcpListener::bind(addr).await?;
    println!("[HALCA ARCADE SERVER] Listening on TCP Port {} (Multi-Game & HTTP Binary Mirror Ready)", addr);

    let room = Arc::new(Mutex::new(RoomState::new()));
    let (tx, _rx) = broadcast::channel::<(String, ServerPacket)>(200);

    loop {
        let (socket, client_addr) = listener.accept().await?;
        let room = room.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = buf_reader.read_line(&mut line) => {
                        if result.unwrap_or(0) == 0 {
                            let mut r = room.lock().unwrap();
                            r.players.remove(&client_addr);
                            let list = r.players.values().cloned().collect::<Vec<_>>();
                            let packet = ServerPacket::LobbyStateBroadcast {
                                players: list,
                                max_capacity: r.max_capacity,
                            };
                            let _ = tx.send((client_addr.to_string(), packet));
                            break;
                        }

                        let trimmed = line.trim();

                        // Serve Fast HTTP Binary Mirror Downloads
                        if trimmed.starts_with("GET /releases/") {
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 {
                                let requested_path = parts[1];
                                let file_path = format!(".{}", requested_path);

                                if let Ok(contents) = std::fs::read(&file_path) {
                                    let header = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                        contents.len()
                                    );
                                    let _ = writer.write_all(header.as_bytes()).await;
                                    let _ = writer.write_all(&contents).await;
                                    let _ = writer.flush().await;
                                    println!("[HTTP MIRROR] Served binary {} ({}) to {}", requested_path, contents.len(), client_addr);
                                    return;
                                }
                            }
                            let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                            let _ = writer.write_all(not_found.as_bytes()).await;
                            let _ = writer.flush().await;
                            return;
                        }

                        // Game TCP Protocol Handling
                        if let Ok(packet) = serde_json::from_str::<ClientPacket>(trimmed) {
                            let mut r = room.lock().unwrap();
                            match packet {
                                ClientPacket::JoinLobby { player_name } => {
                                    if r.players.len() < r.max_capacity {
                                        let player = PlayerProgress::new(client_addr.to_string(), player_name);
                                        r.players.insert(client_addr, player);
                                    }
                                    let list = r.players.values().cloned().collect::<Vec<_>>();
                                    let packet = ServerPacket::LobbyStateBroadcast {
                                        players: list,
                                        max_capacity: r.max_capacity,
                                    };
                                    let _ = tx.send(("SERVER".to_string(), packet));
                                }
                                ClientPacket::SetReadyStatus { is_ready } => {
                                    if let Some(p) = r.players.get_mut(&client_addr) {
                                        p.is_ready = is_ready;
                                    }
                                    let list = r.players.values().cloned().collect::<Vec<_>>();
                                    let ready_count = list.iter().filter(|p| p.is_ready).count();

                                    let packet = ServerPacket::LobbyStateBroadcast {
                                        players: list.clone(),
                                        max_capacity: r.max_capacity,
                                    };
                                    let _ = tx.send(("SERVER".to_string(), packet));

                                    if ready_count >= 1 && ready_count == list.len() && !r.is_racing {
                                        r.is_racing = true;
                                        r.current_quote = QuoteGenerator::get_random_quote();
                                        r.finished_podium.clear();

                                        let start_packet = ServerPacket::RaceStarted {
                                            target_quote: r.current_quote.clone(),
                                        };
                                        let _ = tx.send(("SERVER".to_string(), start_packet));
                                    }
                                }
                                ClientPacket::UpdateTypingProgress { typed_char_count, error_count, elapsed_millis: _ } => {
                                    let total_quote_len = r.current_quote.len().max(1);
                                    let current_podium_count = r.finished_podium.len();
                                    let mut newly_finished: Option<PlayerProgress> = None;

                                    if let Some(p) = r.players.get_mut(&client_addr) {
                                        p.progress_pct = (typed_char_count as f32 / total_quote_len as f32).clamp(0.0, 1.0);
                                        p.accuracy = if typed_char_count > 0 {
                                            let correct = typed_char_count.saturating_sub(error_count);
                                            ((correct as f32 / typed_char_count as f32) * 100.0) as u32
                                        } else {
                                            100
                                        };

                                        if p.progress_pct >= 1.0 && p.finished_place.is_none() {
                                            let place = current_podium_count + 1;
                                            p.finished_place = Some(place);
                                            newly_finished = Some(p.clone());
                                        }
                                    }

                                    if let Some(finished_player) = newly_finished {
                                        r.finished_podium.push(finished_player);
                                        if r.finished_podium.len() == r.players.len() {
                                            r.is_racing = false;
                                            let finish_packet = ServerPacket::RaceFinished {
                                                podium: r.finished_podium.clone(),
                                            };
                                            let _ = tx.send(("SERVER".to_string(), finish_packet));
                                        }
                                    }

                                    let list = r.players.values().cloned().collect::<Vec<_>>();
                                    let update_packet = ServerPacket::RaceStateBroadcast { players: list };
                                    let _ = tx.send(("SERVER".to_string(), update_packet));
                                }
                                ClientPacket::SendPowerUp { target_player_id, powerup_name } => {
                                    let attacker_name = r.players.get(&client_addr).map(|p| p.name.clone()).unwrap_or("Hacker".to_string());
                                    let attack_packet = ServerPacket::PowerUpReceived {
                                        attacker_name,
                                        powerup_name,
                                    };
                                    let _ = tx.send((target_player_id, attack_packet));
                                }
                                ClientPacket::SendLobbyChat { sender, message } => {
                                    let chat_packet = ServerPacket::LobbyChatReceived { sender, message };
                                    let _ = tx.send(("SERVER".to_string(), chat_packet));
                                }
                            }
                        }
                        line.clear();
                    }
                    Ok((_sender_target, packet)) = rx.recv() => {
                        let json = serde_json::to_string(&packet).unwrap() + "\n";
                        let _ = writer.write_all(json.as_bytes()).await;
                    }
                }
            }
        });
    }
}
