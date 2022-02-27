use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::{self, Value};
use tera::{Context, Tera};

use crate::cli::config::{Config, RenderMode};
use crate::lib;

use super::models::annotation::Annotation;
use super::models::book::Book;
use super::models::entry::Entry;
use super::result::{LibError, LibResult};
use super::utils;

/// Provides a simple interface to add templates and render [`Entry`]s.
#[derive(Debug)]
pub struct Registry {
    /// Template registry containing all the parsed templates.
    registry: Tera,

    /// Stores the default template.
    default_flat: Template,

    /// Stores the default template.
    default_split: Template,

    /// Stores a list of all [`Template`]s in the registry. See [`Template`]
    /// and [`Registry::render()`] for more information.
    templates: Vec<Template>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut registry = Tera::default();

        registry.register_filter("join_paragraph", join_paragraph);
        registry
            .add_raw_template(
                lib::defaults::DEFAULT_TEMPLATE_FLAT_NAME,
                lib::defaults::DEFAULT_TEMPLATE_FLAT,
            )
            // Should be safe here to unwrap seeing as this is the default
            // template and will be evaluated at compile-time.
            .unwrap();
        registry
            .add_raw_template(
                lib::defaults::DEFAULT_TEMPLATE_SPLIT_NAME,
                lib::defaults::DEFAULT_TEMPLATE_SPLIT,
            )
            // Should be safe here to unwrap seeing as this is the default
            // template and will be evaluated at compile-time.
            .unwrap();

        let default_flat = Template {
            path: PathBuf::new(),
            name: lib::defaults::DEFAULT_TEMPLATE_FLAT_NAME.to_owned(),
            stem: "default-single".to_owned(),
            extension: "md".to_owned(),
        };

        let default_split = Template {
            path: PathBuf::new(),
            name: lib::defaults::DEFAULT_TEMPLATE_SPLIT_NAME.to_owned(),
            stem: "default-multi".to_owned(),
            extension: "md".to_owned(),
        };

        Self {
            registry,
            default_flat,
            default_split,
            templates: Vec::new(),
        }
    }
}

impl Registry {
    /// Adds a template to the registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains either syntax errors or
    /// variables that reference non-existent fields in a [`Entry`].
    pub fn add(&mut self, template: Template) -> LibResult<()> {
        // Attempt to add a new template to the registry. This will fail if the
        // template has syntax errors.
        self.registry
            .add_template_file(&template.path, Some(&template.name))
            .map_err(LibError::InvalidTemplate)?;

        // Run a test render of the new template using an dummy `Entry` to check
        // that the template does not contain variables that reference
        // non-existent fields in a `Entry`.
        self.registry
            .render(&template.name, &Context::from_serialize(Entry::default())?)
            .map_err(LibError::InvalidTemplate)?;

        self.templates.push(template);

        Ok(())
    }

