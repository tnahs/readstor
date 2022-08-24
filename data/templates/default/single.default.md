---
title: { { book.title } }
author: { { book.author } }
last-opened: { { book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") } }
---

# [[{{ book.title }}]] - [[{{ book.author }}]]

{% for annotation in annotations -%}

---

{{ annotation.body | join_paragraph }}

{% if annotation.notes %}{{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}{{ annotation.tags | join(sep=" ") }}{% endif %}

{% endfor %}
