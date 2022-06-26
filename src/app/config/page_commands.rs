use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::traits::ConfigItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCommandsConfig(pub HashMap<String, Vec<PageCommand>>);

impl PageCommandsConfig {
    fn blank() -> Self {
        Self(HashMap::default())
    }
}

impl Default for PageCommandsConfig {
    fn default() -> Self {
        let mut out = Self::blank();

        out.0.insert(
            String::from("item_info:video"),
            vec![
                PageCommand::from("Watch video", "video_player"),
                PageCommand::from("Play audio", "audio_player"),
                PageCommand::from("Download video", "video_downloader"),
                PageCommand::from("Download audio", "audio_downloader"),
                PageCommand::from("Visit channel", "{goto_channel}"),
                PageCommand::from("{mode}", "{toggle_mode}"),
            ],
        );

        out.0.insert(
            String::from("item_info:playlist"),
            vec![
                PageCommand::from("Switch view", "{switch_view}"),
                PageCommand::from("Watch all", "video_player"),
                PageCommand::from("Play all audio", "audio_player"),
                PageCommand::from("Shuffle play all audio", "audio_playlist_shuffle"),
                PageCommand::from(
                    "Shuffle play all audio (loop)",
                    "audio_playlist_shuffle_loop",
                ),
                PageCommand::from("Download all video", "video_downloader"),
                PageCommand::from("Download all audio", "audio_downloader"),
                PageCommand::from("Visit channel", "{goto_channel}"),
                PageCommand::from("{mode}", "{toggle_mode}"),
            ],
        );

        out
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageCommand {
    pub label: String,
    pub command: String,
}

impl PageCommand {
    fn from(label: &str, command: &str) -> Self {
        Self {
            label: label.to_string(),
            command: command.to_string(),
        }
    }
}

impl ConfigItem<'_> for PageCommandsConfig {
    type Struct = PageCommandsConfig;
    const FILE_NAME: &'static str = "page_commands.yml";
}
