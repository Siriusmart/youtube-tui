use crate::{
    config::Search,
    global::structs::{
        ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Message, Page, SingleItemPage,
        Status, Task, Tasks,
    },
    load_configs,
};
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::Framework;

use super::{fake_rand_range, from_channel_url, from_video_url, from_playlist_url};

/// WIP text command support
pub fn run_command(
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
            ))
        }
        ["loadpage"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Usage: `loadpage {page}`"))
        }
        // loads a given page
        ["loadpage", page, ..] => {
            let page = match *page {
                "popular" => Some(Page::MainMenu(MainMenuPage::Popular)),
                "trending" => Some(Page::MainMenu(MainMenuPage::Trending)),
                "watchhistory" => Some(Page::MainMenu(MainMenuPage::History)),
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
        ["popular"] => run_command(&["loadpage", "popular"], framework, terminal),
        ["trending"] => run_command(&["loadpage", "trending"], framework, terminal),
        ["watchhistory"] => run_command(&["loadpage", "watchhistory"], framework, terminal),
        ["search"] => run_command(&["loadpage", "search"], framework, terminal),
        ["search", ..] => run_command(
            &format!("loadpage search {}", command[1..].join(" "))
                .split(' ')
                .collect::<Vec<&str>>(),
            framework,
            terminal,
        ),
        ["channel"] => run_command(&["loadpage", "channel"], framework, terminal),
        ["channel", identifier] => {
            run_command(&["loadpage", "channel", *identifier], framework, terminal)
        }
        ["video"] => run_command(&["loadpage", "video"], framework, terminal),
        ["video", identifier] => {
            run_command(&["loadpage", "video", *identifier], framework, terminal)
        }
        ["playlist"] => run_command(&["loadpage", "playlist"], framework, terminal),
        ["playlist", identifier] => {
            run_command(&["loadpage", "playlist", *identifier], framework, terminal)
        }
        ["history"] => { /* help message */ }
        ["history", "back"] | ["back"] => {
            let _ = framework.revert_last_history();
            framework.data.state.get_mut::<Tasks>().unwrap().priority.push(Task::ClearPage);
            framework.data.state.get_mut::<Tasks>().unwrap().priority.push(Task::RenderAll);
            run_command(&["flush"], framework, terminal);
            framework.data.global.get_mut::<Status>().unwrap().render_image = true;
        },
        ["history", "clear"] => framework.clear_history(),
        ["flush"] => loop {
            // runs all stacked actions
            if let Some(tasks) = framework.data.state.get_mut::<Tasks>().unwrap().pop() {
                let _res = tasks.run(framework, terminal);
                continue;
            }
            break;
        },
        ["reload"] | ["r"] => framework.data.state.get_mut::<Tasks>().unwrap().priority.push(Task::Reload),
        ["reload", "config"] | ["reload", "configs"] | ["r", "config"] | ["r", "configs"] => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                match load_configs(&mut framework.split_clean().0) {
                    Ok(()) => Message::Success(String::from("Config files have been reloaded")),
                    Err(e) => Message::Error(e.to_string()),
                }
        }
        ["q"] | ["quit"] | ["x"] | ["exit"] => {
            framework.data.global.get_mut::<Status>().unwrap().exit = true;
        }
        ["hello", "world"] => {
            let index = fake_rand_range(0, HELLO_WORLDS.len() as i64) as usize;
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(format!("Line #{index}: {}", HELLO_WORLDS[index]));
        }
        _ => {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Error(format!("Unknown command: `{}`", command.join(" ")))
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
