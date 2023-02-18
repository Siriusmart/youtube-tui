use std::fs;

use home::home_dir;

pub fn init_move() {
    let home_dir = home_dir().unwrap();
    let store_dir = home_dir.join(".local/share/youtube-tui/");
    let cache_dir = home_dir.join(".cache/youtube-tui");

    let store_info_dir = store_dir.join("info/");
    let cache_info_dir = cache_dir.join("info/");
    fs::read_dir(store_info_dir).unwrap().for_each(|entry| {
        let entry = entry.unwrap().path();
        let _ = fs::rename(&entry, cache_info_dir.join(entry.file_name().unwrap()));
    });

    let store_thumbnails_dir = store_dir.join("thumbnails/");
    let cache_thumbnails_dir = cache_dir.join("thumbnails/");
    fs::read_dir(store_thumbnails_dir)
        .unwrap()
        .for_each(|entry| {
            let entry = entry.unwrap().path();
            let _ = fs::rename(
                &entry,
                cache_thumbnails_dir.join(entry.file_name().unwrap()),
            );
        });
}
