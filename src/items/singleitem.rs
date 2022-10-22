use std::collections::HashMap;
use tui::{layout::Constraint, style::Style};
use tui_additions::{
    framework::FrameworkItem,
    widgets::{Grid, TextList},
};
use super::ItemInfo;
use crate::{
    config::{AppearanceConfig, MainConfig, CommandsConfig},
    global::{structs::{InvidiousClient, Item, Message, Page, SingleItemPage}, functions::download_all_images}
};

impl Default for SingleItem {
    fn default() -> Self {
        Self {
            item: None,
            iteminfo: ItemInfo::default(),
            grid: Grid::new(
                vec![Constraint::Percentage(30), Constraint::Percentage(70)],
                vec![Constraint::Percentage(100)],
            )
            .unwrap(),
            r#type: SingleItemType::None,
        }
    }
}

/// main item in the `SingleItem(_)` page
#[derive(Clone)]
pub struct SingleItem {
    pub item: Option<Item>,
    pub iteminfo: ItemInfo,
    pub grid: Grid,
    pub r#type: SingleItemType,
}

#[derive(Clone)]
pub enum SingleItemType {
    None,
    Video(SingleVideoItem),
    Playlist(SinglePlaylistItem),
}

#[derive(Clone)]
pub struct SingleVideoItem {
    pub textlist: TextList,
    pub commands: HashMap<String, String>,
}

#[derive(Clone)]
pub struct SinglePlaylistItem {
    pub commands_view: TextList,
    pub videos_view: TextList,
    pub commands: HashMap<String, String>,
    pub is_commands_view: bool,
    pub hovered_video: ItemInfo,
}

impl SingleVideoItem {
    pub fn new(commands: &CommandsConfig) -> Self {
        Self {
            textlist: TextList::default().items(&commands.video.keys().collect()).unwrap(),
            commands: commands.video.clone(),
        }
    }

    pub fn update_appearance(&mut self, appearance: &AppearanceConfig, iteminfo: &tui_additions::framework::ItemInfo) {
        self.textlist.set_border_type(appearance.borders);
        self.textlist
            .set_style(Style::default().fg(appearance.colors.text));

        if iteminfo.selected {
            self.textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
            self.textlist
                .set_selected_style(Style::default().fg(appearance.colors.text_special));
        } else {
            self.textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
            self.textlist
                .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
        }
    }
}

impl SinglePlaylistItem {
    pub fn new(commands: &CommandsConfig, playlist_items: &Vec<Item>) -> Self {
        let mut hovered_video = ItemInfo::default();
        hovered_video.item = if playlist_items.is_empty() {
            None
        } else {
            Some(playlist_items[0].clone())
        };
        Self {
            commands_view: TextList::default().items(&commands.playlist.keys().collect()).unwrap(),
            videos_view: TextList::default().items(&playlist_items).unwrap(),
            commands: commands.playlist.clone(),
            hovered_video,
            is_commands_view: true,
        }
    }

    pub fn update_appearance(&mut self, appearance: &AppearanceConfig, iteminfo: &tui_additions::framework::ItemInfo, grid: &mut Grid) {
        if self.is_commands_view {
            grid.widths = vec![Constraint::Percentage(30), Constraint::Percentage(70)];
            self.commands_view.set_border_type(appearance.borders);
            self.commands_view.set_style(Style::default().fg(appearance.colors.text));

            if iteminfo.selected {
                self.commands_view
                    .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
                self.commands_view
                    .set_selected_style(Style::default().fg(appearance.colors.text_special));
            } else {
                self.commands_view
                    .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
                self.commands_view
                    .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
            }
        } else {
            grid.widths = vec![Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)];
            self.videos_view.set_border_type(appearance.borders);
            self.videos_view.set_style(Style::default().fg(appearance.colors.text));

            if iteminfo.selected {
                self.videos_view
                    .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
                self.videos_view
                    .set_selected_style(Style::default().fg(appearance.colors.text_special));
            } else {
                self.videos_view
                    .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
                self.videos_view
                    .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
            }
        }
    }
}

