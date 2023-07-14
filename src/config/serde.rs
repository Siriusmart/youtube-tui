//! Structs that impl `Into<T>` because `T` does not impl Serde but is used in config files
use crossterm::event::KeyCode;
use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Serialize};

/// `BorderType` but impl `serde`
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BorderTypeSerde {
    Plain,
    Rounded,
    Double,
    Thick,
}

impl Default for BorderTypeSerde {
    fn default() -> Self {
        Self::Rounded
    }
}

impl From<BorderTypeSerde> for BorderType {
    fn from(origianl: BorderTypeSerde) -> BorderType {
        match origianl {
            BorderTypeSerde::Plain => BorderType::Plain,
            BorderTypeSerde::Rounded => BorderType::Rounded,
            BorderTypeSerde::Thick => BorderType::Thick,
            BorderTypeSerde::Double => BorderType::Double,
        }
    }
}

/// all terminal colors
#[derive(Serialize, Deserialize, Clone)]
pub enum ColorVariantSerde {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

/// a single color
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ColorSerde {
    ColorVariant(ColorVariantSerde),
    Hex(String),
}

impl ColorSerde {
    pub fn to_color(self) -> Option<Color> {
        match self {
            Self::ColorVariant(ColorVariantSerde::Reset) => Some(Color::Reset),
            Self::ColorVariant(ColorVariantSerde::Black) => Some(Color::Black),
            Self::ColorVariant(ColorVariantSerde::Red) => Some(Color::Red),
            Self::ColorVariant(ColorVariantSerde::Green) => Some(Color::Green),
            Self::ColorVariant(ColorVariantSerde::Yellow) => Some(Color::Yellow),
            Self::ColorVariant(ColorVariantSerde::Blue) => Some(Color::Blue),
            Self::ColorVariant(ColorVariantSerde::Magenta) => Some(Color::Magenta),
            Self::ColorVariant(ColorVariantSerde::Cyan) => Some(Color::Cyan),
            Self::ColorVariant(ColorVariantSerde::Gray) => Some(Color::Gray),
            Self::ColorVariant(ColorVariantSerde::DarkGray) => Some(Color::DarkGray),
            Self::ColorVariant(ColorVariantSerde::LightRed) => Some(Color::LightRed),
            Self::ColorVariant(ColorVariantSerde::LightGreen) => Some(Color::LightGreen),
            Self::ColorVariant(ColorVariantSerde::LightYellow) => Some(Color::LightYellow),
            Self::ColorVariant(ColorVariantSerde::LightBlue) => Some(Color::LightBlue),
            Self::ColorVariant(ColorVariantSerde::LightMagenta) => Some(Color::LightMagenta),
            Self::ColorVariant(ColorVariantSerde::LightCyan) => Some(Color::LightCyan),
            Self::ColorVariant(ColorVariantSerde::White) => Some(Color::White),
            Self::Hex(s) => {
                if s.len() != 7 || &s[0..1] != "#" {
                    return None;
                }

                Some(Color::Rgb(
                    from_hex(&s[1..3])?,
                    from_hex(&s[3..5])?,
                    from_hex(&s[5..7])?,
                ))
            }
        }
    }
}

/// converts a 2 digit hex number (00 - FF) to u8
fn from_hex(s: &str) -> Option<u8> {
    Some(from_hex_digit(&s[0..1])? * 16 + from_hex_digit(&s[1..2])?)
}

/// converts a single hex digit to u8
fn from_hex_digit(d: &str) -> Option<u8> {
    match d.to_uppercase().as_str() {
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        "A" => Some(10),
        "B" => Some(11),
        "C" => Some(12),
        "D" => Some(13),
        "E" => Some(14),
        "F" => Some(15),
        _ => None,
    }
}

/// serde version of a single key code
#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum KeyCodeSerde {
    Char(char),
    KeyVariants(KeyVariantsSerde),
    F(String),
}

/// values of all default keys (excluding chars and function keys)
#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum KeyVariantsSerde {
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
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
}

impl KeyCodeSerde {
    pub fn to_keycode(self) -> Option<KeyCode> {
        match self {
            Self::KeyVariants(KeyVariantsSerde::Backspace) => Some(KeyCode::Backspace),
            Self::KeyVariants(KeyVariantsSerde::Enter) => Some(KeyCode::Enter),
            Self::KeyVariants(KeyVariantsSerde::Left) => Some(KeyCode::Left),
            Self::KeyVariants(KeyVariantsSerde::Right) => Some(KeyCode::Right),
            Self::KeyVariants(KeyVariantsSerde::Up) => Some(KeyCode::Up),
            Self::KeyVariants(KeyVariantsSerde::Down) => Some(KeyCode::Down),
            Self::KeyVariants(KeyVariantsSerde::Home) => Some(KeyCode::Home),
            Self::KeyVariants(KeyVariantsSerde::End) => Some(KeyCode::End),
            Self::KeyVariants(KeyVariantsSerde::PageUp) => Some(KeyCode::PageUp),
            Self::KeyVariants(KeyVariantsSerde::PageDown) => Some(KeyCode::PageDown),
            Self::KeyVariants(KeyVariantsSerde::Tab) => Some(KeyCode::Tab),
            Self::KeyVariants(KeyVariantsSerde::BackTab) => Some(KeyCode::BackTab),
            Self::KeyVariants(KeyVariantsSerde::Delete) => Some(KeyCode::Delete),
            Self::KeyVariants(KeyVariantsSerde::Insert) => Some(KeyCode::Insert),
            Self::KeyVariants(KeyVariantsSerde::Null) => Some(KeyCode::Null),
            Self::KeyVariants(KeyVariantsSerde::Esc) => Some(KeyCode::Esc),
            Self::KeyVariants(KeyVariantsSerde::CapsLock) => Some(KeyCode::CapsLock),
            Self::KeyVariants(KeyVariantsSerde::ScrollLock) => Some(KeyCode::ScrollLock),
            Self::KeyVariants(KeyVariantsSerde::NumLock) => Some(KeyCode::NumLock),
            Self::KeyVariants(KeyVariantsSerde::PrintScreen) => Some(KeyCode::PrintScreen),
            Self::KeyVariants(KeyVariantsSerde::Pause) => Some(KeyCode::Pause),
            Self::KeyVariants(KeyVariantsSerde::Menu) => Some(KeyCode::Menu),
            Self::KeyVariants(KeyVariantsSerde::KeypadBegin) => Some(KeyCode::KeypadBegin),
            Self::Char(c) => Some(KeyCode::Char(c)),
            Self::F(s) => {
                if s[0..1].to_uppercase().as_str() != "F" {
                    return None;
                }

                let parsed = s[1..].parse::<u8>();

                match parsed {
                    Ok(f) => Some(KeyCode::F(f)),
                    Err(_) => None,
                }
            }
        }
    }
}
