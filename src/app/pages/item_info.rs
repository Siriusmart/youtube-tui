use std::{collections::LinkedList, error::Error};

use crate::{
    app::{
        app::App,
        config::{Action, PageCommand},
        pages::{channel::*, global::*},
    },
    functions::download_all_thumbnails,
    structs::{FullPlayList, FullVideo, Item, ListItem, Page, Row, RowItem, WatchHistory},
    traits::{ItemTrait, PageTrait},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    Frame,
};

use super::global::GlobalItem;

#[derive(Debug, Clone)]
pub enum ItemInfoItem {
    Unknown,
    Video(VideoItemInfo),
    PlayList(PlayListItemInfo),
}

#[derive(Debug, Clone)]
pub struct VideoItemInfo {
    video: FullVideo,
    list: TextList,
    mode: Mode,
    commands: Vec<PageCommand>,
}

#[derive(Debug, Clone)]
pub struct PlayListItemInfo {
    playlist: FullPlayList,
    video_view_list: TextList,
    playlist_view_list: TextList,
    display_view: PlayListDisplayView,
    mode: Mode,
    commands: Vec<PageCommand>,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayListDisplayView {
    VideoList,
    PlayListView,
}

impl PlayListDisplayView {
    pub fn toggle(self) -> Self {
        match self {
            PlayListDisplayView::VideoList => PlayListDisplayView::PlayListView,
            PlayListDisplayView::PlayListView => PlayListDisplayView::VideoList,
        }
    }
}

impl ItemTrait for ItemInfoItem {
    fn select(&mut self, app: App) -> (App, bool) {
        let selected = true;

        (app, selected)
    }

