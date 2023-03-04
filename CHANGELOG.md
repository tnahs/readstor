# Changelog

<!--
## UNRELEASED CHANGES
### Features
### Changes
### Bug Fixes
-->

## UNRELEASED CHANGES

### Features

- Extracting data from iOS's Apple Books plists is now supported. Currently, this will require the
  user to retrieve and manually pipe in the directory containing the data. Note that this feature is
  considered experimental as it hasn't been tested as thoroughly as its macOS counterpart. See the
  [documentation][documentation] for more information.

### Changes

- CLI options are now grouped for better readability.
- The short option name for `--force` is now `-F`.
- The short option name for `--overwrite-existing` is now `-O`.
- The short option name for `--template-directory` is now `-t` in the `render`
  command.
- The short option name for `--directory-template` is now `-t` in the `export`
  and `backup` command.
- Directory templates for `export` and `backup` are now validated before execution.

### Bug Fixes

- `for` loops within templates are now properly validated before excecution. For example,
  previously, the following returned an error only after templates had begun rendering:

  ```jinja
  {% for annotation in annotations %}
    {{ annotation.invalid }}
  {% endfor %}
  ```

## v0.5.1 (2023-02-05)

### Bug Fixes

- Filenames are no longer truncated if they contain a period `.`.

## v0.5.0 (2023-01-27)

### Features

- Added the `--filter <[?*=]FIELD:QUERY>` option allowing books and annotations
  to be filtered down for the `export` and `render` commands. See the
  documentation on [Filtering][filtering] for more information.
- Added the `--auto-confirm-filter` option to auto-confirm the filtered
  results prompt.
- All slugified strings in the `book` and `annotation` contexts have been moved
  to under the `slugs` namespace. For example:

  ```plaintext
  book.slugs
  book.slugs.title
  book.slugs.author
  book.slugs.metadata.last_opened
  ```

- The `book.slugs.metadata.last_opened` value has been added to the `book`
  context.
- A custom directory template can now be passed to the `export` and `backup`
  commands. See the documentation for the [`export`][export] and
  [`backup`][backup] commands for more information. For example:

  ```shell
  readstor export --directory-template "{{ book.author }} - {{ book.title }}"
  ```

- The `--overwrite-existing` command has been added to the `export` command to
  toggle whether or not to overwrite existing files. This used to be true all
  the time and is now customizable.

### Changes

- The short option name for `--quiet` is now `-q`.
- Removed the `data` and `resources` directories from the `export` command.

### Bug Fixes

- Slugified strings no longer appear in output data when using the `export`
  command.

- The `--trim-blocks` option now only leaves a single trailing line-break.

## v0.4.1 (2022-12-24)

### Bug Fixes

- `Books::tags` now returns a unique list of `#tags`.
- Passing a nonexistent template-group to `--template-group` now returns an
  error instead of silently failing.

## v0.4.0 (2022-12-22)

### Features

- Added the `--text-wrap <WIDTH>` option to enable text-wrapping.

### Changes

- Implemented a more robust pre- and post-processor.
- The configuration key `name-templates` is now `names`.
- A template's `names.annotations` value is now a list of dictionaries. This
  dictionary contains a field for the rendered filename as well as fields for
  some of its respective annotation's metadata. The primary reason for this
  change is to allow the user to optionally sort `names.annotations` by these
  metadata fields. See the documentation for
  [Context Reference - Names][names] for more information.
- The short option name for `--trim-blocks` is now `-t`.

## v0.3.0 (2022-10-09)

### Features

- Overhauled templates workflow.
  - A template's config is now set within the header of the file inside an
    HTML-comment. As a result, the filename of a template no longer matters.
    The only exception is when naming a template partial, these must begin with
    an underscore (`_`).
  - Nested rendered template outputs are now optional via the `structure` key
    in the template's config and can be customized via the
    `name-templates.directory` key.
  - Template output filenames are now customizable via the
    `name-templates.book` and `name-templates.annotation` keys in the template's
    config.
  - All [Tera][tera] features are now supported!
  - Added `--trim-blocks` to naively remove extra line-breaks from the final
    rendered template. This is only a temporary solution until [Tera][tera]
    implements this internally.
  - Added `--template-group` option to render only subset of templates found in
    the templates directory.
- Added pre-processing options to `render` and `export`.
  - `--extract-tags` to extract `#tags` from notes.
  - `--normalize-whitespace` to reduce 3+ line-breaks to 2.
  - `--ascii-only` to convert all Unicode characters to ASCII.
  - `--ascii-symbols` to convert only a subset of "smart" Unicode symbols to
    ASCII.
- Added `--quiet` flag to silence terminal output.
- Added `--databases-directory` option to use a custom databases path.
- Added `Book::tags`, a compiled list of all the tags within a book's
  annotations.
- Releases will now have binaries for Apple Silicon and Intel.
- Added CI, build, docs and publish actions.

### Changes

- Moved `--templates-directory` option under `render` command.
- Renamed `--templates` to `--templates-directory`.
- Renamed `--output` to `--output-directory`.
- Removed logging verbosity option from cli.
- Removed nested directory from output file structure i.e. `data`, `renders`,
  `backups`.
- Databases backup directories now have a `-` between the date and version:
  `[YYYY-MM-DD-HHMMSS]-[VERSION]`

## v0.2.0 (2022-01-31)

### Features

- Output message now includes the export location.

### Changes

- Reworked CLI commands.
- Updated license to MIT/Apache-2.0.
- Renamed 'assets' directory to 'resources'.
- Renamed 'items' directory to 'data'.
- Documented how to implement custom templates.

### Bug Fixes

- Fixed [#3][#3] Wrong default template location.

## v0.1.2 (2021-11-04)

### Features

- A `.gitkeep` file is now added inside each `assets` folder.
- Verified version support for Apple Books 4.1 on macOS Monterey 12.x.

### Bug Fixes

- `--backup` now copies only the `AEAnnotation` and `BKLibrary` directories.

## v0.1.1 (2021-10-30)

### Changes

- Fixed minor issues with the `Cargo.toml` file to work better with
  [crates.io][crates-io].

## v0.1.0 (2021-10-30)

- This initial release contains the core functionality: (1) save all
  annotations and notes as JSON (2) export them via a custom (or the default)
  template using the [Tera][tera] syntax or (3) backup the
  current Apple Books databases.

[#3]: https://github.com/tnahs/readstor/issues/3
[backup]: https://tnahs.github.io/readstor/latest/00-intro/02-04-backup.html
[crates-io]: https://crates.io
[documentation]: https://tnahs.github.io/readstor/latest/
[export]: https://tnahs.github.io/readstor/latest/00-intro/02-03-export.html
[filtering]: https://tnahs.github.io/readstor/latest/00-intro/02-01-filter.md
[names]: https://tnahs.github.io/readstor/latest/01-templates/06-03-names.html
[tera]: https://tera.netlify.app/
