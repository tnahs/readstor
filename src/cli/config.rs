use crate::cli;
use crate::lib::applebooks;

use std::path::PathBuf;

use once_cell::sync::Lazy;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

use super::args::ArgOptions;

/// Returns a path to a temp directory to use for reading and writing data
/// during development/testing.
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
/// /var/folders/58/8yrgg8897ld633zt0qg9ww680000gn/T/readstor/
/// ```
pub static TEMP_OUTPUT: Lazy<PathBuf> = Lazy::new(|| std::env::temp_dir().join("readstor"));

#[derive(Debug)]
pub struct Config {
    /// The path to the root databases directory. This value can either point
    /// to the official Apple Books directory or one used in development or
    /// testing. See [`ABDatabase::get_database()`] for information on how the
    /// directory is structured.
    pub databases: PathBuf,

    /// The path to the output directory.
    pub output: PathBuf,

    /// Flag to enable/disable terminal output.
    pub is_quiet: bool,
}

impl Config {
    pub fn app(options: ArgOptions) -> Self {
        let databases = options
            .databases
            .unwrap_or_else(|| applebooks::defaults::DATABASES.to_owned());

        let output = options
            .output
            .unwrap_or_else(|| cli::defaults::OUTPUT.to_owned());

        Self {
            databases,
            output,
            is_quiet: options.is_quiet,
        }
    }

    pub fn dev(options: ArgOptions) -> Self {
        let databases = options
            .databases
            .unwrap_or_else(|| cli::defaults::MOCK_DATABASES.join("books-annotated"));

        let output = options.output.unwrap_or_else(|| TEMP_OUTPUT.join("dev"));

        Self {
            databases,
            output,
            is_quiet: options.is_quiet,
        }
    }

    #[cfg(test)]
    pub fn test(name: &str) -> Self {
        Self {
            databases: cli::defaults::MOCK_DATABASES.join(name),
            output: TEMP_OUTPUT.join("tests").join(name),
            is_quiet: true,
        }
    }
}
