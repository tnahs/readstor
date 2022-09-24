//! Defines defaults for working with this templates.

#[allow(unused_imports)] // For docs.
use super::template::{ContextMode, StructureMode};

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
pub const CONFIG_TAG_OPEN: &str = "<!-- readstor";

/// The closing tag for defining a config block in a template. See
/// [`CONFIG_TAG_OPEN`] for more information.
pub const CONFIG_TAG_CLOSE: &str = "-->\n";

/// The default template used to generate the output filename for a template
/// with [`ContextMode::Book`].
pub const FILENAME_TEMPLATE_BOOK: &str = "{{ book.author }} - {{ book.title }}";

/// The default template used to generate the output filename for a template
/// with [`ContextMode::Annotation`].
pub const FILENAME_TEMPLATE_ANNOTATION: &str =
    "{{ annotation.metadata.slugs.created }}-{{ book.slugs.title }}";

/// The default template used to generate the directory name for a template with
/// [`StructureMode::Nested`] or [`StructureMode::NestedGrouped`].
pub const DIRECTORY_TEMPLATE: &str = "{{ book.author }} - {{ book.title }}";
