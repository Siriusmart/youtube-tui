use invidious::structs::channel::Channel;
use serde::{Deserialize, Serialize};
use thousands::Separable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullChannel {
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub thumbnail: String,
    pub description: String,
    pub sub_count: String,
    pub total_views: String,
}

impl From<Channel> for FullChannel {
    fn from(original: Channel) -> Self {
        Self {
            author: original.author,
            author_id: original.authorId,
            author_url: original.authorUrl,
            thumbnail: original.authorThumbnails[4].url.clone(),
            description: original.description,
            sub_count: original.subCount.separate_with_commas(),
            total_views: original.totalViews.separate_with_commas(),
        }
    }
}
