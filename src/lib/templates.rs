//! Defines the [`TemplateManager`] struct used to build and interact with
//! templates.

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use pathdiff::diff_paths;
use serde::Serialize;
use serde_json::{self, Value};
use tera::{Context, Tera};

use super::models::annotation::Annotation;
use super::models::book::Book;
use super::models::entry::Entry;
use super::result::{LibError, LibResult};
use super::utils;

// TODO: Document
static TEMPLATE: Lazy<Template> = Lazy::new(|| Template {
    path: PathBuf::new(),
    registry_name: "readstor/default".to_string(),
    display_name: "default".to_string(),
    extension: "md".to_string(),
    kind: TemplateKind::Single,
});

// TODO: Document
const TEMPLATE_STR: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/templates/default/single.default.md"
));

/// A struct providing a simple interface to build and render [`Template`]s.
///
/// Template data is stored in two different locations: the `registry` holds all
/// the parsed templates ready for rendering while `templates` holds each
/// template's metadata. When rendering, each template is rendered based on its
/// [`TemplateKind`].
//
// TODO: Rename `registry`, `templates` and `Template`.
#[derive(Debug)]
pub struct TemplateManager {
    /// An instance of [`Tera`] containing all the parsed templates.
    registry: Tera,

    /// A list of all the registered [`Template`]s' metadata.
    templates: Vec<Template>,
}

impl Default for TemplateManager {
    fn default() -> Self {
        let mut registry = Tera::default();
        registry.register_filter(stringify!(join_paragraph), join_paragraph);

        Self {
            registry,
            templates: Vec::new(),
        }
    }
}

impl TemplateManager {
    /// Builds the [`TemplateManager`]'s [`Template`]s.
    ///
    /// Delegates [`Template`] building depending on whether a templates
    /// directory is provided or not. If none is provided then the default
    /// template is built.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to a directory containing templates.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template's filename does not follow the proper naming convention.
    /// * A template contains either syntax errors or variables that reference
    /// non-existent fields in a [`Book`]/[`Annotation`].
    pub fn build(&mut self, path: &Option<PathBuf>) -> LibResult<()> {
        if let Some(path) = path {
            self.build_from_dir(path)?;
        } else {
            self.build_default();
        }

        Ok(())
    }

