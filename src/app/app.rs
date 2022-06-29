use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use crate::{
    app::pages::main_menu::*,
    structs::{AppHistory, Item, Page, Row, SearchSettings, WatchHistory},
    traits::{ItemTrait, PageTrait},
};
use crossterm::event::KeyCode;
use invidious::blocking::Client;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::{
    config::{Action, Config},
    pages::{channel::Channel, item_info::ItemInfo, search::Search},
};

#[derive(Debug, Clone)]
pub struct App {
    pub config: Config,
    pub page: Page,
    pub state: Vec<Row>, // Item
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: Arc<Mutex<Option<String>>>,
    pub load: bool,
    pub render: bool,
    pub popup_focus: bool,
    pub history: Vec<AppHistory>,
    pub watch_history: WatchHistory,
    pub search_settings: SearchSettings,
    pub search_text: String,
    pub page_no: usize,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::load().unwrap();
        let state = MainMenu::default();
        let selectable = App::selectable(&state);
        Self {
            page: Page::default(),
            state,
            selectable,
            client: Client::new(config.main.server_url.clone()),
            selected: None,
            hover: None,
            message: Arc::new(Mutex::new(None)),
            load: true,
            render: true,
            popup_focus: false,
            history: Vec::new(),
            config,
            watch_history: WatchHistory::load(),
            search_settings: SearchSettings::default(),
            search_text: String::new(),
            page_no: 1,
        }
    }
}

impl App {
    pub fn selectable(state: &Vec<Row>) -> Vec<Vec<(usize, usize)>> {
        let mut selectable = Vec::new();

        for (y, row) in state.iter().enumerate() {
            let mut row_vec = Vec::new();
            for (x, row_item) in row.items.iter().enumerate() {
                if match &row_item.item {
                    Item::Global(item) => item.selectable(),
                    Item::MainMenu(item) => item.selectable(),
                    Item::ItemInfo(item) => item.selectable(),
                    Item::Search(item) => item.selectable(),
                    Item::Channel(item) => item.selectable(),
                } {
                    row_vec.push((x, y));
                }
            }
            if row_vec.len() != 0 {
                selectable.push(row_vec);
            }
        }

        selectable
    }

