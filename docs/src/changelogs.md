# Changelogs

### v0.9.3 (Latest)

#### Fixed
- Album playlists doesn't load
- Re-implemented invidious support alongside rustypipe.

### v0.9.2 (Latest)

#### Fixed
- [#111](https://github.com/Siriusmart/youtube-tui/issues/111) don't know how did that line got there.

### v0.9.1
 
#### Added
- Mouse scroll behavior (configurable) for SearchBar and command capture.

#### Fixed
- The `:video [url]` command unable to identify video ID.
- Clicking on area below text list does what you expect it to.

---

### v0.9.0

#### Added

- Support for RustyPipe in replacement of the Invidious API.
- Separated out the code for fetching video info from the rest of the mess, making it easier to add new backends.

#### Fixed

- All Invidious issues, because no longer depends on it.

---

### v0.8.3

#### Added

- Added channel commands menu.

#### Fixed

- Patched invidious-rs to fix [#80](https://github.com/Siriusmart/youtube-tui/issues/80).

### v0.8.2

#### Added

- Added `hover-*` variables for item list.

### v0.8.0

#### Added

- Built in MPV player, controlled by command `mpv [libmpv command] [args...]`
- Command/search histories.
- Keys remapping, custom commands.

#### Fixed

- Some keys captured in search bar and are not working [#37](https://github.com/Siriusmart/youtube-tui/issues/37).

---

### v0.7.4

#### Added

- `Ctrl + w` and `Ctrl + u` in search bar and command mode.
- Keybindings works even when an item is selected (`legacy_input_handling: false` in `main.yml`).
- Creating fake key input using `:key`

#### Fixed

- Search filters popup closing when you click on it.
- Configured keybindings in all items.
- Fixed multiple issues in [#35](https://github.com/Siriusmart/youtube-tui/issues/35)

### v0.7.3

#### Added

- Lazy feed syncing (configurable), so you are not actively ddosing the Invidious instance as badly.

#### Fixed

- Search bar no longer shows escaped search query: the original text string is displayed instead.
- Fixed something about labeling channels with \* although no new video is published.
- Blank screen on loadpage if itemlist is empty.

### v0.7.2

#### Added

- Feeds page, and the subscriptions system.
- Reduced compile time needed by a lot.

#### Fixed

- Search url not escaped, so searches with symbols or spaces are now working.

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

---

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

---

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

---

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
