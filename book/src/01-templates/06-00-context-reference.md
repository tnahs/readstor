# Context Reference

Every template is injected with a "context" i.e. the data currently available
to rendering. ReadStor injects three different objects into every template
context: `book`, `annotation` (or `annotations` depending on the
[Context Mode][context-modes]) and `names`.

| Name          | Description                                                       |
| ------------- | ----------------------------------------------------------------- |
| `book`        | The current [Book][book] being rendered.                          |
| `annotation`  | A single [Annotation][annotation] belonging to the current book.  |
| `annotations` | Multiple [Annotations][annotation] belonging to the current book. |
| `names`       | A set of [Names][names] for generating backlinks between files.   |

[annotation]: ./06-02-annotation.md
[book]: ./06-01-book.md
[context-modes]: ../01-templates/02-02-context-modes.md
[names]: ./06-03-names.md
