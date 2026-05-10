use std::path::PathBuf;

/// Returns the configuration directory for youtube-tui.
///
/// If `YOUTUBETUI_CONFIG_HOME` is set, it is used directly as the config directory.
/// Otherwise falls back to the platform default with a `youtube-tui` subdirectory:
/// - Linux:   `~/.config/youtube-tui/`
/// - Windows: `%APPDATA%\youtube-tui\`
/// - macOS:   `~/Library/Application Support/youtube-tui/`
pub fn config_dir() -> PathBuf {
    std::env::var("YOUTUBETUI_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::config_dir()
                .expect("no config dir, set YOUTUBETUI_CONFIG_HOME to override")
                .join("youtube-tui")
        })
}

/// Returns the data directory for youtube-tui.
///
/// If `YOUTUBETUI_DATA_HOME` is set, it is used directly as the data directory.
/// Otherwise falls back to the platform default with a `youtube-tui` subdirectory:
/// - Linux:   `~/.local/share/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\`
/// - macOS:   `~/Library/Application Support/youtube-tui/`
pub fn data_dir() -> PathBuf {
    std::env::var("YOUTUBETUI_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .expect("no data dir, set YOUTUBETUI_DATA_HOME to override")
                .join("youtube-tui")
        })
}

/// Returns the cache directory for youtube-tui.
///
/// If `YOUTUBETUI_CACHE_HOME` is set, it is used directly as the cache directory.
/// Otherwise falls back to the platform default with a `youtube-tui` subdirectory:
/// - Linux:   `~/.cache/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\cache\`
/// - macOS:   `~/Library/Caches/youtube-tui/`
pub fn cache_dir() -> PathBuf {
    std::env::var("YOUTUBETUI_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::cache_dir()
                .expect("no cache dir, set YOUTUBETUI_CACHE_HOME to override")
                .join("youtube-tui")
        })
}

/// Returns the storage directory for rustypipe.
///
/// Uses the parent of `YOUTUBETUI_DATA_HOME` (sibling to the youtube-tui data dir) if set,
/// otherwise falls back to the platform data dir with a `rustypipe` subdirectory.
pub fn rustypipe_dir() -> PathBuf {
    std::env::var("YOUTUBETUI_DATA_HOME")
        .map(|p| {
            let base = PathBuf::from(p);
            let parent = base.parent().map(|p| p.to_path_buf()).unwrap_or(base);
            parent.join("rustypipe")
        })
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .expect("no data dir, set YOUTUBETUI_DATA_HOME to override")
                .join("rustypipe")
        })
}

/// Returns the default save path string for the `save-path` env variable.
pub fn default_save_path() -> String {
    let p = data_dir().join("saved");
    let mut s = p.to_string_lossy().to_string();
    if !s.ends_with(std::path::MAIN_SEPARATOR) {
        s.push(std::path::MAIN_SEPARATOR);
    }
    s
}

/// Returns the default download path string for the `download-path` env variable.
///
/// If `YOUTUBETUI_DOWNLOAD_HOME` is set, it is used directly as the download directory.
/// Otherwise falls back to the platform downloads directory.
pub fn default_download_path() -> String {
    let downloads = std::env::var("YOUTUBETUI_DOWNLOAD_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::download_dir().expect("no download dir, set YOUTUBETUI_DOWNLOAD_HOME to override")
        });
    let mut s = downloads.to_string_lossy().to_string();
    if !s.ends_with(std::path::MAIN_SEPARATOR) {
        s.push(std::path::MAIN_SEPARATOR);
    }
    format!("{s}%(title)s-%(id)s.%(ext)s")
}
