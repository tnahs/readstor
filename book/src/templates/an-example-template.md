# An Example Template

The following is an example template along with its expected output and output structure. In fact,
it's almost identical to the default template that comes with ReadStor.

## Template Syntax

[Tera][tera], a Jinja-flavored templating language, provides a rich set of tools for building a wide
range of template complexities. The following is a brief intro to the syntax.

Values can be accessed by placing the desired attribute between double curly-braces:

```jinja2
{{ book.author }}
```

A list of items can be iterated over using a `for` loop:

```jinja2
{% for annotation in annotations %} ... {% endfor %}
```

Conditional behavior can be achieved by using `if`/`else` statements:

```jinja2
{% if annotation.notes %}notes: {{ annotation.notes }}{% endif %}
```

Values can be modified with `filters`. Here, a list of tags is concatenated with
a space as a delimiter.

```jinja2
{{ annotation.tags | join(sep=" ") }}
```

And finally comments can be added like so:

```jinja2
{# Hi! I'm a comment! #}
```

> <i class="fa fa-info-circle"></i> See Tera's [documentation][tera-documentation] to learn more
> about these and more advanced features, including template [inheritance][tera-inheritance], the
> [include][tera-include] tag, [macros][tera-macros] and the full list of available
> [filters][tera-filters].

## Template

Templates consist of two main sections, the configuration block written in YAML (and some
[Tera][tera]) and the template body written in [Tera][tera].

The configuration for this example template describes that this template, grouped under the name
`my-vault`, will render a single file for each book and place them all directly into the output
directory. Each output filename will follow the pattern of `Author - Title.md`.

```plaintext
<!-- readstor
group: my-vault
context: book
structure: flat
extension: md
names:
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

{% if annotation.notes %}notes: {{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}tags: {{ annotation.tags | join(sep=" ") }}{% endif %}

{% endfor %}
```

## Output Structure

The output structure is primarily determined by the `structure` and `context` keys. With the
`structure` set to `flat` all the output files will be placed inside the
[output directory][output-directory] with no structure. With the `context` set to `book`, a
single file will be created for each book and, if the template body requests, will contain all its
respective annotations. See [Context Modes][context-modes] and [Structure Modes][structure-modes]
for more information.

```plaintext
[output-directory]
 ├── Krishnamurti - Think on These Things.md
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md
 └── Robert Henri - The Art Spirit.md
```

## Output Rendered File

### `Krishnamurti - Think on These Things.md`

```markdown
---
title: Think on These Things
author: Krishnamurti
last-opened: 2021-11-02T18:30
---

# Krishnamurti - Think on These Things

---

Do you know what intelligence is? It is the capacity, surely, to think freely,
without fear, without a formula, so that you begin to discover for yourself what
is real, what is true; but if you are frightened you will never be intelligent.
Any form of ambition, spiritual or mundane, breeds anxiety, fear; therefore
ambition does not help to bring about a mind that is clear, simple, direct, and
hence intelligent.

tags: #education #intelligence #ambition #fear

---

To find out is not to come to a conclusion. I don’t know if you see the
difference. The moment you come to a conclusion as to what intelligence is, you
cease to be intelligent. That is what most of the older people have done: they
have come to conclusions. Therefore they have ceased to be intelligent. So you
have found out one thing right off: that an intelligent mind is one which is
constantly learning, never concluding.

tags: #learning #intelligence

---

The deeper the mind penetrates its own thought processes, the more clearly it
understands that all forms of thinking are conditioned; therefore the mind is
spontaneously very still—which does not mean that it is asleep. On the contrary,
the mind is then extraordinarily alert, no longer being drugged by mantrams, by
the repetition of words, or shaped by discipline. This state of silent alertness
is also part of awareness; and if you go into it still more deeply you will find
that there is no division between the person who is aware and the object of
which he is aware.

tags: #thinking
```

### `Richard P. Feynman - "Surely You're Joking, Mr. Feynman!".md`

```markdown
---
title: "Surely You're Joking, Mr. Feynman!"
author: Richard P. Feynman
last-opened: 2021-11-02T18:27
---

# Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"

---

After the dinner we went off into another room, where there were different
conversations going on. There was a Princess Somebody of Denmark sitting at a
table with a number of people around her, and I saw an empty chair at their
table and sat down.

She turned to me and said, “Oh! You’re one of the Nobel-Prize-winners. In what
field did you do your work?”

“In physics,” I said.

“Oh. Well, nobody knows anything about that, so I guess we can’t talk about it.”

“On the contrary,” I answered. “It’s because somebody knows something about it
that we can’t talk about physics. It’s the things that nobody knows anything
about that we can discuss. We can talk about the weather; we can talk about
social problems; we can talk about psychology; we can talk about international
finance—gold transfers we can’t talk about, because those are understood—so it’s
the subject that nobody knows anything about that we can all talk about!”

I don’t know how they do it. There’s a way of forming ice on the surface of the
face, and she did it! She turned to talk to somebody else.
```

### `Robert Henri - The Art Spirit.md`

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

[context-modes]: /templates/configuration/context-modes.md
[output-directory]: /intro/options/global.md#--output-directory-path
[structure-modes]: /templates/configuration/structure-modes.md
[tera]: https://tera.netlify.app/
[tera-documentation]: https://tera.netlify.app/docs/
[tera-filters]: https://tera.netlify.app/docs/#built-in-filters
[tera-include]: https://tera.netlify.app/docs/#include
[tera-inheritance]: https://tera.netlify.app/docs/#inheritance
[tera-macros]: https://tera.netlify.app/docs/#macros
