use std::fmt;

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::{
    app::app::App,
    functions::center_rect,
    structs::{SearchSettings, Page},
    traits::{KeyInput, SelectItem},
    widgets::{force_clear::ForceClear, horizontal_split::HorizontalSplit},
};

use super::search::Search;

#[derive(Debug, Clone)]
pub enum GlobalItem {
    SearchBar,
    SearchSettings,
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
}

impl KeyInput for GlobalItem {
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
                    let state = Search::default();
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
                _ => {}
            },
            GlobalItem::SearchSettings => {
                app.search_settings.key_input(key);
            }
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
        popup_render: bool,
        message: &Option<String>,
        search_settings: &mut SearchSettings,
        search_text: &String,
    ) -> bool {
        let area = frame.size();
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
                let paragraph = Paragraph::new(search_text.clone()).block(block);
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

                                search_settings.text_list.area(chunks[0]);

                                if search_settings.row {
                                    search_settings
                                        .text_list
                                        .selected_style(Style::default().fg(Color::LightRed));

                                    search_settings
                                        .select_text_list
                                        .selected_style(Style::default().fg(Color::LightYellow));
                                } else {
                                    search_settings
                                        .text_list
                                        .selected_style(Style::default().fg(Color::LightYellow));

                                    search_settings
                                        .select_text_list
                                        .selected_style(Style::default().fg(Color::LightRed));
                                }

                                frame.render_widget(search_settings.text_list.clone(), chunks[0]);

                                search_settings.select_text_list.area(chunks[1]);

                                frame.render_widget(
                                    search_settings.select_text_list.clone(),
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
                        return true;
                    }
                }
            }
        }

        false
    }
}
