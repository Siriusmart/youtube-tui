# Appearance config

Appearance config controls the colours and the "looks and feel" of the TUI, it is located in `~/.config/youtube-tui/appearance.yml`.

## Example appearance config

```yaml
borders: Rounded
colors:
  text: Reset
  text_special: Reset
  text_secondary: Reset
  text_error: LightRed
  outline: Reset
  outline_selected: LightBlue
  outline_hover: LightRed
  outline_secondary: LightYellow
  message_outline: !Rgb # todo: represent colours with hex values or `rgb(r,g,b)`
  - 255
  - 127
  - 0
  message_error_outline: LightRed
  message_success_outline: LightGreen
  item_info:
    tag: Gray
    title: LightBlue
    description: Gray
    author: LightGreen
    viewcount: LightYellow
    length: LightCyan
    published: LightMagenta
    video_count: !Rgb
    - 131
    - 141
    - 255
    sub_count: !Rgb
    - 101
    - 255
    - 186
    likes: !Rgb
    - 200
    - 255
    - 129
    genre: !Rgb
    - 255
    - 121
    - 215
```

<hr>

Below are the description of each of the fields:

### borders

The style of the borders/outline, if outdated view <a href="https://docs.rs/tui/latest/tui/widgets/enum.BorderType.html" target=_blank>*here*</a>.

*Accept: `Plain`/`Rounded`/`Double`/`Thick`*

### Literally everything else

Any colours, here are the 2 main represenations of colours, for more check out this page <a href="https://docs.rs/tui/latest/tui/style/enum.Color.html" target=_blank>*here*</a>.

#### Terminal colour

This can be modified by the themes of your terminal.

#### RGB

RGB colour values, requires TrueColour support in terminal (for instance Windows CMD cannot display RBG colours).

> Hex colour values to be implemented.
