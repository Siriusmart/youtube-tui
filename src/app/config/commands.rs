use serde::{Deserialize, Serialize};

use crate::functions::insert_vec;

use super::{Config, EnvVar};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandsConfig {
    pub video_player: CommandConfig,
    pub audio_player: CommandConfig,
    pub image_viewer: CommandConfig,
    pub video_downloader: CommandConfig,
    pub audio_downloader: CommandConfig,
    pub terminal: CommandConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub command: String,
    pub open_in_console: bool,
    pub args: Vec<String>,
}

impl CommandConfig {
    pub fn as_vec(&self) -> Vec<String> {
        let mut args = self.args.clone();
        args.insert(0, self.command.clone());
        args
    }

    pub fn as_command_vec(self, env: EnvVar, config: &Config) -> Result<Vec<String>, ()> {
        let mut out;
        if self.open_in_console {
            out = config.commands.terminal.as_vec();
            let index = match out.iter().position(|x| x == "{command}") {
                Some(index) => {
                    out.swap_remove(index);
                    index
                }
                None => out.len(),
            };

            insert_vec(&mut out, self.as_vec(), index);
        } else {
            out = self.as_vec();
        }

        for x in out.iter_mut() {
            match x.as_str() {
                "{url}" => {
                    if let Some(url) = &env.url {
                        *x = url.clone();
                    } else {
                        return Err(());
                    }
                }

                "{video_save_location}" => {
                    *x = config.main.yt_dl.video_path.clone();
                }

                "{audio_save_location}" => {
                    *x = config.main.yt_dl.audio_path.clone();
                }

                _ => {}
            }
        }

        Ok(out)
    }
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            video_player: CommandConfig {
                command: String::from("mpv"),
                open_in_console: false,
                args: vec![String::from("--no-terminal"), String::from("{url}")],
            },
            audio_player: CommandConfig {
                command: String::from("mpv"),
                open_in_console: true,
                args: vec![String::from("--no-video"), String::from("{url}")],
            },
            image_viewer: CommandConfig {
                command: String::from("mpv"),
                open_in_console: false,
                args: vec![String::from("{url}"), String::from("--no-terminal")],
            },
            video_downloader: CommandConfig {
                command: String::from("yt-dlp"),
                open_in_console: true,
                args: vec![
                    String::from("{url}"),
                    String::from("-o"),
                    String::from("{video_save_location}"),
                ],
            },
            audio_downloader: CommandConfig {
                command: String::from("yt-dlp"),
                open_in_console: true,
                args: vec![
                    String::from("{url}"),
                    String::from("-o"),
                    String::from("{audio_save_location}"),
                    String::from("-x"),
                ],
            },
            terminal: CommandConfig {
                command: String::from("konsole"),
                open_in_console: false,
                args: vec![String::from("-e"), String::from("{command}")],
            },
        }
    }
}
