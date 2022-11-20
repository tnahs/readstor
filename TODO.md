# TODO

## Next

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

- [ ] After mdbook is complete, update internal docs.
- [ ] Add `# Arguments` to public methods.
- [ ] Config file support.
- [ ] Test [Tera][tera] macros and inheritances.
- [ ] Checkout [fern][fern] for stdout/stderr and file logging.
- [ ] Print `backup` summary.

## Features

- [ ] Extract data from iOS's `com.apple.ibooks-sync.plist` and `Books.plist` files.

## Future

- [ ] Internationalization.

[fern]: https://docs.rs/fern/latest/fern/
[slugify]: https://tera.netlify.app/docs/#slugify
[tera]: https://tera.netlify.app/
[textwrap]: https://docs.rs/textwrap/latest/textwrap/
