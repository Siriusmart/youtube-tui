use crate::global::common::CommonPlaylist;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPlaylists {
    pub playlists: Vec<CommonPlaylist>,
    pub continuation: Option<String>,
}
