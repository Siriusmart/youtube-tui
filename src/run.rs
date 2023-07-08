use crossterm::event::{self, Event, MouseButton, MouseEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io::Stdout};
use tui_additions::{
    framework::{Framework, FrameworkDirection},
    widgets::TextField,
};

use crate::{
    config::*,
    global::{functions::*, structs::*},
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

        framework
            .data
            .global
            .get_mut::<Status>()
            .unwrap()
            .storage
            .clear();

        // exits the function is `.exit` is true - a way for items/commands to exit the program
        if framework.data.global.get::<Status>().unwrap().exit {
            break;
        }
        *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

        match event::read()? {
            Event::Mouse(mouse)
                if framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .mouse_support =>
            {
                if mouse.kind != MouseEventKind::Down(MouseButton::Left) {
                    continue;
                }

                *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

                let status = framework.data.global.get_mut::<Status>().unwrap();
                status.command_capture = None;

                // check if the search filter popup is clicked
                let mut searchfilter_clicked = false;
                if status.search_filter_opened {
                    let (mut frameworkclean, state) = framework.split_clean();
                    for row in state.0.iter_mut() {
                        for item in row.items.iter_mut() {
                            if item.item.r#type()
                                == "youtube_ratatui::items::searchfilters::SearchFilter"
                                && item.item.mouse_event(
                                    &mut frameworkclean,
                                    0,
                                    0,
                                    mouse.column,
                                    mouse.row,
                                )
                            {
                                searchfilter_clicked = true;
                                break;
                            }
                        }

                        if searchfilter_clicked {
                            break;
                        }
                    }
                }

                let updated = if searchfilter_clicked {
                    true
                } else {
                    framework.mouse_event(mouse.column, mouse.row)
                };

                if updated {
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                }
            }
            Event::Key(key) => {
                // check if key binds to any commands
                let command_to_run = framework
                    .data
                    .global
                    .get::<CommandBindings>()
                    .unwrap()
                    .get_command(&key, framework.data.state.get::<Page>().unwrap());
                if !command_to_run.is_empty() {
                    run_command(&apply_envs(command_to_run), framework, terminal);
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                }

                // 1. get the corresponding action
                // 2. check if action is deselect, if yes, deselect
                // 3. check is anything is selected, if yes, run `.key_event()` with the key
                // 4. if nothing is selected, do stuff like moving the cursor or exiting

                let action = framework
                    .data
                    .global
                    .get::<KeyBindingsConfig>()
                    .unwrap()
                    .get(key);

                if action == Some(KeyAction::Deselect) {
                    let _ = framework.deselect();
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
                        .command_capture = None;
                    continue;
                }

                // if something is selected, pass the key input into that item
                // if nothing is selected, the following big chunk of code handles to movement of
                // cursor and stuff
                if framework.is_selected() {
                    if let Err(e) = framework.key_input(key) {
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Error(e.to_string());
                    };
                    continue;
                }

                // the first part check for if the keys should be captured for entering commands
                if framework
                    .data
                    .global
                    .get::<Status>()
                    .unwrap()
                    .command_capture
                    .is_some()
                    // the second part runs the command, and see if the returned boolean
                    // `is_updated` is true, if so update the screen
                    && command_capture(&mut framework.split_clean().0, key)
                {
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::RenderAll);
                    continue;
                }

                if let Some(action) = action {
                    let mut render = true;
                    match action {
                        KeyAction::StartCommandCapture => {
                            framework
                                .data
                                .global
                                .get_mut::<Status>()
                                .unwrap()
                                .command_capture = Some(TextField::default());
                        }
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
                            if !framework.history.is_empty() {
                                clear_envs(
                                    &mut framework.data.state.get::<StateEnvs>().unwrap().0.clone(),
                                );
                            }
                            if framework.revert_last_history().is_err() {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(String::from(
                                        "This is already the beginning of history",
                                    ))
                            } else {
                                let status = framework.data.global.get_mut::<Status>().unwrap();
                                status.render_image = true;

                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::ClearPage);
                                let state_envs =
                                    framework.data.state.get_mut::<StateEnvs>().unwrap();
                                set_envs(state_envs.clone().0.into_iter(), &mut state_envs.0);
                                update_provider(&mut framework.data);
                            }
                        }
                        KeyAction::FirstHistory => {
                            if framework.revert_history(0).is_err() {
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Error(String::from(
                                        "This is already the beginning of history",
                                    ))
                            } else {
                                let status = framework.data.global.get_mut::<Status>().unwrap();
                                status.render_image = true;

                                update_provider(&mut framework.data);
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
