# Names

A single `names` object is injected into all template contexts regardless of
the template's [Context Mode][context-modes]. These contain all the file and
directory names rendered from the `names` key in the template's config. See
[Names][names] for more information.

## Template Fields

### Names

| Attribute           | Type                               | Description             |
| ------------------- | ---------------------------------- | ----------------------- |
| `names`             | dictionary                         | names object            |
| `names.book`        | string                             | rendered book filename  |
| `names.annotations` | list\[annotation-name-attributes\] | annotations object      |
| `names.directory`   | string                             | rendered directory name |

### Annotation Name Attributes

| Attribute             | Type   | Description                  |
| --------------------- | ------ | ---------------------------- |
| `annotation.filename` | string | rendered annotation filename |
| `annotation.created`  | string | date created                 |
| `annotation.modified` | string | date modified                |
| `annotation.location` | string | location string              |

## Example Data - Names

With the following `names` configuration:

```yaml
names:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}"
  directory: "{{ book.author }} - {{ book.title }}"
```

```json
{
  "book": "Robert Henri - The Art Spirit.md",
  "annotations": [
    {
      "filename": "2021-11-02-181510-the-art-spirit.md",
      "created": "2021-11-02T18:15:10.700510978Z",
      "modified": "2021-11-02T18:15:20.879488945Z",
      "location": "6.26.4.2.636.2.1:0"
    },
    {
      "filename": "2021-11-02-180445-the-art-spirit.md",
      "created": "2021-11-02T18:04:45.184863090Z",
      "modified": "2021-11-02T18:12:30.355533123Z",
      "location": "6.26.4.2.446.2.1:0"
    },
    {
      "filename": "2021-11-02-181325-the-art-spirit.md",
      "created": "2021-11-02T18:13:25.905355930Z",
      "modified": "2021-11-02T18:14:12.444134950Z",
      "location": "6.24.4.2.296.2.1:0"
    },
    {
      "filename": "2021-11-02-181250-the-art-spirit.md",
      "created": "2021-11-02T18:12:50.826642036Z",
      "modified": "2021-11-02T18:12:51.831905841Z",
      "location": "6.18.4.2.20.2.1:0"
    }
  ],
  "directory": "Robert Henri - The Art Spirit"
}
```

## Example Template - Names

```jinja2
# {{ book.author }} - {{ book.title }}

{% for annotation in names.annotations | sort(attribute="location") -%}
![[{{ annotation.filename }}]]
{% endfor %}
```

[context-modes]: ./02-02-context-modes.md
[names]: ./02-05-names.md
