use std::path::Path;
use std::path::PathBuf;

use rusqlite::OpenFlags;
use rusqlite::{Connection, Row};

use super::defaults::APPLEBOOKS_DATABASES;
#[allow(unused_imports)] // For docs.
use crate::lib::models::annotation::Annotation;
#[allow(unused_imports)] // For docs.
use crate::lib::models::book::Book;
use crate::lib::result::{ApplicationError, Result};

pub struct ABDatabase;

impl ABDatabase {
    /// Queries the appropriate Apple Books database and returns a
    /// `Result<Vec<T>>` where `T` is a type that implements [`ABQueryable`].
    /// In most cases `T` would be either [`Book`] or [`Annotation`].
    ///
    /// See [`static@APPLEBOOKS_DATABASES`] for more information.
    pub fn query<T: ABQueryable>() -> Result<Vec<T>> {
        // Returns the appropriate database based on `T`.
        let path = Self::get_database::<T>(&APPLEBOOKS_DATABASES)?;

        let connection =
            match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
                Ok(connection) => connection,
                Err(_) => {
                    return Err(ApplicationError::DatabaseConnection {
                        name: T::DATABASE_NAME.to_string(),
                        path: path.display().to_string(),
                    });
                }
            };

        let mut statement = match connection.prepare(T::QUERY) {
            Ok(statement) => statement,
            Err(_) => {
                return Err(ApplicationError::DatabaseUnsupported);
            }
        };

        let items = statement
            .query_map([], |row| Ok(T::from_row(row)))
            // The `rusqlite` documentation for `query_map` states 'Will return
            // Err if binding parameters fails.' So this should be safe because
            // `query_map` is given no parameters here.
            .unwrap()
            // Using `filter_map` here because we know from a few lines above
            // that all the items are wrapped in an `Ok`. At this point the
            // there should be nothing that would fail in regards to querying
            // and creating an instance of T unless there's an error in the
            // implementation of the [`ABQueryable`] trait. See [`ABQueryable`]
            // for more information.
            .filter_map(std::result::Result::ok)
            .collect();

        Ok(items)
    }

    /// Returns a `PathBuf` to the `AEAnnotation` or `BKLibrary` database.
    ///
    /// The `databases` directory should contains the following structure as
    /// this is the way Apple Books' `Documents` directory is set up.
    ///
    /// ```plaintext
    /// [databases]
    ///  │
    ///  ├─ AEAnnotation
    ///  │  ├─ AEAnnotation*.sqlite
    ///  │  └─ ...
    ///  │
    ///  ├─ BKLibrary
    ///  │  ├─ BKLibrary*.sqlite
    ///  │  └─ ...
    ///  │
    ///  └─ ...
    /// ```
    fn get_database<T: ABQueryable>(databases: &Path) -> Result<PathBuf> {
        let mut root = databases.to_owned();

        // Appends `DATABASE_NAME` twice to the root path. Prepping the path
        // for generating a glob string.
        //
        // [databases]
        //  │
        //  ├─ DATABASE_NAME
        //  │  ├─ DATABASE_NAME*.sqlite
        //  │  └─ ...
        //  │
        //  └─ ...
        root.extend([T::DATABASE_NAME.to_string(), T::DATABASE_NAME.to_string()].iter());

        // The path should be valid and UTF-8 because we're targeting macOS.
        // TODO Is this actually the case? Run tests to see it break.
        let path = root.display().to_string();

        // Generates `/path/to/databases/DATABASE_NAME/DATABASE_NAME*.sqlite`
        let glob_path = format!("{}*.sqlite", path);

        let mut databases: Vec<PathBuf> = glob::glob(&glob_path)
            .unwrap() // This is safe as we know the glob pattern is valid.
            .filter_map(std::result::Result::ok)
            .collect();

        if databases.is_empty() {
            return Err(ApplicationError::DatabaseMissing {
                name: T::DATABASE_NAME.to_string(),
                path,
            });
        }

        // The default Apple Books' database directory contains only a single
        // database file that starts with the `DATABASE_NAME` and ends with
        // `.sqlite`. If there are more then we'd possibly run into unexpected
        // behaviors.
        if databases.len() > 1 {
            return Err(ApplicationError::DatabaseUnresolvable {
                name: T::DATABASE_NAME.to_string(),
                path,
            });
        }

        // This is safe because we know that databases vec is not empty.
        Ok(databases.pop().unwrap())
    }
}

/// This trait is an attempt at reducing code duplication and standardizing
/// how [`Book`] and [`Annotation`] instances are created. Thus it should only
/// have to be implemented by said structs. It allows instances to be created
/// generically over the rows of their respective databases `BKLibrary*.sqlite`
/// and `AEAnnotation*.sqlite`. See [`static@APPLEBOOKS_DATABASES`] for path
/// information.
///
/// The [`ABQueryable::from_row`] and [`ABQueryable::QUERY`] methods are
/// strongly coupled in that the declared rows in the `SELECT` statement *must*
/// map directly to the `rusqlite`'s `Row.get` method e.g. the first row of the
/// `SELECT` statement maps to `row.get(0)` etc. The `unwrap` on the `Row.get`
/// methods will panic if the index is out of range or the there's a type
/// mismatch to the struct field it's been mapped to.
///
/// The databases seem to be related via a UUID field.
///
/// ```plaintext
/// Book         ZBKLIBRARYASSET.ZASSETID ─────────┐
/// Annotation   ZAEANNOTATION.ZANNOTATIONASSETID ─┘
/// ```
pub trait ABQueryable {
    /// The database's name being either `BKLibrary` or `AEAnnotation`.
    const DATABASE_NAME: ABDatabaseName;

    /// The query to retrieve rows that are subsequently passed into
    /// [`ABQueryable::from_row`] to create instances of the implemented type.
    const QUERY: &'static str;

    /// Constructs an instance of the respective type from a `rusqlite::Row`.
    fn from_row(row: &Row) -> Self;
}

/// Describes Apple Books' two databases.
pub enum ABDatabaseName {
    BKLibrary,
    AEAnnotation,
}

/// Provides a `to_string` method to convert `ABDatabaseName`s to `String`s.
impl std::fmt::Display for ABDatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ABDatabaseName::BKLibrary => write!(f, "BKLibrary"),
            ABDatabaseName::AEAnnotation => write!(f, "AEAnnotation"),
        }
    }
}
