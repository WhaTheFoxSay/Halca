use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::games::type_racer::models::typing::PlayerProgress;

pub struct LobbyState {
    pub player_name_input: String,
    pub is_ready: bool,
    pub players: Vec<PlayerProgress>,
    pub chat_messages: Vec<(String, String)>,
    pub server_ip: String,
}

impl LobbyState {
    pub fn new(server_ip: String) -> Self {
        Self {
            player_name_input: "CyberTyper".to_string(),
            is_ready: false,
            players: vec![],
            chat_messages: vec![
                ("System".to_string(), "Selamat datang di 4-Player Cyber Typing Royale Server!".to_string()),
                ("System".to_string(), "Tekan [SPACE] untuk toggle status READY.".to_string()),
            ],
            server_ip,
        }
    }
}

pub fn draw_lobby(f: &mut Frame, state: &LobbyState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(3),
        ])
        .split(f.size());

    let ready_count = state.players.iter().filter(|p| p.is_ready).count();
    let title_text = format!(
        "[+] 4-PLAYER PVP CYBER TYPING ROYALE | SERVER: {} | PLAYERS: {}/4 | READY: {}/{}",
        state.server_ip,
        state.players.len(),
        ready_count,
        state.players.len()
    );
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55),
            Constraint::Percentage(45),
        ])
        .split(chunks[1]);

    let mut player_items = Vec::new();
    for i in 0..4 {
        if let Some(p) = state.players.get(i) {
            let ready_str = if p.is_ready { "[ STATUS: READY ]" } else { "[ STATUS: WAITING... ]" };
            let style = if p.is_ready {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            player_items.push(ListItem::new(format!("SLOT {:02} | {:<16} | {}", i + 1, p.name, ready_str)).style(style));
        } else {
            let style = Style::default().fg(Color::DarkGray);
            player_items.push(ListItem::new(format!("SLOT {:02} | [ EMPTY SLOT - WAITING PLAYER ]", i + 1)).style(style));
        }
    }

    let slot_list = List::new(player_items)
        .block(Block::default().title(" [ 4-Player Waiting Room Slots ] ").borders(Borders::ALL));
    f.render_widget(slot_list, body_chunks[0]);

    let mut chat_str = String::new();
    for (sender, msg) in &state.chat_messages {
        chat_str.push_str(&format!("[{}]: {}\n", sender, msg));
    }
    let chat_panel = Paragraph::new(chat_str)
        .wrap(Wrap { trim: true })
        .block(Block::default().title(" [ Live Lobby Chat Box ] ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(chat_panel, body_chunks[1]);

    let footer_text = if state.is_ready {
        " [ STATUS: YOU ARE READY! ]  Wait for all players to start...  |  [SPACE] Cancel Ready  |  [Q] Exit "
    } else {
        " [ STATUS: NOT READY ]  Press [SPACE] to mark yourself as READY!  |  [Q] Exit "
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
