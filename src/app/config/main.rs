use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub yt_dl: YtDlConfig,
    pub max_watch_history: usize,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            yt_dl: YtDlConfig::default(),
            max_watch_history: 50,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YtDlConfig {
    pub video_path: String,
    pub audio_path: String,
}

impl Default for YtDlConfig {
    fn default() -> Self {
        Self {
            video_path: String::from("~/Downloads/%(title)s.%(ext)s"),
            audio_path: String::from("~/Downloads/%(title)s.%(ext)s"),
        }
    }
}
