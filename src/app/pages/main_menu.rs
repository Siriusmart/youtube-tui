use std::{collections::LinkedList, error::Error};

use crate::{
    app::{
        app::{App, Item, Page, Row, RowItem},
        pages::global::{GlobalItem, ListItem},
    },
    functions::download_all_thumbnails,
    traits::{KeyInput, LoadItem, SelectItem},
    widgets::text_list::TextList,
};
use crossterm::event::KeyCode;
use tui::layout::Constraint;
#[derive(Debug, Clone)]
pub enum MainMenuItem {
    SeletorTab(MainMenuSelector),
    VideoList(Option<(LinkedList<ListItem>, TextList, Option<usize>)>), // Videos, List, page
}

// app.client.trending(None)

impl SelectItem for MainMenuItem {
    fn select(&mut self, mut app: App) -> (App, bool) {
        let selected = match self {
            MainMenuItem::SeletorTab(selector) => {
                if app.page == (Page::MainMenu { tab: *selector }) {
                    return (app, false);
                }
                app.page = Page::MainMenu { tab: *selector };
                app.load = true;
                app.message = Some(String::from("Loading videos..."));
                false
            }

            _ => true,
        };

        (app, selected)
    }

    fn selectable(&self) -> bool {
        true
    }
}

impl KeyInput for MainMenuItem {
    fn key_input(&mut self, key: KeyCode, app: App) -> App {
        match self {
            MainMenuItem::VideoList(Some((_, list, _))) => match key {
                KeyCode::Up => list.up(),
                KeyCode::Down => list.down(),
                KeyCode::PageUp => list.selected = 0,
                KeyCode::PageDown => list.selected = list.items.len() - 1,
                _ => {}
            },
            _ => {}
        }

        app
    }
}

impl LoadItem for MainMenuItem {
    fn load_item(&self, app: &App) -> Result<Box<Self>, Box<dyn Error>> {
        let mut this = self.clone();

        match &mut this {
            MainMenuItem::VideoList(enum_items) => match app.page {
                Page::MainMenu { tab } => match tab {
                    MainMenuSelector::Trending => {
                        let list: LinkedList<ListItem> = app
                            .client
                            .trending(None)?
                            .videos
                            .into_iter()
                            .map(|t| ListItem::Video(t.into()))
                            .collect();

                        download_all_thumbnails(list.clone())?;

                        let mut text_list = TextList::default();

                        if let Some((video_list, text_list, _)) = enum_items {
                            *text_list = TextList::default();
                            text_list.items(textlist_from_video_list(&list));

                            *video_list = list;
                        } else {
                            text_list.items(textlist_from_video_list(&list));
                            *enum_items = Some((list, text_list, None));
                        };
                    }

                    MainMenuSelector::Popular => {
                        let list: LinkedList<ListItem> = app
                            .client
                            .popular(None)?
                            .items
                            .into_iter()
                            .map(|t| ListItem::Video(t.into()))
                            .collect();

                            download_all_thumbnails(list.clone())?;

                        let mut text_list = TextList::default();

                        if let Some((video_list, text_list, _)) = enum_items {
                            *text_list = TextList::default();
                            text_list.items(textlist_from_video_list(&list));

                            *video_list = list;
                        } else {
                            text_list.items(textlist_from_video_list(&list));
                            *enum_items = Some((list, text_list, None));
                        };
                    }

                    _ => {}
                },
            },
            _ => {}
        }

        Ok(Box::new(this))
    }
}

fn textlist_from_video_list(original: &LinkedList<ListItem>) -> Vec<String> {
    original
        .iter()
        .map(|item| match item {
            ListItem::Video(video) => video
                .title
                .clone()
                .chars()
                .map(|c| if c.is_ascii() { c } else { '?' })
                .collect(),
            _ => {
                unreachable!()
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
                    item: Item::Global(GlobalItem::SearchBar(String::new())),
                    constraint: Constraint::Percentage(100),
                }],
                centered: false,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![
                    RowItem {
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
                    },
                ],
                centered: true,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![RowItem {
                    item: Item::MainMenu(MainMenuItem::VideoList(None)),
                    constraint: Constraint::Percentage(100),
                }],
                centered: false,
                height: Constraint::Min(6),
            },
            Row {
                items: vec![RowItem {
                    item: Item::Global(GlobalItem::MessageBar),
                    constraint: Constraint::Percentage(100),
                }],
                centered: false,
                height: Constraint::Length(3),
            },
        ]
    }
}
