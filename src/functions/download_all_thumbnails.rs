use std::{collections::LinkedList, error::Error, io::Cursor, thread};

use futures::future::join_all;
use tokio::runtime::Runtime;

use crate::app::pages::global::ListItem;

pub enum ItemType {
    Video,
    Playlist,
}

pub fn download_all_thumbnails(list: LinkedList<ListItem>) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        let rt: Runtime = tokio::runtime::Runtime::new().unwrap();
        let mut urls: Vec<(&str, &str, ItemType)> = Vec::new();
        for i in list.iter() {
            match i {
                ListItem::Video(video) => {
                    urls.push((&video.video_thumbnail, &video.video_id, ItemType::Video));
                }
                _ => {}
            }
        }

        let _ = rt.block_on(download_items(urls));
    });

    Ok(())
}

pub async fn download_items(urls: Vec<(&str, &str, ItemType)>) -> Result<(), Box<dyn Error>> {
    let mut actions = Vec::new();
    let mut path = home::home_dir().expect("Cannot get your home directory");
    path.push(".siriusmart");
    path.push("youtube-tui");
    path.push("cache");
    path.push("thumbnails");

    for (url, video_id, item_type) in urls {
        path.push(match item_type {
            ItemType::Video => "videos",
            ItemType::Playlist => "playlists",
        });

        path.push(format!("{}.png", video_id));

        if !path.exists() {
            actions.push(fetch_url(
                url,
                path.clone().into_os_string().into_string().unwrap(),
            ));
        }

        path.pop();
        path.pop();
    }

    join_all(actions).await;

    Ok(())
}

async fn fetch_url(url: &str, path: String) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(path)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
