use std::sync::{Arc, Mutex};

use invidious::blocking::Client;

use crate::{
    app::app::App,
    structs::{Page, Row},
};

#[derive(Debug, Clone)]
pub struct AppHistory {
    pub page: Page,
    pub state: Vec<Row>, // Item
    pub selectable: Vec<Vec<(usize, usize)>>,
    pub hover: Option<(usize, usize)>, // x, y
    pub selected: Option<(usize, usize)>,
    pub client: Client,
    pub message: Arc<Mutex<Option<String>>>,
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
