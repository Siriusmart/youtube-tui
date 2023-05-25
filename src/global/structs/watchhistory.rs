use crate::global::{structs::Item, traits::Collection};
use serde::{Deserialize, Serialize};
use typemap::Key;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WatchHistory(pub Vec<Item>);

impl Key for WatchHistory {
    type Value = Self;
}

impl Collection<Item> for WatchHistory {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/watch_history.json";

    fn items(&self) -> &Vec<Item> {
        &self.0
    }

    fn items_mut(&mut self) -> &mut Vec<Item> {
        &mut self.0
    }

    fn from_items(items: Vec<Item>) -> Self {
        Self(items)
    }
}
