use crate::global::traits::Collection;

use super::Item;
use serde::{Deserialize, Serialize};
use typemap::Key;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Library(pub Vec<Item>);

impl Key for Library {
    type Value = Self;
}

impl Collection<Item> for Library {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/library.json";

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
