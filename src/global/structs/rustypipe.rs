use crate::global::common::{
    channel::Channel, hidden::SearchItem, universal::Playlist, video::Video, CommonChannel,
    CommonImage, CommonPlaylist, CommonThumbnail, CommonVideo,
};
use rustypipe::{
    client::RustyPipe,
    model::{
        richtext::{ToHtml, ToPlaintext},
        ChannelItem, PlaylistItem, Thumbnail, VideoItem, YouTubeItem,
    },
    param::search_filter::{ItemType, Length, Order, SearchFilter, UploadDate},
};

use crate::{
    config::{SearchFilterDate, SearchFilterDuration, SearchFilterSort, SearchFilterType},
    global::{functions::viewcount_text, traits::SearchProviderTrait},
    RUNTIME,
};

#[derive(Clone)]
pub struct RustyPipeWrapper(RustyPipe);

impl Default for RustyPipeWrapper {
    fn default() -> Self {
        Self(
            RustyPipe::builder()
                .storage_dir(home::home_dir().unwrap().join(".local/share/rustypipe"))
                .build()
                .unwrap(),
        )
    }
}

fn thumbnail_convert(thumb: Thumbnail) -> CommonThumbnail {
    CommonThumbnail {
        quality: "unknown".to_string(),
        url: thumb.url,
        width: thumb.width,
        height: thumb.height,
    }
}

fn image_convert(thumb: Thumbnail) -> CommonImage {
    CommonImage {
        url: thumb.url,
        width: thumb.width,
        height: thumb.height,
    }
}

fn video_item_convert(video: VideoItem) -> CommonVideo {
    crate::global::common::CommonVideo {
        title: video.name,
        id: video.id,
        author: video
            .channel
            .clone()
            .map(|c| c.name)
            .unwrap_or("Unknown channel".to_string()),
        author_id: video
            .channel
            .clone()
            .map(|c| c.id)
            .unwrap_or("Unknown channel id".to_string()),
        author_url: video
            .channel
            .map(|c| format!("https://www.youtube.com/channel/{}", c.id))
            .unwrap_or("Unknown channel id".to_string()),
        thumbnails: video.thumbnail.into_iter().map(thumbnail_convert).collect(),
        description: video.short_description.clone().unwrap_or_default(),
        description_html: video.short_description.unwrap_or_default(),
        views: video.view_count.unwrap_or_default(),
        length: video.duration.unwrap_or_default(),
        published: video
            .publish_date
            .map(|t| t.unix_timestamp() as u64)
            .unwrap_or_default(),
        published_text: video
            .publish_date_txt
            .unwrap_or("Unknown publish date".to_string()),
        premiere_timestamp: 0,
        live: video.is_live,
        premium: false,
        upcoming: video.is_upcoming,
    }
}

fn playlist_item_convert(playlist: PlaylistItem) -> CommonPlaylist {
    crate::global::common::CommonPlaylist {
        title: playlist.name,
        id: playlist.id,
        thumbnail: playlist
            .thumbnail
            .into_iter()
            .map(|item| item.url)
            .collect(),
        author: playlist
            .channel
            .clone()
            .map(|c| c.name)
            .unwrap_or("Unknown channel".to_string()),
        author_id: playlist
            .channel
            .clone()
            .map(|c| c.id)
            .unwrap_or("Unknown channel id".to_string()),
        author_verified: playlist
            .channel
            .map(|c| c.verification.verified())
            .unwrap_or_default(),
        video_count: playlist.video_count.unwrap_or_default() as i32,
        videos: Vec::new(),
    }
}

fn channel_item_convert(channel: ChannelItem) -> CommonChannel {
    crate::global::common::CommonChannel {
        name: channel.name,
        url: format!("https://www.youtube.com/channel/{}", channel.id),
        id: channel.id,
        verified: channel.verification.verified(),
        thumbnails: channel.avatar.into_iter().map(image_convert).collect(),
        auto_generated: false,
        subscribers: channel.subscriber_count.unwrap_or_default() as u32,
        video_count: 0,
        description: channel.short_description.clone(),
        description_html: channel.short_description,
    }
}

impl SearchProviderTrait for RustyPipeWrapper {
    fn supports_video(&self) -> bool {
        true
    }

