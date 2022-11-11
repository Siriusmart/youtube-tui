use crate::{
    config::{
        AppearanceConfig, AppearanceConfigSerde, CommandsConfig, CommandsConfigSerde,
        KeyBindingsConfig, MainConfig, MinDimentions, PagesConfig, Search,
    },
    global::{
        structs::{InvidiousClient, Message, Page, Status, Tasks, WatchHistory},
        traits::ConfigTrait,
    },
};
use home::home_dir;
use std::{error::Error, fs};
use tui_additions::framework::Framework;

/// app to run before the app starts
// init tasks:
//  - create folders like `~/.config/youtube-tui/` and `~/.cache/youtube-tui/thumbnails/`
//  - load all config files
//  - insert data
pub fn init(framework: &mut Framework) -> Result<(), Box<dyn Error>> {
    // creating files
    let home_dir = home_dir().unwrap();
    let config_path = home_dir.join(".config/youtube-tui/");
    let cache_thumbnails_path = home_dir.join(".cache/youtube-tui/thumbnails/");
    let history_thumbnails_path =
        home_dir.join(".local/share/youtube-tui/watch_history/thumbnails/");

    if !&config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }

    if !&cache_thumbnails_path.exists() {
        fs::create_dir_all(&cache_thumbnails_path).unwrap();
    }

    if !&history_thumbnails_path.exists() {
        fs::create_dir_all(&history_thumbnails_path).unwrap();
    }

    // watch history init
    WatchHistory::init_move();

    // inserting data
    let main_config = *MainConfig::load()?;

    framework
        .data
        .global
        .insert::<InvidiousClient>(InvidiousClient::new(main_config.invidious_instance.clone()));
    framework
        .data
        .global
        .insert::<WatchHistory>(WatchHistory::load());
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
    framework
        .data
        .global
        .insert::<CommandsConfig>(CommandsConfig::from(*CommandsConfigSerde::load()?));

    framework.data.state.insert::<Tasks>(Tasks::default());
    framework.data.state.insert::<Page>(Page::default());
    framework.data.state.insert::<Search>(*Search::load()?);
    framework
        .data
        .state
        .insert::<MinDimentions>(MinDimentions::default());
    framework.data.state.insert::<Status>(Status::default());

    Ok(())
}
