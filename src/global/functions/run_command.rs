use crate::{
    config::Search,
    global::structs::{
        ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Message, Page, SingleItemPage,
        Status, Task, Tasks,
    },
    load_configs,
};
use run_script::ScriptOptions;
use std::{env, io::Stdout, thread};
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::Framework;

use super::{
    fake_rand_range, from_channel_url, from_playlist_url, from_video_url, set_clipboard,
    update_provider,
};

/// runs text command - command from the command line (not TUI) which response is just a string
pub fn text_command(command: &str) -> Option<String> {
    match command
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["help"] => Some(String::from(HELP_MSG)),
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
    command.split(";;").into_iter().for_each(|single_command| {
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
        ["help"] | ["h"] => {
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
        ["loadpage"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Usage: `loadpage {page}`"));
        }
        // loads a given page
        ["loadpage", page, ..] => {
            let page = match *page {
                "popular" => Some(Page::MainMenu(MainMenuPage::Popular)),
                "trending" => Some(Page::MainMenu(MainMenuPage::Trending)),
                "watchhistory" => Some(Page::MainMenu(MainMenuPage::History)),
                "subscriptions" => Some(Page::MainMenu(MainMenuPage::Subscriptions)),
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
        // redirects to the relevant `loadpage` command
        ["popular"] => run_single_command(&["loadpage", "popular"], framework, terminal),
        ["trending"] => run_single_command(&["loadpage", "trending"], framework, terminal),
        ["watchhistory"] => run_single_command(&["loadpage", "watchhistory"], framework, terminal),
        ["subscriptions"] => {
            run_single_command(&["loadpage", "subscriptions"], framework, terminal)
        }
        ["bookmarks"] => run_single_command(&["loadpage", "bookmarks"], framework, terminal),
        ["library"] => run_single_command(&["loadpage", "library"], framework, terminal),
        ["search"] => run_single_command(&["loadpage", "search"], framework, terminal),
        ["search", ..] => run_single_command(
            &format!("loadpage search {}", command[1..].join(" "))
                .split(' ')
                .collect::<Vec<&str>>(),
            framework,
            terminal,
        ),
        ["channel"] => run_single_command(&["loadpage", "channel"], framework, terminal),
        ["channel", identifier] => {
            run_single_command(&["loadpage", "channel", *identifier], framework, terminal)
        }
        ["video"] => run_single_command(&["loadpage", "video"], framework, terminal),
        ["video", identifier] => {
            run_single_command(&["loadpage", "video", *identifier], framework, terminal)
        }
        ["playlist"] => run_single_command(&["loadpage", "playlist"], framework, terminal),
        ["playlist", identifier] => {
            run_single_command(&["loadpage", "playlist", *identifier], framework, terminal)
        }
        ["history"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Usage: `history [back/clear]`"))
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
        ["reload", "config"] | ["reload", "configs"] | ["r", "config"] | ["r", "configs"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                match load_configs(&mut framework.split_clean().0) {
                    Ok(()) => Message::Success(String::from("Config files have been reloaded")),
                    Err(e) => Message::Error(e.to_string()),
                };
        }
        ["q"] | ["quit"] | ["x"] | ["exit"] => {
            framework.data.global.get_mut::<Status>().unwrap().exit = true;
        }
        ["hello", "world"] => {
            let index = fake_rand_range(0, HELLO_WORLDS.len() as i64) as usize;
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(format!("Line #{index}: {}", HELLO_WORLDS[index]));
        }
        ["version"] | ["v"] => {
            *framework.data.global.get_mut::<Message>().unwrap() = Message::Message(format!(
                "{} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ));
        }
        ["run"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Usage: run [command]"));
        }
        ["run", ..] => {
            let command = command[1..].join(" ");
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(command.clone());
            thread::spawn(move || {
                run_script::run(&command, &Vec::new(), &ScriptOptions::new()).unwrap();
            });
        }
        ["copy"] | ["cp"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Usage: copy [text]"));
        }
        ["copy", ..] | ["cp", ..] => {
            set_clipboard(command[1..].join(" "));
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Success(String::from("Copied to clipboad"));
        }
        _ => {
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

const HELP_MSG: &str = "\x1b[32mYouTube TUI commands\x1b[0m

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
    \x1b[33mloadpage library\x1b[0m                  Loads the library (saved items) page
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
    \x1b[33mrun [command]\x1b[0m                   Runs a system command (e.g. `run firefox example.com`)
    \x1b[33mcopy [text]\x1b[0m                     Copies text to clipboard

\x1b[91mALT:\x1b[0m
\x1b[37malts links back to the original command\x1b[30m
    \x1b[33m[page] (additional options)\x1b[0m     `loadpage [page]`
    \x1b[33mback\x1b[0m                            `history back`
    \x1b[33mr\x1b[0m                               `reload`
    \x1b[33mreload/r config/configs\x1b[0m         `reload configs`
    \x1b[33mq, exit, x\x1b[0m                      `quit`
    \x1b[33mcp [text]\x1b[0m                       `copy [text]`

\x1b[37mOnly load page and informational commands can be used from command line, the rest can only be used in (`:`) command mode inside the TUI.\x1b[0m";
