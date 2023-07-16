use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::Paragraph,
};
use tui_additions::{
    framework::{FrameworkClean, FrameworkItem},
    widgets::{Grid, TextList},
};
use typemap::Key;

use crate::{
    config::{AppearanceConfig, KeyBindingsConfig, MainConfig, Provider},
    global::{functions::set_envs, structs::*},
};

use super::{ItemInfo, VidSelect};

#[derive(Clone)]
pub struct ChannelList {
    pub selector: TextList,
    pub channel_display: ItemInfo,
    pub grid: Grid,
    pub channels: Vec<FullChannelItem>,
}

impl Default for ChannelList {
    fn default() -> Self {
        Self {
            selector: TextList::default(),
            channel_display: ItemInfo::default(),
            grid: Grid::new(
                vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                vec![Constraint::Percentage(100)],
            )
            .unwrap(),
            channels: Vec::new(),
        }
    }
}

impl ChannelList {
    fn update_appearance(
        &mut self,
        info: &tui_additions::framework::ItemInfo,
        appearance: &AppearanceConfig,
    ) {
        if info.selected {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_selected));
            self.selector
                .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
        } else if info.hover {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_hover));
            self.selector
                .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
        } else {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline));
            self.selector
                .set_cursor_style(Style::default().fg(appearance.colors.outline));
        }
    }

    fn select_at_cursor(&mut self, framework: &mut FrameworkClean) {
        let tasks = framework.data.state.get_mut::<Tasks>().unwrap();

        if self.selector.selected >= self.selector.items.len() {
            self.selector.last().unwrap();
            tasks.priority.push(Task::RenderAll);
        }
    }

    fn update_unread(&mut self, subscriptions: &Subscriptions) {
        self.selector
            .set_items(
                &[format!(
                    "All subscriptions{}",
                    if !subscriptions.0.is_empty()
                        && subscriptions.0.iter().any(|item| item.has_new)
                    {
                        "*"
                    } else {
                        ""
                    }
                )]
                .into_iter()
                .chain(
                    subscriptions.0.iter().map(|subtiem| {
                        format!("{subtiem}{}", if subtiem.has_new { "*" } else { "" })
                    }),
                )
                .collect::<Vec<_>>(),
            )
            .unwrap();
    }

    fn set_env(&self, framework: &mut FrameworkClean) {
        let id = if let Some(item) = &self.channel_display.item {
            item.id().unwrap().to_string()
        } else {
            "invalid".to_string()
        };
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        set_envs(
            [
                (
                    String::from("hover-channel-url"),
                    format!(
                        "{}channel/{id}",
                        match framework.data.global.get::<Status>().unwrap().provider {
                            Provider::YouTube => "https://youtube.com/",
                            Provider::Invidious => mainconfig.invidious_instance.as_str(),
                        }
                    ),
                ),
                (String::from("hover-channel-id"), id),
            ]
            .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        )
    }
}

impl FrameworkItem for ChannelList {
    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let subscriptions = framework.data.global.get::<Subscriptions>().unwrap();
        self.selector.set_border_type(appearance.borders);
        self.grid.set_border_type(appearance.borders);
        self.channels = subscriptions.get_channels();

        self.update_unread(subscriptions);

        if self.selector.selected >= self.selector.items.len() {
            self.selector.last()?;
        }

