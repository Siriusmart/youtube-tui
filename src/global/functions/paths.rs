use std::path::PathBuf;

/// Returns the configuration directory for youtube-tui.
///
/// - Linux: `~/.config/youtube-tui/`
/// - Windows: `%APPDATA%\youtube-tui\`
/// - macOS: `~/Library/Application Support/youtube-tui/`
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| home::home_dir().unwrap().join(".config"))
        .join("youtube-tui")
}

/// Returns the data directory for youtube-tui.
///
/// - Linux: `~/.local/share/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\`
/// - macOS: `~/Library/Application Support/youtube-tui/`
pub fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| home::home_dir().unwrap().join(".local").join("share"))
        .join("youtube-tui")
}

/// Returns the cache directory for youtube-tui.
///
/// - Linux: `~/.cache/youtube-tui/`
/// - Windows: `%LOCALAPPDATA%\youtube-tui\cache\`
/// - macOS: `~/Library/Caches/youtube-tui/`
pub fn cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| home::home_dir().unwrap().join(".cache"))
        .join("youtube-tui")
}

/// Returns the storage directory for rustypipe.
///
/// - Linux: `~/.local/share/rustypipe/`
/// - Windows: `%LOCALAPPDATA%\rustypipe\`
/// - macOS: `~/Library/Application Support/rustypipe/`
pub fn rustypipe_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| home::home_dir().unwrap().join(".local").join("share"))
        .join("rustypipe")
}

/// Returns the default save path string for the `save-path` env variable.
/// This is used in command strings, so it returns a String with trailing separator.
pub fn default_save_path() -> String {
    let p = data_dir().join("saved");
    let mut s = p.to_string_lossy().to_string();
    // Ensure trailing path separator for command substitution
    if !s.ends_with(std::path::MAIN_SEPARATOR) {
        s.push(std::path::MAIN_SEPARATOR);
    }
    s
}

/// Returns the default download path string for the `download-path` env variable.
pub fn default_download_path() -> String {
    let downloads = dirs::download_dir()
        .unwrap_or_else(|| home::home_dir().unwrap().join("Downloads"));
    let mut s = downloads.to_string_lossy().to_string();
    if !s.ends_with(std::path::MAIN_SEPARATOR) {
        s.push(std::path::MAIN_SEPARATOR);
    }
    format!("{s}%(title)s-%(id)s.%(ext)s")
}
