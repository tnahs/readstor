<p align="center"><img src="./extra/logo/logo-256.png"></p>
<h1 align="center">ReadStor - A CLI for Apple Books annotations</h1>

ReadStor is a simple CLI for exporting user-generated data from Apple Books. The goal of this project is to facilitate data-migration from Apple Books to any other platform. Currently Apple Books provides no simple way to do this. Exporting is possible but not ideal and often times truncates long annotations.

Version `0.1.x` contains the core functionality: (1) export all annotations and notes as JSON (2) render them via a custom (or the default) template using the [Tera](https://tera.netlify.app/) syntax or (3) backup the current Apple Books databases. See [Output Structure](#output-structure) for more information.

Note that this repository is a heavy work-in-progress and things are bound to change. Templating documentation is not yet ready but a peek into this repo (see `StorItem`) and the [Tera docs](https://tera.netlify.app/docs/) should be enough information for the curious.

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

## Version Support

The following versions have been verified as working.

_Note that using iCloud to "Sync collections, bookmarks, and highlights across devices" is currently unverified and might produce unexpected results._

- macOS Monterey 12.x
    - Apple Books 4.1
- macOS Big Sur 11.x
    - Apple Books 3.2

## Output Structure

```plaintext
[output] ── [default: ~/.readstor]
 │
 ├─ data
 │   │
 │   ├─ Author - Title
 │   │   │
 │   │   ├─ data
 │   │   │   ├─ book.json
 │   │   │   └─ annotations.json
 │   │   │
 │   │   └─ resources
 │   │       ├─ .gitkeep
 │   │       ├─ Author - Title.epub   ─┐
 │   │       ├─ cover.jpeg             ├─ These are not exported.
 │   │       └─ ...                   ─┘
 │   │
 │   ├─ Author - Title
 │   │   └─ ...
 │   │
 │   └─ ...
 │
 ├─ renders
 │   │
 │   └─ default ── [template-name]
 │       ├─ Author - Title.[template-ext]
 │       ├─ Author - Title.txt
 │       ├─ Author - Title.txt
 │       └─ ...
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

## CLI Hierarchy / Functionality

### 0.2.x

``` plaintext
readstor 0.2.0
A CLI for Apple Books annotations

USAGE:
    readstor [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -o, --output <PATH>    Sets the [output] path [default: ~/.readstor]
    -f, --force            Runs even if Apple Books is open
    -v                     Sets the logging verbosity
    -h, --help             Print help information
    -V, --version          Print version information

SUBCOMMANDS:
    export    Exports Apple Books' data to [output]
    render    Renders annotations via a template to [output]
    backup    Backs-up Apple Books' databases to [output]
    help      Print this message or the help of the given subcommand(s)
```

### 1.x Target

``` plaintext
readstor
    dump                      Runs 'save', 'export' and 'backup'
        --output [PATH]           Sets a custom [output] path
    save                      Saves Apple Books' database data to [output]
        --output [PATH]           Sets a custom [output] path
        --force                   Runs even if Apple Books is open
    export                    Exports annotations/books via templates to [output]
        --output [PATH]           Sets a custom [output] path
        --templates [PATH]        Sets a custom templates path
        --force                   Runs even if Apple Books is open
    backup                    Backs-up Apple Books' databases to [output]
        --force                   Runs even if Apple Books is open
    sync                      Adds new annotations/books from AppleBooks to the [user-database]
    add                       Adds an annotation/book to the [user-database]
    search [QUERY]            Searches [user-database]
    random                    Returns a random annotation from the [user-database]
        --count                   Sets number of random annotations
    check                     Prompts to delete unintentional annotations from the [user-database]
        --source                  Also deletes annotations from Apple Books' database
    info                      Prints ReadStor info
    -v,                       Sets the logging verbosity
```

```toml
# `~/.readstor/config.toml`

output = "./output"
templates = "./templates"
user-database = "./database.sqlite"
backup = true
extract-tags = true
```
