use std::collections::HashMap;

use crate::traits::ConfigItem;
use serde::{Deserialize, Serialize};

fn max_watch_history_default() -> usize {
    50
}

fn server_url_default() -> String {
    String::from("https://vid.puffyan.us")
}

fn env_default() -> HashMap<String, String> {
    let mut out = HashMap::default();

    out.insert(
        String::from("download_location"),
        String::from("~/Downloads/%(title)s.%(ext)s"),
    );

    out
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainConfig {
    #[serde(default = "max_watch_history_default")]
    pub max_watch_history: usize,

    #[serde(default = "server_url_default")]
    pub server_url: String,

    #[serde(default = "env_default")]
    pub env: HashMap<String, String>,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            max_watch_history: max_watch_history_default(),
            server_url: server_url_default(),
            env: env_default(),
        }
    }
}

impl ConfigItem<'_> for MainConfig {
    type Struct = MainConfig;
    const FILE_NAME: &'static str = "main.yml";
}
