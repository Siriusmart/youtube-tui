# Custom commands

Define custom commands. It can be found in `~/.config/youtube-tui/cmdefine.yml`.

> This is still a work in progress, perhaps in the future scripting will be allowed to control and interact with elements within the TUI.

```yml
print: echo
pause: mpv sprop pause yes ;; echo mpv Player paused
next: mpv playlist-next ;; echo mpv Skipped
resume: mpv sprop pause no ;; echo mpv Player resumed
search: loadpage search
rc: reload configs
```

---

General format:

```yml
new command: original command
```

This allows for alternative shorthand commands.

<sub>More about commands run `youtube-tui help` or check out [commands](commands.md).</sub>

> Try not to use self referencing commands, it'll make a mess.
