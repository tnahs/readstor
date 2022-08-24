pub mod app;
pub mod dev;
pub mod test;

use std::path::PathBuf;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

/// TODO: Document
pub trait Config: Send + Sync {
    /// TODO: Document
    fn options(&self) -> &ConfigOptions;
}

impl std::fmt::Debug for dyn Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Is there a way to get this to print the concrete type's name?
        f.debug_struct("Config")
            .field("options", &self.options())
            .finish()
    }
}

#[derive(Debug)]
pub struct ConfigOptions {
    /// Returns the path to the root databases directory. This value can either
    /// point to the official Apple Books directory or one used in development
    /// or testing. See [`ABDatabase::get_database()`] for information on how
    /// the directory is structured.
    databases: PathBuf,

    /// TODO: Document
    output: PathBuf,

    /// TODO: Document
    templates: Option<PathBuf>,

    /// TODO: Document
    is_quiet: bool,
}

impl ConfigOptions {
    pub fn databases(&self) -> &PathBuf {
        &self.databases
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn templates(&self) -> &Option<PathBuf> {
        &self.templates
    }

    pub fn is_quiet(&self) -> bool {
        self.is_quiet
    }
}
