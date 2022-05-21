use chrono::{prelude::*, Utc};
use invidious::structs::hidden::{PopularItem, TrendingVideo};
use thousands::Separable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniVideo {
    pub title: String,
    pub video_id: String,
    pub video_thumbnail: String,
    pub length: String,
    pub view_count: String,
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
            length: hrtime::from_sec_padded(original.lengthSeconds as u64),
            view_count: original.viewCount.separate_with_commas(),
            author: original.author,
            author_url: original.authorUrl,
            published: format!(
                "{}{}",
                original.publishedText,
                if Utc::now().timestamp() - original.published as i64 > 86400 {
                    let datetime: DateTime<Utc> = DateTime::from_utc(
                        NaiveDateTime::from_timestamp(original.published as i64, 0),
                        Utc,
                    );
                    format!(" [{}]", datetime.format("%Y/%m/%d"))
                } else {
                    String::new()
                }
            ),
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
            length: hrtime::from_sec_padded(original.lengthSeconds as u64),
            view_count: original.viewCount.separate_with_commas(),
            author: original.author,
            author_url: original.authorUrl,
            published: format!(
                "{}{}",
                original.publishedText,
                if Utc::now().timestamp() - original.published as i64 > 86400 {
                    let datetime: DateTime<Utc> = DateTime::from_utc(
                        NaiveDateTime::from_timestamp(original.published as i64, 0),
                        Utc,
                    );
                    format!(" [{}]", datetime.format("%Y/%m/%d"))
                } else {
                    String::new()
                }
            ),
            description: String::new(),
        }
    }
}
