# Commands config

The commands config file decides what options (that will run a certain command on select) to be display in the *video* and *playlist* page respectively, it is located in `~/.config/youtube-tui/commands.yml`.

## Example commands config

```yaml
launch_command: loadpage popular ;; flush ;; history clear # suggested to set page to watchhistory if you don't want to wait for popular to load
video:
- Play video: ${video-player} ${embed-url}
- Play audio: ${terminal-emulator} ${video-player} ${embed-url} --no-video
- Play audio (loop): ${terminal-emulator} ${video-player} ${embed-url} --no-video --loop-file=inf
- View channel: :channel ${channel-id}
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
- View channel: :channel ${channel-id}
- Open in browser: ${browser} ${url}
- Download all video (webm): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos}
- Download all audio (opus): ${terminal-emulator} ${youtube-downloader} -o ${download-path} ${all-videos} -x
- 'Mode: ${provider}': '%switch-provider%'
```

## Env variables

Notice that a lot of the commands contains the `${label}` pattern, this actually replaces the text with the env variables set in `main.yml`, or is added by the current page (video or playlist) on-the-go.

Replacing all these with known values it might look something like this:

```yaml
launch_command: loadpage popular ;; flush ;; history clear
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
|`url`|search, popular, trending, video, playlist|String url to the web page|
|`id`|video, playlist|String id of the video or playlist|
|`channel-id`|video, playlist|String id of the channel|
|`embed-url`|video|String url to the embed video (required to play video using mpv from Invidious)|
|`all-videos`|playlist|String url***s*** separated by space to all embed videos in a playlist|
|`hover-url`|trending, popular, search|Url of the currenly hovering item.|
|`hover-channel-url`|feed|Url of the currenly hovering channel.|
|`hover-channel-id`|feed|ID of the currenly hovering channel.|
|`hover-video-url`|feed|Url of the currenly hovering video.|
|`hover-video-id`|feed|ID of the currenly hovering video.|
|`all-ids`|playlist|IDs of all videos in a playlist, separated with space.|
