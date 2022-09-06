//! Defines the [`Template`] and [`PartialTemplate`] structs and various helper
//! structs and enums to represent a templates metadata.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::lib::models::annotation::Annotation;
use crate::lib::models::book::Book;
use crate::lib::models::entry::Entry;
use crate::lib::result::{LibError, LibResult};
use crate::lib::utils;

#[allow(unused_imports)] // For docs.
use super::manager::TemplateManager;

/// A struct representing a fully configured template.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Template {
    /// The template's id.
    ///
    /// This is typically a file path relative to the templates directory. It
    /// serves to identify a template within the registry when rendering. This
    /// is one of two fields that are passed to Tera when registering the
    /// template. The other one being [`Template.contents`].
    ///
    /// ```plaintext
    /// --> /path/to/templates/nested/template.md
    /// --> nested/template.md
    /// ```
    #[serde(skip_deserializing)]
    pub id: String,

    /// The unparsed contents of the template.
    ///
    /// This gets parsed and validated during registration. This is one of two
    /// fields that are passed to Tera when registering the template. The other
    /// one being [`Template.id`].
    #[serde(skip_deserializing)]
    pub contents: String,

    /// The template's group name.
    ///
    /// This is used to identify and optionally sort templates into a single
    /// directory i.e. when multiple templates are intended for a single output.
    /// The most common case would be a template used to render a [`Book`] and
    /// one to render each of its [`Annotation`]s separately. Used when the
    /// template's output mode is either [`OutputMode::FlatGrouped`] or
    /// [`OutputMode::NestedGrouped`].
    pub group: String,

    /// The template's output mode i.e. how the output should be structured.
    ///
    /// See [`OutputMode`] for more information.
    pub output_mode: OutputMode,

    /// The template's render context i.e what the template intends to render.
    ///
    /// See [`RenderContext`] for more information.
    pub render_context: RenderContext,

    /// The default template used when generating an output filename for the
    /// template when its render context is [`RenderContext::Book`].
    #[serde(default = "Template::default_filename_template_book")]
    pub filename_template_book: String,

    /// The default template used when generating an output filename for the
    /// template when its render context is [`RenderContext::Annotation`].
    #[serde(default = "Template::default_filename_template_annotation")]
    pub filename_template_annotation: String,

    /// The default template used when generating a nested output directory for
    /// the template when its output mode is either [`OutputMode::Nested`] or
    /// [`OutputMode::NestedGrouped`].
    #[serde(default = "Template::default_nested_directory_template")]
    pub nested_directory_template: String,

    /// The template's file extension.
    pub extension: String,
}

impl Template {
    /// Creates a new instance of [`Template`].
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the template relative to the templates directory.
    /// * `string` - The contents of the template file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The template's opening and closing config tags have syntax errors.
    /// * The tempalte's config has syntax errors or is missing required fields.
    pub fn new<P>(path: P, string: &str) -> LibResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let (config, contents) = Self::split_template_config_contents(string).ok_or(
            LibError::InvalidTemplateConfig {
                path: path.display().to_string(),
            },
        )?;

        let mut template: Self = serde_yaml::from_str(config)?;

        template.id = path.display().to_string();
        template.contents = contents.to_owned();

        Ok(template)
    }

    /// See [`Template.filename_template_book`].
    fn default_filename_template_book() -> String {
        super::defaults::FILENAME_TEMPLATE_BOOK.to_owned()
    }

    /// See [`Template.filename_template_annotation`].
    fn default_filename_template_annotation() -> String {
        super::defaults::FILENAME_TEMPLATE_ANNOTATION.to_owned()
    }

    /// See [`Template.default_nested_directory_template`].
    fn default_nested_directory_template() -> String {
        super::defaults::NESTED_DIRECTORY_TEMPLATE.to_owned()
    }

    /// Returns a tuple containing the template's configuration and its contents
    /// respectively.
    ///
    /// Returns `None` if the template's configuration is formatted incorrectly.
    fn split_template_config_contents(string: &str) -> Option<(&str, &str)> {
        let open = super::defaults::CONFIG_TAG_OPEN;
        let close = super::defaults::CONFIG_TAG_CLOSE;

        if !string.starts_with(open) {
            return None;
        }

        let config_start = open.len();
        let config_end = string.find(close)?;

        let contents_start = config_end + close.len();

        let config = &string[config_start..config_end];
        let mut contents = &string[contents_start..];

        if contents.starts_with('\n') {
            contents = &contents[1..];
        }

        Some((config, contents))
    }
}

impl std::fmt::Debug for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Template")
            .field("id", &self.id)
            .field("group", &self.group)
            .field("output_mode", &self.output_mode)
            .field("render_context", &self.render_context)
            .finish()
    }
}

