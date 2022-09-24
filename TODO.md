# TODO

## v0.3.0

- [ ] Use less strict method to find template configuration.
- [ ] Move `--templates` option into `render` command.
- [ ] Add option to de-unicode text e.g. convert smart quotes to regular quotes.
- [ ] Add option to extract tags or not.
- [ ] Add option to trim blocks.
- [ ] Add option to render only a single template from `templates` dir.
- [ ] Add `book.tags` so we can have a list of all the tags within a book.
- [ ] Rework backup and restore scripts to use `rsync`.

## Internal Improvements

- [ ] Update crates.
- [ ] Add `# Arguments` to public methods.
- [ ] Maybe `book.author` should be `book.authors`?
- [ ] Move from `chrono` > `time` crate.
- [ ] Implement `From<&'a Row> for T`.
- [ ] `termcolor` for pretty output.

## Features

- [ ] Extract annotations from iOS's `com.apple.ibooks-sync.plist` and
      `Books.plist` files.
  - [ ] Add a guide on how to access/find these files.

## Future

- [ ] Internationalization.
