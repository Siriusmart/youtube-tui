use std::fmt::Display;

use super::functions::{date_text, secs_display_string, viewcount_text};
use invidious::structs::hidden::{PopularItem, SearchItem, TrendingVideo};

// Items are things like a single video/playlist and channel
// they are displayed by the item info widget in `iteminfo.rs`
#[derive(Clone)]
pub enum Item {
    // minivideos are all videos that appeared without actually clicking into the video
    MiniVideo(MiniVideoItem),
}

#[derive(Clone)]
pub struct MiniVideoItem {
    pub title: String,
    pub id: String,
    pub thumbnail_url: String,
    pub length: String,
    pub views: Option<String>,
    pub channel: String,
    pub channel_id: String,
    pub published: String,
    pub description: Option<String>,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MiniVideo(video) => &video.title,
        })
    }
}

impl Item {
    // the file save path for thumbnails (which is by their id)
    pub fn thumbnail_id(&self) -> &str {
        match self {
            Self::MiniVideo(video) => &video.id,
        }
    }

    // cannot impl From<T> because it also need `image_index`
    pub fn from_trending_video(original: TrendingVideo, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbmails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: Some(viewcount_text(original.views)),
            channel: original.author,
            channel_id: original.author_id,
            published: format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            ),
            description: Some(original.description),
        })
    }

    pub fn from_popular_item(original: PopularItem, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length),
            views: Some(viewcount_text(original.views)),
            channel: original.author,
            channel_id: original.author_id,
            published: format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            ),
            description: None,
        })
    }

    pub fn from_search_item(original: SearchItem, image_index: usize) -> Self {
        match original {
            SearchItem::Video {
                title,
                id,
                author,
                author_id,
                author_url: _,
                length,
                thumbnails,
                description,
                description_html: _,
                views,
                published,
                published_text,
                live: _,
                paid: _,
                premium: _,
            } => Self::MiniVideo(MiniVideoItem {
                title,
                id,
                thumbnail_url: thumbnails[image_index].url.clone(),
                length: secs_display_string(length as u32),
                views: Some(viewcount_text(views)),
                channel: author,
                channel_id: author_id,
                published: format!("{} [{}]", published_text, date_text(published)),
                description: Some(description),
            }),
            _ => todo!(),
        }
    }
}