/// An enum representing the ways to structure a template's rendered files.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputMode {
    /// When selected, the template is rendered to the output directory without
    /// any structure.
    ///
    /// ```yaml
    /// output-mode: flat
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [template-name-01].[extension]
    ///  ├─ [template-name-01].[extension]
    ///  └─ ...
    /// ```
    Flat,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `template-group`. This useful
    /// if there are multiple templates being rendered to the same directory.
    ///
    /// ```yaml
    /// output-mode: flat-grouped
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [template-group-01]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-group-02]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    FlatGrouped,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `nested-directory-template`.
    /// This useful if multiple templates are used to represent a single book
    /// i.e. a book template used to render a book's information to a single
    /// file and an annotation template used to render each annotation to a
    /// separate file.
    ///
    /// ```yaml
    /// output-mode: nested
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [author-title-01]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   ├─ [template-name-01].[extension]
    ///  │   └─ ...
    ///  │
    ///  ├─ [author-title-02]
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    Nested,

    /// When selected, the template is rendered to the output directory and
    /// placed inside a directory named after its `template-group` and another
    /// named after its `nested-directory-template`. This useful if multiple
    /// templates are used to represent a single book i.e. a book template and
    /// an annotation template.
    ///
    /// ```yaml
    /// output-mode: nested-grouped
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  │
    ///  ├─ [template-group-01]
    ///  │   │
    ///  │   ├─ [author-title-01]
    ///  │   │   ├─ [template-name-01].[extension]
    ///  │   │   ├─ [template-name-01].[extension]
    ///  │   │   └─ ...
    ///  │   │   
    ///  │   ├─ [author-title-02]
    ///  │   │   ├─ [template-name-02].[extension]
    ///  │   │   ├─ [template-name-02].[extension]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  │
    ///  ├─ [template-group-02]
    ///  │   ├─ [author-title-01]
    ///  │   │   └─ ...
    ///  │   └─ ...
    ///  └─ ...
    /// ```
    NestedGrouped,
}

/// An enum representing what a template intends to render.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenderContext {
    /// When selected, the template rendered to a single file containing a
    /// [`Book`] and all its [`Annotation`]s.
    ///
    /// ```yaml
    /// render-context: book
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  └─ [template-name].[extension]
    /// ```
    Book,

    /// When selected, the template is rendered to multiple files containing a
    /// [`Book`] and only one its [`Annotation`]s.
    ///
    /// ```yaml
    /// render-context: annotation
    /// ```
    ///
    /// ```plaintext
    /// [output]
    ///  ├─ [template-name].[extension]
    ///  ├─ [template-name].[extension]
    ///  ├─ [template-name].[extension]
    ///  └─ ...
    /// ```
    Annotation,
}

/// A struct representing all the output file and directory names for a given
/// template.
///
/// This is used to (1) name files and directories when rendering templates to
/// disk and (2) is included in the template's context so that files/direcories
/// related to the template can be references within the tenplate.
///
/// See [`TemplateManager::render()`] for more information.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Names {
    /// The output filename for a template with [`RenderContext::Book`].
    pub book: String,

    /// The output filenames for a template with [`RenderContext::Annotation`].
    pub annotations: HashMap<String, String>,

    /// The directory name for a template with [`OutputMode::Nested`] or
    /// [`OutputMode::NestedGrouped`].
    pub nested_directory: String,
}

impl Names {
    /// Creates a new instance of [`Names`] given an [`Entry`] and a [`Template`].
    ///
    /// Note that all names are generated regardless of the template's
    /// [`RenderContext`]. For example, when a separate template is used to
    /// render a [`Book`] and another for its [`Annotation`]s, it's important
    /// that both templates have access to the other's filenames so they can
    /// link to one another if the user desires.
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] injected into the filename templates.
    /// * `template` - The [`Template`] containing the filename templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any filename templates have syntax errors or are referencing
    /// non-existent fields in their respective contexts.
    pub fn new(entry: &Entry, template: &Template) -> LibResult<Self> {
        let book = Self::render_filename_book(entry, template)?;
        let annotations = Self::render_filenames_annotation(entry, template)?;
        let nested_directory = Self::render_name_nested_directory(entry, template)?;

        Ok(Self {
            book,
            annotations,
            nested_directory,
        })
    }

    fn render_filename_book(entry: &Entry, template: &Template) -> LibResult<String> {
        let context = TemplateContext::filename_book(entry);

        let name = Tera::one_off(
            &template.filename_template_book,
            &Context::from_serialize(context)?,
            false,
        )?;

        let name = utils::to_safe_string(&name);

        Ok(format!("{}.{}", name, template.extension))
    }

    fn render_filenames_annotation(
        entry: &Entry,
        template: &Template,
    ) -> LibResult<HashMap<String, String>> {
        let mut annotations = HashMap::new();

        for annotation in &entry.annotations {
            let context = TemplateContext::filename_annotation(&entry.book, annotation);

            let name = Tera::one_off(
                &template.filename_template_annotation,
                &Context::from_serialize(context)?,
                false,
            )?;

            let name = utils::to_safe_string(&name);

            annotations.insert(
                annotation.metadata.id.clone(),
                format!("{}.{}", name, template.extension),
            );
        }

        Ok(annotations)
    }

    fn render_name_nested_directory(entry: &Entry, template: &Template) -> LibResult<String> {
        let context = TemplateContext::filename_book(entry);

        let name = Tera::one_off(
            &template.nested_directory_template,
            &Context::from_serialize(context)?,
            false,
        )?;

        let name = utils::to_safe_string(&name);

        Ok(name)
    }
}

