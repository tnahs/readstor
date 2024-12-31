//! Defines types to represent the output file/directory names of rendered
//! templates.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::contexts::annotation::AnnotationContext;
use crate::contexts::book::BookContext;
use crate::contexts::entry::EntryContext;
use crate::models::datetime::DateTimeUtc;
use crate::render::template::TemplateRaw;
use crate::result::Result;
use crate::utils;

/// A struct representing the raw template strings for generating output file and directory names.
#[derive(Debug, Clone, Deserialize)]
pub struct Names {
    /// The default template used when generating an output filename for the template when its
    /// context mode is [`ContextMode::Book`][book].
    ///
    /// [book]: crate::render::template::ContextMode::Book
    #[serde(default = "Names::default_book")]
    pub book: String,

    /// The default template used when generating an output filename for the template when its
    /// context mode is [`ContextMode::Annotation`][annotation].
    ///
    /// [annotation]: crate::render::template::ContextMode::Annotation
    #[serde(default = "Names::default_annotation")]
    pub annotation: String,

    /// The default template used when generating a nested output directory for the
    /// template when its structure mode is either [`StructureMode::Nested`][nested] or
    /// [`StructureMode::NestedGrouped`][nested-grouped].
    ///
    /// [nested]: crate::render::template::StructureMode::Nested
    /// [nested-grouped]: crate::render::template::StructureMode::NestedGrouped
    #[serde(default = "Names::default_directory")]
    pub directory: String,
}

impl Default for Names {
    fn default() -> Self {
        Self {
            book: Self::default_book(),
            annotation: Self::default_annotation(),
            directory: Self::default_directory(),
        }
    }
}

impl Names {
    /// Returns the default template for a book's filename.
    fn default_book() -> String {
        super::defaults::FILENAME_TEMPLATE_BOOK.to_owned()
    }

    /// Returns the default template for an annotation's filename.
    fn default_annotation() -> String {
        super::defaults::FILENAME_TEMPLATE_ANNOTATION.to_owned()
    }

    /// Returns the default template for a directory.
    fn default_directory() -> String {
        super::defaults::DIRECTORY_TEMPLATE.to_owned()
    }
}

/// A struct representing the rendered template strings for all the output file and directory names
/// for a given template.
///
/// This is used to (1) name files and directories when rendering templates to disk and (2) is
/// included in the template's context so that files/direcories related to the template can be
/// references within the tenplate.
///
/// See [`Templates::render()`][render] for more information.
///
/// [render]: crate::render::templates::Templates::render()
#[derive(Debug, Default, Clone, Serialize)]
pub struct NamesRender {
    /// The output filename for a template with [`ContextMode::Book`][book].
    ///
    /// [book]: crate::render::template::ContextMode::Book
    pub book: String,

    /// The output filenames for a template with [`ContextMode::Annotation`][annotation].
    ///
    /// Internally this field is stored as a `HashMap` but is converted into a `Vec` before it's
    /// injected into a template.
    ///
    /// [annotation]: crate::render::template::ContextMode::Annotation
    #[serde(serialize_with = "utils::serialize_hashmap_to_vec")]
    pub annotations: HashMap<String, AnnotationNameAttributes>,

    /// The directory name for a template with [`StructureMode::Nested`][nested] or
    /// [`StructureMode::NestedGrouped`][nested-grouped].
    ///
    /// [nested]: crate::render::template::StructureMode::Nested
    /// [nested-grouped]: crate::render::template::StructureMode::NestedGrouped
    pub directory: String,
}

impl NamesRender {
    /// Creates a new instance of [`NamesRender`].
    ///
    /// Note that all names are generated regardless of the template's [`ContextMode`][context-mode].
    /// For example, when a separate template is used to render a [`Book`][book] and another for its
    /// [`Annotation`][annotation]s, it's important that both templates have access to the other's
    /// filenames so they can link to one another if the user desires.
    ///
    /// # Arguments
    ///
    /// * `entry` - The context injected into the filename templates.
    /// * `template` - The template containing the filename templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any templates have syntax errors or are referencing non-existent fields
    /// in their respective contexts.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    /// [book]: crate::models::book::Book
    /// [context-mode]: crate::render::template::ContextMode
    pub fn new(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<Self> {
        Ok(Self {
            book: Self::render_book_filename(entry, template)?,
            annotations: Self::render_annotation_filenames(entry, template)?,
            directory: Self::render_directory_name(entry, template)?,
        })
    }

    /// Returns the rendered annotation filename based on its id.
    ///
    /// # Arguments
    ///
    /// * `annotation_id` - The annotation's id.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_annotation_filename(&self, annotation_id: &str) -> String {
        self.annotations
            .get(annotation_id)
            // This should theoretically never fail as the `NamesRender` instance is created from
            // the `Entry`. This means they contain the same exact keys and it should therefore be
            // safe to unwrap. An error here would be critical and should fail.
            .expect("`NamesRender` instance missing `Annotation` present in `Entry`")
            .filename
            .clone()
    }

