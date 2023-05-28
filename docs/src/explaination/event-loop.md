# Event loop

The event loop is like the glue that puts everything together.

## Before the event loop

But before the TUI enters it's main event loop, it does several things.

### 1. Check what command is being ran.

If the command does not need to enter the main TUI, don't enter the main TUI.

For example, `help` or `version` just returns a text message on your terminal screen. In [`main.rs`](https://github.com/Siriusmart/youtube-tui/blob/f3b1912e0d99b46fd8c7b0be2a1606a7b087b650/src/main.rs#LL19C1-L19C43) line 19, it is checked whether the entered command is a valid text command. If so, the program exits without ever reaching the TUI code.

### 2. Enable raw mode

If you've tried killing youtube-tui while it's running (`killall youtube-tui`), your terminal should be in a weird state - where everything, including mouse movement seem to be captured. That's the raw mode.

I have no idea what is special about it, but it is in the examples of `tui-rs` [here](https://github.com/fdehau/tui-rs/blob/fafad6c96109610825aad89c4bba5253e01101ed/examples/block.rs#L18) (one of the main dependency of this TUI).

### 3. Init

There are a bunch of things happening in this stage, but it really just boils down to a few basic things:

- Reading/creating config files, subscriptions, libraries and inserting them to the *pool of shared data* (global data) within the [framework](https://docs.rs/tui-additions/0.1.13/tui_additions/framework/struct.Framework.html) struct.

Global data is used here, the reason should be obvious. All pages, should share the same config, there is no need to keep extra copies of the (usually) same thing and waste memory.

- Insert some structs needed for the TUI to run.

These structs includes [`Message`](https://github.com/Siriusmart/youtube-tui/blob/master/src/global/structs/message.rs) storing the message to be displayed in message bar, and [`Status`](https://github.com/Siriusmart/youtube-tui/blob/master/src/global/structs/status.rs), useful for storing all sorts of values, for example helping with the running (mostly rendering) of the TUI.

- Move some files

Some files have to be moved to `~/.cache/youtube-tui/` from (usually) `~/.local/share/youtube-tui/` for stored content to be used by the TUI, this is why running multiple instances of the program can mess things up - as one instance exits the files got moved from `.cache` to `.local`, and the other instance no longer sees the files they thought was still in `.cache`.

> The full init code is [here](https://github.com/Siriusmart/youtube-tui/blob/master/src/init.rs)

---

todo...
