# macOS - Library Location

> <i class="fa fa-exclamation-circle"></i> The following information assumes that Apple Books
> syncing with iCloud Drive is disabled!

Apple Books for macOS stores its data in two directories within `~/Library/Containers/`.

## Assets

EPUBs, PDFs, Audiobooks, etc. are stored in:

```plaintext
~/Library/Containers/com.apple.BKAgentService
```

## Databases

The books and annotations databases are stored in:

```plaintext
~/Library/Containers/com.apple.iBooksX
```

The books database is located at:

```plaintext
~/Library/Containers/com.apple.iBooksX/Data/Documents/BKLibrary/BKLibrary***.sqlite
```

The annotations database is located at:

```plaintext
~/Library/Containers/com.apple.iBooksX/Data/Documents/AEAnnotation/AEAnnotation***.sqlite
```

> <i class="fa fa-info-circle"></i> Note that the database names will vary therefore `***` is used
> in the filenames here.
