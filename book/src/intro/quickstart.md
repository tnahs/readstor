# Quickstart

The following is an overview of some of the more common options.

## Default Directories

Running `readstor` with no arguments uses the following default directories:

- databases directory: `~/Library/Containers/com.apple.iBooksX/Data/Documents`
- output directory: `~/.readstor`

We can change the output directory using the [`--output-directory`][output-directory] option:

```bash
readstor [COMMAND] --output-directory "/path/to/output"
```

And change the databases directory using the [`--databases-directory`][databases-directory] option:

```bash
readstor [COMMAND] --databases-directory "/path/to/databases"
```

## Filter

[Filters][filter] can be used to run commands on a subset of annotations. _Note that all filters
are case-insensitive._

We can filter annotations where the book's title is _exactly_ `the art spirit`:

```bash
readstor [COMMAND] --filter "=title:the art spirit"
# '=' operator: exact -------^
```

Or, filter annotations where the book's author _contains_ `henri`:

```bash
readstor [COMMAND] --filter "author:henri"
# no operator: contains ----^
```

Or, filter annotations that contain the tag `#star`:

```bash
readstor [COMMAND] --filter "tag:#star" --extract-tags
```

We can alos combine filters:

```bash
readstor [COMMAND]                   \
    --filter "=title:the art spirit" \
    --filter "tag:#star"             \
    --extract-tags
```

## Custom Templates

A custom [templates directory][templates-directory] can be declared by using the
`--templates-directory` option. A template directory can contain any number of templates structured
in any way, therefore [template groups][template-groups] are used to name them and define relations
between them.

Render a single template group found inside a custom templates directory:

```bash
readstor render                                \
    --templates-directory "/path/to/templates" \
    --template-group "my-template-group"
```

In this example, `my-template-group` refers not to any file or directory name but rather the `group`
declared within any of the templates found inside the `/path/to/templates` directory.

For example `/path/to/templates/my-template.jinja2`:

```jinja
<!-- readstor
group: my-template-group
context: book
structure: flat
extension: md
-->

# {{ book.author }} - {{ book.title }}

{% for annotation in annotations -%}
{{ annotation.body }}

{% endfor %}
```

## Pre- and Post-processing

We can run pre- and post-processors on the data to modify it before completing the command.
Pre-processors are run on the raw data while post-processors are run on the output data. Therefore
pre-processors apply only to the `render` and `export` commands while post-processors only apply to
the `render` command.

Generate easily editable text by normalizing whitespace, converting any "smart" Unicode symbols to
ASCII and hard-wrapping the text to 100 characters.

```bash
readstor render            \
    --normalize-whitespace \
    --ascii-symbols        \
    --trim-blocks          \
    --wrap-text 100
```

> <i class="fa fa-info-circle"></i> See [normalize whitespace][normalize-whitespace], [ascii
> symbols][ascii-symbols], [trim blocks][trim-blocks], [wrap text][wrap-text] for more information.

## Complete Examples

### Render

Renders all annotations containing the `#star` tag using a template named `my-template-group` found
inside the `/path/to/templates` directory. Runs the pre-processes: `ascii-symbols`, `extract-tags`
and `normalize-whitespace`, and the post-processes `trim-blocks` and `wrap-text`. The resulting
files are output to `/path/to/output`.

```bash
readstor render                                \
    --output-directory "/path/to/output"       \
    --templates-directory "/path/to/templates" \
    --template-group "my-template-group"       \
    --filter "tag:#star"                       \
    --ascii-symbols                            \
    --extract-tags                             \
    --normalize-whitespace                     \
    --trim-blocks                              \
    --wrap-text 100
```

### Export

Exports all annotations. Runs the pre-processes: `ascii-symbols`, `extract-tags` and
`normalize-whitespace`. The resulting files are output to `/path/to/output` using a custom
[directory template][directory-template].

```bash
readstor export                                                  \
    --output-directory "/path/to/output"                         \
    --directory-template "{{ book.title }} by {{ book.author }}" \
    --ascii-symbols                                              \
    --extract-tags                                               \
    --normalize-whitespace
```

### Back-up

Backs-up macOS's Apple Books databases to `/path/to/output` using a custom [directory
template][directory-template].

```bash
readstor backup                          \
    --output-directory "/path/to/output" \
    --directory-template "{{ now | date(format='%Y-%m-%d') }}-v{{ version }}"
```

[ascii-symbols]: /intro/options/preprocess.md#--ascii-symbols
[backup]: /intro/commands.md#backup
[databases-directory]: /intro/options/global.md#--databases-directory-path
[directory-template]: /intro/options/export.html#--directory-template-template
[export]: /intro/commands.md#export
[extract-tags]: /intro/options/preprocess.md#--extract-tags
[filter]: /intro/options/filter.md
[normalize-whitespace]: /intro/options/preprocess.md#--normalize-whitespace
[output-directory]: /intro/options/global.md#--output-directory-path
[plists-directory]: /intro/options/global.md#--plists-directory-path
[render]: /intro/commands.md#render
[template-groups]: /intro/options/render.md#--templates-directory-path
[templates-directory]: /intro/options/render.md#--template-group-group
[templates]: https://github.com/tnahs/readstor/tree/main/templates
[trim-blocks]: /intro/options/postprocess.md#--trim-blocks
[wrap-text]: /intro/options/postprocess.md#--wrap-text-width
