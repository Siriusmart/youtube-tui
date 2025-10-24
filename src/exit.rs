use home::home_dir;
use std::{error::Error, fs};
use tui_additions::framework::Framework;

use crate::{
    config::MainConfig,
    global::{
        structs::*,
        traits::{Collection, CollectionNoId},
    },
};

/// function to run when the app ends
// exit tasks:
//  - move thumbnails of videos in watch history to `~/.local/share/youtube-tui/watch_history/thumbnails`
//  - remove `~/.cache`
pub fn exit(framework: &mut Framework) -> Result<(), Box<dyn Error>> {
    let limits = framework.data.global.get::<MainConfig>().unwrap().limits;
    let watchhistory = framework.data.global.get_mut::<WatchHistory>().unwrap();
    watchhistory.trim(limits.watch_history);
    // watchhistory.exit_move();
    let _ = watchhistory.save();
    let subscriptions = framework.data.global.get::<Subscriptions>().unwrap();
    // subscriptions.exit_move();
    let _ = subscriptions.save();
    // framework.data.global.get::<Library>().unwrap().exit_move();
    let searchhistory = framework.data.global.get_mut::<SearchHistory>().unwrap();
    searchhistory.trim(limits.search_history);
    let _ = searchhistory.save();
    let commandhistory = framework.data.global.get_mut::<CommandHistory>().unwrap();
    commandhistory.trim(limits.commands_history);
    let _ = commandhistory.save();

    // let home_dir = home_dir().unwrap();
    // let cache_path = home_dir.join(".cache/youtube-tui/");

    /*
    if cache_path.exists() {
        fs::remove_dir_all(cache_path)?;
    }
    */

    Ok(())
}
