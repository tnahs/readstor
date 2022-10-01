# Changelog

## v0.3.0 (UNRELEASED)

- Removed `render -t/--template`.
- `-t/--templates` is now global and accepts a path to a directory with
  templates.
- Added `--quiet` to silence output.
- Removed `-v` logging verbosity.
- Switched to `Config` trait for more flexibility.
- Switched from `loggerv` to `env_logger`.
- Removed nested directory from output file structure i.e. `data`, `renders`,
  `backups`.
- Databases backup directories now have a `-` between the date and version:
  `[YYYY-MM-DD-HHMMSS]-[VERSION]`
- Added the option to use a custom `databases` path.
- Overhauled templates workflow.
- Templates can now be optionally placed in a `group` directory. This was
  previously hard-coded.
- A template's configuration is now set within the header of the file inside an
  HTML-comment. As a result the filename of a template no longer matters. The
  only exception to filenames is when naming a template partial, these must
  begin with an underscore (`_`).
- Template partials, inheritance and `{% include %}` tags are now fully
  supported!
- Added CI, build and publish actions.
- Release will now have binaries for Apple Silicon and Intel.

## v0.2.0

- Verified version support for Apple Books 4.1 on macOS Monterey 12.x.
- Better handling of testing/dev databases.
- Added `clippy::pedantic`.
- A `.gitkeep` file is now added inside each `assets` folder.
- `--backup` now copies only the `AEAnnotation` and `BKLibrary` directories.

## v0.1.2

- Reworked CLI commands.
- Updated license to MIT/Apache-2.0.
- Renamed 'assets' directory to 'resources'.
- Renamed 'items' directory to 'data'.
- Documented how to implement custom templates.
- Moved from `anyhow` to `color_eyre`.
- Fixed [#3](https://github.com/tnahs/readstor/issues/3) Wrong default template
  location.

## v0.1.1

- Fixed minor issues with the `Cargo.toml` file to work better with
  [crates.io](https://crates.io).

## v0.1.0

- This initial release contains the core functionality: (1) save all annotations
  and notes as JSON (2) export them via a custom (or the default) template using
  the Tera syntax or (3) backup the current Apple Books databases.