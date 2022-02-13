use std::path::PathBuf;

pub mod app;
pub mod dev;
pub mod test;

#[allow(unused_imports)] // For docs.
use crate::lib::applebooks::database::ABDatabase;

/// TODO Document
pub trait Configuration: Send + Sync {
    /// Returns the path to the root databases directory. This value can either
    /// point to the official Apple Books directory or one used in development
    /// or testing. See [`ABDatabase::get_database()`] for information on how
    /// the directory is structured.
    fn databases(&self) -> &PathBuf;

    /// Returns the path to the output directory where data, renders and
    /// back-ups are saved to.
    fn output(&self) -> &PathBuf;
}

impl std::fmt::Debug for dyn Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Is there a way to get this to print the concrete type's name?
        f.debug_struct("Configuration")
            .field("databases", &self.databases())
            .field("output", &self.output())
            .finish()
    }
}
