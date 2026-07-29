use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_room_config(f: &mut Frame, capacity: usize) {
    let area = centered_rect(65, 30, f.size());
    f.render_widget(Clear, area);

    let popup_block = Block::default()
        .title(" [ MULTIPLAYER ROOM CAPACITY CONFIG ] ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Reset).fg(Color::Cyan));
    f.render_widget(popup_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(area);

    let prompt = Paragraph::new("Pilih kapasitas jumlah pemain (2 hingga 4 Player) dalam 1 Room Match:")
        .style(Style::default().fg(Color::White));
    f.render_widget(prompt, chunks[0]);

    let bars = format!("[ {} ]", "██ ".repeat(capacity) + &"░░ ".repeat(4 - capacity));
    let selector_text = format!("    < <  [ {} PLAYERS CAPACITY ]  > >\n    {}", capacity, bars);
    let selector_box = Paragraph::new(selector_text)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" [ Use LEFT / RIGHT Arrow Keys ] "));
    f.render_widget(selector_box, chunks[1]);

    let help = Paragraph::new(" [LEFT/RIGHT] Adjust Capacity (2-4)  |  [ENTER] Confirm & Enter Lobby ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
