# TODO

## Book

- [ ] Add docs for new filters: `strip`& `slugify`.
- [ ] Links to example templates refer to `main` and not the current release.
      All links containing `https://github.com/tnahs/readstor/main` must be
      converted to a tag-aware one: `https://github.com/tnahs/readstor/v0.3.0`.
      Is it possible to automate this nicely for every release?
- [ ] Add more documentation for each annotation/book field.

## v0.6.0

- [ ] How do we handle `date_last_opened` for a book on iOS?

## Next

- [ ] Check `cargo clippy` GitHub Action.
- [ ] Update README to be more focused on template outputs and exporting rather
      than a general purpose CLI.
- [ ] Update how the summaries are printed out. With the ability to skip
      writing files, the current method will display incorrect information.
- [ ] Add `display` command to display annotations in the terminal with
      [`minus`][minus].
- [ ] Display a more information-rich table when filtering:

  ```plaintext
   Found 11 annotations from 2 books
  ┌───────────────────────┬─────────────────┬──────────────────┐
  │ Title                 │ Author          │ # of Annotations │
  ├───────────────────────┼─────────────────┼──────────────────┤
  │ Think on These Things │ J. Krishnamurti │ 3                │
  │ The Art Spirit        │ Robert Henri    │ 8                │
  └───────────────────────┴─────────────────┴──────────────────┘

  Continue? [y/N]: █
  ```

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

- [ ] Add more `FilterType`s and `FilterOperator`s.

## Internal

- [ ] Document `cli` crate.
- [ ] Add more tests in:
  - `lib/process/processors.rs`
  - `lib/process/mod.rs`
- [ ] Improve `epubcfi` parser.
- [ ] Add teardown for testing.
- [ ] Use 100-120 character line-length for comments.
- [ ] Can we add tests inside the `lib::render::template.rs` to verify that the
      example template configs are valid? This should also check the `names`
      field for any errors in requested values.
- [ ] Add filsystem tests for when skipping/overwriting files using the
      `export` and `render` commands.
- [ ] Document `cli` module.
- [ ] Is there a way to consolidate clippy lints between bin/lib?
- [ ] Test [Tera][tera] macros and inheritances.

## Features

- [ ] Config file support.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[indicatif]: https://docs.rs/indicatif/latest/indicatif/
[minus]: https://docs.rs/minus/latest/minus/
[tera]: https://tera.netlify.app/
