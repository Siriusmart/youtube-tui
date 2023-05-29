use super::*;
use crate::{global::{
    functions::{download_all_images, DownloadRequest},
    traits::{Collection, CollectionItem},
}, config::SyncConfig};
use chrono::Utc;
use home::home_dir;
use serde::*;
use std::{
    error::Error,
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
    sync::{atomic::AtomicU32, mpsc, Arc},
    thread,
};
use typemap::Key;

#[derive(Serialize, Deserialize, Default, Clone)]
/// id, videos
pub struct Subscriptions(pub Vec<SubItem>);

impl Key for Subscriptions {
    type Value = Self;
}

impl Subscriptions {
    /// Res<(success, failed)>
    pub fn sync(
        &mut self,
        client: &InvidiousClient,
        image_index: usize,
        download_thumbnails: bool,
        syncconfig: SyncConfig
    ) -> (u32, u32, u32, u32) {
        let failed = Arc::new(AtomicU32::new(0));
        let success = Arc::new(AtomicU32::new(0));
        let empty = Arc::new(AtomicU32::new(0));
        let cached = Arc::new(AtomicU32::new(0));

        let len = self.0.len();
        let now = chrono::Utc::now().timestamp() as u64;
        let (tx, rx) = mpsc::channel();
        let mut channels = Vec::new();
        std::mem::swap(&mut self.0, &mut channels);

        channels.into_iter().for_each(|mut item| {
            if item.last_sync > now - syncconfig.sync_videos_cooldown_secs {
                tx.send(item).unwrap();
                cached.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return;
            }
            let tx = tx.clone();
            let client = client.clone();
            let success = success.clone();
            let failed = failed.clone();
            let empty = empty.clone();
            thread::spawn(move || {
                let res = sync_one(&item.channel.id, &client, image_index, download_thumbnails, syncconfig.sync_channel_info && syncconfig.sync_channel_cooldown_secs + item.last_sync_channel < now);
                match res {
                    Ok((videos, _channel)) if videos.is_empty() => {
                        empty.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                    Ok((videos, channel)) => {
                        success.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        // cannot just compare video publish timestamp to sync timestamp
                        // because publish timestamp is hugely inaccurate seen here
                        // https://github.com/iv-org/invidious/issues/570
                item.has_new = !videos.is_empty() && (item.videos.is_empty() || videos[0].id != item.videos[0].id);
                        item.videos = item.videos;
                        item.last_sync = now;

                        if let Some(channel) = channel {
                            item.channel = channel;
                            item.last_sync_channel = now;
                        }
                    }
                    _ => {
                        failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                tx.send(item)
            });
        });

        for item in rx.into_iter().take(len) {
            self.0.push(item);
        }

        self.0.sort();

        let _ = self.save();

        (
            success.load(std::sync::atomic::Ordering::Relaxed),
            failed.load(std::sync::atomic::Ordering::Relaxed),
            empty.load(std::sync::atomic::Ordering::Relaxed),
            cached.load(std::sync::atomic::Ordering::Relaxed),
        )
    }

    pub fn sync_one(
        &mut self,
        id: &str,
        client: &InvidiousClient,
        image_index: usize,
        download_thumbnails: bool,
        syncconfig: &SyncConfig
    ) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().timestamp() as u64;
        match self.0.iter_mut().find(|item| &item.channel.id == id) {
            Some(item) => {
                let (videos, channel) = sync_one(&id, &client, image_index, download_thumbnails, syncconfig.sync_channel_info && syncconfig.sync_channel_cooldown_secs + item.last_sync_channel < now)?;

                item.has_new = !videos.is_empty() && (item.videos.is_empty() || videos[0].id != item.videos[0].id);
                item.videos = videos;
                item.last_sync = now;
                if let Some(channel) = channel {
                    item.channel = channel;
                    item.last_sync_channel = now;
                }
            }
            None => {
                let (videos, channel) = sync_one(&id, &client, image_index, download_thumbnails, true)?;
                self.0.push(SubItem { channel: channel.unwrap(), videos, last_sync: now, last_sync_channel: now, has_new: true })

            }
        }

        self.0.sort();

        self.save()?;
        Ok(())
    }

    pub fn remove_one(&mut self, id: &str) -> bool {
        if let Some(i) = self.0.iter().position(|item| item.channel.id == id) {
            self.0.remove(i);
            return true;
        }

        false
    }

    pub fn get_all_videos(&self) -> Vec<MiniVideoItem> {
        let mut videos = self
            .0
            .iter()
            .flat_map(|item| item.videos.clone())
            .collect::<Vec<_>>();
        videos.sort();
        videos
    }

    pub fn get_channels(&self) -> Vec<FullChannelItem> {
        self.0.iter().map(|item| item.channel.clone()).collect()
    }
}

fn sync_one(
    id: &str,
    client: &InvidiousClient,
    image_index: usize,
    download_thumbnails: bool,
    sync_channel_info: bool,
) -> Result<(Vec<MiniVideoItem>, Option<FullChannelItem>), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();

    if sync_channel_info {
    let client2 = client.clone();
    let id2 = id.to_string();
    thread::spawn(move || {
        tx.send(
            client2
                .0
                .channel(&id2, None)
                .map(|channel| Item::from_full_channel(channel, image_index).into_fullchannel())
                .ok(),
        )
    });
    }
    let mut videos = client
        .0
        .channel_videos(id, None)?
        .videos
        .into_iter()
        .map(|video| {
            Item::from_channel_video(video, image_index)
                .into_minivideo()
                .unwrap()
        })
        .collect::<Vec<_>>();
    videos.sort();

    let channel =

    if sync_channel_info {
        Some(rx
        .recv()
        .unwrap()
        .ok_or(Errors::StrError("failed to get channel info {id}"))??)
    } else {
        None
    };

    if download_thumbnails {
        let thumbnails: Vec<Option<DownloadRequest>> = videos
            .iter()
            .map(|video| {
                Some(DownloadRequest {
                    url: video.thumbnail_url.clone(),
                    id: video.id.clone(),
                })
            })
            .collect::<Vec<_>>();
        thread::spawn(move || {
            download_all_images(thumbnails);
        });
        if let Some(channel) = &channel {
            download_all_images(vec![Some(DownloadRequest { url: channel.thumbnail_url.clone(), id: channel.id.clone()})]);
        }
    }

    Ok((
        videos,
        channel
    ))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SubItem {
    pub channel: FullChannelItem,
    pub videos: Vec<MiniVideoItem>,
    pub last_sync: u64,
    pub last_sync_channel: u64,
    pub has_new: bool,
}

impl Eq for SubItem {}
impl Ord for SubItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for SubItem {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.channel.name)
    }
}

