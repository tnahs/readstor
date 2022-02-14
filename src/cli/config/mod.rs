pub mod app;
pub mod dev;
pub mod test;

use std::path::PathBuf;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

/// TODO Document
pub trait Config: Send + Sync {
    /// Returns the path to the root databases directory. This value can either
    /// point to the official Apple Books directory or one used in development
    /// or testing. See [`ABDatabase::get_database()`] for information on how
    /// the directory is structured.
    fn databases(&self) -> &PathBuf;

    // /// Returns the path to the output directory where data, renders and
    // /// back-ups are saved to.
    // fn output(&self) -> &PathBuf;
    fn options(&self) -> &ConfigOptions;
}

impl std::fmt::Debug for dyn Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Is there a way to get this to print the concrete type's name?
        f.debug_struct("Config")
            .field("databases", &self.databases())
            .field("options", &self.options())
            .finish()
    }
}

#[derive(Debug)]
pub struct ConfigOptions {
    output: PathBuf,
    templates: Vec<PathBuf>,
    is_quiet: bool,
}

impl ConfigOptions {
    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn templates(&self) -> &[PathBuf] {
        self.templates.as_ref()
    }

    pub fn is_quiet(&self) -> bool {
        self.is_quiet
    }
}
