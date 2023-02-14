use std::fs;

use super::ItemInfo;
use crate::{
    config::{AppearanceConfig, CommandsConfig, KeyBindingsConfig, MainConfig, Provider},
    global::{
        functions::{apply_envs, download_all_images, set_envs},
        structs::{
            InvidiousClient, Item, KeyAction, Message, Page, SingleItemPage, StateEnvs, Status,
            Task, Tasks, WatchHistory,
        },
    },
};
use home::home_dir;
use tui::{
    layout::{Constraint, Rect},
    style::Style,
};
use tui_additions::{
    framework::{FrameworkClean, FrameworkItem},
    widgets::{Grid, TextList},
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

/// variations that the struct can hold
#[derive(Clone)]
pub enum SingleItemType {
    None,
    Video(SingleVideoItem),
    Playlist(Box<SinglePlaylistItem>),
}

#[derive(Clone)]
pub struct SingleVideoItem {
    pub textlist: TextList,
    pub commands: Vec<(String, String)>,
}

#[derive(Clone)]
pub struct SinglePlaylistItem {
    pub commands_view: TextList,
    pub videos_view: TextList,
    pub commands: Vec<(String, String)>,
    pub is_commands_view: bool,
    pub hovered_video: ItemInfo,
}

impl SingleVideoItem {
    pub fn new(commands: &CommandsConfig) -> Self {
        Self::new_with_map(commands
                .video
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect())
    }

    pub fn new_local(commands: &CommandsConfig) -> Self {
        Self::new_with_map(commands
                .local_video
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect())
    }

    pub fn new_with_map(commands: Vec<(String, String)>) -> Self {
        Self {
            textlist: TextList::default()
                .items(
                    &commands
                        .iter()
                        .map(|command| &command.0)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            commands,
        }
    }

    /// find all occurances of ${provider}
    pub fn update_provider(&mut self) -> Vec<usize> {
        self.commands
            .iter()
            .enumerate()
            .filter(|(_index, (display, _command))| display.contains("${provider}"))
            .map(|(index, _)| index)
            .collect()
    }

    pub fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
    ) {
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

    /// creates a hashmap from `self`, containing info of the current item
    pub fn inflate_load(
        &self,
        mainconfig: &MainConfig,
        status: &Status,
        item: &Item,
    ) -> Vec<(String, String)> {
        let video_item = item.fullvideo().unwrap();

        vec![
            (
                String::from("url"),
                match status.provider {
                    Provider::Invidious => format!(
                        "{}/watch?v={}",
                        mainconfig.invidious_instance, video_item.id
                    ),
                    Provider::YouTube => format!("'https://youtu.be/{}'", video_item.id),
                },
            ),
            (String::from("id"), video_item.id.clone()),
            (
                String::from("embed-url"),
                match status.provider {
                    Provider::Invidious => {
                        format!("{}/embed/{}", mainconfig.invidious_instance, video_item.id)
                    }
                    Provider::YouTube => format!("'https://youtube.com/embed/{}'", video_item.id),
                },
            ),
            (String::from("channel-id"), video_item.channel_id.clone()),
            (
                String::from("channel-url"),
                match status.provider {
                    Provider::YouTube => {
                        format!("https://www.youtube.com/channel/{}", video_item.channel_id)
                    }
                    Provider::Invidious => format!(
                        "{}/channel/{}",
                        mainconfig.invidious_instance, video_item.channel_id
                    ),
                },
            ),
        ]
    }
}

impl SinglePlaylistItem {
    pub fn new(commands: &CommandsConfig, playlist_items: &[Item]) -> Self {
        Self::new_with_map(commands
                .playlist
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect(), playlist_items)
    }

    pub fn new_local(commands: &CommandsConfig, playlist_items: &[Item]) -> Self {
        Self::new_with_map(commands
                .local_playlist
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect(), playlist_items)
    }

    pub fn new_with_map(commands: Vec<(String, String)>, playlist_items: &[Item]) -> Self {
        let hovered_video = ItemInfo::new(if playlist_items.is_empty() {
            None
        } else {
            Some(playlist_items[0].clone())
        });

        Self {
            commands_view: TextList::default()
                .items(
                    &commands
                        .iter()
                        .map(|command| &command.0)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            videos_view: TextList::default()
                .items(&{
                    let mut items = vec!["Switch view"];
                    items.extend(
                        playlist_items
                            .iter()
                            .map(|item| item.minivideo().unwrap().title.as_str()),
                    );
                    items
                })
                .unwrap(),
            commands,
            hovered_video,
            is_commands_view: true,
        }
    }

    pub fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
        grid: &mut Grid,
    ) {
        if self.is_commands_view {
            grid.widths = vec![Constraint::Percentage(30), Constraint::Percentage(70)];
            self.commands_view.set_border_type(appearance.borders);
            self.commands_view
                .set_style(Style::default().fg(appearance.colors.text));

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
            grid.widths = if self.videos_view.selected == 0 {
                vec![Constraint::Percentage(30), Constraint::Percentage(70)]
            } else {
                vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ]
            };
            self.videos_view.set_border_type(appearance.borders);
            self.videos_view
                .set_style(Style::default().fg(appearance.colors.text));

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

    /// find all occurances of ${provider}
    pub fn update_provider(&mut self) -> Vec<usize> {
        self.commands
            .iter()
            .enumerate()
            .filter(|(_index, (display, _command))| display.contains("${provider}"))
            .map(|(index, _)| index)
            .collect()
    }

    /// creates a hashmap from `self`, containing info of the current item
    pub fn inflate_load(&self, item: &Item) -> Vec<(String, String)> {
        let playlist_item = item.fullplaylist().unwrap();

        vec![
            (String::from("id"), playlist_item.id.clone()),
            (String::from("channel-id"), playlist_item.channel_id.clone()),
            (
                String::from("all-ids"),
                playlist_item
                    .videos
                    .iter()
                    .map(|video| video.minivideo().unwrap().id.as_str())
                    .collect::<Vec<&str>>()
                    .join(" "),
            ),
        ]
    }
}

impl SingleItemType {
    /// self is none
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// update colours every render
    pub fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
        grid: &mut Grid,
    ) {
        match self {
            Self::None => {}
            Self::Playlist(playlistitem) => {
                playlistitem.update_appearance(appearance, iteminfo, grid)
            }
            Self::Video(videoitem) => videoitem.update_appearance(appearance, iteminfo),
        }
    }

    pub fn inflate_load(
        &self,
        mainconfig: &MainConfig,
        status: &Status,
        item: &Option<Item>,
    ) -> Vec<(String, String)> {
        let item = if let Some(item) = item.as_ref() {
            item
        } else {
            return Vec::new();
        };

        match self {
            Self::Video(singlevideoitem) => singlevideoitem.inflate_load(mainconfig, status, item),
            Self::Playlist(singleplaylistitem) => singleplaylistitem.inflate_load(item),
            Self::None => Vec::new(),
        }
    }
}

impl SingleItem {
    fn infalte_item_update(
        &self,
        mainconfig: &MainConfig,
        status: &Status,
    ) -> Vec<(String, String)> {
        if let SingleItemType::Playlist(singleplaylistitem) = &self.r#type {
            if singleplaylistitem.videos_view.selected == 0 {
                return vec![(String::from("hover-url"), String::from("not avaliable"))];
            }
            vec![(
                String::from("hover-url"),
                match &self.item {
                    Some(item) => format!(
                        "{}/watch?v={}",
                        match status.provider {
                            Provider::YouTube => "https://youtube.com",
                            Provider::Invidious => &mainconfig.invidious_instance,
                        },
                        item.fullplaylist().unwrap().videos
                            [singleplaylistitem.videos_view.selected - 1]
                            .minivideo()
                            .unwrap()
                            .id
                    ),
                    None => String::from("not avaliable"),
                },
            )]
        } else {
            vec![(String::from("hover-url"), String::from("not avaliable"))]
        }
    }
    /// update colours and layout every render
    fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
    ) {
        self.grid.set_border_type(appearance.borders);

        if iteminfo.selected {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_selected));
        } else if iteminfo.hover {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_hover));
        } else {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline));
        }

        self.r#type
            .update_appearance(appearance, iteminfo, &mut self.grid);
    }

    /// update hover item preview
    fn update(&mut self) {
        if let SingleItemType::Playlist(singleplaylistitem) = &mut self.r#type {
            let SinglePlaylistItem {
                hovered_video,
                videos_view,
                ..
            } = &mut **singleplaylistitem;
            if videos_view.items.is_empty() || videos_view.selected == 0 {
                hovered_video.item = None;
                return;
            }

            if hovered_video.item.is_none()
                || self.item.as_ref().unwrap().fullplaylist().unwrap().videos
                    [videos_view.selected - 1]
                    .id()
                    != hovered_video.item.as_ref().unwrap().id()
            {
                hovered_video.item = Some(
                    self.item.as_ref().unwrap().fullplaylist().unwrap().videos
                        [videos_view.selected - 1]
                        .clone(),
                );
            }
        }
    }

    /// handle enter presses
    fn select_at_cursor(
        &mut self,
        framework: &mut FrameworkClean,
        // info: tui_additions::framework::ItemInfo,
    ) {
        match &mut self.r#type {
            SingleItemType::Video(singlevideoitem) => {
                let command_string = singlevideoitem.commands[singlevideoitem.textlist.selected]
                    .1
                    .clone();

                // check if the command starts with an ':' which case should be captured
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::Command(apply_envs(command_string)));
            }
            SingleItemType::Playlist(singleplaylistitem) => {
                let command_string = singleplaylistitem.commands
                    [singleplaylistitem.commands_view.selected]
                    .1
                    .clone();

                // checks for special cases
                match command_string.as_str() {
                    "%switch-view%" => {
                        singleplaylistitem.is_commands_view = !singleplaylistitem.is_commands_view;
                        // self.update_appearance(
                        //     framework.data.global.get::<AppearanceConfig>().unwrap(),
                        //     &info,
                        // );
                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Success(String::from("Switched view"));
                    }
                    _ => {
                        // check if the command starts with an ':' which case should be captured
                        framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::Command(apply_envs(command_string)));
                    }
                };
            }
            _ => return,
        }

        framework
            .data
            .state
            .get_mut::<Tasks>()
            .unwrap()
            .priority
            .push(Task::RenderAll);
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
        let status = framework.data.global.get::<Status>().unwrap();
        if popup_render {
            return;
        }

        if status.provider_updated {
            set_envs(
                self.infalte_item_update(
                    framework.data.global.get::<MainConfig>().unwrap(),
                    status,
                )
                .into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );
        }

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        self.update_appearance(appearance, &info);

        let chunks = self.grid.chunks(area).unwrap()[0].clone();

        frame.render_widget(self.grid.clone(), area);

        // only continue if `self.item` is Some
        if self.item.is_none() {
            return;
        }

        match &mut self.r#type {
            SingleItemType::Video(typeinfo) => {
                // 2 by 1 grid, item info in the first cell and textlist at the second
                if status.provider_updated {
                    typeinfo.update_provider().into_iter().for_each(|index| {
                        typeinfo.textlist.items[index] = typeinfo.commands[index].0.clone().replace(
                            "${provider}",
                            framework
                                .data
                                .global
                                .get::<Status>()
                                .unwrap()
                                .provider
                                .as_str(),
                        )
                    });
                }
                self.iteminfo
                    .render(frame, framework, chunks[0], popup_render, info);
                typeinfo.textlist.set_height(chunks[1].height);
                frame.render_widget(typeinfo.textlist.clone(), chunks[1]);
            }
            SingleItemType::Playlist(typeinfo) => {
                // 3 by 1 grid if hovering a video inside the playlist
                // if not then 2 by 1
                //
                // item info in the first cell, textlists in the second, hovering video on 3rd (if
                // present)
                if typeinfo.is_commands_view {
                    if status.provider_updated {
                        typeinfo.update_provider().into_iter().for_each(|index| {
                            typeinfo.commands_view.items[index] =
                                typeinfo.commands[index].0.clone().replace(
                                    "${provider}",
                                    framework
                                        .data
                                        .global
                                        .get::<Status>()
                                        .unwrap()
                                        .provider
                                        .as_str(),
                                )
                        });
                    }
                    typeinfo.commands_view.set_height(chunks[1].height);
                    frame.render_widget(typeinfo.commands_view.clone(), chunks[1]);
                } else {
                    typeinfo.videos_view.set_height(chunks[1].height);
                    frame.render_widget(typeinfo.videos_view.clone(), chunks[1]);

                    if typeinfo.videos_view.selected != 0 {
                        typeinfo.hovered_video.render(
                            frame,
                            framework,
                            chunks[2],
                            popup_render,
                            info,
                        );
                    }
                }
                self.iteminfo
                    .render(frame, framework, chunks[0], popup_render, info);
            }
            SingleItemType::None => {}
        }
    }

    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        *self = Self::default();

        let page = framework.data.state.get::<Page>().unwrap();

        let r#type = if let Page::SingleItem(r#type) = page {
            r#type
        } else {
            unreachable!("item `SingleItem` cannot be used in {page:?}")
        };

        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        // load items using the invidious api
        // gets the item that it needs to load from `data.state.Page`
        self.item = match r#type {
            SingleItemPage::Video(id) => {
                let client = &framework.data.global.get::<InvidiousClient>().unwrap().0;
                self.r#type = SingleItemType::Video(SingleVideoItem::new(
                    framework.data.global.get::<CommandsConfig>().unwrap(),
                ));
                let video = Item::from_full_video(client.video(id, None)?, mainconfig.image_index);
                if mainconfig.images.display() {
                    download_all_images(vec![(&video).into()]);
                }
                Some(video)
            }
            SingleItemPage::Playlist(id) => {
                let client = &framework.data.global.get::<InvidiousClient>().unwrap().0;
                let playlist =
                    Item::from_full_playlist(client.playlist(id, None)?, mainconfig.image_index);
                let videos = &playlist.fullplaylist()?.videos;
                self.r#type = SingleItemType::Playlist(
                    SinglePlaylistItem::new(
                        framework.data.global.get::<CommandsConfig>().unwrap(),
                        videos,
                    )
                    .into(),
                );

                if mainconfig.images.display() {
                    download_all_images({
                        let mut items = videos.iter().map(|item| item.into()).collect::<Vec<_>>();
                        items.extend([(&playlist).into()].into_iter());
                        items
                    });
                }

                Some(playlist)
            }
            SingleItemPage::LocalVideo(id) => {
                let s = fs::read_to_string(home_dir().unwrap().join(format!(
                    ".local/share/youtube-tui/watch_history/info/{id}.json"
                )))?;
                self.r#type = SingleItemType::Video(SingleVideoItem::new_local(
                    framework.data.global.get::<CommandsConfig>().unwrap(),
                ));

                Some(serde_json::from_str(&s)?)
            }

            SingleItemPage::LocalPlaylist(id) => {
                let s = fs::read_to_string(home_dir().unwrap().join(format!(
                    ".local/share/youtube-tui/watch_history/info/{id}.json"
                )))?;

                let playlist: Item = serde_json::from_str(&s)?;
                let videos = &playlist.fullplaylist()?.videos;
                self.r#type = SingleItemType::Playlist(
                    SinglePlaylistItem::new_local(
                        framework.data.global.get::<CommandsConfig>().unwrap(),
                        videos,
                    )
                    .into(),
                );

                Some(playlist)
            }
        };
        self.iteminfo.item = self.item.clone();

        if let Some(item) = &self.item {
            if item.is_unknown() {
                return Ok(());
            }

            let item = item.clone();
            let max_watch_history = framework
                .data
                .global
                .get::<MainConfig>()
                .unwrap()
                .max_watch_history;

            // push to watch history
            let watch_history = framework.data.global.get_mut::<WatchHistory>().unwrap();
            watch_history.push(item, max_watch_history)?;
            watch_history.save()?;
        }

        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        // need to update provider every time the item loads or else it will display `${provider}`
        // instead of the actual provider (e.g. `YouTube`)
        set_envs(
            self.r#type
                .inflate_load(
                    mainconfig,
                    framework.data.global.get::<Status>().unwrap(),
                    &self.item,
                )
                .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );

        set_envs(
            self.infalte_item_update(mainconfig, framework.data.global.get::<Status>().unwrap())
                .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );

        Ok(())
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // discard any key inputs if `self.is_none()` becuase nothing can happen if self is none
        if self.r#type.is_none() {
            return Ok(());
        }

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

        let updated = match &mut self.r#type {
            SingleItemType::Video(singlevideoitem) => match action {
                // move the cursor in the textlist, only update the screen if it is changed
                KeyAction::MoveUp => singlevideoitem.textlist.up().is_ok(),
                KeyAction::MoveDown => singlevideoitem.textlist.down().is_ok(),
                KeyAction::MoveLeft => singlevideoitem.textlist.first().is_ok(),
                KeyAction::MoveRight => singlevideoitem.textlist.last().is_ok(),
                KeyAction::Select => {
                    self.select_at_cursor(framework);
                    return Ok(());
                }
                _ => false,
            },
            SingleItemType::Playlist(singleplaylistitem) => {
                // there are 2 possible states in a playlist item
                // they are handelled separately
                if singleplaylistitem.is_commands_view {
                    match action {
                        KeyAction::MoveUp => singleplaylistitem.commands_view.up().is_ok(),
                        KeyAction::MoveDown => singleplaylistitem.commands_view.down().is_ok(),
                        KeyAction::MoveLeft => singleplaylistitem.commands_view.first().is_ok(),
                        KeyAction::MoveRight => singleplaylistitem.commands_view.last().is_ok(),
                        KeyAction::Select => {
                            self.select_at_cursor(framework);
                            return Ok(());
                        }
                        _ => false,
                    }
                } else {
                    let updated = match action {
                        // checks if it is updated, if it is and selected is not 0 (is hovering on
                        // a video), then also need to update the iteminfo
                        KeyAction::MoveUp => {
                            if singleplaylistitem.videos_view.selected == 1 {
                                // going from a hovering video to not hovering will make the image
                                // stay on the screen, therefore it needs to be removed by clearing
                                // the screen
                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::ClearPage);
                            } else if singleplaylistitem.videos_view.selected == 0 {
                                return Ok(());
                            }

                            let updated = singleplaylistitem.videos_view.up().is_ok();
                            if singleplaylistitem.videos_view.selected != 0 {
                                singleplaylistitem.hovered_video.item = Some(
                                    self.item.as_ref().unwrap().fullplaylist()?.videos
                                        [singleplaylistitem.videos_view.selected - 1]
                                        .clone(),
                                );
                            }
                            updated
                        }
                        KeyAction::MoveDown => {
                            if singleplaylistitem.videos_view.selected
                                == singleplaylistitem.videos_view.items.len() - 1
                            {
                                return Ok(());
                            }

                            let updated = singleplaylistitem.videos_view.down().is_ok();
                            if updated && singleplaylistitem.videos_view.selected != 0 {
                                singleplaylistitem.hovered_video.item = Some(
                                    self.item.as_ref().unwrap().fullplaylist()?.videos
                                        [singleplaylistitem.videos_view.selected - 1]
                                        .clone(),
                                );
                            }
                            updated
                        }
                        KeyAction::MoveLeft => {
                            if singleplaylistitem.videos_view.selected != 0 {
                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::ClearPage);
                            } else if singleplaylistitem.videos_view.selected == 0 {
                                return Ok(());
                            }

                            let updated = singleplaylistitem.videos_view.first().is_ok();
                            singleplaylistitem.hovered_video.item = None;
                            updated
                        }
                        KeyAction::MoveRight => {
                            if singleplaylistitem.videos_view.selected
                                == singleplaylistitem.videos_view.items.len() - 1
                            {
                                return Ok(());
                            }
                            let updated = singleplaylistitem.videos_view.last().is_ok();
                            if updated && singleplaylistitem.videos_view.selected != 0 {
                                singleplaylistitem.hovered_video.item = Some(
                                    self.item.as_ref().unwrap().fullplaylist()?.videos
                                        [singleplaylistitem.videos_view.selected - 1]
                                        .clone(),
                                );
                            }
                            updated
                        }
                        KeyAction::Select => {
                            if singleplaylistitem.videos_view.selected == 0 {
                                singleplaylistitem.is_commands_view =
                                    !singleplaylistitem.is_commands_view;
                                self.update_appearance(
                                    framework.data.global.get::<AppearanceConfig>().unwrap(),
                                    &info,
                                );
                                *framework.data.global.get_mut::<Message>().unwrap() =
                                    Message::Success(String::from("Switched view"));
                            } else {
                                framework
                                    .data
                                    .state
                                    .get_mut::<Tasks>()
                                    .unwrap()
                                    .priority
                                    .push(Task::LoadPage(Page::SingleItem(SingleItemPage::Video(
                                        self.item.as_ref().unwrap().fullplaylist()?.videos
                                            [singleplaylistitem.videos_view.selected - 1]
                                            .minivideo()?
                                            .id
                                            .clone(),
                                    ))));
                            }

                            true
                        }
                        _ => false,
                    };

                    if updated {
                        framework
                            .data
                            .global
                            .get_mut::<Status>()
                            .unwrap()
                            .render_image = true;
                        set_envs(
                            self.infalte_item_update(
                                framework.data.global.get::<MainConfig>().unwrap(),
                                framework.data.global.get::<Status>().unwrap(),
                            )
                            .into_iter(),
                            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
                        );
                    }

                    updated
                }
            }
            SingleItemType::None => false,
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

        let textlist = match &mut self.r#type {
            SingleItemType::Video(SingleVideoItem { textlist, .. }) => textlist,
            SingleItemType::Playlist(singleplaylistitem) => {
                if singleplaylistitem.is_commands_view {
                    &mut singleplaylistitem.commands_view
                } else {
                    let y = (y - chunk.y) as usize + singleplaylistitem.videos_view.scroll;
                    if singleplaylistitem.videos_view.selected != 0 && y == 0 {
                        framework
                            .data
                            .state
                            .get_mut::<Tasks>()
                            .unwrap()
                            .priority
                            .push(Task::ClearPage);
                    }
                    &mut singleplaylistitem.videos_view
                }
            }
            _ => return false,
        };

        let y = (y - chunk.y) as usize + textlist.scroll;

        // clicking on already selected item
        if y == textlist.selected || y == textlist.selected + 2 || y == textlist.selected + 1 {
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
        }

        if y >= textlist.selected + 2 {
            textlist.selected = y - 2;
        }

        self.update();

        set_envs(
            self.infalte_item_update(
                framework.data.global.get::<MainConfig>().unwrap(),
                framework.data.global.get::<Status>().unwrap(),
            )
            .into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );

        // render the new image
        framework
            .data
            .global
            .get_mut::<Status>()
            .unwrap()
            .render_image = true;

        true
    }
}
