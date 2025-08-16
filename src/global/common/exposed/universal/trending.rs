use serde::{Deserialize, Serialize};

use crate::global::common::CommonVideo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trending {
    pub videos: Vec<CommonVideo>,
}
