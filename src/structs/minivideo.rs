use chrono::{prelude::*, Utc};
use invidious::structs::hidden::{PopularItem, TrendingVideo, SearchItem};
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

impl From<SearchItem> for MiniVideo {
    fn from(original: SearchItem) -> Self {
        match original {
            SearchItem::Video { title, videoId, author, authorId: _, authorUrl, lengthSeconds, videoThumbnails, description, descriptionHtml: _, viewCount, published, publishedText, liveNow: _, paid: _, premium: _ } => {
                MiniVideo {
                    title: title,
                    video_id: videoId,
                    video_thumbnail: videoThumbnails[0].url.clone(),
                    length: hrtime::from_sec_padded(lengthSeconds as u64),
                    view_count: viewCount.separate_with_commas(),
                    author: author,
                    author_url: authorUrl,
                    published: format!(
                        "{}{}",
                        publishedText,
                        if Utc::now().timestamp() - published as i64 > 86400 {
                            let datetime: DateTime<Utc> = DateTime::from_utc(
                                NaiveDateTime::from_timestamp(published as i64, 0),
                                Utc,
                            );
                            format!(" [{}]", datetime.format("%Y/%m/%d"))
                        } else {
                            String::new()
                        }
                    ),
                    description: description,
                }
            }

            _ => unreachable!(),
        }
    }
}