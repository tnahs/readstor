use std::path::PathBuf;

use once_cell::sync::Lazy;

/// Defines the environment variable key used to determine whether the application is being
/// developed on or not. If so, the Apple Books databases path is bypassed and redirected to a local
/// testing/dev database.
pub const READSTOR_DEV: &str = "READSTOR_DEV";

/// Defines the environment variable key used to set the application's log level. Valid values are:
/// `error`, `warn`, `info`, `debug` and `trace`.
pub const READSTOR_LOG: &str = "READSTOR_LOG";

/// Defines the default output directory.
///
/// The full path:
/// ```plaintext
/// /users/[user]/.readstor
/// ```
pub static OUTPUT_DIRECTORY: Lazy<PathBuf> =
    Lazy::new(|| lib::defaults::HOME_DIRECTORY.join(".readstor"));

/// Defines the default template string. This is used as a fallback if the user doesn't supply a
/// templates directory.
pub static TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/basic/basic.jinja2"
));

/// Defines the root path to the test/mock databases.
pub static TEST_DATABASES_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases"].iter());
    path
});

pub static TEST_PLISTS_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "plists"].iter());
    path
});

#[cfg(test)]
pub mod testing {

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum MockDatabases {
        Empty,
        BooksNew,
        BooksAnnotated,
    }

    impl std::fmt::Display for MockDatabases {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "empty"),
                Self::BooksNew => write!(f, "books-new"),
                Self::BooksAnnotated => write!(f, "books-annotated"),
            }
        }
    }

    impl From<MockDatabases> for PathBuf {
        fn from(databases: MockDatabases) -> Self {
            TEST_DATABASES_DIRECTORY.join(databases.to_string())
        }
    }

    /// Defines the root path to the mock plists.
    pub static MOCK_PLISTS_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = lib::defaults::CRATE_ROOT.to_owned();
        path.extend(["data", "plists"].iter());
        path
    });

    #[derive(Debug, Clone, Copy)]
    pub enum MockPlists {
        Empty,
        BooksNew,
        BooksAnnotated,
    }

    impl std::fmt::Display for MockPlists {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "empty"),
                Self::BooksNew => write!(f, "books-new"),
                Self::BooksAnnotated => write!(f, "books-annotated"),
            }
        }
    }

    impl From<MockPlists> for PathBuf {
        fn from(databases: MockPlists) -> Self {
            MOCK_PLISTS_DIRECTORY.join(databases.to_string())
        }
    }
}
