# Installation

It's about download (or building) the binary and putting them in an easily accessible location (like `/bin/`).

## Install from Crates.io (recommended)

Crates.io is a repository for programs written in Rust, and the YouTube TUI is avaliable <a href="https://crates.io/crates/youtube-tui" target=_blank>*there*</a>.

To install using this method, you will need `rustc` and `cargo` present. <a href="https://www.rust-lang.org/tools/install" target=_blank>*Here*</a>'s a tutorial on how to get them.

Now, run the following command:

```sh
cargo install youtube-tui
```

> To check and update all programs installed from Crates.io, you can use CLI tools like <a href="https://crates.io/crates/cargo-update" target=_blank>*cargo-update*</a>.

## AUR for Arch Linux (recommended)

The YouTube TUI is avaliable in the AUR <a href="https://aur.archlinux.org/packages/youtube-tui-git" target=_blank>*here*</a>.

Use an <a href="https://aur.archlinux.org" target=_blank>AUR</a> helper like <a href="https://aur.archlinux.org/packages/yay" target=_blank>`yay`</a> to install.

```sh
yay -S youtube-tui # replace `yay` with your AUR helper
```

## Download a compiled binary

Download the compiled binary for your system from <a href="https://github.com/Siriusmart/youtube-tui/releases" target=_blank>*GitHub releases*</a> and manually place it in *a folder* (e.g. `~/.cargo/bin/` or `/bin/`)

## Build from source

Use the `cargo` command:

```sh
cargo install --git https://github.com/siriusmart/youtube-tui
```

### Confirm YouTube TUI has been installed

Run the following command in terminal:

```sh
youtube-tui
```

If installed correctly, a TUI should be launched. Press `q` to close the TUI.
