use invidious::structs::hidden::SearchItem;
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
