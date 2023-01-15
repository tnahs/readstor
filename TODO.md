# TODO

## Book

- [ ] Links to example templates refer to `main` and not the current release.
      All links containing `https://github.com/tnahs/readstor/main` must be
      converted to a tag-aware one: `https://github.com/tnahs/readstor/v0.3.0`.
      Is it possible to automate this nicely for every release?
- [ ] Add more documentation for each annotation/book field.

## Next

- [ ] Add a quick-start/cheatsheet to `README.md` and book.
- [ ] Add `--overwrite` flag to force overwriting existing files:

  - [x] Add to the `export` command
  - [ ] Add tests for the `export` command
  - [x] Add to the `render` command
  - [ ] Add tests for the `render` command
  - [ ] Update how the summary is printed out. With the ability to skip writing
        files, the current method will display incorrect information.

- [ ] Improve stdout messages with [`indicatif`][indicatif]

  ```plaintext
  ◆ Rendering Templates:
    • initializing data...
    • running pre-processors...
    • initializing templates...
    • rendering templates...
    • running post-processors...
    • writing templates...
    • rendered 1 template into 99 files to /path/to/output/directory
  ```

- [ ] Add `display` command to display annotations in the terminal with
      [`minus`][minus].
- [ ] Simplify template-groups file naming workflow. Add internal awareness of
      template groups.

## Ideas

- [ ] Convert `Book::authors` into a list of authors?
- [ ] Should we add pre- and post-processing options to the template's config?
      We could also keep the cli pre- and post-processing options and merge
      them with the ones local to each template.

  ```yaml
  group: extended-config
  # ...
  pre-process:
    extract-tags: true
    normalize-whitespace: true
    convert-to-ascii: all # or 'symbols'
  post-process:
    trim-blocks: true
    wrap-text: 80
  ```

## Internal

- [ ] Add teardown for testing.
- [ ] Document `cli` module.
- [ ] Is there a way to consolidate clippy lints between bin/lib?
- [ ] Test [Tera][tera] macros and inheritances.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist`.
- [ ] Config file support.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[indicatif]: https://docs.rs/indicatif/latest/indicatif/
[minus]: https://docs.rs/minus/latest/minus/
[tera]: https://tera.netlify.app/
