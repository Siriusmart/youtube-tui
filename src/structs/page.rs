use crate::app::pages::{main_menu::MainMenuSelector, item_info::DisplayItem};

#[derive(Debug, PartialEq, Clone)]
pub enum Page {
    MainMenu(MainMenuSelector),
    ItemDisplay(DisplayItem),
}

impl Default for Page {
    fn default() -> Self {
        Self::MainMenu(MainMenuSelector::default())
    }
}