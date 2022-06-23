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

> ### What's new in this commit
>
> * Code is being cleaned up but still unreadable
> * Haven't touched this project for a week because of exams, so I'm just trying to figure out how stuff works again
> * Just a little save stat before I overhaul the configs and how it's handled

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

## Usage

### To launch

```bash
youtube-tui # or path to the binary
```

### Movement

|Key|What does it do|
|---|---|
|Arrow keys|Move in corresponding direction|
|Right arrow|Move cursor right
|Enter|Select/Launch|
|Q|Quit the program|

> ### Note
>
> These keys only work when nothing is selected. When something is selected, your key presses are passed directly to the "object" you've selected. Press escape (Esc) if you want to deselect.

## Credits

* [ytfzf](https://github.com/pystardust/ytfzf) by [pystardust](https://github.com/pystardust) - The TUI I used to watch YouTube, gave me an idea how this program is going to work
* [Terminal Typeracer](https://gitlab.com/ttyperacer/terminal-typeracer) by [Darrien Glasser](https://gitlab.com/DarrienG) - A very clean looking TUI for typing speed test, gave me idea on how this program should look like
* [Invidious](https://invidious.io) - For having a nice API for doing YouTube searches and stuff. (I made a wrapper for the API you can check it [here](https://crates.io/crates/invidious) out if you want to)


... and of course, credits to myself for not having the ability to read the docs for ytfzf and decided to make my own instead. 
## Bug reports

If there is an issue with this program, please open an issue on GitHub, thank you :D
