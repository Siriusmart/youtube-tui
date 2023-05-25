use crate::{
    config::*,
    global::{functions::*, structs::*},
};

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
    LazyRendered,
    Command(String),
}

/// multiple tasks joined together, with duplicates removed
#[derive(Clone, PartialEq, Eq)]
pub struct TaskQueue {
    pub render: RenderTask,
    pub reload: bool,
    pub load_page: Option<Page>,
    pub clear_all: bool,
    pub lazy_rendered: bool,
    pub commands: Vec<String>,
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self {
            render: RenderTask::None,
            reload: false,
            load_page: None,
            clear_all: false,
            lazy_rendered: false,
            commands: Vec::new(),
        }
    }
}

impl TaskQueue {
    /// add task to queue
    pub fn push(&mut self, task: Task) {
        match task {
            Task::RenderAll => self.render = RenderTask::All,
            Task::Reload => self.reload = true,
            Task::RenderOnly(x, y) => match &mut self.render {
                RenderTask::Only(renders) => {
                    if !renders.contains(&(x, y)) {
                        renders.push((x, y));
                    }
                }
                RenderTask::None => self.render = RenderTask::Only(vec![(x, y)]),
                _ => {}
            },
            Task::LoadPage(page) => self.load_page = Some(page),
            Task::ClearPage => self.clear_all = true,
            Task::LazyRendered => self.lazy_rendered = true,
            Task::Command(s) => self.commands.push(s),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// runs all tasks in a task queue
    pub fn run(
        mut self,
        framework: &mut Framework,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        // if there is any commands, run them first as they may modify data, which is rendered
        // later in this function
        for command in self.commands.iter() {
            run_command(command, framework, terminal);
        }

        if self.clear_all {
            terminal.clear()?;
        }

        if self.lazy_rendered {
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .render_image = false;
        }

        // save state in history, then replace all items by whats in the new page and run `.load()` on them
        if let Some(page) = self.load_page {
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(page.load_msg(framework));
            framework.push_history();

            // clear all envs modified by the previous state (keeping ones that are there when the
            // program launches), then add the envs set in main config
            clear_envs(&mut framework.data.state.get_mut::<StateEnvs>().unwrap().0);
            set_envs(
                framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .env
                    .clone()
                    .into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );
            // reset cursor position
            framework.cursor = CursorState::default();

            let page_config = page.to_page_config(framework);
            *framework.data.state.get_mut::<MinDimentions>().unwrap() =
                MinDimentions::new(page_config.min_width(), page_config.min_height());

            let state = page_config.to_state(framework);
            framework.set_state(state);
            framework.data.global.get_mut::<Status>().unwrap().reset();
            *framework.data.state.get_mut::<Page>().unwrap() = page;
            Self::render_force_clear(framework, terminal)?;
            *framework.data.global.get_mut::<Message>().unwrap() = Message::None;

            *framework.data.global.get_mut::<Message>().unwrap() = if let Err(e) = framework.load()
            {
                Message::Error(e.to_string())
            } else {
                Message::None
            };
            let status = framework.data.global.get_mut::<Status>().unwrap();
            status.provider_updated = true;
            status.render_image = true;
            self.render = RenderTask::All;
        }

        if self.reload {
            // same as in loadpage
            clear_envs(&mut framework.data.state.get_mut::<StateEnvs>().unwrap().0);
            set_envs(
                framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .env
                    .clone()
                    .into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );

            // reload simply runs `.load()` on all items
            *framework.data.global.get_mut::<Message>().unwrap() =
                Message::Message(String::from("Reloading page..."));
            Self::render_force_clear(framework, terminal)?;
            *framework.data.global.get_mut::<Message>().unwrap() = if let Err(e) = framework.load()
            {
                Message::Error(e.to_string())
            } else {
                Message::None
            };
            let status = framework.data.global.get_mut::<Status>().unwrap();
            status.provider_updated = true;
            status.render_image = true;
            self.render = RenderTask::All;
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

        if framework
            .data
            .global
            .get::<Status>()
            .unwrap()
            .provider_updated
        {
            update_provider(&mut framework.data);
        }

        Ok(())
    }

    /// the render task runs this function
    pub fn render(
        framework: &mut Framework,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|frame| {
            Self::render_with_frame(framework, frame);
        })?;

        Ok(())
    }

    /// this function force clears the terminal before rendering, removing sixels and halfblock images
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

    /// this function renders onto the given frame
    pub fn render_with_frame(
        framework: &mut Framework,
        frame: &mut Frame<CrosstermBackend<Stdout>>,
    ) {
        let min_dimentions = framework.data.state.get::<MinDimentions>().unwrap();
        let area = frame.size();

        framework
            .data
            .global
            .get_mut::<Status>()
            .unwrap()
            .prev_frame = Some(area);

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

/// a single task
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
