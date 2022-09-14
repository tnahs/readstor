# Render

`render` using `single` Render Mode

```plaintext
[output]
 │
 ├─ default ── (omitted if a custom template is used)
 │   ├─ Author - Title.[extension]
 │   ├─ Author - Title.[extension]
 │   └─ ...
 │
 ├─ [template-name]
 │   ├─ Author - Title.[extension]
 │   ├─ Author - Title.[extension]
 │   └─ ...
 │
 ├─ Author - Title
 │   └─ ...
 └─ ...
```

`render` using `multi` Render Mode

```plaintext
[output]
 │
 ├─ default ── (omitted if a custom template is used)
 │   │
 │   ├─ Author - Title
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[extension]
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[extension]
 │   │   └─ ...
 │   │
 │   ├─ Author - Title
 │   │   └─ ...
 │   └─ ...
 │
 ├─ [template-name]
 │   │
 │   ├─ Author - Title
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[extension]
 │   │   ├─ [YYYY-MM-DD-HHMMSS]-[title].[extension]
 │   │   └─ ...
 │   │
 │   ├─ Author - Title
 │   │   └─ ...
 │   └─ ...
 └─ ...
```