impl SingleItemType {
    pub fn update_appearance(&mut self, appearance: &AppearanceConfig, iteminfo: &tui_additions::framework::ItemInfo, grid: &mut Grid) {
        match self {
            Self::None => {},
            Self::Playlist(playlistitem) => playlistitem.update_appearance(appearance, iteminfo, grid),
            Self::Video(videoitem) => videoitem.update_appearance(appearance, iteminfo)
        }
    }
}

impl SingleItem {
    fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
    ) {
        self.grid.set_border_type(appearance.borders);

        if iteminfo.selected {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_selected));
        } else {
            if iteminfo.hover {
                self.grid
                    .set_border_style(Style::default().fg(appearance.colors.outline_hover));
            } else {
                self.grid
                    .set_border_style(Style::default().fg(appearance.colors.outline));
            }
        }

        self.r#type.update_appearance(appearance, iteminfo, &mut self.grid);
    }
}

impl FrameworkItem for SingleItem {
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
        self.update_appearance(appearance, &info);

        let chunks = self.grid.chunks(area).unwrap()[0].to_owned();

        frame.render_widget(self.grid.clone(), area);

        // only continue if `self.item` is Some
        if self.item.is_none() {
            return;
        }

        match &mut self.r#type {
            SingleItemType::Video(typeinfo) => {
                self.iteminfo
                    .render(frame, framework, chunks[0], popup_render, info);
                typeinfo.textlist.set_height(chunks[1].height);
                frame.render_widget(typeinfo.textlist.clone(), chunks[1]);
            }
            SingleItemType::Playlist(typeinfo) => {
                self.iteminfo
                    .render(frame, framework, chunks[0], popup_render, info);
                if typeinfo.is_commands_view {
                    typeinfo.commands_view.set_height(chunks[1].height);
                    frame.render_widget(typeinfo.commands_view.clone(), chunks[1]);
                } else {
                    typeinfo.videos_view.set_height(chunks[1].height);
                    frame.render_widget(typeinfo.videos_view.clone(), chunks[1]);
                    typeinfo.hovered_video.render(frame, framework, chunks[2], popup_render, info);
                }
            }
            SingleItemType::None => {}
        }
    }

    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let page = framework.data.state.get::<Page>().unwrap();

        let r#type = if let Page::SingleItem(r#type) = page {
            r#type
        } else {
            unreachable!("item `SingleItem` cannot be used in {page:?}")
        };

        let client = &framework.data.global.get::<InvidiousClient>().unwrap().0;
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();

        self.item = match r#type {
            SingleItemPage::Video(id) => match client.video(id, None) {
                Ok(video) => {
                    self.r#type = SingleItemType::Video(SingleVideoItem::new(framework.data.global.get::<CommandsConfig>().unwrap()));
                    let video = Item::from_full_video(video, mainconfig.image_index);
                    if mainconfig.images.display() {
                        download_all_images(vec![(&video).into()]);
                    }
                    Some(video)
                },
                Err(e) => {
                    *self = Self::default();
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(e.to_string());
                    None
                }
            },
            SingleItemPage::Playlist(id) => match client.playlist(id, None) {
                Ok(playlist) => {
                    let playlist = Item::from_full_playlist(playlist, mainconfig.image_index);
                    let videos = if let Item::FullPlaylist(playlist) = &playlist {
                        &playlist.videos
                    } else {
                        unreachable!();
                    };
                    self.r#type = SingleItemType::Playlist(SinglePlaylistItem::new(framework.data.global.get::<CommandsConfig>().unwrap(), videos));
                    if mainconfig.images.display() {
                        download_all_images(vec![(&playlist).into()]);
                    }

                    Some(playlist)
                },
                Err(e) => {
                    *self = Self::default();
                    *framework.data.global.get_mut::<Message>().unwrap() =
                        Message::Error(e.to_string());
                    None
                }
            }
        };
        self.iteminfo.item = self.item.clone();

        Ok(())
    }
}
