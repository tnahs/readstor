---
title: {{ book.title }}
author: {{ book.author }}
id: {{ book.metadata.id }}
last-opened: {{ book.metadata.last_opened | date(format="%Y-%m-%dT%H:%M") }}
---

# {{ book.title }} - {{ book.author }}
