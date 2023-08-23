use std::{fs, path::PathBuf};

use home::home_dir;

use crate::config::MainConfig;

pub fn find_library_item(id: &str, mainconfig: &MainConfig) -> Option<PathBuf> {
    fs::read_dir(
        home_dir()
            .unwrap()
            .join(mainconfig.env.get("save-path")?.replacen("~/", "", 1)),
    )
    .ok()?
    .filter_map(|entry| Some(entry.ok()?.path()))
    .find(|stem| stem.as_os_str().to_str().unwrap_or_default().contains(id))
}
