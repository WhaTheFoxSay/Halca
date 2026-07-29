use std::{
    error::Error,
    io,
    process::Command,
    time::{Duration, Instant},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame, Terminal,
};
use halca_rpg::{
    config::{load_server_config, save_server_config, test_server_connection},
    games::type_racer::{
        models::typing::PlayerProgress,
        ui::{
            edit_name::draw_edit_name,
            lobby::{draw_lobby, LobbyState},
            main_menu::{draw_main_menu, MainMenuState},
            podium::draw_podium,
            room_config::draw_room_config,
            typing_race::{draw_typing_race, RaceState},
        },
    },
};

pub enum ClientScreen {
    ServerConnectInput,
    ServerConnectFailed,
    ArcadeHubMenu,
    UninstallConfirm,
    TypeRacerMainMenu,
    TypeRacerEditName,
    TypeRacerRoomConfig,
    TypeRacerLobby,
    TypeRacerSingleplayerRace,
    TypeRacerMultiplayerRace,
    TypeRacerPodium,
}

pub struct ServerConnectState {
    pub server_input: String,
    pub is_online: bool,
    pub failed_option: usize,
    pub status_msg: String,
}

impl ServerConnectState {
    pub fn new() -> Self {
        let config = load_server_config();
        Self {
            server_input: config.server_address,
            is_online: false,
            failed_option: 0,
            status_msg: String::new(),
        }
    }
}

pub struct ArcadeHubState {
    pub selected_game: usize,
}

impl ArcadeHubState {
    pub fn new() -> Self {
        Self { selected_game: 0 }
    }

