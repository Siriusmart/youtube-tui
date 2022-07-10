![](https://img.shields.io/github/languages/top/siriusmart/youtube-tui?label=rust)
![](https://shields.io/github/license/siriusmart/youtube-tui)
[![](https://img.shields.io/crates/d/youtube-tui?label=crates.io%20downloads)](https://crates.io/crates/youtube-tui)
[![](https://img.shields.io/crates/v/youtube-tui?label=crates.io%20version)](https://crates.io/crates/youtube-tui)
![](https://shields.io/github/stars/siriusmart/youtube-tui?style=social)

# YouTube TUI

An aesthetically pleasing pleasing YouTube TUI written in Rust.

## Overview

YouTube TUI is a text user interface that provides a clean UI for browsing YouTube content. It can perform searches and view channels in the terminal, play videos and playlists with external programs like MPV. Rust is used in writing this program for its better performance and ease of maintaining.

This program is being actively developed and there will be new features coming up every commit.

![Screenshot](https://cdn.discordapp.com/attachments/906941311142219816/990684947830419526/Screenshot_20220626_192433.png)

## User manual

[YouTube TUI user manual](https://siriusmart.github.io/youtube-tui)

> ### What's new in this commit
>
> * Refresh pages
> * Fixed cache not being cleared after exit
> * Key modifiers for key bindings
> * Image loading can be disabled in main config


## Installation

### Install from crates.io

```bash
cargo install youtube-tui
```
### Clone from GitHub and build

```bash
git clone https://github.com/sirusmart/youtube-tui && cd youtube-tui && cargo build --release
```

The binary is located at `./target/release/youtube-tui`, go move it to other locations so that you can launch the program with `youtube-tui` command.

> ### Dependencies
>
> This program does not requires any dependencies, but it is suggested these three things on your system that can be launched via command:
>
> 1. A video player (Defaults to `mpv`)
> 2. A terminal emulator (Defaults to `konsole`)
> 3. A YouTube downloader (Defaults to `yt-dlp`, strongly suggest NOT to use `youtube-dl` because it is now very slow)
>
> None of these dependencies are required as you can change them in config (in `commands.yml`)

## Usage

### To launch

```bash
youtube-tui # or path to the binary
```

### Movement

|Key|What does it do|
|---|---|
|Arrow/Vim keys|Move in corresponding direction|
|Enter|Select/Launch|
|Q|Quit the program|

Check the user manual [here](https://siriusmart.github.io/youtube-tui)

> ### Note
>
> These keys only work when nothing is selected. When something is selected, your key presses are passed directly to the "object" you've selected. Press escape (Esc) if you want to deselect.

### Config

All config files are loacted at `~/.config/youtube-tui/`, will write documentations for that later

## Known issues

### Missing hash key: "selected"

When viewing the playlists page in a channel, it gives you `Missing hash key: "selected"`. This is because Invidious was not able to fetch the requested playlists.

You should also see an error when visiting [this URL](https://vid.puffyan.us/api/v1/channels/UCAkuTH35kk3W1EL9vq6dj6A/playlists)

Here's the [opened issue](https://github.com/iv-org/invidious/issues/3154)

## Todo (First piority on top)

* Put hard coded options into config files
* User manual + documentations
* Go directly to a page by URL
* Vim-like commands in status bar
* Command line launch options
* Channel search and channel video sort
* Recommended videos

## Help needed

Guys please I need help I'm kinda bad at coding tbh, these are stuff that I need help with

* Publishing to the AUR
* Printing full resolution images to the terminal with Sixels

## Credits

* [ytfzf](https://github.com/pystardust/ytfzf) by [pystardust](https://github.com/pystardust) - The TUI I used to watch YouTube, gave me an idea how this program is going to work
* [Terminal Typeracer](https://gitlab.com/ttyperacer/terminal-typeracer) by [Darrien Glasser](https://gitlab.com/DarrienG) - A very clean looking TUI for typing speed test, gave me idea on how this program should look like
* [Invidious](https://invidious.io) - For having a nice API for doing YouTube searches and stuff. (I made a wrapper for the API you can check it [here](https://crates.io/crates/invidious) out if you want to)


... and of course, credits to myself for not having the ability to read the docs for ytfzf and decided to make my own instead. 

## Anything Missing?

If there is a bug or you got a nice idea on what can be added to this program, feel free to open a GitHub issue. Thx :D
