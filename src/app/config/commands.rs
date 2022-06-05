use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use serde::{Deserialize, Serialize};

use crate::functions::insert_vec;

use super::{Config, EnvVar};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandsConfigTransitional {
    video_player: Option<CommandConfig>,
    audio_player: Option<CommandConfig>,
    image_viewer: Option<CommandConfig>,
    video_downloader: Option<CommandConfig>,
    audio_downloader: Option<CommandConfig>,
    terminal: Option<CommandConfig>,
    playlist_audio_all: Option<CommandConfig>,
    playlist_video_all: Option<CommandConfig>,
    playlist_shuffle_audio_all: Option<CommandConfig>,
    download_all_audio: Option<CommandConfig>,
    download_all_video: Option<CommandConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandsConfig {
    pub video_player: CommandConfig,
    pub audio_player: CommandConfig,
    pub image_viewer: CommandConfig,
    pub video_downloader: CommandConfig,
    pub audio_downloader: CommandConfig,
    pub terminal: CommandConfig,
    pub playlist_audio_all: CommandConfig,
    pub playlist_video_all: CommandConfig,
    pub playlist_shuffle_audio_all: CommandConfig,
    pub download_all_audio: CommandConfig,
    pub download_all_video: CommandConfig,
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
            playlist_audio_all: CommandConfig {
                command: String::from("mpv"),
                open_in_console: true,
                args: vec![String::from("--no-video"), String::from("{url}")],
            },
            playlist_video_all: CommandConfig {
                command: String::from("mpv"),
                open_in_console: false,
                args: vec![String::from("{url}"), String::from("--no-terminal")],
            },
            playlist_shuffle_audio_all: CommandConfig {
                command: String::from("mpv"),
                open_in_console: true,
                args: vec![
                    String::from("--no-video"),
                    String::from("{url}"),
                    String::from("--shuffle"),
                ],
            },
            download_all_audio: CommandConfig {
                command: String::from("yt-dlp"),
                open_in_console: true,
                args: vec![
                    String::from("{url}"),
                    String::from("-o"),
                    String::from("{audio_save_location}"),
                    String::from("-x"),
                ],
            },
            download_all_video: CommandConfig {
                command: String::from("yt-dlp"),
                open_in_console: true,
                args: vec![
                    String::from("{url}"),
                    String::from("-o"),
                    String::from("{video_save_location}"),
                ],
            },
        }
    }
}

impl From<CommandsConfigTransitional> for CommandsConfig {
    fn from(original: CommandsConfigTransitional) -> CommandsConfig {
        let mut out = Self::default();

        if let Some(video_player) = original.video_player {
            out.video_player = video_player;
        }

        if let Some(audio_player) = original.audio_player {
            out.audio_player = audio_player;
        }

        if let Some(image_viewer) = original.image_viewer {
            out.image_viewer = image_viewer;
        }

        if let Some(video_downloader) = original.video_downloader {
            out.video_downloader = video_downloader;
        }

        if let Some(audio_downloader) = original.audio_downloader {
            out.audio_downloader = audio_downloader;
        }

        if let Some(terminal) = original.terminal {
            out.terminal = terminal;
        }

        out
    }
}

impl CommandsConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut config = home::home_dir().expect("Cannot get your home directory");
        let mut commands = CommandsConfig::default();
        config.push(".config");
        config.push("youtube-tui");
        config.push("commands.yml");

        if config.exists() {
            let content = fs::read_to_string(config.as_os_str())?;
            let commands_transitional: CommandsConfigTransitional = serde_yaml::from_str(&content)?;
            commands = commands_transitional.into();
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config.as_os_str())?;

        write!(file, "{}", serde_yaml::to_string(&commands)?)?;

        Ok(commands)
    }
}
