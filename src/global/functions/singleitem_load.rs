use crate::{
    config::MainConfig,
    global::{functions::download_all_images, structs::Item, traits::SearchProviderWrapper},
};
use std::{error::Error, fs};
use home::home_dir;

pub fn load_playlist(id: &str, mainconfig: &MainConfig) -> Result<Item, Box<dyn Error>> {
    let playlist =
        Item::from_full_playlist(SearchProviderWrapper::playlist(id)?, mainconfig.image_index);
    let videos = &playlist.fullplaylist()?.videos;

    if mainconfig.images.display() {
        download_all_images({
            let mut items = videos.iter().map(|item| item.into()).collect::<Vec<_>>();
            items.extend([(&playlist).into()]);
            items
        });
    }

    Ok(playlist)
}

pub fn load_video(id: &str, mainconfig: &MainConfig) -> Result<Item, Box<dyn Error>> {
    let video = Item::from_full_video(SearchProviderWrapper::video(id)?, mainconfig.image_index);
    if mainconfig.images.display() {
        download_all_images(vec![(&video).into()]);
    }

    Ok(video)
}

pub fn load_channel(id: &str, mainconfig: &MainConfig) -> Result<Item, Box<dyn Error>> {
    let home_dir = home_dir().unwrap();
    let cache_path = home_dir.join(format!(".cache/youtube-tui/channels/{id}.json"));
    
    if cache_path.exists() {
        if let Ok(content) = fs::read_to_string(&cache_path) {
            if let Ok(item) = serde_json::from_str::<Item>(&content) {
                return Ok(item);
            }
        }
    }
    
    let channel = Item::from_full_channel(
        SearchProviderWrapper::channel(id)?,
        mainconfig.image_index,
    );
    
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&cache_path, serde_json::to_string(&channel)?)?;
    
    if mainconfig.images.display() {
        download_all_images(vec![(&channel).into()]);
    }
    
    Ok(channel)
}
