use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use youtube_tui::app::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|mut frame| {
            app.render(&mut frame);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    Ok(())
}