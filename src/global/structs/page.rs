use std::fmt::Debug;

use crate::config::{PageConfig, PagesConfig, Search};
use tui_additions::framework::Framework;
use typemap::Key;

// Page can be converted into PageConfig, which can then be converted into State
/// Covers all possible pages and variants
#[derive(Clone, PartialEq, Eq)]
pub enum Page {
    MainMenu(MainMenuPage),
    Search(Search),
    SingleItem(SingleItemPage),
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}(_)",
            match self {
                Self::MainMenu(_) => "MainMenu",
                Self::Search(_) => "Search",
                Self::SingleItem(_) => "SingleItem",
            }
        ))
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::MainMenu(MainMenuPage::default())
    }
}

impl Key for Page {
    type Value = Self;
}

/// page variants for the main menu
#[derive(Clone, PartialEq, Eq)]
pub enum MainMenuPage {
    Trending,
    Popular,
}

impl Default for MainMenuPage {
    fn default() -> Self {
        Self::Trending
    }
}

/// Different items to be displayed on a single item page
#[derive(Clone, PartialEq, Eq)]
pub enum SingleItemPage {
    Video(String),
    Playlist(String),
}

impl Page {
    pub fn to_page_config(&self, framework: &Framework) -> PageConfig {
        let pages_config = framework.data.global.get::<PagesConfig>().unwrap();
        match self {
            Self::MainMenu(_) => pages_config.main_menu.clone(),
            Self::Search(_) => pages_config.search.clone(),
            Self::SingleItem(_) => pages_config.singleitem.clone(),
        }
    }

    // each page displays a text when loading, and that text is taken from config
    pub fn load_msg(&self, framework: &Framework) -> String {
        let pages_config = framework.data.global.get::<PagesConfig>().unwrap();
        match self {
            Self::MainMenu(_) => pages_config.main_menu.message.clone(),
            Self::Search(_) => pages_config.search.message.clone(),
            Self::SingleItem(_) => pages_config.singleitem.message.clone(),
        }
    }
}
