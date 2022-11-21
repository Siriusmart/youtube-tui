use super::serde::{KeyCodeSerde, KeyVariantsSerde};
use crate::global::{
    structs::KeyAction,
    traits::{ConfigTrait, EXTENSION},
};
use crossterm::event::KeyCode;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};
use typemap::Key;

// The outer HashMap<KeyCode, T> gets a value for whatever code is being pressed
// The inner HashMap<u8, KeyAction> gets the action using a u8 representing a key modifier
// The u8 for each modifier key can be found in https://docs.rs/crossterm/latest/src/crossterm/event.rs.html#587-603
// The sum of u8 for each key modifier would mean having all of those modifiers at the same time
/// `keybindings.yml`, contains a hashmap of key + key modifier to action pair
#[derive(Clone)]
pub struct KeyBindingsConfig(pub HashMap<KeyCode, HashMap<u8, KeyAction>>);

impl Key for KeyBindingsConfig {
    type Value = Self;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBindingsConfigSerde(pub HashMap<KeyCodeSerde, HashMap<u8, KeyAction>>);

impl ConfigTrait for KeyBindingsConfigSerde {
    const LABEL: &'static str = "keybindings";
}

impl Default for KeyBindingsConfigSerde {
    fn default() -> Self {
        Self(HashMap::from([
            // movement keys                                            // Alt left should be back
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Left),
                HashMap::from([(0, KeyAction::MoveLeft), (4, KeyAction::Back)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Right),
                HashMap::from([(0, KeyAction::MoveRight)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Up),
                HashMap::from([(0, KeyAction::MoveUp)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Down),
                HashMap::from([(0, KeyAction::MoveDown)]),
            ),
            // vim keybindings
            (
                KeyCodeSerde::Char('h'),
                HashMap::from([(0, KeyAction::MoveLeft)]),
            ),
            (
                KeyCodeSerde::Char('j'),
                HashMap::from([(0, KeyAction::MoveDown)]),
            ),
            (
                KeyCodeSerde::Char('k'),
                HashMap::from([(0, KeyAction::MoveUp)]),
            ),
            (
                KeyCodeSerde::Char('l'),
                HashMap::from([(0, KeyAction::MoveRight)]),
            ),
            // functional keys
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Enter),
                HashMap::from([(0, KeyAction::Select)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Esc),
                HashMap::from([(0, KeyAction::Deselect)]),
            ),
            (
                KeyCodeSerde::F(String::from("F5")),
                HashMap::from([(0, KeyAction::Reload)]),
            ),
            (
                KeyCodeSerde::Char('q'),
                HashMap::from([(0, KeyAction::Exit)]),
            ),
            (
                KeyCodeSerde::Char('r'),
                HashMap::from([(2, KeyAction::Reload)]),
            ),
            // history
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Backspace),
                HashMap::from([(0, KeyAction::Back)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::End),
                HashMap::from([(0, KeyAction::ClearHistory)]),
            ),
            (
                KeyCodeSerde::KeyVariants(KeyVariantsSerde::Home),
                HashMap::from([(0, KeyAction::FirstHistory)]),
            ),
        ]))
    }
}

impl KeyBindingsConfigSerde {
    /// converts KeyBindingsConfigSerde into KeyBindingsConfig
    pub fn into(self) -> Option<KeyBindingsConfig> {
        let mut out = HashMap::new();

        // simply loops over the serde hashmap and insert them into the new one
        for (keycodeserde, map) in self.0.into_iter() {
            out.insert(keycodeserde.to_keycode()?, map);
        }

        Some(KeyBindingsConfig(out))
    }
}

impl KeyBindingsConfig {
    /// generates a new file if the original one is invalid
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let serde = *KeyBindingsConfigSerde::load()?;
        // check if it be converted from `KeyBindingsConfigSerde` to `KeyBindingsConfig`
        let try_into = serde.into();

        if let Some(keybindings) = try_into {
            return Ok(keybindings);
        }

        let config_path = home_dir().unwrap().join(format!(
            ".config/youtube-tui/{}.{}",
            KeyBindingsConfigSerde::LABEL,
            EXTENSION
        ));

        // if it cannot, back it up and regenerate it
        let mut new_path = config_path.clone();
        new_path.pop();
        new_path.push(format!(
            "{}-{}.{}",
            KeyBindingsConfigSerde::LABEL,
            chrono::offset::Local::now(),
            EXTENSION
        ));
        fs::rename(&config_path, &new_path)?;

        // here generates a default and write it to the file
        let default = KeyBindingsConfigSerde::default();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;

        file.write_all(serde_yaml::to_string(&default)?.as_bytes())?;

        match default.into() {
            Some(keybindings) => Ok(keybindings),
            // if the default still cannot be converted, panic and exit the program
            None => panic!(
                "the default for {}.{} is invalid",
                KeyBindingsConfigSerde::LABEL,
                EXTENSION
            ),
        }
    }
}
