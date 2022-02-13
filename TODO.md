# TODO

- [ ] Add `--quiet` to silence output.
- [ ] Fix `*_defaults` e.g. `applebooks_defaults` in docs.
- [ ] Override `docs.rs` index page.
- [ ] Add `dump` command to execute `export`, `render` and `backup`.
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
