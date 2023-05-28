use home::home_dir;
use std::{error::Error, fs};
use tui_additions::framework::Framework;

use crate::global::{structs::*, traits::Collection};

/// function to run when the app ends
// exit tasks:
//  - move thumbnails of videos in watch history to `~/.local/share/youtube-tui/watch_history/thumbnails`
//  - remove `~/.cache`
pub fn exit(framework: &Framework) -> Result<(), Box<dyn Error>> {
    framework
        .data
        .global
        .get::<WatchHistory>()
        .unwrap()
        .exit_move();
    let subscriptions = framework.data.global.get::<Subscriptions>().unwrap();
    subscriptions.exit_move();
    subscriptions.save()?;
    framework.data.global.get::<Library>().unwrap().exit_move();

    let home_dir = home_dir().unwrap();
    let cache_path = home_dir.join(".cache/youtube-tui/");

    if cache_path.exists() {
        fs::remove_dir_all(cache_path)?;
    }

    Ok(())
}
