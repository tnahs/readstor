# Book

A single `book` object is injected into all template contexts regardless of the template's [Context
Mode][context-modes].

## Template Fields - Book

| Attribute                         | Type       | Description                |
| --------------------------------- | ---------- | -------------------------- |
| `book`                            | dictionary | book object                |
| `book.title`                      | string     | title                      |
| `book.author`                     | string     | author                     |
| `book.metadata`                   | dictionary | metadata                   |
| `book.metadata.id`                | string     | unique id                  |
| `book.metadata.last_opened`       | datetime   | date last opened           |
| `book.slugs`                      | dictionary | slugs object               |
| `book.slugs.title`                | string     | title slugified            |
| `book.slugs.author`               | string     | author slugified           |
| `book.slugs.metadata`             | datetime   | slugs metadata object      |
| `book.slugs.metadata.last_opened` | datetime   | date last opened slugified |

## Example Data - Book

```json
{
  "title": "The Art Spirit",
  "author": "Robert Henri",
  "tags": ["#artist", "#being", "#inspiration"],
  "metadata": {
    "id": "1969AF0ECA8AE4965029A34316813924",
    "last_opened": "2021-11-02T18:27:04.781938076Z"
  },
  "slugs": {
    "title": "the-art-spirit",
    "author": "robert-henri"
  }
}
```

## Example Template - Book

```jinja2
---
title: {{ book.title }}
author: {{ book.author }}
id: {{ book.metadata.id }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%d-%H:%M") }}
---
```

> <i class="fa fa-info-circle"></i> Here [Tera][tera]'s [`date`][tera-date] filter is used to format
> a `datetime` object into a human-readable date.

[context-modes]: ../configuration/context-modes.md
[tera]: https://keats.github.io/tera/
[tera-date]: https://keats.github.io/tera/docs/#date