    fn selectable(&self) -> bool {
        true
    }

    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        let action = match app.config.keybindings.0.get(&key) {
            Some(action) => *action,
            None => return (false, app),
        };
        match self {
            ItemInfoItem::Video(videoinfo) => match action {
                Action::Up => videoinfo.list.up(),
                Action::Down => videoinfo.list.down(),
                Action::FirstItem => videoinfo.list.first(),
                Action::LastItem => videoinfo.list.last(),
                Action::Select => {
                    let command = &videoinfo.commands[videoinfo.list.selected];

                    match command.command.as_str() {
                        "{toggle_mode}" => videoinfo.mode.toggle(),
                        "{goto_channel}" => {
                            let state = Channel::default();
                            let mut history = app.history.clone();
                            history.push(app.into());

                            return (
                                false,
                                App {
                                    history,
                                    page: Page::Channel(
                                        ChannelPage::Home,
                                        videoinfo.video.channel_id.clone(),
                                    ),
                                    selectable: App::selectable(&state),
                                    state,
                                    ..Default::default()
                                },
                            );
                        }
                        _ => {
                            match app.config.commands.0.get(&command.command) {
                                Some(command) => {
                                    let mut env = app.config.main.env.clone();

                                    env.insert(
                                        String::from("embed_url"),
                                        format!(
                                            "{}/embed/{}",
                                            match videoinfo.mode {
                                                Mode::Invidious => app.client.server.as_str(),
                                                Mode::Youtube => "https://youtube.com",
                                            },
                                            videoinfo.video.video_id
                                        ),
                                    );

                                    match command.run_command(&env) {
                                        Some(e) => {
                                            *app.message.lock().unwrap() =
                                                Some(format!("Unknown variable `{}`", e))
                                        }
                                        None => {
                                            *app.message.lock().unwrap() =
                                                Some(command.message.clone())
                                        }
                                    }
                                }
                                None => {
                                    *app.message.lock().unwrap() =
                                        Some(format!("Unkown command `{}`", &command.command));
                                }
                            };
                        }
                    };
                }
                _ => {}
            },

            ItemInfoItem::PlayList(playlistinfo) => match playlistinfo.display_view {
                PlayListDisplayView::PlayListView => match action {
                    Action::Up => playlistinfo.playlist_view_list.up(),
                    Action::Down => playlistinfo.playlist_view_list.down(),
                    Action::FirstItem => playlistinfo.playlist_view_list.first(),
                    Action::LastItem => playlistinfo.playlist_view_list.last(),
                    Action::Select => {
                        let command =
                            &playlistinfo.commands[playlistinfo.playlist_view_list.selected];

                        match command.command.as_str() {
                            "{toggle_mode}" => playlistinfo.mode.toggle(),
                            "{goto_channel}" => {
                                let state = Channel::default();
                                let mut history = app.history.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::Channel(
                                            ChannelPage::Home,
                                            playlistinfo.playlist.playlist_id.clone(),
                                        ),
                                        selectable: App::selectable(&state),
                                        state,
                                        ..Default::default()
                                    },
                                );
                            }

                            "{switch_view}" => {
                                playlistinfo.display_view = playlistinfo.display_view.toggle()
                            }
                            _ => {
                                match app.config.commands.0.get(&command.command) {
                                    Some(command) => {
                                        let mut env = app.config.main.env.clone();

                                        env.insert(
                                            String::from("embed_url"),
                                            format!(
                                                "{}/playlist?list={}",
                                                match playlistinfo.mode {
                                                    Mode::Invidious => app.client.server.as_str(),
                                                    Mode::Youtube => "https://youtube.com",
                                                },
                                                playlistinfo.playlist.playlist_id
                                            ),
                                        );

                                        match command.run_command(&env) {
                                            Some(e) => {
                                                *app.message.lock().unwrap() =
                                                    Some(format!("Unknown variable `{}`", e))
                                            }
                                            None => {
                                                *app.message.lock().unwrap() =
                                                    Some(command.message.clone())
                                            }
                                        }
                                    }
                                    None => {
                                        *app.message.lock().unwrap() =
                                            Some(format!("Unkown command `{}`", &command.command));
                                    }
                                };
                            }
                        };
                    }

                    _ => {}
                },
                PlayListDisplayView::VideoList => match action {
                    Action::Up => playlistinfo.video_view_list.up(),
                    Action::Down => playlistinfo.video_view_list.down(),
                    Action::FirstItem => playlistinfo.video_view_list.first(),
                    Action::LastItem => playlistinfo.video_view_list.last(),
                    Action::Select => match playlistinfo.video_view_list.selected {
                        0 => {
                            playlistinfo.display_view = playlistinfo.display_view.toggle();
                        }

                        x => {
                            let state = ItemInfo::default();
                            let mut history = app.history.clone();
                            history.push(app.into());

                            return (
                                false,
                                App {
                                    history,
                                    page: Page::ItemDisplay(DisplayItem::Video(
                                        playlistinfo.playlist.videos[x - 1].video_id.clone(),
                                    )),
                                    selectable: App::selectable(&state),
                                    state,
                                    ..Default::default()
                                },
                            );
                        }
                    },

                    _ => {}
                },
            },
            _ => {}
        }

        (true, app)
    }

    fn load_item(
        &self,
        app: &App,
        watch_history: &mut WatchHistory,
    ) -> Result<Item, Box<dyn Error>> {
        let mut this = self.clone();

        if let ItemInfoItem::Unknown = this {
            match &app.page {
                Page::ItemDisplay(display_item) => match display_item {
                    DisplayItem::Video(id) => {
                        let mut list = TextList::default();
                        let video: FullVideo = app.client.video(id, None)?.into();
                        let _ = download_all_thumbnails(LinkedList::from([(
                            video.video_thumbnail.clone(),
                            video.video_id.clone(),
                        )]));
                        let commands = app
                            .config
                            .page_commands
                            .0
                            .get("item_info:video")
                            .unwrap_or(&Vec::new())
                            .clone();
                        list.items(commands.iter().map(|item| item.label.to_string()).collect());

                        let iteminfo = VideoItemInfo {
                            video,
                            list,
                            mode: Mode::Youtube,
                            commands,
                        };

                        watch_history.push(
                            iteminfo.video.video_id.clone(),
                            ListItem::FullVideo(iteminfo.video.clone()),
                            &app.config,
                        );

                        this = ItemInfoItem::Video(iteminfo);
                    }

                    DisplayItem::PlayList(id) => {
                        let mut videos_text_list = TextList::default();
                        let mut playlist_text_list = TextList::default();
                        let playlist: FullPlayList = app.client.playlist(id, None)?.into();
                        let _ = download_all_thumbnails(
                            playlist
                                .videos
                                .iter()
                                .map(|v| (v.video_thumbnail.clone(), v.video_id.clone()))
                                .collect(),
                        );

                        videos_text_list.items = vec![String::from("Swtich view")];
                        videos_text_list
                            .items
                            .extend(playlist.videos.iter().map(|v| v.title.clone()));

                        let commands = app
                            .config
                            .page_commands
                            .0
                            .get("item_info:playlist")
                            .unwrap_or(&Vec::new())
                            .clone();
                        playlist_text_list
                            .items(commands.iter().map(|item| item.label.to_string()).collect());

                        let iteminfo = PlayListItemInfo {
                            playlist: playlist,
                            video_view_list: videos_text_list,
                            playlist_view_list: playlist_text_list,
                            mode: Mode::Youtube,
                            display_view: PlayListDisplayView::PlayListView,
                            commands,
                        };

                        watch_history.push(
                            iteminfo.playlist.playlist_id.clone(),
                            ListItem::FullPlayList(iteminfo.playlist.clone()),
                            &app.config,
                        );

                        this = ItemInfoItem::PlayList(iteminfo);
                    }
                },

                _ => {}
            }
        }
        Ok(Item::ItemInfo(this))
    }

    fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: App,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        popup_render: bool,
    ) -> (bool, App) {
        let out = (false, app);

        if popup_render {
            return out;
        }

        match self {
            ItemInfoItem::Video(videoinfo) => {
                // list.items[videoinfo.list.items.len() - 1] = format!("Mode: {}", videoinfo.mode);

                let split = HorizontalSplit::default()
                    .percentages(vec![40, 60])
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));

                let chunks = split.inner(rect);

                videoinfo.list.area(chunks[1]);

                let mut list = videoinfo.list.clone();

                list.items.iter_mut().for_each(|item| match item.as_str() {
                    "{mode}" => *item = format!("Mode: {}", videoinfo.mode),
                    _ => {}
                });
                if selected {
                    list.selected_style(Style::default().fg(Color::LightRed));
                } else {
                    list.selected_style(Style::default().fg(Color::LightYellow));
                }

                frame.render_widget(split, rect);

                if !popup_focus {
                    let item_display = ItemDisplay {
                        item: ListItem::FullVideo(videoinfo.video.clone()),
                    };

                    frame.render_widget(item_display, chunks[0]);
                }

                frame.render_widget(list, chunks[1]);
            }

            ItemInfoItem::PlayList(playlistinfo) => {
                let split = HorizontalSplit::default()
                    .percentages(vec![40, 60])
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));

                let chunks = split.inner(rect);

                frame.render_widget(split, rect);

                if !popup_focus {
                    let item_display = ItemDisplay {
                        item: ListItem::FullPlayList(playlistinfo.playlist.clone()),
                    };

                    frame.render_widget(item_display, chunks[0]);
                }

                match playlistinfo.display_view {
                    PlayListDisplayView::VideoList => {
                        playlistinfo.video_view_list.area(chunks[1]);
                        let mut list = playlistinfo.video_view_list.clone();
                        list.area(chunks[1]);
                        if selected {
                            list.selected_style(Style::default().fg(Color::LightRed));
                        } else {
                            list.selected_style(Style::default().fg(Color::LightYellow));
                        }

                        frame.render_widget(list, chunks[1]);
                    }
                    PlayListDisplayView::PlayListView => {
                        playlistinfo.playlist_view_list.area(chunks[1]);
                        let mut list = playlistinfo.playlist_view_list.clone();

                        list.items.iter_mut().for_each(|item| match item.as_str() {
                            "{mode}" => *item = format!("Mode: {}", playlistinfo.mode),
                            _ => {}
                        });

                        list.area(chunks[1]);

                        if selected {
                            list.selected_style(Style::default().fg(Color::LightRed));
                        } else {
                            list.selected_style(Style::default().fg(Color::LightYellow));
                        }

                        frame.render_widget(list, chunks[1]);
                    }
                }
            }

            ItemInfoItem::Unknown => {
                let split = HorizontalSplit::default()
                    .percentages(vec![40, 60])
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));

                frame.render_widget(split, rect);
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Video(String),
    PlayList(String),
}

pub struct ItemInfo;

impl PageTrait for ItemInfo {
    fn message() -> String {
        String::from("Loading item info...")
    }

    fn min() -> (u16, u16) {
        (21, 12)
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
                    item: Item::ItemInfo(ItemInfoItem::Unknown),
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

impl ItemInfoItem {}
