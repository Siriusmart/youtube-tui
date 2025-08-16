use crate::global::common::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SearchItem {
    #[serde(rename = "video")]
    Video(CommonVideo),
    #[serde(rename = "playlist")]
    Playlist(CommonPlaylist),
    #[serde(rename = "channel")]
    Channel(CommonChannel),
}
