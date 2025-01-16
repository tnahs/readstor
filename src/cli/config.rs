use std::path::PathBuf;

use color_eyre::eyre::Context;
use lib::applebooks::ios::ABPlist;
use lib::applebooks::macos::ABDatabase;
use lib::applebooks::Platform;

use super::args::GlobalOptions;
use super::{utils, CliResult};

#[derive(Debug)]
pub struct Config {
    /// The Apple Books platform.
    pub platform: Platform,

    /// The data directory.
    pub data_directory: PathBuf,

    /// The path to the output directory.
    pub output_directory: PathBuf,

    /// Flag to enable/disable terminal output.
    pub is_quiet: bool,
}

impl Config {
    /// Creates a new instance of [`Config`].
    ///
    /// # Arguments
    ///
    /// * `platform` - Which platform to build for.
    /// * `options` - The config options.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * Any IO errors are encountered.
    /// * There are any errors finding/reading the iOS device.
    pub fn new(platform: Platform, options: GlobalOptions) -> CliResult<Self> {
        let data_directory = Self::get_data_directory(platform, options.data_directory)
            .wrap_err("Failed while retrieving source data directory")?;

        let output_directory = Self::get_output_directory(options.output_directory);

        Ok(Self {
            platform,
            data_directory,
            output_directory,
            is_quiet: options.is_quiet,
        })
    }

    fn get_output_directory(path: Option<PathBuf>) -> PathBuf {
        if let Some(path) = path {
            return path;
        }

        if utils::is_development_env() {
            lib::defaults::TEMP_OUTPUT_DIRECTORY.to_owned()
        } else {
            super::defaults::OUTPUT_DIRECTORY.to_owned()
        }
    }

    fn get_data_directory(platform: Platform, path: Option<PathBuf>) -> CliResult<PathBuf> {
        if let Some(path) = path {
            return Ok(path);
        }

        let path = match platform {
            Platform::MacOs => {
                let destination = lib::defaults::TEMP_OUTPUT_DIRECTORY.join("macos-data");
                std::fs::create_dir_all(&destination)?;

                if utils::is_development_env() {
                    let source = super::defaults::TEST_DATABASES_DIRECTORY.join("books-annotated");
                    ABDatabase::save_to(&destination, Some(&source))?;
                } else {
                    ABDatabase::save_to(&destination, None)?;
                };

                destination
            }
            Platform::IOs => {
                let destination = lib::defaults::TEMP_OUTPUT_DIRECTORY.join("ios-data");
                std::fs::create_dir_all(&destination)?;

                if utils::is_development_env() {
                    let source = super::defaults::TEST_PLISTS_DIRECTORY.join("books-annotated");
                    ABPlist::save_to(&destination, Some(&source))?;
                } else {
                    ABPlist::save_to(&destination, None)?;
                }

                destination
            }
        };

        Ok(path)
    }
}

#[cfg(test)]
pub mod testing {

    use super::*;

    use crate::cli::defaults::testing::{MockDatabases, MockPlists};

    impl Config {
        fn test_macos(databases: MockDatabases) -> Self {
            let output_directory = lib::defaults::TEMP_OUTPUT_DIRECTORY
                .join("tests-macos")
                .join(databases.to_string());

            Self {
                platform: Platform::MacOs,
                data_directory: databases.into(),
                output_directory,
                is_quiet: true,
            }
        }

        fn test_ios(plists: MockPlists) -> Self {
            let output_directory = lib::defaults::TEMP_OUTPUT_DIRECTORY
                .join("tests-ios")
                .join(plists.to_string());

            Self {
                platform: Platform::IOs,
                data_directory: plists.into(),
                output_directory,
                is_quiet: true,
            }
        }
    }

    pub struct TestConfig;

    impl TestConfig {
        pub fn macos_empty() -> Config {
            Config::test_macos(MockDatabases::Empty)
        }

        pub fn macos_new() -> Config {
            Config::test_macos(MockDatabases::BooksNew)
        }

        pub fn macos_annotated() -> Config {
            Config::test_macos(MockDatabases::BooksAnnotated)
        }

        pub fn ios_empty() -> Config {
            Config::test_ios(MockPlists::Empty)
        }

        pub fn ios_new() -> Config {
            Config::test_ios(MockPlists::BooksNew)
        }

        pub fn ios_annotated() -> Config {
            Config::test_ios(MockPlists::BooksAnnotated)
        }
    }
}
