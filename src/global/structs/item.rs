use crate::global::functions::{date_text, secs_display_string, viewcount_text};
use invidious::structs::{
    hidden::{
        Playlist, PlaylistItem, PopularItem, SearchItem, SearchItemTransition, TrendingVideo,
    },
    universal::Playlist as FullPlaylist,
    video::Video,
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
    FullVideo(FullVideoItem),
    FullPlaylist(FullPlaylistItem),
    Unknown(SearchItemTransition),
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
    pub published: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct MiniPlaylistItem {
    pub title: String,
    pub id: String,
    pub channel: String,
    pub channel_id: String,
    pub video_count: u32,
    pub thumbnail_url: String,
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

#[derive(Clone)]
pub struct FullVideoItem {
    pub title: String,
    pub id: String,
    pub thumbnail_url: String,
    pub length: String,
    pub views: String,
    pub channel: String,
    pub channel_id: String,
    pub sub_count: String,
    pub published: String,
    pub description: String,
    pub likes: String,
    // pub dislikes: Option<String>, TODO
    pub genre: String,
}

#[derive(Clone)]
pub struct FullPlaylistItem {
    pub title: String,
    pub id: String,
    pub channel: String,
    pub channel_id: String,
    pub video_count: u32,
    pub description: String,
    pub views: String,
    pub thumbnail_url: String,
    pub videos: Vec<Item>,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::MiniVideo(video) => &video.title,
            Self::MiniPlaylist(playlist) => &playlist.title,
            Self::MiniChannel(channel) => &channel.name,
            Self::FullVideo(video) => &video.title,
            Self::FullPlaylist(playlist) => &playlist.title,
            Self::Unknown(_) => "Unknown item",
        })
    }
}

impl Item {
    // the file save path for thumbnails (which is by their id)
    pub fn thumbnail_id(&self) -> &str {
        match self {
            Self::MiniVideo(video) => &video.id,
            Self::MiniPlaylist(playlist) => &playlist.id,
            Self::MiniChannel(channel) => &channel.id,
            Self::FullVideo(video) => &video.id,
            Self::FullPlaylist(playlist) => &playlist.id,
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
            published: Some(format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            )),
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
            published: Some(format!(
                "{} [{}]",
                original.published_text,
                date_text(original.published)
            )),
            description: None,
        })
    }

    pub fn from_mini_playlist(original: Playlist) -> Self {
        Self::MiniPlaylist(MiniPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            thumbnail_url: original.thumbnail,
        })
    }

    pub fn from_search_item(original: SearchItem, image_index: usize) -> Self {
        match original {
            SearchItem::Video {
                title,
                id,
                author,
                author_id,
                length,
                thumbnails,
                description,
                views,
                published,
                published_text,
                ..
            } => Self::MiniVideo(MiniVideoItem {
                title,
                id,
                thumbnail_url: thumbnails[image_index].url.clone(),
                length: secs_display_string(length as u32),
                views: Some(viewcount_text(views)),
                channel: author,
                channel_id: author_id,
                published: Some(format!("{} [{}]", published_text, date_text(published))),
                description: Some(description),
            }),
            SearchItem::Playlist {
                title,
                id,
                author,
                author_id,
                video_count,
                thumbnail,
                ..
            } => Self::MiniPlaylist(MiniPlaylistItem {
                title,
                id,
                channel: author,
                channel_id: author_id,
                video_count,
                thumbnail_url: thumbnail,
            }),
            SearchItem::Channel {
                name,
                id,
                thumbnails,
                subscribers,
                video_count,
                description,
                ..
            } => Self::MiniChannel(MiniChannelItem {
                name,
                id,
                thumbnail_url: format!("https://{}", thumbnails[image_index].url),
                video_count,
                sub_count: subscribers,
                sub_count_text: viewcount_text(subscribers as u64),
                description,
            }),
            SearchItem::Unknown(searchitem_transitional) => Self::Unknown(searchitem_transitional),
        }
    }

    pub fn from_full_video(original: Video, image_index: usize) -> Self {
        Self::FullVideo(FullVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.clone(),
            length: secs_display_string(original.length as u32),
            views: viewcount_text(original.views),
            channel: original.author,
            channel_id: original.author_id,
            sub_count: original.sub_count_text,
            published: original.published_text,
            description: original.description,
            likes: viewcount_text(original.likes as u64),
            genre: original.genre,
        })
    }

    pub fn from_full_playlist(original: FullPlaylist, image_index: usize) -> Self {
        Self::FullPlaylist(FullPlaylistItem {
            title: original.title,
            id: original.id,
            channel: original.author,
            channel_id: original.author_id,
            video_count: original.video_count,
            description: original.description,
            views: viewcount_text(original.views),
            thumbnail_url: original.thumbnail,
            videos: original
                .videos
                .into_iter()
                .map(|video| Self::from_playlist_item(video, image_index))
                .collect(),
        })
    }

    pub fn from_playlist_item(original: PlaylistItem, image_index: usize) -> Self {
        Self::MiniVideo(MiniVideoItem {
            title: original.title,
            id: original.id,
            thumbnail_url: original.thumbnails[image_index].url.to_owned(),
            length: secs_display_string(original.length),
            views: None,
            channel: original.author,
            channel_id: original.author_id,
            published: None,
            description: None,
        })
    }
}
