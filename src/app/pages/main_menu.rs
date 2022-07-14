use std::{collections::LinkedList, error::Error, fs};

use crate::{
    app::{
        app::{App, AppNoState},
        config::{
            Action, ConstraintTransitional, ItemTransitional, LayoutConfig,
            MainMenuItemTransitional, RowItemTransitional, RowTransitional,
        },
        pages::{global::*, item_info::*},
    },
    functions::download_all_thumbnails,
    structs::{Item, ListItem, Page, WatchHistory},
    traits::{ItemTrait, PageTrait},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};
use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
#[derive(Debug, Clone)]
pub enum MainMenuItem {
    SeletorTab(MainMenuSelector),
    VideoList(Option<(LinkedList<ListItem>, TextList, Option<usize>)>), // Videos, List, page
}

// app.client.trending(None)

impl ItemTrait for MainMenuItem {
    fn load_item(&self, app: &App, _: &mut WatchHistory) -> Result<Item, Box<dyn Error>> {
        let mut this = self.clone();

        match &mut this {
            MainMenuItem::VideoList(enum_items) => match app.page {
                Page::MainMenu(tab) => match tab {
                    MainMenuSelector::Trending => {
                        let list: LinkedList<ListItem> = app
                            .client
                            .trending(None)?
                            .videos
                            .into_iter()
                            .map(|t| ListItem::MiniVideo(t.into()))
                            .collect();

                        if app.config.main.display_thumbnails {
                            let _ = download_all_thumbnails(
                                list.iter()
                                    .map(|t| match t {
                                        ListItem::MiniVideo(v) => {
                                            (v.video_thumbnail.clone(), v.video_id.clone())
                                        }
                                        _ => unreachable!(),
                                    })
                                    .collect(),
                            );
                        }

                        if let Some((video_list, text_list, _)) = enum_items {
                            *text_list = TextList::default();
                            text_list.items(textlist_from_video_list(&list));

                            *video_list = list;
                        } else {
                            let mut text_list = TextList::default();
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
                            .map(|t| ListItem::MiniVideo(t.into()))
                            .collect();

                        if app.config.main.display_thumbnails {
                            let _ = download_all_thumbnails(
                                list.iter()
                                    .map(|t| match t {
                                        ListItem::MiniVideo(v) => {
                                            (v.video_thumbnail.clone(), v.video_id.clone())
                                        }
                                        _ => unreachable!(),
                                    })
                                    .collect(),
                            );
                        }

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

                    MainMenuSelector::History => {
                        let mut list = LinkedList::new();
                        let mut text_list = TextList::default();

                        app.watch_history.0.iter().for_each(|t| {
                            let file = fs::read_to_string(
                                home::home_dir()
                                    .expect("Cannot get your home directory")
                                    .join(".local/share/youtube-tui/watch_history/info")
                                    .join(format!("{}.json", t.id())),
                            );

                            if let Ok(file) = file {
                                let item: ListItem = serde_json::from_str(&file).unwrap();

                                let title = item.to_string();

                                list.push_back(item);
                                text_list.items.push(title);
                            }
                        });

                        *enum_items = Some((list, text_list, None));
                    }
                },

                _ => {}
            },
            _ => {}
        }

        Ok(Item::MainMenu(this))
    }

    fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: AppNoState,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        popup_render: bool,
    ) -> (bool, AppNoState) {
        let out = (false, app);

        if popup_render {
            return out;
        }

        let mut style = Style::default().fg(if selected {
            Color::LightBlue
        } else if hover {
            Color::LightRed
        } else {
            Color::Reset
        });

        match self {
            MainMenuItem::SeletorTab(selector) => {
                if !hover && &out.1.page == &(Page::MainMenu(*selector)) {
                    style = style.fg(Color::LightYellow);
                }
                let text = match selector {
                    MainMenuSelector::Trending => "Trending",
                    MainMenuSelector::Popular => "Popular",
                    MainMenuSelector::History => "History",
                };

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(style);
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, rect);
            }
            MainMenuItem::VideoList(data) => {
                let split = HorizontalSplit::default()
                    .percentages(vec![60, 40])
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));

                let chunks = split.inner(rect);

                frame.render_widget(split, rect);

                if let Some((videos, list, _)) = data {
                    if selected {
                        list.selected_style(Style::default().fg(Color::LightRed));
                    } else {
                        list.selected_style(Style::default().fg(Color::LightYellow));
                    }

                    if let Some(item) = videos.iter().nth(list.selected) {
                        if !popup_render {
                            frame.render_widget(ItemDisplay { item: item.clone(), render_image: !popup_focus }, chunks[1]);
                        }
                    }

                    list.area(chunks[0]);

                    let list = list.clone();

                    frame.render_widget(list, chunks[0]);
                }
            }
        }
        out
    }

    fn reset(&mut self) {
        if let Self::VideoList(something) = self {
            *something = None;
        }
    }

    fn select(&mut self, mut app: App) -> (App, bool) {
        let selected = match self {
            MainMenuItem::SeletorTab(selector) => {
                if app.page == (Page::MainMenu(*selector)) {
                    return (app, false);
                }
                app.page = Page::MainMenu(*selector);
                app.load = true;
                false
            }

            _ => true,
        };

        (app, selected)
    }

    fn selectable(&self) -> bool {
        true
    }

    fn key_input(&mut self, key: KeyEvent, app: App) -> (bool, App) {
        match self {
            MainMenuItem::VideoList(Some((list, textlist, _))) => {
                let action = match app.config.keybindings.0.get(&key) {
                    Some(action) => action,
                    None => return (false, app),
                };
                match action {
                    Action::Up => textlist.up(),
                    Action::Down => textlist.down(),
                    Action::FirstItem => textlist.first(),
                    Action::LastItem => textlist.last(),
                    Action::Select => {
                        let state = app.config.layouts.item_info.clone().into();
                        let mut history = app.history.clone();
                        history.push(app.into());

                        return (
                            false,
                            App {
                                history,
                                page: Page::ItemDisplay(
                                    match list.iter().nth(textlist.selected).unwrap() {
                                        ListItem::FullVideo(item) => {
                                            DisplayItem::Video(item.video_id.clone())
                                        }
                                        ListItem::FullPlayList(item) => {
                                            DisplayItem::PlayList(item.playlist_id.clone())
                                        }
                                        ListItem::MiniPlayList(item) => {
                                            DisplayItem::PlayList(item.playlist_id.clone())
                                        }
                                        ListItem::MiniVideo(item) => {
                                            DisplayItem::Video(item.video_id.clone())
                                        }
                                        _ => unreachable!(),
                                    },
                                ),
                                selectable: App::selectable(&state),
                                state,
                                ..Default::default()
                            },
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        (true, app)
    }
}

impl MainMenuItem {}

pub fn textlist_from_video_list(original: &LinkedList<ListItem>) -> Vec<String> {
    original.iter().map(|item| item.to_string()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MainMenuSelector {
    Trending,
    Popular,
    History,
}

impl Default for MainMenuSelector {
    fn default() -> Self {
        Self::Popular
    }
}

pub struct MainMenu;
impl PageTrait for MainMenu {
    fn default() -> LayoutConfig {
        LayoutConfig {
            layout: vec![
                RowTransitional {
                    items: vec![
                        RowItemTransitional {
                            item: ItemTransitional::Global(GlobalItem::SearchBar),
                            constraint: ConstraintTransitional::Min(16),
                        },
                        RowItemTransitional {
                            item: ItemTransitional::Global(GlobalItem::SearchSettings),
                            constraint: ConstraintTransitional::Length(5),
                        },
                    ],
                    centered: false,
                    height: ConstraintTransitional::Length(3),
                },
                RowTransitional {
                    items: vec![
                        RowItemTransitional {
                            item: ItemTransitional::MainMenu(
                                MainMenuItemTransitional::SelectorTab(MainMenuSelector::Popular),
                            ),
                            constraint: ConstraintTransitional::Length(15),
                        },
                        RowItemTransitional {
                            item: ItemTransitional::MainMenu(
                                MainMenuItemTransitional::SelectorTab(MainMenuSelector::Trending),
                            ),
                            constraint: ConstraintTransitional::Length(15),
                        },
                        RowItemTransitional {
                            item: ItemTransitional::MainMenu(
                                MainMenuItemTransitional::SelectorTab(MainMenuSelector::History),
                            ),
                            constraint: ConstraintTransitional::Length(15),
                        },
                    ],
                    centered: true,
                    height: ConstraintTransitional::Length(3),
                },
                RowTransitional {
                    items: vec![RowItemTransitional {
                        item: ItemTransitional::MainMenu(MainMenuItemTransitional::VideoList),
                        constraint: ConstraintTransitional::Percentage(100),
                    }],
                    centered: false,
                    height: ConstraintTransitional::Min(6),
                },
                RowTransitional {
                    items: vec![RowItemTransitional {
                        item: ItemTransitional::Global(GlobalItem::MessageBar),
                        constraint: ConstraintTransitional::Percentage(100),
                    }],
                    centered: false,
                    height: ConstraintTransitional::Length(3),
                },
            ],
            min: (45, 15),
            message: String::from("Loading home page..."),
            def_selected: None,     
        }
    }
}
