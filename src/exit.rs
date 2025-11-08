use home::home_dir;
use std::{collections::HashSet, error::Error, fs};
use tui_additions::framework::Framework;

use crate::{
    config::MainConfig,
    global::{
        structs::*,
        traits::{Collection, CollectionNoId},
    },
    CACHED_BEFORE,
};

/// function to run when the app ends
// exit tasks:
//  - move thumbnails of videos in watch history to `~/.local/share/youtube-tui/watch_history/thumbnails`
//  - remove `~/.cache`
pub fn exit(framework: &mut Framework) -> Result<(), Box<dyn Error>> {
    let limits = framework.data.global.remove::<MainConfig>().unwrap().limits;
    let mut watchhistory = framework.data.global.remove::<WatchHistory>().unwrap();
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
    let mut commandhistory = framework.data.global.remove::<CommandHistory>().unwrap();
    commandhistory.trim(limits.commands_history);
    let _ = commandhistory.save();
    let library = framework.data.global.get_mut::<Library>().unwrap();
    let _ = library.save();

    let cached_after: HashSet<String> = HashSet::from_iter(
        library
            .items()
            .iter()
            .filter_map(|item| item.id())
            .map(str::to_string)
            .chain(
                watchhistory
                    .items()
                    .iter()
                    .filter_map(|item| item.id())
                    .map(str::to_string),
            ),
    );

    let cached_before = CACHED_BEFORE.get().unwrap();

    let info_path = home_dir().unwrap().join(".local/share/youtube-tui/info/");
    let thumbnail_path = home_dir().unwrap().join(".local/share/youtube-tui/info/");

    // remove cache that no longer exists
    for deleted in cached_before
        .iter()
        .filter(|id| !cached_after.contains(*id))
    {
        let _ = fs::remove_file(info_path.join(deleted).with_extension("json"));
        let _ = fs::remove_file(thumbnail_path.join(deleted));
    }

    LocalStore::save_only(&cached_after);

    // let home_dir = home_dir().unwrap();
    // let cache_path = home_dir.join(".cache/youtube-tui/");

    /*
    if cache_path.exists() {
        fs::remove_dir_all(cache_path)?;
    }
    */

    Ok(())
}
