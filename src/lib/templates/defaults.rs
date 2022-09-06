//! Defines defaults for working with this templates.

#[allow(unused_imports)] // For docs.
use super::template::{OutputMode, RenderContext};

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
/// with [`RenderContext::Book`].
pub const FILENAME_TEMPLATE_BOOK: &str = "{{ book.author }} - {{ book.title }}";

/// The default template used to generate the output filename for a template
/// with [`RenderContext::Annotation`].
pub const FILENAME_TEMPLATE_ANNOTATION: &str =
    "{{ annotation.metadata.created | date(format='%Y-%m-%dT%H%M') }}-{{ book.slug_title }}";

/// The default template used to generate the directory name for a template with
/// [`OutputMode::Nested`] or [`OutputMode::NestedGrouped`].
pub const NESTED_DIRECTORY_TEMPLATE: &str = "{{ book.author }} - {{ book.title }}";
