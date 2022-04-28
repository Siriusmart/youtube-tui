use crate::app::app::{GlobalItem, Item, Row, RowItem};
use invidious::structs::video::Video;
use tui::layout::Constraint;

#[derive(Debug)]
pub enum MainMenuItem {
    SeletorTab(MainMenuSelector),
    VideoList(Option<Vec<Video>>, Option<usize>), // Videos, index
    VideoDetails(Option<Video>),                  // Single video
}

#[derive(Debug)]
pub enum MainMenuSelector {
    Trending,
    Popular,
    History,
}

impl MainMenuSelector {
    pub fn default() -> Self {
        Self::Trending
    }
}

pub struct MainMenu;

impl MainMenu {
    pub fn default() -> Vec<Row> {
        vec![
            Row {
                items: vec![RowItem {
                    item: Item::Global(GlobalItem::SearchBar(Vec::new())),
                    constraint: Constraint::Percentage(100),
                }],
                centered: false,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![RowItem {
                    item: Item::MainMenu(MainMenuItem::SeletorTab(MainMenuSelector::Trending)),
                    constraint: Constraint::Length(15),
                },
                RowItem {
                    item: Item::MainMenu(MainMenuItem::SeletorTab(MainMenuSelector::Popular)),
                    constraint: Constraint::Length(15),
                },
                RowItem {
                    item: Item::MainMenu(MainMenuItem::SeletorTab(MainMenuSelector::History)),
                    constraint: Constraint::Length(15),
                }],
                centered: true,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![
                    RowItem {
                        item: Item::MainMenu(MainMenuItem::VideoList(None, None)),
                        constraint: Constraint::Percentage(40),
                    },
                    RowItem {
                        item: Item::MainMenu(MainMenuItem::VideoDetails(None)),
                        constraint: Constraint::Percentage(60),
                    },
                ],
                centered: false,
                height: Constraint::Min(6),
            },
            Row {
                items: vec![RowItem {
                    item: Item::Global(GlobalItem::MessageBar(None)),
                    constraint: Constraint::Percentage(100),
                }],
                centered: false,
                height: Constraint::Length(3),
            },
        ]
    }
}
