# The main config

The main config file is located in `~/.config/youtube-tui/main.yml`.

## Example main.yml

```yaml
mouse_support: true
invidious_instance: https://vid.puffyan.us
max_watch_history: 50
allow_unicode: false
message_bar_default: All good :)
images: Sixels
refresh_after_modifying_search_filters: true
image_index: 4
provider: YouTube
env:
  browser: firefox
  terminal-emulator: konsole -e
  youtube-downloader: yt-dlp
  video-player: mpv
```

<hr>

Below are the description of each of the fields:

### mouse_support

Whether mouse click events are supports, if `false` then mouse will not do anything to the program.

*Accept: `true`/`false`*

### invidious_instance

The Invidious instance you want to use, a full list of Invidious instances can be found here <a href="https://api.invidious.io" target=_blank>*here*</a>.

*Accept: string of a valid url to an Invidious instance*
 
### max_watch_history

The maximum length that the watch history can hold, a value higher will record more items, but will also result in a larger file size in storage.

*Accept: positive integer below 2<sup>*your CPU architecture*</sup> - 1*
 
### allow_unicode

Enable unicode in video and playlist names, doing so may cause unwanted behaviors like video name continuing into the info field to the right.

*Accept: `true`/`false`*
 
### message_bar_default

The default message displayed in the message bar.

*Accept: any string*
 
### images

How to display thumbnails, if `None` is selected video thumbnails will not be downloaded in the first place.

*Accept: `Sixels`/`HalfBlocks`/`None`*

### refresh_after_modifying_search_filters

Whether to refresh the current search page after search filters are modified

*Accept: `true`/`false`*

### image_index

The index in the array of thumbnail qualities you want to download

Typically these are the avaliable qualities:

|Index|Label|Resolution|
|---|---|---|
|0|maxres|1280 x 720|
|1|maxresdefault|1280 x 720|
|2|sddefault|640 x 480|
|3|high|480 x 360|
|4|medium|320 x 180|
|5|default|120 x 90|
|6|start|120 x 90|
|7|middle|120 x 90|
|8|end|120 x 90|

Usually you don't want to use the max resolution as it will create a large gap between the page being loaded and before the thumbnails are started to get displayed

*Accept: integer that is a valid index*

### Provider

This changes the `${url}` and `${embed_url}` of videos, allowing you to watch videos from Invidious if it is restricted on YouTube.

(Don't always use Invidious if YouTube is working, because first of all the load time if gonna be much slower, and secondly you will be DDoSing Invidious)

*Accept: `YouTube`/`Invidious`*

### env

Env are variables that can be used in `commands.yml`, this allows you to change multiple commands by modifying just one env variable. And not to be confused with system/terminal environment variables, these are just *"a thing"* that you can use in the TUI.

*Accept: `string_key: string_value` pairs*
