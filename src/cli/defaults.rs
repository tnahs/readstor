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
pub static OUTPUT_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| lib::defaults::HOME.join(".readstor"));

/// Returns a path to a temp directory to use for reading and writing data during
/// development/testing.
///
/// Internally this returns the value of the TMPDIR environment variable if it is set, otherwise it
/// returns `/tmp`. See [`std::env::temp_dir()`] for more information.
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
/// /var/folders/58/8yrgg8897ld633zt0qg9ww680000gn/T/readstor/
/// ```
pub static TEMP_OUTPUT_DIRECTORY: Lazy<PathBuf> =
    Lazy::new(|| std::env::temp_dir().join("readstor"));

/// Defines the default template string. This is used as a fallback if the user doesn't supply a
/// templates directory.
pub static TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/basic/basic.jinja2"
));

/// Defines the root path to the mock databases.
pub static MOCK_DATABASES: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases"].iter());
    path
});

#[derive(Debug, Clone, Copy)]
pub enum MockDatabases {
    Empty,
    BooksNew,
    BooksAnnotated,
}

impl std::fmt::Display for MockDatabases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MockDatabases::Empty => write!(f, "empty"),
            MockDatabases::BooksNew => write!(f, "books-new"),
            MockDatabases::BooksAnnotated => write!(f, "books-annotated"),
        }
    }
}

impl From<MockDatabases> for PathBuf {
    fn from(databases: MockDatabases) -> Self {
        let name = match databases {
            MockDatabases::Empty => "empty",
            MockDatabases::BooksNew => "books-new",
            MockDatabases::BooksAnnotated => "books-annotated",
        };

        MOCK_DATABASES.join(name)
    }
}

/// Defines the root path to the mock plists.
pub static MOCK_PLISTS: Lazy<PathBuf> = Lazy::new(|| {
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
            MockPlists::Empty => write!(f, "empty"),
            MockPlists::BooksNew => write!(f, "books-new"),
            MockPlists::BooksAnnotated => write!(f, "books-annotated"),
        }
    }
}

impl From<MockPlists> for PathBuf {
    fn from(databases: MockPlists) -> Self {
        let name = match databases {
            MockPlists::Empty => "empty",
            MockPlists::BooksNew => "books-new",
            MockPlists::BooksAnnotated => "books-annotated",
        };

        MOCK_PLISTS.join(name)
    }
}
