# Partial Templates

When working with larger templates it can be helpful to separate them into smaller building blocks.
One way to do this is to create partial templates and `include` them in other templates.

Partial templates differ in two important ways from their normal template counterparts.

1. They _must_ begin with an underscore `_` as this is currently the only indicator of whether a
   template is partial or not.

2. They require no configuration block as they inherit their configuration from the template that
   `include`s them.

> <i class="fa fa-info-circle"></i> See the [using-partials][using-partials] templates for a fully
> working example.

## Including a Partial Template

Partial templates are included using the `include` tag:

```jinja2
{% include "_my-partial-template.md" %}
```

Where `"_my-partial-template.md"` is a path relative to the [templates
directory][templates-directory]. Therefore, this would be pointing to a template at the root of the
[templates directory][templates-directory]. If we wanted to organize our templates into different
directories we would have to add the directory names for any directories between the including
template and the [templates directory][templates-directory].

For example, with the following structure:

```plaintext
[templates-directory]
 └── my-vault-templates
     ├── _annotation.md
     ├── _book.md
     └── template.md
```

We would use the following `include` tags:

```jinja2
{% include "my-vault-templates/_book.jinja2" %}
{% include "my-vault-templates/_annotation.jinja2" %}
```

> <i class="fa fa-info-circle"></i> See the documentation for [Tera][tera]'s [include][tera-include]
> tag for more information on its features and limitations.

[templates-directory]: ../intro/options/render.md#--templates-directory-path
[tera]: https://keats.github.io/tera/
[tera-include]: https://keats.github.io/tera/docs/#include
[using-partials]: https://github.com/tnahs/readstor/tree/main/templates/using-partials
