use crate::app::pages::main_menu::*;
use invidious::blocking::Client;
use tui::layout::Constraint;

#[derive(Debug)]
pub struct App {
    pub page: Page,
    pub state: Vec<Row>,          // Item, Selectable
    pub selected: Option<Item>,
    pub client: Client,
}

impl App {
    pub fn new() -> Self {
        Self {
            page: Page::default(),
            selected: None,
            state: MainMenu::default(),
            client: Client::new(String::from("https://vid.puffyan.us")),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
}

#[derive(Debug)]
pub enum GlobalItem {
    SearchBar(Vec<char>),
    MessageBar(Option<String>),
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