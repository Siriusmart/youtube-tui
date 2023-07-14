use std::error::Error;

use crate::{config::*, global::structs::*};

#[cfg(feature = "clipboard")]
use crate::global::functions::*;

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
        match framework
            .data
            .global
            .get::<KeyBindingsConfig>()
            .unwrap()
            .get(key)
        {
            #[cfg(feature = "clipboard")]
            Some(KeyAction::Paste) => {
                let content = get_clipboard();
                if content.is_empty() {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("Clipboard empty"));
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                    return Ok(());
                }

                // push all characters at cursor location
                content.chars().for_each(|c| {
                    let _ = self.text_field.push(c);
                });
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::RenderAll);
                return Ok(());
            }
            Some(KeyAction::RemoveWord) => {
                remove_word(&mut self.text_field);
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::RenderAll);
                return Ok(());
            }
            Some(KeyAction::ClearLine) => {
                self.text_field.content.clear();
                self.text_field.scroll = 0;
                self.text_field.cursor = 0;
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::RenderAll);
                return Ok(());
            }
            _ => {}
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

                let mut search = framework.data.state.get_mut::<Search>().unwrap();
                search.query = self.text_field.content.clone();
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
