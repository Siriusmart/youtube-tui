fn main() {
    // On Windows, help the linker find mpv.lib when building with the mpv feature.
    //
    // Priority:
    //   1. MPV_LIB_DIR env var                — CI / custom installs
    //   2. %LOCALAPPDATA%\youtube-tui\mpv-dev — persistent user location (survives cargo install)
    //   3. <manifest>/mpv-dev                 — local project checkout
    //   4. Run setup-mpv-dev.ps1              — download and generate mpv.lib on first build
    //
    // After locating mpv.lib, libmpv-2.dll is copied to both:
    //   - %CARGO_HOME%\bin\  (where cargo install places the exe)
    //   - target\release\    (where cargo build places the exe)
    #[cfg(all(target_os = "windows", feature = "mpv"))]
    {
        println!("cargo:rerun-if-env-changed=MPV_LIB_DIR");
        println!("cargo:rerun-if-env-changed=LOCALAPPDATA");
        println!("cargo:rerun-if-env-changed=CARGO_HOME");

        // 1. Explicit env var override (CI / custom installs)
        if let Ok(dir) = std::env::var("MPV_LIB_DIR") {
            let dir = std::path::PathBuf::from(dir);
            println!("cargo:rustc-link-search=native={}", dir.display());
            copy_dll(&dir);
            return;
        }

        // Persistent user-level location — always writable, survives across `cargo install` runs.
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
                copy_dll(candidate);
                return;
            }
        }

        // 4. Neither location has mpv.lib — run setup-mpv-dev.ps1 to download it.
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
        copy_dll(&output_dir);
    }
}

/// Copy libmpv-2.dll next to the final binary so it can be found at runtime.
///
/// Two destinations are tried:
///   1. %CARGO_HOME%\bin\  — where `cargo install` places the exe
///   2. The `target/<profile>/` directory — where `cargo build` places the exe,
///      derived by walking up from OUT_DIR
#[cfg(all(target_os = "windows", feature = "mpv"))]
fn copy_dll(mpv_dir: &std::path::Path) {
    let dll = mpv_dir.join("libmpv-2.dll");
    if !dll.exists() {
        // DLL not present in the mpv-dev dir — nothing to copy.
        return;
    }

    // 1. cargo install destination: %CARGO_HOME%\bin\
    if let Ok(cargo_home) = std::env::var("CARGO_HOME") {
        let dest = std::path::Path::new(&cargo_home)
            .join("bin")
            .join("libmpv-2.dll");
        let _ = std::fs::copy(&dll, &dest);
    }

    // 2. cargo build destination: OUT_DIR is something like
    //    <root>/target/<profile>/build/<crate>/out
    //    Walking up 3 levels reaches <root>/target/<profile>/
    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let target_profile = std::path::Path::new(&out_dir)
            .parent() // out
            .and_then(|p| p.parent()) // <crate>
            .and_then(|p| p.parent()) // build
            .and_then(|p| p.parent()); // <profile> (e.g. release / debug)
        if let Some(dir) = target_profile {
            let dest = dir.join("libmpv-2.dll");
            let _ = std::fs::copy(&dll, dest);
        }
    }
}
