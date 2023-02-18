use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typemap::Key;

use crate::global::traits::ConfigTrait;

/// Stores combinations of label and commands
#[derive(Clone)]
pub struct CommandsConfig {
    pub launch_command: String,
    pub video: Vec<(String, String)>,
    pub playlist: Vec<(String, String)>,
    pub local_video: Vec<(String, String)>,
    pub local_playlist: Vec<(String, String)>,
}

impl Key for CommandsConfig {
    type Value = Self;
}

impl From<CommandsConfigSerde> for CommandsConfig {
    fn from(original: CommandsConfigSerde) -> Self {
        Self {
            launch_command: original.launch_command,
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
            local_video: original
                .local_video
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
            local_playlist: original
                .local_playlist
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
        }
    }
}

/// Hashmaps are better formatted in YAML, impls `Into<CommandsConfig>`
// uses vector to keep the ordering of the commands, and hashmap to have that key - value pair look
#[derive(Serialize, Deserialize, Clone)]
pub struct CommandsConfigSerde {
    #[serde(default = "launch_command_default")]
    pub launch_command: String,
    #[serde(default = "video_default")]
    pub video: Vec<HashMap<String, String>>,
    #[serde(default = "playlist_default")]
    pub playlist: Vec<HashMap<String, String>>,
    #[serde(default = "local_video_default")]
    pub local_video: Vec<HashMap<String, String>>,
    #[serde(default = "local_playlist_default")]
    pub local_playlist: Vec<HashMap<String, String>>,
}

impl ConfigTrait for CommandsConfigSerde {
    const LABEL: &'static str = "commands";
}

impl Default for CommandsConfigSerde {
    fn default() -> Self {
        Self {
            launch_command: launch_command_default(),
            video: video_default(),
            playlist: playlist_default(),
            local_video: local_video_default(),
            local_playlist: local_playlist_default(),
        }
    }
}

// `${label}` will be replaced by the values set in `main.yml` in `env`
// Different pages may contain different `env`s (for example `url` is different in each page)

fn launch_command_default() -> String {
    String::from("loadpage watchhistory ;; flush ;; history clear")
}

fn video_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(
            String::from("Play video"),
            String::from("run ${video-player} '${embed-url}'"),
        )]),
        HashMap::from([(
            String::from("Play audio"),
            String::from("run ${terminal-emulator} ${video-player} '${embed-url}' --no-video"),
        )]),
        HashMap::from([(
            String::from("Play audio (loop)"),
            String::from(
                "run ${terminal-emulator} ${video-player} '${embed-url}' --no-video --loop-file=inf",
            ),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("run ${browser} '${url}'"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save video"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Save audio"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}

fn playlist_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Reload as online mode"),
            String::from("playlist ${id}"),
        )]),
        HashMap::from([(
            String::from("Play all videos"),
            String::from("run ${video-player} ${all-videos}"),
        )]),
        HashMap::from([(
            String::from("Play all audio"),
            String::from("run ${terminal-emulator} ${video-player} ${all-videos} --no-video"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all audio (loop)"),
            String::from("run ${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle --loop-playlist=inf"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("run ${browser} '${url}'"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save all audio"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run mkdir '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}

fn local_video_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(
            String::from("Reload as online mode"),
            String::from("video ${id}"),
        )]),
        HashMap::from([(
            String::from("Play video"),
            String::from("run ${video-player} '${embed-url}'"),
        )]),
        HashMap::from([(
            String::from("Play audio"),
            String::from("run ${terminal-emulator} ${video-player} '${embed-url}' --no-video"),
        )]),
        HashMap::from([(
            String::from("Play audio (loop)"),
            String::from(
                "run ${terminal-emulator} ${video-player} '${embed-url}' --no-video --loop-file=inf",
            ),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("run ${browser} '${url}'"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save video"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Save audio"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}

fn local_playlist_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Reload as online mode"),
            String::from("playlist ${id}"),
        )]),
        HashMap::from([(
            String::from("Play all videos"),
            String::from("run ${video-player} ${all-videos}"),
        )]),
        HashMap::from([(
            String::from("Play all audio"),
            String::from("run ${terminal-emulator} ${video-player} ${all-videos} --no-video"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all audio (loop)"),
            String::from("run ${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle --loop-playlist=inf"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            String::from("run ${browser} '${url}'"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save all audio"),
            String::from("bookmark ${id} ;; run rm -rf '${save-path}${id}' ;; run mkdir '${save-path}${id}' ;; run ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o '${save-path}${id}/%(id)s'")
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}
