# Default search options

The default search query/filters when launching the TUI, this can be found in `~/.config/youtube-tui/search.yml`, but should not be "customised" like how you would to a config file because it's just a configurable default value.

## Example search default value

```yaml
query: ''
filters:
  sort: Relevance
  date: None
  duration: None
  type: All
```

<hr>

Below are the description of each field

### query

The default query when you launch the TUI.

*Accept: any string*

### sort

How the search results are sorted.

*Accept: `Relevance`/`Rating`/`Date`/`Views`*

### date

Restrict videos publish date.

*Accept: `None`, `Hour`, `Day`, `Week`, `Month`, `Year`*

### duration

Restrict videos duration.

*Accept: `None`, `Short`, `Medium`, `Long`*

### type

Restrict video types

*Accept: `All`, `Video`, `Channel`, `Playlist`*

> All these options can also be changed in runtime using [search filters](basic_usage.md#searching)
