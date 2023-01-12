# TODO

## Book

- [ ] Links to example templates refer to `main` and not the current release.
      All links containing `https://github.com/tnahs/readstor/main` must be
      converted to a tag-aware one: `https://github.com/tnahs/readstor/v0.3.0`.
      Is it possible to automate this nicely for every release?
- [ ] Add more documentation for each annotation/book field.

## Next

- [ ] Add a quick-start/cheatsheet to `README.md` and book.
- [ ] Add `--overwrite` flag to force overwriting existing files.
- [ ] Add an argument to `export` and `backup` to set the directory name template.

  ```shell
  readstor export \
      --directory-template "{{ book.slugs.author }}--{{ book.slugs.title }}"
  ```

  ```shell
  readstor backup \
      --directory-template "{{ now() | date(format='%Y-%m-%d-%H%M%S') }}-{{ version }}"
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

- [ ] Convert `Book::authors` into a list of authors?
- [ ] Simplify how template names are defined in the `names` key.

## Internal

- [ ] Add teardown for testing.
- [ ] Document `cli` module.
- [ ] Is there a way to consolidate clippy lints between bin/lib?
- [ ] Test [Tera][tera] macros and inheritances.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.
- [ ] Config file support.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[indicatif]: https://docs.rs/indicatif/latest/indicatif/
[tera]: https://tera.netlify.app/
