use std::{collections::HashMap, fs, sync::OnceLock};

use home::home_dir;

use crate::global::structs::Item;

static mut LOCALSTORE: OnceLock<LocalStore> = OnceLock::new();

/// cached access and write files to ~/.local/share
#[derive(Default)]
pub struct LocalStore {
    info: HashMap<String, Item>,
}

impl LocalStore {
    pub fn init() {
        unsafe {
            let _ = LOCALSTORE.set(Self::default());
        }
    }

    pub fn get_info(id: &str) -> Option<Item> {
        let localstore = unsafe { LOCALSTORE.get_mut() }.unwrap();

        match localstore.info.get(id) {
            Some(item) => Some(item.clone()),
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

    pub fn set_info(id: String, item: Item) {
        let localstore = unsafe { LOCALSTORE.get_mut() }.unwrap();
        localstore.info.insert(id, item);
    }
}
