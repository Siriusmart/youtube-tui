use std::{collections::LinkedList, error::Error};

use crossterm::event::KeyCode;
use invidious::structs::hidden::SearchItem as InvidiousSearchItem;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    Frame,
};

use crate::{
    app::{app::App, config::Action},
    functions::download_all_thumbnails,
    structs::{
        Item, ListItem, MiniChannel, MiniPlayList, MiniVideo, Page, Row, RowItem, WatchHistory,
    },
    traits::{ItemTrait, PageTrait},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};

use super::{
    channel::{Channel, ChannelPage},
    global::GlobalItem,
    item_info::{DisplayItem, ItemInfo},
    main_menu::textlist_from_video_list,
};

#[derive(Debug, Clone)]
pub enum SearchItem {
    Search {
        results: Option<LinkedList<ListItem>>,
        text_list: TextList,
    },
}

impl ItemTrait for SearchItem {
    fn select(&mut self, app: App) -> (App, bool) {
        (app, true)
    }

    fn selectable(&self) -> bool {
        true
    }

    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        let action = match app.config.keybindings.0.get(&key) {
            Some(action) => action,
            None => return (false, app),
        };
        match self {
            
            SearchItem::Search { results, text_list } => match action {
                Action::Up => text_list.up(),
                Action::Down => text_list.down(),
                Action::FirstItem => text_list.selected = 0,
                Action::LastItem => text_list.selected = text_list.items.len() - 1,
                Action::Select => {
                    if let Some(results_unwrapped) = results {
                        match results_unwrapped.iter().nth(text_list.selected).unwrap() {
                            ListItem::PageTurner(b) => {
                                let new_page_no;
                                if *b {
                                    new_page_no = app.page_no + 1;
                                } else {
                                    new_page_no = app.page_no - 1;
                                }

                                *results = None;
                                *text_list = TextList::default();

                                let state = Search::default();
                                let mut history = app.history.clone();
                                let search_text = app.search_text.clone();
                                let search_settings = app.search_settings.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::Search,
                                        selectable: App::selectable(&state),
                                        state,
                                        search_text,
                                        search_settings,
                                        load: true,
                                        page_no: new_page_no,
                                        ..Default::default()
                                    },
                                );
                            }

                            ListItem::MiniVideo(video) => {
                                let state = ItemInfo::default();
                                let mut history = app.history.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::ItemDisplay(DisplayItem::Video(
                                            video.video_id.clone(),
                                        )),
                                        selectable: App::selectable(&state),
                                        state,
                                        ..Default::default()
                                    },
                                );
                            }

                            ListItem::MiniPlayList(playlist) => {
                                let state = ItemInfo::default();
                                let mut history = app.history.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::ItemDisplay(DisplayItem::PlayList(
                                            playlist.playlist_id.clone(),
                                        )),
                                        selectable: App::selectable(&state),
                                        state,
                                        ..Default::default()
                                    },
                                );
                            }

                            ListItem::MiniChannel(channel) => {
                                let state = Channel::default();
                                let mut history = app.history.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::Channel(
                                            ChannelPage::Home,
                                            channel.author_id.clone(),
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
                }
                _ => {}
            },
        }

        (true, app)
    }

    fn load_item(&self, app: &App, _: &mut WatchHistory) -> Result<Item, Box<dyn Error>> {
        let mut this = self.clone();

        match &mut this {
            SearchItem::Search { results, text_list } => {
                // panic!("got here");

                let mut args = vec![app.search_text.clone()];
                args.extend(app.search_settings.clone().to_vec());

                let invidious_results = app
                    .client
                    .search(Some(&format!(
                        "page={}&q={}",
                        app.page_no,
                        args.join("&").as_str()
                    )))?
                    .items;

                let mut items = LinkedList::new();

                for item in invidious_results {
                    match item {
                        InvidiousSearchItem::Video {
                            title: _,
                            videoId: _,
                            author: _,
                            authorId: _,
                            authorUrl: _,
                            lengthSeconds: _,
                            videoThumbnails: _,
                            description: _,
                            descriptionHtml: _,
                            viewCount: _,
                            published: _,
                            publishedText: _,
                            liveNow: _,
                            paid: _,
                            premium: _,
                        } => {
                            items.push_back(ListItem::MiniVideo(MiniVideo::from(item)));
                        }

                        InvidiousSearchItem::Playlist {
                            title: _,
                            playlistId: _,
                            author: _,
                            authorId: _,
                            authorUrl: _,
                            videoCount: _,
                            videos: _,
                        } => {
                            items.push_back(ListItem::MiniPlayList(MiniPlayList::from(item)));
                        }

                        InvidiousSearchItem::Channel {
                            author: _,
                            authorId: _,
                            authorUrl: _,
                            authorThumbnails: _,
                            subCount: _,
                            videoCount: _,
                            description: _,
                            descriptionHtml: _,
                        } => {
                            items.push_back(ListItem::MiniChannel(MiniChannel::from(item)));
                        }

                        _ => {}
                    }
                }

                let mut minimum_length = 0;

                if app.page_no != 1 {
                    minimum_length += 1;
                    items.push_front(ListItem::PageTurner(false));
                }

                if items.len() == minimum_length {
                    *app.message.lock().unwrap() = Some(String::from("No results found"));
                } else {
                    items.push_back(ListItem::PageTurner(true));
                }

                text_list.items = textlist_from_video_list(&items);

                let mut thumbnails = LinkedList::new();

                for item in items.iter() {
                    match item {
                        ListItem::MiniVideo(video) => {
                            thumbnails
                                .push_back((video.video_thumbnail.clone(), video.video_id.clone()));
                        }

                        ListItem::MiniPlayList(playlist) => {
                            if let Some(thumbnail) = &playlist.thumbnail {
                                thumbnails
                                    .push_back((thumbnail.clone(), playlist.playlist_id.clone()));
                            }
                        }

                        ListItem::MiniChannel(channel) => {
                            thumbnails.push_back((
                                format!("https:{}", channel.thumbnail.clone()),
                                channel.author_id.clone(),
                            ));

                            // panic!("{:?}", channel.thumbnail.clone());
                        }

                        _ => {}
                    }
                }

                let _ = download_all_thumbnails(thumbnails);

                *results = Some(items);
            }
        }

        Ok(Item::Search(this))
    }

    fn render_item<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: App,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        _: bool,
    ) -> (bool, Option<Item>, App) {
        let out = (false, None, app);
        match self {
            SearchItem::Search { results, text_list } => {
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

                let mut text_list = text_list.clone();
                text_list.area(chunks[0]);

                if let Some(result) = results {
                    if let Some(item) = result.iter().nth(text_list.selected) {
                        if !popup_focus {
                            frame.render_widget(ItemDisplay { item: item.clone() }, chunks[1]);
                        }
                    }
                }

                if selected {
                    text_list.selected_style(Style::default().fg(Color::LightRed));
                } else {
                    text_list.selected_style(Style::default().fg(Color::LightYellow));
                }

                frame.render_widget(text_list, chunks[0]);
            }
        }

        out
    }
}

pub struct Search;

impl PageTrait for Search {
    fn message() -> String {
        String::from("Loading search results...")
    }

    fn min() -> (u16, u16) {
        (45, 12)
    }

    fn default() -> Vec<Row> {
        vec![
            Row {
                items: vec![
                    RowItem {
                        item: Item::Global(GlobalItem::SearchBar),
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
                items: vec![RowItem {
                    item: Item::Search(SearchItem::Search {
                        results: None,
                        text_list: TextList::default(),
                    }),
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
