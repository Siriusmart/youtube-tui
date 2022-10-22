//! Structs that impl `Into<T>` because `T` does not impl Serde but is used in config files

use serde::{Deserialize, Serialize};
use tui::widgets::BorderType;

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
