//! Defines types for backing-up macOS's Apple Books databases.

use std::path::Path;

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::applebooks::macos::utils::APPLEBOOKS_VERSION;
use crate::applebooks::macos::ABDatabase;
use crate::result::Result;

/// The default back-up directory template.
///
/// Outputs `[YYYY-MM-DD-HHMMSS]-[VERSION]` e.g. `1970-01-01-120000-v0.1-0000`.
pub const DIRECTORY_TEMPLATE: &str = "{{ now |  date(format='%Y-%m-%d-%H%M%S')}}-{{ version }}";

/// A struct for running the back-up task.
#[derive(Debug, Clone, Copy)]
pub struct BackupRunner;

impl BackupRunner {
    /// Runs the back-up task.
    ///
    /// # Arguments
    ///
    /// * `databases` - The directory to back-up.
    /// * `output` - The ouput directory.
    /// * `options` - The back-up options.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn run<O>(databases: &Path, output: &Path, options: O) -> Result<()>
    where
        O: Into<BackupOptions>,
    {
        let options: BackupOptions = options.into();

        Self::backup(databases, output, options)?;

        Ok(())
    }

    /// Backs-up macOS's Apple Books databases to disk.
    ///
    /// # Arguments
    ///
    /// * `databases` - The directory to back-up.
    /// * `output` - The ouput directory.
    /// * `options` - The back-up options.
    ///
    /// The `output` strucutre is as follows:
    ///
    /// ```plaintext
    /// [ouput-directory]
    ///  │
    ///  ├── [YYYY-MM-DD-HHMMSS-VERSION]
    ///  │    │
    ///  │    ├── AEAnnotation
    ///  │    │   ├── AEAnnotation*.sqlite
    ///  │    │   └── ...
    ///  │    │
    ///  │    └─ BKLibrary
    ///  │       ├── BKLibrary*.sqlite
    ///  │       └── ...
    ///  │
    ///  ├── [YYYY-MM-DD-HHMMSS-VERSION]
    ///  │    └── ...
    ///  └── ...
    /// ```
    ///
    /// See [`ABMacos`][abmacos] for information
    ///
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    ///
    /// [abmacos]: crate::applebooks::macos::ABMacos
    pub fn backup(databases: &Path, output: &Path, options: BackupOptions) -> Result<()> {
        let directory_template = if let Some(template) = options.directory_template {
            Self::validate_template(&template)?;
            template
        } else {
            DIRECTORY_TEMPLATE.to_string()
        };

        // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
        let directory_name = Self::render_directory_name(&directory_template)?;

        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]
        let destination_root = output.join(directory_name);

        for name in &[
            ABDatabase::Books.to_string(),
            ABDatabase::Annotations.to_string(),
        ] {
            // -> [databases-directory]/[name]
            let source = databases.join(name.clone());
            // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]/[name]
            let destination = destination_root.join(name);

            crate::utils::copy_dir(source, destination)?;
        }

        Ok(())
    }

    /// Validates a template by rendering it.
    ///
    /// Seeing as [`BackupNameContext`] requires no external context, this is a pretty
    /// straightforward validation check. The template is rendered and an empty [`Result`] is
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to validate.
    fn validate_template(template: &str) -> Result<()> {
        Self::render_directory_name(template).map(|_| ())
    }

    /// Renders the directory name from a template string.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string to render.
    fn render_directory_name(template: &str) -> Result<String> {
        let context = BackupNameContext::default();
        crate::utils::render_and_sanitize(template, context)
    }
}

/// A struct representing options for the [`BackupRunner`] struct.
#[derive(Debug)]
pub struct BackupOptions {
    /// The template to use render for rendering the back-up's ouput directory.
    pub directory_template: Option<String>,
}

/// A struct represening the template context for back-ups.
///
/// This is primarily used for generating directory names.
#[derive(Debug, Serialize)]
struct BackupNameContext {
    /// The current datetime.
    now: DateTime<Local>,

    /// The currently installed version of Apple Books for macOS.
    version: String,
}

impl Default for BackupNameContext {
    fn default() -> Self {
        Self {
            now: Local::now(),
            version: APPLEBOOKS_VERSION.to_owned(),
        }
    }
}

#[cfg(test)]
mod test_backup {

    use tera::Tera;

    use super::*;

    fn load_raw_template(directory: &str, filename: &str) -> String {
        let path = crate::defaults::TEST_TEMPLATES
            .join(directory)
            .join(filename);
        std::fs::read_to_string(path).unwrap()
    }

    // Tests that the default template returns no error.
    #[test]
    fn default_template() {
        let context = BackupNameContext::default();
        let context = &tera::Context::from_serialize(context).unwrap();

        Tera::one_off(DIRECTORY_TEMPLATE, context, false).unwrap();
    }

    // Tests that all valid context fields return no errors.
    #[test]
    fn valid_context() {
        let template = load_raw_template("valid-context", "valid-backup.txt");

        let context = BackupNameContext::default();
        let context = &tera::Context::from_serialize(context).unwrap();

        Tera::one_off(&template, context, false).unwrap();
    }

    // Tests that an invalid context field returns an error.
    #[test]
    #[should_panic]
    fn invalid_context() {
        let template = load_raw_template("invalid-context", "invalid-backup.txt");

        let context = BackupNameContext::default();
        let context = &tera::Context::from_serialize(context).unwrap();

        Tera::one_off(&template, context, false).unwrap();
    }
}
