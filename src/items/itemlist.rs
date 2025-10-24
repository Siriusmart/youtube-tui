use std::error::Error;

use crate::{
    config::*,
    global::{functions::*, structs::*, traits::SearchProviderWrapper},
    items::ItemInfo,
};
use home::home_dir;
use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
};
use tui_additions::{
    framework::{FrameworkClean, FrameworkItem},
    widgets::{Grid, TextList},
};

/// An item list displays a list of items
// It consists of a 1 x 2 grid, with the left cell displaying a text list, the right displaying item info of the currently hovered item
#[derive(Clone)]
pub struct ItemList {
    pub info: ItemInfo,
    pub items: Vec<Item>,
    pub textlist: TextList,
    pub grid: Grid,
}

impl ItemList {
    pub fn infalte_item_update(
        &self,
        mainconfig: &MainConfig,
        status: &Status,
    ) -> Vec<(String, String)> {
        if self.textlist.items.is_empty() {
            return Vec::new();
        }

        match &self.items[self.textlist.selected] {
            Item::MiniVideo(MiniVideoItem {
                id,
                title,
                channel_id,
                channel,
                ..
            })
            | Item::FullVideo(FullVideoItem {
                id,
                title,
                channel_id,
                channel,
                ..
            }) => {
                vec![
                    (
                        String::from("hover-url"),
                        format!(
                            "{}/watch?v={id}",
                            match status.provider {
                                Provider::YouTube => "https://youtube.com",
                                Provider::Invidious => &mainconfig.invidious_instance,
                            }
                        ),
                    ),
                    (String::from("hover-title"), title.clone()),
                    (String::from("hover-id"), id.clone()),
                    (String::from("hover-type"), String::from("video")),
                    (String::from("hover-channel-id"), channel_id.clone()),
                    (String::from("hover-channel"), channel.clone()),
                ]
            }
            Item::MiniPlaylist(MiniPlaylistItem {
                id,
                title,
                channel_id,
                channel,
                ..
            })
            | Item::FullPlaylist(FullPlaylistItem {
                id,
                title,
                channel_id,
                channel,
                ..
            }) => {
                vec![
                    (
                        String::from("hover-url"),
                        format!(
                            "{}/playlist?list={id}",
                            match status.provider {
                                Provider::YouTube => "https://youtube.com",
                                Provider::Invidious => &mainconfig.invidious_instance,
                            }
                        ),
                    ),
                    (String::from("hover-title"), title.clone()),
                    (String::from("hover-id"), id.clone()),
                    (String::from("hover-type"), String::from("playlist")),
                    (String::from("hover-channel-id"), channel_id.clone()),
                    (String::from("hover-channel"), channel.clone()),
                ]
            }
            Item::MiniChannel(MiniChannelItem { id, name, .. })
            | Item::FullChannel(FullChannelItem { id, name, .. }) => {
                vec![
                    (
                        String::from("hover-url"),
                        format!(
                            "{}/channel/{id}",
                            match status.provider {
                                Provider::YouTube => "https://youtube.com",
                                Provider::Invidious => &mainconfig.invidious_instance,
                            }
                        ),
                    ),
                    (String::from("hover-title"), name.clone()),
                    (String::from("hover-id"), id.clone()),
                    (String::from("hover-channel-id"), id.clone()),
                    (String::from("hover-channel"), name.clone()),
                    (String::from("hover-type"), String::from("channel")),
                ]
            }
            Item::Page(_) => {
                vec![(String::from("hover-url"), String::from("not avaliable"))]
            }
        }
    }

