<!-- readstor
group: nested
output-mode: nested-grouped
render-context: annotation
nested-directory-template: "{{ book.author }} - {{ book.title }}"
filename-template-book: "{{ book.author }} - {{ book.title }}"
filename-template-annotation: "{{ annotation.metadata.created | date(format='%Y-%m-%d-%H%M%S') }}-{{ book.slug_title }}"
extension: md
-->

---
title: {{ book.title }}
author: {{ book.author }}
tags: {{ annotation.tags | join(sep=" ") }}
id: {{ annotation.metadata.id }}
book-id: {{ annotation.metadata.book_id }}
created: {{ annotation.metadata.created | date(format="%Y-%m-%dT%H:%M") }}
modified: {{ annotation.metadata.modified | date(format="%Y-%m-%dT%H:%M") }}
location: {{ annotation.metadata.location }}
epubcfi: {{ annotation.metadata.epubcfi }}
---

# [[ {{ links.book }} ]]

{{ annotation.body }}

{% if annotation.notes %}{{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}{{ annotation.tags | join(sep=" ") }}{% endif -%}
