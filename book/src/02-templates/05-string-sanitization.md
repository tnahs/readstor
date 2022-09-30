# String Sanitization

To avoid any unexpected behavior, all user-provided strings are sanitized
before they are used for naming files or directories. The primary characters that
are removed are the colon `:`, the forward slash `/` and line breaks `\n`
`\r`.

This following values are sanitized:

- Template `group` names
- The rendered values from the `name-templates` key:
  - `name-templates.book`
  - `name-templates.annotation`
  - `name-templates.directory`
