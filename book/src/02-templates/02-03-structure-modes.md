# Structure Modes

|              |                                                 |
| ------------ | ----------------------------------------------- |
| Name         | `ouput`                                         |
| Type         | string                                          |
| Valid Values | `flat` `flat-grouped` `nested` `nested-grouped` |
| Required     | true                                            |
| Default      | -                                               |

The structure mode determines how the output directories and files are structured.
ReaStor provides four output modes: `flat`, `flat-grouped`, `nested` and
`nested-grouped`.

## Flat Mode

```yaml
group: my-vault
structure: flat
```

When selected, the template is rendered to the output directory without any
structure. All the files are placed directly within the output directory. No
additional directories are created.

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 ├── Robert Henri - The Art Spirit.md
 └── ...
```

## Flat & Grouped Mode

```yaml
group: my-vault
structure: flat-grouped
```

When selected, the template is rendered to the output directory and placed
inside a directory named after its `group`. This useful if there are multiple
templates being rendered to the same directory or when multiple templates are
intended to be part of a single ouput.

```plaintext
[output-directory]
 └── my-vault
      ├── Krishnamurti - Think on These Things.md
      ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
      ├── Robert Henri - The Art Spirit.md
      └── ...
```

## Nested Mode

```yaml
group: my-vault
structure: nested
```

When selected, the template is rendered to the output directory and placed
inside a directory named after the `name-templates.directory` field. This useful
if multiple templates are used to represent a single book i.e. a book template
used to render a book's information to a separate file and an annotation
template used to render each annotation to a separate file.

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

## Nested & Grouped Mode

```yaml
group: my-vault
structure: nested-grouped
```

When selected, the template is rendered to the output directory and placed
inside a directory named after its `group` and another named after the
`name-templates.directory` field. This useful if multiple templates are used to
represent a single book i.e. a book template and an annotation template.

```plaintext
[output-directory]
 └── my-vault
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
           └── Robert Henri - The Art Spirit.md
```
