use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    sync::OnceLock,
};

use crate::global::common::{
    channel::Channel,
    hidden::{PopularItem, SearchItem},
    universal::Playlist,
    video::Video,
    CommonPlaylist, CommonVideo,
};
use dyn_clone::DynClone;

use crate::{
    config::{Search, SearchProvider},
    MAIN_CONFIG,
};

#[allow(unused_variables)]
pub trait SearchProviderTrait: DynClone + Send {
    fn supports_channel(&self) -> bool {
        false
    }
    fn channel(&self, id: &str) -> Result<Channel, Box<dyn Error>> {
        unimplemented!("channel not implemented")
    }

    fn supports_channel_videos(&self) -> bool {
        false
    }
    fn channel_videos(&self, id: &str) -> Result<Vec<CommonVideo>, Box<dyn Error>> {
        unimplemented!("channel_videos not implemented")
    }

    fn supports_channel_playlists(&self) -> bool {
        false
    }
    fn channel_playlists(&self, id: &str) -> Result<Vec<CommonPlaylist>, Box<dyn Error>> {
        unimplemented!("channel_playlists not implemented")
    }

    fn supports_trending(&self) -> bool {
        false
    }
    fn trending(&self) -> Result<Vec<CommonVideo>, Box<dyn Error>> {
        unimplemented!("trending not implemented")
    }

    fn supports_popular(&self) -> bool {
        false
    }
    fn popular(&self) -> Result<Vec<PopularItem>, Box<dyn Error>> {
        unimplemented!("popular not implemented")
    }

    fn supports_search(&self) -> bool {
        false
    }
    fn search(&self, filters: &Search) -> Result<Vec<SearchItem>, Box<dyn Error>> {
        unimplemented!("search not implemented")
    }

    fn supports_video(&self) -> bool {
        false
    }
    fn video(&self, id: &str) -> Result<Video, Box<dyn Error>> {
        unimplemented!("video not implemented")
    }

    fn supports_playlist(&self) -> bool {
        false
    }
    fn playlist(&self, id: &str) -> Result<Playlist, Box<dyn Error>> {
        unimplemented!("playlist not implemented")
    }

    /*
    fn supports_album(&self) -> bool {
        false
    }
    fn album(&self) -> Result {

    }
    */
}

dyn_clone::clone_trait_object!(SearchProviderTrait);

#[derive(Default, Clone)]
pub struct SearchProviderWrapper(HashMap<SearchProvider, Box<dyn SearchProviderTrait>>);

struct UnsupportedError(pub &'static str);

impl Display for UnsupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Operation `{}` is not supported by {}",
            self.0,
            unsafe { MAIN_CONFIG.get().unwrap() }
                .search_provider
                .as_str()
        ))
    }
}

impl Debug for UnsupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl Error for UnsupportedError {}

static mut SEARCH_PROVIDER: OnceLock<SearchProviderWrapper> = OnceLock::new();

impl SearchProviderWrapper {
    pub fn init() {
        unsafe {
            if SEARCH_PROVIDER.get().is_some() {
                *SEARCH_PROVIDER.get_mut().unwrap() = Self::default()
            } else {
                let _ = SEARCH_PROVIDER.set(Self::default());
            }
        }
    }

    #[allow(clippy::borrowed_box)]
    fn get() -> &'static Box<dyn SearchProviderTrait> {
        let provider = &unsafe { MAIN_CONFIG.get() }.unwrap().search_provider;
        unsafe { SEARCH_PROVIDER.get_mut() }
            .unwrap()
            .0
            .entry(*provider)
            .or_insert(provider.create())
    }

    pub fn channel(id: &str) -> Result<Channel, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_channel() {
            return Err(UnsupportedError("channel").into());
        }
        provider.channel(id)
    }

    pub fn channel_videos(id: &str) -> Result<Vec<CommonVideo>, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_channel_videos() {
            return Err(UnsupportedError("channel_videos").into());
        }
        provider.channel_videos(id)
    }

    pub fn channel_playlists(id: &str) -> Result<Vec<CommonPlaylist>, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_channel_playlists() {
            return Err(UnsupportedError("channel_playlists").into());
        }
        provider.channel_playlists(id)
    }

    pub fn trending() -> Result<Vec<CommonVideo>, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_trending() {
            return Err(UnsupportedError("trending").into());
        }
        provider.trending()
    }

    pub fn popular() -> Result<Vec<PopularItem>, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_popular() {
            return Err(UnsupportedError("popular").into());
        }
        provider.popular()
    }

    pub fn search(filters: &Search) -> Result<Vec<SearchItem>, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_search() {
            return Err(UnsupportedError("search").into());
        }
        provider.search(filters)
    }

    pub fn video(id: &str) -> Result<Video, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_video() {
            return Err(UnsupportedError("video").into());
        }
        provider.video(id)
    }

    pub fn playlist(id: &str) -> Result<Playlist, Box<dyn Error>> {
        let provider = Self::get();
        if !provider.supports_playlist() {
            return Err(UnsupportedError("playlist").into());
        }
        provider.playlist(id)
    }
}
