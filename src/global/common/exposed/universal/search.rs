use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    pub items: Vec<SearchItem>,
}
