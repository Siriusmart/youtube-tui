use crate::global::{structs::KeyAction, traits::ConfigTrait};
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

// The outer HashMap<KeyCode, T> gets a value for whatever code is being pressed
// The inner HashMap<u8, KeyAction> gets the action using a u8 representing a key modifier
// The u8 for each modifier key can be found in https://docs.rs/crossterm/latest/src/crossterm/event.rs.html#587-603
// The sum of u8 for each key modifier would mean having all of those modifiers at the same time
/// `keybindings.yml`, contains a hashmap of key + key modifier to action pair
#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBindingsConfig(pub HashMap<KeyCode, HashMap<u8, KeyAction>>);

impl ConfigTrait for KeyBindingsConfig {
    const LABEL: &'static str = "keybindings";
}

impl Key for KeyBindingsConfig {
    type Value = Self;
}

impl Default for KeyBindingsConfig {
    fn default() -> Self {
        Self(HashMap::from([
            // movement keys                                            // Alt left should be back
            (
                KeyCode::Left,
                HashMap::from([(0, KeyAction::MoveLeft), (4, KeyAction::Back)]),
            ),
            (KeyCode::Right, HashMap::from([(0, KeyAction::MoveRight)])),
            (KeyCode::Up, HashMap::from([(0, KeyAction::MoveUp)])),
            (KeyCode::Down, HashMap::from([(0, KeyAction::MoveDown)])),
            // vim keybindings
            (
                KeyCode::Char('h'),
                HashMap::from([(0, KeyAction::MoveLeft)]),
            ),
            (
                KeyCode::Char('j'),
                HashMap::from([(0, KeyAction::MoveDown)]),
            ),
            (KeyCode::Char('k'), HashMap::from([(0, KeyAction::MoveUp)])),
            (
                KeyCode::Char('l'),
                HashMap::from([(0, KeyAction::MoveRight)]),
            ),
            // functional keys
            (KeyCode::Enter, HashMap::from([(0, KeyAction::Select)])),
            (KeyCode::Esc, HashMap::from([(0, KeyAction::Deselect)])),
            (KeyCode::Char('q'), HashMap::from([(0, KeyAction::Exit)])),
            (KeyCode::Char('r'), HashMap::from([(2, KeyAction::Reload)])),
            // history
            (KeyCode::Backspace, HashMap::from([(0, KeyAction::Back)])),
            (KeyCode::End, HashMap::from([(0, KeyAction::ClearHistory)])),
            (KeyCode::Home, HashMap::from([(0, KeyAction::FirstHistory)])),
        ]))
    }
}
