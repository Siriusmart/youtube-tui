# Basic usage

Here is a basic how-to guide on the TUI.

## Cursor

The *cursor* can be moved using arrow keys, or Vim keybindings (hjkl).

![](./images/cursor-showcasae.png)

The item with the cursor hovering will have a <u>red outline</u>.

> Everything here refers to the *latest default config*, including the keybindings.

|Function|Key(s)|
|---|---|
|Select|Enter|
|Deselect|Esc|
|Cursor up|Up arrow / `k`|
|Cursor down|Down arrow / `j`|
|Cursor left|Left arrow / `h`|
|Cursor right|Right arrow / `l`|
|Previous page|Backspace / Alt + Left arrow|
|First page history|Home|
|Clear page history|End|
|Paste from clipboard|`Ctrl` + `V`|
|Enter command mode|`:`|
|Quit|`q`|

## Selection

Selecting an item allows you to move the cursor within that item, to select an item, press `Enter`.

When nothing is selected, you can move the cursor between items, to deselect from an item, hit `Esc`.

## Searching

Type the search query when the search bar and press `Enter`, use arrow keys to move the cursor around.

To apply search filters, select the button with 3 dots (`...`) to the right of the search bar, hit enter to start modifying and enter again to save. Pressing `Esc` should reload the current search page to apply the filters.

![](./images/search-filters-showcase.png)

## Playing videos and playlists

> This part assumes that you use `mpv` as your video player, `konsole` as your terminal emulator, `yt-dlp` as your YouTube video downloader, and `firefox` as your browser.
> 
> If that is not the case, you can learn how to change that in the *custom commands* section.

Press `Enter` to select a video or playlist from any lists, then move the cursor to the *bottom item* where you can play, download and open the webpage in browser.

### Playlist views

The playlist page allows two different view modes, the first of which is *commands view* - similar to what the video page offers.

The other one is *videos view*, which allows you to look at each videos in the playlist individually.

## Command mode

Command mode is like that of Vim, it can be started by pressing `:` when nothing is selected.

More about commands in the [next chapter](commands.md).

## Buttons

Buttons have no use on their own, but allows you to navigate between pages.

## Mouse click control

![](./images/mouse-showcase.png)

Mouse click controls has been added, you can now use your mouse to navigate around the TUI. Here are the general rules:

1. Clicking an item *moves the cursor* to that item, clicking again *selects* the item
2. Clicking on buttons, or items on a list has the same effect as pressing `Enter` on them
3. Clicking outside a popup closes the popup

However, there are some downsides to not using your keyboard.

1. Cannot move up or down to items not on screen in a list (you can do that with `Up` or `Down arrow`)
2. Cannot access function keys like `Backspace` or `Q`
