# Commands

## `render`

Render Apple Books' data via templates.

> <i class="fa fa-info-circle"></i> See [Templates][templates] for a full guide
> on creating templates.

> <i class="fa fa-info-circle"></i> See [Pre-process][pre-process],
> [Post-process][post-process] and [Render][render] options for available
> options.

## `export`

Export Apple Books' data as JSON.

> <i class="fa fa-info-circle"></i> See [Pre-process][pre-process] options
> for available options.

Outputs using the following structure:

```plaintext
[ouput-directory]
 ├── [author-title]
 │    ├── book.json
 │    └── annotations.json
 │
 ├── [author-title]
 │    └── ...
 └── ...
```

Example output structure:

```plaintext
[ouput-directory]
 ├── Krishnamurti - Think on These Things
 │   ├── annotations.json
 │   └── book.json
 ├── Richard P. Feynman - "Surely You're Joking, Mr. Feynman!"
 │   ├── annotations.json
 │   └── book.json
 └── Robert Henri - The Art Spirit
     ├── annotations.json
     └── book.json
```

Example `book.json`:

```json
{
  "title": "The Art Spirit",
  "author": "Robert Henri",
  "tags": [],
  "metadata": {
    "id": "1969AF0ECA8AE4965029A34316813924",
    "last_opened": "2021-11-02T18:27:04.781938076Z"
  },
  "slugs": {
    "title": "the-art-spirit",
    "author": "robert-henri"
  }
}
```

Example `annotations.json`:

```json
[
  {
    "body": "We are not here to do what has already been done.",
    "style": "purple",
    "notes": "",
    "tags": [],
    "metadata": {
      "id": "C932CE69-8584-4555-834C-797DF84E6825",
      "book_id": "1969AF0ECA8AE4965029A34316813924",
      "created": "2021-11-02T18:12:50.826642036Z",
      "modified": "2021-11-02T18:12:51.831905841Z",
      "location": "6.18.4.2.20.2.1:0",
      "epubcfi": "epubcfi(/6/18[Part09_Split0]!/4/2/20/2/1,:0,:49)",
      "slugs": {
        "created": "2021-11-02-181250",
        "modified": "2021-11-02-181250"
      }
    }
  },
  {
    "body": "The object of painting a picture...",
    "style": "yellow",
    "notes": "",
    "tags": ["#artist", "#being"],
    "metadata": {
      "id": "3FCC630A-55E6-4D6F-8E8F-DAD7C4E20A1C",
      "book_id": "1969AF0ECA8AE4965029A34316813924",
      "created": "2021-11-02T18:13:25.905355930Z",
      "modified": "2021-11-02T18:14:12.444134950Z",
      "location": "6.24.4.2.296.2.1:0",
      "epubcfi": "epubcfi(/6/24[Part09_Split3]!/4/2/296/2,/1:0,/7:257)",
      "slugs": {
        "created": "2021-11-02-181325",
        "modified": "2021-11-02-181325"
      }
    }
  },
  {
    "body": "Of course it is not easy to go one’s road...",
    "style": "blue",
    "notes": "",
    "tags": [],
    "metadata": {
      "id": "9D1B71B1-895C-446F-A03F-50C01146F532",
      "book_id": "1969AF0ECA8AE4965029A34316813924",
      "created": "2021-11-02T18:04:45.184863090Z",
      "modified": "2021-11-02T18:12:30.355533123Z",
      "location": "6.26.4.2.446.2.1:0",
      "epubcfi": "epubcfi(/6/26[Part09_Split4]!/4/2/446/2/1,:0,:679)",
      "slugs": {
        "created": "2021-11-02-180445",
        "modified": "2021-11-02-180445"
      }
    }
  },
  {
    "body": "Do not let the fact that things are not made for you...",
    "style": "green",
    "notes": "",
    "tags": ["#inspiration"],
    "metadata": {
      "id": "4620564A-0B64-4099-B5D6-6C9116A03AFF",
      "book_id": "1969AF0ECA8AE4965029A34316813924",
      "created": "2021-11-02T18:15:10.700510978Z",
      "modified": "2021-11-02T18:15:20.879488945Z",
      "location": "6.26.4.2.636.2.1:0",
      "epubcfi": "epubcfi(/6/26[Part09_Split4]!/4/2/636/2/1,:0,:166)",
      "slugs": {
        "created": "2021-11-02-181510",
        "modified": "2021-11-02-181510"
      }
    }
  }
]
```

> <i class="fa fa-info-circle"></i> This `export` was run with the
> [`--extract-tags`][extract-tags] option.

## `backup`

Back-up Apple Books' databases.

Outputs using the following structure:

```plaintext
[ouput-directory]
 └─ [YYYY-MM-DD-HHMMSS-VERSION]
     ├─ AEAnnotation
     │   ├─ AEAnnotation*.sqlite
     │   └─ ...
     └─ BKLibrary
         ├─ BKLibrary*.sqlite
         └─ ...
```

Example output:

```plaintext
[ouput-directory]
 └── 2022-10-09-152506-v4.4-5177
     ├── AEAnnotation
     │   ├── AEAnnotation_v10312011_1727_local.sqlite
     │   ├── AEAnnotation_v10312011_1727_local.sqlite-shm
     │   └── AEAnnotation_v10312011_1727_local.sqlite-wal
     └── BKLibrary
         ├── BKLibrary-1-091020131601.sqlite
         ├── BKLibrary-1-091020131601.sqlite-shm
         └── BKLibrary-1-091020131601.sqlite-wal
```

[extract-tags]: ./02-06-preprocess.md#--extract-tags
[post-process]: ./02-07-postprocess.md
[pre-process]: ./02-06-preprocess.md
[render]: ./02-02-render.md
[templates]: ../01-templates/00-templates.md
