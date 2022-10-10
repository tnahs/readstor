# TODO

## Next

- [ ] Implement `Processor::postprocess`.
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
- [ ] We might be able to leverage [Tera][tera]'s [slugify][slugify] filter and
      remove slugs from the `Book` and `Annotation` structs.
- [ ] Drop `indexmap`, `serde_json/preserve_order` and `tera/preserve_order`
      dependency by using a `Vec` of something like the `NameAnnotation` struct.
      This would allow the user to use the `sort` filter to sort the links by
      any field within `NameAnnotation`.

  ```rust
  pub struct Names {
      pub book: String,
      pub annotations: Vec<NameAnnotation>,
      pub directory: String,
  }
  ```

  ```rust
  struct NameAnnotation {
    id: String,
    name: String,
    date_created: DatetimeUtc,
    date_modified: DatetimeUtc,
    location: String,
  }
  ```

  ```jinja
  {% for link in annotation.links | sort(attribute="location")  %}
  ![[{{ link }}]]
  {% endfor %}
  ```

- [ ] Refactor `TemplateManager::render` and its sibling rendering methods.
- [ ] After mdbook is complete, update internal docs.
- [ ] Add `# Arguments` to public methods.
- [ ] Config file support.
- [ ] Checkout [fern][fern] for stdout/stderr and file logging.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[slugify]: https://tera.netlify.app/docs/#slugify
[tera]: https://tera.netlify.app/
[textwrap]: https://docs.rs/textwrap/latest/textwrap/
