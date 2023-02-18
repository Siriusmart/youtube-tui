use crate::{
    config::{
        AppearanceConfig, CommandBindings, CommandBindingsSerde, CommandsConfig,
        CommandsConfigSerde, KeyBindingsConfig, MainConfig, MinDimentions, PagesConfig, Search,
    },
    global::{
        functions::{init_move, run_command},
        structs::{
            InvidiousClient, Library, Message, Page, StateEnvs, Status, Tasks, WatchHistory,
        },
        traits::{Collection, ConfigTrait},
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
    let home_dir = home_dir().unwrap();

    // creating files
    [
        ".cache/youtube-tui/thumbnails/",
        ".cache/youtube-tui/info/",
        ".local/share/youtube-tui/thumbnails/",
        ".local/share/youtube-tui/info/",
        ".local/share/youtube-tui/saved/",
    ]
    .into_iter()
    .for_each(|s| {
        let dir = home_dir.join(s);
        if !dir.exists() {
            fs::create_dir_all(dir).unwrap();
        }
    });

    init_move();

    load_configs(&mut framework.split_clean().0).ok();

    framework
        .data
        .global
        .insert::<WatchHistory>(WatchHistory(WatchHistory::load()));
    framework
        .data
        .global
        .insert::<Library>(Library(Library::load()));
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
