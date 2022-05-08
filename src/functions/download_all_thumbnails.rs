use std::{collections::LinkedList, error::Error, io::Cursor};

use futures::executor::block_on;

use crate::app::pages::global::ListItem;

pub enum ItemType {
    Video,
    Playlist,
}

pub fn download_all_thumbnails(list: &LinkedList<ListItem>) -> Result<(), Box<dyn Error>>{
    let mut urls: Vec<(&str, &str, ItemType)> = Vec::new();
    for i in list.iter() {
        match i {
            ListItem::Video(video) => {
                urls.push((&video.video_thumbnail, &video.video_id, ItemType::Video));
            }
            _ => {}
        }
    }

    block_on(download_items(urls))?;

    Ok(())
}

pub async fn download_items(urls: Vec<(&str, &str, ItemType)>) -> Result<(), Box<dyn Error>>{
    let mut dir = home::home_dir().expect("Cannot get your home directory");
    dir.push(".siriusmart");
    dir.push("youtube-tui");
    dir.push("cache");

    for (url, video_id, item_type) in urls {
        dir.push(match item_type {
            ItemType::Video => "videos",
            ItemType::Playlist => "playlists",
        });
        
        fetch_url(url, &format!("{:?}/{}.png", dir, video_id)).await?;
    }

    Ok(())
}

async fn fetch_url(url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}