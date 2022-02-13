<p align="center"><img src="./extra/logo/logo-256.png"></p>
<h1 align="center">ReadStor - A CLI for Apple Books annotations</h1>

ReadStor is a simple CLI for exporting user-generated data from Apple Books. The goal of this project is to facilitate data-migration from Apple Books to any other platform. Currently Apple Books provides no simple way to do this. Exporting is possible but not ideal and often times truncates long annotations.

Version `0.1.x` contained the core functionality: (1) save all annotations and notes as JSON (2) render them via a custom (or the default) template using the [Tera](https://tera.netlify.app/) syntax or (3) backup the current Apple Books databases. See [Output Structure](#output-structure) for more information.

Note that this repository is a heavy work-in-progress and things are bound to change.

## Installation

### Using Homebrew

```console
$ brew tap tnahs/readstor
$ brew install readstor
```

```console
$ readstor --version
```

### Using Cargo

```console
$ cargo install readstor
```

## CLI

```console
$ readstor --help

readstor 0.2.0
A CLI for Apple Books annotations

USAGE:
    readstor [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -o, --output <OUTPUT>    Sets the OUTPUT path [default: ~/.readstor]
    -f, --force              Runs even if Apple Books is open
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    export    Exports Apple Books' data to OUTPUT
    render    Renders annotations via a template to OUTPUT
    backup    Backs-up Apple Books' databases to OUTPUT
    help      Print this message or the help of the given subcommand(s)
```

## Version Support

The following versions have been verified as working.

_Note that using iCloud to "Sync collections, bookmarks, and highlights across devices" is currently unverified and might produce unexpected results._

- macOS Monterey 12.x
    - Apple Books 4.1
    - Apple Books 4.2
- macOS Big Sur 11.x
    - Apple Books 3.2

## Output Structure

### `export`

```plaintext
[output] ── [default: ~/.readstor]
 │
 └─ data
     │
     ├─ Author - Title
     │   │
     │   ├─ data
     │   │   ├─ book.json
     │   │   └─ annotations.json
     │   │
     │   └─ resources
     │       ├─ .gitkeep
     │       ├─ Author - Title.epub   ─┐
     │       ├─ cover.jpeg             ├─ These are not exported.
     │       └─ ...                   ─┘
     │
     ├─ Author - Title
     │   └─ ...
     │
     └─ ...
```

### `render`

```plaintext
[output] ── [default: ~/.readstor]
 │
 └─ renders
     │
     ├─ default ── (omitted if a custom template is used)
     │   ├─ Author - Title.[template-ext]
     │   ├─ Author - Title.txt
     │   └─ ...
     │
     ├─ [template-name]
     │   ├─ Author - Title.[template-ext]
     │   ├─ Author - Title.txt
     │   └─ ...
     │   
     └─ ...
```

### `backup`

```plaintext
[output] ── [default: ~/.readstor]
 │
 └─ backups
     │
     ├─ 2021-01-01-000000 v3.2-2217 ── [YYYY-MM-DD-HHMMSS VERSION]
     │   │
     │   ├─ AEAnnotation
     │   │   ├─ AEAnnotation*.sqlite
     │   │   └─ ...
     │   │
     │   └─ BKLibrary
     │       ├─ BKLibrary*.sqlite
     │       └─ ...
     │
     │─ 2021-01-02-000000 v3.2-2217
     │   └─ ...
     │
     └─ ...
```

## 1.x Target

``` plaintext
USAGE:
    readstor [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -o, --output <OUTPUT>    Sets the OUTPUT path [default: ~/.readstor]
    -f, --force              Runs even if Apple Books is open
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    export            Exports Apple Books' data to OUTPUT
    render            Renders annotations via a template to OUTPUT
    backup            Backs-up Apple Books' databases to OUTPUT
    help              Print this message or the help of the given subcommand(s)
    dump              Runs 'save', 'export' and 'backup'
    save              Saves Apple Books' database data to OUTPUT
    export            Exports annotations/books via templates to OUTPUT
    backup            Backs-up Apple Books' databases to OUTPUT
    sync              Adds new annotations/books from AppleBooks to the USER-DATABASE
    add               Adds an annotation/book to the USER-DATABASE
    search <QUERY>    Searches the USER-DATABASE
    random            Returns a random annotation from the USER-DATABASE
    check             Prompts to delete unintentional annotations from the USER-DATABASE
    info              Prints ReadStor info
```

```toml
# `~/.readstor/config.toml`

output = "./output"
templates = "./templates"
user-database = "./database.sqlite"
backup = true
extract-tags = true
```

## Creating a Custom Template

### Syntax

The templating syntax is based on Jinja2 and Django templates. In a nutshell, values are accessed by placing an attribute between `{{ }}` e.g. `{{ book.title }}`. [Filters](https://tera.netlify.app/docs/#filters) can manipulate the accessed values e.g. `{{ name | capitalize }}`. And [statements](https://tera.netlify.app/docs/#control-structures) placed between `{% %}` e.g. `{% if my_var %} ... {% else %} ... {% endif %}`, can be used for control flow. For more information, see the [Tera](https://tera.netlify.app/docs/#templates) documentation.

### Attributes

Every template has access to two object: the current book as `book` and its annotations as `annotations`.

#### Book

```plaintext
book {
    title
    author
    metadata {
        id
        last_opened
    }
}
```

#### Book Attributes

|          Attribute          |          Description          |    Type    |
| --------------------------- | ----------------------------- | ---------- |
| `book.title`                | title of the book             | `string`   |
| `book.author`               | author of the book            | `string`   |
| `book.metadata.id`          | book's unique identifier      | `string`   |
| `book.metadata.last_opened` | date the book was last opened | `datetime` |

#### Book Example

Here the [`date`](https://tera.netlify.app/docs/#date) filter is used to format a `datetime` object into a human-readable date.

```jinja
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date }}
```

### Annotations

```plaintext
annotations [
    annotation {
        body
        style
        notes
        tags
        metadata {
            id
            book_id
            created
            modified
            location
            epubcfi
        }
    },
    ...
]
```

#### Annotations Attributes

|           Attribute            |               Description                |      Type      |
| ------------------------------ | ---------------------------------------- | -------------- |
| `annotations`                  | book's annotations                       | `[annotation]` |
| `annotation.body`              | annotation's body                        | `[string]`     |
| `annotation.style`             | annotation's style/color e.g. 'yellow'   | `string`       |
| `annotation.notes`             | annotation's notes                       | `string`       |
| `annotation.tags`              | annotation's tags                        | `[string]`     |
| `annotation.metadata.id`       | annotation's unique identifier           | `string`       |
| `annotation.metadata.book_id`  | book's unique identifier                 | `string`       |
| `annotation.metadata.created`  | date the annotation was created          | `datetime`     |
| `annotation.metadata.modified` | date the annotation was modified         | `datetime`     |
| `annotation.metadata.location` | `epubcfi` parsed  into a location string | `string`       |
| `annotation.metadata.epubcfi`  | `epubcfi`                                | `string`       |

#### Annotation Example

Here the `join_paragraph` filter concatenates a list of strings with line-breaks and the [`join`](https://tera.netlify.app/docs/#join) filter does the same but with a specific separator passed to the `sep` keyword. This example also shows how to loop over the `annotations` using the `{% for %} ... {% endfor %}` statement.

```jinja
{% for annotation in annotations %}

{{ annotation.body | join_paragraph }}

notes: {{ annotation.notes }}
tags: {{ annotation.tags | join(sep=" ") }}

{% endfor %}
```
