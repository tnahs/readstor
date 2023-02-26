# Global

The following options affect all [Commands][commands].

## `--output-directory <PATH>`

Set the output directory for all [Commands][commands].

Default: `~/.readstor`.

## `--databases-directory <PATH>`

Set the directory containing macOS's Apple Books databases

Default: `~/Library/Containers/com.apple.iBooksX/Data/Documents`

The databases directory should contain the databases for macOS's Apple Books.
These databases are: `AEAnnotation*.sqlite` and `BKLibrary*.sqlite`. The
directory should follow the following structure:

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

> <i class="fa fa-info-circle"></i> This can be useful when running ReadStor
> on databases backed-up with the [`backup`][backup] command. Note that the
> [`backup`][backup] command produces an output structure identical to this.
> So backing up and extracting data would require little effort.

## `--plists-directory <PATH>`

Set the directory containing iOS's Apple Books plists

> <i class="fa fa-exclamation-circle"></i> Experimental! Extracting data from
> Apple Books for iOS hasn't been tested as thoroughly as its macOS counterpart.
> Please submit an [issue][github-issues] if you run into any.

The plists directory should contain the `Books.plist` and
`com.apple.ibooks-sync.plist`. The directory should follow the following
structure:

```plaintext
[plists-directory]
 │
 ├── Books.plist
 ├── com.apple.ibooks-sync.plist
 └── ...
```

> <i class="fa fa-info-circle"></i> See
> [iOS - Library Location][ios-library-location] and
> [iOS - Access Library][ios-access-library] on how to retrieve these files.

## `--force`

Run even if Apple Books is currently running.

## `--quiet`

Silence output messages.

[backup]: ./01-commands.md#backup
[commands]: ./01-commands.md
[ios-library-location]: ../02-apple-books/02-01-library-location.md
[ios-access-library]: ../02-apple-books/02-01-access-library.md
[github-issues]: https://github.com/tnahs/readstor/issues
