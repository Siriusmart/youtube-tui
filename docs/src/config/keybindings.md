# Keybindings config

The keybindings config binds each key to an action, or multiple keys for the same action. It can be found in `~/.config/youtube-tui/keybindings.yml`.

## Example keybindings config

```yaml
Up:
  0: MoveUp
Down:
  0: MoveDown
Right:
  0: MoveRight
Left:
  0: MoveLeft
  4: Back
Esc:
  0: Deselect
Backspace:
  0: Back
Enter:
  0: Select
```

## Keys

Keys are the name of the keys that are pressed in the keyboard, all keys are avaliable <a href="https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html" target=_blank>*here*</a>.

## Key modifiers

Key modifiers are the modifier keys that are pressed along with the actual key, for instance in `Ctrl + C` would have the modifier `Ctrl` and the key `C`.

Each modifier has its own code, for instance `Shift` would be `1` and `Ctrl` would be `2`. The final modifier will be the <u>sum</u> of all modifier keys. (`Ctrl + Shift` would be a `3`).

### Keys reference

All possible keys can be found <a href="https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html" target=_blank>*here*</a>.

> Enums are represented using the character `!`, for example the `q` key would be `!Char 'q'`
>
> `Shift + Q` however would be `!Char 'Q'` with `0` as modifier code as `Shift` turns `q` into an upper case character.

### Modifiers reference

|Modifier|Code|
|---|---|
|None|`0`|
|`Shift`|`1`|
|`Ctrl`|`2`|
|`Alt`|`4`|
|`Super`/"Windows" key|`8`|
|`Hyper`|`16`|
|`Meta`|`32`|

All key modifiers (if any are added) will be in <a href="https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html" target=_blank>*the code*</a>.