    pub fn key_input(mut self, key: KeyCode) -> App {
        let action = self.config.keybindings.0.get(&key);

        if let Some((x, y)) = self.selected {
            if action != Some(&Action::Deselect) {
                let mut item = self.state[y].items[x].item.clone();
                let updated;
                (updated, self) = item.key_input(key, self);
                if updated {
                    self.state[y].items[x].item = item;
                }

                return self;
            }
        }

        let action = match action {
            Some(action) => *action,
            None => return self,
        };

        match action {
            Action::Select => {
                if self.hover.is_some() && self.selected.is_none() {
                    let (mut x, mut y) = self.hover.unwrap();
                    (x, y) = self.selectable[y][x];

                    let select;
                    (self, select) = self.state[y]
                        .items
                        .iter()
                        .nth(x)
                        .clone()
                        .unwrap()
                        .item
                        .clone()
                        .select(self);
                    if select {
                        self.selected = Some((x, y));
                    }

                    return self;
                }
            }
            Action::Deselect => {
                if self.selected.is_some() {
                    self.selected = None;
                    self.popup_focus = false;
                    return self;
                }
            }

            _ => {}
        }

        match &mut self.hover {
            Some((x, y)) => match action {
                Action::Up => {
                    if *y > 0 {
                        let temp_y = *y - 1;
                        if *x > self.selectable[temp_y].len() {
                            let temp_x = self.selectable[temp_y].len();
                            if temp_x > self.selectable[*y].len() - 1 {
                                *x = self.selectable[*y].len() - 1;
                            }
                        }
                        *y -= 1;
                        if *x > self.selectable[*y].len() - 1 {
                            *x = self.selectable[*y].len() - 1;
                        }
                    }
                }
                Action::Down => {
                    if *y < self.selectable.len() - 1 {
                        *y += 1;
                        if *x > self.selectable[*y].len() - 1 {
                            *x = self.selectable[*y].len() - 1;
                        }
                    }
                }

                Action::Left => {
                    if *x > 0 {
                        *x -= 1;
                    }
                }

                Action::Right => {
                    if *x < self.selectable[*y].len() - 1 {
                        *x += 1;
                    }
                }

                _ => {}
            },
            None => match action {
                Action::Up => {
                    self.hover = Some((0, 0));
                }
                Action::Down => {
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                Action::Left => {
                    self.hover = Some((0, 0));
                }
                Action::Right => {
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                _ => {}
            },
        }

        self
    }

    pub fn render<B: Backend>(mut self, frame: &mut Frame<B>) -> Self {
        let size = frame.size();
        let mut popups = Vec::new();

        let min = match self.page {
            Page::MainMenu(_) => MainMenu::min(),
            Page::ItemDisplay(_) => ItemInfo::min(),
            Page::Search => Search::min(),
            Page::Channel(_, _) => Channel::min(),
        };

        if size.width < min.0 || size.height < min.1 {
            let paragraph = Paragraph::new(format!(
                "Window too small. Minimum size for this page is {} x {}. Current size is {} x {}",
                min.0, min.1, size.width, size.height
            ))
            .block(Block::default())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, size);
            return self;
        }

        let hover_selected = if let Some((x, y)) = self.hover {
            Some(self.selectable[y][x])
        } else {
            None
        };

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.state
                    .iter()
                    .map(|row| row.height)
                    .collect::<Vec<Constraint>>(),
            )
            .split(size);

        let mut state = self.state.clone();

        for (y, (row, row_chunk)) in state
            .iter_mut()
            .zip(vertical_chunks.clone().iter_mut())
            .enumerate()
        {
            let mut constraints = LinkedList::new();
            let mut length = match row.centered {
                true => Some(0),
                false => None,
            };
            for item in row.items.iter() {
                constraints.push_back(item.constraint);
                if let Some(length_value) = length {
                    length = Some(match item.constraint {
                        Constraint::Length(l) | Constraint::Max(l) | Constraint::Min(l) => {
                            l + length_value
                        }
                        Constraint::Percentage(p) => length_value + size.width * p / 100,
                        _ => unreachable!(),
                    })
                }
            }

            if let Some(i) = length {
                let extra_constraint = Constraint::Length((size.width - i) / 2);
                constraints.push_front(extra_constraint);
            } else {
                constraints.push_front(Constraint::Length(0));
            }

            constraints.push_back(Constraint::Length(0));

            let mut chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.into_iter().collect::<Vec<Constraint>>())
                .split(*row_chunk)
                .into_iter();

            frame.render_widget(Block::default(), chunks.next().unwrap());

            for (x, (chunk, item)) in chunks
                .zip(row.items.iter_mut().map(|i| &mut i.item))
                .enumerate()
            {
                let selected = self.selected == Some((x, y));

                let hover = hover_selected == Some((x, y));
                let popup_focus = self.popup_focus;

                let hold =
                    item.render_item(frame, chunk, self, selected, hover, popup_focus, false);

                self = hold.1;

                if hold.0 {
                    popups.push((item, selected, hover, chunk));
                }
            }
        }

        for (item, selected, hover, chunk) in popups {
            let hold = item.render_item(frame, chunk, self, selected, hover, true, true);
            self = hold.1;

        }

        self.state = state;

        self
    }

    pub fn pop(mut self) -> (App, bool) {
        if self.history.len() == 0 {
            *self.message.lock().unwrap() = Some(String::from("This is the beginning of history"));
            return (self, false);
        }

        let app_history = self.history.pop().unwrap();

        self = Self {
            state: app_history.state,
            selectable: app_history.selectable,
            selected: app_history.selected,
            hover: app_history.hover,
            message: app_history.message,
            page: app_history.page,
            client: app_history.client,
            load: app_history.load,
            render: app_history.render,
            history: self.history.clone(),
            config: self.config,
            watch_history: self.watch_history,
            search_settings: self.search_settings,
            popup_focus: app_history.popup_focus,
            search_text: app_history.search_text,
            page_no: app_history.page_no,
        };

        (self, true)
    }

    pub fn home(&mut self) {
        *self = App::default();
    }
}
