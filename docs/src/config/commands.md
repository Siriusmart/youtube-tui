# Commands config

The commands config file decides what options (that will run a certain command on select) to be display in the *video* and *playlist* page respectively, it is located in `~/.config/youtube-tui/commands.yml`.

## Example commands config

```yaml
launch_command: loadpage library ;; flush ;; history clear ;; key Esc 0 ;; key Up 0 ;; key Up 0 ;; key Left 0 ;; key Enter 0 # the key commands select the searchbar on launch
video:
- Reload updated video: run rm '~/.cache/youtube-tui/info/${id}.json' ;; video ${id} # remove the cached info first, then reload the page
- Play video: parrun ${video-player} '${embed-url}'
- Play audio: mpv stop ;; resume ;; mpv sprop loop-file no ;; mpv loadfile '${embed-url}' ;; echo mpv Player started
- Play audio (loop): mpv stop ;; resume ;; mpv sprop loop-file inf ;; mpv loadfile '${embed-url}' ;; echo mpv Player started
- View channel: channel ${channel-id}
- Subscribe to channel: sync ${channel-id}
- Open in browser: parrun ${browser} '${url}'
- Toggle bookmark: togglemark ${id}
- Save video to library: bookmark ${id} ;; run rm -rf '${save-path}${id}.*' ;; parrun ${terminal-emulator} ${youtube-downloader} '${embed-url}' -o '${save-path}%(title)s[%(id)s].%(ext)s'
- Save audio to library: bookmark ${id} ;; parrun rm -rf '${save-path}${id}.*' ;; parrun ${terminal-emulator} ${youtube-downloader} '${embed-url}' -x -o '${save-path}%(title)s[%(id)s].%(ext)s'
- 'Mode: ${provider}': switchprovider

# ...
```

<hr>

The few required fields are:

- `video`
- `saved_video`
- `playlist`
- `saved_playlist`
- `channel`

## Env variables

Notice that a lot of the commands contains the `${label}` pattern, this actually replaces the text with the env variables set in `main.yml`, or is added by the current page (video or playlist) on-the-go.

<!-- Replacing all these with known values it might look something like this: -->
<!--  -->
<!-- ```yaml -->
<!-- launch_command: loadpage popular ;; flush ;; history clear -->
<!-- video: -->
<!-- - Play video: mpv 'https://youtube.com/embed/dQw4w9WgXcQ' -->
<!-- - Play audio: konsole -e mpv 'https://youtube.com/embed/dQw4w9WgXcQ' --no-video -->
<!-- - Open in browser: firefox 'https://youtu.be/dQw4w9WgXcQ' -->
<!-- - Download video (webm): konsole -e yt-dlp -o '~/Downloads/%(title)s-%(id)s.%(ext)s' 'https://youtube.com/embed/dQw4w9WgXcQ' -->
<!-- - 'Mode: ${provider}': '%switch-provider%' -->
<!--  -->
<!-- playlist: -->
<!-- - Switch view: '%switch-view%' -->
<!-- - Play all videos: mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' # and much more... -->
<!-- - Play all audio: konsole -e mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' --no-video -->
<!-- - Shuffle play all audio: konsole -e mpv 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' --no-video --shuffle -->
<!-- - Open in browser: firefox 'https://www.youtube.com/playlist?list=PLdgHTasZAjYZlCXN9rTcX9LFOQ-RIrzCs' -->
<!-- - Download all video (webm): konsole -e yt-dlp -o '~/Downloads/%(title)s-%(id)s.%(ext)s' 'https://youtube.com/embed/Z8oiddSsB6I' 'https://youtube.com/embed/yiS0DPekSDQ' 'https://youtube.com/embed/YhM8GYuDFps' -->
<!-- - 'Mode: ${provider}': '%switch-provider%' -->
<!-- ``` -->

<!-- > Global commands can also be used here. (Ones that start with an `:`). -->

<!-- Below are the description of each of the fields: -->
<!--  -->
<!-- ### video -->
<!--  -->
<!-- Commands to be displayed in a video page. -->
<!--  -->
<!-- *Accept: `string_label: string_command` pairs* -->
<!--  -->
<!-- ### playlist -->
<!--  -->
<!-- Commands to be displayed in a playlist page (commands view). -->
<!--  -->
<!-- *Accept: `string_label: string_command` pairs* -->

### Env reference

Does not include custom env set in `main.yml`.

|Name|Page|Value|
|---|---|---|
|`url`|search, popular, trending, video, playlist, channel (main)|String url to the web page|
|`id`|video, playlist, channel (main)|String id of the video, playlist or channel|
|`title`|video, playlist|Title of video or playlist|
|`name`|channel|Name of channel|
|`channel-id`|video, playlist|String id of the channel|
|`embed-url`|video|String url to the embed video (required to play video using mpv from Invidious)|
|`all-videos`|playlist|String url***s*** separated by space to all embed videos in a playlist|
|`hover-url`|trending, popular, search|Url of the currenly hovering item.|
|`hover-id`|trending, popular, search|Url of the currenly hovering item.|
|`hover-title`|trending, popular, search|Title of the currenly hovering item.|
|`hover-channel`|trending, popular, search|Channel name of the currenly hovering item.|
|`hover-channel-id`|trending, popular, search|Channel ID of the currenly hovering item.|
|`hover-channel`|trending, popular, search|Url of the currenly hovering item.|
|`hover-channel-id`|trending, popular, search|Url of the currenly hovering item.|
|`hover-channel-url`|feed|Url of the currenly hovering channel.|
|`hover-channel-id`|feed|ID of the currenly hovering channel.|
|`hover-video-url`|feed|Url of the currenly hovering video.|
|`hover-video-id`|feed|ID of the currenly hovering video.|
|`all-ids`|playlist|IDs of all videos in a playlist, separated with space.|
|`offline-path`|Saved video and playlist only|Direct path to saved file.|
|`mpv-queuelist`|Online playlists only|Valid mpv command to queue all videos in the list.|
|`offline-queuelist`|Saved playlists only|Valid mpv command to queue all saved videos in the list.|
