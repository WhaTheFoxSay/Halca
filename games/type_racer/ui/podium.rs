use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::games::type_racer::models::typing::PlayerProgress;

pub fn draw_podium(f: &mut Frame, podium: &[PlayerProgress]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("[+] 4-PLAYER TYPING ROYALE MATCH FINISHED [+]")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(55),
        ])
        .split(chunks[1]);

    let first = podium.get(0).map(|p| p.name.as_str()).unwrap_or("-");
    let second = podium.get(1).map(|p| p.name.as_str()).unwrap_or("-");
    let third = podium.get(2).map(|p| p.name.as_str()).unwrap_or("-");

    let podium_ascii = format!(
        "\n\n\
                 [ 1ST PLACE - GOLD ]\n\
                     [ {} ]\n\
                    ┌───────┐\n\
      [ 2ND PLACE ] │   1   │ [ 3RD PLACE ]\n\
      [ {} ]    │ GOLD  │    [ {} ]\n\
        ┌───────┐   │       │   ┌───────┐\n\
        │   2   │   │       │   │   3   │\n\
        │SILVER │   │       │   │BRONZE │\n\
        └───────┴───┴───────┴───┴───────┘\n",
        first, second, third
    );
    let banner = Paragraph::new(podium_ascii)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().title(" [ VICTORY PODIUM ] ").borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(banner, body_chunks[0]);

    let mut items = Vec::new();
    for (i, p) in podium.iter().enumerate() {
        let medal = match i {
            0 => "[GOLD]",
            1 => "[SILVER]",
            2 => "[BRONZE]",
            _ => "[RANK]",
        };
        let line_str = format!(
            "RANK {:02} | {:<16} | {:<10} | {:>3} WPM | ACC: {}%",
            i + 1, p.name, medal, p.wpm, p.accuracy
        );
        let style = match i {
            0 => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            1 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
            2 => Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White),
        };
        items.push(ListItem::new(line_str).style(style));
    }

    let rank_list = List::new(items)
        .block(Block::default().title(" [ Full Match Leaderboard ] ").borders(Borders::ALL));
    f.render_widget(rank_list, body_chunks[1]);

    let help = Paragraph::new(" Press [SPACE] to return to Main Menu  |  [Q] Return to Arcade ")
        .style(Style::default().fg(Color::Green))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
