# macOS - Library Location

Apple Books for macOS stores its data primarily in two directories within the
`~/Library/Containers/` directory.

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

> <i class="fa fa-info-circle"></i> Note that the database names will vary
> therefore `***` is used in the filenames here.
