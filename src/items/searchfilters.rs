use crate::{
    config::{AppearanceConfig, KeyBindingsConfig, MainConfig, Search, SearchFilters},
    global::{
        functions::popup_area,
        structs::{KeyAction, Message, Page, Status, Task, Tasks},
    },
};
use tui::{
    layout::{Alignment, Constraint},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
};
use tui_additions::{
    framework::{FrameworkClean, FrameworkItem},
    widgets::{Grid, TextList},
};

const POPUP_MIN_WIDTH: u16 = 22;
const POPUP_MIN_HEIGHT: u16 = 9;
const POPUP_WIDTH_PERCENTAGE: u16 = 70;
const POPOUP_HEIGHT_PERCENTAGE: u16 = 70;

/// a tiny button when not selected, renders a popup when selected
#[derive(Clone)]
pub struct SearchFilter {
    pub left_textlist: TextList,
    pub right_textlist: TextList,
    pub right_options: Vec<Vec<&'static str>>,
    pub left_options: Vec<&'static str>,
    pub current_hover: bool,
    pub grid: Grid,
    pub previous_state: Option<SearchFilters>,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            left_textlist: TextList::default(),
            right_textlist: TextList::default(),
            right_options: Vec::new(),
            left_options: Vec::new(),
            // false = left, true = right
            current_hover: false,
            grid: Grid::new(
                vec![Constraint::Percentage(50), Constraint::Percentage(50)],
                vec![Constraint::Percentage(100)],
            )
            .unwrap(),
            previous_state: None,
        }
    }
}

impl SearchFilter {
    pub fn update(&mut self, framework: &FrameworkClean) {
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        if self.current_hover {
            self.left_textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
            self.right_textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
        } else {
            self.right_textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
            self.left_textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
        }

        self.right_textlist.selected = framework
            .data
            .state
            .get::<Search>()
            .unwrap()
            .filters
            .get_selected(self.left_textlist.selected);
        self.right_textlist
            .set_items(&self.right_options[self.left_textlist.selected])
            .unwrap();
    }
}

impl FrameworkItem for SearchFilter {
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
        popup_render: bool,
        info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render && info.selected {
            let frame_area = frame.size();
            let (area, success) = match popup_area(
                (POPUP_WIDTH_PERCENTAGE, POPOUP_HEIGHT_PERCENTAGE),
                (POPUP_MIN_WIDTH, POPUP_MIN_HEIGHT),
                frame_area,
            ) {
                Ok(area) => (area, true),
                Err(area) => (area, false),
            };

            frame.render_widget(Clear, area);

            let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

            if !success {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(appearance.borders)
                    .border_style(Style::default().fg(appearance.colors.outline_selected));

                let paragraph = Paragraph::new(format!(
                    "{}Current: {}x{}\nRequired: {}x{}",
                    "\n".repeat(frame_area.height as usize / 2 - 1),
                    frame_area.width,
                    frame_area.height,
                    POPUP_MIN_WIDTH,
                    POPUP_MIN_HEIGHT,
                ))
                .alignment(Alignment::Center)
                .style(Style::default().fg(appearance.colors.text_error))
                .block(block);

                frame.render_widget(paragraph, area);

                return;
            }

            let chunks = self.grid.chunks(area).unwrap().remove(0);

            frame.render_widget(self.grid.clone(), area);

            self.right_textlist.selected = framework
                .data
                .state
                .get::<Search>()
                .unwrap()
                .filters
                .get_selected(self.left_textlist.selected);

            self.left_textlist.set_height(chunks[0].height);
            frame.render_widget(self.left_textlist.clone(), chunks[0]);

            self.right_textlist.set_height(chunks[1].height);
            frame.render_widget(self.right_textlist.clone(), chunks[1]);
        } else if !popup_render {
            let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(appearance.borders)
                .border_style(Style::default().fg(if info.hover {
                    appearance.colors.outline_hover
                } else if info.selected {
                    appearance.colors.outline_secondary
                } else {
                    appearance.colors.outline
                }));

            let button = Paragraph::new("...").block(block);

            frame.render_widget(button, area);
        }
    }

    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        (self.left_options, self.right_options) = SearchFilters::get_all().into_iter().unzip();
        self.left_textlist.set_border_type(appearance.borders);
        self.right_textlist.set_border_type(appearance.borders);
        self.grid.set_border_type(appearance.borders);

        self.update(framework);

        self.left_textlist.set_items(&self.left_options)?;
        self.right_textlist
            .set_items(&self.right_options[self.left_textlist.selected])?;
        self.grid
            .set_border_style(Style::default().fg(appearance.colors.outline_selected));

        Ok(())
    }

    fn select(&mut self, framework: &mut tui_additions::framework::FrameworkClean) -> bool {
        framework
            .data
            .state
            .get_mut::<Status>()
            .unwrap()
            .popup_opened = true;
        framework
            .data
            .state
            .get_mut::<Tasks>()
            .unwrap()
            .priority
            .push(Task::ClearPage);
        self.update(framework);

        // create a clone of search options only if `refresh_after_modifying_search_filters` is true
        // and is a search page
        if !framework
            .data
            .global
            .get::<MainConfig>()
            .unwrap()
            .refresh_after_modifying_search_filters
        {
            return true;
        }

        if let Page::Search(search_options) = framework.data.state.get::<Page>().unwrap() {
            self.previous_state = Some(search_options.filters);
        }
        true
    }

    fn deselect(&mut self, framework: &mut tui_additions::framework::FrameworkClean) -> bool {
        framework
            .data
            .state
            .get_mut::<Status>()
            .unwrap()
            .popup_opened = false;

        // refresh page only if changed and enabled in options
        let search_options = framework.data.state.get::<Search>().unwrap().clone();
        if self.previous_state.is_none()
            || !framework
                .data
                .global
                .get::<MainConfig>()
                .unwrap()
                .refresh_after_modifying_search_filters
            || self.previous_state.unwrap() == search_options.filters
        {
            return true;
        }

        let page = framework.data.state.get_mut::<Page>().unwrap();
        if let Page::Search(search) = page {
            *search = search_options;
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Reload);
        }
        true
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let action = if let Some(keyactions) = framework
            .data
            .global
            .get::<KeyBindingsConfig>()
            .unwrap()
            .0
            .get(&key.code)
        {
            if let Some(action) = keyactions.get(&key.modifiers.bits()) {
                *action
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        let hovered_textlist = if self.current_hover {
            &mut self.right_textlist
        } else {
            &mut self.left_textlist
        };

        let current_hover_before = self.current_hover;

        let updated = match action {
            KeyAction::Select => {
                self.current_hover = !self.current_hover;
                true
            }
            KeyAction::MoveUp => hovered_textlist.up().is_ok(),
            KeyAction::MoveDown => hovered_textlist.down().is_ok(),
            KeyAction::MoveLeft => hovered_textlist.first().is_ok(),
            KeyAction::MoveRight => hovered_textlist.last().is_ok(),
            _ => false,
        };

        if updated {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);

            if current_hover_before {
                framework
                    .data
                    .state
                    .get_mut::<Search>()
                    .unwrap()
                    .filters
                    .set_index(
                        self.left_textlist.selected,
                        self.right_textlist.selected,
                        framework.data.global.get_mut::<Message>().unwrap(),
                    );
            } else {
                self.right_textlist
                    .set_items(&self.right_options[self.left_textlist.selected])?;
            }
        }

        self.update(framework);

        Ok(())
    }
}
