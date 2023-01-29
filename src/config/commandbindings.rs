use crossterm::event::{KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

use crate::global::{
    structs::{ChannelDisplayPage, ChannelDisplayPageType, MainMenuPage, Page, SingleItemPage},
    traits::ConfigTrait,
};

use super::serde::KeyCodeSerde;

#[derive(Clone)]
pub struct CommandBindings {
    pub global: HashMap<KeyCode, HashMap<u8, String>>,
    pub search: HashMap<KeyCode, HashMap<u8, String>>,
    pub popular: HashMap<KeyCode, HashMap<u8, String>>,
    pub trending: HashMap<KeyCode, HashMap<u8, String>>,
    pub video: HashMap<KeyCode, HashMap<u8, String>>,
    pub playlist: HashMap<KeyCode, HashMap<u8, String>>,
    pub channel_main: HashMap<KeyCode, HashMap<u8, String>>,
    pub channel_videos: HashMap<KeyCode, HashMap<u8, String>>,
    pub channel_playlists: HashMap<KeyCode, HashMap<u8, String>>,
    pub watchhistory: HashMap<KeyCode, HashMap<u8, String>>,
}

impl Key for CommandBindings {
    type Value = Self;
}

impl CommandBindings {
    pub fn get_command(&self, key: &KeyEvent, page: &Page) -> String {
        let mut out = String::new();

        if let Some(command) = get_command(key, &self.global) {
            out.push_str(command);
        }

        let command = match page {
            Page::Search(_) => get_command(key, &self.search),
            Page::MainMenu(MainMenuPage::Trending) => get_command(key, &self.trending),
            Page::MainMenu(MainMenuPage::Popular) => get_command(key, &self.popular),
            Page::MainMenu(MainMenuPage::History) => get_command(key, &self.watchhistory),
            Page::SingleItem(SingleItemPage::Video(_)) => get_command(key, &self.video),
            Page::SingleItem(SingleItemPage::Playlist(_)) => get_command(key, &self.playlist),
            Page::ChannelDisplay(ChannelDisplayPage {
                r#type: ChannelDisplayPageType::Main,
                ..
            }) => get_command(key, &self.channel_main),
            Page::ChannelDisplay(ChannelDisplayPage {
                r#type: ChannelDisplayPageType::Videos,
                ..
            }) => get_command(key, &self.channel_videos),
            Page::ChannelDisplay(ChannelDisplayPage {
                r#type: ChannelDisplayPageType::Playlists,
                ..
            }) => get_command(key, &self.channel_playlists),
        };

        if let Some(command) = command {
            if !out.is_empty() {
                out.push_str("&&");
            }
            out.push_str(command);
        }

        out
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandBindingsSerde {
    #[serde(default = "global_default")]
    pub global: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "search_default")]
    pub search: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "popular_default")]
    pub popular: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "trending_default")]
    pub trending: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "video_default")]
    pub video: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "playlist_default")]
    pub playlist: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "channel_main_default")]
    pub channel_main: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "channel_videos_default")]
    pub channel_videos: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "channel_playlists_default")]
    pub channel_playlists: HashMap<KeyCodeSerde, HashMap<u8, String>>,
    #[serde(default = "watchhistory_default")]
    pub watchhistory: HashMap<KeyCodeSerde, HashMap<u8, String>>,
}

impl ConfigTrait for CommandBindingsSerde {
    const LABEL: &'static str = "commandbindings";
}

impl CommandBindingsSerde {
    pub fn into(self) -> Option<CommandBindings> {
        Some(CommandBindings {
            global: de_serde(self.global)?,
            search: de_serde(self.search)?,
            popular: de_serde(self.popular)?,
            channel_main: de_serde(self.channel_main)?,
            channel_videos: de_serde(self.channel_videos)?,
            channel_playlists: de_serde(self.channel_playlists)?,
            playlist: de_serde(self.playlist)?,
            video: de_serde(self.video)?,
            trending: de_serde(self.trending)?,
            watchhistory: de_serde(self.watchhistory)?,
        })
    }
}

impl Default for CommandBindingsSerde {
    fn default() -> Self {
        Self {
            global: global_default(),
            search: search_default(),
            popular: popular_default(),
            channel_main: channel_main_default(),
            channel_videos: channel_videos_default(),
            channel_playlists: channel_playlists_default(),
            playlist: playlist_default(),
            video: video_default(),
            trending: trending_default(),
            watchhistory: watchhistory_default(),
        }
    }
}

fn de_serde(
    original: HashMap<KeyCodeSerde, HashMap<u8, String>>,
) -> Option<HashMap<KeyCode, HashMap<u8, String>>> {
    let mut out = HashMap::new();

    // simply loops over the serde hashmap and insert them into the new one
    for (keycodeserde, map) in original.into_iter() {
        out.insert(keycodeserde.to_keycode()?, map);
    }

    Some(out)
}

fn get_command<'a>(
    key: &'a KeyEvent,
    map: &'a HashMap<KeyCode, HashMap<u8, String>>,
) -> Option<&'a str> {
    Some(map.get(&key.code)?.get(&key.modifiers.bits())?)
}

// default functions

fn global_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (
            KeyCodeSerde::Char('f'),
            HashMap::from([(2, String::from("run ${browser} '${url}'"))]),
        ),
        (
            KeyCodeSerde::Char('c'),
            HashMap::from([(2, String::from("cp ${url}"))]),
        ),
    ])
}

fn search_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn channel_main_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::default()
}

fn channel_playlists_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn channel_videos_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn playlist_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn popular_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn trending_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))])),
    ])
}

fn video_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::default()
}

fn watchhistory_default() -> HashMap<KeyCodeSerde, HashMap<u8, String>> {
    HashMap::from([
        (KeyCodeSerde::Char('a'), HashMap::from([(2, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video"))])),
        (KeyCodeSerde::Char('A'), HashMap::from([(1, String::from("run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle"))])),
        (KeyCodeSerde::Char('p'), HashMap::from([(2, String::from("run mpv '${hover-url}'"))]))
    ])
}
