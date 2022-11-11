use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typemap::Key;

use crate::global::traits::ConfigTrait;

/// Stores combinations of label and commands
#[derive(Clone)]
pub struct CommandsConfig {
    pub video: Vec<(String, String)>,
    pub playlist: Vec<(String, String)>,
}

impl Key for CommandsConfig {
    type Value = Self;
}

impl From<CommandsConfigSerde> for CommandsConfig {
    fn from(original: CommandsConfigSerde) -> Self {
        Self {
            video: original
                .video
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
            playlist: original
                .playlist
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
        }
    }
}

/// Hashmaps are better formatted in YAML, impls Into<CommandsConfig>
// uses vector to keep the ordering of the commands, and hashmap to have that key - value pair look
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

// `${label}` will be replaced by the values set in `main.yml` in `env`
// Different pages may contain different `env`s (for example `url` is different in each page)

fn video_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(
            String::from("Play video"),
            String::from("${video-player} ${embed-url}"),
        )]),
        HashMap::from([(
            String::from("Play audio"),
            String::from("${terminal-emulator} ${video-player} ${embed-url} --no-video"),
        )]),
        HashMap::from([(
            String::from("Play audio (loop)"),
            String::from(
                "${terminal-emulator} ${video-player} ${embed-url} --no-video --loop-file=inf",
            ),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("${browser} ${url}"),
        )]),
        HashMap::from([(
            String::from("Download video (webm)"),
            String::from(
                "${terminal-emulator} ${youtube-downloader} -o ${download-path} ${embed-url}",
            ),
        )]),
        HashMap::from([(
            String::from("Download audio (opus)"),
            String::from(
                "${terminal-emulator} ${youtube-downloader} -o ${download-path} ${embed-url} -x",
            ),
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("%switch-provider%"),
        )]),
    ]
}

fn playlist_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Play all videos"),
            String::from("${video-player} ${all-videos}"),
        )]),
        HashMap::from([(
            String::from("Play all audio"),
            String::from("${terminal-emulator} ${video-player} ${all-videos} --no-video"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all audio"),
            String::from("${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all audio (loop)"),
            String::from("${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle --loop-playlist=inf"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("${browser} ${url}"),
        )]),
        HashMap::from([(
            String::from("Download all video (webm)"),
            String::from("${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos}")
        )]),
        HashMap::from([(
            String::from("Download all audio (opus)"),
            String::from("${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos} -x")
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("%switch-provider%"),
        )]),
    ]
}
