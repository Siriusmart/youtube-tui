use invidious::structs::hidden::{Playlist, SearchItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MiniPlayList {
    pub title: String,
    pub playlist_id: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_count: usize,
    pub thumbnail: Option<String>,
}

impl From<SearchItem> for MiniPlayList {
    fn from(original: SearchItem) -> Self {
        match original {
            SearchItem::Playlist {
                title,
                playlistId,
                author,
                authorId,
                authorUrl,
                videoCount,
                videos,
            } => MiniPlayList {
                title,
                playlist_id: playlistId,
                author,
                author_id: authorId,
                author_url: authorUrl,
                video_count: videoCount as usize,
                thumbnail: if videos.len() == 0 {
                    None
                } else {
                    Some(videos[0].videoThumbnails[4].url.clone())
                },
            },

            _ => {
                unreachable!()
            }
        }
    }
}

impl From<Playlist> for MiniPlayList {
    fn from(original: Playlist) -> Self {
        Self {
            title: original.title,
            playlist_id: original.playlistId,
            author: original.author,
            author_id: original.authorId,
            author_url: original.authorUrl,
            video_count: original.videoCount as usize,
            thumbnail: if original.videos.len() == 0 {
                None
            } else {
                Some(original.videos[0].videoThumbnails[4].url.clone())
            },
        }
    }
}
