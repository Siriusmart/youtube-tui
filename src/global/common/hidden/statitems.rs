use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Software {
    pub name: String,
    pub version: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub users: Users,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users {
    pub total: u32,
    #[serde(rename = "activeHalfyear")]
    pub half_year: u32,
    #[serde(rename = "activeMonth")]
    pub month: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    #[serde(rename = "updatedAt")]
    pub updated: u64,
    #[serde(rename = "lastChannelRefreshedAt")]
    pub last_channel_refresh: u64,
}
