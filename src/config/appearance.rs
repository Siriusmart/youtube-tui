use super::serde::BorderTypeSerde;
use crate::global::traits::ConfigTrait;
use serde::{Deserialize, Serialize};
use tui::{style::Color, widgets::BorderType};
use typemap::Key;

/// `appearance.yml`, impl serde version of AppearanceConfig
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppearanceConfigSerde {
    #[serde(default)]
    pub borders: BorderTypeSerde,
    #[serde(default)]
    pub colors: ColorsConfig,
}

/// `appearance.yml`, this struct is stored in `data.global`
#[derive(Clone)]
pub struct AppearanceConfig {
    pub borders: BorderType,
    pub colors: ColorsConfig,
}

/// Includes all configurable colors
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ColorsConfig {
    #[serde(default = "text_default")]
    pub text: Color,
    #[serde(default = "text_special_default")]
    pub text_special: Color,
    #[serde(default = "text_secondary_default")]
    pub text_secondary: Color,
    #[serde(default = "text_error_default")]
    pub text_error: Color,
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
    #[serde(default = "message_success_outline_default")]
    pub message_success_outline: Color,
    #[serde(default)]
    pub item_info: ItemInfoColors,
}

/// Colors used by ItemInfo
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
    #[serde(default = "video_count_default")]
    pub video_count: Color,
    #[serde(default = "sub_count_default")]
    pub sub_count: Color,
    #[serde(default = "likes_default")]
    pub likes: Color,
    #[serde(default = "genre_default")]
    pub genre: Color,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            text: text_default(),
            text_special: text_special_default(),
            text_secondary: text_secondary_default(),
            outline: outline_default(),
            outline_selected: outline_selected_default(),
            outline_hover: outline_hover_default(),
            outline_secondary: outline_secondary_default(),
            message_outline: message_outline_default(),
            message_error_outline: message_error_outline_default(),
            message_success_outline: message_success_outline_default(),
            text_error: text_error_default(),
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
            video_count: video_count_default(),
            sub_count: sub_count_default(),
            likes: likes_default(),
            genre: genre_default(),
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

fn text_default() -> Color {
    Color::Reset
}

fn text_special_default() -> Color {
    Color::Reset
}

fn text_secondary_default() -> Color {
    Color::Reset
}

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

fn message_success_outline_default() -> Color {
    Color::LightGreen
}

fn text_error_default() -> Color {
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

fn video_count_default() -> Color {
    Color::Rgb(131, 141, 255)
}

fn sub_count_default() -> Color {
    Color::LightYellow
}

fn likes_default() -> Color {
    Color::Rgb(200, 255, 129)
}

fn genre_default() -> Color {
    Color::Rgb(255, 121, 215)
}