    fn video(
        &self,
        id: &str,
    ) -> Result<crate::global::common::video::Video, Box<dyn std::error::Error>> {
        let query = self.0.query();
        let (player, details) = RUNTIME
            .get()
            .unwrap()
            .block_on(async { futures::join!(query.player(id), query.video_details(id)) });

        let player = player?;
        let details = details?;

        Ok(Video {
            r#type: "video".to_string(),
            title: player
                .details
                .name
                .unwrap_or("<Unable to fetch video name>".to_string()),
            id: player.details.id,
            thumbnails: player
                .details
                .thumbnail
                .iter()
                .cloned()
                .map(thumbnail_convert)
                .collect(),
            storyboards: Vec::new(),
            description: details.description.to_plaintext(),
            description_html: details.description.to_html(),
            published: details
                .publish_date
                .map(|time| time.to_utc().unix_timestamp() as u64)
                .unwrap_or_default(),
            published_text: details
                .publish_date_txt
                .unwrap_or("Unknown publish date".to_string()),
            keywords: player.details.keywords,
            views: details.view_count,
            likes: details.like_count.unwrap_or_default(),
            dislikes: 0, // TODO
            paid: false,
            premium: false,
            family_friendly: false,
            allowed_regions: Vec::new(),
            genre: String::from("Unknown genre"),
            genre_url: None,
            author: details.channel.name,
            author_id: details.channel.id.clone(),
            author_url: format!("https://www.youtube.com/channel/{}", details.channel.id),
            author_thumbnails: details
                .channel
                .avatar
                .iter()
                .cloned()
                .map(image_convert)
                .collect(),
            sub_count_text: viewcount_text(details.channel.subscriber_count.unwrap_or(0)),
            length: player.details.duration,
            allow_ratings: true,
            rating: 0_f32,
            listed: true,
            live: player.details.is_live,
            upcoming: false,
            premiere_timestamp: 0,
            dash: "No dash".to_string(),
            adaptive_formats: Vec::new(),   // TODO
            format_streams: Vec::new(),     // TODO
            captions: Vec::new(),           // TODO
            recommended_videos: Vec::new(), // TODO
        })
    }

    fn supports_search(&self) -> bool {
        true
    }

    fn search(
        &self,
        filters: &crate::config::Search,
    ) -> Result<Vec<crate::global::common::hidden::SearchItem>, Box<dyn std::error::Error>> {
        let date = match filters.filters.date {
            SearchFilterDate::None => None,
            SearchFilterDate::Hour => Some(UploadDate::Hour),
            SearchFilterDate::Day => Some(UploadDate::Day),
            SearchFilterDate::Week => Some(UploadDate::Week),
            SearchFilterDate::Month => Some(UploadDate::Month),
            SearchFilterDate::Year => Some(UploadDate::Year),
        };

        let sort = match filters.filters.sort {
            SearchFilterSort::Relevance => None,
            SearchFilterSort::Date => Some(Order::Date),
            SearchFilterSort::Views => Some(Order::Views),
            SearchFilterSort::Rating => Some(Order::Rating),
        };

        let duration = match filters.filters.duration {
            SearchFilterDuration::None => None,
            SearchFilterDuration::Long => Some(Length::Long),
            SearchFilterDuration::Short => Some(Length::Short),
            SearchFilterDuration::Medium => Some(Length::Medium),
        };

        let r#type = match filters.filters.r#type {
            SearchFilterType::All => None,
            SearchFilterType::Video => Some(ItemType::Video),
            SearchFilterType::Playlist => Some(ItemType::Playlist),
            SearchFilterType::Channel => Some(ItemType::Channel),
        };

        let res = RUNTIME.get().unwrap().block_on(
            self.0.query().search_filter(
                filters.query.clone(),
                &SearchFilter::new()
                    .date_opt(date)
                    .sort_opt(sort)
                    .length_opt(duration)
                    .item_type_opt(r#type),
            ),
        )?;

        Ok(res
            .items
            .items
            .into_iter()
            .map(|item: YouTubeItem| match item {
                YouTubeItem::Video(video) => SearchItem::Video(video_item_convert(video)),
                YouTubeItem::Playlist(playlist) => {
                    SearchItem::Playlist(playlist_item_convert(playlist))
                }
                YouTubeItem::Channel(channel) => SearchItem::Channel(channel_item_convert(channel)),
            })
            .collect())
    }

    fn supports_channel(&self) -> bool {
        true
    }

