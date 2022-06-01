use chrono::{DateTime, NaiveDateTime, Utc};
use invidious::structs::universal::Playlist;
use serde::{Deserialize, Serialize};

use super::MiniVideo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullPlayList {
    pub title: String,
    pub playlist_id: String,
    pub video_count: usize,
    pub views: usize,
    pub updated: String,
    pub thumbnail: Option<String>,
    pub description: String,
    pub author: String,
    pub author_id: String,
    pub videos: Vec<MiniVideo>,
}

impl From<Playlist> for FullPlayList {
    fn from(original: Playlist) -> Self {
        Self {
            title: original.title,
            playlist_id: original.playlistId,
            video_count: original.videoCount as usize,
            views: original.viewCount as usize,
            updated: {
                let datetime: DateTime<Utc> = DateTime::from_utc(
                    NaiveDateTime::from_timestamp(original.updated as i64, 0),
                    Utc,
                );
                format!(" [{}]", datetime.format("%Y/%m/%d"))
            },
            thumbnail: if original.videos.len() == 0 {
                None
            } else {
                Some(original.videos[0].videoThumbnails[4].url.clone())
            },
            description: original.description,
            author: original.author,
            author_id: original.authorId,
            videos: original.videos.into_iter().map(MiniVideo::from).collect(),
        }
    }
}
