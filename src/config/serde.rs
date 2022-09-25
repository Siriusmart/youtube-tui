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

impl Into<BorderType> for BorderTypeSerde {
    fn into(self) -> BorderType {
        match self {
            Self::Plain => BorderType::Plain,
            Self::Rounded => BorderType::Rounded,
            Self::Thick => BorderType::Thick,
            Self::Double => BorderType::Double,
        }
    }
}