    /// Iterates through all [`Template`]s renders them based on their
    /// [`TemplateKind`].
    ///
    /// # Arguments
    ///
    /// * `entry` - The [`Entry`] to be rendered.
    /// * `path` - A path to a directory to save the rendered file.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn render(&self, entry: &Entry, path: &Path) -> LibResult<()> {
        for template in &self.templates {
            match template.kind {
                TemplateKind::Single => self.render_single(path, template, entry)?,
                TemplateKind::Multi => self.render_multi(path, template, entry)?,
                // Partials are rendered by the templates that `include` them.
                TemplateKind::Partial => {}
            }
        }
        Ok(())
    }

    /// Builds [`Template`]s from a directory containing user created template
    /// files.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * A template's filename does not follow the proper naming convention.
    /// * A template contains either syntax errors or variables that reference
    /// non-existent fields in a [`Book`]/[`Annotation`].
    fn build_from_dir(&mut self, path: &Path) -> LibResult<()> {
        let root = path;

        for path in utils::iter_dir_shallow(&root) {
            let template = Template::new(root, &path)?;
            self.register(template)?;
        }

        log::debug!(
            "Built {} template(s) from {}",
            self.templates.len(),
            root.display()
        );

        Ok(())
    }

    /// Builds the default [`Template`].
    ///
    /// # Panics
    ///
    /// Panics if the default template contains an inheritance chain that cannot
    /// be built. This should not be an issue as the current default template
    /// contains no inheritance chains.
    fn build_default(&mut self) {
        // TODO: Is there a way to avoid this clone?
        let template = TEMPLATE.clone();

        self.registry
            .add_raw_template(&template.registry_name, TEMPLATE_STR)
            // Unwrap should be okay here as were not building a template
            // inheritance chain.
            .unwrap();

        self.templates.push(template);

        log::debug!("Built the default template");
    }

    /// Adds a template to the registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains either syntax errors or
    /// variables that reference non-existent fields in a
    /// [`Book`]/[`Annotation`].
    fn register(&mut self, template: Template) -> LibResult<()> {
        // FIXME: This currently fails when trying to validate templates with
        // inheritance chains.
        // Self::validate_template(&template)?;

        // TODO: Remove this unwrap.
        self.registry
            .add_template_file(&template.path, Some(&template.registry_name))?;

        log::debug!("Added template: `{}`", template.registry_name);

        self.templates.push(template);

        Ok(())
    }

    /// Validates that a template does not contain variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    ///
    /// Tera checks for invalid syntax when a new template is registered however
    /// the template's use of variables can only be checked when a context is
    /// supplied. This method performs a test render with a dummy context to
    /// check for valid use of variables.
    ///
    /// # Errors
    //
    /// Will return `Err` if the template contains variables that reference
    /// non-existent fields in an [`Entry`]/[`Book`]/[`Annotation`].
    fn validate_template(template: &Template) -> Result<(), LibError> {
        // Caching values here to avoid lifetime issues.
        let entry = Entry::default();
        let book = Book::default();
        let annotation = Annotation::default();

        let contexts = match template.kind {
            TemplateKind::Single => {
                vec![TemplateContext::single(&entry)]
            }
            TemplateKind::Multi => {
                vec![
                    TemplateContext::multi_book(&book),
                    TemplateContext::multi_annotation(&book, &annotation),
                ]
            }
            // Partials are validated by the templates that `include` them.
            TemplateKind::Partial => Vec::new(),
        };

        let template_string = fs::read_to_string(&template.path)?;

        for context in contexts {
            Tera::one_off(&template_string, &Context::from_serialize(context)?, true)?;
        }

        Ok(())
    }

    /// Renders an [`Entry`] to disk with the following structure:
    ///
    /// ```plaintext
    /// [path]
    ///  │
    ///  └─ [template-name]
    ///      ├─ [author-title].[template-ext]
    ///      ├─ [author-title].[template-ext]
    ///      └─ ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    //
    // TODO: add `serde_json::Error` as possible error.
    fn render_single(&self, path: &Path, template: &Template, entry: &Entry) -> LibResult<()> {
        // -> [path]/[template-name]
        let root = path.join(&template.display_name);

        std::fs::create_dir_all(&root)?;

        let file = format!("{}.{}", entry.name(), template.extension);
        let file = root.join(file);
        let file = File::create(&file)?;

        let context = TemplateContext::single(entry);

        self.registry.render_to(
            &template.registry_name,
            &Context::from_serialize(context)?,
            file,
        )?;

        Ok(())
    }

    /// Renders an [`Entry`] to disk with the following structure:
    ///
    /// ```plaintext
    /// [path]
    ///  │
    ///  └─ [template-name]
    ///      │
    ///      └─ [author-title]
    ///          ├─ [author-title].[template-ext]
    ///          ├─ [YYYY-MM-DD-HHMMSS]-[title].[template-ext]
    ///          ├─ [YYYY-MM-DD-HHMMSS]-[title].[template-ext]
    ///          └─ ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    //
    // TODO: add `serde_json::Error` as possible error.
    fn render_multi(&self, path: &Path, template: &Template, entry: &Entry) -> LibResult<()> {
        // -> [path]/[template-name]/[author-title]
        let root = path.join(&template.display_name).join(entry.name());

        std::fs::create_dir_all(&root)?;

        // -> [path]/[template-name]/[author-title]/[author-title].[template.book-ext]
        let file = format!("{}.{}", entry.name(), template.extension);
        let file = root.join(file);
        let file = File::create(&file)?;

        let context = TemplateContext::multi_book(&entry.book);

        self.registry.render_to(
            &template.registry_name,
            &Context::from_serialize(context)?,
            file,
        )?;

        for annotation in &entry.annotations {
            // -> [path]/[template-name]/[author-title]/[YYYY-MM-DD-HHMMSS]-[title].[template-ext]
            let file = format!(
                "{}-{}.{}",
                annotation.date_created_pretty(),
                utils::to_slug_string(&entry.book.title, '-'),
                template.extension
            );
            let file = root.join(file);
            let file = File::create(&file)?;

            let context = TemplateContext::multi_annotation(&entry.book, annotation);

            self.registry.render_to(
                &template.registry_name,
                &Context::from_serialize(context)?,
                file,
            )?;
        }

        Ok(())
    }
}

/// A struct representing a template's metadata e.g. its location on disk, its
/// name, its extension etc.
///
/// [`Template`]s are used to retrieve its respective parsed template data from
/// the registry and determine the path and file name of the final rendered file
/// on disk.
///
/// Templates follow a strict naming convention:
///
/// ```plaintext
/// [template-kind].[template-name].[template-ext]
/// ```
/// * `[template-kind]` - A string representing how a template will be rendered.
/// It must be either `multi`, `single` or `partial`.  See [`TemplateKind`] for
/// more information.
/// * `[template-name]` - The template's name.
/// * `[temaplte-ext]` - The template's file extension.
#[derive(Debug, Clone)]
struct Template {
    /// The path to the template.
    ///
    /// `[templates]/nested/single.default.md`
    path: PathBuf,

