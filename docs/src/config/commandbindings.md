# Command bindings

Commands can be binded to keys (just like normal keybindings), it allows functionalities such as <u>playing hovered videos</u>. Bindings can be either *global* or *page specific*, it is located in `~/.config/youtube-tui/commandbindings.yml`.

## Example commandbindings config

```yml
global:
  'f':
    2: run ${browser} '${url}'
  'c':
    2: cp ${url}
video: {}
search:
  'a':
    2: run ${terminal-emulator} mpv '${hover-url}' --no-video
  'A':
    1: run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle
  'p':
    2: run mpv '${hover-url}'
watchhistory:
  'A':
    1: run ${terminal-emulator} mpv '${hover-url}' --no-video --loop-playlist=inf --shuffle
  'p':
    2: run mpv '${hover-url}'
  'a':
    2: run ${terminal-emulator} mpv '${hover-url}' --no-video
# etc
```

<hr>

Specifications are same as [`comamnds.yml`](commands.md), the exact same envs can be used.

Default bindings can be found in [this section](../commands.md#command-bindings).
