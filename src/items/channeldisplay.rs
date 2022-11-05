use super::ItemInfo;
use crate::{
    config::{AppearanceConfig, KeyBindingsConfig, MainConfig},
    global::{
        functions::download_all_images,
        structs::{
            ChannelDisplayPageType, InvidiousClient, Item, KeyAction, Page, SingleItemPage, Task,
            Tasks, Message,
        },
    },
};
use tui::{
    layout::Constraint,
    style::Style,
    widgets::{Block, Borders},
};
use tui_additions::{
    framework::FrameworkItem,
    widgets::{Grid, TextList},
};

/// the 4 pages that a channel has (including the default "blank" page when loading)
#[derive(Clone)]
pub enum ChannelDisplay {
    None,
    Main {
        channel: Item,
    },
    Videos {
        videos: Vec<Item>,
        textlist: TextList,
        iteminfo: ItemInfo,
        grid: Grid,
    },
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
            Self::Main { channel } => {
                let block = Block::default()
                    .border_type(appearance.borders)
                    .borders(Borders::ALL)
                    .border_style(border_style);
                let inner = block.inner(area);

                frame.render_widget(block, area);
                ItemInfo {
                    item: Some(channel.clone()),
                }
                .render(frame, framework, inner, popup_render, info);
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
        let action = if let Some(keyactions) = framework
            .data
            .global
            .get::<KeyBindingsConfig>()
            .unwrap()
            .0
            .get(&key.code)
        {
            if let Some(action) = keyactions.get(&key.modifiers.bits()) {
                *action
            } else {
                return Ok(());
            }
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
                        // on select loads that in singleitem
                        if !videos.is_empty() {
                            framework
                                .data
                                .state
                                .get_mut::<Tasks>()
                                .unwrap()
                                .priority
                                .push(Task::LoadPage(Page::SingleItem(SingleItemPage::Video(
                                    videos[textlist.selected].minivideo().id.clone(),
                                ))));
                        } else {
                            *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(String::from("There is nothing to select"));
                        }
                        true
                    },
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
                    iteminfo.item = Some(videos[textlist.selected].clone());
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
                        if !playlists.is_empty() {
                            framework
                                .data
                                .state
                                .get_mut::<Tasks>()
                                .unwrap()
                                .priority
                                .push(Task::LoadPage(Page::SingleItem(SingleItemPage::Playlist(
                                    playlists[textlist.selected].miniplaylist().id.clone(),
                                ))));
                        } else {
                            *framework.data.global.get_mut::<Message>().unwrap() = Message::Error(String::from("There is nothing to select"));
                        }
                        true
                    },
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
                download_all_images(vec![(&channel).into()]);
                *self = Self::Main { channel }
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
                download_all_images(videos.iter().map(|item| item.into()).collect());
                *self = Self::Videos {
                    textlist: TextList::default()
                        .ascii_only(!mainconfig.allow_unicode)
                        .border_type(appearance.borders)
                        .style(Style::default().fg(appearance.colors.text))
                        .items(&videos)?,
                    iteminfo: ItemInfo {
                        item: videos.first().cloned(),
                    },
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
                download_all_images(playlists.iter().map(|item| item.into()).collect());
                *self = Self::Playlists {
                    textlist: TextList::default()
                        .ascii_only(!mainconfig.allow_unicode)
                        .border_type(appearance.borders)
                        .style(Style::default().fg(appearance.colors.text))
                        .items(&playlists)?,
                    iteminfo: ItemInfo {
                        item: playlists.first().cloned(),
                    },
                    grid: Grid::new(
                        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
                        vec![Constraint::Percentage(100)],
                    )?
                    .border_type(appearance.borders),
                    playlists,
                };
            }
        }

        Ok(())
    }
}
