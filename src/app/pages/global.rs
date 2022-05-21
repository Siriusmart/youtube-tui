use std::fmt;

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::app::App,
    traits::{KeyInput, SelectItem},
};

#[derive(Debug, Clone)]
pub enum GlobalItem {
    SearchBar(String),
    MessageBar,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Youtube,
    Invidious,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Youtube => write!(f, "Youtube"),
            Mode::Invidious => write!(f, "Invidious"),
        }
    }
}

impl Mode {
    pub fn toggle(&mut self) {
        match self {
            Mode::Youtube => *self = Mode::Invidious,
            Mode::Invidious => *self = Mode::Youtube,
        }
    }
}

impl SelectItem for GlobalItem {
    fn select(&mut self, app: App) -> (App, bool) {
        let selected = match self {
            GlobalItem::SearchBar(_) => true,
            _ => false,
        };

        (app, selected)
    }

    fn selectable(&self) -> bool {
        match self {
            GlobalItem::SearchBar(_) => true,
            _ => false,
        }
    }
}

impl KeyInput for GlobalItem {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        match self {
            GlobalItem::SearchBar(search_bar) => match key {
                KeyCode::Char(c) => {
                    search_bar.push(c);
                }
                KeyCode::Backspace => {
                    search_bar.pop();
                }
                KeyCode::Enter => {}
                _ => {}
            },
            _ => {}
        }

        (true, app)
    }
}

impl GlobalItem {
    pub fn render_item<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        message: &Option<String>,
    ) {
        match self {
            GlobalItem::SearchBar(search) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(tui::style::Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }))
                    .title("Search YouTube")
                    .title_alignment(Alignment::Center);
                let paragraph = Paragraph::new(search.clone()).block(block);
                frame.render_widget(paragraph, rect);
            }
            GlobalItem::MessageBar => {
                // let color = Color::LightYellow;
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if message.is_some() {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));
                let paragraph =
                    Paragraph::new(message.clone().unwrap_or(String::from("All good :)")))
                        .block(block);
                frame.render_widget(paragraph, rect);
            }
        }
    }
}
