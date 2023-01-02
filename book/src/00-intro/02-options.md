# Options

## Global

### `--output-directory <PATH>`

Set the output directory for all [Commands][commands]. Defaults to `~/.readstor`.

### `--databases-directory <PATH>`

Set a custom databases directory.

This can be useful when running ReadStor on databases backed-up with the
[`backup`][backup] command. The output structure the [`backup`][backup] command
creates is identical to the required databases directory structure.

The databases directory should contain the following structure:

```plaintext
[databases-directory]
 │
 ├─ AEAnnotation
 │  ├─ AEAnnotation*.sqlite
 │  └─ ...
 │
 ├─ BKLibrary
 │  ├─ BKLibrary*.sqlite
 │  └─ ...
 └─ ...
```

### `--force`

Run even if Apple Books is currently running.

### `--quiet`

Silence output messages.

## Filtering

### `--filter <[OP]FIELD:QUERY>`

Filter books/annotation before outputting.

Filtering allows you to specify, to a certain degree, which books and/or
annotations to output. Currently, this is available for the [`export`][export]
and [`render`][render] commands.

For example, this filter would only [`render`][render] annotations where its
respective book's title is _exactly_ `the art spirit` AND their tags _contain_
the `#star` tag.

```shell
readstor render \
    --extract-tags \
    --filter "=title:the art spirit" \
    --filter "tag:#star"
```

This filter would [`export`][export] annotations where its respective book's
author _contains_ the string `krishnamurti` AND their tags _contain either_
`#star` or `#love`.

```shell
readstor export \
    --extract-tags \
    --filter "author:krishnmurti" \
    --filter "?tag:#star #love"
```

> <i class="fa fa-exclamation-circle"></i> Note that filters are
> case-insensitive.

#### Filter Syntax

