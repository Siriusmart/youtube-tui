use crate::global::common::{hidden::*, CommonImage, CommonThumbnail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    #[serde(rename = "type")]
    pub r#type: String,
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<CommonThumbnail>,
    pub storyboards: Vec<Storyboard>,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,

    pub keywords: Vec<String>,
    #[serde(rename = "viewCount")]
    pub views: u64,
    #[serde(rename = "likeCount")]
    pub likes: u32,
    #[serde(rename = "dislikeCount")]
    pub dislikes: u32,

    pub paid: bool,
    pub premium: bool,
    #[serde(rename = "isFamilyFriendly")]
    pub family_friendly: bool,
    #[serde(rename = "allowedRegions")]
    pub allowed_regions: Vec<CountryCode>,
    pub genre: String,
    #[serde(rename = "genreUrl")]
    pub genre_url: Option<String>,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Vec<CommonImage>,

    #[serde(rename = "subCountText")]
    pub sub_count_text: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "allowRatings")]
    pub allow_ratings: bool,
    pub rating: f32,
    #[serde(rename = "isListed")]
    pub listed: bool,
    #[serde(rename = "liveNow")]
    pub live: bool,
    #[serde(rename = "isUpcoming")]
    pub upcoming: bool,
    #[serde(rename = "premiereTimestamp")]
    #[serde(default)]
    pub premiere_timestamp: u64,
    #[serde(rename = "dashUrl")]
    pub dash: String,

    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormat>,
    #[serde(rename = "formatStreams")]
    pub format_streams: Vec<FormatStream>,

    pub captions: Vec<Caption>,

    #[serde(rename = "recommendedVideos")]
    pub recommended_videos: Vec<VideoShort>,
}
