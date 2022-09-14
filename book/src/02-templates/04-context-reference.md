# Context Reference

## Book

### Book Data Structure

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

| Attribute                   | Type       | Description                 |
| --------------------------- | ---------- | --------------------------- |
| `book`                      | dictionary | The book object             |
| `book.title`                | string     | The book's title            |
| `book.author`               | string     | The book's author           |
| `book.metadata`             | dictionary | The book's metadata         |
| `book.metadata.id`          | string     | The book's unique id        |
| `book.metadata.last_opened` | datetime   | The book's last opened date |
|                             |            |                             |
| `book.slug_title`           | string     |                             |
| `book.slug_author`          | string     |                             |

### Example Book Template

Here the [`date`](https://tera.netlify.app/docs/#date) filter is used to format
a `datetime` object into a human-readable date.

```jinja
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date }}
```

## Annotation

### Annotation Data Structure

```plaintext
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
}
```

### Annotation Attributes

<!-- markdownlint-disable MD013 -->

| Attribute                      | Type         | Description                        |
| ------------------------------ | ------------ | ---------------------------------- |
| `annotation`                   | dictionary   | The annotation object              |
| `annotation.body`              | string       | The annotation's body              |
| `annotation.style`             | string       | The annotation's style/color       |
| `annotation.notes`             | string       | The annotation's notes             |
| `annotation.tags`              | list[string] | The annotation's tags              |
| `annotation.metadata`          | dictionary   | The annotation's metadata          |
| `annotation.metadata.id`       | string       | The annotation's unique id         |
| `annotation.metadata.book_id`  | string       | The book's unique id               |
| `annotation.metadata.created`  | datetime     | The annotation's creation date     |
| `annotation.metadata.modified` | datetime     | The annotation's modification date |
| `annotation.metadata.location` | string       | The annoataion's location string   |
| `annotation.metadata.epubcfi`  | string       | The annotation's epubcfi           |

<!-- markdownlint-enable MD013 -->

### Annotation Example

```plaintext
{{ annotation.body }}

notes: {{ annotation.notes }}
tags: {{ annotation.tags | join(sep=" ") }}
```