impl PartialEq for SubItem {
    fn eq(&self, other: &Self) -> bool {
        self.channel.id == other.channel.id
    }
}

impl PartialOrd for SubItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let get_timestamp = |item: &Self| match item.videos.first() {
            Some(video) => video.timestamp.unwrap(),
            None => 0,
        };
        get_timestamp(other).partial_cmp(&get_timestamp(self))
    }
}

impl CollectionItem for SubItem {
    fn id(&self) -> Option<&str> {
        Some(&self.channel.id)
    }

    fn children_ids(&self) -> Vec<&str> {
        self.videos
            .iter()
            .map(|video| video.id.as_str())
            .collect::<_>()
    }
}

impl Collection<SubItem> for Subscriptions {
    const INDEX_PATH: &'static str = ".local/share/youtube-tui/subscriptions.json";

    fn items(&self) -> &Vec<SubItem> {
        &self.0
    }

    fn items_mut(&mut self) -> &mut Vec<SubItem> {
        &mut self.0
    }

    fn from_items(items: Vec<SubItem>) -> Self {
        Self(items)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(home_dir().unwrap().join(Self::INDEX_PATH))?;

        let save_string = serde_json::to_string_pretty(self)?;
        file.write_all(save_string.as_bytes())?;
        Ok(())
    }

    fn push(&mut self, _: SubItem, _: Option<usize>) -> Result<(), Box<dyn Error>> {
        unimplemented!("not in use");
    }

    fn load() -> Self {
        let path = home_dir().unwrap().join(Self::INDEX_PATH);
        let res = (|| -> Result<Self, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        if let Ok(mut subs) = res {
            subs.0.sort();
            subs
        } else {
            // if the file does exist, back it up
            // if it doesn't exist, it will throw an error but we dont care
            let mut new_path = path.clone();
            new_path.pop();
            new_path.push(format!(
                "{}.{}.old",
                Self::INDEX_PATH,
                chrono::offset::Local::now()
            ));
            let _ = fs::rename(&path, &new_path);

            Self::default()
        }
    }
}
