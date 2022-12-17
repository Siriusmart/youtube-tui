use crate::{
    config::Search,
    global::structs::{
        ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Message, Page, Status, Task,
        Tasks,
    },
    load_configs,
};
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::Framework;

use super::fake_rand_range;

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
                "history" => Some(Page::MainMenu(MainMenuPage::History)),
                "channel" => {
                    if command.len() == 2 {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Message(String::from(
                                "Usage: `loadpage channel {id/url}` or `channel {id/url}`",
                            ));
                        return;
                    }
                    // a youtube channel id is exactly 24 characters long, so if the identifier to 24 chars
                    // long, then just assume its the channel id
                    let id = if command[2].len() == 24 {
                        command[2].to_string()
                    } else {
                        // the channel id comes after `/channel/` in an url
                        let index = if let Some(index) = command[2].find("/channel/") {
                            // if there isnt 24 characters after `/channel/`, that url must not have
                            // contained a channel id
                            if command[2].len() < index + 33 {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(format!(
                                        "Cannot find channel id from string `{}`",
                                        command[2]
                                    ));
                                return;
                            }
                            index + 9
                        } else {
                            *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(
                                format!("Cannot find channel id from string `{}`", command[2]),
                            );
                            return;
                        };

                        command[2][index..index + 24].to_string()
                    };

                    Some(Page::ChannelDisplay(ChannelDisplayPage {
                        id,
                        r#type: ChannelDisplayPageType::Main,
                    }))
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
        ["history"] => { /* help message */ }
        ["history", "clear"] => framework.clear_history(),
        ["flush"] => loop {
            // runs all stacked actions
            if let Some(tasks) = framework.data.state.get_mut::<Tasks>().unwrap().pop() {
                let _res = tasks.run(framework, terminal);
                continue;
            }
            break;
        },
        ["reload", "config"] | ["reload", "configs"] => {
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
