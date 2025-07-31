use invidious::{ClientSync, ClientSyncTrait};

use crate::global::traits::SearchProviderTrait;

// used in `data.global`
/// Holds the invidious client
#[derive(Clone)]
pub struct InvidiousClient(pub ClientSync);

impl InvidiousClient {
    pub fn new(instance: String) -> Self {
        Self(ClientSync::default().instance(instance))
    }
}

impl SearchProviderTrait for InvidiousClient {
    fn supports_video(&self) -> bool {
        true
    }

    fn video(&self, id: &str) -> Result<invidious::video::Video, Box<dyn std::error::Error>> {
        self.0.video(id, None).map_err(|e| e.into())
    }

    fn supports_search(&self) -> bool {
        true
    }

    fn search(
        &self,
        filters: &crate::config::Search,
    ) -> Result<Vec<invidious::hidden::SearchItem>, Box<dyn std::error::Error>> {
        self.0
            .search(Some(&filters.to_string()))
            .map(|res| res.items)
            .map_err(|e| e.into())
    }

    fn supports_channel(&self) -> bool {
        true
    }

    fn channel(&self, id: &str) -> Result<invidious::channel::Channel, Box<dyn std::error::Error>> {
        self.0.channel(id, None).map_err(|e| e.into())
    }

    fn supports_popular(&self) -> bool {
        true
    }

    fn popular(&self) -> Result<Vec<invidious::hidden::PopularItem>, Box<dyn std::error::Error>> {
        self.0
            .popular(None)
            .map(|res| res.items)
            .map_err(|e| e.into())
    }

    fn supports_trending(&self) -> bool {
        true
    }

    fn trending(&self) -> Result<Vec<invidious::CommonVideo>, Box<dyn std::error::Error>> {
        self.0
            .trending(None)
            .map(|res| res.videos)
            .map_err(|e| e.into())
    }

    fn supports_playlist(&self) -> bool {
        true
    }

    fn playlist(
        &self,
        id: &str,
    ) -> Result<invidious::universal::Playlist, Box<dyn std::error::Error>> {
        self.0.playlist(id, None).map_err(|e| e.into())
    }

    fn supports_channel_videos(&self) -> bool {
        true
    }

    fn channel_videos(
        &self,
        id: &str,
    ) -> Result<Vec<invidious::CommonVideo>, Box<dyn std::error::Error>> {
        self.0
            .channel_videos(id, None)
            .map(|res| res.videos)
            .map_err(|e| e.into())
    }

    fn supports_channel_playlists(&self) -> bool {
        true
    }

    fn channel_playlists(
        &self,
        id: &str,
    ) -> Result<Vec<invidious::CommonPlaylist>, Box<dyn std::error::Error>> {
        self.0
            .channel_playlists(id, None)
            .map(|res| res.playlists)
            .map_err(|e| e.into())
    }
}
