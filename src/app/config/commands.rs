use serde::{Deserialize, Serialize};
use std::{collections::HashMap, thread};

use crate::traits::ConfigItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Command {
    pub args: Vec<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandsConfig(pub HashMap<String, Command>);

impl Command {
    pub fn as_vec(&self, variables: &HashMap<String, String>) -> Result<Vec<String>, String> {
        let mut out = Vec::new();

        for arg in self.args.iter() {
            out.push(match check_arg(arg) {
                ArgType::Normal(s) => s,
                ArgType::Variable(s) => match variables.get(s) {
                    Some(s) => s.to_string(),
                    None => return Err(s.to_string()),
                },
            })
        }

        Ok(out)
    }

    pub fn run_command(&self, variables: &HashMap<String, String>) -> Option<String> {
        let mut args = match self.as_vec(variables) {
            Ok(args) => args,
            Err(s) => return Some(s),
        }
        .into_iter();
        thread::spawn(move || {
            let mut command = std::process::Command::new(args.next().unwrap());
            for arg in args {
                command.arg(arg);
            }

            let _ = command.output();
        });

        None
    }
}

impl Default for CommandsConfig {
    fn default() -> Self {
        let mut out = Self::blank();

        out.insert(
            "video_player",
            vec!["mpv", "{embed_url}", "--no-terminal"],
            "Launched mpv",
        );
        out.insert(
            "audio_player",
            vec!["konsole", "-e", "mpv", "{embed_url}", "--no-video"],
            "Opened mpv in new konsole window",
        );
        out.insert(
            "audio_playlist_shuffle",
            vec![
                "konsole",
                "-e",
                "mpv",
                "{embed_url}",
                "--no-video",
                "--shuffle",
            ],
            "Opened mpv in new konsole window",
        );
        out.insert(
            "audio_playlist_loop",
            vec![
                "konsole",
                "-e",
                "mpv",
                "{embed_url}",
                "--no-video",
                "--loop-playlist=inf",
            ],
            "Opened mpv in new konsole window",
        );
        out.insert(
            "audio_playlist_shuffle_loop",
            vec![
                "konsole",
                "-e",
                "mpv",
                "{embed_url}",
                "--no-video",
                "--loop-playlist=inf",
                "--shuffle",
            ],
            "Opened mpv in new konsole window",
        );
        out.insert(
            "video_downloader",
            vec![
                "konsole",
                "-e",
                "yt-dlp",
                "{embed_url}",
                "-o",
                "{download_location}",
            ],
            "Download has started",
        );

        out.insert(
            "audio_downloader",
            vec![
                "konsole",
                "-e",
                "yt-dlp",
                "{embed_url}",
                "-o",
                "{download_location}",
                "-x",
            ],
            "Download has started",
        );

        out
    }
}

impl CommandsConfig {
    fn blank() -> Self {
        Self(HashMap::default())
    }

    fn insert(&mut self, label: &str, command: Vec<&str>, msg: &str) {
        self.0.insert(
            label.to_string(),
            Command {
                args: command
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>(),
                message: msg.to_string(),
            },
        );
    }
}

impl ConfigItem<'_> for CommandsConfig {
    type Struct = CommandsConfig;
    const FILE_NAME: &'static str = "commands.yml";
}
enum ArgType<'a> {
    Normal(String),
    Variable(&'a str),
}

fn check_arg(s: &str) -> ArgType {
    if s.starts_with('{') && s.ends_with('}') {
        ArgType::Variable(&s[1..s.len() - 1])
    } else {
        ArgType::Normal(s.to_string())
    }
}
