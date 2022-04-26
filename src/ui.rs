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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    let size = f.size();

    if size.width < 45 || size.height < 12 {
        let paragraph = Paragraph::new(Span::styled(
            "The terminal is too small to run this application (min. 45x12)",
            Style::default().fg(Color::Red),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(paragraph, size);
        return Ok(());
    }

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(size.height - 6),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    match app.state {
        State::MainMenu {
            selector,
            selected,
            navigation,
        } => {
            // Search bar

            let color = if navigation == Some(MainMenuItem::SearchBar) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::SearchBar) {
                Color::LightBlue
            } else {
                Color::Reset
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let search_text = String::from_iter(app.search_bar.iter());
            let paragraph = Paragraph::new(search_text).block(block);
            f.render_widget(paragraph, vertical_chunks[0]);

            // Status bar
            let color = if navigation == Some(MainMenuItem::StatusBar) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::StatusBar) {
                Color::LightBlue
            } else {
                Color::Reset
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let status_text = app.status.clone().unwrap_or(String::from("All good :)"));
            let paragraph = Paragraph::new(status_text).block(block);
            f.render_widget(paragraph, vertical_chunks[2]);

            let chunk = vertical_chunks[1];

            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(chunk.height - 3)].as_ref())
                .split(chunk);

            // Selection bar

            let side_margin = (vertical_chunks[0].width - 45) / 2;
            let selector_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(side_margin), Constraint::Length(45)].as_ref())
                .split(vertical_chunks[0])[1];
            let selector_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(15),
                        Constraint::Length(15),
                        Constraint::Length(15),
                        Constraint::Length(0),
                    ]
                    .as_ref(),
                )
                .split(selector_chunks);

            let color = if navigation == Some(MainMenuItem::Trending) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::Trending) {
                Color::LightBlue
            } else {
                Color::Reset
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let paragraph = Paragraph::new(String::from("Trending"))
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, selector_chunks[0]);

            let color = if navigation == Some(MainMenuItem::Popular) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::Popular) {
                Color::LightBlue
            } else {
                Color::Reset
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let paragraph = Paragraph::new(String::from("Popular"))
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, selector_chunks[1]);

            let color = if navigation == Some(MainMenuItem::History) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::History) {
                Color::LightBlue
            } else {
                Color::Reset
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let paragraph = Paragraph::new(String::from("History"))
                .block(block)
                .alignment(Alignment::Center);

            f.render_widget(paragraph, selector_chunks[2]);

            // Body
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(vertical_chunks[1]);

            let color = if navigation == Some(MainMenuItem::Videos) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::Videos) {
                Color::LightBlue
            } else {
                Color::Reset
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let paragraph = Paragraph::new(String::from("")).block(block);
            f.render_widget(paragraph, body_chunks[0]);

            let color = if navigation == Some(MainMenuItem::VideoDetails) {
                Color::LightRed
            } else if selected == Some(MainMenuItem::VideoDetails) {
                Color::LightBlue
            } else {
                Color::Reset
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(color));
            let paragraph = Paragraph::new(String::from("")).block(block);
            f.render_widget(paragraph, body_chunks[1]);
        }
    };
    Ok(())
}
