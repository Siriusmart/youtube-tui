![](https://img.shields.io/github/languages/top/siriusmart/youtube-tui?label=rust)
![](https://shields.io/github/license/siriusmart/youtube-tui)
[![](https://img.shields.io/crates/d/youtube-tui?label=crates.io%20downloads)](https://crates.io/crates/youtube-tui)
[![](https://img.shields.io/crates/v/youtube-tui?label=crates.io%20version)](https://crates.io/crates/youtube-tui)
![](https://shields.io/github/stars/siriusmart/youtube-tui?style=social)

# YouTube TUI

An aesthetically pleasing YouTube TUI written in Rust.

## Overview
YouTube TUI is a text user interface that provides a clean TUI for browsing YouTube. It can perform YouTube query searches, view and inspect channels, and play the desired content as a regular video or audio-only stream for saving bandwidth.


![Screenshot](https://cdn.discordapp.com/attachments/906941311142219816/990684947830419526/Screenshot_20220626_192433.png)

## User manual

[YouTube TUI user manual](https://siriusmart.github.io/youtube-tui)

Changelog:

* AM
* REWRITING
* THE
* ENTRIE
* THING
* FROM
* SCRATCH
* THEN
* NOTICED
* A
* SINGLE
* TYPO
* IN
* THE
* OLD
* VERSION
* GRRRRR

So I fixed that one single typo and continued with stuff

## Installation

###Install from the AUR

```bash
yay -S youtube-tui-git # Or any other AUR helper
```

### Install from crates.io

```bash
cargo install youtube-tui
```

### Clone from GitHub and build
```bash
git clone https://github.com/sirusmart/youtube-tui && cd youtube-tui && cargo build --release
```
Once build, the compiled binary will be located at `./target/release/youtube-tui`. The program is not in your path yet, so you'll have to manually move to to a place from which you would be able to execute it. A common way to do so is to copy the binary executable to `/usr/local/bin/youtube-tui`. 

### Dependencies
This program does not require any dependencies, but it is strongly recommended that there are at least three things present in your system's path as executable commands:

* A video player (Defaults to `MPV`)
* A terminal emulator (Defaults to `konsole`)
* A YouTube downloader (Defaults to `yt-dlp`, it is recommended that you do not use  `yt-dl`, as the download speeds are ludicrously slow)

> None of these dependencies is required, and you can change the defaults in the config file `commands.yml`

### Movement

|Key|What does it do|
|---|---|
|Arrow/Vim keys|Move in the corresponding direction|
|Enter|Select/Launch|
|Q|Quit the program|

Check the user manual [here](https://siriusmart.github.io/youtube-tui)

> ### Note
>
> These keys only work when nothing is selected. When something is selected, your key presses are passed directly to the "object" you've selected. Press escape (Esc) if you want to deselect.

### Config

All config files are located at `~/.config/youtube-tui/`, will write documentation for that later

## Known issues

### Missing hash key: "selected"

When viewing the playlists page in a channel, it gives you a `Missing hash key: "selected"`. This is because Invidious was not able to fetch the requested playlists.

You should also see an error when visiting [this URL](https://vid.puffyan.us/api/v1/channels/UCAkuTH35kk3W1EL9vq6dj6A/playlists)

Here's the [opened issue](https://github.com/iv-org/invidious/issues/3154)

## Todo (Priority on top)

* Go directly to a page by URL
* Vim-like commands in the status bar
* Save as local playlist/video
* Command line launch options
* Channel search and channel video sort
* Recommended videos

## Help needed

Guys please I need help I'm kinda bad at coding tbh, so these are stuff that I need help with

* Publishing to the AUR
* Printing full resolution images to the terminal with Sixel

## Credits

* [ytfzf](https://github.com/pystardust/ytfzf) by [pystardust](https://github.com/pystardust) - The TUI I used to watch on YouTube, gave me an idea of how this program is going to work
* [Terminal Typeracer](https://gitlab.com/ttyperacer/terminal-typeracer) by [Darrien Glasser](https://gitlab.com/DarrienG) - A very clean-looking TUI for typing speed test, gave me an idea of how this program should look like
* [Invidious](https://invidious.io) - For having a nice API for doing YouTube searches and stuff. (I made a wrapper for the API you can check it [here](https://crates.io/crates/invidious) out if you want to)


... and of course, credits to myself for not having the ability to read the docs for ytfzf and decided to make my own instead. 

## Anything Missing?

If there is a bug or you got a nice idea of what can be added to this program, feel free to open a GitHub issue. Thx :D
