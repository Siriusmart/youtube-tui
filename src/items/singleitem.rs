use super::ItemInfo;
use crate::{
    config::{AppearanceConfig, CommandsConfig, KeyBindingsConfig, MainConfig, Provider},
    global::{
        functions::download_all_images,
        structs::{
            InvidiousClient, Item, KeyAction, Message, Page, SingleItemPage, Status, Task, Tasks,
            WatchHistory,
        },
    },
};
use std::{collections::HashMap, thread};
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
    pub provider: Provider,
}

#[derive(Clone)]
pub struct SinglePlaylistItem {
    pub commands_view: TextList,
    pub videos_view: TextList,
    pub commands: Vec<(String, String)>,
    pub is_commands_view: bool,
    pub hovered_video: ItemInfo,
    pub provider: Provider,
}

impl SingleVideoItem {
    pub fn new(commands: &CommandsConfig, mainconfig: &MainConfig) -> Self {
        Self {
            textlist: TextList::default()
                .items(
                    &commands
                        .video
                        .iter()
                        .map(|command| &command.0)
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            commands: commands
                .video
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect(),
            provider: mainconfig.provider,
        }
    }

    /// updates the %switch-provider% command in the list
    pub fn update_provider(&mut self) {
        self.commands
            .iter()
            .enumerate()
            .for_each(|(index, (display, command))| {
                if command.as_str() == "%switch-provider%" {
                    self.textlist.items[index] =
                        display.replace("${provider}", self.provider.as_str());
                }
            })
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
    pub fn inflate_map(&self, mainconfig: &MainConfig, item: &Item) -> HashMap<String, String> {
        let video_item = item.fullvideo().unwrap();

        HashMap::from([
            (
                String::from("url"),
                match self.provider {
                    Provider::Invidious => format!(
                        "'{}/watch?v={}'",
                        mainconfig.invidious_instance, video_item.id
                    ),
                    Provider::YouTube => format!("'https://youtu.be/{}'", video_item.id),
                },
            ),
            (String::from("id"), video_item.id.clone()),
            (
                String::from("embed-url"),
                match self.provider {
                    Provider::Invidious => format!(
                        "'{}/embed/{}'",
                        mainconfig.invidious_instance, video_item.id
                    ),
                    Provider::YouTube => format!("'https://youtube.com/embed/{}'", video_item.id),
                },
            ),
            (String::from("channel_id"), video_item.channel_id.clone()),
            (
                String::from("channel_url"),
                match self.provider {
                    Provider::YouTube => {
                        format!("https://www.youtube.com/channel/{}", video_item.channel_id)
                    }
                    Provider::Invidious => format!(
                        "{}/channel/{}",
                        mainconfig.invidious_instance, video_item.channel_id
                    ),
                },
            ),
        ])
    }

    /// add "env variables" from `self.inflate_map` to an existing hashmap
    pub fn inflate(
        &self,
        mut env: HashMap<String, String>,
        mainconfig: &MainConfig,
        item: &Item,
    ) -> HashMap<String, String> {
        env.extend(self.inflate_map(mainconfig, item));
        env
    }
}

impl SinglePlaylistItem {
    pub fn new(
        commands: &CommandsConfig,
        mainconfig: &MainConfig,
        playlist_items: &[Item],
    ) -> Self {
        let hovered_video = ItemInfo::new(if playlist_items.is_empty() {
            None
        } else {
            Some(playlist_items[0].clone())
        });
        Self {
            commands_view: TextList::default()
                .items(
                    &commands
                        .playlist
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
            commands: commands
                .playlist
                .clone()
                .into_iter()
                .map(|(display, command)| (display, command))
                .collect(),
            hovered_video,
            is_commands_view: true,
            provider: mainconfig.provider,
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

    /// updates the %switch-provider% command in the list
    pub fn update_provider(&mut self) {
        self.commands
            .iter()
            .enumerate()
            .for_each(|(index, (display, command))| {
                if command.as_str() == "%switch-provider%" {
                    self.commands_view.items[index] =
                        display.replace("${provider}", self.provider.as_str());
                }
            })
    }

    /// creates a hashmap from `self`, containing info of the current item
    pub fn inflate_map(&self, mainconfig: &MainConfig, item: &Item) -> HashMap<String, String> {
        let playlist_item = item.fullplaylist().unwrap();
        HashMap::from([
            (
                String::from("url"),
                match self.provider {
                    Provider::YouTube => {
                        format!("https://www.youtube.com/playlist?list={}", playlist_item.id)
                    }
                    Provider::Invidious => format!(
                        "{}/playlist?list={}",
                        mainconfig.invidious_instance, playlist_item.id
                    ),
                },
            ),
            (String::from("id"), playlist_item.id.clone()),
            (
                String::from("all-videos"),
                match self.provider {
                    Provider::YouTube => playlist_item
                        .videos
                        .iter()
                        .map(|video| {
                            format!("'https://youtu.be/{}'", video.minivideo().unwrap().id)
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                    Provider::Invidious => playlist_item
                        .videos
                        .iter()
                        .map(|video| {
                            format!(
                                "'{}/watch?v={}'",
                                mainconfig.invidious_instance,
                                video.minivideo().unwrap().id
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                },
            ),
            (String::from("channel_id"), playlist_item.channel_id.clone()),
            (
                String::from("channel_url"),
                match self.provider {
                    Provider::YouTube => format!(
                        "https://www.youtube.com/channel/{}",
                        playlist_item.channel_id
                    ),
                    Provider::Invidious => format!(
                        "{}/channel/{}",
                        mainconfig.invidious_instance, playlist_item.channel_id
                    ),
                },
            ),
        ])
    }

    /// add "env variables" from `self.inflate_map` to an existing hashmap
    pub fn inflate(
        &self,
        mut env: HashMap<String, String>,
        mainconfig: &MainConfig,
        item: &Item,
    ) -> HashMap<String, String> {
        env.extend(self.inflate_map(mainconfig, item));
        env
    }
}

impl SingleItemType {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

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

    fn update(&mut self) {
        if let SingleItemType::Playlist(singleplaylistitem) = &mut self.r#type {
            let SinglePlaylistItem {
                hovered_video,
                videos_view,
                ..
            } = &mut **singleplaylistitem;
            if videos_view.items.is_empty() {
                hovered_video.item = None;
                return;
            }

            if self.item.as_ref().unwrap().fullplaylist().unwrap().videos[videos_view.selected].id()
                != hovered_video.item.as_ref().unwrap().id()
            {
                hovered_video.item = Some(
                    self.item.as_ref().unwrap().fullplaylist().unwrap().videos
                        [videos_view.selected]
                        .clone(),
                );
            }
        }
    }

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

                match command_string.as_str() {
                    // checks for special cases that the items should consume the command
                    // instead of running it
                    "%switch-provider%" => {
                        singlevideoitem.provider.rotate();
                        singlevideoitem.update_provider();
                        *framework.data.global.get_mut::<Message>().unwrap() = Message::Success(
                            format!("Switched provider to {}", singlevideoitem.provider.as_str()),
                        );
                    }
                    _ => {
                        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
                        // joins the env from the one in mainconfig and video info
                        let env = singlevideoitem.inflate(
                            mainconfig.env.clone(),
                            mainconfig,
                            self.item.as_ref().unwrap(),
                        );
                        let command_string = apply_env(command_string, &env);

                        // check if the command starts with an ':' which case should be captured
                        if &command_string[0..1] == ":" {
                            framework
                                .data
                                .state
                                .get_mut::<Tasks>()
                                .unwrap()
                                .priority
                                .push(Task::Command(command_string[1..].to_string()))
                        }

                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Success(command_string.clone());

                        // this allows creating commands from string
                        let mut command = execute::command(command_string);

                        // run the command in a new thread so it doesn't freeze the current one
                        thread::spawn(move || {
                            let _ = command.output();
                        });
                    }
                }
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
                    "%switch-provider%" => {
                        singleplaylistitem.provider.rotate();
                        singleplaylistitem.update_provider();
                    }
                    // same as before if string is not a special case then the run the command
                    _ => {
                        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
                        let env = singleplaylistitem.inflate(
                            mainconfig.env.clone(),
                            mainconfig,
                            self.item.as_ref().unwrap(),
                        );
                        let command_string = apply_env(command_string, &env);

                        // check if the command starts with an ':' which case should be captured
                        if &command_string[0..1] == ":" {
                            framework
                                .data
                                .state
                                .get_mut::<Tasks>()
                                .unwrap()
                                .priority
                                .push(Task::Command(command_string[1..].to_string()))
                        }

                        *framework.data.global.get_mut::<Message>().unwrap() =
                            Message::Success(command_string.clone());
                        let mut command = execute::command(command_string);

                        thread::spawn(move || {
                            let _ = command.output();
                        });
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

    pub fn update_provider(&mut self) {
        match &mut self.r#type {
            SingleItemType::Video(singlevideoitem) => singlevideoitem.update_provider(),
            SingleItemType::Playlist(singleplaylistitem) => singleplaylistitem.update_provider(),
            SingleItemType::None => {}
        }
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

        let chunks = self.grid.chunks(area).unwrap()[0].clone();

        frame.render_widget(self.grid.clone(), area);

        // only continue if `self.item` is Some
        if self.item.is_none() {
            return;
        }

        match &mut self.r#type {
            SingleItemType::Video(typeinfo) => {
                // 2 by 1 grid, item info in the first cell and textlist at the second
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
                self.iteminfo
                    .render(frame, framework, chunks[0], popup_render, info);
                if typeinfo.is_commands_view {
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

        let client = &framework.data.global.get::<InvidiousClient>().unwrap().0;
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();

        // load items using the invidious api
        // gets the item that it needs to load from `data.state.Page`
        self.item = match r#type {
            SingleItemPage::Video(id) => {
                self.r#type = SingleItemType::Video(SingleVideoItem::new(
                    framework.data.global.get::<CommandsConfig>().unwrap(),
                    framework.data.global.get::<MainConfig>().unwrap(),
                ));
                let video = Item::from_full_video(client.video(id, None)?, mainconfig.image_index);
                if mainconfig.images.display() {
                    download_all_images(vec![(&video).into()]);
                }
                Some(video)
            }
            SingleItemPage::Playlist(id) => {
                let playlist =
                    Item::from_full_playlist(client.playlist(id, None)?, mainconfig.image_index);
                let videos = &playlist.fullplaylist()?.videos;
                self.r#type = SingleItemType::Playlist(
                    SinglePlaylistItem::new(
                        framework.data.global.get::<CommandsConfig>().unwrap(),
                        framework.data.global.get::<MainConfig>().unwrap(),
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
        };
        self.iteminfo.item = self.item.clone();
        // need to update provider every time the item loads or else it will display `${provider}`
        // instead of the actual provider (e.g. `YouTube`)
        self.update_provider();

        if let Some(item) = &self.item {
            if item.is_unknown() {
                return Ok(());
            }

            let item = item.clone();
            let max_watch_history = mainconfig.max_watch_history;

            // push to watch history
            let watch_history = framework.data.global.get_mut::<WatchHistory>().unwrap();
            watch_history.push(item, max_watch_history);
            watch_history.save()?;
        }

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
                    match action {
                        // checks if it is updated, if it is and selected is not 0 (is hovering on
                        // a video), then also need to update the iteminfo
                        KeyAction::MoveUp => {
                            let updated = singleplaylistitem.videos_view.up().is_ok();
                            if updated && singleplaylistitem.videos_view.selected != 0 {
                                singleplaylistitem.hovered_video.item = Some(
                                    self.item.as_ref().unwrap().fullplaylist()?.videos
                                        [singleplaylistitem.videos_view.selected - 1]
                                        .clone(),
                                );
                            } else {
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
                            }
                            updated
                        }
                        KeyAction::MoveDown => {
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
                            let updated = singleplaylistitem.videos_view.first().is_ok();
                            if updated && singleplaylistitem.videos_view.selected != 0 {
                                singleplaylistitem.hovered_video.item = Some(
                                    self.item.as_ref().unwrap().fullplaylist()?.videos
                                        [singleplaylistitem.videos_view.selected - 1]
                                        .clone(),
                                );
                            }
                            updated
                        }
                        KeyAction::MoveRight => {
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
                    }
                }
            }
            SingleItemType::None => false,
        };

        if updated {
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

/// apply the env hashmap to a command string
pub fn apply_env(mut command: String, env: &HashMap<String, String>) -> String {
    env.iter().for_each(|(key, value)| {
        command = command.replace(&format!("${{{key}}}"), value);
    });
    command
}
