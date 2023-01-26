# Template Groups

|              |                             |
| ------------ | --------------------------- |
| Name         | `group`                     |
| Type         | string                      |
| Valid Values | any                         |
| Required     | <i class="fa fa-check"></i> |
| Default      | -                           |

Groups are used to identify multiple templates that are intended to be part
of a single output. Conversely, they also provide a way to separate unrelated
templates rendered to the same output directory. The most common use case would
be when using a pair of templates where one is used to render a book and the
other to render each of its annotations separately.

> <i class="fa fa-exclamation-circle"></i> Group names are sanitized to make sure
> they interact well with the file system. See
> [String Sanitization][string-sanitization] for more information.

Grouping is triggered when one or more templates are set to either the
`flat-grouped` or the `nested-grouped` [Structure Mode][structure-modes].
The output files are placed within a directory named after the `group`.

For example, if two templates share these values in their configurations:

```yaml
group: my-vault
structure: flat-grouped
```

The output will be:

```plaintext
[output-directory]
 ├── my-vault
 │   ├── 2021-11-02-180445-the-art-spirit.md
 │   ├── 2021-11-02-181250-the-art-spirit.md
 │   ├── 2021-11-02-181325-the-art-spirit.md
 │   ├── 2021-11-02-181510-the-art-spirit.md
 │   ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │   ├── 2021-11-02-182319-think-on-these-things.md
 │   ├── 2021-11-02-182426-think-on-these-things.md
 │   ├── 2021-11-02-182543-think-on-these-things.md
 │   ├── 2021-11-02-182648-think-on-these-things.md
 │   ├── 2021-11-02-182805-think-on-these-things.md
 │   ├── Krishnamurti - Think on These Things.md
 │   ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 │   └── Robert Henri - The Art Spirit.md
 │
 ├── [other-group]
 │    └── ...
 └── ...
```

And, if two templates share these values in their configurations:

```yaml
group: my-vault
structure: nested-grouped
```

The output will be:

```plaintext
[output-directory]
 ├── my-vault
 │   ├── Krishnamurti - Think on These Things
 │   │   ├── 2021-11-02-182319-think-on-these-things.md
 │   │   ├── 2021-11-02-182426-think-on-these-things.md
 │   │   ├── 2021-11-02-182543-think-on-these-things.md
 │   │   ├── 2021-11-02-182648-think-on-these-things.md
 │   │   ├── 2021-11-02-182805-think-on-these-things.md
 │   │   └── Krishnamurti - Think on These Things.md
 │   ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   │   ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │   │   └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 │   └── Robert Henri - The Art Spirit
 │       ├── 2021-11-02-180445-the-art-spirit.md
 │       ├── 2021-11-02-181250-the-art-spirit.md
 │       ├── 2021-11-02-181325-the-art-spirit.md
 │       ├── 2021-11-02-181510-the-art-spirit.md
 │       └── Robert Henri - The Art Spirit.md`
 │
 ├── [other-group]
 │    └── ...
 └── ...
```

[string-sanitization]: ./05-string-sanitization.md
[structure-modes]: ./02-03-structure-modes.md
