# TODO

## v0.3.0

- [ ] `render -t/--template` has been removed.
- [ ] `-t/--templates` is now global and accepts a path to a directory with templates.
- [ ] Added `--quiet` to silence output.
- [ ] Removed `-v` logging verbosity.
- [ ] Switched to `Config` trait for more flexibility.
- [ ] Switched from `loggerv` to `env_logger`.

## v1.x

- [ ] Add `# Arguments` to docs.
- [ ] Fix `*_defaults` e.g. `applebooks_defaults` in docs.
- [ ] Add `dump` command to execute `export`, `render` and `backup`.
- [ ] Override `docs.rs` index page.
- [ ] Implement `Config` search paths
    - `$HOME/.readstor.toml`
    - `$HOME/.readstor/readstor.toml`
- [ ] Maybe `book.author` should be `book.authors`?
- [ ] Move from `chrono` > `time` crate
- [ ] Implement `From<&'a Row> for T`
- [ ] `termcolor` for pretty output
- [ ] Test "Sync collections, bookmarks, and highlights across devices"
    - `/Users/[USER]/Library/Mobile Documents/iCloud~com~apple~iBooks/Documents`
- [ ] `check` annotations and 'delete' from source database

```sql
UPDATE ZAEANNOTATION
    SET ZANNOTATIONDELETED = 1
    WHERE ZANNOTATIONUUID='188880E0-AFEB-494B-82E6-20C4506783E3';
```

```toml
# `~/.readstor/readstor.toml`

output = "./output"
templates = ["./templates/*", "/path/to/template.md"]
templates = "./templates/*"
backup = true
```
