use std::path::{Path, PathBuf};

use rusqlite::OpenFlags;
use rusqlite::{Connection, Row};

#[allow(unused_imports)] // For docs.
use crate::lib::models::annotation::Annotation;
#[allow(unused_imports)] // For docs.
use crate::lib::models::book::Book;
use crate::lib::result::{LibError, LibResult};

#[allow(unused_imports)] // For docs.
use super::defaults as applebooks_defaults;
use super::utils::APPLEBOOKS_VERSION;

pub struct ABDatabase;

impl ABDatabase {
    /// Queries an Apple Books database based on the `databases` path and `T`.
    ///
    /// The `databases` path determines where the databases are located while
    /// `T` determines which of the two databases will be queried. `T` should
    /// be either [`Book`] or [`Annotation`] referring to `BKLibrary*.sqlite`
    /// or `AEAnnotation*.sqlite`.
    ///
    /// See [`ABDatabase::get_database()`] for information on how the
    /// `databases` directory should be structured.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the database cannot be opened or the underlying
    /// schema has changed, meaning this application is out of sync with the
    /// latest version of Apple Books.
    #[allow(clippy::missing_panics_doc)]
    pub fn query<T: ABQuery>(databases: &Path) -> LibResult<Vec<T>> {
        // Returns the appropriate database based on `T`.
        let path = Self::get_database::<T>(databases)?;

        let connection =
            match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
                Ok(connection) => connection,
                Err(_) => {
                    return Err(LibError::DatabaseConnection {
                        name: T::DATABASE_NAME.to_string(),
                        path: path.display().to_string(),
                    });
                }
            };

        let mut statement = match connection.prepare(T::QUERY) {
            Ok(statement) => statement,
            Err(_) => {
                return Err(LibError::UnsupportedVersion {
                    version: APPLEBOOKS_VERSION.to_owned(),
                });
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
            // implementation of the [`ABQuery`] trait. See [`ABQuery`] for
            // more information.
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
    fn get_database<T: ABQuery>(databases: &Path) -> LibResult<PathBuf> {
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
            return Err(LibError::DatabaseMissing {
                name: T::DATABASE_NAME.to_string(),
                path,
            });
        }

        // The default Apple Books' database directory contains only a single
        // database file that starts with the `DATABASE_NAME` and ends with
        // `.sqlite`. If there are more then we'd possibly run into unexpected
        // behaviors.
        if databases.len() > 1 {
            return Err(LibError::DatabaseUnresolvable {
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
/// and `AEAnnotation*.sqlite`. See [`static@applebooks_defaults::DATABASES`]
/// for path information.
///
/// The [`ABQuery::from_row()`] and [`ABQuery::QUERY`] methods are strongly
/// coupled in that the declared rows in the `SELECT` statement *must* map
/// directly to the `rusqlite`'s `Row.get` method e.g. the first row of the
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
pub trait ABQuery {
    /// The database's name being either `BKLibrary` or `AEAnnotation`.
    const DATABASE_NAME: ABDatabaseName;

    /// The query to retrieve rows that are subsequently passed into
    /// [`ABQuery::from_row()`] to create instances of the implemented type.
    const QUERY: &'static str;

    /// Constructs an instance of the respective type from a `rusqlite::Row`.
    fn from_row(row: &Row) -> Self;
}

/// Describes Apple Books' two databases.
pub enum ABDatabaseName {
    Books,
    Annotations,
}

/// Provides a `to_string` method to convert `ABDatabaseName`s to `String`s.
impl std::fmt::Display for ABDatabaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ABDatabaseName::Books => write!(f, "BKLibrary"),
            ABDatabaseName::Annotations => write!(f, "AEAnnotation"),
        }
    }
}
