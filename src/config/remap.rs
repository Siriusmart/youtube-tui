use std::{
    collections::HashMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use home::home_dir;
use serde::{Deserialize, Serialize};
use typemap::Key;

use crate::global::traits::{ConfigTrait, EXTENSION};

use super::{serde::KeyCodeSerde, WriteConfig};

#[derive(Clone, Serialize, Deserialize)]
pub struct RemapConfigSerde(pub HashMap<KeyCodeSerde, HashMap<u8, RemapItemSerde>>);

#[derive(Clone, Serialize, Deserialize)]
pub struct RemapItemSerde {
    pub code: KeyCodeSerde,
    pub modifiers: u8,
}

#[derive(Clone)]
pub struct RemapItem {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl RemapItemSerde {
    pub fn try_into_remap(self) -> Option<RemapItem> {
        Some(RemapItem {
            code: self.code.to_keycode()?,
            modifiers: KeyModifiers::from_bits(self.modifiers)?,
        })
    }
}
impl RemapConfigSerde {
    /// converts KeyBindingsConfigSerde into KeyBindingsConfig
    pub fn into(self) -> Option<RemapConfig> {
        let mut out = HashMap::new();

        // simply loops over the serde hashmap and insert them into the new one
        for (remapserde, map) in self.0.into_iter() {
            let mut keyout = HashMap::new();
            for (modifier, item) in map.into_iter() {
                keyout.insert(modifier, item.try_into_remap()?);
            }
            out.insert(remapserde.to_keycode()?, keyout);
        }

        Some(RemapConfig(out))
    }
}

impl ConfigTrait for RemapConfigSerde {
    const LABEL: &'static str = "remaps";
}

impl Default for RemapConfigSerde {
    fn default() -> Self {
        Self(HashMap::from([(
            KeyCodeSerde::Char('a'),
            HashMap::from([(
                5,
                RemapItemSerde {
                    code: KeyCodeSerde::Char('b'),
                    modifiers: 5,
                },
            )]),
        )]))
    }
}

#[derive(Clone)]
pub struct RemapConfig(pub HashMap<KeyCode, HashMap<u8, RemapItem>>);

impl RemapConfig {
    pub fn load(write: WriteConfig) -> Result<Self, Box<dyn Error>> {
        let serde = *RemapConfigSerde::load(write)?;
        let try_into = serde.into();

        if let Some(keybindings) = try_into {
            return Ok(keybindings);
        }

        let config_path = home_dir().unwrap().join(format!(
            ".config/youtube-tui/{}.{}",
            RemapConfigSerde::LABEL,
            EXTENSION
        ));

        // if it cannot, back it up and regenerate it
        let mut new_path = config_path.clone();
        new_path.pop();
        new_path.push(format!(
            "{}-{}.{}",
            RemapConfigSerde::LABEL,
            chrono::offset::Local::now(),
            EXTENSION
        ));
        fs::rename(&config_path, &new_path)?;

        // here generates a default and write it to the file
        let default = RemapConfigSerde::default();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;

        file.write_all(serde_yaml::to_string(&default)?.as_bytes())?;

        match default.into() {
            Some(keybindings) => Ok(keybindings),
            None => panic!(
                "the default for {}.{} is invalid",
                RemapConfigSerde::LABEL,
                EXTENSION
            ),
        }
    }

    pub fn get(&self, key: &mut KeyEvent) {
        if let Some(remaps) = self.0.get(&key.code) {
            if let Some(remapped) = remaps.get(&key.modifiers.bits()) {
                key.code = remapped.code;
                key.modifiers = remapped.modifiers;
            }
        }
    }
}

impl Key for RemapConfig {
    type Value = Self;
}
