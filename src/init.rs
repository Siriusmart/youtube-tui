use crate::{
    config::{
        AppearanceConfig, CommandBindings, CommandBindingsSerde, CommandsConfig,
        CommandsConfigSerde, KeyBindingsConfig, MainConfig, MinDimentions, PagesConfig, Search,
    },
    global::{
        functions::run_command,
        structs::{InvidiousClient, Message, Page, StateEnvs, Status, Tasks, WatchHistory},
        traits::ConfigTrait,
    },
};
use home::home_dir;
use std::{error::Error, fs, io::Stdout};
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::{Framework, FrameworkClean};

/// app to run before the app starts
// init tasks:
//  - create folders like `~/.config/youtube-tui/` and `~/.cache/youtube-tui/thumbnails/`
//  - load all config files
//  - insert data
pub fn init(
    framework: &mut Framework,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    command: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // creating files
    let home_dir = home_dir().unwrap();
    let cache_thumbnails_path = home_dir.join(".cache/youtube-tui/thumbnails/");
    let history_thumbnails_path =
        home_dir.join(".local/share/youtube-tui/watch_history/thumbnails/");

    if !&cache_thumbnails_path.exists() {
        fs::create_dir_all(&cache_thumbnails_path).unwrap();
    }

    if !&history_thumbnails_path.exists() {
        fs::create_dir_all(&history_thumbnails_path).unwrap();
    }

    // watch history init
    WatchHistory::init_move();

    load_configs(&mut framework.split_clean().0).ok();

    framework.data.global.insert::<Message>(Message::None);
    framework.data.global.insert::<Status>(Status {
        provider: framework.data.global.get::<MainConfig>().unwrap().provider,
        ..Status::default()
    });

    framework.data.state.insert::<Tasks>(Tasks::default());
    framework.data.state.insert::<Page>(Page::default());
    framework
        .data
        .state
        .insert::<MinDimentions>(MinDimentions::default());
    framework
        .data
        .state
        .insert::<StateEnvs>(StateEnvs::default());

    run_command(
        command.unwrap_or(
            &framework
                .data
                .global
                .get::<CommandsConfig>()
                .unwrap()
                .launch_command
                .clone(),
        ),
        framework,
        terminal,
    );
    Ok(())
}

/// reload all config files
pub fn load_configs(framework: &mut FrameworkClean) -> Result<(), Box<dyn Error>> {
    let home_dir = home_dir().unwrap();
    let config_path = home_dir.join(".config/youtube-tui/");

    if !&config_path.exists() {
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
        .insert::<CommandsConfig>(CommandsConfig::from(*CommandsConfigSerde::load()?));
    framework
        .data
        .global
        .insert::<WatchHistory>(WatchHistory::load());
    framework
        .data
        .global
        .insert::<AppearanceConfig>(AppearanceConfig::load()?);
    framework.data.global.insert::<MainConfig>(main_config);
    framework
        .data
        .global
        .insert::<PagesConfig>(*PagesConfig::load()?);
    framework
        .data
        .global
        .insert::<KeyBindingsConfig>(KeyBindingsConfig::load()?);
    framework
        .data
        .global
        .insert::<CommandBindings>((*CommandBindingsSerde::load()?).into().unwrap());
    framework.data.state.insert::<Search>(*Search::load()?);

    Ok(())
}
