# Annotation

The number of `annotation` objects injected into a template depends on
the template's [Context Mode][context-modes]. When the
[Context Mode][context-modes-annotation] is set to `annotation`, a single
`annotation` object is injected into the template's context. When the
[Context Mode][context-modes-book] is set to `book`, multiple `annotation`
objects, under the name `annotations`, are injected into the template's context.

## Template Fields - Annotation

| Attribute                            | Type               | Description             |
| ------------------------------------ | ------------------ | ----------------------- |
| `annotations`                        | list\[dictionary\] | annotation objects      |
| `annotation`                         | dictionary         | annotation object       |
| `annotation.body`                    | string             | body                    |
| `annotation.style`                   | string             | highlight style/color   |
| `annotation.notes`                   | string             | notes                   |
| `annotation.tags`                    | list\[string\]     | tags                    |
| `annotation.metadata`                | dictionary         | metadata                |
| `annotation.metadata.id`             | string             | unique id               |
| `annotation.metadata.book_id`        | string             | book's unique id        |
| `annotation.metadata.created`        | datetime           | date created            |
| `annotation.metadata.modified`       | datetime           | date modified           |
| `annotation.metadata.location`       | string             | location string         |
| `annotation.metadata.epubcfi`        | string             | [epubcfi][epubcfi]      |
| `annotation.metadata.slugs`          | dictionary         | slugs object            |
| `annotation.metadata.slugs.created`  | string             | date created slugified  |
| `annotation.metadata.slugs.modified` | string             | date modified slugified |

<!-- TODO: Add a note on highlight style/color. -->
<!-- TODO: Add a note on tags when tag extraction flag is implemented. -->

## Example Data - Annotation

```json
{
  "body": "Of course it is not easy to go one’s road...",
  "style": "blue",
  "notes": "",
  "tags": [],
  "metadata": {
    "id": "9D1B71B1-895C-446F-A03F-50C01146F532",
    "book_id": "1969AF0ECA8AE4965029A34316813924",
    "created": "2021-11-02T18:04:45.184863090Z",
    "modified": "2021-11-02T18:12:30.355533123Z",
    "location": "6.26.4.2.446.2.1:0",
    "epubcfi": "epubcfi(/6/26[c_9780465008575_ps_Part09_Split4]!/4/2/446/2/1,:0,:679)",
    "slugs": {
      "created": "2021-11-02-180445",
      "modified": "2021-11-02-180445"
    }
  }
}
```

## Example Template - Annotation

```jinja2
{% for annotation in annotations -%}

---

{{ annotation.body }}

{%- if annotation.notes %}notes: {{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}tags: {{ annotation.tags | join(sep=" ") }}{% endif %}

{% endfor %}
```

> <i class="fa fa-info-circle"></i> Here the [`join`][join] filter is used to
> join an array of items into a space-separated string.

[context-modes]: ./02-02-context-modes.md
[context-modes-book]: ./02-02-context-modes.md#the-book-context
[context-modes-annotation]: ./02-02-context-modes.md#the-annotation-context
[join]: https://tera.netlify.app/docs/#join
[epubcfi]: https://w3c.github.io/epub-specs/epub33/epubcfi/
