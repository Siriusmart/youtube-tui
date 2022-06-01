use std::{collections::LinkedList, fs};

use serde::{Deserialize, Serialize};

use crate::app::config::Config;

use super::ListItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistory(pub LinkedList<ListItem>);

impl Default for WatchHistory {
    fn default() -> Self {
        WatchHistory(LinkedList::new())
    }
}

impl WatchHistory {
    pub fn load() -> Self {
        let mut dir = home::home_dir().expect("Cannot get your home directory");
        dir = dir.join(".local/share/youtube-tui/watch_history/watch_history.json");

        if !dir.exists() {
            let out = Self::default();

            fs::write(dir, serde_json::to_string(&out).unwrap())
                .expect("Cannot write watch history");

            return out;
        }

        let contents = fs::read_to_string(dir).expect("Cannot read watch history file");

        serde_json::from_str(&contents).expect("Cannot parse watch history file")
    }

    pub fn push(&mut self, item: String, listitem: ListItem, config: &Config) {
        if let Some(v) = self.0.front() {
            if *v.id() == item {
                return;
            }
        }

        let home_dir = home::home_dir().expect("Cannot get your home directory");

        let mut original_dir = home_dir.join(".cache/youtube-tui/thumbnails");

        match &listitem {
            ListItem::FullVideo(video) => {
                let mut save_file = home_dir.join(".local/share/youtube-tui/watch_history/info");
                save_file.push(format!("{}.json", video.video_id));
                fs::write(save_file, serde_json::to_string(&listitem).unwrap()).unwrap();

                let file_name = format!("{}.png", video.video_id);
                original_dir.push(file_name.clone());

                if original_dir.exists() {
                    let mut new_dir =
                        home_dir.join(".local/share/youtube-tui/watch_history/thumbnails");
                    new_dir.push(file_name);

                    if !new_dir.exists() {
                        fs::copy(original_dir, new_dir).expect("Cannot copy thumbnail");
                    }
                }
            }

            ListItem::FullPlayList(playlist) => {
                let mut save_file = home_dir.join(".local/share/youtube-tui/watch_history/info");
                save_file.push(format!("{}.json", playlist.playlist_id));
                fs::write(save_file, serde_json::to_string(&listitem).unwrap()).unwrap();

                let file_name = format!("{}.png", playlist.playlist_id);
                original_dir.push(file_name.clone());

                if original_dir.exists() {
                    let mut new_dir =
                        home_dir.join(".local/share/youtube-tui/watch_history/thumbnails");
                    new_dir.push(file_name);

                    if !new_dir.exists() {
                        fs::copy(original_dir, new_dir).expect("Cannot copy thumbnail");
                    }
                }
            }
            _ => {}
        }

        self.0.push_front(listitem);

        if self.0.len() > config.main.max_watch_history {
            let back = self.0.pop_back();
            if let Some(item) = back {
                let id = item.id();
                let _ = fs::remove_file(
                    home_dir
                        .join(".local/share/youtube-tui/watch_history/info")
                        .join(format!("{}.json", id.clone())),
                );
                let _ = fs::remove_file(
                    home_dir
                        .join(".local/share/youtube-tui/watch_history/thumbnails")
                        .join(format!("{}.png", id)),
                );
            }
        }

        fs::write(
            home_dir.join(".local/share/youtube-tui/watch_history/watch_history.json"),
            serde_json::to_string(&self).unwrap(),
        )
        .expect("Cannot write watch history");
    }
}
