use crate::{
    app::pages::{
        channel::{Channel, ChannelDisplayItem, ChannelItem, ChannelPage},
        global::GlobalItem,
        item_info::{ItemInfo, ItemInfoItem},
        main_menu::{MainMenu, MainMenuItem, MainMenuSelector},
        search::{Search, SearchItem},
    },
    structs::{Item, Row, RowItem, State},
    traits::{ConfigItem, PageTrait},
    widgets::text_list::TextList,
};
use serde::{Deserialize, Serialize};
use tui::layout::Constraint;

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutsConfigs {
    #[serde(default = "MainMenu::default")]
    pub main_menu: LayoutConfig,
    #[serde(default = "Channel::default")]
    pub channel: LayoutConfig,
    #[serde(default = "ItemInfo::default")]
    pub item_info: LayoutConfig,
    #[serde(default = "Search::default")]
    pub search: LayoutConfig,
}

impl Default for LayoutsConfigs {
    fn default() -> Self {
        Self {
            main_menu: MainMenu::default(),
            channel: Channel::default(),
            item_info: ItemInfo::default(),
            search: Search::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ConstraintTransitional {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    Max(u16),
    Min(u16),
}

impl Into<Constraint> for ConstraintTransitional {
    fn into(self) -> Constraint {
        match self {
            Self::Percentage(i) => Constraint::Percentage(i),
            Self::Ratio(i1, i2) => Constraint::Ratio(i1, i2),
            Self::Length(i) => Constraint::Length(i),
            Self::Max(i) => Constraint::Max(i),
            Self::Min(i) => Constraint::Min(i),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutConfig {
    pub layout: Vec<RowTransitional>,
    pub min: (u16, u16),
    pub message: String,
    pub def_selected: Option<(usize, usize)>,
}

impl Into<State> for LayoutConfig {
    fn into(self) -> State {
        State(self.layout.into_iter().map(|item| item.into()).collect())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RowTransitional {
    pub items: Vec<RowItemTransitional>,
    pub centered: bool,
    pub height: ConstraintTransitional,
}

impl Into<Row> for RowTransitional {
    fn into(self) -> Row {
        Row {
            items: self.items.into_iter().map(|item| item.into()).collect(),
            centered: self.centered,
            height: self.height.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RowItemTransitional {
    pub item: ItemTransitional,
    pub constraint: ConstraintTransitional,
}

impl Into<RowItem> for RowItemTransitional {
    fn into(self) -> RowItem {
        RowItem {
            item: self.item.into(),
            constraint: self.constraint.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemTransitional {
    Global(GlobalItem),
    MainMenu(MainMenuItemTransitional),
    ItemInfo,
    Search(SearchItemTransitional),
    Channel(ChannelItemTransitional),
}

impl Into<Item> for ItemTransitional {
    fn into(self) -> Item {
        match self {
            Self::Global(i) => Item::Global(i),
            Self::MainMenu(i) => Item::MainMenu(i.into()),
            Self::ItemInfo => Item::ItemInfo(ItemInfoItem::Unknown),
            Self::Search(i) => Item::Search(i.into()),
            Self::Channel(i) => Item::Channel(i.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MainMenuItemTransitional {
    SelectorTab(MainMenuSelector),
    VideoList,
}

impl Into<MainMenuItem> for MainMenuItemTransitional {
    fn into(self) -> MainMenuItem {
        match self {
            Self::SelectorTab(i) => MainMenuItem::SeletorTab(i),
            Self::VideoList => MainMenuItem::VideoList(None),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ChannelItemTransitional {
    InfoDisplay,
    SelectItems(ChannelPage),
}

impl Into<ChannelItem> for ChannelItemTransitional {
    fn into(self) -> ChannelItem {
        match self {
            Self::InfoDisplay => ChannelItem::InfoDisplay(ChannelDisplayItem::Unknown),
            Self::SelectItems(i) => ChannelItem::SelectItems(i),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SearchItemTransitional {
    Search,
}

impl Into<SearchItem> for SearchItemTransitional {
    fn into(self) -> SearchItem {
        match self {
            Self::Search => SearchItem::Search {
                results: None,
                text_list: TextList::default(),
            },
        }
    }
}

impl ConfigItem<'_> for LayoutsConfigs {
    type Struct = LayoutsConfigs;
    const FILE_NAME: &'static str = "layouts.yml";
}
