# Template Groups

|              |         |
| ------------ | ------- |
| Name         | `group` |
| Type         | string  |
| Valid Values | any     |
| Required     | true    |
| Default      | n/a     |

Groups are used to identify multiple templates that are intended to be part of a
single output. The most common use case would be when using a pair templates:
one used to render a book and the other to render each of its annotations
separately. Conversely, they also provide a way to separate unrelated templates
rendered to the same output directory.

Grouping is triggered when one or more templates are set to either the
`flat-grouped` or the `nested-grouped` structure modes. The output files are
placed within a directory named after the `group`.

For example, if two templates share these fields in their configurations:

```yaml
group: my-vault
structure: flat-grouped
```

The output will be:

```plaintext
[output-directory]
 ├── my-vault
 │    ├── 2021-11-02-180445-the-art-spirit.md
 │    ├── 2021-11-02-181250-the-art-spirit.md
 │    ├── 2021-11-02-181325-the-art-spirit.md
 │    ├── 2021-11-02-181510-the-art-spirit.md
 │    ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │    ├── 2021-11-02-182319-think-on-these-things.md
 │    ├── 2021-11-02-182426-think-on-these-things.md
 │    ├── 2021-11-02-182543-think-on-these-things.md
 │    ├── 2021-11-02-182648-think-on-these-things.md
 │    ├── 2021-11-02-182805-think-on-these-things.md
 │    ├── Krishnamurti - Think on These Things.md
 │    ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 │    └── Robert Henri - The Art Spirit.md
 │
 ├── [group]
 │    └── ...
 └── ...
```

And, if two templates share these fields in their configurations:

```yaml
group: my-vault
structure: nested-grouped
```

The output will be:

```plaintext
[output-directory]
 ├── my-vault
 │    ├── Krishnamurti - Think on These Things
 │    │    ├── 2021-11-02-182319-think-on-these-things.md
 │    │    ├── 2021-11-02-182426-think-on-these-things.md
 │    │    ├── 2021-11-02-182543-think-on-these-things.md
 │    │    ├── 2021-11-02-182648-think-on-these-things.md
 │    │    ├── 2021-11-02-182805-think-on-these-things.md
 │    │    └── Krishnamurti - Think on These Things.md
 │    ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │    │    ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
 │    │    └── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 │    └── Robert Henri - The Art Spirit
 │         ├── 2021-11-02-180445-the-art-spirit.md
 │         ├── 2021-11-02-181250-the-art-spirit.md
 │         ├── 2021-11-02-181325-the-art-spirit.md
 │         ├── 2021-11-02-181510-the-art-spirit.md
 │         └── Robert Henri - The Art Spirit.md`
 │
 ├── [group]
 │    └── ...
 └── ...
```
