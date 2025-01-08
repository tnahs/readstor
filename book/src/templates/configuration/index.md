# Configuration

A template's configuration describes both what the template expects to render and how the output
structure and naming should be. Every template must start with a configuration block.

## Overview

The configuration starts off as a basic HTML comment tag...

```markdown
<!-- -->
```

... with the word `readstor`...

```markdown
<!-- readstor -->
```

...and a new line.

```markdown
<!-- readstor

-->
```

The YAML configuration is then placed inside the tag. For example:

```markdown
<!-- readstor
group: my-vault
context: book
structure: nested
extension: md
names:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.slugs.metadata.created }}-{{ book.slugs.title }}"
  directory: "{{ book.author }} - {{ book.title }}"
-->

...
```

> <i class="fa fa-exclamation-circle"></i> Note that the final rendered output file will not include
> its template's configuration. Additionally, if the configuration ended with trailing line-breaks,
> a single one of them is removed. This allows for some extra whitespace while working with a
> template without affecting final rendered output.

A quick rundown of each configuration key:

| Key         | Description                                                                       |
| ----------- | --------------------------------------------------------------------------------- |
| `group`     | The [Template Group][template-groups] name.                                       |
| `context`   | The [Context Mode][context-modes] or what the template will render.               |
| `structure` | The [Structure Mode][structure-modes] or how the output files will be structured. |
| `extension` | The template's output [File Extension][file-extensions].                          |
| `names`     | The template [Names][names] for generating file and directory names.              |

[context-modes]: ../configuration/context-modes.md
[file-extensions]: ../configuration/file-extensions.md
[names]: ../configuration/names.md
[structure-modes]: ../configuration/structure-modes.md
[template-groups]: ../configuration/template-groups.md
