use std::path::PathBuf;

/// Returns the base config directory, respecting XDG_CONFIG_HOME on all platforms.
///
/// On Linux this is handled by `dirs` already. On Windows and macOS, `dirs`
/// ignores XDG env vars, so we check manually first. This lets Windows users
/// keep their config in `~/.config` by setting `XDG_CONFIG_HOME`.
fn sys_config_base() -> PathBuf {
    std::env::var("YOUTUBETUI_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::config_dir()
                .expect("no config dir, set the YOUTUBETUI_CONFIG_HOME variable to set a directory")
        })
}

/// Returns the base data directory, respecting XDG_DATA_HOME on all platforms.
fn sys_data_base() -> PathBuf {
    std::env::var("YOUTUBETUI_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .expect("no data dir, set the YOUTUBETUI_DATA_HOME variable to set a directory")
        })
}

/// Returns the base cache directory, respecting XDG_CACHE_HOME on all platforms.
fn sys_cache_base() -> PathBuf {
    std::env::var("YOUTUBETUI_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::cache_dir()
                .expect("no cache dir, set the YOUTUBETUI_CACHE_HOME variable to set a directory")
        })
}

/// Returns the base cache directory, respecting XDG_CACHE_HOME on all platforms.
fn sys_download_base() -> PathBuf {
    std::env::var("YOUTUBETUI_DOWNLOAD_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::cache_dir().expect(
                "no cache dir, set the YOUTUBETUI_DOWNLOAD_HOME variable to set a directory",
            )
        })
}

/// Returns the configuration directory for youtube-tui.
///
/// Respects `XDG_CONFIG_HOME` on all platforms. Platform defaults:
/// - Linux:   `~/.config/youtube-tui/`
/// - Windows: `%APPDATA%\youtube-tui\`   (or `~/.config/youtube-tui/` if XDG_CONFIG_HOME is set)
/// - macOS:   `~/Library/Application Support/youtube-tui/`
pub fn config_dir() -> PathBuf {
    sys_config_base().join("youtube-tui")
}

/// Returns the data directory for youtube-tui.
///
/// Respects `XDG_DATA_HOME` on all platforms. Platform defaults:
/// - Linux:   `~/.local/share/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\`
/// - macOS:   `~/Library/Application Support/youtube-tui/`
pub fn data_dir() -> PathBuf {
    sys_data_base().join("youtube-tui")
}

/// Returns the cache directory for youtube-tui.
///
/// Respects `XDG_CACHE_HOME` on all platforms. Platform defaults:
/// - Linux:   `~/.cache/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\cache\`
/// - macOS:   `~/Library/Caches/youtube-tui/`
pub fn cache_dir() -> PathBuf {
    sys_cache_base().join("youtube-tui")
}

/// Returns the storage directory for rustypipe.
///
/// Respects `XDG_DATA_HOME` on all platforms.
pub fn rustypipe_dir() -> PathBuf {
    sys_data_base().join("rustypipe")
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
pub fn default_download_path() -> String {
    let downloads = sys_download_base();
    let mut s = downloads.to_string_lossy().to_string();
    if !s.ends_with(std::path::MAIN_SEPARATOR) {
        s.push(std::path::MAIN_SEPARATOR);
    }
    format!("{s}%(title)s-%(id)s.%(ext)s")
}
