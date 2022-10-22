use std::error::Error;

use tui::{layout::Constraint, style::Style};
use tui_additions::{
    framework::FrameworkItem,
    widgets::{Grid, TextList},
};

use super::ItemInfo;
use crate::{
    config::{AppearanceConfig, KeyBindingsConfig, MainConfig},
    global::{
        functions::download_all_images,
        structs::{
            InvidiousClient, Item, KeyAction, MainMenuPage, Page, SingleItemPage, Task, Tasks,
        },
    },
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
        let mainconfig = framework.data.global.get::<MainConfig>().unwrap();
        self.update_appearance(appearance, mainconfig, &info);

        // creates the grid
        let grid = self.grid.clone();
        let chunks = grid.chunks(area).unwrap()[0].to_owned();

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
        let page = framework.data.state.get::<Page>().unwrap();
        let image_index = framework
            .data
            .global
            .get::<MainConfig>()
            .unwrap()
            .image_index;
        let client = &framework.data.global.get::<InvidiousClient>().unwrap().0;

        // fetch the items using the invidious api
        match page {
            Page::MainMenu(MainMenuPage::Trending) => {
                self.items = client
                    .trending(None)?
                    .videos
                    .into_iter()
                    .map(|item| Item::from_trending_video(item, image_index))
                    .collect();
            }
            Page::MainMenu(MainMenuPage::Popular) => {
                self.items = client
                    .popular(None)?
                    .items
                    .into_iter()
                    .map(|item| Item::from_popular_item(item, image_index))
                    .collect();
            }
            Page::Search(search) => {
                self.items = client
                    .search(Some(&search.to_string()))?
                    .items
                    .into_iter()
                    .map(|item| Item::from_search_item(item, image_index))
                    .collect();
            }
            _ => unreachable!("item `ItemList` cannot be used in `{page:?}`"),
        }

        if framework
            .data
            .global
            .get::<MainConfig>()
            .unwrap()
            .images
            .display()
        {
            // download thumbnails of all videos in the list
            download_all_images(self.items.iter().map(|item| item.into()).collect());
        }

        // update the items in text list
        self.textlist.set_items(&self.items).unwrap();
        self.update();

        Ok(())
    }

    fn key_event(
        &mut self,
        framework: &mut tui_additions::framework::FrameworkClean,
        key: crossterm::event::KeyEvent,
        _info: tui_additions::framework::ItemInfo,
    ) -> Result<(), Box<dyn Error>> {
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

        // move the textlist cursor in the corresponding directions
        let updated = match action {
            KeyAction::MoveUp => self.textlist.up().is_ok(),
            KeyAction::MoveDown => self.textlist.down().is_ok(),
            KeyAction::MoveLeft => self.textlist.first().is_ok(),
            KeyAction::MoveRight => self.textlist.last().is_ok(),
            KeyAction::Select => {
                let page_to_load = match &self.items[self.textlist.selected] {
                    Item::MiniVideo(video) => {
                        Page::SingleItem(SingleItemPage::Video(video.id.clone()))
                    }
                    Item::MiniPlaylist(playlist) => {
                        Page::SingleItem(SingleItemPage::Playlist(playlist.id.clone()))
                    }
                    _ => unimplemented!(),
                };
                framework
                    .data
                    .state
                    .get_mut::<Tasks>()
                    .unwrap()
                    .priority
                    .push(Task::LoadPage(page_to_load));
                false
            }
            _ => false,
        };

        // only create a render task if the key event actually changed something
        if updated {
            self.info.item = Some(self.items[self.textlist.selected].clone());
            framework
                .data
                .state
                .get_mut::<Tasks>()
                .unwrap()
                .last
                .push(Task::RenderAll);
        }

        Ok(())
    }
}

impl ItemList {
    // change `self.item` to the currently selected item
    pub fn update(&mut self) {
        if self.items.is_empty() {
            self.info.item = None;
            return;
        }

        self.info.item = Some(self.items[self.textlist.selected].clone());
    }
}
