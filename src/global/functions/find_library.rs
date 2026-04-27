use std::{fs, path::PathBuf};

use crate::config::MainConfig;

use super::paths;

pub fn find_library_item(id: &str, mainconfig: &MainConfig) -> Option<PathBuf> {
    let save_path_str = mainconfig.env.get("save-path")?;

    // Resolve save-path: handle tilde expansion for backward compatibility
    let save_path = if save_path_str.starts_with("~/") || save_path_str.starts_with("~\\") {
        home::home_dir()
            .unwrap()
            .join(&save_path_str[2..])
    } else {
        // If it's an absolute path (e.g., from paths::default_save_path()), use directly
        let p = PathBuf::from(save_path_str);
        if p.is_absolute() {
            p
        } else {
            // Relative path: treat as relative to data_dir for safety
            paths::data_dir().join(save_path_str)
        }
    };

    fs::read_dir(save_path)
        .ok()?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .find(|stem| stem.as_os_str().to_str().unwrap_or_default().contains(id))
}
