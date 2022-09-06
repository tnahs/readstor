---
title: {{ book.title }}
author: {{ book.author }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") }}
---

# {{ book.author }} - {{ book.title }} 

{% for annotation in annotations -%}

---

{{ annotation.body }}

{% if annotation.notes %}{{ annotation.notes }}{% endif -%}
{%- if annotation.tags %}{{ annotation.tags | join(sep=" ") }}{% endif %}

{% endfor %}
