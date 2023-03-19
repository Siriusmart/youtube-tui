use crate::{config::serde::*, global::traits::*};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};
use tui::{style::Color, widgets::BorderType};
use typemap::Key;

/// `appearance.yml`, impl serde version of AppearanceConfig
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AppearanceConfigSerde {
    #[serde(default)]
    pub borders: BorderTypeSerde,
    #[serde(default)]
    pub colors: ColorsConfigSerde,
}

/// `appearance.yml`, this struct is stored in `data.global`
#[derive(Clone)]
pub struct AppearanceConfig {
    pub borders: BorderType,
    pub colors: ColorsConfig,
}

impl AppearanceConfig {
    /// generates a new file if the original one is invalid
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let serde = *AppearanceConfigSerde::load()?;
        let try_into = Self::from(serde);

        // check if it be converted from `AppearanceConfigSerde` to `AppearanceConfig`
        if let Some(appearance) = try_into {
            return Ok(appearance);
        }

        let config_path = home_dir().unwrap().join(format!(
            ".config/youtube-tui/{}.{}",
            AppearanceConfigSerde::LABEL,
            EXTENSION
        ));

        // if it cannot, back it up and regenerate it
        let mut new_path = config_path.clone();
        new_path.pop();
        new_path.push(format!(
            "{}-{}.{}",
            AppearanceConfigSerde::LABEL,
            chrono::offset::Local::now(),
            EXTENSION
        ));
        fs::rename(&config_path, &new_path)?;

        // here generates a default and write it to the file
        let default = AppearanceConfigSerde::default();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;

        file.write_all(serde_yaml::to_string(&default)?.as_bytes())?;

        match Self::from(default) {
            Some(appearance) => Ok(appearance),
            // if the default still cannot be converted, panic and exit the program
            None => panic!(
                "the default for {}.{} is invalid",
                AppearanceConfigSerde::LABEL,
                EXTENSION
            ),
        }
    }
}

/// Includes all configurable colors
#[derive(Clone, Copy)]
pub struct ColorsConfig {
    pub text: Color,
    pub text_special: Color,
    pub text_secondary: Color,
    pub text_error: Color,
    pub outline: Color,
    pub outline_selected: Color,
    pub outline_hover: Color,
    pub outline_secondary: Color,
    pub message_outline: Color,
    pub message_error_outline: Color,
    pub message_success_outline: Color,
    pub command_capture: Color,
    pub item_info: ItemInfoColors,
}

