# Archive Library

> <i class="fa fa-info-circle"></i> The following snippet is taken from
> [scripts/apple-books/archive.sh][script].

Archiving the library is as simple as running two `rsync` commands. This should
save all the relevant Apple Books data and metadata to a single directory. Make
sure to replace `[PATH-TO-ARCHIVE]` with a valid path to said directory.

```sh
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

```sh
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

[script]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/archive.sh
