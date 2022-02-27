pub mod app;
pub mod dev;
pub mod test;

use std::path::PathBuf;

use clap::ArgEnum;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

/// TODO Document
pub trait Config: Send + Sync {
    fn options(&self) -> &ConfigOptions;
}

impl std::fmt::Debug for dyn Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Is there a way to get this to print the concrete type's name?
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

    /// TODO Document
    output: PathBuf,

    /// TODO Document
    templates: Vec<PathBuf>,

    /// TODO Document
    render_mode: RenderMode,

    /// TODO Document
    is_quiet: bool,
}

impl ConfigOptions {
    pub fn databases(&self) -> &PathBuf {
        &self.databases
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    pub fn templates(&self) -> &[PathBuf] {
        self.templates.as_ref()
    }

    pub fn render_mode(&self) -> &RenderMode {
        &self.render_mode
    }

    pub fn is_quiet(&self) -> bool {
        self.is_quiet
    }
}

#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum RenderMode {
    Single,
    Multi,
}
