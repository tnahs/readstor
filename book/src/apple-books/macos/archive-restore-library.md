# macOS - Archive/Restore Library

> <i class="fa fa-info-circle"></i> Complete archive and restore scripts are available in
> [scripts/apple-books][scripts].

<!-- TODO: Add a paragraph on what the goal of archiving is. -->

## What To Archive/Restore?

Apple Books for macOS stores its data in two locations:

```plaintext
~/Library/Containers/com.apple.BKAgentService
~/Library/Containers/com.apple.iBooksX
```

> <i class="fa fa-info-circle"></i> See [macOS - Library Location][macos-library-location] for more
> information.

However, archiving and restoring only these directories might miss some metadata. Searching
`~/Library/Containers` for anything that contains `Books` yields some other directories:

```shell
find ~/Library/Containers -type d -name "*Books*"

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

Using `BK` as a proxy for `Books` yields no additional directories:

```shell
find ~/Library/Containers -type d -name "*BK*"

./com.apple.BKAgentService
./com.apple.iBooksX/Data/Documents/BKSeriesDatabase
./com.apple.iBooksX/Data/Documents/BKLibraryDataSourceDevelopment
./com.apple.iBooksX/Data/Documents/BKLibrary
./com.apple.iBooksX/Data/Documents/BKBookstore
```

So it's safe to assume any directory starting with `com.apple.Books` or `com.apple.BK` is important.
Therefore, these two globs along with Apple Books' `Group Container` should be used for archiving
and restoring:

```plaintext
~/Library/Containers/com.apple.BK*
~/Library/Containers/com.apple.iBooks*
~/Library/Group\ Containers/group.com.apple.iBooks
```

## <i class="fa fa-exclamation-circle"></i> Important Note

Archiving/restoring will work **only** if the path to the username **has not changed** since the
library was archived. Doing a few searches shows that the username has been hard-coded into some
files. This is most evident in the `BKLibrary-*.sqlite` file which contains absolute paths to
library files.

For example:

```shell
find ~/Library/Containers/com.apple.BKAgentService \
    -type f -a -exec grep -l --exclude=\*.{htm,html,xhtml} $USER {} +

.../.com.apple.containermanagerd.metadata.plist
.../Container.plist
.../Data/Documents/iBooks/Books/Books.plist
```

```shell
find ~/Library/Containers/com.apple.iBooksX* \
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

## Archive Library

> <i class="fa fa-info-circle"></i> The following snippet is taken from
> [scripts/apple-books/macos-archive.sh][script-macos-archive].

Archiving the library is as simple as running two `rsync` commands. This should save all the
relevant Apple Books data and metadata to a single directory. Make sure to replace
`[PATH-TO-ARCHIVE]` with a valid path to said directory.

```shell
rsync \
    --archive \
    --extended-attributes \
    $HOME/Library/Containers/com.apple.BK* \
    $HOME/Library/Containers/com.apple.iBooks* \
    [PATH-TO-ARCHIVE]/Containers

rsync \
    --archive \
    --extended-attributes \
    $HOME/Library/Group\ Containers/group.com.apple.iBooks \
    [PATH-TO-ARCHIVE]/Group\ Containers
```

For example, if `[PATH-TO-ARCHIVE]` is:

```plaintext
~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6
```

Our `rsync` commands would be:

```shell
rsync \
    --archive \
    --extended-attributes \
    $HOME/Library/Containers/com.apple.BK* \
    $HOME/Library/Containers/com.apple.iBooks* \
    ~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6/Containers

rsync \
    --archive \
    --extended-attributes \
    $HOME/Library/Group\ Containers/group.com.apple.iBooks \
    ~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6/Group\ Containers
```

And the resulting archive would resemble:

```plaintext
~/archives
  └── 2022-10-08--apple-books-v4.4-5177--macos-v12.6
      ├── Containers
      │   ├── com.apple.BKAgentService
      │   ├── com.apple.iBooks.BooksNotificationContentExtension
      │   ├── com.apple.iBooks.engagementExtension
      │   ├── com.apple.iBooks.iBooksSpotlightExtension
      │   ├── com.apple.iBooksX
      │   ├── com.apple.iBooksX-SecureUserDefaults
      │   ├── com.apple.iBooksX.BooksThumbnail
      │   ├── com.apple.iBooksX.CacheDelete
      │   ├── com.apple.iBooksX.DiskSpaceEfficiency
      │   └── com.apple.iBooksX.SharingExtension
      └── Group Containers
          └── group.com.apple.iBooks
```

# Restore Library

> <i class="fa fa-info-circle"></i> The following snippets are taken from
> [scripts/apple-books/macos-restore.sh][script-macos-restore].

> <i class="fa fa-exclamation-circle"></i> When restoring a library make sure that the current
> version of Apple Books for macOS and the version the archive was created from are identical. Not
> doing so could lead to unexpected results.

Restoring the library takes an extra step. First we need to clear out the current Apple Books
library. We can delete all the library files and directories by using the paths we determined from
[What To Archive/Restore?](#what-to-archiverestore).

```shell
rm -rf $HOME/Library/Containers/com.apple.BK*
rm -rf $HOME/Library/Containers/com.apple.iBooks*
rm -rf $HOME/Library/Group\ Containers/group.com.apple.iBooks
```

Finally, we can run the reverse `rsync` commands and restore the archive we previously made. Make
sure to replace `[PATH-TO-ARCHIVE]` with a valid path.

```shell
rsync \
    --archive \
    --extended-attributes \
    [PATH-TO-ARCHIVE]/Containers/ \
    $HOME/Library/Containers/

rsync \
    --archive \
    --extended-attributes \
    [PATH-TO-ARCHIVE]/Group\ Containers/ \
    $HOME/Library/Group\ Containers/
```

> <i class="fa fa-exclamation-circle"></i> The trailing forward-slash after `Containers` and
> `Group\Containers` here is important. It tells `rsync` to move the archive directory's _contents_
> into the target. Otherwise, it would move the archive _directory_ into the target.

For example, if `[PATH-TO-ARCHIVE]` is:

```plaintext
~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6
```

Our `rsync` command would be:

```shell
rsync \
    --archive \
    --extended-attributes \
    ~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6/Containers/ \
    $HOME/Library/Containers/  # Note the forward-slash! ---------------^

rsync \
    --archive \
    --extended-attributes \
    ~/archives/2022-10-08--apple-books-v4.4-5177--macos-v12.6/Group\ Containers/ \
    $HOME/Library/Group\ Containers/  # Note the forward-slash! ---------------^
```

[macos-library-location]: /apple-books/macos/library-location.md
[scripts]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/
[script-macos-restore]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/macos-restore.sh
[script-macos-archive]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/macos-archive.sh
