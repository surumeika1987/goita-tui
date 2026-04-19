use goita::{ApplyResult, BoardDirection};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::{error::Error, io};

mod app;
mod ui;

use crate::app::{App, CurrentScreen, GameSelection, GameSettingSelection, TitleSelection};
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Title(selection) => {
                    if title_key_event_hadler(app, key.code, selection) {
                        break;
                    }
                }
                CurrentScreen::GameSettings(selection) => {
                    game_settings_key_event_handler(app, key.code, selection)
                }
                CurrentScreen::Game(selection) => {
                    game_key_event_handler(app, key.code, selection);
                }
                CurrentScreen::RoundOver(_) => {
                    round_over_key_event_handler(app, key.code);
                }
                CurrentScreen::GameOver(_) => {
                    game_over_key_event_handler(app, key.code);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn title_key_event_hadler(app: &mut App, key_code: KeyCode, selection: TitleSelection) -> bool {
    match key_code {
        KeyCode::Up => app.current_screen = CurrentScreen::Title(selection.previous()),
        KeyCode::Down => app.current_screen = CurrentScreen::Title(selection.next()),
        KeyCode::Enter => match selection {
            TitleSelection::Start => {
                app.current_screen = CurrentScreen::GameSettings(GameSettingSelection::Player1)
            }
            TitleSelection::Settings => {}
            TitleSelection::Rules => {}
            TitleSelection::Exit => return true,
        },
        KeyCode::Esc => return true,
        _ => {}
    }

    false
}

fn game_settings_key_event_handler(
    app: &mut App,
    key_code: KeyCode,
    selection: GameSettingSelection,
) {
    match key_code {
        KeyCode::Up => app.current_screen = CurrentScreen::GameSettings(selection.previous()),
        KeyCode::Down => app.current_screen = CurrentScreen::GameSettings(selection.next()),
        KeyCode::Left => match selection {
            GameSettingSelection::Player1 => {
                change_player_settings_preve(app, BoardDirection::North)
            }
            GameSettingSelection::Player2 => {
                change_player_settings_preve(app, BoardDirection::East)
            }
            GameSettingSelection::Player3 => {
                change_player_settings_preve(app, BoardDirection::South)
            }
            GameSettingSelection::Player4 => {
                change_player_settings_preve(app, BoardDirection::West)
            }
            GameSettingSelection::InitialTurnPlayer => change_init_turn_player_preve(app),
            GameSettingSelection::WinningScore => change_winning_score_decrase(app),
            GameSettingSelection::Start => {}
        },
        KeyCode::Right => match selection {
            GameSettingSelection::Player1 => {
                change_player_settings_next(app, BoardDirection::North)
            }
            GameSettingSelection::Player2 => change_player_settings_next(app, BoardDirection::East),
            GameSettingSelection::Player3 => {
                change_player_settings_next(app, BoardDirection::South)
            }
            GameSettingSelection::Player4 => change_player_settings_next(app, BoardDirection::West),
            GameSettingSelection::InitialTurnPlayer => change_init_turn_player_next(app),
            GameSettingSelection::WinningScore => change_winning_score_increase(app),
            GameSettingSelection::Start => {}
        },
        KeyCode::Enter => match selection {
            GameSettingSelection::Start => {
                start_new_game(app);
            }
            _ => {}
        },
        KeyCode::Esc => app.current_screen = CurrentScreen::Title(TitleSelection::Start),
        _ => {}
    }
}

fn change_player_settings_next(app: &mut App, player: BoardDirection) {
    match player {
        BoardDirection::North => app.game_setting.player1 = app.game_setting.player1.next(),
        BoardDirection::East => app.game_setting.player2 = app.game_setting.player2.next(),
        BoardDirection::South => app.game_setting.player3 = app.game_setting.player3.next(),
        BoardDirection::West => app.game_setting.player4 = app.game_setting.player4.next(),
    }
}

fn change_player_settings_preve(app: &mut App, player: BoardDirection) {
    match player {
        BoardDirection::North => app.game_setting.player1 = app.game_setting.player1.previous(),
        BoardDirection::East => app.game_setting.player2 = app.game_setting.player2.previous(),
        BoardDirection::South => app.game_setting.player3 = app.game_setting.player3.previous(),
        BoardDirection::West => app.game_setting.player4 = app.game_setting.player4.previous(),
    }
}

fn change_init_turn_player_next(app: &mut App) {
    app.game_setting.initial_turn_player = match app.game_setting.initial_turn_player {
        BoardDirection::North => BoardDirection::East,
        BoardDirection::East => BoardDirection::South,
        BoardDirection::South => BoardDirection::West,
        BoardDirection::West => BoardDirection::North,
    }
}

fn change_init_turn_player_preve(app: &mut App) {
    app.game_setting.initial_turn_player = match app.game_setting.initial_turn_player {
        BoardDirection::North => BoardDirection::West,
        BoardDirection::East => BoardDirection::North,
        BoardDirection::South => BoardDirection::East,
        BoardDirection::West => BoardDirection::South,
    }
}

fn change_winning_score_increase(app: &mut App) {
    if app.game_setting.winning_score < 990 {
        app.game_setting.winning_score += 10;
    }
}

fn change_winning_score_decrase(app: &mut App) {
    if app.game_setting.winning_score > 10 {
        app.game_setting.winning_score -= 10;
    }
}

fn game_key_event_handler(app: &mut App, key_code: KeyCode, selection: GameSelection) {
    match key_code {
        KeyCode::Up => app.current_screen = CurrentScreen::Game(selection.vertical_previous()),
        KeyCode::Down => app.current_screen = CurrentScreen::Game(selection.vertical_next()),
        KeyCode::Left => app.current_screen = CurrentScreen::Game(selection.horizontal_previous()),
        KeyCode::Right => app.current_screen = CurrentScreen::Game(selection.horizontal_next()),
        KeyCode::Enter => match selection {
            GameSelection::Top(_) | GameSelection::Bottom(_) => {
                let selection = match selection {
                    GameSelection::Top(index) => index * 2,
                    GameSelection::Bottom(index) => index * 2 + 1,
                    GameSelection::Pass => unreachable!(),
                };
                place_piece(app, selection);
            }
            GameSelection::Pass => {
                pass_turn(app);
            }
        },
        KeyCode::Char(' ') => {
            pass_turn(app);
        }
        KeyCode::Backspace => {
            app.revert_view_hand();
        }
        // Debug
        KeyCode::Char('q') => app.current_screen = CurrentScreen::Title(TitleSelection::Start),
        _ => {}
    }
}

fn pass_turn(app: &mut App) {
    app.pass_turn();
}

fn place_piece(app: &mut App, selection: u8) {
    if let Some(result) = app.place_piece(selection) {
        match result {
            ApplyResult::Continuing => {}
            ApplyResult::RoundOver(round_result) => {
                app.current_screen = CurrentScreen::RoundOver(round_result);
            }
        }
    }
}

fn start_new_game(app: &mut App) {
    app.start_new_game();
    start_new_round(app);
}

fn start_new_round(app: &mut App) {
    let deal_event = app.start_new_round();
    // TODO: 配牌イベントをハンドリングする
    if let Some(current_turn_player) = app.current_turn_player() {
        app.sync_hand(current_turn_player);
    }
    app.sync_board();
    app.current_screen = CurrentScreen::Game(GameSelection::Top(0));
}

fn round_over_key_event_handler(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            if let Some(result) = app.game_result() {
                app.current_screen = CurrentScreen::GameOver(result);
            } else {
                start_new_round(app);
            }
        }
        _ => {}
    }
}

fn game_over_key_event_handler(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            app.current_screen = CurrentScreen::Title(TitleSelection::Start);
        }
        _ => {}
    }
}