    pub fn games_list() -> Vec<&'static str> {
        vec![
            "[1] GAME 1: CYBER TYPE RACER (4-Player Typing Royale)",
            "[2] GAME 2: COMING SOON (Next Terminal Game)",
            "[3] UNINSTALL HALCA ARCADE PLATFORM",
            "[4] EXIT HALCA ARCADE PLATFORM",
        ]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(should_uninstall) = res {
        if should_uninstall {
            println!("\n[+] Initiating safe uninstaller script...");
            let home = std::env::var("HOME").unwrap_or_default();
            let uninstaller = format!("{}/.halca/source/uninstall.sh", home);

            let status = if std::path::Path::new(&uninstaller).exists() {
                Command::new("bash").arg(uninstaller).status()
            } else if std::path::Path::new("./uninstall.sh").exists() {
                Command::new("bash").arg("./uninstall.sh").status()
            } else {
                Ok(std::process::ExitStatus::default())
            };

            if let Err(e) = status {
                println!("[!] Uninstall error: {:?}", e);
            }
        }
    } else if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<bool, Box<dyn Error>> {
    let mut screen = ClientScreen::ServerConnectInput;
    let mut connect_state = ServerConnectState::new();
    let mut arcade_state = ArcadeHubState::new();
    let mut menu_state = MainMenuState::new();
    let mut lobby_state = LobbyState::new(connect_state.server_input.clone());
    let mut race_state = RaceState::new("The cyber realm rewards the swift and punishes the slow. Type fast and claim victory!".to_string());
    let mut podium_players: Vec<PlayerProgress> = vec![];

    loop {
        menu_state.tick_frame += 1;

        terminal.draw(|f| match screen {
            ClientScreen::ServerConnectInput => draw_server_connect_input(f, &connect_state),
            ClientScreen::ServerConnectFailed => {
                draw_server_connect_input(f, &connect_state);
                draw_server_connect_failed(f, &connect_state);
            }
            ClientScreen::ArcadeHubMenu => draw_arcade_hub(f, &arcade_state, &connect_state),
            ClientScreen::UninstallConfirm => {
                draw_arcade_hub(f, &arcade_state, &connect_state);
                draw_uninstall_confirm(f);
            }
            ClientScreen::TypeRacerMainMenu => draw_main_menu(f, &menu_state),
            ClientScreen::TypeRacerEditName => {
                draw_main_menu(f, &menu_state);
                draw_edit_name(f, &menu_state.player_name);
            }
            ClientScreen::TypeRacerRoomConfig => {
                draw_main_menu(f, &menu_state);
                draw_room_config(f, menu_state.room_capacity);
            }
            ClientScreen::TypeRacerLobby => draw_lobby(f, &lobby_state),
            ClientScreen::TypeRacerSingleplayerRace | ClientScreen::TypeRacerMultiplayerRace => draw_typing_race(f, &race_state),
            ClientScreen::TypeRacerPodium => draw_podium(f, &podium_players),
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match screen {
                    ClientScreen::ServerConnectInput => match key.code {
                        KeyCode::Enter => {
                            let target = connect_state.server_input.trim().to_string();
                            if !target.is_empty() {
                                if test_server_connection(&target) {
                                    save_server_config(&target);
                                    connect_state.is_online = true;
                                    connect_state.status_msg = format!("CONNECTED TO {}", target);
                                    lobby_state = LobbyState::new(target);
                                    screen = ClientScreen::ArcadeHubMenu;
                                } else {
                                    connect_state.is_online = false;
                                    screen = ClientScreen::ServerConnectFailed;
                                }
                            }
                        }
                        KeyCode::Tab => {
                            connect_state.is_online = false;
                            connect_state.status_msg = "OFFLINE MODE".to_string();
                            screen = ClientScreen::ArcadeHubMenu;
                        }
                        KeyCode::Esc => return Ok(false),
                        KeyCode::Backspace => {
                            connect_state.server_input.pop();
                        }
                        KeyCode::Char(c) => {
                            if connect_state.server_input.len() < 40 {
                                connect_state.server_input.push(c);
                            }
                        }
                        _ => {}
                    },
                    ClientScreen::ServerConnectFailed => match key.code {
                        KeyCode::Up => {
                            if connect_state.failed_option > 0 {
                                connect_state.failed_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if connect_state.failed_option < 2 {
                                connect_state.failed_option += 1;
                            }
                        }
                        KeyCode::Enter => match connect_state.failed_option {
                            0 => {
                                let target = connect_state.server_input.trim().to_string();
                                if test_server_connection(&target) {
                                    save_server_config(&target);
                                    connect_state.is_online = true;
                                    connect_state.status_msg = format!("CONNECTED TO {}", target);
                                    lobby_state = LobbyState::new(target);
                                    screen = ClientScreen::ArcadeHubMenu;
                                } else {
                                    connect_state.is_online = false;
                                    screen = ClientScreen::ServerConnectFailed;
                                }
                            }
                            1 => {
                                screen = ClientScreen::ServerConnectInput;
                            }
                            2 => {
                                connect_state.is_online = false;
                                connect_state.status_msg = "OFFLINE MODE".to_string();
                                screen = ClientScreen::ArcadeHubMenu;
                            }
                            _ => {}
                        },
                        KeyCode::Esc => screen = ClientScreen::ServerConnectInput,
                        _ => {}
                    },
                    ClientScreen::ArcadeHubMenu => match key.code {
                        KeyCode::Up => {
                            if arcade_state.selected_game > 0 {
                                arcade_state.selected_game -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if arcade_state.selected_game + 1 < ArcadeHubState::games_list().len() {
                                arcade_state.selected_game += 1;
                            }
                        }
                        KeyCode::Enter => match arcade_state.selected_game {
                            0 => screen = ClientScreen::TypeRacerMainMenu,
                            1 => {} // Coming Soon
                            2 => screen = ClientScreen::UninstallConfirm,
                            3 => return Ok(false),
                            _ => {}
                        },
                        KeyCode::Char('c') | KeyCode::Char('C') => screen = ClientScreen::ServerConnectInput,
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(false),
                        _ => {}
                    },
                    ClientScreen::UninstallConfirm => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            screen = ClientScreen::ArcadeHubMenu;
                        }
                        _ => {}
                    },
                    ClientScreen::TypeRacerMainMenu => match key.code {
                        KeyCode::Up => {
                            if menu_state.selected_option > 0 {
                                menu_state.selected_option -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if menu_state.selected_option + 1 < MainMenuState::menu_options().len() {
                                menu_state.selected_option += 1;
                            }
                        }
                        KeyCode::Enter => match menu_state.selected_option {
                            0 => {
                                race_state = RaceState::new("Singleplayer Speed Test: Practice typing fast to improve your WPM score!".to_string());
                                race_state.start_time = Some(Instant::now());
                                race_state.players_progress = vec![
                                    PlayerProgress::new("1".to_string(), menu_state.player_name.clone()),
                                    PlayerProgress::new("2".to_string(), "AI_Bot_Easy".to_string()),
                                ];
                                screen = ClientScreen::TypeRacerSingleplayerRace;
                            }
                            1 => screen = ClientScreen::TypeRacerRoomConfig,
                            2 => screen = ClientScreen::TypeRacerEditName,
                            3 => screen = ClientScreen::ArcadeHubMenu,
                            _ => {}
                        },
                        KeyCode::Char('q') | KeyCode::Esc => screen = ClientScreen::ArcadeHubMenu,
                        _ => {}
                    },
                    ClientScreen::TypeRacerEditName => match key.code {
                        KeyCode::Char(c) => {
                            if menu_state.player_name.len() < 16 {
                                menu_state.player_name.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            menu_state.player_name.pop();
                        }
                        KeyCode::Enter | KeyCode::Esc => {
                            if menu_state.player_name.trim().is_empty() {
                                menu_state.player_name = "CyberTyper".to_string();
                            }
                            screen = ClientScreen::TypeRacerMainMenu;
                        }
                        _ => {}
                    },
                    ClientScreen::TypeRacerRoomConfig => match key.code {
                        KeyCode::Left => {
                            if menu_state.room_capacity > 2 {
                                menu_state.room_capacity -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if menu_state.room_capacity < 4 {
                                menu_state.room_capacity += 1;
                            }
                        }
                        KeyCode::Enter => {
                            lobby_state = LobbyState::new(connect_state.server_input.clone());
                            lobby_state.players = vec![
                                PlayerProgress::new("1".to_string(), menu_state.player_name.clone()),
                            ];
                            screen = ClientScreen::TypeRacerLobby;
                        }
                        KeyCode::Esc => screen = ClientScreen::TypeRacerMainMenu,
                        _ => {}
                    },
                    ClientScreen::TypeRacerLobby => match key.code {
                        KeyCode::Char(' ') => {
                            lobby_state.is_ready = !lobby_state.is_ready;
                            if let Some(p) = lobby_state.players.first_mut() {
                                p.is_ready = lobby_state.is_ready;
                            }

                            if lobby_state.is_ready {
                                race_state = RaceState::new("The cyber realm rewards the swift and punishes the slow. Type fast and claim victory!".to_string());
                                race_state.start_time = Some(Instant::now());
                                race_state.players_progress = lobby_state.players.clone();
                                screen = ClientScreen::TypeRacerMultiplayerRace;
                            }
                        }
                        KeyCode::Esc => screen = ClientScreen::TypeRacerMainMenu,
                        KeyCode::Char('q') => return Ok(false),
                        _ => {}
                    },
                    ClientScreen::TypeRacerSingleplayerRace | ClientScreen::TypeRacerMultiplayerRace => match key.code {
                        KeyCode::Char(c) => {
                            if race_state.user_input.len() < race_state.target_quote.len() {
                                race_state.user_input.push(c);
                                race_state.calculate_metrics();

                                if let Some(p) = race_state.players_progress.first_mut() {
                                    p.progress_pct = (race_state.user_input.len() as f32 / race_state.target_quote.len() as f32).clamp(0.0, 1.0);
                                    p.wpm = race_state.wpm;
                                    p.accuracy = race_state.accuracy;
                                }

                                if race_state.user_input.len() >= race_state.target_quote.len() {
                                    podium_players = race_state.players_progress.clone();
                                    screen = ClientScreen::TypeRacerPodium;
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            race_state.user_input.pop();
                            race_state.calculate_metrics();
                        }
                        KeyCode::Esc => screen = ClientScreen::TypeRacerMainMenu,
                        _ => {}
                    },
                    ClientScreen::TypeRacerPodium => match key.code {
                        KeyCode::Char(' ') | KeyCode::Enter => screen = ClientScreen::TypeRacerMainMenu,
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(false),
                        _ => {}
                    },
                }
            }
        }
    }
}

fn draw_server_connect_input(f: &mut Frame, state: &ServerConnectState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_text = "[===] ==================== [ HALCA SERVER CONNECTION SETUP ] ==================== [===]\n               >>> ENTER GAME SERVER ADDRESS TO CONNECT AND PLAY <<<";
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let input_area = centered_rect(70, 45, chunks[1]);
    let input_text = format!("\n  Masukkan Alamat IP / Domain Server Tujuan:\n\n  > {} _\n\n  [Contoh: 10.85.12.2:7777 atau server.halca.net:7777]", state.server_input);
    let input_box = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" [ INPUT SERVER ADDRESS & PORT ] "));
    f.render_widget(input_box, input_area);

    let help = Paragraph::new(" [ENTER] Join Server  |  [TAB] Offline Mode (Singleplayer)  |  [ESC] Exit ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn draw_server_connect_failed(f: &mut Frame, state: &ServerConnectState) {
    let area = centered_rect(65, 45, f.size());
    f.render_widget(Clear, area);

    let popup_block = Block::default()
        .title(" [ SERVER CONNECTION FAILED ] ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Reset).fg(Color::Red));
    f.render_widget(popup_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(2),
        ])
        .split(area);

    let err_msg = format!("Gagal terhubung ke server [{}]\nServer sedang offline atau alamat IP/Port salah.", state.server_input);
    let prompt = Paragraph::new(err_msg)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD));
    f.render_widget(prompt, chunks[0]);

    let failed_options = [
        "[1] RETRY CONNECTION (Coba Konek Lagi)",
        "[2] TRY ANOTHER ADDRESS (Input IP / Domain Lain)",
        "[3] PLAY OFFLINE (Singleplayer Solo Speed Test)",
    ];

    let items: Vec<ListItem> = failed_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == state.failed_option {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*opt).style(style)
        })
        .collect();

    let option_list = List::new(items)
        .block(Block::default().title(" [ Choose Action ] ").borders(Borders::ALL));
    f.render_widget(option_list, chunks[1]);

    let help = Paragraph::new(" [UP/DOWN] Select Option  |  [ENTER] Confirm Choice ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_arcade_hub(f: &mut Frame, state: &ArcadeHubState, conn_state: &ServerConnectState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(f.size());

    let status_str = if conn_state.is_online {
        format!("ONLINE SERVER: {}", conn_state.server_input)
    } else {
        "MODE: OFFLINE (SINGLEPLAYER ONLY)".to_string()
    };

    let title_text = format!(
        "[===] ==================== [ HALCA MULTI-GAME TERMINAL ARCADE ] ==================== [===]\n               >>> STATUS: {} <<<",
        status_str
    );
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = ArcadeHubState::games_list()
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let style = if i == state.selected_game {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*g).style(style)
        })
        .collect();

