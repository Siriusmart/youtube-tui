use crate::{
    app::pages::{global::*, main_menu::*},
    traits::{KeyInput, SelectItem},
};
use crossterm::event::KeyCode;
use invidious::blocking::Client;
use tui::layout::Constraint;

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
                        self = item.key_input(key, self);
                        self.state[y].items[x].item = Item::Global(item);
                    }
                    Item::MainMenu(item) => {
                        let mut item = item.clone();
                        self = item.key_input(key, self);
                        self.state[y].items[x].item = Item::MainMenu(item);
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
}

#[derive(Debug, PartialEq)]
pub enum Page {
    MainMenu { tab: MainMenuSelector },
}

impl Page {
    pub fn default() -> Self {
        Self::MainMenu {
            tab: MainMenuSelector::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
}

#[derive(Debug)]
pub struct Row {
    pub items: Vec<RowItem>,
    pub centered: bool,
    pub height: Constraint,
}

#[derive(Debug)]
pub struct RowItem {
    pub item: Item,
    pub constraint: Constraint,
}
