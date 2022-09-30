# Links

A single `links` object is injected into all template contexts regardless
of the template's [Context Mode][context-modes]. These contain all the file
and directory names rendered from the `name-templates` key in the template's
config. See [Name Templates][name-templates] for more information.

## Template Fields - Links

| Attribute                     | Type       | Description                   |
| ----------------------------- | ---------- | ----------------------------- |
| `links`                       | dictionary | link object                   |
| `links.book`                  | string     | rendered book file name       |
| `links.annotations`           | dictionary | annotations object            |
| `links.annotations - key:_`   | string     | annotation id                 |
| `links.annotations - _:value` | string     | rendered annotation file name |
| `links.directorty`            | string     | rendered directory name       |

## Example Data - Links

With the following `name-templates` configuration:

```yaml
name-templates:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}"
  directory: "{{ book.author }} - {{ book.title }}"
```

```json
{
  "book": "Robert Henri - The Art Spirit.md",
  "annotations": {
    "C932CE69-8584-4555-834C-797DF84E6825": "2021-11-02-181250-the-art-spirit.md",
    "4620564A-0B64-4099-B5D6-6C9116A03AFF": "2021-11-02-181510-the-art-spirit.md",
    "9D1B71B1-895C-446F-A03F-50C01146F532": "2021-11-02-180445-the-art-spirit.md",
    "3FCC630A-55E6-4D6F-8E8F-DAD7C4E20A1C": "2021-11-02-181325-the-art-spirit.md"
  },
  "directory": "Robert Henri - The Art Spirit"
}
```

## Example Template - Links

```jinja2
# {{ book.author }} - {{ book.title }}

{% for _, link in links.annotations -%}
![[{{ link }}]]
{% endfor %}
```

[context-modes]: ./02-02-context-modes.md
[name-templates]: ./02-05-name-templates.md