/// Colors used by ItemInfo
#[derive(Clone, Copy)]
pub struct ItemInfoColors {
    pub tag: Color,
    pub title: Color,
    pub description: Color,
    pub author: Color,
    pub viewcount: Color,
    pub length: Color,
    pub published: Color,
    pub video_count: Color,
    pub sub_count: Color,
    pub likes: Color,
    pub genre: Color,
    pub page_turner: Color,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorsConfigSerde {
    #[serde(default = "text_default")]
    pub text: ColorSerde,
    #[serde(default = "text_special_default")]
    pub text_special: ColorSerde,
    #[serde(default = "text_secondary_default")]
    pub text_secondary: ColorSerde,
    #[serde(default = "text_error_default")]
    pub text_error: ColorSerde,
    #[serde(default = "outline_default")]
    pub outline: ColorSerde,
    #[serde(default = "outline_selected_default")]
    pub outline_selected: ColorSerde,
    #[serde(default = "outline_hover_default")]
    pub outline_hover: ColorSerde,
    #[serde(default = "outline_secondary_default")]
    pub outline_secondary: ColorSerde,
    #[serde(default = "message_outline_default")]
    pub message_outline: ColorSerde,
    #[serde(default = "message_error_outline_default")]
    pub message_error_outline: ColorSerde,
    #[serde(default = "message_success_outline_default")]
    pub message_success_outline: ColorSerde,
    #[serde(default = "command_capture_default")]
    pub command_capture: ColorSerde,
    #[serde(default)]
    pub item_info: ItemInfoColorsSerde,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemInfoColorsSerde {
    #[serde(default = "tag_default")]
    pub tag: ColorSerde,
    #[serde(default = "title_default")]
    pub title: ColorSerde,
    #[serde(default = "description_default")]
    pub description: ColorSerde,
    #[serde(default = "author_default")]
    pub author: ColorSerde,
    #[serde(default = "viewcount_default")]
    pub viewcount: ColorSerde,
    #[serde(default = "length_default")]
    pub length: ColorSerde,
    #[serde(default = "published_default")]
    pub published: ColorSerde,
    #[serde(default = "video_count_default")]
    pub video_count: ColorSerde,
    #[serde(default = "sub_count_default")]
    pub sub_count: ColorSerde,
    #[serde(default = "likes_default")]
    pub likes: ColorSerde,
    #[serde(default = "genre_default")]
    pub genre: ColorSerde,
    #[serde(default = "page_turner_default")]
    pub page_turner: ColorSerde,
}

// uses a custom `into` for Option<T> instead of T so that we can know that the config is invalid
// without panicking
impl ColorsConfigSerde {
    pub fn into(self) -> Option<ColorsConfig> {
        Some(ColorsConfig {
            text: self.text.to_color()?,
            text_special: self.text_special.to_color()?,
            text_secondary: self.text_secondary.to_color()?,
            text_error: self.text_error.to_color()?,
            outline: self.outline.to_color()?,
            outline_selected: self.outline_selected.to_color()?,
            outline_hover: self.outline_hover.to_color()?,
            outline_secondary: self.outline_secondary.to_color()?,
            message_outline: self.message_outline.to_color()?,
            message_error_outline: self.message_error_outline.to_color()?,
            message_success_outline: self.message_success_outline.to_color()?,
            command_capture: self.command_capture.to_color()?,
            item_info: self.item_info.into()?,
        })
    }
}

impl ItemInfoColorsSerde {
    pub fn into(self) -> Option<ItemInfoColors> {
        Some(ItemInfoColors {
            tag: self.tag.to_color()?,
            title: self.title.to_color()?,
            description: self.description.to_color()?,
            author: self.author.to_color()?,
            viewcount: self.viewcount.to_color()?,
            length: self.length.to_color()?,
            published: self.published.to_color()?,
            video_count: self.video_count.to_color()?,
            sub_count: self.sub_count.to_color()?,
            likes: self.likes.to_color()?,
            genre: self.genre.to_color()?,
            page_turner: self.page_turner.to_color()?,
        })
    }
}

impl Default for ColorsConfigSerde {
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
            command_capture: command_capture_default(),
            item_info: ItemInfoColorsSerde::default(),
        }
    }
}

impl Default for ItemInfoColorsSerde {
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
            page_turner: page_turner_default(),
        }
    }
}

impl AppearanceConfig {
    fn from(original: AppearanceConfigSerde) -> Option<Self> {
        Some(Self {
            borders: original.borders.into(),
            colors: original.colors.into()?,
        })
    }
}

impl ConfigTrait for AppearanceConfigSerde {
    const LABEL: &'static str = "appearance";
}

impl Key for AppearanceConfig {
    type Value = Self;
}

// defaults

fn text_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Reset)
}

fn text_special_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Reset)
}

fn text_secondary_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Reset)
}

fn outline_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Reset)
}

fn outline_selected_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightBlue)
}

fn outline_hover_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightRed)
}

fn outline_secondary_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightYellow)
}

fn message_outline_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#FF7F00"))
}

fn message_error_outline_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightRed)
}

fn message_success_outline_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightGreen)
}

fn text_error_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightRed)
}

fn tag_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Gray)
}

fn title_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightBlue)
}

fn description_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Gray)
}

fn author_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightGreen)
}

fn viewcount_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightYellow)
}

fn length_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightCyan)
}

fn published_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::LightMagenta)
}

fn video_count_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#838DFF"))
}

fn sub_count_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#65FFBA"))
}

fn likes_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#C8FF81"))
}

fn genre_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#FF75D7"))
}

fn page_turner_default() -> ColorSerde {
    ColorSerde::ColorVariant(ColorVariantSerde::Gray)
}

fn command_capture_default() -> ColorSerde {
    ColorSerde::Hex(String::from("#64FF64"))
}
