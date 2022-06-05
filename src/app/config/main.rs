use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainConfigTransitional {
    pub yt_dl: Option<YtDlConfig>,
    pub max_watch_history: Option<usize>,
}

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

impl From<MainConfigTransitional> for MainConfig {
    fn from(config: MainConfigTransitional) -> Self {
        let mut out = MainConfig::default();

        if let Some(yt_dl) = config.yt_dl {
            out.yt_dl = yt_dl;
        }

        if let Some(max_watch_history) = config.max_watch_history {
            out.max_watch_history = max_watch_history;
        }

        out
    }
}

impl MainConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut config = home::home_dir().expect("Cannot get your home directory");
        let mut main = MainConfig::default();
        config.push(".config");
        config.push("youtube-tui");
        config.push("main.yml");

        if config.exists() {
            let content = fs::read_to_string(config.as_os_str())?;
            let main_transitional: MainConfigTransitional = serde_yaml::from_str(&content)?;
            main = main_transitional.into();
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config.as_os_str())?;

        write!(file, "{}", serde_yaml::to_string(&main)?)?;

        Ok(main)
    }
}
