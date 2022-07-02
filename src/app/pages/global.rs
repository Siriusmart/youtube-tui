use std::fmt;

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    app::app::{App, AppNoState},
    functions::center_rect,
    structs::{Item, Page, WatchHistory},
    traits::ItemTrait,
    widgets::{force_clear::ForceClear, horizontal_split::HorizontalSplit},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalItem {
    SearchBar,
    SearchSettings,
    MessageBar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl ItemTrait for GlobalItem {
    fn select(&mut self, mut app: App) -> (App, bool) {
        let selected = match self {
            GlobalItem::SearchBar => true,
            GlobalItem::SearchSettings => {
                app.popup_focus = true;
                true
            }
            _ => false,
        };

        (app, selected)
    }

    fn selectable(&self) -> bool {
        match self {
            GlobalItem::SearchBar | GlobalItem::SearchSettings => true,
            _ => false,
        }
    }

    fn key_input(&mut self, key: KeyCode, mut app: App) -> (bool, App) {
        match self {
            GlobalItem::SearchBar => match key {
                KeyCode::Char(c) => {
                    app.search_text.push(c);
                }
                KeyCode::Backspace => {
                    app.search_text.pop();
                }
                KeyCode::Enter => {
                    if app.search_text.len() == 0 {
                        app.message = Some(String::from("Search term cannot be empty"));
                    } else {
                        let state = app.config.layouts.search.clone().into();
                        let mut history = app.history.clone();
                        let search_text = app.search_text.clone();
                        let search_settings = app.search_settings.clone();
                        history.push(app.into());

                        return (
                            false,
                            App {
                                history,
                                page: Page::Search,
                                selectable: App::selectable(&state),
                                state,
                                search_text,
                                search_settings,
                                load: true,
                                ..Default::default()
                            },
                        );
                    }
                }
                _ => {}
            },
            GlobalItem::SearchSettings => {
                app.search_settings.key_input(key, &app.config);
            }
            _ => {}
        }

        (true, app)
    }

    fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: AppNoState,
        selected: bool,
        hover: bool,
        _: bool,
        popup_render: bool,
    ) -> (bool, AppNoState) {
        let area = frame.size();
        let mut out = (false, app);
        match self {
            GlobalItem::SearchBar => {
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
                let mut text = out.1.search_text.clone();
                if selected {
                    text.push('█');
                }
                let paragraph = Paragraph::new(text).block(block);
                frame.render_widget(paragraph, rect);
            }
            GlobalItem::MessageBar => {
                let message = out.1.message.clone();
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
            GlobalItem::SearchSettings => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if selected {
                        Color::LightYellow
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));
                let paragraph = Paragraph::new(String::from("...")).block(block);
                frame.render_widget(paragraph, rect);

                if selected {
                    if popup_render {
                        match center_rect((area.width * 70 / 100, area.height * 70 / 100), area) {
                            Ok(rect) => {
                                let split = HorizontalSplit::default()
                                    .percentages(vec![40, 60])
                                    .border_style(Style::default().fg(if selected {
                                        Color::LightBlue
                                    } else if hover {
                                        Color::LightRed
                                    } else {
                                        Color::Reset
                                    }));
                                let chunks = split.inner(rect);

                                frame.render_widget(split, rect);

                                out.1.search_settings.text_list.area(chunks[0]);

                                if out.1.search_settings.row {
                                    out.1
                                        .search_settings
                                        .text_list
                                        .selected_style(Style::default().fg(Color::LightRed));

                                    out.1
                                        .search_settings
                                        .select_text_list
                                        .selected_style(Style::default().fg(Color::LightYellow));
                                } else {
                                    out.1
                                        .search_settings
                                        .text_list
                                        .selected_style(Style::default().fg(Color::LightYellow));

                                    out.1
                                        .search_settings
                                        .select_text_list
                                        .selected_style(Style::default().fg(Color::LightRed));
                                }

                                frame.render_widget(
                                    out.1.search_settings.text_list.clone(),
                                    chunks[0],
                                );

                                out.1.search_settings.select_text_list.area(chunks[1]);

                                frame.render_widget(
                                    out.1.search_settings.select_text_list.clone(),
                                    chunks[1],
                                );

                                frame.render_widget(ForceClear, chunks[0]);

                                frame.render_widget(ForceClear, chunks[1]);
                            }
                            Err(rect) => {
                                frame.render_widget(Clear, rect);
                            }
                        }
                        //panic!("{:?}", rect);
                    } else {
                        out.0 = true;
                    }
                }
            }
        }

        out
    }

    fn load_item(&self, _: &App, _: &mut WatchHistory) -> Result<Item, Box<dyn std::error::Error>> {
        Ok(Item::Global(self.clone()))
    }
}

impl GlobalItem {}
