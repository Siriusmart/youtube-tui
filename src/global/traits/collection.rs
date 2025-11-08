use home::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

pub trait CollectionItem {
    fn id(&self) -> Option<&str>;
    fn children_ids(&self) -> Vec<&str>;
}

pub trait Collection<T>
where
    Self: Default + Clone + Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned + CollectionItem,
{
    const INDEX_PATH: &'static str;

    fn items(&self) -> &Vec<T>;
    fn items_mut(&mut self) -> &mut Vec<T>;

    fn from_items(items: Vec<T>) -> Self;

    fn trim(&mut self, length: usize) {
        let items = self.items_mut();
        if items.len() > length {
            items.drain(0..items.len() - length);
        }
    }

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
    fn push(&mut self, item: T) -> Result<(), Box<dyn Error>> {
        // removes duplicates and place them on top (if exists)
        let id = match item.id() {
            Some(t) => t,
            None => return Ok(()) // TODO make error message
        };

        let info = home_dir().unwrap().join(".local/share/youtube-tui/info/").join(format!("{id}.json"));

        if let Some(index) = self
            .items_mut()
            .iter_mut()
            .position(|item_in_iter| item_in_iter.id().unwrap_or_default() == id)
        {
            self.items_mut().remove(index);
        }

        // TODO use central cache struct
        if !info.exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(info)?;
            let item_string = serde_json::to_string(&item)?;
            file.write_all(item_string.as_bytes())?;
        }

        self.items_mut().push(item);

        Ok(())
    }

    /// loads watch history from file
    fn load() -> Self {
        let path = home_dir().unwrap().join(Self::INDEX_PATH);
        let res = (|| -> Result<Vec<String>, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        let items = if let Ok(deserialized) = res {
            let info = home_dir().unwrap().join(".local/share/youtube-tui/info/");
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
            new_path.push(format!(
                "{}.{}.old",
                Self::INDEX_PATH,
                chrono::offset::Local::now()
            ));
            let _ = fs::rename(&path, &new_path);

            Vec::new()
        };

        Self::from_items(items)
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

            item.children_ids().iter().for_each(|id| {
                let _ = fs::rename(
                    cache_thumbnails_path.join(id),
                    store_thumbnails_path.join(id),
                );
            })
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

pub trait CollectionNoId<T>
where
    Self: Default + Clone + Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned + PartialEq,
{
    const INDEX_PATH: &'static str;

    fn items(&self) -> &Vec<T>;
    fn items_mut(&mut self) -> &mut Vec<T>;

    fn from_items(items: Vec<T>) -> Self;

    fn trim(&mut self, length: usize) {
        let items = self.items_mut();
        if items.len() > length {
            items.drain(0..items.len() - length);
        }
    }
    /// saves the current state of watch history into a file
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let save_string = serde_json::to_string_pretty(&self)?;
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
    fn push(&mut self, item: T) {
        // removes duplicates and place them on top (if exists)
        if let Some(index) = self
            .items_mut()
            .iter_mut()
            .position(|item_in_iter| *item_in_iter == item)
        {
            self.items_mut().remove(index);
        }

        self.items_mut().push(item);
    }

    /// loads watch history from file
    fn load() -> Self {
        let path = home_dir().unwrap().join(Self::INDEX_PATH);
        let res = (|| -> Result<Vec<T>, Box<dyn Error>> {
            let file_string = fs::read_to_string(&path)?;
            let deserialized = serde_json::from_str(&file_string)?;
            Ok(deserialized)
        })();

        // if res is err, then the file either doesn't exist of has be altered incorrectly, in
        // which case returns Self::default()
        let items = res.unwrap_or_default();

        Self::from_items(items)
    }

    /// remove item based on their id
    fn remove(&mut self, matcher: &T) -> bool {
        let items = self.items_mut();

        if let Some(found) = items
            .iter()
            .enumerate()
            .find(|(_index, item)| *item == matcher)
        {
            items.remove(found.0);
            true
        } else {
            false
        }
    }
}
