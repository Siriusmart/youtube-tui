use std::error::Error;

use crate::{
    config::{AppearanceConfig, Search},
    global::structs::{Message, Page, Task, Tasks},
};
use crossterm::event::KeyCode;
use tui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders},
};
use tui_additions::{framework::FrameworkItem, widgets::TextField};

#[derive(Clone, Default)]
pub struct SearchBar {
    pub text_field: TextField,
}

impl FrameworkItem for SearchBar {
    // basically is a TextField with borders
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
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

    fn selectable(&self) -> bool {
        true
    }
}
