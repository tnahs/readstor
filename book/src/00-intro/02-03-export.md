# Export

The following options affect only the [`export`][export] commands.

## `--directory-template <TEMPLATE>`

Set the output directory template.

|         |                                        |
| ------- | -------------------------------------- |
| Context | [`book`][book]                         |
| Default | `{{ book.author }} - {{ book.title }}` |
| Example | `Robert Henri - The Art Spirit`        |

For example, using the default template, the non-rendered ouput structure would
look like the following:

```plaintext
[ouput-directory]
 ├── {{ book.author }} - {{ book.title }}
 │    ├── book.json
 │    └── annotations.json
 │
 ├── {{ book.author }} - {{ book.title }}
 │    └── ...
 └── ...
```

And when rendered, the ouput structure would result in the following:

```plaintext
[ouput-directory]
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

By default, exising files are left as is. With this flag, existing files are
overwritten if they are re-exported.

[book]: ../01-templates/06-01-book.md
[export]: ./01-commands.md#export
