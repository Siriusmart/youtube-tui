use std::{collections::LinkedList, error::Error};

use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{app::App, config::Action},
    functions::download_all_thumbnails,
    structs::{
        FullChannel, Item, ListItem, MiniPlayList, MiniVideo, Page, Row, RowItem, WatchHistory,
    },
    traits::{ItemTrait, PageTrait},
    widgets::{horizontal_split::HorizontalSplit, item_display::ItemDisplay, text_list::TextList},
};

use super::{
    global::GlobalItem,
    item_info::{DisplayItem, ItemInfo},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelPage {
    Home,
    Playlists,
    Videos,
}

#[derive(Debug, Clone)]
pub enum ChannelItem {
    InfoDisplay(ChannelDisplayItem),
    SelectItems(ChannelPage),
}

#[derive(Debug, Clone)]
pub enum ChannelDisplayItem {
    Playlists(Vec<MiniPlayList>, TextList),
    Videos(Vec<MiniVideo>, TextList),
    Home(FullChannel),
    Unknown,
}

impl ItemTrait for ChannelItem {
    fn load_item(&self, app: &App, _: &mut WatchHistory) -> Result<Item, Box<dyn Error>> {
        let mut this = self.clone();

        if let Page::Channel(channel_page, id) = &app.page {
            if let ChannelItem::InfoDisplay(displayitem) = &mut this {
                match channel_page {
                    ChannelPage::Home => {
                        let channel = app.client.channel(id, None)?;
                        let mut linked_list = LinkedList::new();
                        linked_list.push_back((
                            channel.authorThumbnails[4].url.clone(),
                            channel.authorId.clone(),
                        ));
                        let _ = download_all_thumbnails(linked_list);

                        *displayitem = ChannelDisplayItem::Home(channel.into());
                    }
                    ChannelPage::Playlists => {
                        let mut textlist = TextList::default();
                        let playlists = app
                            .client
                            .channel_playlists(id, None)?
                            .playlists
                            .into_iter()
                            .map(|item| {
                                textlist.items.push(item.title.clone());
                                item.into()
                            })
                            .collect::<Vec<MiniPlayList>>();

                        let mut linked_list = LinkedList::new();
                        for item in playlists.iter() {
                            if let Some(thumbnail) = &item.thumbnail {
                                linked_list
                                    .push_back((thumbnail.clone(), item.playlist_id.clone()));
                            }
                        }
                        let _ = download_all_thumbnails(linked_list);

                        *displayitem = ChannelDisplayItem::Playlists(playlists, textlist);
                    }

                    ChannelPage::Videos => {
                        let mut textlist = TextList::default();
                        let videos = app
                            .client
                            .channel_videos(id, None)?
                            .videos
                            .into_iter()
                            .map(|item| {
                                textlist.items.push(item.title.clone());
                                item.into()
                            })
                            .collect::<Vec<MiniVideo>>();

                        let mut linked_list = LinkedList::new();
                        for item in videos.iter() {
                            linked_list
                                .push_back((item.video_thumbnail.clone(), item.video_id.clone()));
                        }
                        let _ = download_all_thumbnails(linked_list);

                        *displayitem = ChannelDisplayItem::Videos(videos, textlist);
                    }
                }
            }
        }

        Ok(Item::Channel(this))
    }

    fn render_item<B: Backend>(
        // &mut self,
        // frame: &mut Frame<B>,
        // rect: Rect,
        // selected: bool,
        // hover: bool,
        // popup_focus: bool,
        // page: &Page,
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: App,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        _: bool,
    ) -> (bool, App) {
        let mut out = (false, app);

        let mut style = Style::default().fg(if selected {
            Color::LightBlue
        } else if hover {
            Color::LightRed
        } else {
            Color::Reset
        });

        match self {
            ChannelItem::InfoDisplay(displayitem) => match displayitem {
                ChannelDisplayItem::Unknown => {
                    let block = Block::default()
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL)
                        .border_style(style);

                    frame.render_widget(block, rect);
                }
                ChannelDisplayItem::Home(channel) => {
                    let block = Block::default()
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL)
                        .border_style(style);

                    let inner = block.inner(rect);
                    frame.render_widget(block, rect);
                    frame.render_widget(
                        ItemDisplay {
                            item: ListItem::FullChannel(channel.clone()),
                        },
                        inner,
                    );
                }
                ChannelDisplayItem::Videos(videos, textlist) => {
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

                    match &mut textlist.area {
                        Some(area) if *area != chunks[0] => {
                            *area = chunks[0];
                            out.0 = true;
                        }
                        None => {
                            textlist.area = Some(chunks[0]);
                            out.0 = true;
                        }

                        _ => {}
                    };

                    textlist.area(chunks[0]);

                    let mut textlist = textlist.clone();

                    if selected {
                        textlist.selected_style(Style::default().fg(Color::LightRed));
                    } else {
                        textlist.selected_style(Style::default().fg(Color::LightYellow));
                    }

                    if let Some(item) = videos.iter().nth(textlist.selected) {
                        if !popup_focus {
                            frame.render_widget(
                                ItemDisplay {
                                    item: ListItem::MiniVideo(item.clone()),
                                },
                                chunks[1],
                            );
                        }
                    }

                    frame.render_widget(textlist, chunks[0]);
                }
                ChannelDisplayItem::Playlists(playlists, textlist) => {
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

                    textlist.area(chunks[0]);

                    let mut textlist = textlist.clone();

                    if selected {
                        textlist.selected_style(Style::default().fg(Color::LightRed));
                    } else {
                        textlist.selected_style(Style::default().fg(Color::LightYellow));
                    }

                    if let Some(item) = playlists.iter().nth(textlist.selected) {
                        if !popup_focus {
                            frame.render_widget(
                                ItemDisplay {
                                    item: ListItem::MiniPlayList(item.clone()),
                                },
                                chunks[1],
                            );
                        }
                    }

                    frame.render_widget(textlist, chunks[0]);
                }
            },
            ChannelItem::SelectItems(selected_page) => {
                if !hover {
                    if let Page::Channel(channel_page, _) = &out.1.page {
                        if *selected_page == *channel_page {
                            style = style.fg(Color::LightYellow);
                        }
                    }
                }

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(style);
                let paragraph = Paragraph::new(format!("{:?}", selected_page))
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, rect);
            }
        }
        out
    }

    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        let action = match app.config.keybindings.0.get(&key) {
            Some(action) => action,
            None => return (false, app),
        };

        match self {
            ChannelItem::InfoDisplay(displayitem) => match displayitem {
                ChannelDisplayItem::Videos(videos, textlist) => match action {
                    Action::Up => {
                        textlist.up();
                    }
                    Action::Down => {
                        textlist.down();
                    }
                    Action::FirstItem => textlist.first(),
                    Action::LastItem => textlist.last(),
                    Action::Select => {
                        let state = ItemInfo::default();
                        let mut history = app.history.clone();
                        history.push(app.into());

                        return (
                            false,
                            App {
                                history,
                                page: Page::ItemDisplay(DisplayItem::Video(
                                    videos[textlist.selected].video_id.clone(),
                                )),
                                selectable: App::selectable(&state),
                                state,
                                ..Default::default()
                            },
                        );
                    }
                    _ => {}
                },

                ChannelDisplayItem::Playlists(playlists, textlist) => match action {
                    Action::Up => {
                        textlist.up();
                    }
                    Action::Down => {
                        textlist.down();
                    }
                    Action::FirstItem => textlist.first(),
                    Action::LastItem => textlist.last(),
                    Action::Select => {
                        let state = ItemInfo::default();
                        let mut history = app.history.clone();
                        history.push(app.into());

                        return (
                            false,
                            App {
                                history,
                                page: Page::ItemDisplay(DisplayItem::PlayList(
                                    playlists[textlist.selected].playlist_id.clone(),
                                )),
                                selectable: App::selectable(&state),
                                state,
                                ..Default::default()
                            },
                        );
                    }
                    _ => {}
                },

                _ => unreachable!(),
            },
            _ => {
                unreachable!()
            }
        }

        (true, app)
    }

    fn select(&mut self, app: App) -> (App, bool) {
        let cloned = self.clone();
        match &cloned {
            ChannelItem::InfoDisplay(_) => (app, true),
            ChannelItem::SelectItems(tab) => {
                if let Page::Channel(channel_page, id) = &app.page.clone() {
                    if *tab == *channel_page {
                        return (app, false);
                    }

                    let state = Channel::default();
                    let mut history = app.history.clone();
                    history.push(app.into());

                    return (
                        App {
                            history,
                            page: Page::Channel(tab.clone(), id.clone()),
                            selectable: App::selectable(&state),
                            state,
                            ..Default::default()
                        },
                        false,
                    );
                } else {
                    (app, false)
                }
            }
        }
    }

    fn selectable(&self) -> bool {
        true
    }
}

pub struct Channel;

impl PageTrait for Channel {
    fn message() -> String {
        String::from("Loading channel info...")
    }

    fn min() -> (u16, u16) {
        (45, 15)
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
                items: vec![
                    RowItem {
                        item: Item::Channel(ChannelItem::SelectItems(ChannelPage::Home)),
                        constraint: Constraint::Length(15),
                    },
                    RowItem {
                        item: Item::Channel(ChannelItem::SelectItems(ChannelPage::Videos)),
                        constraint: Constraint::Length(15),
                    },
                    RowItem {
                        item: Item::Channel(ChannelItem::SelectItems(ChannelPage::Playlists)),
                        constraint: Constraint::Length(15),
                    },
                ],
                centered: true,
                height: Constraint::Length(3),
            },
            Row {
                items: vec![RowItem {
                    item: Item::Channel(ChannelItem::InfoDisplay(ChannelDisplayItem::Unknown)),
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
