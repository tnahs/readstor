//! Defines the context for [`Names`][names] data.
//!
//![names]: crate::templates::template::Names

use std::collections::HashMap;

use serde::Serialize;
use tera::Tera;

use crate::models::datetime::DateTimeUtc;
use crate::result::Result;

use super::super::template::TemplateRaw;
use super::annotation::AnnotationContext;
use super::entry::EntryContext;
use super::template::TemplateContext;

/// A struct representing the rendered template strings for all the output file
/// and directory names for a given template.
///
/// This is used to (1) name files and directories when rendering templates to
/// disk and (2) is included in the template's context so that files/direcories
/// related to the template can be references within the tenplate.
///
/// See [`Templates::render()`][render] for more information.
///
/// [render]: super::super::Templates::render()
#[derive(Debug, Default, Clone, Serialize)]
pub struct NamesContext {
    /// The output filename for a template with [`ContextMode::Book`][book].
    ///
    /// [book]: super::super::ContextMode::Book
    pub book: String,

    /// The output filenames for a template with
    /// [`ContextMode::Annotation`][annotation].
    ///
    /// Internally this field is stored as a `HashMap` but is converted into a
    /// `Vec` before it's injected into a template.
    ///
    /// [annotation]: super::super::ContextMode::Annotation
    #[serde(serialize_with = "crate::templates::utils::serialize_hashmap_to_vec")]
    pub annotations: HashMap<String, AnnotationNameAttributes>,

    /// The directory name for a template with
    /// [`StructureMode::Nested`][nested] or
    /// [`StructureMode::NestedGrouped`][nested-grouped].
    ///
    /// [nested]: super::super::StructureMode::Nested
    /// [nested-grouped]: super::super::StructureMode::NestedGrouped
    pub directory: String,
}

impl NamesContext {
    /// Creates a new instance of [`NamesContext`].
    ///
    /// Note that all names are generated regardless of the template's
    /// [`ContextMode`][context-mode]. For example, when a separate template
    /// is used to render a [`Book`][book] and another for its
    /// [`Annotation`][annotation]s, it's important that both templates have
    /// access to the other's filenames so they can link to one another if the
    /// user desires.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`EntryContext`] injected into the filename templates.
    /// * `template` - The [`TemplateRaw`] containing the filename templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any templates have syntax errors or are
    /// referencing non-existent fields in their respective contexts.
    ///
    /// [annotation]: crate::models::annotation::Annotation
    /// [book]: crate::models::book::Book
    /// [context-mode]: super::super::ContextMode
    pub fn new(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<Self> {
        Ok(Self {
            book: Self::render_book_filename(entry, template)?,
            annotations: Self::render_annotation_filenames(entry, template)?,
            directory: Self::render_directory_name(entry, template)?,
        })
    }

    fn render_book_filename(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<String> {
        let context = TemplateContext::name_book(entry);

        let mut filename = Tera::one_off(
            &template.names.book,
            &tera::Context::from_serialize(context)?,
            false,
        )?;

        filename = crate::utils::sanitize_string(&filename);

        Ok(format!("{filename}.{}", template.extension))
    }

    fn render_annotation_filenames(
        entry: &EntryContext<'_>,
        template: &TemplateRaw,
    ) -> Result<HashMap<String, AnnotationNameAttributes>> {
        let mut annotations = HashMap::new();

        for annotation in &entry.annotations {
            let context = TemplateContext::name_annotation(&entry.book, annotation);

            let mut filename = Tera::one_off(
                &template.names.annotation,
                &tera::Context::from_serialize(context)?,
                false,
            )?;

            filename = crate::utils::sanitize_string(&filename);
            filename = format!("{filename}.{}", template.extension);

            annotations.insert(
                annotation.metadata.id.clone(),
                AnnotationNameAttributes::new(annotation, filename),
            );
        }

        Ok(annotations)
    }

    fn render_directory_name(entry: &EntryContext<'_>, template: &TemplateRaw) -> Result<String> {
        let context = TemplateContext::name_book(entry);

        let mut directory_name = Tera::one_off(
            &template.names.directory,
            &tera::Context::from_serialize(context)?,
            false,
        )?;

        directory_name = crate::utils::sanitize_string(&directory_name);

        Ok(directory_name)
    }
}

/// A struct representing the rendered filename for a template with
/// [`ContextMode::Annotation`][context-mode] along with a set of attributes
/// used for sorting within a template.
///
/// For example:
///
/// ```jinja
/// {% for name in names.annotations | sort(attribute="location") -%}
/// ![[{{ name.filename }}]]
/// {% endfor %}
/// ```
/// See [`NamesContext::annotations`][names-context] for more information.
///
/// See [`AnnotationMetadata`][annotation-metadata] for undocumented fields.
///
/// [annotation-metadata]: crate::models::annotation::AnnotationMetadata
/// [context-mode]: super::super::ContextMode::Annotation
/// [names-context]: NamesContext#structfield.annotations
#[derive(Debug, Default, Clone, Serialize)]
#[allow(missing_docs)]
pub struct AnnotationNameAttributes {
    /// The rendered filename for a template with
    /// [`ContextMode::Annotation`][context-mode].
    ///
    /// [context-mode]: super::super::ContextMode
    pub filename: String,
    pub created: DateTimeUtc,
    pub modified: DateTimeUtc,
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
