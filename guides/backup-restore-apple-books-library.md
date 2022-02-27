# Backup/Restore Apple Books' Library

> This guide assumes that Apple Books syncing in iCloud Drive is disabled and the library exists on the local disk. For information on how to do this see [Disabling Apple Books iCloud Drive](#disabling-apple-books-icloud-drive)

## What to Archive

Apple Books stores the bulk of its data in two locations:

1. The location of EPUBs, PDFs, Audiobooks, etc.:

    ```plaintext
    ~/Library/Containers/com.apple.BKAgentService
    ```

2. The location of the library's databases: `BKLibrary***.sqlite` and `AEAnnotation***.sqlite`:

    ```plaintext
    ~/Library/Containers/com.apple.iBooksX
    ```

However, only archiving and restoring these directories might miss some of the metadata.

```console
$ find ~/Library/Containers -type d -name "*Books*"

./com.apple.BKAgentService/Data/Documents/iBooks
./com.apple.BKAgentService/Data/Documents/iBooks/Books
./com.apple.iBooksX
./com.apple.iBooksX/Data/Documents/BCCloudKit-iBooks
./com.apple.iBooksX/Data/Documents/BCCloudAsset-iBooks
./com.apple.iBooksX/Data/Documents/BKBookstore
./com.apple.iBooksX/Data/Library/Caches/com.apple.iBooksX
./com.apple.iBooksX.CacheDelete
./com.apple.iBooksX.DiskSpaceEfficiency
./com.apple.iBooksX.SharingExtension
./com.apple.iBooksX-SecureUserDefaults
```

And using `BK` as a proxy for `Books`:

```console
$ find ~/Library/Containers -type d -name "*BK*"

./com.apple.BKAgentService
./com.apple.iBooksX/Data/Documents/BKSeriesDatabase
./com.apple.iBooksX/Data/Documents/BKLibraryDataSourceDevelopment
./com.apple.iBooksX/Data/Documents/BKLibrary
./com.apple.iBooksX/Data/Documents/BKBookstore
```

So it's good to assume any directory with the string `Books` or `BK` is important. Therefore these two globs will be archived:

```plaintext
~/Library/Containers/com.apple.BK*
~/Library/Containers/com.apple.iBooks*
```

## Archiving The Library

Run the following `tar` command:

```console
$ tar \
    --create \
    --gzip \
    --file=$PATH_TO_ARCHIVE \
    --directory=$HOME/Library/Containers \
    com.apple.BK* \
    com.apple.iBooksX*
```

> "Why `tar`?"
>
> After some research, it was the only way to archive a full directory while preserving symlinks and xattrs (Extended Attributes). It's important the archive maintain as much metadata as possible.

This will create an archive relative to `~/Library/Containers` i.e.:

```plaintext
[ARCHIVE_NAME]
├── com.apple.BKAgentService
│   ├── ...
│   └── ...
├── com.apple.iBooksX
│   ├── ...
│   └── ...
└── com.apple.*
    ├── ...
    └── ...
```

## Restoring The Library

Make sure Apple Books is closed or run:

```console
$ osascript -e 'tell application "Books" to quit'
```

Delete the current library using the same globs.

<!-- TODO Add note about `Group Containers` -->

```console
$ rm -rf $HOME/Library/Containers/com.apple.BK*
$ rm -rf $HOME/Library/Containers/com.apple.iBooks*
$ rm -rf $HOME/Library/Group\ Containers/group.com.apple.iBooks
```

Extract the archive directly into `~/Library/Containers`.

```console
$ tar \
    --extract \
    --file=$PATH_TO_ARCHIVE \
    --directory=$HOME/Library/Containers
```

That's it! At this point the library should be fully restored.

## Important Note

> This method will **only** work if the path to the user's home folder *does not change*. Doing a few quick searches through the Apple Books directories shows that the path to the user's home folder has been hard-coded. This is most evident in the `BKLibrary-***.sqlite` file which contains absolute paths to library files.
>
> If the username *must* be changed, a search and replace can be attempted. Whether this will work or not is unconfirmed as of 2020-12-30.

```console
$ find ~/Library/Containers/com.apple.BKAgentService -type f \
    -a -exec grep -l --exclude=\*.{htm,html,xhtml} $USER {} +

.../.com.apple.containermanagerd.metadata.plist
.../Container.plist
.../Data/Documents/iBooks/Books/Books.plist
```

```console
$ find ~/Library/Containers/com.apple.iBooksX* -type f \
    -a -exec grep -l --exclude=\*.{htm,html,xhtml} $USER {} +

.../com.apple.iBooksX/.com.apple.containermanagerd.metadata.plist
.../com.apple.iBooksX/Container.plist
.../com.apple.iBooksX/Data/Documents/BKLibrary/BKLibrary-1-091020131601.sqlite-wal
.../com.apple.iBooksX/Data/Documents/BKLibrary/BKLibrary-1-091020131601.sqlite
.../com.apple.iBooksX-SecureUserDefaults/.com.apple.containermanagerd.metadata.plist
.../com.apple.iBooksX-SecureUserDefaults/Container.plist
.../com.apple.iBooksX.CacheDelete/.com.apple.containermanagerd.metadata.plist
.../com.apple.iBooksX.CacheDelete/Container.plist
.../com.apple.iBooksX.DiskSpaceEfficiency/.com.apple.containermanagerd.metadata.plist
.../com.apple.iBooksX.DiskSpaceEfficiency/Container.plist
.../com.apple.iBooksX.SharingExtension/.com.apple.containermanagerd.metadata.plist
.../com.apple.iBooksX.SharingExtension/Container.plist
```

## Disabling Apple Books iCloud Drive

### Disable Sync in Apple Books

- In Apple Books
    - Go to `Books` > `Preferences...`
        - In the `General` tab
            - Un-check `Sync collections, bookmarks, and highlights across devices`

### Disable and Clear iCloud Drive

- `System Preferences` > `Apple ID`
    - In the `iCloud` tab
         1. Un-check `iCloud Drive`
         2. Click `Manage...` at the bottom
            1. Select `Apple Books`
            2. Click `Delete all Files...` then `Delete`