    fn update_appearance(
        &mut self,
        appearance: &AppearanceConfig,
        mainconfig: &MainConfig,
        iteminfo: &tui_additions::framework::ItemInfo,
    ) {
        self.textlist.set_ascii_only(!mainconfig.allow_unicode);
        self.grid.set_border_type(appearance.borders);
        self.textlist.set_border_type(appearance.borders);
        self.textlist
            .set_style(Style::default().fg(appearance.colors.text));

        if iteminfo.selected {
            self.grid
                .set_border_style(Style::default().fg(appearance.colors.outline_selected));
            self.textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_hover));
            self.textlist
                .set_selected_style(Style::default().fg(appearance.colors.text_special));
        } else {
            self.textlist
                .set_cursor_style(Style::default().fg(appearance.colors.outline_secondary));
            self.textlist
                .set_selected_style(Style::default().fg(appearance.colors.text_secondary));
            if iteminfo.hover {
                self.grid
                    .set_border_style(Style::default().fg(appearance.colors.outline_hover));
            } else {
                self.grid
                    .set_border_style(Style::default().fg(appearance.colors.outline));
            }
        }
    }

    /// handles select (enter)
    fn select_at_cursor(&self, framework: &mut FrameworkClean) {
        let page_to_load =
            if LocalStore::get_info(self.items[self.textlist.selected].id().unwrap_or_default())
                .is_some()
            {
                match &self.items[self.textlist.selected] {
                    Item::MiniVideo(MiniVideoItem { id, .. })
                    | Item::FullVideo(FullVideoItem { id, .. }) => {
                        Some(Page::SingleItem(SingleItemPage::Video(id.clone())))
                    }
                    Item::MiniPlaylist(MiniPlaylistItem { id, .. })
                    | Item::FullPlaylist(FullPlaylistItem { id, .. }) => {
                        Some(Page::SingleItem(SingleItemPage::Playlist(id.clone())))
                    }
                    Item::MiniChannel(MiniChannelItem { id, .. })
                    | Item::FullChannel(FullChannelItem { id, .. }) => {
                        Some(Page::ChannelDisplay(ChannelDisplayPage {
                            id: id.to_string(),
                            r#type: ChannelDisplayPageType::Main,
                        }))
                    }
                    // Item::Unknown(_) => {
                    //     *framework.data.global.get_mut::<Message>().unwrap() =
                    //         Message::Message(String::from("Unknown item"));
                    //     framework
                    //         .data
                    //         .state
                    //         .get_mut::<Tasks>()
                    //         .unwrap()
                    //         .priority
                    //         .push(Task::RenderAll);
                    //     None
                    // }
                    Item::Page(b) => match framework.data.state.get::<Page>().unwrap() {
                        Page::Search(search) => Some(Page::Search(Search {
                            page: if *b { search.page + 1 } else { search.page - 1 },
                            ..search.clone()
                        })),
                        _ => unreachable!("Page turners can only be used in search pages"),
                    },
                }
            } else {
                match &self.items[self.textlist.selected] {
                    Item::MiniVideo(MiniVideoItem { id, .. })
                    | Item::FullVideo(FullVideoItem { id, .. }) => {
                        Some(Page::SingleItem(SingleItemPage::Video(id.clone())))
                    }
                    Item::MiniPlaylist(MiniPlaylistItem { id, .. })
                    | Item::FullPlaylist(FullPlaylistItem { id, .. }) => {
                        Some(Page::SingleItem(SingleItemPage::Playlist(id.clone())))
                    }
                    Item::MiniChannel(MiniChannelItem { id, .. })
                    | Item::FullChannel(FullChannelItem { id, .. }) => {
                        Some(Page::ChannelDisplay(ChannelDisplayPage {
                            id: id.clone(),
                            r#type: ChannelDisplayPageType::Main,
                        }))
                    }
                    // Item::Unknown(_) => {
                    //     *framework.data.global.get_mut::<Message>().unwrap() =
                    //         Message::Message(String::from("Unknown item"));
                    //     framework
                    //         .data
                    //         .state
                    //         .get_mut::<Tasks>()
                    //         .unwrap()
                    //         .priority
                    //         .push(Task::RenderAll);
                    //     None
                    // }
                    Item::Page(b) => match framework.data.state.get::<Page>().unwrap() {
                        Page::Search(search) => Some(Page::Search(Search {
                            page: if *b { search.page + 1 } else { search.page - 1 },
                            ..search.clone()
                        })),
                        _ => unreachable!("Page turners can only be used in search pages"),
                    },
                }
            };

        if let Some(page_to_load) = page_to_load {
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::LoadPage(page_to_load));
        }
    }
}

impl Default for ItemList {
    fn default() -> Self {
        Self {
            info: ItemInfo::default(),
            items: Vec::new(),
            textlist: TextList::default().non_ascii_replace(' '),
            grid: Grid::new(
                vec![Constraint::Percentage(60), Constraint::Percentage(40)],
                vec![Constraint::Percentage(100)],
            )
            .unwrap(),
        }
    }
}

impl FrameworkItem for ItemList {
    fn message(
        &mut self,
        framework: &mut FrameworkClean,
        data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    ) -> bool {
        if !data.contains_key("type") {
            return false;
        }

        let updated = data.get("type").is_some_and(|v| {
            v.downcast_ref::<String>()
                .is_some_and(|v| match v.as_str() {
                    "scrollup" => self.textlist.up().is_ok(),
                    "scrolldown" => self.textlist.down().is_ok(),
                    _ => false,
                })
        });

        if updated && !self.items.is_empty() {
            self.update(framework);
            set_envs(
                self.infalte_item_update(
                    framework.data.global.get::<MainConfig>().unwrap(),
                    framework.data.global.get::<Status>().unwrap(),
                )
                .into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );

            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .render_image = true;
            self.info.item = Some(self.items[self.textlist.selected].clone());
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .priority
                .push(Task::RenderAll);
        }

        updated
    }

    fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: ratatui::layout::Rect,
        popup_render: bool,
        info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let status = framework.data.global.get::<Status>().unwrap();
        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();

        if status.provider_updated {
            set_envs(
                self.infalte_item_update(mainconfig, status).into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );
        }

        self.update_appearance(appearance, mainconfig, &info);

        // creates the grid
        let grid = self.grid.clone();
        let chunks = grid.chunks(area).unwrap()[0].clone();

        // creates the text list in cell (0, 1)
        self.textlist.set_height(chunks[0].height);
        self.textlist
            .set_cursor_style(Style::default().fg(if info.selected {
                appearance.colors.outline_hover
            } else {
                appearance.colors.outline_secondary
            }));

