# Name Templates

Using the same templating syntax, output files and directory names can be
customized. ReadStor will inject a different context into each field's template
during render time and set the the template's output filename to the resulting
string.

| Field                       | Context      |
| --------------------------- | ------------ |
| `name-templates.book`       | `book`       |
| `name-templates.annotation` | `annotation` |
| `name-templates.directory`  | `book`       |

See [Context Reference](./04-context-reference.md) for more information.

Note that the template's `context` matters when setting `name-templates`. For
example, if the template's `context` is set to `book`:

<!-- markdownlint-disable MD013 -->

```yaml
group: my-vault
structure: flat
context: book # <- Here!
extension: md
name-templates:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.metadata.slug_created }}-{{ book.slug_title }}"
```

<!-- markdownlint-enable MD013 -->

ReadStor will generate a single file for each book and name them using the
`name-templates.book` field's template. The resulting output would be:

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 ├── Robert Henri - The Art Spirit.md
 └── ...
```

<!-- markdownlint-disable MD013 -->

However if the `context` is changed to `annotation`:

```yaml
group: my-vault
structure: flat
context: annotation # <- Here!
extension: md
name-templates:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.metadata.slug_created }}-{{ book.slug_title }}"
```

<!-- markdownlint-enable MD013 -->

ReadStor will now generate a single file for each annotation in each book and
name them using the `name-templates.annotation` field's template. The resulting
output would be:

```plaintext
[output-directory]
 ├── 2021-11-02-180445-the-art-spirit.md
 ├── 2021-11-02-181250-the-art-spirit.md
 ├── 2021-11-02-181325-the-art-spirit.md
 ├── 2021-11-02-181510-the-art-spirit.md
 ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 ├── 2021-11-02-182319-think-on-these-things.md
 ├── 2021-11-02-182426-think-on-these-things.md
 ├── 2021-11-02-182543-think-on-these-things.md
 ├── 2021-11-02-182648-think-on-these-things.md
 ├── 2021-11-02-182805-think-on-these-things.md
 └── ...
```

## Outut Book Name Template

|              |                                        |
| ------------ | -------------------------------------- |
| Name         | `name-templates.book`                  |
| Type         | string                                 |
| Valid Values | any                                    |
| Required     | No                                     |
| Default      | `{{ book.author }} - {{ book.title }}` |

Defines the naming template to use when rendering a template with its `context`
set to `book`.

## Output Annotation Name Template

|              |                                                                |
| ------------ | -------------------------------------------------------------- |
| Name         | `name-templates.annotation`                                    |
| Type         | string                                                         |
| Valid Values | any                                                            |
| Required     | No                                                             |
| Default      | `{{ annotation.metadata.slug_created }}-{{ book.slug_title }}` |

Defines the naming template to use when rendering a template with its `context`
set to `annotation`.

## Output Directoy Name Template

|              |                                        |
| ------------ | -------------------------------------- |
| Name         | `name-templates.directory`             |
| Type         | string                                 |
| Valid Values | any                                    |
| Required     | No                                     |
| Default      | `{{ book.author }} - {{ book.title }}` |

Defines the naming template to use when rendering a template with its
`structure` set to `nested` or the `nested-grouped`.
