use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typemap::Key;

use crate::global::traits::ConfigTrait;

#[derive(Clone)]
pub struct CommandsConfig {
    pub video: HashMap<String, String>,
    pub playlist: HashMap<String, String>,
}

impl Key for CommandsConfig {
    type Value = Self;
}

impl From<CommandsConfigSerde> for CommandsConfig {
    fn from(original: CommandsConfigSerde) -> Self {
        Self {
            video: HashMap::from_iter(original.video.into_iter().map(|item| item.into_iter().last().unwrap())),
            playlist: HashMap::from_iter(original.playlist.into_iter().map(|item| item.into_iter().last().unwrap())),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandsConfigSerde {
    #[serde(default = "video_default")]
    pub video: Vec<HashMap<String, String>>,
    #[serde(default = "playlist_default")]
    pub playlist: Vec<HashMap<String, String>>,
}

impl ConfigTrait for CommandsConfigSerde {
    const LABEL: &'static str = "commands";
}

impl Default for CommandsConfigSerde {
    fn default() -> Self {
        Self {
            video: video_default(),
            playlist: playlist_default(),
        }
    }
}

fn video_default() -> Vec<HashMap<String, String>> {
    vec![HashMap::from(
        [(
        String::from("Play video"),
        String::from("${video-player} ${embed-url}"),
    )])]
}

fn playlist_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Play entire playlist"),
            String::from("${video-player} ${embed-url}"),
        )]),
    ]
}