    let games_list = List::new(items)
        .block(Block::default().title(" [ AVAILABLE ARCADE GAMES ] ").borders(Borders::ALL));
    f.render_widget(games_list, chunks[1]);

    let help = Paragraph::new(" [UP/DOWN] Select Game  |  [ENTER] Confirm  |  [C] Change Server  |  [Q] Exit ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

fn draw_uninstall_confirm(f: &mut Frame) {
    let area = centered_rect(65, 30, f.size());
    f.render_widget(Clear, area);

    let popup_block = Block::default()
        .title(" [ UNINSTALL HALCA ARCADE PLATFORM ] ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Reset).fg(Color::Red));
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

    let prompt = Paragraph::new("Apakah Anda yakin ingin menghapus Halca Arcade dari sistem Anda?")
        .style(Style::default().fg(Color::White));
    f.render_widget(prompt, chunks[0]);

    let warn_box = Paragraph::new("Proses ini hanya akan menghapus binary 'halca' dan folder aplikasi.\nDependency bawaan sistem tidak akan disentuh.")
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" [ Safe Uninstaller Guarantee ] "));
    f.render_widget(warn_box, chunks[1]);

    let help = Paragraph::new(" Press [Y] to Confirm Uninstall  |  Press [N/Esc] to Cancel ")
        .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
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
