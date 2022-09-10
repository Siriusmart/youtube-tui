use super::{serde::BorderTypeSerde, ConfigTrait};
use serde::{Deserialize, Serialize};
use tui::{style::Color, widgets::BorderType};
use typemap::Key;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppearanceConfigSerde {
    #[serde(default)]
    pub borders: BorderTypeSerde,
    #[serde(default)]
    pub colors: ColorsConfig,
}

#[derive(Clone)]
pub struct AppearanceConfig {
    pub borders: BorderType,
    pub colors: ColorsConfig,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ColorsConfig {
    #[serde(default = "outline_default")]
    pub outline: Color,
    #[serde(default = "outline_selected_default")]
    pub outline_selected: Color,
    #[serde(default = "outline_hover_default")]
    pub outline_hover: Color,
    #[serde(default = "outline_secondary_default")]
    pub outline_secondary: Color,
    #[serde(default = "message_outline_default")]
    pub message_outline: Color,
    #[serde(default = "message_error_outline_default")]
    pub message_error_outline: Color,
    #[serde(default = "error_text_default")]
    pub error_text: Color,
    #[serde(default)]
    pub item_info: ItemInfoColors,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ItemInfoColors {
    #[serde(default = "tag_default")]
    pub tag: Color,
    #[serde(default = "title_default")]
    pub title: Color,
    #[serde(default = "description_default")]
    pub description: Color,
    #[serde(default = "author_default")]
    pub author: Color,
    #[serde(default = "viewcount_default")]
    pub viewcount: Color,
    #[serde(default = "length_default")]
    pub length: Color,
    #[serde(default = "published_default")]
    pub published: Color,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            outline: outline_default(),
            outline_selected: outline_selected_default(),
            outline_hover: outline_hover_default(),
            outline_secondary: outline_secondary_default(),
            message_outline: message_outline_default(),
            message_error_outline: message_error_outline_default(),
            error_text: error_text_default(),
            item_info: ItemInfoColors::default(),
        }
    }
}

impl Default for ItemInfoColors {
    fn default() -> Self {
        Self {
            tag: tag_default(),
            title: title_default(),
            description: description_default(),
            author: author_default(),
            viewcount: viewcount_default(),
            length: length_default(),
            published: published_default(),
        }
    }
}

impl From<AppearanceConfigSerde> for AppearanceConfig {
    fn from(original: AppearanceConfigSerde) -> Self {
        Self {
            borders: original.borders.into(),
            colors: original.colors,
        }
    }
}

impl ConfigTrait for AppearanceConfigSerde {
    const LABEL: &'static str = "appearance";
}

impl Key for AppearanceConfig {
    type Value = Self;
}

// defaults

fn outline_default() -> Color {
    Color::Reset
}

fn outline_selected_default() -> Color {
    Color::LightBlue
}

fn outline_hover_default() -> Color {
    Color::LightRed
}

fn outline_secondary_default() -> Color {
    Color::LightYellow
}

fn message_outline_default() -> Color {
    Color::Rgb(255, 127, 0)
}

fn message_error_outline_default() -> Color {
    Color::LightRed
}

fn error_text_default() -> Color {
    Color::LightRed
}

fn tag_default() -> Color {
    Color::Gray
}

fn title_default() -> Color {
    Color::LightBlue
}

fn description_default() -> Color {
    Color::Gray
}

fn author_default() -> Color {
    Color::LightGreen
}

fn viewcount_default() -> Color {
    Color::LightYellow
}

fn length_default() -> Color {
    Color::LightCyan
}

fn published_default() -> Color {
    Color::LightMagenta
}
