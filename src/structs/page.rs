use crate::app::{
    config::Config,
    pages::{channel::ChannelPage, item_info::DisplayItem, main_menu::MainMenuSelector},
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
    pub fn message(&self, config: &Config) -> String {
        match self {
            Self::MainMenu(_) => config.layouts.main_menu.message.clone(),
            Self::ItemDisplay(_) => config.layouts.item_info.message.clone(),
            Self::Channel(_, _) => config.layouts.channel.message.clone(),
            Self::Search => config.layouts.search.message.clone(),
        }
    }

    pub fn min(&self, config: &Config) -> (u16, u16) {
        match self {
            Self::MainMenu(_) => config.layouts.main_menu.min,
            Self::ItemDisplay(_) => config.layouts.item_info.min,
            Self::Channel(_, _) => config.layouts.channel.min,
            Self::Search => config.layouts.search.min,
        }
    }
}
