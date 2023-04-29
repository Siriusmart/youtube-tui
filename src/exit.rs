use home::home_dir;
use std::{error::Error, fs};
use tui_additions::framework::Framework;

use crate::global::{structs::WatchHistory, traits::Collection};

/// function to run when the app ends
// exit tasks:
//  - move thumbnails of videos in watch history to `~/.local/share/youtube-tui/watch_history/thumbnails`
//  - remove `~/.cache`
pub fn exit(framework: &Framework) -> Result<(), Box<dyn Error>> {
    if let Some(history) = framework.data.global.get::<WatchHistory>() {
        history.exit_move()
    }

    let home_dir = home_dir().unwrap();
    let cache_path = home_dir.join(".cache/youtube-tui/");

    if cache_path.exists() {
        fs::remove_dir_all(cache_path)?;
    }

    Ok(())
}
