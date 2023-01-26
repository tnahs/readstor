//! Defines types for backing-up Apple Books' databases.

use std::path::Path;

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::applebooks::database::ABDatabaseName;
use crate::applebooks::utils::APPLEBOOKS_VERSION;
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

    /// Backs-up Apple Books' databases to disk.
    ///
    /// # Arguments
    ///
    /// * `databases` - The directory to back-up.
    /// * `output` - The ouput directory.
    /// * `options` - The back-up options.
    ///
    /// The `databases` directory should contains the following structure as
    /// this is the way Apple Books' `Documents` directory is set up.
    ///
    /// ```plaintext
    /// [databases]
    ///  │
    ///  ├── AEAnnotation
    ///  │   ├── AEAnnotation*.sqlite
    ///  │   └── ...
    ///  │
    ///  ├── BKLibrary
    ///  │   ├── BKLibrary*.sqlite
    ///  │   └── ...
    ///  └── ...
    /// ```
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
    /// # Errors
    ///
    /// Will return `Err` if any IO errors are encountered.
    pub fn backup(databases: &Path, output: &Path, options: BackupOptions) -> Result<()> {
        let directory_template = options
            .directory_template
            .unwrap_or_else(|| DIRECTORY_TEMPLATE.to_string());

        // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
        let directory_name = Self::render_directory_name(&directory_template)?;

        // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]
        let destination_root = output.join(directory_name);

        for name in &[
            ABDatabaseName::Books.to_string(),
            ABDatabaseName::Annotations.to_string(),
        ] {
            // -> [databases-directory]/[name]
            let source = databases.join(name.clone());
            // -> [ouput-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]/[name]
            let destination = destination_root.join(name);

            crate::utils::copy_dir(source, destination)?;
        }

        Ok(())
    }

    fn render_directory_name(template: &str) -> Result<String> {
        let context = BackupContext::default();
        crate::utils::render_and_sanitize(template, context)
    }
}

/// A struct representing options for the [`BackupRunner`] struct.
#[derive(Debug)]
pub struct BackupOptions {
    /// The template to use render for rendering the back-up's ouput directory.
    pub directory_template: Option<String>,
}

#[derive(Debug, Serialize)]
struct BackupContext {
    now: DateTime<Local>,
    version: String,
}

impl Default for BackupContext {
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

    use crate::defaults::TEST_TEMPLATES;

    use super::*;

    fn load_raw_template(directory: &str, filename: &str) -> String {
        let path = TEST_TEMPLATES.join(directory).join(filename);
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn context() {
        let template = load_raw_template("valid-context", "valid-backup.txt");

        let context = BackupContext::default();
        let context = &tera::Context::from_serialize(context).unwrap();

        Tera::one_off(&template, context, false).unwrap();
    }

    #[test]
    fn default_template() {
        let context = BackupContext::default();
        let context = &tera::Context::from_serialize(context).unwrap();

        Tera::one_off(DIRECTORY_TEMPLATE, context, false).unwrap();
    }
}
