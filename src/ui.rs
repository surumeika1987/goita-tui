use crate::app::{
    App, CurrentScreen, GameSetting, GameSettingSelection, PlayerSetting, TitleSelection,
};
use goita::{BoardDirection, Piece, PieceWithFacing, Team};
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

    render_header(frame, chunks[0]);

    render_game(frame, chunks[1], app);

    match app.current_screen {
        CurrentScreen::Title(selection) => render_title(frame, &selection),
        CurrentScreen::GameSettings(selection) => render_game_settings(frame, app, &selection),
        _ => {}
    }

    render_footer(frame, app, chunks[2]);
}

fn active_style() -> Style {
    Style::default().fg(Color::Black).bg(Color::Green)
}

fn render_header(frame: &mut Frame, chunk: Rect) {
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

fn render_footer(frame: &mut Frame, app: &App, chunk: Rect) {
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

fn render_title(frame: &mut Frame, selection: &TitleSelection) {
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

fn render_game_settings(frame: &mut Frame, app: &App, selection: &GameSettingSelection) {
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

fn render_game(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .split(area);

    render_team_scores(frame, chunks[0], app);

    let ne_board_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);
    render_player_board(frame, ne_board_chunks[0], app, BoardDirection::North);
    render_player_board(frame, ne_board_chunks[1], app, BoardDirection::East);

    let sw_board_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    render_player_board(frame, sw_board_chunks[0], app, BoardDirection::South);
    render_player_board(frame, sw_board_chunks[1], app, BoardDirection::West);

    render_player_hand(frame, chunks[3], app);
}

fn render_team_scores(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let ns_team_block = Block::default()
        .title("南北チーム")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));
    let ew_team_block = Block::default()
        .title("東西チーム")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let ns_team_text = Paragraph::new(Line::from(format!(
        "{}/{}点",
        app.team_score(Team::NorthSouth).unwrap_or_default(),
        app.winning_score().unwrap_or_default(),
    )))
    .block(ns_team_block);

    let ew_team_text = Paragraph::new(Line::from(format!(
        "{}/{}点",
        app.team_score(Team::EastWest).unwrap_or_default(),
        app.winning_score().unwrap_or_default(),
    )))
    .block(ew_team_block);

    frame.render_widget(ns_team_text, chunks[0]);
    frame.render_widget(ew_team_text, chunks[1]);
}

fn render_player_board(frame: &mut Frame, area: Rect, app: &App, player: BoardDirection) {
    let mut board_block = Block::default()
        .title(format!("{} - 受け/攻め", player_to_str(player)))
        .borders(Borders::ALL)
        .style(Style::default());
    let mut board_area = board_block.inner(area);

    if let Some(current_turn_player) = app.current_turn_player() {
        if current_turn_player == player {
            board_block = board_block.style(Style::default().fg(Color::Green));
        }
    }

    frame.render_widget(board_block, area);

    render_pieces(
        frame,
        board_area,
        app.view_board[usize::from(player)]
            .as_ref()
            .unwrap_or(&Vec::new()),
        None,
    );
}

fn render_player_hand(frame: &mut Frame, area: Rect, app: &App) {
    let hand_title_string = if let Some(current_turn_player) = app.current_turn_player() {
        format!("{} - 持ち駒", player_to_str(current_turn_player))
    } else {
        String::from("持ち駒")
    };
    let mut hand_block = Block::default()
        .title(hand_title_string)
        .borders(Borders::ALL)
        .style(Style::default());
    let mut hand_area = hand_block.inner(area);

    frame.render_widget(hand_block, area);

    let hand_with_facing = app
        .view_hand
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|piece| PieceWithFacing::FaceUp(*piece))
        .collect::<Vec<PieceWithFacing>>();

    render_pieces(frame, hand_area, &hand_with_facing, None);
}

fn piece_to_string(piece: Piece) -> String {
    match piece {
        Piece::King => String::from("王"),
        Piece::Rook => String::from("飛"),
        Piece::Bishop => String::from("角"),
        Piece::Gold => String::from("金"),
        Piece::Silver => String::from("銀"),
        Piece::Knight => String::from("馬"),
        Piece::Lance => String::from("香"),
        Piece::Pawn => String::from("し"),
    }
}

fn render_pieces(
    frame: &mut Frame,
    area: Rect,
    pieces: &Vec<PieceWithFacing>,
    selection: Option<u8>,
) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    for i in 0..2 {
        let board_piece_strings = pieces
            .iter()
            .enumerate()
            .filter(|(j, _)| j % 2 == i)
            .map(|(_, x)| match x {
                PieceWithFacing::FaceUp(piece) => piece_to_string(*piece),
                PieceWithFacing::FaceDown(_) => String::from("裏"),
            })
            .collect::<Vec<String>>();

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(vertical_chunks[i]);

        for j in 0..4 {
            let board_piece_string = if let Some(piece_string) = board_piece_strings.get(j) {
                piece_string.clone()
            } else {
                String::from("　")
            };

            let mut board_piece_style = Style::default();
            if let Some(selection) = selection {
                if selection == (i * 4 + j) as u8 {
                    board_piece_style = active_style();
                }
            }

            let board_piece_text =
                Paragraph::new(Line::from(board_piece_string).style(board_piece_style));
            frame.render_widget(board_piece_text, horizontal_chunks[j]);
        }
    }
}
