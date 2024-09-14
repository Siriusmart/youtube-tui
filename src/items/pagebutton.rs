use crate::{config::AppearanceConfig, global::structs::*};
use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use tui_additions::framework::FrameworkItem;

/// button that on press will go to another page instead of selecting it
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PageButton {
    Trending,
    Popular,
    ChannelMain,
    ChannelVideos,
    ChannelPlaylists,
    History,
    Feed,
    Library,
}

impl PageButton {
    pub fn page(&self, current_page: &Page) -> Page {
        match self {
            Self::Trending => Page::MainMenu(MainMenuPage::Trending),
            Self::Popular => Page::MainMenu(MainMenuPage::Popular),
            Self::ChannelMain => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Main,
            }),
            Self::ChannelVideos => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Videos,
            }),
            Self::ChannelPlaylists => Page::ChannelDisplay(ChannelDisplayPage {
                id: current_page.channeldisplay().id.clone(),
                r#type: ChannelDisplayPageType::Playlists,
            }),
            Self::History => Page::MainMenu(MainMenuPage::History),
            Self::Feed => Page::Feed,
            Self::Library => Page::MainMenu(MainMenuPage::Library),
        }
    }

    // fn update_toggleable(&mut self, page: &Page) {
    //     if *self == Self::Popular || *self == Self::Trending {
    //         match page {
    //             Page::MainMenu(MainMenuPage::Popular) => *self = Self::Popular,
    //             Page::MainMenu(MainMenuPage::Trending) => *self = Self::Trending,
    //             _ => {}
    //         }
    //     }
    // }
}

impl ToString for PageButton {
    fn to_string(&self) -> String {
        match self {
            Self::Popular => String::from("Popular"),
            Self::Trending => String::from("Trending"),
            Self::ChannelMain => String::from("Main"),
            Self::ChannelVideos => String::from("Videos"),
            Self::ChannelPlaylists => String::from("Playlists"),
            Self::History => String::from("History"),
            Self::Feed => String::from("Feed"),
            Self::Library => String::from("Library"),
        }
    }
}

impl FrameworkItem for PageButton {
    // fn load_item(
    //     &mut self,
    //     framework: &mut tui_additions::framework::FrameworkClean,
    //     _info: tui_additions::framework::ItemInfo,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     self.update_toggleable(framework.data.state.get::<Page>().unwrap());
    //     Ok(())
    // }

    // it is basically a paragraph (text) with borders
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

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let same_page = &self.page(framework.data.state.get::<Page>().unwrap())
            == framework.data.state.get::<Page>().unwrap();

        let block = Block::default()
            .border_type(appearance.borders)
            .border_style(Style::default().fg(if info.hover {
                appearance.colors.outline_hover
            } else if same_page {
                appearance.colors.outline_secondary
            } else {
                appearance.colors.outline
            }))
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(self.to_string())
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }

    // when selected creates a load page task, but returns false to show that it is not being selected
    fn select(&mut self, framework: &mut tui_additions::framework::FrameworkClean) -> bool {
        let current_page = framework.data.state.get::<Page>().unwrap();
        let self_page = self.page(current_page);

        // if self_page == *current_page {
        //     match self {
        //         Self::Trending => *self = Self::Popular,
        //         Self::Popular => *self = Self::Trending,
        //     }
        // }

        framework
            .data
            .state
            .get_mut::<Tasks>()
            .unwrap()
            .priority
            .push(Task::LoadPage(self.page(&self_page)));

        false
    }

    fn selectable(&self) -> bool {
        true
    }
}
