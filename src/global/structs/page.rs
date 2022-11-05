use std::fmt::Debug;

use crate::config::{PageConfig, PagesConfig, Search};
use tui_additions::framework::Framework;
use typemap::Key;

// Page can be converted into PageConfig, which can then be converted into State
/// Covers all possible pages and variants
#[derive(Clone, PartialEq)]
pub enum Page {
    MainMenu(MainMenuPage),
    Search(Search),
    SingleItem(SingleItemPage),
    ChannelDisplay(ChannelDisplayPage),
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}(_)",
            match self {
                Self::MainMenu(_) => "MainMenu",
                Self::Search(_) => "Search",
                Self::SingleItem(_) => "SingleItem",
                Self::ChannelDisplay(_) => "ChannelDisplay",
            }
        ))
    }
}

impl Page {
    pub fn channeldisplay(&self) -> &ChannelDisplayPage {
        if let Self::ChannelDisplay(channeldisplaypage) = self {
            channeldisplaypage
        } else {
            panic!("not a channel display");
        }
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
#[derive(Clone, Copy, PartialEq)]
pub enum MainMenuPage {
    Trending,
    Popular,
}

impl Default for MainMenuPage {
    fn default() -> Self {
        Self::Trending
    }
}

#[derive(Clone, PartialEq)]
pub struct ChannelDisplayPage {
    pub id: String,
    pub r#type: ChannelDisplayPageType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChannelDisplayPageType {
    Main,
    Videos,
    Playlists,
}

/// Different items to be displayed on a single item page
#[derive(Clone, PartialEq)]
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
            Self::ChannelDisplay(_) => pages_config.channeldisplay.clone(),
        }
    }

    // each page displays a text when loading, and that text is taken from config
    pub fn load_msg(&self, framework: &Framework) -> String {
        let pages_config = framework.data.global.get::<PagesConfig>().unwrap();
        match self {
            Self::MainMenu(_) => pages_config.main_menu.message.clone(),
            Self::Search(_) => pages_config.search.message.clone(),
            Self::SingleItem(_) => pages_config.singleitem.message.clone(),
            Self::ChannelDisplay(_) => pages_config.channeldisplay.message.clone(),
        }
    }
}
