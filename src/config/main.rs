use crate::global::traits::ConfigTrait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

/// `main.yml`, the main config file
#[derive(Serialize, Deserialize, Clone)]
pub struct MainConfig {
    #[serde(default = "mouse_support_default")]
    pub mouse_support: bool,
    #[serde(default = "invidious_instance_default")]
    pub invidious_instance: String,
    #[serde(default = "max_watch_history_default")]
    pub max_watch_history: usize,
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
    #[serde(default = "provider_default")]
    pub provider: Provider,
    #[serde(default = "shell_default")]
    pub shell: String,
    #[serde(default = "default_env")]
    pub env: HashMap<String, String>,
}

impl Key for MainConfig {
    type Value = Self;
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            mouse_support: mouse_support_default(),
            invidious_instance: invidious_instance_default(),
            max_watch_history: max_watch_history_default(),
            allow_unicode: allow_unicode_default(),
            message_bar_default: message_bar_default_default(),
            images: images_default(),
            image_index: image_index_default(),
            refresh_after_modifying_search_filters: refresh_after_modifying_search_filters_default(
            ),
            provider: provider_default(),
            shell: shell_default(),

            env: default_env(),
        }
    }
}

impl ConfigTrait for MainConfig {
    const LABEL: &'static str = "main";
}

/// how images are handled/displayed
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

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Provider {
    YouTube,
    Invidious,
}

impl Provider {
    pub fn as_str(&self) -> &str {
        match self {
            Self::YouTube => "YouTube",
            Self::Invidious => "Invidious",
        }
    }

    pub fn rotate(&mut self) {
        *self = match self {
            Self::YouTube => Self::Invidious,
            Self::Invidious => Self::YouTube,
        };
    }
}

// default functions

fn invidious_instance_default() -> String {
    String::from("https://vid.puffyan.us")
}

fn message_bar_default_default() -> String {
    String::from("All good :)")
}

const fn images_default() -> Images {
    Images::Sixels
}

const fn image_index_default() -> usize {
    4
}

const fn allow_unicode_default() -> bool {
    true
}

const fn refresh_after_modifying_search_filters_default() -> bool {
    true
}

const fn provider_default() -> Provider {
    Provider::YouTube
}

const fn max_watch_history_default() -> usize {
    50
}

const fn mouse_support_default() -> bool {
    true
}

fn default_env() -> HashMap<String, String> {
    HashMap::from([
        (String::from("video-player"), String::from("mpv")),
        (String::from("browser"), String::from("firefox")),
        (
            String::from("terminal-emulator"),
            String::from("konsole -e"),
        ),
        (String::from("youtube-downloader"), String::from("yt-dlp")),
        (
            String::from("download-path"),
            String::from("~/Downloads/%(title)s-%(id)s.%(ext)s"),
        ),
        (
            String::from("save-path"),
            String::from("~/.local/share/youtube-tui/saved/"),
        ),
    ])
}

fn shell_default() -> String {
    String::from("sh")
}
