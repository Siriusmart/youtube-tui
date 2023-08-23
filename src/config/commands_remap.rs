use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

use crate::global::traits::ConfigTrait;

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandsRemapConfig(pub HashMap<String, String>);

impl ConfigTrait for CommandsRemapConfig {
    const LABEL: &'static str = "cmdefine";
}

impl Key for CommandsRemapConfig {
    type Value = Self;
}

impl Default for CommandsRemapConfig {
    fn default() -> Self {
        Self(HashMap::from([
            ("h".to_string(), "help".to_string()),
            ("v".to_string(), "version".to_string()),
            ("cp".to_string(), "copy".to_string()),
            ("print".to_string(), "echo".to_string()),
            ("popular".to_string(), "loadpage popular".to_string()),
            ("trending".to_string(), "loadpage trending".to_string()),
            (
                "watchhistory".to_string(),
                "loadpage watchhistory".to_string(),
            ),
            ("feed".to_string(), "loadpage feed".to_string()),
            ("bookmarks".to_string(), "loadpage bookmarks".to_string()),
            ("library".to_string(), "loadpage library".to_string()),
            ("search".to_string(), "loadpage search".to_string()),
            ("channel".to_string(), "loadpage channel".to_string()),
            ("video".to_string(), "loadpage video".to_string()),
            ("playlist".to_string(), "loadpage playlist".to_string()),
            ("back".to_string(), "history back".to_string()),
            ("r".to_string(), "reload".to_string()),
            ("rc".to_string(), "reload configs".to_string()),
            ("q".to_string(), "quit".to_string()),
            ("x".to_string(), "quit".to_string()),
            ("exit".to_string(), "quit".to_string()),
            ("sub".to_string(), "sync".to_string()),
        ]))
    }
}

impl CommandsRemapConfig {
    pub fn get(&self, cmd: &[&str]) -> Option<String> {
        for i in (0..cmd.len() + 1).rev() {
            if let Some(remapped) = self.0.get(&cmd[0..i].join(" ")) {
                return Some(format!("{remapped} {}", cmd[i..cmd.len()].join(" ")));
            }
        }

        None
    }
}
