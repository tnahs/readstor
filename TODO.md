# TODO

## v0.3.0

- [ ] Finish mdbook.
- [ ] Add redirection from docs root.

## Next

- [ ] Implement `Processor::run_postprocess`.
- [ ] Add `PostprocessOptions` for `Processor`.

  This will probably require that we save rendered templates and post-
  process them back up at the application level in order to avoid sending
  `PostprocessOptions` all the way down to the `TemplateManager::render_*`
  methods.

  ```rust
  pub struct TemplateManager {
    // ...
    // Possible changes to struct names.
    templates: Vec<TemplateRaw>,
    partials: Vec<TemplatePartialRaw>,
    renders: Vec<TemplateRendered>,
    // ...
  }

  pub struct TemplateRendered {
    path: PathBuf,
    filename: String,
    contents: String,
  }

  impl TemplateRendered {
    pub fn write(&self) {
      // Writes out self to disk.
    }
  }

  impl TemplateManager {
    pub fn write(&self) {
      // Writes out all rendered templates to disk. Called after post-processing.
    }
  }
  ```

- [ ] Add [`textwrap`][textwrap] post-processor. `--textwrap=80`
- [ ] Refactor `TemplateManager::render` and its sibling rendering methods.
- [ ] After mdbook is complete, update internal docs.
- [ ] Add `# Arguments` to public methods.
- [ ] Config file support.
- [ ] Checkoutl [fern][fern] for stdout/stderr and file logging.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[textwrap]: https://docs.rs/textwrap/latest/textwrap/
