use crate::global::traits::ConfigTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

/// `main.yml`, the main config file
#[derive(Serialize, Deserialize, Clone)]
pub struct MainConfig {
    #[serde(default = "invidious_instance_default")]
    pub invidious_instance: String,
    #[serde(default = "allow_unicode_default")]
    pub allow_unicode: bool,
    #[serde(default = "message_bar_default_default")]
    pub message_bar_default: String,
    #[serde(default = "images_default")]
    pub images: Images,
    #[serde(default = "refresh_after_modifying_search_filters_default")]
    pub refresh_after_modifying_search_filters: bool,
    #[serde(default = "image_index_default")]
    // The image to download from the array of images provided by the invidious api
    // 0 is usually `maxres` and 3 (default) is good enough for normal uses without having huge files sizes
    // Check `https://{any invidious instance}/api/v1/videos/{any valid video id}` property videoThumbnails in Firefox dev tools to see it for yourself
    pub image_index: usize,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl Key for MainConfig {
    type Value = Self;
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            invidious_instance: invidious_instance_default(),
            allow_unicode: allow_unicode_default(),
            message_bar_default: message_bar_default_default(),
            images: images_default(),
            image_index: image_index_default(),
            refresh_after_modifying_search_filters: refresh_after_modifying_search_filters_default(
            ),

            env: HashMap::default(),
        }
    }
}

impl ConfigTrait for MainConfig {
    const LABEL: &'static str = "main";
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Images {
    r#None,
    HalfBlocks,
    Sixels,
}

impl Images {
    pub fn display(&self) -> bool {
        !(self == &Self::None)
    }

    pub fn use_sixels(&self) -> bool {
        self == &Self::Sixels
    }
}

// default functions

fn invidious_instance_default() -> String {
    String::from("https://invidious.sethforprivacy.com")
}

fn message_bar_default_default() -> String {
    String::from("All good :)")
}

const fn images_default() -> Images {
    Images::HalfBlocks
}

const fn image_index_default() -> usize {
    4
}

const fn allow_unicode_default() -> bool {
    false
}

const fn refresh_after_modifying_search_filters_default() -> bool {
    true
}
