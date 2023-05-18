use super::ItemInfo;
use crate::{
    config::*,
    global::{functions::*, structs::*},
};
use tui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders},
};
use tui_additions::{
    framework::{FrameworkClean, FrameworkItem},
    widgets::{Grid, TextList},
};

/// the 4 pages that a channel has (including the default "blank" page when loading)
#[derive(Clone)]
pub enum ChannelDisplay {
    /// a blank item, will turn into one of the other variants when `.load()` depending on the page
    None,
    /// main channel display page
    Main { channel: Item, iteminfo: ItemInfo },
    /// latest videos
    Videos {
        videos: Vec<Item>,
        textlist: TextList,
        iteminfo: ItemInfo,
        grid: Grid,
    },
    /// created playlists
    Playlists {
        playlists: Vec<Item>,
        textlist: TextList,
        iteminfo: ItemInfo,
        grid: Grid,
    },
}

impl Default for ChannelDisplay {
    fn default() -> Self {
        Self::None
    }
}

impl ChannelDisplay {
    fn infalte_item_update(
        &self,
        mainconfig: &MainConfig,
        status: &Status,
    ) -> Vec<(String, String)> {
        match self {
            ChannelDisplay::Videos {
                videos, textlist, ..
            } => vec![(
                String::from("hover-url"),
                format!(
                    "{}/watch?v={}",
                    match status.provider {
                        Provider::YouTube => "https://youtube.com",
                        Provider::Invidious => &mainconfig.invidious_instance,
                    },
                    videos[textlist.selected].id().unwrap_or_default()
                ),
            )],
            ChannelDisplay::Playlists {
                playlists,
                textlist,
                ..
            } => vec![(
                String::from("hover-url"),
                format!(
                    "{}/playlist?list={}",
                    match status.provider {
                        Provider::YouTube => "https://youtube.com",
                        Provider::Invidious => &mainconfig.invidious_instance,
                    },
                    playlists[textlist.selected].id().unwrap_or_default()
                ),
            )],
            _ => Vec::new(),
        }
    }
    /// update the style of the item (colours, etc), ran on ever render
    fn update_appearance(
        &mut self,
        info: &tui_additions::framework::ItemInfo,
        appearance: &AppearanceConfig,
    ) {
        // is runs on every render
        match self {
            ChannelDisplay::Videos { textlist, grid, .. } => {
                if info.selected {
                    textlist
                        .set_selected_style(Style::default().fg(appearance.colors.text_special));
                    textlist.set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
                    grid.set_border_style(Style::default().fg(appearance.colors.outline_selected));
                } else {
                    if info.hover {
                        grid.set_border_style(Style::default().fg(appearance.colors.outline_hover));
                    } else {
                        grid.set_border_style(Style::default().fg(appearance.colors.outline));
                    }
                    textlist
                        .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
                    textlist
                        .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
                }
            }
            ChannelDisplay::Playlists { textlist, grid, .. } => {
                if info.selected {
                    textlist
                        .set_selected_style(Style::default().fg(appearance.colors.text_special));
                    textlist.set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
                    grid.set_border_style(Style::default().fg(appearance.colors.outline_selected));
                } else {
                    if info.hover {
                        grid.set_border_style(Style::default().fg(appearance.colors.outline_hover));
                    } else {
                        grid.set_border_style(Style::default().fg(appearance.colors.outline));
                    }
                    textlist
                        .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
                    textlist
                        .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
                }
            }
            _ => {}
        }
    }

