# Render

The following options affect only the [`render`][render] commands.

## `--templates-directory <PATH>`

Set a custom templates directory.

> <i class="fa fa-info-circle"></i> See the default [templates][templates] for fully working
> examples.

## `--template-group <GROUP>`

Render specified [Template Groups][template-groups].

> <i class="fa fa-exclamation-circle"></i> Passing non-existent [Template Groups][template-groups]
> will return an error.

Multiple [Template Groups][template-groups] can be passed using the following syntax.

```bash
readstor
    # ...
    --template-group basic
    --template-group using-backlinks
    # ..
```

[render]: ../commands.md#render
[template-groups]: ../../templates/configuration/template-groups.md
[templates]: https://github.com/tnahs/readstor/tree/main/templates
