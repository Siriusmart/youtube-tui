use crate::global::structs::Item;
use home::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};
use typemap::Key;

pub trait Collection
where
    Self: Default + Clone + Serialize + DeserializeOwned + Key,
{
    const INDEX_PATH: &'static str;

    fn items(&self) -> &Vec<Item>;
    fn items_mut(&mut self) -> &mut Vec<Item>;

    /// saves the current state of watch history into a file
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let save_string = serde_json::to_string_pretty(
            &self
                .items()
                .iter()
                .map(|item| item.id().unwrap_or_default())
                .filter(|id| !id.is_empty())
                .collect::<Vec<&str>>(),
        )?;
        let path = home_dir().unwrap().join(Self::INDEX_PATH);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(save_string.as_bytes())?;

        Ok(())
    }

    /// add an item to watch history
    fn push(&mut self, item: Item, max_length: Option<usize>) -> Result<(), Box<dyn Error>> {
        let info = home_dir().unwrap().join(".cache/youtube-tui/info/");

        // removes duplicates and place them on top (if exists)
        let id = item.id().unwrap_or("invalid-dump");
        if let Some(index) = self
            .items_mut()
            .iter_mut()
            .position(|item_in_iter| item_in_iter.id().unwrap_or_default() == id)
        {
            self.items_mut().remove(index);
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(info.join(format!("{id}.json")))?;
        let item_string = serde_json::to_string(&item)?;
        file.write_all(item_string.as_bytes())?;

        self.items_mut().push(item);

        // if length of history exceeds the maximum history length, removes that oldest item
        if self.items().len() > max_length.unwrap_or(usize::MAX) {
            self.items_mut().remove(0);
        }

        Ok(())
    }

    /// loads watch history from file
    fn load() -> Vec<Item> {
        let path = home_dir().unwrap().join(Self::INDEX_PATH);
        let res = (|| -> Result<Vec<String>, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        if let Ok(deserialized) = res {
            let info = home_dir().unwrap().join(".cache/youtube-tui/info/");
            deserialized
                .into_iter()
                .filter_map(|id| fs::read_to_string(info.join(format!("{id}.json"))).ok())
                .filter_map(|file_content| serde_json::from_str(&file_content).ok())
                .collect()
        } else {
            // if the file does exist, back it up
            // if it doesn't exist, it will throw an error but we dont care
            let mut new_path = path.clone();
            new_path.pop();
            new_path.push(format!("index-{}.json", chrono::offset::Local::now(),));
            let _ = fs::rename(&path, &new_path);

            Vec::new()
        }
    }

    /// moves thumbnails of videos in watch history from cache back to storage when exiting, so that thumbnails can be viewed offline
    fn exit_move(&self) {
        let home_dir = home_dir().unwrap();
        let store_thumbnails_path = home_dir.join(".local/share/youtube-tui/thumbnails/");
        let store_info_path = home_dir.join(".local/share/youtube-tui/info/");
        let cache_thumbnails_path = home_dir.join(".cache/youtube-tui/thumbnails/");
        let cache_info_path = home_dir.join(".cache/youtube-tui/info");

        self.items().iter().for_each(|item| {
            let id = item.id().unwrap_or("invalid-dump");
            let _ = fs::rename(
                cache_thumbnails_path.join(id),
                store_thumbnails_path.join(id),
            );

            let _ = fs::rename(
                cache_info_path.join(format!("{id}.json")),
                store_info_path.join(format!("{id}.json")),
            );

            if let Item::FullPlaylist(fullplaylist) = item {
                fullplaylist.videos.iter().for_each(|video| {
                    let id = video.id().unwrap_or("invalid-dump");
                    let _ = fs::rename(
                        cache_thumbnails_path.join(id),
                        store_thumbnails_path.join(id),
                    );
                })
            }
        })
    }

    /// remove item based on their id
    fn remove(&mut self, id: &str) -> bool {
        let items = self.items_mut();

        if let Some(found) = items
            .iter()
            .enumerate()
            .find(|(_index, item)| item.id() == Some(id))
        {
            items.remove(found.0);
            true
        } else {
            false
        }
    }
}
