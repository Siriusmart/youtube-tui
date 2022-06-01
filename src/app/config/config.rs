use std::error::Error;

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
        Ok(Self {
            main: MainConfig::load()?,
            commands: CommandsConfig::load()?,
        })
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
