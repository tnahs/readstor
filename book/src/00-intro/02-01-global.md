# Global

The following options affect all [Commands][commands].

## `--output-directory <PATH>`

Set the output directory for all [Commands][commands]. Defaults to `~/.readstor`.

## `--databases-directory <PATH>`

Set a custom databases directory.

This can be useful when running ReadStor on databases backed-up with the
[`backup`][backup] command. The output structure the [`backup`][backup] command
creates is identical to the required databases directory structure.

The databases directory should contain the following structure:

```plaintext
[databases-directory]
 │
 ├── AEAnnotation
 │   ├── AEAnnotation*.sqlite
 │   └── ...
 │
 ├── BKLibrary
 │   ├── BKLibrary*.sqlite
 │   └── ...
 └── ...
```

## `--force`

Run even if Apple Books is currently running.

## `--quiet`

Silence output messages.

[backup]: ./01-commands.md#backup
[commands]: ./01-commands.md
