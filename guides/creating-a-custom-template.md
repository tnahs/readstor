# Creating a Custom Template

```plaintext
[name].[template-kind].[extension]
```

```plaintext
my-template1.entry.[extension]
my-template2.book.[extension]
my-template2.annotations.[extension]
```

```plaintext
[name].single.[extension]
[name].multi.[extension]
[name].partial.[extension]
```

```plaintext
[templates]
 │
 ├─ CUSTOM-TEMPLATE-A.single.[extension]
 │
 ├─ CUSTOM-TEMPLATE-B.multi.[extension]
 │
 ├─ CUSTOM-TEMPLATE-C
 │   ├─ name.multi.[extension]
 │   ├─ name-book.partial.[extension]
 │   ├─ name-annotation.partial.[extension]
 │   └─ ...
 │
 ├─ _HIDDEN-TEMPLATE-A.[extension]
 │
 ├─ .HIDDEN-TEMPLATE-B.[extension]
 │
 ├─ _HIDDEN-TEMPLATE-C.[extension]
 │   └─ ...
 │
 └─ ...
```

```plaintext
[output]
 │
 ├─ CUSTOM-TEMPLATE-A
 │  ├─ [author-title].md
 │  ├─ [author-title].md
 │  └─ ...
 │
 ├─ CUSTOM-TEMPLATE-B
 │  ├─ [author-title].md
 │  ├─ [YYYY-MM-DD-HHMMSS]-[title].md
 │  ├─ [YYYY-MM-DD-HHMMSS]-[title].md
 │  └─ ...
 │
 ├─ CUSTOM-TEMPLATE-C
 │  ├─ [YYYY-MM-DD-HHMMSS]-[title].md
 │  ├─ [YYYY-MM-DD-HHMMSS]-[title].md
 │  └─ ...
 └─ ...
```

_Templates or directories prefixed with a dot (.) or an underscore (_) will be
ignored.\_

## Syntax

The templating syntax is based on Jinja2 and Django templates. In a nutshell, values are accessed by placing an attribute between `{{ }}` e.g. `{{ book.title }}`. [Filters](https://tera.netlify.app/docs/#filters) can manipulate the accessed values e.g. `{{ name | capitalize }}`. And [statements](https://tera.netlify.app/docs/#control-structures) placed between `{% %}` e.g. `{% if my_var %} ... {% else %} ... {% endif %}`, can be used for control flow. For more information, see the [Tera](https://tera.netlify.app/docs/#templates) documentation.

## Attributes

Every template has access to two object: the current book as `book` and its annotations as `annotations`.

### Book

```plaintext
book {
    title
    author
    metadata {
        id
        last_opened
    }
}
```

### Book Attributes

| Attribute                   | Description                   | Type       |
| --------------------------- | ----------------------------- | ---------- |
| `book.title`                | title of the book             | `string`   |
| `book.author`               | author of the book            | `string`   |
| `book.metadata.id`          | book's unique identifier      | `string`   |
| `book.metadata.last_opened` | date the book was last opened | `datetime` |

### Book Example

Here the [`date`](https://tera.netlify.app/docs/#date) filter is used to format a `datetime` object into a human-readable date.

```jinja
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date }}
```

## Annotations

```plaintext
annotations [
    annotation {
        body
        style
        notes
        tags
        metadata {
            id
            book_id
            created
            modified
            location
            epubcfi
        }
    },
    ...
]
```

### Annotations Attributes

| Attribute                      | Description                             | Type           |
| ------------------------------ | --------------------------------------- | -------------- |
| `annotations`                  | book's annotations                      | `[annotation]` |
| `annotation.body`              | annotation's body                       | `[string]`     |
| `annotation.style`             | annotation's style/color e.g. 'yellow'  | `string`       |
| `annotation.notes`             | annotation's notes                      | `string`       |
| `annotation.tags`              | annotation's tags                       | `[string]`     |
| `annotation.metadata.id`       | annotation's unique identifier          | `string`       |
| `annotation.metadata.book_id`  | book's unique identifier                | `string`       |
| `annotation.metadata.created`  | date the annotation was created         | `datetime`     |
| `annotation.metadata.modified` | date the annotation was modified        | `datetime`     |
| `annotation.metadata.location` | `epubcfi` parsed into a location string | `string`       |
| `annotation.metadata.epubcfi`  | `epubcfi`                               | `string`       |

### Annotation Example

```jinja
{% for annotation in annotations %}

{{ annotation.body }}

notes: {{ annotation.notes }}
tags: {{ annotation.tags | join(sep=" ") }}

{% endfor %}
```