/// An enum representing all possible template contexts.
///
/// This primarily used to shuffle data to fit a certain shape before it's
/// injected into a template.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TemplateContext<'a> {
    /// Used when rendering both a [`Book`] and its [`Annotation`]s in a
    /// template. Includes all the output filenames and the nested directory
    /// name.
    Book {
        /// The [`Book`] being injected into the template.
        book: &'a Book,

        /// The [`Annotation`]s being injected into the template.
        annotations: &'a [Annotation],

        /// The filenames and nested directory name.
        #[serde(rename = "links")]
        names: &'a Names,
    },
    /// Used when rendering a single annotation in a template. Includes all the
    /// output filenames and the nested directory name.
    Annotation {
        /// The [`Book`] being injected into the template.
        book: &'a Book,

        /// The [`Annotation`] being injected into the template.
        annotation: &'a Annotation,

        /// The filenames and nested directory name.
        #[serde(rename = "links")]
        names: &'a Names,
    },
    /// Used when rendering the output filename for a template with
    /// [`RenderContext::Book`].
    FilenameBook {
        /// The [`Book`] being injected into the template.
        book: &'a Book,

        /// The [`Annotation`] being injected into the template.
        annotations: &'a [Annotation],
    },
    /// Used when rendering the output filename for a template with
    /// [`RenderContext::Annotation`].
    FilnameAnnotation {
        /// The [`Book`] being injected into the template.
        book: &'a Book,

        /// The [`Annotation`] being injected into the template.
        annotation: &'a Annotation,
    },
}

#[allow(missing_docs)]
impl<'a> TemplateContext<'a> {
    #[must_use]
    pub fn book(entry: &'a Entry, names: &'a Names) -> Self {
        Self::Book {
            book: &entry.book,
            annotations: &entry.annotations,
            names,
        }
    }

    #[must_use]
    pub fn annotation(book: &'a Book, annotation: &'a Annotation, names: &'a Names) -> Self {
        Self::Annotation {
            book,
            annotation,
            names,
        }
    }

    #[must_use]
    pub fn filename_book(entry: &'a Entry) -> Self {
        Self::FilenameBook {
            book: &entry.book,
            annotations: &entry.annotations,
        }
    }

    #[must_use]
    pub fn filename_annotation(book: &'a Book, annotation: &'a Annotation) -> Self {
        Self::FilnameAnnotation { book, annotation }
    }
}

/// A struct representing a unconfigured partial template.
///
/// Partial templates get their configuration from the normal templates that
/// `{% include %}` them.
#[derive(Clone)]
pub struct PartialTemplate {
    /// The template's id.
    ///
    /// This is typically a file path relative to the templates directory. It
    /// serves to identify a partial template when called in an `{% include %}`
    /// statement from within a normal template. This field is passed to Tera
    /// when registering the template.
    ///
    /// ```plaintext
    /// --> /path/to/templates/nested/template.md
    /// --> nested/template.md
    /// --> {% include "nested/template.md" %}
    /// ````
    pub id: String,

    /// The unparsed contents of the template.
    ///
    /// This gets parsed and validated only when a normal template that includes
    /// it is being parsed and valiated. This field is passed to Tera when
    /// registering the template.
    pub contents: String,
}

impl PartialTemplate {
    /// Creates a new instance of [`PartialTemplate`].
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the template relative to the templates directory.
    /// * `string` - The contents of the template file.
    pub fn new<P>(path: P, string: &str) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            id: path.as_ref().display().to_string(),
            contents: string.to_owned(),
        }
    }
}

impl std::fmt::Debug for PartialTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialTemplate")
            .field("id", &self.id)
            .finish()
    }
}
