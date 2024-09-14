use crate::{config::MainConfig, global::functions::download_all_images, global::structs::Item};
use invidious::{ClientSync, ClientSyncTrait};
use std::error::Error;

pub fn load_playlist(
    client: &ClientSync,
    id: &str,
    mainconfig: &MainConfig,
) -> Result<Item, Box<dyn Error>> {
    let playlist = Item::from_full_playlist(client.playlist(id, None)?, mainconfig.image_index);
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

pub fn load_video(
    client: &ClientSync,
    id: &str,
    mainconfig: &MainConfig,
) -> Result<Item, Box<dyn Error>> {
    let video = Item::from_full_video(client.video(id, None)?, mainconfig.image_index);
    if mainconfig.images.display() {
        download_all_images(vec![(&video).into()]);
    }

    Ok(video)
}
