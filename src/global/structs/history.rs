use crate::global::{
    structs::Item,
    traits::{Collection, CollectionNoId},
};
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SearchHistory(pub Vec<String>);

impl Key for SearchHistory {
    type Value = Self;
}

impl CollectionNoId<String> for SearchHistory {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/search_history.json";

    fn items(&self) -> &Vec<String> {
        &self.0
    }

    fn items_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }

    fn from_items(items: Vec<String>) -> Self {
        Self(items)
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommandHistory(pub Vec<String>);

impl Key for CommandHistory {
    type Value = Self;
}

impl CollectionNoId<String> for CommandHistory {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/command_history.json";

    fn items(&self) -> &Vec<String> {
        &self.0
    }

    fn items_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }

    fn from_items(items: Vec<String>) -> Self {
        Self(items)
    }
}

/*
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ChannelHistory(pub Vec<String>);

impl Key for ChannelHistory {
    type Value = Self;
}

impl CollectionNoId<String> for ChannelHistory {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/channel_history.json";

    fn items(&self) -> &Vec<String> {
        &self.0
    }

    fn items_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }

    fn from_items(items: Vec<String>) -> Self {
        Self(items)
    }
}
*/
