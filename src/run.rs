use crossterm::event::{self, Event, MouseButton, MouseEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
#[cfg(feature = "mpv")]
use std::time::{Duration, Instant};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
    io::Stdout,
};
use tui_additions::framework::Framework;

use crate::{
    config::*,
    global::{functions::*, structs::*},
    items::*,
};

/// the main event loop of the program
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    framework: &mut Framework,
) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "mpv")]
    let tick_rate = Duration::from_secs(1);
    #[cfg(feature = "mpv")]
    let mut last_tick = Instant::now();
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

        #[cfg(feature = "mpv")]
        if !event::poll(
            tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0)),
        )? {
            // do tick changes
            last_tick = Instant::now();
            // TaskQueue::render_filter(framework, terminal, |item| (*item).type_id() == TypeId::of::<MessageBar>())?;
            if MessageBar::is_mpv_render(&framework.split_clean().0) {
                TaskQueue::render(framework, terminal)?;
            }
            continue;
        }

        *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

        let mut updated = false;

        match event::read()? {
            Event::Mouse(mouse)
                if framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .mouse_support =>
            {
                let scroll_behaviour = framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .textbar_scroll_behaviour;
                let command_capture = &mut framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .command_capture;

                match mouse.kind {
                    // copied from messagebar.rs::key_event
                    // if modify this, also modify it there
                    MouseEventKind::ScrollUp if command_capture.is_some() => {
                        updated = updated
                            || match scroll_behaviour {
                                TextbarScrollBehaviour::Character => {
                                    command_capture.as_mut().unwrap().left().is_ok()
                                }
                                TextbarScrollBehaviour::History => {
                                    let status = framework.data.global.get::<Status>().unwrap();
                                    let history =
                                        framework.data.global.get::<CommandHistory>().unwrap();
                                    if status.command_history_index != Some(0)
                                        && !history.0.is_empty()
                                    {
                                        let new_index =
                                            status.command_history_index.unwrap_or(history.0.len())
                                                - 1;
                                        let new_content = history.0[new_index].clone();

                                        let status =
                                            framework.data.global.get_mut::<Status>().unwrap();
                                        let text_field = status.command_capture.as_mut().unwrap();
                                        if status.command_history_index.is_none() {
                                            status.command_editing_cache =
                                                text_field.content.clone();
                                        }
                                        text_field.content = new_content;
                                        status.command_history_index = Some(new_index);
                                        let _ = text_field.last();
                                        true
                                    } else {
                                        false
                                    }
                                }
                                TextbarScrollBehaviour::Word => {
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
                                    true
                                }
                            }
                    }
                    MouseEventKind::ScrollUp => {
                        let data: Box<dyn Any> = Box::new("scrollup".to_string());
                        updated = updated
                            || framework.message(HashMap::from([("type".to_string(), data)]))
                    }
                    MouseEventKind::ScrollDown if command_capture.is_some() => {
                        updated = updated
                            || match scroll_behaviour {
                                TextbarScrollBehaviour::Word => {
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
                                    true
                                }
                                TextbarScrollBehaviour::History => {
                                    let status = framework.data.global.get::<Status>().unwrap();
                                    if status.command_history_index.is_none() {
                                        false
                                    } else {
                                        let history =
                                            framework.data.global.get::<CommandHistory>().unwrap();
                                        if status.command_history_index == Some(history.0.len() - 1)
                                        {
                                            let status =
                                                framework.data.global.get_mut::<Status>().unwrap();
                                            let text_field =
                                                status.command_capture.as_mut().unwrap();
                                            status.command_history_index = None;
                                            text_field.content =
                                                status.command_editing_cache.clone();
                                        } else {
                                            let new_index = status
                                                .command_history_index
                                                .unwrap_or(history.0.len())
                                                + 1;
                                            let new_content = history.0[new_index].clone();

                                            let status =
                                                framework.data.global.get_mut::<Status>().unwrap();
                                            let text_field =
                                                status.command_capture.as_mut().unwrap();
                                            text_field.content = new_content;
                                            status.command_history_index = Some(new_index);
                                            let _ = text_field.last();
                                        }
                                        true
                                    }
                                }
                                TextbarScrollBehaviour::Character => {
                                    command_capture.as_mut().unwrap().right().is_ok()
                                }
                            }
                    }
                    MouseEventKind::ScrollDown => {
                        let data: Box<dyn Any> = Box::new("scrolldown".to_string());
                        updated = updated
                            || framework.message(HashMap::from([("type".to_string(), data)]))
                    }
                    _ => {}
                }

                if mouse.kind != MouseEventKind::Down(MouseButton::Left) {
                    if updated {
                        framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::RenderAll);
                    }

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
                            if (*item.item).type_id() == TypeId::of::<SearchFilter>()
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

                updated = updated
                    || if searchfilter_clicked {
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
            Event::Key(key) => key_input(key, framework, terminal),
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
