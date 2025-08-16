use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mix {
    pub title: String,
    #[serde(rename = "midId")]
    pub id: String,
    pub videos: Vec<MixVideo>,
}
