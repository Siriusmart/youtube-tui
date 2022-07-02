use crossterm::event::KeyCode;
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    Frame,
};

use crate::{
    app::{
        app::{App, AppNoState},
        pages::{
            channel::ChannelItem, global::GlobalItem, item_info::ItemInfoItem,
            main_menu::MainMenuItem, search::SearchItem,
        },
    },
    traits::ItemTrait,
};

use super::WatchHistory;

#[derive(Clone)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
    ItemInfo(ItemInfoItem),
    Search(SearchItem),
    Channel(ChannelItem),
}

impl Item {
    pub fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        match self {
            Item::Global(item) => item.key_input(key, app),
            Item::MainMenu(item) => item.key_input(key, app),
            Item::ItemInfo(item) => item.key_input(key, app),
            Item::Search(item) => item.key_input(key, app),
            Item::Channel(item) => item.key_input(key, app),
        }
    }

    pub fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: AppNoState,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        popup_render: bool,
    ) -> (bool, AppNoState) {
        match self {
            Item::Global(item) => {
                item.render_item(frame, rect, app, selected, hover, popup_focus, popup_render)
            }
            Item::MainMenu(item) => {
                item.render_item(frame, rect, app, selected, hover, popup_focus, popup_render)
            }
            Item::ItemInfo(item) => {
                item.render_item(frame, rect, app, selected, hover, popup_focus, popup_render)
            }
            Item::Search(item) => {
                item.render_item(frame, rect, app, selected, hover, popup_focus, popup_render)
            }
            Item::Channel(item) => {
                item.render_item(frame, rect, app, selected, hover, popup_focus, popup_render)
            }
        }
    }

    pub fn select(&mut self, app: App) -> (App, bool) {
        match self {
            Item::Global(item) => item.select(app),
            Item::MainMenu(item) => item.select(app),
            Item::ItemInfo(item) => item.select(app),
            Item::Search(item) => item.select(app),
            Item::Channel(item) => item.select(app),
        }
    }

    pub fn load_item(
        &self,
        app: &App,
        watchhistory: &mut WatchHistory,
    ) -> Result<Self, Box<dyn Error>> {
        match self {
            Item::Global(item) => item.load_item(app, watchhistory),
            Item::MainMenu(item) => item.load_item(app, watchhistory),
            Item::ItemInfo(item) => item.load_item(app, watchhistory),
            Item::Search(item) => item.load_item(app, watchhistory),
            Item::Channel(item) => item.load_item(app, watchhistory),
        }
    }
}

#[derive(Clone)]
pub struct Row {
    pub items: Vec<RowItem>,
    pub centered: bool,
    pub height: Constraint,
}

#[derive(Clone)]
pub struct RowItem {
    pub item: Item,
    pub constraint: Constraint,
}
