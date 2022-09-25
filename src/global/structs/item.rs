use crate::global::functions::{date_text, secs_display_string, viewcount_text};
use invidious::structs::hidden::{
    Playlist, PopularItem, SearchItem, SearchItemTransition, TrendingVideo,
};
use std::fmt::Display;

/// Items are things like a single video/playlist and channel
// they are displayed by the item info widget in `iteminfo.rs`
#[derive(Clone)]
pub enum Item {
    // minivideos are all videos that appeared without actually clicking into the video
    MiniVideo(MiniVideoItem),
    MiniPlaylist(MiniPlaylistItem),
    MiniChannel(MiniChannelItem),
    Unknown(SearchItemTransition),
}

/// info from a video from search, trending and popular page
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

#[derive(Clone)]
pub struct MiniPlaylistItem {
    pub title: String,
    pub id: String,
    pub channel: String,
    pub channel_id: String,
    pub video_count: u32,
    // (id, url)
    pub thumbnail: Option<(String, String)>,
}

#[derive(Clone)]
pub struct MiniChannelItem {
    pub name: String,
    pub id: String,
    pub thumbnail_url: String,
    pub sub_count: u32,
    pub sub_count_text: String,
    pub video_count: u32,
    pub description: String,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MiniVideo(video) => &video.title,
            Self::MiniPlaylist(playlist) => &playlist.title,
            Self::MiniChannel(channel) => &channel.name,
            Self::Unknown(_) => "Unknown item",
        })
    }
}

impl Item {
    // the file save path for thumbnails (which is by their id)
    pub fn thumbnail_id(&self) -> &str {
        match self {
            Self::MiniVideo(video) => &video.id,
            Self::MiniPlaylist(playlist) => match &playlist.thumbnail {
                Some((id, _)) => &id,
                None => "invalid",
            },
            Self::MiniChannel(channel) => &channel.id,
            Self::Unknown(_) => "invalid",
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

    pub fn from_mini_playlist(original: Playlist, image_index: usize) -> Self {
        Self::MiniPlaylist(MiniPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            thumbnail: if original.videos.len() == 0 {
                None
            } else {
                Some((
                    original.videos[0].id.clone(),
                    original.videos[0].thumbnails[image_index].url.clone(),
                ))
            },
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
            SearchItem::Playlist {
                title,
                id,
                author,
                author_id,
                author_url: _,
                video_count,
                videos,
            } => Self::MiniPlaylist(MiniPlaylistItem {
                title,
                id,
                channel: author,
                channel_id: author_id,
                video_count,
                thumbnail: if videos.len() == 0 {
                    None
                } else {
                    Some((
                        videos[0].id.clone(),
                        videos[0].thumbnails[image_index].url.clone(),
                    ))
                },
            }),
            SearchItem::Channel {
                name,
                id,
                url: _,
                thumbnails,
                subscribers,
                video_count,
                description,
                description_html: _,
            } => {
                Self::MiniChannel(MiniChannelItem {
                name,
                id,
                thumbnail_url: format!("https://{}", thumbnails[image_index].url),
                video_count,
                sub_count: subscribers,
                sub_count_text: viewcount_text(subscribers as u64),
                description,
            })},
            SearchItem::Unknown(searchitem_transitional) => Self::Unknown(searchitem_transitional),
        }
    }
}
