# Layout config

The layout config determins the position of where each item is placed, mainly their order. The config file can be found in `~/.config/youtube-tui/pages.yml`.

## Example layout config

```yaml
main_menu:
  layout:
  - type: NonCenteredRow
    items:
    - SearchBar
    - SearchFilters
  - type: CenteredRow
    items:
    - Popular
    - Trending
    - History
  - type: NonCenteredRow
    items:
    - ItemList
  - type: NonCenteredRow
    items:
    - MessageBar
  message: Loading main menu...
  command: key Esc 0 ;; key Up 0 ;; key Left 0 ;; key Enter 0 # this focuses on the search bar

# and much more ...
```

## Items

Each item is an individual "thing", these things can be optionally selectable, or hoverable by the cursor.

## Rows

A row is a horizontal row of items, it can be either centered (like the buttons) or non centered (which will align to the left).

> Non centered rows are faster and less crash prone compared to centered rows.

Each item are ordered from left to right.

## Message

The message to display when loading the page.

## Command

Command to run when the page loads, you can use the `key` command to select an item on load.

### Items reference

|Item|Can be used in page|Description|
|---|---|---|
|MessageBar|Any|The panel (default in the bottom of every page) that displays message and error messages|
|SearchBar|Any|A text field that searches that entered query|
|SearchFilters|Any|A button that brings up a popup for modifying search filters|
|Trending|Any|Loads the trending page|
|Popular|Any|Loads the popular page|
|History|Any|Loads the watch history page|
|ItemList|Main menu/search|Display multiple videos, channels, or playlists in a list|
|SingleItemInfo|Single item|Display info of one single item (a single video or playlist)|
|ChannelDisplay|Channel display|Display information of a channel, depending on the page|
|ChannelMain|Channel display|Loads the main channel page|
|ChannelVideos|Channel display|Loads the channel videos page|
|ChannelPlaylists|Channel display|Loads the channel playlists page|
