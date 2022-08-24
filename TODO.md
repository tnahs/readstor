# TODO

## v0.3.0

- [x] `render -t/--template` has been removed.
- [x] `-t/--templates` is now global and accepts a path to a directory with templates.
- [x] Added `--quiet` to silence output.
- [x] Removed `-v` logging verbosity.
- [x] Switched to `Config` trait for more flexibility.
- [x] Switched from `loggerv` to `env_logger`.
- [x] Added template types denoted by a prefix:
  - `single.` renders a book and all its annotations to a single file.
  - `multi.` renders a book and all its annotations to separate files.
  - `partial.` renders as only a part of another themplate. Does not render on its own.
- [x] Removed nested directory from output file structure i.e. `data`, `renders`, `backups`.
- [x] Databases backup directories now have a `-` between the date and version
      `[YYYY-MM-DD-HHMMSS]-[VERSION]`
- [x] Added the option to use a custom `databases` path.
- [ ] Rework how templates are managed.
- [ ] Complete CLI help documentation in `cli-help.txt`.
- [ ] Cleanup `README.md`'s directory structure documentation.
- [ ] Rework backup and restore scripts to use `rsync`.
- [ ] Update `creating-a-custom-template.md`.
- [ ] Update `backup-restore-apple-books-library.md`
- [ ] Compile to Apple Silicon and Intel.

## Internal Improvements

- [ ] Change line width to default rustfmt and update docstrings/comments.
- [ ] Add `# Arguments` to public methods.
- [ ] Maybe `book.author` should be `book.authors`?
- [ ] Move from `chrono` > `time` crate.
- [ ] Implement `From<&'a Row> for T`.
- [ ] `termcolor` for pretty output.

## Features

- [ ] Extract annotations from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.
  - Add a guide on how to access/find these files.
- [ ] Implement `Config` search paths.
  - `$HOME/.readstor.toml`
  - `$HOME/.readstor/readstor.toml`

```yaml
# `~/.readstor/readstor.yaml`

output: ./output
templates: ./templates
force: true
quiet: true
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
