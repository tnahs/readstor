# Backup/Restore Apple Books' Library

This guide assumes that Apple Books syncing in iCloud Drive is disabled and the
library exists on the local disk.

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

So it's good to assume any directory with the string `Books` or `BK` is
important. Therefore these two globs will be archived:

```plaintext
~/Library/Containers/com.apple.BK*
~/Library/Containers/com.apple.iBooks*
```

## Archiving The Library

Run the following `rsync` command:

```console
$ rsync \
    --verbose \
    --progress \
    --archive \
    --extended-attributes \
    $HOME/Library/Containers/com.apple.BK* \
    $HOME/Library/Containers/com.apple.iBooks* \
    [PATH_TO_ARCHIVE]
```

## Restoring The Library

Make sure Apple Books is closed or run:

```console
$ osascript -e 'tell application "Books" to quit'
```

Delete the current library using the same globs.

<!-- TODO: Add note about `Group Containers` -->

```console
$ rm -rf $HOME/Library/Containers/com.apple.BK*
$ rm -rf $HOME/Library/Containers/com.apple.iBooks*
$ rm -rf $HOME/Library/Group\ Containers/group.com.apple.iBooks
```

Extract the archive directly into `~/Library/Containers`.

The trailing slash after `[PATH_TO_ARCHIVE]` here is important. It tells `rsync`
to move the _contents_ of this directory into another. Otherwise it would move
the directory as a whole.

```console
$ rsync \
    --verbose \
    --progress \
    --archive \
    [PATH_TO_ARCHIVE]/ \
    $HOME/Library/Containers/
```

That's it! At this point the library should be fully restored.

## Important Note

This method will work **only** if the path to the user's home folder _has not
changed_. Doing a few quick searches through the Apple Books directories shows
that the path to the user's home folder has been hard-coded. This is most
evident in the `BKLibrary-***.sqlite` file which contains absolute paths to
library files.

If the username _must_ be changed, a search and replace can be attempted.
Whether this will work or not is unconfirmed.

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
