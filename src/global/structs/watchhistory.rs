use super::Item;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};
use typemap::Key;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WatchHistory(pub Vec<Item>);

impl Key for WatchHistory {
    type Value = Self;
}

impl WatchHistory {
    /// saves the current state of watch history into a file
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let save_string = serde_json::to_string_pretty(
            &self
                .0
                .iter()
                .map(|item| item.id().unwrap_or_default())
                .filter(|id| !id.is_empty())
                .collect::<Vec<&str>>(),
        )?;
        let path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history/index.json");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(save_string.as_bytes())?;

        Ok(())
    }

    /// add an item to watch history
    pub fn push(&mut self, item: Item, max_items: usize) -> Result<(), Box<dyn Error>> {
        let info = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history/info/");

        // removes duplicates and place them on top (if exists)
        let id = item.id().unwrap_or("invalid-dump");
        if let Some(index) = self
            .0
            .iter()
            .position(|item_in_iter| item_in_iter.id().unwrap_or_default() == id)
        {
            let _ = fs::remove_file(info.join(format!("{id}.json")));
            self.0.remove(index);
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(info.join(format!("{id}.json")))?;
        let item_string = serde_json::to_string(&item)?;
        file.write_all(item_string.as_bytes())?;

        self.0.push(item);

        // if length of history exceeds the maximum history length, removes that oldest item
        if self.0.len() > max_items {
            let _ = fs::remove_file(
                info.join(format!("{}.json", self.0[0].id().unwrap_or("invalid-dump"))),
            );
            self.0.remove(0);
        }

        Ok(())
    }

    /// loads watch history from file
    pub fn load() -> Self {
        let path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history/index.json");
        let res = (|| -> Result<Vec<String>, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        if let Ok(deserialized) = res {
            let info = home_dir()
                .unwrap()
                .join(".local/share/youtube-tui/watch_history/info/");
            Self(
                deserialized
                    .into_iter()
                    .filter_map(|id| fs::read_to_string(info.join(format!("{id}.json"))).ok())
                    .filter_map(|file_content| serde_json::from_str(&file_content).ok())
                    .collect(),
            )
        } else {
            // if the file does exist, back it up
            // if it doesn't exist, it will throw an error but we dont care
            let mut new_path = path.clone();
            new_path.pop();
            new_path.push(format!("index-{}.json", chrono::offset::Local::now(),));
            let _ = fs::rename(&path, &new_path);

            Self::default()
        }
    }

    /// moves all thumbnails from storage to cache when starting
    pub fn init_move() {
        let history_thumbnails_path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history/thumbnails/");
        let cache_thumbnails_path = home_dir().unwrap().join(".cache/youtube-tui/thumbnails/");

        fs::read_dir(history_thumbnails_path)
            .unwrap()
            .into_iter()
            .for_each(|path| {
                let path = path.unwrap().path();
                let _ = fs::rename(
                    path.clone(),
                    cache_thumbnails_path.join(path.file_name().unwrap()),
                );
            });
    }

    /// moves thumbnails of videos in watch history from cache back to storage when exiting, so that thumbnails can be viewed offline
    pub fn exit_move(&self) {
        let history_thumbnails_path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history/thumbnails/");
        let cache_thumbnails_path = home_dir().unwrap().join(".cache/youtube-tui/thumbnails/");

        self.0.iter().for_each(|item| {
            let id = item.id().unwrap();
            let _ = fs::rename(
                cache_thumbnails_path.join(id),
                history_thumbnails_path.join(id),
            );

            if let Ok(fullplaylist) = item.fullplaylist() {
                fullplaylist.videos.iter().for_each(|video| {
                    let id = video.id().unwrap_or("invalid-dump");
                    let _ = fs::rename(
                        cache_thumbnails_path.join(id),
                        history_thumbnails_path.join(id),
                    );
                })
            }
        })
    }
}
