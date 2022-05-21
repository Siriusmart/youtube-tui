use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use super::{commands::CommandsConfig, MainConfig};

#[derive(Clone, Debug)]
pub struct Config {
    pub commands: CommandsConfig,
    pub main: MainConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            commands: CommandsConfig::default(),
            main: MainConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut commands = CommandsConfig::default();
        let mut main = MainConfig::default();

        let mut config = home::home_dir().expect("Cannot get your home directory");
        config.push(".config");
        config.push("youtube-tui");

        config.push("commands.yml");

        if config.exists() {
            let content = fs::read_to_string(config.as_os_str())?;
            commands = serde_yaml::from_str(&content)?;
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(config.as_os_str())?;

            write!(file, "{}", serde_yaml::to_string(&commands)?)?;
        }

        config.pop();
        config.push("main.yml");

        if config.exists() {
            let content = fs::read_to_string(config.as_os_str())?;
            main = serde_yaml::from_str(&content)?;
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(config.as_os_str())?;

            write!(file, "{}", serde_yaml::to_string(&main)?)?;
        }

        Ok(Self { main, commands })
    }
}

pub struct EnvVar {
    pub url: Option<String>,
}

impl Default for EnvVar {
    fn default() -> Self {
        Self { url: None }
    }
}
