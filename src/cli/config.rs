use crate::cli;
use crate::lib::applebooks;

use std::path::PathBuf;

use once_cell::sync::Lazy;

use super::Options;

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
    /// testing. See [`ABDatabase::get_database()`][get-database] for
    /// information on how the directory is structured.
    ///
    /// [get-database]: crate::lib::applebooks::database::ABDatabase::get_database()
    pub databases_directory: PathBuf,

    /// The path to the output directory.
    pub output_directory: PathBuf,

    /// Flag to enable/disable terminal output.
    pub is_quiet: bool,
}

impl From<crate::cli::Options> for Config {
    fn from(options: crate::cli::Options) -> Self {
        // Selects the appropriate Config depending on the environment. In a
        // development environment this sets the `databases` to local mock databases
        // directory and the `output` to a temp directory on disk.
        //
        // Note that the appropriate environment variable to signal a development
        // env should be set in the `.cargo/config.toml` file.
        if cli::utils::is_development_env() {
            Self::dev(options)
        } else {
            Self::app(options)
        }
    }
}

impl Config {
    pub fn app(options: Options) -> Self {
        let output_directory = options
            .output_directory
            .unwrap_or_else(|| cli::defaults::OUTPUT.to_owned());

        let databases_directory = options
            .databases_directory
            .unwrap_or_else(|| applebooks::defaults::DATABASES.to_owned());

        Self {
            databases_directory,
            output_directory,
            is_quiet: options.is_quiet,
        }
    }

    pub fn dev(options: Options) -> Self {
        let output_directory = options
            .output_directory
            .unwrap_or_else(|| TEMP_OUTPUT.join("dev"));

        let databases_directory = options
            .databases_directory
            .unwrap_or_else(|| cli::defaults::MOCK_DATABASES.join("books-annotated"));

        Self {
            databases_directory,
            output_directory,
            is_quiet: options.is_quiet,
        }
    }

    #[cfg(test)]
    pub fn test(name: &str) -> Self {
        Self {
            databases_directory: cli::defaults::MOCK_DATABASES.join(name),
            output_directory: TEMP_OUTPUT.join("tests").join(name),
            is_quiet: true,
        }
    }
}
