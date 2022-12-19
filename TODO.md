# TODO

## Book

- [ ] Links to example templates refer to `main` and not the current release.
      All links containing `https://github.com/tnahs/readstor/main` must be
      converted to a tag-aware one: `https://github.com/tnahs/readstor/v0.3.0`.
      Is it possible to automate this nicely for every release?
- [ ] Add more documentation for each annotation/book field.

## Next

- [ ] Add an argument to `export` and `backup` to set the directory name template.
- [ ] Should we add pre- and post-processing options to the template's config?
      We could also keep the cli pre- and post-processing options and merge them
      with the ones local to each template.

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

- [ ] Simplify how template names are defined in the `names` key.
- [ ] Rename and slightly refactor `Data` to `Entries`.
- [ ] Add `# Arguments` to public methods.
- [ ] Is there a way to consolidate clippy lints between bin/lib?
- [ ] Test [Tera][tera] macros and inheritances.
- [ ] Checkout [fern][fern] for stdout/stderr and file logging.
- [ ] Config file support.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[tera]: https://tera.netlify.app/
