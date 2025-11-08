# Commands

Commands can be entered to the TUI by pressing the `:` key, the same as in Vim.

Run the help command to view more detailed help.

```sh
youtube-tui help
```

> Env variables can be used by passing in as `${key}`, such as `:channel ${channel-id}` when in a video or playlist page.

**Loadpage commands** can also be used when launching, for example `youtube-tui loadpage popular` or `youtube-tui popular`.

> Commands can be joined together using `;;`.

Below are the avaliable commands:

## Loadpage

`loadpage` can be used to load a specific page.

```vim
loadpage [page]
```

## History

`history` is used to manage page history (`Backspace` equivalent).

```vim
history back
history clear
```

## Utility

```vim
reload // reloads the page
reload configs // reload config files in ~/.config/youtube-tui/
flush
quit
run [command]
parrun [command]
key [keycode] [keymodifier]
echo [mode] (message) # run youtube-tui help to learn more about modes
rmcache [id]
```

> The `flush` command is used to run all tasks in queue immediately, this is usually done automatically.
>
> But for when tasks are stacked up in the *same* event loop and the order of which they are executed matters, this command can be used to force the already stacked up commands to be ran first.

> `run` is used for running *blocking commands*, while `parrun` is non-blocking.

> Valid keycodes are the same as in [`keybindings.yml`](./config/keybindings.md) and [`commandbindings.yml`](./config/commandbindings.md). For a full list of keys, check out [`KeyCodeSerde`](https://docs.rs/youtube-tui/latest/youtube_tui/config/serde/enum.KeyCodeSerde.html) in [`/src/config/serde.rs`](https://github.com/Siriusmart/youtube-tui/blob/master/src/config/serde.rs).
>
> More about keymodifiers can be found in the doc page for [`keybindings.yml`](./config/keybindings.md)

## Library

```vim
bookmark [id]                   Bookmark item with ID (item must be already loaded)
unmark [id]                     Remove bookmark item with ID
togglemark [id]                 Toggle bookmark status
sub/sync [id or url]            Add channel to subscription, or sync an existing channel
unsub [id or url]               Remove channel from subscription
syncall                         Sync all subscriptions
```

## MPV commands

<sub>Only with the [`mpv`](installation.md#mpv-default) feature.</sub>

```vim
mpv prop [label]                Gets mpv property
mpv sprop [label] [value]       Set mpv property
mpv tprop [label] [value]       Toggle a yes/no property
mpv [command]                   Runs a libmpv command
```

> Note that properties and commands are **libmpv** commands, *not* mpv commands. Please refer to [mpv reference](https://mpv.io/manual/master/).

## Text commands

Text commands generates a *text only response* without launching the TUI.

```vim
help
version
```

## Command bindings

Commands can be binded to keys just like normal key bindings, bindings can be edited in `commandbindings.yml`. Below are the default bindings:

|Key|Description|
|---|---|
|`Ctrl + F`|Open page in browser|
|`Ctrl + C`|Copy page url|
|`Ctrl + P`|Play hovered video|
|`Ctrl + A`|Play hovered audio|
|`Shift + A`|Play hovered audio on repeat (shuffled if hovering a playlist)|
