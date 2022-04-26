use std::error::Error;
use youtube_tui::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

pub fn update(app: &mut App, key: KeyCode) -> Result<(), Box<dyn Error>> {
    app.status = Some(format!("{:?}", app.state));

    match key {
        KeyCode::Up => {
            app.up();
        }
        KeyCode::Down => {
            app.down();
        }
        KeyCode::Left => {
            app.left();
        }
        KeyCode::Right => {
            app.right();
        }
        KeyCode::Enter => {
            app.enter();
        }
        _ => {
        }
    }
    Ok(())
}
