# Options

## Global

### `--databases-directory`

Setting a custom database directory:

The `databases` directory should contains the following structure as this is the
way Apple Books' `Documents` directory is set up. This is also how backup
directories are created when backing up databases with ReadStor.

```plaintext
[databases]
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

### `--output-directory`

### `--force`

### `--quiet`

## Template Options

### `--templates-directory`

### `--template-group`

### `--trim-blocks`

## Pre-process Options

### `--extract-tags`

### `--normalize-linebreaks`

### `--ascii-all`

### `--ascii-symbols`

<!-- [Daring Fireball - SmartyPants](https://daringfireball.net/projects/smartypants/) -->
<!-- [Python-Markdown - SmartyPants](https://python-markdown.github.io/extensions/smarty/) -->
