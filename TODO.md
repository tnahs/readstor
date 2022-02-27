# TODO

## v0.3.0

- [x] `render -t/--template` has been removed.
- [x] `-t/--templates` is now global and accepts a path to a directory with templates.
- [x] Added `--quiet` to silence output.
- [x] Removed `-v` logging verbosity.
- [x] Switched to `Config` trait for more flexibility.
- [x] Switched from `loggerv` to `env_logger`.
- [x] Added `render-mode`
    - `single` renders all annotations to a single file.
    - `multi` renders each annotation to a separate file.
- [x] Removed nested directory from output file structure i.e. `data`, `renders`, `backups`.
- [x] Databases backup directories now have a `-` between the date and version `[YYYY-MM-DD-HHMMSS]-[VERSION]`
- [ ] Cleanup `README.md`'s directory structure documentation.
- [ ] Add the option to use a custom `databases` path.
- [ ] Expand `ConfigOptions` into `Config`. The nesting is no longer needed.

## Internal Improvements

- [ ] Rework how templates are managed.
- [ ] Maybe `book.author` should be `book.authors`?
- [ ] Add `# Arguments` to docs.
- [ ] Move from `chrono` > `time` crate
- [ ] Implement `From<&'a Row> for T`
- [ ] `termcolor` for pretty output

## Features

- [ ] Extract annotations from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.
    - Add a guide on how to access/find these files.
- [ ] Implement `Config` search paths
    - `$HOME/.readstor.toml`
    - `$HOME/.readstor/readstor.toml`

```toml
# `~/.readstor/readstor.toml`

output = "./output"
templates = "./templates"
template-mode = "multi"
force = true
quiet = true
```

## CLI 1.x Target

```plaintext
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
    export            Exports Apple Books' data to OUTPUT
        macos
        ios
        user
    render            Renders annotations via a template to OUTPUT
        macos
        ios
        user
    backup            Backs-up Apple Books' databases to OUTPUT
    help              Print this message or the help of the given subcommand(s)
    sync              Adds new annotations/books from AppleBooks to the USER-DATABASE
    add               Adds an annotation/book to the USER-DATABASE
    search <QUERY>    Searches the USER-DATABASE
    random            Returns a random annotation from the USER-DATABASE
    check             Prompts to delete unintentional annotations from the USER-DATABASE
    info              Prints ReadStor info
```
