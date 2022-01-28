use std::path::PathBuf;

use super::args::Args;
use super::defaults::{DATABASES_DEV, DEV_READSTOR, OUTPUT};
use crate::lib::applebooks::defaults::DATABASES;

#[derive(Debug)]
pub struct Config {
    pub output: PathBuf,
    pub databases: PathBuf,
}

// TODO Document
// This is primarily used for testing.
impl Default for Config {
    fn default() -> Self {
        Self {
            output: OUTPUT.to_owned(),
            databases: DATABASES.to_owned(),
        }
    }
}

impl Config {
    pub fn new(args: &Args) -> Self {
        // Bypass the official database if the application is being worked on.
        let databases = if Self::is_development_mode() {
            log::warn!("Running in development mode.");
            DATABASES_DEV.to_owned().join("books-annotated")
        } else {
            DATABASES.to_owned()
        };

        let output = args.output.clone().unwrap_or_else(|| OUTPUT.to_owned());

        let config = Self { output, databases };

        log::debug!("Running with `config`: {:#?}.", &config);

        config
    }
    /// TODO Document
    fn is_development_mode() -> bool {
        std::env::var_os(DEV_READSTOR).is_some()
    }
}
