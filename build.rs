fn main() {
    // On Windows, help the linker find mpv.lib when building with the mpv feature.
    // Check MPV_LIB_DIR env var, then fall back to a local mpv-dev directory.
    #[cfg(all(target_os = "windows", feature = "mpv"))]
    {
        if let Ok(dir) = std::env::var("MPV_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", dir);
        } else {
            // Check for a local mpv-dev directory (created by setup-mpv-dev.ps1)
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let local_mpv = std::path::Path::new(&manifest_dir).join("mpv-dev");
            if local_mpv.exists() {
                println!(
                    "cargo:rustc-link-search=native={}",
                    local_mpv.display()
                );
            }
        }
    }
}