    /// The template's registry name. The name used to uniquely identify a
    /// template in the registry and within a template's `include` statement.
    ///
    /// `[templates]/nested/single.default.md` -> `nested/single.default.md`
    ///
    /// ```jinja
    /// {% include "nested/single.default.md" %}
    /// ```
    registry_name: String,

    /// The template's display name. Used for naming the directory where the
    /// template's rendered files go. See [`TemplateManager::render_single()`]
    /// or [`TemplateManager::render_multi()`] for more information.
    ///
    /// `[templates]/nested/single.default.md` -> `default`
    display_name: String,

    /// The template's file extension.
    ///
    /// `[templates]/nested/single.default.md` -> `md`
    extension: String,

    /// The template kind. See [`TemplateKind`] for more information.
    ///
    /// `[templates]/nested/single.default.md` -> `TemplateKind::Single`
    kind: TemplateKind,
}

impl Template {
    /// Constructs a new [`Template`] from the templates' root directory and a
    /// path to a template within it. The template's relative path is necessary
    /// for both identifying a template within the registry and building the
    /// template's inheritance chains.
    //
    // TODO: Can we reduce error handling code duplication?
    fn new(root: &Path, path: &Path) -> LibResult<Self> {
        // Converts an absolute path to one relative to the `templates`
        // directory.
        //
        // `path/to/templates/nested/single.default.md` -> `nested/single.default.md`
        let registry_name = diff_paths(path, root)
            // TODO: Document this unwrap.
            .unwrap()
            .to_string_lossy()
            .to_string();

        let file_name = utils::get_file_name(&path).ok_or(LibError::InvalidTemplateName {
            path: path.display().to_string(),
        })?;

        // `single.default.md` -> (`single`, `default.md`)
        let (kind, remainder) = file_name
            .split_once('.')
            .ok_or(LibError::InvalidTemplateName {
                path: path.display().to_string(),
            })?;

        // `default.md` -> (`default`, `md`)
        let (name, extension) =
            remainder
                .rsplit_once('.')
                .ok_or(LibError::InvalidTemplateName {
                    path: path.display().to_string(),
                })?;

        let kind: TemplateKind = kind.parse().map_err(|_| LibError::InvalidTemplateName {
            path: path.display().to_string(),
        })?;

        let template = Self {
            path: path.to_owned(),
            registry_name,
            display_name: name.to_owned(),
            extension: extension.to_owned(),
            kind,
        };

        Ok(template)
    }
}

/// TODO: Document
#[derive(Debug, Clone, Copy)]
enum TemplateKind {
    /// TODO: Document
    Single,

    /// TODO: Document
    Multi,

    /// TODO: Document
    Partial,
}

impl std::str::FromStr for TemplateKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "single" => Self::Single,
            "multi" => Self::Multi,
            "partial" => Self::Partial,
            _ => return Err("invalid template kind.".into()),
        };

        Ok(kind)
    }
}

/// TODO: Document
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "mode")]
enum TemplateContext<'a> {
    #[serde(rename = "entry")]
    Single {
        book: &'a Book,
        annotations: &'a [Annotation],
    },

    #[serde(rename = "book")]
    MultiBook { book: &'a Book },

    #[serde(rename = "annotation")]
    MultiAnnotation {
        book: &'a Book,
        annotation: &'a Annotation,
    },
}

impl<'a> TemplateContext<'a> {
    fn single(entry: &'a Entry) -> Self {
        Self::Single {
            book: &entry.book,
            annotations: &entry.annotations,
        }
    }

    fn multi_book(book: &'a Book) -> Self {
        Self::MultiBook { book }
    }

    fn multi_annotation(book: &'a Book, annotation: &'a Annotation) -> Self {
        Self::MultiAnnotation { book, annotation }
    }
}

/// Custom template function for Tera. Joins a list of paragraph blocks with
/// double line-breaks.
///
/// <https://github.com/Keats/tera/blob/master/src/builtins/filters/array.rs>
//
// TODO: Should this have the option for different number of breaks?
fn join_paragraph(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = tera::try_get_value!("join_paragraph", "value", Vec<Value>, value);

    let rendered = value
        .iter()
        .map(serde_json::Value::as_str)
        // TODO: Is there a more elegant way to do this?
        .map(|v| v.unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(serde_json::to_value(&rendered).unwrap())
}

// TODO: Test that templates pass and fail registration.
#[cfg(test)]
mod test_templates {
    // use super::*;
}
