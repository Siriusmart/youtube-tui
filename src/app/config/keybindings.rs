use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

use crate::traits::ConfigItem;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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
    Refresh,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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

        for ((modifiers, code), v) in self.0.into_iter() {
            out.insert(
                KeyEvent {
                    code: code.into(),
                    modifiers: modifiers.into(),
                },
                v,
            );
        }

        KeybindingsConfig(out)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyModifierTransitional {
    Shift,
    Control,
    Alt,
    r#None,
}

impl Into<KeyModifiers> for KeyModifierTransitional {
    fn into(self) -> KeyModifiers {
        match self {
            Self::Shift => KeyModifiers::SHIFT,
            Self::Control => KeyModifiers::CONTROL,
            Self::Alt => KeyModifiers::ALT,
            Self::None => KeyModifiers::NONE,
        }
    }
}

impl Into<KeyModifiersTransitional> for KeyModifierTransitional {
    fn into(self) -> KeyModifiersTransitional {
        KeyModifiersTransitional(vec![self])
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModifiersTransitional(Vec<KeyModifierTransitional>);

impl Into<KeyModifiers> for KeyModifiersTransitional {
    fn into(self) -> KeyModifiers {
        KeyModifiers::from_bits(
            self.0
                .into_iter()
                .map(|item| Into::<KeyModifiers>::into(item).bits())
                .sum(),
        )
        .unwrap()
    }
}

impl Default for KeyModifiersTransitional {
    fn default() -> Self {
        Self(vec![KeyModifierTransitional::None])
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeybindingsConfigTransitional(
    pub HashMap<(KeyModifiersTransitional, KeyCodeTransitional), Action>,
);

impl Default for KeybindingsConfigTransitional {
    fn default() -> Self {
        let mut out = HashMap::default();

        out.insert(
            (KeyModifiersTransitional::default(), KeyCodeTransitional::Up),
            Action::Up,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Down,
            ),
            Action::Down,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Left,
            ),
            Action::Left,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Right,
            ),
            Action::Right,
        );

        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Char('h'),
            ),
            Action::Left,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Char('j'),
            ),
            Action::Down,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Char('k'),
            ),
            Action::Up,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Char('l'),
            ),
            Action::Right,
        );

        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::PageUp,
            ),
            Action::FirstItem,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::PageDown,
            ),
            Action::LastItem,
        );

        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Enter,
            ),
            Action::Select,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Esc,
            ),
            Action::Deselect,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Char('q'),
            ),
            Action::Exit,
        );

        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Home,
            ),
            Action::Home,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::Backspace,
            ),
            Action::Back,
        );
        out.insert(
            (
                KeyModifierTransitional::Alt.into(),
                KeyCodeTransitional::Left,
            ),
            Action::Back,
        );
        out.insert(
            (
                KeyModifiersTransitional::default(),
                KeyCodeTransitional::End,
            ),
            Action::ClearHistory,
        );
        out.insert(
            (
                KeyModifierTransitional::Control.into(),
                KeyCodeTransitional::Char('r'),
            ),
            Action::Refresh,
        );

        Self(out)
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        KeybindingsConfigTransitional::default().into()
    }
}

#[derive(Clone)]
pub struct KeybindingsConfig(pub HashMap<KeyEvent, Action>);

impl ConfigItem<'_> for KeybindingsConfig {
    type Struct = KeybindingsConfigTransitional;
    const FILE_NAME: &'static str = "keybindings.yml";
}
