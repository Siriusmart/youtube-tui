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
  message_outline: '#FF7F00'
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
    video_count: '#838DFF'
    sub_count: '#65FFBA'
    likes: '#C8FF81'
    genre: '#FF75D7'
    page_turner: Gray
```

<hr>

Below are the description of each of the fields:

### borders

The style of the borders/outline, if outdated view <a href="https://docs.rs/tui/latest/tui/widgets/enum.BorderType.html" target=_blank>*here*</a>.

*Accept: `Plain`/`Rounded`/`Double`/`Thick`*

### Literally everything else

Any colours, here are the 2 main represenations of colours, for more check out this page <a href="https://docs.rs/tui/latest/tui/style/enum.Color.html" target=_blank>*here*</a>.

#### Terminal colour

This can be modified by the themes of your terminal (e.g. white, green, etc).

#### Hex

Hex should be a string that starts with the `#` character, and can be from `000000` (black) to `FFFFFF` (white).

> RGB color values has been deprecated
