#![cfg(test)]

use std::path::PathBuf;

use crate::cli;

use super::{Config, ConfigOptions, RenderMode};

#[derive(Debug)]
pub struct TestConfig {
    options: ConfigOptions,
}

impl Config for TestConfig {
    fn options(&self) -> &ConfigOptions {
        &self.options
    }
}

impl TestConfig {
    /// TODO Document
    pub fn new(name: &str) -> Self {
        Self {
            options: ConfigOptions {
                databases: cli::defaults::MOCK_DATABASES.join(name),
                output: Self::build_output_directory(name),
                render_mode: RenderMode::Multi,
                templates: Vec::new(),
                is_quiet: true,
            },
        }
    }

    /// Returns a path to a temp directory to use for reading and writing data
    /// during testing.
    ///
    /// Internally this returns the value of the TMPDIR environment variable if
    /// it is set, otherwise it returns `/tmp`. See [`std::env::temp_dir()`] for
    /// more information.
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
    /// /var/folders/58/8yrgg8897ld633zt0qg9ww680000gn/T/readstor/tests/books-annotated
    /// ```
    fn build_output_directory(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.extend(["readstor", "tests", name].iter());
        path
    }
}
