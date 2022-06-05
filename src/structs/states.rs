use tui::layout::Constraint;

use crate::app::pages::{
    channel::ChannelItem, global::GlobalItem, item_info::ItemInfoItem, main_menu::MainMenuItem,
    search::SearchItem,
};

#[derive(Debug, Clone)]
pub enum Item {
    Global(GlobalItem),
    MainMenu(MainMenuItem),
    ItemInfo(ItemInfoItem),
    Search(SearchItem),
    Channel(ChannelItem),
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
