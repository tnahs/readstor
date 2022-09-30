# Apple Books

> <i class="fa fa-exclamation-circle"></i> This guide assumes that Apple Books
> syncing in iCloud Drive is disabled!

## <i class="fa fa-exclamation-circle"></i> Important Note

This method will work **only** if the path to the user's home folder _has not
changed_ from when the library was backed up to when its restored.

Doing a few quick searches through the Apple Books directories shows that the
path to the user's home folder has been hard-coded. This is most evident in the
`BKLibrary-***.sqlite` file which contains absolute paths to files.

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
