use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::cli;
use crate::cli::args::ArgOptions;

use super::{Config, ConfigOptions};

/// TODO: Document
pub static DEV_DATABASES: Lazy<PathBuf> =
    Lazy::new(|| cli::defaults::MOCK_DATABASES.join("books-annotated"));

/// TODO: Document
/// Returns a path to a temp directory to use for reading and writing data
/// during testing.
///
/// Internally this returns the value of the TMPDIR environment variable if it
/// is set, otherwise it returns `/tmp`. See [`std::env::temp_dir()`] for more
/// information.
///
/// The full path:
///
/// ```plaintext
/// [temp_dir]/readstor/[name]
/// ```
///
/// For example:
///
/// ```plaintext
/// /var/folders/58/8yrgg8897ld633zt0qg9ww680000gn/T/readstor/dev/
/// ```
pub static DEV_OUTPUT: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = std::env::temp_dir();
    path.extend(["readstor", "dev"].iter());
    path
});

#[derive(Debug)]
pub struct DevConfig {
    options: ConfigOptions,
}

impl Config for DevConfig {
    fn options(&self) -> &ConfigOptions {
        &self.options
    }
}

impl From<ArgOptions> for DevConfig {
    fn from(options: ArgOptions) -> Self {
        let databases = options
            .databases
            .unwrap_or_else(|| DEV_DATABASES.to_owned());

        let output = options.output.unwrap_or_else(|| DEV_OUTPUT.to_owned());

        Self {
            options: ConfigOptions {
                databases,
                output,
                templates: options.templates,
                is_quiet: options.is_quiet,
            },
        }
    }
}

/// TODO: Document
pub fn is_development_env() -> bool {
    match std::env::var_os(cli::defaults::READSTOR_DEV) {
        // This ensures that if the variable exists but is an empty value, the
        // function will return false.
        Some(value) => !value.is_empty(),
        None => false,
    }
}