        Ok(())
    }

    fn render(
        &mut self,
        frame: &mut ratatui::Frame<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: ratatui::layout::Rect,
        popup_render: bool,
        info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();

        self.update_appearance(&info, appearance);
        let chunks = self.grid.chunks(area).unwrap()[0].clone();
        frame.render_widget(self.grid.clone(), area);
        self.selector.set_height(chunks[1].height);
        frame.render_widget(self.selector.clone(), chunks[1]);

        if self.selector.items.len() == 1 {
            frame.render_widget(
                Paragraph::new("Subscribe to some channels first, come back later\n\nHey, Siriusmart here. I originally planned to add a commands textlist at channel main pages so that you can subscribe to channels, but the complexity of this update is starting to get out of hand, as it requires the two items (channel and video list) to communicate with each other somehow. So for now the only ways you can subscribe to channels in single item page (videos or playlists), or run `youtube-tui help` to check out the related commands.\n\nThe rest will come in a few git commits.").wrap(ratatui::widgets::Wrap { trim: true }),
                chunks[0],
            );
            return;
        }
        if self.selector.selected == 0 {
            let now = chrono::Utc::now().timestamp() as u64;
            let subscriptions = framework.data.global.get_mut::<Subscriptions>().unwrap();
            let paragraph = subscriptions
                .0
                .iter()
                .map(|item| {
                    format!(
                        "  {} (last synced {} day{} ago){}",
                        item.channel.name,
                        (now - item.last_sync) / 86400,
                        if now - item.last_sync > 172800 {
                            "s"
                        } else {
                            ""
                        },
                        if item.has_new { "*" } else { "" }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            frame.render_widget(
                Paragraph::new(format!("Subscriptions (last sync):\n\n{paragraph}")),
                chunks[0],
            );
            return;
        }
        if self.channel_display.item.is_none() {
            frame.render_widget(
                Paragraph::new("Nothing to see here\n\nMaybe try something else"),
                chunks[0],
            );
            return;
        }
        self.channel_display
            .render(frame, framework, chunks[0], popup_render, info);
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = framework.data.state.get_mut::<Tasks>().unwrap();
        let previously_selected = self.selector.selected;

        let action = if let Some(action) = framework
            .data
            .global
            .get::<KeyBindingsConfig>()
            .unwrap()
            .get(key)
        {
            action
        } else {
            return Ok(());
        };
        match action {
            KeyAction::MoveDown if self.selector.down().is_ok() => {
                tasks.priority.push(Task::RenderAll)
            }
            KeyAction::MoveUp if self.selector.up().is_ok() => tasks.priority.push(Task::RenderAll),
            KeyAction::MoveLeft | KeyAction::First => {
                if self.selector.first().is_ok() {
                    tasks.priority.push(Task::RenderAll)
                }
            }
            KeyAction::MoveRight | KeyAction::End => {
                if self.selector.last().is_ok() {
                    tasks.priority.push(Task::RenderAll)
                }
            }
            KeyAction::Select => {
                self.select_at_cursor(framework);
                return Ok(());
            }
            _ => return Ok(()),
        }

        if self.selector.selected != previously_selected {
            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .storage
                .insert::<SubSelect>(SubSelect(self.selector.selected));

            if self.selector.selected == 0 {
                self.channel_display.item = None;
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::ClearPage);
            } else {
                if framework.data.state.get::<VidSelect>().unwrap().0 {
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::ClearPage);
                }
                self.channel_display.item = self
                    .channels
                    .get(self.selector.selected - 1)
                    .map(|channel| Item::FullChannel(channel.clone()));
                let subscriptions = framework.data.global.get_mut::<Subscriptions>().unwrap();
                let item = subscriptions.0.get_mut(self.selector.selected - 1);
                let mut found = false;
                match item {
                    Some(item)
                        if item.channel.id == self.channels[self.selector.selected - 1].id =>
                    {
                        if item.has_new {
                            item.has_new = false;
                            found = true;
                        }
                    }
                    _ => subscriptions.0.iter_mut().for_each(|item| {
                        if item.channel.id == self.channels[self.selector.selected - 1].id {
                            found = true;
                            item.has_new = false;
                        }
                    }),
                }

                if found {
                    self.update_unread(subscriptions);
                }
                self.set_env(framework);
            }

            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .render_image = true;

            return Ok(());
        }

        if self.selector.selected == 0 {
            return Ok(());
        }

        match framework
            .data
            .global
            .get::<Subscriptions>()
            .unwrap()
            .0
            .get(self.selector.selected - 1)
        {
            Some(item)
                if item.channel.id
                    != match &self.channel_display.item {
                        Some(displayed) => displayed.id().unwrap_or_default(),
                        None => "",
                    } => {}
            Some(_) => {}
            None => self.channel_display.item = None,
        };

        Ok(())
    }

    fn mouse_event(
        &mut self,
        framework: &mut FrameworkClean,
        x: u16,
        y: u16,
        _absolute_x: u16,
        _absolute_y: u16,
    ) -> bool {
        let chunk = self
            .grid
            .chunks(
                if let Some(prev_frame) = framework.data.global.get::<Status>().unwrap().prev_frame
                {
                    prev_frame
                } else {
                    return false;
                },
            )
            .unwrap()[0][1];

        if !chunk.intersects(Rect::new(x, y, 1, 1)) {
            return false;
        }

        let previously_selected = self.selector.selected;
        let y = (y - chunk.y) as usize + self.selector.scroll;

        if y == self.selector.selected
            || y == self.selector.selected + 2
            || y == self.selector.selected + 1
        {
            self.select_at_cursor(framework);
            return true;
        }

        // clicking on rows after the last item
        if y > self.selector.items.len() + 1 {
            return false;
        }

        // moving the cursor
        if y <= self.selector.selected {
            self.selector.selected = y;
        }

        if y >= self.selector.selected + 2 {
            self.selector.selected = y - 2;
        }

        if self.selector.selected == previously_selected {
            return false;
        }

        self.update(framework);
        self.set_env(framework);
        // render the new image
        let status = framework.data.global.get_mut::<Status>().unwrap();
        status.render_image = true;
        status
            .storage
            .insert::<SubSelect>(SubSelect(self.selector.selected));

        if framework.data.state.get::<VidSelect>().unwrap().0 {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::ClearPage);
        }
        true
    }
}

impl ChannelList {
    // change `self.item` to the currently selected item
    pub fn update(&mut self, framework: &mut FrameworkClean) {
        if self.selector.selected == 0 || self.channels.get(self.selector.selected - 1).is_none() {
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
            self.channel_display.item = None;
            return;
        }

        self.channel_display.item = Some(Item::FullChannel(
            self.channels[self.selector.selected - 1].clone(),
        ));
    }
}

#[derive(Clone, Copy, Default)]
pub struct SubSelect(pub usize);

impl Key for SubSelect {
    type Value = Self;
}
