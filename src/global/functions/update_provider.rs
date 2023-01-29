use std::env;
use tui_additions::framework::FrameworkData;

use crate::{
    config::{MainConfig, Provider},
    global::structs::{
        ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Page, SingleItemPage, StateEnvs,
        Status,
    },
};

use super::set_envs;

pub fn update_provider(data: &mut FrameworkData) {
    let mainconfig = data.global.get::<MainConfig>().unwrap();
    let status = data.global.get::<Status>().unwrap();
    let page = data.state.get::<Page>().unwrap();

    let envs = match page {
        Page::ChannelDisplay(ChannelDisplayPage { id, r#type }) => vec![(
            String::from("url"),
            format!(
                "{}/channel/{id}{}",
                match status.provider {
                    Provider::YouTube => "https://youtube.com",
                    Provider::Invidious => &mainconfig.invidious_instance,
                },
                match r#type {
                    ChannelDisplayPageType::Main => "",
                    ChannelDisplayPageType::Videos => "/videos",
                    ChannelDisplayPageType::Playlists => "/playlists",
                }
            ),
        )],
        Page::Search(search) => vec![(
            String::from("url"),
            match status.provider {
                Provider::YouTube => {
                    format!("https://youtube.com/results?{}", search.to_string())
                }
                Provider::Invidious => format!(
                    "{}/search?{}",
                    mainconfig.invidious_instance,
                    search.to_string()
                ),
            },
        )],
        Page::MainMenu(MainMenuPage::Popular) => vec![(
            String::from("url"),
            match status.provider {
                Provider::YouTube => String::from("https://youtube.com"),
                Provider::Invidious => {
                    format!("{}/feed/popular", mainconfig.invidious_instance)
                }
            },
        )],
        Page::MainMenu(MainMenuPage::Trending) => vec![(
            String::from("url"),
            match status.provider {
                Provider::YouTube => String::from("https://www.youtube.com/feed/trending"),
                Provider::Invidious => {
                    format!("{}/feed/trending", mainconfig.invidious_instance)
                }
            },
        )],
        Page::MainMenu(MainMenuPage::History) => vec![(
            String::from("url"),
            match status.provider {
                Provider::YouTube => String::from("https://www.youtube.com/feed/history"),
                Provider::Invidious => {
                    format!("{}/feed/history", mainconfig.invidious_instance)
                }
            },
        )],
        Page::SingleItem(SingleItemPage::Video(id)) => vec![
            (
                String::from("url"),
                match status.provider {
                    Provider::Invidious => {
                        format!("{}/watch?v={}", mainconfig.invidious_instance, id)
                    }
                    Provider::YouTube => format!("https://youtu.be/{}", id),
                },
            ),
            (
                String::from("embed-url"),
                match status.provider {
                    Provider::Invidious => {
                        format!("{}/embed/{}", mainconfig.invidious_instance, id)
                    }
                    Provider::YouTube => format!("https://youtube.com/embed/{}", id),
                },
            ),
            (
                String::from("channel-url"),
                match status.provider {
                    Provider::YouTube => {
                        format!(
                            "https://www.youtube.com/channel/{}",
                            env::var("channel-id").unwrap()
                        )
                    }
                    Provider::Invidious => format!(
                        "{}/channel/{}",
                        mainconfig.invidious_instance,
                        env::var("channel-id").unwrap()
                    ),
                },
            ),
        ],
        Page::SingleItem(SingleItemPage::Playlist(id)) => vec![
            (
                String::from("url"),
                match status.provider {
                    Provider::YouTube => {
                        format!("https://www.youtube.com/playlist?list={}", id)
                    }
                    Provider::Invidious => {
                        format!("{}/playlist?list={}", mainconfig.invidious_instance, id)
                    }
                },
            ),
            (
                String::from("all-videos"),
                match status.provider {
                    Provider::YouTube => env::var("all-ids")
                        .unwrap()
                        .split(' ')
                        .map(|id| format!("'https://youtu.be/{id}'"))
                        .collect::<Vec<_>>()
                        .join(" "),
                    Provider::Invidious => env::var("all-ids")
                        .unwrap()
                        .split(' ')
                        .map(|id| format!("'{}/watch?v={id}'", mainconfig.invidious_instance,))
                        .collect::<Vec<_>>()
                        .join(" "),
                },
            ),
            (
                String::from("channel-url"),
                match status.provider {
                    Provider::YouTube => format!(
                        "https://www.youtube.com/channel/{}",
                        env::var("channel-id").unwrap()
                    ),
                    Provider::Invidious => format!(
                        "{}/channel/{}",
                        mainconfig.invidious_instance,
                        env::var("channel-id").unwrap()
                    ),
                },
            ),
        ],
    };

    data.global.get_mut::<Status>().unwrap().provider_updated = true;
    set_envs(
        envs.into_iter(),
        &mut data.state.get_mut::<StateEnvs>().unwrap().0,
    );
}
