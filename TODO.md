# TODO

## v0.3.0

- [x] Removed `render -t/--template`.
- [x] `-t/--templates` is now global and accepts a path to a directory with templates.
- [x] Added `--quiet` to silence output.
- [x] Removed `-v` logging verbosity.
- [x] Switched to `Config` trait for more flexibility.
- [x] Switched from `loggerv` to `env_logger`.
- [x] Removed nested directory from output file structure i.e. `data`,
      `renders`, `backups`.
- [x] Databases backup directories now have a `-` between the date and version:
      `[YYYY-MM-DD-HHMMSS]-[VERSION]`
- [x] Added the option to use a custom `databases` path.
- [x] Overhauled templates workflow.
- [x] Templates can now be optionally placed in their `group` folder. This was
      previously hard-coded.
- [x] A template's configuration is now set within the header of the file inside
      an HTML-comment. As a result the filename of a template no longer matters.
      The only exception to filenames is when naming a template partial, these
      must begin with an underscore.
- [x] Template partials, inheritance and `{% include %}` statements are now
      fully supported.
- [x] Compile to Apple Silicon and Intel.
- [ ] Clear extra spaces when rendering template.
- [ ] Option to render only a single template from `templates` dir.
- [ ] Complete `mdBook`.
- [ ] Update `README.md`'s to reflect all changes.
- [ ] Use a shorter readme for crates.io.
- [ ] Revisit `exclude` in `Cargo.toml`.
- [ ] Rework backup and restore scripts to use `rsync`.
- [ ] Update crates.

## Internal Improvements

- [x] Change line width to default rustfmt and update docstrings/comments.
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
