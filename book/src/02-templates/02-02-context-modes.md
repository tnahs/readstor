# Context Modes

|              |                     |
| ------------ | ------------------- |
| Name         | `context`           |
| Type         | string              |
| Valid Values | `book` `annotation` |
| Required     | true                |
| Default      | -                   |

At render time each template is injected with a "context", in other words, the
data it will render. ReadStor provides two different contexts: a `book` context
and an `annotation` context.

## The Book Context

```yaml
context: book
```

When selected, a single file is rendered containing the data from a single book
and its annotations. For example, represented here in YAML:

```yaml
book:
  title: The Art Spirit
  author: Robert Henri
annotations:
  - body: We are not here to do what has already been done.
    notes: ""
    tags: []
    metadata:
      id: C932CE69-8584-4555-834C-797DF84E6825
  - body: The object of painting a picture...
    notes: ""
    tags:
      - "#artist"
      - "#being"
    metadata:
      id: C932CE69-8584-4555-834C-797DF84E6825
  # ...
```

_Note that this is a subset of the available data inside an `annotation`
context. See [Context Reference - Book](./04-context-reference.md#book) for more
information._

## The Annotation Context

```yaml
context: annotation
```

When selected, multiple files are rendered, each containing the data from a
single annotation and its respective book. For example, represented here in
YAML:

```yaml
book:
  title: The Art Spirit
  author: Robert Henri
annotation:
  body: We are not here to do what has already been done.
  notes: ""
  tags: []
  metadata:
    id: C932CE69-8584-4555-834C-797DF84E6825
```

_Note that this is a subset of the available data inside an `annotation`
context. See [Context Reference - Annotation](./04-context-reference.md#annotation)
for more information._

## Important Note: Context Modes

A template acts differently depening on its context. When the context mode is
set to `book`, a single file is rendered from the template. When the context
mode is set to `annotation` mutltiple files are rendered, one for each
annotation within the book.

And when the context mode is: `book`:

```yaml
context: book
structure: nested
```

The output structure is as follows:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │    └── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │    └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit
      └── Robert Henri - The Art Spirit.md`
```

And when the context mode is: `annotation`:

```yaml
context: annotation
structure: nested
```

The output structure is as follows:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │    ├── 2021-11-02-182319-think-on-these-things.md
 │    ├── 2021-11-02-182426-think-on-these-things.md
 │    ├── 2021-11-02-182543-think-on-these-things.md
 │    ├── 2021-11-02-182648-think-on-these-things.md
 │    └── 2021-11-02-182805-think-on-these-things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │    └── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 └── Robert Henri - The Art Spirit
      ├── 2021-11-02-180445-the-art-spirit.md
      ├── 2021-11-02-181250-the-art-spirit.md
      ├── 2021-11-02-181325-the-art-spirit.md
      └── 2021-11-02-181510-the-art-spirit.md
```

When both templates are rendered, we get:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │    ├── 2021-11-02-182319-think-on-these-things.md
 │    ├── 2021-11-02-182426-think-on-these-things.md
 │    ├── 2021-11-02-182543-think-on-these-things.md
 │    ├── 2021-11-02-182648-think-on-these-things.md
 │    ├── 2021-11-02-182805-think-on-these-things.md
 │    └── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │    ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │    └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit
      ├── 2021-11-02-180445-the-art-spirit.md
      ├── 2021-11-02-181250-the-art-spirit.md
      ├── 2021-11-02-181325-the-art-spirit.md
      ├── 2021-11-02-181510-the-art-spirit.md
      └── Robert Henri - The Art Spirit.md`
```
