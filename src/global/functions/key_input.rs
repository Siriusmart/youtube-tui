use std::{any::TypeId, io::Stdout};

use crossterm::event::KeyEvent;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui_additions::framework::{Framework, FrameworkDirection};

use crate::{config::*, global::structs::*, items::SearchBar};

use super::*;

pub fn key_input(
    mut key: KeyEvent,
    framework: &mut Framework,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    framework
        .data
        .global
        .get::<RemapConfig>()
        .unwrap()
        .get(&mut key);

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
        return;
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
        return;
    }

    // if something is selected, pass the key input into that item
    // if nothing is selected, the following big chunk of code handles to movement of
    // cursor and stuff
    if framework.is_selected() {
        let (x, y) = framework.cursor.selected(&framework.selectables).unwrap();
        if let Err(e) = framework.key_input(key) {
            *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(e.to_string());
        };

        let selected = framework.state.get_mut(x, y);
        if framework
            .data
            .global
            .get::<MainConfig>()
            .unwrap()
            .legacy_input_handling
            || (*selected).type_id() == TypeId::of::<SearchBar>()
        {
            return;
        }
    }

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

    if let Some(action) = action {
        let mut render = true;
        match action {
            KeyAction::StartCommandCapture => {
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .reset_command_capture();
            }
            KeyAction::Exit => framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Command("q".to_string())),
            KeyAction::MoveUp if !framework.is_selected() => {
                let _ = framework.r#move(FrameworkDirection::Up);
            }
            KeyAction::MoveDown if !framework.is_selected() => {
                let _ = framework.r#move(FrameworkDirection::Down);
            }
            KeyAction::MoveLeft if !framework.is_selected() => {
                let _ = framework.r#move(FrameworkDirection::Left);
            }
            KeyAction::MoveRight if !framework.is_selected() => {
                let _ = framework.r#move(FrameworkDirection::Right);
            }
            KeyAction::Reload => framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Reload),
            KeyAction::Back => {
                if !framework.history.is_empty() {
                    clear_envs(&mut framework.data.state.get::<StateEnvs>().unwrap().0.clone());
                }
                if framework.revert_last_history().is_err() {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("This is already the beginning of history"))
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
                    let state_envs = framework.data.state.get_mut::<StateEnvs>().unwrap();
                    set_envs(state_envs.clone().0.into_iter(), &mut state_envs.0);
                    update_provider(&mut framework.data);
                }
            }
            KeyAction::FirstHistory => {
                if framework.revert_history(0).is_err() {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("This is already the beginning of history"))
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
                        Message::Error(String::from("This is already the beginning of history"))
                } else {
                    framework.clear_history();
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Success(String::from("History cleared!"))
                }
            }
            KeyAction::Select if !framework.is_selected() => {
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
