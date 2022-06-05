use invidious::structs::hidden::SearchItem;
use serde::{Deserialize, Serialize};
use thousands::Separable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniChannel {
    pub author: String,
    pub author_url: String,
    pub author_id: String,
    pub thumbnail: String,
    pub description: String,
    pub sub_count: String,
    pub video_count: String,
}

impl From<SearchItem> for MiniChannel {
    fn from(original: SearchItem) -> Self {
        match original {
            SearchItem::Channel {
                author,
                authorId,
                authorUrl,
                authorThumbnails,
                subCount,
                videoCount,
                description,
                descriptionHtml: _,
            } => Self {
                author,
                author_id: authorId,
                author_url: authorUrl,
                thumbnail: authorThumbnails[4].url.clone(),
                description,
                sub_count: subCount.separate_with_commas(),
                video_count: videoCount.to_string(),
            },

            _ => unreachable!(),
        }
    }
}
