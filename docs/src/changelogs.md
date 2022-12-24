# Changelogs

## v0.6.0

### Added

- Mouse support
- Vim-like commands
- Pasting with `Ctrl + V` in search bar and command mode

<hr>

## v0.5.3

#### Fixed

- Search bar could not handle non English characters (e.g. <a href="https://github.com/Siriusmart/youtube-tui/issues/14" target=_blank>Russian</a>)

### v0.5.2

#### Added

- Improved config files (including hex colour support)
- Page turners (next/prev page in search)

### Fixed

- Where the image and text are displaying in the same place, and the image is displayed over the text

### v0.5.1

#### Added

- Image lazy loading (will not rerender image if already rendered)
- Optional features that can be disabled

### v0.5.0

#### Added

- Entire TUI rewritten from scratch, now using the <a href="https://crates.io/crates/tui-additions" target=_blank>`tui-additions`</a> crate
- Sixels image support
- Improved config files

<hr>

## v0.4.4

#### Added

- Added default select item for each page (can be changed in `layouts.yml`)

#### Fixes

- Video list view in playlists not working
- Channel view scroll causes panic
- Search options gray backgound + image covering popup

### v0.4.3

#### Added

- Refreshing page
- Key modifiers for key bindings
- Image loading can be disabled

#### Fixes

- Fixed cache not being cleared after exit
