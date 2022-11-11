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

### Sort

How the search results are sorted.

*Accept: `Relevance`/`Rating`/`Date`/`Views`*
