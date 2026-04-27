use std::fs;

use super::paths;

pub fn init_move() {
    let store_dir = paths::data_dir();
    let cache_dir = paths::cache_dir();

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
