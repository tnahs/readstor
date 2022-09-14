# Configuration

Every template must start with a configuration block:

```markdown
<!-- readstor
group: my-vault
context: book
structure: nested
extension: md
name-templates:
  book: "{{ book.author }} - {{ book.title }}"
  annotation: "{{ annotation.metadata.slug_created }}-{{ book.slug_title }}"
  directory: "{{ book.author }} - {{ book.title }}"
-->
```
