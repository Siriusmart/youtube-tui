use crate::global::structs::{MainMenuPage, Page, Task, Tasks};
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::Framework;

/// WIP text command support
pub fn run_command(
    command: &[&str],
    framework: &mut Framework,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    // match a command splitted by space to a bunch of avaliable commands
    match command {
        [] => {}
        ["loadpage"] => { /* help message */ }
        ["loadpage", page] => {
            let page = match *page {
                "popular" => Some(Page::MainMenu(MainMenuPage::Popular)),
                _ => None,
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
        ["history"] => { /* help message */ }
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
        _ => {}
    }
}
