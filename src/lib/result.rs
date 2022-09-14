//! Defines the [`LibResult`] type and the [`LibError`] enum for working with
//! this library.

/// A generic result type used in this library.
pub type LibResult<T> = std::result::Result<T, LibError>;

/// An enum representing all possible error's this library can run into.
#[derive(Debug, thiserror::Error)]
pub enum LibError {
    /// Error returned when the Apple Books databse cannot be found.
    #[error("Missing `{name}*.sqlite` in `{path}`")]
    DatabaseMissing {
        /// The basename of the database: `BKLibrary` or `AEAnnotation`.
        name: String,
        /// The full path to the missing database's parent folder.
        path: String,
    },

    /// Error returned when the Apple Books databse path cannot be resolved.
    #[error("Cannot resolve path to `{name}*.sqlite` in `{path}`")]
    DatabaseUnresolvable {
        /// The basename of the database: `BKLibrary` or `AEAnnotation`.
        name: String,
        /// The full path to the unresolvable database's parent folder.
        path: String,
    },

    /// Error returned if there are any issues with connecting to an Apple Books
    /// database.
    #[error("Unable to connect to `{name}*.sqlite` at `{path}`")]
    DatabaseConnection {
        /// The basename of the database: `BKLibrary` or `AEAnnotation`.
        name: String,
        /// The path where the database was looked for.
        path: String,
    },

    /// Error returned when querying the database fails. This means that the
    /// Apple Books database schema is different than the one the query has been
    /// designed against. In that case the currently installed version of Apple
    /// Books us unsupported.
    #[error("Apple Books {version} unsupported")]
    UnsupportedVersion {
        /// The currently installed Apple Books version number.
        version: String,
    },

    #[error(
        "Cannot read config for: `{path}`. Templates must have their config \
        defined in yaml between an opening block: '<!-- readstor' and a closing \
        block: '-->'. "
    )]
    /// Error returned when a syntax error is detected in how a template's
    /// config block is defined. This does not include yaml syntax error.
    InvalidTemplateConfig {
        /// The partial path to the template e.g. `nested/template.md`.
        path: String,
    },

    /// Error returned if [`tera`] encounters any errors.
    #[error(transparent)]
    InvalidTemplate(#[from] tera::Error),

    /// Error returned if [`serde_yaml`] encounters any errors in deserialization.
    #[error(transparent)]
    DeserializationError(#[from] serde_yaml::Error),

    /// Error returned if any other IO errors are encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
