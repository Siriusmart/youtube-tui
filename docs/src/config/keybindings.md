# Keybindings config

The keybindings config binds each key to an action, or multiple keys for the same action. It can be found in `~/.config/youtube-tui/keybindings.yml`.

## Example keybindings config

```yaml
'q':
  0: Exit
Down:
  0: MoveDown
'r':
  2: Reload
Enter:
  0: Select
'l':
  0: MoveRight
Up:
  0: MoveUp
'j':
  0: MoveDown
End:
  0: ClearHistory
Right:
  0: MoveRight
Backspace:
  0: Back
'h':
  0: MoveLeft
F5:
  0: Reload
'k':
  0: MoveUp
Esc:
  0: Deselect
Home:
  0: FirstHistory
Left:
  0: MoveLeft
  4: Back
```

## Keys

Keys can be:

- A single character (e.g. `'q'`)
- Named keys (e.g. `Up`, `Down`)
- Function keys (e.g. `F5`)

## Key modifiers

Key modifiers are the modifier keys that are pressed along with the actual key, for instance in `Ctrl + C` would have the modifier `Ctrl` and the key `C`.

Each modifier has its own code, for instance `Shift` would be `1` and `Ctrl` would be `2`. The final modifier will be the <u>sum</u> of all modifier keys. (`Ctrl + Shift` would be a `3`).

### Keys reference

All possible keys can be found <a href="https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html" target=_blank>*here*</a>.

### Key actions

All key actions can be found in [`KeyAction`](https://docs.rs/youtube-tui/latest/youtube_tui/global/structs/enum.KeyAction.html) under [`/src/global/structs/keyaction.rs`](https://github.com/Siriusmart/youtube-tui/blob/master/src/global/structs/keyaction.rs).

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
