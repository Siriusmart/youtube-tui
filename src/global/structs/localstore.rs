// currently 3 copies of the same item is stored in ram
// - in localstore
// - in watch history
// - in the actual state
//
// this needs to change
use std::{
    collections::{HashMap, HashSet},
    env::home_dir,
    fs::{self, OpenOptions},
    io::Write,
    sync::OnceLock,
};

use crate::global::structs::Item;

static mut LOCALSTORE: OnceLock<LocalStore> = OnceLock::new();

pub struct LocalRecord {
    item: Item,
    is_new: bool,
}

/// cached access and write files to ~/.local/share
#[derive(Default)]
pub struct LocalStore {
    info: HashMap<String, LocalRecord>,
    downloaded_images: HashSet<String>,
}

impl LocalStore {
    pub fn add_image(id: String) {
        unsafe { LOCALSTORE.get_mut() }.unwrap().downloaded_images.insert(id);
    }

    pub fn init() {
        unsafe {
            let _ = LOCALSTORE.set(Self::default());
        }
    }

    pub fn rm_cache(id: &str) {
        unsafe { LOCALSTORE.get_mut() }.unwrap().info.remove(id);
    }

    pub fn get_info(id: &str) -> Option<Item> {
        let localstore = unsafe { LOCALSTORE.get_mut() }.unwrap();

        match localstore.info.get(id) {
            Some(LocalRecord { item, .. }) => Some(item.clone()),
            None => {
                let path = home_dir()
                    .unwrap()
                    .join(format!(".local/share/youtube-tui/info/{id}.json"));

                if path.exists() {
                    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?
                } else {
                    None
                }
            }
        }
    }

    pub fn set_info(id: String, item: Item, is_new: bool) {
        let localstore = unsafe { LOCALSTORE.get_mut() }.unwrap();
        localstore.info.insert(id, LocalRecord { item, is_new });
    }

    pub fn save_only(ids: &HashSet<String>) {
        let info_path = home_dir().unwrap().join(".local/share/youtube-tui/info/");

        for (id, LocalRecord { item, is_new }) in unsafe { LOCALSTORE.get() }.unwrap().info.iter() {
            let info = info_path.join(id).with_extension("json");
            if *is_new && ids.contains(id) {
                let mut file = match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(info)
                {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                let item_string = serde_json::to_string(&item).unwrap();
                let _ = file.write_all(item_string.as_bytes());
            }
        }
    }

    pub fn list_downloaded_images() -> &'static HashSet<String> {
        &unsafe { LOCALSTORE.get() }.unwrap().downloaded_images
    }
}
