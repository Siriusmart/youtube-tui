use crate::global::common::CommonVideo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<CommonVideo>,
    pub continuation: Option<String>,
}
