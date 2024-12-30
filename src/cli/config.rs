use std::path::PathBuf;

use lib::applebooks;

use super::utils::is_development_env;
use super::Options;

#[derive(Debug)]
pub struct Config {
    /// The data directory.
    pub data_directory: DataDirectory,

    /// The path to the output directory.
    pub output_directory: PathBuf,

    /// Flag to enable/disable terminal output.
    pub is_quiet: bool,
}

impl Default for Config {
    fn default() -> Self {
        // Creates the appropriate `Config` depending on the environment. In a development
        // environment this sets the output directory to a temp directory on disk.
        //
        // Note that the appropriate environment variable to signal a development env should be set
        // in the `.cargo/config.toml` file.
        let output_directory = if is_development_env() {
            super::defaults::TEMP_OUTPUT_DIRECTORY.to_owned()
        } else {
            super::defaults::OUTPUT_DIRECTORY.to_owned()
        };

        Self {
            data_directory: DataDirectory::default(),
            output_directory,
            is_quiet: false,
        }
    }
}

impl From<Options> for Config {
    fn from(options: Options) -> Self {
        let data_directory =
            DataDirectory::new(options.databases_directory, options.plists_directory);

        Self::default()
            .data_directory(data_directory)
            .output_directory(options.output_directory)
    }
}

impl Config {
    fn data_directory(mut self, data_directory: DataDirectory) -> Self {
        self.data_directory = data_directory;
        self
    }

    fn output_directory(mut self, path: Option<PathBuf>) -> Self {
        let Some(path) = path else {
            return self;
        };

        self.output_directory = path;
        self
    }
}

#[derive(Debug)]
pub enum DataDirectory {
    Macos(PathBuf),
    Ios(PathBuf),
    Both {
        path_macos: PathBuf,
        path_ios: PathBuf,
    },
}

impl Default for DataDirectory {
    fn default() -> Self {
        // Creates the appropriate `DataDirectory` depending on the environment. In a development
        // environment this sets the path to a local mock databases directory.
        //
        // Note that the appropriate environment variable to signal a development env should be set
        // in the `.cargo/config.toml` file.
        if is_development_env() {
            Self::Macos(super::defaults::MockDatabases::BooksAnnotated.into())
        } else {
            Self::Macos(applebooks::macos::defaults::DATABASES.to_owned())
        }
    }
}

impl DataDirectory {
    fn new(databases: Option<PathBuf>, plists: Option<PathBuf>) -> Self {
        match [databases, plists] {
            [Some(path), None] => Self::Macos(path),
            [None, Some(path)] => Self::Ios(path),
            [Some(path_macos), Some(path_ios)] => Self::Both {
                path_macos,
                path_ios,
            },
            _ => Self::default(),
        }
    }
}

#[cfg(test)]
pub mod test_config {

    use super::*;

    use crate::cli::defaults::MockDatabases;
    use crate::cli::defaults::MockPlists;

    impl Config {
        fn test_macos(databases: MockDatabases) -> Self {
            let name = databases.to_string();

            Self {
                data_directory: DataDirectory::Macos(databases.into()),
                output_directory: crate::cli::defaults::TEMP_OUTPUT_DIRECTORY
                    .join("tests-macos")
                    .join(name),
                is_quiet: true,
            }
        }

        fn test_ios(plists: MockPlists) -> Self {
            let name = plists.to_string();

            Self {
                data_directory: DataDirectory::Ios(plists.into()),
                output_directory: crate::cli::defaults::TEMP_OUTPUT_DIRECTORY
                    .join("tests-ios")
                    .join(name),
                is_quiet: true,
            }
        }

        fn test_both(databases: MockDatabases, plists: MockPlists) -> Self {
            let name = format!("{databases}_{plists}");

            Self {
                data_directory: DataDirectory::Both {
                    path_macos: databases.into(),
                    path_ios: plists.into(),
                },
                output_directory: crate::cli::defaults::TEMP_OUTPUT_DIRECTORY
                    .join("tests-both")
                    .join(name),
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

        pub fn both_empty() -> Config {
            Config::test_both(MockDatabases::Empty, MockPlists::Empty)
        }

        pub fn both_new() -> Config {
            Config::test_both(MockDatabases::BooksNew, MockPlists::BooksNew)
        }

        pub fn both_annotated() -> Config {
            Config::test_both(MockDatabases::BooksAnnotated, MockPlists::BooksAnnotated)
        }
    }
}
