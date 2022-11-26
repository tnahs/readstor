# TODO

## Book

- [ ] Links to example templates refer to `main` and not the current release.
      All links containing `https://github.com/tnahs/readstor/main` must be
      converted to a tag-aware one: `https://github.com/tnahs/readstor/v0.3.0`.
      Is it possible to automate this nicely for every release?
- [ ] Add more documentation for each annotation/book field.

## Next

- [ ] Simplify and consolidate how template names are defined.
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
[slugify]: https://tera.netlify.app/docs/#slugify
[tera]: https://tera.netlify.app/
[textwrap]: https://docs.rs/textwrap/latest/textwrap/
