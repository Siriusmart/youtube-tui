use chrono::{DateTime, NaiveDateTime, Utc};
use invidious::structs::video::Video;
use thousands::Separable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullVideo {
    pub title: String,
    pub video_id: String,
    pub video_thumbnail: String,
    pub description: String,
    pub views: String,
    pub published: String,
    pub nsfw: bool,
    pub genre: String,
    pub channel: String,
    pub channel_id: String,
    pub subcount: String,
    pub length: String,
    pub public: bool,
    pub live: bool,
    pub upcoming: bool,
    pub recommendations: Vec<String>,
}

impl From<Video> for FullVideo {
    fn from(original: Video) -> Self {
        Self {
            title: original.title,
            video_id: original.videoId,
            video_thumbnail: original.videoThumbnails[5].url.clone(),
            description: original.description,
            views: original.viewCount.separate_with_commas(),
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
            nsfw: original.isFamilyFriendly,
            genre: original.genre,
            channel: original.author,
            channel_id: original.authorId,
            subcount: original.subCountText,
            length: hrtime::from_sec_padded(original.lengthSeconds as u64),
            public: original.isListed,
            live: original.liveNow,
            upcoming: original.isUpcoming,
            recommendations: original
                .recommendedVideos
                .into_iter()
                .map(|x| x.videoId)
                .collect(),
        }
    }
}