    /// handles when select (enter) is pressed, generally loads the hovered item in a
    /// `SingleItemPage`
    fn select_at_cursor(&self, framework: &mut FrameworkClean) {
        match self {
            Self::None | Self::Main { .. } => {}
            Self::Videos {
                videos, textlist, ..
            } => {
                // on select loads that in singleitem
                if !videos.is_empty() {
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::LoadPage(Page::SingleItem(SingleItemPage::Video(
                            videos[textlist.selected].minivideo().unwrap().id.clone(),
                        ))));
                } else {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("There is nothing to select"));
                }
            }
            Self::Playlists {
                playlists,
                textlist,
                ..
            } => {
                if !playlists.is_empty() {
                    framework
                        .data
                        .state
                        .get_mut::<Tasks>()
                        .unwrap()
                        .priority
                        .push(Task::LoadPage(Page::SingleItem(SingleItemPage::Playlist(
                            playlists[textlist.selected]
                                .miniplaylist()
                                .unwrap()
                                .id
                                .clone(),
                        ))));
                } else {
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(String::from("There is nothing to select"));
                }
            }
        }
    }

    /// updates the video/playlist preview
    fn update(&mut self) {
        match self {
            Self::Videos {
                videos: items,
                textlist,
                iteminfo,
                ..
            }
            | Self::Playlists {
                playlists: items,
                textlist,
                iteminfo,
                ..
            } => {
                if !items.is_empty()
                    && items[textlist.selected].id() != iteminfo.item.as_ref().unwrap().id()
                {
                    iteminfo.item = Some(items[textlist.selected].clone())
                }
            }
            _ => {}
        }
    }

    /// check if self should be able to be selected
    pub fn selectable(&self) -> bool {
        !(matches!(self, Self::Main { .. }) || matches!(self, Self::None))
    }
}

impl FrameworkItem for ChannelDisplay {
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
        let border_style = Style::default().fg(if info.hover {
            appearance.colors.outline_hover
        } else if info.selected {
            appearance.colors.outline_selected
        } else {
            appearance.colors.outline
        });

