# Pre-process

The following options affect only the [`render`][render] and [`export`][export] commands.

## `--extract-tags`

Extract `#tags` from [`annotation.notes`][annotation].

All matches are removed from [`annotation.notes`][annotation] and placed into
[`annotation.tags`][annotation].

> <i class="fa fa-exclamation-circle"></i> Tags _must_ start with a hash symbol `#` followed by
> a letter `[a-zA-Z]` and then a series of any characters. A tag ends when a space or another `#`
> is encountered.

## `--normalize-whitespace`

Normalize whitespace in [`annotation.body`][annotation].

Trims whitespace and replaces all line-breaks with two consecutive line-breaks: `\n\n`.

## `--ascii-all`

Convert all Unicode characters to ASCII.

All Unicode characters found in [`book.title`][book], [`book.author`][book] and
[`annotation.body`][annotation] are converted to ASCII.

## `--ascii-symbols`

Convert "smart" Unicode symbols to ASCII.

"Smart" Unicode symbols found in [`book.title`][book], [`book.author`][book] and
[`annotation.body`][annotation] are converted to ASCII.

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

[annotation]: /templates/context-reference/annotation.md
[book]: /templates/context-reference/book.md
[daring-fireball]: https://daringfireball.net/projects/smartypants/
[export]: /intro/commands.md#export
[python-markdown]: https://python-markdown.github.io/extensions/smarty/
[render]: /intro/commands.md#render
