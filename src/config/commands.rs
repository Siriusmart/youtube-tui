use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typemap::Key;

use crate::global::traits::ConfigTrait;

/// Stores combinations of label and commands
#[derive(Clone)]
pub struct CommandsConfig {
    pub launch_command: String,
    pub video: Vec<(String, String)>,
    pub saved_video: Vec<(String, String)>,
    pub playlist: Vec<(String, String)>,
    pub saved_playlist: Vec<(String, String)>,
    pub channel: Vec<(String, String)>,
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
            saved_video: original
                .saved_video
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
            playlist: original
                .playlist
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
            saved_playlist: original
                .saved_playlist
                .into_iter()
                .map(|hashmap| hashmap.into_iter().last().unwrap())
                .collect(),
            channel: original
                .channel
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
    #[serde(default = "saved_video_default")]
    pub saved_video: Vec<HashMap<String, String>>,
    #[serde(default = "playlist_default")]
    pub playlist: Vec<HashMap<String, String>>,
    #[serde(default = "saved_playlist_default")]
    pub saved_playlist: Vec<HashMap<String, String>>,
    #[serde(default = "channel_default")]
    pub channel: Vec<HashMap<String, String>>,
}

impl ConfigTrait for CommandsConfigSerde {
    const LABEL: &'static str = "commands";
}

impl Default for CommandsConfigSerde {
    fn default() -> Self {
        Self {
            launch_command: launch_command_default(),
            video: video_default(),
            saved_video: saved_video_default(),
            playlist: playlist_default(),
            saved_playlist: saved_playlist_default(),
            channel: channel_default(),
        }
    }
}

// `${label}` will be replaced by the values set in `main.yml` in `env`
// Different pages may contain different `env`s (for example `url` is different in each page)

fn launch_command_default() -> String {
    String::from(
        "loadpage library ;; flush ;; history clear ;; key Esc 0 ;; key Up 0 ;; key Up 0 ;; key Left 0 ;; key Enter 0",
    )
}

