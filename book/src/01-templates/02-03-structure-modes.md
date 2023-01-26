# Structure Modes

|              |                                                 |
| ------------ | ----------------------------------------------- |
| Name         | `ouput`                                         |
| Type         | string                                          |
| Valid Values | `flat` `flat-grouped` `nested` `nested-grouped` |
| Required     | <i class="fa fa-check"></i>                     |
| Default      | -                                               |

The structure mode determines how the output directories and files are
structured. ReadStor provides four structure modes: `flat`, `flat-grouped`,
`nested` and `nested-grouped`.

## Flat Mode

```yaml
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
inside a directory named after its `group`. This is useful if multiple template
groups are being rendered to the same directory.

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
structure: nested
```

When selected, the template is rendered to the
[output directory][output-directory] and placed inside a directory named after
the `names.directory` key. This is useful if a template group contains multiple
templates.

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things
 │   ├── 2021-11-02-182319-think-on-these-things.md
 │   ├── 2021-11-02-182426-think-on-these-things.md
 │   ├── 2021-11-02-182543-think-on-these-things.md
 │   ├── 2021-11-02-182648-think-on-these-things.md
 │   ├── 2021-11-02-182805-think-on-these-things.md
 │   └── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │   └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 ├── Robert Henri - The Art Spirit
 │   ├── 2021-11-02-180445-the-art-spirit.md
 │   ├── 2021-11-02-181250-the-art-spirit.md
 │   ├── 2021-11-02-181325-the-art-spirit.md
 │   ├── 2021-11-02-181510-the-art-spirit.md
 │   └── Robert Henri - The Art Spirit.md`
 └── ...
```

## Nested & Grouped Mode

```yaml
group: my-vault
structure: nested-grouped
```

When selected, the template is rendered to the
[output directory][output-directory] and placed inside a directory named after
its `group` and another named after the `names.directory` key. This is useful
if multiple template groups are being rendered to the same directory and if a
template group contains multiple templates.

```plaintext
[output-directory]
 └── my-vault
     ├── Krishnamurti - Think on These Things
     │   ├── 2021-11-02-182319-think-on-these-things.md
     │   ├── 2021-11-02-182426-think-on-these-things.md
     │   ├── 2021-11-02-182543-think-on-these-things.md
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

[output-directory]: ../00-intro/02-01-global.md#--output-directory-path
