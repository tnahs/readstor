# Backup

The following options affect only the [`backup`][backup] commands.

## `--directory-template <TEMPLATE>`

Set the output directory template

|                |                                                             |
| -------------- | ----------------------------------------------------------- |
| Context        | [Backup Context](#backup-context)                           |
| Default        | `{{ now \| date(format='%Y-%m-%d-%H%M%S') }}-{{ version }}` |
| Example Output | `1970-01-01-120000-v0.1-0000`                               |

> <i class="fa fa-exclamation-circle"></i> Note that an escaping backslash `\` was required to
> nest a pipe `|` inside a markdown table. In other words, the default value does not contain
> a backslash.

For example, using the default template, the non-rendered ouput structure would look like the
following:

```plaintext
[ouput-directory]
 └── {{ now | date(format='%Y-%m-%d-%H%M%S') }}-{{ version }}
      ├── AEAnnotation
      │   ├── AEAnnotation*.sqlite
      │   └── ...
      └── BKLibrary
          ├── BKLibrary*.sqlite
          └── ...
```

And when rendered, the ouput structure would result in the following:

```plaintext
[ouput-directory]
 └── 2022-10-09-152506-v4.4-5177
     ├── AEAnnotation
     │   ├── AEAnnotation_v10312011_1727_local.sqlite
     │   ├── AEAnnotation_v10312011_1727_local.sqlite-shm
     │   └── AEAnnotation_v10312011_1727_local.sqlite-wal
     └── BKLibrary
         ├── BKLibrary-1-091020131601.sqlite
         ├── BKLibrary-1-091020131601.sqlite-shm
         └── BKLibrary-1-091020131601.sqlite-wal
```

### Backup Context

| Attribute | Type     | Description                                  |
| --------- | -------- | -------------------------------------------- |
| `now`     | datetime | the current datetime                         |
| `version` | string   | the current version of Apple Books for macOS |

[backup]: /intro/commands.md#backup
