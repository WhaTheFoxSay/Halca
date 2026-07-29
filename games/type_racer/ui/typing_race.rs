use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::games::type_racer::models::typing::PlayerProgress;

pub struct RaceState {
    pub target_quote: String,
    pub user_input: String,
    pub start_time: Option<std::time::Instant>,
    pub wpm: u32,
    pub accuracy: u32,
    pub players_progress: Vec<PlayerProgress>,
    pub countdown_sec: Option<u32>,
    pub active_sabotage: Option<String>,
}

impl RaceState {
    pub fn new(target_quote: String) -> Self {
        Self {
            target_quote,
            user_input: String::new(),
            start_time: None,
            wpm: 0,
            accuracy: 100,
            players_progress: vec![],
            countdown_sec: None,
            active_sabotage: None,
        }
    }

    pub fn calculate_metrics(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed_mins = start.elapsed().as_secs_f64() / 60.0;
            if elapsed_mins > 0.0 {
                let words_typed = self.user_input.len() as f64 / 5.0;
                self.wpm = (words_typed / elapsed_mins) as u32;
            }

            let mut correct_chars = 0;
            let target_chars = self.target_quote.chars().collect::<Vec<_>>();
            let input_chars = self.user_input.chars().collect::<Vec<_>>();

            for (i, ch) in input_chars.iter().enumerate() {
                if i < target_chars.len() && *ch == target_chars[i] {
                    correct_chars += 1;
                }
            }

            if !input_chars.is_empty() {
                self.accuracy = ((correct_chars as f64 / input_chars.len() as f64) * 100.0) as u32;
            }
        }
    }
}

pub fn draw_typing_race(f: &mut Frame, state: &RaceState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(f.size());

    let title_text = if let Some(sec) = state.countdown_sec {
        format!("[+] GET READY! MATCH STARTS IN {} SECONDS...", sec)
    } else {
        format!("[+] 4-PLAYER TYPING ROYALE IN PROGRESS! WPM: {} | ACCURACY: {}%", state.wpm, state.accuracy)
    };
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let target_chars = state.target_quote.chars().collect::<Vec<_>>();
    let input_chars = state.user_input.chars().collect::<Vec<_>>();
    let mut spans = Vec::new();

    for (i, ch) in target_chars.iter().enumerate() {
        if i < input_chars.len() {
            if input_chars[i] == *ch {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
            } else {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White).bg(Color::Red).add_modifier(Modifier::BOLD)));
            }
        } else if i == input_chars.len() {
            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)));
        } else {
            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::DarkGray)));
        }
    }

    let quote_paragraph = Paragraph::new(Line::from(spans))
        .wrap(Wrap { trim: true })
        .block(Block::default().title(" [ Target Typing Quote Prompt ] ").borders(Borders::ALL));
    f.render_widget(quote_paragraph, chunks[1]);

    let mut track_items = Vec::new();
    for i in 0..4 {
        if let Some(p) = state.players_progress.get(i) {
            let filled = (p.progress_pct * 30.0) as usize;
            let empty = 30 - filled.min(30);
            let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
            let place_str = p.finished_place.map(|pos| format!("  RANK #{}", pos)).unwrap_or_default();

            let line_str = format!(
                "P{:02} | {:<14} {} {:>3.0}% | {:>3} WPM | ACC: {}%{}",
                i + 1, p.name, bar, p.progress_pct * 100.0, p.wpm, p.accuracy, place_str
            );
            let style = if p.finished_place.is_some() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };
            track_items.push(ListItem::new(line_str).style(style));
        } else {
            track_items.push(ListItem::new(format!("P{:02} | [ EMPTY LANE ]", i + 1)).style(Style::default().fg(Color::DarkGray)));
        }
    }

    let track_list = List::new(track_items)
        .block(Block::default().title(" [ Live 4-Player Race Track ] ").borders(Borders::ALL));
    f.render_widget(track_list, chunks[2]);

    let progress = (state.user_input.len() as f64 / state.target_quote.len() as f64).clamp(0.0, 1.0);
    let gauge = Gauge::default()
        .block(Block::default().title(" [ Your Typing Completion ] ").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent((progress * 100.0) as u16);
    f.render_widget(gauge, chunks[3]);
}
