use serde::{Deserialize, Serialize};

use crate::global::common::CommonThumbnail;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MixVideo {
    title: String,
    #[serde(rename = "mixId")]
    id: String,
    #[serde(rename = "audioId")]
    audio_id: String,
    author: String,
    #[serde(rename = "authorUrl")]
    author_url: String,
    videos: Vec<CommonThumbnail>,
    index: u32,
    #[serde(rename = "lengthSeconds")]
    length: u32,
}
