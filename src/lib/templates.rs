use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{self, Value};
use tera::{Context, Tera};

use crate::lib;

use super::models::data::Entry;
use super::result::{LibError, LibResult};
use super::utils;

/// Provides a simple interface to add templates and render [`Entry`]s.
#[derive(Debug)]
pub struct Templates {
    /// Template registry containing all the parsed templates.
    registry: Tera,

    /// Stores the default template.
    default: Template,

    /// Stores a list of all [`Template`]s in the registry. See [`Template`]
    /// and [`Templates::render()`] for more information.
    templates: Vec<Template>,
}

impl Default for Templates {
    fn default() -> Self {
        let mut registry = Tera::default();

        registry.register_filter("join_paragraph", join_paragraph);
        registry
            .add_raw_template(
                lib::defaults::DEFAULT_TEMPLATE_NAME,
                lib::defaults::DEFAULT_TEMPLATE,
            )
            // Should be safe here to unwrap seeing as this is the default
            // template and will be evaluated at compile-time.
            .unwrap();

        let default = Template {
            path: PathBuf::new(),
            name: lib::defaults::DEFAULT_TEMPLATE_NAME.to_owned(),
            stem: "default".to_owned(),
            extension: "txt".to_owned(),
        };

        Self {
            registry,
            default,
            templates: Vec::new(),
        }
    }
}

impl Templates {
    /// Adds a template to the registry.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the template contains either syntax errors or
    /// variables that reference non-existent fields in a [`Entry`].
    pub fn add(&mut self, template: Template) -> LibResult<()> {
        // Attempt to add a new template to the registry. This will fail if
        // the template has syntax errors.
        self.registry
            .add_template_file(&template.path, Some(&template.name))
            .map_err(LibError::InvalidTemplate)?;

        // Run a test render of the new template using an dummy `Entry` to
        // check that the template does not contain variables that reference
        // non-existent fields in a `Entry`.
        self.registry
            .render(&template.name, &Context::from_serialize(Entry::default())?)
            .map_err(LibError::InvalidTemplate)?;

        self.templates.push(template);

        Ok(())
    }

    /// Exports a [`Entry`] to disk with the following structure:
    ///
    /// ```plaintext
    /// [path]
    ///  │
    ///  └─ [template-name]
    ///      │
    ///      ├─ Author - Title.ext
    ///      ├─ Author - Title.ext
    ///      └─ ...
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    // TODO add `serde_json::Error` as possible error.
    pub fn render(&self, entry: &Entry, path: &Path) -> LibResult<()> {
        // TODO Document
        if self.templates.is_empty() {
            self.render_to_file(path, &self.default, entry)?;
            return Ok(());
        }

        // TODO Document
        for template in &self.templates {
            self.render_to_file(path, template, entry)?;
        }

        Ok(())
    }

    /// TODO Document
    fn render_to_file(
        &self,
        path: &Path,
        template: &Template,
        entry: &Entry,
    ) -> LibResult<()> {
        // -> [path]/[template-name]
        let template_path = path.join(&template.stem);

        std::fs::create_dir_all(&template_path)?;

        let file_name = format!("{}.{}", entry.name(), template.extension);
        let file_path = template_path.join(file_name);
        let file = fs::File::create(&file_path)?;

        self.registry
            .render_to(&template.name, &Context::from_serialize(entry)?, file)
            .map_err(LibError::InvalidTemplate)?;

        Ok(())
    }
}

/// Defines a struct representing a template's metadata e.g where to find it,
/// what its named etc. The templates data is handled by `Tera`. However a
/// [`Template`] provides a clean interface retrieve its respective parsed
/// template data from the registry and determine the path and file name of the
/// rendered template.
///
/// See [`Templates::render()`] for more information.
#[derive(Debug)]
pub struct Template {
    /// The path to the template.
    pub path: PathBuf,

    /// The template's file stem e.g. `/path/to/default.md` -> `default.md`.
    ///
    /// This field is used to both to identify a template in the the
    /// [`Templates`] `registry` and is the name given to the directory where
    /// its respective template renders to.
    ///
    /// See [`Templates::render()`] for more information.
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
