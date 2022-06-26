use std::collections::HashMap;

use crossterm::event::KeyCode;
use serde::{Serialize, Deserialize};

use crate::traits::ConfigItem;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Select,
    Deselect,
    FirstItem,
    LastItem,
    Home,
    Exit,
    Back,
    ClearHistory,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyCodeTransitional {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

impl Into<KeyCode> for KeyCodeTransitional {
    fn into(self) -> KeyCode {
        match self {
            Self::Backspace => KeyCode::Backspace,
            Self::Enter => KeyCode::Enter,
            Self::Left => KeyCode::Left,
            Self::Right => KeyCode::Right,
            Self::Up => KeyCode::Up,
            Self::Down => KeyCode::Down,
            Self::Home => KeyCode::Home,
            Self::End => KeyCode::End,
            Self::PageUp => KeyCode::PageUp,
            Self::PageDown => KeyCode::PageDown,
            Self::Tab => KeyCode::Tab,
            Self::BackTab => KeyCode::BackTab,
            Self::Delete => KeyCode::Delete,
            Self::Insert => KeyCode::Insert,
            Self::F(n) => KeyCode::F(n),
            Self::Char(c) => KeyCode::Char(c),
            Self::Null => KeyCode::Null,
            Self::Esc => KeyCode::Esc,
        } 
    }
}

impl Into<KeybindingsConfig> for KeybindingsConfigTransitional {
    fn into(self) -> KeybindingsConfig {
        let mut out = HashMap::default();

        for (k, v) in self.0.into_iter() {
            out.insert(k.into(), v);
        }

        KeybindingsConfig(out)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeybindingsConfigTransitional (pub HashMap<KeyCodeTransitional, Action>);

impl Default for KeybindingsConfigTransitional {
    fn default() -> Self {
        let mut out = HashMap::default();

        out.insert(KeyCodeTransitional::Up, Action::Up);
        out.insert(KeyCodeTransitional::Down, Action::Down);
        out.insert(KeyCodeTransitional::Left, Action::Left);
        out.insert(KeyCodeTransitional::Right, Action::Right);

        out.insert(KeyCodeTransitional::Char('h'), Action::Left);
        out.insert(KeyCodeTransitional::Char('j'), Action::Down);
        out.insert(KeyCodeTransitional::Char('k'), Action::Up);
        out.insert(KeyCodeTransitional::Char('l'), Action::Right);

        out.insert(KeyCodeTransitional::PageUp, Action::FirstItem);
        out.insert(KeyCodeTransitional::PageDown, Action::LastItem);

        out.insert(KeyCodeTransitional::Enter, Action::Select);
        out.insert(KeyCodeTransitional::Esc, Action::Deselect);
        out.insert(KeyCodeTransitional::Char('q'), Action::Exit);
        
        out.insert(KeyCodeTransitional::Home, Action::Home);
        out.insert(KeyCodeTransitional::Backspace, Action::Back);
        out.insert(KeyCodeTransitional::End, Action::ClearHistory);

        Self(out)
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        KeybindingsConfigTransitional::default().into()
    }
}

#[derive(Debug, Clone)]
pub struct KeybindingsConfig (pub HashMap<KeyCode, Action>);


impl ConfigItem<'_>  for KeybindingsConfig {
    type Struct = KeybindingsConfigTransitional;
    const FILE_NAME: &'static str = "keybindings.yml";
}
