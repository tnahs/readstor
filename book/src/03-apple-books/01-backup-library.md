# Backup Apple Books' Library

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
rsync \
    --verbose \
    --progress \
    --archive \
    --extended-attributes \
    $HOME/Library/Containers/com.apple.BK* \
    $HOME/Library/Containers/com.apple.iBooks* \
    [PATH_TO_ARCHIVE]
```
