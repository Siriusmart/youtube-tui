use crate::{
    config::{serde::KeyCodeSerde, *},
    global::{functions::*, structs::*, traits::*},
    load_configs,
};
use crossterm::event::{KeyEvent, KeyModifiers};
use home::home_dir;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    env,
    error::Error,
    fs,
    io::Stdout,
    process::{Command, Stdio},
};
use tui_additions::framework::Framework;

/// runs text command - command from the command line (not TUI) which response is just a string
pub fn text_command(command: &str) -> Option<String> {
    match command
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["help"] => Some(help_msg(
            &CommandsRemapConfig::load(WriteConfig::Dont).unwrap(),
        )),
        ["version"] => Some(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )),
        _ => None,
    }
}

pub fn run_command(
    command: &str,
    framework: &mut Framework,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    command.split(";;").for_each(|single_command| {
        run_single_command(
            &single_command
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>(),
            framework,
            terminal,
        )
    });
}

/// runs a command in the TUI, returns true if its a loadpage command, false if not
pub fn run_single_command(
    command: &[&str],
    framework: &mut Framework,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    // match a command splitted by space to a bunch of avaliable commands
    match command {
        [] => {}
        ["bookmark", id] => {
            match (|| -> Result<Item, Box<dyn Error>> {
                Ok(serde_json::from_str(&fs::read_to_string(
                    home_dir()
                        .unwrap()
                        .join(format!(".cache/youtube-tui/info/{id}.json")),
                )?)?)
            })() {
                Ok(item) => {
                    let library = framework.data.global.get_mut::<Library>().unwrap();
                    let _ = library.push(item);
                    let _ = library.save();
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Success(String::from("Bookmark added"))
                }
                Err(e) => {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(format!("Unknown item: {e}"))
                }
            }
        }
        ["unmark", id] => {
            let library = framework.data.global.get_mut::<Library>().unwrap();

            if library.remove(id) {
                let _ = library.save();
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Success(String::from("Bookmark removed"))
            } else {
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Error(String::from("No item with that ID found"))
            }
        }
        ["togglemark", id] => {
            let library = framework.data.global.get_mut::<Library>().unwrap();
            if library.remove(id) {
                let _ = library.save();
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Success(String::from("Bookmark removed"))
            } else {
                run_single_command(&["bookmark", id], framework, terminal);
            }
        }
        ["help"] => {
            *framework.data.global.get_mut::<Message>().unwrap() = Message::Message(String::from(
                "Avaliable commands can be viewed by running `youtube-tui help` in terminal",
            ));
        }
        ["switchprovider"] => {
            let status = framework.data.global.get_mut::<Status>().unwrap();

            status.provider.rotate();
            status.provider_updated = true;
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(format!("Provider updated to {}", status.provider.as_str()));
            update_provider(&mut framework.data);
        }
        // loads a given page
        ["loadpage", page, ..] => {
            let page = match *page {
                "popular" => Some(Page::MainMenu(MainMenuPage::Popular)),
                "trending" => Some(Page::MainMenu(MainMenuPage::Trending)),
                "watchhistory" => Some(Page::MainMenu(MainMenuPage::History)),
                "feed" => Some(Page::Feed),
                "library" => Some(Page::MainMenu(MainMenuPage::Library)),
                "channel" => {
                    if command.len() == 2 {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Message(String::from("Usage: `loadpage channel {id/url}`"));
                        return;
                    }

                    match from_channel_url(command[2]) {
                        Ok(id) => Some(Page::ChannelDisplay(ChannelDisplayPage {
                            id,
                            r#type: ChannelDisplayPageType::Main,
                        })),
                        Err(e) => {
                            *framework.data.global.get_mut::<Message>().unwrap() =
                                Message::Error(e);
                            return;
                        }
                    }
                }
                "video" => {
                    if command.len() == 2 {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Message(String::from("Usage: `loadpage video {id/url}`"));
                        return;
                    }

                    match from_video_url(command[2]) {
                        Ok(id) => Some(Page::SingleItem(SingleItemPage::Video(id))),
                        Err(e) => {
                            *framework.data.global.get_mut::<Message>().unwrap() =
                                Message::Error(e);
                            return;
                        }
                    }
                }
                "playlist" => {
                    if command.len() == 2 {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Message(String::from("Usage: `loadpage playlist {id/url}`"));
                        return;
                    }

                    match from_playlist_url(command[2]) {
                        Ok(id) => Some(Page::SingleItem(SingleItemPage::Playlist(id))),
                        Err(e) => {
                            *framework.data.global.get_mut::<Message>().unwrap() =
                                Message::Error(e);
                            return;
                        }
                    }
                }
                "search" => {
                    if command.len() == 2 {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Message(String::from("Usage: `search {query}`"));
                        return;
                    }

                    // search for a query, although the command is matched as an array, the original query can
                    // be reconstructed by joining the string with a space in between
                    let search = framework.data.state.get_mut::<Search>().unwrap();
                    search.query = command[2..].join(" ");
                    let cloned = search.clone();
                    Some(Page::Search(cloned))
                }
                _ => {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(format!("Unknown page: `{page}`"));
                    None
                }
            };

            if let Some(page) = page {
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::LoadPage(page))
            }
        }
        ["history", "back"] | ["back"] => {
            let _ = framework.revert_last_history();
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::ClearPage);
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            run_single_command(&["flush"], framework, terminal);
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .render_image = true;
        }
        ["history", "clear"] => {
            framework.clear_history();
        }
        ["flush"] => loop {
            // runs all stacked actions
            if let Some(tasks) = framework.data.state.get_mut::<Tasks>().unwrap().pop() {
                let _res = tasks.run(framework, terminal);
                continue;
            }
            break;
        },
        ["reload"] | ["r"] => {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Reload);
        }
        ["reload", "configs"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                match load_configs(&mut framework.split_clean().0) {
                    Ok(()) => Message::Success(String::from("Config files have been reloaded")),
                    Err(e) => Message::Error(e.to_string()),
                };
        }
        ["quit"] => {
            framework.data.global.get_mut::<Status>().unwrap().exit = true;
        }
        ["hello", "world"] => {
            let index = fake_rand_range(0, HELLO_WORLDS.len() as i64) as usize;
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(format!("Line #{index}: {}", HELLO_WORLDS[index]));
        }
        ["version"] => {
            *framework.data.global.get_mut::<Message>().unwrap() = Message::Message(format!(
                "{} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ));
        }
        ["run", ..] => {
            let command = command[1..].join(" ");
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(command.clone());
            if let Ok(mut child) =
                Command::new(&framework.data.global.get::<MainConfig>().unwrap().shell)
                    .args(["-c", &command])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
            {
                let _ = child.wait();
            }
        }
        ["parrun", ..] => {
            let command = command[1..].join(" ");
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(command.clone());
            let _ = Command::new(&framework.data.global.get::<MainConfig>().unwrap().shell)
                .args(["-c", &command])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
        #[cfg(feature = "clipboard")]
        ["copy", ..] => {
            set_clipboard(command[1..].join(" "));
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(String::from("Copied to clipboad"));
        }
        #[cfg(not(feature = "clipboard"))]
        ["copy", ..] => {
            *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(String::from(
                "Feature `clipboard` is disabled and not compiled",
            ));
        }
        ["sync", identifier] => {
            let id = if identifier.len() == 24 {
                identifier.to_string()
            } else {
                let splitted = identifier.split_once("/channel/");

                match splitted {
                    Some((_, actual_stuff)) if actual_stuff.len() >= 24 => {
                        actual_stuff[0..24].to_string()
                    }
                    _ => {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Error(String::from("Invalid identifier: no channel ID found"));
                        return;
                    }
                }
            };

            let client = framework
                .data
                .global
                .get::<InvidiousClient>()
                .unwrap()
                .clone();

            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Syncing..."));
            terminal.draw(|frame| framework.render(frame)).unwrap();

            let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
            let image_index = mainconfig.image_index;
            let download_thumbnails = mainconfig.images.display();
            let syncing = mainconfig.syncing;

            let subscriptions = framework.data.global.get_mut::<Subscriptions>().unwrap();
            match subscriptions.sync_one(&id, &client, image_index, download_thumbnails, &syncing) {
                Ok(()) => {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Success(String::from("Channel synced"));
                }
                Err(e) => {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(format!("Sync failed: {e}"));
                }
            };
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
        }
        ["unsub", identifier] => {
            let id = if identifier.len() == 24 {
                identifier.to_string()
            } else {
                let splitted = identifier.split_once("/channel/");

                match splitted {
                    Some((_, actual_stuff)) if actual_stuff.len() >= 24 => {
                        actual_stuff[0..24].to_string()
                    }
                    _ => {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Error(String::from("Invalid identifier: no channel ID found"));
                        return;
                    }
                }
            };

            if framework
                .data
                .global
                .get_mut::<Subscriptions>()
                .unwrap()
                .remove_one(&id)
            {
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Success(String::from("Unsubscribed from channel"));
            } else {
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Error(String::from("Channel not found in subscriptions"));
            }
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
        }
        ["syncall"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Syncing..."));
            terminal.draw(|frame| framework.render(frame)).unwrap();

            let client = framework
                .data
                .global
                .get::<InvidiousClient>()
                .unwrap()
                .clone();
            let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
            let image_index = mainconfig.image_index;
            let download_thumbnails = mainconfig.images.display();
            let syncing = mainconfig.syncing;

            let (success, failed, empty, cached) = framework
                .data
                .global
                .get_mut::<Subscriptions>()
                .unwrap()
                .sync(&client, image_index, download_thumbnails, syncing);

            *framework.data.global.get_mut::<Message>().unwrap() = Message::Success(format!(
                "Subscriptions synced: {success} success{} | {failed} fail | {cached} cached",
                if empty != 0 {
                    format!(" (which {empty} empty)")
                } else {
                    String::new()
                }
            ));

            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
        }
        ["key", keycode, modifier] => {
            let (keycodeserde, modifier) =
                match (|| -> Result<(KeyCodeSerde, u8), Box<dyn Error>> {
                    Ok((serde_yaml_ng::from_str(keycode)?, modifier.parse()?))
                })() {
                    Ok(stuff) => stuff,
                    Err(e) => {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Error(format!("Cannot parse keycode: `{e}`"));
                        return;
                    }
                };
            let keycode = match keycodeserde.to_keycode() {
                Some(code) => code,
                None => {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error("Unknown keycode".to_string());
                    return;
                }
            };
            let keymodifier = KeyModifiers::from_bits_truncate(modifier);
            key_input(KeyEvent::new(keycode, keymodifier), framework, terminal)
        }
        #[cfg(feature = "mpv")]
        ["mpv", "prop", property] => {
            let res = framework
                .data
                .global
                .get::<MpvWrapper>()
                .unwrap()
                .property(property.to_string());

            *framework.data.global.get_mut::<Message>().unwrap() = match res {
                Some(value) => Message::Message(format!("Value: `{value}`")),
                None => Message::Error("No such property".to_string()),
            };
        }
        #[cfg(feature = "mpv")]
        ["mpv", "tprop", property] => {
            let mpv = framework.data.global.get::<MpvWrapper>().unwrap();
            let res = mpv.property(property.to_string());

            let toset = match res {
                Some(value) if value.as_str() == "yes" || value.as_str() == "true" => "no",
                _ => "yes",
            };

            let res = mpv.set_property(property.to_string(), toset.to_string());
            *framework.data.global.get_mut::<Message>().unwrap() = match res {
                MpvResponse::Copy => Message::Mpv(format!("Set `{property}` to `{toset}`")),
                MpvResponse::Error(e) => Message::Error(format!("MPV error: {e}")),
                _ => unreachable!(),
            };
        }
        #[cfg(feature = "mpv")]
        ["mpv", "sprop", name, ..] => {
            let value = command[3..].join(" ");
            let res = framework
                .data
                .global
                .get::<MpvWrapper>()
                .unwrap()
                .set_property(name.to_string(), value.clone());

            *framework.data.global.get_mut::<Message>().unwrap() = match res {
                // MpvResponse::Copy => Message::Mpv(format!("Set `{name}` to `{value}`")),
                MpvResponse::Copy => return,
                MpvResponse::Error(e) => Message::Mpv(format!("MPV error: {e}")),
                _ => unreachable!(),
            };
        }
        #[cfg(feature = "mpv")]
        ["mpv", name, ..] => {
            let res = framework.data.global.get::<MpvWrapper>().unwrap().command(
                name.to_string(),
                command[2..].iter().map(|s| s.to_string()).collect(),
            );

            *framework.data.global.get_mut::<Message>().unwrap() = match res {
                // MpvResponse::Copy => Message::Mpv("MPV player OK.".to_string()),
                MpvResponse::Copy => return,
                MpvResponse::Error(e) => Message::Mpv(format!("MPV error: {e}")),
                _ => unreachable!(),
            };
        }
        #[cfg(not(feature = "mpv"))]
        ["mpv", ..] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Error(String::from("Feature `mpv` is disabled and not compiled"));
        }
        ["echo", r#type, ..] => {
            *framework.data.global.get_mut::<Message>().unwrap() = match *r#type {
                "message" => Message::Message(command[2..].join(" ")),
                "mpv" => Message::Mpv(command[2..].join(" ")),
                "success" => Message::Success(command[2..].join(" ")),
                "error" => Message::Error(command[2..].join(" ")),
                "none" => Message::None,
                _ => Message::Error(format!("Unknown type `{}`", r#type)),
            }
        }
        _ => {
            if let Some(cmd) = framework
                .data
                .global
                .get_mut::<CommandsRemapConfig>()
                .unwrap()
                .get(command)
            {
                run_command(&cmd, framework, terminal);
                return;
            }

            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Error(format!("Unknown command: `{}`", command.join(" ")));
        }
    }
}

const HELLO_WORLDS: &[&str] = &[
    "printf(\"Hello World\")",
    "std::cout << \"Hello World\"",
    "DISPLAY \"Hello World\".    .",
    "printIn(\"Hello World\")",
    "disp('Hello World')",
    "System.Console.WriteLine(\"Hello World\")",
    "console.lof 'Hello World'",
    "WriteLn('Hello World')",
    "print('Hello World')",
    "main = putStrLn \"Hello World\"",
    "writeln ('Hello, world.')",
    "puts 'Hello World'",
    "print(\"Hello World\")",
    "db    'Hello World', 10, 0",
    "cat('Hello World')",
    "println('Hello World')",
    "echo \"Hello World\"",
    "System.out.println(\"Hello World\")",
    "println('Hello World\")",
    "printfn \"Hello World\"",
    "(print \"Hello World\")",
    "console.log(\"Hello World\")",
    "BEGIN DISPLAY(\"Hello World\") END.",
    "print \"Hello World\"",
    "puts \"Hello World\"",
    "console.log 'Hello World'",
    "print *, \"Hello World\"",
    "<h1>Hello World<\\h1>",
];

fn help_msg(cmdefines: &CommandsRemapConfig) -> String {
    format!("\x1b[32mYouTube TUI commands\x1b[0m

\x1b[37mfor more visit https://siriusmart.github.io/youtube-tui/commands.html\x1b[0m

\x1b[91mUSAGE:\x1b[0m
    youtube-tui (command)

\x1b[91mINFORMATIONAL:\x1b[0m
    \x1b[33mhelp\x1b[0m                            Display this message
    \x1b[33mversion\x1b[0m                         Print version info and exit

\x1b[91mLOADPAGE:
\x1b[37mloadpage [page] loads the corresponding page\x1b[0m
    \x1b[33mloadpage popular\x1b[0m                Loads the popular videos page
    \x1b[33mloadpage trending\x1b[0m               Loads the trending videos page
    \x1b[33mloadpage watchhistory\x1b[0m           Loads the watch history page
    \x1b[33mloadpage subscriptions\x1b[0m          Loads the subscriptions page
    \x1b[33mloadpage bookmarks\x1b[0m              Loads the bookmarks page
    \x1b[33mloadpage library\x1b[0m                Loads the library (saved items) page
    \x1b[33mloadpage feed\x1b[0m                   Loads the library (feed) page
    \x1b[33mloadpage search [query]\x1b[0m         Loads the search page with the given query
    \x1b[33mloadpage video [identifier]\x1b[0m     Loads the video item page
    \x1b[33mloadpage playlist [identifier]\x1b[0m  Loads the playlist item page
    \x1b[33mloadpage channel [identifier]\x1b[0m   Loads the channel item page

\x1b[91mHISTORY:\x1b[0m
    \x1b[33mhistory back\x1b[0m                    Revert back to previous state
    \x1b[33mhistory clear\x1b[0m                   Clear all previously saved states, making the current state the original

\x1b[91mUTILITY:\x1b[0m
    \x1b[33mreload\x1b[0m                          Reloads the current page
    \x1b[33mreload configs\x1b[0m                  Reload all config files
    \x1b[33mflush\x1b[0m                           Run all tasks in queue immediately
    \x1b[33mquit\x1b[0m                            Immediately exit
    \x1b[33mrun [command]\x1b[0m                   Runs a system command (e.g. `run rm -rf / --no-preserve-root`)
    \x1b[33mparrun [command]\x1b[0m                Runs a system command non blocking (e.g. `run firefox example.com`)
    \x1b[33mcopy [text]\x1b[0m                     Copies text to clipboard
    \x1b[33mkey [keycode] [keymodifier]\x1b[0m     Create a key input event
    \x1b[33mecho [mode] [message]\x1b[0m           Dispalys a message in message bar, mode: none, success, warn, error, mpv (can be overwritten by mpv player)

\x1b[91mLIBRARY:\x1b[0m
    \x1b[33mbookmark [id]\x1b[0m                   Bookmark item with ID (item must be already loaded)
    \x1b[33munmark [id]\x1b[0m                     Remove bookmark item with ID
    \x1b[33mtogglemark [id]\x1b[0m                 Toggle bookmark status
    \x1b[33msub/sync [id or url]\x1b[0m            Add channel to subscription, or sync an existing channel
    \x1b[33munsub [id or url]\x1b[0m               Remove channel from subscription
    \x1b[33msyncall\x1b[0m                         Sync all subscriptions

\x1b[91mMPV:\x1b[0m
    \x1b[33mmpv prop [label]\x1b[0m                Gets mpv property
    \x1b[33mmpv sprop [label] [value]\x1b[0m       Set mpv property
    \x1b[33mmpv tprop [label] [value]\x1b[0m       Toggle a yes/no property
    \x1b[33mmpv [command]\x1b[0m                   Runs a libmpv command

\x1b[91mCUSTOM COMMANDS:\x1b[0m
\x1b[37mdefined in cmdefine.yml\x1b[30m
{}

\x1b[37mOnly load page and informational commands should be used from command line, the rest can only be used in (`:`) command mode inside the TUI.\x1b[0m", cmdefines.0.iter().map(|(key, value)| format!("   \x1b[33m{: <28}\x1b[0m     `{value}`", key)).collect::<Vec<_>>().join("\n"))
}
