use ratatui::style::Color;

pub struct MotionAnimator;

impl MotionAnimator {
    pub fn get_pulsing_color(tick: u64) -> Color {
        match (tick / 4) % 6 {
            0 => Color::Cyan,
            1 => Color::Yellow,
            2 => Color::Green,
            3 => Color::Magenta,
            4 => Color::LightRed,
            _ => Color::LightBlue,
        }
    }

    pub fn get_animated_header(tick: u64) -> Vec<String> {
        let frame = (tick / 3) % 3;
        match frame {
            0 => vec![
                "[===] ==================== [ CYBER TYPING ROYALE ] ==================== [===]".to_string(),
                "              >>> PRECISION AND SPEED DETERMINE SURVIVAL <<<                 ".to_string(),
            ],
            1 => vec![
                "<<<>> ==================== [ CYBER TYPING ROYALE ] ==================== <<<<>".to_string(),
                "              >>> TYPE FAST - BREACH THE MAINFRAME SYSTEM <<<                ".to_string(),
            ],
            _ => vec![
                "[***] ==================== [ CYBER TYPING ROYALE ] ==================== [***]".to_string(),
                "              >>> 4-PLAYER MULTIPLAYER ARENA OVER TCP 7777 <<<               ".to_string(),
            ],
        }
    }

    pub fn get_animated_car(tick: u64, width: usize) -> String {
        let max_pos = width.saturating_sub(30).max(1);
        let pos = (tick as usize * 2) % max_pos;
        let left_padding = " ".repeat(pos);
        let right_padding = " ".repeat(max_pos.saturating_sub(pos));

        format!("{}===> [CYBER-RACER] ===>{}", left_padding, right_padding)
    }
}
