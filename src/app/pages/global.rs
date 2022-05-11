use crossterm::event::KeyCode;
use invidious::structs::hidden::{PopularItem, TrendingVideo};
use tui::{style::{Color, Style}, widgets::{Borders, Paragraph, BorderType, Block}, layout::{Alignment, Rect}, Frame, backend::Backend};

use crate::{
    app::app::App,
    traits::{KeyInput, SelectItem},
};

#[derive(Debug, Clone)]
pub enum GlobalItem {
    SearchBar(String),
    MessageBar,
}

impl SelectItem for GlobalItem {
    fn select(&mut self, app: App) -> (App, bool) {
        let selected = match self {
            GlobalItem::SearchBar(_) => true,
            _ => false,
        };

        (app, selected)
    }

    fn selectable(&self) -> bool {
        match self {
            GlobalItem::SearchBar(_) => true,
            _ => false,
        }
    }
}

impl KeyInput for GlobalItem {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) {
        match self {
            GlobalItem::SearchBar(search_bar) => match key {
                KeyCode::Char(c) => {
                    search_bar.push(c);
                }
                KeyCode::Backspace => {
                    search_bar.pop();
                }
                KeyCode::Enter => {}
                _ => {}
            },
            _ => {}
        }

        (true, app)
    }
}

#[derive(Debug, Clone)]
pub enum ListItem {
    Video(MiniVideo),
    PageTurner(bool), // true: +1 | false: -1
}

impl ListItem {
    pub fn id(&self) -> String {
        match self {
            ListItem::Video(video) => video.video_id.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiniVideo {
    pub title: String,
    pub video_id: String,
    pub video_thumbnail: String,
    pub length: String,
    pub view_count: u64,
    pub author: String,
    pub author_url: String,
    pub published: String,
    pub description: String,
}

impl From<TrendingVideo> for MiniVideo {
    fn from(original: TrendingVideo) -> Self {
        MiniVideo {
            title: original.title,
            video_id: original.videoId,
            video_thumbnail: original.videoThumbnails[4].url.clone(),
            length: hrtime::from_sec(original.lengthSeconds as u64),
            view_count: original.viewCount,
            author: original.author,
            author_url: original.authorUrl,
            published: original.publishedText,
            description: original.description,
        }
    }
}

impl From<PopularItem> for MiniVideo {
    fn from(original: PopularItem) -> Self {
        MiniVideo {
            title: original.title,
            video_id: original.videoId,
            video_thumbnail: original.videoThumbnails[5].url.clone(),
            length: hrtime::from_sec(original.lengthSeconds as u64),
            view_count: original.viewCount,
            author: original.author,
            author_url: original.authorUrl,
            published: original.publishedText,
            description: String::new(),
        }
    }
}

impl GlobalItem {
    pub fn render_item<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        message: &Option<String>,
    ) {
        match self {
            GlobalItem::SearchBar(search) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(tui::style::Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }))
                    .title("Search YouTube")
                    .title_alignment(Alignment::Center);
                let paragraph = Paragraph::new(search.clone()).block(block);
                frame.render_widget(paragraph, rect);
            }
            GlobalItem::MessageBar => {
                // let color = Color::LightYellow;
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if message.is_some() {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));
                let paragraph =
                    Paragraph::new(message.clone().unwrap_or(String::from("All good :)")))
                        .block(block);
                frame.render_widget(paragraph, rect);
            }
        }
    }
}
