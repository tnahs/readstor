# Backup

The following options affect only the [`backup`][backup] commands.

## `--directory-template <TEMPLATE>`

Set the output directory template

|                |                                                            |
| -------------- | ---------------------------------------------------------- |
| Context        | [Backup Context](#backup-context)                          |
| Default        | `{{ now \| date(format='%Y-%m-%d-%H%M%S')}}-{{ version }}` |
| Example Output | `1970-01-01-120000-v0.1-0000`                              |

> <i class="fa fa-exclamation-circle"></i> Note that an escaping backslash `\`
> was required to nest a pipe `|` inside a markdown table. In other words, the
> default value does not contain a backslash.

### Backup Context

| Attribute | Type     | Description                     |
| --------- | -------- | ------------------------------- |
| `now`     | datetime | the current datetime            |
| `version` | string   | the current Apple Books version |

[backup]: ./01-commands.md#backup
