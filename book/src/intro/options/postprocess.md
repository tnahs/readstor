# Post-process

The following options affect only the [`render`][render] command.

## `--trim-blocks`

Trim any blocks left after rendering.

> <i class="fa fa-exclamation-circle"></i> Currently, this is a _very_ naive implementation that
> mimics what [`tera`][tera] might do if/when it adds [`trim_blocks`][github-tera]. It is by no
> means smart and will just normalize whitespace regardless of what the template requested.

## `--wrap-text <WIDTH>`

Wrap text to a maximum character width.

Maximum line length is not guaranteed as long words are not broken if their length exceeds the
maximum. Hyphenation is not used, however, existing hyphen can be split on to insert a line-break.

> <i class="fa fa-exclamation-circle"></i> This will naively wrap all the text inside a rendered
> file regardless its structure. Use with caution! Extremely low values may cause unexpected
> results. Values above `80` or so are recommended.

[render]: /intro/commands.md#render
[github-tera]: https://github.com/Keats/tera/issues/637
[tera]: https://docs.rs/tera/latest/tera/
