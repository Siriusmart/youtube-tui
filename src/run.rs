use crossterm::event::{self, Event, MouseButton, MouseEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    any::TypeId,
    error::Error,
    io::Stdout,
    time::{Duration, Instant},
};
use tui_additions::framework::Framework;

use crate::{
    config::*,
    global::{functions::*, structs::*},
    items::{MessageBar, SearchFilter},
};

/// the main event loop of the program
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    framework: &mut Framework,
) -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_secs(1);
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
                            if item.item.type_id() == TypeId::of::<SearchFilter>()
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