    /// TODO Document
    pub fn render(
        &self,
        config: &dyn Config,
        entry: &Entry,
        path: &Path,
    ) -> LibResult<()> {
        if self.templates.is_empty() {
            match config.options().render_mode() {
                RenderMode::Single => {
                    self.render_flat(path, &self.default_flat, entry)?;
                }
                RenderMode::Multi => {
                    self.render_split(path, &self.default_split, entry)?;
                }
            }
        } else {
            match config.options().render_mode() {
                RenderMode::Single => {
                    for template in &self.templates {
                        self.render_flat(path, template, entry)?;
                    }
                }
                RenderMode::Multi => {
                    for template in &self.templates {
                        self.render_split(path, template, entry)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Renders an [`Entry`] to disk with the following structure:
    ///
    /// ```plaintext
    /// [path]
    ///  │
    ///  └─ [template-name]
    ///      ├─ [entry-name].[template-ext]
    ///      ├─ [entry-name].[template-ext]
    ///      └─ ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    // TODO add `serde_json::Error` as possible error.
    fn render_flat(
        &self,
        path: &Path,
        template: &Template,
        entry: &Entry,
    ) -> LibResult<()> {
        // -> [path]/[template-name]
        let root = path.join(&template.stem);

        std::fs::create_dir_all(&root)?;

        let file = format!("{}.{}", entry.name(), template.extension);
        let file = root.join(file);
        let file = fs::File::create(&file)?;

        let context = FlatContext {
            book: &entry.book,
            annotations: &entry.annotations,
        };

        self.registry
            .render_to(&template.name, &Context::from_serialize(context)?, file)
            .map_err(LibError::InvalidTemplate)?;

        Ok(())
    }

    /// Renders an [`Entry`] to disk with the following structure:
    ///
    /// ```plaintext
    /// [path]
    ///  │
    ///  └─ [template-name]
    ///      │
    ///      └─ [entry-name]
    ///          ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
    ///          ├─ [YYYY-MM-DD-HHMMSS]-[book-title].[template-ext]
    ///          └─ ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    // TODO add `serde_json::Error` as possible error.
    fn render_split(
        &self,
        path: &Path,
        template: &Template,
        entry: &Entry,
    ) -> LibResult<()> {
        // -> [path]/[template-name]/[entry-name]
        let root = path.join(&template.stem).join(entry.name());

        std::fs::create_dir_all(&root)?;

        for annotation in &entry.annotations {
            let date = annotation
                .metadata
                .created
                .format(lib::defaults::DATE_FORMAT);
            let name = utils::to_slug_string(&entry.book.title, '-');

            let file = format!("{}-{}.{}", date, name, template.extension);
            let file = root.join(file);
            let file = fs::File::create(&file)?;

            let context = SplitContext {
                book: &entry.book,
                annotation,
            };

            self.registry
                .render_to(&template.name, &Context::from_serialize(context)?, file)
                .map_err(LibError::InvalidTemplate)?;
        }

        Ok(())
    }
}

/// Defines a struct representing a template's metadata e.g where to find it,
/// what its named etc. The templates data is handled by `Tera`. However a
/// [`Template`] provides a clean interface retrieve its respective parsed
/// template data from the registry and determine the path and file name of the
/// rendered template.
///
/// See [`Registry::render()`] for more information.
#[derive(Debug)]
pub struct Template {
    /// The path to the template.
    pub path: PathBuf,

    /// The template's file stem e.g. `/path/to/default.md` -> `default.md`.
    ///
    /// This field is used to both to identify a template in the the
    /// [`Registry`] `registry` and is the name given to the directory where its
    /// respective template renders to.
    ///
    /// See [`Registry::render()`] for more information.
    pub name: String,

    /// The template's file name e.g. `/path/to/default.md` -> `default`.
    pub stem: String,

    /// The template file extension e.g. `/path/to/default.md` -> `md`
    pub extension: String,
}

// https://stackoverflow.com/a/48773009/16968574
impl<P> From<P> for Template
where
    P: Into<PathBuf>,
{
    fn from(path: P) -> Self {
        let path: PathBuf = path.into();

        let name = utils::get_file_name(&path).map_or_else(
            || {
                log::warn!(
                    "Could not read custom template file name. Using default \
                    value: `custom.txt`."
                );
                "custom.txt".to_owned()
            },
            String::from,
        );

        let stem = utils::get_file_stem(&path).map_or_else(
            || {
                log::warn!(
                    "Could not read custom template file stem. Using default \
                    value: `custom`."
                );
                "custom".to_owned()
            },
            String::from,
        );

        let extension = utils::get_file_extension(&path).map_or_else(
            || {
                log::warn!(
                    "Could not read custom template file extension. Using \
                    default value: `txt`."
                );
                "txt".to_owned()
            },
            String::from,
        );

        Self {
            path,
            name,
            stem,
            extension,
        }
    }
}

/// TODO Document
#[derive(Serialize)]
struct FlatContext<'a> {
    book: &'a Book,
    annotations: &'a Vec<Annotation>,
}

/// TODO Document
#[derive(Serialize)]
struct SplitContext<'a> {
    book: &'a Book,
    annotation: &'a Annotation,
}

/// Joins a list of paragraph blocks with double line-breaks.
/// <https://github.com/Keats/tera/blob/master/src/builtins/filters/array.rs>
fn join_paragraph(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = tera::try_get_value!("join_paragraph", "value", Vec<Value>, value);

    let rendered = value
        .iter()
        .map(serde_json::Value::as_str)
        // TODO Is there a more elegant way to do this?
        .map(|v| v.unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(serde_json::to_value(&rendered).unwrap())
}
