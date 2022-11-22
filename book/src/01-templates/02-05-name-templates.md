# Name Templates

| Key                         | Context      |
| --------------------------- | ------------ |
| `name-templates.book`       | `book`       |
| `name-templates.annotation` | `annotation` |
| `name-templates.directory`  | `book`       |

Output files and directory names can be customized using the same [Tera][tera]
syntax. ReadStor will inject a different context into each `name-template`
during render time and set the template's output file/directory name to the
resulting string.

> <i class="fa fa-exclamation-circle"></i> The rendered book, annotation and
> directory names are sanitized to make sure they interact well with the file
> system. See [String Sanitization][string-sanitization] for more information.

Additionally, all the rendered values from these keys are available within the
template's context under the `links` variable regardless of the current context
mode. See [Context Reference - Links][links] for more information.

> <i class="fa fa-exclamation-circle"></i> Note that the template's `context`
> matters when setting `name-templates`.
>
> For example, if the template's `context` is set to `book`:
>
> ```yaml
> group: my-vault
> structure: flat
> context: book # <- Here!
> extension: md
> name-templates:
>   book: "{{ book.author }} - {{ book.title }}"
>   annotation: "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}"
> ```
>
> ReadStor will generate a single file for each book and name them using the
> rendered result of `name-templates.book`. The resulting output would be:
>
> ```plaintext
> [output-directory]
>  ├── Krishnamurti - Think on These Things.md
>  ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
>  ├── Robert Henri - The Art Spirit.md
>  └── ...
> ```
>
> However if the `context` is changed to `annotation`:
>
> ```yaml
> group: my-vault
> structure: flat
> context: annotation # <- Here!
> extension: md
> name-templates:
>   book: "{{ book.author }} - {{ book.title }}"
>   annotation: "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}"
> ```
>
> ReadStor will now generate a single file for each annotation in each book
> and name them using the rendered result of `name-templates.annotation`. The
> resulting output would be:
>
> ```plaintext
> [output-directory]
>  ├── 2021-11-02-180445-the-art-spirit.md
>  ├── 2021-11-02-181250-the-art-spirit.md
>  ├── 2021-11-02-181325-the-art-spirit.md
>  ├── 2021-11-02-181510-the-art-spirit.md
>  ├── 2021-11-02-182059-surely-youre-joking-mr-feynman.md
>  ├── 2021-11-02-182319-think-on-these-things.md
>  ├── 2021-11-02-182426-think-on-these-things.md
>  ├── 2021-11-02-182543-think-on-these-things.md
>  ├── 2021-11-02-182648-think-on-these-things.md
>  ├── 2021-11-02-182805-think-on-these-things.md
>  └── ...
> ```

## Book Name Template

Defines the filename template to use when the parent template's `context` mode
is set to `book`. This template only has access to the `book` context when
its rendered.

|              |                                        |
| ------------ | -------------------------------------- |
| Name         | `name-templates.book`                  |
| Type         | string                                 |
| Valid Values | any                                    |
| Required     | No                                     |
| Default      | `{{ book.author }} - {{ book.title }}` |

## Annotation Name Template

Defines the filename template to use when the parent template's `context` mode
is set to `annotation`. This template has access to the `book` and `annotation`
context when its rendered.

|              |                                                                  |
| ------------ | ---------------------------------------------------------------- |
| Name         | `name-templates.annotation`                                      |
| Type         | string                                                           |
| Valid Values | any                                                              |
| Required     | No                                                               |
| Default      | `{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}` |

## Directory Name Template

Defines the directory name template to use when the parent template's
`structure` mode is set to `nested` or the `nested-grouped`. This template only
has access to the `book` context when its rendered.

|              |                                        |
| ------------ | -------------------------------------- |
| Name         | `name-templates.directory`             |
| Type         | string                                 |
| Valid Values | any                                    |
| Required     | No                                     |
| Default      | `{{ book.author }} - {{ book.title }}` |

## <i class="fa fa-exclamation-circle"></i> Limitations

Why does a single template have both a `name-templates.book` and
`name-templates.annotation` key?

This is mainly the result of a current limitation with ReadStor. Templates
that are part of the same `group` have no relation internally. ReadStor just
renders each template it finds. If multiple templates share the same value for
`group` then they end up in the same directory when the `structure` mode is set
to `grouped` or `nested-grouped`.

As a result of this limitation, if a template requires awareness of its
`group`'s sibling template names i.e. [`links`][links], they _must_ be defined
in each template. This results in some duplication across templates. This
should hopefully be resolved in future versions of ReadStor.

> <i class="fa fa-info-circle"></i> For more information on how to generate and
> use `links` see [Context Reference - Links][links] and [Backlinks][backlinks].

[backlinks]: ./04-backlinks.md
[context-reference]: ./06-context-reference.md
[links]: ./06-03-links.md
[string-sanitization]: ./05-string-sanitization.md
[tera]: https://tera.netlify.app/
