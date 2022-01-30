use std::path::PathBuf;

use super::args::Args;
use super::defaults as cli_defaults;
use crate::lib::applebooks::defaults as applebooks_defaults;

/// Represents the application's base configuration.
#[derive(Debug)]
pub struct Config {
    /// The path to the output directory where data, renders and back-ups are
    /// saved to. Defaults to `~/.readstor`.
    output: PathBuf,

    /// Path to the root databases directory. This value can either point to
    /// the official Apple Books directory or one designed to use during
    /// development. See [`ABDatabase::get_database()`] for information on how
    /// the directory is be structured.
    databases: PathBuf,
}

/// Default implementation for `Config` primarily used for testing.
impl Default for Config {
    fn default() -> Self {
        Self {
            output: cli_defaults::OUTPUT.to_owned(),
            databases: applebooks_defaults::DATABASES.to_owned(),
        }
    }
}

impl Config {
    pub fn new(args: &Args) -> Self {
        // If no output directory is supplied fall back to the default.
        let output = args
            .output
            .clone()
            .unwrap_or_else(|| cli_defaults::OUTPUT.to_owned());

        // Select the appropriate database depending on development mode.
        let databases = if Self::is_development_mode() {
            log::warn!("Running in development mode.");
            cli_defaults::DEV_DATABASES
                .to_owned()
                .join("books-annotated")
        } else {
            applebooks_defaults::DATABASES.to_owned()
        };

        let config = Self { output, databases };

        log::debug!("Running with `config`: {:#?}.", &config);

        config
    }

    /// Returns a bool based on if the `DEV_READSTOR` environment variable is
    /// set or not. This is primarily used to switch to a development database.
    fn is_development_mode() -> bool {
        std::env::var_os(cli_defaults::DEV_READSTOR).is_some()
    }

    /// See [`Config`].
    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    /// See [`Config`].
    pub fn databases(&self) -> &PathBuf {
        &self.databases
    }
}
