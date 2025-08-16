use serde::{Deserialize, Serialize};

use crate::global::common::CommonThumbnail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistItem {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,
    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,

    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<CommonThumbnail>,
    pub index: u32,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
}
