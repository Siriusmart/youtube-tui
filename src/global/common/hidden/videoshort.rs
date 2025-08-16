use serde::{Deserialize, Serialize};

use crate::global::common::CommonThumbnail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoShort {
    #[serde(rename = "videoId")]
    pub id: String,
    pub title: String,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<CommonThumbnail>,
    pub author: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,
    #[serde(rename = "viewCountText")]
    pub views_text: String,
}
