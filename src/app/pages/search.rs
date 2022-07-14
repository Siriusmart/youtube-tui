use std::{collections::LinkedList, error::Error};

use crossterm::event::KeyEvent;
use invidious::structs::hidden::SearchItem as InvidiousSearchItem;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    Frame,
};

use crate::{
    app::{
        app::{App, AppNoState},
        config::{
            Action, ConstraintTransitional, ItemTransitional, LayoutConfig, RowItemTransitional,
            RowTransitional, SearchItemTransitional,
        },
    },
    functions::download_all_thumbnails,
    structs::{Item, ListItem, MiniChannel, MiniPlayList, MiniVideo, Page, WatchHistory},
    traits::{ItemTrait, PageTrait},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};

use super::{
    channel::ChannelPage, global::GlobalItem, item_info::DisplayItem,
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

    fn key_input(&mut self, key: KeyEvent, app: App) -> (bool, App) {
        let action = match app.config.keybindings.0.get(&key) {
            Some(action) => action,
            None => return (false, app),
        };
        match self {
            SearchItem::Search { results, text_list } => match action {
                Action::Up => text_list.up(),
                Action::Down => text_list.down(),
                Action::FirstItem => text_list.first(),
                Action::LastItem => text_list.last(),
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

                                let state = app.config.layouts.search.clone().into();
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
                                let state = app.config.layouts.item_info.clone().into();
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
                                let state = app.config.layouts.item_info.clone().into();
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
                                let state = app.config.layouts.channel.clone().into();
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

                if items.len() != minimum_length {
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

                if app.config.main.display_thumbnails {
                    let _ = download_all_thumbnails(thumbnails);
                }

                *results = Some(items);
            }
        }

        Ok(Item::Search(this))
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

                text_list.area(chunks[0]);

                let mut text_list = text_list.clone();

                if let Some(result) = results {
                    if let Some(item) = result.iter().nth(text_list.selected) {
                        if !popup_render {
                            frame.render_widget(ItemDisplay { item: item.clone(), render_image: !popup_focus }, chunks[1]);
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
                    items: vec![RowItemTransitional {
                        item: ItemTransitional::Search(SearchItemTransitional::Search),
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
            min: (45, 12),
            message: String::from("Loading search results..."),
            def_selected: Some((0,1)),
        }
    }
}
