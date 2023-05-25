use crossterm::event::KeyCode;
use tui::{layout::Constraint, style::Style};
use tui_additions::{
    framework::FrameworkItem,
    widgets::{Grid, TextList},
};

use crate::{config::AppearanceConfig, global::structs::*};

use super::ItemInfo;

#[derive(Clone)]
pub struct ChannelList {
    pub selector: TextList,
    pub channel_display: ItemInfo,
    pub grid: Grid,
}

impl Default for ChannelList {
    fn default() -> Self {
        Self {
            selector: TextList::default(),
            channel_display: ItemInfo::default(),
            grid: Grid::new(
                vec![Constraint::Percentage(40), Constraint::Percentage(60)],
                vec![Constraint::Percentage(100)],
            )
            .unwrap(),
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
}

impl FrameworkItem for ChannelList {
    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        self.selector.set_border_type(appearance.borders);
        self.grid.set_border_type(appearance.borders);

        self.selector.set_items(
            &["Sync all subscriptions".to_string()]
                .into_iter()
                .chain(
                    framework
                        .data
                        .global
                        .get::<Subscriptions>()
                        .unwrap()
                        .0
                        .iter()
                        .map(|subtiem| subtiem.to_string()),
                )
                .collect::<Vec<_>>(),
        )?;

        if self.selector.selected >= self.selector.items.len() {
            self.selector.last()?;
        }

        Ok(())
    }

    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
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
        match key.code {
            KeyCode::Down if self.selector.down().is_ok() => tasks.priority.push(Task::RenderAll),
            KeyCode::Up if self.selector.up().is_ok() => tasks.priority.push(Task::RenderAll),
            KeyCode::Left if self.selector.first().is_ok() => tasks.priority.push(Task::RenderAll),
            KeyCode::Right if self.selector.last().is_ok() => tasks.priority.push(Task::RenderAll),
            _ => return Ok(()),
        }

        if self.selector.selected == 0 {
            if previously_selected != 0 {
                self.channel_display.item = None;
                tasks.priority.push(Task::ClearPage);
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .render_image = true;
            }
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
                    } =>
            {
                self.channel_display.item = Some(Item::FullChannel(item.channel.clone()))
            }
            Some(_) => {}
            None => self.channel_display.item = None,
        };

        Ok(())
    }
}
