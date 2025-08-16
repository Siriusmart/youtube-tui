use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Caption {
    pub label: String,
    #[serde(rename = "language_code")]
    #[serde(default)]
    pub language: String,
    pub url: String,
}
