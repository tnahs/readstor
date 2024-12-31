# String Sanitization

To avoid any unexpected behavior, certain user-provided strings are sanitized before they are used
for naming files or directories. Characters are either removed or replaced with an underscore `_`.

| Character |           Removed           |          Replaced           |
| --------- | :-------------------------: | :-------------------------: |
| `\n`      | <i class="fa fa-check"></i> |                             |
| `\r`      | <i class="fa fa-check"></i> |                             |
| `\0`      | <i class="fa fa-check"></i> |                             |
| `:`       |                             | <i class="fa fa-check"></i> |
| `/`       |                             | <i class="fa fa-check"></i> |

This following values are sanitized:

- [Template Group][template-groups] set in the `group` config key.
- The rendered values from [Names][names] set in the `names` config key.

<!-- TODO: Provide some examples. -->

[names]: /templates/configuration/names.md
[template-groups]: /templates/configuration/template-groups.md
