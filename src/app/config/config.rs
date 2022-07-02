use std::error::Error;

use super::{
    commands::CommandsConfig, KeybindingsConfig, LayoutsConfigs, MainConfig, PageCommandsConfig,
};
use crate::traits::ConfigItem;

#[derive(Clone)]
pub struct Config {
    pub commands: CommandsConfig,
    pub main: MainConfig,
    pub page_commands: PageCommandsConfig,
    pub keybindings: KeybindingsConfig,
    pub layouts: LayoutsConfigs,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            commands: CommandsConfig::default(),
            main: MainConfig::default(),
            page_commands: PageCommandsConfig::default(),
            keybindings: KeybindingsConfig::default(),
            layouts: LayoutsConfigs::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            main: MainConfig::load()?,
            commands: CommandsConfig::load()?,
            page_commands: PageCommandsConfig::load()?,
            keybindings: KeybindingsConfig::load()?.into(),
            layouts: LayoutsConfigs::load()?,
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
