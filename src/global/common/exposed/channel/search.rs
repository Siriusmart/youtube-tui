use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSearch {
    pub items: Vec<SearchItem>,
}
