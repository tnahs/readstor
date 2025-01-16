# Export

The following options affect only the [`export`][export] commands.

## `--directory-template <TEMPLATE>`

Set the output directory template.

|         |                                        |
| ------- | -------------------------------------- |
| Context | [`book`][book]                         |
| Default | `{{ book.author }} - {{ book.title }}` |
| Example | `Robert Henri - The Art Spirit`        |

For example, using the default template, the non-rendered output structure would look like the following:

```plaintext
[output-directory]
 ├── {{ book.author }} - {{ book.title }}
 │    ├── book.json
 │    └── annotations.json
 │
 ├── {{ book.author }} - {{ book.title }}
 │    └── ...
 └── ...
```

And when rendered, the output structure would result in the following:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │   ├── annotations.json
 │   └── book.json
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   ├── annotations.json
 │   └── book.json
 └── Robert Henri - The Art Spirit
     ├── annotations.json
     └── book.json
```

## `--overwrite-existing`

Overwrite existing files.

By default, exising files are skipped.

[book]: ../../templates/context-reference/book.md
[export]: ../commands.md#export
