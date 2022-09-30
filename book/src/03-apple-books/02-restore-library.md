# Restore Apple Books' Library

Make sure Apple Books is closed or run:

```console
osascript -e 'tell application "Books" to quit'
```

Delete the current library using the same globs.

<!-- TODO: Add note about `Group Containers` -->

```console
rm -rf $HOME/Library/Containers/com.apple.BK*
rm -rf $HOME/Library/Containers/com.apple.iBooks*
rm -rf $HOME/Library/Group\ Containers/group.com.apple.iBooks
```

Extract the archive directly into `~/Library/Containers`.

The trailing slash after `[PATH_TO_ARCHIVE]` here is important. It tells `rsync`
to move the _contents_ of this directory into another. Otherwise it would move
the directory as a whole.

```console
rsync \
    --verbose \
    --progress \
    --archive \
    [PATH_TO_ARCHIVE]/ \
    $HOME/Library/Containers/
```

That's it! At this point the library should be fully restored.
