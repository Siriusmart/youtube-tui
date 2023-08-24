use crate::{
    config::*,
    global::{functions::*, structs::*, traits::*},
};
use home::home_dir;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    error::Error,
    fs::{self},
    io::Stdout,
};
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

    load_configs(&mut framework.split_clean().0)?;

    framework
        .data
        .global
        .insert::<CommandHistory>(CommandHistory::load());
    framework
        .data
        .global
        .insert::<SearchHistory>(SearchHistory::load());
    framework
        .data
        .global
        .insert::<WatchHistory>(WatchHistory::load());
    framework
        .data
        .global
        .insert::<Subscriptions>(Subscriptions::load());
    framework.data.global.insert::<Library>(Library::load());
    framework.data.global.insert::<Message>(Message::None);

    framework.data.global.insert::<Status>(Status {
        provider: framework.data.global.get::<MainConfig>().unwrap().provider,
        ..Status::default()
    });

    #[cfg(feature = "mpv")]
    framework
        .data
        .global
        .insert::<MpvWrapper>(MpvWrapper::spawn());

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
    let main_config = *MainConfig::load(WriteConfig::Try)?;

    framework
        .data
        .global
        .insert::<InvidiousClient>(InvidiousClient::new(main_config.invidious_instance.clone()));
    framework
        .data
        .global
        .insert::<CommandsConfig>(CommandsConfig::from(*CommandsConfigSerde::load(
            main_config.write_config,
        )?));
    framework
        .data
        .global
        .insert::<AppearanceConfig>(AppearanceConfig::load(main_config.write_config)?);
    framework
        .data
        .global
        .insert::<PagesConfig>(*PagesConfig::load(main_config.write_config)?);
    framework
        .data
        .global
        .insert::<CommandsRemapConfig>(*CommandsRemapConfig::load(main_config.write_config)?);
    framework
        .data
        .global
        .insert::<KeyBindingsConfig>(KeyBindingsConfig::load(main_config.write_config)?);
    framework
        .data
        .global
        .insert::<RemapConfig>(RemapConfig::load(main_config.write_config)?);
    framework.data.global.insert::<CommandBindings>(
        (*CommandBindingsSerde::load(main_config.write_config)?)
            .into()
            .unwrap(),
    );
    framework
        .data
        .state
        .insert::<Search>(*Search::load(main_config.write_config)?);
    framework.data.global.insert::<MainConfig>(main_config);

    Ok(())
}