        let textlist = self.textlist.clone();

        frame.render_widget(grid, area);
        frame.render_widget(textlist, chunks[0]);

        // used the `.render()` function in self.info because it is an ItemInfo and impls FrameworkItem instead of Widget
        self.info
            .render(frame, framework, chunks[1], popup_render, info);
    }

    fn selectable(&self) -> bool {
        true
    }

    fn load_item(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
        *self = Self::default();

        let page = framework.data.state.get::<Page>().unwrap();
        let image_index = framework
            .data
            .global
            .get::<MainConfig>()
            .unwrap()
            .image_index;

        // fetch the items using the invidious api
        match page {
            Page::MainMenu(MainMenuPage::Trending) => {
                self.items = SearchProviderWrapper::trending()?
                    .into_iter()
                    .map(|item| Item::from_common_video(item, image_index))
                    .collect();
            }
            Page::MainMenu(MainMenuPage::Popular) => {
                self.items = SearchProviderWrapper::popular()?
                    .into_iter()
                    .map(|item| Item::from_popular_item(item, image_index))
                    .collect();
            }
            Page::MainMenu(MainMenuPage::Library) => {
                let history = framework.data.global.get::<Library>().unwrap();
                self.items = history.0.clone().into_iter().rev().collect();
            }
            Page::MainMenu(MainMenuPage::History) => {
                // the vector needs to be reversed because the latest watch history is pushed to
                // the back, meaning it needs to be reversed so that the latests one are on top
                let history = framework.data.global.get::<WatchHistory>().unwrap();
                self.items = history.0.clone().into_iter().rev().collect();
            }
            Page::Search(search) => {
                self.items = SearchProviderWrapper::search(search)?
                    .into_iter()
                    .map(|item| Item::from_search_item(item, image_index))
                    .collect();
                if !self.items.is_empty() {
                    self.items.push(Item::Page(true));
                }
                if search.page != 1 {
                    self.items.insert(0, Item::Page(false));
                }
            }
            _ => unreachable!("item `ItemList` cannot be used in `{page:?}`"),
        }

        // update the items in text list
        self.textlist.set_items(&self.items).unwrap();
        self.update(framework);

        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        let status = framework.data.global.get::<Status>().unwrap();
        if mainconfig.images.display() {
            // download thumbnails of all videos in the list
            download_all_images(self.items.iter().map(|item| item.into()).collect());
        }

        set_envs(
            self.infalte_item_update(mainconfig, status).into_iter(),
            &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
        );
        update_provider(framework.data);

        Ok(())
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
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

        // move the textlist cursor in the corresponding directions
        let updated = match action {
            KeyAction::MoveUp => self.textlist.up().is_ok(),
            KeyAction::MoveDown => self.textlist.down().is_ok(),
            KeyAction::MoveLeft | KeyAction::First => self.textlist.first().is_ok(),
            KeyAction::MoveRight | KeyAction::End => self.textlist.last().is_ok(),
            KeyAction::Select => {
                self.select_at_cursor(framework);
                false
            }
            _ => false,
        };

        // only create a render task if the key event actually changed something
        if updated && !self.items.is_empty() {
            self.update(framework);
            set_envs(
                self.infalte_item_update(
                    framework.data.global.get::<MainConfig>().unwrap(),
                    framework.data.global.get::<Status>().unwrap(),
                )
                .into_iter(),
                &mut framework.data.state.get_mut::<StateEnvs>().unwrap().0,
            );

            framework
                .data
                .global
                .get_mut::<Status>()
                .unwrap()
                .render_image = true;
            self.info.item = Some(self.items[self.textlist.selected].clone());
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
        framework: &mut tui_additions::framework::FrameworkClean,
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
            .unwrap()[0][0];

        if !chunk.intersects(Rect::new(x, y, 1, 1)) {
            return false;
        }

        let previously_selected = self.textlist.selected;
        let y = (y - chunk.y) as usize + self.textlist.scroll;

        // clicking on already selected item
        if y == self.textlist.selected
            || y == self.textlist.selected + 2
            || y == self.textlist.selected + 1
        {
            self.select_at_cursor(framework);
            return true;
        }

        // clicking on rows after the last item
        if y > self.textlist.items.len() + 1 {
            let _ = self.textlist.last();
        } else if y <= self.textlist.selected {
            self.textlist.selected = y;
        } else if y >= self.textlist.selected + 2 {
            self.textlist.selected = y - 2;
        }

        if self.textlist.selected == previously_selected {
            return false;
        }

        self.update(framework);
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

impl ItemList {
    // change `self.item` to the currently selected item
    pub fn update(&mut self, framework: &mut FrameworkClean) {
        let selected = self.items.get(self.textlist.selected);

        if selected.is_none() {
            return;
        }

        if matches!(selected, Some(Item::Page(_))) {
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
            self.info.item = None;
            return;
        }

        self.info.item = Some(self.items[self.textlist.selected].clone());
    }
}
