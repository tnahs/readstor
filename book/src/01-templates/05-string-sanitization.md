# String Sanitization

To avoid any unexpected behavior, certain user-provided strings are sanitized
before they are used for naming files or directories. Characters are either
removed or replaced with an underscore `_`.

| Character |           Removed           |          Replaced           |
| --------- | :-------------------------: | :-------------------------: |
| `\n`      | <i class="fa fa-check"></i> |                             |
| `\r`      | <i class="fa fa-check"></i> |                             |
| `\0`      | <i class="fa fa-check"></i> |                             |
| `:`       |                             | <i class="fa fa-check"></i> |
| `/`       |                             | <i class="fa fa-check"></i> |

This following values are sanitized:

- [Template Group][template-groups] names
- The rendered values from [Name Templates][name-templates]

[name-templates]: ./02-05-name-templates.md
[template-groups]: ./02-01-template-groups.md
