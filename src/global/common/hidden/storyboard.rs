use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyboard {
    pub url: String,
    #[serde(rename = "templateUrl")]
    pub template_url: String,
    pub width: u32,
    pub height: u32,
    pub count: i32,
    pub interval: u32,
    #[serde(rename = "storyboardWidth")]
    pub storyboard_width: u16,
    #[serde(rename = "storyboardHeight")]
    pub storyboard_height: u16,
    #[serde(rename = "storyboardCount")]
    #[serde(default)]
    pub storyboard_count: i16,
}
