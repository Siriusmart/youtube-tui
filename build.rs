fn main() {
    // On Windows, help the linker find mpv.lib when building with the mpv feature.
    //
    // Priority:
    //   1. MPV_LIB_DIR env var              — CI / custom installs
    //   2. %LOCALAPPDATA%\youtube-tui\mpv-dev — persistent user location (survives cargo install)
    //   3. <manifest>/mpv-dev               — local project checkout
    //   4. Run setup-mpv-dev.ps1            — download and generate mpv.lib on first build
    #[cfg(all(target_os = "windows", feature = "mpv"))]
    {
        println!("cargo:rerun-if-env-changed=MPV_LIB_DIR");
        println!("cargo:rerun-if-env-changed=LOCALAPPDATA");

        // 1. Explicit env var override (CI / custom installs)
        if let Ok(dir) = std::env::var("MPV_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", dir);
            return;
        }

        // Persistent user-level location — works for both `cargo build` and `cargo install`.
        // %LOCALAPPDATA%\youtube-tui\mpv-dev is always writable and survives across installs.
        let persistent_dir = std::env::var("LOCALAPPDATA")
            .map(|d| {
                std::path::PathBuf::from(d)
                    .join("youtube-tui")
                    .join("mpv-dev")
            })
            .ok();

        // Local project-relative fallback (convenient for development checkouts)
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let local_dir = std::path::Path::new(&manifest_dir).join("mpv-dev");

        // 2 & 3. Check both locations for an existing mpv.lib — fast path
        for candidate in persistent_dir.iter().chain(std::iter::once(&local_dir)) {
            let lib = candidate.join("mpv.lib");
            println!("cargo:rerun-if-changed={}", lib.display());
            if lib.exists() {
                println!("cargo:rustc-link-search=native={}", candidate.display());
                return;
            }
        }

        // 4. Neither location has mpv.lib — run setup-mpv-dev.ps1 to download it.
        //    Output goes to the persistent user location so subsequent installs are instant.
        let script = std::path::Path::new(&manifest_dir).join("setup-mpv-dev.ps1");
        if !script.exists() {
            panic!(
                "\n\nERROR: mpv.lib not found and setup-mpv-dev.ps1 is missing.\n\
                 Run the setup script manually:\n\
                 \n  .\\setup-mpv-dev.ps1\n\n\
                 Or set MPV_LIB_DIR to a directory containing mpv.lib.\n"
            );
        }

        // Prefer the persistent directory; fall back to the local project dir.
        let output_dir = persistent_dir.unwrap_or(local_dir.clone());

        eprintln!(
            "cargo:warning=mpv.lib not found — running setup-mpv-dev.ps1 (~30MB download)..."
        );

        let status = std::process::Command::new("powershell")
            .args([
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                &script.to_string_lossy(),
                "-OutputDir",
                &output_dir.to_string_lossy(),
            ])
            .status()
            .expect("Failed to launch PowerShell to run setup-mpv-dev.ps1");

        if !status.success() {
            panic!(
                "\n\nERROR: setup-mpv-dev.ps1 failed (exit code: {:?}).\n\
                 Run it manually to see the full error:\n\
                 \n  .\\setup-mpv-dev.ps1\n\n\
                 Or set MPV_LIB_DIR to a directory containing mpv.lib.\n",
                status.code()
            );
        }

        let mpv_lib = output_dir.join("mpv.lib");
        if !mpv_lib.exists() {
            panic!(
                "\n\nERROR: setup-mpv-dev.ps1 completed but mpv.lib was not found at:\n  {}\n\
                 Run the script manually to diagnose:\n\
                 \n  .\\setup-mpv-dev.ps1\n",
                mpv_lib.display()
            );
        }

        println!("cargo:rustc-link-search=native={}", output_dir.display());
    }
}
