use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{self, Value};
use tera::{Context, Tera};

use super::defaults::{DEFAULT_TEMPLATE, DEFAULT_TEMPLATE_NAME};
use super::models::stor::StorItem;
use super::result::{ApplicationError, Result};
use super::utils;

/// Provides a simple interface to add templates and render [`StorItem`]s.
#[derive(Debug)]
pub struct Templates {
    /// Template registry containing all the parsed templates.
    registry: Tera,

    /// TODO Document
    default: Template,

    /// Stores a list of all [`Template`]s in the registry. See [`Template`]
    /// and [`Templates::render`] for more information.
    templates: Vec<Template>,
}

impl Default for Templates {
    fn default() -> Self {
        let mut registry = Tera::default();

        registry.register_filter("join_paragraph", join_paragraph);
        registry
            .add_raw_template(DEFAULT_TEMPLATE_NAME, DEFAULT_TEMPLATE)
            // TODO Add unwrap documentation.
            .unwrap();

        let default = Template {
            path: PathBuf::new(),
            name: DEFAULT_TEMPLATE_NAME.to_owned(),
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
    /// variables that reference non-existent fields in a [`StorItem`].
    pub fn add(&mut self, template: Template) -> Result<()> {
        // Attempt to add a new template to the registry. This will fail if
        // the template has syntax errors.
        match self
            .registry
            .add_template_file(&template.path, Some(&template.name))
        {
            Ok(_) => {}
            Err(err) => return Err(ApplicationError::Template(err)),
        };

        // Run a test render of the new template using an dummy `StorItem` to
        // check that the template does not contain variables that reference
        // non-existent fields in a `StorItem`.
        match self.registry.render(
            &template.name,
            // TODO How would this fail?
            &Context::from_serialize(StorItem::default())?,
        ) {
            Ok(_) => {}
            Err(err) => return Err(ApplicationError::Template(err)),
        };

        self.templates.push(template);

        Ok(())
    }

    /// Exports a [`StorItem`] to disk with the following structure:
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
    pub fn render(&self, stor_item: &StorItem, path: &Path) -> Result<()> {
        if self.templates.is_empty() {
            self.render_single(path, &self.default, stor_item)?;
            return Ok(());
        }

        for template in &self.templates {
            self.render_single(path, template, stor_item)?;
        }

        Ok(())
    }

    fn render_single(
        &self,
        path: &Path,
        template: &Template,
        stor_item: &StorItem,
    ) -> Result<()> {
        // -> [path]/[template-name]
        let template_path = path.join(&template.stem);

        std::fs::create_dir_all(&template_path)?;

        let file_name = format!("{}.{}", stor_item.name(), template.extension);
        let file_path = template_path.join(file_name);
        let file = fs::File::create(&file_path)?;

        match self.registry.render_to(
            &template.name,
            // TODO How would this fail?
            &Context::from_serialize(stor_item)?,
            file,
        ) {
            Ok(_) => {}
            Err(err) => return Err(ApplicationError::Template(err)),
        };

        Ok(())
    }
}

/// Defines a struct representing a template's metadata e.g where to find it,
/// what its named etc. The templates data is handled by `Tera`. However a
/// [`Template`] provides a clean interface retrieve its respective parsed
/// template data from the registry and determine the path and file name of the
/// rendered template.
///
/// See [`Templates::render`] for more information.
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
    /// See [`Templates::render`] for more information.
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
