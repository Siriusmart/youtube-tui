use std::{collections::LinkedList, error::Error, io::Cursor, thread};

use futures::future::join_all;
use tokio::runtime::Runtime;

pub enum ItemType {
    Video,
    Playlist,
}

pub fn download_all_thumbnails(
    list: LinkedList<(String, String)>, // (url, id)
) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        let rt: Runtime = tokio::runtime::Runtime::new().unwrap();

        let _ = rt.block_on(download_items(list));
    });

    Ok(())
}

pub async fn download_items(
    urls: LinkedList<(String, String)>,
) -> Result<(), Box<dyn Error>> {
    let mut actions = Vec::new();
    let mut path = home::home_dir().expect("Cannot get your home directory");
    path.push(".cache");
    path.push("youtube-tui");
    path.push("thumbnails");

    for (url, video_id) in urls.iter() {

        path.push(format!("{}.png", video_id));

        if !path.exists() {
            actions.push(fetch_url(
                url.as_str(),
                path.clone().into_os_string().into_string().unwrap(),
            ));
        }

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
