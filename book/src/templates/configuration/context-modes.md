# Context Modes

|              |                             |
| ------------ | --------------------------- |
| Name         | `context`                   |
| Type         | string                      |
| Valid Values | `book` `annotation`         |
| Required     | <i class="fa fa-check"></i> |
| Default      | -                           |

At render time, each template is injected with a "context", in other words, the data it will render.
ReadStor provides two different context modes: `book` and `annotation`. The context mode dictates
not just the data within the context but also changes the number of output files. See [A Note On
Output Structure](#a-note-on-output-structure) for more information.

## The Book Context

|                 |                                                             |
| --------------- | ----------------------------------------------------------- |
| Context Mode    | `book`                                                      |
| Context Objects | [`book`][book] [`annotations`][annotation] [`names`][names] |
| Output Files    | =1                                                          |

> <i class="fa fa-info-circle"></i> Note that `annotations` is plural in the `book` context.

When selected, a single file is rendered out from a context containing the data from a single book
and its annotations. For example, represented here in YAML:

```yaml
book:
  title: The Art Spirit
  author: Robert Henri
  metadata:
    id: 1969AF0ECA8AE4965029A34316813924
    last_opened: 2021-11-02T18:27:04.781938076Z
  slugs:
    title: the-art-spirit
    author: robert-henri
annotations:
  - body: We are not here to do what has already been done.
    style: purple
    notes: ""
    tags: []
    metadata:
      id: C932CE69-8584-4555-834C-797DF84E6825
      book_id: 1969AF0ECA8AE4965029A34316813924
      created: 2021-11-02T18:12:50.826642036Z
      modified: 2021-11-02T18:12:51.831905841Z
      location: 6.18.4.2.20.2.1:0
      epubcfi: epubcfi(/6/18[Part09_Split0]!/4/2/20/2/1,:0,:49)
      slugs:
        created: 2021-11-02-181250
        modified: 2021-11-02-181250
  - body: The object of painting a picture...
    style: yellow
    notes: ""
    tags:
      - "#artist"
      - "#being"
    metadata:
      id: 3FCC630A-55E6-4D6F-8E8F-DAD7C4E20A1C
      book_id: 1969AF0ECA8AE4965029A34316813924
      created: 2021-11-02T18:13:25.905355930Z
      modified: 2021-11-02T18:14:12.444134950Z
      location: 6.24.4.2.296.2.1:0
      epubcfi: epubcfi(/6/24[Part09_Split3]!/4/2/296/2,/1:0,/7:257)
      slugs:
        created: 2021-11-02-181325
        modified: 2021-11-02-181325
  # ...
names:
  book: Robert Henri - The Art Spirit.md
  annotations:
    C932CE69-8584-4555-834C-797DF84E6825: 2021-11-02-181250-the-art-spirit.md
    3FCC630A-55E6-4D6F-8E8F-DAD7C4E20A1C: 2021-11-02-181325-the-art-spirit.md
    # ...
  directory: Robert Henri - The Art Spirit
```

> <i class="fa fa-info-circle"></i> See [Context Reference - Book][book] for more information.

## The Annotation Context

|                 |                                                            |
| --------------- | ---------------------------------------------------------- |
| Context Mode    | `annotation`                                               |
| Context Objects | [`book`][book] [`annotation`][annotation] [`names`][names] |
| Output Files    | >=1                                                        |

When selected, multiple files are rendered out from a context containing the data from a single
annotation and its respective book. For example, represented here in YAML:

```yaml
book:
  title: The Art Spirit
  author: Robert Henri
  metadata:
    id: 1969AF0ECA8AE4965029A34316813924
    last_opened: 2021-11-02T18:27:04.781938076Z
  slugs:
    title: the-art-spirit
    author: robert-henri
annotation:
  body: We are not here to do what has already been done.
  style: purple
  notes: ""
  tags: []
  metadata:
    id: C932CE69-8584-4555-834C-797DF84E6825
    book_id: 1969AF0ECA8AE4965029A34316813924
    created: 2021-11-02T18:12:50.826642036Z
    modified: 2021-11-02T18:12:51.831905841Z
    location: 6.18.4.2.20.2.1:0
    epubcfi: epubcfi(/6/18[Part09_Split0]!/4/2/20/2/1,:0,:49)
    slugs:
      created: 2021-11-02-181250
      modified: 2021-11-02-181250
names:
  book: Robert Henri - The Art Spirit.md
  annotations:
    C932CE69-8584-4555-834C-797DF84E6825: 2021-11-02-181250-the-art-spirit.md
  directory: Robert Henri - The Art Spirit
```

> <i class="fa fa-info-circle"></i> See [Context Reference - Annotation][annotation] for more information.

## A Note On Output Structure

When selecting a context mode it's important to understand how the output files will look. The
following is a quick visualization.

When the context mode is set to `book`, a single file for each book is rendered out from the
template. Using the following configuration keys:

```yaml
context: book
structure: nested
```

The output structure is as follows:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │   └── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit
     └── Robert Henri - The Art Spirit.md`
```

And when the context mode is set to `annotation`, multiple files are rendered out, one for each
annotation within a book. Using the following configuration keys:

```yaml
context: annotation
structure: nested
```

The output structure is as follows:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │   ├── 2021-11-02-182319-think-on-these-things.md
 │   ├── 2021-11-02-182426-think-on-these-things.md
 │   ├── 2021-11-02-182543-think-on-these-things.md
 │   ├── 2021-11-02-182648-think-on-these-things.md
 │   └── 2021-11-02-182805-think-on-these-things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   └── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 └── Robert Henri - The Art Spirit
     ├── 2021-11-02-180445-the-art-spirit.md
     ├── 2021-11-02-181250-the-art-spirit.md
     ├── 2021-11-02-181325-the-art-spirit.md
     └── 2021-11-02-181510-the-art-spirit.md
```

When both templates are rendered to the same directory, we get a complete output with a book and
each of its annotations all rendered out to their own file.

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │   ├── 2021-11-02-182319-think-on-these-things.md
 │   ├── 2021-11-02-182426-think-on-these-things.md
 │   ├── 2021-11-02-182543-think-on-these-things.md
 │   ├── 2021-11-02-182648-think-on-these-things.md
 │   ├── 2021-11-02-182805-think-on-these-things.md
 │   └── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │   └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit
     ├── 2021-11-02-180445-the-art-spirit.md
     ├── 2021-11-02-181250-the-art-spirit.md
     ├── 2021-11-02-181325-the-art-spirit.md
     ├── 2021-11-02-181510-the-art-spirit.md
     └── Robert Henri - The Art Spirit.md
```

[annotation]: /templates/context-reference/annotation.md
[book]: /templates/context-reference/book.md
[names]: /templates/context-reference/names.md
