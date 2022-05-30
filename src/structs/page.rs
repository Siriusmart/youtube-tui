use crate::app::pages::{item_info::DisplayItem, main_menu::MainMenuSelector};

#[derive(Debug, PartialEq, Clone)]
pub enum Page {
    MainMenu(MainMenuSelector),
    ItemDisplay(DisplayItem),
    Search,
}

impl Default for Page {
    fn default() -> Self {
        Self::MainMenu(MainMenuSelector::default())
    }
}
