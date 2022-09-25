use home::home_dir;
use std::{error::Error, fs};

/// function to run when the app ends
// exit tasks:
//  - remove `~/.cache`
pub fn exit() -> Result<(), Box<dyn Error>> {
    let home_dir = home_dir().unwrap();
    let cache_path = home_dir.join(".cache/youtube-tui/");

    if cache_path.exists() {
        fs::remove_dir_all(cache_path)?;
    }

    Ok(())
}
