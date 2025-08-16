use crate::global::common::CommonImage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    #[serde(default)]
    pub verified: bool,
    #[serde(rename = "commentId")]
    pub id: String,
    #[serde(rename = "likeCount")]
    pub likes: u32,
    #[serde(rename = "isEdited")]
    pub edited: bool,
    pub content: String,
    #[serde(rename = "contentHtml")]
    pub content_html: String,
    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,

    pub author: String,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Vec<CommonImage>,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,

    #[serde(rename = "authorIsChannelOwner")]
    pub channel_owner: bool,
    #[serde(default)]
    #[serde(rename = "createrHeart")]
    pub heart: Option<CreatorHeart>,
    #[serde(default)]
    pub replies: Option<Replies>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatorHeart {
    #[serde(rename = "creatorThumbnail")]
    pub thumbnail: String,
    #[serde(rename = "creatorName")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Replies {
    #[serde(rename = "replyCount")]
    replies: u32,
    continuation: String,
}
