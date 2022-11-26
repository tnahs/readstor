# Changelog

## v0.4.0 UNRELEASED CHANGES

- Implemented a more robust pre- and post-processor.
- Added the `--text-wrap [WIDTH]` option to enable text-wrapping
- The configuration key `name-templates` is now `names`.
- A template's `names.annotations` value is now a list of dictionaries. This
  dictionary contains a field for the rendered filename as well as fields
  for some of its respective annotation's metadata. The primary reason for
  this change is to allow the user to optionally sort `names.annotations` by
  these metadata fields. See the documentation for
  [Context Reference - Names][names] for more information.
- The short option name for `--trim-blocks` is now `-t`.

## v0.3.0 (2022-10-09)

- Overhauled templates workflow.
  - A template's config is now set within the header of the file inside an HTML-
    comment. As a result, the filename of a template no longer matters. The
    only exception is when naming a template partial, these must begin with an
    underscore (`_`).
  - Nested rendered template outputs are now optional via the `structure` key in
    the template's config and can be customized via the
    `name-templates.directory` key.
  - Template output filenames are now customizable via the `name-templates.book`
    and `name-templates.annotation` keys in the template's config.
  - All [Tera][tera] features are now supported!
  - Added `--trim-blocks` to naively remove extra linebreaks from the final
    rendered template. This is only a temporary solution until [Tera][tera]
    implements this internally.
  - Added `--template-group` option to render only subset of templates found in
    the templates directory.
- Added pre-processing options to `render` and `export`.
  - `--extract-tags` to extract `#tags` from notes.
  - `--normalize-whitespace` to reduce 3+ linebreaks to 2.
  - `--ascii-only` to convert all Unicode characters to ASCII.
  - `--ascii-symbols` to convert only a subset of "smart" Unicode symbols to ASCII.
- Added `--quiet` flag to silence terminal output.
- Added `--databases-directory` option to use a custom databases path.
- Moved `--templates-directory` option under `render` command.
- Renamed `--templates` to `--templates-directory`.
- Renamed `--output` to `--output-directory`.
- Added `Book.tags`, a compiled list of all the tags within a book's annotations.
- Removed logging verbosity option from cli.
- Removed nested directory from output file structure i.e. `data`, `renders`,
  `backups`.
- Databases backup directories now have a `-` between the date and version:
  `[YYYY-MM-DD-HHMMSS]-[VERSION]`
- Releases will now have binaries for Apple Silicon and Intel.
- Added CI, build, docs and publish actions.
- Switched to `Config` trait for more flexibility.
- Switched from `loggerv` to `env_logger`.

## v0.2.0 (2022-01-31)

- Verified version support for Apple Books 4.1 on macOS Monterey 12.x.
- Better handling of testing/dev databases.
- Added `clippy::pedantic`.
- A `.gitkeep` file is now added inside each `assets` folder.
- `--backup` now copies only the `AEAnnotation` and `BKLibrary` directories.

## v0.1.2 (2021-11-04)

- Reworked CLI commands.
- Updated license to MIT/Apache-2.0.
- Renamed 'assets' directory to 'resources'.
- Renamed 'items' directory to 'data'.
- Documented how to implement custom templates.
- Moved from `anyhow` to `color_eyre`.
- Fixed [#3][#3] Wrong default template
  location.

## v0.1.1 (2021-10-30)

- Fixed minor issues with the `Cargo.toml` file to work better with
  [crates.io][crates-io].

## v0.1.0 (2021-10-30)

- This initial release contains the core functionality: (1) save all annotations
  and notes as JSON (2) export them via a custom (or the default) template using
  the Tera syntax or (3) backup the current Apple Books databases.

[#3]: https://github.com/tnahs/readstor/issues/3
[crates-io]: https://crates.io
[names]: https://tnahs.github.io/readstor/latest/01-templates/06-03-names.html
[tera]: https://tera.netlify.app/
