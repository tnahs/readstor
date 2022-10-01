# Options

## Databases

Setting a custom database directory:

The `databases` directory should contains the following structure as this is the
way Apple Books' `Documents` directory is set up. This is also how backup
directories are created when backing up databases with ReadStor.

```plaintext
[databases]
 │
 ├─ AEAnnotation
 │  ├─ AEAnnotation*.sqlite
 │  └─ ...
 │
 ├─ BKLibrary
 │  ├─ BKLibrary*.sqlite
 │  └─ ...
 └─ ...
```