use crate::{
    app::pages::{
        channel::{Channel, ChannelPage},
        item_info::{DisplayItem, ItemInfo},
        main_menu::{MainMenu, MainMenuSelector},
        search::Search,
    },
    traits::PageTrait,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Page {
    MainMenu(MainMenuSelector),
    ItemDisplay(DisplayItem),
    Channel(ChannelPage, String),
    Search,
}

impl Default for Page {
    fn default() -> Self {
        Self::MainMenu(MainMenuSelector::default())
    }
}

impl Page {
    pub fn message(&self) -> String {
        match self {
            Self::MainMenu(_) => MainMenu::message(),
            Self::ItemDisplay(_) => ItemInfo::message(),
            Self::Channel(_, _) => Channel::message(),
            Self::Search => Search::message(),
        }
    }
}
