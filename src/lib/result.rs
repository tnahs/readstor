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

    /// Error returned when a template does not follow the proper naming
    /// convention.
    #[error(
        "Invalid template name for `{path}`. Templates follow a strict naming convention: \
        `[template-kind].[template-name].[template-ext]` where `[template-kind]` must either \
        `multi`, `single` or `partial`."
        // TODO: Add link guides when they are ready.
    )]
    InvalidTemplateName {
        /// The full path to the invalid template.
        path: String,
    },

    /// Error returned if [`tera`] encounters any errors.
    #[error(transparent)]
    InvalidTemplate(#[from] tera::Error),

    /// Error returned if [`serde`] encounters any errors.
    #[error(transparent)]
    SerializationError(#[from] serde_json::error::Error),

    /// Error returned if any other IO errors are encountered.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
