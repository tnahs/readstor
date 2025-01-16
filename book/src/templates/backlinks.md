# Backlinks

Generating backlinks for a Zettelkasten-like note-taking experience is relatively easy with
ReadStor. It requires two separate templates: One for rendering the book and one for rendering each
of its annotations.

> <i class="fa fa-info-circle"></i> See the [using-backlinks][using-backlinks] templates for a fully
> working example.

## Template Configuration

First, let's define the configurations for our two templates. They should be _almost_ identical to
one another.

- `group` is set to the same value across the two templates. This makes sure that if we select a
  grouped [Structure Mode][structure-modes], the templates will be rendered to the same directory.

- `context` is set to `book` for the book template and `annotation` for the annotation template.

- `structure` can be set to any of the four modes, however, `nested-grouped` feels the most
  appropriate as it would place each book into its own directory and place them all under a
  directory named after the value for the `group` key. See [Structure Modes][structure-modes] for
  more information.

- `extension` is set to `md` as the template will be outputting Markdown.

- `names` can be set to anything as long as the values are identical between the two templates. It
  might seem odd to see the values for `names` duplicated across the two templates. Shouldn't the
  book template define `names.book` and the annotation template define `names.annotation`? Ideally,
  yes. This need for duplication is the result of a current [limitation][names-limitations] of
  ReadStor, therefore they _must_ be identical, so the backlinks are correctly generated.

### Book Template Configuration

```yaml
group: my-vault
context: book # <- The only difference!
structure: nested-grouped
extension: md
names:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.slugs.metadata.created }}-{{ book.slugs.title }}"
  directory: "{{ book.author }} - {{ book.title }}"
```

### Annotation Template Configuration

```yaml
group: my-vault
context: annotation # <- The only difference!
structure: nested-grouped
extension: md
names:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.slugs.metadata.created }}-{{ book.slugs.title }}"
  directory: "{{ book.author }} - {{ book.title }}"
```

## Template Body

With our configuration all set up, we can now use the `names` object, which contains all the
rendered [Names][names], to link between our rendered output files.

> <i class="fa fa-info-circle"></i> See [Context Reference - Names][names] for more information.

### Book Template Body

The `names.annotations` object is a list of dictionaries, where each dictionary refers to a rendered
annotation file and contains its filename along with metadata about its respective annotation. This
allows us to link back to the annotation and sort the links based of off different criteria. In the
example below, [Tera][tera]'s [`sort`][tera-sort] filter is used with the `attribute` argument and
the `location` attribute.

```jinja2
# {{ book.author }} - {{ book.title }}

{% for name in names.annotations | sort(attribute="location")-%}
![[{{ name.filename }}]]
{% endfor %}
```

Alternatively we can use the `names.directory` variable to access the rendered name of the parent
directory. This value is only available if the [Structure Mode][structure-modes] is set to `nested`
or `nested-grouped`.

<!-- TODO(docs): Verify this works! -->

```jinja2
# {{ book.author }} - {{ book.title }}

{% for name in names.annotations -%}
![[{{ name.directory }}/{{ name.filename }}]]
{% endfor %}
```

### Annotation Template Body

Finally, using the `names.book` variable we're able to link back to the source book.

```jinja2
# [[{{ names.book }}]]

{{ annotation.body }}

{% if annotation.notes %}notes: {{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}tags: {{ annotation.tags | join(sep=" ") }}{% endif -%}
```

## Output Structure

```plaintext
[output-directory]
 ├── my-vault
 │   └── Robert Henri - The Art Spirit
 │       ├── 2021-11-02-180445-the-art-spirit.md
 │       ├── 2021-11-02-181250-the-art-spirit.md
 │       ├── 2021-11-02-181325-the-art-spirit.md
 │       ├── 2021-11-02-181510-the-art-spirit.md
 │       └── Robert Henri - The Art Spirit.md
 │
 ├── [group]
 │    └── ...
 └── ...
```

## Output Rendered Files

`Robert Henri - The Art Spirit.md`

```markdown
# Robert Henri - The Art Spirit

![[2021-11-02-180445-the-art-spirit.md]]
![[2021-11-02-181250-the-art-spirit.md]]
![[2021-11-02-181325-the-art-spirit.md]]
![[2021-11-02-181510-the-art-spirit.md]]
```

`2021-11-02-180445-the-art-spirit.md`

```markdown
# [[Robert Henri - The Art Spirit.md]]

Of course it is not easy to go one’s road. Because of our education we
continually get off our track, but the fight is a good one and there is joy in
it if there is any success at all. After all, the goal is not making art. It is
living a life. Those who live their lives will leave the stuff that is really
art. Art is a result. It is the trace of those who have led their lives. It is
interesting to us because we read the struggle and the degree of success the
man made in his struggle to live. The great question is: “What is worth while?”
The majority of people have failed to ask themselves seriously enough, and have
failed to try seriously enough to answer this question.
```

`2021-11-02-181250-the-art-spirit.md`

```markdown
# [[Robert Henri - The Art Spirit.md]]

We are not here to do what has already been done.
```

`2021-11-02-181325-the-art-spirit.md`

```markdown
# [[Robert Henri - The Art Spirit.md]]

The object of painting a picture is not to make a picture—however unreasonable
this may sound. The picture, if a picture results, is a by-product and may be
useful, valuable, interesting as a sign of what has past. The object, which is
back of every true work of art, is the attainment of a state of being, a state
of high functioning, a more than ordinary moment of existence. In such moments
activity is inevitable, and whether this activity is with brush, pen, chisel,
or tongue, its result is but a by-product of the state, a trace, the footprint
of the state.

tags: #artist #being
```

`2021-11-02-181510-the-art-spirit.m`

```markdown
# [[Robert Henri - The Art Spirit.md]]

Do not let the fact that things are not made for you, that conditions are not
as they should be, stop you. Go on anyway. Everything depends on those who go
on anyway.

tags: #inspiration
```

[names]: ./context-reference/names.md
[names-limitations]: ./configuration/names.md#limitations
[structure-modes]: ./configuration/structure-modes.md
[tera]: https://keats.github.io/tera/
[tera-sort]: https://keats.github.io/tera/docs/#sort
[using-backlinks]: https://github.com/tnahs/readstor/tree/main/templates/using-backlinks
