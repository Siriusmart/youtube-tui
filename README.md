## Warning:
* This app is incomplete and is missing many features
* This app is only for *\*nix* systems

## Changes made in this commit
* Added search filters

## Installation
Requirements: Git, Cargo and the Rust Compiler

```bash
cargo install youtube-tui
```

## Features
* View the Trending and Popular tab of YouTube
* View your Watch history
* Download videos and audio
* Watch videos and audio

## Movement
* Use the arrow keys to move around the screen
* Press enter to select an item
* When item is selected all your keys will be passed into it
* Press escape to exit the item
* Press q to quit the program

## Layout
* Each divided area is one item
* Red shows the item your currently on
* Blue shows the selected item
* Yellow shows an item in a special state

## Storage
* Cache is stored in `~/.cache/youtube-tui`, it is cleared when you quit the program
* Storage is stored in `~/.local/share/youtube-tui`
* Config files are stored in `~/.config/youtube-tui`

## Config files
The config files are located in `~/.config/youtube-tui`, run the program once to create the config files.

> ### `main.yml`
> ```yaml
> # The program's main config file
> ---
> yt_dl: # Donwload location for your downloaded videos
>   video_path: ~/Downloads/%(title)s.%(ext)s
>   audio_path: ~/Downloads/%(title)s.%(ext)s
> max_watch_history: 50 # Maximum length of watch history
> ```

> ### `commands.yml`
> ```yaml
> # Command arguments to use when launching applications
> ---
> video_player:
>   command: mpv
>   open_in_console: false
>   args:
>     - "--no-terminal"
>     - "{url}"
> audio_player:
>   command: mpv
>   open_in_console: true
>   args:
>     - "--no-video"
>     - "{url}"
> image_viewer:
>   command: mpv
>   open_in_console: false
>   args:
>     - "{url}"
>     - "--no-terminal"
> video_downloader:
>   command: yt-dlp
>   open_in_console: true
>   args:
>     - "{url}"
>     - "-o"
>     - "{video_save_location}"
> audio_downloader:
>   command: yt-dlp
>   open_in_console: true
>   args:
>     - "{url}"
>     - "-o"
>     - "{audio_save_location}"
>     - "-x"
> terminal:
>   command: konsole
>   open_in_console: false
>   args:
>     - "-e"
>     - "{command}"
> 

## To Do
* Search
* Adaptive config files (auto update when config files when there are new options in the config)

## Bugs
* If there is a bug please open a GitHub issue thx :)