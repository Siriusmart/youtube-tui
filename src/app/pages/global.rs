use crossterm::event::KeyCode;
use invidious::structs::{hidden::{TrendingVideo, PopularItem}, video::Video};

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
    fn key_input(&mut self, key: KeyCode, app: App) -> App {
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

        app
    }
}

#[derive(Debug, Clone)]
pub enum ListItem {
    Video(MiniVideo),
    PageTurner(bool), // true: +1 | false: -1
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
            video_thumbnail: original.videoThumbnails[0].url.clone(),
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
            video_thumbnail: original.videoThumbnails[0].url.clone(),
            length: hrtime::from_sec(original.lengthSeconds as u64),
            view_count: original.viewCount,
            author: original.author,
            author_url: original.authorUrl,
            published: original.publishedText,
            description: String::new(),
        }
    }
}