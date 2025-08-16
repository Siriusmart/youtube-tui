use crate::global::{functions::*, structs::*, traits::CollectionNoId};

#[cfg(feature = "clipboard")]
use crate::config::*;

use crossterm::event::{KeyCode, KeyEvent};
use tui_additions::framework::FrameworkClean;

/// handles key input when the user is entering commands
pub fn command_capture(framework: &mut FrameworkClean, key: KeyEvent) -> bool {
    #[cfg(feature = "clipboard")]
    match framework
        .data
        .global
        .get::<KeyBindingsConfig>()
        .unwrap()
        .get(key)
    {
        Some(KeyAction::Paste) => {
            let textfield = framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture
                .as_mut()
                .unwrap();
            // a textfield must have a width set, because we don't know the width yet just set it to the
            // maximum value possible, so that the cursor will not be moved because of the small value
            textfield.set_width(u16::MAX);
            let content = get_clipboard();
            if content.is_empty() {
                *framework.data.global.get_mut::<Message>().unwrap() =
                    Message::Error(String::from("Clipboard empty"));
                return true;
            }

            content.chars().for_each(|c| {
                let _ = textfield.push(c);
            });
            return true;
        }
        Some(KeyAction::RemoveWord) => {
            remove_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            return true;
        }
        Some(KeyAction::ClearLine) => {
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture
                .as_mut()
                .unwrap()
                .content
                .clear();
            return true;
        }
        Some(KeyAction::PreviousWord) => {
            previous_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            return true;
        }
        Some(KeyAction::NextWord) => {
            next_word(
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture
                    .as_mut()
                    .unwrap(),
            );
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
            return true;
        }
        // warning: really shitty code, it works, dont touch
        Some(KeyAction::PreviousEntry) => {
            let status = framework.data.global.get::<Status>().unwrap();
            let history = framework.data.global.get::<CommandHistory>().unwrap();
            if status.command_history_index != Some(0) && !history.0.is_empty() {
                let new_index = status.command_history_index.unwrap_or(history.0.len()) - 1;
                let new_content = history.0[new_index].clone();

                let status = framework.data.global.get_mut::<Status>().unwrap();
                let text_field = status.command_capture.as_mut().unwrap();
                if status.command_history_index.is_none() {
                    status.command_editing_cache = text_field.content.clone();
                }
                text_field.content = new_content;
                status.command_history_index = Some(new_index);
                let _ = text_field.last();
                return true;
            }
            return false;
        }
        Some(KeyAction::NextEntry) => {
            let status = framework.data.global.get::<Status>().unwrap();
            if status.command_history_index.is_none() {
                return false;
            } else {
                let history = framework.data.global.get::<CommandHistory>().unwrap();
                if status.command_history_index == Some(history.0.len() - 1) {
                    let status = framework.data.global.get_mut::<Status>().unwrap();
                    let text_field = status.command_capture.as_mut().unwrap();
                    status.command_history_index = None;
                    text_field.content = status.command_editing_cache.clone();
                    text_field.cursor = text_field.cursor.min(text_field.content.len());
                    if !text_field.content.is_empty() {
                        let _ = text_field.last();
                    }
                } else {
                    let new_index = status.command_history_index.unwrap_or(history.0.len()) + 1;
                    let new_content = history.0[new_index].clone();

                    let status = framework.data.global.get_mut::<Status>().unwrap();
                    let text_field = status.command_capture.as_mut().unwrap();
                    text_field.content = new_content;
                    status.command_history_index = Some(new_index);
                    let _ = text_field.last();
                }
                return true;
            }
        }
        _ => {}
    }

    let status = framework.data.global.get_mut::<Status>().unwrap();
    let textfield = status.command_capture.as_mut().unwrap();
    // a textfield must have a width set, because we don't know the width yet just set it to the
    // maximum value possible, so that the cursor will not be moved because of the small value
    textfield.set_width(u16::MAX);

    match key.code {
        KeyCode::Char(c) => textfield.push(c).is_ok(),
        KeyCode::Up => textfield.first().is_ok(),
        KeyCode::Down => textfield.last().is_ok(),
        KeyCode::Left => textfield.left().is_ok(),
        KeyCode::Right => textfield.right().is_ok(),
        KeyCode::Backspace => textfield.pop().is_ok(),
        KeyCode::Enter => {
            let content = textfield.content.clone();
            framework
                .data
                .global
                .get_mut::<CommandHistory>()
                .unwrap()
                .push(content);
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::Command(apply_envs(
                    framework
                        .data
                        .global
                        .get_mut::<Status>()
                        .unwrap()
                        .command_capture
                        .as_mut()
                        .unwrap()
                        .content
                        .clone(),
                )));
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .command_capture = None;
            true
        }
        _ => false,
    }
}
