# Apple Books

> <i class="fa fa-exclamation-circle"></i> The following information assumes
> that Apple Books syncing with iCloud Drive is disabled!

> <i class="fa fa-info-circle"></i> Complete archive and restore scripts are
> available in [scripts/apple-books][scripts].

## What To Archive/Restore?

Apple Books stores the bulk of its data in two locations:

1. The location of EPUBs, PDFs, Audiobooks, etc.:

   ```plaintext
   ~/Library/Containers/com.apple.BKAgentService
   ```

2. The location of the library's databases:

   ```plaintext
   ~/Library/Containers/com.apple.iBooksX
   ```

However, only archiving and restoring these directories might miss some
metadata. Searching `~/Library/Containers` for anything that contains `Books`
yields some other directories:

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

Using `BK` as a proxy for `Books` yields no additional directores:

```console
$ find ~/Library/Containers -type d -name "*BK*"

./com.apple.BKAgentService
./com.apple.iBooksX/Data/Documents/BKSeriesDatabase
./com.apple.iBooksX/Data/Documents/BKLibraryDataSourceDevelopment
./com.apple.iBooksX/Data/Documents/BKLibrary
./com.apple.iBooksX/Data/Documents/BKBookstore
```

So it's safe to assume any directory starting with `com.apple.Books` or
`com.apple.BK` is important. Therefore, these two globs along with Apple Books'
`Group Container` should be used for archiving and restoring:

```plaintext
~/Library/Containers/com.apple.BK*
~/Library/Containers/com.apple.iBooks*
~/Library/Group\ Containers/group.com.apple.iBooks
```

## <i class="fa fa-exclamation-circle"></i> Important Note

Archiving/restoring will work **only** if the path to the username **has not
changed** since the library was archived. Doing a few searches shows that the
username has been hard-coded into some files. This is most evident in the
`BKLibrary-*.sqlite` file which contains absolute paths to library files.

For example:

```console
$ find ~/Library/Containers/com.apple.BKAgentService \
    -type f -a -exec grep -l --exclude=\*.{htm,html,xhtml} $USER {} +

.../.com.apple.containermanagerd.metadata.plist
.../Container.plist
.../Data/Documents/iBooks/Books/Books.plist
```

```console
$ find ~/Library/Containers/com.apple.iBooksX* \
    -type f -a -exec grep -l --exclude=\*.{htm,html,xhtml} $USER {} +

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

[scripts]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/
