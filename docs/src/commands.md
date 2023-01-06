# Commands

Commands can be entered to the TUI by pressing the `:` key, the same as in Vim. Some commands have shorter *alternatives* that can be used instead.

> Env variables can be used by passing in as `${key}`, such as `:channel ${channel-id}` when in a video or playlist page.

**Loadpage commands** can also be used when launching, for example `youtube-tui loadpage popular` or `youtube-tui popular`.

Below are the avaliable commands:

## Loadpage

`loadpage` can be used to load a specific page.

```vim
loadpage popular (alt: `popular`)
loadpage trending (alt: `trending`)
loadpage watchhistory (alt: `watchhistory`)
loadpage search [search query] (alt: `search [search query]`)
loadpage video [id or url] (alt: `video [id or url]`)
loadpage playlist [id or url] (alt: `playlist [id or url] `)
loadpage channel [id or url] (alt: `channel [id or url] `)
```

## History

`history` is used to manage page history (`Backspace` equivalent).

```vim
history back (alt: `back`)
history clear
```

## Utility

```vim
reload (alt `r`)
reload configs (alt `reload/r config/configs`)
flush
quit (alt `q`, `exit`, `x`)
run [command]
```

> The `flush` command is used to run all tasks in queue immediately, this is usually done automatically.
>
> But for when tasks are stacked up in the *same* event loop and the order of which they are executed matters, this command can be used to force the already stacked up commands to be ran first.

## Text commands

Text commands generates a *text only response* without launching the TUI.

```vim
help
version
```
