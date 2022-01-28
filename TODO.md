# TODO

## v0.2.0

- [x] Fix broken default template
- [x] Rename 'assets' to 'resources'
- [x] Rework CLI commands
- [ ] Update README

## Other

- [ ] Document how to implement custom templates
- [ ] Print more info on run e.g. where the export happened
- [ ] Implement `From<&'a Row> for T`
- [ ] Implement `Config` search paths
    - `$HOME/.readstor.toml`
    - `$HOME/.readstor/config.toml`
    - `$HOME/.config/readstor/config.toml`
- [ ] More logging
- [ ] More tests
- [ ] Test "Sync collections, bookmarks, and highlights across devices"
    - [ ] `/Users/[USER]/Library/Mobile Documents/iCloud~com~apple~iBooks/Documents`
- [ ] `check` annotations and 'delete' from source database

```sql
UPDATE ZAEANNOTATION
    SET ZANNOTATIONDELETED = 1
    WHERE ZANNOTATIONUUID='188880E0-AFEB-494B-82E6-20C4506783E3';
```

- [ ] Atomic writes
- [ ] Add highlights from other sources?
- [ ] Move from `chrono` > `time` crate
- [ ] `termcolor` for pretty output
