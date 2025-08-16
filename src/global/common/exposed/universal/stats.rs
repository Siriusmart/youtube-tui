use crate::global::common::hidden::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub version: String,
    pub software: Software,
    #[serde(rename = "openRegistrations")]
    pub registrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
}
