# Changelogs

## v0.7.2

### Added

- Feeds page, and the subscriptions system.
- Reduced compile time needed by a lot.

### Fixed

- I remember fixing something but forgot what it was D:

### v0.7.1

#### Added

- Options for config loading. (`write_config` in `main.yml`)
- Parrallel/blocking commands `parrun`/`run`

#### Fixed

- Fixed crashing when launching without permissions to write to config files (<a href="https://github.com/Siriusmart/youtube-tui/issues/29" target=_blank>issue 29</a>)

### v0.7.0

#### Added

- File system based caching
- Bookmark library
- Save video/playlists for offline viewing

#### Fixed

- Empty ItemList crash (<a href="https://github.com/Siriusmart/youtube-tui/issues/26" target=_blank>issue 26</a>)

<hr>

## v0.6.2

### Added

- Command bindings

### Fixed

- Image not cleared when going back to "switch view"

### v0.6.1

#### Added

- Rework of the env system, now using environment variables provided by the OS as opposed to implmenting own version of env
- Ability to use envs in command mode
- Launch commands (including `youtube-tui help`)

#### Fixed

- Screen blinking when pressing up arrow on the first item in playlist page, videos view

### v0.6.0

#### Added

- Mouse support
- Vim-like commands
- Pasting with `Ctrl + V` in search bar and command mode

<hr>

## v0.5.3

### Fixed

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

### Added

- Added default select item for each page (can be changed in `layouts.yml`)

### Fixes

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
