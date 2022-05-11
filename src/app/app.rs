use std::{collections::LinkedList, error::Error};

use crate::{
    app::pages::{global::*, item_info::*, main_menu::*},
    traits::{KeyInput, SelectItem},
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

#[derive(Debug)]
pub struct App {
    pub page: Page,
    pub state: Vec<Row>, // Item
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: Option<String>,
    pub load: bool,
    pub render: bool,
    pub history: Vec<AppHistory>,
}

impl App {
    pub fn new() -> Self {
        let state = MainMenu::default();
        let selectable = App::selectable(&state);
        Self {
            page: Page::default(),
            state,
            selectable,
            client: Client::new(String::from("https://vid.puffyan.us")),
            selected: None,
            hover: None,
            message: None,
            load: true,
            render: true,
            history: Vec::new(),
        }
    }

    pub fn selectable(state: &Vec<Row>) -> Vec<Vec<(usize, usize)>> {
        let mut selectable = Vec::new();

        for (y, row) in state.iter().enumerate() {
            let mut row_vec = Vec::new();
            for (x, row_item) in row.items.iter().enumerate() {
                if match &row_item.item {
                    Item::Global(item) => item.selectable(),
                    Item::MainMenu(item) => item.selectable(),
                    Item::ItemInfo(item) => item.selectable(),
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
        if let Some((x, y)) = self.selected {
            if key != KeyCode::Esc {
                match &mut self.state[y].items[x].item {
                    Item::Global(item) => {
                        let mut item = item.clone();
                        let hold = item.key_input(key, self);
                        self = hold.1;
                        if hold.0 {
                            self.state[y].items[x].item = Item::Global(item);
                        }
                    }
                    Item::MainMenu(item) => {
                        let mut item = item.clone();
                        let hold = item.key_input(key, self);
                        self = hold.1;
                        if hold.0 {
                            self.state[y].items[x].item = Item::MainMenu(item);
                        }
                    }
                    Item::ItemInfo(item) => {
                        let mut item = item.clone();
                        let hold = item.key_input(key, self);
                        self = hold.1;
                        if hold.0 {
                            self.state[y].items[x].item = Item::ItemInfo(item);
                        }
                    }
                }

                return self;
            }
        }

        match key {
            KeyCode::Enter => {
                if self.hover.is_some() && self.selected.is_none() {
                    let (mut x, mut y) = self.hover.unwrap();
                    (x, y) = self.selectable[y][x];
                    match self.state[y]
                        .items
                        .iter()
                        .nth(x)
                        .clone()
                        .unwrap()
                        .item
                        .clone()
                    {
                        Item::Global(mut item) => {
                            let held = item.select(self);
                            self = held.0;
                            if held.1 {
                                self.selected = Some((x, y));
                            }
                        }
                        Item::MainMenu(mut item) => {
                            let held = item.select(self);
                            self = held.0;
                            if held.1 {
                                self.selected = Some((x, y));
                            }
                        }
                        Item::ItemInfo(mut item) => {
                            let held = item.select(self);
                            self = held.0;
                            if held.1 {
                                self.selected = Some((x, y));
                            }
                        }
                    }

                    return self;
                }
            }
            KeyCode::Esc => {
                if self.selected.is_some() {
                    self.selected = None;
                    return self;
                }
            }

            _ => {}
        }

        match &mut self.hover {
            Some((x, y)) => match key {
                KeyCode::Up => {
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
                KeyCode::Down => {
                    if *y < self.selectable.len() - 1 {
                        *y += 1;
                        if *x > self.selectable[*y].len() - 1 {
                            *x = self.selectable[*y].len() - 1;
                        }
                    }
                }

                KeyCode::Left => {
                    if *x > 0 {
                        *x -= 1;
                    }
                }

                KeyCode::Right => {
                    if *x < self.selectable[*y].len() - 1 {
                        *x += 1;
                    }
                }

                _ => {}
            },
            None => match key {
                KeyCode::Up => {
                    self.hover = Some((0, 0));
                }
                KeyCode::Down => {
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                KeyCode::Left => {
                    self.hover = Some((0, 0));
                }
                KeyCode::Right => {
                    self.hover = Some((0, self.selectable.len() - 1));
                }
                _ => {}
            },
        }

        self
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let size = frame.size();

        if size.width < 45 || size.height < 16 {
            let paragraph = Paragraph::new(format!(
                "Window too small. Minimum size 45 x 16. Current size is {} x {}",
                size.width, size.height
            ))
            .block(Block::default())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, size);
            return;
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

        for (y, (row, row_chunk)) in self
            .state
            .iter_mut()
            .zip(vertical_chunks.clone().into_iter())
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
                .split(row_chunk)
                .into_iter();

            frame.render_widget(Block::default(), chunks.next().unwrap());

            for (x, (chunk, item)) in chunks
                .zip(row.items.iter_mut().map(|i| &mut i.item))
                .enumerate()
            {
                let selected = self.selected == Some((x, y));

                let hover = hover_selected == Some((x, y));

                match item {
                    Item::Global(i) => {
                        i.render_item(frame, chunk, selected, hover, &self.message);
                    }
                    Item::MainMenu(i) => {
                        i.render_item(frame, chunk, selected, hover, &self.page);
                    }
                    Item::ItemInfo(i) => {
                        i.render_item(frame, chunk, selected, hover);
                    }
                }
            }
        }
    }

    pub fn pop(&mut self) {
        if self.history.len() == 0 {
            self.message = Some(String::from("This is the beginning of history"));
            return;
        }

        let app_history = self.history.pop().unwrap();

        *self = Self {
            state: app_history.state,
            selectable: app_history.selectable,
            selected: app_history.selected,
            hover: app_history.hover,
            message: app_history.message,
            page: app_history.page,
            client: app_history.client,
            load: app_history.load,
            render : app_history.render,
            history: self.history.clone(),
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Page {
    MainMenu(MainMenuSelector),
    ItemDisplay(DisplayItem),
}

impl Page {
    pub fn default() -> Self {
        Self::MainMenu(MainMenuSelector::default())
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
    ItemInfo(ItemInfoItem),
}

#[derive(Debug, Clone)]
pub struct Row {
    pub items: Vec<RowItem>,
    pub centered: bool,
    pub height: Constraint,
}

#[derive(Debug, Clone)]
pub struct RowItem {
    pub item: Item,
    pub constraint: Constraint,
}

#[derive(Debug, Clone)]
pub struct AppHistory {
    pub page: Page,
    pub state: Vec<Row>, // Item
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: Option<String>,
    pub load: bool,
    pub render: bool,
}

impl From<App> for AppHistory {
    fn from(original: App) -> Self {
        Self {
            page: original.page,
            state: original.state,
            selectable: original.selectable,
            hover: original.hover,
            selected: original.selected,
            client: original.client,
            message: original.message,
            load: original.load,
            render: original.render,
        }
    }
}