    fn channel(
        &self,
        id: &str,
    ) -> Result<crate::global::common::channel::Channel, Box<dyn std::error::Error>> {
        let q = self.0.query();
        let (info, artist) = RUNTIME
            .get()
            .unwrap()
            .block_on(async { futures::join!(q.channel_info(id), q.music_artist(id, false)) });

        let info = info?;
        let artist = artist?;

        Ok(Channel {
            name: artist.name,
            id: info.id,
            url: info.url,
            banners: Vec::new(), // TODO?
            thumbnails: artist.header_image.into_iter().map(image_convert).collect(),
            subscribers: info.subscriber_count.unwrap_or_default() as u32,
            total_views: info.view_count.unwrap_or_default(),
            joined: info
                .create_date
                .map(|d| d.with_hms(0, 0, 0).unwrap().as_utc().unix_timestamp())
                .unwrap_or_default() as u64,
            auto_generated: false,
            family_friendly: false,
            description: info.description.clone(),
            description_html: info.description,
            allowed_regions: Vec::new(), // TODO
            lastest_videos: Vec::new(),
            related_channels: Vec::new(),
        })
    }

    fn supports_trending(&self) -> bool {
        true
    }

    fn trending(
        &self,
    ) -> Result<Vec<crate::global::common::CommonVideo>, Box<dyn std::error::Error>> {
        let res = RUNTIME.get().unwrap().block_on(self.0.query().trending())?;

        Ok(res.into_iter().map(video_item_convert).collect())
    }

    fn supports_playlist(&self) -> bool {
        true
    }

    fn playlist(
        &self,
        id: &str,
    ) -> Result<crate::global::common::universal::Playlist, Box<dyn std::error::Error>> {
        let res = RUNTIME
            .get()
            .unwrap()
            .block_on(self.0.query().playlist(id))?;

        Ok(Playlist {
            title: res.name,
            id: res.id,
            thumbnail: res.thumbnail.into_iter().map(|t| t.url).collect(),
            author: res
                .channel
                .clone()
                .map(|c| c.name)
                .unwrap_or("Unknown channel".to_string()),
            author_id: res
                .channel
                .clone()
                .map(|c| c.id)
                .unwrap_or("Unknown channel".to_string()),
            author_thumbnails: Vec::new(), // TODO why?
            description: res
                .description
                .clone()
                .map(|t| t.to_plaintext())
                .unwrap_or_default(),
            description_html: res.description.map(|t| t.to_html()).unwrap_or_default(),
            video_count: res.video_count as u32,
            views: 0,
            updated: res
                .last_update
                .map(|d| d.with_hms(0, 0, 0).unwrap().as_utc().unix_timestamp())
                .unwrap_or_default() as u64,
            listed: true,
            videos: res
                .videos
                .items
                .into_iter()
                .map(|item| crate::global::common::hidden::PlaylistItem {
                    title: item.name,
                    id: item.id,
                    author: res
                        .channel
                        .clone()
                        .map(|c| c.name)
                        .unwrap_or("Unknown channel".to_string()),
                    author_id: res
                        .channel
                        .clone()
                        .map(|c| c.id)
                        .unwrap_or("Unknown channel id".to_string()),
                    author_url: res
                        .channel
                        .clone()
                        .map(|c| format!("https://www.youtube.com/channel/{}", c.id))
                        .unwrap_or("Unknown channel id".to_string()),
                    thumbnails: item.thumbnail.into_iter().map(thumbnail_convert).collect(),
                    index: 0,
                    length: res.video_count as u32,
                })
                .collect(),
        })
    }

    fn supports_channel_videos(&self) -> bool {
        true
    }

    fn channel_videos(&self, id: &str) -> Result<Vec<CommonVideo>, Box<dyn std::error::Error>> {
        let res = RUNTIME
            .get()
            .unwrap()
            .block_on(self.0.query().channel_videos(id))?;

        Ok(res
            .content
            .items
            .into_iter()
            .map(video_item_convert)
            .collect())
    }

    fn supports_channel_playlists(&self) -> bool {
        true
    }

    fn channel_playlists(
        &self,
        id: &str,
    ) -> Result<Vec<CommonPlaylist>, Box<dyn std::error::Error>> {
        let res = RUNTIME
            .get()
            .unwrap()
            .block_on(self.0.query().channel_playlists(id))?;

        Ok(res
            .content
            .items
            .into_iter()
            .map(playlist_item_convert)
            .collect())
    }
}
