# Export

The following options affect only the [`export`][export] commands.

## `--directory-template <TEMPLATE>`

Set the output directory template.

|         |                                        |
| ------- | -------------------------------------- |
| Context | [`book`][book]                         |
| Default | `{{ book.author }} - {{ book.title }}` |
| Example | `Robert Henri - The Art Spirit`        |

## `--overwrite-existing`

Overwrite existing files.

By default, exising files are left as is. With this flag, existing files are
overwritten if they are re-exported.

[book]: ../01-templates/06-01-book.md
[export]: ./01-commands.md#export
