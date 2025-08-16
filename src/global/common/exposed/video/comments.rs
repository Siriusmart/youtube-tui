use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comments {
    #[serde(rename = "commentCount")]
    pub comment_count: Option<u32>,
    #[serde(rename = "videoId")]
    pub id: String,
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}
