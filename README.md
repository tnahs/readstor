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

readstor 0.3.0
A CLI for Apple Books annotations

USAGE:
    readstor [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -o, --output <OUTPUT>          Sets the OUTPUT path [default: ~/.readstor]
    -t, --templates <TEMPLATES>    Sets a custom templates directory
    -f, --force                    Runs even if Apple Books is open
    -i, --quiet                    Silences output messages
    -h, --help                     Print help information
    -V, --version                  Print version information

SUBCOMMANDS:
    export    Exports Apple Books' data to OUTPUT
    render    Renders annotations via templates to OUTPUT
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
[output]
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
 └─ ...
```

### `render` using `single` Render Mode

```plaintext
[output]
 │
 ├─ default ── (omitted if a custom template is used)
 │   ├─ Author - Title.[template-ext]
 │   ├─ Author - Title.[template-ext]
 │   └─ ...
 │
 ├─ [template-name]
 │   ├─ Author - Title.[template-ext]
 │   ├─ Author - Title.[template-ext]
 │   └─ ...
 │ 
 ├─ Author - Title
 │   └─ ...
 └─ ...
```

### `render` using `multi` Render Mode

```plaintext
[output]
 │
 ├─ default ── (omitted if a custom template is used)
 │   │
 │   ├─ Author - Title
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
 │   │   └─ ...
 │   │ 
 │   ├─ Author - Title
 │   │   └─ ...
 │   └─ ...
 │
 ├─ [template-name]
 │   │
 │   ├─ Author - Title
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
 │   │   └─ ...
 │   │ 
 │   ├─ Author - Title
 │   │   └─ ...
 │   └─ ...
 └─ ...
```

### `backup`

```plaintext
[output]
 │
 ├─ 2021-01-01-000000-v3.2-2217 ── [YYYY-MM-DD-HHMMSS-VERSION]
 │   │
 │   ├─ AEAnnotation
 │   │   ├─ AEAnnotation*.sqlite
 │   │   └─ ...
 │   │
 │   └─ BKLibrary
 │       ├─ BKLibrary*.sqlite
 │       └─ ...
 │
 │─ 2021-01-02-000000-v3.2-2217
 │   └─ ...
 └─ ...
```
