use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_edit_name(f: &mut Frame, current_name: &str) {
    let area = centered_rect(60, 25, f.size());
    f.render_widget(Clear, area);

    let popup_block = Block::default()
        .title(" [ EDIT PLAYER NICKNAME ] ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Reset).fg(Color::Yellow));
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

    let prompt = Paragraph::new("Ketik nama panggilan baru kamu di bawah ini:")
        .style(Style::default().fg(Color::White));
    f.render_widget(prompt, chunks[0]);

    let input_text = format!(" > {} _", current_name);
    let input_box = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" [ INPUT NICKNAME ] "));
    f.render_widget(input_box, chunks[1]);

    let help = Paragraph::new(" [ENTER] Save & Back  |  [ESC] Cancel ")
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
