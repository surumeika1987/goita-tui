use crate::app::{App, CurrentScreen, TitleSelection};
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
        CurrentScreen::Title(selection) => draw_title(frame, selection),
        _ => {}
    }

    draw_footer(frame, app, chunks[2]);
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

fn draw_title(frame: &mut Frame, selection: TitleSelection) {
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

    let active_style = Style::default().fg(Color::Black).bg(Color::Green);

    match selection {
        TitleSelection::Start => start_block = start_block.style(active_style),
        TitleSelection::Settings => settings_block = settings_block.style(active_style),
        TitleSelection::Rules => rules_block = rules_block.style(active_style),
        TitleSelection::Exit => exit_block = exit_block.style(active_style),
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
