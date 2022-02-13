use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::cli::defaults as cli_defaults;

use super::Configuration;

/// TODO Document
pub static DEV_DATABASES: Lazy<PathBuf> =
    Lazy::new(|| cli_defaults::MOCK_DATABASES.join("books-annotated"));

/// TODO Document
/// Returns a path to a temp directory to use for reading and writing data
/// during testing.
///
/// Internally this returns the value of the TMPDIR environment variable if
/// it is set, otherwise it returns `/tmp`. See [`std::env::temp_dir()`]
/// for more information.
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

#[derive(Debug, Default)]
pub struct DevConfig;

impl Configuration for DevConfig {
    fn databases(&self) -> &PathBuf {
        &DEV_DATABASES
    }

    fn output(&self) -> &PathBuf {
        &DEV_OUTPUT
    }
}

/// TODO Document
pub fn is_development_env() -> bool {
    match std::env::var_os(cli_defaults::READSTOR_DEV) {
        // This ensures that if the variable exists but is an empty value, the
        // function will return false.
        Some(value) => !value.is_empty(),
        None => false,
    }
}
