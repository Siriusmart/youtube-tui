use std::{
    collections::LinkedList,
    error::Error,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    app::{app::App, config::EnvVar, pages::global::*},
    functions::download_all_thumbnails,
    structs::{FullPlayList, FullVideo, Item, ListItem, Page, Row, RowItem, WatchHistory},
    traits::{KeyInput, SelectItem},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    Frame,
};

use super::{
    channel::{Channel, ChannelPage},
    global::GlobalItem,
};

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
}

#[derive(Debug, Clone)]
pub struct PlayListItemInfo {
    playlist: FullPlayList,
    video_view_list: TextList,
    playlist_view_list: TextList,
    display_view: PlayListDisplayView,
    mode: Mode,
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

impl SelectItem for ItemInfoItem {
    fn select(&mut self, app: App) -> (App, bool) {
        let selected = true;

        (app, selected)
    }

    fn selectable(&self) -> bool {
        true
    }
}

impl KeyInput for ItemInfoItem {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        match self {
            ItemInfoItem::Video(videoinfo) => match key {
                KeyCode::Up => videoinfo.list.up(),
                KeyCode::Down => videoinfo.list.down(),
                KeyCode::PageUp => videoinfo.list.selected = 0,
                KeyCode::PageDown => videoinfo.list.selected = videoinfo.list.items.len() - 1,
                KeyCode::Enter => {
                    if videoinfo.list.selected < 5 {
                        *app.message.lock().unwrap() = Some(String::from("Launched application"));
                    }

                    match videoinfo.list.selected {
                        0 => {
                            let command =
                                match app.config.commands.video_player.clone().as_command_vec(
                                    EnvVar {
                                        url: Some(match videoinfo.mode {
                                            Mode::Youtube => {
                                                format!(
                                                    "https://youtu.be/{}",
                                                    videoinfo.video.video_id
                                                )
                                            }
                                            Mode::Invidious => {
                                                format!(
                                                    "{}/embed/{}",
                                                    app.client.server, videoinfo.video.video_id
                                                )
                                            }
                                        }),
                                        ..Default::default()
                                    },
                                    &app.config,
                                ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                            run_command(command, app.message.clone());
                        }
                        1 => {
                            let command =
                                match app.config.commands.audio_player.clone().as_command_vec(
                                    EnvVar {
                                        url: Some(match videoinfo.mode {
                                            Mode::Youtube => {
                                                format!(
                                                    "https://youtu.be/{}",
                                                    videoinfo.video.video_id
                                                )
                                            }
                                            Mode::Invidious => {
                                                format!(
                                                    "{}/embed/{}",
                                                    app.client.server, videoinfo.video.video_id
                                                )
                                            }
                                        }),
                                        ..Default::default()
                                    },
                                    &app.config,
                                ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                            run_command(command, app.message.clone());
                        }
                        2 => {
                            let command =
                                match app.config.commands.video_downloader.clone().as_command_vec(
                                    EnvVar {
                                        url: Some(match videoinfo.mode {
                                            Mode::Youtube => {
                                                format!(
                                                    "https://youtu.be/{}",
                                                    videoinfo.video.video_id
                                                )
                                            }
                                            Mode::Invidious => {
                                                format!(
                                                    "{}/embed/{}",
                                                    app.client.server, videoinfo.video.video_id
                                                )
                                            }
                                        }),
                                        ..Default::default()
                                    },
                                    &app.config,
                                ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                            run_command(command, app.message.clone());
                        }
                        3 => {
                            let command =
                                match app.config.commands.audio_downloader.clone().as_command_vec(
                                    EnvVar {
                                        url: Some(match videoinfo.mode {
                                            Mode::Youtube => {
                                                format!(
                                                    "https://youtu.be/{}",
                                                    videoinfo.video.video_id
                                                )
                                            }
                                            Mode::Invidious => {
                                                format!(
                                                    "{}/embed/{}",
                                                    app.client.server, videoinfo.video.video_id
                                                )
                                            }
                                        }),
                                        ..Default::default()
                                    },
                                    &app.config,
                                ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                            run_command(command, app.message.clone());
                        }
                        4 => {
                            let mut dir = home::home_dir().unwrap();

                            dir.push(".cache");
                            dir.push("youtube-tui");
                            dir.push("thumbnails");
                            dir.push(format!("{}.png", videoinfo.video.video_id));

                            let command =
                                match app.config.commands.image_viewer.clone().as_command_vec(
                                    EnvVar {
                                        url: Some((*dir.as_path().to_string_lossy()).to_string()),
                                        ..Default::default()
                                    },
                                    &app.config,
                                ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                            run_command(command, app.message.clone());
                        }
                        5 => {
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
                        6 => {
                            videoinfo.mode.toggle();
                        }

                        _ => {}
                    }
                }
                _ => {}
            },

            ItemInfoItem::PlayList(playlistinfo) => match playlistinfo.display_view {
                PlayListDisplayView::PlayListView => match key {
                    KeyCode::Up => playlistinfo.playlist_view_list.up(),
                    KeyCode::Down => playlistinfo.playlist_view_list.down(),
                    KeyCode::PageUp => playlistinfo.playlist_view_list.selected = 0,
                    KeyCode::PageDown => {
                        playlistinfo.playlist_view_list.selected =
                            playlistinfo.playlist_view_list.items.len() - 1
                    }
                    KeyCode::Enter => {
                        if playlistinfo.playlist_view_list.selected > 0
                            && playlistinfo.playlist_view_list.selected < 7
                        {
                            *app.message.lock().unwrap() =
                                Some(String::from("Launched application"));
                        }

                        match playlistinfo.playlist_view_list.selected {
                            0 => {
                                playlistinfo.display_view = playlistinfo.display_view.toggle();
                            }

                            1 => {
                                let command = match app
                                    .config
                                    .commands
                                    .playlist_video_all
                                    .clone()
                                    .as_command_vec(
                                        EnvVar {
                                            url: Some(match playlistinfo.mode {
                                                Mode::Youtube => {
                                                    format!(
                                                        "https://www.youtube.com/playlist?list={}",
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                                Mode::Invidious => {
                                                    format!(
                                                        "{}/playlist?list={}",
                                                        app.client.server,
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                            }),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                                run_command(command, app.message.clone());
                            }

                            2 => {
                                let command = match app
                                    .config
                                    .commands
                                    .playlist_audio_all
                                    .clone()
                                    .as_command_vec(
                                        EnvVar {
                                            url: Some(match playlistinfo.mode {
                                                Mode::Youtube => {
                                                    format!(
                                                        "https://www.youtube.com/playlist?list={}",
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                                Mode::Invidious => {
                                                    format!(
                                                        "{}/playlist?list={}",
                                                        app.client.server,
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                            }),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                                run_command(command, app.message.clone());
                            }

                            3 => {
                                let command = match app
                                    .config
                                    .commands
                                    .playlist_shuffle_audio_all
                                    .clone()
                                    .as_command_vec(
                                        EnvVar {
                                            url: Some(match playlistinfo.mode {
                                                Mode::Youtube => {
                                                    format!(
                                                        "https://www.youtube.com/playlist?list={}",
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                                Mode::Invidious => {
                                                    format!(
                                                        "{}/playlist?list={}",
                                                        app.client.server,
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                            }),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                                run_command(command, app.message.clone());
                            }

                            4 => {
                                let command = match app
                                    .config
                                    .commands
                                    .download_all_video
                                    .clone()
                                    .as_command_vec(
                                        EnvVar {
                                            url: Some(match playlistinfo.mode {
                                                Mode::Youtube => {
                                                    format!(
                                                        "https://www.youtube.com/playlist?list={}",
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                                Mode::Invidious => {
                                                    format!(
                                                        "{}/playlist?list={}",
                                                        app.client.server,
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                            }),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                                run_command(command, app.message.clone());
                            }

                            5 => {
                                let command = match app
                                    .config
                                    .commands
                                    .download_all_audio
                                    .clone()
                                    .as_command_vec(
                                        EnvVar {
                                            url: Some(match playlistinfo.mode {
                                                Mode::Youtube => {
                                                    format!(
                                                        "https://www.youtube.com/playlist?list={}",
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                                Mode::Invidious => {
                                                    format!(
                                                        "{}/playlist?list={}",
                                                        app.client.server,
                                                        playlistinfo.playlist.playlist_id
                                                    )
                                                }
                                            }),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                    Ok(command) => command,
                                    Err(_) => {
                                        *app.message.lock().unwrap() =
                                            Some(String::from("Error in parsing launch command"));
                                        return (false, app);
                                    }
                                };

                                run_command(command, app.message.clone());
                            }

                            6 => {
                                let mut dir = home::home_dir().unwrap();

                                dir.push(".cache");
                                dir.push("youtube-tui");
                                dir.push("thumbnails");
                                dir.push(format!("{}.png", playlistinfo.playlist.playlist_id));

                                let command =
                                    match app.config.commands.image_viewer.clone().as_command_vec(
                                        EnvVar {
                                            url: Some(
                                                (*dir.as_path().to_string_lossy()).to_string(),
                                            ),
                                            ..Default::default()
                                        },
                                        &app.config,
                                    ) {
                                        Ok(command) => command,
                                        Err(_) => {
                                            *app.message.lock().unwrap() = Some(String::from(
                                                "Error in parsing launch command",
                                            ));
                                            return (false, app);
                                        }
                                    };

                                run_command(command, app.message.clone());
                            }

                            7 => {
                                let state = Channel::default();
                                let mut history = app.history.clone();
                                history.push(app.into());

                                return (
                                    false,
                                    App {
                                        history,
                                        page: Page::Channel(
                                            ChannelPage::Home,
                                            playlistinfo.playlist.author_id.clone(),
                                        ),
                                        selectable: App::selectable(&state),
                                        state,
                                        load: true,
                                        ..Default::default()
                                    },
                                );
                            }

                            8 => {
                                playlistinfo.mode.toggle();
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                },
                PlayListDisplayView::VideoList => match key {
                    KeyCode::Up => playlistinfo.video_view_list.up(),
                    KeyCode::Down => playlistinfo.video_view_list.down(),
                    KeyCode::PageUp => playlistinfo.video_view_list.selected = 0,
                    KeyCode::PageDown => {
                        playlistinfo.video_view_list.selected =
                            playlistinfo.video_view_list.items.len() - 1
                    }
                    KeyCode::Enter => match playlistinfo.video_view_list.selected {
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
}

impl ItemInfoItem {
    pub fn load_item(
        &self,
        app: &App,
        watch_history: &mut WatchHistory,
    ) -> Result<Self, Box<dyn Error>> {
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
                        list.items(vec![
                            String::from("Watch video"),
                            String::from("Play audio"),
                            String::from("Download video"),
                            String::from("Download audio"),
                            String::from("View thumbnail"),
                            String::from("Visit channel"),
                            String::from("Mode placeholder"),
                        ]);

                        let iteminfo = VideoItemInfo {
                            video,
                            list,
                            mode: Mode::Youtube,
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

                        playlist_text_list.items(vec![
                            String::from("Switch view"),
                            String::from("Watch all"),
                            String::from("Play all audio"),
                            String::from("Shuffle play all audio"),
                            String::from("Download all video"),
                            String::from("Download all audio"),
                            String::from("View thumbnail"),
                            String::from("Visit channel"),
                            String::from("Mode placeholder"),
                        ]);

                        let iteminfo = PlayListItemInfo {
                            playlist: playlist,
                            video_view_list: videos_text_list,
                            playlist_view_list: playlist_text_list,
                            mode: Mode::Youtube,
                            display_view: PlayListDisplayView::PlayListView,
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
        Ok(this)
    }

    pub fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        popup_focus: bool,
    ) {
        match self {
            ItemInfoItem::Video(videoinfo) => {
                let index = videoinfo.list.items.len() - 1;
                videoinfo.list.items[index] = format!("Mode: {}", videoinfo.mode);
                drop(index);

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
                let mut list = videoinfo.list.clone();

                list.area(chunks[1]);
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
                        let index = playlistinfo.playlist_view_list.items.len() - 1;
                        playlistinfo.playlist_view_list.items[index] =
                            format!("Mode: {}", playlistinfo.mode);
                        drop(index);

                        let mut list = playlistinfo.playlist_view_list.clone();
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
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Video(String),
    PlayList(String),
}

pub struct ItemInfo;

impl ItemInfo {
    pub fn message() -> String {
        String::from("Loading item info...")
    }

    pub fn min() -> (u16, u16) {
        (21, 12)
    }

    pub fn default() -> Vec<Row> {
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

fn run_command(args: Vec<String>, message: Arc<Mutex<Option<String>>>) {
    thread::spawn(move || {
        let mut args = args.into_iter();
        let mut command = Command::new(args.next().unwrap());
        for arg in args {
            command.arg(arg);
        }

        let res = command.spawn();
        if let Err(e) = res {
            *message.lock().unwrap() = Some(
                e.to_string()
                    .lines()
                    .next()
                    .unwrap_or("An error occured")
                    .to_string(),
            );
        }
    });
}
