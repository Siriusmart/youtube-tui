use std::{collections::LinkedList, error::Error, fs};

use crate::{
    app::{
        app::App,
        pages::{global::*, item_info::*},
    },
    functions::download_all_thumbnails,
    structs::{Item, ListItem, Page, Row, RowItem},
    traits::{KeyInput, LoadItem, SelectItem},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Rect},
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

impl SelectItem for MainMenuItem {
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
}

impl KeyInput for MainMenuItem {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        match self {
            MainMenuItem::VideoList(Some((video_list, list, _))) => match key {
                KeyCode::Up => list.up(),
                KeyCode::Down => list.down(),
                KeyCode::PageUp => list.selected = 0,
                KeyCode::PageDown => list.selected = list.items.len() - 1,
                KeyCode::Enter => {
                    let state = ItemInfo::default();
                    let mut history = app.history.clone();
                    history.push(app.into());

                    return (
                        false,
                        App {
                            history,
                            page: Page::ItemDisplay(DisplayItem::Video(
                                video_list.iter().nth(list.selected).unwrap().id(),
                            )),
                            selectable: App::selectable(&state),
                            state,
                            ..Default::default()
                        },
                    );
                }
                _ => {}
            },
            _ => {}
        }

        (true, app)
    }
}

impl LoadItem for MainMenuItem {
    fn load_item(&self, app: &App) -> Result<Box<Self>, Box<dyn Error>> {
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

                        download_all_thumbnails(
                            list.iter()
                                .map(|t| match t {
                                    ListItem::MiniVideo(v) => {
                                        (v.video_thumbnail.clone(), v.video_id.clone())
                                    }
                                    _ => unreachable!(),
                                })
                                .collect(),
                        )?;

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

                        download_all_thumbnails(
                            list.iter()
                                .map(|t| match t {
                                    ListItem::MiniVideo(v) => {
                                        (v.video_thumbnail.clone(), v.video_id.clone())
                                    }
                                    _ => unreachable!(),
                                })
                                .collect(),
                        )?;

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
                                    .join(format!("{}.json", t)),
                            );

                            if let Ok(file) = file {
                                let item: ListItem = serde_json::from_str(&file).unwrap();

                                let title = match &item {
                                    ListItem::FullVideo(v) => v.title.clone(),
                                    _ => unreachable!(),
                                };

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

        Ok(Box::new(this))
    }
}

impl MainMenuItem {
    pub fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        page: &Page,
    ) {
        let mut style = Style::default().fg(if selected {
            Color::LightBlue
        } else if hover {
            Color::LightRed
        } else {
            Color::Reset
        });

        match self {
            MainMenuItem::SeletorTab(selector) => {
                if !hover && page == &(Page::MainMenu(*selector)) {
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
                    list.area(chunks[0]);
                    let mut list = list.clone();

                    if selected {
                        list.selected_style(Style::default().fg(Color::LightRed));
                    } else {
                        list.selected_style(Style::default().fg(Color::LightYellow));
                    }

                    if let Some(item) = videos.iter().nth(list.selected) {
                        if !popup_focus {
                            frame.render_widget(ItemDisplay { item: item.clone() }, chunks[1]);
                        }
                    }

                    frame.render_widget(list, chunks[0]);
                }
            }
        }
    }
}

fn textlist_from_video_list(original: &LinkedList<ListItem>) -> Vec<String> {
    original
        .iter()
        .map(|item| match item {
            ListItem::MiniVideo(video) => video
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

impl Default for MainMenuSelector {
    fn default() -> Self {
        Self::Popular
    }
}

pub struct MainMenu;

impl MainMenu {
    pub fn message() -> String {
        String::from("Loading home page...")
    }

    pub fn min() -> (u16, u16) {
        (45, 15)
    }

    pub fn default() -> Vec<Row> {
        vec![
            Row {
                items: vec![
                    RowItem {
                        item: Item::Global(GlobalItem::SearchBar(String::new())),
                        constraint: Constraint::Min(16),
                    },
                    RowItem {
                        item: Item::Global(GlobalItem::SearchSettings),
                        constraint: Constraint::Length(5),
                    },
                ],
                centered: false,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![
                    RowItem {
                        item: Item::MainMenu(MainMenuItem::SeletorTab(MainMenuSelector::Popular)),
                        constraint: Constraint::Length(15),
                    },
                    RowItem {
                        item: Item::MainMenu(MainMenuItem::SeletorTab(MainMenuSelector::Trending)),
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
