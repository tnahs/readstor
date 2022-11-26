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

## Template Options

### `--templates-directory <PATH>`

Set a custom templates directory.

> <i class="fa fa-exclamation-circle"></i> See the default [templates][templates]
> for fully working examples.

### `--template-group <GROUP>`

Render specified [Template Groups][template-groups].

> <i class="fa fa-exclamation-circle"></i> Only exact matches are rendered.

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
[github-tera]: https://github.com/Keats/tera/issues/637
[python-markdown]: https://python-markdown.github.io/extensions/smarty/
[template-groups]: ../01-templates/02-01-template-groups.md
[templates]: https://github.com/tnahs/readstor/tree/main/templates
[tera]: https://docs.rs/tera/latest/tera/
