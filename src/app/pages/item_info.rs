use crate::{
    app::app::{App, Item, Page, Row, RowItem},
    traits::{KeyInput, LoadItem, SelectItem},
    widgets::{horizontal_split::HorizontalSplit, text_list::TextList},
};
use crossterm::event::KeyCode;
use invidious::structs::video::Video;
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
    Video(Option<VideoItemInfo>),
}

#[derive(Debug, Clone)]
pub struct VideoItemInfo {
    video: Video,
    list: TextList,
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
            ItemInfoItem::Video(Some(video)) => match key {
                KeyCode::Up => video.list.up(),
                KeyCode::Down => video.list.down(),
                KeyCode::PageUp => video.list.selected = 0,
                KeyCode::PageDown => video.list.selected = video.list.items.len() - 1,
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

        if let ItemInfoItem::Unknown = self {
            match &app.page {
                Page::ItemDisplay(display_item) => match display_item {
                    DisplayItem::Video(id) => {
                        if let ItemInfoItem::Video(videoinfo) = &mut this {
                            let mut list = TextList::default();
                            let video = app.client.video(id, None)?;
                            list.items(vec![
                                String::from("Watch from YouTube (Recommeneded)"),
                                String::from("Watch from Invidious"),
                                String::from("Play audio only from YouTube (Recommended)"),
                                String::from("Play audio only from Invidious"),
                                String::from("Download from YouTube (Recommended)"),
                                String::from("Download from Invidious"),
                                String::from("Download audio only from YouTube (Recommended)"),
                                String::from("Download audio only from Invidious"),
                                String::from("Visit channel"),
                                String::from("View thumbnail"),
                            ]);
                            *videoinfo = Some(VideoItemInfo { video, list });
                        }
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
            }

            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Video(String),
}

pub struct ItemInfo;

impl ItemInfo {
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
                    item: Item::ItemInfo(ItemInfoItem::Video(None)),
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
