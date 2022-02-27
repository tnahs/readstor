---
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") }}
tags: {{ annotation.tags | join(sep=" ") }}
id: {{ annotation.metadata.id }}
book-id: {{ annotation.metadata.book_id }}
created: {{ annotation.metadata.created | date(format="%Y-%m-%dT%H:%M")}}
modified: {{ annotation.metadata.modified | date(format="%Y-%m-%dT%H:%M") }}
location: {{ annotation.metadata.location }}
epubcfi: {{ annotation.metadata.epubcfi }}
---

# [[{{ book.title }}]] - [[{{ book.author }}]]

{{ annotation.body | join_paragraph }}

{% if annotation.notes %}{{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}{{ annotation.tags | join(sep=" ") }}{% endif -%}
