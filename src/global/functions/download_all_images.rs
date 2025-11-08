use crate::global::structs::{Item, LocalStore};
use std::{collections::HashSet, error::Error, fs, io::Cursor, path::PathBuf, sync::OnceLock, thread};

pub struct DownloadRequest {
    pub url: String,
    pub id: String,
}

impl From<&Item> for Option<DownloadRequest> {
    // turn `Item` into a thumbnail download request
    fn from(item: &Item) -> Self {
        Some(match item {
            Item::MiniVideo(minivideo) => DownloadRequest {
                url: minivideo.thumbnail_url.clone(),
                id: minivideo.id.clone(),
            },
            Item::MiniPlaylist(miniplaylist) => DownloadRequest {
                url: miniplaylist.thumbnail_url.clone(),
                id: miniplaylist.id.clone(),
            },
            Item::MiniChannel(minichannel) => DownloadRequest {
                url: minichannel.thumbnail_url.clone(),
                id: minichannel.id.clone(),
            },
            Item::FullVideo(fullvideo) => DownloadRequest {
                url: fullvideo.thumbnail_url.clone(),
                id: fullvideo.id.clone(),
            },
            Item::FullPlaylist(fullplaylist) => DownloadRequest {
                url: fullplaylist.thumbnail_url.clone(),
                id: fullplaylist.id.clone(),
            },
            Item::FullChannel(fullchannel) => DownloadRequest {
                url: fullchannel.thumbnail_url.clone(),
                id: fullchannel.id.clone(),
            },
            _ => return None,
        })
    }
}

/// Function to download all thumbnails (or just any files) to `~/.cache/thumbnails` with  no file exitension (cuz its not needed)
pub fn download_all_images(downloads: Vec<Option<DownloadRequest>>) {
    // do not download the images if non of the features are enabled
    if cfg!(not(any(feature = "sixel", feature = "halfblock"))) {
        return;
    }

    let path = home::home_dir()
        .expect("Cannot get your home directory")
        .join(".local/share/youtube-tui/thumbnails/");

    downloads.into_iter().flatten().for_each(|req| {
        LocalStore::add_image(req.id.clone());
        let path = path.clone().join(req.id);
        if path.exists() {
            return;
        }

        thread::spawn(move || {
            let _ = download_single(&req.url, path);
        });
    })
}

fn httpreq_get(url: &str) -> Result<Vec<u8>, http_req::error::Error> {
    let mut buffer = Vec::new();
    http_req::request::get(url, &mut buffer)?;
    Ok(buffer)
}

fn download_single(url: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let res = httpreq_get(url)?;
    let mut file = fs::File::create(path)?;
    let mut content = Cursor::new(res);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
