# Pre-process

The following options affect only the [`render`][render] and [`export`][export]
commands.

## `--extract-tags`

Extract `#tags` from [`annotation.notes`][annotation].

All matches are removed from [`annotation.notes`][annotation] and placed into
[`annotation.tags`][annotation]. Additionally, all `#tags` within a book are
compiled and placed them into [`book.tags`][book].

> <i class="fa fa-exclamation-circle"></i> Tags _must_ start with a hash symbol
> `#` followed by a letter `[a-zA-Z]`.

## `--normalize-whitespace`

Normalize whitespace in [`annotation.body`][annotation].

Trims whitespace and replaces all line-breaks with two consecutive line-breaks:
`\n\n`.

## `--ascii-all`

Convert all Unicode characters to ASCII.

All Unicode characters found in [`book.title`][book], [`book.author`][book] and
[`annotation.body`][annotation] are converted to ASCII.

## `--ascii-symbols`

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

[annotation]: ../01-templates/06-02-annotation.md
[book]: ../01-templates/06-01-book.md
[daring-fireball]: https://daringfireball.net/projects/smartypants/
[export]: ./01-commands.md#export
[python-markdown]: https://python-markdown.github.io/extensions/smarty/
[render]: ./01-commands.md#render
