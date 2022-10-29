use crate::config::{AppearanceConfig, MinDimentions};

use super::{message::Message, page::Page, status::Status};
use std::{error::Error, io::Stdout, mem};
use tui::{
    backend::CrosstermBackend, layout::Alignment, style::Style, widgets::Paragraph, Frame, Terminal,
};
use tui_additions::framework::{CursorState, Framework};
use typemap::Key;

/// tasks to be put on taskqueues
#[derive(Clone)]
pub enum Task {
    RenderAll,
    Reload,
    RenderOnly(usize, usize),
    LoadPage(Page),
    ClearPage,
}

/// multiple tasks joined together, with duplicates removed
#[derive(Clone, PartialEq, Eq)]
pub struct TaskQueue {
    pub render: RenderTask,
    pub reload: bool,
    pub load_page: Option<Page>,
    pub clear_all: bool,
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self {
            render: RenderTask::None,
            reload: false,
            load_page: None,
            clear_all: false,
        }
    }
}

impl TaskQueue {
    // add task to queue
    pub fn push(&mut self, task: Task) {
        match task {
            Task::RenderAll => self.render = RenderTask::All,
            Task::Reload => self.reload = true,
            Task::RenderOnly(x, y) => match &mut self.render {
                RenderTask::Only(renders) => {
                    if !renders.contains(&(x, y)) {
                        renders.push((x, y));
                        println!("{:?}", self.render);
                    }
                }
                RenderTask::None => self.render = RenderTask::Only(vec![(x, y)]),
                _ => {}
            },
            Task::LoadPage(page) => self.load_page = Some(page),
            Task::ClearPage => self.clear_all = true,
            // _ => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    // runs all tasks in a task queue
    pub fn run(
        self,
        framework: &mut Framework,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        if self.clear_all {
            terminal.clear()?;
        }

        // save state in history, then replace all items by whats in the new page and run `.load()` on them
        if let Some(page) = self.load_page {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(page.load_msg(framework));
            framework.push_history();
            framework.cursor = CursorState::default();

            let page_config = page.to_page_config(framework);
            *framework.data.state.get_mut::<MinDimentions>().unwrap() =
                MinDimentions::new(page_config.min_width(), page_config.min_height());

            let state = page_config.to_state(framework);
            framework.set_state(state);
            framework.data.state.get_mut::<Status>().unwrap().reset();
            *framework.data.state.get_mut::<Page>().unwrap() = page;
            Self::render_force_clear(framework, terminal)?;
            *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

            *framework.data.global.get_mut::<Message>().unwrap() = if let Err(e) = framework.load()
            {
                Message::Error(format!("{}", e))
            } else {
                Message::None
            };
            Self::render(framework, terminal)?;
        }

        if self.reload {
            // reload simply runs `.load()` on all items
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Reloading page..."));
            Self::render_force_clear(framework, terminal)?;
            *framework.data.global.get_mut::<Message>().unwrap() = if let Err(e) = framework.load()
            {
                Message::Error(format!("{}", e))
            } else {
                Message::None
            };
            Self::render(framework, terminal)?;
        }

        match self.render {
            RenderTask::All => {
                Self::render(framework, terminal)?;
            }
            RenderTask::Only(_locations) => {
                // need to file an issue to tui-rs suggesting this as a feature
                unimplemented!("tui-rs does not support partial re-rendering");
            }
            RenderTask::None => {}
        }

        Ok(())
    }

    // the render task runs this function
    pub fn render(
        framework: &mut Framework,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|frame| {
            Self::render_with_frame(framework, frame);
        })?;

        Ok(())
    }

    // this function force clears the terminal before rendering, removing sixels and halfblock images
    pub fn render_force_clear(
        framework: &mut Framework,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.clear()?;
        terminal.draw(|frame| {
            Self::render_with_frame(framework, frame);
        })?;

        Ok(())
    }

    pub fn render_with_frame(
        framework: &mut Framework,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
    ) {
        let min_dimentions = framework.data.state.get::<MinDimentions>().unwrap();
        let area = frame.size();

        // if the minimum width and height is not meet, then displays a "protective screen" to prevent panicking
        if area.width < min_dimentions.width || area.height < min_dimentions.height {
            let paragraph = Paragraph::new(format!(
                "{}Current: {} x {}\nRequired: {} x {}",
                "\n".repeat(area.height as usize / 2 - 1),
                area.width,
                area.height,
                min_dimentions.width,
                min_dimentions.height
            ))
            .alignment(Alignment::Center)
            .style(
                Style::default().fg(framework
                    .data
                    .global
                    .get::<AppearanceConfig>()
                    .unwrap()
                    .colors
                    .text_error),
            );
            frame.render_widget(paragraph, area);
            return;
        }

        framework.render(frame);
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderTask {
    All,
    None,
    Only(Vec<(usize, usize)>),
}

/// priority will get executed first, last will get executed after priority queue finished
#[derive(Clone, Default)]
pub struct Tasks {
    pub priority: TaskQueue,
    pub last: TaskQueue,
}

impl Tasks {
    // clears and returns the `priority` task queue, if it is already cleared then returns the `last` task queue
    pub fn pop(&mut self) -> Option<TaskQueue> {
        if !self.priority.is_empty() {
            return Some(mem::take(&mut self.priority));
        }

        if !self.last.is_empty() {
            return Some(mem::take(&mut self.last));
        }

        None
    }
}

impl Key for Tasks {
    type Value = Self;
}
