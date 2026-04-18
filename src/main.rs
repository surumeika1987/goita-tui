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

use crate::app::{App, CurrentScreen, TitleSelection};
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
                CurrentScreen::Title(selection) => title_key_event_hadler(app, key.code, selection),
                _ => {}
            }

            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    Ok(())
}

fn title_key_event_hadler(app: &mut App, key_code: KeyCode, selection: TitleSelection) {
    match key_code {
        KeyCode::Up => app.current_screen = CurrentScreen::Title(selection.previous()),
        KeyCode::Down => app.current_screen = CurrentScreen::Title(selection.next()),
        _ => {}
    }
}
