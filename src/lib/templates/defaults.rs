//! Defines defaults for working with this templates.

/// The opening tag for defining a config block in a template.
///
/// A template's config must be placed at the top of the file and placed
/// inside an HTML-flavored comment tag.
///
/// ```html
/// <!-- readstor
/// ...
/// -->
/// ```
pub const CONFIG_TAG_OPEN: &str = "<!-- readstor\n";

/// The closing tag for defining a config block in a template. See
/// [`CONFIG_TAG_OPEN`] for more information.
pub const CONFIG_TAG_CLOSE: &str = "\n-->\n";

/// The default template used to generate the output filename for a template
/// with [`ContextMode::Book`][book].
///
/// [book]: super::template::ContextMode::Book
pub const FILENAME_TEMPLATE_BOOK: &str = "{{ book.author }} - {{ book.title }}";

/// The default template used to generate the output filename for a template
/// with [`ContextMode::Annotation`][annotation].
///
/// [annotation]: super::template::ContextMode::Annotation
pub const FILENAME_TEMPLATE_ANNOTATION: &str =
    "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}";

/// The default template used to generate the directory name for a template with
/// [`StructureMode::Nested`][nested] or
/// [`StructureMode::NestedGrouped`][nested-grouped].
///
///
/// [nested]: super::template::StructureMode::Nested
/// [nested-grouped]: super::template::StructureMode::NestedGrouped
pub const DIRECTORY_TEMPLATE: &str = "{{ book.author }} - {{ book.title }}";
