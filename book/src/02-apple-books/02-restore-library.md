# Restore Library

> <i class="fa fa-info-circle"></i> The following snippets are taken from
> [scripts/apple-books/restore.sh][script].

Restoring the library takes an extra step. First we need to clear out
the current Apple Books library. We can delete all the library files and
directories by using the paths we determined from
[What To Archive/Restore?][what-to-archive-restore].

```shell
rm -rf $HOME/Library/Containers/com.apple.BK*
rm -rf $HOME/Library/Containers/com.apple.iBooks*
rm -rf $HOME/Library/Group\ Containers/group.com.apple.iBooks
```

Finally, we can run the reverse `rsync` commands and restore the archive we
previously made. Make sure to replace `[PATH-TO-ARCHIVE]` with a valid path.

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

> <i class="fa fa-exclamation-circle"></i> The trailing forward-slash after
> `Containers` and `Group\ Containers` here is important. It tells `rsync` to
> move the archive directory's _contents_ into the target. Otherwise, it would
> move the archive _directory_ into the target.

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

[what-to-archive-restore]: ./00-apple-books.md#what-to-archiverestore
[script]: https://github.com/tnahs/readstor/tree/main/scripts/apple-books/restore.sh
