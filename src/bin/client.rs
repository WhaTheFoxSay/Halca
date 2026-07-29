use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use halca_rpg::games::type_racer::{
    models::typing::PlayerProgress,
    ui::{
        edit_name::draw_edit_name,
        lobby::{draw_lobby, LobbyState},
        main_menu::{draw_main_menu, MainMenuState},
        podium::draw_podium,
        room_config::draw_room_config,
        typing_race::{draw_typing_race, RaceState},
    },
};

pub enum ClientScreen {
    ArcadeHubMenu,
    TypeRacerMainMenu,
    TypeRacerEditName,
    TypeRacerRoomConfig,
    TypeRacerLobby,
    TypeRacerSingleplayerRace,
    TypeRacerMultiplayerRace,
    TypeRacerPodium,
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
            "[3] EXIT HALCA ARCADE PLATFORM",
        ]
    }
}

fn get_server_address() -> String {
    std::env::var("HALCA_SERVER_ADDR").unwrap_or_else(|_| "10.85.12.2:7777".to_string())
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

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    let mut screen = ClientScreen::ArcadeHubMenu;
    let mut arcade_state = ArcadeHubState::new();
    let mut menu_state = MainMenuState::new();
    let server_addr = get_server_address();
    let mut lobby_state = LobbyState::new(server_addr.clone());
    let mut race_state = RaceState::new("The cyber realm rewards the swift and punishes the slow. Type fast and claim victory!".to_string());
    let mut podium_players: Vec<PlayerProgress> = vec![];

    loop {
        menu_state.tick_frame += 1;

        terminal.draw(|f| match screen {
            ClientScreen::ArcadeHubMenu => draw_arcade_hub(f, &arcade_state),
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
                            1 => {}
                            2 => return Ok(()),
                            _ => {}
                        },
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
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
                        KeyCode::Char('q') | KeyCode::Esc => {
                            screen = ClientScreen::ArcadeHubMenu;
                        }
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
                            lobby_state = LobbyState::new(get_server_address());
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
                        KeyCode::Char('q') => return Ok(()),
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
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                }
            }
        }
    }
}

fn draw_arcade_hub(f: &mut Frame, state: &ArcadeHubState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_text = "[===] ==================== [ HALCA MULTI-GAME TERMINAL ARCADE ] ==================== [===]\n               >>> SELECT A GAME TO LAUNCH & PLAY OVER TCP 7777 <<<";
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

    let help = Paragraph::new(" [UP/DOWN] Select Game  |  [ENTER] Launch Game  |  [Q] Exit Arcade ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
