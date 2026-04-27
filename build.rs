fn main() {
    // On Windows, help the linker find mpv.lib when building with the mpv feature.
    // Priority: MPV_LIB_DIR env var → local mpv-dev/ dir → run setup-mpv-dev.ps1 to create it.
    #[cfg(all(target_os = "windows", feature = "mpv"))]
    {
        // Tell Cargo to re-run this script only when these change, not on every build.
        println!("cargo:rerun-if-changed=mpv-dev/mpv.lib");
        println!("cargo:rerun-if-env-changed=MPV_LIB_DIR");

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let local_mpv = std::path::Path::new(&manifest_dir).join("mpv-dev");
        let mpv_lib = local_mpv.join("mpv.lib");

        // 1. Explicit env var override (CI / custom setups)
        if let Ok(dir) = std::env::var("MPV_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", dir);
            return;
        }

        // 2. mpv.lib already exists locally from a previous run — fast path
        if mpv_lib.exists() {
            println!("cargo:rustc-link-search=native={}", local_mpv.display());
            return;
        }

        // 3. Run setup-mpv-dev.ps1 to download and generate mpv.lib automatically
        let script = std::path::Path::new(&manifest_dir).join("setup-mpv-dev.ps1");
        if !script.exists() {
            panic!(
                "\n\nERROR: mpv.lib not found and setup-mpv-dev.ps1 is missing.\n\
                 Run the setup script manually:\n\
                 \n  .\\setup-mpv-dev.ps1\n\n\
                 Or set MPV_LIB_DIR to a directory containing mpv.lib.\n"
            );
        }

        eprintln!(
            "cargo:warning=mpv.lib not found — running setup-mpv-dev.ps1 to download it (~30MB)..."
        );

        let status = std::process::Command::new("powershell")
            .args([
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                &script.to_string_lossy(),
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

        if !mpv_lib.exists() {
            panic!(
                "\n\nERROR: setup-mpv-dev.ps1 completed but mpv.lib was not found at:\n  {}\n\
                 Run the script manually to diagnose:\n\
                 \n  .\\setup-mpv-dev.ps1\n",
                mpv_lib.display()
            );
        }

        println!("cargo:rustc-link-search=native={}", local_mpv.display());
    }
}
