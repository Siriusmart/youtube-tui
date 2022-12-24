# Commands config

The commands config file decides what options (that will run a certain command on select) to be display in the *video* and *playlist* page respectively, it is located in `~/.config/youtube-tui/commands.yml`.

## Example commands config

```yaml
video:
- Play video: ${video-player} ${embed-url}
- Play audio: ${terminal-emulator} ${video-player} ${embed-url} --no-video
- Play audio (loop): ${terminal-emulator} ${video-player} ${embed-url} --no-video --loop-file=inf
- View channel: :channel ${channel_id}
- Open in browser: ${browser} ${url}
- Download video (webm): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${embed-url}
- Download audio (opus): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${embed-url} -x
- 'Mode: ${provider}': '%switch-provider%'
playlist:
- Switch view: '%switch-view%'
- Play all videos: ${video-player} ${all-videos}
- Play all audio: ${terminal-emulator} ${video-player} ${all-videos} --no-video
- Shuffle play all audio: ${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle
- Shuffle play all audio (loop): ${terminal-emulator} ${video-player} ${all-videos} --no-video --shuffle --loop-playlist=inf
- View channel: :channel ${channel_id}
- Open in browser: ${browser} ${url}
- Download all video (webm): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos}
- Download all audio (opus): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos} -x
- 'Mode: ${provider}': '%switch-provider%'
```

## Env variables

Notice that a lot of the commands contains the `${label}` pattern, this actually replaces the text with the env variables set in `main.yml`, or is added by the current page (video or playlist) on-the-go.

Replacing all these with known values it might look something like this:

```yaml
video:
- Play video: mpv 'https://youtube.com/embed/dQw4w9WgXcQ'
- Play audio: konsole -e mpv 'https://youtube.com/embed/dQw4w9WgXcQ' --no-video
- Open in browser: firefox 'https://youtu.be/dQw4w9WgXcQ'
- Download video (webm): konsole -e yt-dlp -o '~/Downloads/%(title)s-%(id)s.%(ext)s' 'https://youtube.com/embed/dQw4w9WgXcQ'
- 'Mode: ${provider}': '%switch-provider%'

playlist:
- Switch view: '%switch-view%'
- Play all videos: mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' # and much more...
- Play all audio: konsole -e mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' --no-video
- Shuffle play all audio: konsole -e mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' --no-video --shuffle
- Open in browser: firefox 'https://www.youtube.com/playlist?list=PLdgHTasZAjYZlCXN9rTcX9LFOQ-RIrzCs'
- Download all video (webm): konsole -e yt-dlp -o '~/Downloads/%(title)s-%(id)s.%(ext)s' 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps'
- 'Mode: ${provider}': '%switch-provider%'
```

> Global commands can also be used here. (Ones that start with an `:`).

## Item commands

(will be replaced by global vim-like commands in the next major version)

Notice that there is another pattern in the commands, it is the `%label%` commands. They are special cases that are consumed by the item and is never actually ran as a system command.

For instance `%switch-provider%` toggles the provider between `YouTube` and `Invidious`.

<hr>

Below are the description of each of the fields:

### video

Commands to be displayed in a video page.

*Accept: `string_label: string_command` pairs*

### playlist

Commands to be displayed in a playlist page (commands view).

*Accept: `string_label: string_command` pairs*

## Env reference

Does not include custom env set in `main.yml`.

|Name|Page|Value|
|---|---|---|
|`url`|video & playlist|String url to the web page|
|`embed-url`|video|String url to the embed video (required to play video using mpv from Invidious)|
|`all-videos`|playlist|String url***s*** separated by space to all embed videos in a playlist|
|`provider`|video & playlist|To display the current provider, is the only env that can be displayed in the label, and can only be used when `%switch-provider%` is the command|

## Item command reference

|Name|Page|Use|
|---|---|---|
|`swtich-provider`|video & playlist|Toggles the provider between `YouTube` and `Invidious`|
