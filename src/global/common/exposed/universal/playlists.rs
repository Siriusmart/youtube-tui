use crate::global::common::{hidden::*, CommonImage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub title: String,
    #[serde(rename = "playlistId")]
    pub id: String,
    #[serde(rename = "playlistThumbnail")]
    pub thumbnail: String,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Vec<CommonImage>,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,

    #[serde(rename = "videoCount")]
    pub video_count: u32,
    #[serde(rename = "viewCount")]
    pub views: u64,
    pub updated: u64,
    #[serde(rename = "isListed")]
    pub listed: bool,

    pub videos: Vec<PlaylistItem>,
}
