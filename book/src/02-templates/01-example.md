# A Simple Example

The configuration for this template describes that this template, grouped under
the name `my-vault`, will render a single file for each book and place them all
directly into the output directory. Each output filename will follow the pattern
of `Author - Title.md`.

## Template

<!-- prettier-ignore -->
```plaintext
<!-- readstor
group: my-vault
context: book
structure: flat
extension: md
name-templates:
  book: "{{ book.author }} - {{ book.title }}"
-->

---
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") }}
---

# {{ book.author }} - {{ book.title }}

{% for annotation in annotations -%}

---

{{ annotation.body }}

{% if annotation.notes %}{{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}tags: {{ annotation.tags | join(sep=" ") }}{% endif %}

{% endfor %}
```

## Output Structure

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit.md
```

## Output

```plaintext
---
title: The Art Spirit
author: Robert Henri
last-opened: 2021-11-02T18:27
---

# Robert Henri - The Art Spirit

---

We are not here to do what has already been done.

---

The object of painting a picture is not to make a picture—however unreasonable
this may sound. The picture, if a picture results, is a by-product and may be
useful, valuable, interesting as a sign of what has past. The object, which is
back of every true work of art, is the attainment of a state of being, a state
of high functioning, a more than ordinary moment of existence. In such moments
activity is inevitable, and whether this activity is with brush, pen, chisel, or
tongue, its result is but a by-product of the state, a trace, the footprint of
the state.

tags: #artist #being

---

Of course it is not easy to go one’s road. Because of our education we
continually get off our track, but the fight is a good one and there is joy in
it if there is any success at all. After all, the goal is not making art. It is
living a life. Those who live their lives will leave the stuff that is really
art. Art is a result. It is the trace of those who have led their lives. It is
interesting to us because we read the struggle and the degree of success the man
made in his struggle to live. The great question is: “What is worth while?” The
majority of people have failed to ask themselves seriously enough, and have
failed to try seriously enough to answer this question.

---

Do not let the fact that things are not made for you, that conditions are not as
they should be, stop you. Go on anyway. Everything depends on those who go on
anyway.

tags: #inspiration
```
