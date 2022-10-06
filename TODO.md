# TODO

## v0.3.0

- [ ] Sort `links.annotations` by filename.
- [ ] Add `book.tags` so we can have a list of all the tags within a book.
- [ ] Rework backup and restore scripts to use `rsync`.
- [ ] Improve error messages.
- [ ] Add test for template validation: syntax and variables.
- [ ] Add redirection from docs root.

## Internal Improvements

- [ ] Add `PostprocessOptions` for `Processor`.
- [ ] Refactor `TemplateManager::render` and its sibling rendering methods. It's
      currently a bit overloaded.
- [ ] Add `# Arguments` to public methods.
- [ ] After mdbook is complete, update internal docs.
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
