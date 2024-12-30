# iOS - Library Location

> <i class="fa fa-exclamation-circle"></i> The following information assumes that Apple Books
> syncing with iCloud Drive is disabled!

Apple Books for iOS stores its data in the `Books` directory on an iOS device.

```plaintext
[ios-device]
 │
 ├── Books
 │   ├── Managed/
 │   ├── MetadataStore/
 │   ├── Purchases/
 │   ├── Sync/
 │   ├── Books.plist ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ Books data
 │   ├── com.apple.ibooks-sync.plist ╌╌╌╌╌╌╌╌╌╌ Annotations data
 │   ├── 376FAA7E4CF81729.epub ╌╌╌╌╌╌╌╌╌╌╌╌╌┐
 │   ├── 669FEE1FFBB29D81.epub              ├╌╌ EPUBs
 │   ├── F788455723912C6D.epub ╌╌╌╌╌╌╌╌╌╌╌╌╌┘
 │   └── ...
 └── ...
```

Both the EPUBs and plists are stored in:

<!-- TODO: Where do PDFs and Audiobooks sit? -->

```plaintext
[ios-device]/Books
```

The books plist is located at:

```plaintext
[ios-device]/Books/Books.plist
```

The annotations plist is located at:

```plaintext
[ios-device]/Books/com.apple.ibooks-sync.plist
```

> <i class="fa fa-info-circle"></i> See [iOS - Access Library][ios-access-library] for more information.

[ios-access-library]: /apple-books/ios/access-library.md
