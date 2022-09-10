use crate::{
    config::{
        AppearanceConfig, AppearanceConfigSerde, ConfigTrait, KeyBindingsConfig, MainConfig,
        MinDimentions, PagesConfig, Search,
    },
    global::{invidiousclient::InvidiousClient, message::Message, page::Page, tasks::Tasks},
};
use home::home_dir;
use std::{error::Error, fs};
use tui_additions::framework::Framework;

// init tasks:
//  - create folders like `~/.config/youtube-tui/` and `~/.cache/youtube-tui/thumbnails/`
//  - load all config files
//  - insert data

pub fn init(framework: &mut Framework) -> Result<(), Box<dyn Error>> {
    // creating files
    let home_dir = home_dir().unwrap();
    let config_path = home_dir.join(".config/youtube-tui/");
    let cache_path = home_dir.join(".cache/youtube-tui/");

    if !&config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }

    if !&cache_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }

    // inserting data
    let main_config = *MainConfig::load()?;

    framework
        .data
        .global
        .insert::<InvidiousClient>(InvidiousClient::new(main_config.invidious_instance.clone()));
    framework
        .data
        .global
        .insert::<AppearanceConfig>(AppearanceConfig::from(*AppearanceConfigSerde::load()?));
    framework.data.global.insert::<MainConfig>(main_config);
    framework
        .data
        .global
        .insert::<PagesConfig>(*PagesConfig::load()?);
    framework
        .data
        .global
        .insert::<KeyBindingsConfig>(*KeyBindingsConfig::load()?);
    framework.data.global.insert::<Message>(Message::None);

    framework.data.state.insert::<Tasks>(Tasks::default());
    framework.data.state.insert::<Page>(Page::default());
    framework.data.state.insert::<Search>(*Search::load()?);
    framework
        .data
        .state
        .insert::<MinDimentions>(MinDimentions::default());

    Ok(())
}
