# TODO

## 0.7.0

- [ ] Re-work stdout messages.
- [ ] Backups from iOS need a `version` in their context.

## Book

- [ ] Update with new CLI structure.
- [ ] Add docs for new filters: `strip` & `slugify`.
- [ ] Links to example templates refer to `main` and not the current release. All links
      containing `https://github.com/tnahs/readstor/main` must be converted to a tag-aware one:
      `https://github.com/tnahs/readstor/v0.3.0`. Is it possible to automate this nicely for every
      release?
- [ ] Add more documentation for each annotation/book field.

## Next

- [ ] Add `--udid` flag to `ios` platform to pass custom device UDID.
- [ ] Simplify template-groups file naming workflow. Add internal awareness of template groups.
- [ ] Update README to focus on template outputs and exporting rather than a general purpose CLI.
- [ ] Check `cargo clippy` GitHub Action.
- [ ] Update how the summaries are printed out. With the ability to skip writing files, the current
      method will display incorrect information.
- [ ] Find more fields in the iOS plist.
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

## Ideas

- [ ] Config file support.
- [ ] Convert `Book::authors` into a list of authors?
- [ ] Should we add pre- and post-processing options to the template's config? We could also keep
      the cli pre- and post-processing options and merge them with the ones local to each template.

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

- [ ] Add more `FilterType`s and `FilterOperator`s.

## Internal

### Improvements

- [ ] Use a trait for `extract_books`/`extract_annotations`.
- [ ] Improve `epubcfi` parser.

### Documentation

- [ ] Document `cli` module.

### Testing

- [ ] Can we add tests inside the `lib::render::template.rs` to verify that the example template
      configs are valid? This should also check the `names` field for any errors in requested values.
- [ ] Add filsystem tests for when skipping/overwriting files using the `export` and `render` commands.
- [ ] Add teardown for testing.
- [ ] Test [Tera][tera] macros and inheritances.

[fern]: https://docs.rs/fern/latest/fern/
[indicatif]: https://docs.rs/indicatif/latest/indicatif/
[minus]: https://docs.rs/minus/latest/minus/
[tera]: https://keats.github.io/tera/