fn video_default() -> Vec<HashMap<String, String>> {
    #[cfg(target_os = "windows")]
    let q = "\"";
    #[cfg(not(target_os = "windows"))]
    let q = "'";

    #[cfg(target_os = "windows")]
    let rm_cmd = |pattern: &str| format!("run cmd /c del /q /s \"{pattern}\"");
    #[cfg(not(target_os = "windows"))]
    let rm_cmd = |pattern: &str| format!("run rm -rf '{pattern}'");

    vec![
        HashMap::from([(
            String::from("Reload updated video"),
            String::from("rmcache ${id} ;; video ${id}"),
        )]),
        HashMap::from([(
            String::from("Play video"),
            format!("parrun ${{video-player}} {q}${{embed-url}}{q}"),
        )]),
        HashMap::from([(
            String::from("Play audio"),
            format!("mpv stop ;; resume ;; mpv sprop loop-file no ;; mpv loadfile {q}${{embed-url}}{q} ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("Play audio (loop)"),
            format!("mpv stop ;; resume ;; mpv sprop loop-file inf ;; mpv loadfile {q}${{embed-url}}{q} ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Subscribe to channel"),
            String::from("sync ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            format!("parrun ${{browser}} {q}${{url}}{q}"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save video to library"),
            format!("bookmark ${{id}} ;; {} ;; parrun ${{terminal-emulator}} ${{youtube-downloader}} {q}${{embed-url}}{q} -o {q}${{save-path}}%(title)s[${{id}}].%(ext)s{q}", rm_cmd("${save-path}${id}.*"))
        )]),
        HashMap::from([(
            String::from("Save audio to library"),
            format!("bookmark ${{id}} ;; {} ;; parrun ${{terminal-emulator}} ${{youtube-downloader}} {q}${{embed-url}}{q} -x -o {q}${{save-path}}%(title)s[${{id}}].%(ext)s{q}", rm_cmd("${save-path}${id}.*"))
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}

fn saved_video_default() -> Vec<HashMap<String, String>> {
    #[cfg(target_os = "windows")]
    let q = "\"";
    #[cfg(not(target_os = "windows"))]
    let q = "'";

    #[cfg(target_os = "windows")]
    let rm_cmd = |pattern: &str| format!("run cmd /c del /q /s \"{pattern}\"");
    #[cfg(not(target_os = "windows"))]
    let rm_cmd = |pattern: &str| format!("run rm {pattern}");

    vec![
        HashMap::from([(
            String::from("Reload updated video"),
            String::from("rmcache ${id} ;; video ${id}"),
        )]),
        HashMap::from([(
            String::from("[Offline] Play saved file"),
            format!("parrun ${{video-player}} {q}${{offline-path}}{q} --force-window"),
        )]),
        HashMap::from([(
            String::from("[Offline] Play saved file (audio)"),
            format!("mpv stop ;; resume ;; mpv sprop loop-file no ;; mpv loadfile {q}${{offline-path}}{q} ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("[Offline] Play saved file (audio loop)"),
            format!("mpv stop ;; resume ;; mpv sprop loop-file inf ;; mpv loadfile {q}${{offline-path}}{q} ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Subscribe to channel"),
            String::from("sync ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            format!("parrun ${{browser}} {q}${{url}}{q}"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Redownload video to library"),
            format!("bookmark ${{id}} ;; {} ;; parrun ${{terminal-emulator}} ${{youtube-downloader}} ${{embed-url}} -o {q}${{save-path}}%(title)s[${{id}}].%(ext)s{q}", rm_cmd("${save-path}*${id}*.*")),
        )]),
        HashMap::from([(
            String::from("Redownload audio to library"),
            format!("bookmark ${{id}} ;; {} ;; parrun ${{terminal-emulator}} ${{youtube-downloader}} ${{embed-url}} -x -o {q}${{save-path}}%(title)s[${{id}}].%(ext)s{q}", rm_cmd("${save-path}*${id}*.*"))
        )]),
        HashMap::from([(
            String::from("Delete saved file"),
            rm_cmd("${save-path}*${id}*.*")
        )]),
    ]
}

fn playlist_default() -> Vec<HashMap<String, String>> {
    #[cfg(target_os = "windows")]
    let q = "\"";
    #[cfg(not(target_os = "windows"))]
    let q = "'";

    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Reload updated playlist"),
            String::from("rmcache ${id} ;; playlist ${id}"),
        )]),
        HashMap::from([(
            String::from("Play all (videos)"),
            String::from("parrun ${video-player} ${url}"),
        )]),
        HashMap::from([(
            String::from("Play all (audio)"),
            String::from("mpv stop ;; resume ;; ${mpv-queuelist} ;; mpv sprop loop-playlist no ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all (audio loop)"),
            String::from("mpv stop ;; resume ;; ${mpv-queuelist} ;; mpv sprop loop-playlist yes ;; mpv playlist-shuffle ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Subscribe to channel"),
            String::from("sync ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            format!("parrun ${{browser}} {q}${{url}}{q}"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Save playlist videos to library"),
            {
                #[cfg(target_os = "windows")]
                { String::from("bookmark ${id} ;; run cmd /c rd /s /q \"${save-path}*${id}*\" ;; parrun ${terminal-emulator} ${youtube-downloader} ${all-videos} -o \"${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s\"") }
                #[cfg(not(target_os = "windows"))]
                { String::from("bookmark ${id} ;; run rm -rf '${save-path}*${id}*' ;; parrun ${terminal-emulator} bash -c \"${youtube-downloader} ${all-videos} -o '\"'${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s'\"'\"") }
            }
        )]),
        HashMap::from([(
            String::from("Save playlist audio to library"),
            {
                #[cfg(target_os = "windows")]
                { String::from("bookmark ${id} ;; run cmd /c rd /s /q \"${save-path}*${id}*\" ;; parrun ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o \"${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s\"") }
                #[cfg(not(target_os = "windows"))]
                { String::from("bookmark ${id} ;; run rm -rf '${save-path}*${id}*' ;; parrun ${terminal-emulator} bash -c \"${youtube-downloader} ${all-videos} -x -o '\"'${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s'\"'\"") }
            }
        )]),
        HashMap::from([(
            String::from("Mode: ${provider}"),
            String::from("switchprovider"),
        )]),
    ]
}

fn saved_playlist_default() -> Vec<HashMap<String, String>> {
    #[cfg(target_os = "windows")]
    let q = "\"";
    #[cfg(not(target_os = "windows"))]
    let q = "'";

    vec![
        HashMap::from([(String::from("Switch view"), String::from("%switch-view%"))]),
        HashMap::from([(
            String::from("Reload updated playlist"),
            String::from("rmcache ${id} ;; playlist ${id}"),
        )]),
        HashMap::from([(
            String::from("[Offline] Play all (videos)"),
            String::from("parrun ${video-player} ${save-path}*${id}*/* --force-window"),
        )]),
        HashMap::from([(
            String::from("[Offline] Play all (audio)"),
            String::from("mpv stop ;; resume ;; ${offline-queuelist} ;; mpv sprop loop-playlist no ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("[Offline] Shuffle play all (audio loop)"),
            String::from("mpv stop ;; resume ;; ${offline-queuelist} ;; mpv sprop loop-playlist yes ;; mpv playlist-shuffle ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("View channel"),
            String::from("channel ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Subscribe to channel"),
            String::from("sync ${channel-id}"),
        )]),
        HashMap::from([(
            String::from("Open in browser"),
            format!("parrun ${{browser}} {q}${{url}}{q}"),
        )]),
        HashMap::from([(
            String::from("Toggle bookmark"),
            String::from("togglemark ${id}")
        )]),
        HashMap::from([(
            String::from("Redownload playlist videos to library"),
            {
                #[cfg(target_os = "windows")]
                { String::from("bookmark ${id} ;; run cmd /c rd /s /q \"${save-path}*${id}*\" ;; parrun ${terminal-emulator} ${youtube-downloader} ${all-videos} -o \"${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s\"") }
                #[cfg(not(target_os = "windows"))]
                { String::from("bookmark ${id} ;; run rm -rf ${save-path}*${id}* ;; parrun ${terminal-emulator} bash -c \"${youtube-downloader} ${all-videos} -o '\"'${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s'\"'\"") }
            }
        )]),
        HashMap::from([(
            String::from("Redownload playlist audio to library"),
            {
                #[cfg(target_os = "windows")]
                { String::from("bookmark ${id} ;; run cmd /c rd /s /q \"${save-path}*${id}*\" ;; parrun ${terminal-emulator} ${youtube-downloader} ${all-videos} -x -o \"${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s\"") }
                #[cfg(not(target_os = "windows"))]
                { String::from("bookmark ${id} ;; run rm -rf ${save-path}*${id}* ;; parrun ${terminal-emulator} bash -c \"${youtube-downloader} ${all-videos} -x -o '\"'${save-path}${title}[${id}]/%(title)s[%(id)s].%(ext)s'\"'\"") }
            }
        )]),
        HashMap::from([(
            String::from("Delete saved files"),
            {
                #[cfg(target_os = "windows")]
                { String::from("run cmd /c rd /s /q \"${save-path}*${id}*\"") }
                #[cfg(not(target_os = "windows"))]
                { String::from("run rm -rf ${save-path}*${id}*") }
            }
        )]),
    ]
}

fn channel_default() -> Vec<HashMap<String, String>> {
    vec![
        HashMap::from([(
            String::from("Reload updated channel"),
            String::from("rmcache ${id} ;; channel ${id}"),
        )]),
        HashMap::from([(
            String::from("Subscribe to channel"),
            String::from("sync ${id}"),
        )]),
        HashMap::from([(
            String::from("Play all (videos)"),
            String::from("parrun ${video-player} ${url}"),
        )]),
        HashMap::from([(
            String::from("Play all (audio)"),
            String::from("mpv stop ;; resume ;; mpv loadfile ${url} ;; mpv sprop loop-playlist no ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
        HashMap::from([(
            String::from("Shuffle play all (audio loop)"),
            String::from("mpv stop ;; resume ;; mpv loadfile ${url} ;; mpv sprop loop-playlist yes ;; mpv playlist-shuffle ;; mpv playlist-play-index 0 ;; echo mpv Player started"),
        )]),
    ]
}
