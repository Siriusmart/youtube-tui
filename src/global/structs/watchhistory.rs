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
        let save_string = serde_json::to_string(self)?;
        let path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history.json");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        file.write_all(save_string.as_bytes())?;

        Ok(())
    }

    /// add an item to watch history
    pub fn push(&mut self, item: Item, max_items: usize) {
        // removes duplicates and place them on top (if exists)
        if let Some(index) = self
            .0
            .iter()
            .position(|item_in_iter| item_in_iter.id() == item.id())
        {
            self.0.remove(index);
        }
        self.0.push(item);

        // if length of history exceeds the maximum history length, removes that oldest item
        if self.0.len() > max_items {
            self.0.remove(0);
        }
    }

    /// loads watch history from file
    pub fn load() -> Self {
        let path = home_dir()
            .unwrap()
            .join(".local/share/youtube-tui/watch_history.json");
        let res = (|| -> Result<Self, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        if let Ok(deserialized) = res {
            deserialized
        } else {
            // if the file does exist, back it up
            // if it doesn't exist, it will throw an error but we dont care
            let mut new_path = path.clone();
            new_path.pop();
            new_path.push(format!(
                "watch_history-{}.json",
                chrono::offset::Local::now(),
            ));
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
        })
    }
}
