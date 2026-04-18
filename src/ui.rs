use crate::app::{
    App, CurrentScreen, GameSetting, GameSettingSelection, PlayerSetting, TitleSelection,
};
use goita::BoardDirection;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_header(frame, chunks[0]);

    match app.current_screen {
        CurrentScreen::Title(selection) => draw_title(frame, &selection),
        CurrentScreen::GameSettings(selection) => draw_game_settings(frame, app, &selection),
        _ => {}
    }

    draw_footer(frame, app, chunks[2]);
}

fn active_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::Green)
}

fn draw_header(frame: &mut Frame, chunk: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_text = Paragraph::new(Text::styled(
        "GOITA TUI v0.1.0",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title_text, chunk);
}

fn draw_footer(frame: &mut Frame, app: &App, chunk: Rect) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let current_key_hint = {
        match app.current_screen {
            CurrentScreen::Title(_) => "<UP><DOWN>選択 <ENTER>決定",
            _ => "",
        }
    };

    let footer_text = Paragraph::new(Text::styled(
        current_key_hint,
        Style::default().fg(Color::Yellow),
    ))
    .block(footer_block);
    frame.render_widget(footer_text, chunk);
}

fn centered_rect(size_x: u16, size_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - size_y) / 2),
            Constraint::Length(size_y),
            Constraint::Length((r.height - size_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - size_x) / 2),
            Constraint::Length(size_x),
            Constraint::Length((r.width - size_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_title(frame: &mut Frame, selection: &TitleSelection) {
    let popup_block = Block::default()
        .title("タイトル")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(20, 14, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let mut start_block = Block::default().borders(Borders::ALL);
    let mut settings_block = Block::default().borders(Borders::ALL);
    let mut rules_block = Block::default().borders(Borders::ALL);
    let mut exit_block = Block::default().borders(Borders::ALL);

    match selection {
        TitleSelection::Start => start_block = start_block.style(active_style()),
        TitleSelection::Settings => settings_block = settings_block.style(active_style()),
        TitleSelection::Rules => rules_block = rules_block.style(active_style()),
        TitleSelection::Exit => exit_block = exit_block.style(active_style()),
    }

    let start_text = Paragraph::new(Line::from("スタート")).block(start_block);
    let settings_text = Paragraph::new(Line::from("設定")).block(settings_block);
    let rules_text = Paragraph::new(Line::from("ルール")).block(rules_block);
    let exit_text = Paragraph::new(Line::from("終了")).block(exit_block);

    frame.render_widget(start_text, popup_chunks[0]);
    frame.render_widget(settings_text, popup_chunks[1]);
    frame.render_widget(rules_text, popup_chunks[2]);
    frame.render_widget(exit_text, popup_chunks[3]);
}

fn draw_game_settings(frame: &mut Frame, app: &App, selection: &GameSettingSelection) {
    let popup_block = Block::default()
        .title("ゲーム設定")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(30, 25, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let mut player1_block = Block::default().borders(Borders::ALL);
    let mut player2_block = Block::default().borders(Borders::ALL);
    let mut player3_block = Block::default().borders(Borders::ALL);
    let mut player4_block = Block::default().borders(Borders::ALL);
    let mut initial_turn_player_block = Block::default().borders(Borders::ALL);
    let mut winning_score_block = Block::default().borders(Borders::ALL);
    let mut start_block = Block::default().borders(Borders::ALL);

    match selection {
        GameSettingSelection::Player1 => player1_block = player1_block.style(active_style()),
        GameSettingSelection::Player2 => player2_block = player2_block.style(active_style()),
        GameSettingSelection::Player3 => player3_block = player3_block.style(active_style()),
        GameSettingSelection::Player4 => player4_block = player4_block.style(active_style()),
        GameSettingSelection::InitialTurnPlayer => {
            initial_turn_player_block = initial_turn_player_block.style(active_style())
        }
        GameSettingSelection::WinningScore => {
            winning_score_block = winning_score_block.style(active_style())
        }
        GameSettingSelection::Start => start_block = start_block.style(active_style()),
    }

    let player1_text = Paragraph::new(Line::from(player_text_string(app, BoardDirection::North)))
        .block(player1_block);
    let player2_text = Paragraph::new(Line::from(player_text_string(app, BoardDirection::East)))
        .block(player2_block);
    let player3_text = Paragraph::new(Line::from(player_text_string(app, BoardDirection::South)))
        .block(player3_block);
    let player4_text = Paragraph::new(Line::from(player_text_string(app, BoardDirection::West)))
        .block(player4_block);
    let initial_turn_player_text =
        Paragraph::new(Line::from(inital_player_text_string(app))).block(initial_turn_player_block);
    let winning_score_text =
        Paragraph::new(Line::from(winning_score_text_string(app))).block(winning_score_block);
    let start_text = Paragraph::new(Line::from("スタート")).block(start_block);

    frame.render_widget(player1_text, popup_chunks[0]);
    frame.render_widget(player2_text, popup_chunks[1]);
    frame.render_widget(player3_text, popup_chunks[2]);
    frame.render_widget(player4_text, popup_chunks[3]);
    frame.render_widget(initial_turn_player_text, popup_chunks[4]);
    frame.render_widget(winning_score_text, popup_chunks[5]);
    frame.render_widget(start_text, popup_chunks[6]);
}

// プレイヤーを文字列に変換するヘルパー関数
fn player_to_str(player: BoardDirection) -> &'static str {
    match player {
        BoardDirection::North => "北",
        BoardDirection::East => "東",
        BoardDirection::South => "南",
        BoardDirection::West => "西",
    }
}

// プレイヤーテキストブロック用のヘルパー関数
fn player_text_string(app: &App, player: BoardDirection) -> String {
    let player_str = player_to_str(player);

    let player_setting = match player {
        BoardDirection::North => &app.game_setting.player1,
        BoardDirection::East => &app.game_setting.player2,
        BoardDirection::South => &app.game_setting.player3,
        BoardDirection::West => &app.game_setting.player4,
    };

    let player_setting_str = match player_setting {
        PlayerSetting::Player => "プレイヤー",
        PlayerSetting::CPU => "コンピューター",
    };

    format!("{}: <{}>", player_str, player_setting_str)
}

// 親プレイヤーテキストブロック用のヘルパー関数
fn inital_player_text_string(app: &App) -> String {
    let init_player_str = player_to_str(app.game_setting.initial_turn_player);
    format!("開始プレイヤー: <{}>", init_player_str)
}

// 勝利点テキストブロック用のヘルパー関数
fn winning_score_text_string(app: &App) -> String {
    format!("勝利点: <{}>", app.game_setting.winning_score)
}