    /// Renders the filename for a template with [`ContextMode::Book`][context-mode].
    ///
    /// # Arguments
    ///
    /// * `entry` - The context to inject into the template.
    /// * `template` - The template to render.
    ///
    /// [context-mode]: crate::render::template::ContextMode::Book
    fn render_book_filename(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<String> {
        let context = NamesContext::book(&entry.book, &entry.annotations);

        let filename = utils::render_and_sanitize(&template.names.book, context)?;
        let filename = utils::build_and_sanitize_filename(&filename, &template.extension);

        Ok(filename)
    }

    /// Renders the filename for a template with [`ContextMode::Annotation`][context-mode].
    ///
    /// # Arguments
    ///
    /// * `entry` - The context to inject into the template.
    /// * `template` - The template to render.
    ///
    /// [context-mode]: crate::render::template::ContextMode::Annotation
    fn render_annotation_filenames(
        entry: &EntryContext<'_>,
        template: &TemplateRaw,
    ) -> Result<HashMap<String, AnnotationNameAttributes>> {
        let mut annotations = HashMap::new();

        for annotation in &entry.annotations {
            let context = NamesContext::annotation(&entry.book, annotation);

            let filename = utils::render_and_sanitize(&template.names.annotation, context)?;
            let filename = utils::build_and_sanitize_filename(&filename, &template.extension);

            annotations.insert(
                annotation.metadata.id.clone(),
                AnnotationNameAttributes::new(annotation, filename),
            );
        }

        Ok(annotations)
    }

    /// Renders the directory name for a template with [`StructureMode::Nested`][nested] or
    /// [`StructureMode::NestedGouped`][nested-grouped].
    ///
    /// # Arguments
    ///
    /// * `entry` - The context to inject into the template.
    /// * `template` - The template to render.
    ///
    /// [nested]: crate::render::template::StructureMode::Nested
    /// [nested-grouped]: crate::render::template::StructureMode::NestedGrouped
    fn render_directory_name(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<String> {
        let context = NamesContext::directory(&entry.book);

        utils::render_and_sanitize(&template.names.directory, context)
    }
}

/// A struct representing the rendered filename for a template with
/// [`ContextMode::Annotation`][context-mode] along with a set of attributes used for sorting within
/// a template.
///
/// For example:
///
/// ```jinja
/// {% for name in names.annotations | sort(attribute="location") -%}
/// ![[{{ name.filename }}]]
/// {% endfor %}
/// ```
/// See [`AnnotationMetadata`][annotation-metadata] for undocumented fields.
///
/// [annotation-metadata]: crate::models::annotation::AnnotationMetadata
/// [context-mode]: crate::render::template::ContextMode::Annotation
#[derive(Debug, Default, Clone, Serialize)]
pub struct AnnotationNameAttributes {
    /// The rendered filename for a template with
    /// [`ContextMode::Annotation`][context-mode].
    ///
    /// [context-mode]: crate::render::template::ContextMode
    pub filename: String,
    #[allow(missing_docs)]
    pub created: DateTimeUtc,
    #[allow(missing_docs)]
    pub modified: DateTimeUtc,
    #[allow(missing_docs)]
    pub location: String,
}

impl AnnotationNameAttributes {
    /// Creates a new instance of [`AnnotationNameAttributes`].
    fn new(annotation: &AnnotationContext<'_>, filename: String) -> Self {
        Self {
            filename,
            created: annotation.metadata.created,
            modified: annotation.metadata.modified,
            location: annotation.metadata.location.clone(),
        }
    }
}

/// An enum representing the different template contexts for rendering file and directory names.
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum NamesContext<'a> {
    /// The context when rendering a filename for a template with [`ContextMode::Book`][context-mode].
    ///
    /// [context-mode]: crate::render::template::ContextMode::Book
    Book {
        book: &'a BookContext<'a>,
        annotations: &'a [AnnotationContext<'a>],
    },
    /// The context when rendering a filename for a template with [`ContextMode::Annotation`][context-mode].
    ///
    /// [context-mode]: crate::render::template::ContextMode::Annotation
    Annotation {
        book: &'a BookContext<'a>,
        annotation: &'a AnnotationContext<'a>,
    },
    /// The context when rendering the directory name for a template with
    /// [`StructureMode::Nested`][nested] or [`StructureMode::NestedGouped`][nested-grouped].
    ///
    /// [nested]: crate::render::template::StructureMode::Nested
    /// [nested-grouped]: crate::render::template::StructureMode::NestedGrouped
    Directory { book: &'a BookContext<'a> },
}

impl<'a> NamesContext<'a> {
    fn book(book: &'a BookContext<'a>, annotations: &'a [AnnotationContext<'a>]) -> Self {
        Self::Book { book, annotations }
    }

    fn annotation(book: &'a BookContext<'a>, annotation: &'a AnnotationContext<'a>) -> Self {
        Self::Annotation { book, annotation }
    }

    fn directory(book: &'a BookContext<'a>) -> Self {
        Self::Directory { book }
    }
}
