use crate::config::{PageConfig, PagesConfig, Search};
use tui_additions::framework::Framework;
use typemap::Key;

// Page can be converted into PageConfig, which can then be converted into State
/// Covers all possible pages and variants
#[derive(Clone, PartialEq, Eq)]
pub enum Page {
    MainMenu(MainMenuPage),
    Search(Search),
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

impl Page {
    pub fn to_page_config(&self, framework: &Framework) -> PageConfig {
        let pages_config = framework.data.global.get::<PagesConfig>().unwrap();
        match self {
            Self::MainMenu(_) => pages_config.main_menu.clone(),
            Self::Search(_) => pages_config.search.clone(),
        }
    }

    // each page displays a text when loading, and that text is taken from config
    pub fn load_msg(&self, framework: &Framework) -> String {
        let pages_config = framework.data.global.get::<PagesConfig>().unwrap();
        match self {
            Self::MainMenu(_) => pages_config.main_menu.message.clone(),
            Self::Search(_) => pages_config.main_menu.message.clone(),
        }
    }
}
