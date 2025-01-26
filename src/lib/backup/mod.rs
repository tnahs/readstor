//! Defines types for backing-up macOS's Apple Books databases.

use std::path::Path;

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::applebooks::ios::ABPlist;
use crate::applebooks::macos::utils::APPLEBOOKS_VERSION;
use crate::applebooks::macos::ABDatabase;
use crate::applebooks::Platform;
use crate::result::Result;
use crate::strings;

/// The default back-up directory template.
///
/// Outputs `[YYYY-MM-DD-HHMMSS]-[VERSION]` e.g. `1970-01-01-120000-v0.1-0000`.
pub const DIRECTORY_TEMPLATE: &str = "{{ now |  date(format='%Y-%m-%d-%H%M%S')}}-{{ version }}";

/// Backs-up data to disk.
///
/// For macOS, the output structure is as follows:
///
/// ```plaintext
/// [output-directory]
///  │
///  ├── [YYYY-MM-DD-HHMMSS-VERSION] <- Customizeable
///  │    │
///  │    ├── AEAnnotation
///  │    │   ├── AEAnnotation*.sqlite
///  │    │   └── ...
///  │    │
///  │    └── BKLibrary
///  │        ├── BKLibrary*.sqlite
///  │        └── ...
///  │
///  ├── [YYYY-MM-DD-HHMMSS-VERSION]
///  │    └── ...
///  └── ...
/// ```
///
/// See [`ABMacOs`][abmacos] for information
///
///
/// For iOS, the output structure is as follows:
///
/// ```plaintext
/// [output-directory]
///  │
///  ├── [YYYY-MM-DD-HHMMSS-VERSION] <- Customizeable
///  │    │
///  │    ├── Books.plist
///  │    └── com.apple.ibooks-sync.plist
///  │
///  ├── [YYYY-MM-DD-HHMMSS-VERSION]
///  │    └── ...
///  └── ...
/// ```
///
/// # Arguments
///
/// * `platform` - Which platform to perform the backup for.
/// * `source` - Where the source data is located.
/// * `destination` - Where to place the backup.
/// * `options` - The back-up options.
///
/// # Errors
///
/// Will return `Err` if any IO errors are encountered.
///
/// [abmacos]: crate::applebooks::macos::ABMacOs
pub fn run<O>(platform: Platform, source: &Path, destination: &Path, options: O) -> Result<()>
where
    O: Into<BackupOptions>,
{
    let options: BackupOptions = options.into();

    let context = match platform {
        Platform::MacOs => BackupNameContext::macos(),
        Platform::IOs => BackupNameContext::ios(),
    };

    let directory_template = if let Some(template) = options.directory_template {
        self::validate_template(&template, &context)?;
        template
    } else {
        DIRECTORY_TEMPLATE.to_string()
    };

    // -> [YYYY-MM-DD-HHMMSS]-[VERSION]
    let directory_name = self::render_directory_name(&directory_template, &context)?;

    // -> [output-directory]/[YYYY-MM-DD-HHMMSS]-[VERSION]
    let destination = destination.join(directory_name);

    std::fs::create_dir_all(&destination)?;

    match platform {
        Platform::MacOs => ABDatabase::save_to(&destination, Some(source))?,
        Platform::IOs => ABPlist::save_to(&destination, Some(source))?,
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
fn validate_template(template: &str, context: &BackupNameContext) -> Result<()> {
    self::render_directory_name(template, context).map(|_| ())
}

/// Renders the directory name from a template string.
///
/// # Arguments
///
/// * `template` - The template string to render.
fn render_directory_name(template: &str, context: &BackupNameContext) -> Result<String> {
    strings::render_and_sanitize(template, context)
}

/// A struct representing options for running back-ups.
#[derive(Debug)]
pub struct BackupOptions {
    /// The template to use render for rendering the back-up's output directory.
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

impl BackupNameContext {
    fn macos() -> Self {
        Self {
            now: Local::now(),
            version: APPLEBOOKS_VERSION.to_owned(),
        }
    }

    // TODO(0.7.0): Get iOS version or Apple Books version.
    fn ios() -> Self {
        Self {
            now: Local::now(),
            version: "ios-?".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::defaults::test::TemplatesDirectory;
    use crate::utils;

    mod macos {

        use super::*;

        // Tests that the default template returns no error.
        #[test]
        fn default_directory_template() {
            let context_macos = BackupNameContext::macos();

            strings::render_and_sanitize(DIRECTORY_TEMPLATE, context_macos).unwrap();
        }

        // Tests that all valid context fields return no errors.
        #[test]
        fn valid_context() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::ValidContext,
                "valid-backup.txt",
            );

            let context_macos = BackupNameContext::macos();

            strings::render_and_sanitize(&template, context_macos).unwrap();
        }

        // Tests that an invalid context field returns an error.
        #[test]
        #[should_panic(expected = "Failed to render '__tera_one_off'")]
        fn invalid_context() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-backup.txt",
            );
            let context_macos = BackupNameContext::macos();

            strings::render_and_sanitize(&template, context_macos).unwrap();
        }
    }

    mod ios {

        use super::*;

        // Tests that the default template returns no error.
        #[test]
        fn default_directory_template() {
            let context_ios = BackupNameContext::ios();

            strings::render_and_sanitize(DIRECTORY_TEMPLATE, context_ios).unwrap();
        }

        // Tests that all valid context fields return no errors.
        #[test]
        fn valid_context() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::ValidContext,
                "valid-backup.txt",
            );

            let context_ios = BackupNameContext::ios();

            strings::render_and_sanitize(&template, context_ios).unwrap();
        }

        // Tests that an invalid context field returns an error.
        #[test]
        #[should_panic(expected = "Failed to render '__tera_one_off'")]
        fn invalid_context() {
            let template = utils::testing::load_template_str(
                TemplatesDirectory::InvalidContext,
                "invalid-backup.txt",
            );
            let context_ios = BackupNameContext::ios();

            strings::render_and_sanitize(&template, context_ios).unwrap();
        }
    }
}
