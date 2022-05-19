use std::{
    collections::LinkedList,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    app::{app::App, pages::global::*},
    functions::{download_all_thumbnails, ItemType},
    structs::{FullVideo, Item, ListItem, Page, Row, RowItem},
    traits::{KeyInput, LoadItem, SelectItem},
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
}

#[derive(Debug, Clone)]
pub struct VideoItemInfo {
    video: FullVideo,
    list: TextList,
    mode: Mode,
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
    fn key_input(&mut self, key: KeyCode, mut app: App) -> (bool, App) {
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
                            let command = vec![
                                String::from("mpv"),
                                String::from("--no-terminal"),
                                match videoinfo.mode {
                                    Mode::Youtube => {
                                        format!("https://youtu.be/{}", videoinfo.video.video_id)
                                    }
                                    Mode::Invidious => {
                                        format!(
                                            "{}/embed/{}",
                                            app.client.server, videoinfo.video.video_id
                                        )
                                    }
                                },
                            ];

                            run_command(command, app.message.clone());
                        }
                        1 => {
                            let command = vec![
                                String::from("konsole"),
                                String::from("-e"),
                                String::from("mpv"),
                                String::from("--no-video"),
                                match videoinfo.mode {
                                    Mode::Youtube => {
                                        format!("https://youtu.be/{}", videoinfo.video.video_id)
                                    }
                                    Mode::Invidious => {
                                        format!(
                                            "{}/embed/{}",
                                            app.client.server, videoinfo.video.video_id
                                        )
                                    }
                                },
                            ];

                            run_command(command, app.message.clone());
                        }
                        2 => {
                            let command = vec![
                                String::from("konsole"),
                                String::from("-e"),
                                String::from("yt-dlp"),
                                match videoinfo.mode {
                                    Mode::Youtube => {
                                        format!("https://youtu.be/{}", videoinfo.video.video_id)
                                    }
                                    Mode::Invidious => {
                                        format!(
                                            "{}/embed/{}",
                                            app.client.server, videoinfo.video.video_id
                                        )
                                    }
                                },
                                String::from("-o"),
                                String::from("~/Downloads/%(title)s.%(ext)s"),
                            ];
                            run_command(command, app.message.clone());
                        }
                        3 => {
                            let command = vec![
                                String::from("konsole"),
                                String::from("-e"),
                                String::from("yt-dlp"),
                                match videoinfo.mode {
                                    Mode::Youtube => {
                                        format!("https://youtu.be/{}", videoinfo.video.video_id)
                                    }
                                    Mode::Invidious => {
                                        format!(
                                            "{}/embed/{}",
                                            app.client.server, videoinfo.video.video_id
                                        )
                                    }
                                },
                                String::from("-o"),
                                String::from("~/Downloads/%(title)s.%(ext)s"),
                                String::from("-x"),
                            ];
                            run_command(command, app.message.clone());
                        }
                        4 => {
                            let mut dir = home::home_dir().unwrap();

                            dir.push(".cache");
                            dir.push("youtube-tui");
                            dir.push("thumbnails");
                            dir.push("videos");
                            dir.push(format!("{}.png", videoinfo.video.video_id));
                            let command = vec![
                                String::from("mpv"),
                                String::from("--no-terminal"),
                                (*dir.as_path().to_string_lossy()).to_string(),
                            ];

                            run_command(command, app.message.clone());
                        }
                        5 => {
                            *app.message.lock().unwrap() = Some(String::from(
                                "Too lazy to implement that what do you expect?",
                            ));
                        }
                        6 => {
                            videoinfo.mode.toggle();
                        }

                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }

        (true, app)
    }
}

impl LoadItem for ItemInfoItem {
    fn load_item(&self, app: &App) -> Result<Box<Self>, Box<dyn std::error::Error>> {
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
                            ItemType::Video,
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
                        this = ItemInfoItem::Video(VideoItemInfo {
                            video,
                            list,
                            mode: Mode::Youtube,
                        });
                    }
                },

                _ => {}
            }
        }
        Ok(Box::new(this))
    }
}

impl ItemInfoItem {
    pub fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
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

                let item_display = ItemDisplay {
                    item: ListItem::FullVideo(videoinfo.video.clone()),
                };

                frame.render_widget(item_display, chunks[0]);

                frame.render_widget(list, chunks[1]);
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
}

pub struct ItemInfo;

impl ItemInfo {
    pub fn message() -> String {
        String::from("Loading item info...")
    }

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
