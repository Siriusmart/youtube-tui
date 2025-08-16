use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelComments {
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub content: String,
    pub continuation: Option<String>,
}
