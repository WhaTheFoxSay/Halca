use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use super::motion_animator::MotionAnimator;

pub struct MainMenuState {
    pub selected_option: usize,
    pub player_name: String,
    pub room_capacity: usize,
    pub tick_frame: u64,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            player_name: "CyberTyper".to_string(),
            room_capacity: 4,
            tick_frame: 0,
        }
    }

    pub fn menu_options() -> Vec<&'static str> {
        vec![
            "[1] SINGLEPLAYER MODE (SOLO SPEED TEST)",
            "[2] MULTIPLAYER MODE (CUSTOM ROOM 2-4 PLAYERS)",
            "[3] EDIT PLAYER NICKNAME",
            "[4] RETURN TO ARCADE HUB MENU",
        ]
    }
}

pub fn draw_main_menu(f: &mut Frame, state: &MainMenuState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header_lines = MotionAnimator::get_animated_header(state.tick_frame);
    let color = MotionAnimator::get_pulsing_color(state.tick_frame);
    let title_text = header_lines.join("\n");
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[1]);

    let items: Vec<ListItem> = MainMenuState::menu_options()
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == state.selected_option {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*opt).style(style)
        })
        .collect();

    let car_motion_str = MotionAnimator::get_animated_car(state.tick_frame, body_chunks[0].width as usize);
    let menu_block = Block::default()
        .title(format!(" [ TYPE RACER MENU ] --- {}", car_motion_str))
        .borders(Borders::ALL);
    let menu_list = List::new(items).block(menu_block);
    f.render_widget(menu_list, body_chunks[0]);

    let profile_text = format!(
        "\n[+] PLAYER NICKNAME:\n    > {}\n\n[+] MULTIPLAYER ROOM CAPACITY:\n    > {} Players (Max 4)\n\n[+] NETWORK SERVER:\n    > 10.85.12.2:7777\n\n[+] HIGH SCORE:\n    > 120 WPM (Personal Best)",
        state.player_name, state.room_capacity
    );
    let profile = Paragraph::new(profile_text)
        .block(Block::default().title(" [ PLAYER PROFILE & CONFIG ] ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(profile, body_chunks[1]);

    let help = Paragraph::new(" [UP/DOWN] Select Menu  |  [ENTER] Confirm  |  [Q] Return to Arcade ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
