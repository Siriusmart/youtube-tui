# Installation

The YouTube TUI is *not* tested for Windows.

## Install from Crates.io (recommended)

Crates.io is a repository for programs written in Rust, and the YouTube TUI is available <a href="https://crates.io/crates/youtube-tui" target=_blank>*there*</a>.

To install using this method, you will need `rustc` and `cargo` present. <a href="https://www.rust-lang.org/tools/install" target=_blank>*Here*</a>'s a tutorial on how to get them.

Now, run the following command:

```sh
cargo install youtube-tui
```

> To check and update all programs installed from Crates.io, you can use CLI tools like <a href="https://crates.io/crates/cargo-update" target=_blank>*cargo-update*</a>.

## AUR for Arch Linux (recommended)

The YouTube TUI is available in the AUR <a href="https://aur.archlinux.org/packages/youtube-tui-git" target=_blank>*here*</a>.

Use an <a href="https://aur.archlinux.org" target=_blank>AUR</a> helper like <a href="https://aur.archlinux.org/packages/yay" target=_blank>`yay`</a> to install.

```sh
yay -S youtube-tui # lastest crates.io release, recommended
yay -S youtube-tui-git # latest git version, potentially untested
yay -S youtube-tui-full-bin # default binary (out of date)
yay -S youtube-tui-nodefaults-bin # minimal binary (out of date)
```
## Installation on NixOS 
Add the following Nix code to your NixOS Configuration
```nix 
environment.systemPackages = [
  pkgs.youtube-tui
];
```

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

## Features

The TUI has features that can be enabled/disabled when compiling.

```sh
cargo install youtube-tui # install with all default features
cargo install youtube-tui --no-default-features # install without any features enabled
cargo install youtube-tui --no-default-features -F 'halfblock' # install with only HalfBlocks support (but not Sixels)
cargo install youtube-tui --no-default-features -F 'clipboard' -F 'sixel' # can install with multiple features by doing this
cargo install youtube-tui --all-features # install with all features (even if not included in default)
```

### `halfblock` (default)

Display images through HalfBlocks, work best in terminals with TrueColour support.

### `sixel` (default)

Display images with Sixels, allows the display of images at full definition. Not present on windows.

Enabling this will also enable `halfblock`.

Requires <a href="https://github.com/saitoha/libsixel" target=_blank>`libsixel`</a>.

### `clipboard` (default)

Allows clipboard pasting in commands and search bar.

Requires <a href="https://xcb.freedesktop.org/" target=_blank>`libxcb`</a>.
