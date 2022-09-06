<!-- readstor
group: nested
output-mode: nested-grouped
render-context: book
nested-directory-template: "{{ book.author }} - {{ book.title }}"
filename-template-book: "{{ book.author }} - {{ book.title }}"
filename-template-annotation: "{{ annotation.metadata.created | date(format='%Y-%m-%d-%H%M%S') }}-{{ book.slug_title }}"
extension: md
-->

---
title: {{ book.title }}
author: {{ book.author }}
id: {{ book.metadata.id }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") }}
---

# [[ {{ links.book }} ]]

{% for _, link in links.annotations -%}
![[ {{ link }} ]]
{% endfor %}
