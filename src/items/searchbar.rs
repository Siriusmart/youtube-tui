use std::error::Error;

use crate::{
    config::*,
    global::{structs::*, traits::CollectionNoId},
};

#[cfg(feature = "clipboard")]
use crate::global::functions::get_clipboard;
use crate::global::functions::{next_word, previous_word, remove_word};

use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders},
};
use tui_additions::{framework::FrameworkItem, widgets::TextField};

/// the search bar item
#[derive(Clone, Default)]
pub struct SearchBar {
    pub text_field: TextField,
    pub history_index: Option<usize>,
    pub custom_value_cache: String,
}

impl FrameworkItem for SearchBar {
    // basically is a TextField with borders
    fn render(
        &mut self,
        frame: &mut ratatui::Frame<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: ratatui::layout::Rect,
        popup_render: bool,
        info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

        let block = Block::default()
            .title("Search YouTube")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(appearance.borders)
            .border_style(Style::default().fg(if info.hover {
                appearance.colors.outline_hover
            } else if info.selected {
                appearance.colors.outline_selected
            } else {
                appearance.colors.outline
            }));

        let inner = block.inner(area);

        frame.render_widget(block, area);

        self.text_field.set_width(inner.width);

        frame.render_widget(self.text_field.clone(), inner);
    }

    // sets text field content to the curreny search query, thats it
    // kinda pointless if it doesnt break anything gonna remove it
    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        let search = framework.data.state.get::<Search>().unwrap().query.clone();
        self.text_field.content = search;
        let _ = self.text_field.last();

        Ok(())
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        let mut render = true;
        match framework
            .data
            .global
            .get::<KeyBindingsConfig>()
            .unwrap()
            .get(key)
        {
            _ if matches!(key.code, KeyCode::Char(_)) && key.modifiers.bits() < 2 => render = false,
            #[cfg(feature = "clipboard")]
            Some(KeyAction::Paste) => {
                let content = get_clipboard();
                if content.is_empty() {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("Clipboard empty"));
                }

                // push all characters at cursor location
                content.chars().for_each(|c| {
                    let _ = self.text_field.push(c);
                });
            }
            Some(KeyAction::RemoveWord) => remove_word(&mut self.text_field),
            Some(KeyAction::PreviousWord) => previous_word(&mut self.text_field),
            Some(KeyAction::NextWord) => next_word(&mut self.text_field),
            Some(KeyAction::ClearLine) => {
                self.text_field.content.clear();
                self.text_field.scroll = 0;
                self.text_field.cursor = 0;
            }
            Some(KeyAction::First | KeyAction::MoveUp) => self.text_field.cursor = 0,
            Some(KeyAction::End | KeyAction::MoveDown) => {
                self.text_field.cursor = self.text_field.content.len()
            }
            Some(KeyAction::PreviousEntry) => {
                let history = framework.data.global.get::<SearchHistory>().unwrap();
                if self.history_index != Some(0) && !history.0.is_empty() {
                    if self.history_index.is_none() {
                        self.custom_value_cache = self.text_field.content.clone();
                        self.history_index = None
                    }
                    self.history_index = Some(self.history_index.unwrap_or(history.0.len()) - 1);
                    self.text_field.content = history.0[self.history_index.unwrap()].clone();
                    let _ = self.text_field.last();
                } else {
                    render = false
                }
            }
            Some(KeyAction::NextEntry) => {
                let history = framework.data.global.get::<SearchHistory>().unwrap();
                if self.history_index.is_none() {
                    render = false
                } else if self.history_index == Some(history.0.len() - 1) {
                    self.history_index = None;
                    self.text_field.content = self.custom_value_cache.clone();
                    let _ = self.text_field.last();
                } else {
                    self.history_index = self.history_index.map(|n| n + 1);
                    self.text_field.content = history.0[self.history_index.unwrap()].clone();
                    let _ = self.text_field.last();
                }
            }
            _ => render = false,
        }

        if render {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            return Ok(());
        }
        // pasting clipboard content

        let updated = match key.code {
            KeyCode::Backspace => self.text_field.remove(self.text_field.cursor).is_ok(),
            KeyCode::Char(c) => self.text_field.push(c).is_ok(),
            KeyCode::Up => self.text_field.first().is_ok(),
            KeyCode::Down => self.text_field.last().is_ok(),
            KeyCode::Right => self.text_field.right().is_ok(),
            KeyCode::Left => self.text_field.left().is_ok(),
            KeyCode::Enter => {
                if self.text_field.content.is_empty() {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Message(String::from("Search string must not be empty"));
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                    return Ok(());
                }

                let search = framework.data.state.get_mut::<Search>().unwrap();
                search.query = self.text_field.content.clone();
                framework
                    .data
                    .global
                    .get_mut::<SearchHistory>()
                    .unwrap()
                    .push(search.query.clone());
                let search = search.clone();
                let tasks = framework.data.state.get_mut::<Tasks>().unwrap();
                tasks.priority.push(Task::LoadPage(Page::Search(search)));
                false
            }
            _ => false,
        };

        // only re-render screen if updated
        if updated {
            let tasks = framework.data.state.get_mut::<Tasks>().unwrap();
            tasks.priority.push(Task::RenderAll);
        }

        Ok(())
    }

    fn mouse_event(
        &mut self,
        _framework: &mut tui_additions::framework::FrameworkClean,
        mut x: u16,
        y: u16,
        _absolute_x: u16,
        _absolute_y: u16,
    ) -> bool {
        if y != 1 || x == 0 {
            return false;
        }

        x -= 1; // there is 1 character to the left of the text field

        if x == self.text_field.cursor as u16 {
            return false;
        }

        if x > self.text_field.content.len() as u16 {
            return self.text_field.last().is_ok();
        }

        self.text_field.cursor = x as usize;
        self.text_field.update().is_ok()
    }

    fn selectable(&self) -> bool {
        true
    }
}
