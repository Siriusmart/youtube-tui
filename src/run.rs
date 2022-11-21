use crossterm::event::{self, Event};
use std::{error::Error, io::Stdout};
use tui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::{Framework, FrameworkDirection};

use crate::{
    config::KeyBindingsConfig,
    global::structs::{KeyAction, Message, Status, Task, Tasks},
};

/// the main event loop of the program
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    framework: &mut Framework,
) -> Result<(), Box<dyn Error>> {
    loop {
        // repeat forever until all tasks are ran (and Tasks is cleared)
        if let Some(tasks) = framework.data.state.get_mut::<Tasks>().unwrap().pop() {
            tasks.run(framework, terminal)?;
            continue;
        }

        *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

        match event::read()? {
            Event::Key(key) => {
                // 1. get the corresponding action
                // 2. check if action is deselect, if yes, deselect
                // 3. check is anything is selected, if yes, run `.key_event()` with the key
                // 4. if nothing is selected, do stuff like moving the cursor or exiting

                let action = if let Some(keyactions) = framework
                    .data
                    .global
                    .get::<KeyBindingsConfig>()
                    .unwrap()
                    .0
                    .get(&key.code)
                {
                    keyactions.get(&key.modifiers.bits()).copied()
                } else {
                    None
                };

                if action == Some(KeyAction::Deselect) {
                    let _ = framework.deselect();
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                }

                // if something is selected, pass the key input into that item
                // if nothing is selected, the following big chunk of code handles to movement of
                // cursor and stuff
                if framework.is_selected() {
                    if let Err(e) = framework.key_input(key) {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Error(format!("{}", e));
                    };
                } else if let Some(action) = action {
                    let mut render = true;
                    match action {
                        KeyAction::Exit => break,
                        KeyAction::MoveUp => framework.r#move(FrameworkDirection::Up)?,
                        KeyAction::MoveDown => framework.r#move(FrameworkDirection::Down)?,
                        KeyAction::MoveLeft => framework.r#move(FrameworkDirection::Left)?,
                        KeyAction::MoveRight => framework.r#move(FrameworkDirection::Right)?,
                        KeyAction::Reload => framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::Reload),
                        KeyAction::Back => {
                            if framework.revert_last_history().is_err() {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(String::from(
                                        "This is already the beginning of history",
                                    ))
                            } else {
                                framework
                                    .data
                                    .global
                                    .get_mut::<Status>()
                                    .unwrap()
                                    .render_image = true;
                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::ClearPage);
                            }
                        }
                        KeyAction::FirstHistory => {
                            if framework.revert_history(0).is_err() {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(String::from(
                                        "This is already the beginning of history",
                                    ))
                            } else {
                                framework
                                    .data
                                    .global
                                    .get_mut::<Status>()
                                    .unwrap()
                                    .render_image = true;
                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::ClearPage);
                            }
                        }
                        KeyAction::ClearHistory => {
                            if framework.history.is_empty() {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(String::from(
                                        "This is already the beginning of history",
                                    ))
                            } else {
                                framework.clear_history();
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Success(String::from("History cleared!"))
                            }
                        }
                        KeyAction::Select => {
                            let _ = framework.select();
                        }
                        _ => render = false,
                    }
                    if render {
                        framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::RenderAll);
                    }
                }
            }
            // always render if there is a screen resize event
            Event::Resize(_, _) => {
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::RenderAll);
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .render_image = true;
            }
            _ => {}
        }
    }
    Ok(())
}
