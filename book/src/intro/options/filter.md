# Filter

The following options affect only the [`render`][render] and [`export`][export] commands.

## `--filter <[OP]{FIELD}:{QUERY}>`

Filter books/annotations before outputting.

Filtering allows you to specify, to a certain degree, which books and/or annotations to output.
Currently, this is available for the [`export`][export] and [`render`][render] commands.

For example, this filter would only [`render`][render] annotations where its respective book's title
is _exactly_ `the art spirit` AND their tags _contain_ the `#star` tag.

```bash
readstor render \
    --extract-tags \
    --filter "=title:the art spirit" \
    --filter "tag:#star"
```

This filter would [`export`][export] annotations where its respective book's author _contains_ the
string `krishnamurti` AND their tags _contain either_ `#star` or `#love`.

```bash
readstor export \
    --extract-tags \
    --filter "author:krishnmurti" \
    --filter "?tag:#star #love"
```

> <i class="fa fa-exclamation-circle"></i> Note that filters are case-insensitive.

### Filter Results

After all the filters are run, a confirmation prompt is shown with a brief summary of the filtered
down books/annotations.

```bash
readstor render \
    --extract-tags \
    --filter "=title:the art spirit" \
    --filter "tag:#star"
...
   ----------------------------------------------------------------
   Found 9 annotations from 2 books:
    • Think on These Things by Krishnamurti
    • The Art Spirit by Robert Henri
   ----------------------------------------------------------------
   Continue? [y/N]: █
```

> <i class="fa fa-info-circle"></i> This prompt can be auto-confirmed by passing the
> [`--auto-confirm-filter`](#--auto-confirm-filter) flag.

### Filter Syntax

A filter consists of three parts: an optional [`operator`](#operator), a [`field`](#field) and a
[`query`](#query). The syntax structure is as follows:

```plaintext
[operator]{field}:{query}
```

For example, breaking down the command from above:

```plaintext
readstor render \
    --extract-tags \
    --filter "=title:the art spirit" \
              │└──┬┘ └───────────┬┘
              │   │              │
              │   │              └────────── query: the art spirit
              │   └───────────────────────── field: title
              └────────────────────────── operator: = (exact)
    --filter "tag:#star"
              └┬┘ └──┬┘
               │     │
               │     └────────────────────── query: #star
               └──────────────────────────── field: tag
                                operator (default): ? (any)
```

### Operator

The `operator` token determines how matching will be handled against the `query`.

|              |                                            |
| ------------ | ------------------------------------------ |
| Name         | `operator`                                 |
| Description  | The match operation to use when filtering. |
| Valid Values | `?`(any) `*` (all) `=` (exact)             |
| Required     | No                                         |
| Default      | `?` (any)                                  |

When a filter is processed, the `query` is split on its spaces to create its component queries. For
example, the input string `the art sprit` turns into three parts: `the`, `art` and `spirit`, and
depending on the `operator` these three parts are handled differently in order to determine if an
annotation is filtered out or not.

| Operator | Name  | Description                                                |
| -------- | ----- | ---------------------------------------------------------- |
| `?`      | Any   | Matches if _any_ part of the split query is a match.       |
| `*`      | All   | Matches if _all_ parts of the split query are a match.     |
| `=`      | Exact | Matches if the original unsplit query is an _exact_ match. |

> <i class="fa fa-exclamation-circle"></i> Note that when searching for an exact match in the `tags`
> field i.e. `=tags:[query]`, the query remains split and the set of tags in the query is compared
> to those in each annotation.

### Field

The `field` token determines which field to run the filter on.

|              |                                 |
| ------------ | ------------------------------- |
| Name         | `field`                         |
| Description  | The field to use for filtering. |
| Valid Values | `title` `author` `tags`         |
| Required     | Yes                             |
| Default      | -                               |

Currently, only three fields are supported:

| Name     | Searches    | Description               |
| -------- | ----------- | ------------------------- |
| `title`  | books       | The title of the book.    |
| `author` | books       | The author of the book.   |
| `tags`   | annotations | The annotation's `#tags`. |

### Query

The `query` string determines what will be searched in the specified `field`. A `query` is a space
delineated set of words where each word can potentially be a separate search term depending on the
specified [`operator`](#operator).

|              |                                  |
| ------------ | -------------------------------- |
| Name         | `query`                          |
| Description  | A space delineated query string. |
| Valid Values | Any                              |
| Required     | Yes                              |
| Default      | -                                |

## `--auto-confirm-filter`

Auto-confirm [Filter Results](#filter-results).

[export]: /intro/commands.md#export
[render]: /intro/commands.md#render
