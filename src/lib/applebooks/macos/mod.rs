//! Defines types for interacting with the macOS's Apple Books databases.
//!
//! The [`ABMacos`] struct, is used to to directly interact with the Apple ! Books databases while
//! the [`ABQuery`] trait provides an interface for ! generating types from either of the Apple Books
//! databases.

pub mod defaults;
pub mod utils;

use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags};

use crate::result::{Error, Result};

use self::utils::APPLEBOOKS_VERSION;

/// A struct for interacting with macOS's Apple Books databases.
///
/// A directory containing macOS's Apple Books databases should conform to the following structure
/// as this is how the official directory is structured.
///
/// ```plaintext
/// [databases]
///  │
///  ├── AEAnnotation
///  │   ├── AEAnnotation*.sqlite
///  │   └── ...
///  │
///  ├── BKLibrary
///  │   ├── BKLibrary*.sqlite
///  │   └── ...
///  └── ...
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ABMacos;

impl ABMacos {
    /// Extracts data from the books database and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing macOS's Apple Books databases.
    ///
    /// See [`ABMacos`] for more information on how the databases directory should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The database cannot be found/opened.
    /// * The version of Apple Books is unsupported.
    pub fn extract_books<T>(path: &Path) -> Result<Vec<T>>
    where
        T: ABQuery,
    {
        Self::query::<T>(path, ABDatabase::Books)
    }

    /// Extracts data from the annotations database and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing macOS's Apple Books databases.
    ///
    /// See [`ABMacos`] for more information on how the databases directory should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The database cannot be found/opened.
    /// * The version of Apple Books is unsupported.
    pub fn extract_annotations<T>(path: &Path) -> Result<Vec<T>>
    where
        T: ABQuery,
    {
        Self::query::<T>(path, ABDatabase::Annotations)
    }

    /// Queries and extracts data from one of the databases and converts them into `T`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing macOS's Apple Books databases.
    /// * `database` - Which database to query.
    ///
    /// See [`ABMacos`] for more information on how the databases directory should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if:
    /// * The database cannot be found/opened
    /// * The version of Apple Books is unsupported.
    #[allow(clippy::missing_panics_doc)]
    fn query<T>(path: &Path, database: ABDatabase) -> Result<Vec<T>>
    where
        T: ABQuery,
    {
        // Returns the appropriate database based on its name.
        let path = Self::get_database(path, database)?;

        let Ok(connection) = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        else {
            return Err(Error::DatabaseConnection {
                name: database.to_string(),
                path: path.display().to_string(),
            });
        };

        // This will only fail if the database schema has changes. This means that the Apple Books
        // database schema is different than the one the query has been designed against. In that
        // case,  the currently installed version of Apple Books is unsupported.
        let mut statement = match connection.prepare(T::QUERY) {
            Ok(statement) => statement,
            Err(error) => {
                return Err(Error::UnsupportedMacosVersion {
                    error: error.to_string(),
                    version: APPLEBOOKS_VERSION.to_owned(),
                });
            }
        };

        let items = statement
            .query_map([], |row| Ok(T::from_row(row)))
            // The `rusqlite` documentation for `query_map` states 'Will return Err if binding
            // parameters fails.' So this should be safe because `query_map` is given no parameters.
            .unwrap()
            // Using `filter_map` here because we know from a few lines above that all the items
            // are wrapped in an `Ok`. At this point the there should be nothing that would fail
            // in regards to querying and creating an instance of T unless there's an error in the
            // implementation of the `ABQuery` trait. See `ABQuery` for more information.
            .filter_map(std::result::Result::ok)
            .collect();

        Ok(items)
    }

    /// Returns a [`PathBuf`] to the `AEAnnotation` or `BKLibrary` database.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory containing macOS's Apple Books databases.
    /// * `database` - Which database path to get.
    ///
    /// See [`ABMacos`] for more information on how the databases directory should be structured.
    fn get_database(path: &Path, database: ABDatabase) -> Result<PathBuf> {
        // (a) -> `/path/to/databases/DATABASE_NAME/`
        let path = path.join(database.to_string());

        // (b) -> `/path/to/databases/DATABASE_NAME/DATABASE_NAME*.sqlite`
        let pattern = format!("{database}*.sqlite");
        let pattern = path.join(pattern);
        let pattern = pattern.to_string_lossy();

        let mut databases: Vec<PathBuf> = glob::glob(&pattern)
            // This should be safe to unwrap seeing we know the pattern is valid and in production
            // the path (b) will always be valid UTF-8 as it's a path to a default macOS
            // application's container.
            .unwrap()
            .filter_map(std::result::Result::ok)
            .collect();

        // macOS's default Apple Books database directory contains only a single database file that
        // starts with the `DATABASE_NAME` and ends with `.sqlite`. If there are more then we'd
        // possibly run into unexpected behaviors.
        match &databases[..] {
            [_] => Ok(databases.pop().unwrap()),
            _ => Err(Error::MissingDefaultDatabase),
        }
    }
}

/// A trait for standardizing how types are created from the Apple Books databases.
///
/// This trait allows for instances to be created generically over the rows of their respective
/// databases `BKLibrary*.sqlite` and `AEAnnotation*.sqlite`.
///
/// The [`ABQuery::from_row()`] and [`ABQuery::QUERY`] methods are strongly coupled in that the
/// declared rows in the `SELECT` statement *must* map directly to the `rusqlite`'s `Row::get()`
/// method e.g. the first row of the `SELECT` statement maps to `row.get(0)` etc. The `unwrap` on
/// the `Row::get()` methods will panic if the index is out of range or the there's a type mismatch
/// to the struct field it's been mapped to.
///
/// The databases seem to be related via a UUID field.
///
/// ```plaintext
/// Book         ZBKLIBRARYASSET.ZASSETID ─────────┐
/// Annotation   ZAEANNOTATION.ZANNOTATIONASSETID ─┘
/// ```
pub trait ABQuery {
    /// The query to retrieve rows from the database. The rows are then passed
    /// into [`ABQuery::from_row()`] to create instances of the implementing
    /// type.
    const QUERY: &'static str;

    /// Constructs an instance of the implementing type from a [`rusqlite::Row`].
    fn from_row(row: &rusqlite::Row<'_>) -> Self;
}

/// An enum representing macOS's Apple Books databases.
#[derive(Debug, Clone, Copy)]
pub enum ABDatabase {
    /// The books database.
    Books,

    /// The annotations database.
    Annotations,
}

impl std::fmt::Display for ABDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ABDatabase::Books => write!(f, "BKLibrary"),
            ABDatabase::Annotations => write!(f, "AEAnnotation"),
        }
    }
}