        // matches itself to render differently depending on the enum variation
        match self {
            Self::None => {
                // the block is created when rendered because its quite simple and should not be a
                // performance issue
                let block = Block::default()
                    .border_type(appearance.borders)
                    .borders(Borders::ALL)
                    .border_style(border_style);
                frame.render_widget(block, area);
            }
            Self::Main { iteminfo, .. } => {
                let block = Block::default()
                    .border_type(appearance.borders)
                    .borders(Borders::ALL)
                    .border_style(border_style);
                let inner = block.inner(area);

                frame.render_widget(block, area);
                iteminfo.render(frame, framework, inner, popup_render, info);
            }
            Self::Videos {
                textlist,
                iteminfo,
                grid,
                ..
            }
            | Self::Playlists {
                textlist,
                iteminfo,
                grid,
                ..
            } => {
                let inner = &grid.chunks(area).unwrap()[0];

                frame.render_widget(grid.clone(), area);
                textlist.set_height(inner[0].height);
                frame.render_widget(textlist.clone(), inner[0]);
                iteminfo.render(frame, framework, inner[1], popup_render, info);
            }
        }
    }

    fn select(&mut self, _framework: &mut tui_additions::framework::FrameworkClean) -> bool {
        self.selectable()
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        match self {
            Self::Videos {
                videos,
                textlist,
                iteminfo,
                ..
            } => {
                let updated = match action {
                    KeyAction::MoveUp => textlist.up().is_ok(),
                    KeyAction::MoveDown => textlist.down().is_ok(),
                    KeyAction::MoveLeft => textlist.first().is_ok(),
                    KeyAction::MoveRight => textlist.last().is_ok(),
                    KeyAction::Select => {
                        self.select_at_cursor(framework);
                        return Ok(());
                    }
                    _ => false,
                };

                // only update iteminfo (it requires cloning) if changed and not empty
                if updated && !videos.is_empty() {
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
                    iteminfo.item = Some(videos[textlist.selected].clone());
                    set_envs(
                        self.infalte_item_update(
                            framework.data.global.get::<MainConfig>().unwrap(),
                            framework.data.global.get::<Status>().unwrap(),
                        )
                        .into_iter(),
                        &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
                    );
                }
            }
            Self::Playlists {
                playlists,
                textlist,
                iteminfo,
                ..
            } => {
                let updated = match action {
                    KeyAction::MoveUp => textlist.up().is_ok(),
                    KeyAction::MoveDown => textlist.down().is_ok(),
                    KeyAction::MoveLeft => textlist.first().is_ok(),
                    KeyAction::MoveRight => textlist.last().is_ok(),
                    KeyAction::Select => {
                        self.select_at_cursor(framework);
                        return Ok(());
                    }
                    _ => false,
                };

                if updated && !playlists.is_empty() {
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
                    iteminfo.item = Some(playlists[textlist.selected].clone());
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let page = framework.data.state.get::<Page>().unwrap().channeldisplay();

        match page.r#type {
            ChannelDisplayPageType::Main => {
                let channel = Item::from_full_channel(
                    framework
                        .data
                        .global
                        .get::<InvidiousClient>()
                        .unwrap()
                        .0
                        .channel(&page.id, None)?,
                    mainconfig.image_index,
                );
                if mainconfig.images.display() {
                    download_all_images(vec![(&channel).into()]);
                }
                *self = Self::Main {
                    iteminfo: ItemInfo::new(Some(channel.clone())),
                    channel,
                }
            }
            ChannelDisplayPageType::Videos => {
                let videos = framework
                    .data
                    .global
                    .get::<InvidiousClient>()
                    .unwrap()
                    .0
                    .channel_videos(&page.id, None)?
                    .videos
                    .into_iter()
                    .map(|video| Item::from_channel_video(video, mainconfig.image_index))
                    .collect::<Vec<_>>();
                if mainconfig.images.display() {
                    download_all_images(videos.iter().map(|item| item.into()).collect());
                }
                *self = Self::Videos {
                    textlist: TextList::default()
                        .ascii_only(!mainconfig.allow_unicode)
                        .border_type(appearance.borders)
                        .style(Style::default().fg(appearance.colors.text))
                        .items(&videos)?,
                    iteminfo: ItemInfo::new(videos.first().cloned()),
                    grid: Grid::new(
                        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
                        vec![Constraint::Percentage(100)],
                    )?
                    .border_type(appearance.borders),
                    videos,
                };
            }
            ChannelDisplayPageType::Playlists => {
                let playlists = framework
                    .data
                    .global
                    .get::<InvidiousClient>()
                    .unwrap()
                    .0
                    .channel_playlists(&page.id, None)?
                    .playlists
                    .into_iter()
                    .map(Item::from_channel_playlist)
                    .collect::<Vec<_>>();
                if mainconfig.images.display() {
                    download_all_images(playlists.iter().map(|item| item.into()).collect());
                }
                *self = Self::Playlists {
                    textlist: TextList::default()
                        .ascii_only(!mainconfig.allow_unicode)
                        .border_type(appearance.borders)
                        .style(Style::default().fg(appearance.colors.text))
                        .items(&playlists)?,
                    iteminfo: ItemInfo::new(playlists.first().cloned()),
                    grid: Grid::new(
                        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
                        vec![Constraint::Percentage(100)],
                    )?
                    .border_type(appearance.borders),
                    playlists,
                };
            }
        }

        set_envs(
            self.infalte_item_update(mainconfig, framework.data.global.get::<Status>().unwrap())
                .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );

        Ok(())
    }

    fn mouse_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        x: u16,
        y: u16,
        _absolute_x: u16,
        _absolute_y: u16,
    ) -> bool {
        match self {
            Self::None | Self::Main { .. } => return false,
            Self::Videos { textlist, grid, .. } | Self::Playlists { textlist, grid, .. } => {
                let chunk = grid
                    .chunks(
                        if let Some(prev_frame) =
                            framework.data.global.get::<Status>().unwrap().prev_frame
                        {
                            prev_frame
                        } else {
                            return false;
                        },
                    )
                    .unwrap()[0][0];

                if !chunk.intersects(Rect::new(x, y, 1, 1)) {
                    return false;
                }

                let y = (y - chunk.y) as usize + textlist.scroll;

                // clicking on already selected item
                if y == textlist.selected
                    || y == textlist.selected + 2
                    || y == textlist.selected + 1
                {
                    self.select_at_cursor(framework);
                    return true;
                }

                // clicking on rows after the last item
                if y > textlist.items.len() + 1 {
                    return false;
                }

                // moving the cursor
                if y <= textlist.selected {
                    textlist.selected = y;
                } else if y >= textlist.selected + 2 {
                    textlist.selected = y - 2;
                }

                self.update();

                // render the new image
                framework
                    .data
                    .global
                    .get_mut::<Status>()
                    .unwrap()
                    .render_image = true;
            }
        }
        set_envs(
            self.infalte_item_update(
                framework.data.global.get::<MainConfig>().unwrap(),
                framework.data.global.get::<Status>().unwrap(),
            )
            .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );

        true
    }
}