A filter consists of three parts: an optional [`operator`](#operator), a
[`field`](#field) and a [`query`](#query). The syntax structure is as follows:

```plaintext
[operator][field]:[query]
```

For example, looking at part of the command from above, we can see the three
district parts of a filter:

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

#### Operator

The `operator` token determines how matching will be handled against the
`query`.

|              |                                            |
| ------------ | ------------------------------------------ |
| Name         | `operator`                                 |
| Description  | The match operation to use when filtering. |
| Valid Values | `?`(any) `*` (all) `=` (exact)             |
| Required     | No                                         |
| Default      | `?` (any)                                  |

When a filter is processed, the `query` is split on its spaces to create its
component queries. For example, the input string `the art sprit` turns into
three parts: `the`, `art` and `spirit`, and depending on the `operator` these
three parts are handled differently in order to determine if an annotation is
filtered out or not.

| Operator | Name  | Description                                                |
| -------- | ----- | ---------------------------------------------------------- |
| `?`      | Any   | Matches if _any_ part of the split query is a match.       |
| `*`      | All   | Matches if _all_ parts of the split query are a match.     |
| `=`      | Exact | Matches if the original unsplit query is an _exact_ match. |

> <i class="fa fa-exclamation-circle"></i> Note that when searching for an
> exact match in the `tags` field i.e. `=tags:[query]`, the query remains split
> and the set of tags in the query is compared to those in each annotation.

#### Field

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
| `title`  | Books       | The title of the book.    |
| `author` | Books       | The author of the book.   |
| `tags`   | Annotations | The annotation's `#tags`. |

#### Query

The `query` string determines what will be searched in the specified `field`. A
`query` is a space delineated set of words where each word can potentially be a
separate search term depending on the specified [`operator`](#operator).

|              |                                  |
| ------------ | -------------------------------- |
| Name         | `query`                          |
| Description  | A space delineated query string. |
| Valid Values | Any                              |
| Required     | Yes                              |
| Default      | -                                |

## Template Options

### `--templates-directory <PATH>`

Set a custom templates directory.

> <i class="fa fa-exclamation-circle"></i> See the default
> [templates][templates] for fully working examples.

### `--template-group <GROUP>`

Render specified [Template Groups][template-groups].

> <i class="fa fa-exclamation-circle"></i> Passing nonexistent
> [Template Groups][template-groups] will return an error.

Multiple [Template Groups][template-groups] can be passed using the following
syntax.

```sh
readstor \
    # ...
    --template-group basic
    --template-group using-backlinks
    # ..
```

## Pre-process Options

### `--extract-tags`

Extract `#tags` from [`annotation.notes`][annotation].

All matches are removed from [`annotation.notes`][annotation] and placed into
[`annotation.tags`][annotation]. Additionally, all `#tags` within a book are
compiled and placed them into [`book.tags`][book].

> <i class="fa fa-exclamation-circle"></i> Tags _must_ start with a hash symbol
> `#` followed by a letter `[a-zA-Z]`.

### `--normalize-whitespace`

Normalize whitespace in [`annotation.body`][annotation].

Trims whitespace and replaces all line-breaks with two consecutive line-breaks:
`\n\n`.

### `--ascii-all`

Convert all Unicode characters to ASCII.

All Unicode characters found in [`book.title`][book], [`book.author`][book] and
[`annotation.body`][annotation] are converted to ASCII.

### `--ascii-symbols`

Convert "smart" Unicode symbols to ASCII.

"Smart" Unicode symbols found in [`book.title`][book], [`book.author`][book]
and [`annotation.body`][annotation] are converted to ASCII.

| Character                                  | Unicode | Unicode Number | ASCII |
| ------------------------------------------ | :-----: | :------------: | :---: |
| Left Single Quotation Mark                 |    ‘    |     U+2018     |   '   |
| Right Single Quotation Mark                |    ’    |     U+2019     |   '   |
| Left Double Quotation Mark                 |    “    |     U+201C     |   "   |
| Right Double Quotation Mark                |    ”    |     U+201D     |   "   |
| Right-Pointing Double Angle Quotation Mark |    »    |     U+00BB     |  <<   |
| Left-Pointing Double Angle Quotation Mark  |    «    |     U+00AB     |  >>   |
| Horizontal Ellipsis                        |    …    |     U+2026     |  ...  |
| En Dash                                    |    –    |     U+2013     |  --   |
| Em Dash                                    |    —    |     U+2014     |  ---  |

Characters and transliterations taken from:

- [Daring Fireball - SmartyPants][daring-fireball]
- [Python-Markdown - SmartyPants][python-markdown]

## Post-process Options

### `--trim-blocks`

Trim any blocks left after rendering.

> <i class="fa fa-exclamation-circle"></i> Currently, this is a _very_ naive
> implementation that mimics what [`tera`][tera] might do if/when it adds
> [`trim_blocks`][github-tera]. It is by no means smart and will just normalize
> whitespace regardless of what the template requested.

### `--wrap-text`

Wrap text to a maximum character width.

Maximum line length is not guaranteed as long words are not broken if their
length exceeds the maximum. Hyphenation is not used, however, existing hyphen
can be split on to insert a line-break.

> <i class="fa fa-exclamation-circle"></i> This will naively wrap all the
> text inside a rendered file regardless its structure. Use with caution!
> Extremely low values may cause unexpected results. Values above `80` or so
> are recommended.

[annotation]: ../01-templates/06-02-annotation.md
[backup]: ./01-commands.md#backup
[book]: ../01-templates/06-01-book.md
[commands]: ./01-commands.md
[daring-fireball]: https://daringfireball.net/projects/smartypants/
[export]: ./01-commands.md#export
[github-tera]: https://github.com/Keats/tera/issues/637
[python-markdown]: https://python-markdown.github.io/extensions/smarty/
[render]: ./01-commands.md#render
[template-groups]: ../01-templates/02-01-template-groups.md
[templates]: https://github.com/tnahs/readstor/tree/main/templates
[tera]: https://docs.rs/tera/latest/tera/
