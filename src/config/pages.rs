use super::ConfigTrait;
use crate::items::{ItemList, MessageBar, PageButton, SearchBar, SearchFilter};
use serde::{Deserialize, Serialize};
use std::slice;
use tui::layout::Constraint;
use tui_additions::framework::{Framework, FrameworkItem, Row, RowItem, State};
use typemap::Key;

// Minimum screen dimention for the tui to display without panicking,
// is automatically generated from PageConfig
// will be inserted into `framework.data.state`
#[derive(Clone, Copy)]
pub struct MinDimentions {
    pub width: u16,
    pub height: u16,
}

impl MinDimentions {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl Default for MinDimentions {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Key for MinDimentions {
    type Value = Self;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PagesConfig {
    #[serde(default = "main_menu_default")]
    pub main_menu: PageConfig,
    #[serde(default = "search_default")]
    pub search: PageConfig,
}

impl Key for PagesConfig {
    type Value = Self;
}

impl ConfigTrait for PagesConfig {
    const LABEL: &'static str = "pages";
}

impl Default for PagesConfig {
    fn default() -> Self {
        Self {
            main_menu: main_menu_default(),
            search: search_default(),
        }
    }
}

// This struct impls Into<State>,  the corresponding page is converted into State on page load
// Each page has its own minimum width and height, which is automatically determined by the items in the page
// If it doesn't meet the minimum dimentions, a "protective screen" will be shown to prevent panicking
#[derive(Clone, Serialize, Deserialize)]
pub struct PageConfig {
    pub layout: Vec<PageRow>,
    pub message: String,
}

impl PageConfig {
    pub fn min_width(&self) -> u16 {
        self.layout
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| constraint_to_u16(&item.width()))
                    .sum::<u16>()
            })
            .max()
            .unwrap()
    }

    pub fn min_height(&self) -> u16 {
        self.layout
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| constraint_to_u16(&item.height()))
                    .max()
                    .unwrap()
            })
            .sum::<u16>()
    }

    pub fn to_state(&self, framework: &mut Framework) -> State {
        State(
            self.layout
                .iter()
                .map(|row| Row {
                    centered: row.is_centered(),
                    height: max_constraint(
                        &row.iter().map(|item| item.height()).collect::<Vec<_>>(),
                    ),
                    items: row
                        .iter()
                        .map(|item| RowItem {
                            item: item.to_framework_item(framework),
                            width: item.width(),
                        })
                        .collect::<Vec<_>>(),
                })
                .collect::<Vec<_>>(),
        )
    }
}

// CenteredRow will have its items centered, while NonCenteredRow will align to the left
#[derive(Clone, Serialize, Deserialize)]
pub enum PageRow {
    CenteredRow(Vec<PageItems>),
    NonCenteredRow(Vec<PageItems>),
}

impl PageRow {
    pub fn iter(&self) -> slice::Iter<PageItems> {
        match self {
            Self::CenteredRow(iter) | Self::NonCenteredRow(iter) => iter.iter(),
        }
    }

    pub fn is_centered(&self) -> bool {
        match self {
            Self::CenteredRow(_) => true,
            Self::NonCenteredRow(_) => false,
        }
    }

    pub fn from_vec(items: Vec<PageItems>, centered: bool) -> Self {
        if centered {
            Self::CenteredRow(items)
        } else {
            Self::NonCenteredRow(items)
        }
    }
}

// PageItems will be converted into Box<dyn FrameworkItem> on page load to be used an item in the framework
// Seen https://docs.rs/tui-additions/latest/tui_additions/framework/trait.FrameworkItem.html
// Each item has a minimum width and height for it to render without panicking
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum PageItems {
    SearchBar,
    ItemList,
    MessageBar,
    Trending,
    Popular,
    SearchFilters,
}

impl PageItems {
    pub fn to_framework_item(&self, framework: &mut Framework) -> Box<dyn FrameworkItem> {
        match *self {
            Self::SearchBar => Box::new(SearchBar::default()),
            Self::Popular => Box::new(PageButton::Popular),
            Self::Trending => Box::new(PageButton::Trending),
            Self::MessageBar => Box::new(MessageBar::default()),
            Self::ItemList => Box::new(ItemList::new(framework)),
            Self::SearchFilters => Box::new(SearchFilter::default()),
        }
    }

    pub fn width(&self) -> Constraint {
        match self {
            Self::Popular | Self::Trending => Constraint::Length(15),
            Self::SearchBar => Constraint::Min(16),
            Self::MessageBar => Constraint::Min(3),
            Self::ItemList => Constraint::Min(9),
            Self::SearchFilters => Constraint::Length(5),
        }
    }

    pub fn height(&self) -> Constraint {
        match self {
            Self::Popular
            | Self::Trending
            | Self::MessageBar
            | Self::SearchBar
            | Self::SearchFilters => Constraint::Length(3),
            Self::ItemList => Constraint::Min(6),
        }
    }
}

fn constraint_to_u16(constraint: &Constraint) -> u16 {
    match constraint {
        Constraint::Max(length) | Constraint::Min(length) | Constraint::Length(length) => *length,
        _ => unreachable!(),
    }
}

fn max_constraint(constraints: &[Constraint]) -> Constraint {
    let mut max_out = Constraint::Length(0);
    let mut max = 0_u16;

    constraints.iter().for_each(|constraint| {
        let self_len = constraint_to_u16(constraint);
        if self_len > max {
            max = self_len;
            max_out = *constraint;
        }
    });

    max_out
}

// default functions

fn main_menu_default() -> PageConfig {
    PageConfig {
        layout: vec![
            PageRow::from_vec(vec![PageItems::SearchBar, PageItems::SearchFilters], false),
            PageRow::from_vec(vec![PageItems::Trending, PageItems::Popular], true),
            PageRow::from_vec(vec![PageItems::ItemList], false),
            PageRow::from_vec(vec![PageItems::MessageBar], false),
        ],
        message: String::from("Loading main menu..."),
    }
}

fn search_default() -> PageConfig {
    PageConfig {
        layout: vec![
            PageRow::from_vec(vec![PageItems::SearchBar, PageItems::SearchFilters], false),
            PageRow::from_vec(vec![PageItems::ItemList], false),
            PageRow::from_vec(vec![PageItems::MessageBar], false),
        ],
        message: String::from("Loading search results..."),
    }
}
